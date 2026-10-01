# Example queries

Run any of these with `db/.venv/bin/python db/tools/q.py "<query>"` (read-only), or in `db/dbctl.sh sql`.

## Claims

```surql
-- full-text search, restricted to an era, with who asserts it and on what evidence
SELECT key, text, ->asserted_by->actor.labels.en AS claimants, ->cites->source.id AS sources
FROM claim WHERE text @@ 'glide bomb' AND eras CONTAINS era:e9 LIMIT 10;

-- everything the knowledge base says about one system (and its variants)
SELECT key, kind, text FROM claim
WHERE ->about->system CONTAINS system:fibre_optic_fpv OR ->about->system.parent CONTAINS system:fibre_optic_fpv;

-- claims resting only on Ukrainian official claims (epistemic = claimed, claimant on the UA side)
SELECT key, text FROM claim WHERE epistemic = "claimed" AND ->asserted_by->actor.side CONTAINS "ua" LIMIT 20;
```

## Claim graph

```surql
-- what disputes a claim (contradicts is symmetric: traverse both ways, drop the claim itself)
SELECT key, array::distinct(<->contradicts<->claim.key)[WHERE $this != $parent.key] AS disputed_by
FROM claim WHERE key = '07-0030-17';

-- strongly corroborated claims: independent support with strength >= 0.7
SELECT key, text, count(<-(supports WHERE independent = true AND strength >= 0.7)) AS independent_support
FROM claim ORDER BY independent_support DESC LIMIT 10;

-- canonical claims only (drop extracted copies)
SELECT key, text FROM claim WHERE count(->duplicates) = 0 AND topics CONTAINS topic:t03 LIMIT 20;

-- every explained contradiction with its resolution
SELECT in.text AS a, out.text AS b, resolution FROM contradicts WHERE state = "explained";
```

## Numbers

```surql
-- a time series straight from observations (all claimants), in the metric's default unit where convertible
SELECT time.from AS month, value, low, high, claim.key AS claim, claim->asserted_by->actor.labels.en AS by
FROM observation WHERE metric = metric:kab_launches AND unit = unit:count_per_month ORDER BY month;

-- the game's cleaned series (one point per period)
SELECT points FROM sim_series:kab_launches__ru;

-- kill-zone depth claims per era
SELECT claim.eras.code AS eras, value, low, high, claim.text AS text FROM observation WHERE metric = metric:kill_zone_depth;
```

## Geography

```surql
-- claims located within 25 km of Pokrovsk
LET $p = (SELECT VALUE centroid FROM ONLY place:pokrovsk);
SELECT labels.en AS place, <-about<-claim.key AS claims FROM place WHERE centroid != NONE AND geo::distance(centroid, $p) < 25000;

-- places inside Zaporizhzhia Oblast
SELECT labels.en FROM place WHERE centroid != NONE AND centroid INSIDE (SELECT VALUE geometry FROM ONLY place:zaporizhzhia_oblast);
```

## Measure / countermeasure (the tech tree)

```surql
-- what counters fibre-optic FPVs, and what they counter
SELECT in.labels.en AS counter, out.labels.en AS measure, lag_days, first_observed.from AS since, effect
FROM counters WHERE in = system:fibre_optic_fpv OR out = system:fibre_optic_fpv;

-- two-step chains: measure -> counter -> counter-counter
SELECT labels.en AS measure, <-counters<-system.labels.en AS counters,
       <-counters<-system<-counters<-system.labels.en AS counter_counters
FROM system:fibre_optic_fpv;
```

## Game feed

```surql
SELECT era, side, system.labels.en AS system, availability FROM sim_loadout WHERE era = era:e7 AND side = "ua";
SELECT key, era, dist, unit.symbol AS unit FROM sim_param WHERE key = "kill_zone_depth" ORDER BY era;
```

## History

```surql
-- how a claim looked before it was corrected (audit trail)
SELECT at, event, before.text FROM revision WHERE target = claim:c03_0029_007 ORDER BY at;
```
