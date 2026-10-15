# Roadmap: the tidy-up, the dioramas as data, and the drones (plan, 2026-10-15)

Status: plan, nothing started. It absorbs the local `pending/` notes of 11–12 October, which are now gone, and
it records the owner's decisions of 15 October. When this plan is done, the game will:

- sit in a repo with two halves;
- be a workspace of crates with its own name;
- read its dioramas from data files;
- let the player request evacuations;
- put the player's hands on the drones.

This round leaves the knowledge base's content alone, apart from its tidy-up. It adds no new dioramas.

## Decisions (2026-10-15)

| # | decision |
|---|---|
| 1 | First the restructure and the crate cleanups, then the dioramas as data. |
| 2 | No new arcs or demos for now; story planning only (step 9). |
| 3 | Future-proof, tidy, structured; breaking paths is fine. The game will grow a lot, the knowledge base less. Two halves: `db/` holds the knowledge base with its generated `research/` and the `web/` explorer; `game/` holds the game, its design notes and the feed exported from the knowledge base. |
| 4 | Three READMEs: top level, game, db. No fresh screenshots for them now. |
| 5 | Evacuation is an action the player requests; the game is not tied to the order of the real battle. |
| 6 | Drones matter most: the player flies them directly. |
| 7 | No spoken Ukrainian yet. Sound: whatever is easy now. |
| 8 | Not now: demos per dossier, the Framework 13 pass, waiting-on-assets items. |

## Progress (2026-10-15)

| step | state | commits |
|---|---|---|
| 1. Two halves, READMEs | done; records keep `research/…` paths, read relative to `db/` | 0e11056, 840f97d |
| 2. Names, crates, fast profile | done: `bav-core`, `bav-sim`, `bav-look`, `bavovniatko`; the battle cannot reach the picture; G18 helpers | f632e26, c3fae27, e09a00c, 4086fe5 |
| 3. Knowledge-base leftovers | done, except `kind_label` (the dataset's labels are capitalised) and one titled link | f21290e, 690afa1, 089f347, a7b9b26 |
| 4. Dioramas as data | done for the ground, layout, place and clock (4a), forces and script (4b), and the net's callsigns, Robotyne's now its own (4c); the film's shots stay in code until the demos (step T) need them | 00858f0, dd72647, 4d72abf |
| 5. Quick fixes | done: densest-group aim, summer craters, the Kornet test (fire did not suppress), then suppression: a post under fire keeps its ATGM team down 20–60 s; three missions now keep the crest's Kornet down from 286 s to 427 s | e023554, 7e32b4c |
| 6. Evacuation on request | done: Alt+click or V sends one of four Bradleys; wounded call for help; men falling back drag the wounded; behind the start line counts as out | 7e32b4c |
| 7. Drones | research done and loaded (topics 10 and 11, 516 claims). The owner's calls (2026-10-15): no strike drones at Vuhledar; one FPV mission at Mala Tokmachka to introduce the mechanism; self-levelling flight with the mouse (acro maybe later); moving the team costs time, and at Mala Tokmachka it is hard to find. Built: the FPV (`bav_sim::drones`, camera `fpv_control`, the analogue feed), the drone team found and moved. Their drones (Orlans, Lancets, EW as a field) still to come | 6b05516, 3899789, c75433e |
| 8. Sound | done: an ERA crack, our and their guns' reports from kilometres back, rocket ripples, the Vikhr launch; beds for the nearest fire and the nearest Ka-52; the wind bed follows the day's wind, the engines' pitch the nearest vehicle's speed. The older sounds are sample-identical and the day is unchanged | fd28911 |
| 9. Story planning | done: `arcs.md` | 8aec1bb |

Every step that touched the game replayed both days byte-identically against the last session's runs, except
where a change meant to change the day: the camera's new aim (the battle identical, only the filmed drone's yaw
moved), suppression, and the evacuation, which are measured with `band.sh` instead (fast profile, seeds 0–3):

