-- severity: warn
-- Sources that no claim cites (normal only for background material).
SELECT VALUE id FROM source WHERE count(<-cites) = 0;
