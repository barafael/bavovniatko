# The places as data

Status: done for the screen's words, the tours' shots, the set, the registry of dioramas and Avdiivka's shared part
(2026-10-25). A fifth place is files, an entry in `dioramas/index.ron`, and code only where it brings a set or a rule
of its own. Progress is logged at the end of this file.

## Why

The places were data for the battle and the ground (`dioramas/*.ron`, since the roadmap's step 4), but the code still
asked which diorama it was in about eighty times: the HUD and the debrief named each place and laid out each
tally by hand, the tours' shot lists were code by variant, the look chose its set and its woods by variant,
`bav_core::data` kept a table of files by variant, and the xtask knew the days by name. Adding a place meant
touching all of them. The owner's rule for the refactor: every day replays byte for byte as before, and the
picture, the HUD's text, the debrief and the films do not change.

## What is where now

| what | where | read by |
|---|---|---|
| which dioramas there are, in order: each an id, its files (merged into one record), its shots file | `dioramas/index.ron` | bav-core's build script (`crates/core/build.rs`), which includes every file it names; the xtask |
| the tally's and the fire panel's lines, as templates (`{committed}`, `{shells}`, …) | each day's `hud` | `bavovniatko::screen`, `hud.rs` |
| the debrief: the place in the heading (none: the diorama's own), its rows (a label and a figure, rows gated on what the day has: `If(Flew, …)`, `If(Field, …)`, `If(Runway, …)`, `If(Drops, …)`, a row per area, the duel's hits and runs), the columns' table | each day's `debrief` | `bavovniatko::screen`, `hud.rs` (`Ledger`) |
| where `BAV_VIEW=ground` starts | `ground_team` (a place's map, or the single file) | `dev.rs` |
| the set raised over the ground, with its own light (`PanelBlocks`, `WaterTower`, `Antonov`, `CokePlant`), and the woods (`Pine`, else the steppe's belts) | `set`, `woods` | `bav_look::data`, `world.rs` |
| the minute tour, the film's part, the tour's title card, the trailer's day (`reel`: card, end, switches) | `dioramas/shots/<id>.ron` | the showcase (`bav_core::data::shots_source`), the xtask (`xtask/src/days.rs`) |
| what Avdiivka's three parts share (sides, callsigns, thermal, Space, the tally's lines, their guns, EW, FPVs, the drone team) | `dioramas/avdiivka/forces.ron`, merged after the map and before the part | the same readers as before |

`Diorama` is now an index into the build script's table: its `id()`, `files()` and `shots()` are the index's, and
`attacker()`, `season()`, `summer()`, the title and the clock are the files' (they were already). `Diorama::ALL` is
the index in order. The ground cache is kept under the id (`hostomel-1-<key>`), not the variant's old name.

Mala Tokmachka's second line in the stats log (`picked_up=… wounded=…`) is written where the day has evacuation, which
is what it reports.

## What stayed in code, and why

- **The sets' builders and their light** (`world.rs`'s town, `robotyne.rs`, `hostomel.rs`, `avdiivka.rs`): each is
  a different builder (panel blocks on a ridge, a water tower, the Antonov hangar and the Mriya, the coke plant), so
  the file names the set and `world.rs` matches on it. A new place either reuses one or brings its own module and a
  variant of `bav_look::data::Set`.
- **The figures the HUD and the debrief can show** (`screen::Figure`, the tally's and the fire panel's keys): each is
  a number read from the battle. A new kind of figure is code; which figures a place shows, under which words, is
  data. The few phrases a figure carries with it (`   aircraft over n   down m`, `   VOG  n [C]`, the runway's
  `enough for an Il-76`, the airlift's fates) are written once, for whatever day has the thing.
- **The duel's tank** in Stepove's debrief is found as the T-90M (`VKind::T90m`), as `bradley.rs` knows it.
- **The season's look** (`summer()`, `season()`): already data; the picture reads it where it paints.
- **`Diorama` stays as a type**, no longer an enum: the run's place in the registry, fixed at startup. Nothing matches
  on it; `hostomel()` and `avdiivka()` are gone.

## How to add a place now

1. Write its files under `dioramas/` (a single `<id>.ron`, or a directory with a shared `map.ron`, the map's
   `osm.ron` from `tools/osm/osm2ron.py`, an optional shared part like Avdiivka's `forces.ron`, and one file per
   part): the place, the date and clock, the sides and season, the ground and layout (`bav_core::data`), the forces
   and the script (`bav_sim::data`), the words and the screen's `hud` and `debrief` (`bavovniatko::screen`),
   `ground_team`, and the `set` (and `woods` if they are pine). Run `fmtron dioramas`.
2. Add an entry to `dioramas/index.ron`: its id and its files in merge order; a field may be in one file only.
3. For a tour, write `dioramas/shots/<id>.ron` (its `card` and `minute` shots; `film` and `reel` if it is in the film
   and the trailer) and name it in the entry. `cargo xtask tour <id>` then films it.
4. `cargo test -p bav-core -p bav-sim -p bav-look -p bavovniatko` reads every diorama with every crate's schema
   and checks the formatting; `cargo xtask smoke` starts it under every switch.
5. Code only where the place brings something new: a set (a builder and its light, and a `Set` variant), a rule of
   the battle, or a figure the screen has never shown.

## Progress

- 2026-10-25: the refactor, in seven commits, each leaving all seven days `same` (sound log, film segments and tally,
  and the debrief's ledger too) against their baselines (`fx-v`, `r2-r`, `f3-h1`, `f3-h2`, `m4-a1`, `m4-a2`,
  `m3-a3-x`). `6b644dd` the screen's words into the files; the text compared by a build of the commit before and
  one of this, both patched to log every tally, fire panel and debrief as it changed over each day's minute tour:
  the same, line for line, all seven days. `bc197a2` the shots into `dioramas/shots/`: the minute tours by the
  same-day runs; the film's two parts filmed before and after (`BAV_TOUR=film`, 320x180), sound log, segments,
  cards, ledger and tally the same. `1a2e5da` the set and the woods as data. `d9f57b4` the registry
  (`dioramas/index.ron`, the build script, `Diorama` an index). `6048081` the xtask reads the index and the shots
  files: `tour` takes any id with a shots file (Avdiivka's three parts now among them, with title cards of their
  own), the trailer and the film take the days with a `reel`, the end card is built for however many there are
  (for two, as before); a test reads the reel's days back as reel.rs had them. `dc28b02` Avdiivka's shared fields
  into `forces.ron`. `54814fd` `crates/sim/src/units.rs` split at its section rules into `units/`: a pure move,
  checked line for line, the plugin unchanged. The picture: frames of each day's minute tour at 640x360 before and
  after differ no more than two runs of one build do (the particles and the frames a loaded machine drops already
  differ between runs: the largest RMSE of a sampled frame is about 0.04 for Mala Tokmachka either way, 0 for
  Avdiivka). No balance changed, so no bands were run. Not done: the briefings' shared paragraphs and the sources,
  which sit inside each part's `words` (the merge is by whole fields); the end-to-end `cargo xtask film` and
  `trailer`, which were not run (their days, cards and switches are tested as data).