| run | vehicles lost (burned + abandoned) | men back | dead | wounded out |
|---|---|---|---|---|
| Vuhledar, with suppression | 25–30 of 33 (24–26 before; the band is 24–30) | 44–82 of 320 | 135–172 | — |
| Robotyne, hands-off (no scripted rescue now) | 16–18 of 20 | 66–107 of 140 | 23–46 | 10–17 of 43–57 |
| Vuhledar, with the drone team findable (hands-off: nobody moves it) | 25–31 of 33; found about twice a day | 43–82 of 320 | 132–177 | — |
| Robotyne, every call answered after 30 s (`BAV_EVAC=30`) | 20–23 of 24 | 103–121 of 152 | 30–47 | 9–21 of 48–74 |

Before this round Robotyne's day sent four Bradleys at a fixed minute (24 vehicles, 97–108 back of 176). Now the
hands-off day sends none, and the player's Bradleys each bring out about seven, the wounded first, when they get
through; some burn on the way in, as they did on the day.

## 1. The repo in two halves

```
README.md                 the project, a map of the two halves, the licence
LICENSE-*, .github/
game/
  README.md
  Cargo.toml              the workspace (step 2)
  crates/                 the game's crates (step 2)
  dioramas/               one data file per diorama (step 4)
  assets/  tools/  scripts/  devlog/
  design/                 was design/: chapters, campaign, units, plans (this file moves too)
  data/                   was game-data/: the feed the knowledge base exports for the game
db/
  README.md
  schema/ vocab/ fixes/ dump/ tools/ staging/ checks/ templates/ queries.md dbctl.sh
  research/               was research/: generated by tools/render.py
  web/                    was web/: the explorer
```

- Use `git mv` for every move, in one commit with nothing else in it, so history follows the files.
- Then sweep the paths. Over forty files name a moved directory:
  - the Pages workflow (`paths:` and `build.sh`);
  - `render.py`, `webexport.py`, `sim.py`, `archive.py`, `kb.py` and the other db tools;
  - the explorer's `kb.rs`, `bridge.rs` and `engine.rs`, which fetch the dataset;
  - `game/tools/bake`, the vehicle credits and `.gitignore`;
  - `LICENSE-CONTENT.md` and every README;
  - the design notes' links.
- About 19,000 record fields hold repo paths (`anchor.file`, `prov.input`, `topic.file`, `geometry_source`), all of
  the form `research/…`. Rather than rewrite them, stored paths are now read as relative to `db/`, where they
  still hold. Only the tools' resolution changes (`kb.RESEARCH`, `kb.DB_DIR`), so the dump doesn't change. GitHub
  links into `research/` from outside the repo break, which is accepted.
- Proof:
  - `render.py --check`;
  - the db smoke test;
  - a byte-identical `webexport` and `dump export`;
  - `db/web/build.sh` and a local run of the explorer;
  - the game builds, and `same-day.sh` on both dioramas is identical, since the move touches no code.
- **The three READMEs.** The top one says what the project is and maps the two halves, with one paragraph each
  and links. `game/README.md` and `db/README.md` are rewritten for their new contents: how to build and run,
  the layout, where things are generated from. `db/research/` and `db/web/` keep their READMEs as sub-READMEs. The
  showcase section another session left uncommitted in `game/README.md` (that session is no longer running) is
  folded in.
- Memories that name old paths are updated.
- The merged branch `vuhledar-diorama` is deleted.

## 2. The game's crates

The game is one crate called `vuhledar`, about 11,000 long lines. Every edit rebuilds all of it, and a release
build takes 6–20 minutes when other sessions load the machine. The rename and the split come together:

- **Names:** the binary `bavovniatko` (`cargo run --release -- robotyne`), the crates `bav-*`, the switches
  `BAV_*` (34 of them), and F12 shots as `<diorama>-NNN.png`. Scripts, READMEs, devlog instructions and memories
  follow. The old `VUHLEDAR_*` names are not kept as aliases.
