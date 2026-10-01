#!/usr/bin/env python3
"""Regenerate the research documents from the knowledge base.

  render.py --check   compare what would be generated with research/ (the round-trip gate); no writes
  render.py --write   write research/0*.md, glossary.md, contradictions.md, sources.yaml and sources/NN-*.yaml

Documents come from `passage` records (verbatim prose and tables, ordered), the glossary from `term`,
and the source indexes from `source`. README.md stays hand-written.
"""
from __future__ import annotations

import argparse
import datetime as dt
import difflib
import re
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402

GENERATED = "<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->"
YAML_HEADER = ("# Source index for bavovniatko research, generated from the knowledge base by db/tools/render.py.\n"
               "# `aliases` lists other ids used for the same source; `found_in` lists topic files.\n")
TYPE_OUT = {"think_tank": "think-tank", "osint_dataset": "osint-dataset"}


def topics(conn) -> dict[str, dict]:
    return {r["id"].id: r for r in kb.one(conn, "SELECT id, code, file FROM topic")}


def eras(conn) -> dict[str, str]:
    return {r["id"].id: r["code"] for r in kb.one(conn, "SELECT id, code FROM era")}


def render_documents(conn, header: bool) -> dict[Path, str]:
    out = {}
    for tid, t in sorted(topics(conn).items()):
        rows = kb.one(conn, "SELECT order, markdown, ext.tight AS tight FROM passage WHERE topic = $t ORDER BY order",
                      {"t": kb.rid("topic", tid)})
        parts = []
        for i, r in enumerate(rows):
            parts.append(r["markdown"])
            if i < len(rows) - 1:
                parts.append("\n" if r.get("tight") else "\n\n")
        text = "".join(parts) + "\n"
        if header:
            text = GENERATED + "\n\n" + text
        out[kb.REPO / t["file"]] = text
    return out


def render_glossary(conn, header: bool) -> str:
    rows = kb.one(conn, "SELECT display, gloss, topics_raw, group, order FROM term ORDER BY order")
    groups: dict[str, list] = {}
    for r in rows:
        groups.setdefault(r["group"], []).append(r)
    parts = [(kb.DB_DIR / "templates" / "glossary_header.md").read_text().rstrip("\n")]
    for g, items in groups.items():
        parts.append(f"## {g}")
        parts.append("\n".join(["| term | meaning | topics |", "|---|---|---|"] +
                               [f"| {r['display']} | {r['gloss']} | {r['topics_raw']} |" for r in items]))
    text = "\n\n".join(parts) + "\n"
    return (GENERATED + "\n\n" + text) if header else text


def render_contradictions(conn) -> str:
    rows = kb.one(conn, """SELECT in AS a, out AS b, strength, rationale, resolution, state,
        in.text AS at, out.text AS bt, in.key AS ak, out.key AS bk,
        in->asserted_by->actor.labels.en AS aw, out->asserted_by->actor.labels.en AS bw,
        in->cites->source.id AS asrc, out->cites->source.id AS bsrc,
        in.topics[0].code AS topic FROM contradicts ORDER BY topic, strength DESC""")
    tops = {r["id"].id: r for r in kb.one(conn, "SELECT id, code, labels.en AS name FROM topic")}
    # collapse duplicate clusters: follow `duplicates` (copy -> canonical) so one dispute is listed once
    canon = {kb.rid_str(r["in"]): kb.rid_str(r["out"]) for r in kb.one(conn, "SELECT in, out FROM duplicates")}

    def root(c: str) -> str:
        seen = set()
        while c in canon and c not in seen:
            seen.add(c)
            c = canon[c]
        return c

    uniq: dict[tuple, dict] = {}
    for r in rows:
        k = tuple(sorted((root(kb.rid_str(r["a"])), root(kb.rid_str(r["b"])))))
        if k[0] == k[1]:
            continue
        if k not in uniq or r["strength"] > uniq[k]["strength"]:
            uniq[k] = {**r, "copies": uniq.get(k, {}).get("copies", 0) + 1}
        else:
            uniq[k]["copies"] += 1
    rows = list(uniq.values())
    by_topic: dict[str, list] = {}
    for r in rows:
        by_topic.setdefault(r.get("topic") or "?", []).append(r)
    out = [GENERATED, "", "# Contradictions", "",
           "Pairs of claims that cannot both be true as stated, judged from rule-based candidates "
           "(db/tools/relate.py) and stored as `contradicts` edges. *Explained* means the difference has a "
           "stated cause (method, scope, date); *open* means it is unresolved. Claim ids resolve in the "
           "knowledge base; source ids in [sources.yaml](sources.yaml).", "",
           f"{len(rows)} distinct contradictions (duplicate claims collapsed): {sum(1 for r in rows if r.get('state') == 'explained')} explained, "
           f"{sum(1 for r in rows if r.get('state') != 'explained')} open.", ""]
    fmt_src = lambda xs: ", ".join(f"`{x.id}`" for x in (xs or [])[:3])
    fmt_who = lambda xs: "; ".join(xs or []) or "—"
    for code, items in sorted(by_topic.items()):
        name = next((t["name"] for t in tops.values() if t["code"] == code), code)
        out += [f"## {code} {name}", ""]
        for r in items:
            dup = f" · {r['copies']} judged pairs" if r.get("copies", 1) > 1 else ""
            out += [f"- **{r.get('state', 'open')}** · strength {r['strength']:.1f}{dup} — {r['rationale']}",
                    f"  - `{r['ak']}` {r['at']} *(claimant: {fmt_who(r.get('aw'))}; {fmt_src(r.get('asrc'))})*",
                    f"  - `{r['bk']}` {r['bt']} *(claimant: {fmt_who(r.get('bw'))}; {fmt_src(r.get('bsrc'))})*"]
            if r.get("resolution"):
                out.append(f"  - *Resolution:* {r['resolution']}")
        out.append("")
    return "\n".join(out)


