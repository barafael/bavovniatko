<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# 13 — Shooting down long-range drones: effectors and their cueing, 2022–2026

Scope: the Ukrainian means that kill or divert Russian long-range drones over the rear and the cities: Shahed-136/Geran-2, Gerbera and Parodiya decoys, the jet Geran-3/4/5, and reconnaissance drones on the same routes. It covers mobile fire groups, anti-aircraft guns, MANPADS, helicopters and fighters, interceptor drones, lasers and electronic warfare (EW) against navigation, and for each the cue it needs, its reach, its cost and how it fails. Ballistic and cruise-missile defence appears only where it competes for the same means. Detection and fusion (radars, acoustic sensors, ePPO) are a sibling draft (air-sensing) and appear here only as the cue an effector receives.

Accessed 2026-11-18. English-language sources, Ukrainian outlets paired with Western analysis. Russian capabilities appear only as Ukrainian or Western outlets report them. **Reported** means a figure a named party states (a count, a spec, a measured rate); **estimate** means a figure the source itself presents as approximate or derived; **computed** means arithmetic done here on a source's figures. Kill counts and shares from the Air Force, the General Staff, the Unmanned Systems Forces (USF) or a manufacturer are official or maker claims: Ukraine does not publish a per-effector breakdown, and the Air Force's "downed/suppressed" total mixes kills with EW diversions [src:isis-2026-anokhin-shahed-monthly]. Much of the 2025–2026 figure base for interceptors, guns and EW is already in the knowledge base (files 01–03, chapter c18); this draft reuses those claims by source id and adds the effector-level detail: crews, weapons, cueing, ranges, rounds and failure modes.

---

## 1. Summary

- **Four eras of the cheap layer.** In autumn 2022 the answer to the first Shaheds was improvised: small teams with machine guns, MANPADS, thermal sights and laser pointers, cued by a central air picture, tablets and residents' Telegram reports [src:stripes-2022-wapo-drone-hunters][src:twz-2022-trevithick-drone-hunters][src:defenceexpress-2022-mfg-tactics]. Through 2023 and the first half of 2024 mobile fire groups (MFGs) became the main Shahed killer: the Air Force commander said they made more than 80% of Shahed kills in the first half of 2024 [src:onlineua-2024-oleshchuk-mfg-80pct]. From 2025 Shaheds flew above 2 km, out of machine-gun reach [src:kyivindependent-2025-ihnat-high-altitude], and interceptor drones took over, from about 10–25% of kills in winter 2025–26 to about 40–50% in spring 2026 [src:kyivpost-2026-syrsky-may-interceptors][src:reuters-2026-hunder-defeat-shahed]. From mid-2026 the jet Gerans outran both the guns and the propeller interceptors [src:pravda-2026-ihnat-jet-drones].
- **Every effector depends on the cue.** In 2022 MFG crews got about 10 minutes' warning to reach position [src:stripes-2022-wapo-drone-hunters]. By 2026 a Frontliner report found crews often warned only once they could already hear the drone, too late to engage [src:frontliner-2026-derkachov-mfg-modernise]. Interceptor crews describe a few minutes between a Shahed appearing on radar and leaving range [src:reuters-2026-hunder-defeat-shahed]. A Merops crew says a calibrated radar with a good operator gives the best data, that flight-path programmes can be off by about a kilometre, and that the earlier interceptor's camera had to get within about 200 m before it could find the target [src:lb-2026-amelina-merops-crews].
- **Guns: cheap per kill and short-ranged.** A Gepard kills a Shahed with "a handful" of 35 mm rounds at about $600 each, so a few thousand dollars a kill [src:euromaidan-2025-axe-gepard-math][src:dronewarfare-2026-cuas-costs]. Skynex fires about 10 programmable AHEAD rounds per target out to about 4 km [src:espreso-2025-skynex-rounds][src:kyivindependent-2024-york-skynex]. Both are point defences that Shaheds above 2–3 km can overfly. A ZU-23-2 gunner put his own hit rate at "every hundredth shot" [src:gwara-2025-kulish-mfg-zu23].
- **Interceptor drones: the cheapest kill, but limited by weather, crews and speed.** Sting costs about $2,000 and flies at about 280–340 km/h depending on the source. An operator flies it onto a radar dot over a video link, and from 2026 onboard AI makes the terminal lock [src:reuters-2026-peleschuk-interceptors-gulf][src:migflug-2026-grunwald-interceptors][src:dronexl-2026-kesteloo-skyfall-autonomy]. Fog and cloud defeat the cameras: one crew logged ten launches in a night and found no Shahed [src:reuters-2026-hunder-defeat-shahed]. Russia answered with rear-view cameras and evasive manoeuvres [src:militarnyi-2025-kalchenko-shahed-rear-cameras], and then with jets.
- **EW diverts or kills about half on some nights, and it spills over.** On 4 July 2025, 208 of 539 drones and decoys were "diverted", most likely by jamming [src:euromaidan-2025-axe-gepard-math]. Reuters says EW can neutralise nearly half an attack on some nights [src:reuters-2026-hunder-defeat-shahed]. Russia climbed the CRPA ladder from 8 to 16 elements on Shaheds by March 2025 [src:bi-2025-loh-shahed-crpa-16]. Ukrainian GNSS spoofing moved civilians' phones into Russian time zones [src:therecord-2024-antoniuk-spoofing-phones].
- **Lasers are claimed, not yet verified against Shaheds.** Tryzub was shown in April 2025 and again in May 2026 with claimed ranges of 3 km against drones and 5 km against aircraft, aimed by hand with a joystick [src:euromaidan-2025-tryzub-frontline-report][src:razomua-2026-tryzub-manual]. In July 2026 the Deputy Commander-in-Chief said it kills FPVs and would be ready against Shaheds "in two to three months" [src:euromaidan-2026-mukhina-lebedenko-laser]. No documented Shahed kill by a laser was found, and no HPM or EMP system against Shaheds was found at all.

---

## 2. Mobile fire groups

### 2.1 Composition and weapons, by era

