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
  (polyline built from graph edges) by arc length" — `drive_vehicles` already steers for a look-ahead point and
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
3. **Movement on paths**: `Path` in `drive_vehicles` for both sides; columns with slowest pace; taking positions.
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
  spawn_vehicle, drive_vehicles, ColumnOrder, Soldier), `fire.rs` (Post, spawn_positions, FireControl, stocks),
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
