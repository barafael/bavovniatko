#!/usr/bin/env python3
"""Phase 8: the game feed.

  sim.py series              rule-built time series from observations -> sim_series
  sim.py load                agent-derived rows in db/staging/sim/*.yaml -> sim_loadout, sim_param, sim_terrain_profile
  sim.py export [--out DIR]  game-data/sim-YYYYMMDD.json (+ latest.json): eras, loadouts, params, series,
                             terrain profiles, the measure/countermeasure graph and source attribution

Everything carries `derived_from` claims; the export lists the sources behind them for attribution.
Staging format (db/staging/sim/<name>.yaml), a mapping with any of these lists:
  loadouts: [{era: e2, side: ua, system: system:himars, availability: limited, first_available: "2022-06",
              quantity_note: "...", derived_from: [03-0012-04, ...], method: "...", notes: "..."}]
  params:   [{key: kill_zone_depth, era: e8, side: ru, dist: {type: triangular, min: 10, mode: 15, max: 25},
              unit: km, description: "...", derived_from: [...], method: "..."}]
  terrain:  [{key: huliaipole_e, place: place:huliaipole_e_sample_box, description: "...", params: {...},
              derived_from: [...], method: "...", attribution: "© OpenStreetMap contributors (ODbL)"}]
"""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import sys
from collections import defaultdict
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402
from load import claim_rid, ptime, rec_id  # noqa: E402

UTC = dt.timezone.utc
NOW = dt.datetime.now(UTC)
STAGE = kb.DB_DIR / "staging" / "sim"
OUT = kb.REPO / "game-data"
SCHEMA_VERSION = 1

# metric -> (series description, preferred claimant order). Values are converted to the metric's default unit.
SERIES = {
    "kab_launches": "Russian guided glide bombs (KAB) dropped per month",
    "shahed_launches": "Shahed/Geran one-way attack drones launched per month",
    "jet_geran_launches": "Jet-powered Geran launches per month",
    "territory_change_net": "Net change of Russian-occupied area per month (DeepState geometry)",
    "grey_zone_area": "Grey-zone area (DeepState)",
    "territory_gained": "Area gained per month by side",
    "ugv_missions": "Ukrainian ground-robot (UGV) missions per month",
    "interception_rate": "Share of Russian drones/missiles intercepted",
    "kill_zone_depth": "Depth of the drone kill zone",
    "artillery_rounds_fired": "Artillery rounds fired per day",
    "ballistic_missile_launches": "Russian ballistic missile launches per month",
}
METHOD_RANK = {"computed": 0, "reported": 1, "estimated": 2, "derived": 3}


def prov(method="rule", input_=None):
    p = {"by": "sim", "method": method, "run": "sim@" + NOW.strftime("%Y%m%dT%H%M%SZ"), "at": NOW}
    if input_:
        p["input"] = input_
    return p


def cmd_series(conn):
    """One point per period per (metric, side). Only observations already in the metric's default unit and stated
    for a whole period of the series' precision are used (no scaling a single day up to a month). When a period has
    several candidates, the one closest to the period's median wins (robust against sub-scope figures), then
    method (computed > reported > estimated > derived), then more sources."""
    import statistics
    n = 0
    for metric, desc in SERIES.items():
        rows = kb.one(conn, """SELECT claim, value, low, high, unit, time, side, method, count(claim->cites) AS nsrc,
                metric.default_unit AS du FROM observation WHERE metric = $m AND time != NONE""", {"m": kb.rid("metric", metric)})
        slots = defaultdict(list)
        for r in rows:
            if kb.rid_str(r["unit"]) != kb.rid_str(r["du"]):
                continue
            t = r["time"]
            if t.get("to") and t["to"] != t["from"]:
                continue                                   # multi-period aggregates are not series points
            if t["precision"] == "day" and "per_" in kb.rid_str(r["unit"]):
                continue                                   # a single day's rate is not a period value
            mid = r.get("value") if r.get("value") is not None else (
                (r["low"] + r["high"]) / 2 if r.get("low") is not None and r.get("high") is not None else None)
            if mid is None:
                continue
            slots[(r.get("side"), kb.rid_str(r["du"]), t["from"], t["precision"])].append((mid, r))
        series = defaultdict(list)
        for (side, du, frm, prec), cands in slots.items():
            med = statistics.median(m for m, _ in cands)
            mid, best = min(cands, key=lambda c: (abs(c[0] - med), METHOD_RANK.get(c[1].get("method"), 9), -c[1].get("nsrc", 0)))
            series[(side, du)].append({"t": frm, "precision": prec, "claim": best["claim"],
                                       **{k: best[k] for k in ("value", "low", "high") if best.get(k) is not None},
                                       "_alts": len(cands)})
        for (side, du), pts in series.items():
            pts.sort(key=lambda p: p["t"])
            if len(pts) < 2:
                continue
            for p in pts:
                p.pop("_alts", None)
            key = f"{metric}__{side or 'all'}"
            kb.run(conn, "UPSERT type::record('sim_series', $k) CONTENT $d", {"k": key, "d": {
                "key": key, "metric": kb.rid("metric", metric), "side": side, "unit": rec_id(du), "points": pts,
                "description": desc, "reviewed": False, "prov": prov(), "ext": {},
                "method": "one point per period from whole-period observations in the metric's default unit; "
                          "closest to the period median, then computed > reported > estimated > derived, then more sources"}})
            n += 1
    print(f"sim_series: {n} series")


