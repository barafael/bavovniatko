# The showcase film: one film of the two dioramas (plan, 2026-10-12)

## Context

The showcase machinery exists and works: `src/showcase.rs` films a directed tour of a day headless on a fixed
step and logs every sound; `scripts/showcase.sh` with `scripts/showcase_mix.py` turns the frames into a
one-minute 1080p mp4 with the soundtrack rebuilt offline. Each diorama has had one such tour (2026-10-11).
Since then the dioramas gained most of what a viewer should see: vehicle fates and turret tosses, crews bailing
out, TOS fireballs and smoke screens, the jammed feed and return-to-home, the Javelin from the tree line, a
sky, the drone's display, Ukrainian on the net (`src/text.rs`, Liberation Mono with Cyrillic in
`src/fonts/`), a laid-out debrief, and a drone that holds its position and does not rock.

Nothing in `design/` records a showcase decision. The owner's decisions (2026-10-12):

- **Purpose:** a pitch to people who matter. It must sell the idea in one viewing.
- **Form:** one combined film, about four minutes: Vuhledar, then Mala Tokmachka. The thesis is the README's:
  one minefield seen from both sides.
- **Telling:** cards (title, intertitles, time cards) and the in-game radio net. No narrator, no music.
- **Cards' language:** English for an English audience, with Ukrainian place names and the English in
  parentheses: Вугледар (Vuhledar), Мала Токмачка (Mala Tokmachka), Павлівка (Pavlivka).
- **HUD:** off for the atmosphere shots (dawn, late light, the wide looks over the wrecks), on for the
  operator's shots.
- **End card:** per the one recorded narration decision (design/next.md, "Decisions (2026-10-11)"): the record
  speaks once, at the end, as narration, not as a comparison. The two closing lines already in
  `show_debrief` (`src/hud.rs`) are that text.

## The film (about 4:20)

Title 5 s, Vuhledar about 127 s with its cards, the intertitle 5 s, Mala Tokmachka about 118 s, end card 10 s.

Rules for every shot, from `devlog/footage-comparison.md` and `devlog/drone-flights.md`: the drone sits
250–600 m off and 100–200 m up, gimbal 45–70° down, fov 7–14° (x4–x7); the horizon stays out of frame; no
placed shot below fov 7 (the 1 m texels blur past x7); a shot starts a second or two before its event and ends
on the result; cards are hard cuts between shots, never text over the picture.

### Cards

All in English with Ukrainian place names. Rendered by the script with ImageMagick in Liberation Mono: a
title line, a smaller second line, a dim one-line note. 2.5 s for time cards, 5 s for the part cards.

| id | where | lines |
|---|---|---|
| T | film start | BAVOVNIATKO / Two dioramas of the Russo-Ukrainian war / Everything here is simulated, and seen through a Ukrainian drone. |
| V | before V1 | ВУГЛЕДАР (VUHLEDAR), 6 February 2023 / Russian columns attack the 72nd Mechanised Brigade. The roads are left clear; the fields beside them are mined. |
| V-a | before V7 | 08:40  A second column. The same road. |
| V-b | before V8 | 10:20  The Alga battalion, from Павлівка (Pavlivka). |
| V-c | before V10 | 13:20  No more armour. Infantry, on foot. |
| R | before R1 (the intertitle) | Four months later. The other side of the minefield. / South of МАЛА ТОКМАЧКА (MALA TOKMACHKA), 8 June 2023 / The 47th Mechanised Brigade goes forward into the Russian minefields. Now the columns are ours. |
| R-a | before R5 | 05:29  The second group, eleven vehicles, on the eastern track. |
| R-b | before R8 | 07:02  Four Bradleys go back in for the men. |
| R-c | before R10 | 08:50  Everyone back to the start line. |
| E | film end | both days' ledgers, four rows each, from `ledger.txt`; then the two closing lines from `show_debrief`, verbatim. 10 s. |

The time-card texts are the day's banner labels (`director.rs` beats), so shot and caption are typed together
in `shots()`; the owner words the title, intertitle and end card.

### Part one: Вугледар (Vuhledar)

