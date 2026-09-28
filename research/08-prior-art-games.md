# 08 — Prior-art games and wargames

Research for **bavovniatko** (Bevy wargame of the Russo-Ukrainian war, Ukrainian side only). Accessed 2026-09-28. Every claim is cited inline with its source id; the source list is in `sources/08-prior-art-games.yaml`.

## 1. Overview

Nobody has built what bavovniatko is aiming for. The prior art falls into five groups, and each one captures a different piece of the war.

- **Modern RTS and tactical games** (Broken Arrow, WARNO, Combat Mission: Black Sea) model massed combined arms between NATO-style and Russian forces. Broken Arrow's unit classes are infantry, recon, vehicles, support, helicopters and air, with no drone class [src:wikipedia-2025-broken-arrow]. WARNO is set in 1989 [src:wikipedia-2024-warno]. CM: Black Sea is a *fictional 2017* Russia–Ukraine war. It has UAVs and EW, but it was designed in 2009–2013 and deliberately avoids the "frozen conflict" that followed 2014 [src:wikipedia-2014-combat-mission-black-sea].
- **Operational and strategic sims** of the real war exist mostly as community scenarios or small indie titles. Examples are a TOAW IV "Ukraine 2022" scenario [src:matrixforums-2022-toaw-ukraine-2022] and Sand Table's *Ukraine War 2022* [src:steam-2024-ukraine-war-2022]. Command: Modern Operations' Ukraine-2022 scenarios are mostly *hypothetical NATO interventions* [src:matrixgames-2022-cmo-ukraine-scenarios].
- **Drone games and trainers**, many of them Ukrainian, are first-person and cover one drone at a time. Examples are UFDS [src:steam-2025-ufds], Obriy [src:ukrainesarmsmonitor-2026-obriy] and the arcade *Death From Above* [src:steam-2024-death-from-above]. They model piloting, bombs and sometimes EW, but never the formation or operational picture.
- **Board and professional wargames** are the most thoughtful about logistics, politics and attrition, but they cluster at the start of the war or at grand-strategic scale. GMT's *Defiance* covers Feb–Apr 2022 [src:gmt-2025-defiance], and Farren's *Glory to the Heroes* uses 150 km hexes [src:paxsims-2023-farren-glory-to-the-heroes]. Expert reviewers say that "relatively few of the existing commercial wargames" capture the dispersed, drone-saturated battlefield of 2025 [src:paxsims-2025-brynen-emergent-approaches].
- **Real-war gamification** may be the most important "prior art" of all. The Army of Drones Bonus (ADB) points economy is a live game system run by the Ukrainian state [src:united24-2025-khomenko-adb-launch][src:mod-2026-adb-results], and it is ethically contested [src:wotr-2026-almajdalani-gamified-war].

## 2. Games catalog

