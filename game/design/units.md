# Units

Status: complete inventory of every unit named in the chapter dossiers, the era sim feed and the
topic research that our 19 chapters + 4 vignettes need depicted. Written 2026-10-02.

**What this is for.** Every entry is a candidate game asset (model, sprite, icon or effect), tied to
the chapters where it appears. Machine-readable companions:

- `design/units-by-chapter.yaml` — chapter → unit crosswalk with canonical `system:` ids.
- The 10 systems the registry lacked (marked ✚) were merged into `db/vocab/entities/system.yaml` on
  2026-10-10; Appendix A lists them.

Entries marked ✚ were registry gaps, merged on 2026-10-10. Every unit now has a canonical id in the registry.

**Sources.** `game/data/sim-20260930.json` (558 era loadouts, 581 systems), `db/vocab/entities/`
(system + actor registries), and the chapter dossiers in `db/research/chapters/`.

c07 (Bakhmut), c09 (Robotyne), c14 (Kursk) and c15 (Pokrovsk–Myrnohrad) were rebuilt on 2026-10-10 from
the systems their dossier claims are `about` in the knowledge base, filtered to depictable hardware.

**Conventions.** Chapter codes follow `design/chapters.md`: c01 march on Kyiv, c02 Snake
Island/Moskva, c03 Azovstal, c04 Kherson, c05 Kerch bridge, c06 Kharkiv '22, c07 Bakhmut, c08
Vuhledar, c09 Robotyne, c10 Krynky, c11 Avdiivka, c12 A-50 hunt, c13 Black Sea drone war, c14 Kursk,
c15 Pokrovsk, c16 Spiderweb, c17 Kupiansk pipeline, c18 night over Kyiv, c19 Vivaldi; v01 Brovary
ambush, v02 Bradley vs T-90M. Sides: `[UA]`, `[RU]`, `[both]`, `[other]` (North Korea, Iran-derived,
shadow fleet).

---

## 1. Armour and fighting vehicles

| unit | side | chapters | note |
|---|---|---|---|
| T-72B3 / B3 | RU | c01 c07 c08 c11 v01 | the workhorse; Brovary column (v01 has T-72B3 vs "older T-72" contested) |
| T-72B3A | RU | e8–e9 | Arena-M testbed in 2026 |
| T-80BV / BVM | RU | c08 c11 c14 | marine assaults at Vuhledar |
| ✚ T-80U | RU | c06 | captured example on a flatbed, Kharkiv '22 |
| T-90 | RU | c07 c08 | 90th Tank Regiment support; sent to Wagner at Bakhmut |
| T-90M Proryv | RU | c06 c11 v02 | the Stepove duel antagonist; first intact capture at Izium |
| turtle (shed) tank / super turtle T-80 | RU | c15 e7–e9 | anti-FPV shed armour; 11 lost at Razine (c15) |
| T-64 | UA | c08 | firing from hides |
| Leopard 2A4 / 2A6 | UA | c09 c11 | 47th Bde; Mala Tokmachka stall (v03 proposed); tree-line raids |
| Leopard 2R / Wisent 1 / BMR-2 / M1150 | UA | c09 | the breaching vehicles; 3 Leopard 2Rs and a Wisent lost on 8 Jun 2023 |
| T-64 with mine rollers | UA | c08 c09 | 47th Bde; a roller survives four blasts, but one stacked mine |
| M1A1 Abrams | UA | c11 c14 c15 v02 | 47th Bde; Novy Put breakthrough (c14) |
| Challenger 2 (incl. cope cage) | UA | c09 | 82nd Bde; first one lost near Robotyne |
| Marder | UA | c09 c14 | 82nd and 95th Air Assault Bdes |
| MRAP / Wolfhound | UA | c07 c09 | Bakhmut street fighting; burned past the Verbove trench |
| BMP-1 / 1AM Basurmanin / 2 / 3 | both | c01 c08 c09 c10 c11 c14 v01 | line IFVs of assault columns; UA BMP-2s at Verbove and Kursk |
| BMD-4M | RU | c09 | common loss on the Orikhiv axis |
| BMD-2 / BMD family | RU | c01 | Moshchun bridge crossing |
| ✚ BMD-3 | RU | c10 | media-photo wreck only |
| MT-LB | RU | c09 c11 v02 | Bradley's 30-second triple kill |
| BTR-82A | RU | c04 c08 c09 v01 | "one BTR per 80 soldiers" at Kherson |
| M2A2 ODS Bradley | UA | c09 c11 c14 v02 | 17 lost on 8 Jun 2023; armoured taxis; also 3rd Assault and 67th Bdes at Avdiivka |
| M113 | UA | c19 | Nove raid force |
| Stryker | UA | c09 c14 c19 | 82nd Bde (Lancet hit at Robotyne); Kursk break-in; Nove raid |
| BTR-4 | UA | c19 | same raid |
| BTR-60 / BTR-80 VBIED | UA | c19 | remote TM-62-packed breaching vehicles |