- **The crates**, drawn along the line the determinism already follows:
  - `bav-core`: geometry and the terrain height (`geo`), the layout and diorama types, the dev switches, and the
    data-file schema of step 4. No rendering.
  - `bav-sim`: units, fire, the director and the per-diorama scripts. Everything that decides the battle, run
    in the `Step` sets. It talks to the picture only through messages (`DirectHit`, `Wrecked`, `Blast`, …).
  - `bav-look`: fx, the ground paint, sky, feed, meshes and the baked models, audio, HUD, text. It reacts to the
    messages and draws from its own random stream.
  - `bavovniatko`: the app. `main`, the camera and controls, showcase, dev tools.
- Today sim code calls look code directly: `units.rs` spawns fires, pool fires and ERA pops, and `fire.rs`
  spawns trails. The split turns those calls into messages that `bav-look` handles in a later `Step`. The order
  of sounds in `audio.log` must not change, which is what `same-day.sh` checks.
- **A fast profile:** `[profile.fast]` inherits release with `lto = false`, `codegen-units = 16` and incremental
  builds. It is used by `shot.sh`, `record.sh` and `band.sh`; `same-day.sh` and `showcase.sh` stay on release.
  With the crates split, a look edit no longer recompiles the sim.
- Small items: `log_stats` prints `f32::MIN` before anything moves; clippy stays clean on the whole workspace.
- Proof: both same-days byte-identical after every stage (rename, then each crate extracted).

## 3. The knowledge base's leftovers

These are done in their new home, after step 1, with the defaults chosen as follows:

- `ids.py`, dependency-free (safe-key regex, ⟨⟩ quoting, one `definitions(text)` parser), imported by both
  `kb.py` and `webexport.py`. The Pages workflow keeps building without SurrealDB.
- `load.py` commits about 50 claims per transaction instead of one per claim.
- In the explorer:
  - `Rows` becomes a `RowsState` enum (`Loading`, `NoDb`, `Failed`, `Ready`) used by every page;
  - `ui::key_link` and `kb::data_url`;
  - the hidden-fields list comes from the exporter;
  - `kind_label` reads `kind.labels.en`;
  - the map's excluded place kinds become a `mappable` attribute on `kind`;
  - one fetch helper.
- The game's G18 helpers go in with step 2's split: `mix_in`, the sound bank as an array, an RGBA8 image helper,
  `fx::missile_trail` and the texel disc.
- Proof: the db smoke test, a byte-identical dump and export, the explorer built for wasm32 and clicked through.

## 4. The dioramas as data

- One file per diorama under `game/dioramas/`: `vuhledar.ron` and `robotyne.ron`. Each holds:
  - the layout: belts and gaps, roads, villages, minefields, trenches, the jam line, the defence line;
  - the forces and where they start;
  - the day's script as timed acts (say, order, jam, release a group, a fire mission);
  - the weather and season: sky preset, wind, temperature;
  - the radio net's lines and callsigns, in English key and Ukrainian;
  - the film's shots.
- The schema lives in `bav-core`. Rust keeps only behaviour.
- `geo.rs`'s Vuhledar layout, `robotyne.rs`, the director's per-diorama tables and the showcase's shot lists
  empty into the files.
- The pending G4 items are re-judged with the layout in data: the Vuhledar barrage targets, and `TRENCH_Z` and
  `UA_LINE_Z` used outside Robotyne.
- Robotyne's own radio net goes in here. It replaces "Turret's off. Jack in the box." and "Bird: interference"
  with lines and fictional callsigns of its own. That is a text change and is proven as one: segments and tally
  identical, the audio log identical.
- Proof: both same-days byte-identical against a HEAD worktree. Each diorama runs from a file the game reads at
  start-up, and a broken file gives an error that names the line.
- Persistence (saving the field at dusk) stays with the winter campaign.

## 5. Quick fixes (from the Robotyne notes)

- **The densest group:**
  - `Target::Men` aims at the densest 60 m cell instead of the centroid of every man within 1.4 km;
  - `Target::MenNear` does the same within its radius, which also fixes the film's empty V10.
