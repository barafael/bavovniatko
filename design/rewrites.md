# Rewrites: the big ones, measured first

*A plan, written 2026-10-11 after the code-review and /simplify passes. It covers the items those passes left
for a decision: G6–G8 (game), W6–W10 (explorer), D2 (knowledge base).*

## Who is doing what

Checked with the other sessions before writing this:

- **No one has planned or started any item below.**
- `bavovniatko-0f` built the second diorama, Robotyne, committed as 6217571. Vuhledar stayed byte-identical by
  `same-day.sh`. It asks for the following:
  - `fx`'s `spawn`, `Spawn`, `Kind`, `Emitter` and its new `Smoke` (line of sight) stay as they are;
  - the 252 ground chunk images stay savable and loadable (milestone 1 of `next.md` persists them);
  - painting stays deterministic;
  - `hud.rs` now has a briefing, tally and debrief per diorama ("THE RECORD" is gone), so step 7 is unblocked.
- `bavovniatko-1c` has committed all of its /simplify cleanups: knowledge base (d107828), explorer (d41c484)
  and game (14ce119: G4 layout constants, G9 no per-frame allocations, G10 early returns in `fire.rs`).
  - The game now has `ObservationPost` as a plain `Res`, `Layout.defence_z`, and `Minefield::near_into` with a
    reused buffer. Robotyne lays about 50,000 mines and queries them for every moving soldier, so that buffer
    is where the mine cost lives if step 5 ever shows one.

So as of 14ce119 nothing below is blocked on anyone, and `game/src` is free to edit.

## What was done (2026-10-11, the same day)

| step | state | result |
|---|---|---|
| 1 W7 table | done | one 1.2 s freeze became batches of about 150–250 ms with the page responsive between them; the floor is the browser laying the whole table out again per batch (see below) |
| 2 W10 colours | done | every chart, legend and map colour is a stylesheet variable; checked in light and dark screenshots |
| 3 W6 map | done | no long tasks on the slider (one 59 ms stall before) |
| 4 D2 write path | done | `kb.Writer`; a second `load.py all` is byte-identical; three publisher actors that were written twice per run are written once |
| 5 G0 render costs | measured | the frame is CPU-bound; the ground as a shadow caster is half of it (see step 5) |
| 7 G6 HUD text | done | `set_if_neq` on the six texts; `same-day.sh` byte-identical against a worktree build of HEAD |

**What the table rewrite taught.** The first "after" benchmark was misread twice. A `>=` inside a `view!`
attribute (`hidden=move || shown.get() >= total`) silently ended the `<button>` tag: the attribute became the row
count, `disabled` vanished, and the click handler's block became a child expression that ran at render, so every
table grew on its own. Keep comparisons out of `view!` attribute values. Once fixed, three browser experiments
(DevTools protocol, `game/target/plan/web/table-probe*.py`) showed where a batch's time goes: locking the column
widths (`table-layout: fixed`) changed nothing, appending a prebuilt fragment instead of node by node changed
nothing, and even pure DOM cloning costs 50–110 ms per 100 rows, growing with the table. It is the full-table
layout the browser runs after each batch. `content-visibility: auto` cannot help: size containment is ignored on
table rows. Under 100 ms per block would need row virtualisation (render only the rows in the scroll port);
that is the next step if a 2,000-row table ever matters more than the query that produced it.

## What the measurements say

Measured on 569fa81 (the game) and d41c484 (the explorer), using timers added in a scratch worktree only.

