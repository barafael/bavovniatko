#!/usr/bin/env python3
"""Phase 1: deterministic import (no LLM) of the research base into SurrealDB.

Loads, idempotently:
  vocab     kinds, eras (+ sub-phases), topics, units          from db/vocab/*.yaml
  sources   research/sources/*.yaml (merged, aliases kept)     -> source, actor (publishers, authors)
  terms     research/glossary.md tables                        -> term
  passages  research/0*.md split into blocks                   -> passage
  meta      "Open questions/gaps" and "Game/sim relevance"     -> question, design_note

Re-running only writes records whose content changed (so the audit trail stays meaningful).
"""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402
import mdparse  # noqa: E402

VOCAB = kb.DB_DIR / "vocab"
UTC = dt.timezone.utc
RUN = "import_base@" + dt.datetime.now(UTC).strftime("%Y%m%dT%H%M%SZ")
NOW = dt.datetime.now(UTC)

SOURCE_KIND = {"think-tank": "think_tank", "osint-dataset": "osint_dataset"}
ACTOR_KIND_FOR_SOURCE = {"think-tank": "think_tank", "news": "outlet", "review": "outlet", "official": "government_body",
                         "academic": "outlet", "game": "game_studio", "encyclopedia": "platform"}
SIDE_FOR_ORIGIN = {"ukrainian": "ua", "western": "western", "international": "international", "other": "other"}
PUBLISHER_ALIASES = {
    "DeepState UA": "DeepState", "ISW": "Institute for the Study of War",
    "Institute for the Study of War / Critical Threats Project": "Institute for the Study of War",
    "The Kyiv Independent": "Kyiv Independent", "United24 Media": "UNITED24 Media",
    "RFE/RL": "Radio Free Europe/Radio Liberty", "Mezha / Ukrainska Pravda": "Mezha",
    "Digital State": "Ministry of Digital Transformation of Ukraine",
}


def prov(input_: str | None = None) -> dict:
    p = {"by": "import_base", "method": "import", "run": RUN, "at": NOW}
    if input_:
        p["input"] = input_
    return p


def d(s) -> dt.datetime:
    if isinstance(s, dt.datetime):
        return s.astimezone(UTC)
    if isinstance(s, dt.date):
        return dt.datetime(s.year, s.month, s.day, tzinfo=UTC)
    return dt.datetime.fromisoformat(str(s)).replace(tzinfo=UTC)


def partial_date(v) -> tuple[dict | None, str | None]:
    """'2025-06-01' | '2025-06' | 2025 | date | 'unknown' -> (time object, raw)."""
    if v is None:
        return None, None
    raw = str(v)
    if isinstance(v, dt.date):
        return {"from": d(v), "precision": "day"}, raw
    m = re.fullmatch(r"(\d{4})(?:-(\d{2}))?(?:-(\d{2}))?", raw.strip())
    if not m:
        return None, raw
    y, mo, da = int(m[1]), m[2], m[3]
    if da:
        return {"from": dt.datetime(y, int(mo), int(da), tzinfo=UTC), "precision": "day"}, raw
    if mo:
        return {"from": dt.datetime(y, int(mo), 1, tzinfo=UTC), "precision": "month"}, raw
    return {"from": dt.datetime(y, 1, 1, tzinfo=UTC), "precision": "year"}, raw


def labels(en: str, **kw) -> dict:
    out = {"en": en}
    out.update({k: v for k, v in kw.items() if v})
    return out


def content_hash(obj) -> str:
    def default(o):
        if isinstance(o, dt.datetime):
            return o.isoformat()
        if hasattr(o, "table_name"):
            return kb.rid_str(o)
        if hasattr(o, "get_coordinates"):
            return repr(o)
        return str(o)
    body = {k: v for k, v in obj.items() if k not in ("prov",)}
    return hashlib.sha256(json.dumps(body, sort_keys=True, default=default).encode()).hexdigest()[:16]


