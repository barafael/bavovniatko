# Hostomel: the third diorama, in two sub-chapters

Status: planned (2026-10-19). Progress is logged at the end of this file.

## Context

Vuhledar and Robotyne are both about a column on a road. Hostomel (Antonov airport, 24 Feb 2022) was picked as the
next chapter because it adds new play:
- the enemy comes by air;
- the player fires the Igla himself;
- the fight is infantry holding a place;
- the real stake is the runway: whether the Il-76 airlift can land.

It plays as two sub-chapters on one map, each with the Record and Command modes and the same map-table
planning phase (Allotment, placement, Support, Supply, orders over the radio, Enemy staff) that Command already
has. Sources: `db/research/chapters/c01-march-on-kyiv.md` (§1, §3, §4, §6) and `design/campaign.md` row 1.

The engine has none of the new pieces yet:
- **Air:** `robotyne.rs` has hovering Ka-52s that can't be hit except by an abstract roll. No landings, no
  transports.
- **Infantry:** defenders on foot don't move; the attackers' fire is cosmetic; there is no area control.
- **Missiles:** they only target `Vehicle`s, with no seeker and no flares.
- **Light:** there is no dusk or night outside Robotyne's dawn.
- **Diorama-specific code:** about 30 places in the crates branch on Vuhledar or Robotyne by name.

## The two sub-chapters

| | **I. The landing (Десант)** | **II. The counterattack (Контратака)** |
|---|---|---|
| window | 24 Feb, about 09:30 → 13:00 | 24 Feb, about 17:00 → 21:30 |
| player | the 4th NGU Rapid Reaction Brigade's rear echelon, fewer than 200 men | the 72nd, 80th and 95th, the 3rd SSO Regiment and volunteers, with artillery and Su-24s |
| enemy | about 34 helicopters (Mi-8, Ka-52, Mi-24/35) landing 200–300 VDV (31st Air Assault and 45th Spetsnaz Brigades) | the VDV holding the field, with Ka-52 cover |
| player's means | Igla teams (fired directly), ZU-23-2, small arms, a few BTRs, two D-30s at Horenka (fire missions) | ATGM, mortars and artillery, infantry assault groups, BMP-2/T-64, Su-24 bomb runs (Support) |
| light, weather | overcast at 93%, low cloud, light rain, 1–5 °C, thin snow | dusk at 17:31, then night; artillery fires and burning wrecks light the field |
| Record band | Helicopters down at the field: 2–5 (history is 3 Ka-52 + 1 Mi-8 there). The defenders hold for more than an hour, then leave by about 13:00 out of ammunition, with about 20 captured at the radar. The VDV hold the field. | The field is retaken by 21:00, the runway is cratered, and the Ukrainians pull back. No Il-76 lands. |
| what decides it | when to withdraw (leave too late and you are captured); spending Iglas on the right targets; keeping the runway unusable | clearing the zones, cratering the runway, and pulling back before the column (25 Feb) |

**The airlift is the stake in both.** 18 Il-76s with the 76th Division are on standby.
- In the Record they turn back in mid-air.
- In Command a rule decides: if the runway is held, intact and quiet at a check time, they land, and the day
  is lost outside history. The runway counts as not intact when there are craters, wrecks or wrecked
  helicopters on it.

**Carry-over.** Sub-chapter I writes an `Aftermath`: wreck positions (a downed Ka-52 on the runway is an obstacle),
which posts fell, and the Iglas left. Sub-chapter II starts from it:
- in Command, from the player's own I;
- in the Record, from the historical I, so the Record stays fixed.

## Phases

### 0. One diorama is data plus a plugin (no behaviour change)
- **Registry.** Replace the two-arm `Diorama` matches (listed by the exploration: `core` diorama/data/text/geo/field,
  `look` world/terrain/audio, `sim` director/staff/units/fire/player, `bavovniatko` hud/showcase/dev/main/camera,
  xtask reel) with a per-diorama registration:
  - a `DioramaSpec` from the `.ron` (names, light, briefing/debrief/sources text, OP label, time-of-flight
    table, hotkeys);
  - a plugin per diorama for the code that is truly specific (`spawn_set`, the Robotyne craters).
- **Generic dawn and dusk.** `daylight` becomes one function of latitude, date and weather. The Vuhledar and
  Robotyne curves stay as they are, behind their specs, so the Record does not move.
