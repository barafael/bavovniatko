#!/usr/bin/env python3
"""Run a read-only SurrealQL query against the knowledge base and print JSON (for agents and humans).

    db/.venv/bin/python db/tools/q.py "SELECT id, text FROM claim WHERE text @@ 'glide bomb' LIMIT 5"
Statements that could write (CREATE, UPDATE, UPSERT, DELETE, RELATE, INSERT, DEFINE, REMOVE) are refused.
"""
import json
import re
import sys

import kb

q = " ".join(sys.argv[1:]) or sys.stdin.read()
if re.search(r"\b(CREATE|UPDATE|UPSERT|DELETE|RELATE|INSERT|DEFINE|REMOVE|ALTER|KILL|LIVE|BEGIN|COMMIT)\b", q, re.IGNORECASE):
    sys.exit("q.py is read-only; use the loaders to write")


print(json.dumps(kb.plain(kb.one(kb.connect(), q)), ensure_ascii=False, indent=1))
