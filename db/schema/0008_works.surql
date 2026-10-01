-- 0008_works: creative/technical works that claims are about but that are not war systems:
-- video games, board and professional wargames, training simulators, map products, datasets.

DEFINE TABLE IF NOT EXISTS work SCHEMAFULL COMMENT "Games, wargames, simulators, map products, datasets (topics 07, 08)";
DEFINE FIELD IF NOT EXISTS labels    ON work TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS aliases   ON work TYPE set<string> DEFAULT [];
DEFINE FIELD IF NOT EXISTS kind      ON work TYPE record<kind> ASSERT $value.domain = "work";
DEFINE FIELD IF NOT EXISTS makers    ON work TYPE array<record<actor>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS released  ON work TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS summary   ON work TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov      ON work TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext       ON work TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS work_name ON work FIELDS labels.en FULLTEXT ANALYZER name_text BM25;

DEFINE TABLE OVERWRITE about TYPE RELATION IN claim | question | design_note OUT place | event | system | actor | metric | work ENFORCED SCHEMAFULL
  COMMENT "claim -> entity it concerns";
DEFINE TABLE OVERWRITE depicts TYPE RELATION IN media OUT place | event | system | actor | work ENFORCED SCHEMAFULL;
