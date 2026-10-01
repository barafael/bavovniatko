-- severity: error
-- Measure/countermeasure links must cite at least one claim.
SELECT VALUE id FROM counters WHERE count(evidence) = 0;