Geometry (`src/geo.rs` Vuhledar layout): the dacha road runs (2450,1350) to (760,-1010), deploy point
(1010,-300); the kill zone is the road between the middle belt z=-110 and the first belt z=-660, with ATGM
teams at (780,-664) and (930,-658); January wrecks at (1428,526), (1228,203), (1109,-94), (970,-290); the
Pavlivka road deploys at (-830,-200). Clock 06:40 + 0.36 min per game second. `pos` is (x, z, agl); fov in
degrees start→end; `Col(i)` is `Target::Vehicle(i)`; `Wave(w,i)`, `MenNear(x,z)` and `Ground` are new.

| # | shot | from (clock) | s | pos0 → pos1 | aim | fov | hud | notes |
|---|---|---|---|---|---|---|---|---|
| V1 | Dawn: the January wrecks on the dacha road | 6 (06:42) | 7 | (1270,250,140) → (1250,215,140) | Point(1228,203) | 12→9 | off | near-overhead, horizon out; ends before the 06:45 banner |
| V2 | The column in single file behind its smoke | 15 (06:45) | 9 | (1130,70,170) → (1110,10,170) | Col(2) | 14→10 | on | the net's "column on the dacha road" and the first jamming land in-shot |
| V3 | Ground: the Stugna-P sight on the lead tank, the launch | 62 (07:02) | 12 | team nearest (930,-658), sight on | Ground, tracks nearest live attacker | sight | on | launch at +4 s; the fault of every earlier cut (missiles as specks) answered from the sight |
| V4 | The kill zone, wide: the column stacks at the deploy point | 76 (07:07) | 10 | (1110,-380,170) → (1090,-410,170) | Col(1) | 11→8 | on | slide to 82 if the lead is not yet at (1010,-300) in the scout |
| V5 | The kill zone, close: mines, crews in the snow, 155 mm landing | 100 (07:16) | 10 | (1060,-250,140) → (1040,-290,140) | Col(2) | 8→7 | on | fire He at 76 and 84 (lands about 102 and 110); fates and bail-outs |
| V6 | TOS-1A on the southern blocks | 163 (07:38) | 10 | (-110,-900,160) → (-130,-925,160) | Point(-290,-1170) | 20→16 | on | rockets land 165–175; from the drone, no parapet |
| V-a | card | | 2.5 | | | | | |
| V7 | The second column passes the morning's wrecks at the middle belt | 470 (09:29) | 10 | (1190,-50,150) → (1170,-100,150) | Wave(2,0) | 10→8 | on | `Vehicle(0)` would pick wave 1's wreck, hence `Wave` |
| V-b | card | | 2.5 | | | | | |
| V8 | Alga's BMP-1s in the shoulder mines on the Pavlivka road | 790 (11:24) | 10 | (-740,-240,150) → (-760,-280,150) | Wave(3,1) | 10→7 | on | |
| V9 | Flown: the operator pushes in over the third column at its wrecks | 1036 (12:52) | 12 | start (1250,-80,160); legs 0–3 hover, 3–9 `w+`, 9–11 `z`, 11–12 hover | look Wave(4,0) | 20, then by stick | on | fallback: placed (1120,-150,150) → (1100,-190,150), Wave(4,0), 12→8 |
| V-c | card | | 2.5 | | | | | |
| V10 | Infantry on foot under the mortars | 1190 (13:48) | 10 | (-900,300,100) → (-910,275,100) | MenNear(-950,250) | 9→7 | on | 100 m up, a man about 120 px tall at 1080p |
| V11 | Late afternoon over the dacha road: everything that came down it is still there | 1440 (15:18) | 8 | (1060,-630,220) → (1090,-570,220) | Point(1040,-330) | 22→18 | off | |
| V12 | The ledger | 1448 (15:21) | 7 | as V11 end | debrief | 18 | debrief | before t=1500 |

### Part two: south of Мала Токмачка (Mala Tokmachka)