def local_ids(xs):
    return [claim_rid(x) if not str(x).startswith("claim:") else rec_id(x) for x in xs or []]


def cmd_load(conn):
    stats = defaultdict(int)
    for f in sorted(STAGE.glob("*.yaml")):
        doc = yaml.safe_load(f.read_text()) or {}
        rel = f"db/staging/sim/{f.name}"
        for r in doc.get("loadouts", []):
            d = {"era": kb.rid("era", r["era"]), "side": r["side"], "system": rec_id(r["system"]),
                 "availability": r["availability"], "derived_from": local_ids(r.get("derived_from")),
                 "method": r.get("method", "agent derivation from claims"), "reviewed": bool(r.get("reviewed", False)),
                 "prov": prov("agent", rel)}
            if r.get("first_available"):
                fa = str(r["first_available"])
                d["first_available"] = ptime({"from": fa, "precision": {1: "year", 2: "month", 3: "day"}[fa.count("-") + 1]})
            for k in ("quantity_note", "notes"):
                if r.get(k):
                    d[k] = r[k]
            kb.run(conn, "DELETE sim_loadout WHERE era = $d.era AND side = $d.side AND system = $d.system; CREATE sim_loadout CONTENT $d", {"d": d})
            stats["sim_loadout"] += 1
        for r in doc.get("params", []):
            d = {"key": r["key"], "dist": r["dist"], "unit": kb.rid("unit", r["unit"]), "description": r["description"],
                 "derived_from": local_ids(r.get("derived_from")), "method": r.get("method", "agent derivation from claims"),
                 "reviewed": bool(r.get("reviewed", False)), "prov": prov("agent", rel), "ext": {}}
            if r.get("era"):
                d["era"] = kb.rid("era", r["era"])
            if r.get("side"):
                d["side"] = r["side"]
            kb.run(conn, "DELETE sim_param WHERE key = $d.key AND era = $d.era AND side = $d.side; CREATE sim_param CONTENT $d", {"d": d})
            stats["sim_param"] += 1
        for r in doc.get("terrain", []):
            d = {"key": r["key"], "description": r["description"], "params": r["params"],
                 "derived_from": local_ids(r.get("derived_from")), "method": r.get("method", "agent derivation from claims"),
                 "reviewed": bool(r.get("reviewed", False)), "prov": prov("agent", rel), "ext": {}}
            if r.get("place"):
                d["place"] = rec_id(r["place"])
            if r.get("attribution"):
                d["attribution"] = r["attribution"]
            kb.run(conn, "UPSERT type::record('sim_terrain_profile', $k) CONTENT $d", {"k": r["key"], "d": d})
            stats["sim_terrain_profile"] += 1
    print(dict(stats))


def cmd_check(conn) -> int:
    """Validate db/staging/sim/*.yaml without writing: referenced eras, systems, places, units and claims exist."""
    errs = []
    exists = lambda rid: bool(kb.one(conn, "SELECT VALUE id FROM $r", {"r": rid}))
    for f in sorted(STAGE.glob("*.yaml")):
        doc = yaml.safe_load(f.read_text()) or {}
        for sec in doc:
            if sec not in ("loadouts", "params", "terrain"):
                errs.append(f"{f.name}: unknown section {sec}")
        for i, r in enumerate(doc.get("loadouts", [])):
            w = f"{f.name} loadouts[{i}]"
            if r.get("availability") not in ("none", "prototype", "scarce", "limited", "common", "abundant"):
                errs.append(f"{w}: availability")
            if r.get("side") not in ("ua", "ru"):
                errs.append(f"{w}: side")
            for rid in (kb.rid("era", r.get("era", "?")), rec_id(r.get("system", "system:?"))):
                if not exists(rid):
                    errs.append(f"{w}: {kb.rid_str(rid)} does not exist")
        for i, r in enumerate(doc.get("params", [])):
            w = f"{f.name} params[{i}] {r.get('key')}"
            if not exists(kb.rid("unit", r.get("unit", "?"))):
                errs.append(f"{w}: unknown unit {r.get('unit')}")
            if r.get("era") and not exists(kb.rid("era", r["era"])):
                errs.append(f"{w}: unknown era")
            if (r.get("dist") or {}).get("type") not in ("point", "uniform", "triangular", "normal", "lognormal", "table"):
                errs.append(f"{w}: dist.type")
        for i, r in enumerate(doc.get("terrain", [])):
            if r.get("place") and not exists(rec_id(r["place"])):
                errs.append(f"{f.name} terrain[{i}]: place {r['place']} does not exist")
        for sec in ("loadouts", "params", "terrain"):
            for i, r in enumerate(doc.get(sec, [])):
                if not r.get("derived_from"):
                    errs.append(f"{f.name} {sec}[{i}]: derived_from is required")
                for cid in local_ids(r.get("derived_from")):
                    if not exists(cid):
                        errs.append(f"{f.name} {sec}[{i}]: claim {kb.rid_str(cid)} does not exist")
    for e in errs[:60]:
        print("ERROR", e)
    print("OK" if not errs else f"{len(errs)} errors")
    return 1 if errs else 0