## 2. Artillery and rocket artillery

| unit | side | chapters | note |
|---|---|---|---|
| 2S1 Gvozdika | both | v01 c01 | 72nd Bde shoot-and-scoot |
| 2S3 Akatsiya | both | v01 c01 | as above |
| 2S7 Pion | both | c01 c07 | training-centre guns moved to the front; drone-cued at Bakhmut |
| 2S19 Msta-S | RU | e2–e8 | the 152 mm SP workhorse |
| 2S4 Tyulpan | RU | c08 | seen Dec 2022 |
| D-30 | both | c01 | National Guard at Horenka |
| 120 mm mortars | both | everywhere | incl. 80 mm at Krynky (c10) |
| MT-12 Rapira | UA | v01 | AT gun in the ambush group |
| M777 / M109 / Krab / CAESAR / PzH 2000 / Zuzana 2 / Archer | UA | e2 onward | the 155 mm fleet; CAESAR + Bohdana at Snake Island (c02); 17th Bde M109s (c07); ~55 guns at Robotyne (c09) |
| 122 mm guns / old mortars | UA | c07 | 24th Bde guns; mortars built in 1938–43 |
| 2S22 Bohdana | UA | c02 onward | the prototype's 46-round finale; domestic workhorse by e9 |
| BM-21 Grad | both | everywhere | v01 ambush group member |
| BM-27 Uragan | both | c02 | Danube-delta shore firing |
| BM-30 Smerch / Tornado-S | RU | e2 onward | |
| TOS-1 / TOS-1A | RU | c01 c03 c07 c08 c10 v01 | thermobaric MLRS, iconic |
| ISDM Zemledeliye | RU | e5 onward | remote minelaying MLRS |
| HIMARS / M270 / MARS II | UA | c04 c06 c14 c15 onward | bridge/depot interdiction; Glushkovo bridge (c14) |
| GMZ minelayer | UA | c08 | TM-62 belts |
| UR-77 Meteorit / MICLIC | UA | c09 | lane charges in the breach plan |
| ISDM Zemledeliye / remote-detonated mines | RU | c09 | scatterable fields; mines left in empty trenches |
| RAAM 155 mm mine shells | UA | c08 | the RAAM-corridor set piece |

## 3. Air defence

| unit | side | chapters | note |
|---|---|---|---|
| S-300PS/PT / S-300V | both | everywhere | also radar-as-bait in c12 |
| S-400 | RU | c05 c12 c14 | Kerch bridge cover; ATACMS target in Kursk |
| S-125 | UA | e1 onward | legacy low-altitude layer |
| Buk-M1 (and RU Buk) | both | c01 c06 c12 | pop-up threat vs jets; 15 UA brigades in 2022 |
| Tor / Tor-M2 / Tor-M2KM | RU | c02 | island SHORAD; one lashed to a boat deck |
| Pantsir-S1 / S1M | RU | c02 c03 c05 c16 | tug sterns, bridge deck, useless at Spiderweb point-blank |
| Strela-10 | RU | c02 | island air defence |
| MANPADS (Igla/Stinger) | both | everywhere | Hostomel Ka-52 kill; the 47th Bde's eighth Ka-52 (c09); night-over-Kyiv mobile groups |
| 9K33 Osa | RU | c14 | hit by HIMARS in Kursk |
| Gepard | UA | e3 onward | Shahed gunnery |
| Skynex | UA | e8–e9 | point defence, reliability problems |
| Patriot PAC-2/PAC-3 (incl. MSE) | UA | c12 c18 | the A-50 "SAMbush"; rationed rounds in c18 |
| SAMP/T | UA | e5 onward | the other Iskander-capable system |
| NASAMS / IRIS-T SLM | UA | e3 onward | medium SAM layer |
| S-200 (restored) | UA | c12 | the 23 Feb kill at 170–300 km |
| Neptune RK-360 coastal battery | UA | c02 | Tatra CP, launchers, transport-loaders; the Moskva |
| Harpoon coastal | UA | c02 | Vasily Bekh |
| mobile fire groups | UA | c18 | pickups + AA guns/MANPADS vs Geran waves |
| decoy SAM sites | UA | e5 onward | Metinvest fakes, dummies |
| Podlet-K1 radar | RU | c12 | Belarus-based low-altitude radar |
| Zoopark / counter-battery radars | RU | c06 c04 | Balakliia's only one, destroyed |
| AK-630 / AK-630M CIWS | RU | c02 c13 | Moskva and Ivanovets futile swarm defence |
| Bal / Bastion coastal | RU | c05 | bridge coverage |

