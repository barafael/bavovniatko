#!/usr/bin/env python3
"""Entity resolution for staged claims: from free-text mentions to canonical records.

  entities.py propose     cluster all mentions/claimants/places/counter systems across db/staging/claims/
                          -> db/staging/entities/proposed.yaml (input for the curation step)
  entities.py unmatched   list staged names that the curated registry does not resolve yet
  entities.py geocode     look up places without geometry via OSM Nominatim (1 req/s, cached)
                          -> db/staging/entities/geocode.jsonl

The curated registry is db/vocab/entities/*.yaml (one list per entity type):
  - {id: system:geran_3, type: system, kind: long_range_drone, en: "Geran-3", uk: "Герань-3",
     aliases: ["jet Geran", "Geran-3 jet drone"], parent: system:shahed_geran, note: "..."}
Every staged name must resolve through `en` or `aliases` (case/diacritics/punctuation-insensitive).
"""
from __future__ import annotations

import argparse
import json
import re
import sys
import time
import unicodedata
from collections import Counter, defaultdict
from pathlib import Path

import requests
import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402

STAGING = kb.DB_DIR / "staging"
ENT_DIR = STAGING / "entities"
REGISTRY = kb.DB_DIR / "vocab" / "entities"          # one YAML list per file: actor.yaml, system.yaml, …
GEOCODE = ENT_DIR / "geocode.jsonl"
UA = "bavovniatko-research/0.1"          # no contact details (privacy); Nominatim asks for an identifying UA


def norm(name: str) -> str:
    s = unicodedata.normalize("NFKD", name).encode("ascii", "ignore").decode() or name
    s = s.lower().replace("’", "'").replace("ʼ", "'")
    s = re.sub(r"^(the)\s+", "", s)
    s = re.sub(r"[^a-z0-9а-яіїєґ]+", " ", s).strip()
    return s


def staged():
    for f in sorted((STAGING / "claims").glob("*.jsonl")):
        for line in f.read_text().splitlines():
            if line.strip():
                yield f.stem, json.loads(line)


def mentions():
    """Yield (type, name, kind, topic, local_id) for every entity reference in the staging files."""
    for nn, r in staged():
        lid = r["local_id"]
        if r["type"] == "counter":
            yield "system", r["counter"], r.get("counter_kind"), nn, lid
            yield "system", r["measure"], r.get("measure_kind"), nn, lid
            continue
        for m in r.get("mentions", []):
            if m["type"] != "metric":
                yield m["type"], m["name"], m.get("kind"), nn, lid
        for c in r.get("claimants", []):
            yield "actor", c["name"], c.get("kind"), nn, lid
        for o in r.get("observations", []):
            if o.get("place"):
                yield "place", o["place"], None, nn, lid


def load_registry() -> list[dict]:
    out = []
    for f in sorted(REGISTRY.glob("*.yaml")) if REGISTRY.is_dir() else []:
        out += yaml.safe_load(f.read_text()) or []
    return out


def resolver() -> dict[tuple[str, str], str]:
    """(type, normalised name) -> record id string, from the curated registry."""
    out = {}
    for e in load_registry():
        for n in [e["en"], *(e.get("aliases") or [])]:
            out[(e["type"], norm(n))] = e["id"]
    return out


def cmd_propose():
    clusters: dict[tuple[str, str], dict] = {}
    for typ, name, kind, nn, lid in mentions():
        key = (typ, norm(name))
        c = clusters.setdefault(key, {"names": Counter(), "kinds": Counter(), "topics": set(), "n": 0, "examples": []})
        c["names"][name] += 1
        if kind:
            c["kinds"][kind] += 1
        c["topics"].add(nn)
        c["n"] += 1
        if len(c["examples"]) < 3:
            c["examples"].append(lid)
    # publishers already in the database are actors too
    pubs = {norm(r["labels"]["en"]): r["id"] for r in kb.one(kb.connect(), "SELECT id, labels FROM actor")}
    out = defaultdict(list)
    for (typ, n), c in sorted(clusters.items(), key=lambda kv: (kv[0][0], -kv[1]["n"])):
        out[typ].append({
            "name": c["names"].most_common(1)[0][0], "variants": sorted(c["names"]), "n": c["n"],
            "kinds": dict(c["kinds"]), "topics": sorted(c["topics"]), "examples": c["examples"],
            **({"existing": kb.rid_str(pubs[n])} if typ == "actor" and n in pubs else {})})
    ENT_DIR.mkdir(parents=True, exist_ok=True)
    (ENT_DIR / "proposed.yaml").write_text(yaml.safe_dump(dict(out), sort_keys=False, allow_unicode=True, width=140))
    print({t: len(v) for t, v in out.items()}, "->", ENT_DIR / "proposed.yaml")


def cmd_unmatched(only: str | None = None) -> int:
    res = resolver()
    missing = Counter()
    for typ, name, *_ in mentions():
        if only and typ != only:
            continue
        if (typ, norm(name)) not in res:
            missing[(typ, name)] += 1
    for (t, n), k in missing.most_common():
        print(f"{t:7} {k:4}  {n}")
    print(f"{len(missing)} unresolved names")
    return 1 if missing else 0