| title | developer (country) | year | genre/scale | depicts this war? | models well | misses | source id |
|---|---|---|---|---|---|---|---|
| Broken Arrow | Steel Balalaika (Russia-based); pub. Slitherine | 2025 | large-scale RTS, up to 5v5 | No: US vs Russia | unit/loadout customisation, APS, modern air/AD, infantry recon drones as kit | drones are an accessory; massed armour and air; no FPV, EW or logistics on the store page (a roadmap for loitering munitions is unverified) | [src:wikipedia-2025-broken-arrow] [src:strategyandwargaming-2025-broken-arrow-review] |
| WARNO | Eugen Systems (France) | EA 2022 / 1.0 2024 | division-scale RTS + turn-based campaign, 10v10 | No: 1989 | historical OOBs, income-based deployment | pre-drone era by design | [src:wikipedia-2024-warno] |
| Combat Mission: Black Sea | Battlefront.com | 2014 (Steam 2021) | battalion WeGo/RT tactics | Partly: fictional 2017 war with US | UAV recon/strike, EW conditions, APS, PGMs, thermal optics | predates 2022; no FPV swarms or kill-zone logistics; deliberately avoids the frozen-conflict phase | [src:wikipedia-2014-combat-mission-black-sea] |
| Command: Modern Operations (Ukraine 2022 scenarios) | community authors; pub. Matrix/Slitherine | 2022 | air/naval/operational sim | Hypothetical: NATO convoy/NFZ what-ifs | sensor/platform database | the ground war as fought | [src:matrixgames-2022-cmo-ukraine-scenarios] |
| TOAW IV "Ukraine 2022" scenario | sPzAbt653, Josant (community) | 2022 | operational hex, battalion–brigade | Yes (e1) | 222 UA / 136 RU unit OOB, supply radius, low Russian proficiency for poor C2 | the engine forces Javelins into "rockets & missiles"; no drone layer described | [src:matrixforums-2022-toaw-ukraine-2022] |
| Ukraine War 2022 | Sand Table Software (country not stated) | 2024 | real-time operational, maps up to 2000 km | Yes: 20 scenarios 2022–23 (Kyiv, Kherson, Bakhmut) + what-ifs | theatre scale, air and airborne ops | drones and logistics not described; almost no reviews | [src:steam-2024-ukraine-war-2022] |
| Radio Commander | Serious Sim | 2019 | radio-only command, Vietnam | No | "you only know what they know"; the player updates the map | reports too precise, so the fog feels "theatrical" | [src:vice-2019-zacny-radio-commander] |
| Radio General | Foolish Mortals (Canada) | 2020 | voice-command WWII | No | double-blind map, vague status reports | a gimmick; weak AI; voice recognition fights efficiency | [src:saveorquit-2020-radio-general] |
| Squad | Offworld Industries (Canada) | 2015 EA / 2020 | 50v50 milsim FPS | Factions only (RU and AFU) | commander role calls UAV recon and artillery | not the war; drones minimal | [src:wikipedia-2020-squad] |
| Arma Reforger "Realistic Combat Drones" mod | SalamiBoiDev on Bohemia's platform | 2025–26 | milsim mod | War-inspired | signal model (LOS, jamming, terrain occlusion, RSSI/LQ), video degradation, backpack jammers, battery | no fiber-optic; squad scale only | [src:armaplatform-2025-realistic-combat-drones] |
| Arma 3 | Bohemia Interactive (Czechia) | 2013 | milsim | No, but its clips circulate as fake Ukraine footage | visual realism | realism becomes a disinformation vector | [src:bohemia-2023-arma3-fake-news] |
| Glory to the Heroes | Deaf Tone Games (Ukraine) | EA; 1.0 planned 2026 | PvPvE milsim, up to 425 km² ops | Yes: Krynky, Chasiv Yar canal, Izium plant | trench combat, massed artillery, UAVs incl. FPV, vehicles with crew positions | Russian-faction PvP drew Ukrainian backlash | [src:steam-2026-glory-to-the-heroes] [src:mezha-2024-danylov-gtth-russian-faction] |
| Ukraine War Stories | Starni Games (Kyiv, Ukraine) | 2022 | visual novel, free | Yes (e1): civilians in Hostomel, Bucha, Mariupol | eyewitness-based civilian experience | no military layer, by intent | [src:steam-2022-ukraine-war-stories] |
| Death From Above | Rockodile / Lesser Evil (country not verified) | 2023 EA / 2024 | arcade drone action | Yes: AFU drone operator | three roles (ground operator, quad bomber, FPV); demining, convoys | explicitly not a sim; short | [src:steam-2024-death-from-above] |
| Squad 22: ZOV | SPN Studio (Russia) | 2025 | RTT, Russian propaganda | Yes, from the aggressor's view (2014, Mariupol, Avdiivka) | none worth copying | state-backed propaganda; blocked in Ukraine | [src:kyivindependent-2025-hodunova-squad22-zov] |
| UFDS | Simtech Solutions / Drone Fight Club Academy (Ukraine) | 2025 | FPV trainer + light wave/resource mode | Yes | real UA drones, physics, bombs, PID tuning, RC input, interceptor course | Steam edition stripped of tactics | [src:steam-2025-ufds] [src:cbs-2026-clarke-ufds] |
| Obriy | Twist Robotics (Ukraine) | 2024–26 | military drone trainer | Yes: 50,000+ km² of real front terrain | EW scenarios, navigator/crew roles, anti-Shahed interceptors, terrain updated with the front | military-only | [src:ukrainesarmsmonitor-2026-obriy] |
| WeTrueGun GTA V/FiveM server | WeTrueGun (Ukraine) | 2026 | modded sandbox | Shahed interception | skill upkeep, decompression | not real training (by its makers' account) | [src:rbcukraine-2026-kovalenko-gta-shahed] |
| Steel Beasts Pro / PE | eSim Games | UA use 2022–25 | armour sim, crew → brigade CPX | Used by the UA military | ballistics, engineering, logistics; cheap staff exercises | PE limited to 8 workstations; drones via VBS3 integration | [src:llnl-2023-lasch-simulation-support-uaf] [src:defender-2025-pokotylo-steel-beasts] |
| VBS4 + Ukrainian terrain | Bohemia Interactive Simulations | 2024 | tactical / mission rehearsal | Ukrainian terrain | drones, counter-drone, loitering munitions, trenches, AI entities | brigade ops, logistics and morale (vendor's own admission) | [src:nextgov-2024-breeden-vbs4-ukraine] |

## 3. Drone games & training simulators

- **UFDS** is the key crossover. Drone Fight Club Academy trained 5,000+ Ukrainian military pilots on it and sells a de-tacticised Steam edition for about $30. The military gets the full version free [src:cbs-2026-clarke-ufds]. The Steam build has Academy courses (kamikaze, bomber, air interceptor, racing), a Battleground wave mode with resource management, and real drones (Angel Arrow, LuckyStrike, Mimic 3T, Phantom, Dzhmil) [src:steam-2025-ufds].
- **Obriy** (Twist Robotics) is the military-grade counterpart. It trains pilot and navigator crews and mission planning, has EW scenarios and an anti-Shahed interceptor version (March 2026), and runs on 50,000+ km² of satellite-derived front terrain that is updated as the line moves. It is used by 150+ units and 50+ training centres [src:ukrainesarmsmonitor-2026-obriy].
- **Commercial FPV sims** are part of the pipeline. A Ukrainian vendor calls Liftoff the most popular sim among beginners and instructors, with VelociDrone for racing and DRL for beginners. It lists Ukrainian military sims (FPV Battleground by BAZU, UFDS, Obriy) and RealFlight, Unigine Sim and XFly in UAV programmes [src:vgi9-2025-drone-simulators]. This is promotional, and Uncrashed and DCL usage is unverified.
- **Game engines get repurposed.** The WeTrueGun school runs a GTA V/FiveM server for Shahed hunts. Its makers say real interceptor training uses Obriy [src:rbcukraine-2026-kovalenko-gta-shahed].
- **Arcade and commercial titles.** *Death From Above* is an arcade take on the AFU drone operator (ground, quad and FPV roles) that donates to Come Back Alive and Army of Drones [src:steam-2024-death-from-above]. The Arma Reforger drone mods have the most transferable EW model among consumer games [src:armaplatform-2025-realistic-combat-drones].
- **Army of Drones Bonus: the war's own game layer.** Units upload verified strike video to DELTA, earn ePoints and spend them on the Brave1 Market (a Vampire bomber cost 43 points). Infantry kills were repriced from 2 to 6 points, which reportedly doubled confirmed eliminations within a month [src:united24-2025-khomenko-adb-launch]. The 2025 total was 819,737 video-confirmed strikes. Planned 2026 additions are points for air defence and aviation against drones, for snipers, and "depth coefficients" for strikes on logistics and UAV crews [src:mod-2026-adb-results]. Capturing an enemy and evacuating wounded score higher than killing [src:wotr-2026-almajdalani-gamified-war].

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
- **Ukrainian military simulation.** From Dec 2022 to May 2023, volunteers used Steel Beasts PE to run brigade and battalion CPXs for newly generated formations. They integrated JCATS, SBPro and VBS3, with VBS3 drones feeding commanders' picture [src:llnl-2023-lasch-simulation-support-uaf]. By 2025 the National Guard had adopted a Ukrainian-localised Steel Beasts Pro [src:defender-2025-pokotylo-steel-beasts]. VBS4 now ships Ukrainian terrain with drones and loitering munitions, and its vendor concedes that brigade ops, logistics and morale were weak points [src:nextgov-2024-breeden-vbs4-ukraine].
- **The Russian side** runs command-staff games (KShVI) that rehearse procedure, including EW and counter-UAV, but do not test assumptions. That left planners blind to drone saturation and ISR transparency [src:paxsims-2026-brynen-how-russia-wargames].
- **AI in wargaming.** A September 2026 GPT-assisted negotiation game was drafted in about two hours. Its designer insists that humans choose the abstractions and that AI output needs verification and playtesting [src:paxsims-2026-taylor-gpt-negotiation-game].

## 6. Critical perspectives & ethics (including Ukrainian views)

- **Ukrainian creators are the majority voice.** A 2026 academic survey counts 200+ Ukrainian-made games responding to the invasion (2022–2025). It frames them as documentation, emotional processing and cultural response [src:ehgs-2026-kot-ukrainian-games-of-war]. *Ukraine War Stories* shows the documentary and civilian approach [src:steam-2022-ukraine-war-stories].
- **Playing as Russians is a red line for many Ukrainians.** When Deaf Tone brought back Russian-faction PvP in *Glory to the Heroes*, Ukrainian press and players pushed back [src:mezha-2024-danylov-gtth-russian-faction]. This supports bavovniatko's Ukraine-only player stance.
- **Propaganda is the mirror-image pitfall.** *Squad 22: ZOV* was made with Russian MoD backing, frames the invasion as "liberation", and is marketed for cadet and Yunarmy training [src:kyivindependent-2025-hodunova-squad22-zov]. Roblox war games let children play either side [src:propastop-2023-war-not-a-game].
- **Realism turns into disinformation.** Arma 3 footage keeps being passed off as real Ukraine combat footage [src:bohemia-2023-arma3-fake-news].
- **Gamifying the real war.** UFDS's CEO calls selling a combat-trainer-as-game "a very sensitive question" [src:cbs-2026-clarke-ufds]. WOTR warns that ADB-style points invite Goodhart's-law distortions and blur civilian reporting into targeting [src:wotr-2026-almajdalani-gamified-war]. A critic quoted on *Death From Above*'s store page calls its mix of politics and games uncomfortable [src:steam-2024-death-from-above].
- **Designers' humility.** Dockter and Herman say that "wiser men and women would avoid" designing an ongoing war, and that it is still worth doing as a first draft of history [src:insidegmt-2023-dockter-herman-defiance].

## 7. Design lessons: gaps in existing games that bavovniatko could fill

1. **The kill-zone commander is an empty niche.** RTSs mass armour and air [src:strategyandwargaming-2025-broken-arrow-review], and drone games put you in a single drone [src:steam-2025-ufds]. Nobody gives the company-to-brigade commander dispersed 2–3 person positions, drone crews, EW and a ~10 km no-vehicle belt [src:paxsims-2025-brynen-emergent-approaches][src:rusi-2025-watling-combined-arms]. Aim for that.
2. **Surveillance without capacity.** Radio-only fog games were faulted for reports that are too precise [src:vice-2019-zacny-radio-commander]. This war's fog is the opposite: you see more targets than you can hit [src:rusi-2025-watling-combined-arms]. Model detection as abundant and effectors, sorties and battery or fibre as scarce.
3. **Last-mile logistics as a core loop.** Most games abstract supply into radii [src:matrixforums-2022-toaw-ukraine-2022]. Make resupply by foot, quad, horse and UAV a player-managed and interdictable system [src:paxsims-2025-brynen-emergent-approaches].
4. **Model EW as signal physics, not a buff.** Use the Arma mod's LOS, occlusion, jamming and RSSI/LQ model as a template [src:armaplatform-2025-realistic-combat-drones]. Tie it to era-gated countermeasures so the measure/countermeasure cycle is systemic.
5. **Data-driven, era-patched equipment.** TOAW had to fake Javelins [src:matrixforums-2022-toaw-ukraine-2022]. Obriy updates its terrain as the front moves [src:ukrainesarmsmonitor-2026-obriy]. Bavovniatko should keep equipment and doctrine as per-era data (e1…e9), because no competitor covers e8/e9 at operational scale. Defiance stops at April 2022 [src:gmt-2025-defiance].
6. **A requisition economy with the ADB's lessons.** A points-for-effects economy is authentic, since the real one gets repriced like a live game [src:united24-2025-khomenko-adb-launch]. Weight it toward capture, evacuation and depth strikes the way the real schema does [src:mod-2026-adb-results][src:wotr-2026-almajdalani-gamified-war], and let Goodhart-style distortions surface as a mechanic, not a high score.
7. **A political layer that constrains operations.** Political objectives overriding military advice [src:mwi-2022-lacey-wargaming-long-war], Defiance's political tracks [src:gmt-2025-defiance] and Train's non-kinetic chits [src:hollandspiele-2017-train-ukrainian-crisis] all point to orders from above ("hold this town") that you cannot refuse.
8. **Built-in ethics.** Make the player Ukrainian only [src:mezha-2024-danylov-gtth-russian-faction]. Keep a stylised, non-photoreal drone feed with a visible game HUD so clips cannot pass as real footage [src:bohemia-2023-arma3-fake-news]. Have a civilian presence without gamifying civilian harm [src:steam-2022-ukraine-war-stories]. Show no kill-count leaderboards.
9. **A doctrinally rigid AI opponent.** Russian staff games rehearse procedure rather than adapt [src:paxsims-2026-brynen-how-russia-wargames]. That supports a scripted, wave-based assault AI that adapts slowly across eras.

## 8. Game/sim relevance

- The strongest mechanical references are the Arma drone mod (signal and EW model), UFDS and Obriy (drone roles, crews, interceptors, terrain from real front) and RUSI/Watling (battlefield geometry and the offensive phase cycle) [src:armaplatform-2025-realistic-combat-drones][src:ukrainesarmsmonitor-2026-obriy][src:rusi-2025-watling-combined-arms].
- For UI and fog of war, Radio Commander's player-maintained map is a good template for a DeepStateMap-style overview that is *your* belief state, not ground truth. Add latency and noise in reports [src:vice-2019-zacny-radio-commander].
- Plan per-era content. Invasion-phase models (Defiance, TOAW) can calibrate e1, and nobody covers e8/e9 [src:gmt-2025-defiance][src:matrixforums-2022-toaw-ukraine-2022].
- The ADB point schema is a ready-made, sourced table of relative target values for an in-game requisition system [src:united24-2025-khomenko-adb-launch][src:mod-2026-adb-results].
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

## 10. Open questions/gaps

- **Not verified this pass** because the search budget ran out. Leads not confirmed:
  - Regiments (Bird's Eye Games / MicroProse)
  - Flashpoint Campaigns
  - Steel Division
  - Men of War / Gates of Hell (whether the Men of War developer is Ukrainian is not confirmed)
  - Gray Zone Warfare
  - Escape from Tarkov (setting)
  - 4A Games and Frogwares wartime context
  - S.T.A.L.K.E.R. 2 / GSC context: the PC Gamer and GamesBeat pages did not load
  - CSIS and UK Dstl wargames
  - US Army drone-wargaming lessons
- Broken Arrow's roadmap for FPV and Shahed-style munitions appeared only in a search snippet. Check the devlogs.
- Whether Combat Mission: Black Sea ever received FPV or 2022-era content is unknown. Forum threads suggest community mods only.
- Uncrashed, DCL and Brave1-backed sims: actual use in Ukrainian drone schools needs first-party confirmation (the VGI-9 claim is promotional).
- ADB point values drift (2→6 in 2025; later reports cite higher values that I did not fetch). A time series is needed for per-era modelling.
- Developer countries unverified: Rockodile/Lesser Evil (Death From Above), Sand Table Software.
- The "Play for Ukraine: wargaming as a resistance pleasure" article (Taylor & Francis, 2024) returned 403 and was not read.

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