## 4. Fixed-wing aircraft

| unit | side | chapters | note |
|---|---|---|---|
| Su-34 | RU | everywhere | KAB carrier; most-shot-down type |
| Su-35S | RU | c01 c11 c12 | high CAP over the Azov, R-77-1 shots |
| Su-30SM | RU | c12 c13 c16 | Azov patrols; the Magura V7 kill (2 May 2025) |
| Su-25 | RU | c01 c07 c11 | close support, MANPADS fodder |
| Su-24 / Su-24M | RU | e1–e6 | |
| Su-24M / Su-24MR | UA | c02 c12 | toss-bombing, MR lost at Snake Island |
| Su-27 | UA | c01 c02 | toss-bombing raid; Oksanchenko |
| MiG-29 | UA | c01 c07 c12 c14 onward | Ghost of Kyiv myth carrier; JDAM-ER/SDB bomber at Bakhmut and on the Seim |
| MiG-31K | RU | c12 | Kinzhal carrier |
| MiG-31 (parked) | RU | c16 | 30 at Belaya |
| Tu-95MS | RU | c02 c16 c18 | cruise-missile carrier; Spiderweb's main target |
| Tu-22M3 | RU | c03 c12 c16 | Azovstal strikes; S-200 claim at 308 km |
| Tu-160 | RU | c16 | parked at Olenya/Belaya, flees to Anadyr |
| A-50 / A-50U / A-50M | RU | c12 c16 | radomes struck on video at Ivanovo; radar-scope UI is the chapter's main visual |
| A-60 | RU | c12 | hit at Taganrog, Nov 2025 |
| Il-22M11 command post | RU | c12 | limped home from Strilkove |
| Il-76 / Il-78M | RU | c01 c16 | aborted Hostomel airlift; Dyagilevo ramp |
| An-12 | RU | c16 | confirmed Spiderweb kill at Olenya |
| Il-38 | RU | c05 | bridge patrol |
| Be-12 | RU | c13 | USV spotter flying boat |
| E-96M decoy | RU | c12 | 2022 Kyiv-frame jamming decoy |
| An-225 Mriya (wreck) | UA | c01 | destroyed in its hangar, iconic one-off |
| F-16 | UA | c07+ c18 | jet-drone intercepts; F-16+Patriot Su-35 ambush |
| Mirage 2000-5 | UA | c18 | second fighter type |
| Gripen C/D, E/F | UA | future | pledged |

## 5. Helicopters