- **Sub-chapters.** `Diorama::Hostomel(Part)` loads `dioramas/hostomel/map.ron` (shared ground, areas, sets) plus
  `landing.ron` or `counter.ron` (battle part); `BAV_DIORAMA=hostomel-1|hostomel-2`.
- **Check:** `scripts/same-day.sh` for both existing dioramas is `same`, and the bands are unchanged.

### 1. The airfield map
- **Area features** in `geo::Layout`: polygons with a kind (runway, taxiway, apron, concrete, forest, base
  compound, railway, fence).
  - Terrain painting for each kind.
  - Forest is filled with tree instances by density, using the existing `trees.rs` crowns, plus a
    far-distance canopy impostor.
  - The runway (3,500 m at about 150°) fits the box diagonally, since `HALF_Z` 1792 is enough for its 3 km of
    north–south extent. Anchor it at the OSM aerodrome centre, 50.6048 N, 30.1909 E.
- **Procedural set** (`look/hostomel.rs`, `meshes.rs` builders):
  - the Antonov hangar (wide arched roof) and the Mriya's outdoor shelter;
  - the control tower, the five-storey office south-east of the strip, and the radar;
  - the railway with its single road crossing;
  - the NGU base barracks and the five-storey staff flats outside the gate, reusing `panel_block`;
  - fences, light masts, and parked Antonov aircraft as static props.
- **Light and weather.** A late-winter palette with patchy thin snow. A low cloud base, with the sky shader's
  cloud deck lowered to about 300 m so helicopters drop out of it. Light rain as a particle layer.
- **Heights.** Flat, about 160–180 m, with a `Noise` term and a shallow fall to the Irpin floodplain in the
  south-west.

### 2. Aircraft
- **Generalise `Helicopter` into `Aircraft {kind, hp, state}`**, with kinds Ka52, Mi24, Mi8 and Il76. It flies
  paths (the stream comes in low from the north-east along the Dnipro, about 15 m above the ground) with
  turn-rate and speed limits.
- **States:**
  - **Gunships:** Ingress → Orbit/Strafe. Ka-52 and Mi-24 make rocket and gun runs on spotted posts and
    soldiers, through the existing `Blast`, using `Spotted` in Command.
  - **Transports:** Approach → Flare → Land → Unload (about 16 VDV per Mi-8, spawned with `spawn_soldier` from
    the ramp, as `VState::Dismounting` does) → Lift → Egress.
  - **Il-76:** high approach, then landing, or turning back in mid-air.
- **Damage.** HP per kind. Hits from Igla, ZU-23 bursts and massed small arms; Ka-52 armour shrugs off rifles.
  - Outcomes: smoke, then forced landing or crash. The airframe falls on a ballistic arc and leaves a burning
    wreck, which is a runway obstacle and cover.
  - Kills go into `HitLog` (`note()` already takes non-vehicles) and the tally.
- **Script acts:** `Stream { waves: [(kind, count, lz)] }`, `Airlift { at, check }`. The Robotyne Ka-52s move
  onto `Aircraft` with their behaviour kept. Robotyne's same-day baseline is redone deliberately for this, and
  its bands rechecked.

### 3. The infantry fight
- **Both sides' men move.** `move_soldiers` drops the attacker-only gate, and goals come from orders, areas and
  posts.
- **Fire teams** (`Team`, 4–8 men) with fire-and-move between cover. `cover_near` gains buildings, hangars,
  wrecks and forest edge.
- **Small arms are real in both directions.** Soldiers wear down posts (`Post.strength`) as shells do. Posts and
  teams carry **ammunition**, so running dry forces a withdrawal (the history of I).
- **Areas** (`ControlArea`: runway north, runway south, apron, hangar, tower, base): control goes to the side
  with men inside and none of the other's, shown on the HUD and the map table. Men cut off when their area
  falls are captured.
- The Vuhledar and Robotyne Records must stay `same`. Any behaviour change for them is gated or rebaselined on
  purpose, with bands rechecked.

### 4. The Igla, fired by the player
- **A new `Mode::Manpads` view**, modelled on Ground/`player_atgm`, with a procedural 9K38 tube in view
  (`spawn_launcher`). Keys cycle the Igla teams as Ground cycles the ATGM posts.
