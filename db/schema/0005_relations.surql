-- 0005_relations: the graph. All relations are ENFORCED (both ends must exist).

-- evidence & attribution --------------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS cites TYPE RELATION IN claim OUT source ENFORCED SCHEMAFULL
  COMMENT "claim -> source it rests on";
DEFINE FIELD IF NOT EXISTS locator ON cites TYPE option<string>;              -- page, §, table row
DEFINE FIELD IF NOT EXISTS quote   ON cites TYPE option<string> ASSERT $value = NONE OR string::len($value) <= 400;
DEFINE FIELD IF NOT EXISTS support ON cites TYPE "direct" | "secondary" | "background" DEFAULT "direct" INLINE;
DEFINE FIELD IF NOT EXISTS prov    ON cites TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS cites_unique ON cites FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS asserted_by TYPE RELATION IN claim OUT actor ENFORCED SCHEMAFULL
  COMMENT "claim -> actor who asserts it (distinct from the outlet that reported it)";
DEFINE FIELD IF NOT EXISTS role    ON asserted_by TYPE "claimant" | "estimator" | "reporter" | "analyst" DEFAULT "claimant" INLINE;
DEFINE FIELD IF NOT EXISTS said_at ON asserted_by TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS prov    ON asserted_by TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS asserted_by_unique ON asserted_by FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS about TYPE RELATION IN claim | question | design_note OUT place | event | system | actor | metric ENFORCED SCHEMAFULL
  COMMENT "claim -> entity it concerns";
DEFINE FIELD IF NOT EXISTS role ON about TYPE "subject" | "location" | "instrument" | "target" | "measure" | "mention" DEFAULT "subject" INLINE;
DEFINE FIELD IF NOT EXISTS prov ON about TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS about_unique ON about FIELDS in, out UNIQUE;

-- claim ↔ claim epistemics ------------------------------------------------------------------------
-- Shared fields: strength (0–1, inline for filtered traversal), rationale, method, prov.
DEFINE TABLE IF NOT EXISTS supports TYPE RELATION IN claim OUT claim ENFORCED SCHEMAFULL CHANGEFEED 365d
  COMMENT "A makes B more likely (corroboration)";
DEFINE FIELD IF NOT EXISTS strength    ON supports TYPE float INLINE ASSERT $value >= 0 AND $value <= 1;
DEFINE FIELD IF NOT EXISTS independent ON supports TYPE option<bool>;        -- different original source, not a relay
DEFINE FIELD IF NOT EXISTS rationale   ON supports TYPE string;
DEFINE FIELD IF NOT EXISTS method      ON supports TYPE "rule" | "agent" | "human";
DEFINE FIELD IF NOT EXISTS prov        ON supports TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS supports_unique ON supports FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS weakens TYPE RELATION IN claim OUT claim ENFORCED SCHEMAFULL CHANGEFEED 365d
  COMMENT "A makes B less likely without contradicting it outright";
DEFINE FIELD IF NOT EXISTS strength  ON weakens TYPE float INLINE ASSERT $value >= 0 AND $value <= 1;
DEFINE FIELD IF NOT EXISTS rationale ON weakens TYPE string;
DEFINE FIELD IF NOT EXISTS method    ON weakens TYPE "rule" | "agent" | "human";
DEFINE FIELD IF NOT EXISTS prov      ON weakens TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS weakens_unique ON weakens FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS contradicts TYPE RELATION IN claim OUT claim ENFORCED SCHEMAFULL CHANGEFEED 365d
  COMMENT "A and B cannot both be true as stated. Symmetric: stored once, query with <->contradicts<->";
DEFINE FIELD IF NOT EXISTS strength   ON contradicts TYPE float INLINE ASSERT $value >= 0 AND $value <= 1;
DEFINE FIELD IF NOT EXISTS rationale  ON contradicts TYPE string;
DEFINE FIELD IF NOT EXISTS resolution ON contradicts TYPE option<string>;     -- why they differ: units, scope, date, method
DEFINE FIELD IF NOT EXISTS state      ON contradicts TYPE "open" | "explained" | "resolved" DEFAULT "open" INLINE;
DEFINE FIELD IF NOT EXISTS method     ON contradicts TYPE "rule" | "agent" | "human";
DEFINE FIELD IF NOT EXISTS prov       ON contradicts TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS contradicts_unique ON contradicts FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS refines TYPE RELATION IN claim OUT claim ENFORCED SCHEMAFULL CHANGEFEED 365d
  COMMENT "A narrows, qualifies or corrects B";
