# Plan: the research base as a SurrealDB claim graph

Status: **built** on 2026-09-30. Where the build departed from this plan:
- **Container control:** `db/dbctl.sh` runs SurrealDB with `docker run`, because Docker Compose isn't installed.
- **Claim ids:** imported claims use ids derived from their staging key (`claim:c03_0004_01`) rather than
  ULIDs. They are stable because the passages are frozen. New claims may use ULIDs.
- **Passages:** passages keep the verbatim prose, so the documents round-trip exactly, and claims link to them
  via `includes`.
- **Data corrections:** corrections after the import live in `db/fixes/`, and the audit trail begins with them.
- **Place geometry:** OSM geometry is simplified (Douglas–Peucker, tolerance set per kind) so the dump stays
  small (14 MB).
- **Counts:**

  | records | count |
  |---|---|
  | claims | 3,706 |
  | observations | 3,143 |
  | claim relations | 1,233 |
  | measure/countermeasure links | 221 |
  | entities | 1,366 |
  | archived sources | 568 of 583 |

## Goal

Turn `research/` (9 topic files, 583 sources, about 1,900 inline citations) into a SurrealDB graph:

- **Atomic claims.** Each claim says one thing, cites its sources, names who asserts it (the claimant), and is
  scoped in time, era, place and topic.
- **Claim-to-claim relations.** A claim can strengthen, weaken, contradict, refine, supersede or duplicate another,
  and every such link carries a rationale.
- **Geography.** Places carry GeoJSON geometry, and claims can be linked to places or carry geometry of their own.
- **Multimedia as links and metadata.** Media is a URL plus metadata in the database, with a separate tool that
  makes a private offline backup of everything the database points to (sources and media).
- **A typed game feed.** The Bevy game reads era loadouts, the measure/countermeasure graph, terrain profiles and
  time series. Each value traces back to the claims it was derived from.

Decisions already made:
- **Knowledge base plus game feed:** the database serves both purposes.
- **The database becomes canonical after the import:** markdown and `sources.yaml` are generated from it.
- **Extraction is done by Claude Code agents:** they write JSONL staging files, and deterministic Python loaders
  import them.
- **Media is stored as links and metadata only,** with an offline backup that lives outside git.

## Target platform

- **SurrealDB 3.3.x**, the current stable line (3.3.0, 24 Sep 2026), pinned to an exact patch version in
  `db/compose.yaml`. The container runs `surrealdb/surrealdb:v3.3.x` with the SurrealKV storage engine, stored in
  `db/data/` (gitignored). SurrealKV supports `VERSION` time-travel reads; the memory backend dropped them in 3.3.
- **3.x features the schema relies on:**
  - `DEFINE TABLE … TYPE RELATION IN … OUT … ENFORCED`
  - `COMPUTED` fields
  - `REFERENCE … ON DELETE REJECT`
  - `TYPE … FLEXIBLE`
  - `INLINE` edge fields, for filtered traversals such as `->(contradicts WHERE strength > 0.7)`
  - `geometry<…>`
  - HNSW vector indexes and full-text analyzers
  - `DEFINE BUCKET`, reserved for later
  - Strict mode is set at the database level.
- **Tooling is Python 3.** The `surrealdb` SDK 2.0.0 is on PyPI, and phase 0 checks that it works against 3.3. The
  fallback is the HTTP `/sql` endpoint. Loaders are idempotent and resumable.
- **Licensing:** SurrealDB is licensed under BSL 1.1. Check that before *embedding* the engine in a shipped game.
  The default plan avoids the question: the game consumes an exported, versioned data file instead.
