-- severity: warn
-- A "figure" claim should carry at least one structured observation.
SELECT VALUE id FROM claim WHERE kind = "figure" AND count((SELECT VALUE id FROM observation WHERE claim = $parent.id)) = 0;
