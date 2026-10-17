# The game

The playable dioramas of bavovniatko, built in Bevy 0.19. Each is one bounded historical episode that runs by
itself; the design is in [`design/`](design/) (the principles in [`chapters.md`](design/chapters.md), the
campaign in [`campaign.md`](design/campaign.md), the work in progress in [`roadmap.md`](design/roadmap.md)). There
are two so far, one minefield seen from both sides:

- **Vuhledar, 6 February 2023** (`cargo run --release`): Russian columns into the 72nd Brigade's kill zone.
- **Mala Tokmachka, 8 June 2023** (`cargo run --release -- robotyne`): the 47th Brigade's breach into the Russian
  minefields, the first day of the Robotyne chapter.

To watch rather than play, `cargo xtask film` produces a four-minute film of both, directed from
`design/showcase.md` ([below](#the-showcase-film)).

## Vuhledar

Russian armoured columns drive the same dirt roads, day after day, into the 72nd Mechanised Brigade's kill zone.
The roads are left clear and the fields beside them are mined. ATGM teams wait in the tree lines, and artillery
is cued by drones. You see it the way most of the world did: through a Ukrainian drone's grey, grainy video feed.

The history is in [`db/research/chapters/c08-vuhledar.md`](../db/research/chapters/c08-vuhledar.md). The design
principles are in [`design/chapters.md`](design/chapters.md):

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

The history is in [`db/research/chapters/c09-robotyne.md`](../db/research/chapters/c09-robotyne.md) (§3, §5, set
piece 1). The outcome is fixed: the breach fails. What is yours is the cost: how many crews and infantry come
back. You have a battery with 64 rounds, six smoke missions (smoke hides the column from the Kornets and the
Russian guns, and from your own drone), one HIMARS mission, and the order to the column.

Their electronic warfare is a field, not a line. Jammers along the crest switch on and off, each a mast with a
red lamp while it jams; your drone's link frays near them and past the jam line. A 155 mm mission on one knocks
it out. And watch the satellite count: when the GNSS jamming suddenly lifts, their Ka-52s are a minute out.

## The showcase film

[`design/showcase.md`](design/showcase.md) is one film of both dioramas, about four minutes: a title,
Vuhledar, an intertitle, Mala Tokmachka, and an end card built from the two days' ledgers. It is the thesis of
this README, shown: one minefield from both sides. There is no narrator and no music — cards and the in-game
radio net carry it.

It is filmed by the same machinery the game runs on. The binary runs headless on a fixed step with
`BAV_TOUR=film` (which selects the film's shot list in `crates/bavovniatko/src/showcase.rs` instead of the one-minute tours),
the script places the drone and flies it, cards are hard cuts over black between the shots, and each part's
soundtrack is rebuilt offline from the game's log of every sound it played (`xtask/src/mix.rs`), each part
levelled to -18 LUFS. The tasks are an xtask (`xtask/`): plain Rust, with ffmpeg, ffprobe and ImageMagick on the
path.
The tour is deterministic, so every run films the same film.

```sh
cd game
cargo xtask film               # the film, 1920x1080, into devlog/videos/
cargo xtask film --scout       # the same at 640x360 with contact sheets, into devlog/film/, for retiming
```

The trailer ([`design/trailer.md`](design/trailer.md)) is the action of both days, about two and a half
minutes, and the battle picks its shots: `cargo xtask trailer` runs each day once logging every blast and hit,
frames the best moments, films them in as many runs as it takes, and cuts them together (`--scout`: 640x360 with
a contact sheet, into `devlog/film/`).

`--film` takes tens of minutes on the machine (both parts, 1080p); `--scout` a few. The only switches the tour
reads are `BAV_TOUR=film` and `BAV_SHOWCASE=<dir>`; the script sets them, so they are rarely typed by
hand. The film and its row are recorded in `devlog/videos/` and `devlog/videos.md`.

## Running

```sh
cd game
cargo run --release
```

The day plays as the Record (Літопис): the forces, the stock and the script as the archive has them. Command
(Командування, [`design/command.md`](design/command.md), in the making) is the other way to play it: the player's
side is an Allotment (наряд сил) of Units and Support chosen from what was there that day, under a limit of
points. Until the map table exists, `BAV_ALLOT="stugna@0,javelin@3,t64@9,155*8,himars*3"` (a unit at a position, or
so many of an item; the ids are the diorama file's `catalog` and `support`) plays Command with those picks;
`BAV_PLAY=command` without it plays Command with the Record's allotment. An allotment over the points, with more of a unit than
there were, or with a position given twice, stops the game with the reason. A unit's position is a number from
the field (`crates/core/src/field.rs`): the diorama's own posts first, then its tank hides, then every tree line's
edge and the ruins of the town's edge; a post goes where men can hold it, a tank in a hide or a tree line, and
nothing on the enemy's side of the grey zone. `BAV_FIELD=1` lists the positions by number in the log and draws the
field over the picture: the positions (green ours, yellow in the grey zone, red theirs), the road heads, the grey
zone's edges and the tracked routes from each road head; it lifts the drone's 700 m ceiling, so
`BAV_CAM="0,0,3200,0,-90,70"` shows the whole box. `BAV_COLUMN="120:0:leo2a6+bradley+bradley@54"` sends a Column at game
second 120 from road head 0 to position 54: it drives the field's route at its slowest vehicle's pace, fans out
abreast at the position's edge, puts its men into it and holds; its progress is logged every 30 s.

In Command our units take Orders over the net, with its delay and the enemy's jamming: O in the drone's view opens
the orders, a click picks one of ours, G / K / B order it on, to hold, or back, and a click on a ringed position sends
it there. `BAV_ORDER="200:100:hold,450:100:position@18,800:100:back"` gives orders at game times (wave 100 is the
first Column; the script's waves count from 1), `BAV_ORDERS=open` starts with the orders open.

You need Rust stable (pinned in `rust-toolchain.toml`) and a Vulkan GPU. An integrated one will do: on a Radeon
780M at 2560x1600 a frame takes about 10 ms. Everything else is generated in code:
terrain, ground textures, trees, buildings, people and sound. The two WGSL shaders are embedded.

The vehicles and the Ka-52 are real 3D models: the freely licensed ones catalogued in
[`db/research/assets/`](../db/research/assets/README.md), baked into `assets/vehicles/*.veh` (credits in
[`assets/vehicles/CREDITS.md`](assets/vehicles/CREDITS.md)) and built into the binary. The Leopard 2R, the Wisent
and the BMR-2 have no model of their own: they sit on the Leopard 2A4's and the T-64's hulls, with their tops,
plows and rollers built in code.

To rebake after changing a model or its spec, download the models (`db/research/assets/fetch.py`) and run:

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
| Alt+click, V | Mala Tokmachka: send a Bradley for the wounded at the cursor, or (V) where the last call for evacuation came from; four for the day. At Vuhledar nothing gets down to the tree line by day |
| 4 | Mala Tokmachka: fly the FPV (one for the morning). The mouse points it and it goes where it points; W/S throttle, A/D bank, and it levels itself. Its analogue picture thins with range and behind the ground; near their jammers the control link drops out in spells |
| M | move the drone team once the enemy has found it (a ranging round lands close): the feed is down while it moves. A team that stays is hit and loses a drone. Vuhledar's team is found a few times a day; Mala Tokmachka's rarely |
| Space | Vuhledar: ATGM teams AUTO (wait until the column commits), HOLD, or FREE. Mala Tokmachka: the column PUSH ON, HOLD, or PULL BACK (picking up the infantry) |
| 1, 2, Tab | the drone, or the observer (a nine-storey roof; a water tower by the N08) |
| 3 | into a tree line behind an ATGM team; again for the next team. Mouse looks, right button holds the sight, left click launches |
| P, `[`, `]` | pause, slow down, speed up |
| F12 | screenshot |
| Esc, F10 | settings: the frame rate and its cap (60 fps by default), vsync, fullscreen, render scale, anti-aliasing, shadows, view distance, brightness, the video look, smoke and fire. Arrow keys pick and change, everything applies at once, and it is kept in `~/.config/bavovniatko/settings.txt` |

On the right, under the tally, the hit feed recounts the last hits: what hit, from where, at what, and what it did
(our hits in white, hits on our vehicles in amber), for 45 seconds each.

## Layout

The game is a Cargo workspace of four crates, layered so the battle cannot reach the picture:

| crate | path | contents |
|---|---|---|
| `bav-core` | `crates/core/` | what every part shares: the ground's geometry and both layouts (`geo.rs`, `geo/robotyne.rs`), which diorama is on the table and the sides (`diorama.rs`), the words in two registers (`text.rs`), the frame's `Step` order and the startup sets (`schedule.rs`), the `BAV_*` switches (`dev.rs`) and the small vocabulary they name (`vocab.rs`) |
| `bav-sim` | `crates/sim/` | the battle, deterministic: vehicles, soldiers, mines, damage and wrecks (`units.rs`), positions, ATGM teams, artillery, RAAM, smoke and HIMARS (`fire.rs`), the day's script (`director.rs`), the Ka-52s and the dug-in tanks (`robotyne.rs`), the player's drone and posts (`player.rs`), and the queues it fills for the picture and the sound (`fx.rs`, `paint.rs`, `sound.rs`) |
| `bav-look` | `crates/look/` | the picture and the sound, which read the battle and play its queues from their own random stream: the painted ground (`terrain.rs`), belts, towns and light (`world.rs`), the belts' trees (`trees.rs`), Mala Tokmachka's set and light (`robotyne.rs`), sky, particles and fire (`fx.rs`), the models (`models.rs`, `meshes.rs`, `baked.rs`), the video feed (`feed.rs`), the synthesised sound (`audio.rs`), and the WGSL shaders |
| `bavovniatko` | `crates/bavovniatko/` | the game: the app (`main.rs`), the drone, OP and tree-line views and their controls (`camera.rs`), the HUD, briefing and debrief (`hud.rs`), the settings overlay (`settings.rs`), the development systems (`dev.rs`) and the filmed showcase (`showcase.rs`) |

`bav-sim` does not depend on `bav-look`. The battle spawns its entities with model handles the look builds at
startup, and asks for effects, ground marks and sounds through queues the look plays later in the frame. So
nothing the picture does (its particles, its random stream) can change how a day goes.

| path | contents |
|---|---|
| `design/` | the design notes: chapters, campaign, units, the plans and their decisions (the current one is [`roadmap.md`](design/roadmap.md)) |
| `data/` | the game feed exported from the knowledge base (`db/tools/sim.py export`) |
| `assets/vehicles/`, `tools/bake/` | the baked vehicle models, and the tool that bakes them (a workspace of its own) |
| `scripts/` | headless shots and clips, the showcase film, the same-day and balance checks |
| `devlog/` | the screenshot and video history of development (local only, not in git) |

## Development

Headless runs render offscreen on the GPU, with no window and no sound. `shot.sh`, `record.sh` and `band.sh` build
with the `fast` profile (release without LTO, incremental): a fraction of a release build's time. `same-day.sh`
and the showcase stay on release.

```sh
BAV_HEADLESS=1 BAV_SKIP_BRIEFING=1 BAV_SHOT=out.png BAV_SHOT_AT=30 cargo run
scripts/record.sh <name> "<x,z,agl,yaw,pitch,fov>" <from-s> <secs> "note" [vehicle-idx]
scripts/shot.sh <name> "<x,z,agl,yaw,pitch,fov>" <game-s> "note" [diorama]
```

Angles in `BAV_CAM` are degrees; yaw 0 looks north, 180 south.

`cargo xtask tour [robotyne] [out.mp4]` films a directed one-minute tour of a single day at 1920x1080 with sound
(the four-minute film of both dioramas is `cargo xtask film`, [above](#the-showcase-film)). The game
runs on a fixed step while filming, logs every sound it plays, and the xtask rebuilds the soundtrack offline
(`cargo xtask mix <dir> <out.wav>` does it again for a kept work dir). It takes about ten minutes, with no window or speakers, and the day is deterministic, so
every run films the same tour.

That determinism is also the regression test: `scripts/same-day.sh before` and, after a change,
`scripts/same-day.sh after before` films the day fast (headless, tiny frames) and compares the sound log, the film
segments and the minute-by-minute tally. A refactor should leave all three byte for byte the same. It holds
because every game system runs in one fixed order each frame (`Step` in `crates/core/src/schedule.rs`): give a new system a `Step`. `BAV_SEED=<n>` runs the day on other random streams, and `scripts/band.sh [diorama] [seeds]` films it on
several and lists the final tallies side by side: one run says little about balance, the band of a few does.
`BAND_TAG=<name>` keeps a variant's runs apart, with its switches in the environment: `BAND_TAG=evac BAV_EVAC=30
scripts/band.sh robotyne` answers every call for evacuation after 30 s, as a player would. `BAV_FIRE="260:he@-468;702"`
calls a 155 mm mission on a point at game second 260 (or, without `@`, on the stalled vehicle nearest the drone).
`BAV_FPV="300:-150;724"` launches the FPV at game second 300 and flies it by itself at the enemy vehicle nearest
that point (at height, then the dive). `BAV_OPERATOR=off` keeps the drone team from being found (the film sets it).

`BAV_LINEUP=x,z,heading` parks every vehicle model in a row at that spot, live and (behind them, turrets
turned) burned out, to look the models over; with `BAV_LINEUP_FIRE=1` the burned-out row burns (a hull in flames, a smoulder and a pool of diesel in turn), to look at the fires and their smoke side by side. `BAV_CAM` places the drone, `BAV_LOOK=<idx>` keeps the gimbal on a vehicle (`hit`, `pool` or `grass` on the hull a missile is flying at, the newest pool fire or a grass fire's front, for test shots of the fire looks), and `BAV_SPEED`
scales game time. `BAV_DIAG=1` logs the frame time, the CPU time a frame took across threads, the GPU time of every render pass and the entity counts every ten seconds, and any frame over 25 ms on its own
(headless, it also lifts the frame cap), and `BAV_OFF=shadows,msaa,feed,trees,terrain,particles,vshadows,shade` leaves parts of the
picture out, so a slow frame can be charged to what makes it slow before anything is rewritten. The tree belts cast no
shadow maps: the ground darkens itself where a baked height map of the canopy and the terrain stands between it and
the sun (`terrain::occluder_map`); `BAV_SHADE_DEBUG=1` paints that term alone, white in the sun and black in shade. The picture's
cost has four knobs, with defaults measured for the 780M at 2560x1600 (`design/perf.md`): `BAV_SCALE` (the 3D
picture's size against the screen's; by default it is capped near 1920x1200 and the feed pass scales it up),
`BAV_AA=msaa2|msaa4|smaa|off`, `BAV_SHADOW_MAP=<texels>` (2048) and `BAV_CASCADES=<n>,<first m>` (3, 450).
`BAV_SKY=overcast` puts the unbroken stratus of the real 6 February over Vuhledar instead of the footage week's
broken cloud and hard sun (the default). `BAV_DEBRIEF=1` ends the day as soon as `BAV_WARP_TO` arrives
(or at once), to look at the debrief without playing the day through. The painted ground (252 chunks, about 2.5 s of
the start) is kept in `~/.cache/bavovniatko/ground/<diorama>-<key>/`, whose key hashes the painting's code and the
diorama's file, so a change to either paints afresh; `BAV_GROUND=<dir>` keeps it there instead and
`BAV_GROUND=off` always paints. Every test screenshot and clip goes into `devlog/` (local, gitignored), indexed in `devlog/shots.md` and
`devlog/videos.md`. The history is part of the development.