- **Seeker:**
  - hold to uncage, the growl as the seeker locks onto the heat, then the lock tone;
  - the lock comes from aspect, range and background (cloud or ground clutter lowers it);
  - minimum range about 500 m, maximum about 5.2 km, ceiling below the cloud base.
  - The set piece: let the Ka-52 pass inside minimum range, then re-acquire it.
- **The missile:** `fly_guided`'s turn-limited integration and segment hit test, with proportional navigation
  toward an `Aircraft` `Transform`.
- **Flares:** a per-kind chance to dispense, low for the Ka-52 (RUSI: they often failed to), and a chance the
  missile goes for the decoy.
- **Limits:** a few missiles per team. A launch reveals the team (`Spotted`), and the gunships hunt it.
- **ZU-23-2:** an automatic AA post with bursts, tracers and limited ammunition. It is not player-manned.

### 5. Sub-chapter I content
- `landing.ron`:
  - the script (the Kalibr strikes at 06:00–07:00 shown as smoke already on the field, the stream at about
    10:30–11:00, the waves, the Ka-52 strafing, the Il-76 turn-back);
  - positions (the NGU posts, the radar, the tower, the hangar, the base);
  - the Ukrainian catalog (Igla team, ZU-23-2, rifle group, BTR, D-30 section off-map at Horenka) with costs,
    plus Support, Allotment, Supply (little: it is a rear echelon) and staff rules;
  - the staff rules: gunships go for spotted Iglas, and later waves shift landing zone after losses.
- The briefing, debrief and sources go into the spec, and the Record band into `design/hostomel.md`. The
  outcome is measured by `scripts/band.sh hostomel-1`, with tally lines for helicopters down, minutes held,
  men captured and the airlift.

### 6. Dusk and night
- **Light.** The dusk curve from phase 0, with fire lights from burning wrecks and buildings, flares and muzzle
  flashes, and illumination rounds as a Support item.
- **Thermal view.** A `FeedMode::Thermal` for the sights and the drone (the Javelin CLU shader path already
  exists, `sight=1` in `feed.wgsl`), with hot engines and bodies.
- Night lowers the ranges at which posts and soldiers see in `line_of_sight` users, except through thermals.

### 7. Sub-chapter II content
- `counter.ron`:
  - it starts from the `Aftermath`;
  - the VDV hold the areas, and Ka-52s stay on call for them until dark;
  - the Ukrainian side attacks: the Robotyne-style attacker path, orders and columns reused, entering along the
    E373 and from Horenka;
  - Support: the 72nd's artillery and Su-24 bomb runs (off-map: sound, shadow and bombs until a model exists).
- **The runway objective.** A `Runway` damage state counts craters and wrecks per 500 m segment. Fire missions
  on it can be called once the area is ours or under observation.
- **The withdrawal.** The win band wants it after the runway is cut and before a scripted column-arrival
  pressure.
- Bands: `band.sh hostomel-2`.

### 8. Balance, showcase, docs
- Band runs for both parts in the Record (against history) and Command (staff off ≈ Record, then on).
- Reel shots in the showcase, devlog stills and videos, the README keys (the Igla view), and progress in
  `design/hostomel.md`.

## Models still required (a job for a separate agent)

Run alongside phases 0–1, through the existing pipeline:
- sources in `db/research/assets/README.md`;
- downloads with `db/research/assets/fetch.py` into `db/research/assets/files/`;
- a `Spec` row in `tools/bake/src/spec.rs`, then `cargo run --release -- <id>` in `tools/bake`;
- the id in `look/src/baked.rs` (`IDS` and `veh!`);
- `CREDITS.md`, rewritten by the bake;
- each model checked with `BAV_LINEUP`.

The agent does not add `VKind`s or touch gameplay. It delivers baked `.veh` files, credits, and a lineup shot in
the devlog.

