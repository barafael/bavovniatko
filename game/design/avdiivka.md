# Avdiivka: the fourth diorama, the northern shoulder in three parts

Status: planned (2026-10-21). Progress is logged at the end of this file.

## Context

Avdiivka (c11) is the campaign's keystone: the one hold-then-withdraw chapter kept at full length, the chapter
of the shell famine, and the point where the player's drones turn from eyes into weapons (`arcs.md` §6). The
whole battle runs four months and a 20 × 15 km salient. This plan builds its opening and its best-known duel on
one map, the northern shoulder: Stepove, the railway, the spoil heap and the coke plant's edge. The grind, the
pipe, the city and the withdrawal are later parts of the same chapter and are listed at the end as not now.

It was picked after Hostomel because nearly all of it is already in the engine:
- Russian armoured columns into a prepared kill zone, seen from a drone: Vuhledar;
- Bradleys and a dug-in defence: Robotyne;
- men on foot, posts with ammunition, areas held by whoever is in them, dusk and night, thermals: Hostomel;
- heavy bombs from off-map aircraft (`Act::Bombs`), the EW field, Lancets and Orlans through the enemy staff,
  the FPV, Command's map table, orders and Supply.

What it adds is small, and each piece is reused by later chapters:
- **a map turned to the attack** (the Russians come from the north-east, and the column game runs along z);
- **the spoil heap**, a hill the defenders see from and the attackers climb (Pokrovsk's terrikons later);
- **autumn** and a dry October (Kursk and Pokrovsk);
- **the Bradley in the player's hands**: a gunner's sight, 25 mm AP and HE, the TOW, and a tank that can be
  blinded and immobilised without being penetrated;
- **FPVs as a daily stock** instead of one flight, and cluster shells;
- **KABs** as the air war the player cannot answer.

Sources: `db/research/chapters/c11-avdiivka.md` (§1, §2 10–20 Oct, §3, §4, §5, §6 set pieces 1, 2 and 6, §10)
and `v02-stepove-bradley.md`; `design/campaign.md` row 10; `design/arcs.md` §1 and §6. The reference footage
is in `reference-footage/2023-10_avdiivka-columns` and `2023-12_stepove-bradley-vs-t90m` (local only).

## The three parts

| | **I. The first wave (Перша хвиля)** | **II. The second wave (Друга хвиля)** | **III. Stepove in the snow (Степове в снігу)** |
|---|---|---|---|
| window | 10 Oct 2023, about 05:40 → 12:00 | 19 Oct 2023, about 06:30 → 13:00 | 11 Jan 2024, about 14:00 → 16:00 |
| player | the 47th Brigade's drone operator and fire controller at Stepove | the same | the gunner of the second Bradley, then the FPV crew |
| enemy | the 1st Army Corps' 114th Brigade and the 41st Army's 90th Tank Division: columns of 20–40 vehicles from Krasnohorivka and Kamianka, Orlans, Lancets, Su-34s with KABs | the same, having learned: smaller groups under smoke, infantry dismounting earlier, more Lancets, and the heap stormed on foot with tanks at its base | a T-90M with a pair of BMPs and a section of infantry at the village, under smoke |
| player's means | the drone (thermal), the 55th Artillery Brigade's 155 mm with HE and cluster, mortars, RAAM at night's end, a stock of FPVs, ATGM posts (Stugna, Javelin), Bradleys and a Leopard 2A6 in hides, the observers on the heap | the same, less the night's mines, plus what the ledger of I left | two Bradleys, one of them yours; one FPV; the trench |
| light, weather | clear to half cloud, dry, 0 → 10 °C; sunrise 06:40, UTC+3 | overcast, warm (to 18 °C), wind gusting to 33 km/h | overcast, −3 °C, light snow on 12 cm of cover; sunset 15:56, UTC+2 |
| Record band | The line holds. Of about 60–80 vehicles sent on the shoulder, 25–40 burn or are left in the fields (10–15 of them tanks); we lose 2–5. Russian footholds in the fields short of the railway; Stepove, the heap and the plant ours. | The line holds. Of 80–100 sent, 35–55 lost; we lose 3–6. The heap's north-west face reached and held against; the railway south-east of Stepove not reached. | The T-90M is blinded and stopped by 25 mm fire, its crew runs, and an FPV burns it. The trench holds. No Bradley is lost. |
| what decides it | where the drone looks, what the guns are spent on, when the ATGM posts and the Bradleys open fire, and whether the FPVs go on armour or men | the same, plus keeping the heap: the observers and the drops on the men climbing it | the second run: AP or blinding fire, the TOW, and when to break off |

**The field remembers across the parts.** II starts from I's `Aftermath` (the wrecks in the fields), and III from
II's, under snow: the January duel is fought among October's hulls. In the Record each part starts from the
historical aftermath in its file; in Command from the player's own, as at Hostomel.

**The famine is not yet here.** October is before the ration bites (c11 §5: the supplemental stalled from
October, 5:1 by February). I and II give the guns a working day's rounds, and the debrief counts them. The
chapter's later parts will set the ration from history (`campaign.md` open question 12).

**Three modes of seeing, one new.** The drone (1, I for thermal), the observers on the heap (2), a tree line
behind an ATGM post (3), the FPV (4), and new: a Bradley's gunner's seat (6). The views are a person each, as
always.

## The map

**The box.** The game's fixed 4608 × 3584 m, turned so that +z points north-east, the way the attack comes.
Centre 48.191 N 37.705 E; about 74,200 m per degree of longitude there. In that frame (to be checked against the
OSM once fetched, as Hostomel's were):

| place | map (x, z) | role |
|---|---|---|
| Stepove (48.1959, 37.6790) | about (1700, −980) | the village, the 47th's anchor, the duel's site; the water tower the observers' second post |
| the AKHZ spoil heap (48.1826, 37.7140), top 231 m over a base of 180–200 | about (−1160, −190) | the observers' post and the 110th's trench; the thing II is about |
| the coke plant's north edge (about 48.176, 37.702) | about (−1060, −1320), the works filling the box's south-west corner | the backdrop and the rear; the FPV crews fly from it |
| the railway (OSM) | from the north-west edge past Stepove to the plant | the kill zone's far edge: the field between the village and the rails |
| Krasnohorivka (48.2067, 37.7245) | about 460 m beyond the north-east edge; road head at about (180, 1792) | the northern columns' start line |
| Vesele and Kamianka | beyond the east and south-east corner; road head at about (−2304, 1500) | the eastern columns' start line |
| Berdychi, Lastochkyne | beyond the west edge | our rear, Supply's road heads |

The Russians enter at +z and advance to −z, Vuhledar's convention, so the grey zone, `defence_z`, `foothold_z`,
`jam_z` and the mortar band hold as written. The southern pincer (Vodiane, Opytne, Sieverne) is outside the box
and outside this plan.

**The ground.** Flat at 195–205 m, falling a little south-west to the plant; the heap a 35–45 m cone with a flat
top and terraces (c11 §3: SRTM, not Kyiv Post's 150 m); the quarry lakes outside the box. Harvested fields,
stubble and sunflower stalks; shelterbelts turning; the railway's embankment and the concrete wall along it
north of the plant, which gave the defenders their fire control.

## Phases

### 0. The map turned (no behaviour change)
- **A bearing on the anchor.** `Anchor` gains `bearing`, the compass bearing of the map's +z (180 today). Where
  it is 180 nothing changes, to the bit:
  - `tools/osm/osm2ron.py` projects through it;
  - `bav_core::sun` turns the azimuth into the map's frame (the look's sky and the baked shade read that);
  - the HUD's heading, compass letter and lat/lon go through it (`camera.rs`, `hud.rs`);
  - the tally's drone position is in map metres and stays.
- **Season.** `summer: bool` becomes `season: Winter | Summer | Autumn`, the two existing days mapped as they are.
  Autumn gets its own ground palette (stubble, dark plough, dust) and leaf cells in the tree atlas (yellow and
  bare rows; the pine and the summer cells untouched, which the Hostomel review showed must be checked).
- **Sub-chapters.** `Diorama::Avdiivka(Part)` reads `dioramas/avdiivka/map.ron` + `osm.ron` + `wave1.ron`,
  `wave2.ron` or `stepove.ron`; `BAV_DIORAMA=avdiivka-1|2|3`.
- **Check:** same-day `same` for Vuhledar (`fx-v`), Robotyne (`r2-r`) and both Hostomel parts; the bands unchanged.

### 1. The shoulder's map
- `tools/osm/fetch.sh avdiivka` with a box about 48.165–48.215 N, 37.650–37.750 E; `osm2ron.py` with the bearing.
  Stepove's houses, the plant's blocks, the railway, the belts, the fields; the heap's outline (OSM `landuse` or
  `man_made=spoil_heap`, to be seen) painted as a new `AreaKind::Spoil`, black-grey with the terraces' lines.
- **A `Mound` height term** (centre, radius, height, the flat top's radius) for the heap, and a slope term for the
  fall to the plant. The heap's slope slows men (phase 5).
- **Procedural set** (`look/avdiivka.rs`, `meshes.rs`): the coke batteries and their chimneys, the quench tower and
  the coal towers along the box's south-west edge, built like the Antonov hangar; the wall along the rails; the
  water tower and the grain store at Stepove; the dachas' sheds. Stepove's houses from their OSM footprints.
- **Light and weather.** The sun computed for 48.19 N on days 283 (UTC+3) and 292, and day 11 (UTC+2). Dust behind
  the columns on 10 October; the gusting wind of 19 October in the drone's flight and the smoke; snow cover on
  11 January with Vuhledar's winter palette.
- **Assumptions to check:** the OSM is the map of 2026 and the plant is in ruins there; the village as it stood
  in 2023 may have fewer buildings than the map. The heap's terraces are drawn by eye from the footage.

### 2. The vehicles and the air
- **T-90M** as a `VKind` (Relikt wrap; the catalogue's `minehffd` scan, decimated), the one tank with a model of
  its own at this date; the T-72B3 and T-80BV ride on the existing `T72` and `T80`; BMP-1, BMP-2 and MT-LB exist.
- **KABs.** `Act::Bombs` already flies a line of heavy bombs from an off-map carrier; the KAB is that with the
  glide's long stand-off, the whistle, and `Bombs` aimed by the staff at what it has spotted (a post, a hide,
  the plant). In the Record a few a morning (c11: "massed air strikes" on 10 Oct; 30–60 a day only by February).
- **Su-25 rocket runs**: the catalogue has a free, traceable model after all (the table below), so they can follow
  the KABs: an `Air` sortie on a fixed-wing path, a rocket run on a spotted post, and a chance for the Iglas the
  47th had with it. The five aircraft Ukraine claimed in ten days are then a band, not a closing line.
- **Damage without penetration.** A hull gains states short of `destroyed`: **blinded** (optics gone: it stops,
  turns, and its crew bails after a while), **immobilised** (a track), **engine** (it rolls to a stop and smokes).
  25 mm AP and HE, 30 mm and ERA pops land in them by aspect and range; the hit feed says which. Vuhledar's and
  Robotyne's days stay `same` because nothing there fires 25 mm at a tank today (the Bradleys at Mala Tokmachka
  fire at posts), which the same-day check proves.

### 3. The Bradley in the player's hands
- **`Mode::Bradley`** (key 6), modelled on the tree-line view and the Igla: the gunner's seat of a Bradley in
  its hide or on a run. The sight with a day and a thermal channel (the ODS's thermal sight; the feed's mode 6
  from Hostomel), the reticle stabilised while the hull moves, the 25 mm with a selector (AP, HE) and the ready
  boxes' counts, the coax, and the TOW (two tubes, a long reload, the wire's flight through `fly_guided`).
