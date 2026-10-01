-- 0003_entities: the things claims are about or come from.

-- sources: everything cited ----------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS source SCHEMAFULL CHANGEFEED 365d
  COMMENT "Cited material. ids keep the research slugs (source:⟨rusi-2025-watling-third-year⟩)";
DEFINE FIELD IF NOT EXISTS title         ON source TYPE string;
DEFINE FIELD IF NOT EXISTS url           ON source TYPE string;
DEFINE FIELD IF NOT EXISTS alt_urls      ON source TYPE set<string> DEFAULT [];
DEFINE FIELD IF NOT EXISTS publisher     ON source TYPE option<record<actor>> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS authors       ON source TYPE array<record<actor>> REFERENCE ON DELETE REJECT DEFAULT [];
DEFINE FIELD IF NOT EXISTS published     ON source TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS published_raw ON source TYPE option<string>;
DEFINE FIELD IF NOT EXISTS accessed      ON source TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS type          ON source TYPE record<kind> ASSERT $value.domain = "source";
DEFINE FIELD IF NOT EXISTS origin        ON source TYPE "ukrainian" | "western" | "international" | "other";
DEFINE FIELD IF NOT EXISTS reliability   ON source TYPE "high" | "medium" | "low";
DEFINE FIELD IF NOT EXISTS verified      ON source TYPE "fetched" | "search-only";
DEFINE FIELD IF NOT EXISTS language      ON source TYPE string DEFAULT "en";
DEFINE FIELD IF NOT EXISTS eras          ON source TYPE set<record<era>> REFERENCE ON DELETE REJECT DEFAULT [];
DEFINE FIELD IF NOT EXISTS topics        ON source TYPE set<record<topic>> REFERENCE ON DELETE REJECT DEFAULT [];
DEFINE FIELD IF NOT EXISTS found_in      ON source TYPE set<record<topic>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS legacy_ids    ON source TYPE set<string> DEFAULT [];   -- aliases used in research/*.md
DEFINE FIELD IF NOT EXISTS notes         ON source TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov          ON source TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext           ON source TYPE object FLEXIBLE DEFAULT {};
DEFINE FIELD IF NOT EXISTS created_at    ON source DEFAULT time::now() READONLY;
DEFINE FIELD IF NOT EXISTS updated_at    ON source VALUE time::now();
DEFINE INDEX IF NOT EXISTS source_url     ON source FIELDS url;
DEFINE INDEX IF NOT EXISTS source_legacy  ON source FIELDS legacy_ids;
DEFINE INDEX IF NOT EXISTS source_title   ON source FIELDS title FULLTEXT ANALYZER en_text BM25;

-- actors: people, outlets, institutions, military units, governments -------------------------------
DEFINE TABLE IF NOT EXISTS actor SCHEMAFULL COMMENT "Claimants, publishers, authors, units, governments";
DEFINE FIELD IF NOT EXISTS labels       ON actor TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS aliases      ON actor TYPE set<string> DEFAULT [];
DEFINE FIELD IF NOT EXISTS kind         ON actor TYPE record<kind> ASSERT $value.domain = "actor";
DEFINE FIELD IF NOT EXISTS side         ON actor TYPE option<"ua" | "ru" | "western" | "international" | "other">;
DEFINE FIELD IF NOT EXISTS parent       ON actor TYPE option<record<actor>>;
DEFINE FIELD IF NOT EXISTS echelon      ON actor TYPE option<string>;            -- military units: brigade, corps …
DEFINE FIELD IF NOT EXISTS external_ids ON actor TYPE object FLEXIBLE DEFAULT {}; -- { wikidata, url, … }
DEFINE FIELD IF NOT EXISTS notes        ON actor TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov         ON actor TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext          ON actor TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS actor_name   ON actor FIELDS labels.en FULLTEXT ANALYZER name_text BM25;

-- places: settlements, admin areas, rivers, forests, roads, sectors, sample boxes -------------------
DEFINE TABLE IF NOT EXISTS place SCHEMAFULL COMMENT "Geography; geometry is WGS84 (lon, lat)";
DEFINE FIELD IF NOT EXISTS labels           ON place TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS aliases          ON place TYPE set<string> DEFAULT [];
DEFINE FIELD IF NOT EXISTS kind             ON place TYPE record<kind> ASSERT $value.domain = "place";
DEFINE FIELD IF NOT EXISTS geometry         ON place TYPE option<geometry<point | line | polygon | multipoint | multiline | multipolygon | collection>>;
DEFINE FIELD IF NOT EXISTS centroid         ON place TYPE option<geometry<point>>;
DEFINE FIELD IF NOT EXISTS parent           ON place TYPE option<record<place>>;
DEFINE FIELD IF NOT EXISTS external_ids     ON place TYPE object FLEXIBLE DEFAULT {}; -- { osm, geonames, katottg, wikidata }
DEFINE FIELD IF NOT EXISTS geometry_source  ON place TYPE option<string>;
DEFINE FIELD IF NOT EXISTS geometry_licence ON place TYPE option<string>;
DEFINE FIELD IF NOT EXISTS notes            ON place TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov             ON place TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext              ON place TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS place_name       ON place FIELDS labels.en FULLTEXT ANALYZER name_text BM25;

-- events: battles, operations, truces, policy changes ------------------------------------------------
DEFINE TABLE IF NOT EXISTS event SCHEMAFULL COMMENT "Battles, operations, truces, decisions, technical cut-offs";
DEFINE FIELD IF NOT EXISTS labels  ON event TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS aliases ON event TYPE set<string> DEFAULT [];
DEFINE FIELD IF NOT EXISTS kind    ON event TYPE record<kind> ASSERT $value.domain = "event";
DEFINE FIELD IF NOT EXISTS time    ON event TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS eras    ON event TYPE set<record<era>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS places  ON event TYPE set<record<place>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS summary ON event TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov    ON event TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext     ON event TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS event_name ON event FIELDS labels.en FULLTEXT ANALYZER name_text BM25;

-- systems: weapons, platforms, EW, software, techniques ------------------------------------------------
DEFINE TABLE IF NOT EXISTS system SCHEMAFULL COMMENT "Weapons, platforms, EW, software and techniques (the game's tech nodes)";
DEFINE FIELD IF NOT EXISTS labels     ON system TYPE { en: string, uk: option<string>, uk_translit: option<string>, ru: option<string>, ru_translit: option<string> };
DEFINE FIELD IF NOT EXISTS aliases    ON system TYPE set<string> DEFAULT [];
DEFINE FIELD IF NOT EXISTS kind       ON system TYPE record<kind> ASSERT $value.domain = "system";
DEFINE FIELD IF NOT EXISTS sides      ON system TYPE set<"ua" | "ru" | "western" | "other"> DEFAULT [];
DEFINE FIELD IF NOT EXISTS first_seen ON system TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS eras       ON system TYPE set<record<era>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS parent     ON system TYPE option<record<system>>;      -- variants: Geran-3 → Shahed/Geran
DEFINE FIELD IF NOT EXISTS summary    ON system TYPE option<string>;
DEFINE FIELD IF NOT EXISTS specs      ON system TYPE object FLEXIBLE DEFAULT {};   -- loose; firm numbers live in observations
DEFINE FIELD IF NOT EXISTS prov       ON system TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext        ON system TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS system_name ON system FIELDS labels.en FULLTEXT ANALYZER name_text BM25;

-- glossary terms ---------------------------------------------------------------------------------
DEFINE TABLE IF NOT EXISTS term SCHEMAFULL COMMENT "Glossary (frontline slang, Russian terms as reported, systems, map terms)";
DEFINE FIELD IF NOT EXISTS term     ON term TYPE string;
DEFINE FIELD IF NOT EXISTS gloss    ON term TYPE string;
DEFINE FIELD IF NOT EXISTS lang     ON term TYPE "uk" | "ru" | "en" | "mixed";
DEFINE FIELD IF NOT EXISTS group    ON term TYPE string;
DEFINE FIELD IF NOT EXISTS order    ON term TYPE int;
DEFINE FIELD IF NOT EXISTS verified ON term TYPE bool DEFAULT true;
DEFINE FIELD IF NOT EXISTS topics   ON term TYPE set<record<topic>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS prov     ON term TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext      ON term TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS term_text ON term FIELDS term FULLTEXT ANALYZER name_text BM25;

-- media: links + metadata only (files live in the private offline archive, see snapshot) ------------
DEFINE TABLE IF NOT EXISTS media SCHEMAFULL COMMENT "Images, video, maps, datasets, documents — URL and metadata only";
DEFINE FIELD IF NOT EXISTS url         ON media TYPE string;
DEFINE FIELD IF NOT EXISTS kind        ON media TYPE record<kind> ASSERT $value.domain = "media";
DEFINE FIELD IF NOT EXISTS title       ON media TYPE option<string>;
DEFINE FIELD IF NOT EXISTS publisher   ON media TYPE option<record<actor>>;
DEFINE FIELD IF NOT EXISTS found_via   ON media TYPE option<record<source>>;
DEFINE FIELD IF NOT EXISTS licence     ON media TYPE option<string>;
DEFINE FIELD IF NOT EXISTS captured    ON media TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS geo         ON media TYPE option<geometry<point | line | polygon | multipoint | multiline | multipolygon | collection>>;
DEFINE FIELD IF NOT EXISTS description ON media TYPE option<string>;
DEFINE FIELD IF NOT EXISTS duration_s  ON media TYPE option<float>;
DEFINE FIELD IF NOT EXISTS prov        ON media TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext         ON media TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS media_url   ON media FIELDS url UNIQUE;

-- snapshots: one offline backup of a source or media URL -----------------------------------------------
DEFINE TABLE IF NOT EXISTS snapshot SCHEMAFULL COMMENT "Private offline backup of a URL (see tools/archive.py); files live outside git";
DEFINE FIELD IF NOT EXISTS target       ON snapshot TYPE record<source | media> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS url          ON snapshot TYPE string;
DEFINE FIELD IF NOT EXISTS final_url    ON snapshot TYPE option<string>;
DEFINE FIELD IF NOT EXISTS retrieved_at ON snapshot TYPE datetime;
DEFINE FIELD IF NOT EXISTS http_status  ON snapshot TYPE option<int>;
DEFINE FIELD IF NOT EXISTS ok           ON snapshot TYPE bool;
DEFINE FIELD IF NOT EXISTS mime         ON snapshot TYPE option<string>;
DEFINE FIELD IF NOT EXISTS bytes        ON snapshot TYPE option<int>;
DEFINE FIELD IF NOT EXISTS sha256       ON snapshot TYPE option<string>;
DEFINE FIELD IF NOT EXISTS archive_path ON snapshot TYPE option<string>;   -- relative to $BAVOVNIATKO_ARCHIVE
DEFINE FIELD IF NOT EXISTS warc_file    ON snapshot TYPE option<string>;
DEFINE FIELD IF NOT EXISTS wayback_url  ON snapshot TYPE option<string>;
DEFINE FIELD IF NOT EXISTS via          ON snapshot TYPE option<string>;   -- direct | wayback | curl …
DEFINE FIELD IF NOT EXISTS error        ON snapshot TYPE option<string>;
DEFINE FIELD IF NOT EXISTS ext          ON snapshot TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS snapshot_target ON snapshot FIELDS target;
DEFINE INDEX IF NOT EXISTS snapshot_sha    ON snapshot FIELDS sha256;