| unit | side | chapters | note |
|---|---|---|---|
| Ka-52 / Ka-52M | RU | c01 c08 c09 onward | lob fire at Robotyne is its signature role |
| Mi-28N / ✚ Mi-28NM | RU | c14 e4 onward | NM is the LMUR platform; FPV clips one's tail rotor (c14) |
| UH-60 Black Hawk | UA | c15 | HUR landing in northern Pokrovsk |
| Mi-24 / Mi-35M | RU | c01 c13 c16 | Hostomel stream; Olenya runway |
| Mi-24VP | RU | c13 | Baltic detachment, lost at sea |
| Mi-8 (transport / ✚ Mi-8MTPR-1 EW) | RU | c01 c12 c13 | Bryansk "five minutes"; USV hunting |
| Ka-29 | RU | c13 | kills an R-73-armed USV on video |
| Ka-27 | RU | c13 | Kunikov search; anti-USV units |
| Mi-8 (air bridge) | UA | c03 | 7 night Dnipro–Azovstal missions, NVG fit |
| Mi-24 (escort) | UA | c03 | decoy for the air bridge |
| Mi-8/Mi-24 (Shahed hunting) | UA | c18 | 440+ drones downed in May 2026 |
| Mi-14 | UA | c02 | Col. Bedzai's rescue amphibian, shot down 7 May 2022 |

## 6. Ships, boats and port set pieces

| unit | side | chapters | note |
|---|---|---|---|
| cruiser Moskva | RU | c02 | the flagship; two Neptunes, capsize under tow |
| Vasily Bykov / Sergey Kotov / Pavel Derzhavin (Project 22160) | RU | c02 c05 c13 | patrol ships |
| Admiral Makarov (frigate) + Admiral Grigorovich class | RU | c13 | Sevastopol swarm raid |
| Admiral Essen (frigate) | RU | c02 | claimed Neptune hit Apr 2022 |
| Ivanovets (Tarantul corvette) | RU | c13 | 6 Magura hits at Donuzlav |
| Tsezar Kunikov / Novocherkassk / Olenegorsky Gornyak (Ropucha LSTs) | RU | c03 c05 c13 | Berdiansk, Feodosia, Novorossiysk |
| Samum | RU | c13 | Sea Baby hit |
| Ivan Khurs (intel ship) | RU | c13 | 6-hour chase |
| Vasily Bekh (rescue tug) | RU | c02 | Harpoons; Tor-M2KM aboard |
| Sig (tanker) | RU | c05 c13 | Sea Baby hits |
| Ivan Golubets (minesweeper) | RU | c13 | swarm raid |
| Vladimir Kozitsky (survey ship) | RU | c13 | |
| Professor Nikolai Muru (tug) | RU | c13 | |
| Sapfir (rescue ship) | RU | c02 | the "13 dead" trap |
| Kommuna (salvage ship) | RU | c02 | sent to the wreck |
| Vsevolod Bobrov (logistics ship) | RU | c02 | claimed hit |
| Slavyanin / Avangard (rail ferries) | RU | c05 | struck Mar–Apr 2026 |
| Kerch truck ferries | RU | c05 | MoLoCHKa campaign |
| KS-701 Tunets patrol boats | RU | c13 | |
| Grachonok / Sobol / Shmel gunboat / Mangust / BK-16 / Okhotnik | RU | c05 c13 | bridge-guard flotilla |
| Raptor patrol boats | RU | c02 c13 | TB2's dawn kills; Kerch Bay firefight |
| Serna / Dyugon / Ondatra / Akula landing craft | RU | c02 c13 | the ramp kill; convoy hunts |
| Project 436BIS target ship (decoy) | RU | c05 | radar decoy moored by the bridge |
| Kilo-class submarine | RU | c05 c13 | ✚ underwater Sea Baby kill, Novorossiysk Dec 2025 |
| Kairos / Virat (shadow-fleet tankers) | other | c13 | ✚ Sea Baby hits off Turkey |
| Hiurza-M gunboats | UA | c02 | the aborted May assault |
| Magura V5 / V7, Sea Baby family | UA | c05 c13 | see §7 |
| pontoon ferries, barges, barge bridges, river tugs, commandeered civilian craft | RU | c04 c05 | the Kherson crossing fleet is the chapter's target set |
| 40-car troop train / fuel trains | RU | c04 c05 | Brylivka strike; Kerch tank cars |

## 7. Naval drones

