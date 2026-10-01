-- severity: error
-- An observation must say something: a value, a bound, or the figure as written.
SELECT VALUE id FROM observation WHERE value = NONE AND low = NONE AND high = NONE AND value_text = NONE;
