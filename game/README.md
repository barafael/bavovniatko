# Vuhledar and Mala Tokmachka

The playable dioramas of bavovniatko, built in Bevy 0.19. There are two, one minefield seen from both sides:

- **Vuhledar, 6 February 2023** (`cargo run --release`): Russian columns into the 72nd Brigade's kill zone.
- **Mala Tokmachka, 8 June 2023** (`cargo run --release -- robotyne`): the 47th Brigade's breach into the Russian
  minefields, the first day of the Robotyne chapter.

## Vuhledar

Russian armoured columns drive the same dirt roads, day after day, into the 72nd Mechanised Brigade's kill zone.
The roads are left clear and the fields beside them are mined. ATGM teams wait in the tree lines, and artillery
is cued by drones. You see it the way most of the world did: through a Ukrainian drone's grey, grainy video feed.

The history is in [`research/chapters/c08-vuhledar.md`](../research/chapters/c08-vuhledar.md). The design
principles are in [`design/chapters.md`](../design/chapters.md):

- **The diorama runs by itself.** The day plays out whether or not you do anything.
- **You matter at the margin.** You are a drone operator and fire controller: you spot, call 155 mm, RAAM and
  HIMARS, and decide when the ATGM teams open fire.
- **Winning means reproducing history.** The town holds, and the columns die in the fields in roughly the numbers
  of the record. The record shapes the day, and the debrief tells only the day's own account.
- **The wrecks are never cleared.** The fields start the day holding the wrecks of the January assaults. Every
  new column drives past the last one's wrecks, down the same road.

## Mala Tokmachka

Robotyne is Vuhledar from the other side. Before dawn the 47th Mechanised Brigade goes south up a long open slope
towards the 291st Regiment's trench on the crest: nine vehicles behind a Leopard 2R and a BMR-2 on the western
track, then eleven on the eastern. The guns have finished their preparation, and the column is late. The
minefields are hundreds of metres deep, with anti-tank mines stacked two high, and every lane is registered for
the Russian guns. Kornets, dug-in tanks and Ka-52s firing from beyond reach do the rest. Four Bradleys go back
in for the men.

The history is in [`research/chapters/c09-robotyne.md`](../research/chapters/c09-robotyne.md) (§3, §5, set
piece 1). The outcome is fixed: the breach fails. What is yours is the cost: how many crews and infantry come
back. You have a battery with 64 rounds, six smoke missions (smoke hides the column from the Kornets and the
Russian guns, and from your own drone), one HIMARS mission, and the order to the column.

## Running

```sh
cd game
cargo run --release
```

You need Rust stable (pinned in `rust-toolchain.toml`) and a Vulkan GPU. Everything else is generated in code:
terrain, ground textures, trees, buildings, people and sound. The two WGSL shaders are embedded.

The vehicles and the Ka-52 are real 3D models: the freely licensed ones catalogued in
[`research/assets/`](../research/assets/README.md), baked into `assets/vehicles/*.veh` (credits in
[`assets/vehicles/CREDITS.md`](assets/vehicles/CREDITS.md)) and built into the binary. The Leopard 2R, the Wisent
and the BMR-2 have no model of their own: they sit on the Leopard 2A4's and the T-64's hulls, with their tops,
plows and rollers built in code.

To rebake after changing a model or its spec, download the models (`research/assets/fetch.py`) and run:

```sh
cd tools/bake && cargo run --release            # all of them; or name some: -- t72 bmp2
```

`tools/bake/src/spec.rs` says which model becomes which vehicle: how to turn it to face -Z, its units and real
length, and which of its pieces make up the turret (by node, or by a ring above the deck). `BAKE_PIECES=<y>` lists
the pieces that reach above a height, to write a ring from.

## Controls

| input | does |
|---|---|
| WASD, Q/E, R/F | fly the drone, turn it, climb or descend |
| right-drag, wheel | look around, zoom |
| left click | 155 mm fire mission, 4 rounds (a drone over the target tightens the sheaf) |
| Shift+click | Vuhledar: RAAM, scattering mines onto a lane. Mala Tokmachka: smoke |
| Ctrl+click | HIMARS (two missions all day at Vuhledar, one at Mala Tokmachka) |
| Space | Vuhledar: ATGM teams AUTO (wait until the column commits), HOLD, or FREE. Mala Tokmachka: the column PUSH ON, HOLD, or PULL BACK (picking up the infantry) |
| 1, 2, Tab | the drone, or the observer (a nine-storey roof; a water tower by the N08) |
| 3 | into a tree line behind an ATGM team; again for the next team. Mouse looks, right button holds the sight, left click launches |
| P, `[`, `]` | pause, slow down, speed up |
| F12 | screenshot |

