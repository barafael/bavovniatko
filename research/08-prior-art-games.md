# 08 — Prior-art games and wargames

Research for **bavovniatko** (Bevy wargame of the Russo-Ukrainian war, Ukrainian side only). Accessed 2026-09-28. Every claim is cited inline with its source id; the source list is in `sources/08-prior-art-games.yaml`.

## 1. Overview

Nobody has built what bavovniatko is aiming for. The prior art falls into five groups, and each one captures a different piece of the war.

- **Modern RTS and tactical games** (Broken Arrow, WARNO, Regiments, Combat Mission: Black Sea) model massed combined arms between NATO-style and Russian forces. Broken Arrow's unit classes are infantry, recon, vehicles, support, helicopters and air, with no drone class [src:wikipedia-2025-broken-arrow]. Its drones are recon or strike aircraft inside the air-defence matrix [src:steam-2026-broken-arrow-anniversary-update], and loitering munitions were still only a roadmap item in 2026 [src:steam-2025-broken-arrow-roadmap]. WARNO and Regiments are set in 1989 [src:wikipedia-2024-warno][src:steam-2022-regiments]. CM: Black Sea is a *fictional 2017* Russia–Ukraine war. It has UAVs and EW, but it was designed in 2009–2013 and deliberately avoids the "frozen conflict" that followed 2014 [src:wikipedia-2014-combat-mission-black-sea]. It has had no official 2022-era content since [src:steam-2023-cmbs-update-2-18].
- **Operational and strategic sims** of the real war exist mostly as community scenarios or small indie titles. Examples are a TOAW IV "Ukraine 2022" scenario [src:matrixforums-2022-toaw-ukraine-2022] and Sand Table's *Ukraine War 2022* [src:steam-2024-ukraine-war-2022]. Command: Modern Operations' Ukraine-2022 scenarios are mostly *hypothetical NATO interventions* [src:matrixgames-2022-cmo-ukraine-scenarios]. The best commercial models of an operational layer are Cold War and WWII titles: Flashpoint Campaigns' WEGO orders and SOPs on 500 m hexes [src:steam-2023-flashpoint-southern-storm], and Steel Division 2's turn-based battalion campaign wrapped around real-time battles [src:steam-2019-steel-division-2]. The one large-audience RTS of the 2022 war, *Gostomel Heroes*, is told from the Russian side [src:steam-2026-gostomel-heroes-dlc].
- **Drone games and trainers**, many of them Ukrainian, are first-person and cover one drone at a time. Examples are UFDS [src:steam-2025-ufds], Obriy [src:ukrainesarmsmonitor-2026-obriy], *Remote Reaper* [src:steam-2025-remote-reaper] and the arcade *Death From Above* [src:steam-2024-death-from-above]. They model piloting, bombs and sometimes EW, but never the formation or operational picture. The first attempt to join the two, *Hell Of War* (early access, September 2026), puts the player in a command drone that can drop into an FPV [src:steam-2026-hell-of-war].
- **Board and professional wargames** are the most thoughtful about logistics, politics and attrition, but they cluster at the start of the war or at grand-strategic scale. GMT's *Defiance* covers Feb–Apr 2022 [src:gmt-2025-defiance], and Farren's *Glory to the Heroes* uses 150 km hexes [src:paxsims-2023-farren-glory-to-the-heroes]. Expert reviewers say that "relatively few of the existing commercial wargames" capture the dispersed, drone-saturated battlefield of 2025 [src:paxsims-2025-brynen-emergent-approaches].
- **Real-war gamification** may be the most important "prior art" of all. The Army of Drones Bonus (ADB) points economy is a live game system run by the Ukrainian state. Its prices have been rebalanced like a live game's: a killed soldier went from 2 to 6 to 12 points between early 2025 and early 2026 [src:united24-2025-khomenko-adb-launch][src:kyivpost-2026-korshak-epoints]. It is ethically contested [src:wotr-2026-almajdalani-gamified-war].

## 2. Games catalog

