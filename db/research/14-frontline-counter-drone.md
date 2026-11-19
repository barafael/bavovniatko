<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# 14 — Defending against tactical drones at the front, 2023–2026

Scope: how Ukrainian units at the front sense and counter Russian tactical drones (Lancets, radio and fibre-optic FPVs, roadside "waiting" drones, Orlan/ZALA/Supercam scouts, Molniya and bombers), and how both sides hunt each other's drone crews. The weight is on the defender: what each sensor can and cannot see, what each counter costs, and how each one failed in turn. Topics already covered in the knowledge base are summarised briefly here and cited to their existing sources: the measure → countermeasure table in `02-ew-countermeasures`, the 2023 Russian EW and Lancet picture in `11-drones-ru-ew-2023`, and the road-interdiction stories in the Kursk (c14) and Pokrovsk (c15) dossiers.

Accessed 2026-11-18. English-language sources, Ukrainian outlets paired with Western analysis. Russian claims appear only as Ukrainian or Western outlets report them, and are marked as claims. **Reported** means a figure a named party states (a count, a spec, a measured rate). **Estimate** means a figure the source itself presents as approximate or derived. **Vendor claim** means a manufacturer's own specification, untested in any fetched source. One source is known only from a search-engine excerpt and is flagged where used.

---

## 1. Summary

- **Every frontline sensor is blind to something.** RF detectors (Chuika, Obriy, ZORKO) read analogue FPV video out to about 2.5–4 km, and some show the attacker's own feed. They cannot see fibre-optic drones, which emit nothing, and they go blind as the Russians move to digital video and to new bands [src:united24-2026-brizard-detectors][src:esd-2025-felstead-obriy][src:euromaidan-2026-mukhina-molniya-digital]. Acoustic sensors hear fibre drones, but only at 150–450 m (vendor claim) [src:militarnyi-2026-zvook-nw0]. Small radars see fibre drones "several kilometres" out, but they emit and so can be found [src:twz-2025-altman-fibre-radar]. Against a fibre drone perched on the ground, only eyes on the road work: a drone sweep before a convoy moves [src:devua-2025-khandusenko-zhduny].
- **Jamming won 2023–2024 and lost at the end of 2024.** Trench, backpack and vehicle "dome" jammers protect a bubble of about 150–300 m [src:militarnyi-2024-kvertus-infantry-ew][src:pravda-2025-fpv-ew-shield]. Russian FPVs spread across the bands faster than the jammers could follow, and fibre ended conventional frontline EW at the end of 2024, in a Ukrainian EW company commander's account [src:euromaidan-2025-zoria-jamming-fails]. Jamming also causes fratricide: RUSI found it makes friendly drone operations "periodic rather than continuous" [src:rusi-2025-watling-third-year].
- **Fibre FPVs traded range and agility for immunity.** In early 2025 the range was about 10 km, with "degraded flight performance" and a risk of snagging [src:rusi-2025-watling-third-year]. By late 2025 most flew 10–25 km, and some Russian types reached about 50 km [src:bi-2025-epstein-fibre-wires]. The fibre trail let a Ukrainian unit follow it back to a Russian crew in February 2025 [src:euromaidan-2025-martyniuk-fibre-trace], but by May 2025 the busiest fields were so littered with old fibre that this "doesn't work anymore" [src:euromaidan-2025-axe-fibre-everywhere].
- **The waiting drone *(zhdun)* is the hardest threat.** A fibre FPV lands by a road with its motors off and its camera on. It can wait 12 hours according to one analyst [src:trenchart-2025-axe-waiting-drones], or days to two weeks on a large battery according to a Ukrainian maker's tests [src:devua-2025-khandusenko-zhduny]. Rubikon saturated 100–300 m road sections with them to cut the Sudzha supply road in February 2025 [src:trenchart-2025-axe-waiting-drones]. Counters are drone sweeps of the route, FPVs or grenades dropped on the perched drone, nets (under which waiting drones can also hide), and speed: one reporter outran a fibre drone at over 160 km/h [src:euromaidan-2025-zoria-khyzhak-zhduny][src:united24-2025-barkhush-fiber-counters][src:imi-2026-ratynskyi-detector].
- **Interceptor drones broke the Orlan/ZALA → Lancet chain.** RUSI counted 1,000–1,500 Russian reconnaissance-drone orbits a day in August 2024, then saw them thinned by radar-cued interceptors [src:rusi-2025-watling-third-year]. Lancet strikes reportedly fell from 180 in August 2024 to 24 in November 2024, according to a Ukrainian volunteer figure [src:united24-2025-toporkova-fpv-interceptors]. Russian scouts climbed to 4–5 km and gained rear-view cameras [src:euromaidan-2025-woodall-underground-interceptors].
- **Air defence near the front survives by staying back and by lying.** RUSI reported that whenever Ukraine pushed air defence forward, Russia "successfully found and engaged" it, usually with Iskander ballistic missiles [src:rusi-2025-watling-third-year]. A Lancet hit on an IRIS-T TRML-4D radar was filmed in June 2023 (secondary, unverified) [src:armyrecognition-2023-lancet-trml4d]. Decoys drew Lancets onto a fake IRIS-T in Kherson and a fake Su-25 at Kryvyi Rih in 2023 [src:telegraph-2023-barnes-iris-t-decoy][src:twz-2023-newdick-su25-decoy].
- **Both sides hunt the crews.** Russia's Rubikon finds Ukrainian operators by radio reconnaissance; one Ukrainian brigade reportedly lost up to 70% of its operators in a week [src:rferl-2025-rubicon]. From June 2025 Ukraine's new drone commander made Russian operators the main target, worth 25 points in the e-points system [src:euromaidan-2025-axe-hunt-drone-crews]. Passive direction-finders such as Kvertus Azimuth are offered for this, with ranges of about 15 km on the ground and 60 km raised on an aerostat (vendor claims) [src:twz-2024-newdick-aero-azimuth].

---

## 2. Detection at the front: what each sensor sees

### 2.1 Comparison