| date / era | composition and vehicle | weapons, sights | source |
|---|---|---|---|
| Oct 2022 (e3) | 4–5 people, usually 2 MANPADS launchers, a radio and a vehicle. The National Guard alone had about 130 MFGs among more than 250 firing groups | MANPADS (Igla) | [src:defenceexpress-2022-mfg-tactics] |
| Nov 2022 (e3) | Dozens of units in Mykolaiv of about six people each, from police, army, territorial defence and the National Guard. Night shifts rotated between firing points chosen to reduce civilian risk | Heavy machine guns, including a .50 cal from a destroyed Russian tank mounted on a convertible; thermal sensors; a laser to illuminate; a WWII flare gun | [src:stripes-2022-wapo-drone-hunters] |
| Dec 2022 (e3) | Small, ad hoc, dispersed teams. A UAZ-452 van carried twin PKT 7.62 mm guns on a local mount. MANPADS on vehicles were the exception | Small arms, machine guns on pickups and cars; a gun-like thermal sensor spots, then a laser marks the target for the others | [src:twz-2022-trevithick-drone-hunters] |
| Feb 2023 (e4) | Kyiv's "Shahed and missile hunters" in off-road passenger cars, ready for combat in no more than one minute | Heavy machine guns, thermal imagers, laser target designators | [src:pravda-2023-balachuk-pavliuk-mobile-groups] |
| Nov 2023 (e5–e6) | SUVs with a tripod and Mistral; light vehicles with PKT, DShKM, Maxim or Browning guns and workshop-made AA sights; trucks with ZU-23-2; upgraded Zastava M75 20 mm guns with HikVision thermal optics feeding a tablet; mobile EW teams with Nota | Machine guns, ZU-23-2, M75, Stinger and Mistral; National Guard Spaceray searchlights that detect drones to about 2 km, even when painted matte black | [src:militarnyi-2023-safronov-mobile-teams] |
| Jan 2024 (e6) | M2HB Browning with a "special digital" thermal sight with automatic target acquisition; Strela-2 with a thermal sight; Igla with Guide TK 431 thermal imagers (100 bought by the Prytula Foundation) | | [src:militarnyi-2024-special-sights-shaheds] |
| Apr 2024 (e6) | New Ukrainian searchlights tested by several mobile groups, as part of a system with a laser and a thermal sight to find and track targets and adjust fire | | [src:militarnyi-2024-searchlights] |
| Jun 2024 (e6b) | "Thousands of servicemen who go to firing positions almost every night" (Air Force commander Oleshchuk) | Small arms, AA guns, MANPADS, upgraded with night vision, thermal imagers, laser pointers, searchlights, EW devices and software | [src:onlineua-2024-oleshchuk-mfg-80pct] |
| Sep 2025 (e8) | Officially three soldiers, two on duty at the observed position, each cross-trained to fire alone. ZU-23 on GAZ-66 trucks; pickups with Brownings on the Oskil | ZU-23 with the Ukrainian ZIR+ system (optics, rangefinder, thermal imager, camera) | [src:gwara-2025-kulish-mfg-zu23] |
| Oct 2025 (e8) | 68th Jaeger Brigade: teams of three rotating every two hours, from underground, camouflaged positions with a spare position, because FPVs hunt the first | Browning (about 4 km maximum, about 2 km effective), large-calibre rifle against FPVs, electronic thermal sight | [src:armyinform-2025-68th-hunt-in-dark] |
| Feb–Mar 2026 (e9) | Pickups with machine guns, crews of 2–3, moving often | Browning M2 with thermal optics "considered the most effective"; Igla or Stinger against faster targets; the crews want interceptor drones of their own | [src:frontliner-2026-derkachov-mfg-modernise] |

Training: the Chaika and Chaika-M VR simulators, codified for the Armed Forces in November 2025, train MFG crews with virtual Igla, Stinger, DShK and M2 at night, with a searchlight [src:euromaidanpress-2025-seagull-flyswatter].

**Volunteer air-defence groups (from June 2025).** A Cabinet pilot lets volunteer formations of territorial communities form air-defence groups. They are subordinate to Territorial Defence commanders, and to the Air Force Command for air-defence tasks. Members may use small arms, strike UAVs, private manned aircraft, EW and AI systems, and must be exempt from mobilisation and trained [src:mod-2025-volunteer-air-defence-groups].

### 2.2 How they are cued

| date | the cue | lead time | source |
|---|---|---|---|
| Oct 2022 | "Centralised information, what is flying, where and where from" | n/s | [src:defenceexpress-2022-mfg-tactics] |
| Nov 2022 | Tablets showing approaching Shaheds; residents' Telegram reports; then a thermal sensor and a laser on site | "roughly 10 minutes" to reach a firing position after an alert | [src:stripes-2022-wapo-drone-hunters][src:twz-2022-trevithick-drone-hunters] |
| Nov 2023 | Acoustic sensors feeding a single database through monitoring points; a detection passed "across units until one can engage it"; Come Back Alive planned radio kits for air-picture data and target designation | n/s | [src:militarnyi-2023-safronov-mobile-teams] |
| Sep 2025 | "Readiness number one" sends all MFGs to their positions when Shaheds are launched; crews follow Telegram and maps; positions are moved to the corridors used most often | about 40 minutes from launch to arrival | [src:gwara-2025-kulish-mfg-zu23] |
| Oct 2025 | One gunner watches a tablet in an underground command post that receives "data on all detected aerial targets"; crews aim with lead, because the sound lags the drone | n/s | [src:armyinform-2025-68th-hunt-in-dark] |
| Feb 2026 | Crews "often receive warnings only after they already hear the drone"; an expert proposes 30 kg vehicle radars giving coordinates, speed and altitude, and a planned "anti-drone dome" of radars and acoustic sensors that would give azimuth and time of arrival | once a drone is heard overhead, it is in the strike zone or leaves it within seconds | [src:frontliner-2026-derkachov-mfg-modernise] |

**Where sources disagree.** The 2022 crews had a working cue of about 10 minutes, and the 2025 crews got about 40 minutes' notice of a launch. The 2026 Frontliner report says crews are often warned too late. These measure different things: a launch warning tells a crew when to man the positions, while a track tells it where to point the gun. The 2026 complaint is about the second.

### 2.3 Reach against the target

| weapon | reach | kind | source |
|---|---|---|---|
| Machine guns | effective engagement altitude up to 1 km (Air Force, 2026) | reported | [src:rbc-2026-ihnat-interview-ranges] |
| Browning M2 | about 4 km maximum, about 2 km effective | reported by a gunner | [src:armyinform-2025-68th-hunt-in-dark] |
| ZU-23-2 | effective range 2,500 m | reported | [src:gwara-2025-kulish-mfg-zu23] |
| MANPADS | effective up to 4 km, effective operating altitude about 2 km | reported (Air Force) | [src:rbc-2026-ihnat-interview-ranges] |
| Spaceray searchlight | detects drones to about 2 km | reported | [src:militarnyi-2023-safronov-mobile-teams] |
| Shahed altitude | "low-flying slow Shaheds" (2023); above 2 km, then a dive onto the target (May 2025); 1,500–2,800 m, sometimes 3,000 m (Sep 2025); either very low (up to about 200 m) or 1,000–4,000 m (Jul 2025) | reported | [src:euromaidan-2023-mfg-five-shaheds][src:kyivindependent-2025-ihnat-high-altitude][src:gwara-2025-kulish-mfg-zu23][src:gwara-2025-kulish-decoy-waves] |
| Shahed speed | about 170 km/h rising to over 200 km/h (Reuters, 2026); about 200 km/h (Gwara, 2025) | reported | [src:reuters-2026-hunder-defeat-shahed][src:gwara-2025-kulish-decoy-waves] |

Ihnat said in May 2025 that at above 2 km the drones "become more visible to our radars" but stay "out of reach of small arms, heavy machine guns, and mobile fire teams" [src:kyivindependent-2025-ihnat-high-altitude]. By February 2026 he described Shaheds flying either above MFG reach or low along reservoirs and river beds, on routes planned from Russian satellite, signals and weather intelligence [src:rbc-2026-ihnat-interview-ranges].

### 2.4 How many they killed

