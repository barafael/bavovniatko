#!/usr/bin/env python3
"""Phase 5: relations between claims (supports / weakens / contradicts / refines / supersedes / duplicates).

  relate.py candidates [--max-per-claim 6]   rule-based candidate pairs from the database
                                             -> db/staging/relations/candidates.jsonl
  relate.py batches --size 40                split candidates into db/staging/relations/batches/bNNN.jsonl
  relate.py load                             judged pairs (db/staging/relations/judged/*.jsonl) -> relation edges
  relate.py status                           how many batches are judged

Candidate rules (no LLM):
  numeric   same metric, same side, overlapping time, comparable units: agreeing values -> "supports?",
            disjoint values -> "contradicts?"
  entities  claims sharing >= 2 specific entities (rare ones, IDF-weighted) in overlapping eras
  text      near-identical wording in different passages -> "duplicates?"
Each candidate carries a `hint`; the judging agent decides the relation, its direction and strength.
Judged line: {"pair": "…", "relation": "supports|weakens|contradicts|refines|supersedes|duplicates|none",
              "from": "<claim id>", "to": "<claim id>", "strength": 0.0–1.0, "rationale": "…",
              "resolution": "… (contradicts only, optional)", "independent": true|false (supports only)}
"""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import itertools
import json
import math
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402

REL = kb.DB_DIR / "staging" / "relations"
RELATIONS = ["supports", "weakens", "contradicts", "refines", "supersedes", "duplicates"]
# war-wide series: two observations are comparable without sharing a subject entity
GLOBAL_METRICS = {"kab_launches", "shahed_launches", "jet_geran_launches", "territory_change_net", "territory_gained",
                  "territory_occupied", "territory_occupied_share", "grey_zone_area", "ballistic_missile_launches",
                  "casualties_killed", "casualties_total", "recruitment", "mobilisation", "awol_cases", "ugv_missions",
                  "kill_zone_depth", "fire_ratio", "artillery_rounds_fired", "aid_value", "casualties_per_km2"}
UTC = dt.timezone.utc


def claims(conn) -> dict[str, dict]:
    rows = kb.one(conn, """SELECT id, key, text, kind, epistemic, status, eras, topics, time,
        ->cites->source AS sources, ->asserted_by->actor.labels.en AS claimants,
        ->about->? AS about FROM claim""")
    return {kb.rid_str(r["id"]): r for r in rows}


def observations(conn):
    return kb.one(conn, """SELECT claim, metric, value, low, high, qualifier, unit, unit.dimension AS dim,
        value_base, unit.to_base AS to_base, unit.base AS base, time, side, place FROM observation""")


def interval(o) -> tuple[float, float] | None:
    f = o.get("to_base") or 1.0
    if o.get("low") is not None or o.get("high") is not None:
        lo = o.get("low") if o.get("low") is not None else o.get("value")
        hi = o.get("high") if o.get("high") is not None else o.get("value")
        if lo is None or hi is None:
            return None
        return lo * f, hi * f
    v = o.get("value")
    if v is None:
        return None
    q = o.get("qualifier")
    if q == "approx":
        return v * 0.85 * f, v * 1.15 * f
    if q == "at_least":
        return v * f, math.inf
    if q == "at_most":
        return -math.inf, v * f
    return v * f, v * f


def unit_key(o) -> str:
    base = o.get("base")
    return kb.rid_str(base) if base else kb.rid_str(o["unit"])


def span(t) -> tuple[dt.datetime, dt.datetime] | None:
    if not t:
        return None
    a = t["from"]
    b = t.get("to") or a
    pad = {"day": 1, "month": 31, "quarter": 92, "year": 366}[t["precision"]]
    return a, b + dt.timedelta(days=pad)


def overlaps(s1, s2) -> bool:
    return s1 is None or s2 is None or (s1[0] < s2[1] and s2[0] < s1[1])


def similar_span(s1, s2, min_jaccard: float = 0.7) -> bool:
    """Both periods mostly coincide (a monthly figure is not comparable with a yearly total)."""
    if s1 is None or s2 is None:
        return True
    inter = (min(s1[1], s2[1]) - max(s1[0], s2[0])).total_seconds()
    union = (max(s1[1], s2[1]) - min(s1[0], s2[0])).total_seconds()
    return union > 0 and inter / union >= min_jaccard


def words(text: str) -> set[str]:
    return set(re.findall(r"[a-z0-9]{3,}", text.lower()))


def pair_id(a: str, b: str) -> str:
    x, y = sorted([a, b])
    return hashlib.sha1(f"{x}|{y}".encode()).hexdigest()[:12]