| unit | side | chapters | note |
|---|---|---|---|
| Magura V5 | UA | c13 | Group 13's kamikaze boat |
| Magura V5 R-73 "SeeDragon" fit | UA | c13 | two Mi-8s, 31 Dec 2024 |
| Magura V7 (AIM-9 fit) | UA | c13 | Su-30SM kill, 2 May 2025 |
| Sea Baby (surface / HMG / RPV-16 / minelaying / underwater variants) | UA | c05 c13 | Crimean Bridge; Kerch Bay firefight |
| Mamai | UA | c13 | Novorossiysk hits |
| prototype Starlink USV | UA | c13 | 2022 prologue |
| Katran | RU | c13 | Russia's own USV |
| KMZ "Dandelion" USV | RU | c13 | |

## 8. Aerial drones

**Recon**

| unit | side | chapters | note |
|---|---|---|---|
| DJI Mavic 3T / Matrice / Autel | both | everywhere | the sky's background noise |
| Leleka-100, A1-CM Furia, Shark / Shark-M, PD-1, SKIF, RQ-7 | UA | e1 onward | HIMARS spotting, fire adjustment |
| hexacopter cargo drone (20 kg) | UA | c10 | bridgehead resupply |
| Grifon501 recon teams | UA | c10 | 36th Bde |
| Orlan-10/30 (incl. Leer-3 EW fit), Zala, Supercam, Eleron, Korsar, Forpost | RU | everywhere | 1,000–1,500 orbits/day by e7 |
| Orion | RU | e1 | scarce MALE |

**FPV**

| unit | side | chapters | note |
|---|---|---|---|
| radio FPV (generic) | both | e1 → | the war's signature weapon |
| fibre-optic FPV (KVN / KVS ring-wing) | RU (UA later) | c14 c15 c17 c19 | KVN debuts at Kursk; 60% of RU FPVs in key sectors by e9 |
| zhdun (ambush "waiting" drone) | both | e6 onward | |
| AI terminal-guidance FPV | both | e6 onward | |
| EW-hunting FPV | UA | e8 | |
| Molniya-2 mothership + FPV cargo | RU | c17 | 40–50 km launches |
| ✚ Spiderweb FPV quadcopter ("Osa") | UA | c16 | 117 launched from cabin roofs; disambiguate from 9K33 Osa SAM |
| Dzhmil, Phantom, LuckyStrike, Mimic 3T, Angel Arrow | UA | e6 onward | named UA types |

**Bomber droppers**

| unit | side | chapters | note |
|---|---|---|---|
| R18 octocopter (Aerorozvidka) | UA | c01 | night thermal bombing of the convoy |
| Vampire / "Baba Yaga" heavy bomber | UA | c15 e5 → e9 | night bombing, mining, resupply (Myrnohrad, c15), UGV air-drop (c19) |
| converted agricultural bomber | UA | c09 | night counter-battery bombing |
| loudspeaker drone | UA | c15 | broadcasts surrender instructions |
| Russian Vampire copies / Berdysh | RU | e8–e9 | |
| Kazhan | UA | e8 | |
| "Mamont" heavy FPV carrier | UA | c10 | Prytula's 40 |
| "Desantnyk" marsupial drone | UA | c19 | mother-and-child concept |

**Long-range strike**

| unit | side | chapters | note |
|---|---|---|---|
| Shahed-136 / Geran-2 | RU | e3 onward | the mass raider |
| Geran-1 | RU | e3 | |
| jet Gerans (Geran-3/4/5) | RU | e7 onward | c18's "around the clock" layer; R-60/R-73 armed variants |
| Gerbera, Parodiya decoys, Italmas | RU | e7–e9 | 35–40% of waves are decoys |
| Banderol | RU | e8–e9 | |
| Molniya / Molniya-2, V2U, BM-35, Klin, XK-F358 mesh pods | RU | e7–e9 | Starlink-era deep strikers |
| Liutyi | UA | e6 onward | |
| FP-1 / FP-2 (S-8-firing) | UA | e9 | drone-borne rocket artillery |
| Palianytsia, Flamingo, Hornet | UA | e7–e9 | middle/deep strike; Vivaldi river-crossing kills |
| Aces campaign | UA | e8 | interceptors economics (01) |

**Interceptors**

| unit | side | chapters | note |
|---|---|---|---|
| Sting / Sting 2.0 | UA | e7 onward | 10,000+/month by e9 |
| ZIRKA | UA | c18 | unveiled 1 Jul 2026, AI tracking |
| Bullet, P1-SUN, Last Shadow T200 | UA | e8–e9 | |
| LITAVR+ | UA | e8–e9 | jet-killer class |
| propeller interceptors, radar-cued interceptors | UA | e8 onward | |

