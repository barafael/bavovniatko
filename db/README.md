# bavovniatko knowledge base (SurrealDB)

The research base as a claim graph. It holds atomic claims with their evidence and claimants,
relations between claims (supports, weakens, contradicts, refines, supersedes, duplicates), places
with geometry, media links, and a derived game feed. It is the **source of truth**: the files in
`research/` are generated from it.

The design rationale is in [PLAN.md](PLAN.md). This page explains how to use it.

## Setup

```sh
db/dbctl.sh start            # SurrealDB 3.3.0 in Docker, SurrealKV (versioned) in db/data/, on 127.0.0.1:8047
python3 -m venv db/.venv && db/.venv/bin/pip install "surrealdb==2.0.0" pyyaml jsonschema requests warcio
db/.venv/bin/python db/tools/migrate.py                 # apply db/schema/NNNN_*.surql
db/.venv/bin/python db/tools/dump.py restore --ns bavovniatko --db kb   # rebuild from the committed dump
db/dbctl.sh sql              # interactive SurrealQL shell
```

`db/.env` holds the credentials and the ns/db names. It is gitignored. Create it with `SURREAL_USER`,
`SURREAL_PASS`, `SURREAL_PORT=8047`, `SURREAL_NS=bavovniatko` and `SURREAL_DB=kb`.

## What is where

| path | contents |
|---|---|
| `schema/` | Ordered, idempotent migrations. Never edit an applied one; add a new one. |
| `vocab/` | Curated vocabularies: `eras.yaml`, `topics.yaml`, `kinds.yaml`, `units.yaml`, `metrics.yaml`, `entities.yaml` (the entity registry) |
| `staging/` | Agent-produced input: `claims/NN.jsonl`, `relations/judged/*.jsonl`, `sim/*.yaml`, `sources/*.yaml` (new sources), and `entities/geocode.jsonl` (Nominatim cache). The briefs are `EXTRACTION.md`, `ENTITIES.md` and `RELATIONS.md`. |
| `checks/` | Integrity queries. Each returns violating ids and has a severity header. |
| `fixes/` | Idempotent data corrections, e.g. extraction errors found in review. Applied with `migrate.py --fixes`. |
| `templates/` | Fixed text for generated files (the glossary preamble) |
| `dump/` | **The committed data.** Deterministic JSONL per table, which rebuilds the database exactly together with `schema/`. |
| `tools/` | The Python tooling (below) |
| `data/` | Database files (gitignored) |

## Data model in one screen

- **Vocabularies:**
  - `era`: e1–e9, plus sub-phases e6a/b, e8a/b and e9a/b.
  - `topic`: t00–t08.
  - `kind`: open vocabularies, with ids like `kind:["place","settlement"]`.
  - `unit`, with conversion to a base unit.
  - `metric`.
- **Entities:** `source`, `actor` (outlets, officials, units, governments), `place` (with WGS84
  geometry), `event`, `system` (weapons, EW, software, techniques), `work` (games, simulators, map
  products, datasets), `term`, `media` (links only) and `snapshot` (offline backups).
- **Claims:** `claim` carries text, kind, epistemic status, time with precision, eras, topics and
  sides. Each claim has these links:
  - `observation`: its numbers, with metric, value or range, unit, time, side and place.
  - `passage`: the verbatim prose of the documents, linked to claims by `includes`.
  - `question`: open gaps.
  - `design_note`: game ideas.
- **Edges:**
  - Evidence and subjects: `cites` (claim → source), `asserted_by` (claim → actor), `about`
    (claim → entity).
  - Claim to claim: `supports`, `weakens`, `contradicts` (symmetric: query with
    `<->contradicts<->`), `refines`, `supersedes`, `duplicates`.
  - Structure: `counters` (system → system, the measure/countermeasure graph), `part_of`,
    `draws_on`, `illustrates`, `depicts`, `same_as`.
- **Game feed:** `sim_loadout`, `sim_param`, `sim_series` and `sim_terrain_profile`. They are
  derived from claims, and every row records `derived_from`, `method` and `reviewed`.
- **History:** `revision` keeps the prior state of audited records on every update or delete.
  Change feeds are kept for 365 days, and SurrealKV `VERSION` reads give point-in-time queries.

