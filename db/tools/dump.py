#!/usr/bin/env python3
"""Deterministic, engine-independent JSONL dump of the knowledge base (committed to git).

  dump.py export [--dir db/dump]         one sorted JSONL file per table (computed fields omitted)
  dump.py restore --ns N --db D [--dir]  apply migrations to an empty database and load a dump into it
  dump.py verify                         export -> restore into a scratch db -> export -> byte-compare

Typed values are tagged so they survive JSON: {"$rid": [table, key]}, {"$dt": iso}, {"$geo": GeoJSON},
{"$dec": "…"}. Together with db/schema/ this rebuilds the database exactly.
"""
from __future__ import annotations

import argparse
import datetime as dt
import decimal
import filecmp
import json
import re
import shutil
import sys
import tempfile
import uuid
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402
from surrealdb import RecordID  # noqa: E402
from surrealdb.data.types.geometry import Geometry, GeometryCollection  # noqa: E402

DUMP = kb.DB_DIR / "dump"
SKIP_TABLES = {"migration"}
# load order: vocabularies, entities, claims & meta, audit, then relations (both ends must exist)
ORDER = ["kind", "era", "topic", "unit", "metric", "actor", "place", "event", "system", "work", "source", "term",
         "media", "snapshot", "claim", "observation", "passage", "question", "design_note", "revision"]


def geo_json(g) -> dict:
    name = type(g).__name__
    if name == "GeometryCollection":
        return {"type": "GeometryCollection", "geometries": [geo_json(x) for x in g.geometries]}
    types = {"GeometryPoint": "Point", "GeometryLine": "LineString", "GeometryPolygon": "Polygon",
             "GeometryMultiPoint": "MultiPoint", "GeometryMultiLine": "MultiLineString",
             "GeometryMultiPolygon": "MultiPolygon"}
    return {"type": types[name], "coordinates": g.get_coordinates()}


def enc(v):
    if isinstance(v, RecordID):
        return {"$rid": [v.table_name, enc(v.id)]}
    if isinstance(v, dt.datetime):
        return {"$dt": v.astimezone(dt.timezone.utc).isoformat().replace("+00:00", "Z")}
    if isinstance(v, (Geometry, GeometryCollection)):
        return {"$geo": geo_json(v)}
    if isinstance(v, decimal.Decimal):
        return {"$dec": str(v)}
    if isinstance(v, dict):
        return {k: enc(x) for k, x in v.items() if x is not None}
    if isinstance(v, (list, tuple)):
        return [enc(x) for x in v]
    if hasattr(v, "to_string") or type(v).__name__ in ("Datetime", "Duration"):
        return {"$dt": str(v)} if type(v).__name__ == "Datetime" else str(v)
    return v


def dec(v):
    if isinstance(v, dict):
        if "$rid" in v:
            t, k = v["$rid"]
            return RecordID(t, dec(k))
        if "$dt" in v:
            return dt.datetime.fromisoformat(v["$dt"].replace("Z", "+00:00"))
        if "$geo" in v:
            return kb.geom(v["$geo"])
        if "$dec" in v:
            return decimal.Decimal(v["$dec"])
        return {k: dec(x) for k, x in v.items()}
    if isinstance(v, list):
        return [dec(x) for x in v]
    return v


def schema_info(conn):
    info = kb.one(conn, "INFO FOR DB")
    tables = sorted(t for t in info["tables"] if t not in SKIP_TABLES)
    computed, relations = {}, set()
    for t in tables:
        if re.search(r"TYPE RELATION", info["tables"][t]):
            relations.add(t)
        fields = kb.one(conn, f"INFO FOR TABLE {t}")["fields"]
        computed[t] = {f for f, d in fields.items() if " COMPUTED " in d}
    return tables, computed, relations


def export(conn, out: Path) -> dict[str, int]:
    tables, computed, _ = schema_info(conn)
    out.mkdir(parents=True, exist_ok=True)
    counts = {}
    for t in tables:
        rows = kb.one(conn, f"SELECT * FROM {t}")
        lines = []
        for r in rows:
            r = {k: v for k, v in r.items() if k not in computed[t]}
            lines.append(json.dumps(enc(r), ensure_ascii=False, sort_keys=True, separators=(",", ":")))
        lines.sort()
        path = out / f"{t}.jsonl"
        if lines:
            path.write_text("\n".join(lines) + "\n")
        elif path.exists():
            path.unlink()
        counts[t] = len(lines)
    for stale in set(p.stem for p in out.glob("*.jsonl")) - set(tables):
        (out / f"{stale}.jsonl").unlink()
    return counts


def restore(conn, src: Path):
    _, _, relations = schema_info(conn)
    files = {p.stem: p for p in src.glob("*.jsonl")}
    order = [t for t in ORDER if t in files] + sorted(t for t in files if t not in ORDER and t not in relations) \
        + sorted(t for t in files if t in relations)
    for t in order:
        rows = [dec(json.loads(l)) for l in files[t].read_text().splitlines() if l.strip()]
        for i in range(0, len(rows), 200):
            chunk = rows[i:i + 200]
            if t in relations:
                kb.run(conn, f"INSERT RELATION INTO {t} $rows", {"rows": chunk})
            else:   # CREATE (not INSERT) so explicitly supplied VALUE fields such as updated_at are kept
                kb.run(conn, "FOR $r IN $rows { CREATE $r.id CONTENT $r; }", {"rows": chunk})
        print(f"restored {t:14} {len(rows)}")


def fresh_db(ns: str, db: str):
    root = kb.connect()
    kb.run(root, f"DEFINE NAMESPACE IF NOT EXISTS {ns}; USE NS {ns}; DEFINE DATABASE {db} STRICT;")
    conn = kb.connect(ns, db)
    for f in sorted((kb.DB_DIR / "schema").glob("[0-9][0-9][0-9][0-9]_*.surql")):
        kb.run(conn, f.read_text())
    return root, conn


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    e = sub.add_parser("export"); e.add_argument("--dir", default=str(DUMP))
    r = sub.add_parser("restore"); r.add_argument("--ns", required=True); r.add_argument("--db", required=True)
    r.add_argument("--dir", default=str(DUMP))
    sub.add_parser("verify")
    args = ap.parse_args()
    if args.cmd == "export":
        counts = export(kb.connect(), Path(args.dir))
        print(" ".join(f"{t}={n}" for t, n in counts.items() if n))
    elif args.cmd == "restore":
        _, conn = fresh_db(args.ns, args.db)
        restore(conn, Path(args.dir))
    elif args.cmd == "verify":
        tmp = Path(tempfile.mkdtemp(prefix="kbdump_"))
        a, b = tmp / "a", tmp / "b"
        export(kb.connect(), a)
        name = f"verify_{uuid.uuid4().hex[:8]}"
        root, conn = fresh_db("test", name)
        try:
            restore(conn, a)
            export(conn, b)
        finally:
            kb.run(root, f"USE NS test; REMOVE DATABASE {name};")
        cmp = filecmp.dircmp(a, b)
        bad = cmp.diff_files + cmp.left_only + cmp.right_only
        print("dump round-trip:", "IDENTICAL" if not bad else f"DIFFERS in {bad}")
        shutil.rmtree(tmp)
        sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