class Writer:
    """Upsert records, skipping unchanged ones (content hash kept in ext._hash)."""

    def __init__(self, conn):
        self.conn = conn
        self.cache: dict[str, dict] = {}
        self.stats = Counter()

    def existing(self, table: str) -> dict:
        if table not in self.cache:
            rows = kb.one(self.conn, f"SELECT id, ext._hash AS h FROM {table}")
            self.cache[table] = {kb.rid_str(r["id"]): r.get("h") for r in rows}
        return self.cache[table]

    def put(self, rid, data: dict):
        table = rid.table_name
        data = dict(data)
        h = content_hash(data)
        ext = dict(data.pop("ext", {}) or {})
        ext["_hash"] = h
        data["ext"] = ext
        ex = self.existing(table)
        key = kb.rid_str(rid)
        if ex.get(key) == h:
            self.stats[f"{table}:unchanged"] += 1
            return
        # MERGE, not CONTENT: records such as actors are shared with the curated entity registry (load.py),
        # so an import must never wipe fields it does not own
        kb.run(self.conn, "UPSERT $id MERGE $data", {"id": rid, "data": data})
        self.stats[f"{table}:{'updated' if key in ex else 'created'}"] += 1
        ex[key] = h


# --- vocab ----------------------------------------------------------------------------------------

def load_vocab(w: Writer):
    for domain, slugs in yaml.safe_load((VOCAB / "kinds.yaml").read_text()).items():
        for s in slugs:
            w.put(kb.rid("kind", [domain, s]), {"labels": labels(s.replace("_", " ").capitalize())})
    eras = yaml.safe_load((VOCAB / "eras.yaml").read_text())
    for e in sorted(eras, key=lambda e: "parent" in e):          # parents first
        t = {"from": d(e["from"]), "precision": e["precision"]}
        if e.get("to"):
            t["to"] = d(e["to"])
        rec = {"code": e["code"], "labels": labels(e["en"]), "time": t, "order": e["order"], "summary": e["summary"]}
        if e.get("parent"):
            rec["parent"] = kb.rid("era", e["parent"])
        w.put(kb.rid("era", e["id"]), rec)
    for t in yaml.safe_load((VOCAB / "topics.yaml").read_text()):
        w.put(kb.rid("topic", t["id"]), {"code": t["code"], "labels": labels(t["en"]), "file": t["file"]})
    units = yaml.safe_load((VOCAB / "units.yaml").read_text())
    for u in sorted(units, key=lambda u: "base" in u):
        rec = {"symbol": u["symbol"], "labels": labels(u["symbol"] or u["id"]), "dimension": u["dimension"]}
        if u.get("base"):
            rec["base"], rec["to_base"] = kb.rid("unit", u["base"]), float(u["to_base"])
        w.put(kb.rid("unit", u["id"]), rec)


def era_rid(code: str):
    return kb.rid("era", code.split("-")[0])


def topic_rid(code: str):
    """'03', '03-fires-air', 'c02-snake-island-moskva' -> topic record id."""
    m = re.match(r"((?:\d\d|[cv]\d\d))(?:-|$)", str(code))
    return kb.rid("topic", kb.topic_key(m[1])) if m else None


# --- sources & actors -------------------------------------------------------------------------------

def canon_publisher(p: str) -> str:
    p = re.sub(r"\s*\(.*?\)\s*", " ", str(p)).strip()
    p = re.split(r"\s+[,;]\s+", p)[0]
    p = re.sub(r"\s+", " ", p).strip()
    return PUBLISHER_ALIASES.get(p, p)