| title | developer (country) | year | genre/scale | depicts this war? | models well | misses | source id |
|---|---|---|---|---|---|---|---|
| Broken Arrow | Steel Balalaika (Russia-based); pub. Slitherine | 2025 | large-scale RTS, up to 5v5 | No: US vs Russia in the Baltics | unit/loadout customisation, APS, modern air/AD; recon and strike drones (Forpost, Korsar, RQ-7, Predator) | drones are aircraft that SHORAD and autocannons shoot down, and drone HP was cut so one Tor missile kills one; "loitering ammunition" (missiles with man-in-the-loop or AI terminal guidance) was a 2026 roadmap item still being defined in March 2026; no FPV, EW or logistics layer | [src:wikipedia-2025-broken-arrow] [src:strategyandwargaming-2025-broken-arrow-review] [src:steam-2026-broken-arrow-anniversary-update] [src:steam-2025-broken-arrow-roadmap] [src:steam-2026-broken-arrow-march-qa] |
| WARNO | Eugen Systems (France) | EA 2022 / 1.0 2024 | division-scale RTS + turn-based campaign, 10v10 | No: 1989 | historical OOBs, income-based deployment | pre-drone era by design | [src:wikipedia-2024-warno] |
| Steel Division 2 | Eugen Systems (France) | 2019 | RTS + turn-based 1:1 strategic campaign on maps up to 150×100 km | No: Bagration 1944 | battalion moves and supply between real-time battles, auto-resolve, deck building | WWII; no drones | [src:steam-2019-steel-division-2] |
| Regiments | Bird's Eye Games (country not stated); pub. MicroProse | 2022; DLC 2024 | real-time tactics, regiment scale | No: 1989 Germany | platoon-level command (no per-soldier micro), on-the-fly task forces with off-map support, multi-day operations with resource management between battles | pre-drone; development wound down after 2024 | [src:steam-2022-regiments] |
| Flashpoint Campaigns: Southern Storm / Cold War | On Target Simulations; pub. Matrix | 2023 / 2025 (Red Storm 2014) | grand-tactical WEGO, 500 m hexes, battalion–regiment | No: 1989 | orders plus SOPs rather than micro, EW, air superiority, weather, morale and readiness, 4–24 h battles | Cold War; no drones | [src:steam-2023-flashpoint-southern-storm] |
| Combat Mission: Black Sea | Battlefront.com (bought by Slitherine 2024) | 2014 (Steam 2021) | battalion WeGo/RT tactics | Partly: fictional 2017 war with US | UAV recon/strike, EW conditions, APS, PGMs, thermal optics | predates 2022; the last official update (v2.18, March 2023) added only PBEM++; no FPV swarms or kill-zone logistics; deliberately avoids the frozen-conflict phase | [src:wikipedia-2014-combat-mission-black-sea] [src:steam-2023-cmbs-update-2-18] |
| Ukrainian Warfare: Gostomel Heroes | Cats Who Play (country not stated; Russian framing) | 2026 | squad–company RTS campaign | Yes (e1), from the Russian side: the VDV at Hostomel through to the Istanbul draft | modular damage without health bars, cover, morale and fatigue, resupply and encirclement, persistent squads, recon drones and large UAVs | calls the war the "Special Military Operation" and adds a Bucha skirmish map; about 2,000 Steam reviews, 90% positive; not offered in the Ukrainian or German Steam stores | [src:steam-2026-gostomel-heroes] [src:steam-2026-gostomel-heroes-dlc] |
| Hell Of War: Combined Arms | Ravens Row (country not stated; English and Ukrainian audio) | EA Sept 2026 | "real-time command action": FPS/RTS hybrid | War-inspired | command drone as the main view (recon, orders, fire support); drop into an infantry commander, vehicle, FPV or bomb-drop drone without pausing; limited fire-support budget | brand new and unreviewed | [src:steam-2026-hell-of-war] |
| Men of War II | Best Way (Ukraine); pub. Fulqrum | 2024 | WWII real-time tactics | No | squad-level direct control | WWII; the invasion delayed release several times | [src:wikipedia-2024-men-of-war-ii] |
| Call to Arms – Gates of Hell: Ostfront | Barbedwire Studios (Sagunt, Spain) with Digitalmindsoft | 2021 | WWII RTS/RTT (Men of War lineage) | No | dynamic campaign | WWII; not a Ukrainian studio | [src:steam-2021-gates-of-hell-ostfront] |
| Command: Modern Operations (Ukraine 2022 scenarios) | community authors; pub. Matrix/Slitherine | 2022 | air/naval/operational sim | Hypothetical: NATO convoy/NFZ what-ifs | sensor/platform database | the ground war as fought | [src:matrixgames-2022-cmo-ukraine-scenarios] |
| TOAW IV "Ukraine 2022" scenario | sPzAbt653, Josant (community) | 2022 | operational hex, battalion–brigade | Yes (e1) | 222 UA / 136 RU unit OOB, supply radius, low Russian proficiency for poor C2 | the engine forces Javelins into "rockets & missiles"; no drone layer described | [src:matrixforums-2022-toaw-ukraine-2022] |
| Ukraine War 2022 | Sand Table Software (country not stated) | 2024 | real-time operational, maps up to 2000 km | Yes: 20 scenarios 2022–23 (Kyiv, Kherson, Bakhmut) + what-ifs | theatre scale, air and airborne ops | drones and logistics not described; almost no reviews | [src:steam-2024-ukraine-war-2022] |
| Radio Commander | Serious Sim | 2019 | radio-only command, Vietnam | No | "you only know what they know"; the player updates the map | reports too precise, so the fog feels "theatrical" | [src:vice-2019-zacny-radio-commander] |
| Radio General | Foolish Mortals (Canada) | 2020 | voice-command WWII | No | double-blind map, vague status reports | a gimmick; weak AI; voice recognition fights efficiency | [src:saveorquit-2020-radio-general] |
| Squad | Offworld Industries (Canada) | 2015 EA / 2020 | 50v50 milsim FPS | Factions only (RU and AFU) | commander role calls UAV recon and artillery | not the war; drones minimal | [src:wikipedia-2020-squad] |
| Gray Zone Warfare | MADFINGER Games (Czechia) | EA 2024 | PvE-first open-world tactical shooter | No: fictional Southeast Asian island | — | unrelated setting | [src:steam-2024-gray-zone-warfare] |
| Escape from Tarkov | Battlestate Games | 1.0 2025 | extraction FPS | No: fictional PMC war in north-western Russia, 2015–2026 | — | fictional internal-Russian conflict | [src:wikipedia-2025-escape-from-tarkov] |
| S.T.A.L.K.E.R. 2: Heart of Chornobyl | GSC Game World (Kyiv; Prague office) | 2024 | open-world shooter | No: the Chornobyl Zone | full Ukrainian voice acting | not a war game; context in §6 | [src:wikipedia-2025-gsc-game-world] [src:euromaidanpress-2025-stalker2-gsc-undesirable] |
| Arma Reforger "Realistic Combat Drones" mod | SalamiBoiDev on Bohemia's platform | 2025–26 | milsim mod | War-inspired | signal model (LOS, jamming, terrain occlusion, RSSI/LQ), video degradation, backpack jammers, battery | no fiber-optic; squad scale only | [src:armaplatform-2025-realistic-combat-drones] |
| Arma 3 | Bohemia Interactive (Czechia) | 2013 | milsim | No, but its clips circulate as fake Ukraine footage | visual realism | realism becomes a disinformation vector | [src:bohemia-2023-arma3-fake-news] |
| Glory to the Heroes | Deaf Tone Games (Ukraine) | EA; 1.0 planned 2026 | PvPvE milsim, up to 425 km² ops | Yes: Krynky, Chasiv Yar canal, Izium plant | trench combat, massed artillery, UAVs incl. FPV, vehicles with crew positions | Russian-faction PvP drew Ukrainian backlash | [src:steam-2026-glory-to-the-heroes] [src:mezha-2024-danylov-gtth-russian-faction] |
| Ukraine War Stories | Starni Games (Kyiv, Ukraine) | 2022 | visual novel, free | Yes (e1): civilians in Hostomel, Bucha, Mariupol | eyewitness-based civilian experience | no military layer, by intent | [src:steam-2022-ukraine-war-stories] |
| Death From Above | Rockodile / Lesser Evil (Germany) | 2023 EA / 2024 | arcade drone action | Yes: AFU drone operator | three roles (ground operator, quad bomber, FPV); demining, convoys | explicitly not a sim; short | [src:steam-2024-death-from-above] [src:steam-2026-death-from-above-germany] |
| Remote Reaper | Vladyslav Fomenko (Ukraine, with active pilots) | 2025 | FPV strike sim | Yes | jamming on control *and* video links, terrain occlusion between ground station and drone, battery, armour weak spots, warhead choice (shaped charge, HE-frag, EFP), real RC radios | single drone; no crew or formation layer | [src:steam-2025-remote-reaper] |
| Squad 22: ZOV | SPN Studio (Russia) | 2025 | RTT, Russian propaganda | Yes, from the aggressor's view (2014, Mariupol, Avdiivka) | none worth copying | state-backed propaganda; blocked in Ukraine | [src:kyivindependent-2025-hodunova-squad22-zov] |
| UFDS | Simtech Solutions / Drone Fight Club Academy (Ukraine) | 2025 | FPV trainer + light wave/resource mode | Yes | real UA drones, physics, bombs, PID tuning, RC input, interceptor course; the military edition added the P1-SUN and Bullet interceptors in 2026 | Steam edition stripped of tactics | [src:steam-2025-ufds] [src:cbs-2026-clarke-ufds] [src:euromaidanpress-2026-ufds-interceptors] |
| Obriy | Twist Robotics (Ukraine) | 2024–26 | military drone trainer | Yes: 50,000+ km² of real front terrain | EW scenarios, navigator/crew roles, anti-Shahed interceptors, terrain updated with the front | military-only | [src:ukrainesarmsmonitor-2026-obriy] |
| WeTrueGun GTA V/FiveM server | WeTrueGun (Ukraine) | 2026 | modded sandbox | Shahed interception | skill upkeep, decompression | not real training (by its makers' account) | [src:rbcukraine-2026-kovalenko-gta-shahed] |
| Steel Beasts Pro / PE | eSim Games | UA use 2022–25 | armour sim, crew → brigade CPX | Used by the UA military | ballistics, engineering, logistics; cheap staff exercises | PE limited to 8 workstations; drones via VBS3 integration | [src:llnl-2023-lasch-simulation-support-uaf] [src:defender-2025-pokotylo-steel-beasts] |
| VBS4 + Ukrainian terrain | Bohemia Interactive Simulations | 2024 | tactical / mission rehearsal | Ukrainian terrain | drones, counter-drone, loitering munitions, trenches, AI entities | brigade ops, logistics and morale (vendor's own admission) | [src:nextgov-2024-breeden-vbs4-ukraine] |
| Chaika / Chaika-M ("Flyswatter") | Ukrainian (codified by the MoD, Nov 2025) | 2025 | VR air-defence trainer | Yes | engaging FPVs, Lancets, Shaheds and cruise missiles with MANPADS, heavy machine guns and small arms, incl. night and searchlight | military-only | [src:euromaidanpress-2025-seagull-flyswatter] |

## 3. Drone games & training simulators

- **UFDS** is the key crossover. Drone Fight Club Academy trained 5,000+ Ukrainian military pilots on it and sells a de-tacticised Steam edition for about $30. The military gets the full version free [src:cbs-2026-clarke-ufds]. The Steam build has Academy courses (kamikaze, bomber, air interceptor, racing), a Battleground wave mode with resource management, and real drones (Angel Arrow, LuckyStrike, Mimic 3T, Phantom, Dzhmil) [src:steam-2025-ufds]. In August 2026 the P1-SUN (SkyFall) and Bullet (General Cherry) became the first real interceptor drones in the military edition, which units request free through a form. The developers pitch it as a way for makers to show new platforms to operators before delivery [src:euromaidanpress-2026-ufds-interceptors].
- **Obriy** (Twist Robotics) is the military-grade counterpart. It trains pilot and navigator crews and mission planning, has EW scenarios and an anti-Shahed interceptor version (March 2026), and runs on 50,000+ km² of satellite-derived front terrain that is updated as the line moves. It is used by 150+ units and 50+ training centres [src:ukrainesarmsmonitor-2026-obriy].
- **Commercial FPV sims** are part of the pipeline. A Ukrainian vendor calls Liftoff the most popular sim among beginners and instructors, with VelociDrone for racing and DRL for beginners. It lists Ukrainian military sims (FPV Battleground by BAZU, UFDS, Obriy) and RealFlight, Unigine Sim and XFly in UAV programmes [src:vgi9-2025-drone-simulators]. This is promotional. No first-party school or MoD source naming Liftoff, VelociDrone, Uncrashed or DCL was found; the named, verified military sims are all domestic.
- **Domestic sims beyond drones.** The MoD codified the Chaika and Chaika-M ("Flyswatter") VR air-defence simulators in November 2025. Troops practise against FPVs, Lancets, Shaheds and cruise missiles with virtual Igla and Stinger MANPADS, DShK and M2 machine guns, including night scenes with the searchlights that mobile fire groups use. The MoD claims "significant skill improvement after just 7–10 sessions", and its footage looks like a first-person shooter with a weapon-select screen [src:euromaidanpress-2025-seagull-flyswatter].
- **Game engines get repurposed.** The WeTrueGun school runs a GTA V/FiveM server for Shahed hunts. Its makers say real interceptor training uses Obriy [src:rbcukraine-2026-kovalenko-gta-shahed].
- **Arcade and commercial titles.** *Death From Above* is an arcade take on the AFU drone operator (ground, quad and FPV roles) that donates to Come Back Alive and Army of Drones [src:steam-2024-death-from-above]. Its makers are German: it sits in Steam's "Games Composed in Germany" showcase, had donated EUR 20,500 by September 2025, and added a skin pack co-designed with the Unmanned Systems Forces [src:steam-2026-death-from-above-germany]. *Remote Reaper*, by an independent Ukrainian developer working with active pilots, has the richest consumer FPV model. Jamming can hit the control link as well as video in the last seconds of a run, terrain between ground station and drone weakens the link, and the warhead has to match the target (shaped charge, HE-frag or EFP) [src:steam-2025-remote-reaper]. The Arma Reforger drone mods have the most transferable EW model among moddable milsims [src:armaplatform-2025-realistic-combat-drones].
- **Army of Drones Bonus: the war's own game layer.** Units upload verified strike video to DELTA, earn e-points (ye-baly) and spend them on the Brave1 Market (a Vampire bomber cost 43 points). Infantry kills were repriced from 2 to 6 points, which reportedly doubled confirmed eliminations within a month [src:united24-2025-khomenko-adb-launch]. The 2025 total was 819,737 video-confirmed strikes [src:mod-2026-adb-results]. Capturing an enemy and evacuating wounded score higher than killing [src:wotr-2026-almajdalani-gamified-war]. By June 2026, 400+ units had ordered 500,000+ systems with points, and points also paid for reconnaissance, logistics and evacuation missions [src:euromaidanpress-2026-epoints-400-units]. The schema changes as edge cases surface: when a unit spent two FPVs on a soldier who then killed himself and got no credit, Fedorov proposed paying 12 points for video-confirmed "self-destruction" [src:euromaidanpress-2026-epoints-self-destruction].

  | when | change or value | source |
  |---|---|---|
  | before Apr 2025 | soldier killed 2 | [src:united24-2025-khomenko-adb-launch] |
  | Apr 2025 | soldier killed 6; tank damaged 20, destroyed 40; MLRS up to 50; Vampire bomber costs 43 | [src:united24-2025-khomenko-adb-launch] |
  | Aug 2025 | relaunched and tied to the Brave1 Market | [src:euromaidanpress-2026-epoints-400-units] |
  | Jan 2026 | soldier killed 12, wounded 8; drone operator 25; tank 40; MLRS 50; manned helicopter 100; prisoner taken alive 120 | [src:kyivpost-2026-korshak-epoints] |
  | Jan 2026 (planned) | points for air defence and aviation against drones, for snipers; "depth coefficients" for strikes on logistics and UAV crews | [src:mod-2026-adb-results] |
  | Mar 2026 | kill still 12; sniping, mobile fire groups and army aviation now scored; points can buy components; 12 proposed for confirmed self-destruction | [src:euromaidanpress-2026-epoints-self-destruction] |
  | Mar–May 2026 | Delta scores reconnaissance detections of high-value targets more than 15 km behind the line and ranks units monthly | [src:euromaidanpress-2026-delta-recon-ranking] |
  | Jun 2026 | reconnaissance, logistics and evacuation missions earn points; 800+ products listed | [src:euromaidanpress-2026-epoints-400-units] |

  Outlets keep repeating stale values (a June 2026 piece still quotes "roughly six points" per kill), so date every value you use [src:euromaidanpress-2026-epoints-400-units].

## 4. Board/hobby wargames

- **Defiance: 2nd Russo-Ukrainian War 2022-?** (GMT; D. B. Dockter & Mark Herman) covers the Kyiv and Chernihiv campaigns from 24 Feb to about 1 Apr 2022 in 3–4 day turns. It has five political tracks (Zelenskyy, NATO, Lukashenko, Putin, Russian MoD), supply, drones/air/missiles, VOVK partisans and a solitaire bot. The GMT page read "At the Printer" [src:gmt-2025-defiance]. The designer diary folds drones into detection and recon-strike, abstracts EW, ties Russian supply to railheads within about 50 miles, and lets troop quality outweigh force ratios. It calls the game "a first draft of history" [src:insidegmt-2023-dockter-herman-defiance].
- **Ukrainian Crisis** (Brian Train; a print-and-play in March 2014, boxed by Hollandspiele in 2017) is the precedent for designing while events unfold. Players spend finite chits across force, diplomacy, propaganda and prestige, and the conflict may never turn kinetic [src:hollandspiele-2017-train-ukrainian-crisis].
- **Glory to the Heroes** (Maj Ed Farren, British Army; free print-and-play) is a grand-strategic design derived from Philip Sabin, with 150 km hexes, monthly turns, attrition, climate, strategic bombing and propaganda [src:paxsims-2023-farren-glory-to-the-heroes]. It shares its name with Deaf Tone's shooter but is unrelated.
- **Other BGG listings** found by search include *2022: Ukraine* (the first year), *Putin's War 2022*, *Ukraine 2022: Tabletop Wargame* and *To Kyiv!*. Only their existence is confirmed [src:bgg-2022-ukraine].
- **Professional use of hobby games.** The Farren game circulates through PAXsims as a teaching tool [src:paxsims-2023-farren-glory-to-the-heroes]. A commercial armour sim (Steel Beasts PE) was repurposed for Ukrainian staff training, covered in §5.

## 5. Professional/defense wargaming

- **The pre-war track record was mixed.** A Marine Corps University game two weeks before the invasion anticipated the main axes. It missed because it had Russia destroy Ukrainian air defences first and could not model Zelenskyy's leadership or Ukrainian morale [src:wotr-2022-lacey-wargame-before-the-war]. The long-war follow-up (three-month turns, drones, munitions depletion, mobilisation, aid and sanctions) forecast stalemate and found that political objectives overrode military advice [src:mwi-2022-lacey-wargaming-long-war]. Earlier RAND Baltic wargaming shows how pre-2022 professional work centred on NATO–Russia massed manoeuvre [src:rand-2016-shlapak-baltics].
- **The 2023 counteroffensive showed the limits.** Eight US–Ukrainian tabletop exercises shaped the plan. A senior Ukrainian officer said afterwards that the methods "doesn't work" given ubiquitous drones, trenches and no air superiority. Brynen argues for proper post-mortems rather than rejecting wargaming [src:paxsims-2023-brynen-wargaming-doesnt-work].
- **The 2025 battlefield described by RUSI** has three zones (contested, a ~30 km middle and deep), a seven-phase offensive cycle, small dispersed packets, EW integral down to platoon, and FPV crews who "can see far more targets than they can hit" [src:rusi-2025-watling-combined-arms]. Soft vehicles cannot operate within about 10 km of the front, resupply goes by foot, quad bike, horse and UAV, and few commercial wargames model any of this [src:paxsims-2025-brynen-emergent-approaches].
- **Western training still lagged the drone war in 2025.** Interviews with trainers in Germany found that the German colonel running staff training for Ukrainians "used a wargame scenario straight out of the Cold War, refusing to add drones and other modern weaponry". The US training group cited "broken simulation platforms", and Ukrainian trainees rejected a drone course until it was rebuilt around frontline tactics in June 2025 [src:mwi-2025-hood-matisek-tingle-learn-or-lose]. A retired US general argues for digital games as training for fights full of "electronic jamming, drones, sensors, and missiles". He notes that Russia has codified PC-based simulator training for drone and anti-drone operators [src:wotr-2025-votel-military-gaming].
- **NATO–Russia games since 2025 put drones at the centre.**
  - A December 2025 game by Die Welt and the German Wargaming Center (Helmut Schmidt University) had Russia seize Marijampolė with about 15,000 troops. The German brigade in Lithuania failed to intervene "in part because Russia used drones to lay mines on roads leading out of its base". A Baltic ambassador disputed the premise that Lithuania would not fight [src:paxsims-2026-hybrid-warfare-nato-cohesion].
  - The ARRC's exercise Arrcade Strike (May 2026) rehearsed defending Estonia in 2030 from a "Ukraine-style bunker" under Charing Cross station. The plan has thousands of drones lead the counterattack. Yet the British Army is estimated to be 80–90% short of the drones it needs and would run out in under a week at a few hundred a day [src:guardian-2026-sabbagh-arrcade-strike].
  - Canadian Army structure games (2024–26), run by Maj Ed Farren, reached the lesson that "a platoon with a drone is better than platoon without a drone, but that wasn't the problem". The problem was that the enemy's "sense and strike complex was pretty much unassailable" by jamming, counter-UAS or counter-battery fire [src:canadianarmytoday-2026-thatcher-wargaming-modernization].
- **Dstl builds on commercial engines.** Since 2019 Dstl and Slitherine/Matrix Pro Simulations have built professional wargames on commercial game technology. They have been used for contingency planning, British Army structure and "the potential of remote and autonomous systems", and Dstl claims they cut analysis time and cost by 50% [src:govuk-2024-dstl-slitherine-gaming-tech]. No public Dstl game specifically on Ukraine's drone lessons was found.
- **Ukrainian military simulation.** From Dec 2022 to May 2023, volunteers used Steel Beasts PE to run brigade and battalion CPXs for newly generated formations. They integrated JCATS, SBPro and VBS3, with VBS3 drones feeding commanders' picture [src:llnl-2023-lasch-simulation-support-uaf]. By 2025 the National Guard had adopted a Ukrainian-localised Steel Beasts Pro [src:defender-2025-pokotylo-steel-beasts]. VBS4 now ships Ukrainian terrain with drones and loitering munitions, and its vendor concedes that brigade ops, logistics and morale were weak points [src:nextgov-2024-breeden-vbs4-ukraine]. Domestic tools fill the drone and air-defence end: UFDS and Obriy for crews and interceptors, and the MoD-codified Chaika VR trainers for shooting down drones [src:euromaidanpress-2026-ufds-interceptors][src:ukrainesarmsmonitor-2026-obriy][src:euromaidanpress-2025-seagull-flyswatter].
- **The Russian side** runs command-staff games (KShVI) that rehearse procedure, including EW and counter-UAV, but do not test assumptions. That left planners blind to drone saturation and ISR transparency [src:paxsims-2026-brynen-how-russia-wargames].
- **AI in wargaming.** A September 2026 GPT-assisted negotiation game was drafted in about two hours. Its designer insists that humans choose the abstractions and that AI output needs verification and playtesting [src:paxsims-2026-taylor-gpt-negotiation-game].

## 6. Critical perspectives & ethics (including Ukrainian views)

- **Ukrainian creators are the majority voice.** A 2026 academic survey counts 200+ Ukrainian-made games responding to the invasion (2022–2025). It frames them as documentation, emotional processing and cultural response [src:ehgs-2026-kot-ukrainian-games-of-war]. *Ukraine War Stories* shows the documentary and civilian approach [src:steam-2022-ukraine-war-stories]. A Ukrainian scholar argues that war games work as immersive "deep media" for information and counter-propaganda, and that this role calls for ethical and regulatory frameworks for their design [src:obraz-2024-zinovieva-deep-media]. Usachova reads *Play for Ukraine*, in which players ran DDoS attacks on websites said to serve the Russian army, as a "resistance pleasure" that turns players into "digital soldiers". The game there is a weapon, not a representation of war [src:tandf-2024-usachova-play-for-ukraine].
- **Ukraine's big studios build through the war.** GSC paused S.T.A.L.K.E.R. 2 at the invasion and moved part of the team to Prague [src:wikipedia-2025-gsc-game-world]. One of its original developers, Volodymyr Yezhov, was killed near Bakhmut in December 2022. In November 2025 Russia declared GSC "undesirable", citing about $17m it had sent to a fund that bought strike drones [src:euromaidanpress-2025-stalker2-gsc-undesirable]. Frogwares' staff split between humanitarian work, the volunteer army and hybrid work, and the studio pulled its games from sale in Russia [src:wikipedia-2025-frogwares]. 4A Games had already moved its headquarters from Kyiv to Malta in 2014 [src:wikipedia-2025-4a-games], and Best Way's Men of War II was delayed by the invasion [src:wikipedia-2024-men-of-war-ii].
- **Playing as Russians is a red line for many Ukrainians.** When Deaf Tone brought back Russian-faction PvP in *Glory to the Heroes*, Ukrainian press and players pushed back [src:mezha-2024-danylov-gtth-russian-faction]. This supports bavovniatko's Ukraine-only player stance.
- **Propaganda is the mirror-image pitfall.** *Squad 22: ZOV* was made with Russian MoD backing, frames the invasion as "liberation", and is marketed for cadet and Yunarmy training [src:kyivindependent-2025-hodunova-squad22-zov]. The more polished version reaches a global audience. *Ukrainian Warfare: Gostomel Heroes* (March 2026) opens with "the brilliant Russian airborne assault" on Hostomel and has about 2,000 mostly positive Steam reviews [src:steam-2026-gostomel-heroes]. Its developer calls its next project another game "on the topic of the Special Military Operation", and its free DLC adds a skirmish map of Bucha [src:steam-2026-gostomel-heroes-dlc]. The store title reads as Ukrainian, which shows how naming alone can launder a perspective. Roblox war games let children play either side [src:propastop-2023-war-not-a-game].
- **Realism turns into disinformation.** Arma 3 footage keeps being passed off as real Ukraine combat footage [src:bohemia-2023-arma3-fake-news].
- **Gamifying the real war.** UFDS's CEO calls selling a combat-trainer-as-game "a very sensitive question" [src:cbs-2026-clarke-ufds]. WOTR warns that ADB-style points invite Goodhart's-law distortions and blur civilian reporting into targeting [src:wotr-2026-almajdalani-gamified-war]. The March 2026 proposal to pay points when a cornered Russian soldier kills himself shows how a points schema is pulled into ever darker edge cases [src:euromaidanpress-2026-epoints-self-destruction]. A critic quoted on *Death From Above*'s store page calls its mix of politics and games uncomfortable [src:steam-2024-death-from-above].
- **Designers' humility.** Dockter and Herman say that "wiser men and women would avoid" designing an ongoing war, and that it is still worth doing as a first draft of history [src:insidegmt-2023-dockter-herman-defiance].

## 7. Design lessons: gaps in existing games that bavovniatko could fill

1. **The kill-zone commander is an empty niche.** RTSs mass armour and air [src:strategyandwargaming-2025-broken-arrow-review], and drone games put you in a single drone [src:steam-2025-ufds]. Nobody gives the company-to-brigade commander dispersed 2–3 person positions, drone crews, EW and a ~10 km no-vehicle belt [src:paxsims-2025-brynen-emergent-approaches][src:rusi-2025-watling-combined-arms]. Aim for that. The nearest attempt, *Hell Of War*, makes a command drone the main camera but is still a hero game with a fire-support budget [src:steam-2026-hell-of-war]. The Canadian games' lesson belongs at the core: the problem to solve is the enemy's sense-and-strike complex, not your own number of drones [src:canadianarmytoday-2026-thatcher-wargaming-modernization].
2. **Surveillance without capacity.** Radio-only fog games were faulted for reports that are too precise [src:vice-2019-zacny-radio-commander]. This war's fog is the opposite: you see more targets than you can hit [src:rusi-2025-watling-combined-arms]. Model detection as abundant and effectors, sorties and battery or fibre as scarce. Ukraine's own scoring now pays for deep detections (more than 15 km behind the line) separately from strikes, a ready-made split between sensing and killing [src:euromaidanpress-2026-delta-recon-ranking].
3. **Last-mile logistics as a core loop.** Most games abstract supply into radii [src:matrixforums-2022-toaw-ukraine-2022]. Make resupply by foot, quad, horse and UAV a player-managed and interdictable system [src:paxsims-2025-brynen-emergent-approaches].
4. **Model EW as signal physics, not a buff.** Use the Arma mod's LOS, occlusion, jamming and RSSI/LQ model as a template [src:armaplatform-2025-realistic-combat-drones], with Remote Reaper's split between control and video links and its terrain loss between ground station and drone [src:steam-2025-remote-reaper]. Tie it to era-gated countermeasures so the measure/countermeasure cycle is systemic. Avoid Broken Arrow's shortcut of treating drones as small helicopters for air defence to shoot at [src:steam-2026-broken-arrow-anniversary-update].
5. **Data-driven, era-patched equipment.** TOAW had to fake Javelins [src:matrixforums-2022-toaw-ukraine-2022]. Obriy updates its terrain as the front moves [src:ukrainesarmsmonitor-2026-obriy]. Bavovniatko should keep equipment and doctrine as per-era data (e1…e9), because no competitor covers e8/e9 at operational scale. Defiance stops at April 2022 [src:gmt-2025-defiance].
6. **A requisition economy with the ADB's lessons.** A points-for-effects economy is authentic, since the real one gets repriced like a live game [src:united24-2025-khomenko-adb-launch]. Weight it toward capture, evacuation and depth strikes the way the real schema does [src:mod-2026-adb-results][src:wotr-2026-almajdalani-gamified-war], and let Goodhart-style distortions surface as a mechanic, not a high score.
7. **A political layer that constrains operations.** Political objectives overriding military advice [src:mwi-2022-lacey-wargaming-long-war], Defiance's political tracks [src:gmt-2025-defiance] and Train's non-kinetic chits [src:hollandspiele-2017-train-ukrainian-crisis] all point to orders from above ("hold this town") that you cannot refuse.
8. **Built-in ethics.** Make the player Ukrainian only [src:mezha-2024-danylov-gtth-russian-faction]. Keep a stylised, non-photoreal drone feed with a visible game HUD so clips cannot pass as real footage [src:bohemia-2023-arma3-fake-news]. Have a civilian presence without gamifying civilian harm [src:steam-2022-ukraine-war-stories]. Show no kill-count leaderboards.
9. **A doctrinally rigid AI opponent.** Russian staff games rehearse procedure rather than adapt [src:paxsims-2026-brynen-how-russia-wargames]. That supports a scripted, wave-based assault AI that adapts slowly across eras.
10. **An operational wrapper around tactical fights.** Steel Division 2 moves battalions and supply on a turn-based 150×100 km map and resolves contacts as real-time battles or auto-resolve [src:steam-2019-steel-division-2]. Flashpoint Campaigns plays by orders and SOPs in WEGO turns rather than micro [src:steam-2023-flashpoint-southern-storm]. Either fits a brigade commander who sets intent and lets dispersed crews execute.

## 8. Game/sim relevance

- The strongest mechanical references are the Arma drone mod (signal and EW model), UFDS and Obriy (drone roles, crews, interceptors, terrain from real front) and RUSI/Watling (battlefield geometry and the offensive phase cycle) [src:armaplatform-2025-realistic-combat-drones][src:ukrainesarmsmonitor-2026-obriy][src:rusi-2025-watling-combined-arms].
- For UI and fog of war, Radio Commander's player-maintained map is a good template for a DeepStateMap-style overview that is *your* belief state, not ground truth. Add latency and noise in reports [src:vice-2019-zacny-radio-commander].
- Plan per-era content. Invasion-phase models (Defiance, TOAW) can calibrate e1, and nobody covers e8/e9 [src:gmt-2025-defiance][src:matrixforums-2022-toaw-ukraine-2022].
- The ADB point schema is a ready-made, sourced table of relative target values for an in-game requisition system [src:united24-2025-khomenko-adb-launch][src:mod-2026-adb-results]. Its dated values (§3) can be era data: e8 prices from 2025 and e9 prices from 2026, when captures, deep reconnaissance and evacuation are also scored [src:kyivpost-2026-korshak-epoints][src:euromaidanpress-2026-epoints-400-units].
- For a Ukraine-only game, *Gostomel Heroes* is the closest commercial competitor for e1 and the clearest counter-example. It has the same map (Hostomel, Bucha) from the other side, and players want that level of simulation (modular damage, morale, supply) [src:steam-2026-gostomel-heroes].
- Ethics guardrails are prior-art-driven: Ukraine-only POV, no photoreal "footage", and no kill leaderboards [src:mezha-2024-danylov-gtth-russian-faction][src:bohemia-2023-arma3-fake-news][src:wotr-2026-almajdalani-gamified-war].

## 9. Terms

- FPV — first-person-view drone, flown via goggles; usually a one-way strike drone.
- Dropper / bomber — a multirotor that drops munitions (e.g., Vampire, Baba Yaga) rather than diving in.
- Loitering munition — a drone munition that searches before striking (e.g., Lancet, Shahed-type).
- Shahed — Iranian-designed long-range one-way attack drone used by Russia; the target of interceptor sims.
- EW — electronic warfare: jamming, spoofing, direction finding.
- RSSI/LQ — received signal strength / link quality; the drone-link health measures used in the Arma mod.
- Kill zone — the ~10+ km band near the front where drones make vehicle movement lethal.
- ISR — intelligence, surveillance, reconnaissance.
- ADB / ePoints — Army of Drones Bonus, a points-for-verified-strikes programme.
- DELTA — Ukraine's situational-awareness system; strike videos are verified there.
- Brave1 Market — Ukrainian defence marketplace where units spend ePoints.
- WeGo — simultaneous-turn system (Combat Mission) where both sides plot, then execute together.
- OOB — order of battle.
- CPX — command post exercise; staff training driven by a simulation.
- Matrix game — argument-based adjudicated wargame, common in professional gaming.
- KShVI — Russian command-staff military games (procedural rehearsal).
- P500 — GMT's preorder system; a game goes to print after 500 orders.
- PvPvE — players fight each other and AI at the same time.
- Double-blind — neither side sees the other's forces except through reports.
- Goodhart's law — when a measure becomes a target, it stops measuring what matters.
- Sense-and-strike complex (recon-strike complex) — the linked sensors, C2 and fires that find and hit targets within minutes; the thing Canadian games found hardest to break.
- Loitering ammunition — Broken Arrow's term for its planned one-way attack munitions (missiles with man-in-the-loop or AI terminal guidance).
- Chaika / Chaika-M ("Seagull" / "Flyswatter") — Ukrainian VR air-defence simulators codified by the MoD in 2025.
- RTCA — "real-time command action", Hell Of War's label for a command-drone view with drop-in direct control.
- SOP — standard operating procedure; in Flashpoint Campaigns, standing rules that units follow between orders.

## 10. Open questions/gaps

- **CSIS wargames on Russia–Ukraine or NATO–Russia**: not found. csis.org returns 403. The archived Futures Lab page lists Ukraine analysis (an energy-truce piece) but no Russia–Ukraine wargame, and PAXsims only references CSIS's Taiwan games. The NATO–Russia games that were found are German, British and Canadian (§5).
- **Dstl on Ukraine drone lessons**: the gov.uk search API found only the Dstl–Slitherine case study, the Defence Wargaming Centre pages and a strategic-communications game (Defending Defender). No Ukraine drone game was public.
- **Commercial FPV sims in Ukrainian schools**: no first-party source names Liftoff, VelociDrone, Uncrashed or DCL. Searches of Euromaidan Press's archive for each name found nothing. Only the vendor claim [src:vgi9-2025-drone-simulators] remains, and the verified military sims are all domestic.
- **Combat Mission: Black Sea community mods** for FPV or 2022 content: the Battlefront forums returned 403. Official content was checked via the Steam news API, and none has shipped [src:steam-2023-cmbs-update-2-18].
- **Developer countries still unverified**: Sand Table Software (the store page and the studio site give none), Bird's Eye Games (Regiments), Cats Who Play (Gostomel Heroes), Ravens Row (Hell Of War) and Battlestate Games (Escape from Tarkov). Death From Above's makers are confirmed German.
- **ADB values before April 2025** beyond the infantry price, and the exact date of the 6→12 repricing, which falls between April 2025 and January 2026.
- Why *Gostomel Heroes* is missing from the Ukrainian and German Steam stores (regional withdrawal, rating or developer choice) is not established.
- The *Play for Ukraine* full text (Taylor & Francis) returns 403. Only its abstract and metadata were read.

## 11. Sources

- wikipedia-2025-broken-arrow — Broken Arrow facts; Russia-based developer; no drone unit class.
- strategyandwargaming-2025-broken-arrow-review — enthusiast review; massed modern combined arms.
- wikipedia-2024-warno — Eugen's 1989 division-scale RTS; genre baseline.
- wikipedia-2014-combat-mission-black-sea — fictional 2017 war; UAV/EW; Battlefront's 2014 timeline note.
- matrixgames-2022-cmo-ukraine-scenarios — CMO Ukraine-2022 scenarios are NATO what-ifs.
- matrixforums-2022-toaw-ukraine-2022 — TOAW IV invasion scenario devlog; OOB, supply, C2.
- steam-2024-ukraine-war-2022 — Sand Table operational sim, 2022–23 scenarios.
- vice-2019-zacny-radio-commander — radio-only fog of war and its critique.
- saveorquit-2020-radio-general — voice-command double-blind WWII game.
- wikipedia-2020-squad — Squad factions (RU/AFU), commander UAV.
- armaplatform-2025-realistic-combat-drones — Reforger drone/EW signal-model mod.
- bohemia-2023-arma3-fake-news — game footage used as fake war footage.
- steam-2026-glory-to-the-heroes — Ukrainian milsim: Krynky, Chasiv Yar, Izium; FPV.
- mezha-2024-danylov-gtth-russian-faction — Ukrainian backlash to the Russian faction.
- steam-2022-ukraine-war-stories — Kyiv studio's civilian visual novels.
- steam-2024-death-from-above — arcade AFU drone game; charity model.
- kyivindependent-2025-hodunova-squad22-zov — Russian MoD-backed propaganda RTT.
- steam-2025-ufds — Ukrainian FPV trainer's consumer edition.
- cbs-2026-clarke-ufds — UFDS background, 5,000 pilots, ethics quote.
- ukrainesarmsmonitor-2026-obriy — Obriy military trainer, anti-Shahed, real terrain.
- rbcukraine-2026-kovalenko-gta-shahed — GTA V/FiveM Shahed-hunt server.
- vgi9-2025-drone-simulators — which commercial and UA sims schools use (promotional).
- united24-2025-khomenko-adb-launch — ADB launch, point values, Brave1 prices.
- mod-2026-adb-results — official 2025 ADB totals and 2026 expansions.
- wotr-2026-almajdalani-gamified-war — ethics of ADB gamification.
- gmt-2025-defiance — GMT board game of Feb–Apr 2022.
- insidegmt-2023-dockter-herman-defiance — Defiance designer diary.
- hollandspiele-2017-train-ukrainian-crisis — 2014 hybrid-war board game.
- paxsims-2023-farren-glory-to-the-heroes — grand-strategic print-and-play.
- bgg-2022-ukraine — BGG listing plus other listed titles (search-only).
- wotr-2022-lacey-wargame-before-the-war — MCU pre-invasion wargame.
- mwi-2022-lacey-wargaming-long-war — MCU long-war game; stalemate forecast.
- rand-2016-shlapak-baltics — RAND Baltic wargame (search-only).
- paxsims-2023-brynen-wargaming-doesnt-work — 2023 counteroffensive tabletop critique.
- rusi-2025-watling-combined-arms — how Ukraine fights in 2025.
- paxsims-2025-brynen-emergent-approaches — commercial wargames miss these realities.
- paxsims-2026-brynen-how-russia-wargames — Russian KShVI limits.
- paxsims-2026-taylor-gpt-negotiation-game — AI-assisted wargame design.
- nextgov-2024-breeden-vbs4-ukraine — VBS4 Ukrainian terrain; drones in pro sims.
- llnl-2023-lasch-simulation-support-uaf — Steel Beasts/JCATS/VBS3 for UA staff training.
- defender-2025-pokotylo-steel-beasts — National Guard adopts localised Steel Beasts.
- ehgs-2026-kot-ukrainian-games-of-war — academic survey of 200+ Ukrainian war games.
- propastop-2023-war-not-a-game — Roblox war games and propaganda risks.
- steam-2025-broken-arrow-roadmap — 2026 roadmap: loitering munitions planned.
- steam-2026-broken-arrow-march-qa — developers define "loitering ammunition"; no FPV or Shahed units.
- steam-2026-broken-arrow-anniversary-update — drones reclassed as helicopter targets; HP cut.
- steam-2023-cmbs-update-2-18 — last official CM: Black Sea update (PBEM++ only).
- steam-2022-regiments — Regiments: 1989 RTT, platoon command, operations.
- steam-2023-flashpoint-southern-storm — Flashpoint: WEGO, SOPs, EW, 500 m hexes; Cold War successor.
- steam-2019-steel-division-2 — turn-based operational campaign around RTS battles.
- wikipedia-2024-men-of-war-ii — Best Way (Ukraine); invasion delayed release.
- steam-2021-gates-of-hell-ostfront — Barbedwire Studios, Sagunt (Spain).
- steam-2024-gray-zone-warfare — Madfinger; fictional SE Asian setting.
- wikipedia-2025-escape-from-tarkov — fictional Norvinsk PMC war.
- wikipedia-2025-gsc-game-world — GSC wartime relocation to Prague.
- euromaidanpress-2025-stalker2-gsc-undesirable — Russia bans GSC; $17m to troops; Yezhov killed.
- wikipedia-2025-frogwares — Frogwares during the invasion.
- wikipedia-2025-4a-games — 4A Games' move to Malta (2014).
- steam-2026-gostomel-heroes — Russian-framed RTS of Hostomel 2022; regional availability.
- steam-2026-gostomel-heroes-dlc — "Special Military Operation"; Bucha map.
- steam-2026-hell-of-war — command-drone RTS/FPS hybrid (EA 2026).
- steam-2026-death-from-above-germany — Death From Above's makers are German; donations.
- steam-2025-remote-reaper — Ukrainian FPV sim: link jamming, warheads.
- euromaidanpress-2026-ufds-interceptors — interceptors added to UFDS Military.
- euromaidanpress-2025-seagull-flyswatter — MoD-codified VR air-defence sims.
- kyivpost-2026-korshak-epoints — January 2026 e-point price list.
- euromaidanpress-2026-epoints-self-destruction — 12-point kill; proposed self-destruction points.
- euromaidanpress-2026-delta-recon-ranking — points for detections more than 15 km deep.
- euromaidanpress-2026-epoints-400-units — scope widened to recon, logistics, evacuation.
- govuk-2024-dstl-slitherine-gaming-tech — Dstl wargames on commercial engines.
- guardian-2026-sabbagh-arrcade-strike — ARRC exercise; 80–90% drone shortfall.
- paxsims-2026-hybrid-warfare-nato-cohesion — Die Welt/HSU game; drone-laid mines.
- canadianarmytoday-2026-thatcher-wargaming-modernization — the sense-strike complex lesson.
- mwi-2025-hood-matisek-tingle-learn-or-lose — Cold War scenarios in Ukrainian staff training.
- wotr-2025-votel-military-gaming — digital games for training; Russian drone-sim codification.
- tandf-2024-usachova-play-for-ukraine — DDoS game as "resistance pleasure".
- obraz-2024-zinovieva-deep-media — Ukrainian view of war games as deep media.