- **The driver takes orders.** The player does not drive: G / K / B order the pair on, to hold, or back, as in
  Command, and the second Bradley is the pair's other half (the duel's "trapped between them").
- **The AI Bradley** (phase 6's defence): a vehicle unit in a hide engages what it sees within about 2 km, AP on
  BMPs and MT-LBs, the TOW on tanks, HE on men; it reverses under smoke when hit (the footage).
- **Limits.** A launch of the TOW and a long burst reveal the hide (`Spotted`), and the Lancets come for it. The
  25 mm does not penetrate a tank's front: the blinding fire is the set piece, as the gunner told it.

### 4. The drones as a stock
- **FPVs by the day.** `SupportKind::Fpv` becomes a count per day (about 20 a day per strike company by autumn
  2023, `arcs.md` §6; the Record gives the shoulder a dozen), flown one at a time from the plant's edge with the
  existing goggles, link breakup and dive. The headless `BAV_FPV` flies them for the bands.
- **The drop.** The recon drone carries a few munitions and drops on a key (roadmap 7.2): a ballistic fall in
  the day's wind, a VOG-sized blast on men. II's heap is where it is introduced: men climbing the face under
  drops. The picture shows the release and the fall.
- **Their side.** Orlans and Lancets through the staff as now; the Lancets prefer Bradleys and the Leopard. The
  EW field with jammers on the vehicles themselves only from III (c11 §5: February).

### 5. The heap and the infantry
- **Men climb.** `move_soldiers` reads the ground's slope: the heap's face slows men to a crawl and stops
  vehicles at its foot. The attackers' goal is the top; the defenders' trench is on the first terrace.
- **Sight from height.** The observers and the trench on the heap see over the fields (the terrain already
  decides `line_of_sight`; the heap's height is new ground, not new code). Smoke on the heap blinds them.
- **Areas** (Hostomel's `fight.zones`): Stepove, the heap, the rail line, the dachas; held by whoever is in
  them; the HUD and the table show them. Posts carry ammunition and are worn down by the VDV-style return fire,
  both from Hostomel.
- **Cluster shells.** `ShellKind::Cluster`: a wide sheaf of small blasts on men and soft vehicles, what the 55th
  used to drive infantry into cover before HE flushed them out (c11 §5). Shift+click in I and II.

### 6. The defence
- **Positions** from the field (`field.rs`): the posts along the rails and in the village, the hides behind the
  belts, the trench on the heap, the plant's edge.
- **Units** (the catalog): rifle posts, Stugna and Javelin posts, a Bradley (and its pair), a Leopard 2A6 in a
  hide, the observers, the drone team; **Support**: 155 mm HE, cluster, mortars, RAAM, the FPV stock, Mavics;
  no HIMARS on the shoulder in the Record (a Command item). Allotment points and Supply as Command has them;
  Supply's road heads at the west edge, the Lastochkyne road.
- **Mines by night.** The RAAM of Vuhledar laid in the dark before dawn of I (the footage's pattern: mines on the
  approaches, then drones, then Bradleys), as pre-laid belts in the Record and a player's item in Command.

### 7. Part I content
- `wave1.ron`: the clock from 05:40 (the day in about 1500 s), the fire preparation (`Prep` and a `Tos` on the
  plant's edge: "the strongest fire preparation in Donbas"), three columns of 20–30 (T-72B3 and T-80BV leading,
  BMP-1, BMP-2 and MT-LB, infantry riding) from the north-east road head at Stepove and from the south-east at
  the heap and the rails, 20 minutes apart, under smoke; `Infantry` groups behind them; Orlan passes; the staff's
  Lancets; three KABs in the morning on the plant and the village; the net's lines in the 47th's voice.
- The Record's allotment: two Bradley pairs, a Leopard, four ATGM posts, the heap's trench and observers, the
  55th's rounds, a dozen FPVs, a RAAM belt. The HUD tallies vehicles committed, burning, left in the field, ours
  lost, FPVs left, rounds left, the areas.
- The debrief in the day's own terms: what came down each lane, what stopped it, what it cost, what is left,
  with the closing lines (the day across the whole salient: 70 vehicles by the geolocated count; the second
  wave to come).
- Band: `scripts/band.sh avdiivka-1 0 1 2 3`, with tally lines for vehicles by kind, the areas, and the stock.

### 8. Part II content
- `wave2.ron`: 19 October, overcast and gusting; starts from I's `Aftermath`. Four columns of 20–25 (the General
  Staff's "almost 50 tanks and over 100 AFVs in a day" across the salient, half of it here), smaller groups
  under more smoke, the infantry dismounting at the belts; the heap stormed on foot with tanks at its foot
  (c11 §6 set piece 2, folded in from about 16 October: a deliberate conflation, flagged in the debrief's
  sources); the staff with more Lancets and the KABs on the heap. The closing lines say the heap fell on 28
  October.
- The Record's allotment as I's less what I lost (the ledger carries), with no RAAM.
- Band: `band.sh avdiivka-2`.

### 9. Part III content
- `stepove.ron`: 11 January 2024, about 14:00, snow; starts from II's `Aftermath` under snow. The Record is
  small: a T-90M with two BMP-2s and a section of infantry come into the village's east end under smoke and
  fire on the trench; the first Bradley's pass fails by script; the player is the second's gunner; an FPV is
  on the stock for the finish. The sources give no more of that day, and the part invents no more: it is a
  vignette (twenty minutes of clock), and the full winter day at Stepove (27 December's 21 tanks and 14 IFVs)
  is an alternative part if the owner wants a third full day.
- The debrief: the duel as the hit feed saw it, the trench, the pair, and the closing line (the brigade's
  caption; the Bradley record of the winter, in the day's own words).
- Band: `band.sh avdiivka-3`, with a scripted gunner for the headless runs (`BAV_GUN="…"`, like `BAV_FIRE`).

### 10. Balance, showcase, docs
- Band runs for all three parts in the Record against the bands above, and in Command (staff off ≈ Record,
  then on).
- Reel shots and the film's shot list; devlog stills and clips; the README's Avdiivka section and the key for
  the Bradley; progress here.

## Models still required (a job for a separate agent)

Through the existing pipeline (`db/research/assets/README.md`, `fetch.py`, a `Spec` row in
`tools/bake/src/spec.rs`, the id in `look/src/baked.rs`, `CREDITS.md`, a lineup shot):

| id | model | candidate source (README line) | role | split | needed by |
|---|---|---|---|---|---|
| `t90m` | T-90M Proryv | T-90M, minehffd, 206k ✓ (l.70), decimate to ≤30k; dual attribution as noted at l.52 | the duel's antagonist; a few in the October columns | hull + turret | phase 2 |
| `t72b3` | T-72B3 | T-72B3, MirzaArrafiERV_45, 83k ✓ (l.74) | the October workhorse; **optional**, the `T72` bake stands in | hull + turret | — |
| `t80bv` | T-80BV | T-80 base, MirzaArrafiERV_45, 45k ✓ (l.73), Kontakt-1 added in code | 65 lost over the battle; **optional**, `T80` stands in | hull + turret | — |
| `su25` | Su-25 | "[PBR] Sukhoi Su-25", Immersive3A, 16k ✓ (README §8, l.234; re-verified 2026-10-21 by the models agent: CC BY, the uploader's own, no rip admission); alt tnikita 16k ✓ | rocket runs in I and II, once phase 2 gives `Air` a fixed-wing path | hull only | phase 2, after the KABs |
| `kamaz` | a truck | untraceable credits (l.50): **none** | Supply's columns on the Lastochkyne road; **optional** | — | — |

Not models, built in code: the Bradley's sight and reticle, the coke plant's set, the heap's terraces, the
water tower, the drop munition and the cluster sheaf, the T-72B3's and T-80BV's reactive armour as part of the
existing bakes if the optional rows are skipped.

## Critical files
- `crates/core/src/{data,geo,sun,diorama,field}.rs` (the bearing, `Mound`, `Spoil`, the season, the parts);
  `tools/osm/{fetch.sh,osm2ron.py}`; `dioramas/avdiivka/{map,osm,wave1,wave2,stepove}.ron`
- `crates/sim/src/units.rs` (the new hull states, the T-90M), `fire.rs` (cluster, 25 mm, the TOW),
  new `crates/sim/src/bradley.rs` (the gunner's seat), `drones.rs` (the stock, the drop), `fight.rs` (slopes,
  areas), `staff.rs` (KABs, Lancet preferences), `director.rs` (acts as needed), `allot.rs` (catalog kinds)
- `crates/look/src/{terrain,trees,world,sky,meshes,feed}.rs`, new `look/src/avdiivka.rs`
- `crates/bavovniatko/src/{camera,hud,table,showcase}.rs`
- `design/avdiivka.md` (this file: decisions, bands, progress)

## Verification
- **After every phase:** `scripts/same-day.sh` is `same` for Vuhledar, Robotyne and both Hostomel parts, except
  where a phase changes a day on purpose, recorded here with its band check. Clippy clean; `cargo xtask smoke`
  passes with `avdiivka-1|2|3` added.
- **Each part:** `scripts/band.sh avdiivka-1|2|3` in the Record lands in the band; Command with the historical
  allotment and the staff off overlaps it; with the staff on, it is recorded here.
- **Devlog:** headless shots of the map from the heap, the first column under the drone at dawn, the cluster
  sheaf on dismounting infantry, the Bradley's sight on a T-90M, the FPV's dive on the stopped tank, and the
  village under snow among October's wrecks. A played session of each part.

## Not now (the chapter's later parts, kept on record)
- **The grind (November to January):** infantry assaults of 5–7 men at night, mud and fog that ground the drones,
  the promka ground away, the ration falling. Needs fog, night infantry on both sides, and day ticks on the map
  table.
- **The pipe (mid-January)** and the dachas in fog: c17's mechanic, hidden-axis events the player blunts by
  spending ISR on the entry.
- **The city and the withdrawal (February):** blocks, the neck, Zenit, the night walk-out, the 3rd Assault as a
  one-time card, the withdrawal mini-game. Needs the town builder (Vuhledar's panel blocks are the start), the
  belief-state table, and the tone test from `campaign.md` open question 11.
- **The southern pincer** (Vodiane, Opytne, Sieverne): another box.
- **Bombers falling (February):** the Patriot ambushes are c12's interlude.

## Progress

- 2026-10-21: planned. The models job (table above) runs as a separate agent.
- 2026-10-21: phase 0. `Anchor::bearing` (180 by default): `geo::east_north`, `map_offset`, `map_heading` and
  `true_heading` turn offsets and headings between the map's frame and the compass; `osm2ron.py` projects through
  it, the Hostomel look turns the sun's azimuth into the map's frame, and the HUD's heading and lat/lon go through
  it. `summer: bool` became `season: Winter | Summer | Autumn` (`Diorama::summer()` now means "no snow"). Same-day
  `same` for Vuhledar (`fx-v`), Robotyne (`r2-r`) and both Hostomel parts (new baselines `h0-1`, `h0-2`, the first
  for Hostomel).
- 2026-10-21: the T-90M is baked (`0aa85f0`, the models agent): 29k triangles, 1024 textures, the turret with its
  cage and Kord turning; the scale set over the gun-forward length (9.56 m) since the model's hull span includes
  the rear fuel drums. The agent found the Su-25 the catalogue already lists (table above).
- 2026-10-21: phase 1, first part. The box turned to bearing 75 (not the plan's 45: the attack on Stepove ran from
  Krasnohorivka in the east-north-east, and on the heap from Kamianka), its centre at 48.1898 N 37.6998 E: the
  village at about (1050, -1300), the heap at (-1066, 830), the plant along the -x end, Krasnohorivka 470 m beyond
  the +z edge. The map from OpenStreetMap (`tools/osm/fetch.sh avdiivka`): 89 roads, 105 rails (the disused lines
  kept: they were there on the day), 106 areas, 118 buildings; `man_made=spoil_heap` as a new `AreaKind::Spoil`,
  the heap raised by a new `Mound` height term (38 m, a flat top, terraces every 12 m). Autumn: the fields by their
  blocks as stubble, plough, fallow and standing sunflower stalks (`terrain::autumn_colour`), the map's farmland
  and grass left to that painting where there is no snow, June's crowns turned gold and rust cell by cell in the
  tree atlas, and the map's woods planted with the belts' kinds rather than Hostomel's pines. The building code
  shared with Hostomel (`look/buildings.rs`); `look/avdiivka.rs` raises the set and lights the day from the
  computed sun (clear in October, overcast in January). Three parts load (`avdiivka-1|2|3`); II and III are I's day
  with their own clock, season and sun until phases 8 and 9. **Assumptions to check:** the OSM is the map of
  2026, the plant in ruins there; the heap's terraces are drawn by eye; the routes and positions are placed by eye
  against the dossier's description and the field overlay (`BAV_FIELD=1`).
- 2026-10-21: phase 1, second part. The heap on the map's own outline (`Heap`: the `Spoil` area rises over 170 m in
  from its edge, its foot wandering and its benches shifted by noise, so it is not rings), the coke plant's skyline
  by eye inside its area (two batteries with their coal and quench towers, five chimneys, the fuel farm's tanks
  where the map has them; the map has the footprints but none of the heights), and Stepove as the `town` (the
  TOS, the field's ruin positions, the advance measured to the village's edge). The T-90M is a `VKind` (Relikt:
  a missile destroys it 55 in 100 against the T-72B3's 72; a carousel; the T-72's slow reverse and pivot), and
  part III's tank is one. The models merged (`74922c5`: the T-90M and the Su-25).
- 2026-10-21: part I's first band, hands-off (`scripts/band.sh avdiivka-1 0 1 2 3`, t1): of 74 sent, knocked out
  38/37/31/39 (destroyed 21–22, left 10–18), withdrew 17–26, Russian dead 352–377 of 678, our casualties 2–6, the
  player's rounds untouched. The plan's band (25–40 of 60–80) holds without a hand on it; what the hands-off day
  lacks is our own losses (no vehicles of ours are in the field yet: phase 6) and the guns' share (the brigade's
  120 registered rounds killed 0–2: the points and the sheaf want looking at in phase 10).
- 2026-10-21: phase 2, first part. `Cause::Autocannon`: a burst of 25 mm on a tank does not get through, and
  `resolve_direct_hits` rolls what it took instead: the sights (35 in 100: the hull drives blind on the ghost
  plan and the crew gets out), a track (25), the engine deck (12, on fire), or nothing but sparks; on a carrier it
  penetrates as a missile would. `Strike::Cluster` / `ShellKind::Cluster` (DPICM): a mission of two shells, each
  a rain of 24 small blasts over sixty metres, lethal to men in the open and nothing to a hull; Shift+click where
  the file says `shift_fire: Cluster`, `SupportKind::Cluster(n)` on the stock, "DPICM" on the HUD. KABs: the script's
  `Bombs` act now takes `say` (the net's line) and drops by the attacking side, so part I has three KAB pairs in
  the Record (on the village, the heap and the plant); in Command the staff's `kabs` budget (`kab_every` seconds
  apart, the rule `kab`) puts a FAB-500 on the freshest post or hide it has spotted, forty seconds after the
  warning. Not yet: the Su-25 runs (the model is baked), the blinded hull's turret spinning in the look.
- 2026-10-21: phase 3 and 6, first part. `UnitKind::Dug(Bradley)`: a whole Bradley hull-down at a hide (the
  layout's `hides` hold `Dug` units as they hold tanks), spawned through `spawn_vehicle_at` with `BradleyArms`
  (300 AP, 230 HE, 7 TOWs). `bav_sim::bradley`: by itself it works what it sees within 1.8 km out of the smoke,
  a TOW at a tank it can hold (`Missile`, 0.3–0.7 by range), AP bursts at everything else (a hit 0.2–0.7 by range,
  a tank's front taking a third of that), the turret following its aim. The player's seat (`Mode::Bradley`, key
  6, cycling the Bradleys still in action): mouse look turns the turret, the right button is the ISU at x4 (day
  TV, or the thermal channel on I), the left button a burst along the crosshair (X picks AP or HE: AP into the
  hull it meets, HE as six small blasts where the line meets the ground, for men), T a TOW that `fly_guided`
  steers from this sight as it steers the Stugna. The Record has two Bradleys in Stepove's hides and the tank in
  the third. `BAV_VIEW=bradley` starts in the seat, the turret on the nearest enemy vehicle. Not yet: the pair's
  runs (G / K / B to the driver), the first pass that fails by script in part III, our vehicles lost in the debrief,
  the 25 mm's own report (the 30 mm's stands in).
- 2026-10-21: phase 5, first part. `geo::slope` (the rise per metre over a four-metre step): men on a grade over
  0.1 slow to a scramble (a third of a walking pace on the heap's face), and a driver's wanted speed goes to
  nothing past a grade of 0.24, so the hulls stop at the heap's foot. Flat fields and the belts' swells are far
  below both thresholds, so the older days are untouched (same-day). A third route runs up the heap's north-east
  face to its top, for the storming parties. Not yet: the heap's areas held by whoever is on them (Hostomel's
  `fight.zones` want the fight's posts, which are the column game's posts here), the drops from the drone.
- 2026-10-21: phases 7 and 8, first part. Part II's own day: the first wave's wrecks laid along the lanes where
  its band left them (34 hulls, `old_wrecks` now each part's own), overcast (`overcast: true`, the computed sun
  behind cloud) with the wind gusting from the west (`wind`, which `day_wind` now reads from the file), four
  smaller groups under more smoke with the infantry dismounting at the belts, three KAB pairs, and from 08:15
  the heap stormed on foot (thirty groups up the face through the morning, tanks at its foot). Part III has both
  waves' wrecks under the snow and the T-90M. The jam line moved past the heap's far foot (1350), since the heap
  is ours to fly over. Not yet: parts II's and III's bands (phase 10), part III's duel as a script.