**Loitering**

| unit | side | chapters | note |
|---|---|---|---|
| Lancet-3 / 3M / Izdeliye-53 / predetonating | RU | c07 c09 e3 onward | the artillery hunter |
| KUB-BLA | RU | e3 | |

## 9. Missiles and large munitions

Depicted mostly as projectile effects + launch flashes; silhouettes matter for the debrief screens.

| unit | side | chapters | note |
|---|---|---|---|
| Neptune R-360 / Long Neptune | UA | c02 onward | two-missile salvo, 6 min flight |
| Iskander-M / Iskander-K | RU | everywhere | c18's 28-missile salvo record |
| Kalibr | RU | c02 c05 c06 c18 | ship-launched |
| Kh-101 / Kh-555 / Kh-22/32 / Kh-38 / Kh-59/69 | RU | e1 onward | |
| Kinzhal | RU | e1 onward | MiG-31K |
| Zircon | RU | c18 | 4 from Kursk, 0 intercepted |
| Onyx / Zircon (AShM) | RU | c13 | |
| Oreshnik | RU | e7 | Dnipro 21 Nov 2024 |
| KN-23 / KN-24 / Hwasong-11C | other | c14 | North Korean ballistic |
| Tochka-U | both | e1–e2 | |
| ATACMS | UA | c09 c14 e5 onward | Berdiansk helicopter base (c09); depleted by the Iran war |
| Storm Shadow / SCALP-EG | UA | c09 c14 e4 onward | Novocherkassk, BSF HQ, Popov's command post |
| HARM | UA | e2 onward | SEAD |
| JDAM-ER / AASM Hammer / GBU-39 | UA | c07 c14 e4 onward | Bakhmut high-rises; Seim bridges and pontoons |
| FAB-250/500/1500/3000 UMPK, KAB, UMPB D-30SN, UMPK-PD | RU | e4 onward | glide-bomb layer; Azovstal's FAB-3000 |
| Excalibur / GMLRS / Krasnopol / BONUS / DPICM | both | e2 onward | guided-round layer |
| Vikhr / Ataka / LMUR (Izdeliye-305) | RU | c08 c09 | Ka-52/Mi-28 ATGMs; Robotyne's defining threat |
| S-8 / S-13 rockets | both | everywhere | helo lob fire, FP-2 |
| Javelin / NLAW / Stugna / TOW / Corsar / ✚ RPG-18 | UA | c01 c03 c08 v01 v02 | the ATGM layer |
| R-77-1 / R-27 / R-73 / R-60 / AIM-9 / Meteor | both | c12 c13 | AAMs incl. drone-fired |
| Metinvest decoy M777 / fake Stugna | UA | e7 onward | $1,000 decoys that pulled Kalibr strikes |

## 10. UGVs, robots and odd vehicles

| unit | side | chapters | note |
|---|---|---|---|
| logistics UGV (generic) | both | e7 onward | resupply workhorse |
| kamikaze UGV | both | e9 | |
| ✚ NEO-1 demining robot | UA | e8–e9 | 460k-hectare clearance programme |
| Khartiia brigade UGVs | UA | c17 | 3–4-mission lifespan near Kupiansk |
| TW-12.7 Droid / gun robots | UA | c15 e8 | armed UGVs (93rd Bde at Dobropillia) |
| Zmiy | UA | c15 | casevac under 152 mm fire |
| NC13 Bullspike UGV | UA | c19 | fires into shelters, then detonates |
| air-dropped kamikaze UGV | UA | c19 | heavy bombers drop UGVs 10+ km deep |
| electric scooters / wheeled boards | RU | c17 | pipeline transit |
| motorcycles / ATVs / buggies | RU | c06 onward | assault rush tactic; 6–8-bike groups with jammers (c15); c19's reinforcement pipeline |
| civilian cars | RU | c15 | the fog column into Pokrovsk |
| quad bikes | UA | c01 | Aerorozvidka night teams |
| GAZ-66 | UA | c03 | tunnel ambulance |
| Ural truck / KamAZ-6350 / fuel tankers | RU | c01 c04 c14 c19 | convoy life; 14 reserve trucks burned near Rylsk (c14) |
| HMMWV | UA | c14 e1 onward | SSO pull-out in Kursk |
| armoured trains | RU | e6 onward | railway guard |
| wooden cabin trucks (Spiderweb) | UA | c16 | 8 fake hunting cabins, remotely opened roofs |
| booby-trapped container | UA | c16 | Chelyabinsk warehouse |
| trained dolphins (optional gag) | RU | c05 | anti-sabotage programme, pre-2022 |