Geometry (`src/robotyne.rs`): western track x≈-492, eastern x≈665; `UA_LINE_Z=-900` with the overwatch team
at (-520,-896); forward barrier z -430..-250, main barrier z 110..590; `TRENCH_Z=702`; water tower
(-1150,-1470). Clock 03:50 + 0.30 min per second. Sun: -3.8° at t=110, +2.1° at t=200; filmable from about
t=130 (the old opener at t=20 was black). Hands-off pacing from `devlog/runs/20261011-rob-day7-smoke.log`:
lead at z=-1066 (t=120), -809 (180), -552 (240), -390 (300); first kills 300–360; second group 330; rescue
640, pick-ups 660–720; nothing moves after 960.

| # | shot | from (clock) | s | pos0 → pos1 | aim | fov | hud | notes |
|---|---|---|---|---|---|---|---|---|
| R | intertitle card | | 5 | | | | | |
| R1 | The guns in the dark: 155 mm on the crest trench | 60 (04:08) | 6 | (-760,-120,200) → (-770,-100,200) | Point(-930,700) | 10→8 | off | prep mission 8 lands 61–71 there; drop if the sheet reads black |
| R2 | First light: the first group moves off down the western track | 132 (04:29) | 9 | (-400,-1150,160) → (-410,-1115,160) | Col(3) | 14→10 | on | nine vehicles pass under the camera |
| R3 | Ground, eye level: the Leopard 2R with its plow passes the overwatch position | 152 (04:35) | 10 | team nearest (-520,-896), sight off | Ground, head follows the nearest attacker vehicle | 54 | on | the lead crosses z=-900 at about 159, 18 m from the eye |
| R4 | The plow in the forward minefield: the lane, the first mine | 318 (05:25) | 12 | (-420,-470,140) → (-430,-440,140) | Col(0) | 8→7 | on | "double mine under the plow" |
| R-a | card | | 2.5 | | | | | |
| R5 | The second group, eleven, on the eastern track | 400 (05:50) | 9 | (720,-700,150) → (700,-660,150) | Wave(2,0) | 12→9 | on | |
| R6 | Stopped vehicles under the Ka-52s; crews out | 455 (06:06) | 12 | (-560,-430,150) → (-570,-400,150) | Col(1) | 9→7 | on | helicopters from 430 |
| R7 | The men in the field between the burning vehicles | 560 (06:38) | 10 | (-540,-380,110) → (-550,-355,110) | MenNear(-492,-300) | 8→7 | on | |
| R-b | card | | 2.5 | | | | | |
| R8 | Flown: trailing the rescue Bradleys up the lane, smoke blooming ahead | 668 (07:10) | 14 | start (-497,-780,120); legs 0–14 `w` | look Wave(3,0) | 12 | on | smoke called at 646 so it blooms 672–680; fallback placed (-430,-420,130) → (-440,-390,130), Wave(3,0), 9→7, from 690 |
| R-c | card | | 2.5 | | | | | |
| R9 | Walking back: the men go down the slope past the wrecks | 1010 (08:53) | 8 | (-560,-700,120) → (-570,-675,120) | MenNear(-495,-600) | 9→8 | on | owner's option: the water tower (`Aim::Op`) instead |
| R10 | Late morning: the row of wrecks along the track, still smoking | 1300 (10:20) | 8 | (-600,-520,200) → (-590,-480,200) | Point(-492,-330) | 14→12 | off | "Quiet on the lanes. Only the smoke." plays in-shot |
| R11 | The ledger | 1308 (10:22) | 7 | as R10 end | debrief | 12 | debrief | |

Every `from` above is a first estimate from the beat table and the day-7 log; packages A–L changed the pacing,
so the scout run (verification 1) retimes V4, V5, V7, V8, V9, R4, R8 and the `MenNear` points.

## Code changes

### `src/showcase.rs`