| sensor | what it reports | range (kind) | cannot see | emits? | sources |
|---|---|---|---|---|---|
| Handheld RF detector (Chuika 3.0/4.0, Obriy, ZORKO, Whoover, Dzyga) | alarm getting louder as the drone closes; a histogram per band; on analogue video, the attacker's own camera feed | up to ~4 km (Chuika, reported); ~2.5 km typical, up to 5 km (Obriy, vendor) | fibre drones; digital or encrypted video (detects but cannot show it); bands it does not cover; autonomous drones with no link | no | [src:kyivpost-2026-chuika-civilians][src:united24-2026-brizard-detectors][src:esd-2025-felstead-obriy][src:imi-2026-ratynskyi-detector] |
| Molniya video seen on a Chuika 4.0 | as above | up to 3 km (reported) | – | no | [src:militarnyi-2026-khomenko-chuika-4] |
| Sense 4 detector | four bands, up to 8.2 GHz | n/s | – | no | [src:euromaidan-2026-zoria-cd-t10] |
| Acoustic sensor (Zvook NW0) | 360° detection on a networked map, AI-classified, fibre drones included | 150–450 m (vendor claim); spaced 400–600 m apart | anything masked by wind, traffic and gunfire (not quantified) | no (data link only) | [src:militarnyi-2026-zvook-nw0] |
| Microphone arrays (concept, early 2025) | sound of motors; fibre drones are louder because they lift the spool | "conventional microphones struggle" beyond ~100 m | – | no | [src:euromaidan-2025-looijen-fibre-ir-sound] |
| IR laser and camera (Kara Dag concept) | reflection from the fibre, which is hard to see by eye; motor heat | n/s; a proposal, not shown fielded | – | yes (the laser) | [src:euromaidan-2025-looijen-fibre-ir-sound] |
| Mobile counter-drone radar (Birds of Magyar, Jan 2025) | a track on an incoming fibre FPV that cues an interceptor | "several kilometres" (reported); TWZ assesses it as a probable Ku-band type with limited early warning | – | yes: "can be detected, triangulated, and targeted" | [src:twz-2025-altman-fibre-radar][src:kyivpost-2025-madyar-fibre-radar] |
| CD-T10 X-band radar (Contra-Drone, 2026) | FPV, Shahed and guided bombs, with AI filtering of birds | 15 km, 3 km altitude (vendor claim); being tested with mobile fire groups | – | yes | [src:euromaidan-2026-zoria-cd-t10] |
| Networked air-defence radar and live map | aircraft above a set altitude, with speed, height and emitted signals, used to cue interceptors against Orlan/ZALA | n/s | low fliers | yes | [src:euromaidan-2025-woodall-underground-interceptors] |
| Eyes and recon drones | a waiting drone in the grass; the fibre glinting in the sun | line of sight; fibre "visible only in direct sunlight or from certain angles" | a camouflaged perched drone; night | the recon drone does | [src:bi-2025-epstein-fibre-wires][src:devua-2025-khandusenko-zhduny] |
| Russian "FPV traffic lights" (Dec 2024) | three states on a roadside mast: "FPV absent", "FPV nearby", "FPV far away" | n/s | probably RF-based, so fibre drones are missed unless it is acoustic (unconfirmed) | n/s | [src:devua-2024-kuzmenko-ru-fpv-traffic-lights] |

### 2.2 RF detectors in detail

- **How they work.** Chuika is a passive receiver that scans for analogue video and gives an audible alarm. When it intercepts a feed it shows what the Russian pilot sees, which helps the user judge the drone's heading [src:kyivpost-2026-chuika-civilians]. A full scan of the Chuika 4.0's bands takes 4–8 s [src:militarnyi-2026-khomenko-chuika-4].
- **The bands chase the enemy.** Chuika 3.0 covered 900–1680, 3060–3700 and 4990–6000 MHz. BlueBird says the Russians "frequently shift to custom or non-standard frequencies", which forces monthly updates [src:united24-2026-brizard-detectors]. Chuika 4.0 widened its bands to 4.5 GHz for Molniya video and added 6.04–8.8 GHz [src:militarnyi-2026-khomenko-chuika-4]. Kara Dag sells updates by subscription because "drone controls and telemetry change weekly". An Azov Corps EW specialist asks for coverage of 200 MHz–15 GHz [src:euromaidan-2026-kossov-obriy-detectors].
- **Digital video.** A digital feed can be detected but not shown on a cheap device [src:euromaidan-2026-kossov-obriy-detectors]. The Molniya's move to digital video in September 2026 threatens to blind the analogue detectors [src:euromaidan-2026-mukhina-molniya-digital].
- **In use.** A war reporter's account from 2025–2026 [src:imi-2026-ratynskyi-detector]:
  - Soldiers carry several detectors because one may catch a signal that another misses.
  - The early Tsukorok could not tell friendly from enemy drones.
  - A detector gives a poor sense of distance.
  - In Druzhkivka (January 2026) a Molniya feed appeared when the drone was under 1 km away, "a couple dozen seconds" from impact.
  - Near Kostiantynivka (December 2025) dozens of drones were overhead but a feed showed only once. A fibre "sleeper" by the road gave no warning at all.
- **Price and spread.** A Chuika 3.0 costs UAH 30,840 (about $690) and has been on sale to civilians since August 2026. Fedorov said detectors "should be everywhere" [src:kyivpost-2026-chuika-civilians]. Detectors can switch a jammer on automatically: "No threat, no activation" [src:euromaidan-2026-kossov-obriy-detectors].

### 2.3 Telling friend from foe

Most drones "sound the same" and share the same radio signatures. Ukrainian infantry sometimes panic and jam every band, or shoot at every drone they see. Operators announce their flights to nearby units in Signal group chats [src:bi-2025-baker-panic-jamming]. RUSI notes that jamming and the deconfliction it requires make friendly drone operations "periodic rather than continuous", and that Ukrainian strikes are timed to planned pauses in friendly jamming [src:rusi-2025-watling-third-year].