| date | measure | value | kind | source |
|---|---|---|---|---|
| 13 Sep–21 Oct 2022 | Shahed-136 destroyed, much credited to mobile units; on 20 Oct, 2 of 14 kills by the National Guard | 223 | reported; military reports rarely say which means killed a drone | [src:defenceexpress-2022-mfg-tactics] |
| 2022 | Shahed-136 recorded over Ukraine; hits | 440; 78 hits (about 18%) | reported (Militarnyi) | [src:militarnyi-2023-safronov-mobile-teams] |
| Nov 2022 | One Mykolaiv team's tally | 3 Shaheds by early November | reported | [src:stripes-2022-wapo-drone-hunters] |
| 25 Nov 2023 | MFG share of one night's kills (74 of 75 Shaheds downed) | about 40%; earlier up to 50%, but only 5–10 drones | reported (Ihnat) | [src:ukrinform-2023-ihnat-mfg-40pct] |
| Dec 2023 | MFGs shoot down "most" Shaheds; 2,900 of 3,700 since Sep 2022 | about 78% overall (computed) | reported (Ihnat) | [src:fakty-2023-ihnat-mfg-most] |
| 1 Jan–26 Jun 2024 | MFG share of Shaheds downed; 2,277 launched, about 86% destroyed | more than 80% | reported (Air Force commander Oleshchuk) | [src:onlineua-2024-oleshchuk-mfg-80pct] |
| Sep 2025 | ZU-23 gunner's estimate of his hit rate | "every hundredth shot hits" | estimate | [src:gwara-2025-kulish-mfg-zu23] |

The 2022 hit share of about 18% [src:militarnyi-2023-safronov-mobile-teams] does not match the roughly 95% shot down in the compiled Air Force reports for the same months (file 01, [src:kaggle-2026-ivaniuk-missile-attacks]). The two count different things (hits on targets against drones not reported shot down) and neither is independent.

### 2.5 What hunts the hunters

- **Decoys find the guns.** Gerberas reveal MFG positions when fired on, and that can bring Russian strikes; crews use red rather than white light [src:gwara-2025-kulish-mfg-zu23]. A brigade's gunners describe Russia sending one Shahed ahead to scout for firing groups, then sending others around their positions, and they hold fire without visual confirmation [src:armyinform-2025-68th-hunt-in-dark].
- **Guided Shaheds attack MFGs.** Beskrestnov reported "another hit by a guided Shahed" on an MFG crew while it engaged a different Shahed, and told commanders to plan camouflage and a course of action in advance [src:onlineua-beskrestnov-guided-shahed-mfg]. Near the front, Shaheds carrying FPVs or mines hunt vehicles, and an Igla was seen attached to a Shahed [src:frontliner-2026-derkachov-mfg-modernise].

---

## 3. Anti-aircraft guns

| system | what sources say | kind | source |
|---|---|---|---|
| Gepard / Cheetah, fleet | about 80 Gepards plus ex-Jordanian Cheetahs (Jul 2025); 15 more being refurbished by Belgium (Jun 2026); repaired in Ukraine from Aug 2025 | reported | [src:euromaidan-2025-axe-gepard-math][src:united24-2026-litnarovych-gepard-17][src:militarnyi-2025-gepard-repair] |
| Gepard, engagement | twin radar-cued 35 mm Oerlikons, up to 1,100 rds/min; targets up to 3 km altitude; "from a few kilometers away" | reported | [src:united24-2026-litnarovych-gepard-17][src:euromaidan-2025-axe-gepard-math] |
| Gepard, rounds per Shahed | bursts of 7–11 rounds often suffice; average 20–30 per target; "a handful"; once several drones with one burst | reported by operators and a developer | [src:united24-2026-litnarovych-gepard-17][src:euromaidan-2025-axe-gepard-math] |
| Gepard, one night | one crew, 17 drones in one night over Khmelnytskyi oblast, with seconds between targets (June 2026); the commander has 39 kills in all | reported (Western Air Command) | [src:united24-2026-litnarovych-gepard-17] |
| Gepard, ammunition | Germany paid $181m for 300,000 new rounds after sending its stocks, then 180,000 more; about $600 a round | reported; the $600 figure relayed from Forbes | [src:euromaidan-2025-axe-gepard-math][src:dronewarfare-2026-cuas-costs] |
| Skynex | four towed 35 mm Revolver Mk3 guns (1,000 rds/min, 4 km), a sensor unit and a command post on the Skymaster battle-management system; in use since spring 2023; two systems reported by April 2024 | reported | [src:kyivindependent-2024-york-skynex][src:espreso-2025-skynex-rounds][src:militarnyi-2025-shumlianskyi-skynex-shaheds] |
| Skynex, rounds per Shahed | about 10 AHEAD rounds; each bursts ahead of the target with 152 tungsten pellets; in one video the first round killed | reported | [src:espreso-2025-skynex-rounds] |
| Skynex, cost per kill | "just over $4,000" per target (RBC-Ukraine via Kyiv Independent, 2024); about €1,120 per AHEAD round from a Romanian contract, so about $12,900 per 10-round burst | the first reported, the second derived by drone-warfare.com; they disagree | [src:kyivindependent-2024-york-skynex][src:dronewarfare-2026-cuas-costs] |
| Skynex, reliability | about 35 Shaheds and 2 cruise missiles for one battery, 12 targets in one engagement; on 1 April 2026 three of eight guns at one site failed within minutes and a Shahed got through | reported (commander; internal report) | [src:militarnyi-2026-kozatskyi-skynex-12][src:militarnyi-2026-khomenko-skynex-issues] |
| ZU-23-2 + ZIR+ | twin 23 mm; ZIR+ adds optics, a rangefinder, a thermal imager and a camera; 2,500 m effective; would suit the lower Shaheds of earlier years better | reported | [src:gwara-2025-kulish-mfg-zu23] |
| Zastava M75 | upgraded 20×110 mm with thermal and IR optics feeding the gunner's tablet | reported | [src:militarnyi-2023-safronov-mobile-teams] |

Other guns and short-range systems named against Shaheds at low and medium altitude include Slinger, Vampire and Avenger [src:frontliner-2026-derkachov-mfg-modernise].

---

## 4. MANPADS, helicopters and fighters

- **MANPADS.** In 2022 they were the MFG's main weapon [src:defenceexpress-2022-mfg-tactics]. Later they were kept for targets too fast or too high for the guns, such as jets and missiles [src:frontliner-2026-derkachov-mfg-modernise]. Thermal sights were added to Strela-2 and Igla [src:militarnyi-2024-special-sights-shaheds]. Stated reach is up to 4 km, with an effective altitude of about 2 km [src:rbc-2026-ihnat-interview-ranges]. No source gave a MANPADS kill share or a lock-on range against the Shahed's small piston engine.
- **Helicopters.** Mi-8 and Mi-24 crews with door and turret machine guns claim more than 3,200 Shaheds from August 2024 to August 2025 [src:kyivindependent-2025-zadorozhnyy-helicopters-shaheds], more than 440 in May 2026 [src:kyivpost-2026-syrsky-may-interceptors], and up to 40% of drones in their zones [src:pravda-2026-dotsenko-helicopters-drones]. **Cueing:** ground radar gives the first coordinates. From mid-2024 an unarmed H125 or H225 with a FLIR UltraFORCE 350 turret tracks the drone at night and guides the gunship into position, and the pair try to bring the debris down in open country. On their own the Mi-8 and Mi-24 are "far less effective" at night or in poor visibility [src:united24-2025-litnarovych-h125-flir]. Russia answered with Shaheds carrying an R-60 (December 2025) or an Igla-S or Verba MANPADS in its tube, fired by an operator watching an onboard camera. TWZ found no evidence that one had yet hit an aircraft and called the combination's effectiveness "questionable" [src:pravda-2026-dotsenko-helicopters-drones][src:twz-2026-newdick-shahed-manpads].
- **Fighters.** A MiG-29 pilot downed three drones on 12 October 2022, and debris from a fifth wounded him before he ejected [src:trenchart-2025-axe-wasting-f16s]. In June 2025 F-16 pilot Lt. Col. Ustymenko downed seven targets and was killed when the last one's blast likely damaged his jet. By then three of the four F-16s Ukraine had lost had gone down on air-defence patrols during raids [src:trenchart-2025-axe-wasting-f16s]. From December 2025 F-16s carried Sniper pods and laser-guided APKWS II rockets (about $30,000–31,000 each, up to 28 or more per sortie, against about 6 air-to-air missiles); the first footage of an APKWS kill on a Shahed appeared in February 2026 [src:united24-2026-f16-apkws]. Reuters puts an F-16's haul at up to 10 Shaheds a night [src:reuters-2026-hunder-defeat-shahed]. Against jet Gerans, Ihnat says fighters or SAMs are needed [src:pravda-2026-ihnat-jet-drones].

