# Next items: what the playthroughs showed, and the plan

*Written 2026-10-11 from `game/next-items.md`, twenty-two observations from playthroughs and from the captures
in `game/devlog/`. Each observation was checked against the code and the captures, the realism questions were
researched, and the work is grouped into twelve packages below, ordered.*

> Better 3D assets are coming soon. Everything here that touches the look of trees, vehicles and soldiers is
> built behind a seam the assets can plug into, and nothing in this plan invests in procedural geometry that an
> asset would replace. The simulation, the effects, the feed, the interface and the words are asset-independent
> and are the bulk of the work.

---

## 1. The observations, and where each one goes

| # | observation (abridged) | what we found | package |
|---|---|---|---|
| 1 | Columns: equal spacing and speed; driving is hard, the terrain is terrible | Spacing is a constant 42 m, speed a constant per kind, no terrain or driver variation (`director.rs` Column, `units.rs` drive_vehicles) | A |
| 2 | Ground resolution too low, squares visible | 1 m texels, linear-filtered; painted earth patches have texel-square edges ([shot](../game/devlog/shots/20261011-2005-snow-squares.jpg)) | B |
| 3 | Smoke needs variation in colour, shape, dispersion, kind; one wind | 9 particle kinds × 8 alpha steps, one 64² puff texture, no rotation, one wind vector, wind only on buoyant and white kinds (`fx.rs`) | C |
| 4 | EW interference: find references, imitate | Row tearing, block freeze, chroma shift in `feed.wgsl`; nothing of a digital link's real failure modes | G |
| 5 | TOS explosion looks wrong | `fx::thermobaric` is 18 short fire quads plus black smoke plus a size-1.6 HE blast; from the drone it reads as pale artillery puffs ([shot](../game/devlog/shots/20261011-2005-tos-from-drone.jpg)) | C |
| 6 | Turret tosses good; add variation and other deaths (fire, broken down with a rotating turret, mobility kills; a track-less tank keeps rolling, can circle); toss blast should be big | One path: `on_wrecked` chars, burns, 40% toss; immobilise = coast and slew; no fate variety (`units.rs`) | D |
| 7 | Trees and vegetation not good from ground level; take Stugna shots | Winter: three rows of uniform branch cards, a palisade ([shot](../game/devlog/shots/20261011-2003-ground-trees-winter.jpg)); summer: hard-alpha leaf blobs ([shot](../game/devlog/shots/20261011-2004-ground-trees-summer.jpg)) | I |
| 8 | Infantry aim through obstacles | No line of sight anywhere in `fire.rs`: teams, rifles and tanks fire on distance and smoke alone | F |
| 9 | EW should apply to sound too | Sound is unaffected by jamming (`audio.rs`, `camera.rs` update_feed) | G |
| 10 | "THE DAY" heading should be a date; align as tabular data | One text blob with hand-spaced columns (`hud.rs` show_debrief) | K |
| 11 | Sources as tabular data on a dedicated tab | The record was removed from the debrief by design/next.md; see the decision in §5 | K |
| 12 | Keep the historical closing lines, drop the heading; redesign the debrief cleanly | As above | K |
| 13 | Tanks should not drive through each other | They can: the lane blocker test ignores a vehicle more than 6 m to the side of the lane you are steering into, so a vehicle fanning out crosses through others; a knocked-out hull coasts without any test (`drive_vehicles`, `coast`). Measured: see §3 A | A |
| 14 | Flames on vehicles need variation, before and after destruction | One emitter kind (`WreckFire`: black smoke and a fire puff above the hull) | C, D |
| 15 | Infantry leaving abandoned vehicles is unrealistic: should run fast, at different speeds, in different directions, to cover | All bail out at once at 1.5–2.3 m/s towards the same point (`crew_bailout`, `move_soldiers`) | E |
| 16 | Place names in Ukrainian with English in parentheses; names and accent throughout | All English; Bevy's default font has no Cyrillic at all (checked: 95 glyphs, none in U+0400–04FF) | J |
| 17 | Skybox variation: grey weather with clouds; find references | No sky: a clear colour plus distance fog (`world.rs`, `camera.rs`) | H |
| 18 | A realistic drone UI, as frequently seen | One telemetry line in the corner | G |
| 19 | Detection overlay: green boxes and arrows arriving asynchronously, a subgame of identifying types, a flag after a while | Nothing of the kind; next.md's "the picture" and the spot/confirm tasks are the design hook | G |
| 20 | Tanks are heavy when hit; humans are light, fly away | Hulls get a pitch and roll kick; a killed man is swapped for a fallen mesh on the spot, never thrown | D, E |
| 21 | Top-attack munition, as used historically | The Javelin exists as a fire-and-forget missile with a 4 m lift, not a top-attack profile; the dossier attests Javelin and Stugna-P at Vuhledar | L |
| 22 | (from 7) ground-level screenshots from Stugna launchers | Two taken today, linked above; more per package | — |

Two assessment shots and two frames were added to `devlog/shots.md` today for this plan.

---

## 2. What the research says

Four research reports were gathered for this plan (the full texts are in the session transcript; the numbers
that matter are here). Items marked *est.* are the researchers' estimates, not published figures.

### 2.1 Weather on the two days (Open-Meteo ERA5 at the diorama; Dnipro SYNOP, the nearest reporting station)

| | 6 February 2023, Vuhledar | 8 June 2023, Mala Tokmachka |
|---|---|---|
| sky | unbroken low stratus deck, 95–100% low cloud all day, base *est.* 200–500 m; a brief break around 19:00 | cirrostratus veil, high cloud 100% until 13:00, thinning to broken altocumulus by 15:00; clear by evening; no low cloud |
| sun | behind the deck; noon elevation 25° | hazy orange disc through cirrus at sunrise 04:39; noon 65° |
| visibility | 3–7 km in the morning (mist), 10 km midday | 20–30 km |
| temperature | −3.7 °C at dawn, −1.0 °C max | 12 °C at 04:00, 25 °C max |
| wind | NE, 4 m/s at dawn, 5.5 m/s midday (gusts 10 m/s) | E 1.5–2.5 m/s at night, SE 3–3.5 m/s by day |
| precipitation | light snow 03–05; snow depth 3–5 cm, patchy | dry |
| sunrise/sunset | 07:54 / 17:36 | 04:39 / 20:31 |

So the Vuhledar day we render (hard low sun, long shadows, after the footage comparison) is the look of the
second week of February, when the viral clips were shot, not of 6 February itself, which was overcast. Both
looks are historical; the campaign's day table in next.md can carry both. The current wind vector in `fx.rs`
is ENE at 3.7 m/s; ERA5 says NE 4–5.5 m/s for Vuhledar, SE 3 m/s for Robotyne (ours is 1.8 m/s).

### 2.2 Smoke, by kind (what each is, why it looks as it does)