def cmd_candidates(max_per_claim: int):
    conn = kb.connect()
    C = claims(conn)
    cand: dict[str, dict] = {}

    def add(a, b, rule, hint, score, detail=""):
        if a == b:
            return
        pid = pair_id(a, b)
        c = cand.setdefault(pid, {"pair": pid, "a": min(a, b), "b": max(a, b), "rules": [], "hints": [], "score": 0.0, "detail": []})
        if rule not in c["rules"]:
            c["rules"].append(rule)
            c["hints"].append(hint)
        c["score"] = max(c["score"], score)
        if detail:
            c["detail"].append(detail)

    # numeric rule: same metric/side/unit, overlapping time, and (war-wide metric or a shared specific subject)
    subj = {cid: {kb.rid_str(e) for e in c.get("about") or [] if e.table_name in ("system", "event", "place", "work", "actor")}
            for cid, c in C.items()}
    groups = defaultdict(list)
    for o in observations(conn):
        groups[(kb.rid_str(o["metric"]), o.get("side"), unit_key(o))].append(o)
    for (metric, side, _), obs in groups.items():
        if metric in ("metric:count_generic", "metric:share_generic"):
            continue
        for o1, o2 in itertools.combinations(obs, 2):
            a, b = kb.rid_str(o1["claim"]), kb.rid_str(o2["claim"])
            if a == b or not similar_span(span(o1.get("time")), span(o2.get("time"))):
                continue
            if metric.split(":", 1)[1] not in GLOBAL_METRICS and not (subj.get(a, set()) & subj.get(b, set())):
                continue
            if (o1.get("place") and o2.get("place")) and o1["place"] != o2["place"]:
                continue
            i1, i2 = interval(o1), interval(o2)
            if not i1 or not i2:
                continue
            agree = i1[0] <= i2[1] and i2[0] <= i1[1]
            d = f"{metric} {side or ''}: {o1.get('value', (o1.get('low'), o1.get('high')))} vs {o2.get('value', (o2.get('low'), o2.get('high')))}"
            add(a, b, "numeric", "supports?" if agree else "contradicts?", 0.9 if not agree else 0.6, d)

    # entity rule (IDF-weighted, rare entities only)
    ent_claims = defaultdict(set)
    for cid, c in C.items():
        for e in c.get("about") or []:
            ent_claims[kb.rid_str(e)].add(cid)
    n = len(C)
    idf = {e: math.log(n / len(cs)) for e, cs in ent_claims.items() if 1 < len(cs) <= 40}
    per_claim = defaultdict(list)
    for cid, c in C.items():
        mine = {kb.rid_str(e) for e in c.get("about") or []} & set(idf)
        scores = Counter()
        for e in mine:
            for other in ent_claims[e]:
                if other != cid:
                    scores[other] += idf[e]
        for other, s in scores.items():
            shared = mine & {kb.rid_str(e) for e in C[other].get("about") or []}
            if len(shared) < 2:
                continue
            e1 = {x.id for x in c.get("eras") or []}
            e2 = {x.id for x in C[other].get("eras") or []}
            if e1 and e2 and not ({x[:2] for x in e1} & {x[:2] for x in e2}):
                continue
            per_claim[cid].append((s, other, sorted(shared)))
    for cid, lst in per_claim.items():
        for s, other, shared in sorted(lst, reverse=True)[:max_per_claim]:
            add(cid, other, "entities", "related?", min(0.5, s / 20), "shared: " + ", ".join(shared))

    # text rule
    toks = {cid: words(c["text"]) for cid, c in C.items()}
    by_word = defaultdict(set)
    for cid, ws in toks.items():
        for w in ws:
            by_word[w].add(cid)
    for cid, ws in toks.items():
        seen = Counter()
        for w in ws:
            if len(by_word[w]) < 30:
                for other in by_word[w]:
                    if other > cid:
                        seen[other] += 1
        for other, k in seen.items():
            j = k / len(ws | toks[other])
            if j >= 0.6:
                add(cid, other, "text", "duplicates?", j, f"jaccard {j:.2f}")

    REL.mkdir(parents=True, exist_ok=True)
    out = []
    for c in sorted(cand.values(), key=lambda c: (-c["score"], c["pair"])):
        for side in ("a", "b"):
            x = C[c[side]]
            c[side + "_claim"] = {"id": c[side], "text": x["text"], "kind": x["kind"], "epistemic": x["epistemic"],
                                  "claimants": x.get("claimants") or [], "sources": [s.id for s in x.get("sources") or []],
                                  "time": {k: (v.date().isoformat() if isinstance(v, dt.datetime) else v) for k, v in (x.get("time") or {}).items()},
                                  "eras": [e.id for e in x.get("eras") or []], "topic": x["key"][:2]}
        out.append(c)
    (REL / "candidates.jsonl").write_text("".join(json.dumps(c, ensure_ascii=False, default=str) + "\n" for c in out))
    print(f"{len(out)} candidate pairs", Counter(r for c in out for r in c["rules"]))


def cmd_batches(size: int):
    rows = [json.loads(l) for l in (REL / "candidates.jsonl").read_text().splitlines() if l.strip()]
    bdir = REL / "batches"
    bdir.mkdir(parents=True, exist_ok=True)
    for f in bdir.glob("b*.jsonl"):
        f.unlink()
    slim = lambda c: {"pair": c["pair"], "rules": c["rules"], "hints": c["hints"], "detail": c["detail"][:3],
                      "a": c["a_claim"], "b": c["b_claim"]}
    for i in range(0, len(rows), size):
        (bdir / f"b{i // size:03d}.jsonl").write_text("".join(json.dumps(slim(c), ensure_ascii=False) + "\n" for c in rows[i:i + size]))
    print(f"{math.ceil(len(rows) / size)} batches of <= {size} in {bdir}")