def load_sources(w: Writer, paths: list[Path] | None = None) -> dict[str, str]:
    """Load source entries (research/sources.yaml format) from `paths` (default: the initial research/sources.yaml).
    Returns legacy id -> canonical source id (aliases included)."""
    merged = []
    for pth in paths or [kb.RESEARCH / "sources.yaml"]:
        merged += yaml.safe_load(pth.read_text()) or []
    pubs: dict[str, dict] = {}
    people: dict[str, str] = {}
    pub_types: dict[str, Counter] = defaultdict(Counter)
    pub_origin: dict[str, Counter] = defaultdict(Counter)
    for s in merged:
        c = canon_publisher(s["publisher"])
        pub_types[c][s["type"]] += 1
        pub_origin[c][s["origin"]] += 1
        pubs.setdefault(c, {"raw": set()})["raw"].add(str(s["publisher"]))
        s.setdefault("_input", "research/sources.yaml")
        for a in s.get("authors") or []:
            people[kb.slug(a)] = a
    known = {kb.rid_str(r) for r in kb.one(w.conn, "SELECT VALUE id FROM actor")}
    for name, info in pubs.items():
        if kb.rid_str(kb.rid("actor", kb.slug(name))) in known and paths:
            continue                                   # existing actors belong to the curated registry; don't re-guess them
        kind = ACTOR_KIND_FOR_SOURCE.get(pub_types[name].most_common(1)[0][0], "organisation")
        side = SIDE_FOR_ORIGIN[pub_origin[name].most_common(1)[0][0]]
        w.put(kb.rid("actor", kb.slug(name)), {
            "labels": labels(name), "aliases": sorted(info["raw"] - {name}),
            "kind": kb.rid("kind", ["actor", kind]), "side": side, "prov": prov("research/sources.yaml")})
    for sl, name in people.items():
        if kb.slug(name) in {kb.slug(p) for p in pubs}:
            continue                                            # an "author" that is really the organisation
        if kb.rid_str(kb.rid("actor", sl)) in known and paths:
            continue
        w.put(kb.rid("actor", sl), {"labels": labels(name), "kind": kb.rid("kind", ["actor", "person"]),
                                    "prov": prov("research/sources.yaml")})

    legacy = {}
    for s in merged:
        pub, raw = partial_date(s.get("published"))
        rec = {
            "title": s["title"], "url": s["url"], "alt_urls": s.get("alt_urls") or [],
            "publisher": kb.rid("actor", kb.slug(canon_publisher(s["publisher"]))),
            "authors": [kb.rid("actor", kb.slug(a)) for a in (s.get("authors") or [])],
            "type": kb.rid("kind", ["source", SOURCE_KIND.get(s["type"], s["type"])]),
            "origin": s["origin"], "reliability": s["reliability"], "verified": s["verified"],
            "eras": [era_rid(e) for e in s.get("eras") or []],
            "topics": [t for t in (topic_rid(x) for x in s.get("found_in") or []) if t],
            "found_in": [t for t in (topic_rid(x) for x in s.get("found_in") or []) if t],
            "legacy_ids": sorted({s["id"], *(s.get("aliases") or [])}),
            "notes": s.get("notes"), "prov": prov(s.get("_input", "research/sources.yaml")),
            "ext": {"publisher_raw": str(s["publisher"]), "topic_tags": s.get("topics") or []},
        }
        if pub:
            rec["published"] = pub
        if raw and raw != "unknown":
            rec["published_raw"] = raw
        if s.get("accessed"):
            rec["accessed"] = d(s["accessed"])
        w.put(kb.rid("source", s["id"]), {k: v for k, v in rec.items() if v is not None})
        for lid in rec["legacy_ids"]:
            legacy[lid] = s["id"]
    return legacy


# --- glossary ---------------------------------------------------------------------------------------

def load_terms(w: Writer):
    path = kb.RESEARCH / "glossary.md"
    order = 0
    for b in mdparse.blocks(path):
        if b.role != "table":
            continue
        group = b.section
        header, rows = mdparse.table_rows(b)
        for cells in rows:
            if len(cells) < 3:
                continue
            display, gloss, topics_raw = cells[0], cells[1], cells[2]
            m = re.match(r"\*\*(.+?)\*\*", display)
            term = (m[1] if m else display).strip()
            cyr = re.findall(r"[Ѐ-ӿ][Ѐ-ӿ\s\-’'ʼ]*[Ѐ-ӿ]", display)
            lang = "ru" if group.lower().startswith("russian") else ("uk" if cyr or "ukrainian" in group.lower() else "en")
            topics = [] if topics_raw.strip() == "all" else [t for t in (topic_rid(x) for x in re.findall(r"\d\d", topics_raw)) if t]
            order += 1
            w.put(kb.rid("term", f"g{order:03d}_{kb.slug(term, 40)}"), {
                "term": term, "display": display, "gloss": gloss, "cyrillic": ", ".join(cyr) or None,
                "lang": lang, "group": group, "order": order, "verified": "*(unverified)*" not in gloss,
                "topics": topics, "topics_raw": topics_raw, "prov": prov("research/glossary.md")})


# --- passages, questions, design notes ------------------------------------------------------------