---

## 3. Counters at the front

### 3.1 Electronic warfare: trench, backpack, vehicle and dome

| system | type | band (MHz) | radius | other | kind | source |
|---|---|---|---|---|---|---|
| Kvertus AD Counter FPV F2 M30 | static dome | 420–1050 | up to 150 m | 24 h; mains power; set up in under 30 s | vendor | [src:militarnyi-2024-kvertus-infantry-ew] |
| Kvertus FPV Backpack F3 U | moving infantry | 850–5900 | ~150 m | 180 min; ≤15 kg | vendor | [src:militarnyi-2024-kvertus-infantry-ew] |
| Kvertus AD G-6+ | directional gun | n/s | up to 1.5 km | 30° beam; 6.5 kg | vendor | [src:militarnyi-2024-kvertus-infantry-ew] |
| Kvertus Kraken F3 | vehicle, directional or dome | 700–1050 | >200 m | 30 or 50 W; ~40 kg; UAH 350–400k | vendor | [src:militarnyi-2024-kvertus-infantry-ew] |
| Dome jammers in general | – | – | 200–300 m; directional jammers reach kilometres | trench EW weighs 20–50 kg plus a generator | reported | [src:pravda-2025-fpv-ew-shield] |
| Serval Cube (58th Brigade, 2026) | dome | n/s | n/s | "suppressed every drone" in a staged trial of Mavics and FPVs; fibre not tested | unit claim | [src:euromaidan-2026-mukhina-serval-cube] |

- **The band race.** In 2023 FPVs flew on a few standard bands, mostly 2.4 GHz and 915 MHz, to which trench EW was tuned [src:pravda-2025-fpv-ew-shield]. Ihor Shutyi (EW company, 56th Brigade) recalls that vehicle jammers worked in early 2024. As Russian control links spread across 750–1050 MHz, a vehicle needed one jamming module in spring 2024, 4–5 by summer and 7 by autumn, and ran short of power [src:euromaidan-2025-zoria-jamming-fails]. Beskrestnov's sector snapshot: 20% of Russian FPVs on 400–490 MHz, 30% on 720–1020 MHz, the rest on 2.1–2.3 GHz. Captured chips cover 300–1100 MHz, so a band can be changed by a firmware update [src:pravda-2025-fpv-ew-shield].
- **Collapse.** Shutyi dates the "collapse" of conventional frontline EW to the end of 2024, because of fibre. Russian FPV reach grew from about 3 km to 10 km by spring 2025, and the big vehicle EW systems were pulled back from the front [src:euromaidan-2025-zoria-jamming-fails].
- **Where EW still works.** In April 2025 a crew from the 58th Brigade drove over a radio FPV hidden on a road in the Kharkiv sector. The soldier believed their stationary EW had kept it from lifting off, and two more jammed drones lay unexploded nearby [src:kyivpost-2025-struck-drone-ambush]. RUSI (early 2025): most vehicles entering the combat zone carry jammers against command links and video, and EW "significantly" reduces drone reliability [src:rusi-2025-watling-third-year].
- **Molniya.** Ukrainian ELINT can detect a Molniya by its video. Its control receiver does not transmit, though, so the control frequency could be anywhere in about 150–2800 MHz, with the operator 30–60 km away amid "signal chaos" (Beskrestnov) [src:tsn-2026-shostak-molniya-ew]. An AI variant with no control antenna was first shot down in July 2026. Beskrestnov says its low radar and IR signature may delay detection until it is "nearly on its target" [src:euromaidan-2026-vivdych-ai-molniya].
- **The jammer is a beacon.** A Ukrainian FPV fitted with a 12-band emission receiver homes on Russian trench EW [src:militarnyi-2025-kushnikov-ew-hunter-fpv]. Russian ELINT satellites map Ukrainian emitters [src:pravda-2025-fpv-ew-shield]. Rubikon picks out EW for strikes; per LostArmour, as relayed by RFE/RL, about 15% of its strikes hit radar, communications and EW [src:rferl-2025-rubicon].

### 3.2 Kinetic counters at close range

- **Small arms.** A Militarnyi report on a Kvertus demonstration called small-arms fire at FPVs "ineffective but still common" [src:militarnyi-2024-kvertus-infantry-ew].
- **Shotguns.** From 2025 shotguns became standard against fibre drones. Business Insider quotes a trainer saying "EVERY unit is equipped with a shotgun"; this is known only from a search-engine excerpt [src:bi-2025-shotguns-fibre]. By June 2026 the 151st Training Centre warned that the 15-inch Russian fibre FPVs carry warheads close to a TM-62 mine:
  - A detonation at 10–15 m is "life-threatening", and the safe engagement distance is now at least 40–50 m.
  - Such a drone is easier to hit with a rifle at 50–100 m.
  - The issued No. 1–5 shot is too fine for it.
  - Some mobile fire teams carry only shotguns [src:militarnyi-2026-khomenko-shotguns-15inch].
- **Other ground-level counters** (2025): heated tungsten wire that snaps fibre, net guns, shotgun teams, daytime and foot logistics, and varied routes [src:united24-2025-barkhush-fiber-counters].
- **Gun turrets and lasers** were proposed in early 2025 with no fielded effect [src:euromaidan-2025-looijen-fibre-ir-sound]. A purported video from March 2026 shows a drone sweeping a road with a laser, perhaps against fibre; this is unverified [src:trenchart-2026-axe-laser-roads].

### 3.3 Passive defence: nets, cages, buildings

- **Net tunnels.**
  - Russia netted about 2 km between Bakhmut and Chasiv Yar in February 2025 [src:defenseexpress-2025-russian-net-tunnels], and Zaporizhzhia began building in September 2025 [src:euromaidan-2025-murdoch-net-tunnels].
  - The MoD's building pace rose from 5 km a day (January 2026) to 12 km a day (February 2026), against a target of 20 km [src:euromaidanpress-2026-tril-net-tunnel-pace].
  - By mid-2026, 1,066 km had been built, with 4,000 km targeted by year's end [src:euromaidan-2026-mukhina-anti-drone-nets].
  - Failure modes: drones "find the entrance or create one" [src:kyivindependent-2025-farrell-24h-pokrovsk]; nets burn and melt, and waiting drones can hide under them [src:united24-2025-barkhush-fiber-counters]; large fibre FPVs fly through gaps in the netting [src:militarnyi-2026-khomenko-shotguns-15inch].