---

## 5. Interceptor drones

### 5.1 Types

| type (maker) | speed, reach | cost | guidance as reported | sources |
|---|---|---|---|---|
| Sting (Wild Hornets) | 280 km/h, about 37 km (Reuters, Mar 2026); over 315 km/h (maker, 2025); 343 km/h (maker figure via MiGFlug) | about $2,000–2,500 | radar finds the Shahed, the Sting is launched toward it, and a pilot flies it in over a video link with a thermal camera; FPV pilots learn it in 3–4 days | [src:reuters-2026-peleschuk-interceptors-gulf][src:defenceexpress-2025-sting-effectiveness][src:migflug-2026-grunwald-interceptors] |
| P1-SUN (SkyFall) | n/s | about $1,000 | the pilot flies it until the target's shape is clear, hands over to auto-targeting (a green box), then commits to the strike; trained on more than 10,000 interception videos | [src:dronexl-2026-kesteloo-skyfall-autonomy] |
| Unnamed Brave1 interceptor (MaXon per the NYT, via DroneXL) | n/s | about $3,500 (fixed-wing) | operator selects the target "in software, one click"; then 95% automated, from launch to kill; first combat use in Kharkiv oblast, June 2026 | [src:euromaidan-2026-mukhina-one-click-interceptor][src:armyinform-2026-autonomous-interceptors][src:dronexl-2026-kesteloo-skyfall-autonomy] |
| Merops / Surveyor (Project Eagle) | over 280 km/h | about $15,000 | catapult-launched, one-way; crew of 3–4 (pilot, navigator, explosives technician, launcher and antenna engineer); its own sensor "illuminates and locks onto the target from a distance"; in Ukraine since May 2025 | [src:twz-2026-altman-merops][src:lb-2026-amelina-merops-crews] |
| ZIRKA (NOCTIS / Vyriy) | 340+ km/h, 30 km radius, 20 min | up to $2,000 | automatic detection, tracking and guidance (maker) | [src:militarnyi-2026-zirka] |
| Last Shadow T200 | 300 km/h, 1 h | n/s | hops video channels against Russian "Shtora" image spoofing | [src:militarnyi-2026-khomenko-t200-shtora] |
| General Chereshnya AIR | n/s | n/s | maker says it downed Gerberas (Jul 2025) | [src:kyivpost-2025-chereshnya-gerbera] |

### 5.2 The cue an interceptor needs

- **Radar to operator to camera.** Crews in a van watch radar dots and steer the interceptor toward one until the Shahed appears on the drone's camera, then fly into it. They have "a few minutes" from the Shahed appearing on radar until it leaves range [src:reuters-2026-hunder-defeat-shahed]. A ground radar cue plus a pilot who takes manual control for the final seconds is the norm, and performance "depends on good radar cueing and trained pilots" [src:migflug-2026-grunwald-interceptors].
- **How good the cue must be.** A Merops crew member says the most accurate data comes from "a calibrated radar with a competent operator", while flight-path programmes can be off by about a kilometre. The previous Ukrainian interceptor had to search for its target from about 200 m with a poorer camera, whereas Merops's own sensor locks on from further out [src:lb-2026-amelina-merops-crews]. These are a crew's statements, not measured accuracies.
- **Detection to kill.** Wild Hornets gives 10–15 seconds from detection to kill, and says 8 in 10 kills need only one interceptor [src:defenceexpress-2025-sting-effectiveness]. Merops crews' best shifts were 47 kills with 54 drones, and 32 with 34 in 2026; a ground station needs at least 32 interceptors a month [src:lb-2026-amelina-merops-crews]. A platoon commander's rule: "Even if you use 50 drones to shoot down one Shahed, it's worth it" [src:reuters-2026-hunder-defeat-shahed].
- **Autonomy shifts the human step.** In 2026 the human role narrowed to selecting a target and authorising the strike [src:dronexl-2026-kesteloo-skyfall-autonomy][src:euromaidan-2026-mukhina-one-click-interceptor]. No one on the record has said how the per-target authorisation copes with hundreds of Shaheds arriving at once [src:dronexl-2026-kesteloo-skyfall-autonomy].

### 5.3 Kill share by date

| date | measure | value | kind | source |
|---|---|---|---|---|
| Oct 2024 | first known Sting | – | reported | [src:defenceexpress-2025-sting-effectiveness] |
| early 2025 | first interceptor kills of Shaheds | – | reported | [src:kse-2025-drone-innovations] |
| Jul 2025 | interceptors downed "dozens" in a 500-drone attack | – | reported (Zelensky) | [src:gwara-2025-kulish-decoy-waves] |
| 30 Nov 2025 | first Sting kills of jet Geran-3s | several of 138 launched | reported | [src:euromaidan-2025-sting-jet-first] |
| winter 2025–26 | interceptors' share of Shahed kills | about 10% (Beskrestnov) or about 25% (Reuters) | reported; they disagree | [src:kyivpost-2026-syrsky-may-interceptors][src:reuters-2026-hunder-defeat-shahed] |
| Feb 2026 | sorties, kills; Kyiv share | about 6,300 sorties, 1,500+ kills; about 70% near Kyiv | reported | [src:isis-2026-anokhin-shahed-monthly][src:justsecurity-2026-bidochko-drone-superpower] |
| Apr 2026 | share of Shahed-type kills; crews | about 40%; about 1,000 crews | reported | [src:reuters-2026-hunder-defeat-shahed] |
| May 2026 | share; USF kills; Sting maker's claim | about 50%; 1,200+; 3,000+ | reported (Beskrestnov, Syrskyi, maker) | [src:kyivpost-2026-syrsky-may-interceptors][src:pravda-2026-sting-3000] |

### 5.4 How they fail

