#!/usr/bin/env python3
"""Apply every migration to a throwaway database and exercise the schema end to end.

Covers: vocab kinds, sources/actors, claims with cites/asserted_by/about, observations with unit
conversion, claim↔claim relations and symmetric contradiction traversal, geometry + spatial
predicates, full-text search, ON DELETE REJECT, the audit trail, and ENFORCED relations.
Exit code 0 = all checks passed. The test database is removed afterwards.
"""
import datetime as dt
import sys
import uuid
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import kb  # noqa: E402

NS, DB = "test", f"smoke_{uuid.uuid4().hex[:8]}"
NOW = dt.datetime.now(dt.timezone.utc)
PROV = {"by": "smoke_test", "method": "import", "at": NOW}
failures = []


def check(name, cond, detail=""):
    print(("ok   " if cond else "FAIL ") + name + (f"  ({detail})" if detail and not cond else ""))
    if not cond:
        failures.append(name)


def expect_error(conn, name, q, vars=None):
    try:
        kb.run(conn, q, vars)
        check(name, False, "no error raised")
    except kb.KBError:
        check(name, True)


def main():
    root = kb.connect()
    kb.run(root, f"DEFINE NAMESPACE IF NOT EXISTS {NS}; USE NS {NS}; DEFINE DATABASE {DB} STRICT;")
    c = kb.connect(NS, DB)
    try:
        for f in sorted((kb.DB_DIR / "schema").glob("[0-9][0-9][0-9][0-9]_*.surql")):
            kb.run(c, f.read_text())
        # applying twice must be a no-op (IF NOT EXISTS everywhere)
        for f in sorted((kb.DB_DIR / "schema").glob("[0-9][0-9][0-9][0-9]_*.surql")):
            kb.run(c, f.read_text())
        check("migrations apply twice", True)

        kb.run(c, """
          CREATE kind:["source","news"]        SET labels = { en: "News" };
          CREATE kind:["actor","outlet"]       SET labels = { en: "Outlet" };
          CREATE kind:["actor","government_body"] SET labels = { en: "Government body" };
          CREATE kind:["place","settlement"]   SET labels = { en: "Settlement" };
          CREATE kind:["place","oblast"]       SET labels = { en: "Oblast" };
          CREATE kind:["system","glide_bomb"]  SET labels = { en: "Glide bomb" };
          CREATE kind:["system","ew"]          SET labels = { en: "Electronic warfare" };
          CREATE era:e9 SET code = "e9-counteroffensive-26", labels = { en: "Counteroffensive 2026" },
                 time = { from: d"2026-01-29", precision: "day" }, order = 9, summary = "…";
          CREATE topic:t03 SET code = "03", labels = { en: "Fires & air" };
          CREATE unit:count_per_month SET symbol = "count/month", labels = { en: "per month" }, dimension = "count/time";
          CREATE unit:count_per_day SET symbol = "count/day", labels = { en: "per day" }, dimension = "count/time",
                 base = unit:count_per_month, to_base = 30.4375;
          CREATE metric:kab_launches SET labels = { en: "KAB glide-bomb launches" }, description = "…",
                 dimension = "count/time", default_unit = unit:count_per_month;
        """)
        check("kind.domain derived from id", kb.one(c, "RETURN kind:['place','oblast'].domain") == "place")

        kb.run(c, """
          CREATE actor:kyiv_post SET labels = { en: "Kyiv Post" }, kind = kind:["actor","outlet"], side = "ua", prov = $p;
          CREATE actor:ua_air_force SET labels = { en: "Ukrainian Air Force", uk: "Повітряні сили" },
                 kind = kind:["actor","government_body"], side = "ua", prov = $p;
          CREATE source:⟨kyivpost-2026-korshak-glide-bombs⟩ SET title = "Glide bombs", url = "https://example.org/a",
                 publisher = actor:kyiv_post, type = kind:["source","news"], origin = "ukrainian", reliability = "medium",
                 verified = "fetched", eras = [era:e9], topics = [topic:t03], prov = $p;
          CREATE claim:a SET key = "k-a", text = "The Air Force counted 8,266 glide bombs in June 2026.", kind = "figure",
                 epistemic = "claimed", eras = [era:e9], topics = [topic:t03], sides = ["ru"],
                 time = { from: d"2026-06-01", to: d"2026-06-30", precision: "month" }, prov = $p;
          CREATE claim:b SET key = "k-b", text = "Russia dropped about 6,000 glide bombs in June 2026.", kind = "figure",
                 epistemic = "estimated", eras = [era:e9], topics = [topic:t03], prov = $p;
          CREATE claim:c SET key = "k-c", text = "Glide-bomb use kept rising throughout the counteroffensive.", kind = "assessment",
                 epistemic = "reported", eras = [era:e9], topics = [topic:t03], prov = $p;
          RELATE claim:a->cites->source:⟨kyivpost-2026-korshak-glide-bombs⟩ SET support = "direct", prov = $p;
          RELATE claim:a->asserted_by->actor:ua_air_force SET prov = $p;
          CREATE observation:o1 SET claim = claim:a, metric = metric:kab_launches, value = 8266, unit = unit:count_per_month,
                 time = { from: d"2026-06-01", to: d"2026-06-30", precision: "month" }, side = "ru", prov = $p;
          CREATE observation:o2 SET claim = claim:b, metric = metric:kab_launches, value = 200, qualifier = "approx",
                 unit = unit:count_per_day, side = "ru", prov = $p;
          RELATE claim:b->contradicts->claim:a SET strength = 0.8, rationale = "6,000 vs 8,266 for the same month",
                 method = "rule", prov = $p;
          RELATE claim:c->supports->claim:a SET strength = 0.5, rationale = "trend consistent", method = "agent", prov = $p;
        """, {"p": PROV})

        row = kb.one(c, "SELECT n_sources, ->asserted_by->actor.labels.en AS who FROM claim:a")[0]
        check("computed n_sources", row["n_sources"] == 1, row)
        check("asserted_by traversal", row["who"] == ["Ukrainian Air Force"], row)
        both = kb.one(c, "SELECT VALUE array::distinct(<->contradicts<->claim) FROM claim:a")[0]
        check("symmetric contradicts traversal", {r.id for r in both} >= {"b"}, both)
        strong = kb.one(c, "SELECT VALUE ->(supports WHERE strength >= 0.5)->claim FROM claim:c")[0]
        check("inline-filtered traversal", [r.id for r in strong] == ["a"], strong)
        vb = kb.one(c, "SELECT VALUE value_base FROM observation:o2")[0]
        check("unit conversion to base", abs(vb - 200 * 30.4375) < 1e-6, vb)
        hits = kb.one(c, "SELECT VALUE id FROM claim WHERE text @@ 'bomb'")
        check("full-text search with stemming", {h.id for h in hits} == {"a", "b", "c"}, hits)

        oblast = {"type": "Polygon", "coordinates": [[[35.0, 47.0], [37.5, 47.0], [37.5, 48.5], [35.0, 48.5], [35.0, 47.0]]]}
        kb.run(c, """
          CREATE place:zaporizhzhia_oblast SET labels = { en: "Zaporizhzhia Oblast" }, kind = kind:["place","oblast"],
                 geometry = $poly, prov = $p;
          CREATE place:huliaipole SET labels = { en: "Huliaipole", uk: "Гуляйполе" }, kind = kind:["place","settlement"],
                 geometry = $pt, parent = place:zaporizhzhia_oblast, prov = $p;
          RELATE claim:c->about->place:huliaipole SET role = "location", prov = $p;
        """, {"p": PROV, "poly": kb.geom(oblast), "pt": kb.geom({"type": "Point", "coordinates": [36.26, 47.66]})})
        inside = kb.one(c, "SELECT VALUE id FROM place WHERE id != place:zaporizhzhia_oblast AND geometry INSIDE (SELECT VALUE geometry FROM ONLY place:zaporizhzhia_oblast)")
        check("spatial INSIDE", [r.id for r in inside] == ["huliaipole"], inside)
        near = kb.one(c, "SELECT VALUE <-about<-claim FROM place WHERE geo::distance(geometry, (36.3, 47.7)) < 10000")
        check("claims near a point", any(r.id == "c" for grp in near for r in grp), near)

        expect_error(c, "REFERENCE ON DELETE REJECT (era in use)", "DELETE era:e9")
        expect_error(c, "kind domain enforced", "CREATE actor:x SET labels = { en: 'X' }, kind = kind:['place','oblast'], prov = $p", {"p": PROV})
        expect_error(c, "ENFORCED relation needs both ends", "RELATE claim:a->cites->source:nope SET prov = $p", {"p": PROV})
        expect_error(c, "relation endpoint types enforced", "RELATE claim:a->cites->actor:kyiv_post SET prov = $p", {"p": PROV})
        expect_error(c, "strength bounds", "RELATE claim:a->supports->claim:b SET strength = 2, rationale = 'x', method = 'human', prov = $p", {"p": PROV})
        expect_error(c, "STRICT database rejects undefined tables", "CREATE nosuchtable:x")
        expect_error(c, "duplicate cites edge rejected", "RELATE claim:a->cites->source:⟨kyivpost-2026-korshak-glide-bombs⟩ SET prov = $p", {"p": PROV})

        kb.run(c, "UPDATE claim:a SET status = 'disputed'")
        revs = kb.one(c, "SELECT VALUE before.status FROM revision WHERE target = claim:a")
        check("audit revision on update", revs == ["active"], revs)
        kb.one(c, "SHOW CHANGES FOR TABLE claim SINCE 0 LIMIT 10")
        check("changefeed readable", True)
    finally:
        kb.run(root, f"USE NS {NS}; REMOVE DATABASE {DB};")
    print(f"\n{'ALL PASSED' if not failures else f'{len(failures)} FAILED'}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