| kind | look | rise | duration | now in the game |
|---|---|---|---|---|
| TDA exhaust screen (diesel into the exhaust) | milky grey-white fog, a ribbon 100–150 m long per vehicle, 2–6 m high | none: isothermal, moves with the wind as a sheet | ≤5–10 min of injection, ≥1 min to disperse | `SmokeScreen` emitter: rises, 35–60 m puffs |
| 902 "Tucha" grenades (HC, not phosphorus) | fan of 6–12 thin grey trails arcing 200–300 m, 7–12 s delay, then ground-level puffs 30–45 m long, 8 m high, merging into a wall in 1.5–2 min; grey, fattens in humid air | none | 2.5 min | `pop_smoke`: 24 white puffs at 14–22 m/s |
| burning vehicle | hit: orange flash, grey-brown puff 1–2 s → propellant: brilliant white-grey jets from hatches, seconds to a minute → fuel and rubber: thick black column, buoyant, rising 20–50 m before bending → smouldering: thin grey-white wisps for hours | buoyant, Briggs 2/3-power rise then bends | black phase 30 min to hours; hull fires 12+ h in the one documented case | black smoke plus fire puff for 150–320 s, then grey smoulder |
| 155 mm HE in frozen ground with thin snow | dark column 20–35 m in 1 s with a bright white snow skirt, bulk falls in 3–5 s, little fine dust so the haze clears in 20–40 s | fast then settles | 20–40 s | five-layer blast: dark spikes, column, clods, skirt, hanging cloud 22–40 s |
| 155 mm HE in dry June chernozem | black-brown column, thicker and longer haze, lightening as it dries | | 1–3 min | same as winter |
| 120 mm mortar | 60% column height, half the cloud | | | size 0.8 |
| WP and smoke rounds | brilliant white tendrils falling 10–20 m/s, then a low pure-white bank 5–15 min; HC canisters grey, 40–90 s each | none | minutes | `Smoke` shell: white puffs plus a 75 s emitter |
| ATGM launch | back-blast dust fan 10–20 m behind the tube for 2–5 s; a thin grey-white motor trail visible only for the 1.5–3 s burn (Stugna-P and Kornet are laser beam-riders, not wire-guided) | | | 8 white puffs; the missile trails white smoke all the way |
| Javelin | a pop and a puff, then the flight motor lights and a steep climb | | | |
| 125 mm muzzle | 0.1 s flash, grey ball 10–15 m that condenses whiter in cold humid air, 5–10 s; ground ring 15–20 m, 2–3 s | | | similar, smaller |
| tracers | Russian green (barium), Ukrainian red (strontium); the trace starts some way out from the muzzle | | | all orange |

Wind: every plume bends the same way; only hot plumes rise first. Rank of rise: fire > WP > HE dust > HC > TDA.

### 2.3 Fire on vehicles: the catalogue

Eleven looks from the footage, with the reason for each, and what the record says about the two days:

- **The two fields differ.** Vuhledar's wrecks are mostly mine and artillery kills: immobilised, abandoned, often
  not burning at first, then re-struck until they burn (Forbes/Axe, 8 and 12 Feb 2023; 31 vehicles including 15
  tanks on the big day per Rob Lee). Mala Tokmachka's Bradleys are mostly intact, stopped, hatches and ramps open,
  a minority burning; "some were later recovered and repaired" (Oryx: eleven Bradleys, ten abandoned).
- Engine-deck fire after a hit on the engine: 0.5–1.5 m flame on the rear deck, white-grey oil and coolant steam
  first, black after 1–3 min; the vehicle may crawl on for a kilometre or two trailing smoke (Kyiv Post T-90A).
- Flames from the driver's hatch and the turret ring: white smoke 10–60 s, then 1–3 m jets through the hatches.
- Reactive armour does not burn: a chain of bright white-orange flashes 1–2 m, grey puffs, flying plates.
- The whole vehicle a torch: a hull-sized fire is *est.* 30–40 MW, flames twice the hull's height, a dense black
  column bending with the wind; intense for 20–60 min, then hours.
- Smouldering next day: thin blue-grey wisps from hatches and louvres, no flame by day, 12–72 h.
- Secondary cook-off, "the candle": each propellant charge a roaring 5–15 m white-yellow jet lasting 1–3 s, in
  irregular clusters of 3–15 over 10–90 s, repeating over minutes (a T-72 holds about 40 charges).
- Turret toss: fireball *est.* 10–20 m across for 0.5–1 s with a grey-black mushroom to 100–200 m in 10–20 s;
  the turret's apex typically 5–30 m; debris and burning charges rain for 3–5 s. Not every ammunition blast
  throws the turret. The toss can be immediate or minutes after the hit, once the fire reaches the carousel.
- Burning propellant sticks: 50–300 yellow-white sparks to 20–50 m, 2–4 s, then burning on the ground.
- Fuel pool fire beside the hull: 2–6 m², 1.5–3 m flames, 10–30 min; in snow a black melt ring with steam; in
  June it seeds a grass fire that walks downwind at 0.1–1 m/s with a low 0.3–1 m flame front.
- Bradley and Leopard: no turret toss; the Leopard's bustle blow-out panel vents once upward; a Bradley's 25 mm
  and TOW rounds crackle like popcorn; an aluminium hull slumps rather than burns. A Bradley field is mostly
  stopped vehicles with open ramps.

### 2.4 Ukrainian names and the voice of the net

- **Romanisation** is the 2010 national system (г→h, и→y, х→kh, ь dropped): Вугледар (Vuhledar), Павлівка
  (Pavlivka), Микільське (Mykilske), Богоявленка (Bohoiavlenka), Водяне (Vodiane), Мар'їнка (Marinka), Мала
  Токмачка (Mala Tokmachka), Роботине (Robotyne), Оріхів (Orikhiv), Вербове (Verbove), Запоріжжя (Zaporizhzhia).
  Never the Russian-derived forms (Ugledar, Rabotino).
- **Formations:** 72-га ОМБр імені Чорних Запорожців (72nd Mechanised Brigade "Black Zaporozhians"); 47-ма ОМБр
  «Маґура» (47th, with the letter ґ); 155-та ОБрМП (155th Naval Infantry); 36-та ОМСБр; 291-й МСП; батальйон
  «Алга». On the net: "сімдесят друга", "морпіхи", "сто п'ятдесят п'ята".
- **The skeleton of the net** (verified idiom): плюс (acknowledged), прийняв (copy), на прийомі (go ahead),
  відбій (cancel), вихід / прихід (shot out / incoming), мінус (destroyed; negative), двохсотий / трьохсотий
  (killed / wounded), пташка (drone), коробочка (armoured vehicle), посадка (tree line), Кропива (the fire-control
  tablet), 4.5.0 (all quiet), накрила (rounds on target), недоліт / переліт, вилка (bracket), РЕБ давить (EW is
  pressing), орки / противник. Callsigns are single nouns (Сокіл, Беркут, Граніт, Маестро); "Сокіл — Маестро"
  on the chatter net, "я Маестро" only on the artillery net. "500" belongs to Russian mouths only.
- **Recommendation:** two registers. In-world text (map labels, radio captions) Cyrillic first, Latin after:
  «Вугледар (Vuhledar)». Player bookkeeping (HUD counters, ledger) Latin first with the Cyrillic beside it or on
  hover. Subtitles: the Ukrainian line full weight, the English gloss dimmer beneath; keep "plus", "minus",
  "300" untranslated in the gloss so the player learns them. Mix in clipped forms (акум, БК, евак, бехи) so it
  sounds like 2023. The researcher wrote 42 sample lines covering the whole day; they go into the string table
  of package J.
