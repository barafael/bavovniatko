-- The explorer's query library: DEFINE FUNCTIONs loaded into the in-browser database after the data
-- (see db/tools/webexport.py). They keep notebook queries short, e.g. `fn::place_history(place:robotyne)`.
-- Every function is read-only. Keep one function per block, with a comment line above it: the notebook lists
-- these comments as the library's documentation.

-- Claims about a place or any place directly inside it, in time order, with claimants and sources.
DEFINE FUNCTION fn::place_history($place: record<place>) {
    LET $places = array::union([$place], (SELECT VALUE id FROM place WHERE parent = $place));
    LET $claims = array::distinct(array::flatten((SELECT VALUE <-about<-claim FROM $places)));
    RETURN SELECT id, time.from AS t, time.precision AS precision, key, kind, epistemic, status, text,
            ->about->place.{id, name: labels.en} AS places, ->asserted_by->actor.{id, name: labels.en} AS claimants,
            ->cites->source.title AS sources
        FROM $claims ORDER BY t, key;
} PERMISSIONS FULL;

-- Places whose centroid lies inside a region's polygon (an oblast, raion or map box).
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
            claim AS id, claim.key AS claim, claim.status AS status, claim->asserted_by->actor.{id, name: labels.en} AS claimants
        FROM observation WHERE metric = $metric ORDER BY t, claim;
} PERMISSIONS FULL;

-- What counters a system, and what it counters (one step each way of the measure/countermeasure graph).
DEFINE FUNCTION fn::counters_of($system: record<system>) {
    RETURN {
        system: { id: $system, name: $system.labels.en },
        countered_by: (SELECT { id: in, name: in.labels.en } AS system, first_observed.from AS since, lag_days, effect,
                rationale, evidence FROM counters WHERE out = $system ORDER BY since),
        counters: (SELECT { id: out, name: out.labels.en } AS system, first_observed.from AS since, lag_days, effect,
                rationale, evidence FROM counters WHERE in = $system ORDER BY since)
    };
} PERMISSIONS FULL;

-- A measure, what countered it, and what countered those (two steps of the arms race).
DEFINE FUNCTION fn::counter_chain($system: record<system>) {
    RETURN SELECT { id: in, name: in.labels.en } AS counter, first_observed.from AS since, lag_days,
            (SELECT { id: in, name: in.labels.en } AS counter, first_observed.from AS since, lag_days
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
        observations: (SELECT metric, metric.labels.en AS metric_label, value, low, high, qualifier, value_text, unit, side, time
            FROM observation WHERE claim = $claim),
        relations: fn::relations($claim),
        canonical: fn::canonical($claim)
    };
} PERMISSIONS FULL;

-- Full-text search over claims, best matches first, with the matching words highlighted.
DEFINE FUNCTION fn::search($q: string) {
    RETURN SELECT id, key, search::highlight("**", "**", 0) AS text, search::score(0) AS score, time.from AS t
        FROM claim WHERE text @0@ $q ORDER BY score DESC LIMIT 100;
} PERMISSIONS FULL;

-- A dossier's (or topic's) dated claims in order, with the historical-outcome claims flagged.
DEFINE FUNCTION fn::timeline($topic: record<topic>) {
    RETURN SELECT id, time.from AS t, time.to AS t_to, time.precision AS precision, key, kind, text,
            ext.outcome = true AS outcome, ->asserted_by->actor.{id, name: labels.en} AS claimants
        FROM claim WHERE topics CONTAINS $topic AND time != NONE ORDER BY t, key;
} PERMISSIONS FULL;

-- The relation edges that touch some claims, as {from, to, rel, …}. One bulk pass per relation table rather than one
-- call per claim, so a broad seed (a place with two hundred claims) costs no more than a narrow one. An edge comes back
-- once for each of its ends that is in the set; the explorer's graph view keeps one of each pair.
--
-- Extracted duplicates are left out: two records saying the same thing is an artefact of the extraction, not an
-- argument, and fn::canonical already folds them together on the claim pages.
DEFINE FUNCTION fn::edges_from($claims: array) {
    LET $c = array::distinct($claims);
    RETURN array::flatten([
        (SELECT "supports" AS rel, in AS from, out AS to, strength, rationale, independent FROM supports WHERE in IN $c),
        (SELECT "supports" AS rel, in AS from, out AS to, strength, rationale, independent FROM supports WHERE out IN $c),
        (SELECT "weakens" AS rel, in AS from, out AS to, strength, rationale FROM weakens WHERE in IN $c),
        (SELECT "weakens" AS rel, in AS from, out AS to, strength, rationale FROM weakens WHERE out IN $c),
        (SELECT "contradicts" AS rel, in AS from, out AS to, strength, rationale, resolution, state FROM contradicts WHERE in IN $c),
        (SELECT "contradicts" AS rel, in AS from, out AS to, strength, rationale, resolution, state FROM contradicts WHERE out IN $c),
        (SELECT "refines" AS rel, in AS from, out AS to, strength, rationale FROM refines WHERE in IN $c),
        (SELECT "refines" AS rel, in AS from, out AS to, strength, rationale FROM refines WHERE out IN $c),
        (SELECT "supersedes" AS rel, in AS from, out AS to, strength, rationale FROM supersedes WHERE in IN $c),
        (SELECT "supersedes" AS rel, in AS from, out AS to, strength, rationale FROM supersedes WHERE out IN $c)
    ]);
} PERMISSIONS FULL;

