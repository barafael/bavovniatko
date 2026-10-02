-- Example queries for the explorer's gallery, added to those in db/queries.md (see db/tools/webexport.py).
-- "-- ## Section" starts a section; a "-- comment" line titles the query below it.

-- ## Places through time

-- The history of a village: every claim about Robotyne, in time order
fn::place_history(place:robotyne);

-- Avdiivka and everything directly inside it (the coke plant, Zenit, the terrikon …)
fn::place_history(place:avdiivka);

-- Places inside Kursk Oblast that the knowledge base mentions
SELECT labels.en AS place, kind FROM fn::in_region(place:kursk_oblast) ORDER BY place;

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
SELECT in.labels.en AS counter, out.labels.en AS measure, lag_days, first_observed.from AS since
FROM counters WHERE lag_days != NONE ORDER BY lag_days LIMIT 20;

-- ## Claims and disputes

-- Full-text search with highlighting
fn::search("glide bomb Kharkiv");

-- One claim with its sources, claimants and relations
fn::claim(claim:c00_0063_04);

-- The strongest contradictions between claimants, with how they are explained
SELECT strength, in.text AS a, in->asserted_by->actor.labels.en AS a_by, out.text AS b,
       out->asserted_by->actor.labels.en AS b_by, resolution
FROM contradicts ORDER BY strength DESC LIMIT 25;

-- Which claimants are contradicted most often
SELECT name, count() AS contradictions FROM (
    SELECT VALUE array::flatten([in->asserted_by->actor.labels.en, out->asserted_by->actor.labels.en]) FROM contradicts
).flatten().map(|$n| { name: $n }) GROUP BY name ORDER BY contradictions DESC LIMIT 20;

-- ## Chapters

-- The win conditions of the Kursk chapter (claims flagged as the historical outcome)
SELECT key, text FROM claim WHERE topics CONTAINS topic:c14 AND ext.outcome = true ORDER BY key;

-- Operation Vivaldi as a dated script
fn::timeline(topic:c19);

-- Outcome claims per chapter
SELECT topics[0].code AS chapter, count() AS outcome_claims FROM claim WHERE ext.outcome = true GROUP BY chapter ORDER BY chapter;