- **Summer craters:** a grassy rim, and a core that heals towards the field's colour with age.
- **The 155 mm on a Kornet team:**
  - fly a scripted fire mission on a crest team headless, and read the result from the log;
  - fix it if the team keeps firing.

## 6. Evacuation, on request

The scripted rescue goes. Today four Bradleys go back in at a fixed time because they did on 8 June. Instead:

- **The player requests it.**
  - Mark a place on the drone's picture, or answer a team's call ("Three hundred. Need evacuation…"), and
    the nearest vehicle that can carry men goes there.
  - At Robotyne that is a Bradley; at Vuhledar a vehicle from the town.
  - It drives under the same fire rules as everything else. It loads the wounded within reach, at a cost in
    time, then comes back. It pops smoke if it carries any, and again when it leaves full.
- **Men drag men.** Comrades falling back take a wounded man with them. Both move slowly and both are exposed.
  This is behaviour, not script.
- **The radio:** the request and the answers in the net's voice, with the callsigns from step 4.
- **The hands-off day changes.** With nobody requesting, nobody goes back in, so fewer wounded come out at
  Robotyne.
  - `band.sh` gets a dev switch that plays a simple requester. It answers every call after a delay, so the band
    stays measurable.
  - The Robotyne band is re-run on seeds 0–3, with and without the requester.
- The debrief already counts "still lying out there"; it adds "brought out on your request".

## 7. Drones in the player's hands

This is the largest step. The player flies one drone today: a Mavic/Matrice-class quadcopter with hover, sport
mode, zoom, a battery and return-to-home. EW is a line on the map. The aim is drones as the player's own hands.

### 7.1 Research first (into the knowledge base)

Research agents, briefed with the privacy rule, answer these as cited claims through `db/fixes` and staging.
The design reads from those claims. For each diorama's date (6 Feb 2023, 8 Jun 2023) and for later dioramas:

- **Our drones:**
  - which types the units there flew: the Mavic 3 and 3T, Autel, Matrice night droppers, the R18 heavy
    octocopter, FPV kamikazes, fixed-wing spotters such as the Leleka and the Shark;
  - how many, and from where: the operator's position and the distance to the line;
  - flight times, speeds and payloads.
- **Drops:**
  - munitions (VOG-17, RKG-3, F-1, shaped charges), drop heights, accuracy and the wind;
  - what hits on men in trenches and on vehicles' roofs do.
- **FPVs in 2023:** how common at each date, frame size, warhead (PG-7 class), range, analog video breakup,
  the terminal dive, where on a hull they hit.
- **Their side:**
  - Orlan-10 spotting for the guns;
  - Lancet strikes on lead and abandoned vehicles, with Robotyne's Leopards in the record;
  - Supercam and Zala, and Russian FPVs from late 2023;
  - the Aeroscope-style location of DJI operators followed by artillery on them.
- **EW:** what jamming does to the control link, the video and GPS for each kind of drone, and how operators
  answered (repeaters, frequency changes, altitude).

### 7.2 What to build (ordered after the research, owner's calls marked)

- **A fleet, not a drone.** The player has the drones the stock gives them on that date and switches between
  them. A drone not being flown hovers, returns or lands. Each has a battery, a link and a launch point. The
  operator is a place on the map that can be found and shelled.
- **The drop drone.**
  - It carries N munitions and drops on a key.
  - The fall is ballistic with the day's wind (`fx::day_wind` becomes a sim value read by both sides).
  - The picture shows the release and the fall.
  - The hits go through the existing blast and `DirectHit` rules, including top attack (package L).
- **The FPV.**
  - Flown directly: angle mode on keys and mouse, acro on a gamepad.
  - Analog video that breaks up with range, behind terrain and under jamming.
  - A dive ends in a hit where it strikes: engine deck, turret roof, tracks, a man. The video ends with it.
  - One per flight; they are a stock to ration.