- **Weather.** One crew logged ten launches in a night and found no Shahed; on a foggy March night Kharkiv crews aborted because the cameras saw nothing [src:reuters-2026-hunder-defeat-shahed]. "No camera has yet handled bad weather well", though one pilot flying manually intercepted in hail, snow and fog [src:lb-2026-amelina-merops-crews]. Heavy snow or rain can keep MFG interceptors on the ground [src:frontliner-2026-derkachov-mfg-modernise].
- **Speed.** When Shaheds sped up from about 170 to over 200 km/h, the first interceptor became ineffective within four months, and interceptors were upgraded to about 300 km/h [src:reuters-2026-hunder-defeat-shahed]. Jets at 300–600 km/h outrun them [src:pravda-2026-ihnat-jet-drones][src:united24-2026-place-jet-gerans]. The Geran-3 cruised at only 300–350 km/h for fuel and sprinted near known air defences, which let Stings catch some [src:euromaidan-2025-sting-jet-first]. The first "ready" jet interceptors failed for lack of machine-vision lock, loiter fuel and agility [src:euromaidan-2026-mukhina-jet-interceptors]. Jets slow when they manoeuvre and can lose control under EW, and one fell after five attempts with an ordinary interceptor [src:lb-2026-amelina-merops-crews].
- **Russian counters.** From November 2025 Shaheds carried rear-view cameras above and below the hull, with extra tail antennas, apparently to see an interceptor entering the rear sector and manoeuvre [src:militarnyi-2025-kalchenko-shahed-rear-cameras]. Shahed evasion introduced in July–August 2025 took engineers about two weeks to counter [src:lb-2026-amelina-merops-crews]. The Russian "Shtora" system spoofs interceptors' video [src:militarnyi-2026-khomenko-t200-shtora].
- **Crews and software.** Pilot training is a bottleneck [src:dronexl-2026-kesteloo-skyfall-autonomy]. Some crews did not update ground-station firmware, and some makers' drones flew poorly [src:lb-2026-amelina-merops-crews].

---

## 6. Lasers, and HPM

| system | claim | status | source |
|---|---|---|---|
| Tryzub (Celebra Tech), Apr 2025 | destroys FPVs, bombs and missiles to 3 km; Shaheds, helicopters and aircraft to 5 km; dazzles to 10 km; power classified; aimed by an operator with a joystick | no engagement cited; service date unknown | [src:euromaidan-2025-tryzub-frontline-report] |
| Tryzub, May 2026 | USF commander Sukharevskyi: "at altitude over 2 km we can already shoot down aircraft with this laser"; earlier burned FPVs and damaged reconnaissance UAV optics at up to 1,500 m | aimed by hand on the video; automatic tracking not visible; analysts call it a functional prototype; no cost or production plan | [src:razomua-2026-tryzub-manual] |
| Tryzub, Jul 2026 | Deputy C-in-C Lebedenko: "It can already shoot down FPVs today, it is already working today on Shaheds"; laser air defence in 2–3 months | no documented Shahed kill; the date has slipped repeatedly | [src:euromaidan-2026-mukhina-lebedenko-laser] |
| Sunray, Feb 2026 | a telescope-like unit on a pickup burned a small drone in seconds, silently and with no visible beam; "a few hundred thousand dollars" (or "a few million"; the article conflicts) | prototype; The Atlantic, relayed by DroneXL | [src:dronexl-2026-sunray] |

The Air Force commander said in June 2024 that MFGs would likely add interceptor operators and "combat lasers" [src:onlineua-2024-oleshchuk-mfg-80pct]. Lasers need line of sight, lose power in cloud, rain, smoke and dust, and must hold the beam on a moving target [src:euromaidan-2026-mukhina-lebedenko-laser][src:razomua-2026-tryzub-manual]. **HPM/EMP:** no source found of a Ukrainian high-power microwave or EMP system used against Shaheds. The only figure found is a vendor's claim of about $0.05 per microwave shot, in a US cost survey [src:dronewarfare-2026-cuas-costs].

---

## 7. EW against Shaheds

- **What it does.** Ukraine's national GNSS jamming and spoofing system (Pokrova, mostly Lima) has been operating since early 2024 [src:militarnyi-2026-khomenko-pokrova-lima]. Its maker claims more than 20,500 Shaheds jammed [src:kyivpost-2026-lima], and its operators claim about half of all "intercepted" Shaheds and missiles [src:militarnyi-2026-kozatskyi-lima-half]. On 4 July 2025, 268 of 539 drones and decoys were shot down, 208 "diverted (likely by Ukrainian jamming)" and 63 struck [src:euromaidan-2025-axe-gepard-math]. Reuters says EW can neutralise nearly half an attack on some nights, but its effect varies [src:reuters-2026-hunder-defeat-shahed]. Trench Art argues jammers are the only truly cost-effective defence [src:trenchart-2025-axe-wasting-f16s].
- **The CRPA ladder.** Chinese CRPAs appeared on Shaheds in January 2025 with 8 elements, then 16 by March 2025, and Russia's 12-element Kometa was found on a Shahed in February. Ihnat: "The more antennas the drone has, the more suppression systems we need" [src:bi-2025-loh-shahed-crpa-16]. The ladder continued to 32 elements in 2026 [src:militarnyi-2026-khomenko-crpa-32][src:defenseexpress-2025-kometa-m-16].
- **Links, not just navigation.** Some Shaheds carry cameras, SIM cards, modems or Starlink, which makes them harder to jam, steerable online, and usable as relays for other drones [src:rbc-2026-ihnat-interview-ranges]. Mesh modems followed in 2025–26 [src:euromaidan-2026-vivdych-xk-f358].
- **Side effects.** Spoofing moved Ukrainians' phones into Moscow, Bryansk, Kursk and Belgorod, shifted their clocks to Russian time, and disrupted taxis and maps (November 2024). The General Staff advised turning off automatic time [src:therecord-2024-antoniuk-spoofing-phones]. At the front, brigade commanders banned Lima-Quant because it blinded their own Mavics [src:forbes-2026-hambling-lima-quant]. Mobile EW teams with Nota rode with the MFGs from 2023 [src:militarnyi-2023-safronov-mobile-teams].

---

## 8. Cost exchange

| item | cost | kind | source |
|---|---|---|---|
| Shahed-136 / Geran-2 | $20,000–50,000 (CSIS, midpoint $35,000); about $20,000 (2022 reporting); export version about $193,000, Russian-made $10,000–50,000 | estimates | [src:csis-2025-jensen-drone-saturation][src:stripes-2022-wapo-drone-hunters][src:kyivindependent-2024-york-skynex] |
| Shahed cost per target struck | about $350,000 | estimate (CSIS) | [src:csis-2025-hollenbeck-shahed-cost-effectiveness] |
| .50 cal round | $2.67–7 | estimate | [src:dronewarfare-2026-cuas-costs] |
| 35 mm Gepard round | about $600; $181m for 300,000 rounds is about $603 | reported; computed | [src:dronewarfare-2026-cuas-costs][src:euromaidan-2025-axe-gepard-math] |
| Gepard cost per Shahed | "a few thousand dollars"; 7–30 rounds × $600 is about $4,000–18,000 | reported; computed | [src:euromaidan-2025-axe-gepard-math][src:united24-2026-litnarovych-gepard-17] |
| 35 mm AHEAD round (Skynex) | about €1,120 (about $1,290) | derived from a Romanian contract | [src:dronewarfare-2026-cuas-costs] |
| Skynex per target | "just over $4,000" (2024) vs about $12,900 per 10-round burst | reported vs derived; they disagree | [src:kyivindependent-2024-york-skynex][src:dronewarfare-2026-cuas-costs] |
| Interceptor drones | under $1,500 (cheapest); P1-SUN about $1,000; Sting about $2,000–2,500; MaXon about $3,500; Merops about $15,000 | maker or official figures | [src:reuters-2026-hunder-defeat-shahed][src:dronexl-2026-kesteloo-skyfall-autonomy][src:migflug-2026-grunwald-interceptors][src:twz-2026-altman-merops] |
| APKWS II rocket | about $30,000–31,000 | reported (contracts) | [src:united24-2026-f16-apkws] |
| IRIS-T / NASAMS / Patriot missile | about $450,000 / $600,000–1m / nearly $4m | reported | [src:kyivindependent-2024-york-skynex] |
| LMM (Martlet) | more than $500,000 per missile, 13 kg, 8 km | reported (Axe) | [src:euromaidan-2025-axe-gepard-math] |
| Laser | "mainly electricity" per shot; vendor claims under $5 | analysis; vendor claim | [src:euromaidan-2026-mukhina-lebedenko-laser][src:dronewarfare-2026-cuas-costs] |

