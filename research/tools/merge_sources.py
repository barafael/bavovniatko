#!/usr/bin/env python3
"""Merge research/sources/*.yaml into research/sources.yaml and check citations.

Dedupes by normalized URL (union of eras/topics, other ids kept as aliases),
reports id collisions, unresolved [src:id] citations and coverage tables.
"""
import collections
import glob
import os
import re
import sys

import yaml

ROOT = sys.argv[1] if len(sys.argv) > 1 else "/home/rafael/bavovniatko/research"
ERAS = [
    "e1-invasion", "e2-donbas-artillery", "e3-counteroffensives-22", "e4-bakhmut",
    "e5-counteroffensive-23", "e6-avdiivka-attrition", "e7-kursk-pokrovsk",
    "e8-drone-kill-zone", "e9-counteroffensive-26",
]


def era_key(era):
    return (ERAS.index(era), "") if era in ERAS else (len(ERAS), era)


def norm_title(t):
    return re.sub(r"[^a-z0-9]+", " ", str(t or "").lower()).strip()


def norm_url(u):
    u = (u or "").strip().lower()
    u = re.sub(r"^https?://(www\.)?", "", u)
    u = u.split("#")[0].rstrip("/")
    return u


merged = {}          # norm_url -> entry
by_id = {}           # id -> norm_url
by_title = {}        # normalized title -> norm_url
problems = []
per_file = {}

for path in sorted(glob.glob(os.path.join(ROOT, "sources", "*.yaml"))):
    stem = os.path.splitext(os.path.basename(path))[0]
    try:
        data = yaml.safe_load(open(path)) or []
    except yaml.YAMLError as e:
        problems.append(f"YAML error in {stem}: {e}")
        continue
    per_file[stem] = data
    for e in data:
        if not isinstance(e, dict) or "id" not in e:
            problems.append(f"{stem}: malformed entry {e!r:.80}")
            continue
        e = dict(e)
        e["eras"] = [str(x) for x in (e.get("eras") or [])]
        e["topics"] = [str(x) for x in (e.get("topics") or [])]
        for era in e["eras"]:
            if era not in ERAS:
                problems.append(f"{stem}: {e['id']} has unknown era {era}")
        key = norm_url(e.get("url"))
        if not key:
            problems.append(f"{stem}: {e['id']} has no url")
            key = "nourl:" + e["id"]
        tkey = norm_title(e.get("title"))
        if key not in merged:
            # same source under another URL (landing page vs PDF): same id or same long title
            alt = by_id.get(e["id"]) or (by_title.get(tkey) if len(tkey) > 25 else None)
            if alt:
                if e.get("url") not in merged[alt].setdefault("alt_urls", []):
                    merged[alt]["alt_urls"].append(e.get("url"))
                key = alt
        if key in merged:
            m = merged[key]
            m["eras"] = sorted(set(m["eras"]) | set(e["eras"]), key=era_key)
            m["topics"] = sorted(set(m["topics"]) | set(e["topics"]))
            m["found_in"] = sorted(set(m["found_in"]) | {stem})
            if e["id"] != m["id"] and e["id"] not in m.setdefault("aliases", []):
                m["aliases"].append(e["id"])
            if m.get("verified") != "fetched" and e.get("verified") == "fetched":
                m["verified"] = "fetched"
        else:
            e["found_in"] = [stem]
            merged[key] = e
        by_id[e["id"]] = key
        by_title.setdefault(tkey, key)

entries = sorted(merged.values(), key=lambda x: x["id"])
all_ids = {}
for m in entries:
    all_ids[m["id"]] = m
    for a in m.get("aliases", []):
        all_ids[a] = m

# citation check
cited = collections.Counter()
for md in sorted(glob.glob(os.path.join(ROOT, "[0-9][0-9]-*.md"))):
    stem = os.path.splitext(os.path.basename(md))[0]
    text = open(md).read()
    for cid in re.findall(r"\[src:([^\]]+)\]", text):
        for c in re.split(r"[,;]\s*(?:src:)?", cid):
            c = c.strip()
            if not c or c == "<id>":
                continue
            cited[c] += 1
            if c not in all_ids:
                problems.append(f"{stem}: unresolved citation [src:{c}]")

uncited = [m["id"] for m in entries if m["id"] not in cited and not any(a in cited for a in m.get("aliases", []))]

key_order = ["id", "aliases", "title", "url", "alt_urls", "publisher", "authors", "published", "accessed",
             "type", "origin", "reliability", "verified", "eras", "topics", "found_in", "notes"]
out = []
for m in entries:
    o = {k: m[k] for k in key_order if k in m}
    o.update({k: v for k, v in m.items() if k not in o})
    out.append(o)

with open(os.path.join(ROOT, "sources.yaml"), "w") as f:
    f.write("# Merged source index for bavovniatko research. Generated from research/sources/*.yaml.\n")
    f.write("# Deduplicated by URL; `aliases` lists other ids used for the same source; `found_in` lists topic files.\n")
    yaml.safe_dump(out, f, sort_keys=False, allow_unicode=True, width=120)

# report
print(f"files: {len(per_file)}  raw entries: {sum(len(v) for v in per_file.values())}  merged: {len(entries)}")
print("\nper topic file (raw / fetched / search-only):")
for stem, data in per_file.items():
    v = collections.Counter(str(e.get("verified")) for e in data if isinstance(e, dict))
    print(f"  {stem:32} {len(data):3}  {v.get('fetched',0):3}  {v.get('search-only',0):3}")
print("\nera coverage (merged sources; topic files touching the era):")
for era in ERAS:
    n = sum(1 for m in entries if era in m["eras"])
    files = sorted({f for m in entries if era in m["eras"] for f in m["found_in"]})
    print(f"  {era:26} {n:3} sources, {len(files)} topics: {', '.join(s[:2] for s in files)}")
print("\norigin:", dict(collections.Counter(m.get("origin") for m in entries)))
print("reliability:", dict(collections.Counter(m.get("reliability") for m in entries)))
print("type:", dict(collections.Counter(m.get("type") for m in entries)))
print(f"\nuncited merged entries: {len(uncited)}")
print(f"\nproblems: {len(problems)}")
for p in problems:
    print("  -", p)
