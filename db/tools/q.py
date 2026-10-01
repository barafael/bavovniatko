#!/usr/bin/env python3
"""Run a read-only SurrealQL query against the knowledge base and print JSON (for agents and humans).

    db/.venv/bin/python db/tools/q.py "SELECT id, text FROM claim WHERE text @@ 'glide bomb' LIMIT 5"
Statements that could write (CREATE, UPDATE, UPSERT, DELETE, RELATE, INSERT, DEFINE, REMOVE) are refused.
"""
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402
from dump import enc  # noqa: E402

q = " ".join(sys.argv[1:]) or sys.stdin.read()
if re.search(r"\b(CREATE|UPDATE|UPSERT|DELETE|RELATE|INSERT|DEFINE|REMOVE|ALTER|KILL|LIVE|BEGIN|COMMIT)\b", q, re.I):
    sys.exit("q.py is read-only; use the loaders to write")


def plain(v):
    v = enc(v)
    if isinstance(v, dict):
        if "$rid" in v:
            return kb.rid_str(kb.rid(v["$rid"][0], v["$rid"][1]))
        if "$dt" in v:
            return v["$dt"]
        return {k: plain(x) for k, x in v.items()}
    if isinstance(v, list):
        return [plain(x) for x in v]
    return v


print(json.dumps(plain(kb.one(kb.connect(), q)), ensure_ascii=False, indent=1))