## Tools

| command | does |
|---|---|
| `migrate.py [--status] [--fixes]` | Apply pending schema migrations, or with `--fixes` the data corrections in `fixes/` (both tracked, run once) |
| `q.py "<SurrealQL>"` | Read-only query that prints JSON. Examples are in [queries.md](queries.md). |
| `smoke_test.py` | Apply the whole schema to a scratch database and exercise it (19 checks) |
| `import_base.py` | Phase 1. Deterministic import of vocab, sources and actors, glossary, passages, questions and design notes |
| `stage.py worksheet\|append\|check NN` | Phase 2. Numbered worksheets for extraction; validated append to staging; coverage check |
| `entities.py propose\|unmatched\|geocode` | Phase 3. Cluster mentions into proposals, find unresolved names, geocode places via Nominatim (cached) |
| `load.py entities\|metrics\|claims\|all` | Phase 4. Load the registry and staged claims (idempotent, content-hashed) |
| `relate.py candidates\|batches\|load\|status` | Phase 5. Rule-based candidate pairs, then agent judgments, then relation edges |
| `archive.py fetch\|verify\|status\|pack` | Phase 6. Private offline backup of every source and media URL (see below) |
| `render.py --check\|--write` | Phase 7. Regenerate `research/` from the database. `--check` is the round-trip gate. |
| `dump.py export\|restore\|verify` | Export the committed JSONL dump, rebuild from it, and verify an exact round trip |
| `check.py [-v]` | Run `checks/*.surql`. Exits 1 on any error-level violation. |
| `sim.py series\|check\|load\|export` | Phase 8. Rule-built time series; validate and load the agent-derived loadouts, parameters and terrain in `staging/sim/`; export `game-data/sim-YYYYMMDD.json` |
| `webexport.py [--check]` | Build the browser explorer's dataset from `db/dump/` (no database needed); see [web/README.md](../web/README.md) |

## Workflow for new research

1. **Research.** Write new findings as markdown the same way as `research/` (cited `[src:…]`), and
   add the new sources to `staging/sources/<name>.yaml` in `research/sources.yaml` format. Load them with
   `load.py sources`.
2. **Extract.** `stage.py worksheet`, then agents follow `staging/EXTRACTION.md`. `stage.py check`
   must be clean.
3. **Entities.** `entities.py unmatched`, then add new names to `vocab/entities.yaml`, then
   `entities.py geocode` for new places.
4. **Load and relate.** `load.py all`, then `relate.py candidates`, `relate.py batches`, agent
   judgments following `staging/RELATIONS.md`, `relate.py validate` and `relate.py load`.
   **Don't reload the initial staging files** over the database. The database is canonical, and `fixes/`
   records corrections made after the import.
5. **Check and publish.** `check.py`, `render.py --write`, `dump.py export`, then commit the dump,
   the regenerated `research/` and any vocab or schema changes **together**.

Edit claims in the database, not in `research/`. Those files carry a "generated" header.

## Offline archive

`archive.py fetch` copies every source and media URL into
`$BAVOVNIATKO_ARCHIVE` (default `~/bavovniatko-archive/`), **outside the repo and never committed**:
- Content-addressed bodies go in `objects/`.
- WARC records go in `warc/`.
- A log goes in `log.jsonl`.

Each attempt becomes a `snapshot` record with its HTTP status, MIME type, size, sha256 and path.
Blocked pages (403/429) fall back to the Wayback Machine's latest capture. `archive.py verify`
re-hashes everything, and `archive.py pack` makes a dated tarball.

Two options are off by default:
- `--wayback-submit` asks the Internet Archive to capture live pages, which is outward-facing.
- `--video` downloads with yt-dlp.

The archive is a private research backup: don't redistribute it or ship it in the game.

## Licences to respect

- **OSM-derived geometry:** ODbL, © OpenStreetMap contributors. The licence is recorded per place.
- **geoBoundaries:** CC BY.
- **DeepState, ISW and ACLED:** their data must not be bundled without permission (see
  `research/07-maps-ui-conventions.md`). Claims *about* them are fine.
- **The game feed** carries attribution for the OSM-derived terrain statistics.