def cmd_validate(only: str | None = None) -> int:
    kinds = yaml.safe_load((kb.DB_DIR / "vocab" / "kinds.yaml").read_text())
    reg = load_registry()
    ids = Counter(e.get("id") for e in reg)
    errs = [f"duplicate id {i}" for i, n in ids.items() if n > 1]
    names = defaultdict(set)
    for e in reg:
        if only and e.get("type") != only:
            continue
        i = e.get("id", "")
        if not re.fullmatch(r"(actor|place|event|system|work):[a-z0-9_]+", i):
            errs.append(f"bad id {i!r} (table:snake_case)")
        elif i.split(":")[0] != e.get("type"):
            errs.append(f"{i}: type {e.get('type')} does not match id")
        if e.get("kind") not in kinds.get(e.get("type"), []):
            errs.append(f"{i}: kind {e.get('kind')} not in kinds.yaml[{e.get('type')}]")
        if not e.get("en"):
            errs.append(f"{i}: missing en")
        if e.get("parent") and e["parent"] not in ids:
            errs.append(f"{i}: parent {e['parent']} not in registry")
        for n in [e.get("en", ""), *(e.get("aliases") or [])]:
            names[(e.get("type"), norm(n))].add(i)
        if e.get("bbox") and len(e["bbox"]) != 4:
            errs.append(f"{i}: bbox must be [west, south, east, north]")
    errs += [f"name '{n}' ({t}) maps to several ids: {sorted(v)}" for (t, n), v in names.items() if len(v) > 1]
    for x in errs[:80]:
        print("ERROR", x)
    print(f"{len(reg)} entries, {len(errs)} errors")
    return 1 if errs else 0


def query_variants(q: str) -> list[str]:
    """The curated query first, then OSM spellings, then the bare name (Nominatim is strict about admin names)."""
    alt = q.replace("Zaporizhzhia Oblast", "Zaporizhia Oblast").replace(" River,", ",")
    bare = q.split(",")[0].replace(" River", "").strip()
    out = []
    for v in (q, alt, bare):
        if v and v not in out:
            out.append(v)
    return out


def cmd_geocode(limit: int | None, retry_failed: bool = False):
    cache = {}
    if GEOCODE.exists():
        for l in GEOCODE.read_text().splitlines():
            if l.strip():
                r = json.loads(l)
                cache[r["id"]] = r
    done = {i for i, r in cache.items() if r.get("found") or not retry_failed}
    todo = [e for e in load_registry() if e["type"] == "place" and not e.get("geometry") and e["id"] not in done
            and e.get("kind") not in ("sample_box", "front_sector", "axis") and not e.get("no_geocode")]
    if limit:
        todo = todo[:limit]
    print(f"geocoding {len(todo)} places via Nominatim")
    with open(GEOCODE, "a") as fh:
        for e in todo:
            hits = []
            for q in query_variants(e.get("geocode_query") or e["en"]):
                params = {"q": q, "format": "jsonv2", "polygon_geojson": 1, "limit": 3, "accept-language": "en",
                          "countrycodes": e.get("countrycodes", "ua,ru,by,md")}
                try:
                    r = requests.get("https://nominatim.openstreetmap.org/search", params=params, headers={"User-Agent": UA}, timeout=30)
                    hits = r.json() if r.ok else []
                except Exception as ex:
                    print("  error", e["id"], ex)
                time.sleep(1.1)
                if hits:
                    break
            best = hits[0] if hits else None
            rec = {"id": e["id"], "query": q, "found": bool(best)}
            if best:
                rec.update({"osm": f"{best.get('osm_type')}/{best.get('osm_id')}", "display_name": best.get("display_name"),
                            "class": best.get("category") or best.get("class"), "type": best.get("type"),
                            "lat": float(best["lat"]), "lon": float(best["lon"]), "geojson": best.get("geojson")})
                if rec.get("geojson"):                     # store simplified geometry (the loader uses the same tolerances)
                    import load
                    tol = load.SIMPLIFY.get(e.get("kind"), load.DEFAULT_TOL)
                    rec["geojson"], rec["simplified_tolerance_deg"] = load.simplify(rec["geojson"], tol), tol
            fh.write(json.dumps(rec, ensure_ascii=False) + "\n")
            fh.flush()
            print(f"  {'ok ' if best else '-- '} {e['id']:40} {rec.get('display_name', '')[:80]}")


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    sub.add_parser("propose")
    u = sub.add_parser("unmatched"); u.add_argument("--type")
    v = sub.add_parser("validate"); v.add_argument("--type")
    g = sub.add_parser("geocode"); g.add_argument("--limit", type=int); g.add_argument("--retry-failed", action="store_true")
    a = ap.parse_args()
    if a.cmd == "propose":
        cmd_propose()
    elif a.cmd == "unmatched":
        sys.exit(cmd_unmatched(a.type))
    elif a.cmd == "validate":
        sys.exit(cmd_validate(a.type))
    else:
        cmd_geocode(a.limit, a.retry_failed)


if __name__ == "__main__":
    main()