The same logic recurs at every date. In 2022 a US officer called fighters against $20,000 drones "completely wasteful" [src:stripes-2022-wapo-drone-hunters]. In 2023 Ihnat valued the MFGs because they let Ukraine "build up a stock of guided anti-aircraft missiles" for cruise missiles [src:fakty-2023-ihnat-mfg-most][src:ukrinform-2023-ihnat-mfg-40pct]. In 2026 Patriot crews switched to manual mode so as not to spend rounds on drones [src:euromaidan-2026-vivdych-patriot-playbook].

---

## 9. Saturation and decoys

- **Share.** About 40% Parodiya decoys in May 2025 raids (Ihnat) [src:kyivindependent-2025-ihnat-high-altitude]; about 40% decoys in 2025 [src:isis-2026-anokhin-shahed-2025]; 35–40% in 2026 [src:frontliner-2026-zvirynskyi-strike-intensity].
- **What they do to the defence.** Parodiya looks like a Shahed on radar and flies at a similar speed. Gerbera is foam or wood with no warhead, and both carry Luneburg lenses that inflate their radar return. Radars "cannot clearly separate 3–4 simultaneous targets and show them as one". Swarms with decoys overload both radar watch and MFGs, while the decoys map Ukrainian air defence and radar coverage at different altitudes. Groups of 5–10 come from one sector at different heights [src:gwara-2025-kulish-decoy-waves]. A Gerbera without a modem or camera holds a stable altitude and does not "trigger the alarm" [src:pravda-2026-konyk-gerbera-fuel-tanks], while others carry a 4G modem for live video [src:kyivpost-2025-chereshnya-gerbera].
- **The cost of a decoy shot.** Fire at a Gerbera spends the ammunition and gives away the position for a follow-up strike [src:gwara-2025-kulish-mfg-zu23]. Air Force gunners hold fire without a visual [src:armyinform-2025-68th-hunt-in-dark]. An interceptor spent on a decoy costs $1,000–15,000 (section 8).

---

## 10. Rules, debris, fratricide and civil airspace

- **Where to shoot.** In 2022 Mykolaiv units chose firing points to reduce civilian risk, and the region reported no one hurt by anti-drone fire, while downed Shaheds in Kyiv fell on apartment blocks and killed civilians [src:stripes-2022-wapo-drone-hunters]. In 2026 Ihnat said that over densely populated areas the method is "chosen mainly for civilian safety" [src:rbc-2026-ihnat-interview-ranges]. Helicopter pairs aim to bring debris down in open terrain [src:united24-2025-litnarovych-h125-flir]. In the 1–2 July 2026 strike, intercept debris started the Kyiv market fire [src:pravda-2026-vakuliuk-kyiv-overnight-damage].
- **Hold fire.** Gunners do not fire without visual confirmation; neighbouring groups held fire too [src:armyinform-2025-68th-hunt-in-dark]. In the 2022 barrages "it is often hard to know which unit destroyed a drone", because several teams fire at once [src:twz-2022-trevithick-drone-hunters].
- **Risk to the shooter.** A drone exploding close to a fighter can bring it down: MiG-29, October 2022; F-16, June 2025 [src:trenchart-2025-axe-wasting-f16s].
- **Fratricide and civil airspace.** No source found describes a Ukrainian rule for deconflicting interceptors, helicopters and fighters in the same sector, or a fratricide incident against Shaheds. No source on civil airspace rules was fetched for this draft (see section 13).

---

## 11. Measure and countermeasure, by era

| era | Russian measure | Ukrainian counter | Russian answer | sources |
|---|---|---|---|---|
| e3 (Sep–Nov 2022) | first Shahed-136 waves, low, at night, on substations | MANPADS teams, machine guns, thermal sights and lasers, central air picture, fighters | night flights only; probing radar coverage; missiles mixed in to mask drones | [src:defenceexpress-2022-mfg-tactics][src:stripes-2022-wapo-drone-hunters] |
| e4–e6a (2023–early 2024) | Shahed waves rising to 625 a month (Dec 2023) | MFGs with heavy MGs, ZU-23, M75, searchlights, acoustic net; Gepard; Skynex from spring 2023 | more drones per night; routes around MFG corridors | [src:militarnyi-2023-safronov-mobile-teams][src:ukrinform-2023-ihnat-mfg-40pct][src:espreso-2025-skynex-rounds] |
| e6–e7 (2024) | GNSS navigation | Pokrova/Lima spoofing; MFGs make 80%+ of kills (H1 2024); helicopter + FLIR pairs (mid-2024) | CRPA antennas (8 → 16 elements, Jan–Mar 2025) | [src:onlineua-2024-oleshchuk-mfg-80pct][src:united24-2025-litnarovych-h125-flir][src:bi-2025-loh-shahed-crpa-16] |
| e8a (2025) | Shaheds above 2 km with a terminal dive; 40% decoys | interceptor drones (first Shahed kills early 2025); F-16 patrols | rear cameras and evasion (Jul–Nov 2025); scouting Shahed and guided Shaheds against MFGs; MANPADS and R-60 on Shaheds against helicopters | [src:kyivindependent-2025-ihnat-high-altitude][src:militarnyi-2025-kalchenko-shahed-rear-cameras][src:twz-2026-newdick-shahed-manpads] |
| e8b–e9a (late 2025–spring 2026) | mass piston waves (8,161 in May 2026) | interceptors at about 50% of kills; AI terminal lock; APKWS on F-16s | jet Gerans at 300–600 km/h; mesh radio | [src:isis-2026-anokhin-shahed-monthly][src:united24-2026-f16-apkws][src:united24-2026-place-jet-gerans] |
| e9b (mid-2026) | jets "beyond" interceptors and MFGs | missiles and fighters; jet interceptors in testing; lasers promised | – | [src:pravda-2026-ihnat-jet-drones][src:euromaidan-2026-mukhina-jet-interceptors][src:euromaidan-2026-mukhina-lebedenko-laser] |

---

## 12. Game/sim relevance

Design observations for Sword or Shield and bavovniatko. These are suggestions drawn from the facts above, not facts themselves.

