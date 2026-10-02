# The knowledge-base explorer

A static website for browsing and querying the bavovniatko knowledge base. The whole database runs **in the
visitor's browser**: SurrealDB 3.3 is compiled to WebAssembly, runs in memory in a Web Worker, and loads the dataset
on every visit (about 10 s). There is no server, and the database is read-only.

```
db/dump/*.jsonl ──db/tools/webexport.py──▶ crates/app/public/data/   the dataset (SurrealQL files + manifest)
web/kb/functions.surql, gallery.surql ──┘                             the fn:: query library and example queries
crates/kbcore    database core: open, load the manifest, seal read-only, query → JSON (native + wasm)
crates/worker    Web Worker hosting kbcore; JSON messages to and from the app
crates/app       Leptos UI (client-side): notebook, chapters, map, arms race, numbers, disputes, claim pages
bridge/          npm package bundling CodeMirror 6 + @surrealdb/codemirror and MapLibre GL → crates/app/bridge/
crates/kbcheck   native check: loads the dataset exactly as the browser does, runs the gallery and write probes
```

## Views

All views are hash routes, so every state is a shareable link.

- **Notebook** (`#/notebook?q=…`): a SurrealQL editor with highlighting and autocomplete, and an example
  gallery. Results show as tables (record ids link to their views) or JSON.
- **Chapters** (`#/chapters/topic:c14`): each chapter's outcome claims, which are its win conditions, and its
  dated claims as the self-running script.
- **Map** (`#/map`, `#/map/place:avdiivka`, `#/map/topic:c19`): places sized by the claims about them in a
  month range, with each place's claim history. A chapter link highlights and fits that chapter's places.
- **Arms race** (`#/arms`): measure/countermeasure links on a timeline, in lanes by the kind of measure, with
  their evidence.
- **Numbers** (`#/series/metric:shahed_launches`): a metric's dated observations, with their ranges, by side.
- **Disputes** (`#/disputes`): contradictions between claims, with extracted copies collapsed onto the canonical
  claim, plus who claims what and how the disagreement is explained.
- **Claim pages** (`#/claim/claim:…`): claimants, sources with quotes, what the claim is about, its numbers, and a
  relation graph.

## Building

You need Rust stable with the `wasm32-unknown-unknown` target, `trunk`, `wasm-opt` (binaryen), Node.js and
Python 3. `rust-toolchain.toml` pins stable, because surrealdb 3.3's `diskann` dependency does not build on
current nightly.

```sh
web/build.sh                          # production build → web/crates/app/dist (static files)
PUBLIC_URL=/bavovniatko/ web/build.sh # when served from a sub-path (e.g. GitHub Pages)
```

To develop:

```sh
python3 db/tools/webexport.py         # after any change to db/dump or web/kb
(cd web/bridge && npm install && npm run build)
(cd web/crates/app && trunk serve --release)       # http://127.0.0.1:8770
cargo run --release -p kbcheck        # in web/: load natively, run every gallery query and the write probes
```

`webexport.py --check` reports whether the dataset on disk is current. The release profile skips LTO so builds
stay fast and fit in memory. `--profile dist`, which `build.sh` uses, adds fat LTO.

## How it stays read-only

`Kb::open` starts SurrealDB with a random root password that never leaves the worker, and with guest access
enabled. The dataset loads as root. `Kb::seal` then drops to an anonymous guest session for good.

Every table is defined with `PERMISSIONS FOR select FULL` only:

- Writes by a guest (`CREATE`, `UPDATE`, `DELETE`, `RELATE`, `INSERT`) are silent no-ops.
- Schema statements (`DEFINE`, `REMOVE`, `INFO`) fail with a permission error.

`kbcheck` asserts this. In any case, a visitor's queries only ever touch their own in-memory copy.

## Why the dataset looks the way it does

The spike (2026-10-02) measured how to load 80,000 rows plus their indexes fastest:

| setup | load in Chromium |
|---|---|
| typed schema, indexes built row by row | about 15–17 s natively, slower in wasm |
| loose schema, indexes built after the data | **7.5 s data + 2.7 s indexes**; Firefox about 11 s |
| IndexedDB-backed engine | 42 s load + 123 s indexes; queries then take up to 9 s, so it is unusable |

- The browser schema is therefore loose (SCHEMALESS tables, with only the analyzers, the COMPUTED fields and the
  permissions kept), and every index is built after the data.
- Fields the explorer doesn't use are dropped (`prov`, timestamps, import hashes).
- Parsing the SurrealQL text is only about 11% of load time, so a binary format would not help much.
- The private archive tables (`snapshot`, `revision`) are never exported.

The production build (`--profile dist`: size-optimised, fat LTO, then `wasm-opt -Oz`) has a 13.3 MB worker (4.7 MB
gzip, 3.3 MB brotli) and is ready in about 12.7 s in Chromium. An opt-level 3 LTO build was 38 MB (9.6 MB gzip) and
ready in 11.5 s, so it doesn't pay off. A first visit downloads about 10.5 MB gzipped: the worker 4.7, the dataset
4.8, the app, editor and map 1.

Query timings are mostly 1–250 ms. One trap: correlated subqueries (`… WHERE x = $parent.id`) do not use indexes.
A `GROUP BY` is usually orders of magnitude faster.

## Before publishing

- A licence for the code and for the data (none chosen yet).
- Attribution is on the About page:
  - place geometry: ODbL, © OpenStreetMap contributors;
  - base map: OpenFreeMap / OpenMapTiles.
- The map is the only part that makes third-party requests: tiles and fonts from OpenFreeMap.
- A review pass over the data for publication: actor notes about pseudonymous people and channels, and quotes,
  which are short but present.