def cmd_status():
    b = sorted((REL / "batches").glob("b*.jsonl"))
    j = {p.stem for p in (REL / "judged").glob("b*.jsonl")} if (REL / "judged").exists() else set()
    print(f"{len(j)}/{len(b)} batches judged; missing: {[p.stem for p in b if p.stem not in j][:30]}")


def cmd_validate(names: list[str]) -> int:
    """Check judged files against their batches: every pair judged once, ids match the pair, fields valid."""
    errs = 0
    for name in names:
        bf, jf = REL / "batches" / f"{name}.jsonl", REL / "judged" / f"{name}.jsonl"
        pairs = {json.loads(l)["pair"]: json.loads(l) for l in bf.read_text().splitlines() if l.strip()}
        if not jf.exists():
            print(f"{name}: not judged yet"); errs += 1; continue
        seen = set()
        for i, line in enumerate(jf.read_text().splitlines(), 1):
            if not line.strip():
                continue
            try:
                j = json.loads(line)
            except json.JSONDecodeError as e:
                print(f"{name}:{i}: invalid JSON {e}"); errs += 1; continue
            p = pairs.get(j.get("pair"))
            msg = []
            if not p:
                msg.append("unknown pair")
            elif j.get("relation") != "none":
                if j.get("relation") not in RELATIONS:
                    msg.append(f"relation must be one of {RELATIONS + ['none']}")
                if {j.get("from"), j.get("to")} != {p["a"]["id"], p["b"]["id"]}:
                    msg.append("from/to must be the pair's two claim ids")
                if not isinstance(j.get("strength"), (int, float)) or not 0 <= j["strength"] <= 1:
                    msg.append("strength must be 0..1")
                if not j.get("rationale"):
                    msg.append("rationale required")
            if j.get("pair") in seen:
                msg.append("pair judged twice")
            seen.add(j.get("pair"))
            if msg:
                print(f"{name}:{i}: " + "; ".join(msg)); errs += 1
        missing = set(pairs) - seen
        if missing:
            print(f"{name}: {len(missing)} pairs not judged"); errs += 1
        elif not any(True for _ in []):
            pass
    print("OK" if not errs else f"{errs} problems")
    return 1 if errs else 0


def cmd_load():
    conn = kb.connect()
    now = dt.datetime.now(UTC)
    stats, problems = Counter(), []
    valid = {kb.rid_str(r) for r in kb.one(conn, "SELECT VALUE id FROM claim")}
    for f in sorted((REL / "judged").glob("b*.jsonl")):
        for line in f.read_text().splitlines():
            if not line.strip():
                continue
            j = json.loads(line)
            rel = j.get("relation")
            if rel == "none":
                stats["none"] += 1
                continue
            if rel not in RELATIONS or j.get("from") not in valid or j.get("to") not in valid or j["from"] == j["to"]:
                problems.append(f"{f.stem}/{j.get('pair')}: bad relation/ids {rel} {j.get('from')} {j.get('to')}")
                continue
            a, b = (kb.rid(*j["from"].split(":", 1)), kb.rid(*j["to"].split(":", 1)))
            d = {"strength": float(j.get("strength", 0.5)), "rationale": j.get("rationale", ""),
                 "method": "agent", "prov": {"by": "relate", "method": "agent", "run": f.stem, "at": now,
                                              "input": f"db/staging/relations/judged/{f.name}"}}
            if rel == "contradicts" and j.get("resolution"):
                d["resolution"] = j["resolution"]
                d["state"] = "explained"
            if rel == "supports" and "independent" in j:
                d["independent"] = bool(j["independent"])
            # contradicts is symmetric: store once (lower id -> higher id)
            if rel == "contradicts" and j["from"] > j["to"]:
                a, b = b, a
            q = f"DELETE {rel} WHERE in = $a AND out = $b; RELATE $a->{rel}->$b CONTENT $d;"
            try:
                kb.run(conn, q, {"a": a, "b": b, "d": d})
                stats[rel] += 1
            except kb.KBError as ex:
                problems.append(f"{f.stem}/{j.get('pair')}: {ex}")
    for p in problems[:40]:
        print("  ", p)
    print(dict(stats), f"problems={len(problems)}")


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    c = sub.add_parser("candidates"); c.add_argument("--max-per-claim", type=int, default=6)
    b = sub.add_parser("batches"); b.add_argument("--size", type=int, default=40)
    sub.add_parser("load")
    sub.add_parser("status")
    v = sub.add_parser("validate"); v.add_argument("batches", nargs="+")
    a = ap.parse_args()
    if a.cmd == "candidates":
        cmd_candidates(a.max_per_claim)
    elif a.cmd == "batches":
        cmd_batches(a.size)
    elif a.cmd == "load":
        cmd_load()
    elif a.cmd == "validate":
        sys.exit(cmd_validate(a.batches))
    else:
        cmd_status()


if __name__ == "__main__":
    main()
