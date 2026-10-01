-- 0007_term_fields: keep the glossary's display form and scripts, so glossary.md can be regenerated exactly.
DEFINE FIELD IF NOT EXISTS display  ON term TYPE string;              -- the term cell as written, e.g. "**bavovna** (бавовна)"
DEFINE FIELD IF NOT EXISTS cyrillic ON term TYPE option<string>;
DEFINE FIELD IF NOT EXISTS topics_raw ON term TYPE option<string>;    -- "00, 03" as written ("all" allowed)