| id | model | candidate source (README line) | role | split | needed by |
|---|---|---|---|---|---|
| `mi8` | Mi-8 (Mi-8AMTSh) | Mi-8, tiunov.se, 17k ✓ (l.222); alt manilov.ap 35k | VDV transport, the bulk of the stream | hull + main rotor (turret role); tail rotor folded into the hull or a third part | phase 2 |
| `mi24` | Mi-24/35 Hind | Mi24, manilov.ap, 71k ✓ (l.224) | gunship in the stream | hull + rotor | phase 2 |
| `il76` | Il-76MD | helijah, 56k ✓ (l.248), dual attribution as noted at l.392 | the aborted airlift, landing in Command | hull only | phase 2 |
| `an225` | An-225 Mriya | An225, manilov.ap, 21k ✓ (l.251) | in its hangar; set piece | hull only, 84 m scale (`length`) | phase 1 |
| `zu23` | ZU-23-2 | ЗУ-23, LuckySkink9661, 264k ✓ (l.189), decimate to ≤20k | the NGU's AA gun | base + traversing guns (turret) | phase 4 |
| `btr4` or `btr80` | NGU BTR | BTR-4 Taher.Maknine 102k ✓ / S1Priv 28k ✓ (l.121–122); BTR-80 perceval-66 51k ✓ (l.118) | the "few BTRs" at the base (which type is a **research task**: check the 4th NGU Brigade's 2022 kit) | hull + turret | phase 5 |
| `bmd2` | BMD-2 | 2w, 10k ✓ (l.126) | not in either window historically. **Optional**: only if the staff or Command brings the 25 Feb column forward | hull + turret | — |
| `su24` | Su-24M | **none verified free** | the bomb runs in II | — | phase 7, **research task**: find a free, traceable source or keep it off-map |

Also for the agent, as research only: confirm the Ka-52 bake's scale next to the Mi-8, and report the
triangle counts and texture sizes of the new set against the 30k / 1024 defaults.

**Not models, built in code in phases 1 and 4:** the hangars, tower, office, radar, flats, fences, rail and
parked civil aircraft (`meshes.rs`), the Igla tube for the first-person view, the VDV and NGU figures (existing
`figure()` with new colours and kit), and the D-30s (off-map).

## Critical files
- `crates/core/src/{diorama,data,geo,text,field}.rs` (registry, area features, sub-chapters);
  `dioramas/hostomel/{map,landing,counter}.ron`
- `crates/sim/src/robotyne.rs` → new `crates/sim/src/air.rs` (Aircraft); `units.rs` (soldiers, teams, areas);
  `fire.rs` (`fly_guided`, `player_atgm`, posts' ammunition); `director.rs` (acts); `staff.rs` (rules);
  `allot.rs` / `supply.rs` (catalog)
- `crates/look/src/{world,terrain,sky,meshes,trees,feed,baked}.rs`, new `look/src/hostomel.rs`
- `crates/bavovniatko/src/{camera,hud,table,showcase,main}.rs`
- `design/hostomel.md` (new: decisions, vocabulary, bands, progress)

## Verification
- **After every phase:** `scripts/same-day.sh` is `same` for Vuhledar and Robotyne. The exception is phase 2's
  deliberate Robotyne rebaseline, recorded with its band check. clippy is clean; `cargo xtask smoke` passes, with
  new smoke entries `hostomel-1` and `hostomel-2`.
- **Each part:** `scripts/band.sh hostomel-1|2` in the Record lands in the historical band. Command with the
  historical allotment and the staff off overlaps it; with the staff on, it is recorded in `design/`.
- **Devlog:** headless shots of the map, the airfield set, the stream arriving, the Igla view with the lock, a
  crash on the runway, and the counterattack at dusk. A played session of each part.

## Progress

- 2026-10-19: planned. The models job (table above) runs as a separate agent.
- 2026-10-20: phase 0, first part (`625f317`). Each diorama's words, keys and enemy guns now come from its file;
  the code no longer asks which place it is, except where a set or a tally is truly the place's own (exhaustive
  matches, so a new diorama is flagged by the compiler). Same-day is `same` for both against `fx-v`/`fx-r`.
- 2026-10-20: phase 1, first part. The map from OpenStreetMap (`tools/osm/`): the runway (3,499 m at 154.6°, 50 m),
  taxiways as concrete, the railway, 172 areas of land cover painted by kind, 245 buildings raised from their
  footprints, the Antonov hangar under a barrel roof, pine woods (a new evergreen in the tree atlas). The box is
  500 m east and 700 m south of the runway's middle. `BAV_DIORAMA=hostomel-1|hostomel-2` loads `map.ron` +
  `osm.ron` + the part's file as one record. The sun is computed for 50.6 N on day 55 (sunset at 17:30 with the -0.83° horizon, the dossier
  17:31; civil dusk ends 18:04, nautical 18:45), under the overcast. **Assumptions to check:** the control tower's place (840, 380) and
  the radar's (520, 1000) are placed by eye; OSM is the map of 2026, and buildings put up since 2022 that are not
  named as such are still in.