| what | measured | verdict |
|---|---|---|
| Game logic per frame (vehicles, soldiers, mines, shells, ATGMs, HUD text, particle update) | under 0.3 ms each, at the busiest minute | not a problem |
| Frame, headless 1600×900, no frame cap | about 25 ms (40 fps), the same with or without particles (0 to 2,400) | the cost is in rendering the scene, not known yet |
| Particles at peak | 2,750 particles, 4,000 entities; update 0.3 ms | no measured cost today |
| Terrain start-up | colour pass 0.36 s, stamps 0.62 s, meshes 1.84 s (height and normal sampling 1.23 s): 2.8 s | the largest start-up cost |
| Peak memory | 562 MB (about 200 MB is the f32 colour buffer used during start-up) | worth reclaiming |
| Painting craters and tracks | about 100 operations a second; CPU 0.01 ms per frame | CPU is fine; the GPU re-upload (a whole 256 KB chunk per change) is unmeasured |
| Explorer: notebook "Show all" on 2,000 rows | **2.1 s** main-thread block (one 1.5 s task); JSON toggle 0.4 s | **the worst thing measured** |
| Explorer: map date slider | mostly smooth; stalls of 60–170 ms on some moves | worth fixing |
| Explorer: disputes filter (215 rows) | no long tasks | not a problem at this size |

## The plan, in order

### 1. Explorer: the notebook table (W7). Done.
The 2.1 s block comes from rendering every row and cell at once.
- **Do:**
  - Grow the table in batches of 200 rows (append, never re-render what is shown).
  - Compute `claim_ref` once per row, not per cell.
  - Pass cells by reference.
  - Fetch the `Kb` context once per table.
  - Measure nested values with a bounded size estimate instead of `to_string()`.
  - Keep the result in an `Arc` so switching between Table and JSON does not clone it.
  - Pretty-print the JSON once, on first show.
- **Target:** no main-thread task over 100 ms on a 2,000-row result.
- **Check:** the interaction benchmark (below) before and after. The rendered rows must stay the same: compare
  `innerText` of the table.
- **Size:** about a day.
- **Result** (`web/bench.py`, quiet machine): "Show all" was one 1,220 ms block; it is now 17 batches of 100
  rows a frame apart, the longest 185 ms, so the page answers between them. The first 200 rows render in 270 ms
  of long tasks (356 before), JSON in 283 (339), back to the table in 93 (140). The target of 100 ms was not
  reached, for the reason above. The table's text after "Show all" is byte-identical to the old build's (530,647
  characters, 8,285 links), compared with `game/target/plan/web/table-text.py` on both builds.

### 2. Explorer: colours from the theme (W10). Done.
This one is a real bug, not just tidying. The stylesheet has dark-mode variants (`--ua: #6aa8ff` and so on), but
`Side::color`, `Relation::color` and the map's feature colours hard-code the light-mode hex values. So in dark
mode, charts, legends and map points have the wrong colours.
- **Do:**
  - SVG and legend colours become `var(--ua)`-style references, or classes.
  - The bridge reads the computed CSS variables for MapLibre once, and again when `prefers-color-scheme`
    changes.
  - `wide_screen()` becomes `matchMedia("(min-width: 64em)")`, the stylesheet's own breakpoint.
- **Check:** screenshots of the claim, numbers and map pages in light and dark mode.
- **Size:** half a day.
- **Result:** `Side::color` and `Relation::color` return `var(--ua)` and friends (two variables added: `--intl`,
  `--highlight`); the map's features carry a `role` (place, highlight, selected, area) and the bridge maps roles to
  the computed variables when a style loads, swapping the base style when the theme changes.

### 3. Explorer: the map Effect (W6). Done.
- **Do:**
  - A `Memo` over the places, keeping each place's sorted month indices, so a slider move counts with a
    binary search instead of re-parsing every date.
  - Read with `.with()` instead of `.get()`, so all 988 rows are not cloned.
  - The highlight set becomes a `HashSet`.
  - The areas (the purple boxes) go to MapLibre from their own Effect, once.
- **Target:** no long tasks while dragging the slider.
- **Size:** half a day.
- **Result:** `Place` keeps each place's sorted month indices; a slider move is two `partition_point`s per place;
  no long tasks on any move.