## 11. Infantry and figure types

Sprite/figure types; formations are `actor:military_unit` records in the DB (already essentially
complete; the FATUM drone unit once proposed here is already `actor:ua_fatum`).

- **RU:** VDV (Hostomel, Kherson), naval infantry (810th, 155th, 40th), motor rifles, Rosgvardia /
  OMON / SOBR, Chechen Akhmat (incl. Akhmat-North), DNR/LNR corps ("Somalia", "Kaskad", Vostok,
  Pyatnashka, Dikaya Diviziya, Arbat, Alga, Atal), Wagner and Storm-Z/V penal "meat assaults",
  spetsnaz (GRU 16th, 2nd/3rd/10th/14th/22nd/24th/81st/346th), Rubikon unmanned-systems crews,
  mobilised waves, BARS, North Korean assault infantry (c14), pipeline "Veterans" specialists (c17),
  2–4-man infiltration groups in civilian clothes (c15 c17 c19), flag-planting "flagovtyky" (c19),
  tunnel cutting-torch teams (c11).
- **UA:** mechanised / motorised / air assault / airborne / mountain brigades, marines (35th–38th,
  501st…), Azov Regiment and 3rd Assault, Kraken, Artan, SSO and HUR teams, SBU Alpha (FPVs),
  National Guard (incl. 4th Rapid Reaction, 12th Operational, 27th Regt), Territorial Defence
  volunteers and armed civilians (25,000 rifles in Kyiv), border guards (Snake Island garrison,
  "Dozor"), mobile fire group crews, Aerorozvidka night teams on quad bikes, drone operators at
  remote bridges (c13), Khartiia / Code 9.2 / K-2 / Birds of Magyar drone units, foreign legion
  (incl. Georgian Legion, Colombian 4th Bn c17), Presidential Brigade "Samurai", 110th's unrotated
  garrison with elderly engineers (c11).

## 12. Structures and terrain set dressing

These are `place:` registry entities (already present) plus props. Diorama anchors per chapter:

- **c01:** Hostomel airfield + Mriya hangar + cratered runway; the 60 km convoy; blown bridges
  (Teteriv, Irpin, Moshchun); Kozarovychi dam breach and flood; WWII pillboxes; TDF roadblocks.
- **c02:** Zmiinyi rock (lighthouse, pier, ramp), Boiko towers gas rigs, the burning Moskva.
- **c03:** Azovstal plant, 24 km tunnel network, drama theatre, evacuation buses, UN/ICRC convoy.
- **c04:** Antonivskyi road + rail bridges, Kakhovka dam road, six pontoon-ferry sites, radar
  reflectors, the withdrawal column.
- **c05:** the Kerch bridge (arch, piers 145/235–236), barrier system (blockships, boom barges, gate
  barges, hedgehogs, nets, smoke generators), X-ray checkpoint, queue of 2,500 trucks.
- **c06:** Balakliia depot, pontoon decoys, Izium trophy park, Z-marked Zhiguli.
- **c07:** Bakhmut blocks as house strongpoints, high-rises (the JDAM targets), "road of life",
  fortifications west of the city, Chasiv Yar ridge.
