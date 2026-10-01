-- 0001_foundation: analyzers, migration log, open vocabularies ("kind"), audit trail.
--
-- Conventions used across all migrations:
--   * every table is SCHEMAFULL; the database is STRICT (tables must be defined before use)
--   * `labels`  = { en, uk?, uk_translit?, ru?, ru_translit? }   Ukrainian spellings are primary in `en`
--   * `time`    = { from, to?, precision }                          partial dates with explicit precision
--   * `prov`    = { by, method, run?, input?, at }                  who/what created or last changed a record
--   * `ext`     = FLEXIBLE object                                    escape hatch; promote to typed fields via migration
--   * open vocabularies live in the `kind` table, ids are [domain, slug], e.g. kind:["place", "settlement"]

DEFINE ANALYZER IF NOT EXISTS en_text TOKENIZERS blank, class, punct FILTERS lowercase, ascii, snowball(english);
DEFINE ANALYZER IF NOT EXISTS name_text TOKENIZERS blank, class, punct FILTERS lowercase, ascii, edgengram(2, 12);

-- migration log ----------------------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS migration SCHEMAFULL COMMENT "Applied schema migrations (written by tools/migrate.py)";
DEFINE FIELD IF NOT EXISTS sha256     ON migration TYPE string;
DEFINE FIELD IF NOT EXISTS applied_at ON migration TYPE datetime;

-- open vocabularies ------------------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS kind SCHEMAFULL
  COMMENT "Extensible vocabularies. id = [domain, slug]; domains: source, actor, place, event, system, media";
DEFINE FIELD IF NOT EXISTS domain      ON kind TYPE string VALUE record::id($this.id)[0];
DEFINE FIELD IF NOT EXISTS labels      ON kind TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS description ON kind TYPE option<string>;
DEFINE FIELD IF NOT EXISTS parent      ON kind TYPE option<record<kind>>;
DEFINE FIELD IF NOT EXISTS ext         ON kind TYPE object FLEXIBLE DEFAULT {};

-- audit trail ------------------------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS revision SCHEMAFULL
  COMMENT "Previous state of audited records on UPDATE/DELETE (filled by table events)";
DEFINE FIELD IF NOT EXISTS target ON revision TYPE record;
DEFINE FIELD IF NOT EXISTS event  ON revision TYPE "UPDATE" | "DELETE";
DEFINE FIELD IF NOT EXISTS before ON revision TYPE object FLEXIBLE;
DEFINE FIELD IF NOT EXISTS at     ON revision TYPE datetime;
DEFINE INDEX IF NOT EXISTS revision_target ON revision FIELDS target;