- Weapon names as spoken: Стугна, Джавелін, Корнет, Леопард, Бредлік, Хаймарс, Сонцепьок (TOS-1A), три сокири
  (M777), Цезар, Краснопіль, Лелека, Фурія, Мавік; беха (BMP), мотолига (MT-LB).

### 2.5 Vehicle deaths, mobility kills, infantry under fire, top attack

- **Mines are the commonest loss**, and they wreck the running gear, not the tank: a TM-62 (7.5 kg TNT, fires at
  150–550 kg on the track) cuts the track and usually takes one or two road wheels and their arm; the cut track
  feeds off the sprocket and lies behind as a straight strip; the vehicle yaws 10–30° towards the dead side and
  stops within 5–10 m (*est.*). Field repair under fire is "generally impossible" (StrategyPage), so the vehicle
  stays where it stopped: *immobilised, abandoned, probably recoverable* until something finishes it. Follow-on
  strikes (artillery, drone drops, at Robotyne the Ka-52s) are how most become "destroyed".
- **The driverless circle is documented**, once: a T-80BV hit in October 2022 lost a track with the engine still
  running and the crew dead or stunned, and drove circles in a field leaving a crop-circle pattern until it
  stalled; one crewman then climbed out dazed. CNN on Vuhledar: "some tanks veer wildly before exploding". So a
  slow circle towards the dead side, one or two turns, is rare but real.
- **A spinning turret** is a *live* tank's fault (2024 T-90M videos with damaged sensors, one spun until it hit a
  tree), never a dead hulk's: once it burns, the hydraulics and electrics are gone. The game should not spin the
  turret of a wreck.
- **Oryx shares** (October 2026 totals): Russian tanks 75% destroyed, 4% damaged, 9% abandoned, 12% captured;
  mid-2023 the destroyed share was lower (62%) and captured higher (27%). Ukrainian Bradleys: 95 destroyed, 17
  damaged, 33 damaged-and-abandoned, 13 abandoned: about 40% of Bradley losses are not burnt out. Turret toss is
  not tallied anywhere; *est.* a third to a half of destroyed T-64/72/80s.
- **Two cook-off timings.** Immediate (about 1 s: a T-90M hit by an ATGM detonated between frames) when the hit
  reaches the carousel; delayed by 20 s to several minutes when a fire starts elsewhere (engine, an open hatch,
  a drone drop), always preceded by dense white smoke then flames (Zaloga: the charges go "like a string of
  firecrackers"). A hatch jet ("blowtorch") of 3–10 m for seconds to a minute is the propellant burning without a
  toss. BMPs burn shorter and more violently and often "disintegrate"; ERA bricks do not go off in a fire and stay
  on the burnt hull; small-arms ammunition pops singly and harmlessly.
- **Turret toss physics.** Typical: "several feet, landing beside the chassis" (Zaloga); documented extremes
  15 m up (a turret on a fifth floor in Mariupol). Launch speed for 3 m is 7.7 m/s, for 10 m 14 m/s, for 30 m
  24 m/s; airborne 1.6–5 s; it lands 0–15 m from the hull, often inverted, sometimes with a second bang on
  landing. The deflagration of *est.* 100 kg of propellant gives an orange-white fireball 15–25 m across for
  0.5–1 s, a flame column 20–40 m, and a blast like 20–40 kg TNT: 5 psi to 15–20 m, fatal within 5 m. The note
  that "the explosion below is quite large" is right: the fireball is two to four hull lengths wide.
- **Crews and dismounts.** A fit crew with open hatches is out in 5–15 s; a concussed one (mine) 20–60 s; *est.*
  10–30% never get out of a penetrated T-72/T-80. The driver is the one with broken legs. First move: put the
  hull between you and the threat, then 5–20 m rushes to the nearest ditch, crater, wreck or tree line; wounded
  dragged at 0.5 m/s. Bradley dismounts: ramp down, 3–5 s to clear, bunch behind the hull, smoke, then run to
  another vehicle 20–60 m away, not towards the enemy (TWZ: "they were following doctrine"). Riders on a hull
  that hits a mine are jolted and slide off within 1–2 m; no clip shows riders lofted.
- **Blast on a body** (Glasstone, Lovelace, JAMA): a 155 mm burst gives 50 psi at 3 m, 5 psi at 9–10 m, 1 psi at
  25–30 m. Within 4 m: killed, body displaced 0–10 m (the JAMA "bizarre" cases, 13 m). At 5–10 m: knocked flat,
  slid 1–3 m, stunned 5–20 s. Beyond 10 m: staggers or drops. A body at the lethal 8 m/s travels about 4 m; a
  20 m "ragdoll" needs 15 m/s and happens only inside a cook-off's fireball zone. Fragments, not blast, kill out
  to 50 m.
- **Top attack.** The 72nd's Javelin ambush at Vuhledar is attested (three BMP-3s and a T-80 in one clip, fire
  adjusted by a Matrice): the look is a hot dot descending at 50–70° onto the turret roof, a flash the size of the
  turret, a grey-brown cloud 10–20 m high within a second, then either the immediate toss, a fire building over
  10–60 s, or the vehicle lurching on 10–50 m. Javelin profile: top-attack apogee 150 m, 14.5 s of flight at
  2 km, 2–3 s lock, soft launch. NLAW is not attested at Vuhledar; SMArt 155 and BONUS are attested elsewhere in
  2023 but not at Vuhledar in January–March, and the 47th's BONUS kill near Robotyne is from 5 July, not 8 June.
  So: the Javelin profile is the top-attack work, and nothing else is historical for these two days.

### 2.6 The drone feed under EW, the OSD, detection overlays, the sights

- **The EW of early 2023** (RUSI "Meatgrinder", interviews February–May 2023): about one major Russian EW
  system per 10 km of front, 7 km behind the line, "weapons free" against UAVs while on station; GNSS
  interference everywhere; about 10,000 Ukrainian drones lost a month; platoon-level directional jammers and
  hijacking arrays near positions. A 56th Brigade EW commander: near Donetsk in autumn 2022 a Matrice "flew
  without any problems"; at Huliaipole in January 2023 "within a couple of weeks we lost or damaged three
  Mavics". RUSI on the June breach: drone use was "fairly binary": if Russian EW was active the drones could not
  get in; and GPS jamming was lifted when the Ka-52s were working.
- **What a digital link (DJI OcuSync, Mavic 3 class) does when jammed**: it holds a clean picture as long as it
  can, then lowers bitrate (soft, smeary), then macroblocks and colour bleeding in moving regions and the frame
  rate halves, then frames repeat for 0.3–2 s and catch up with a jump, then it freezes on the last decoded frame
  with the red disconnected icon and the telemetry stops. **No snow on a digital feed**: snow, rolling bands and
  ghosting belong to analog FPV links. Latency grows to 100 ms and more under interference. The signal indicator
  goes three white dots → two yellow → one red → a red broken link; a flashing banner in the status bar reads
  "Strong Interference to Aircraft. Fly with caution", then "Weak signal", then "Signal Lost"; the controller
  beeps. Control and video are separate links: "picture frozen but the drone obeys" and "picture fine but the
  drone will not answer" are both real.
