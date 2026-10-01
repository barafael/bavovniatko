-- 0004_claims: atomic claims, their structured figures, and the prose/meta records around them.

-- claims ---------------------------------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS claim SCHEMAFULL CHANGEFEED 365d INCLUDE ORIGINAL
  COMMENT "One atomic assertion about the world, with its claimant(s) (asserted_by) and evidence (cites)";
DEFINE FIELD IF NOT EXISTS key        ON claim TYPE string READONLY;          -- deterministic import key (file|anchor|text hash)
DEFINE FIELD IF NOT EXISTS text       ON claim TYPE string ASSERT string::len($value) >= 8 AND string::len($value) <= 800;
DEFINE FIELD IF NOT EXISTS kind       ON claim TYPE "fact" | "figure" | "event" | "assessment" | "forecast" | "definition";
DEFINE FIELD IF NOT EXISTS status     ON claim TYPE "active" | "disputed" | "superseded" | "retracted" DEFAULT "active";
DEFINE FIELD IF NOT EXISTS epistemic  ON claim TYPE "observed" | "reported" | "claimed" | "estimated" | "derived" | "computed";
DEFINE FIELD IF NOT EXISTS time       ON claim TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS eras       ON claim TYPE set<record<era>> REFERENCE ON DELETE REJECT DEFAULT [];
DEFINE FIELD IF NOT EXISTS topics     ON claim TYPE set<record<topic>> REFERENCE ON DELETE REJECT DEFAULT [];
DEFINE FIELD IF NOT EXISTS sides      ON claim TYPE set<"ua" | "ru" | "western" | "other"> DEFAULT [];  -- whose forces/actions it concerns
DEFINE FIELD IF NOT EXISTS geo        ON claim TYPE option<geometry<point | line | polygon | multipoint | multiline | multipolygon | collection>>;
DEFINE FIELD IF NOT EXISTS confidence ON claim TYPE option<float> ASSERT $value = NONE OR ($value >= 0 AND $value <= 1);
DEFINE FIELD IF NOT EXISTS confidence_note ON claim TYPE option<string>;
DEFINE FIELD IF NOT EXISTS anchor     ON claim TYPE option<{ file: string, section: string, order: int, local_id: option<string> }>;
DEFINE FIELD IF NOT EXISTS embedding  ON claim TYPE option<array<float>>;
DEFINE FIELD IF NOT EXISTS embedding_model ON claim TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov       ON claim TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext        ON claim TYPE object FLEXIBLE DEFAULT {};
DEFINE FIELD IF NOT EXISTS created_at ON claim DEFAULT time::now() READONLY;
DEFINE FIELD IF NOT EXISTS updated_at ON claim VALUE time::now();
DEFINE FIELD IF NOT EXISTS n_sources  ON claim COMPUTED count(->cites);
DEFINE INDEX IF NOT EXISTS claim_key    ON claim FIELDS key UNIQUE;
DEFINE INDEX IF NOT EXISTS claim_eras   ON claim FIELDS eras;
DEFINE INDEX IF NOT EXISTS claim_topics ON claim FIELDS topics;
DEFINE INDEX IF NOT EXISTS claim_status ON claim FIELDS status;
DEFINE INDEX IF NOT EXISTS claim_text   ON claim FIELDS text FULLTEXT ANALYZER en_text BM25 HIGHLIGHTS;

-- observations: the structured figure(s) behind a claim ---------------------------------------------------
DEFINE TABLE IF NOT EXISTS observation SCHEMAFULL CHANGEFEED 365d
  COMMENT "A number (or range) for a metric, at a time/place/side, as asserted by its claim";
