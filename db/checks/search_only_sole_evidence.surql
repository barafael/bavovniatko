-- severity: warn
-- Claims whose only evidence is a source we could not fetch (verified = search-only).
SELECT VALUE id FROM claim WHERE count(->cites) > 0 AND array::all(->cites->source.verified, |$v| $v = "search-only");
