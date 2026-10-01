-- severity: error
-- low must not exceed high.
SELECT VALUE id FROM observation WHERE low != NONE AND high != NONE AND low > high;
