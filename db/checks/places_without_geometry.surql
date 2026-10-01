-- severity: warn
-- Places referenced by claims but not yet geocoded.
SELECT VALUE id FROM place WHERE geometry = NONE AND count(<-about) > 0;