def plain(v):
    if hasattr(v, "table_name"):
        return kb.rid_str(v)
    if isinstance(v, dt.datetime):
        return v.date().isoformat() if v.time() == dt.time(0, tzinfo=v.tzinfo) else v.isoformat()
    if isinstance(v, dict):
        return {k: plain(x) for k, x in v.items() if x is not None and k not in ("prov", "ext")}
    if isinstance(v, list):
        return [plain(x) for x in v]
    return v


def cmd_export(conn, out: Path):
    q = lambda s, v=None: plain(kb.one(conn, s, v))
    data = {
        "schema_version": SCHEMA_VERSION,
        "generated": NOW.isoformat(),
        "eras": q("SELECT id, code, labels.en AS name, time, parent, summary FROM era ORDER BY order"),
        "loadouts": q("SELECT era, side, system, system.labels.en AS system_name, availability, first_available, quantity_note, reviewed, derived_from FROM sim_loadout ORDER BY era, side, system"),
        "params": q("SELECT key, era, side, dist, unit.symbol AS unit, description, reviewed, derived_from FROM sim_param ORDER BY key, era, side"),
        "series": q("SELECT key, metric, side, unit.symbol AS unit, description, points, reviewed FROM sim_series ORDER BY key"),
        "terrain_profiles": q("SELECT key, place, description, params, attribution, reviewed, derived_from FROM sim_terrain_profile ORDER BY key"),
        "systems": q("SELECT id, labels.en AS name, kind, sides, parent, first_seen FROM system ORDER BY id"),
        "counters": q("SELECT in AS counter, out AS measure, first_observed, lag_days, effect, evidence FROM counters ORDER BY counter, measure"),
    }
    claims = sorted({c for sec in ("loadouts", "params", "terrain_profiles") for r in data[sec] for c in r.get("derived_from") or []}
                    | {p["claim"] for s in data["series"] for p in s["points"]}
                    | {c for r in data["counters"] for c in r.get("evidence") or []})
    src = q("SELECT id, title, url, publisher.labels.en AS publisher FROM source WHERE id IN (SELECT VALUE out FROM cites WHERE in IN $c)",
            {"c": [rec_id(c) for c in claims]}) if claims else []
    data["attribution"] = {
        "sources": src,
        "notices": sorted({t["attribution"] for t in data["terrain_profiles"] if t.get("attribution")}
                          | {"Derived from the bavovniatko research base; see research/ and db/ for claims and sources."}),
    }
    out.mkdir(parents=True, exist_ok=True)
    body = json.dumps(data, ensure_ascii=False, indent=1, sort_keys=False)
    digest = hashlib.sha256(body.encode()).hexdigest()
    name = f"sim-{NOW:%Y%m%d}.json"
    (out / name).write_text(body + "\n")
    (out / "latest.json").write_text(json.dumps({"file": name, "sha256": digest, "schema_version": SCHEMA_VERSION}, indent=1) + "\n")
    print(f"wrote {out / name} ({len(body) / 1e3:.0f} kB; sha256 {digest[:12]}…): "
          + ", ".join(f"{k}={len(v)}" for k, v in data.items() if isinstance(v, list)))


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    sub.add_parser("series")
    sub.add_parser("load")
    sub.add_parser("check")
    e = sub.add_parser("export"); e.add_argument("--out", default=str(OUT))
    a = ap.parse_args()
    conn = kb.connect()
    sys.exit({"series": lambda: cmd_series(conn), "load": lambda: cmd_load(conn), "check": lambda: cmd_check(conn),
              "export": lambda: cmd_export(conn, Path(a.out))}[a.cmd]() or 0)


if __name__ == "__main__":
    main()
