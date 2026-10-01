#!/usr/bin/env python3
"""Staging area for agent-extracted records (claims, counter links).

  stage.py worksheet NN [--out FILE]  print the topic file as numbered blocks (what agents read)
  stage.py append NN [FILE|-]         validate JSONL lines and append the valid ones to db/staging/claims/NN.jsonl
  stage.py check NN|all               schema + reference + coverage report for a topic's staging file
  stage.py schema                     write db/staging/claim.schema.json (derived from db/vocab)

Every line is one JSON object with "type": "claim" or "counter". See db/staging/EXTRACTION.md.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path

import jsonschema
import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402
import mdparse  # noqa: E402

VOCAB = kb.DB_DIR / "vocab"
STAGING = kb.DB_DIR / "staging"
CLAIMS = STAGING / "claims"

KINDS = yaml.safe_load((VOCAB / "kinds.yaml").read_text())
UNITS = [u["id"] for u in yaml.safe_load((VOCAB / "units.yaml").read_text())]
METRICS = yaml.safe_load((VOCAB / "metrics.yaml").read_text())
ERAS = [e["id"] for e in yaml.safe_load((VOCAB / "eras.yaml").read_text())]
SIDES = ["ua", "ru", "western", "international", "other"]
DATE = r"^\d{4}(-\d{2}(-\d{2})?)?$"

TIME = {"type": "object", "additionalProperties": False, "required": ["from", "precision"],
        "properties": {"from": {"type": "string", "pattern": DATE}, "to": {"type": "string", "pattern": DATE},
                       "precision": {"enum": ["day", "month", "quarter", "year"]}}}
MENTION_TYPES = ["place", "event", "system", "actor", "metric", "work"]


def schema() -> dict:
    claim = {
        "type": "object", "additionalProperties": False,
        "required": ["type", "local_id", "passage", "text", "kind", "epistemic", "cites"],
        "properties": {
            "type": {"const": "claim"},
            "local_id": {"type": "string", "pattern": r"^\d\d-\d{4}-\d{2,3}$"},
            "passage": {"type": "string", "pattern": r"^\d\d/\d{4}$"},
            "text": {"type": "string", "minLength": 8, "maxLength": 800},
            "kind": {"enum": ["fact", "figure", "event", "assessment", "forecast", "definition"]},
            "epistemic": {"enum": ["observed", "reported", "claimed", "estimated", "derived", "computed"]},
            "status": {"enum": ["active", "disputed"]},
            "claimants": {"type": "array", "items": {
                "type": "object", "additionalProperties": False, "required": ["name", "kind"],
                "properties": {"name": {"type": "string", "minLength": 2},
                               "kind": {"enum": KINDS["actor"]},
                               "side": {"enum": SIDES},
                               "role": {"enum": ["claimant", "estimator", "reporter", "analyst"]},
                               "said_at": {"type": "string", "pattern": DATE}}}},
            "cites": {"type": "array", "minItems": 1, "items": {
                "type": "object", "additionalProperties": False, "required": ["source"],
                "properties": {"source": {"type": "string"}, "locator": {"type": "string"},
                               "quote": {"type": "string", "maxLength": 400},
                               "support": {"enum": ["direct", "secondary", "background"]}}}},
            "time": TIME,
            "eras": {"type": "array", "items": {"enum": ERAS}, "uniqueItems": True},
            "sides": {"type": "array", "items": {"enum": ["ua", "ru", "western", "other"]}, "uniqueItems": True},
            "mentions": {"type": "array", "items": {
                "type": "object", "additionalProperties": False, "required": ["name", "type"],
                "properties": {"name": {"type": "string", "minLength": 2}, "type": {"enum": MENTION_TYPES},
                               "kind": {"type": "string"},
                               "role": {"enum": ["subject", "location", "instrument", "target", "measure", "mention"]}}}},
            "observations": {"type": "array", "items": {
                "type": "object", "additionalProperties": False, "required": ["metric", "unit", "value_text"],
                "properties": {
                    "metric": {"type": "string", "pattern": r"^[a-z][a-z0-9_]*$"},
                    "metric_new": {"type": "object", "additionalProperties": False, "required": ["label", "dimension"],
                                   "properties": {"label": {"type": "string"}, "dimension": {"type": "string"}}},
                    "value": {"type": "number"}, "low": {"type": "number"}, "high": {"type": "number"},
                    "value_text": {"type": "string", "minLength": 1},
                    "qualifier": {"enum": ["exact", "approx", "at_least", "at_most", "range"]},
                    "unit": {"enum": UNITS},
                    "time": TIME, "side": {"enum": ["ua", "ru", "western", "other"]},
                    "place": {"type": "string"}, "subject": {"type": "string"},
                    "method": {"enum": ["reported", "estimated", "derived", "computed"]},
                    "note": {"type": "string"}}}},
            "notes": {"type": "string"},
        },
    }
    counter = {
        "type": "object", "additionalProperties": False,
        "required": ["type", "local_id", "counter", "measure", "rationale", "evidence"],
        "properties": {
            "type": {"const": "counter"},
            "local_id": {"type": "string", "pattern": r"^\d\d-x\d{2,3}$"},
            "counter": {"type": "string"}, "counter_kind": {"enum": KINDS["system"]},
            "measure": {"type": "string"}, "measure_kind": {"enum": KINDS["system"]},
            "first_observed": TIME, "lag_days": {"type": "integer", "minimum": 0},
            "effect": {"type": "string"}, "rationale": {"type": "string"},
            "evidence": {"type": "array", "minItems": 1, "items": {"type": "string", "pattern": r"^\d\d-\d{4}-\d{2,3}$"}},
        },
    }
    return {"$schema": "https://json-schema.org/draft/2020-12/schema", "title": "bavovniatko staging record",
            "oneOf": [claim, counter]}


_VALIDATOR = None


def validator():
    global _VALIDATOR
    if _VALIDATOR is None:
        s = schema()
        _VALIDATOR = {"claim": jsonschema.Draft202012Validator(s["oneOf"][0]),
                      "counter": jsonschema.Draft202012Validator(s["oneOf"][1])}
    return _VALIDATOR


def topic_file(nn: str) -> Path:
    return next(kb.RESEARCH.glob(f"{nn}-*.md"))


def source_ids() -> dict[str, str]:
    ids = {}
    for f in sorted((kb.RESEARCH / "sources").glob("*.yaml")):
        for e in yaml.safe_load(f.read_text()):
            ids[e["id"]] = e["id"]
            for a in e.get("aliases") or []:
                ids[a] = e["id"]
    merged = yaml.safe_load((kb.RESEARCH / "sources.yaml").read_text())
    for e in merged:
        for a in [e["id"], *(e.get("aliases") or [])]:
            ids[a] = e["id"]
    return ids


def errors_for(rec: dict, nn: str, passages: set[str], srcs: dict[str, str]) -> list[str]:
    t = rec.get("type")
    if t not in ("claim", "counter"):
        return ['"type" must be "claim" or "counter"']
    errs = [f"{'/'.join(map(str, e.path)) or '(root)'}: {e.message}" for e in validator()[t].iter_errors(rec)]
    if errs:
        return errs
    if not rec["local_id"].startswith(nn + "-"):
        errs.append(f"local_id must start with {nn}-")
    if t == "claim":
        if rec["passage"] not in passages:
            errs.append(f"unknown passage {rec['passage']}")
        elif rec["local_id"][:7] != rec["passage"].replace("/", "-"):
            errs.append("local_id must be <passage NN-OOOO>-<seq>")
        for c in rec["cites"]:
            if c["source"] not in srcs:
                errs.append(f"unknown source id {c['source']}")
        for o in rec.get("observations", []):
            if o["metric"] not in METRICS and "metric_new" not in o:
                errs.append(f"metric {o['metric']} is not in db/vocab/metrics.yaml; add metric_new {{label, dimension}}")
            if all(k not in o for k in ("value", "low", "high")) and re.search(r"\d", o["value_text"]) is not None:
                errs.append("observation has digits in value_text but no value/low/high")
            if o.get("qualifier") == "range" and not ("low" in o and "high" in o):
                errs.append("qualifier range needs low and high")
        for m in rec.get("mentions", []):
            if m.get("kind") and m["type"] in KINDS and m["kind"] not in KINDS[m["type"]]:
                errs.append(f"mention kind {m['kind']} not in kinds.yaml[{m['type']}]: {KINDS[m['type']]}")
    return errs


def load_staged(nn: str) -> list[dict]:
    p = CLAIMS / f"{nn}.jsonl"
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()] if p.exists() else []


def cmd_worksheet(nn: str, out: str | None):
    f = topic_file(nn)
    lines = [f"# Worksheet for {f.name}. Each block is headed by its passage key; cite it as \"passage\".\n"]
    for b in mdparse.blocks(f):
        lines.append(f"<<< {nn}/{b.order:04d} | {b.role} | {b.section or '(preamble)'} >>>")
        lines.append(b.text)
        lines.append("")
    text = "\n".join(lines)
    if out:
        Path(out).write_text(text)
        print(f"wrote {out}")
    else:
        print(text)


def cmd_append(nn: str, src: str):
    f = topic_file(nn)
    passages = {f"{nn}/{b.order:04d}" for b in mdparse.blocks(f)}
    srcs = source_ids()
    have = {r["local_id"] for r in load_staged(nn)}
    text = sys.stdin.read() if src == "-" else Path(src).read_text()
    ok, bad = [], 0
    for i, line in enumerate(text.splitlines(), 1):
        if not line.strip():
            continue
        try:
            rec = json.loads(line)
        except json.JSONDecodeError as e:
            print(f"line {i}: invalid JSON: {e}")
            bad += 1
            continue
        errs = errors_for(rec, nn, passages, srcs)
        if rec.get("local_id") in have:
            errs.append(f"duplicate local_id {rec['local_id']} (already staged)")
        if errs:
            bad += 1
            print(f"line {i} ({rec.get('local_id', '?')}): " + "; ".join(errs))
        else:
            ok.append(json.dumps(rec, ensure_ascii=False))
            have.add(rec["local_id"])
    CLAIMS.mkdir(parents=True, exist_ok=True)
    with open(CLAIMS / f"{nn}.jsonl", "a") as fh:
        for l in ok:
            fh.write(l + "\n")
    print(f"appended {len(ok)}, rejected {bad} (rejected lines were NOT written; fix and re-append them)")
    return 1 if bad else 0


def cmd_check(nn: str) -> int:
    f = topic_file(nn)
    bl = mdparse.blocks(f)
    passages = {f"{nn}/{b.order:04d}": b for b in bl}
    srcs = source_ids()
    recs = load_staged(nn)
    errs = []
    seen = Counter(r.get("local_id") for r in recs)
    errs += [f"duplicate local_id {k}" for k, v in seen.items() if v > 1]
    for r in recs:
        errs += [f"{r.get('local_id')}: {e}" for e in errors_for(r, nn, set(passages), srcs)]
    claims = [r for r in recs if r["type"] == "claim"]
    ids = {r["local_id"] for r in claims}
    for r in recs:
        if r["type"] == "counter":
            errs += [f"{r['local_id']}: evidence {e} is not a staged claim" for e in r["evidence"] if e not in ids]
    # coverage: every [src:] in every extractable passage must be cited by a claim from that passage
    cited_by_passage: dict[str, set[str]] = {}
    for r in claims:
        cited_by_passage.setdefault(r["passage"], set()).update(srcs.get(c["source"], c["source"]) for c in r["cites"])
    missing = []
    for key, b in passages.items():
        if mdparse.meta_section(b.top) or b.role == "heading":
            continue
        want = {srcs.get(s, s) for s in mdparse.src_ids(b.text)}
        gap = want - cited_by_passage.get(key, set())
        if gap:
            missing.append((key, sorted(gap)))
    kinds = Counter(r["kind"] for r in claims)
    n_obs = sum(len(r.get("observations", [])) for r in claims)
    new_metrics = Counter(o["metric"] for r in claims for o in r.get("observations", []) if "metric_new" in o)
    print(f"{nn}: {len(claims)} claims ({dict(kinds)}), {n_obs} observations, "
          f"{sum(r['type'] == 'counter' for r in recs)} counter links, {len(passages)} passages")
    print(f"   passages with claims: {len(cited_by_passage)}; new metrics proposed: {dict(new_metrics)}")
    for e in errs[:40]:
        print("   ERROR", e)
    for key, gap in missing[:60]:
        print(f"   UNCOVERED {key}: {', '.join(gap)}")
    print(f"   => {len(errs)} errors, {len(missing)} passages with uncovered citations")
    return 1 if errs or missing else 0


def cmd_metrics():
    """Collect every metric_new proposal (and every metric use) -> db/staging/metrics_proposed.yaml."""
    uses, props = Counter(), {}
    for f in sorted(CLAIMS.glob("*.jsonl")):
        for line in f.read_text().splitlines():
            if not line.strip():
                continue
            r = json.loads(line)
            for o in r.get("observations", []):
                uses[o["metric"]] += 1
                if "metric_new" in o and o["metric"] not in METRICS:
                    p = props.setdefault(o["metric"], {"labels": Counter(), "dimensions": Counter(), "units": Counter(),
                                                       "topics": set(), "examples": []})
                    p["labels"][o["metric_new"]["label"]] += 1
                    p["dimensions"][o["metric_new"]["dimension"]] += 1
                    p["units"][o["unit"]] += 1
                    p["topics"].add(f.stem)
                    if len(p["examples"]) < 2:
                        p["examples"].append(f"{r['local_id']}: {o['value_text']} ({o.get('subject') or o.get('note') or ''})".strip())
    out = {m: {"uses": uses[m], "label": p["labels"].most_common(1)[0][0], "dimension": p["dimensions"].most_common(1)[0][0],
               "units": dict(p["units"]), "topics": sorted(p["topics"]), "examples": p["examples"]}
           for m, p in sorted(props.items())}
    (STAGING / "metrics_proposed.yaml").write_text(yaml.safe_dump(out, sort_keys=False, allow_unicode=True, width=140))
    print(f"{len(out)} proposed metrics; {sum(1 for m in uses if m in METRICS)} seed metrics in use ->",
          STAGING / "metrics_proposed.yaml")


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    w = sub.add_parser("worksheet"); w.add_argument("nn"); w.add_argument("--out")
    a = sub.add_parser("append"); a.add_argument("nn"); a.add_argument("file", nargs="?", default="-")
    c = sub.add_parser("check"); c.add_argument("nn")
    sub.add_parser("schema")
    sub.add_parser("metrics")
    args = ap.parse_args()
    if args.cmd == "worksheet":
        cmd_worksheet(args.nn, args.out)
    elif args.cmd == "append":
        sys.exit(cmd_append(args.nn, args.file))
    elif args.cmd == "check":
        nns = [f"{i:02d}" for i in range(9)] if args.nn == "all" else [args.nn]
        sys.exit(max(cmd_check(n) for n in nns))
    elif args.cmd == "metrics":
        cmd_metrics()
    elif args.cmd == "schema":
        (STAGING / "claim.schema.json").write_text(json.dumps(schema(), indent=2, ensure_ascii=False) + "\n")
        print("wrote", STAGING / "claim.schema.json")


if __name__ == "__main__":
    main()
