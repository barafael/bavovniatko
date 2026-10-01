-- severity: error
-- Every active claim rests on at least one source.
SELECT VALUE id FROM claim WHERE status IN ["active", "disputed"] AND count(->cites) = 0;
