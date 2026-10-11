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

from surrealdb import RecordID

import kb

DUMP = kb.DB_DIR / "dump"
SKIP_TABLES = {"migration"}
# load order: vocabularies, entities, claims & meta, audit, then relations (both ends must exist)
ORDER = ["kind", "era", "topic", "unit", "metric", "actor", "place", "event", "system", "work", "source", "term",
         "media", "snapshot", "claim", "observation", "passage", "question", "design_note", "revision"]


def load_order(files, relations) -> list[str]:
    """Tables in load order: ORDER first, then the rest alphabetically, relations last (both ends must exist)."""
    return ([t for t in ORDER if t in files] + sorted(t for t in files if t not in ORDER and t not in relations)
            + sorted(t for t in files if t in relations))


def enc(v):
    """The dump's tagged JSON: record ids as {"$rid": [table, key]} and datetimes as {"$dt": iso}."""
    return kb.plain(v, rid=lambda r: {"$rid": [r.table_name, enc(r.id)]}, dt_=lambda d: {"$dt": kb.iso_z(d)})


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
    infos = kb.run(conn, "".join(f"INFO FOR TABLE {t};" for t in tables)) if tables else []
    for t, table_info in zip(tables, infos):
        if re.search(r"TYPE RELATION", info["tables"][t]):
            relations.add(t)
        computed[t] = {f for f, d in table_info["fields"].items() if " COMPUTED " in d}
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
    for stale in {p.stem for p in out.glob("*.jsonl")} - set(tables):
        (out / f"{stale}.jsonl").unlink()
    return counts


def restore(conn, src: Path):
    _, _, relations = schema_info(conn)
    files = {p.stem: p for p in src.glob("*.jsonl")}
    for t in load_order(files, relations):
        rows = [dec(r) for r in kb.jsonl(files[t])]
        for i in range(0, len(rows), 200):
            chunk = rows[i:i + 200]
            if t in relations:
                kb.run(conn, f"INSERT RELATION INTO {t} $rows", {"rows": chunk})
            else:   # CREATE (not INSERT) so explicitly supplied VALUE fields such as updated_at are kept
                kb.run(conn, "FOR $r IN $rows { CREATE $r.id CONTENT $r; }", {"rows": chunk})
        print(f"restored {t:14} {len(rows)}")


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
        _, conn = kb.fresh_db(args.ns, args.db)
        restore(conn, Path(args.dir))
    elif args.cmd == "verify":
        tmp = Path(tempfile.mkdtemp(prefix="kbdump_"))
        a, b = tmp / "a", tmp / "b"
        export(kb.connect(), a)
        name = f"verify_{uuid.uuid4().hex[:8]}"
        root, conn = kb.fresh_db("test", name)
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