- **Cages and turtles.** The chain ran from improvised cope cages (mid-2023) to factory cages, then "turtle" boxes with jammers (April 2024) and an 8 mm steel "super turtle" (August 2024). Each step cost visibility, mobility and gun traverse [src:kyivpost-2024-turtle-tanks]. RUSI describes engineers building cages and steel nets over positions near the line of contact [src:rusi-2025-watling-third-year].
- **Buildings.** Buildings block radio FPVs, so troops hide vehicles in barns and warehouses. Fibre FPVs are not blocked and fly through doors, windows and holes in roofs [src:euromaidan-2025-axe-fibre-everywhere].

### 3.4 Decoys, camouflage, emission control, movement

- **Decoys against Lancets.**
  - A Lancet struck a dummy IRIS-T launcher in Kherson whose camouflage net had "failed" (reported September 2023) [src:telegraph-2023-barnes-iris-t-decoy].
  - Metinvest built metal decoys with real heat and radar sources to fool thermal and radar sensors. An M777 replica costs about £800 and goes up in about 30 minutes [src:telegraph-2023-barnes-iris-t-decoy].
  - On 30 November 2023 a Lancet, watched by an Orlan-type relay drone, struck a detailed Su-25 decoy at Dolgintsevo, about 45 miles from the front. The same base had lost a real MiG-29 and a real Su-25 to Lancets that autumn [src:twz-2023-newdick-su25-decoy].
  - Further decoy material is already in the knowledge base [src:kyivpost-2023-decoys][src:euromaidanpress-2024-martyniuk-decoy-makers].
- **Hides.** RUSI (early 2025): building decoy and reserve positions and hides is "a major task". All command posts are buried [src:rusi-2025-watling-third-year].
- **Emission control.** Detectors are passive by design [src:imi-2026-ratynskyi-detector]. Radars, jammers and drone control links are not. TWZ warns that commercial counter-drone radars can be triangulated and targeted [src:twz-2025-altman-fibre-radar]. A Ukrainian soldier observed that Russian Wi-Fi-style relay antennas need line of sight, "making every antenna a marker" (search excerpt only; not cited).
- **Moving.** Ukrainian crews under Rubikon pressure mask themselves, change launch sites and limit movement (reported via aggregators; see §7). Interceptor teams fly from underground bunkers, with a sapper carrying the drone to a launch point away from the bunker [src:euromaidan-2025-woodall-underground-interceptors]. Russian bloggers advised their crews to move more often, put up false antennas and separate operators from technicians [src:euromaidan-2025-axe-hunt-drone-crews].

### 3.5 Interceptor FPVs against Orlan, ZALA, Supercam and Molniya

- **Scale.** RUSI counted 1,000–1,500 Orlan-10 and ZALA orbits a day in August 2024. By 2025 networked radar and electro-optical guidance of interceptors had "inflicted steady losses" [src:rusi-2025-watling-third-year].
- **Effect on the Lancet.** Ukrainian doctrine targets the ZALA scouts that guide Lancets. Volunteer Serhii Sternenko's figure for Lancet strikes fell from 180 in August 2024 to 24 in November 2024 [src:united24-2025-toporkova-fpv-interceptors]. This is a single partisan count, not an independent series.
- **Parameters** (38th Marine Brigade, 2025, reported) [src:united24-2025-toporkova-fpv-interceptors]:
  - Interceptors fly at 160 km/h out to 35 km, with up to 1 h of endurance, and cost $1,600–2,500.
  - The targets fly at 60–100 mph at about 1,500 m.
  - Interceptors need external target data (position, speed, bearing), usually from radar. Finding a camouflaged scout on a small screen is "like trying to find a needle in a haystack."
- **The crew loop** (113th TDF battery and Phoenix, Kharkiv, October 2025) [src:euromaidan-2025-woodall-underground-interceptors]:
  - Radar tracks the scout from launch.
  - A navigator guides the interceptor on a map, matching terrain to its camera.
  - The pilot makes the final visual acquisition.
  - The interceptor costs about $600, against $30,000–120,000 for the scout.
  - Interceptors struggle at night. Gun batteries had been barred from firing at scouts by day, because firing gave their position away.
- **Russian adaptations.** Scouts climbed to 4–5 km, above MANPADS and guns. They gained rear-view cameras and decoy drones [src:euromaidan-2025-woodall-underground-interceptors].
- **Molniya.** General Cherry interceptors made 43% of Molniya kills in March 2026 [src:euromaidan-2026-vivdych-ai-molniya]. RUSI's Ukrainian interlocutors judged Molniya-type powered bombs of low success and "easily intercepted" [src:rusi-2025-watling-combined-arms].

---

## 4. Fibre-optic FPVs

| item | value | kind | sources |
|---|---|---|---|
| First use | KVN, Kursk, August 2024 | reported | [src:kyivindependent-2025-farrell-fiber-optic] |
| Range, early 2025 | ~10 km | reported by Ukrainian operators to RUSI | [src:rusi-2025-watling-third-year] |
| Range, early 2025 (commentary) | ~2 km (small) to ~20 km (large) | analyst description | [src:euromaidan-2025-looijen-fibre-ir-sound] |
| Range, late 2025 | typically 10–25 km; some Russian types ~50 km | reported | [src:bi-2025-epstein-fibre-wires] |
| Spool cost (2026) | a 50 km spool rose from ~$300 to ~$2,500; a radio FPV costs from ~$350 | reported | [src:kyivindependent-2026-rushton-upper-hand] |
| Russian FPV share on fibre | up to 50% in some units (Lee, May 2026) | analyst estimate | [src:euromaidan-2026-kossov-obriy-detectors] |
| 15-inch fibre FPV (June 2026) | warhead close to a TM-62 mine | reported | [src:militarnyi-2026-khomenko-shotguns-15inch] |