### 4. Knowledge base: one write path (D2). Done.
`import_base.Writer` and `load.py` both do "hash, compare, UPSERT", with different hash functions.
- **Do:** move `Writer` into `kb.py`, with the hash function as a parameter. `load_entities`, `load_metrics` and
  `load_claims` use it, each with its current hash, so every stored `ext._hash` stays valid and a re-run
  changes nothing.
  - `load_metrics` currently UPSERTs all 404 metrics on every run (404 audit revisions each time); through
    `Writer` it skips unchanged ones.
- **Not part of this step:** converging on one hash function. That rewrites every record's `ext._hash` once,
  and creates one audit revision per record. If it is ever wanted, it becomes its own migration with its own
  commit message.
- **Check:**
  - `dump.py export` is byte-identical before and after a full `load.py all`.
  - A second `load.py all` reports everything unchanged.
  - `smoke_test.py` passes.
- **Size:** a day.
- **Result:** all three checks hold on a scratch database restored from the committed dump. Two things found on
  the way: the metrics never carried a hash (they were upserted on every run; now they get one, so the next
  `load.py all` rewrites all 404 once and the dump gains their `ext._hash`), and three publishers spelled two ways
  in `research/sources.yaml` were two writes to one actor per run (grouped by slug now). Also noticed, not
  touched: the committed dump and `db/staging` disagree on claims `c02-0030-11` and `c14-0027-09`, so every
  `load.py all` rewrites those two.

