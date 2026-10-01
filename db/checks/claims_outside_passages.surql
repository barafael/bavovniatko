-- severity: warn
-- Every imported claim should be linked from the passage it came from (renderer and provenance rely on it).
SELECT VALUE id FROM claim WHERE count(<-includes) = 0;