DEFINE FIELD IF NOT EXISTS strength  ON refines TYPE float INLINE ASSERT $value >= 0 AND $value <= 1;
DEFINE FIELD IF NOT EXISTS rationale ON refines TYPE string;
DEFINE FIELD IF NOT EXISTS method    ON refines TYPE "rule" | "agent" | "human";
DEFINE FIELD IF NOT EXISTS prov      ON refines TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS refines_unique ON refines FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS supersedes TYPE RELATION IN claim OUT claim ENFORCED SCHEMAFULL CHANGEFEED 365d
  COMMENT "A replaces B (newer figure, updated count)";
DEFINE FIELD IF NOT EXISTS strength  ON supersedes TYPE float INLINE ASSERT $value >= 0 AND $value <= 1;
DEFINE FIELD IF NOT EXISTS rationale ON supersedes TYPE string;
DEFINE FIELD IF NOT EXISTS method    ON supersedes TYPE "rule" | "agent" | "human";
DEFINE FIELD IF NOT EXISTS prov      ON supersedes TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS supersedes_unique ON supersedes FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS duplicates TYPE RELATION IN claim OUT claim ENFORCED SCHEMAFULL CHANGEFEED 365d
  COMMENT "Same assertion extracted twice (in = the copy, out = the canonical claim)";
DEFINE FIELD IF NOT EXISTS strength  ON duplicates TYPE float INLINE ASSERT $value >= 0 AND $value <= 1;
DEFINE FIELD IF NOT EXISTS rationale ON duplicates TYPE string;
DEFINE FIELD IF NOT EXISTS method    ON duplicates TYPE "rule" | "agent" | "human";
DEFINE FIELD IF NOT EXISTS prov      ON duplicates TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS duplicates_unique ON duplicates FIELDS in, out UNIQUE;

-- structure & game ------------------------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS counters TYPE RELATION IN system OUT system ENFORCED SCHEMAFULL CHANGEFEED 365d
  COMMENT "Measure/countermeasure: in (the counter) defeats or degrades out (the measure)";
DEFINE FIELD IF NOT EXISTS first_observed ON counters TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS lag_days       ON counters TYPE option<int>;
DEFINE FIELD IF NOT EXISTS effect         ON counters TYPE option<string>;
DEFINE FIELD IF NOT EXISTS evidence       ON counters TYPE set<record<claim>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS rationale      ON counters TYPE string;
DEFINE FIELD IF NOT EXISTS prov           ON counters TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE INDEX IF NOT EXISTS counters_unique ON counters FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS part_of TYPE RELATION IN place | event | actor | system OUT place | event | actor | system ENFORCED SCHEMAFULL
  COMMENT "Many-to-many hierarchy (single parents also live in `parent` fields)";
DEFINE FIELD IF NOT EXISTS role ON part_of TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov ON part_of TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };

DEFINE TABLE IF NOT EXISTS includes TYPE RELATION IN passage OUT claim | question | design_note ENFORCED SCHEMAFULL
  COMMENT "passage -> the claims (or questions, notes) it states";
DEFINE FIELD IF NOT EXISTS order ON includes TYPE int DEFAULT 0;
DEFINE INDEX IF NOT EXISTS includes_unique ON includes FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS draws_on TYPE RELATION IN design_note | question OUT claim ENFORCED SCHEMAFULL
  COMMENT "design note / question -> claims it builds on";
DEFINE INDEX IF NOT EXISTS draws_on_unique ON draws_on FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS illustrates TYPE RELATION IN media OUT claim ENFORCED SCHEMAFULL;
DEFINE INDEX IF NOT EXISTS illustrates_unique ON illustrates FIELDS in, out UNIQUE;
DEFINE TABLE IF NOT EXISTS depicts TYPE RELATION IN media OUT place | event | system | actor ENFORCED SCHEMAFULL;
DEFINE INDEX IF NOT EXISTS depicts_unique ON depicts FIELDS in, out UNIQUE;

DEFINE TABLE IF NOT EXISTS same_as TYPE RELATION ENFORCED SCHEMAFULL
  COMMENT "Identity links between records, or from records to aliases";
DEFINE FIELD IF NOT EXISTS note ON same_as TYPE option<string>;
