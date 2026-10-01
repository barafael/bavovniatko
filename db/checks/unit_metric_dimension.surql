-- severity: warn
-- The observation's unit dimension differs from its metric's dimension (often fine, e.g. count vs count/time; review).
SELECT VALUE id FROM observation WHERE unit.dimension != metric.dimension;
