-- severity: warn
-- Every chapter or vignette dossier needs at least one claim flagged as its historical outcome (the win condition).
SELECT VALUE id FROM topic WHERE (string::starts_with(code, "c") OR string::starts_with(code, "v"))
  AND count((SELECT id FROM claim WHERE topics CONTAINS $parent.id AND ext.outcome = true)) = 0;