- **Penalties.** The spool weighs "several kilograms", and the drone is slower and less agile [src:euromaidan-2025-looijen-fibre-ir-sound]. Range is limited and the fibre can catch on obstacles [src:rusi-2025-watling-third-year]. A snag "abruptly" stops a flight [src:bi-2025-epstein-fibre-wires]. Frost makes fibre brittle and freezes it on the spool, and the Dronarium centre grounded its fibre drones in January 2026 [src:euromaidan-2026-kossov-icy-fibre].
- **Speed.** One reporter says fibre drones are slow and "cannot catch cars above about 120 km/h". This is anecdotal [src:imi-2026-ratynskyi-detector].
- **The trail.**
  - On about 18 February 2025, Kara Dag Brigade operators south of Vodyane followed "sunlight-reflecting fibers" to a Russian crew in snow-covered buildings and struck it [src:euromaidan-2025-martyniuk-fibre-trace].
  - By May 2025, old fibre around Pokrovsk and Chasiv Yar made tracing "difficult, if not impossible", and many fibres lead to abandoned launch sites [src:euromaidan-2025-axe-fibre-everywhere].
  - By December 2025 fibre covered roads and fields. Ukrainian rangers had to decide each time whether a loose line was drone wire or a booby trap, which slowed night missions [src:bi-2025-epstein-fibre-wires].
- **Indoors.** A fibre drone can follow a target inside. In April 2025 a Birds of Magyar fibre FPV flew through a warehouse door and hit a BMP inside [src:euromaidan-2025-axe-fibre-everywhere].

---

## 5. Waiting drones *(zhduny)*

| item | value | kind | source |
|---|---|---|---|
| Name | *zhdun*, "the one who waits"; also "waiter" and "sleeper" | – | [src:militarnyi-2025-safronov-dormant-fpvs][src:imi-2026-ratynskyi-detector] |
| Perch | on the ground, in bushes, under the road surface, on buildings with a view; sometimes hovering low over a field | reported | [src:militarnyi-2025-safronov-dormant-fpvs][src:devua-2025-khandusenko-zhduny][src:kyivpost-2025-struck-drone-ambush] |
| Link | mostly fibre (no RF emission, immune to EW); some radio; some with small solar panels | reported | [src:devua-2025-khandusenko-zhduny] |
| Wait | up to 12 h on fibre (Roy, analyst) | estimate | [src:trenchart-2025-axe-waiting-drones] |
| Wait | "sometimes a whole day" (soldiers); TAF tests: 8,000 mAh at ~30% after 3 days, 15,000 mAh at ~30% after 2 weeks | reported / manufacturer test | [src:devua-2025-khandusenko-zhduny] |
| Target zone | 100–300 m road sections, saturated with several drones; group strikes on front, rear and sides; drones also detonated under vehicles like mines | analyst (Perpetua) | [src:trenchart-2025-axe-waiting-drones] |
| Ukrainian use | radio-linked waiting drones since about summer 2024; a Russian blogger described the tactic in November 2023 | reported | [src:devua-2025-khandusenko-zhduny][src:trenchart-2025-axe-waiting-drones] |

- **Effect.** The tactic was "a major factor" in cutting the Sudzha supply road in late February 2025 [src:trenchart-2025-axe-waiting-drones]. Waiting drones parked along roads were part of the Russian means that closed the entrances to Pokrovsk in 2025 (see c15).
- **Finding them.** They are "hard to detect from the air". Units sweep routes with drones before every logistics run [src:devua-2025-khandusenko-zhduny]. The Khyzhak Patrol Police brigade "constantly" monitors its zone and kills perched fibre drones with FPVs and bombers [src:euromaidan-2025-zoria-khyzhak-zhduny]. The 5th Assault Brigade destroyed several before launch in October 2025 [src:militarnyi-2025-safronov-dormant-fpvs]. Gun-armed UGVs reportedly patrol key roads to find them (March 2026) [src:trenchart-2026-axe-laser-roads].
- **Killing them.** Options are to drop a grenade, cut the fibre, or strike the operator. A Ukrainian expert says there is "practically no effective countermeasure" [src:devua-2025-khandusenko-zhduny].
- **Ambush in practice (April 2025).** A Russian team watched a rotation vehicle drive in and hid an FPV "under the road" for the return trip. It went off under the engine guard, and all four soldiers survived with concussions [src:kyivpost-2025-struck-drone-ambush].

---

## 6. Defending radars and air defence against Lancets and FPVs

- **2022–2023: the Lancet's first targets.** Until March 2023 Oryx's confirmed Lancet/Kub hits were mostly on artillery, radars and SAM launchers [src:oryx-2022-lancet-kill-list]. Lancet targets were found by Orlan-10 or Supercam [src:kyivindependent-2023-farrell-lancet].
- **IRIS-T radar, June 2023.** A social-media video showed a Lancet hitting the TRML-4D radar of an IRIS-T SLM; without it the system cannot fire. The source is secondary, unverified and gives no location [src:armyrecognition-2023-lancet-trml4d]. Search excerpts also report Lancet strikes on P-18, 35D6M and AN/TPQ radars and an Israeli ieMHR (2024). None of these was fetched from a qualifying outlet; see §10.
- **Forward air defence is found.** RUSI (early 2025): Russian reconnaissance density "has prevented Ukraine from pushing air defences forward". When Ukraine tried, Russia found and hit them, "usually with 9M723 ballistic missiles" [src:rusi-2025-watling-third-year]. Short-range air defence that stays back still deters the Russian air force [src:rusi-2025-watling-third-year].
- **Gun batteries.** Daylight fire at scouts was banned for a time because it exposed the battery to fire. The 113th battery said "it is quite difficult to work from the ground" [src:euromaidan-2025-woodall-underground-interceptors].
- **The radar's own signature.** Counter-drone radars are emitters that can be triangulated [src:twz-2025-altman-fibre-radar]. Ukraine's CD-T10 radar is being fielded with mobile fire groups [src:euromaidan-2026-zoria-cd-t10]. No fetched source documents a Ukrainian counter-FPV radar lost to a Lancet or FPV.
- **Mirror image: Ukraine against Russian air defence.**
  - On 12–14 January 2026, USF mid-range drones with warheads over 10 kg hit six Russian systems (Tor, Tor-M2, Buk-M1, Buk, Strela-10 and a Vityaz 50N6E radar) 46–160 km behind the line [src:euromaidan-2026-zoria-six-air-defence].
  - The Centre for Information Resilience counted a 300% rise in attacks on Russian air defence and EW in March–April 2026, with nearly 80 systems hit [src:kyivindependent-2026-rushton-upper-hand].
  - How the targets were found is not stated.