- **Failsafe**: after 6 s without the controller the drone brakes, hovers, flies 50 m back along its track, then
  straight home at the return altitude, and stays in return-to-home even when the link comes back until the
  operator cancels. Under GNSS jamming it drops into attitude mode and drifts with the wind, the satellite count
  collapses and the icon goes yellow or red; under spoofing the map marker walks tens of metres while the picture
  shows the drone still. The drone crossing into a jam zone degrades gradually, then falls off a cliff.
- **Sound**: the aircraft carries no microphone; the operator hears only the controller's beeps and vibration and
  the people around them. So "EW applies to sound" means: the controller's warning tones and, in the game's
  convention of listening at the camera, dropouts in step with the frozen frames.
- **The Pilot 2 screen on a Mavic 3T (2023)**, element by element: a top bar (back, status text with flashing
  warnings, flight mode N/S/A/T, battery bar with minutes left, satellite count, the signal dots, battery %,
  settings); a left tool column (Look At, PinPoint, POI, gimbal, Wide/Zoom/IR); a right camera column (mode,
  the record button with a red dot and elapsed time while recording, playback, the zoom factor 7x/28x/56x); a
  camera strip (ISO, shutter, EV, focus, storage); a bottom-left map thumbnail; a bottom-centre compass rose with
  the aircraft icon, the H home mark, the blue controller dot, and around it VS, ALT, ASL, D, horizontal speed,
  wind, gimbal tilt. White sans text with a dark shadow, most readouts every frame, satellites and battery at
  1 Hz. The thermal view adds the palette strip with scene max and min temperatures, white-hot and black-hot, and
  the crosshair becomes an X. The Mavic 3T has **no laser rangefinder**; the Matrice 30T has (the RNG block with a
  red crosshair, slant range, height difference, and a PinPoint that yields the target's coordinates). An
  on-board SD recording has no OSD at all; units publish those with an emblem added and the telemetry strip
  blurred.
- **Detection overlays**: Saker Scout (approved September 2023) and ZIR (November 2024) draw YOLO-style boxes:
  2 px rectangles, class colour or green for detected and red for selected, a label tab "tank 0.87", re-fitted
  every frame so they jitter by a few pixels and flicker off for 1–3 frames, a persistent ID on a tracked target,
  a thicker box and a centre marker on the chosen one. Delta's Vezha classifies "70% of enemy vehicles in 2.2
  seconds" across the streams (2024). None of this was in a Mavic crew's hands in February or June 2023: an
  overlay in the game is an anachronism unless it is the *picture* being drawn by the operator (PinPoints) or
  by the command post. Delta marks are APP-6: red diamond frames for hostile land units with the equipment icon
  inside, blue rectangles for friendly, yellow for unknown; every mark carries a snapshot, a class, a source and
  a time, and is never deleted. Kropyva: tap a mark on the map, choose the type, the coordinate string appears,
  transmit; corrections as ± metres; the gun solution "within seconds".
- **The Stugna-P is not an eyepiece.** The gunner sits up to 50 m from the launcher at a PDU-215 panel (a
  briefcase laptop with a joystick) and sees a 4:3 low-resolution TV or thermal picture at 12x (narrow,
  1°15′ × 0°50′) or 4x (survey), with a fine crosshair and an auto-tracking gate rectangle that holds the vehicle;
  the missile rides a laser beam, so the picture does not shake at launch, only blooms. Our ground view puts the
  eye behind a tube with a tripod and a reticle: that is a Javelin's posture, not a Stugna's.
- **The Javelin CLU** (TC 3-22.37): day view in colour at 4x; wide and narrow thermal views in black and green
  (hot = light green) at 4x and 9x with different reticle patterns; the seeker view with four corner track gates
  and a crosshair that flash until lock and go solid; status words around the frame (DAY, WFOV, NFOV, SEEK, TOP,
  DIR; MISSILE NOT READY amber; BCU red flashing with 30 s left); a 10–15 s seeker cool-down before a lock is
  possible; the gates are set round the hull only.
- **The Kornet** gunner holds a crosshair on the target for the whole 15–18 s flight, by habit 2–3 m above the
  tank until the end; a white-hot thermal or a day periscope with a central mark.

### 2.7 TOS-1A

- **Figures.** 24 rockets of 220 mm, 204–216 kg each with a 90 kg thermobaric fill, range 600–6,000 m; a full
  salvo leaves in pairs over 6–12 s; footprint about 200 × 400 m with the long axis along the line of fire
  (dispersion *est.* 1–2% of range); doctrine in Ukraine was 8–16-rocket salvos, launchers often single. ISW
  has TOS-1A fire on Vuhledar on 27 January; the dossier has 1 and 11 February too.
- **One impact, frame by frame.** A one- or two-frame white-orange flash; an orange fireball that rolls up and
  darkens inside 1–2 s, *est.* 20–26 m across (Roberts' correlation for 40–90 kg of fuel: two-thirds to the full
  height of a nine-storey block); in the same 0.1–0.3 s a pale ring of lifted snow or dust racing out 20–50 m
  along the ground, with a brief condensation shell in humid cold air; then a dark grey-black sooty head over a
  grey-brown dust stem; in town, pale concrete dust from the facades; haze hanging 30–90 s, minutes over a block;
  small fires after. Little crater.
- **The salvo.** Impacts ripple over the same 6–12 s as the launch, scattered over the footprint: a rolling line
  of bursts, not one point. Rockets arrive unlit (the motor burns out near the launcher), so no trails at the
  target.
- **Sound** at 1–3 km (delay 3–9 s at 330 m/s in −3 °C air): a string of deep thuds overlapping into a continuous
  roll for the length of the ripple, then a reverberating tail of 2–5 s; at the launcher a ripping crackle.
- **A finding about the clock.** Open-Meteo's "Europe/Kyiv" applies UTC+3 to February dates, but Kyiv was on
  UTC+2; the correct sunrise and sunset at Vuhledar on 6 February are **06:51 and 16:38 Kyiv time**. The dossier's
  "07:50–17:40" is Moscow time (the clock of occupied Donetsk), and the game's clock and sun follow the dossier:
  `world.rs` puts sunrise at 07:50 and the day starts at 07:40. Either the diorama runs on Moscow time or its sun
  is an hour late. June is unaffected. See decision 6 in §5.

---

## 3. The packages

Each package says what we saw, what the code does, what to build, how to prove it, and what it costs (rough
hours of focused work including the measuring). Packages marked **sim** change the day and therefore the
`same-day.sh` baseline on purpose; they get a new baseline and the check becomes the tally staying in its
historical band. Packages marked **look** must leave the day byte-identical, and `same-day.sh` is their test.

### A. Columns and driving (**sim**) — observations 1 and 13

**Now.** A column spawns with its vehicles exactly 42 m apart and each kind at one speed; following is a speed
match to the vehicle ahead with no reaction lag; the only terrain effect is the suspension's pitch noise.
Overlap: the blocker test in `drive_vehicles` only sees vehicles within 6 m of the lane you are steering
*into*, so a vehicle fanning out to a lane 24 m away crosses whatever sits between; `coast` moves a knocked-out
hull with no test at all; routes 0 and 1 never cross, so that case does not arise. Measured over a whole day with
a counter now in `dev.rs` (`VUHLEDAR_LOG_STATS` prints it each game minute; the day ran on the fixed 20 Hz step):

| | Vuhledar | Robotyne |
|---|---|---|
| frames with two attacking hulls closer than 4 m (through each other) | 74 of 29,337 (3.7 s, at the fan-out in minutes 2, 3 and 10) | 26 of 26,888 (1.3 s, minute 6) |
| most such pairs at once | 1 | 1 |
| frames with two hulls closer than 6.5 m (touching or nearly) | 26,902 (92% of the day, from minute 3 on) | 15,574 (58%) |

So they do pass through each other, briefly, while fanning out; and once a vehicle has pulled out beside a
wreck or coasted into the one ahead, two hulls sit inside each other's footprint for the rest of the day, which
the drone sees at x13.

**Build.**
1. *Stagger.* Spawn spacing 30–60 m (seeded per vehicle), speed per vehicle ±10% of its kind, a reaction lag of
   1–3 s in following (the accordion), a brief random halt now and then (1 in 10 vehicles stops 5–20 s on the
   approach, as drivers lose the one ahead in the exhaust smoke).
2. *Terrain.* A speed factor from where the vehicle is: road 1.0, plowed lane 0.85, field with 3–5 cm snow 0.6,
   mined shoulder field after leaving the road 0.5 while it is turning, crater within 4 m 0.3 for a moment; in
   June the dry field is 0.8. The factor also drives the suspension noise and the track spray rate, so hard
   going is visible.
3. *Separation.* A hull-circle test against every other attacking hull (not only the lane): if another hull's
   centre is within 7 m ahead in the heading, stop; if within 4.5 m to the side while moving laterally, halt the
   lateral move. Knocked-out hulls in `coast` stop against any hull within 5 m. Cost: 40 hulls, 800 pairs a frame,
   trivial.
4. The overlap counter stays in `log_stats` as a regression number: through-frames 0, touching-frames only
   where wrecks genuinely nose into each other (allow a hull to stop 5–6 m behind a wreck, never inside it).

**Prove.** A kill-zone clip like `20261011-0124-killzone-dynamics.mp4` at the same camera, and the overlap count
in the tally. The day's tally must stay in band (24–30 of 33 knocked out at Vuhledar; most crews out at Robotyne).

**Cost.** 6–8 h.

### B. The ground's resolution (**look**) — observation 2

**Now.** 1 m texels, linear-filtered, 256² per chunk. The patchy-snow mask was *painted* into the texels, so
every earth patch has a staircase edge at 1 m, and the drone at x3 from 60 m shows them as squares. The shader
already adds sub-texel furrows and grain, but only as a tint on the texel colour. Going to 0.5 m texels would
cost 4× the texture memory (63 MB → 252 MB) and 4× the paint time; not the way.

**Build.** Move the snow-earth *pattern* out of the texels and into the shader, where it is resolution-free:
1. The texel carries the *mean* cover (how much snow here, 0..1) and the furrow code; the shader turns the mean
   into a patch pattern with world-space fbm at 0.3–2 m scales, thresholded by the mean, with the earth tone
   from the texel. Edges become ragged at any zoom.
2. Craters and scorch keep their painted shape (they are 2–10 m) but get the same noise-broken edge: sample the
   texture at a jittered world position (±0.4 m fbm) so the boundary is never a texel edge.
3. A detail layer at 0.1–0.3 m: clods, snow grain, stubble, modulated by the same thin-snow value, fading out by
   `fwidth` as now.
4. Verify there is no mesh facetting at low sun on the 4 m vertex grid; if there is, the Robotyne trench banks
   need a finer grid locally, but today's shots do not show it.

**Prove.** The same low pass (`snow-squares` camera) before and after; the kill-zone close-up at x13; a Stugna
sight shot. `same-day.sh` identical (the texels still carry the same paint events; only the look changes).

**Cost.** 6 h.

### C. Smoke, fire and the TOS (**look**) — observations 3, 5, 14, and the toss blast of 6

**Now.** One puff texture, nine kinds, eight alpha steps, shared materials so everything batches (keep that).
Particles have no rotation, no per-particle tint, one wind, and buoyancy as a constant. The thermobaric is an
HE blast with extra fire quads.

**Build.** Keep the CPU billboard system and its batching, and add variety where it is free:
1. *Shapes.* A 2×2 atlas of four puff shapes (fbm variants, one wispy), picked per particle, with random
   rotation and mirror: still one material per kind and step (the atlas index is a UV offset on four quad
   meshes). Smoke stops reading as one stamp.
2. *Kinds.* Split the greys by what is burning: `Fog` (TDA: no rise, pale, long-lived), `Hc` (grenade smoke,
   grey, ground-hugging), `Propellant` (white, hot, fast), `Soot` (black, buoyant), `Dust` (earth, settles),
   `Snow`, `Steam`, `Wp` (pure white). About 14 kinds × 8 steps = 112 materials, fine.
3. *Physics.* Buoyancy that decays with age (Briggs-like: rise then bend), per-kind wind coupling, plus a
   turbulence term from `fbm(position, time)` so plumes wobble and two plumes never look alike. The wind stays
   one vector per day but gets a gust envelope in time (the drone already has one; share it).
4. *Emitters.* TDA as a low ribbon at the exhaust (no rise, 100–150 m, 1–2 min); grenade dischargers as a fan of
   8–12 arcs of 2–3 s flight, bursting into ground puffs that merge into a wall over 90 s (the `Smoke` resource
   stays the gameplay truth); artillery smoke rounds as falling white tendrils then a bank; HE as now but with
   the winter and June variants from §2.2 (snow skirt vs long brown haze), and the 120 mm at 60%; missile trails
   that last only the motor burn plus a back-blast fan; tracers green for the Russian side, red for ours.
5. *Fire on vehicles*, as a set of emitter programmes chosen by package D: engine-deck fire (white then black),
   hatch jets, the torch (flames twice the hull, a tall soot column), propellant candle bursts (white jets,
   sparks), the smoulder (thin wisps), the pool fire beside the hull with its melt ring in snow, and in June a
   grass fire that walks downwind as a paint event plus a low flame front.
6. *Thermobaric* (§2.7). Per impact: a two-frame flash, an orange fireball sphere scaling to 20–25 m that
   darkens inside 1.5 s, a ground ring of lifted snow and dust to 20–50 m within 0.2 s, a grey-black head on a
   grey-brown stem with high initial buoyancy, concrete dust in the town, haze for 30–90 s, little crater. The
   salvo: 24 impacts in pairs 0.25–0.5 s apart scattered over 200 × 400 m along the line of fire (today's sigma
   75 m round is close; make it an ellipse), no rocket trails at the target, the thuds overlapping into one roll
   with a 2–5 s tail (a longer `Thermobaric` sample, and no per-impact crack).
7. *Turret toss.* The fireball 15–25 m across for 0.5–1 s with a flame column 20–40 m, a mushroom to 100 m,
   sparks of burning charges raining 3–5 s, debris (hatches, road wheels) out to 50 m, then the hull torch; the
   turret typically 2–8 m up and down beside the hull, inverted, 1 in 10 to 15–30 m, sometimes a second bang when
   it lands; most tosses come 20 s–5 min after the hit, once the fire reaches the carousel (package D decides).

**Prove.** A smoke contact sheet (one frame each of the kinds above at the same camera and time, headless),
the TOS clip from the drone (`tos-from-drone` camera), the kill-zone clip. `same-day.sh` identical: emitters
and particles are in `Step::Effects` after the sim and must not touch the sim's random streams (they use `FxRng`,
which is separate; keep it so).