### 5. Game: find where the frame goes (new, G0). Done.
Nothing in the game logic or the particles explains the frame time, so measure the renderer before rewriting
anything for speed.
- **Do:**
  - Run windowed with Bevy's `RenderDiagnosticsPlugin` (GPU time per pass).
  - Optionally use `tracy` via bevy's `trace_tracy` feature.
  - Toggle the suspects one at a time: shadows (the sun's cascades, point-light shadows), MSAA 4×, the 252
    terrain chunks with their extended material, the tree cards, and the feed post-process.
- **Out:** a ranked list of render costs, written into this file. Rendering rewrites, if any, are planned from
  that list.
- **Size:** half a day.
- **Result: where the frame goes.** Measured headless at 1600×900 with `VUHLEDAR_DIAG=1` (frame times and
  `RenderDiagnosticsPlugin`'s GPU pass times) and `VUHLEDAR_OFF=…`, 45 s into each diorama, on the integrated
  Radeon 780M. Two runs: one while other builds loaded the machine, one quiet; they agree.

  | left out | Vuhledar frame | Robotyne frame | main 3D pass on the GPU |
  |---|---|---|---|
  | nothing | 15.5 ms | 15.3 ms | 3.9 / 5.2 ms |
  | the ground as a shadow caster (it still receives them) | **8.8 ms** | **9.4 ms** | 4.6 / 5.3 ms |
  | the sun's shadow cascades | **6.5 ms** | **7.1 ms** | 2.5 / 3.4 ms |
  | the ground meshes | **7.3 ms** | **7.1 ms** | 0.6 / 1.6 ms |
  | MSAA 4× (loaded run) | 16.5 ms | 16.3 ms | 3.5 / 3.7 ms |
  | the feed post-process (loaded run) | 17.6 ms | 14.2 ms | 4.9 / 5.2 ms |
  | the trees (loaded run) | 17.9 ms | 14.5 ms | 4.4 / 3.8 ms |

  All GPU passes together take about 6 ms, so the frame is CPU-bound, and more than half of it goes to the 252
  ground chunks under the two shadow cascades: without either, the frame is 7 ms. MSAA costs a millisecond; the
  trees and the feed nothing measurable. The ranked list is therefore one item long: **the ground as a shadow
  caster**. Marking the chunks `NotShadowCaster` (what `VUHLEDAR_OFF=groundshadow` does) gives back 6–7 of
  those milliseconds; the ground still receives the shadows of trees, buildings and vehicles, and only loses the
  shadow one hill throws on the next, which the drone's and the OP's views barely show (`target/plan/g0b/*.png`).
  Making that the default is a one-line decision for whoever takes the terrain step (6); it is not taken here,
  because it changes the picture.

### 6. Game: terrain start-up and memory (G8). With the persistence milestone, together with `bavovniatko-0f`.
Robotyne adds a seasonal palette, ploughed fields and trenches to `terrain.rs`, and `next.md` wants the chunk
images saved and loaded. So this step reshapes the terrain to fit both, rather than only speeding it up.
1. **Heights once per chunk.**
   - Sample a 67×67 height grid per chunk (with a one-vertex border) and take normals by central differences
     from it. That is 5× fewer `geo::height` calls.
   - Build the chunk meshes in parallel with `std::thread::scope`, as the colour pass already is.
   - Heights stay identical; normals change slightly (different differencing step).
   - Expect about 1.2 s less start-up.
2. **No global f32 buffer.**
   - Paint straight into each chunk's RGBA8 data.
   - One `for_texels_in_disc(at, r, |chunk, i, w, d|)` serves the start-up stamps and `apply_paint`, replacing
     the three copies of that loop.
   - Expect about 200 MB less peak memory.
   - Colours are quantised to 8 bits at each stamp instead of once at the end. Check with an image diff of all
     252 chunks (mean absolute error under 1/255, no visible change).
3. **Persistence-ready.**
   - `Ground` keeps each chunk's bytes and a dirty rectangle.
   - Save and load are a PNG per chunk.
   - This is the shape milestone 1 of `next.md` needs, so it is done with whoever builds persistence.
4. **GPU upload of dirty rectangles only** (`RenderQueue::write_texture`), only if step 5 shows the whole-chunk
   re-upload matters.
- **Check:** `scripts/same-day.sh` shows the simulation unchanged (terrain is presentation only); the image diff
  for colours; start-up time and memory before and after.
- **Size:** two to three days.

### 7. Game: HUD text (G6). Done.
Building the strings costs 0.03–0.05 ms per frame. The real cost is Bevy re-laying out six text blocks every
frame whether or not they changed.
- **Do:**
  - Write a `Text` only when its string changed (`set_if_neq`).
  - Rebuild the radio log only when a line arrives or ages out.
- **Check:** `same-day.sh` stays identical; frame time from step 5 before and after.
- **Size:** two hours.

### 8. Not now
- **Particles (G7).** No measured cost at today's counts. Revisit if step 5 or Robotyne's dust shows particles
  in the frame. The test is a stress scene at `MAX_PARTICLES` (9,000).
  - If needed: keep the `fx` API, move the particles into one buffer per `Kind` (a dynamic mesh of camera-facing
    quads, depth-sorted on the CPU, alpha in the vertex colour) instead of one entity each.
- **Disputes filter (W8).** No long tasks at 215 rows. Revisit past about 1,000 disputes.
- **Topic kind in the vocabulary (W9).**
  - Add `kind: topic | chapter | vignette` to `vocab/topics.yaml` and the `topic` table, and export it, so
    `router.rs`, `chapters.rs` and `map.rs` stop guessing from the first letter of the code.
  - It is a schema change (a migration, a new field in the dump), so do it with the next migration rather than
    on its own.

## How each step is checked

| area | check |
|---|---|
| game simulation | `game/scripts/same-day.sh before`, then `same-day.sh after before`: sound log, film segments and tally byte-identical |
| game rendering | the step-5 diagnostics; an image diff of the 252 chunk images; start-up time and peak memory |
| explorer | the interaction benchmark: headless Chromium, long tasks per interaction (map slider, disputes filter, notebook query, Show all, JSON toggle); worth adding to `web/` as a script |
| knowledge base | `dump.py export` byte-identical, a second load all unchanged, `smoke_test.py`, `render.py --check` |

## Order and who

| when | steps | who |
|---|---|---|
| done 2026-10-11 | 1 (W7), 2 (W10), 3 (W6), 4 (D2), 5 (G0), 7 (G6) | this session |
| with the persistence milestone of `next.md` | 6 (G8) | together with `bavovniatko-0f` |
| when measurements say so | particles, disputes, topic kind | — |