1. **Two tours.** `shots()` becomes `shots_minute()` (today's two lists, untouched) and `shots_film()`;
   `SHOTS` picks by a new `dev().tour` (`BAV_TOUR=film`; unset keeps the minute tours byte for byte, so
   `scripts/same-day.sh` baselines stay valid).
2. **`Shot` fields:** `card: Option<Card { title, line, note, secs }>`, `hud: bool` (default true),
   `fly: &'static [(f32, f32, &'static str)]` (legs relative to `from`), `launch: Option<f32>` (ground shots).
   `Aim` gains `Ground(Vec2, bool)` (nearest defender `Post` with missiles; the bool is the sight) and
   `Fly(Target)` (the initial look). `Aim::Op` stays for R9's option.
3. **Cards.** At a shot's first filmed frame with a card, `direct` appends
   `"{video_t} {secs}\t{title}\t{line}\t{note}"` to `<dir>/cards.txt`, then advances `tour.video_t` and
   `tour.frame` by the card's length so the frame numbers are reserved and `segments.txt` already includes the
   card time. The script renders the card and links it into the reserved `f%05d.png` range, so
   `showcase_mix.py` needs no offset logic beyond laying a bed under the card span.
4. **Ledger.** `show_debrief` (`src/hud.rs:476`) writes `<dir>/ledger.txt` when a showcase runs: one
   `label\tvalue` per row, and the closing lines prefixed `closing\t`. The end card reads both parts' files.
5. **HUD per shot.** `direct` writes a `HudHidden(bool)` resource each frame; one system in `hud.rs`
   (`Step::Screen`) sets `Visibility` on the same top-level nodes `show_debrief` hides, and restores them.
6. **Ground shots.** On the shot's first frame (and during the skip, so the camera is already there) pick the
   team, call `enter_ground` (`camera.rs:311`), set `view.sight`; each frame call a new
   `camera::track_nearest(view, vehicles)`, which is the tracker now inlined in `env_ground`
   (`src/dev.rs:425`), generalised from `Side::Ru` to `diorama().attacker()`; at `launch` set
   `view.launch = true` once, which `player_atgm` (`src/fire.rs:1148`) takes as a trigger beside the mouse and
   `BAV_AUTO_ATGM`.
7. **Flown shots.** At the shot's first filmed frame `place_camera` places once (pos from `pos0`, yaw and
   gimbal from `look_angles`, targets equal to them as `set_drone_aim` does, fov, zero velocity, `hold = None`,
   `locked = false`) and installs the shot's legs offset by `from`; then it touches nothing for the rest of the
   shot and `drone_control` flies. During the skip the shot is placed at `pos0`, locked.
8. **Day-end guard.** `director.rs:321`: the "quiet" exit is skipped while a showcase runs (`DAY_LENGTH`
   and the foothold exit stay); the debrief shot ends the day as today.
9. The module doc's `scripts/showcase.py` → `scripts/showcase_mix.py`.

### `src/camera.rs`

- `View.locked: bool`: the view is placed by hand this frame. `drone_control`'s early return
  (`camera.rs:426`) tests `view.locked` instead of `dev().showcase.is_some()`; `ground_control`'s sight toggle
  honours it too. `place_camera` sets it for every placed aim.
- `View.launch: bool` (item 6 above).
- A `Flight(Vec<(f32, f32, String)>)` resource holds the scripted legs; `DevPlugin` fills it from
  `BAV_FLY`, the tour replaces it per flown shot; `Stick::scripted` reads the resource.
- `Target` gains `Wave(u32, u32)` (that wave's vehicle by column index) and `MenNear(Vec2)` (the mean of the
  attacker's infantry within 500 m of the point, not dug in; the point itself when nobody is there, so a
  shot never holds a stale aim on an empty field, the fault of two earlier Robotyne shots).
  `FromStr` accepts `w/i` and leaves `Point` as is.

### `src/director.rs`, `src/hud.rs`, `src/fire.rs`, `src/dev.rs`

The one-line hooks named above: the quiet guard, `ledger.txt` and `HudHidden`, the launch trigger,
`BAV_TOUR` and the `Flight` resource.

### `scripts/showcase.sh`

- `scripts/showcase.sh [out]` and `BAV_DIORAMA=robotyne scripts/showcase.sh [out]` keep the minute mode
  exactly as now.