---

## 7. Going on the offence: finding the crews

- **The 2022 baseline.** DJI AeroScope let Russia find Ukrainian operators within "five or six minutes" [src:c4isrnet-2022-skove-cloak-drones]. This is covered in 10 and 11.
- **Russian hunters: Rubikon.**
  - Rubikon was founded on 2 August 2024 and grew to about 5,000 personnel by spring 2026 [src:fpri-2026-lee-putiata-rubicon].
  - It specialises in EW and radio-signal reconnaissance to locate Ukrainian drones and operators. Since February 2025 it has systematically targeted operators flying from far behind the line [src:rferl-2025-rubicon].
  - Per LostArmour, as relayed by RFE/RL: more than 25% of its strikes hit drones, and about 15% hit radar, communications and EW. Rob Lee relays one Ukrainian brigade's report of losing up to 70% of its operators in a week [src:rferl-2025-rubicon].
  - Fedorov's answer was to move operators back by remote control and, in time, autonomy [src:euromaidan-2025-mukhina-rubikon-fedorov].
- **Ukrainian hunters.**
  - In June 2025, Brovdi, the new USF commander, made Russian drone operators the main target [src:euromaidan-2025-axe-hunt-drone-crews]. A killed operator is worth 25 e-points (a wounded one 12), against 12 for a killed soldier.
  - Methods named: following fibre to the crew, short-range drones on dugouts a few kilometres behind the line, and deep strikes on fibre factories [src:euromaidan-2025-axe-hunt-drone-crews].
  - Ukrainian radio intelligence once traced a Molniya's signal back to its operators [src:united24-2025-toporkova-fpv-interceptors].
  - RUSI: direction-finding is used to cue drone orbits, and mapping enemy EW decides when and where drones launch [src:rusi-2025-watling-third-year].
- **Direction-finding hardware.** Kvertus's Aero Azimuth, shown in September 2024, is passive SIGINT on a helium aerostat (Aerobavovna) [src:twz-2024-newdick-aero-azimuth]:
  - Detection range is about 37 miles (60 km) raised, against about 9 miles (15 km) for the ground version.
  - It triangulates operators at 15–19 miles.
  - These are vendor claims, and deployment is unknown.
- **What DF cannot find.** A Molniya's control receiver does not transmit, so DF finds its video, not its operator. Beskrestnov says the operator may be 30–60 km away [src:tsn-2026-shostak-molniya-ew]. Fibre and AI-guided drones give DF nothing [src:euromaidan-2026-vivdych-ai-molniya].

---

## 8. Measure → countermeasure, era by era

Additions to the chain in `02-ew-countermeasures` §2. Only the counter-drone strand is shown.

| era | attacker's measure | defender's counter | the counter's failure | sources |
|---|---|---|---|---|
| e5-counteroffensive-23 (2023) | Orlan/ZALA-cued Lancets on artillery and SAM radars; analogue FPVs on 2.4 GHz and 915 MHz | camouflage nets, decoys (heat and radar sources), trench EW on standard bands, cope cages | the Lancet's pre-detonating warhead defeats nets; decoys work only until reconnaissance gets careful | [src:telegraph-2023-barnes-iris-t-decoy][src:twz-2023-newdick-su25-decoy][src:pravda-2025-fpv-ew-shield][src:kyivpost-2024-turtle-tanks][src:euromaidan-2023-shandra-lancet-new-model] |
| e6-avdiivka-attrition (late 2023–mid 2024) | the FPV boom; bands spread to 750–1050 MHz | backpack and vehicle dome jammers (~150–300 m); turtle armour; handheld detectors | jamming modules per vehicle go from 1 to 7 within a year; power runs short; fratricide | [src:euromaidan-2025-zoria-jamming-fails][src:militarnyi-2024-kvertus-infantry-ew][src:kyivpost-2024-turtle-tanks] |
| e7-kursk-pokrovsk (Aug 2024–Mar 2025) | 1,000–1,500 scout orbits a day; fibre KVN; waiting drones on the Sudzha road | radar-cued interceptors against scouts (Lancet strikes 180 → 24 a month); mobile radar plus interceptor against fibre; fibre tracing | the "collapse" of conventional EW at the end of 2024; radars too few (call for one every 2–4 km) | [src:rusi-2025-watling-third-year][src:united24-2025-toporkova-fpv-interceptors][src:kyivpost-2025-madyar-fibre-radar][src:euromaidan-2025-martyniuk-fibre-trace][src:trenchart-2025-axe-waiting-drones] |
| e8-drone-kill-zone (2025) | fibre FPVs at 10–25 km; Rubikon hunts operators and EW; zhduny on logistics roads | net tunnels; shotguns; drone sweeps and FPV kills of perched drones; operator hunting with e-points; scouts forced to 4–5 km | fibre litter ends tracing; nets get entered or burned; zhduny hide under nets | [src:euromaidan-2025-axe-fibre-everywhere][src:rferl-2025-rubicon][src:euromaidan-2025-axe-hunt-drone-crews][src:united24-2025-barkhush-fiber-counters][src:euromaidan-2025-woodall-underground-interceptors] |
| e9-counteroffensive-26 (2026) | Molniya on unknown control bands, then digital video, then an AI variant with no link; 15-inch fibre FPVs with mine-sized warheads; ~50 km fibre | acoustic networks (Zvook); X-band radar with mobile fire groups; Chuika 4.0 bands; interceptors (43% of Molniya kills); 1,000+ km of netting; rifles instead of shotguns | analogue detectors go blind to digital video; nothing passive sees an AI Molniya; shotgun crews die inside the blast radius | [src:tsn-2026-shostak-molniya-ew][src:euromaidan-2026-mukhina-molniya-digital][src:euromaidan-2026-vivdych-ai-molniya][src:militarnyi-2026-zvook-nw0][src:euromaidan-2026-zoria-cd-t10][src:militarnyi-2026-khomenko-shotguns-15inch][src:euromaidan-2026-mukhina-anti-drone-nets] |