MEDIA_KIND = [("satellite", "satellite_image"), ("video", "video"), ("footage", "video"), ("clip", "video"),
              ("photo", "image"), ("image", "image"), ("map", "map"), ("dataset", "dataset"), ("audio", "audio"),
              ("radio", "audio"), ("recording", "audio"), ("document", "document"), ("report", "document")]


_MEDIA_SEEN: set[str] = set()


def load_media_table(w: Writer, b, rel: str, legacy: dict[str, str]):
    """Rows of a dossier's Media table -> media records (links + metadata only)."""
    header, rows = mdparse.table_rows(b)
    cols = [h.lower() for h in header]
    for cells in rows:
        row = dict(zip(cols, cells))
        url = next((re.search(r"https?://[^\s)>|]+", c)[0] for c in cells if re.search(r"https?://", c)), None)
        if not url:
            continue
        kind_txt = (row.get("kind") or "").lower()
        kind = next((k for word, k in MEDIA_KIND if word in kind_txt), "web_page")
        rec = {"url": url, "kind": kb.rid("kind", ["media", kind]), "title": row.get("what") or None,
               "description": " · ".join(x for x in (row.get("what"), row.get("kind"), row.get("publisher"), row.get("date")) if x),
               "licence": row.get("licence or terms if known") or row.get("licence") or None,
               "prov": prov(rel), "ext": {"document": rel, "date_raw": row.get("date"), "publisher_raw": row.get("publisher")}}
        sids = mdparse.src_ids(" ".join(cells))
        if sids and sids[0] in legacy:
            rec["found_via"] = kb.rid("source", legacy[sids[0]])
        mid = kb.rid("media", hashlib.sha1(url.encode()).hexdigest()[:16])
        if kb.rid_str(mid) in _MEDIA_SEEN:
            continue                                   # the same link listed by several dossiers: first one wins
        _MEDIA_SEEN.add(kb.rid_str(mid))
        w.put(mid, {k: v for k, v in rec.items() if v is not None})


def load_documents(w: Writer, legacy: dict[str, str]):
    unknown = Counter()
    for f in kb.documents():
        code = kb.doc_code(f)
        topic = kb.rid("topic", kb.topic_key(code))
        rel = str(f.relative_to(kb.REPO))
        for b in mdparse.blocks(f):
            for sid in mdparse.src_ids(b.text):
                if sid not in legacy:
                    unknown[sid] += 1
            w.put(kb.rid("passage", f"{kb.topic_key(code)}_{b.order:04d}"), {
                "key": f"{code}/{b.order:04d}", "topic": topic, "section": b.section or "(preamble)",
                "level": b.level, "order": b.order, "role": b.role, "markdown": b.text,
                "prov": prov(rel), "ext": {"tight": b.tight}})
            table = mdparse.meta_section(b.top)
            if b.role == "table" and table == "media":
                load_media_table(w, b, rel, legacy)
            if b.role == "list" and table in ("question", "design_note"):
                for i, item in enumerate(mdparse.list_items(b)):
                    key = f"{code}/{b.order:04d}/{i:02d}"
                    anchor = {"file": rel, "section": b.section, "order": b.order, "local_id": key}
                    w.put(kb.rid(table, f"{kb.topic_key(code)}_{b.order:04d}_{i:02d}"), {
                        "key": key, "text": mdparse.strip_bullet(item), "topics": [topic],
                        "eras": [era_rid(e) for e in sorted(set(re.findall(r"\be[1-9][ab]?\b", item)))],
                        "anchor": anchor, "prov": prov(rel)})
    if unknown:
        print("WARNING unresolved [src:] ids in documents:", dict(unknown))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", choices=["vocab", "sources", "terms", "documents"], action="append")
    args = ap.parse_args()
    parts = args.only or ["vocab", "sources", "terms", "documents"]
    w = Writer(kb.connect())
    legacy = {}
    if "vocab" in parts:
        load_vocab(w)
    if "sources" in parts or "documents" in parts:
        legacy = load_sources(w) if "sources" in parts else {
            lid: r["id"].id for r in kb.one(w.conn, "SELECT id, legacy_ids FROM source") for lid in r["legacy_ids"]}
    if "terms" in parts:
        load_terms(w)
    if "documents" in parts:
        load_documents(w, legacy)
    for k, v in sorted(w.stats.items()):
        print(f"{k:32} {v}")


if __name__ == "__main__":
    main()