**Cost.** 14–18 h (the biggest look package).

### D. Vehicle fates and mobility kills (**sim**) — observations 6, 14, 20

**Now.** `Wrecked` has one script. Immobilised vehicles coast and slew then sit; destroyed ones burn 150–320 s
then smoulder; 40% of carousel tanks toss the turret at once.

**Build.** A `Fate` chosen at the hit from the cause, the kind and the side, driving both the sim state and the
emitter programme (package C):
1. *Mobility kill, track off.* The commonest end. The hull lurches, yaws 10–30° towards the dead side and stops
   within 5–10 m; the cut track lies behind it as a straight strip and one or two road wheels are left on the
   ground (debris meshes); the crew sits stunned 5–20 s (mine: 20–60 s), then bails. Rarely (1 in 15, *est.*),
   with the engine alive and the driver gone, the hull drives a slow circle towards the dead side for one or two
   turns before it stalls: the documented ghost tank.
2. *Immobilised, intact.* No fire. Crew out. Later re-struck by artillery (the brigade guns already do this) and
   only then burns: at Vuhledar this is the commonest end.
3. *Engine fire.* Crawls on 100–500 m trailing white then black smoke, stops, crew out, burns down or not.
4. *Torch.* Burns at once; cook-off bursts over the next minutes; a carousel tank throws its turret on one of
   them (so most tosses are delayed, not instant); Western vehicles never toss, the Leopard vents once.