def _date(v):
    return v.strftime("%Y-%m-%d") if isinstance(v, dt.datetime) else v


def source_entries(conn) -> list[dict]:
    era_code = eras(conn)
    tops = topics(conn)
    stem = {tid: Path(t["file"]).stem for tid, t in tops.items()}
    rows = kb.one(conn, """SELECT *, publisher.labels.en AS pub_name, authors.labels.en AS author_names
                           FROM source ORDER BY id""")
    out = []
    for r in rows:
        sid = r["id"].id
        e = {"id": sid}
        aliases = [a for a in r.get("legacy_ids", []) if a != sid]
        if aliases:
            e["aliases"] = aliases
        e["title"] = r["title"]
        e["url"] = r["url"]
        if r.get("alt_urls"):
            e["alt_urls"] = list(r["alt_urls"])
        e["publisher"] = r.get("ext", {}).get("publisher_raw") or r.get("pub_name")
        e["authors"] = list(r.get("author_names") or [])
        e["published"] = r.get("published_raw") or "unknown"
        e["accessed"] = _date(r.get("accessed"))
        e["type"] = TYPE_OUT.get(r["type"].id[1], r["type"].id[1])
        for k in ("origin", "reliability", "verified"):
            e[k] = r[k]
        e["eras"] = [era_code[x.id] for x in r.get("eras", [])]
        e["topics"] = list(r.get("ext", {}).get("topic_tags") or [])
        e["found_in"] = [stem[x.id] for x in r.get("found_in", [])]
        e["notes"] = r.get("notes")
        out.append(e)
    return out


def render_sources(conn) -> dict[Path, str]:
    entries = source_entries(conn)
    dump = lambda xs: yaml.safe_dump(xs, sort_keys=False, allow_unicode=True, width=120)
    out = {kb.RESEARCH / "sources.yaml": YAML_HEADER + dump(entries)}
    for tid, t in sorted(topics(conn).items()):
        st = Path(t["file"]).stem
        mine = [{k: v for k, v in e.items() if k != "found_in"} for e in entries if st in e["found_in"]]
        out[kb.RESEARCH / "sources" / f"{st}.yaml"] = (
            f"# Sources cited in research/{st}.md, generated from the knowledge base by db/tools/render.py.\n" + dump(mine))
    return out


def normalise_md(text: str) -> str:
    text = text.replace(GENERATED + "\n\n", "")
    text = re.sub(r"\n[ \t]+\n", "\n\n", text)
    return re.sub(r"\n{3,}", "\n\n", text.strip("\n")) + "\n"


def check(conn) -> int:
    bad = 0
    for path, text in {**render_documents(conn, False), kb.RESEARCH / "glossary.md": render_glossary(conn, False)}.items():
        cur = normalise_md(path.read_text())
        ok = normalise_md(text) == cur
        bad += not ok
        print(f"{'ok  ' if ok else 'DIFF'} {path.relative_to(kb.REPO)}")
        if not ok:
            sys.stdout.writelines(list(difflib.unified_diff(cur.splitlines(1), normalise_md(text).splitlines(1), "current", "generated"))[:30])
    # sources: semantic comparison (ids, urls, titles); per-topic files by id set
    gen = {e["id"]: e for e in source_entries(conn)}
    cur = {e["id"]: e for e in yaml.safe_load((kb.RESEARCH / "sources.yaml").read_text())}
    fields = ["title", "url", "publisher", "authors", "type", "origin", "reliability", "verified", "notes", "found_in"]
    diffs = [(i, f) for i in cur for f in fields if i in gen and cur[i].get(f) != gen[i].get(f)]
    missing = set(cur) ^ set(gen)
    ok = not diffs and not missing
    bad += not ok
    print(f"{'ok  ' if ok else 'DIFF'} research/sources.yaml ({len(gen)} sources; {len(diffs)} field diffs; {len(missing)} id diffs)")
    for i, f in diffs[:10]:
        print(f"     {i}.{f}: {cur[i].get(f)!r} != {gen[i].get(f)!r}")
    return 1 if bad else 0


def main():
    ap = argparse.ArgumentParser()
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--check", action="store_true")
    g.add_argument("--write", action="store_true")
    args = ap.parse_args()
    conn = kb.connect()
    if args.check:
        sys.exit(check(conn))
    files = {**render_documents(conn, True), kb.RESEARCH / "glossary.md": render_glossary(conn, True), **render_sources(conn),
             kb.RESEARCH / "contradictions.md": render_contradictions(conn)}
    for path, text in files.items():
        if not path.exists() or path.read_text() != text:
            path.write_text(text)
            print("wrote", path.relative_to(kb.REPO))


if __name__ == "__main__":
    main()
