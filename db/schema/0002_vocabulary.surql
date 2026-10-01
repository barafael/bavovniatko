-- 0002_vocabulary: eras, topics, units and metrics (controlled vocabularies as records, not enums).

-- eras: e1..e9 plus prose sub-phases (e6a, e6b, e8a, e8b, e9a, e9b) as children -----------------
DEFINE TABLE IF NOT EXISTS era SCHEMAFULL COMMENT "War eras; ids are stable (era:e1 … era:e9, era:e6a …)";
DEFINE FIELD IF NOT EXISTS code    ON era TYPE string;                      -- "e1-invasion", "e6a"
DEFINE FIELD IF NOT EXISTS labels  ON era TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS time    ON era TYPE { from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" };
DEFINE FIELD IF NOT EXISTS parent  ON era TYPE option<record<era>> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS order   ON era TYPE int;
DEFINE FIELD IF NOT EXISTS summary ON era TYPE string;
DEFINE FIELD IF NOT EXISTS ext     ON era TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS era_code ON era FIELDS code UNIQUE;

-- topics: the research files 00–08 and finer tags ----------------------------------------------
DEFINE TABLE IF NOT EXISTS topic SCHEMAFULL COMMENT "Research topics (topic:t00 … topic:t08) and sub-topics";
DEFINE FIELD IF NOT EXISTS code        ON topic TYPE string;              -- "03" or "03.helicopters"
DEFINE FIELD IF NOT EXISTS labels      ON topic TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS file        ON topic TYPE option<string>;      -- research/03-fires-air.md
DEFINE FIELD IF NOT EXISTS parent      ON topic TYPE option<record<topic>> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS description ON topic TYPE option<string>;
DEFINE FIELD IF NOT EXISTS ext         ON topic TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS topic_code ON topic FIELDS code UNIQUE;

-- units ------------------------------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS unit SCHEMAFULL COMMENT "Units of measure; to_si converts a value to the dimension's base unit";
DEFINE FIELD IF NOT EXISTS symbol    ON unit TYPE string;                 -- "km²", "rounds/day"
DEFINE FIELD IF NOT EXISTS labels    ON unit TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS dimension ON unit TYPE string;                 -- "area", "count", "count/time", "ratio", "currency", ...
DEFINE FIELD IF NOT EXISTS base      ON unit TYPE option<record<unit>>;   -- the dimension's base unit
DEFINE FIELD IF NOT EXISTS to_base   ON unit TYPE option<float>;          -- multiply to convert into `base`
DEFINE FIELD IF NOT EXISTS ext       ON unit TYPE object FLEXIBLE DEFAULT {};

-- metrics: canonical measures behind observations --------------------------------------------------
DEFINE TABLE IF NOT EXISTS metric SCHEMAFULL COMMENT "Canonical measures (e.g. metric:kab_launches) that observations report";
DEFINE FIELD IF NOT EXISTS labels       ON metric TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS description  ON metric TYPE string;
DEFINE FIELD IF NOT EXISTS dimension    ON metric TYPE string;
DEFINE FIELD IF NOT EXISTS default_unit ON metric TYPE option<record<unit>> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS topics       ON metric TYPE set<record<topic>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS tags         ON metric TYPE set<string> DEFAULT [];
DEFINE FIELD IF NOT EXISTS ext          ON metric TYPE object FLEXIBLE DEFAULT {};