- **Engagement envelopes (slant range / altitude).** Machine guns to about 1 km of altitude (2 km effective ground range for an M2) [src:rbc-2026-ihnat-interview-ranges][src:armyinform-2025-68th-hunt-in-dark]. ZU-23 to 2.5 km [src:gwara-2025-kulish-mfg-zu23]. Gepard to about 3 km of altitude, and Skynex to 4 km [src:united24-2026-litnarovych-gepard-17][src:kyivindependent-2024-york-skynex]. MANPADS to about 4 km, 2 km effective [src:rbc-2026-ihnat-interview-ranges]. The piston Shahed's altitude band should be an era parameter: low in 2022–24, 1.5–3 km from 2025, with a terminal dive.
- **The track tiers fit.** The concept's tiers match the sources. *Cue* (about 1 km) is what flight-path programmes give and what alerts an MFG to man its position. *Track* (tens to hundreds of metres) is what an interceptor's camera needs to find the target: about 200 m for the older interceptors, further with Merops-class seekers [src:lb-2026-amelina-merops-crews]. *Fire control* is the gun's own radar (Gepard, Skynex) or a thermal sight plus searchlight or laser marker at under 2 km. Gun groups without their own radar should be unable to engage on a cue alone: crews "warned only after they already hear the drone" cannot engage [src:frontliner-2026-derkachov-mfg-modernise].
- **Reaction times.** An MFG is ready in 1 minute once in place [src:pravda-2023-balachuk-pavliuk-mobile-groups], has about 10 minutes from alert to position [src:stripes-2022-wapo-drone-hunters], and gets about 40 minutes from a launch warning to arrival [src:gwara-2025-kulish-mfg-zu23]. A Shahed is overhead for seconds, so the gun needs lead. An interceptor has a few-minute window from radar pop-up to the target leaving range [src:reuters-2026-hunder-defeat-shahed], and 10–15 s from on-camera detection to impact (maker) [src:defenceexpress-2025-sting-effectiveness].
- **Ammunition and cost per kill.** Rounds per kill: Gepard 7–30, Skynex about 10; a ZU-23 might hit with 1 shot in 100 [src:united24-2026-litnarovych-gepard-17][src:espreso-2025-skynex-rounds][src:gwara-2025-kulish-mfg-zu23]. Interceptors: about 1.1–1.25 per kill on a good shift, and many more in bad weather [src:lb-2026-amelina-merops-crews][src:defenceexpress-2025-sting-effectiveness]. Use the section 8 prices for the debrief's cost exchange.
- **Failure modes to model.** Fog, low cloud and snow should blind interceptor cameras and gun thermals [src:reuters-2026-hunder-defeat-shahed]. Gun faults: a Skynex lost 3 of 8 guns within minutes [src:militarnyi-2026-khomenko-skynex-issues]. Radar merging 3–4 close targets into one [src:gwara-2025-kulish-decoy-waves]. Shahed rear cameras and evasion when an interceptor closes from behind [src:militarnyi-2025-kalchenko-shahed-rear-cameras]. Jets that a propeller interceptor cannot catch except head-on or when the jet slows [src:euromaidan-2025-sting-jet-first][src:lb-2026-amelina-merops-crews].
- **Firing reveals you.** A burst at a decoy should mark the MFG's position for a later scouting or guided Shahed, and a searchlight should both find and reveal [src:gwara-2025-kulish-mfg-zu23][src:armyinform-2025-68th-hunt-in-dark][src:onlineua-beskrestnov-guided-shahed-mfg]. The concept's "free" rules of engagement should cost exactly this.
- **EW as an area effector with collateral.** Spoofing should divert a share of piston Shaheds (up to about half on a good night), fall off with CRPA element count, and degrade the player's own GNSS-dependent observers, including phones' positions and clocks in ePPO-style reports, and their own drones [src:euromaidan-2025-axe-gepard-math][src:bi-2025-loh-shahed-crpa-16][src:therecord-2024-antoniuk-spoofing-phones][src:forbes-2026-hambling-lima-quant]. A diverted drone is not a kill: the debrief should keep "diverted" separate from "downed", as the Air Force's own figures do not.
- **Debris and over-city rules.** A kill over a built-up area should drop debris with a casualty chance, and a rule "engage only over open ground" should trade kills for safety [src:stripes-2022-wapo-drone-hunters][src:rbc-2026-ihnat-interview-ranges].
- **Lasers.** If modelled before 2026 they are speculative. Even then: hand-aimed, weather-gated, about 3 km against drones by claim, and needing a fire-control track and a dwell time [src:razomua-2026-tryzub-manual][src:euromaidan-2026-mukhina-lebedenko-laser].

---

## 13. Open questions/gaps

- **Cue accuracy in numbers.** No source gives radar track accuracy (metres, update rate) as delivered to an MFG or an interceptor crew. The "about a kilometre" for flight-path programmes and the "about 200 m" camera search distance are one crew's statements [src:lb-2026-amelina-merops-crews].
- **MFG kill rates per group.** There are national shares for one night (40%) and one half-year (80%+), but no per-group kills per night, no rounds per kill for heavy machine guns, and no MFG count after 2022 beyond "thousands of servicemen".
- **MANPADS against Shaheds.** No kill share and no lock-on range against the Shahed's engine were found. A claim that the Shahed's heat signature is too weak to lock came only from opinion pieces and comments and is not used.
- **Lasers.** No independently documented laser kill of a Shahed. Power output, dwell time and fielding status of Tryzub and Sunray remain undisclosed. Pravda's Sunray report (from The Atlantic) blocked fetching, so the DroneXL relay is cited.
- **HPM/EMP.** Nothing found for Ukraine.
- **Fratricide and deconfliction.** No source on how interceptors, helicopters, fighters and guns share a sector, or on friendly-fire incidents. No source on civil airspace rules was fetched (Ukrainian airspace has been closed to civil flights since February 2022, but that is not cited here).
- **Interceptor shares disagree.** Winter 2025–26 is about 10% (Beskrestnov) against about 25% (Reuters). Monthly figures use different scopes (USF only, all forces, the maker), and from May 2026 the Air Force no longer publishes consistent Shahed counts [src:isis-2026-anokhin-shahed-monthly-sep].
- **Skynex cost per kill.** "Just over $4,000" (2024) against about $12,900 per 10-round burst (derived from 2026 contract prices) cannot both hold for the same ammunition. The Forbes original for the $600 Gepard round was not fetched and is relayed via drone-warfare.com.
- **Gerbera cost and signatures.** A figure of about $10,000 per Gerbera appeared only in a search snippet (Kyiv Post) and is not used. No radar or acoustic signature difference between decoys and Shaheds was found.
- **The guided-Shahed attack on an MFG** (Beskrestnov) has a date of 13 April but no year on the page fetched.

---

## 14. Sources