- Sources:
  [Release 3.3](https://surrealdb.com/releases/3.3), [Release 3.0](https://surrealdb.com/releases/3.0),
  [DEFINE FIELD](https://surrealdb.com/docs/surrealql/statements/define/field),
  [DEFINE BUCKET](https://surrealdb.com/docs/surrealql/statements/define/bucket).

## Future-proofing principles

1. **Stable ids that carry no changing meaning.**
   - Sources keep their existing slugs (`source:⟨rusi-2025-watling-third-year⟩`), and the old aliases become
     `same_as` edges.
   - Claims, media and snapshots get ULIDs.
   - Vocabulary records get readable slugs (`era:e6`, `metric:kab_launches_monthly`).
2. **Vocabularies live in tables, not enums.** Eras, topics, metrics, units, actor kinds and system kinds are
   records, so adding one needs no schema change. Literal enums are kept only for closed sets, such as a claim's
   epistemic status.
3. **Schema in ordered, idempotent migrations.** Files are named `db/schema/NNNN_name.surql` and use `DEFINE … IF
   NOT EXISTS`, or `OVERWRITE` where a change is intended. A `migration` table records what has been applied. Every
   table is `SCHEMAFULL`.
4. **An escape hatch on every table:** an `ext: object FLEXIBLE` field for experimental attributes. When an
   attribute proves useful, a migration promotes it to a typed field.
5. **Provenance on everything.** Every record has `prov`, recording who or what created it: an extraction run, an
   agent, a human, or an import. It also records the method, the input file and anchor, and `created_at` /
   `updated_at`. Claims are never hard-deleted. They get `status: retracted | superseded` instead, and `REFERENCE …
   ON DELETE REJECT` guards against dangling links.
6. **History in two layers.**
   - SurrealKV `VERSION` reads give point-in-time queries.
   - A `revision` table, filled by `DEFINE EVENT` on update, keeps a durable audit trail of claims and relations.
   - `CHANGEFEED` on the content tables lets the markdown renderer and game exporter work incrementally.
7. **A plain-text dump in git.** Deterministic, sorted JSONL per table goes in `db/dump/`. Together with the
   migrations it rebuilds the database exactly, gives git history reviewable diffs, and means the project never
   depends on an opaque data directory.
8. **Time with explicit precision.** A time value looks like `{from: datetime, to: option<datetime>, precision:
   "day"|"month"|"quarter"|"year"}`, which covers "May 2026", "Q1 2025", "2024" and ranges.
9. **Multilingual labels.** A `labels` object holds `{en, uk, uk_translit, ru_translit}`, and `aliases` is a set,
   so Ukrainian spellings stay primary.
10. **Embeddings are optional and replaceable.** An embedding field records its model name, so the corpus can be
    re-embedded later without a migration. No embedding model is required on day one.

## Data model

### Nodes

| table | what it holds | key fields |
|---|---|---|
| `source` | Anything cited: a report, article, dataset, map, game page, standard | `title, url, alt_urls, publisher→actor, authors→actor[], published(time), accessed, type, origin, reliability, verified, language, notes` |
| `claim` | One atomic statement about the world | `text` (one sentence, English, Ukrainian spellings), `kind` (`fact`, `figure`, `event`, `assessment`, `forecast`, `definition`), `status` (`active`, `disputed`, `superseded`, `retracted`), `epistemic` (`observed`, `reported`, `claimed`, `estimated`, `derived`, `computed`), `time`, `eras→era[]`, `topics→topic[]`, `geo: option<geometry>`, `confidence` (our assessment, 0–1, with `confidence_note`), `anchor` (topic file and section), `embedding?`, `prov` |
| `observation` | A structured figure behind a `figure` claim; the data behind the game's time series | `claim→claim, metric→metric, value?, low?, high?, qualifier` (`approx`, `at_least`, `at_most`, `range`), `unit→unit, time, side, place→place?, method` (`reported`, `computed`, `derived`) |
| `metric` | Canonical measures, e.g. KAB launches per month, kill-zone depth, main-belt spacing | `labels, description, dimension, default_unit→unit, side_applicable` |
| `unit` | Units with conversions | `symbol, ucum?, dimension, to_si` (factor) |
| `era` | e1–e9, plus prose sub-phases e6a/b, e8a/b, e9a/b as child records | `labels, from, to, parent→era?, summary` |
| `topic` | The 00–08 topics plus finer tags | `labels, parent→topic?` |
| `actor` | Every organisation, person, unit or outlet: claimants, publishers, authors, military units, governments | `labels, kind` (`person`, `outlet`, `think_tank`, `government_body`, `military_unit`, `company`, `ngo`, `platform`), `side` (`ua`, `ru`, `western`, `international`, `other`), `parent→actor?, echelon?, external_ids` (Wikidata, …) |
| `place` | A settlement, raion, oblast, river, forest, road or front sector, or a sample box | `labels, kind, geometry, parent→place?, external_ids` (OSM, GeoNames, KATOTTG, Wikidata), `geometry_source` |
| `event` | Battles, operations (Vivaldi, Spiderweb), truces, policy changes, the Starlink cutoff phases | `labels, kind, time, era→era, places→place[]` |
| `system` | Weapons, platforms, EW, software and doctrine-level techniques (FPV, Geran-3, UMPK, Kometa, Delta, anti-drone nets) | `labels, kind, sides, first_seen(time), parent→system?` (for variants) |
| `term` | Glossary entries | `term, cyrillic?, translit?, gloss, lang, topics` |
| `media` | A link to an image, video, map, dataset or document | `url, kind, title, publisher→actor?, licence, captured(time)?, geo?, description, duration_s?` |
| `snapshot` | One offline backup of a source or media URL | `target→source\|media, retrieved_at, http_status, mime, bytes, sha256, archive_path` (relative to the archive root), `warc_file?, wayback_url?` |
| `passage` | A section of prose in a generated markdown file, so narrative isn't lost when the md is generated | `topic, section_path, order, markdown` (with `[claim:…]` markers), `prov` |
| `question` | An open gap (from the "Open questions" sections) | `text, topics, eras, tried` (what was attempted), `status` (`open`, `resolved`) |
| `design_note` | "Game/sim relevance" bullets: mechanics ideas, not facts | `text, topics, eras` |
| `sim_*` | Game-feed tables (see below) | |

### Relations (all `TYPE RELATION … ENFORCED`)

**Evidence and attribution**

| relation | from → to | fields |
|---|---|---|
| `cites` | claim → source | `locator?` (page or §), `quote?` (a short excerpt, ≤ 300 chars), `support` (`direct`, `secondary`, `background`) |
| `asserted_by` | claim → actor | `role` (`claimant`, `estimator`, `reporter`), `said_at(time)?` |
| `about` | claim → place, event, system, actor, metric | `role` (`subject`, `location`, `instrument`, `target`, `measure`) |

`asserted_by` is kept separate from `cites`. "Syrskyi says" is a different fact from "Euromaidan reported it".

**Claim to claim**

These relations are separate tables, so traversals stay idiomatic (`->contradicts->claim`). Every one of them
carries `strength` (0–1, `INLINE`), `rationale`, `method` (`rule`, `agent`, `human`) and `prov`.

| relation | meaning | extra |
|---|---|---|
| `supports` | A makes B more likely (independent corroboration) | `independent: bool` (a different original source, not just a relay) |
| `weakens` | A makes B less likely without contradicting it outright | |
| `contradicts` | A and B cannot both be true as stated | Stored once and queried with `<->`. `resolution?` explains why they differ: units, scope, date, method |
| `refines` | A narrows or corrects B | |
| `supersedes` | A replaces B (a newer figure, an updated count) | |
| `duplicates` | The same claim, extracted twice | Used for deduplication |

**Structure and the game**

| relation | from → to | fields |
|---|---|---|
| `counters` | system → system | `first_observed(time), lag_days?, effectiveness?, evidence→claim[]`. This is the measure/countermeasure graph (02) and the game's tech tree |
| `part_of` | many-to-many hierarchies (event → event, place → place, actor → actor) | |
| `illustrates` | media → claim | |
| `depicts` | media → place, event or system | |
| `same_as` | any → any | Links external or alias identities, e.g. old source ids |

**Integrity checks.** Some rules can't be enforced at insert time, so they run as queries in `db/checks/*.surql`
on every load. Every `active` claim has at least one `cites`. Every `figure` claim has at least one `observation`.
No claim cites a source whose `verified` is `search-only` without a second source.

### Schema excerpt (illustrative)

```surql
DEFINE TABLE claim SCHEMAFULL CHANGEFEED 90d INCLUDE ORIGINAL;
DEFINE FIELD text       ON claim TYPE string ASSERT string::len($value) BETWEEN 10 AND 600;
DEFINE FIELD kind       ON claim TYPE "fact"|"figure"|"event"|"assessment"|"forecast"|"definition";
DEFINE FIELD status     ON claim TYPE "active"|"disputed"|"superseded"|"retracted" DEFAULT "active";
DEFINE FIELD epistemic  ON claim TYPE "observed"|"reported"|"claimed"|"estimated"|"derived"|"computed";
DEFINE FIELD time       ON claim TYPE option<object>;           -- {from, to?, precision}
DEFINE FIELD eras       ON claim TYPE set<record<era>> REFERENCE ON DELETE REJECT DEFAULT [];
DEFINE FIELD topics     ON claim TYPE set<record<topic>> REFERENCE ON DELETE REJECT DEFAULT [];
DEFINE FIELD geo        ON claim TYPE option<geometry<point|line|polygon|multipolygon|collection>>;
DEFINE FIELD confidence ON claim TYPE option<float> ASSERT $value = NONE OR $value BETWEEN 0 AND 1;
DEFINE FIELD anchor     ON claim TYPE option<object>;           -- {file, section, order}
DEFINE FIELD prov       ON claim TYPE object;                   -- {run, by, method, at}
DEFINE FIELD ext        ON claim TYPE object FLEXIBLE DEFAULT {};
DEFINE FIELD created_at ON claim VALUE time::now() READONLY;
DEFINE FIELD updated_at ON claim VALUE time::now();
DEFINE FIELD n_sources  ON claim COMPUTED count(->cites);
DEFINE INDEX claim_eras ON claim FIELDS eras;
DEFINE ANALYZER en_text TOKENIZERS blank, class FILTERS lowercase, snowball(english);
DEFINE INDEX claim_text ON claim FIELDS text FULLTEXT ANALYZER en_text BM25;

DEFINE TABLE contradicts TYPE RELATION IN claim OUT claim ENFORCED SCHEMAFULL;
DEFINE FIELD strength   ON contradicts TYPE float INLINE ASSERT $value BETWEEN 0 AND 1;
DEFINE FIELD rationale  ON contradicts TYPE string;
DEFINE FIELD resolution ON contradicts TYPE option<string>;
DEFINE FIELD method     ON contradicts TYPE "rule"|"agent"|"human";

DEFINE TABLE place SCHEMAFULL;
DEFINE FIELD geometry ON place TYPE option<geometry<point|polygon|multipolygon|line|multiline>>;
DEFINE INDEX place_osm ON place FIELDS external_ids.osm UNIQUE;
```
Phase 0 validates the exact syntax against 3.3, including `set<…>`, `FULLTEXT` and the `CHANGEFEED` options. The
excerpt shows the intent.

## Pipeline

Every phase can be resumed. Agents write only to `db/staging/`, loaders are the only writers to the database, and
every load ends with `db/checks/`.

### Phase 0: Infrastructure

`db/compose.yaml`, the migrations `0001_vocab`, `0002_core`, `0003_relations`, `0004_sim`, `db/tools/migrate.py`,
and a smoke test that creates, relates, traverses and runs a spatial query.
Decide whether a pinned `surreal` binary replaces Docker. Confirm the Python SDK works against 3.3.

### Phase 1: Deterministic import (no LLM)

| input | becomes |
|---|---|
| `research/sources/*.yaml` | `source`, with aliases turned into `same_as` |
| publishers and authors | `actor` |
| `README.md` era table and 00 §3 sub-phases | `era` |
| file stems | `topic` |
| `glossary.md` | `term` |
| "Open questions/gaps" sections | `question` |

Target counts: 583 sources, 9+6 eras, about 190 terms.

### Phase 2: Claim extraction (Claude Code agents)

One agent per topic file (9), run in batches so the weekly limit can't wreck a run. Each agent reads its file and
writes `db/staging/claims/NN.jsonl`. Each line is one claim:

```json
{"local_id":"03-4.12","text":"...","kind":"figure","epistemic":"claimed",
 "claimants":[{"name":"Ukrainian Air Force","kind":"government_body","side":"ua"}],
 "cites":[{"source":"kyivpost-2026-korshak-glide-bombs","locator":"§3","quote":"..."}],
 "time":{"from":"2026-06-01","to":"2026-06-30","precision":"month"},
 "eras":["e9"],"topics":["03"],
 "mentions":[{"name":"UMPK","type":"system"},{"name":"Kharkiv","type":"place"}],
 "observations":[{"metric":"kab_launches_monthly","value":8266,"unit":"count/month","side":"ru","method":"reported"}],
 "anchor":{"file":"03-fires-air.md","section":"4. Key figures","order":12},
 "passage_ref":"03/2/e9"}
```

- **Every table row and every cited sentence becomes one or more claims.** Game/sim relevance bullets go to
  `design_note`, and the section prose goes to `passage`, with claim markers substituted in.
- **No new research in this phase.** Agents may split and restate what the markdown says, and nothing else. That
  keeps the extraction auditable.
- **`validate_staging.py`** checks each file against a JSON Schema. Every source id must exist, era and topic ids
  must be valid, and every `[src:…]` in the file must be used by at least one claim, so coverage is complete.
- **Expected size:** about 2,500–3,500 claims.

### Phase 3: Entity resolution and geocoding

- **Collect and cluster mentions.** Mentions are clustered deterministically by normalised labels, with Ukrainian
  and Russian transliteration variants. An agent reviews ambiguous merges, and the result goes to
  `db/staging/entities/*.jsonl`.
- **Places:**
  - Settlements are matched to OSM through Nominatim. Its usage policy allows 1 request/s, and requests use the UA
    `bavovniatko-research/0.1` with **no personal data**.
  - Admin polygons come from geoBoundaries (CC BY).
  - The OSM sample boxes from 06 become `place` polygons of kind `sample_box`.
  - `geometry_source` and the licence are stored per place. OSM-derived geometry is ODbL, so its attribution is
    recorded.
- **Metrics and units:** a curated list of about 80–150 metrics and a units table, with conversions such as km²,
  rounds/day, count/month, % and USD.

### Phase 4: Load

`load.py` upserts vocabularies and entities first, then claims with their `cites`, `asserted_by`, `about` and
`observation` records, then passages, design notes and questions.

During import development, a claim's id is derived from `hash(file, anchor, normalised text)`, so re-runs don't
create duplicates. The ids are frozen once the database becomes canonical.

### Phase 5: Relations between claims

1. **Rule-based candidates** (no LLM):
   - Same `metric` with overlapping time, place and side, where the values agree: a `supports` candidate.
     Disjoint ranges: a `contradicts` candidate.
   - Newer observations of the same series: `supersedes`.
   - Identical normalised text: `duplicates`.
   - The README's "Cross-topic notes" are seeds: each note names a known conflict or reconciliation.
2. **Entity-overlap candidates:** claims sharing an event, system or place within the same era.
   Embeddings/HNSW are optional here, and only worth adding if recall is poor.
3. **An agent judges each candidate pair** in batches of about 40, writing `db/staging/relations/*.jsonl`
   (`type | none`, `strength`, `rationale`, `resolution`). Rule-derived contradictions get a rationale too, such as
   "different units: depth vs area".
4. **Load, then run checks.** A `contradictions.md` report is generated from `<->contradicts<->`.

### Phase 6: Media and offline backup

- **Media records:** kinds are image, video, map, dataset and document. They come from the sources of type
  `map`, `dataset`, `video` or `game`, and from media URLs the agents find inside fetched pages. Records are
  links plus metadata only, and they are linked to claims through `illustrates` and `depicts`.
- **`db/tools/archive.py`** makes a private offline backup of everything the database points to, both `source`
  and `media`:
  - **Where:** into `$BAVOVNIATKO_ARCHIVE`, by default `~/bavovniatko-archive/`, **outside the repo, never
    committed**.
  - **How:** pages go into WARC files (`warcio`) plus a readable HTML/PDF copy, stored in content-addressed
    storage (`sha256/ab/cd…`). Datasets are downloaded as files.
  - **Video** needs an explicit `--video` flag and uses `yt-dlp`.
  - **Politeness:** rate-limited per host, with a generic UA and no personal data.
  - **Resumable:** it skips any URL that already has a fresh snapshot.
  - Each fetch becomes a `snapshot` record: status, mime, size, sha256 and archive path.
- **Checking a backup:** `archive.py verify` re-hashes the archive against the `snapshot` records, and
  `archive.py pack` makes a dated tarball for cold storage.
- **Optional:** `archive.py wayback` submits URLs to the Internet Archive's Save Page Now, which records a public
  `wayback_url`. It is off by default because it is an outward-facing action.
- **Copyright:** the archive is a private research backup and is not redistributed. The game ships no archived
  content.

### Phase 7: Generated markdown, and the switch to the database as canonical

- **`render.py`** rebuilds `research/NN-*.md`, `sources.yaml` and `glossary.md` from the `passage`, `claim`,
  `observation`, `source` and `term` records.
  - Figure tables come from `observation`, and contested figures from `contradicts`.
  - Each topic gets a new "Contradictions" section.
  - Generated files start with a "generated — do not edit" header.
- **Round-trip gate** before the switch: every current citation, table row and gap must appear in the generated
  output. `diff`-based review, and a sample of 50 claims read by a person.
- **After the switch:**
  - New research goes through `db/staging/` and the loaders, with no hand edits to the md.
  - `export_dump.py` writes `db/dump/*.jsonl`, sorted and deterministic, for git.
  - A commit includes the dump, the regenerated md and the migrations together.
  - `merge_sources.py` is retired.

### Phase 8: Game feed

The `sim_*` tables are *derived and reviewed*, never hand-typed. Each row carries `derived_from: set<record<claim>>`,
`method`, `reviewed: bool` and `valid_for: set<record<era>>`:

| table | contents |
|---|---|
| `sim_loadout` | Per era, side and system: availability and a rough quantity class. Built from the 00 first-delivery table and 03 |
| `sim_counter_graph` | A view over `counters`: effectiveness decay, lag and unlock conditions (02) |
| `sim_param` | Scalar and distribution parameters: kill-zone depth by era, detection → strike latency, interception rates, shell ratios |
| `sim_series` | Time series: KABs/month, Shaheds, territory km²/month |
| `sim_terrain_profile` | Belt spacing, orientation and width distributions, field sizes and road densities per sample box (06) |

`export_sim.py` writes a versioned, checksummed `game-data/sim-YYYYMMDD.json` for Bevy to load, as JSON or RON.
Embedding SurrealDB in the game is possible later, after the licence check.

## Layout

```
db/
  PLAN.md             this file
  compose.yaml        pinned SurrealDB 3.3.x, SurrealKV at ./data
  schema/             NNNN_*.surql migrations
  checks/             integrity queries, run after every load
  tools/              migrate, import_sources, validate_staging, load, relate_candidates,
                      archive, render, export_dump, export_sim   (Python)
  staging/            agent output (JSONL): claims/, entities/, relations/
  dump/               committed deterministic JSONL per table
  data/               gitignored database files
```

## Cost and risk

- **Agent effort.** Extraction takes 9 agents; relation judging takes roughly 30–80 batch calls, depending on how
  many candidates there are. The weekly limit already interrupted pass 2, so every agent phase is batched,
  resumable from staging, and can pause between batches.
- **Loss of prose quality when md becomes generated.** The mitigation is `passage` records, which keep the narrative
  verbatim with claim markers.
- **Fragmentation into overly fine claims.** The mitigation is an extraction rule of "one claimant, one assertion,
  one time scope", plus deduplication through `duplicates`.
- **Schema churn in SurrealDB 3.x.** The mitigation is an exact version pin, migrations, a JSONL dump that doesn't
  depend on the engine's storage format, and a smoke test on every upgrade.
- **Licences.** ODbL applies to OSM-derived geometry, CC BY to geoBoundaries, and there are restrictions on
  DeepState, ISW and ACLED data. The licence is recorded per record, and `export_sim.py` refuses records with
  non-shippable licences.

## Verification

- `migrate.py` applies the migrations to an empty database twice, and the second run changes nothing.
- After each load, `db/checks/*.surql` returns zero violations: orphan claims, figure claims without observations,
  dangling references, and unknown eras or topics.
- The counts match: 583 sources, every `[src:…]` citation covered, and claim counts per topic in the expected range.
- Spot queries work:
  - `SELECT * FROM claim WHERE eras CONTAINS era:e9 AND ->cites->source.origin CONTAINS "ukrainian"`
  - `SELECT <->contradicts<->claim FROM claim:…`
  - A `geo::distance` or `geo::contains` query on places.
  - The glide-bomb series through `observation`.
- The round-trip render gate passes before the switch.
- `archive.py verify` reports no hash mismatches.
- `export_sim.py` output validates against its JSON Schema.

## Order of work

0 → 1 → 2 (in batches) → 3 → 4 → 5 → 6 (it can run in parallel from phase 1 on, because it needs only
`source`/`media` URLs) → 7 → 8.
