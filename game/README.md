# Vuhledar, 6 February 2023

The first playable diorama of bavovniatko, built in Bevy 0.19.

Russian armoured columns drive the same dirt roads, day after day, into the 72nd Mechanised Brigade's kill zone.
The roads are left clear and the fields beside them are mined. ATGM teams wait in the tree lines, and artillery
is cued by drones. You see it the way most of the world did: through a Ukrainian drone's grey, grainy video feed.

The history is in [`research/chapters/c08-vuhledar.md`](../research/chapters/c08-vuhledar.md). The design
principles are in [`design/chapters.md`](../design/chapters.md):

- **The diorama runs by itself.** The day plays out whether or not you do anything.
- **You matter at the margin.** You are a drone operator and fire controller: you spot, call 155 mm, RAAM and
  HIMARS, and decide when the ATGM teams open fire.
- **Winning means reproducing history.** The town holds, and the columns die in the fields in the numbers the
  record gives. The debrief sets your day beside the record and names every claimant.
- **The wrecks are never cleared.** The fields start the day holding the wrecks of the January assaults. Every
  new column drives past the last one's wrecks, down the same road.

## Running

```sh
cd game
cargo run --release
```

You need Rust stable (pinned in `rust-toolchain.toml`) and a Vulkan GPU. Everything is generated in code:
terrain, ground textures, trees, buildings, vehicles, people and sound. The two WGSL shaders are embedded, so
the binary runs from anywhere.

## Controls

| input | does |
|---|---|
| WASD, Q/E, R/F | fly the drone, turn it, climb or descend |
| right-drag, wheel | look around, zoom |
| left click | 155 mm fire mission, 4 rounds (a drone over the target tightens the sheaf) |
| Shift+click | RAAM: scatter mines onto a lane |
| Ctrl+click | HIMARS (two missions all day) |
| Space | ATGM teams: AUTO (wait until the column commits), HOLD, or FREE |
| 1, 2, Tab | the drone, or the observer on the roof of a nine-storey block |
| P, `[`, `]` | pause, slow down, speed up |
| F12 | screenshot |

## Layout

| file | contents |
|---|---|
| `src/geo.rs` | the map: terrain height, roads, shelterbelts, minefields, positions, lat/lon |
| `src/terrain.rs` | chunked ground with a painted 1 m/texel texture; craters, scorch and tracks are stamped in live |
| `src/world.rs` | shelterbelts, panel blocks, dachas, the power line, light, the steppe beyond |
| `src/meshes.rs` | the procedural modelling kit: T-80BV, T-72B3, T-64BV, BMP-1/2, MT-LB, soldiers, buildings |
| `src/units.rs` | vehicles, soldiers, mines, damage, wrecks, turret tosses |
| `src/fire.rs` | Ukrainian positions and ATGM teams, artillery, RAAM, HIMARS, the Russian return fire |
| `src/director.rs` | the day's script: the columns, the infantry, barrages, TOS-1A, the clock |
| `src/fx.rs`, `src/audio.rs` | particles and light, synthesised sound (delayed by distance at 343 m/s) |
| `src/camera.rs`, `src/shaders/feed.wgsl` | the drone and the OP, and the video-feed post-process |
| `src/hud.rs` | telemetry, radio net, fire control, briefing, debrief |
| `devlog/` | the screenshot and video history of development (local only, not in git) |

## Development

Headless runs render offscreen on the GPU, with no window and no sound:

```sh
VUHLEDAR_HEADLESS=1 VUHLEDAR_SKIP_BRIEFING=1 VUHLEDAR_SHOT=out.png VUHLEDAR_SHOT_AT=30 cargo run
scripts/record.sh <name> "<x,z,agl,yaw,pitch,fov>" <from-s> <secs> "note" [vehicle-idx]
```

`scripts/showcase.sh <out.mp4>` films a directed one-minute tour of the day at 1920x1080 with sound. The game
runs on a fixed step while filming, logs every sound it plays, and `scripts/showcase_mix.py` rebuilds the
soundtrack offline. It takes about ten minutes, with no window or speakers, and the day is deterministic, so
every run films the same tour.

`VUHLEDAR_CAM` places the drone, `VUHLEDAR_LOOK=<idx>` keeps the gimbal on a vehicle, and `VUHLEDAR_SPEED`
scales game time. Every test screenshot and clip goes into `devlog/` (local, gitignored), indexed in `devlog/shots.md` and
`devlog/videos.md`. The history is part of the development.