---

## 9. Game/sim relevance

These are design readings of the facts above for Sword or Shield and bavovniatko. They are kept apart from the cited text.

- **A signature matrix drives the fusion.** Each attacker carries a set of signatures, and each sensor reads a subset:
  - radio FPV: RF video and control, sound, sight;
  - fibre FPV: sound (louder, because it lifts the spool), sight, glint from the fibre in sun, the fibre trail on the ground; no RF;
  - perched fibre drone: sight only, and only up close;
  - Molniya: RF video (analogue until September 2026), sound, radar; its control link is invisible;
  - AI Molniya: radar and sight late, nothing else;
  - Orlan/ZALA at 4–5 km: radar; sound and sight are poor.
- **Detection ranges** to start tuning from (order of magnitude, from §2): RF detector 2.5–4 km for analogue video; acoustic node 150–450 m; small radar several km, 15 km for X-band (vendor); eye a few hundred metres for an FPV, and much less for a perched drone.
- **Warning time is seconds.** A Molniya first seen at under 1 km is "a couple dozen seconds" out. Add a detector scan cycle of 4–8 s as latency before the first report.
- **Detector failure modes to model:** band not covered (monthly updates); digital video (a warning without a picture); fibre (silent); poor distance estimate (an RSSI-only cone); several detectors disagreeing; friendly drones raising alarms.
- **Every effector has a cost:**
  - dome jammer: a 150–300 m bubble that also grounds your own drones (deconfliction pauses) and marks you for a home-on-jam FPV and for Rubikon;
  - radar: sees fibre drones but is a beacon;
  - shotgun: needs fire-control-quality track at 15–40 m, and inside about 15 m a large warhead kills the shooter;
  - nets: entrances, burning, and drones hiding underneath;
  - interceptor: needs a cue from radar and a navigator, struggles at night, and must be flown home and disarmed if it misses.
- **The waiting drone as a hidden-state contact.** It has near-zero existence confidence until a sweep drone or patrol finds it. Spawn it on 100–300 m road sections near choke points and give it hours to days of patience. Speed is a counter, with a soft threshold around 120 km/h.
- **The fibre trail as an operator clue that decays.** Early in e7 a fresh trail points home. As the fight goes on, litter raises the false-trail rate until tracing is useless (e8). Frost raises the snag and failure rate.
- **Hunting the operator by era:** AeroScope (minutes, 2022) → RF DF on uplinks (radio drones only) → fibre tracing (early 2025, then decaying) → nothing for AI drones. The defender's own emitters (jammers, radars, relay antennas) are the attacker's equivalent clues.
- **Pushing air defence forward should be punished.** A system placed within the Russian reconnaissance band (about 15 km of depth with diminishing density, tasked out to 40 km) is found and struck. Decoys with heat and radar sources should soak up Lancets until the enemy's reconnaissance gets more careful.

---

## 10. Open questions/gaps

- **Lancet and FPV losses of Ukrainian radars.** No fetched source in a qualifying outlet lists Ukrainian radar losses to Lancets or FPVs, or says how each radar was found (emission, Orlan sighting, or a pattern of fire). Search excerpts mention P-18, 35D6M, AN/TPQ-36/37, ieMHR and TRML-4D cases from Russian videos. The Oryx Ukrainian-loss page was too large to read in full. A systematic pass over Oryx's radar and SAM sections is the next step.
- **Warning-time measurements.** No source gives measured seconds from first detection (RF, acoustic, radar, eye or ear) to impact for an FPV. Only the "couple dozen seconds" anecdote exists.
- **Hearing and seeing ranges.** No source gives the distance at which an FPV, a fibre FPV, an Orlan at 1.5 km or 4–5 km, or a Molniya can be heard or seen by a soldier.
- **Radar ranges against FPVs.** The Birds of Magyar radar is unnamed ("several kilometres"). CD-T10 gives only a system-wide 15 km. No per-target detection range against a 7-inch FPV was found.
- **Acoustic sensors in combat.** Only vendor ranges were found (Zvook 150–450 m). There are no field results, false-alarm rates, or figures for masking by artillery noise.
- **Effectiveness shares.** There is no breakdown of FPV kills by counter (EW, shotgun, net, interceptor) for any era.
- **Russian waiting-drone counts.** There is no figure for how many zhduny were found or destroyed, or how many vehicles were lost to them.
- **Operator DF in practice.** No 2025–2026 report attributes a specific strike to RF direction-finding on an FPV operator, on either side. Rubikon's methods are described only as "radio-signal reconnaissance". The 70% operator-loss figure is a single relayed claim.
- **Business Insider on shotguns** (June 2025) is known only from a search excerpt; the page redirected and could not be fetched.
- **Lasers against fibre** rest on one unverified video.

---

## 11. Sources

