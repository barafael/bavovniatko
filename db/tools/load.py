#!/usr/bin/env python3
"""Phase 4: load curated entities and staged claims into the knowledge base.

  load.py sources                  db/staging/sources/*.yaml (new sources, research/sources.yaml format) -> source, actor
  load.py entities                 db/vocab/entities/*.yaml (+ geocode cache) -> place/event/system/actor/work
  load.py metrics                  db/vocab/metrics.yaml -> metric
  load.py claims [--topic NN]      db/staging/claims/*.jsonl -> claim, cites, asserted_by, about,
                                   observation, includes, counters
  load.py all                      the three above, in order

Claims keep ids derived from their staging local_id (claim:c03_0004_01). A claim whose content hash
is unchanged is skipped; a changed claim is updated and its outgoing edges and observations rebuilt,
so re-running is safe. Every staged name must resolve through the entity registry
(see tools/entities.py unmatched); unresolved names abort the load.
"""
from __future__ import annotations

import argparse
import calendar
import datetime as dt
import hashlib
import json
import re
import sys
from collections import Counter
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
import entities as ent  # noqa: E402
import kb  # noqa: E402
import mdparse  # noqa: E402

UTC = dt.timezone.utc
NOW = dt.datetime.now(UTC)
RUN = "load@" + NOW.strftime("%Y%m%dT%H%M%SZ")
VOCAB = kb.DB_DIR / "vocab"
OSM_LICENCE = "ODbL 1.0, © OpenStreetMap contributors"


def prov(method: str, input_: str | None = None, by: str = "load") -> dict:
    p = {"by": by, "method": method, "run": RUN, "at": NOW}
    if input_:
        p["input"] = input_
    return p


def pdate(s: str) -> dt.datetime:
    parts = [int(x) for x in s.split("-")]
    return dt.datetime(parts[0], parts[1] if len(parts) > 1 else 1, parts[2] if len(parts) > 2 else 1, tzinfo=UTC)


def ptime(t: dict | None) -> dict | None:
    """Staging time -> schema time. `to` keeps the start of its last period; precision covers both ends."""
    if not t:
        return None
    out = {"from": pdate(t["from"]), "precision": t["precision"]}
    if t.get("to"):
        out["to"] = pdate(t["to"])
    return out


def rec_id(s: str):
    table, key = s.split(":", 1)
    return kb.rid(table, key)


def claim_rid(local_id: str):
    return kb.rid("claim", "c" + local_id.replace("-", "_"))


def lbl(e: dict) -> dict:
    return {k: v for k, v in {"en": e["en"], "uk": e.get("uk"), "uk_translit": e.get("uk_translit"),
                              "ru": e.get("ru"), "ru_translit": e.get("ru_translit")}.items() if v}


def h(obj) -> str:
    return hashlib.sha256(json.dumps(obj, sort_keys=True, default=str).encode()).hexdigest()[:16]


# --- entities ------------------------------------------------------------------------------------

# Douglas–Peucker tolerance (degrees) per place kind: strategic-map detail, not cadastral precision
SIMPLIFY = {"country": 0.02, "oblast": 0.005, "raion": 0.003, "region": 0.005, "sea": 0.01, "reservoir": 0.002,
            "forest": 0.001, "landform": 0.002, "river": 0.002, "road": 0.001, "hromada": 0.001}
DEFAULT_TOL = 0.0003


def _dp(pts, tol):
    if len(pts) < 3:
        return pts
    (x1, y1), (x2, y2) = pts[0], pts[-1]
    dx, dy = x2 - x1, y2 - y1
    norm = (dx * dx + dy * dy) ** 0.5 or 1e-12
    dmax, idx = 0.0, 0
    for i in range(1, len(pts) - 1):
        d = abs(dy * pts[i][0] - dx * pts[i][1] + x2 * y1 - y2 * x1) / norm
        if d > dmax:
            dmax, idx = d, i
    if dmax <= tol:
        return [pts[0], pts[-1]]
    return _dp(pts[: idx + 1], tol)[:-1] + _dp(pts[idx:], tol)


