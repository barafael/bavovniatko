-- The explorer's query library: DEFINE FUNCTIONs loaded into the in-browser database after the data
-- (see db/tools/webexport.py). They keep notebook queries short, e.g. `fn::place_history(place:robotyne)`.
-- Every function is read-only. Keep one function per block, with a comment line above it: the notebook lists
-- these comments as the library's documentation.

-- Claims about a place or any place directly inside it, in time order, with claimants and sources.
DEFINE FUNCTION fn::place_history($place: record<place>) {
    LET $places = array::union([$place], (SELECT VALUE id FROM place WHERE parent = $place));
    LET $claims = array::distinct(array::flatten((SELECT VALUE <-about<-claim FROM $places)));
    RETURN SELECT id, time.from AS t, time.precision AS precision, key, kind, epistemic, status, text,
            ->about->place.labels.en AS places, ->asserted_by->actor.labels.en AS claimants,
            ->cites->source.title AS sources
        FROM $claims ORDER BY t, key;
} PERMISSIONS FULL;

-- Places whose centroid lies inside a region's polygon (an oblast, raion or diorama box).
DEFINE FUNCTION fn::in_region($region: record<place>) {
    LET $g = $region.geometry;
    RETURN IF $g = NONE { [] } ELSE { SELECT VALUE id FROM place WHERE centroid != NONE AND centroid INSIDE $g };
} PERMISSIONS FULL;

-- Claims about any place inside a region.
DEFINE FUNCTION fn::claims_in_region($region: record<place>) {
    RETURN array::distinct(array::flatten((SELECT VALUE <-about<-claim FROM fn::in_region($region))));
} PERMISSIONS FULL;

-- Monthly count of dated claims about systems of one kind (e.g. "armour", "fpv_drone") inside a region.
DEFINE FUNCTION fn::sightings($kind: string, $region: record<place>) {
    LET $k = type::record("kind", ["system", $kind]);
    RETURN SELECT month, count() AS n, array::group(key) AS claims FROM (
            SELECT time::format(time.from, "%Y-%m") AS month, key FROM fn::claims_in_region($region)
            WHERE time != NONE AND ->about->system.kind CONTAINS $k)
        GROUP BY month ORDER BY month;
} PERMISSIONS FULL;

-- Every observation of a metric over time, with its range, side, claimants and claim (a number series).
DEFINE FUNCTION fn::series($metric: record<metric>) {
    RETURN SELECT time.from AS t, time.to AS t_to, value, low, high, qualifier, value_text, unit, side,
            claim AS id, claim.key AS claim, claim.status AS status, claim->asserted_by->actor.labels.en AS claimants
        FROM observation WHERE metric = $metric ORDER BY t, claim;
} PERMISSIONS FULL;

-- What counters a system, and what it counters (one step each way of the measure/countermeasure graph).
DEFINE FUNCTION fn::counters_of($system: record<system>) {
    RETURN {
        system: $system.labels.en,
        countered_by: (SELECT in AS system, in.labels.en AS name, first_observed.from AS since, lag_days, effect,
                rationale FROM counters WHERE out = $system ORDER BY since),
        counters: (SELECT out AS system, out.labels.en AS name, first_observed.from AS since, lag_days, effect,
                rationale FROM counters WHERE in = $system ORDER BY since)
    };
} PERMISSIONS FULL;

-- A measure, what countered it, and what countered those (two steps of the arms race).
DEFINE FUNCTION fn::counter_chain($system: record<system>) {
    RETURN SELECT in AS counter, in.labels.en AS counter_name, first_observed.from AS since, lag_days,
            (SELECT in AS counter, in.labels.en AS counter_name, first_observed.from AS since, lag_days
                FROM counters WHERE out = $parent.in ORDER BY since) AS countered_by
        FROM counters WHERE out = $system ORDER BY since;
} PERMISSIONS FULL;

-- The canonical claim behind a chain of extracted duplicates.
DEFINE FUNCTION fn::canonical($claim: record<claim>) {
    LET $next = (SELECT VALUE out FROM duplicates WHERE in = $claim LIMIT 1)[0];
    RETURN IF $next = NONE { $claim } ELSE { fn::canonical($next) };
} PERMISSIONS FULL;