5. *Brew-up with toss* at the hit (the current look), rarer than now.
6. *Abandoned under fire* (Robotyne's Bradleys): stopped, ramp down, no fire; a minority burn.
7. *The hull as a heavy thing.* On any hit: a settle (the pitch and roll kick we have, scaled by the weapon), a
   displacement of at most a few centimetres, never a jump; the hull sinks 10 cm into the crater when a mine goes
   off under a track.
8. A rotating turret only on a *live* tank with damaged sensors (a rare fate for a hit that disables: the turret
   slews slowly and aimlessly until the crew bails), never on a burning or dead hull: that is what the footage
   shows.
8b. *Pivoting on the dead track.* A mobility-killed tank whose crew is alive and fighting turns on the spot round
   the dead side, slowly (10–15°/s), to put its glacis towards the nearest known Ukrainian position, and keeps
   firing from there; if its turret is jammed it lays the gun by turning the hull. The crew bails only when the
   next hit comes or after a few minutes. It makes the immobilised tank a thing that still fights, which the
   kill zone needs.
9. Fate weights to start from (then tuned to the band): Vuhledar's Russian column, of the vehicles hit, about
   35% immobilised intact and later re-struck, 20% engine fire, 25% torch with delayed cook-off (half of those
   throw the turret after 20 s–5 min), 10% instant brew-up with toss, 10% withdraw damaged. Robotyne's Bradleys
   and Leopards: 60% immobilised intact with the ramp down, 25% engine fire, 15% burn out, no toss.

**Prove.** A fates contact sheet (headless, one wreck per fate), the day's tally in band, and the ledger's
"destroyed / abandoned" split moving towards the record's (UK MoD: "at least 30 mostly intact armoured vehicles
abandoned in a single incident").

**Cost.** 10–12 h, entangled with C.

### E. Infantry: bailing out, running, being thrown (**sim**) — observations 15 and 20

**Now.** Everyone aboard gets out in the same frame at the same spot, walks at 1.5–2.3 m/s (×1.2 fleeing) to
the same goal; a killed man becomes a fallen mesh on the spot.

**Build.**
1. *Getting out.* A fit crew is out in 5–15 s, a concussed one (mine) in 20–60 s, exits staggered 0.5–3 s
   apart from the hatches' sides; the driver often alone when the turret crew are dead, often with broken legs
   (crawls); 10–30% of a penetrated T-72/T-80's crew never get out. A Bradley drops its ramp, the squad clears in
   3–5 s, bunches behind the hull, pops smoke and runs for another vehicle 20–60 m away.
2. *Running.* A sprint of 4–5 m/s for 10–25 s, then a jog, per man ±20%; direction: away from the vehicle, then
   towards the nearest cover (a crater, a wreck, the nearest tree line), with 20–40° of scatter; a man drops
   flat when a blast lands within 30 m and gets up after 2–6 s; wounded crawl as now. Leg animation rate follows
   speed.
3. *Thrown.* A man killed by a blast within 4 m becomes a `Debris` body: displaced 0–10 m (most 1–4 m), tumbling,
   then the fallen mesh where it lands; a man 4–10 m out is knocked flat, slides 1–3 m and lies stunned 5–20 s
   before getting up wounded or not; beyond 10 m he drops or keeps running. A cook-off's fireball zone (5 m) throws
   bodies 10–20 m and knocks men down to 30 m. Riders on a hull that hits a mine slide off within 1–2 m; riders on
   a hull that cooks off are thrown. Those are §2.5's numbers: a body is light, but it is thrown metres, not tens of
   metres.
4. Desant riders already drop off when the vehicle is hit; they now run like the crew.

**Prove.** The infantry clip (`20261010-2030-infantry-wave-3` camera) and a bail-out close-up clip at x13; the
"came back" tally in band.

**Cost.** 6–8 h. Asset note: soldiers will be replaced by assets; the states (stand, run, prone, crawl, fallen,
thrown) are the seam, so define them as an enum the animation reads, and let the procedural figure's leg swing
read it today.

### F. Line of sight for the teams (**sim**) — observation 8

**Now.** ATGM teams, rifles and dug-in tanks fire on distance and smoke alone; a Stugna at x = 560 was seen
firing into a belt.

**Build.** A cheap 2D occluder test, cached:
1. Belts as occluders: a sight line crossing a belt polyline (outside its gaps, within half its width) is
   blocked, unless the shooter stands within 20 m of that belt and is on its side (he is at its edge, looking
   out).
2. Terrain: `geo::raycast` already exists; a coarse version with 25 m steps and the eye at 1.2 m decides
   whether the crest hides the target (matters at Robotyne).
3. The town: a vehicle behind a panel block's footprint is hidden from the OP and from positions inside the town.
4. Cost control: the test runs only when a team is ready to fire (the picture is already built lazily), results
   cached 0.5 s per team–target pair. Nine teams × 30 vehicles × 2 Hz is nothing.
5. The same test serves the picture of package G (what the OP and the teams can see) and the night-before map of
   next.md.

**Prove.** A headless day with a log of every launch and whether the line was clear; a Stugna sight shot from the
x = 560 position. The tally will move a little (fewer launches); band check.

**Cost.** 4–5 h.

### G. The feed: EW, the OSD, the picture (**look**, with one **sim** part) — observations 4, 9, 18, 19

**Now.** The feed shader does row tearing, block freeze and chroma shift; jamming is a scalar; the OSD is a text
line; sound ignores jamming.

**Build** (the numbers and layouts are in §2.6):
1. *The feed as a video pipeline.* The camera renders to an image (the headless path does this already) and a
   fullscreen quad shows it through the feed material. This gives what a post-process cannot: *freeze frames*
   (skip the camera's render for n frames, the quad keeps the last picture), *stale frames and catch-up* (a ring
   of 2–3 images), *resolution drops* (sample the image through a block average when the "bitrate" falls), and
   the grey "no signal" with the OSD still live. Measure the cost; the headless path says it is small.
2. *The digital ladder*, driven by a link quality 0..1 that falls gradually towards the jam line and drops off a
   cliff past it: soft and smeary → macroblocks and colour bleeding where things move, frame rate halved → frames
   repeating 0.3–2 s then a catch-up jump → freeze on the last frame, red broken-link icon, telemetry stopped.
   Two links: video and control can fail separately (a frozen picture while the drone still answers, and the
   reverse). After 6 s without control the drone brakes, hovers, backs 50 m along its track and flies home at the
   return height, and stays in return-to-home until the player cancels it after the link returns: EW costs the
   player the drone's position, not only the picture. GNSS jamming: satellite count collapsing, attitude mode
   (the drone drifts with the gusts it already has), the lat-lon readout walking. No snow, no rolling bands: that
   is analog FPV and not this drone.
3. *Sound under EW.* The drone has no microphone, so what EW really changes is the controller: a flashing
   banner ("Strong Interference to Aircraft. Fly with caution" → "Weak signal" → "Signal Lost") and warning beeps
   rising with the jamming. In the game's convention of listening at the camera, the world's sounds also drop out
   in step with the frozen frames, and the radio net stays clean. The ambience mix already has the hooks
   (`AmbienceLevels`).
4. *The OSD*, the Pilot 2 layout of §2.6 built from UI nodes with `TextShadow`: the top bar (status text that
   flashes the warnings, mode, battery bar and minutes, satellites, the three signal dots, battery %), the left
   tool column, the right camera column with the red record dot and timer and the zoom factor, the camera strip,
   the map thumbnail, the compass rose with VS, ALT, ASL, D, speed, wind and gimbal tilt, a thin crosshair. The
   drone is a Mavic 3T: no rangefinder, so range comes from the map and pins (an honest limit the player feels).
   The OSD never breaks up (the controller draws it over the video), which is the detail that sells the EW.
5. *The picture and the detection overlay* (**later**, with the campaign's spot and confirm tasks; the design
   stays here). §2.6 says AI boxes on a Mavic feed
   are a 2024 thing; what a 2023 operator had was PinPoints and the command post's Delta marks. So the overlay is
   the *picture*, not a detector: a `Detection` per attacking vehicle in the frame accumulates sight time (faster
   when zoomed and close, none under smoke or behind an occluder from package F); after 2–6 s a thin white box
   with "?" appears (the operator's own PinPoint); the player identifies it (a key cycles T-80 / BMP / MT-LB / …,
   or a click on the box) and a correct call labels it; after 10 s the command post confirms it and the box
   becomes an APP-6 red diamond with the equipment icon, which stays on the map for the day (Delta never deletes
   a mark); boxes outside the frame become edge arrows. Confirmed marks are what the ATGM teams and the brigade
   guns act on in AUTO (next.md's "spot" and "confirm"), so misidentification and lost sight have a cost. An
   optional "AI assist" toggle can draw the YOLO-style green boxes with "tank 0.87" labels, jitter and flicker,
   for the later chapters where it is historical. Boxes are UI nodes placed by `Camera::world_to_viewport`.
6. *The sights* (small, after 1–5). The Stugna-P view becomes the PDU-215 panel: a 4:3 low-resolution TV picture at
   12x with a crosshair and an auto-track gate that holds the vehicle, steady at launch (the gunner is 50 m from
   the tube), the launch bloom on the screen; the Javelin view the CLU's black-and-green thermal with its flashing
   corner gates going solid at lock and the status words round the frame; the binoculars a mil scale. The ground
   view's tube-and-tripod posture stays for the Javelin only.

**Prove.** An EW clip with the drone crossing the jam line; an OSD screenshot beside the reference layout; a
detection clip. `same-day.sh` identical for 1–4; a new baseline for 5.

**Cost.** 13–18 h now (pipeline 5, EW 5, OSD 4, sights 3); the picture (6 h) later.

### H. The sky (**look**) — observation 17

**Now.** A clear colour and distance fog.

**Build.** First the clock: Kyiv time (§2.7, decision 6), the sun an hour earlier in `world.rs`, the day's
start and the banners with it, a note in the dossier. Then a sky dome mesh with an unlit procedural shader: a stratus deck (cover, base height, lumpiness from
fbm, drift with the wind), a soft sun disc that shows through thin high cloud and vanishes under a deck, horizon
haze matched to the fog, and the deck's underside lit from the sun's side. Parameters per day from the weather
table in §2.1 (6 Feb: 100% low grey deck, base 300 m, the drone just under it; 8 Jun: a cirrus veil, hazy orange
sunrise, broken altocumulus by afternoon) and for the campaign days of next.md. `bevy_weather` exists for 0.19
with raymarched clouds, but a dome is enough for a grey deck and costs nothing per pixel.

**Prove.** Dawn, noon and dusk shots from the drone and the OP for both days, under both February presets, and
the choice between them made from the pictures. `same-day.sh` identical for the sky; the clock change is a
**sim** change (the banners' times, the light) and gets a baseline.

**Cost.** 7 h.

### I. Trees and vegetation (**look**, asset seam) — observation 7

**Now.** A belt is three rows of trunks at 2.6–5 m spacing with three crossed branch or leaf cards each, plus
scrub cards; from the trench it is a palisade of identical cutouts, uniform in height, with no undergrowth, and
the field in front is a flat plane.

**Decision.** The basics now, so the belts read right from the trench; the tree *look* itself waits for the
assets. What is done now is what the assets will need anyway and what no asset fixes:
1. *The seam.* A `Flora` kit: species × season × damage state → mesh handles (trunk, crown, LOD), and a scatter
   that places instances along the belts with the density, rows, gaps, scrub and shattered trunks it has today.
   The procedural cards are the placeholder kit; a glTF kit plugs in without touching the scatter.
2. *Composition.* Real winter belts are dense pale thickets: per `footage-comparison.md`, ours are darker and
   sparser. More stems per row (1.5–2.5 m), two storeys (tall trees and a shrub layer 1–3 m), height variation
   ±30%, lean, and a litter and snow-drift strip at the belt's foot painted into the ground (package B's shader
   can do the drift).
3. *The trench.* The ground view needs a foreground: the parapet, a few sandbags, spoil, a log, dead grass and
   stubble in the first 20 m (a detail layer in B), so the eye has scale.
4. *Shadow.* The belt casts a shadow band on the field in low sun (today crowns are `NotShadowCaster`); test the
   cost of letting the trunks cast and the crowns not.

5. *Documented for later* (with the assets or after): proper crowns with depth (several card layers at
   different radii, a dark core, lit rims), a wind sway vertex shader on the cards, species variety (ash, locust,
   wild apricot, the oaks of the older belts) and their winter silhouettes, shelled trees as split trunks with
   hanging crowns, a litter of broken branches in the belt, the belt's own ground (leaf litter, snow drifts on the
   lee side, the ditch along the field edge), summer undergrowth and nettles at the trench, and LOD ranges so a
   belt at 2 km is a band of colour and at 20 m a thicket.

**Prove.** The two ground-view shots retaken.

**Cost.** 8 h now; the look lands with the assets.

### J. Ukrainian names and voice (**look**) — observation 16

**Build.**
1. *Font.* Embed Fira Mono (OFL, full Cyrillic including ґ є і ї) as the game's font; Bevy's default subset has
   none. Keep it monospace: the OSD and the ledger want it.
2. *A string table.* `text.rs` with every place, formation and weapon as (uk, en), and a `Name::show(register)`
   that renders «Вугледар (Vuhledar)» in the world register and "Vuhledar (Вугледар)" in the bookkeeping
   register, as §2.4 recommends.
3. *The net.* Radio lines become (uk, en) pairs rendered as two lines, Ukrainian full weight, the English gloss
   dimmer; the 42 researched lines seed the table, with the verified skeleton words (плюс, прийняв, відбій,
   вихід, прихід, мінус, двохсотий, трьохсотий, пташка, коробочка, посадка, Кропива, 4.5.0) left untranslated in
   the gloss. Callsigns for the net: Маестро (the player), Беркут (the guns), Сокіл (the OP), Граніт (the CP);
   fictional, as next.md decided.
4. *Banners, briefing, debrief, README* follow the same table. Russian-derived spellings nowhere.

**Prove.** Screenshots of the HUD, a radio exchange and the debrief; `same-day.sh` identical (strings only).

**Cost.** 6–8 h (the table and the lines are most of it).

### K. The debrief (**look**) — observations 10, 11, 12

**Build.** The end-of-day screen as a proper layout, not a text blob:
1. Heading: the date and the clock span, in both registers («6 лютого 2023 · 6 February 2023»), no "THE DAY".
2. The day's own account as aligned tables (Bevy UI grid): the columns one per row with their vehicles,
   destroyed, abandoned; the men; the cost; the stock left. Numbers right-aligned.
3. The closing lines about what came after stay, without a heading, set apart by space.
4. **Sources**, beneath the account, as a table: source, date, what it says about the day; each row a link that
   opens in the browser on click (Bevy UI has no hyperlinks: a button that runs `xdg-open`, with a platform
   fallback). No casualty figures from the record set against the day's own numbers: the sources say where the
   day came from, the ledger what it cost.
5. R and Esc as now.

**Prove.** Debrief screenshots for both dioramas at 1600×900 and 1920×1080.

**Cost.** 4–5 h.

### L. Top attack (**sim**) — observation 21

The research (§2.5) settles it: the Javelin is the only top-attack weapon attested for these two days (the
72nd's ambush of three BMP-3s and a T-80 in one clip); NLAW is not attested at Vuhledar, and BONUS and SMArt 155
belong to other places and, for the 47th, to July. The current Javelin climbs 4 m.

**Build.** The real profile for both the teams' launches and the player's shot: 2–3 s lock (exists), soft launch
with a pop and a puff, the flight motor lighting a few metres out, a climb to an apogee of 60–150 m by range, a
cruise, a 50–70° dive onto the turret roof, 10–15 s of flight at 1.5–2 km; on the roof hit a turret-sized flash
and a 10–20 m grey-brown cloud, then the fate from package D with the top-hit weights (*est.* 30–40% immediate
cook-off, 40% fire then burn-out, 20% a mobility or firepower kill then abandonment). Kill chance on a top hit
higher than the side hit's 0.72.

**Prove.** A Javelin clip from the tree line (the `ground-javelin` camera) and from the drone. **Cost.** 3–4 h.

---

## 4. Order of work

Measured first, then the sim packages (they change the day once, early, so every later baseline is stable),
then the look packages against a fixed day.

| step | package | why here | the clip |
|---|---|---|---|
| 1 | A columns and driving | the overlap number and the stagger change every later clip | the kill zone, columns staggered, nobody through anybody |
| 2 | D fates + the fire programmes of C | the biggest realism gain per hour; sets what C must render | a fates contact sheet |
| 3 | E infantry | small, visible, finishes the kill zone | the bail-out close-up |
| 4 | F line of sight | small; it also serves G's picture | a day's launch log |
| 5 | C smoke and the TOS | the long look package, against the now-stable day | the smoke sheet, the TOS from the drone |
| 6 | B ground | independent, quick | the low pass before and after |
| 7 | H sky | independent, quick | dawn, noon, dusk on both days |
| 8 | G feed: pipeline, EW, OSD, sights | the pipeline underlies the OSD; the picture waits for the campaign | the jam-line clip, the OSD still |
| 9 | J names and voice | strings; best done once the radio lines of A–G exist | the net in two registers |
| 10 | K debrief | last, on the final numbers | both debriefs |
| 11 | I trees: the seam and the basics now, the look with the assets | the look waits for the assets | the two ground views |
| 12 | L top attack | after D (it is a fate) | a Javelin dive clip |

Total: about 85–105 hours. Steps 1–4 (the sim) about 30 h; 5–8 (the look) about 40 h; 9–12 about 25 h.

Every step ends as before: shots and clips archived in `devlog/`, `same-day.sh` run (identical for the look
steps, a new named baseline for the sim steps), the tally checked against the band, a commit.

---

## 5. Decisions (taken 2026-10-11)

1. **Sources in the debrief.** They stay, and get clickable links (a click opens the URL in the browser). What
   goes: any comparison of the record's casualty figures with the day's own numbers. The ledger is the day's
   account; the sources beneath it are where it came from, not a scoreboard. Package K builds it that way.
2. **Register in the HUD.** Latin first everywhere the player reads fast: "Vuhledar (Вугледар)". In-world text
   (radio captions, map labels) Cyrillic first.
3. **The detection overlay** comes later, with the campaign's spot and confirm tasks. Package G keeps the feed
   pipeline, the EW ladder, the OSD and the sights; the picture model is built only as far as line of sight
   (package F) needs it.
4. **Trees before the assets.** The easy basics now (package I.2–I.4: density and two storeys, height variation,
   the foot strip and the trench foreground, trunk shadows) so the belts read right from the trench; the rest is
   documented in package I for when the assets come.
5. **The 6 February sky.** Package H builds both presets (the overcast of the date and the hard sun of the
   footage), takes the same shots under each, and the choice is made from the pictures.
6. **The clock moves to Kyiv time.** Sunrise 06:51, sunset 16:38 at Vuhledar on 6 February; the sun, the day's
   start, the banners and the clock text shift an hour earlier; the dossier gets a note that its February times
   are Moscow time, and the claim graph is checked for which clock its timeline claims use.
7. **The font** is embedded (Fira Mono, OFL).
8. **A mobility-killed tank with a live crew can still pivot.** With one track gone it can turn on the spot
   round the dead side: to put its frontal armour towards the ATGM teams, or when the turret no longer traverses
   to lay the gun by turning the hull. Added to package D as a fate.