- **The recon drone** stays as it is, and spots for the guns and for the other drones.
- **Their drones:**
  - Orlans overhead cue the guns onto what they see, including the operator;
  - Lancets come for lead and abandoned vehicles;
  - EW ramps the link loss by area, not along one line.
  - This is the sim side of the detection model. Its overlay stays deferred.
- **Headless:** every drone is flown by script for tests and the band, like `BAV_FLY` today.
- **Owner's calls, asked when 7.1 is in:**
  - which drones each diorama gives the player;
  - FPV controls on keyboard;
  - whether losing the operator ends the drone war for the day.

### 7.3 The rest of the "not modelled" list, now ordered behind the drones

- Ka-52 S-8 lob rockets and the GPS-jamming tell before their strikes.
- Infantry against infantry: the Russian counterattack, the pre-dawn height, execution errors.
- Terrain cuts (trench, ditch, dugouts, wire) and night recovery of abandoned vehicles.

## 8. Sound: the easy part

The game synthesises twelve one-shot kinds and five beds. Within reach now, with the same synthesis:

- **New one-shots:** an ERA crack; the outgoing report of our own guns from behind; a rocket-artillery ripple;
  the Vikhr launch.
- **New loops, positioned by gain:** fire crackle near burning hulls and pool fires, a grass fire's roar and
  the Ka-52's rotor thump. The gain follows the distance to the nearest few sources.
- **The engines:** the one engine bed takes its gain and pitch from the nearest moving vehicles instead of
  staying flat.
- **The wind bed** follows `day_wind`'s strength.
- With step 7: an FPV's whine whose pitch follows its throttle, and the drop's release click.

New sounds change `audio.log`. The proof is that segments and tally stay identical and the audio log only gains
lines.

## 9. Story planning (documents only)

Decision 2 holds the arcs until the cleanup is done. Meanwhile the story is planned on paper, in
`game/design/`:

- **The arcs as stories.** Snake Island, Donbas, Kursk and Azov (Mariupol and Azovstal). For each:
  - the perspective character or unit;
  - its beats from the dossiers' set pieces;
  - the recovery beat;
  - what the player's levers are;
  - how the arc sits in `campaign.md`'s eight acts.
- **The campaign's open questions**, each with a recommendation for the owner:
  - refusing the outcome;
  - interludes;
  - a hub or a linear campaign;
  - callsigns across chapters;
  - civilians in the finale;
  - difficulty.
- **The drones through the arcs.** Who flies what in 2022, 2023, 2024 and 2025. Step 7 builds the 2023 kit,
  and the arcs show how the kit grows. That growth is the war's own competence curve.

## Order

1. Step 1: the move, the READMEs, the path sweep.
2. Step 2: the rename, the fast profile, the crates split stage by stage.
3. Step 3 alongside step 2, since the two touch different halves.
4. Step 4: the dioramas as data, with Robotyne's net.
5. Step 5.
6. Step 6.
7. Step 7: research, then the owner's calls, then the build.
8. Step 8, interleaved with 6 and 7, since it is mostly `bav-look`.
9. Step 9, in parallel at any point. It touches only documents.

Every step that touches the game ends with both same-days compared, or a band run for sim changes. The
working tree is shared, so each step commits only its own paths. Step 1 waits until no other session has edits
open in the moved directories.

## Not now (kept on record)

- **Demos per dossier**, written as data files once step 4 exists.
- **The Framework 13.**
- **Spoken Ukrainian.**
- **Waiting on the 3D assets:** MaxxPro, M113, markings and decals, close-up detail, crowns edge-on, the sparse
  Robotyne belt.
- **Only the owner can do these:** play Robotyne in a window, and watch the two age-restricted Vuhledar videos
  (`GCzELk4W2HA`, `E7WDNP_YvRA`).
- **The film's content calls:**
  - V9's framing;
  - R3's tree trunk;
  - keep R1 or not;
  - an end card of 4 lines or 2;
  - a paragraph on the film in `next.md`.
  - V10 is fixed by step 5.
