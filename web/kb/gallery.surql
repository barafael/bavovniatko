-- Example queries for the explorer's gallery, added to those in db/queries.md (see db/tools/webexport.py).
-- "-- ## Section" starts a section; a "-- comment" line titles the query below it.

-- ## Places through time

-- The history of a village: every claim about Robotyne, in time order
fn::place_history(place:robotyne);

-- Avdiivka and everything directly inside it (the coke plant, Zenit, the terrikon …)
fn::place_history(place:avdiivka);

-- Places inside Kursk Oblast that the knowledge base mentions
SELECT {id: id, name: labels.en} AS place, kind FROM fn::in_region(place:kursk_oblast) ORDER BY place.name;

-- ## Sightings and series

-- Tank and armour sightings in Donetsk Oblast, by month
fn::sightings("armour", place:donetsk_oblast);

-- FPV-drone claims in Kharkiv Oblast, by month
fn::sightings("fpv_drone", place:kharkiv_oblast);

-- Share of casualties caused by drones, over time and by claimant
fn::series(metric:drone_share_of_casualties);

-- Shahed/Geran launches per month, every claimant
fn::series(metric:shahed_launches);

-- Glide-bomb (KAB) launches per month
fn::series(metric:kab_launches);

-- ## The arms race

-- What countered fibre-optic FPVs, and what countered those
fn::counter_chain(system:fibre_optic_fpv);

-- Measure and countermeasure around the Shahed
fn::counters_of(system:geran_2);

-- The quickest countermeasures: shortest lag from a measure to its counter
SELECT {id: in, name: in.labels.en} AS counter, {id: out, name: out.labels.en} AS measure, lag_days,
       first_observed.from AS since, evidence
FROM counters WHERE lag_days != NONE ORDER BY lag_days LIMIT 20;

-- ## Claims and disputes

-- Full-text search with highlighting
fn::search("glide bomb Kharkiv");

-- One claim with its sources, claimants and relations
fn::claim(claim:c00_0063_04);

-- The strongest contradictions between claimants, with how they are explained
SELECT strength, in.{id, text} AS a, in->asserted_by->actor.{id, name: labels.en} AS a_by, out.{id, text} AS b,
       out->asserted_by->actor.{id, name: labels.en} AS b_by, resolution
FROM contradicts ORDER BY strength DESC LIMIT 25;

-- Which claimants are contradicted most often
SELECT {id: actor, name: actor.labels.en} AS claimant, contradictions FROM (
    SELECT actor, count() AS contradictions FROM (
        SELECT VALUE array::flatten([in->asserted_by->actor, out->asserted_by->actor]) FROM contradicts
    ).flatten().map(|$a| { actor: $a }) GROUP BY actor
) ORDER BY contradictions DESC LIMIT 20;

-- ## Dossiers

-- The historical outcome of the Kursk incursion (claims flagged as outcome)
SELECT key, text FROM claim WHERE topics CONTAINS topic:c14 AND ext.outcome = true ORDER BY key;

-- Operation Vivaldi as a dated timeline
fn::timeline(topic:c19);

-- Outcome claims per dossier
SELECT {id: topics[0], name: topics[0].labels.en} AS dossier, count() AS outcome_claims FROM claim WHERE ext.outcome = true
GROUP BY dossier ORDER BY dossier;