def _ring(r, tol):
    """Simplify a closed ring: split at the vertex farthest from the start, simplify both halves, re-close."""
    pts = [[round(x, 5), round(y, 5)] for x, y in r]
    if pts[0] == pts[-1]:
        pts = pts[:-1]
    if len(pts) < 4:
        return pts + [pts[0]]
    x0, y0 = pts[0]
    far = max(range(len(pts)), key=lambda i: (pts[i][0] - x0) ** 2 + (pts[i][1] - y0) ** 2)
    a = _dp(pts[: far + 1], tol)
    b = _dp(pts[far:] + [pts[0]], tol)
    s = a[:-1] + b
    if len(s) < 4:
        s = [pts[0], pts[len(pts) // 3], pts[2 * len(pts) // 3], pts[0]]
    return s


def simplify(g: dict, tol: float) -> dict:
    t = g["type"]
    c = g.get("coordinates")
    if t == "Point":
        return {"type": t, "coordinates": [round(c[0], 5), round(c[1], 5)]}
    if t == "LineString":
        return {"type": t, "coordinates": _dp([[round(x, 5), round(y, 5)] for x, y in c], tol)}
    if t == "MultiLineString":
        return {"type": t, "coordinates": [_dp([[round(x, 5), round(y, 5)] for x, y in l], tol) for l in c]}
    if t == "Polygon":
        return {"type": t, "coordinates": [_ring(r, tol) for r in c]}
    if t == "MultiPolygon":
        polys = [[_ring(r, tol) for r in p] for p in c]
        return {"type": t, "coordinates": polys}
    if t == "GeometryCollection":
        return {"type": t, "geometries": [simplify(x, tol) for x in g["geometries"]]}
    return g


def bbox_polygon(b):
    w, s, e, n = b
    return {"type": "Polygon", "coordinates": [[[w, s], [e, s], [e, n], [w, n], [w, s]]]}


def load_entities(conn) -> Counter:
    reg = ent.load_registry()
    geo = {}
    if ent.GEOCODE.exists():
        for l in ent.GEOCODE.read_text().splitlines():
            if l.strip():
                r = json.loads(l)
                geo[r["id"]] = r
    have = {kb.rid_str(r["id"]): r.get("h") for t in ("place", "event", "system", "actor", "work")
            for r in kb.one(conn, f"SELECT id, ext._hash AS h FROM {t}")}
    stats = Counter()
    # parents first so `parent` references resolve on insert order irrelevant (plain record fields)
    for e in reg:
        rid = rec_id(e["id"])
        table = rid.table_name
        data = {"labels": lbl(e), "aliases": sorted(set(e.get("aliases") or [])),
                "kind": kb.rid("kind", [table, e["kind"]]), "prov": prov("human" if e.get("curated") else "agent", "db/vocab/entities.yaml")}
        if e.get("parent"):
            data["parent"] = rec_id(e["parent"])
        if e.get("note"):
            data["summary" if table in ("event", "system", "work") else "notes"] = e["note"]
        if table == "actor" and e.get("side"):
            data["side"] = e["side"]
        if table == "system" and e.get("sides"):
            data["sides"] = e["sides"]
        if table == "place":
            g = e.get("geometry") or (bbox_polygon(e["bbox"]) if e.get("bbox") else None)
            src = e.get("geometry_source") or ("stated in research/06-terrain-geodata.md" if e.get("bbox") else None)
            lic = e.get("geometry_licence")
            gc = geo.get(e["id"])
            if not g and gc and gc.get("found"):
                g = gc.get("geojson") or {"type": "Point", "coordinates": [gc["lon"], gc["lat"]]}
                tol = SIMPLIFY.get(e["kind"], DEFAULT_TOL)
                g = simplify(g, tol)
                src, lic = f"OSM Nominatim ({gc['osm']}), simplified (Douglas–Peucker {tol}°)", OSM_LICENCE
                data["external_ids"] = {"osm": gc["osm"]}
                data["centroid"] = kb.geom({"type": "Point", "coordinates": [gc["lon"], gc["lat"]]})
            if g:
                data["geometry"] = kb.geom(g)
                if g["type"] == "Point":
                    data["centroid"] = kb.geom(g)
                elif "centroid" not in data and e.get("bbox"):
                    w_, s_, e_, n_ = e["bbox"]
                    data["centroid"] = kb.geom({"type": "Point", "coordinates": [(w_ + e_) / 2, (s_ + n_) / 2]})
                data["geometry_source"], data["geometry_licence"] = src, lic
        hh = h({k: v for k, v in data.items() if k != "prov"} | {"g": repr(data.get("geometry"))})
        if have.get(e["id"]) == hh:
            stats[f"{table}:unchanged"] += 1
            continue
        data["ext"] = {"_hash": hh}
        # MERGE keeps fields set elsewhere (e.g. publisher actors from import_base)
        kb.run(conn, "UPSERT $id MERGE $d", {"id": rid, "d": data})
        stats[f"{table}:{'updated' if e['id'] in have else 'created'}"] += 1
    return stats


def metric_aliases() -> dict[str, str]:
    """Proposed metric slug -> canonical slug (db/vocab/metric_aliases.yaml)."""
    p = VOCAB / "metric_aliases.yaml"
    return (yaml.safe_load(p.read_text()) or {}) if p.exists() else {}


def load_metrics(conn) -> Counter:
    stats = Counter()
    units = {u["id"]: u for u in yaml.safe_load((VOCAB / "units.yaml").read_text())}
    for slug, (label, dim, unit) in yaml.safe_load((VOCAB / "metrics.yaml").read_text()).items():
        if unit not in units:
            raise SystemExit(f"metric {slug}: unknown unit {unit}")
        kb.run(conn, "UPSERT $id MERGE $d", {"id": kb.rid("metric", slug), "d": {
            "labels": {"en": label}, "description": label, "dimension": dim, "default_unit": kb.rid("unit", unit)}})
        stats["metric:upserted"] += 1
    return stats


# --- claims ----------------------------------------------------------------------------------------

def load_claims(conn, topics: list[str] | None) -> Counter:
    res = ent.resolver()
    metrics = set(yaml.safe_load((VOCAB / "metrics.yaml").read_text()))
    malias = metric_aliases()
    legacy = {lid: r["id"] for r in kb.one(conn, "SELECT id, legacy_ids FROM source") for lid in r["legacy_ids"]}
    passages = {r["key"]: r["id"] for r in kb.one(conn, "SELECT id, key FROM passage")}
    sections = {r["key"]: r["section"] for r in kb.one(conn, "SELECT key, section FROM passage")}
    files = {r["id"].id[1:]: r["file"] for r in kb.one(conn, "SELECT id, file FROM topic")}
    have = {r["key"]: r.get("h") for r in kb.one(conn, "SELECT key, ext._hash AS h FROM claim")}
    stats, problems = Counter(), []
    counters = []

    def resolve(typ, name):
        rid = res.get((typ, ent.norm(name)))
        if not rid:
            problems.append(f"unresolved {typ} '{name}'")
        return rid

    for nn, r in ent.staged():
        if topics and nn not in topics:
            continue
        if r["type"] == "counter":
            counters.append((nn, r))
            continue
        lid = r["local_id"]
        if mdparse.meta_section(sections.get(r["passage"], "").split(" / ")[0]):
            stats["claim:skipped_meta_section"] += 1       # design notes/questions are imported separately
            continue
        cid = claim_rid(lid)
        seq = int(lid.rsplit("-", 1)[1])
        data = {
            "key": lid, "text": r["text"], "kind": r["kind"], "epistemic": r["epistemic"],
            "status": r.get("status", "active"),
            "eras": [kb.rid("era", e) for e in r.get("eras", [])],
            "topics": [kb.rid("topic", f"t{nn}")], "sides": r.get("sides", []),
            "anchor": {"file": files[nn], "section": sections.get(r["passage"], ""), "order": seq, "local_id": lid},
            "prov": prov("agent", f"db/staging/claims/{nn}.jsonl", by="extraction"),
        }
        if r.get("time"):
            data["time"] = ptime(r["time"])
        if r.get("notes"):
            data["confidence_note"] = r["notes"]
        edges = []
        for c in r["cites"]:
            sid = legacy.get(c["source"])
            if not sid:
                problems.append(f"{lid}: unknown source {c['source']}")
                continue
            edges.append(("cites", sid, {k: c[k] for k in ("locator", "quote", "support") if k in c}))
        for c in r.get("claimants", []):
            a = resolve("actor", c["name"])
            if a:
                d = {"role": c.get("role", "claimant")}
                if c.get("said_at"):
                    d["said_at"] = ptime({"from": c["said_at"], "precision": "day" if c["said_at"].count("-") == 2 else "month"})
                edges.append(("asserted_by", rec_id(a), d))
        for m in r.get("mentions", []):
            if m["type"] == "metric":
                slug = malias.get(m["name"], m["name"])
                if slug in metrics:
                    edges.append(("about", kb.rid("metric", slug), {"role": m.get("role", "measure")}))
                continue
            e = resolve(m["type"], m["name"])
            if e:
                edges.append(("about", rec_id(e), {"role": m.get("role", "subject")}))
        obs = []
        for i, o in enumerate(r.get("observations", [])):
            o = {**o, "metric": malias.get(o["metric"], o["metric"])}
            if o["metric"] not in metrics:
                problems.append(f"{lid}: metric {o['metric']} not consolidated into metrics.yaml")
                continue
            od = {"claim": cid, "metric": kb.rid("metric", o["metric"]), "unit": kb.rid("unit", o["unit"]),
                  "value_text": o["value_text"], "qualifier": o.get("qualifier", "exact"),
                  "method": o.get("method", "reported"), "prov": data["prov"]}
            for k in ("value", "low", "high"):
                if k in o:
                    od[k] = float(o[k])
            if o.get("time") or r.get("time"):
                od["time"] = ptime(o.get("time") or r["time"])
            if o.get("side"):
                od["side"] = o["side"]
            if o.get("place"):
                p = resolve("place", o["place"])
                if p:
                    od["place"] = rec_id(p)
            note = "; ".join(x for x in (o.get("subject") and f"subject: {o['subject']}", o.get("note")) if x)
            if note:
                od["note"] = note
            obs.append((kb.rid("observation", f"{cid.id}_{i:02d}"), od))
        hh = h({"d": {k: v for k, v in data.items() if k != "prov"}, "e": edges,
                "o": [{k: v for k, v in x[1].items() if k != "prov"} for x in obs]} | {"p": r["passage"]})
        if have.get(lid) == hh:
            stats["claim:unchanged"] += 1
            continue
        data["ext"] = {"_hash": hh}
        q = ["UPSERT $cid CONTENT $data;",
             "DELETE $cid->cites, $cid->asserted_by, $cid->about, $cid<-includes;",
             "DELETE observation WHERE claim = $cid;",
             "RELATE $pid->includes->$cid SET order = $seq;"]
        vars_ = {"cid": cid, "data": data, "pid": passages[r["passage"]], "seq": seq}
        # de-duplicate edges per (table, target)
        seen = set()
        for j, (tbl, tgt, d) in enumerate(edges):
            if (tbl, kb.rid_str(tgt)) in seen:
                continue
            seen.add((tbl, kb.rid_str(tgt)))
            d = {**d, "prov": data["prov"]}
            q.append(f"RELATE $cid->{tbl}->$t{j} CONTENT $d{j};")
            vars_[f"t{j}"], vars_[f"d{j}"] = tgt, d
        for j, (oid, od) in enumerate(obs):
            q.append(f"CREATE $o{j} CONTENT $od{j};")
            vars_[f"o{j}"], vars_[f"od{j}"] = oid, od
        try:
            kb.run(conn, "BEGIN;\n" + "\n".join(q) + "\nCOMMIT;", vars_)
        except kb.KBError as ex:
            problems.append(f"{lid}: {ex}")
            continue
        stats[f"claim:{'updated' if lid in have else 'created'}"] += 1
        stats["observation:written"] += len(obs)

    for nn, r in counters:
        a, b = resolve("system", r["counter"]), resolve("system", r["measure"])
        if not (a and b):
            continue
        d = {"rationale": r["rationale"], "evidence": [claim_rid(x) for x in r["evidence"]],
             "prov": prov("agent", f"db/staging/claims/{nn}.jsonl", by="extraction")}
        if r.get("first_observed"):
            d["first_observed"] = ptime(r["first_observed"])
        for k in ("lag_days", "effect"):
            if k in r:
                d[k] = r[k]
        try:
            existing = kb.one(conn, "SELECT VALUE id FROM counters WHERE in = $a AND out = $b", {"a": rec_id(a), "b": rec_id(b)})
            if existing:
                prev = kb.one(conn, "SELECT VALUE evidence FROM ONLY $e", {"e": existing[0]}) or []
                ev = {kb.rid_str(x): x for x in [*prev, *d["evidence"]]}
                if len(ev) == len(prev):
                    stats["counters:unchanged"] += 1
                    continue
                kb.run(conn, "UPDATE $e MERGE $d", {"e": existing[0], "d": {"evidence": [ev[k] for k in sorted(ev)]}})
                stats["counters:merged"] += 1
            else:
                kb.run(conn, "RELATE $a->counters->$b CONTENT $d", {"a": rec_id(a), "b": rec_id(b), "d": d})
                stats["counters:created"] += 1
        except kb.KBError as ex:
            problems.append(f"{r['local_id']}: {ex}")
    if problems:
        print(f"{len(problems)} problems:")
        for p in sorted(set(problems))[:80]:
            print("  ", p)
        stats["problems"] = len(problems)
    return stats


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("what", choices=["sources", "entities", "metrics", "claims", "all"])
    ap.add_argument("--topic", action="append")
    args = ap.parse_args()
    conn = kb.connect()
    stats = Counter()
    if args.what in ("sources", "all"):
        # new sources for research intake; the initial corpus came from research/sources.yaml via import_base
        import import_base
        files = sorted((kb.DB_DIR / "staging" / "sources").glob("*.yaml"))
        if files:
            w = import_base.Writer(conn)
            import_base.load_sources(w, files)
            stats.update(w.stats)
    if args.what in ("metrics", "all"):
        stats += load_metrics(conn)
    if args.what in ("entities", "all"):
        stats += load_entities(conn)
    if args.what in ("claims", "all"):
        stats += load_claims(conn, args.topic)
    for k, v in sorted(stats.items()):
        print(f"{k:28} {v}")
    sys.exit(1 if stats.get("problems") else 0)


if __name__ == "__main__":
    main()