-- The claims within $hops steps (1, 2 or 3) of some claims: the relation edges between them, and the claims they
-- touch. Most of the graph is unconnected (about 70% of claims have no relation at all), so the explorer's graph view
-- always starts from a seed: a claim, or any record whose claims become the seeds.
DEFINE FUNCTION fn::graph($seeds: array, $hops: int) {
    LET $s0 = array::distinct($seeds);
    LET $e1 = fn::edges_from($s0);
    LET $e2 = IF $hops > 1 { fn::edges_from(array::distinct($e1.map(|$e| $e.to))) } ELSE { [] };
    LET $e3 = IF $hops > 2 { fn::edges_from(array::distinct($e2.map(|$e| $e.to))) } ELSE { [] };
    -- array::append takes (array, value) rather than two arrays, so adding edges hop by hop nests them: [[], []].
    -- array::flatten is the one that concatenates.
    LET $edges = array::flatten([$e1, $e2, $e3]);
    LET $ids = array::distinct(array::flatten($edges.map(|$e| [$e.from, $e.to])));
    RETURN {
        edges: $edges,
        claims: (SELECT id, key, text, time.from AS t, kind FROM claim WHERE id IN $ids)
    };
} PERMISSIONS FULL;

-- The claims a record sits at the centre of, so any record can seed the graph: a claim's own canonical claim, a
-- place's or system's claims, a topic's, a metric's observations, or an actor's assertions.
DEFINE FUNCTION fn::seed_claims($r: any) {
    -- type::table returns a table, not a string, so it has to be cast before it can be compared.
    LET $t = <string>type::table($r);
    RETURN IF $t = "claim" { [fn::canonical($r)] }
        ELSE IF $t = "topic" { (SELECT VALUE id FROM claim WHERE topics CONTAINS $r) }
        ELSE IF $t = "metric" { (SELECT VALUE claim FROM observation WHERE metric = $r) }
        ELSE IF $t = "actor" { (SELECT VALUE in FROM asserted_by WHERE out = $r) }
        ELSE { (SELECT VALUE in FROM about WHERE out = $r) };
} PERMISSIONS FULL;

-- Counts for the explorer's front page: what the knowledge base holds, and how much of it is joined to another claim by
-- a relation. The connected count reads the relation tables instead of traversing out of every claim, because there are
-- a few thousand edges and ten thousand claims.
DEFINE FUNCTION fn::stats() {
    LET $ends = array::distinct(array::flatten([
        (SELECT VALUE in FROM supports), (SELECT VALUE out FROM supports),
        (SELECT VALUE in FROM weakens), (SELECT VALUE out FROM weakens),
        (SELECT VALUE in FROM contradicts), (SELECT VALUE out FROM contradicts),
        (SELECT VALUE in FROM refines), (SELECT VALUE out FROM refines),
        (SELECT VALUE in FROM supersedes), (SELECT VALUE out FROM supersedes)
    ]));
    LET $edges = count(SELECT id FROM supports) + count(SELECT id FROM weakens) + count(SELECT id FROM contradicts)
        + count(SELECT id FROM refines) + count(SELECT id FROM supersedes);
    RETURN {
        claims: count(SELECT id FROM claim),
        dated: count(SELECT id FROM claim WHERE time != NONE),
        observations: count(SELECT id FROM observation),
        relations: $edges,
        connected: array::len($ends),
        contradictions: count(SELECT id FROM contradicts),
        sources: count(SELECT id FROM source),
        cited: count(SELECT id FROM source WHERE array::len(<-cites) > 0),
        actors: count(SELECT id FROM actor),
        systems: count(SELECT id FROM system),
        places: count(SELECT id FROM place),
        counters: count(SELECT id FROM counters),
        questions: count(SELECT id FROM question WHERE status = "open"),
        dossiers: count(SELECT id FROM topic WHERE string::starts_with(code, "c") OR string::starts_with(code, "v"))
    };
} PERMISSIONS FULL;

-- Any record with what points at it: claims about it, claims it asserts or that cite it, its parts (records whose
-- parent it is), a kind's members, a system's measure/countermeasure links, and the names of the records its fields
-- refer to. The explorer's record pages use it.
DEFINE FUNCTION fn::record($r: record) {
    LET $record = (SELECT * FROM ONLY $r);
    RETURN {
        record: $record,
        names: object::values($record ?? {}).flatten().filter(|$v| type::is_record($v) AND $v != $r)
            .map(|$v| { id: $v, name: $v.labels.en ?? $v.title }),
        parts: (SELECT id, labels.en ?? title ?? key AS name FROM type::table(record::tb($r)) WHERE parent = $r ORDER BY name),
        members: IF record::tb($r) = "kind" {
            (SELECT id, labels.en ?? title ?? key AS name FROM type::table(record::id($r)[0]) WHERE kind = $r ORDER BY name)
        } ELSE { [] },
        about: (SELECT in AS id, in.key AS key, in.text AS text, in.time.from AS t, in.time.precision AS precision, role
            FROM $r<-about ORDER BY t, key),
        asserted: (SELECT in AS id, in.key AS key, in.text AS text, in.time.from AS t, in.time.precision AS precision, role
            FROM $r<-asserted_by ORDER BY t, key),
        cited_by: (SELECT in AS id, in.key AS key, in.text AS text, in.time.from AS t, in.time.precision AS precision, quote
            FROM $r<-cites ORDER BY t, key),
        counters: (SELECT { id: out, name: out.labels.en } AS system, first_observed.from AS since, lag_days, effect,
            rationale, evidence FROM $r->counters ORDER BY since),
        countered_by: (SELECT { id: in, name: in.labels.en } AS system, first_observed.from AS since, lag_days, effect,
            rationale, evidence FROM $r<-counters ORDER BY since)
    };
} PERMISSIONS FULL;
