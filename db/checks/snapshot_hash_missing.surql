-- severity: error
-- A successful snapshot must record its hash and archive path.
SELECT VALUE id FROM snapshot WHERE ok = true AND (sha256 = NONE OR archive_path = NONE);