- `scripts/showcase.sh --film [out]` (default `devlog/videos/YYYYMMDD-HHMM-film.mp4`) and `--scout` (the same
  at 640x360, fast x264 preset, under `target/film/`):
  1. build once; run the binary for `vuhledar` then `robotyne` with `BAV_TOUR=film` and the usual
     headless env into `$work/<part>` (they share the GPU, so one after the other; about 15–18 min per part
     at 1080p, measured on the first scout and written into the header);
  2. cards: one `magick` call per line of `cards.txt` (Liberation Mono from `src/fonts`, 54/30/22 pt, the
     title colours of today's card), linked into the reserved frame range;
  3. `showcase_mix.py $work/<part> sound.wav 0`; encode each part with identical x264 and AAC settings;
  4. title and end card from the script's own table and the two `ledger.txt` files, each over a wind bed
     (`showcase_mix.py --bed <part> <secs> bed.wav`, the lead-in bed generalised), fade in on the title, fade
     out on the end;
  5. `ffmpeg -f concat` of title, part one, part two, end;
  6. a row in `devlog/videos.md`.

### `scripts/showcase_mix.py`

- Lay the wind-and-rumble bed under each card span from `cards.txt` with 0.3 s fades; add `--bed`.
- Level matching: normalise each part to the same RMS over its filmed segments (about -20 dBFS) before the
  soft clip; check with `ffmpeg -af ebur128` that the two parts are within 1 LU, and keep a `--gain-db` for
  a last correction.

## Verification

1. **Determinism of the game change.** `scripts/same-day.sh before` for both dioramas on the current tree;
   after the code lands, `scripts/same-day.sh after before` for both: audio.log, segments.txt and the tally
   must read `same`. Then run the film tour twice and `cmp` its audio.log, segments.txt, cards.txt and
   ledger.txt between runs (the flown shots and the launch included).
2. **Scout runs** (`--scout`) of both parts; a contact sheet per part, one tile per second
   (`ffmpeg -vf "select='not(mod(n,30))',scale=240:-1,tile=8x15"`); retime the shots; two or three rounds.
3. **Fault checklist on the sheets:** the launch and flight seen from the sight (V3); no parapet anywhere;
   a man at least about 90 px tall (V10, R7, R9); the ledger readable at 1080p (V12, R11); the dark opener
   legible or dropped (R1, R2); people framed, not an empty field (every `MenNear` shot); no horizon in any
   drone shot; no placed shot below fov 7; the flown shots framed, else their fallbacks; the HUD actually off
   on V1, V11, R1, R10 and on everywhere else; the Cyrillic on every card.
4. **Archive:** every sheet and one frame per shot into `game/devlog/shots/YYYYMMDD-HHMM-film-<part>-<shot>.jpg`
   with a row each in `devlog/shots.md`; the finished mp4 and its row in `devlog/videos.md`.
5. **Cut-point review of the final mp4:** the first and last frame of every segment and card
   (`ffmpeg -ss T -frames:v 1`) on one strip per part; the title, intertitle and end card at full size; the
   first 10 s and two card transitions played for the bed fades; total length 4:00–4:30; loudness within
   1 LU between the parts.
6. Watch it once end to end as a stranger would: does the first 20 s say what this is?

## Order of work

| step | what | effort | owner's eye |
|---|---|---|---|
| 1 | `same-day.sh before`, both dioramas | 15 min machine | no |
| 2 | Code: tour switch, `Shot` fields, cards and ledger files, HUD switch, ground and flown aims, `Wave`/`MenNear`, `track_nearest`, launch trigger, `Flight`, quiet guard, `shots_film()` typed from the tables | 4–5 h | no |
| 3 | `same-day.sh after before`, both; must be identical | 15 min machine | no |
| 4 | `showcase.sh --film/--scout`, cards, bed and RMS in the mixer, concat | 2–3 h | no |
| 5 | Scout runs and sheets; retime; the `MenNear` points; R1 keep or drop; flown shots keep or fall back; R9 drone or tower | 1 h a round plus 15 min machine, 2–3 rounds | yes |
| 6 | Full 1080p run, cut-point review, loudness, archive, `videos.md` | 1 h plus 40 min machine | yes: the viewing |
| 7 | Second cut from the owner's notes; README documents `--film`; a paragraph in `design/next.md` recording the decisions above | 1–2 h | yes: the card wording |

Without the owner the work reaches the end of step 5's first round: code, scripts, scout sheets, and a first
full-length scout mp4 to watch. The shot choices and the card texts are the owner's.