- **c08:** Vuhledar high-rises, dacha rubble, minefield corridors.
- **c09:** Surovikin line (trenches, dragon's teeth, AT ditch), the Leopards-in-minefield set piece,
  Ka-52 lob-fire orbit.
- **c10:** flooded Krynky, wrecked road/rail bridge embankments, boats, marsh fires.
- **c11:** coke plant, terrikon, Zenit bunker, Tsarska Okhota, the 160 m tunnel, quarry lakes.
- **c12:** the radar-scope night view (track symbols, range rings) — the chapter's primary visual.
- **c13:** night Black Sea, Sevastopol booms, Donuzlav, Kerch Bay barricade, painted decoys.
- **c14:** Sudzha, border towns, the Seim bridges and pontoons, unmanned dragon's teeth, the single
  supply road, NK assault waves across fields.
- **c15:** mining-town terrikons, drone-line kill zone, net tunnels, the Razine anti-tank ditch,
  repeaters on Myrnohrad's high-rises.
- **c16:** five airfields (Olenya taiga, Belaya, Dyagilevo, Ivanovo radomes, Ukrainka), cabin trucks
  opening roofs, tyre-stacked bombers, Chelyabinsk warehouse.
- **c17:** the 1.2 m gas pipeline under the Oskil, shafts and "rest areas", compressor station,
  apartment-block hideouts, destroyed bridges, pontoons.
- **c18:** Kyiv at night, metro shelters, interceptor fields, damaged towers, burning market.
- **c19:** Lyman forests, burrow shelters, Zherebets/Nitrius rivers, Voronezh pipeline, windbreaks.

## Appendix A — registry additions

Ten systems named in the dossiers/research were absent from `db/vocab/entities/`. None existed under
another id, and all were merged into `db/vocab/entities/system.yaml` on 2026-10-10:

| id | what | why it's missing |
|---|---|---|
| `system:t_80u` | T-80U | c06 flatbed photo only |
| `system:bmd_3` | BMD-3 | c10 media-photo wreck only |
| `system:mi_28nm` | Mi-28NM | topic 03, LMUR platform |
| `system:ondatra` | Ondatra landing craft | c13, Oryx-damaged craft |
| `system:kilo_class_submarine` | Kilo-class submarine | 01, underwater Sea Baby kill |
| `system:kairos` | Kairos (shadow-fleet tanker) | 01, Sea Baby target |
| `system:virat` | Virat (shadow-fleet tanker) | 01, Sea Baby target |
| `system:rpg_18` | RPG-18 Mukha | v01, single weak source (Tyzden) — flagged |
| `system:neo_1` | NEO-1 demining robot | topic 04 |
| `system:spiderweb_fpv` | Spiderweb FPV ("Osa") | c16; must NOT absorb alias "Osa" (= 9K33 Osa SAM) |

Everything else — including Il-76, An-225, Mi-14, 2S1/2S3, MT-LB, all named ships, Plavnik/Amulet-P,
Gaz-66, Zoopark, MARS II, Katran, Dandelion, the Spiderweb cabin launcher, Bradley M2A2 ODS — is
already in the registry. The Avdiivka terrikon, coke plant, Boiko towers, Azovstal, Hostomel airport
and the Crimean/Antonivskyi bridges are `place:` entities, not systems.

## Appendix B — reuse and depiction notes

1. **Families first.** ~35 base hulls (T-72/T-80/T-90/BMP/BTR/BMD/MT-LB, M2/M113/Stryker/BTR-4,
   2S1/2S3/Msta/Grad/Uragan/TOS, Mi-8/Mi-24/Ka-52, Su-24/25/27/30/34/35, Tu-95/22/160, A-50/Il-76)
   cover ~80% of appearances; variants are equipment/livery swaps (cope cage, turtle shed, boat-deck
   Tor, tug-stern Pantsir, AA Magura, NVG Mi-8 air bridge).
2. **Effects over models** for the missile layer: six effect classes (ballistic, cruise, glide bomb,
   ATGM, AAM, loitering) with silhouette variants. 30+ missiles do not need 30 models.
3. **Drones are geometry classes:** quad, hexa, fixed-wing pod, delta wing, jet, mothership — ~6
   geometries, ~50 named skins.
4. **c12 and c16 are UI chapters.** Their biggest assets are the radar scope and the split-screen
   airfield board, not unit models.
5. **Named-ship plates.** The Black Sea chapters benefit from name decals (Ivanovets, Sergey Kotov,
   Tsezar Kunikov…) since players will recognise the Oryx footage.
6. **Damage states matter** where dossiers call for them: blinded-optics T-90M (v02), burnt-out
   bomber hulks (c16), listing Moskva (c02), wreck-in-ramp Serna (c02).
