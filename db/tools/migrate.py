#!/usr/bin/env python3
"""Apply db/schema/NNNN_*.surql migrations in order.

Each migration runs once; its sha256 is recorded in the `migration` table. A migration whose file
changed after it was applied is an error (write a new migration instead), unless --force-reapply
is given, which re-runs it (migrations are written with IF NOT EXISTS / OVERWRITE, so this is safe).

    db/.venv/bin/python db/tools/migrate.py            # apply pending
    db/.venv/bin/python db/tools/migrate.py --status   # list
"""
import argparse
import datetime as dt
import hashlib
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402

SCHEMA = kb.DB_DIR / "schema"
FIXES = kb.DB_DIR / "fixes"      # idempotent data corrections, applied after loading (migrate.py --fixes)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--status", action="store_true")
    ap.add_argument("--force-reapply", metavar="NAME", action="append", default=[])
    ap.add_argument("--fixes", action="store_true", help="apply db/fixes/NNNN_*.surql (data corrections) instead of schema")
    args = ap.parse_args()

    e = kb.env()
    root = kb.connect()
    kb.run(root, f"DEFINE NAMESPACE IF NOT EXISTS {e['SURREAL_NS']}; USE NS {e['SURREAL_NS']}; "
                 f"DEFINE DATABASE IF NOT EXISTS {e['SURREAL_DB']} STRICT;")
    conn = kb.connect()
    # the migration table is created by 0001; before that, nothing is applied
    try:
        applied = {r["id"].id: r for r in kb.one(conn, "SELECT * FROM migration")}
    except kb.KBError:
        applied = {}

    files = sorted((FIXES if args.fixes else SCHEMA).glob("[0-9][0-9][0-9][0-9]_*.surql"))
    for f in files:
        name, sha = ("fix_" if args.fixes else "") + f.stem, hashlib.sha256(f.read_bytes()).hexdigest()
        prev = applied.get(name)
        if args.status:
            state = "pending" if not prev else ("ok" if prev["sha256"] == sha else "CHANGED")
            print(f"{state:8} {name}")
            continue
        if prev and prev["sha256"] == sha and name not in args.force_reapply:
            continue
        if prev and name not in args.force_reapply:
            print(f"error: {name} changed after it was applied; add a new migration "
                  f"or pass --force-reapply {name}", file=sys.stderr)
            return 1
        kb.run(conn, f.read_text())
        kb.run(conn, "UPSERT type::record('migration', $n) SET sha256 = $s, applied_at = $t",
               {"n": name, "s": sha, "t": dt.datetime.now(dt.timezone.utc)})
        print(f"applied  {name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