New (in `db/staging/sources/13-air-effectors.yaml`):
- **stripes-2022-wapo-drone-hunters** — Washington Post (via Stars and Stripes), Nov 2022: Mykolaiv drone-hunter teams, 10-minute alerts, Telegram, thermal and laser, $20k Shahed.
- **twz-2022-trevithick-drone-hunters** — TWZ, Dec 2022: team composition, laser marking, improvised gun trucks, US machine guns with thermal sights.
- **defenceexpress-2022-mfg-tactics** — Defense Express, Oct 2022: MFGs of 4–5 people with 2 MANPADS; 130 National Guard MFGs; 223 Shaheds downed; 85% (Ihnat).
- **pravda-2023-balachuk-pavliuk-mobile-groups** — Ukrainska Pravda, Feb 2023: Kyiv's hunters, ready in one minute.
- **militarnyi-2023-safronov-mobile-teams** — Militarnyi, Nov 2023: MFG weapons, sensors, Spaceray, M75, Nota, acoustic database; 2022 hit figures.
- **ukrinform-2023-ihnat-mfg-40pct** — Ukrinform, Nov 2023: MFGs about 40% of one night's kills.
- **fakty-2023-ihnat-mfg-most** — Fakty, Dec 2023: MFGs shoot down most Shaheds; missile savings.
- **euromaidan-2023-mfg-five-shaheds** — Euromaidan Press, Dec 2023: MFGs down five Shaheds; "low-flying slow Shaheds".
- **militarnyi-2024-special-sights-shaheds** — Militarnyi, Jan 2024: thermal sights on M2, Strela-2, Igla.
- **militarnyi-2024-searchlights** — Militarnyi, Apr 2024: new searchlights with laser and thermal sight.
- **kyivindependent-2024-york-skynex** — Kyiv Independent, Apr 2024: Skynex specs, cost per target, missile prices.
- **onlineua-2024-oleshchuk-mfg-80pct** — Online.ua, Jun 2024: Oleshchuk, MFGs 80%+ of Shahed kills in H1 2024.
- **therecord-2024-antoniuk-spoofing-phones** — The Record, Nov 2024: Ukrainian GNSS spoofing moves phones and clocks.
- **bi-2025-loh-shahed-crpa-16** — Business Insider, Mar 2025: 8- and 16-element CRPAs on Shaheds; Ihnat on EW.
- **euromaidan-2025-tryzub-frontline-report** — Euromaidan Press, Apr 2025: Tryzub claims.
- **kyivindependent-2025-ihnat-high-altitude** — Kyiv Independent, May 2025: Shaheds above 2 km, beyond MFGs; 40% decoys.
- **mod-2025-volunteer-air-defence-groups** — Ministry of Defence, Jun 2025: volunteer air-defence groups.
- **trenchart-2025-axe-wasting-f16s** — Trench Art, Jun 2025: F-16 and MiG-29 losses to drone debris; jammers.
- **gwara-2025-kulish-decoy-waves** — Gwara Media, Jul 2025: decoys, radar merging, Shahed altitudes and tactics.
- **united24-2025-litnarovych-h125-flir** — United24, Jul 2025: H125/H225 FLIR spotters with Mi-8/Mi-24.
- **euromaidan-2025-axe-gepard-math** — Euromaidan Press, Jul 2025: Gepard economics, fleet, ammunition; 4 July diverted figures.
- **espreso-2025-skynex-rounds** — Espreso, Aug 2025: Skynex about 10 AHEAD rounds per target.
- **gwara-2025-kulish-mfg-zu23** — Gwara Media, Sep 2025: ZU-23 with ZIR+, altitudes, 40-minute warning, 1 hit in 100.
- **armyinform-2025-68th-hunt-in-dark** — ArmyInform, Oct 2025: 68th Brigade gunners, tablet cueing, scout Shaheds.
- **militarnyi-2025-kalchenko-shahed-rear-cameras** — Militarnyi, Nov 2025: rear cameras against interceptors.
- **euromaidan-2025-sting-jet-first** — Euromaidan Press, Dec 2025: first Sting kills of Geran-3.
- **twz-2026-newdick-shahed-manpads** — TWZ, Jan 2026: Igla/Verba on Shaheds against helicopters.
- **rbc-2026-ihnat-interview-ranges** — RBC-Ukraine, Feb 2026: engagement altitudes, routes, Shahed links.
- **dronexl-2026-sunray** — DroneXL relaying The Atlantic, Feb 2026: Sunray laser.
- **united24-2026-f16-apkws** — United24, Feb 2026: F-16s with APKWS II.
- **frontliner-2026-derkachov-mfg-modernise** — Frontliner, Mar 2026: MFG crews warned too late; 30 kg radars; the dome.
- **reuters-2026-peleschuk-interceptors-gulf** — Reuters (via The Star), Mar 2026: Sting specs and guidance.
- **twz-2026-altman-merops** — TWZ, Apr 2026: Merops cost and composition.
- **reuters-2026-hunder-defeat-shahed** — Reuters (via The Star), Apr 2026: interceptor crews, weather, shares, speeds.
- **razomua-2026-tryzub-manual** — Razom UA, May 2026: Tryzub manual aiming, status.
- **euromaidan-2026-mukhina-one-click-interceptor** — Euromaidan Press, Jun 2026: 95% automated interceptor.
- **armyinform-2026-autonomous-interceptors** — ArmyInform, Jun 2026: same, official.
- **united24-2026-litnarovych-gepard-17** — United24, Jun 2026: Gepard crew, 17 drones, rounds per target.
- **dronexl-2026-kesteloo-skyfall-autonomy** — DroneXL relaying the NYT, Jun 2026: P1-SUN autonomy, saturation question.
- **euromaidan-2026-mukhina-lebedenko-laser** — Euromaidan Press, Jul 2026: Lebedenko on lasers.
- **lb-2026-amelina-merops-crews** — LB.ua, Sep 2026: Merops crews, cue quality, kills per drones, weather.
- **dronewarfare-2026-cuas-costs** — drone-warfare.com, Sep 2026: ammunition and effector costs.
- **migflug-2026-grunwald-interceptors** — MiGFlug, Oct 2026: how Sting-type interceptors are cued and flown.
- **kyivpost-2025-chereshnya-gerbera** — Kyiv Post, Jul 2025: interceptor kills of Gerberas; Gerbera features.
- **onlineua-beskrestnov-guided-shahed-mfg** — Online.ua, 13 April (year not shown): guided Shahed hits an MFG.

Existing (knowledge base):
- **isis-2026-anokhin-shahed-monthly**, **isis-2026-anokhin-shahed-monthly-sep**, **isis-2026-anokhin-shahed-2025**, **kaggle-2026-ivaniuk-missile-attacks** — volumes, hit rates, the downed/suppressed caveat.
- **kyivpost-2026-syrsky-may-interceptors**, **pravda-2026-sting-3000**, **justsecurity-2026-bidochko-drone-superpower**, **kse-2025-drone-innovations**, **defenceexpress-2025-sting-effectiveness**, **militarnyi-2026-zirka**, **militarnyi-2026-khomenko-t200-shtora**, **euromaidan-2026-mukhina-jet-interceptors** — interceptor types and shares.
- **pravda-2026-ihnat-jet-drones**, **united24-2026-place-jet-gerans** — jets outrun the cheap layer.
- **militarnyi-2025-gepard-repair**, **militarnyi-2025-shumlianskyi-skynex-shaheds**, **militarnyi-2026-kozatskyi-skynex-12**, **militarnyi-2026-khomenko-skynex-issues** — guns.
- **kyivindependent-2025-zadorozhnyy-helicopters-shaheds**, **pravda-2026-dotsenko-helicopters-drones** — helicopters.
- **militarnyi-2026-khomenko-pokrova-lima**, **kyivpost-2026-lima**, **militarnyi-2026-kozatskyi-lima-half**, **forbes-2026-hambling-lima-quant**, **defenseexpress-2025-kometa-m-16**, **militarnyi-2026-khomenko-crpa-32**, **euromaidan-2026-vivdych-xk-f358** — EW and CRPA.
- **csis-2025-jensen-drone-saturation**, **csis-2025-hollenbeck-shahed-cost-effectiveness**, **frontliner-2026-zvirynskyi-strike-intensity**, **pravda-2026-konyk-gerbera-fuel-tanks** — costs, decoys.
- **euromaidan-2026-vivdych-patriot-playbook**, **pravda-2026-vakuliuk-kyiv-overnight-damage**, **euromaidanpress-2025-seagull-flyswatter** — Patriot rationing, debris, MFG training.
