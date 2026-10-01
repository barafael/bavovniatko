-- severity: error
-- No claim relates to itself.
SELECT VALUE id FROM (SELECT id, in, out FROM supports, weakens, contradicts, refines, supersedes, duplicates) WHERE in = out;
