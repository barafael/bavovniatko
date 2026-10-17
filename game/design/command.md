# Command: choosing, placing and ordering forces (plan, 2026-10-17)

## Context

Today each diorama is the historical day replayed: the Russian columns, the Ukrainian posts, the stock of shells
and the Ka-52 runs all come from `dioramas/<day>.ron`, and the player matters at the margin (spotting, fire
missions, ATGM fire discipline, evacuations, one FPV). The owner wants it to become a game that is fun: choose
your forces from what existed at that time and place under a limit, place them on the map table, call support,
and keep feeding the fight with columns ordered from points that come in during the battle, giving units orders
over the radio. The historical day stays as a mode of its own; it is also the yardstick for whether the
simulation is roughly right (the debrief's outcome bands).

This deviates from design principles P2/P3 (game/design/chapters.md: the world runs by itself; winning means
reproducing history) and from campaign.md's "agency is local, never strategic". The plan keeps them intact for
the historical mode and records the new mode as a deliberate exception (a design decision in design/).

## Vocabulary

Bookkeeping register (Latin first, Cyrillic after, as `Name::hud()`); on the net, Ukrainian first.

| concept | word | why |
|---|---|---|
| the historical mode (as now) | **Record** (Літопис) | the docs already call the history "the record" |
| the new mode | **Command** (Командування) | the player commands the forces, not just the fire |
| the pre-battle choice of forces | **Allotment** (Наряд сил) | *наряд сил і засобів*: the forces and means allotted to a task |
| its limit | **allotment points** | |
| a selectable thing (tank platoon, ATGM team, rifle group, battery) | **Unit** (Підрозділ) | the army's generic word |
| off-map support (artillery, HIMARS, air, drones, heli, AA) | **Support** (Підтримка) | not on the map; called, not placed |
| the in-battle points that buy deployments | **Supply** (Підвіз) | what comes up the road in the night: fuel, crews, vehicles |
| a group ordered in together | **Column** (Колона) | already the game's word; slowest vehicle sets the pace |
| where columns enter | **Road head** (В'їзд) | the end of a route at the map's edge |
| the no-placement band in front of the enemy | **Grey zone** (Сіра зона) | the soldiers' own word for it |
| a pre-computed path | **Route** (Маршрут) | already the code's word (`geo::Route`) |
| a holdable spot (tree line, trench, ruin, hide) | **Position** (Позиція), kinds: посадка, окоп, руїна, капонір | `Post` positions already exist; these generalise them |
| a radio order | **Order** (Наказ): Advance (Вперед), Hold (Тримати), Fall back (Відхід), Take position (Зайняти позицію) | `ColumnOrder` already has the first three |
| what the enemy knows of our forces | **Spotted** (Виявлено) | their drones, observers and contacts reveal our positions |
| the enemy's adapting layer over its script | **Enemy staff** (Ворожий штаб) | the script is their plan; the staff answers what happens |

Decided (2026-10-17): the player commands the Ukrainian side only (defending at Vuhledar, attacking at Mala
Tokmachka); English words with the Ukrainian beside; the battle keeps running while the map table is open (P
still pauses).

## Design

### Modes
- `Mode::Record` — exactly today's game: script, forces and stock from the `.ron`; same-day and the bands keep
  working; the debrief compares with history. Default for the first play of a chapter (campaign.md: linear first).
- `Mode::Command` — the new game, chosen at the briefing. The player's side is built from the Allotment and
  Supply. Debrief: the ledger against the Record's band, marked as not the record.

### The enemy in Command: script first, then the Enemy staff
The script stays primary: the enemy's loadout, tactics and opening moves are the day's (the same columns, TOS,
Ka-52s, registered fire). Over it an **Enemy staff** adapts as the battle goes, deterministically (its own seeded
stream, so same-day style checks still work per seed):
- **Spotted**: the enemy's picture of our forces. A unit is spotted when an enemy observer, drone (Orlan/Lancet
  overflights, a new enemy-drone pass) or vehicle sees it (`geo::line_of_sight`), when it fires (ATGM launches,
  a battery's counter-battery signature), or by Ka-52/radio intercepts; spotted positions decay unless refreshed.
- **Reactions**, each a rule with a cooldown and a budget from the day's own stock (no infinite enemy):
  - spotted posts and batteries draw artillery, TOS or a Lancet (today's `russian_artillery` already hunts posts —
    it reads Spotted instead of the true positions);
  - air or HIMARS strikes on a column make the next columns spread out, move under smoke, or wait for night;
  - a flank (our unit on a route the script's columns do not watch) diverts the next wave or sends infantry there;
  - repeated losses on one route shift later waves to the other route or to infantry on foot (as the record did
    at Vuhledar: armour, then men on foot).
- **Composition**: within the day's catalog of the enemy side, later waves trade a little (more IFVs or tanks,
  more infantry, a plow) by what killed the earlier ones (mines → plows; ATGMs → smoke and infantry first).
- In Record the staff is off: the day is the script, exactly as now.

### Data (per diorama, in the `.ron`, read by `bav_core::data` / `bav_sim::data`)
- `catalog: [UnitSpec]` — what existed at that time and place for the player's side: id, name (en, uk), kind
  (vehicle kinds + counts, or infantry team with weapon), allotment cost, supply cost, how many available, which
  positions it may hold (vehicle/infantry/ATGM), source note. Vuhledar: 72nd's T-64s, Stugna/Javelin teams, rifle
  groups, mortar section; Robotyne: Leopard 2A6, Leo 2R, Wisent, BMR-2, Bradley sections, overwatch teams.
- `support: [SupportSpec]` — off-map: 155 mm battery (shells), RAAM, HIMARS, smoke, FPVs, Mavic flights, MANPADS/
  long-range AD (shoots down Ka-52s on station), with allotment cost and uses.
- `allotment: (points, supply_per_minute, supply_start)`.
- `positions: [PositionSpec]` — named holdable spots: (id, kind, polygon or point+radius, capacity, side), the
  placement slots and the order targets. Generated in part (belt edges every ~60 m become посадка positions;
  `ua_positions`/`ru_positions` become trench/firing positions) and hand-annotated where it matters.
- `road_heads: [(id, point, routes)]` and `grey_zone: [polygon]` per side.
- `routes` grows into a **route graph**: nodes at junctions and positions, edges along roads/tracks/fields with
  a mobility class (road, track, field, mined?) per vehicle kind; precomputed at load (`geo`), shortest paths per
  (road head, position) cached. Existing `Route`s become paths in the graph.

### Simulation (`bav_sim`)
- Movement on arbitrary paths: generalise vehicle route-following from "one `Route` by arc length" to "a `Path`
  (polyline built from graph edges) by arc length" — `units::steer_and_move` already steers for a look-ahead point and
  projects with `lateral_near`, so a `Path` that implements the same Polyline API slots in. Defender vehicles
  gain the drive system (today only the attacker drives).
- `Column` entity: members, path, pace = slowest member's speed, spacing; arrives, then members take their
  `Position` (spread by its capacity) and switch to holding behaviour (Overwatch / dug-in fire).
- Units hold `Order` (per unit or per column), received by radio (a short delay; jamming can delay or drop it —
  the EW field already exists). Infantry teams get the same orders; their movement already goes to a goal.
- Allotment applied at setup: placed units spawn at their positions (posts, vehicles in hides, dug in); support
  sets `FireControl` stocks and enables the support kinds.
- Supply: a resource that accrues over game time; deploying a column spends it; columns enter at a road head.

### Interface (`bavovniatko`)
- **Map table** (the one 2D view the design already allows, design/next.md "the night before"): allotment list with
  costs and remaining points; drag units onto positions (grey zone and enemy side refused); support picks; then
  "To the field". Opened before the battle in Command, and during it (key) to compose and send a column and to
  give orders; the battle keeps running while it is open (P still pauses).
- Orders in the field: from the drone view, select a unit/column (click near it), then Advance / Hold / Fall
  back / Take position (click a position marker). Radio lines confirm, in the net's register.

### Phases of work
1. **Words and data**: vocabulary in design/ (a decision record), `Mode` switch, `catalog`/`support`/`allotment`
   in both `.ron`s with the Record forces as one valid allotment (so Record = Command with the historical
   allotment and no supply). Same-day must stay `same` in Record.
2. **Positions and routes**: position annotation (generated + hand), route graph and path cache in `geo`,
   a debug overlay (BAV_* switch) showing positions, routes, grey zone, road heads.
3. **Movement on paths**: `Path` in the driving systems (`units::sense_traffic` to `steer_and_move`) for both sides; columns with slowest pace; taking positions.
4. **Orders**: per-unit/column orders over the radio, with delay and EW; field UI for them.
5. **Map table**: allotment, placement, support picks; Command mode start.
6. **Supply and reinforcement columns** during the battle.
7. **Support** beyond today's (MANPADS/AD vs Ka-52, more drone kinds) as catalog items.
8. **Spotted and the Enemy staff**: the enemy's picture of our forces, then its reaction rules and composition
   shifts, each behind its own switch so a band can measure what it changes.
9. **Balance**: band runs of Command with the historical allotment and the staff off ≈ Record; then with the staff
   on; tune costs and reactions; record in design/.

## Critical files
- `game/dioramas/{vuhledar,robotyne}.ron`; `crates/core/src/data.rs` (spec types), `crates/core/src/geo.rs`
  (Layout, Polyline, Route → graph), `crates/core/src/diorama.rs`
- `crates/sim/src/data.rs` (SimData), `director.rs` (Act::Column, script), `units.rs` (VehicleSpec,
  spawn_vehicle, the driving systems `sense_traffic`, `drive_decisions`, `steer_and_move`, `track_marks`, ColumnOrder, Soldier), `fire.rs` (Post, spawn_positions, FireControl, stocks),
  `evac.rs` (the existing "send a vehicle to a place" pattern)
- `crates/sim/src/director.rs` `russian_artillery` and the script runner (the Enemy staff hooks in there), new
  `crates/sim/src/staff.rs` (Spotted, reactions, composition)
- `crates/bavovniatko/src/hud.rs` (briefing/debrief), `camera.rs` (picking in the field), new `table.rs` (map table)
- `game/design/` — a new `command.md` with the decisions and the vocabulary

## Verification
- Record mode: `scripts/same-day.sh after before` stays `same` after phases 1–3 (movement refactor must not
  change the attacker's driving in Record).
- Command with the historical allotment: `scripts/band.sh` bands overlap the Record's bands.
- Each phase: headless shots of the map table and the overlay; a played session per diorama.

## Progress

### Phase 1, words and data (2026-10-17)
- `bav_core::diorama::Play { Record, Command }`, from `BAV_PLAY` (the briefing will choose it, phase 5).
- Each diorama's file has a `catalog` of Units (`Post(weapon)`, `Hide`, `Vehicle(kind)`, `Evacuation`) and a
  `support` list (`Shells(n)`, `Raam`, `Himars`, `Smoke`, `Brigade(n)`, `Mortar(n)`, `Fpv`), each entry with its
  name in both languages, its allotment cost, its Supply cost (for phase 6) and how many there were; and an
  `allotment` with the points and the Record's picks. Vuhledar: 144 of 150 points; Mala Tokmachka: 304 of 310.
  The costs are first guesses, to be tuned in phase 9.
- `bav_sim::allot` builds the player's side from the picks and checks them (known, available, placed once, within
  the points); the posts, the tanks in their hides (`hides`, two at Vuhledar), the stock, the FPVs and the
  vehicles for the wounded are read from it, no longer from the file directly. The Record's vehicles are checked
  against the script's columns at start.
- `BAV_ALLOT` gives Command its picks until the map table exists.
- The Record is unchanged: `scripts/same-day.sh` gives the same sound log, segments and tally for both days.

### Phase 2, positions and routes (2026-10-17)
- `bav_core::field`: the Positions (trench: the diorama's own posts; hide; tree line: every 80 m along each belt's
  enemy-facing edge outside its gaps, and both edges of the north-south belts; ruin: every 220 m along the town's
  and the dachas' edges), each with the way it faces and whose ground it is on; the Road heads and the Grey zone
  from the diorama's file (`road_heads`, `grey_zone`; the tank `hides` moved there from the battle's part). Vuhledar
  has 235 positions, 76 of them ours.
- The Route graph: the roads and tracks every 40 m, junctions where two roads meet, and a 100 m field grid; a
  stretch crossing a tree line outside its gaps, or the river away from a road, is for men on foot only. Costs
  by Mover (tracked, wheeled, on foot) and going (road, track, field); a Dijkstra tree per road head and mover,
  built once; `Field::path` gives a smoothed polyline from a road head to any place, the shape `steer_and_move`
  already follows.
- The Allotment places units by position number: men where men can hold it, a tank in a hide or a tree line,
  nothing beyond our side of the grey zone. The Record's posts keep their numbers (its T-64 is now `@9`).
- `BAV_FIELD=1`: the overlay and the numbered list. The Record is unchanged (same-day: same on both days).

### Phase 3, movement on paths (2026-10-17)
- `units::RouteBook`: the routes vehicles drive, the layout's at their own indices and each Column's way appended
  as it is sent; everything that read `layout().routes[v.route]` reads the book. `geo::Route::hold` marks a way to a
  position.
- `bav_sim::columns`: a Column's way is the field's tracked route to a point 45 m behind the position, then up to
  it the way it faces, so the vehicles fan out abreast along its edge, facing out. They start in a line at the road
  head and keep the slowest one's pace (a tenth under it, `Vehicle::column_pace`); at the end the men go into the
  position, and the vehicles hold (on a holding route `Overwatch` does not withdraw, and the column order of the
  Record's breach does not reach them). Columns drive on either side: a holding route drives whoever is on it.
- The route graph closes our own minefields to vehicles where we laid them (Vuhledar's belts in front of the town
  and the roads' shoulders): the first test column drove through the 72nd's belt and lost its Leopard. The roads
  and the belts' lanes stay open.
- Our tanks holding a position fire as the dug-in tanks do (`defender_tank`); the hit feed names them as ours.
- `BAV_COLUMN` sends Columns until Supply and the map table exist, and logs their progress.
- Tested: Mala Tokmachka, a Leopard 2A6 and two Bradleys from the western track to the forward tree line, 1350 m,
  there by 330–380 s abreast at its edge; Vuhledar, the same from the T-0509's western end to the first tree line,
  round the end of our mine belt, by 210–400 s, no losses. The Record is unchanged (same-day: same on both days).

### Phase 4, orders (2026-10-17)
- `bav_sim::orders`: in Command each of our units takes Orders (накази) over the net: Advance (Вперед), Hold
  (Тримати), Fall back (Відхід), Take position (Зайняти позицію). A unit is our vehicles sent in together (a Column, or
  one of the script's waves where we attack), under a callsign (Грім · Hrim, Сокіл · Sokil, …); a vehicle sent for
  the wounded answers to its call, not to orders. The Record has none of it.
- The net: the player says the order at once; the unit has it four to eight seconds later, and up to half a minute
  later through jamming. Where the net is jammed hard it may never get there: the player hears nothing back, and the
  operator says the callsign is not answering. The net's jamming is the jammers' field reaching twice as far as it
  does for the drones' links (`Ew::radio_jam_at`): at Mala Tokmachka a unit in the minefield under the crest is cut
  off more often than not; at Vuhledar our Columns behind the town are never jammed. The orders draw from their own
  stream, so giving one shifts nothing else of the day's chance.
- Hold puts `Holding` on the unit's vehicles: the drivers stop where they are, going either way, and a unit holding
  a firing line no longer pulls back of itself. Advance lifts it and turns round a unit falling back. Fall back sends it
  back the way it came. Take position gives the unit a way of its own (`field::Routes::path_from`): back along the
  way it came to behind its rearmost vehicle (out of the tree line it holds, as it went in), then across the field to
  the new position, up to it from behind; the way it came stays the start of the new one, so it can still fall back.
- The route graph now weighs ground by whose it is: the grey zone costs three times as much to cross, the enemy's
  eight; and a search from anywhere starts at a node the mover can reach in a straight line. Our units go forward of
  our side only where there is no other way (Vuhledar's tree line 13 to 18: our own belts close the field behind the
  tree line, and the way goes up the road and along its front).
- The field view: O in the drone's view opens it. Our units are ringed and named over the picture, the positions
  vehicles can take ringed (green ours, yellow the grey zone), the picked unit's way ahead drawn; on the left the
  units with what each is doing (or the order still on the net) and the keys. A click on one of ours picks it; G / K
  / B give Advance / Hold / Fall back; a click on a ring sends it there. While it is open a click calls no fire.
- `BAV_ORDER="200:100:hold,450:100:position@18,800:100:back"` gives orders at game times (wave 100 is the first
  Column, the script's waves count from 1); `BAV_ORDERS=open` starts with the view open.
- Our vehicles coming back off the field are no longer counted in the enemy's tally at Vuhledar.
- The Record keeps the world it had: in the Record the orders add nothing to it, not even a resource. With only
  `OrdersView` inserted (read by nothing), the Record's day changed from 23 s on (same-day on Vuhledar; the same
  code replays itself exactly). Something in the day depends on the world's make-up rather than its contents —
  likely two systems with no order between them; worth finding before anything else has to join the Record.
  (2026-10-17, phases 7 and 8) The same build films Vuhledar's day one of two ways, run to run, loaded or not: the
  same to the last four seconds, and then, one time, three more bursts of machine-gun fire and one more loss of ours
  at 1436 s. Both phases' builds gave both. So the same-day check can say "different" at Vuhledar without a change
  to blame: compare with an earlier run of the other outcome (p8-v and head-v in devlog/same-day), or run it again.
  Ruled out so far (phase 9): Bevy's ambiguity report (schedule `ambiguity_detection`, with bevy's `debug` feature for
  the names) finds 30 pairs of unordered systems that conflict, but in the battle's own steps only `scheduled_columns`
  (idle without `BAV_COLUMN`); the rest are input, camera, sound and picture. The battle's hash maps are looked up by
  key, never iterated; it has no parallel iteration, no unseeded random stream, no wall-clock time. Still open.
- Tested headless: at Vuhledar a Column held, went on, took tree line 18 from 13 and fell back to its road head;
  at Mala Tokmachka hold and advance to both waves, the orders 4–14 s on the net, two lost under the jammers.
  Shots: devlog/shots/*-orders-*.jpg. The Record is unchanged (same-day: same on both days).

### Phase 5, the map table (2026-10-17)
- In Command the briefing leads to the map table, and the table starts the battle. A paper map of the field drawn
  from the layout once (`bavovniatko::table::paint_map`): the ground's relief and contours (every 5 m, every 25 m
  darker), the town's blocks, the roads (tracks dashed), the tree lines with their gaps, the river, the minefields we
  know of in red (Vuhledar's own belts and shoulders; their barrier belts at Mala Tokmachka cross-hatched), the grey
  zone hatched, the road heads; north up, so our side is at the top on both days. Our positions are marked on it,
  lettered where a unit holds one (S Stugna, J Javelin, R riflemen, T a tank in a hide).
- Beside it the Allotment: the units that go to a position (pick one, then a position; a click on a held one takes it
  off), the vehicles for the wounded and the Support as so many of each, the points spent and left, and a line saying
  what a position is, or why a change was refused. Every change is checked as the battle checks it (`allot::build`):
  the points, how many there were, men only where men can stand, a tank only in a hide or a tree line, nothing beyond
  our side of the grey zone. It starts from the day's own picks, or `BAV_ALLOT`'s. "To the field" (Enter) settles it.
- The vehicles of the day's columns (Mala Tokmachka) are listed but not chosen yet: the script sends them, and their
  points stay spent. Supply makes them a choice (phase 6).
- The Allotment is settled at the table (`allot::settle`), so in Command our side comes onto the field when the battle
  starts (`fire::spawn_allotted`): the posts, the tanks in their hides, the rounds and missions, the FPVs, the vehicles
  for the wounded. Nothing reads it before (`allot::settled`). The Record builds our side with theirs at start-up as
  it always did; the posts' and hides' spawning is shared (`spawn_post`, `spawn_hide`) and called in the same order.
- During the battle T opens the same map over the picture, the battle running on under it: our units marked where
  they are with their callsigns and strength, the panel listing what each is doing, and the orders given from it as
  from the drone's view (a click picks a unit, G / K / B, a click on a position). A click on the table calls no fire,
  asks for no evacuation and picks nothing in the picture (`OrdersView::takes_clicks`).
- Headless the table settles its first picks at once; `BAV_TABLE=open` keeps it open for a shot, `BAV_TABLE=battle`
  opens it once the battle is on. `cargo xtask smoke` runs the table too.
- Tested: Command at Vuhledar with other picks (Stugna and Javelins in tree lines 20, 25 and 30, eight 155 mm, three
  HIMARS): four posts and a hide on the field, 128 rounds, the posts' missiles fired. Shots: devlog/shots/*-table-*.jpg.
  The Record is unchanged (same-day: same on both days).

### Phase 6, Supply and reinforcement Columns (2026-10-17)
- `bav_sim::supply`: in Command, Supply (підвіз) comes up the road during the battle: some at the start and more each
  minute of the day's clock (`allotment.supply`: Vuhledar 12 and 2 a minute, Mala Tokmachka 16 and 2), about 60 over
  the day. A Column costs the catalog `supply` of each vehicle and of the team it brings.
- A Column is vehicles of the day's kinds, as many as there were less the Allotment's and what has been sent; it may
  bring a team (Stugna, Javelin or riflemen), which takes the position as a post of ours when the first of the Column
  is there, and is lost with it if none of it gets there. Vuhledar's catalog gains the 72nd's own carriers for it
  (BMP-2, MT-LB); they wear the same paint as theirs, since liveries go by model, not by side.
- On the battle's map table: so many of each vehicle (− +), a team or none, the road head (a click steps to the next),
  a position clicked on the map with none of ours picked (marked on the map), and Send. The panel gives the way's
  length and how much of it runs through the grey zone (`columns::way`, the same way `columns::send` takes), the
  cost, and why not where it cannot go (no way there; not enough Supply; none left). The orders' panel over the drone's
  picture shows the Supply too.
- At Vuhledar the tree lines in front of the town are no place to send vehicles: our own belts close the field behind
  them, and the only way is down the road through the kill zone into the grey zone, where the first test Column swerved
  round the wrecks onto our mined shoulders and was lost with its team. The town's edge, the hides and the tree lines
  at the belts' ends are reached by road on our side. The table now says this before the Supply is spent.
- The script's own columns at Mala Tokmachka still go as the script sends them; making the day's attack the player's
  is for phase 9's balance, with the Enemy staff.
- `BAV_SEND="200:0:bradley+bradley@54:javelin"` sends a Column on Supply at a game time (road head, vehicles, position,
  team); `BAV_COMPOSE="1:bmp2+bmp2@36:javelin"` makes one up on the battle's table without sending it.
- Tested headless: Vuhledar, a BMP-2 and an MT-LB with a Stugna team from road head 0 to the hide west of the
  Wanderer's, 2 km, there at 666 s, the team in the hide 10 s later; a second Column refused for want of Supply.
  Mala Tokmachka, two Bradleys with a Javelin team to the forward tree line, 13 Supply, there at 521 s, the team in
  position, our ATGM teams four. Shots: devlog/shots/*-supply-*.jpg.

### Phase 7, more Support (2026-10-17)
- Two new kinds of Support in the catalog, for Command only (neither is in the Record's picks, so the day as it was
  stays as it was): air defence on call (`AirDefence(missiles)`, Mala Tokmachka: 3 missiles, 10 points, two to be had)
  and more reconnaissance drones (`Mavic(n)`, both days: two more Mavic 3s, 3 points, two to be had).
- The air defence (`robotyne::air_defence`): a battery behind the start line. The Ka-52 is beyond its reach while it
  hangs low behind the ridge; each time one pops up to fire, the battery gets one missile at it, while it has any.
  Three in ten bring it down, four in ten make it turn for home with its Vikhrs unfired, the rest go wide; the net
  says which ("Мінус «алігатор»!"). Its own random stream; the fire panel shows the missiles left (AD).
- The Mavics add to the operator's drones when the battle starts (the drone display's DRONES).
- Points are what make them a choice: at Mala Tokmachka the day's own picks spend 304 of 310, so two batteries of air
  defence and the Mavics (23 points) cost two of the four 155 mm picks and two of the four Bradleys for the wounded.
- The day's radio still says the Ka-52s are too far for anything we have: true of the day, less so with a battery on
  call. A line for it belongs with the Enemy staff's answers (phase 8).
- Tested headless: Mala Tokmachka with two batteries (6 missiles) and the Mavics: two Ka-52s down and four misses
  between 439 and 562 s; DRONES 5. Shot: devlog/shots/*-ad-r.jpg.

### Phase 8, Spotted and the Enemy staff (2026-10-17)
- `bav_sim::staff`, in Command only. **Spotted** is the enemy's picture of our forces, kept apart from the truth: a
  post is spotted when it launches, a tank in its hide when it fires, a vehicle when one of their posts or vehicles
  has it in sight within 1.5 km (not through smoke or a tree line); and an Orlan crosses our side every few minutes
  (Vuhledar 3–6, Mala Tokmachka 2–4) along a 600 m strip, seeing a post under the trees one time in three and a
  vehicle four in five. Each sighting is where it was then, give or take 15–40 m, and fades: a post's after five
  minutes, a vehicle's after two.
- The staff's rules, each with its own budget and stream:
  - **guns**: their artillery fires on what they have spotted of our posts, not where they truly are
    (`director::russian_artillery` reads `Spotted`); with nothing spotted, it works over the area as before;
  - **lancet**: a Lancet comes down out of the sky at a vehicle of ours they have just seen, every four to eight
    minutes at most, while they have any (Vuhledar 2 for the day, Mala Tokmachka 4: c09 §5);
  - **columns** (Vuhledar, where the columns are theirs): the next column learns from what the earlier ones met
    (`Staff::adapt`, from the hit log): six or more lost on its road and fewer than half that on the other, the other
    road; three to mines, a plow in front; four to our missiles and shells, two more smoke vehicles and a BMP-2 in
    its first tank's place. The intercepts tell the player ("Перехоплення: наступна колона йде іншою дорогою").
- The battle's map table rings in red where they think something of ours is: their picture, so a ring sits a little
  off what it marks, and goes when they lose it.
- `BAV_STAFF=off` turns it off in Command; `BAV_STAFF=spot,guns` keeps only the rules named (`spot`, `guns`,
  `lancet`, `columns`), for phase 9's bands to measure each. The smoke runs take Command with it off too.
- Not yet: a flank (our unit where no column of theirs looks) drawing their infantry; the Ka-52s cued by what is
  spotted; their answer to our air defence. The day's radio line about the Ka-52s being out of reach stays the day's.
- Tested headless over a whole day at Vuhledar: the second column came down the other road after the first's losses
  on the dacha road, and the fourth with a plow, two smoke vehicles and a BMP-2 leading; Orlan passes spotted a post
  or so each. At Mala Tokmachka an Orlan spotted six of ours on the slope, and a Lancet went at a Bradley (and missed).
  One run each way says nothing yet about the bands; that is phase 9.

### Phase 9, balance (2026-10-17)
- Bands, four seeds each (devlog/band/p9.log; `BAND_TAG=rec`, `cmd-off` with `BAV_PLAY=command BAV_STAFF=off`,
  `cmd-on` with `BAV_PLAY=command`):

  | | Vuhledar: theirs knocked out | Vuhledar: our losses | Mala Tokmachka: ours stopped | Mala Tokmachka: dead |
  |---|---|---|---|---|
  | Record | 26–29 | 1–7 | 12–18 | 19–35 |
  | Command, the day's picks, staff off | 26–31 | 2–6 | 15–17 | 20–37 |
  | Command, the day's picks, staff on | 27–30 | 2–6 | 16–20 | 16–50 |

- Command with the day's own picks and no staff is the Record's band on both days: building our side when the battle
  starts, from the table, changes nothing that matters.
- The staff is not a thumb on the scale. At Mala Tokmachka it costs us about two more vehicles a morning (the
  Lancets, and their guns on what they have spotted). At Vuhledar it changes nothing measurable: its columns change
  road and put a plow or smoke in front, and still die in the kill zone, which is the record's lesson too; and with
  the day's picks we have no vehicles out for the Lancets, which do not go for a tank in its hide.
- Left as they are: the costs and the Supply. Bands cannot judge a player's choices; the numbers are the file's first
  guess, to be tuned on played sessions (`allotment.points`, each entry's `cost` and `supply`, `allotment.supply`,
  `staff`).
- Next, if the staff should bite harder at Vuhledar: Lancets at spotted tanks in hides (a hit on a `UaTank`), their
  infantry sent at a flank where no column of theirs looks, and the Ka-52s cued by what is spotted.