## Layout

| file | contents |
|---|---|
| `src/diorama.rs` | which diorama is on the table (first argument or `VUHLEDAR_DIORAMA`), who attacks, the season, the clock |
| `src/geo.rs` | the map: terrain height, roads, shelterbelts, minefields, positions, lat/lon (Vuhledar's layout) |
| `src/robotyne.rs` | Mala Tokmachka: its layout and height, the farm and water tower, dug-in tanks, the Ka-52s, the June light |
| `src/terrain.rs` | chunked ground with a painted 1 m/texel texture; craters, scorch and tracks are stamped in live |
| `src/world.rs` | shelterbelts, panel blocks, dachas, the power line, light, the steppe beyond |
| `src/meshes.rs` | the procedural modelling kit: soldiers, buildings, trees, the plows and rollers, the tops of the Leopard 2R, Wisent and BMR-2 |
| `src/baked.rs`, `tools/bake/` | the real vehicle models: the loader for the baked `.veh` files, and the tool that bakes them |
| `src/units.rs` | vehicles, soldiers, mines, damage, wrecks, turret tosses |
| `src/fire.rs` | positions and ATGM teams on both sides, artillery, RAAM, smoke, HIMARS, the vehicles' guns |
| `src/director.rs` | the day's script: the columns, the infantry, barrages, TOS-1A, the clock |
| `src/fx.rs`, `src/audio.rs` | particles and light, synthesised sound (delayed by distance at 343 m/s) |
| `src/camera.rs`, `src/shaders/feed.wgsl` | the drone, the OP and the tree line, and the video-feed post-process |
| `src/hud.rs` | telemetry, radio net, fire control, briefing, debrief |
| `src/dev.rs`, `src/showcase.rs` | the `VUHLEDAR_*` development switches, and the filmed showcase tour |
| `devlog/` | the screenshot and video history of development (local only, not in git) |

## Development

Headless runs render offscreen on the GPU, with no window and no sound:

```sh
VUHLEDAR_HEADLESS=1 VUHLEDAR_SKIP_BRIEFING=1 VUHLEDAR_SHOT=out.png VUHLEDAR_SHOT_AT=30 cargo run
scripts/record.sh <name> "<x,z,agl,yaw,pitch,fov>" <from-s> <secs> "note" [vehicle-idx]
scripts/shot.sh <name> "<x,z,agl,yaw,pitch,fov>" <game-s> "note" [diorama]
```

Angles in `VUHLEDAR_CAM` are degrees; yaw 0 looks north, 180 south.

`scripts/showcase.sh <out.mp4>` films a directed one-minute tour of the day at 1920x1080 with sound. The game
runs on a fixed step while filming, logs every sound it plays, and `scripts/showcase_mix.py` rebuilds the
soundtrack offline. It takes about ten minutes, with no window or speakers, and the day is deterministic, so
every run films the same tour.

That determinism is also the regression test: `scripts/same-day.sh before` and, after a change,
`scripts/same-day.sh after before` films the day fast (headless, tiny frames) and compares the sound log, the film
segments and the minute-by-minute tally. A refactor should leave all three byte for byte the same. It holds
because every game system runs in one fixed order each frame (`Step` in `src/main.rs`): give a new system a `Step`.

`VUHLEDAR_LINEUP=x,z,heading` parks every vehicle model in a row at that spot, live and (behind them, turrets
turned) burned out, to look the models over. `VUHLEDAR_CAM` places the drone, `VUHLEDAR_LOOK=<idx>` keeps the gimbal on a vehicle, and `VUHLEDAR_SPEED`
scales game time. `VUHLEDAR_DIAG=1` logs the frame time and the GPU time of every render pass every ten seconds
(headless, it also lifts the frame cap), and `VUHLEDAR_OFF=shadows,msaa,feed,trees,terrain` leaves parts of the
picture out, so a slow frame can be charged to what makes it slow before anything is rewritten.
`VUHLEDAR_SKY=overcast` puts the unbroken stratus of the real 6 February over Vuhledar instead of the footage week's
broken cloud and hard sun (the default). `VUHLEDAR_DEBRIEF=1` ends the day as soon as `VUHLEDAR_WARP_TO` arrives
(or at once), to look at the debrief without playing the day through. `VUHLEDAR_GROUND=<dir>` reads the 252 ground chunks from that directory's PNGs if they are all there, and
writes them there after painting if not: the ground as a persisted day will keep it. Every test screenshot and clip goes into `devlog/` (local, gitignored), indexed in `devlog/shots.md` and
`devlog/videos.md`. The history is part of the development.