New (in `db/staging/sources/14-frontline-counter-drone.yaml`):
- **euromaidan-2025-zoria-khyzhak-zhduny**: Khyzhak brigade kills perched fibre FPVs with drones (May 2025).
- **militarnyi-2025-safronov-dormant-fpvs**: 5th Assault Brigade finds and kills zhduny; how they perch (Oct 2025).
- **devua-2025-khandusenko-zhduny**: Zhduny mechanics, wait times, battery tests, counters (Oct 2025).
- **trenchart-2025-axe-waiting-drones**: Rubikon's 100–300 m ambush zones, the 12 h wait, Sudzha.
- **kyivpost-2025-struck-drone-ambush**: Ambush FPV under the road; stationary EW; the crew survives.
- **euromaidan-2025-looijen-fibre-ir-sound**: Acoustic and IR-laser detection concepts for fibre drones; spool penalties.
- **twz-2025-altman-fibre-radar**: Mobile radar cues an interceptor against a fibre FPV; radar limits and emissions.
- **kyivpost-2025-madyar-fibre-radar**: Same event; "radar every 2–4 km".
- **united24-2025-khomenko-fibre-detectors**: Same event; "several kilometres".
- **euromaidan-2025-martyniuk-fibre-trace**: Kara Dag follows fibre to a Russian crew (Feb 2025).
- **euromaidan-2025-axe-fibre-everywhere**: Fibre litter ends tracing; fibre drones fly indoors.
- **bi-2025-epstein-fibre-wires**: Fibre webs, snags, 10–25 km and ~50 km ranges.
- **euromaidan-2025-axe-hunt-drone-crews**: Brovdi makes operators the main target; e-points; Russian advice.
- **euromaidan-2025-mukhina-rubikon-fedorov**: Fedorov on Rubikon and remote control.
- **twz-2024-newdick-aero-azimuth**: Aerostat SIGINT to locate operators (vendor claim).
- **kyivpost-2026-chuika-civilians**: Chuika 3.0 range, price, video intercept.
- **esd-2025-felstead-obriy**: Obriy detector bands and range.
- **euromaidan-2026-kossov-obriy-detectors**: Analogue vs digital, weekly updates, jammer coupling, fibre share.
- **imi-2026-ratynskyi-detector**: A reporter's field experience with detectors; warning time; sleeper drone.
- **militarnyi-2026-zvook-nw0**: Acoustic FPV sensor, 150–450 m.
- **euromaidan-2026-zoria-cd-t10**: X-band counter-drone radar; Sense 4 detector.
- **devua-2024-kuzmenko-ru-fpv-traffic-lights**: Russian roadside FPV warning lights.
- **militarnyi-2024-kvertus-infantry-ew**: Kvertus dome, backpack, gun and vehicle jammers.
- **euromaidan-2026-mukhina-serval-cube**: Serval Cube dome trial.
- **bi-2025-baker-panic-jamming**: Friend-or-foe confusion and fratricidal jamming.
- **tsn-2026-shostak-molniya-ew**: Why EW struggles against Molniya.
- **euromaidan-2026-vivdych-ai-molniya**: AI Molniya with no link; interceptor kills.
- **militarnyi-2026-khomenko-shotguns-15inch**: Shotguns against 15-inch fibre FPVs; safe distances.
- **bi-2025-shotguns-fibre**: Shotguns as the stopgap against fibre (search-only).
- **trenchart-2026-axe-laser-roads**: Purported laser road sweep; UGV patrols against waiting drones.
- **united24-2025-toporkova-fpv-interceptors**: Interceptors against scouts; the Lancet decline; parameters.
- **euromaidan-2025-woodall-underground-interceptors**: Radar-cued interceptor crews in bunkers; scout altitudes.
- **telegraph-2023-barnes-iris-t-decoy**: Lancet hits a decoy IRIS-T; Metinvest decoys.
- **twz-2023-newdick-su25-decoy**: Lancet hits a Su-25 decoy at Dolgintsevo.
- **armyrecognition-2023-lancet-trml4d**: Lancet hits an IRIS-T TRML-4D radar (secondary, unverified).
- **euromaidan-2026-zoria-six-air-defence**: Ukrainian drones hit six Russian air-defence systems in 48 h.
- **kyivindependent-2026-rushton-upper-hand**: Fibre spool costs; interceptors against Lancets; attacks on Russian air defence.

Existing (knowledge base):
- **rusi-2025-watling-third-year**: Fibre ~10 km and snag risk; 60–80% FPV failure; EW fratricide; interceptors against 1,000–1,500 orbits a day; forward air defence found and struck; cages and nets; DF cues orbits; transparency to 3/15/40 km.
- **rusi-2025-watling-combined-arms**: Molniya-type bombs "easily intercepted".
- **rferl-2025-rubicon**: Rubikon radio reconnaissance; strike shares; 70% operator-loss claim.
- **fpri-2026-lee-putiata-rubicon**: Rubikon size and structure.
- **united24-2026-brizard-detectors**, **militarnyi-2026-khomenko-chuika-4**, **euromaidan-2026-mukhina-molniya-digital**: Detector bands and their blindness.
- **pravda-2025-fpv-ew-shield**, **euromaidan-2025-zoria-jamming-fails**, **militarnyi-2025-kushnikov-ew-hunter-fpv**: The EW band race, collapse and home-on-jam.
- **united24-2025-barkhush-fiber-counters**, **euromaidan-2026-kossov-icy-fibre**, **kyivindependent-2025-farrell-fiber-optic**: Fibre counters, icing, first use.
- **defenseexpress-2025-russian-net-tunnels**, **euromaidan-2025-murdoch-net-tunnels**, **euromaidanpress-2026-tril-net-tunnel-pace**, **euromaidan-2026-mukhina-anti-drone-nets**, **kyivindependent-2025-farrell-24h-pokrovsk**: Net tunnels.
- **kyivpost-2024-turtle-tanks**: The cage and turtle chain.
- **kyivpost-2023-decoys**, **euromaidanpress-2024-martyniuk-decoy-makers**: Decoys.
- **oryx-2022-lancet-kill-list**, **kyivindependent-2023-farrell-lancet**, **euromaidan-2023-shandra-lancet-new-model**: The Lancet's early targets, spotters and net-defeating warhead.
- **c4isrnet-2022-skove-cloak-drones**: AeroScope operator finding in 2022.