-- Every claim-to-claim relation of a claim, in both directions.
DEFINE FUNCTION fn::relations($claim: record<claim>) {
    RETURN array::flatten([
        (SELECT "supports" AS rel, "to" AS dir, out AS other, out.text AS text, strength, rationale, independent FROM supports WHERE in = $claim),
        (SELECT "supports" AS rel, "from" AS dir, in AS other, in.text AS text, strength, rationale, independent FROM supports WHERE out = $claim),
        (SELECT "weakens" AS rel, "to" AS dir, out AS other, out.text AS text, strength, rationale FROM weakens WHERE in = $claim),
        (SELECT "weakens" AS rel, "from" AS dir, in AS other, in.text AS text, strength, rationale FROM weakens WHERE out = $claim),
        (SELECT "contradicts" AS rel, "to" AS dir, out AS other, out.text AS text, strength, rationale, resolution, state FROM contradicts WHERE in = $claim),
        (SELECT "contradicts" AS rel, "from" AS dir, in AS other, in.text AS text, strength, rationale, resolution, state FROM contradicts WHERE out = $claim),
        (SELECT "refines" AS rel, "to" AS dir, out AS other, out.text AS text, strength, rationale FROM refines WHERE in = $claim),
        (SELECT "refines" AS rel, "from" AS dir, in AS other, in.text AS text, strength, rationale FROM refines WHERE out = $claim),
        (SELECT "supersedes" AS rel, "to" AS dir, out AS other, out.text AS text, strength, rationale FROM supersedes WHERE in = $claim),
        (SELECT "supersedes" AS rel, "from" AS dir, in AS other, in.text AS text, strength, rationale FROM supersedes WHERE out = $claim),
        (SELECT "duplicates" AS rel, "to" AS dir, out AS other, out.text AS text, strength, rationale FROM duplicates WHERE in = $claim),
        (SELECT "duplicates" AS rel, "from" AS dir, in AS other, in.text AS text, strength, rationale FROM duplicates WHERE out = $claim)
    ]);
} PERMISSIONS FULL;

-- One claim with everything around it: claimants, sources (with quotes), what it is about, observations and relations.
DEFINE FUNCTION fn::claim($claim: record<claim>) {
    RETURN {
        claim: (SELECT * FROM ONLY $claim),
        claimants: (SELECT out AS actor, out.labels.en AS name, out.side AS side, role FROM asserted_by WHERE in = $claim),
        sources: (SELECT out AS source, out.title AS title, out.url AS url, out.publisher.labels.en AS publisher,
                out.origin AS origin, out.reliability AS reliability, out.published AS published, quote
            FROM cites WHERE in = $claim),
        about: (SELECT out AS target, out.labels.en AS name, role FROM about WHERE in = $claim),
        observations: (SELECT metric, value, low, high, qualifier, value_text, unit, side, time FROM observation WHERE claim = $claim),
        relations: fn::relations($claim),
        canonical: fn::canonical($claim)
    };
} PERMISSIONS FULL;

-- Full-text search over claims, best matches first, with the matching words highlighted.
DEFINE FUNCTION fn::search($q: string) {
    RETURN SELECT id, key, search::highlight("**", "**", 0) AS text, search::score(0) AS score, time.from AS t
        FROM claim WHERE text @0@ $q ORDER BY score DESC LIMIT 100;
} PERMISSIONS FULL;

-- A chapter's (or topic's) dated claims in order, outcome claims flagged: the self-running script of a diorama.
DEFINE FUNCTION fn::timeline($topic: record<topic>) {
    RETURN SELECT id, time.from AS t, time.to AS t_to, time.precision AS precision, key, kind, text,
            ext.outcome = true AS outcome, ->asserted_by->actor.labels.en AS claimants
        FROM claim WHERE topics CONTAINS $topic AND time != NONE ORDER BY t, key;
} PERMISSIONS FULL;