- The models are blocked on a Sketchfab token (the agent's report: BTR-3E at Hostomel, so a BTR-82A stand-in under
  the id `btr80`; manilov.ap's Su24 is a clean source for the bomb runs).
- 2026-10-20: the review of phases 0-1 (eight findings, all fixed in `9d6825f`): the pine had taken two of summer's
  leaf cells (Robotyne's walnuts and broken trees), the hangar's walls faced in, the taxiway grid ran backwards, the
  runway's frame was measured at the box's cut, the observers sat in the VDV's tower at dusk, the railway was not
  drawn, empty radio lines, and comments could break the merged record (`uncomment`, with tests).
- 2026-10-20: phase 2, first part (`1c58a4d`). `air.rs`: sorties from `Act::Air` fly in at treetop height; Mi-8s
  slow, flare, land and put out sixteen VDV each, then lift and go; Ka-52s and Mi-24s circle the field and dive on
  our posts and men with S-8 salvos (`ShellKind::Rocket`); `AirHit` damages, a hit one smokes and turns away, a
  killed one falls and burns where it lands (`AirWreck`). The Robotyne Ka-52s are left as they were (their day
  stays the same) rather than moved onto `Aircraft`: a deliberate change from the plan. Models fall back to the
  Ka-52 until the bake lands. The landing's stream: 10 Ka-52, 8 Mi-24, 16 Mi-8 (256 VDV) in four waves from 11:00.
- 2026-10-20: phase 4, first part. `manpads.rs`: Igla teams and the ZU-23-2 as `UnitKind::Air` at positions; the
  teams fire by themselves; key 5 takes one (`Mode::Manpads`): standing, mouse look, RMB shoulders the tube, the
  ring (2.2°) holds a target, the growl quickens and turns to the steady tone after 1 s, LMB launches; under 500 m
  the warhead does not arm ("let him pass"). The missile leads its target, may be lost against the ground (half the
  time under 40 m, else 0.3), may take a flare (Ka-52 0.25 to put them out, then 0.45 to take one). First tuning:
  six down in ten minutes was far too many; the band is set in phase 8.
- 2026-10-20: phase 3, first part. `fight.rs` (Hostomel only, from the part's `fight`): posts carry ammunition
  (`Ammo`, spent by `fire::small_arms`), the VDV on foot wear them down (`return_fire`), a dry post pulls out and
  its men walk off towards Horenka (`Leaving`), Space orders everyone out (`SpaceKey::Withdraw`), a spent post with
  six of theirs round it is taken with its men, and five areas (hangar, apron, tower, base, radar) are whoever's men
  are in them; the HUD tallies helicopters down, VDV landed, our men out and taken, and the areas. First tuning, one
  seed (the Record): the field held until 12:49 (history about 13:00), 10 taken (about 20 at the radar), 40 walked
  out, 1 helicopter down and 8 driven off (history: 3 Ka-52 and a Mi-8 at the field), so the Igla's warhead is now
  1.8-3.6 against hp 2-3. Not yet: the men withdrawing are not shot at; capture uses the posts' men, not men on
  foot; the bands over seeds (phase 8).
- 2026-10-20: phase 5, first part. The landing's debrief counts the field's day (the stream, shot down by kind and
  by weapon, driven off, VDV landed and killed, our men lost, out and taken, held until, who has what, the airlift).
  `Act::Airlift`: in the Record the Il-76s turn back; in Command they land if the apron and tower are theirs and
  the runway has 1.8 km whole (`Runway`: 100 m stretches broken by shells, rockets and wrecks). The enemy staff
  spots our Igla launches and the ZU's bursts (`spot_air`) for its gunships (`rule("air")`). Ukrainian for the
  landing's lines. A fight's day runs its whole window (no "quiet" end). Rifle fire among the hangars tells less
  (`rifles: 0.3`): one seed now gives the field held until 12:44, 20 taken, 61 VDV killed (HUR: up to 50), and 6
  helicopters down, so the Record now has three Igla teams of two missiles; helicopters down varied 1 to 6 over the
  runs: the bands over seeds are phase 8's.
- 2026-10-20: phase 7, first part. The counterattack: the VDV in six posts round the apron (riflemen, no ATGMs),
  two pairs of Ka-52s on call until dark, the 72nd's guns working the field (`Prep` at dusk and three more through
  the evening), the Su-24s (`Act::Bombs`: heavy bombs along a line, twice on the apron, once along the runway), a
  company of BMP-2s and MT-LBs down the E373 and another up from Horenka (`horenka` line), infantry behind them, the
  airlift's hour at night, the order to pull back at 21:05. Its debrief (vehicles and men, VDV positions silenced,
  the runway's longest whole stretch, the airlift). The fight now gives the defenders' posts (whoever defends) the
  ammunition and the rifle share, and the attackers an `assault`: men who reached their goal go on to the nearest
  post still holding (per part: 0.06 for the VDV in the morning, 0.25 for ours at dusk). One seed each: the landing
  held until 12:29, 25 taken, 3 down, 43 VDV killed; the counterattack took three of six posts and the hangar by
  20:36, the apron contested. Not yet: the field fully retaken by 21:00 (more pressure, the player's guns), dusk's
  sight lines and thermals (phase 6), the carry-over from the morning.
- 2026-10-20: phase 6, first part. `bav_core::sun` (moved from the look): the sun's place for a diorama with a
  `sun`, and `sight`, how far men see as a share of the day's (1 in daylight, about a third at night); the fight
  keeps it in `fight::Sight`, and the posts' rifles and the attackers' return fire reach no further. The night picture
  is a drone's low-light mode (grainy, blue, the field readable), with the glow of the towns on the cloud. The
  counterattack has its own HUD lines (vehicles, men, VDV posts taken, the areas, the runway; 152 mm, smoke, the
  attack's order). Not yet: thermal views, flares and illumination rounds, the fires lighting the night.
- 2026-10-20: phase 8, the landing's first band (`scripts/band.sh hostomel-1 0 1 2 3`, which now prints the field;
  Hostomel has a minimal shot list so the bands can film it). The morning's wrecks carry into the evening
  (`air::Aftermath`: the Record's in `counter.ron`, the player's own kept at `$XDG_DATA_HOME/bavovniatko/` in
  Command). The VDV's return fire is capped at the nearest ten men on a post; the Record has the order to pull
  out (`Act::PullOut`, Record only: in Command it is the player's Space) at 12:04, since the posts are worn down by
  about 12:20. Band t7: taken 15/30/15/20 (history about 20), walked out 70/30/65/55, VDV killed 63/45/44/51 (HUR:
  up to 50), helicopters down 1/0/3/4 (history: 3 Ka-52 and a Mi-8 at the field), the field held until 12:04-12:08
  (history about 13:00, held "more than an hour"). Next: posts that last longer (cover, the base's walls), so the
  order can come nearer 13:00; more helicopters down without more kills per missile (more teams in the open, the
  ZU's ammunition); the counterattack's band.
- 2026-10-20: the models are in (the bake's `ecd6006`, merged): Mi-8, Mi-24, Il-76, An-225, Su-24, ZU-23-2, a BTR-82A
  standing in for the NGU's BTR-3E (`btr80`), and a BMD-2 (its turret does not turn: a one-piece scan). The
  ZU-23-2 stands at its post and its guns follow the nearest aircraft; the Mriya stands in the Antonov hangar, whose
  doors (the end towards the apron) are open.
- 2026-10-20: placement checked against the OSM's points (twelve stands, a covered stand at 913,477, the fuel farm,
  the threshold) and for play: the hangar's post moved out of the hangar to its west doors (915,650), the offices'
  out of the flight-test centre to (955,1075), in both parts. The control tower and the radar are still placed by
  eye (the OSM has neither).
- 2026-10-20: the aircraft unified (the plan's phase 2 as written): Robotyne's Ka-52s are `air::Flight::Station`
  (hanging behind the ridge, popping up to fire a Vikhr), and Command's air defence fires `AirHit`s, so a Ka-52 it
  brings down falls and burns. `robotyne.rs` keeps only the dug-in tanks; the look's helicopter models are
  `bav_look::air`'s for every diorama with aircraft. Robotyne's Record changes by it (its own stream of chance):
  new same-day baseline `u-r`; band (stopped of 20, killed) 17/27, 15/37, 19/33, 16/30 against 16/30, 17/31, 12/19,
  18/35 before.
- 2026-10-20: the thermal view (`bav_look::thermal`, where the file says `thermal: true`): key H on the drone; men,
  posts, engines and fires carry a heat marker of a colour nothing else is, which the feed's mode 6 draws white
  over a world flattened to one grey, with an uncooled sensor's bloom and noise.
