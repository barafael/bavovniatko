#!/usr/bin/env python3
"""Run the integrity checks in db/checks/*.surql. Each file is one query returning violating record ids,
with a `-- severity: error|warn` header. Exit code 1 if any error-level check has violations.

    db/.venv/bin/python db/tools/check.py [-v]
"""
import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("-v", "--verbose", action="store_true", help="list up to 10 offending ids per check")
    args = ap.parse_args()
    conn = kb.connect()
    errors = 0
    for f in sorted((kb.DB_DIR / "checks").glob("*.surql")):
        text = f.read_text()
        sev = (re.search(r"--\s*severity:\s*(\w+)", text) or [None, "error"])[1]
        rows = kb.one(conn, text) or []
        n = len(rows)
        mark = "ok  " if n == 0 else ("ERR " if sev == "error" else "warn")
        print(f"{mark} {f.stem:30} {n}")
        if n and args.verbose:
            print("       " + ", ".join(kb.rid_str(r) if hasattr(r, "table_name") else str(r) for r in rows[:10]))
        if n and sev == "error":
            errors += 1
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