DEFINE FIELD IF NOT EXISTS claim      ON observation TYPE record<claim> REFERENCE ON DELETE CASCADE;
DEFINE FIELD IF NOT EXISTS metric     ON observation TYPE record<metric> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS value      ON observation TYPE option<float>;
DEFINE FIELD IF NOT EXISTS low        ON observation TYPE option<float>;
DEFINE FIELD IF NOT EXISTS high       ON observation TYPE option<float>;
DEFINE FIELD IF NOT EXISTS value_text ON observation TYPE option<string>;    -- the figure as written ("12:1", "~20,000 (>32,000)")
DEFINE FIELD IF NOT EXISTS qualifier  ON observation TYPE "exact" | "approx" | "at_least" | "at_most" | "range" DEFAULT "exact";
DEFINE FIELD IF NOT EXISTS unit       ON observation TYPE record<unit> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS time       ON observation TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS side       ON observation TYPE option<"ua" | "ru" | "western" | "other">;
DEFINE FIELD IF NOT EXISTS place      ON observation TYPE option<record<place>>;
DEFINE FIELD IF NOT EXISTS method     ON observation TYPE "reported" | "estimated" | "derived" | "computed" DEFAULT "reported";
DEFINE FIELD IF NOT EXISTS note       ON observation TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov       ON observation TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext        ON observation TYPE object FLEXIBLE DEFAULT {};
DEFINE FIELD IF NOT EXISTS value_base ON observation COMPUTED IF value != NONE AND unit.to_base != NONE { value * unit.to_base } ELSE { NONE };
DEFINE INDEX IF NOT EXISTS obs_metric ON observation FIELDS metric;
DEFINE INDEX IF NOT EXISTS obs_claim  ON observation FIELDS claim;

-- passages: the narrative prose of the generated markdown, with [claim:…] markers -------------------
DEFINE TABLE IF NOT EXISTS passage SCHEMAFULL CHANGEFEED 365d
  COMMENT "A block of prose (or a table) in a topic document; the renderer concatenates passages in order";
DEFINE FIELD IF NOT EXISTS key      ON passage TYPE string READONLY;          -- "03/0042"
DEFINE FIELD IF NOT EXISTS topic    ON passage TYPE record<topic> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS section  ON passage TYPE string;                    -- heading path "4. Key figures"
DEFINE FIELD IF NOT EXISTS level    ON passage TYPE int DEFAULT 0;             -- heading level if this passage is a heading
DEFINE FIELD IF NOT EXISTS order    ON passage TYPE int;
DEFINE FIELD IF NOT EXISTS role     ON passage TYPE "heading" | "prose" | "list" | "table" | "rule" | "other";
DEFINE FIELD IF NOT EXISTS markdown ON passage TYPE string;
DEFINE FIELD IF NOT EXISTS prov     ON passage TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext      ON passage TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS passage_key   ON passage FIELDS key UNIQUE;
DEFINE INDEX IF NOT EXISTS passage_order ON passage FIELDS topic, order;

-- open questions / gaps ----------------------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS question SCHEMAFULL COMMENT "An open research gap and what was tried";
DEFINE FIELD IF NOT EXISTS key    ON question TYPE string READONLY;
DEFINE FIELD IF NOT EXISTS text   ON question TYPE string;
DEFINE FIELD IF NOT EXISTS tried  ON question TYPE option<string>;
DEFINE FIELD IF NOT EXISTS status ON question TYPE "open" | "resolved" | "dropped" DEFAULT "open";
DEFINE FIELD IF NOT EXISTS topics ON question TYPE set<record<topic>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS eras   ON question TYPE set<record<era>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS anchor ON question TYPE option<{ file: string, section: string, order: int, local_id: option<string> }>;
DEFINE FIELD IF NOT EXISTS prov   ON question TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext    ON question TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS question_key ON question FIELDS key UNIQUE;

-- design notes: game/sim ideas (not claims about the world) ---------------------------------------------
DEFINE TABLE IF NOT EXISTS design_note SCHEMAFULL COMMENT "Game/sim relevance notes; draw on claims via draws_on";
DEFINE FIELD IF NOT EXISTS key    ON design_note TYPE string READONLY;
DEFINE FIELD IF NOT EXISTS text   ON design_note TYPE string;
DEFINE FIELD IF NOT EXISTS topics ON design_note TYPE set<record<topic>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS eras   ON design_note TYPE set<record<era>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS anchor ON design_note TYPE option<{ file: string, section: string, order: int, local_id: option<string> }>;
DEFINE FIELD IF NOT EXISTS prov   ON design_note TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext    ON design_note TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS design_note_key ON design_note FIELDS key UNIQUE;
