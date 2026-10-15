<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# 11 — Russian drones and electronic warfare as the Ukrainian player meets them, 2023

Scope: the Russian reconnaissance drones, loitering munitions, FPVs and electronic warfare (EW) that faced Ukrainian units at Vuhledar (main assault 6 February 2023, 72nd Mechanised Brigade defending) and south of Mala Tokmachka / Robotyne (breach of 7–8 June 2023, 47th Mechanised Brigade attacking), with a short look at 2024–2025.

Accessed 2026-10-15. English-language sources, Ukrainian outlets paired with Western analysis. Russian claims appear only as Ukrainian or Western outlets report them, and are marked as claims. **Reported** means a figure a named party states (a count, a spec, a measured rate); **estimate** means a figure the source itself presents as approximate or derived. Where only a secondary relay of a paywalled original could be fetched, the relay is cited and the dependence is flagged.

---

## 1. Summary

- **The recon-fire loop was the core threat at both dates.** In 2023 each Russian axis commander kept an orbit of Orlan-10s over the fighting, and the artillery brigade flew its own Orlan "complex", so Ukrainian troops were often watched by two separate Orlan groups, each able to call different fires [src:rusi-2023-watling-meatgrinder]. RUSI's estimate of time from drone detection to rounds was about 3–5 minutes with an organic drone, against about 20–30 minutes when the target came from EW detection or went through a fire-control headquarters [src:rusi-2023-watling-meatgrinder][src:rusi-2022-zabrodskyi-preliminary-lessons].
- **The Lancet was the precision layer, and it peaked during the June–August 2023 counteroffensive.** Russian strike videos counted by the Russian LostArmour site, as reported by Forbes, rose from about 25 a month in early 2023 to a peak of 135 in July and 126 in August, then fell to 50–90 a month [src:forbes-2024-hambling-lancet-blast]. RUSI says Lancet-3Ms "scoured" the narrow Ukrainian breaches and hunted artillery so persistently that Ukrainian guns could not be brought forward [src:rusi-2024-watling-offensive-lessons].
- **Russian FPVs were marginal at Vuhledar and rising fast by late summer 2023.** No source found documents Russian FPVs at Vuhledar in February 2023, and an Atlantic Council analysis then saw "no tactical advantage besides occasional Lancet drone strikes" there [src:dfrlab-2023-vuhledar-war-report]. By September 2023 a 47th Brigade soldier near Robotyne said "Russia has the edge in FPV drones" [src:kyivindependent-2023-farrell-47th-surovikin-line].
- **Russian EW was dense, but patchy and switchable.** RUSI's spring-2023 estimate was about one major EW system per 10 km of front, sited about 7 km back, plus counter-drone jammers or hijacking arrays at platoon level, and Ukrainian drone losses of about 10,000 a month [src:rusi-2023-watling-meatgrinder]. Ukrainian night bombing with drones in June 2023 was "binary": when Russian EW was active the drones "could not get in", and when it relaxed the effects were dramatic [src:rusi-2023-watling-stormbreak].
- **Finding the operator was fast.** A Ukrainian air-reconnaissance spokesman said in October 2022 that Russia could find and begin targeting drone operators via DJI AeroScope in "as little as five or six minutes" [src:c4isrnet-2022-skove-cloak-drones]. DJI's DroneID broadcast the pilot's position unencrypted, which German researchers published in March 2023 [src:rub-2023-droneid-vulnerabilities].
- **There were tells.** Ukrainian troops noted in summer 2023 that Russian attack helicopters were "often heralded by the lifting of GPS jamming" [src:rusi-2023-watling-stormbreak], and RUSI later reported that Russian electronic protection dipped whenever Russian units needed precise coordinates [src:rusi-2024-watling-offensive-lessons].

---

## 2. Reconnaissance drones and the fire loop

### 2.1 The aircraft

| system | what sources say | reported or estimate | sources |
|---|---|---|---|
| Orlan-10 ceiling | 5,000 m | reported spec ("according to open sources", Armada) | [src:armada-2022-withington-master-servant][src:pravda-2022-orlan-stormtroopers][src:unn-2026-bashynskyi-orlan] |
| Orlan-10 endurance | up to 16 h (UNN, 2026) or up to 18 h (Ukrainska Pravda, 2022) | reported spec; sources disagree | [src:unn-2026-bashynskyi-orlan][src:pravda-2022-orlan-stormtroopers] |
| Orlan-10 speed | up to 150 km/h; 90–150 km/h | reported spec | [src:pravda-2022-orlan-stormtroopers][src:unn-2026-bashynskyi-orlan] |
| Orlan-10 reach | up to 120 km on the link, up to 600 km "offline" or with relay | reported spec | [src:unn-2026-bashynskyi-orlan][src:pravda-2022-orlan-stormtroopers] |
| Orlan-10 working heights | reconnaissance drone at 1–1.5 km; relay drone at 4.5–5 km or higher | reported by Ukrainian military sources (2017, Donbas) | [src:unian-2017-orlan-toretsk] |
| Orlan-10 group | usually two or three aircraft: reconnaissance, EW, and relay | reported | [src:unian-2017-orlan-toretsk] |
| Orlan-10 links | a "master" aircraft talks to the pilot on a 930 MHz link and relays commands to two "servant" aircraft carrying an optronic and a COMINT payload | analyst description (Armada) | [src:armada-2022-withington-master-servant] |
| Orlan-10 sensors | Full HD zoom cameras, infrared and thermal imagers, radio direction finders and EW equipment | reported (UNN) | [src:unn-2026-bashynskyi-orlan] |
| Orlan-30 | laser-designates for Krasnopol guided shells; more flights observed in spring 2023 | reported (Ukrainian interviews to RUSI) | [src:rusi-2023-watling-meatgrinder] |
| Leer-3 | Orlan-10s form part of the Leer-3 EW system, one system per brigade; it detects and jams cellphone traffic and can inject fake text messages | analyst description (Armada); RUSI notes it is hard to synchronise with other EW | [src:armada-2022-withington-master-servant][src:rusi-2022-zabrodskyi-preliminary-lessons] |
| Supercam S350 | reconnaissance drone for mapping and monitoring of terrain and targets; two were shot down over Mykolaiv Oblast on 20 March 2023 | reported | [src:pravda-2023-supercam-mykolaiv] |
| Zala, Supercam as spotters | Lancet targets are found by Orlan-10 or SuperCam drones | reported | [src:kyivindependent-2023-farrell-lancet] |

RUSI singled out the Orlan-10 in 2022 as a cheap platform with "high performance" that "proved difficult to counter", though its inertial navigation "makes insufficient account of windage" [src:rusi-2022-zabrodskyi-preliminary-lessons]. RUSI also judged that it flew at medium altitude and was "too cheap to be economical targets for air defences" [src:rusi-2022-zabrodskyi-preliminary-lessons]. Russia nevertheless "did not have enough of these platforms to sustain the loss rate" in the 2022 Donbas battle [src:rusi-2022-zabrodskyi-preliminary-lessons].

### 2.2 The fire loop: minutes from detection to rounds

| measure | value | kind | source |
|---|---|---|---|
| Russian unit with its own UAV, detection to effect (2022) | 3–5 min | estimate from Ukrainian interviews | [src:rusi-2022-zabrodskyi-preliminary-lessons] |
| Same, via a fire-control headquarters (2022) | 20–30 min at tactical level; about 48 h at operational level | estimate | [src:rusi-2022-zabrodskyi-preliminary-lessons] |
| Orlan-10 complexes, detection to effect (spring 2023) | "remains rapid at around 3–5 minutes" | estimate | [src:rusi-2023-watling-meatgrinder] |
| Engagement cued by EW detection (spring 2023) | about 20–30 min | estimate | [src:rusi-2023-watling-meatgrinder] |
| Three-ship Orlan-10 flight, identification to rounds (2022) | within 10 min | analyst statement (Armada) | [src:armada-2022-withington-master-servant] |
| AeroScope detection of a DJI operator to start of targeting (2022) | "as little as five or six minutes" | reported by a Ukrainian unit spokesman | [src:c4isrnet-2022-skove-cloak-drones] |
| Ukrainian M777 displacement after opening fire (June 2023) | 2–15 min, depending on distance from threat systems | reported practice | [src:rusi-2023-watling-stormbreak] |

**Where sources disagree.** The knowledge base already attributes the 20–30 minute figure to a request routed through a headquarters [src:rusi-2022-zabrodskyi-preliminary-lessons]. RUSI's 2023 report gives the same 20–30 minutes for fires cued by *EW detection* rather than by a drone [src:rusi-2023-watling-meatgrinder]. These measure different chains, and both are interview-based estimates. Armada's "within ten minutes" sits between the two and is not sourced to fieldwork [src:armada-2022-withington-master-servant].

**How the loop was wired in 2023.** RUSI describes three interlocking "reconnaissance fires circuits": a direct link from a drone team to its guns, a link through fire control to a battery, and a higher headquarters assigning alternative "means of destruction" against a detected target [src:rusi-2023-watling-meatgrinder]. The Strelets system let specialist infantry (VDV, Wagner) pass sensor detections straight to the guns, so the commander "is on, but does not need to be in the loop" [src:rusi-2023-watling-meatgrinder]. In one December 2022 Russian assault, four drones worked at once: one on Ukrainian artillery, one on reserve routes, one ahead of the assault group, and one over the assault group itself [src:rusi-2023-watling-meatgrinder]. In summer 2023 Russia "doubled down" on this complex, prioritising Krasnopol production and increasing drone designation for it [src:rusi-2023-watling-stormbreak].

### 2.3 Visibility, sound and Ukrainian counters

- **Visibility.** A Ukrainian explainer says the Orlan-10 is "quite difficult to spot, both visually and on radar" and "quite difficult to shoot down" [src:unn-2026-bashynskyi-orlan]. No fetched source describes its sound; see §10.
- **Shooting it down.** Ukrainian units downed Orlan-10s with MANPADS; on 1 May 2022 alone Ukrainian forces reported neutralising 10 [src:pravda-2022-orlan-stormtroopers]. The answer in 2023 was "a combination of countermeasures involving electronic warfare stations and firepower" [src:unn-2026-bashynskyi-orlan].
- **Jamming it.** Ukraine's Bukovel-AD has been in service against the Orlan-10 since 2016 [src:wikipedia-bukovel]. Armada argued the master aircraft's 930 MHz link could be passively detected and jammed, and that killing one aircraft takes the whole three-ship group out [src:armada-2022-withington-master-servant]. In late 2023 a Ukrainian EW station was announced with the stated aim to "blind Zala and Orlan so that they can no longer adjust their Lancets to our positions" [src:kyivindependent-2023-farrell-lancet].
- **What came later.** Radar-cued interceptor drones began thinning Orlans and Zalas only from late 2024 [src:rusi-2025-watling-reynolds-third-year].

### 2.4 How it looked to the Ukrainian side in the two battles

- **Vuhledar (e4).** RUSI's spring-2023 description of Orlan orbits at axis and artillery-brigade level comes from interviews covering December 2022 to May 2023, the window of the Vuhledar assaults [src:rusi-2023-watling-meatgrinder]. No fetched source documents a specific Orlan or Zala at Vuhledar, and Russian commentators described the attacking tanks as "blind, deaf" columns without situational awareness [src:cnn-2023-lister-vuhledar-turkeys].
- **Robotyne (e5).** At the start of the 2023 offensive, Ukrainian troops had to train for "up to 25 UAVs observing their movements" [src:rusi-2023-watling-stormbreak]. In the 7 June attempt south of Mala Tokmachka, the second Ukrainian group was spotted by drones and hit by artillery [src:euromaidan-2023-shandra-leopards-mala-tokmachka][src:guardian-2023-beaumont-wintour-mala-tokmachka]. In October 2023 a 65th Brigade sergeant near Robotyne said Russian artillery, guided by reconnaissance drones, was the main problem, more than Russian counterattacks [src:euromaidan-2023-shandra-robotyne-drones-jamming].

---

## 3. Lancet

### 3.1 The weapon

| item | value | kind | sources |
|---|---|---|---|
| Variants | Lancet-1 (about 1 kg warhead) and Lancet-3 (about 3 kg); the Lancet-3M improved warhead effect and economy of design | reported specs; RUSI on 3M | [src:kyivpost-2023-tuzov-lancet-overview][src:kyivindependent-2023-farrell-lancet][src:rusi-2023-watling-stormbreak] |
| Warhead, newest models | "up to 5 kg" | reported (Reuters/RFE/RL); conflicts with 3 kg above | [src:rferl-2023-lancet-counteroffensive] |
| Range | ">40 km" radius (July 2023); 40–70 km (Kyiv Post); strikes about 65 km from Russian-held ground (autumn 2023) | reported; the 65 km is a demonstrated case | [src:rferl-2023-lancet-counteroffensive][src:kyivpost-2023-tuzov-lancet-overview][src:kyivindependent-2023-farrell-lancet] |
| Endurance | about 40 min | reported spec | [src:rferl-2023-lancet-counteroffensive][src:kyivpost-2023-tuzov-lancet-overview] |
| Speed | about 100 km/h cruise (Ukrainian adviser); 80–110 km/h cruise and about 300 km/h in the terminal dive (Kyiv Post); "stated top speed of around 300 km/h" (Kyiv Independent) | reported; the figures measure different phases | [src:rferl-2023-lancet-counteroffensive][src:kyivpost-2023-tuzov-lancet-overview][src:kyivindependent-2023-farrell-lancet] |
| Weight | about 12 kg (Lancet-3), 5 kg (Lancet-1) | reported | [src:euromaidan-2023-shandra-lancet-new-model][src:kyivpost-2023-tuzov-lancet-overview] |
| Cost | about $35,000 (Oryx estimate); $30,000–35,000 | estimate | [src:rferl-2023-lancet-counteroffensive][src:kyivpost-2023-tuzov-lancet-overview] |
| Launch and link | catapult-launched; video from the nose camera; the Orlan-10 can carry a radio repeater to extend its control range | reported | [src:kyivpost-2023-tuzov-lancet-overview][src:rferl-2023-lancet-counteroffensive] |
| Autonomy | developers claim a mode that flies to a preloaded area and scans for targets without an operator link | manufacturer claim, relayed | [src:kyivpost-2023-tuzov-lancet-overview] |

### 3.2 When it appeared, and how often it flew

- **Arrival.** The Lancet was unveiled in 2019 and used in Syria, and was "not seen until July 2022" in Ukraine (Forbes) [src:forbes-2024-hambling-lancet-blast]. Oryx-logged use rose from about October 2022 [src:oryx-2022-lancet-kill-list], and Kyiv Post dates Russian use to autumn 2022 [src:kyivpost-2023-tuzov-lancet-overview].
- **At Vuhledar (February 2023)** an Atlantic Council analysis saw "no tactical advantage besides occasional Lancet drone strikes" on the Russian side [src:dfrlab-2023-vuhledar-war-report].
- **Monthly strike videos in 2023 (LostArmour count, relayed by Forbes).** These are videos only, and Forbes calls them "likely ... a fraction of the total" [src:forbes-2024-hambling-lancet-blast].

| month (2023) | Lancet strike videos |
|---|---|
| January–February | about 25 a month |
| March–April | almost 50 a month |
| July | 135 (peak) |
| August | 126 |
| September–October | in the 50s |
| November | 89 |
| December | 59 |

Source: [src:forbes-2024-hambling-lancet-blast]. Forbes links the post-August drop to a 9 August 2023 explosion at the Zagorsk optical plant, citing a Ukrainian OSINT group and analyst; Forbes itself says "there is no proven connection" between the plant and ZALA [src:forbes-2024-hambling-lancet-blast].
- **Front-line view.** A Ukrainian artillery crewman told Reuters in July 2023: "Earlier, in spring, they were not using Lancets as often as they are now" [src:rferl-2023-lancet-counteroffensive]. UK Defence Intelligence called the Lancet in November 2023 "a step change" in how Russia uses small drones, increasingly prominent in counter-battery fire [src:pravda-2023-ukmod-lancet-step-change].

### 3.3 Targets

- **Artillery and air defence first.** Until March 2023 Oryx's visually confirmed Lancet/Kub hits were mostly on artillery, radars and SAM launchers [src:oryx-2022-lancet-kill-list]. Russian counter-battery fire shifted in spring 2023 from saturating detected positions to firing Lancets, which also hunted Ukrainian guns as reconnaissance [src:rusi-2023-watling-meatgrinder].
- **Lead elements.** In the 2023 offensive, Lancets and FPVs "flown in complexes with ISR UAVs" struck the lead elements of Ukrainian units [src:rusi-2023-watling-stormbreak]. Kofman and Lee called Ka-52, FPV and Lancet-3 strikes "a pernicious problem", because advancing forces lacked the air-defence and EW cover they had in the defence [src:warontherocks-2023-kofman-lee-three-months].
- **Pinning the guns.** Because the Ukrainian axis was narrow and "the persistent threat from Lancet-3Ms hunting Ukrainian artillery" was constant, Ukrainian guns "could not be brought forwards", so more Russian artillery could cover the lines of advance out of Ukrainian counter-battery range [src:rusi-2024-watling-offensive-lessons].
- **Disabled and abandoned vehicles.** Some videos appear to show Lancets "finishing off" vehicles already abandoned or disabled by something else [src:rferl-2023-lancet-counteroffensive].

### 3.4 Documented strikes around Mala Tokmachka and Robotyne

| date | what | evidence and outcome | source |
|---|---|---|---|
| after 8 Jun 2023 | A Lancet-3 targets an abandoned Leopard 2A6 left from the breach south of Mala Tokmachka | ambiguous result | [src:popularmechanics-2023-roblin-minefield-losses] |
| June 2023 | A Lancet hit is claimed on a French CAESAR; one CAESAR reportedly overturned while swerving to avoid a Lancet | pro-Russian videos, relayed by Reuters | [src:rferl-2023-lancet-counteroffensive] |
| July 2023 | A Russian video shows a Lancet striking a Leopard 2 in Zaporizhzhia Oblast | Russian Telegram video; outcome not confirmed | [src:kyivindependent-2023-farrell-lancet] |
| 15 Aug 2023 | A Lancet hits a Stryker of the 82nd Air Assault Brigade east of Robotyne | Russian video; damage unclear | [src:kyivpost-2023-york-82nd-in-action] |

No fetched source confirms a Lancet kill on a Bradley at Mala Tokmachka or Robotyne. Analyses of the 8 June losses blame mines, artillery, ATGMs and Ka-52s [src:popularmechanics-2023-roblin-minefield-losses][src:guardian-2023-beaumont-wintour-mala-tokmachka].

### 3.5 Hit and kill rates

| measure | value | kind | source |
|---|---|---|---|
| Kub/Lancet hits and misses, to early March 2023 | 113 hits, 41 misses (visually confirmed) | reported (Oryx) | [src:oryx-2022-lancet-kill-list] |
| Lancet success rate | "approximately 30%" | estimate by one Ukrainian source, relayed by Forbes | [src:forbes-2024-hambling-lancet-blast] |
| Qualitative | "limited success rate", but speed and accuracy make sustained Ukrainian firing hard | estimate (RUSI interviews) | [src:rusi-2023-watling-meatgrinder] |
| Ukrainian interceptions | "at least one or two" Lancets shot down a day (defence-ministry adviser, July 2023) against "rare" successful interceptions (Kyiv Independent, November 2023) | claims; sources disagree | [src:rferl-2023-lancet-counteroffensive][src:kyivindependent-2023-farrell-lancet] |
| Lancets destroyed, Feb 2022–Aug 2026 | 10,604 | reported (Ukrainian Air Force) | [src:euromaidan-2026-murdoch-445000-intercepted] |

**Where sources disagree.** Oryx's early tally implies about 73% of filmed Lancets hit something [src:oryx-2022-lancet-kill-list], while the Ukrainian "about 30%" counts all launches, including failures and shoot-downs that are never filmed [src:forbes-2024-hambling-lancet-blast]. A "hit" in either count is not a kill. A net-triggered warhead "is unlikely to cause significant damage compared to a direct hit" [src:kyivindependent-2023-farrell-lancet].

### 3.6 Nets and cages

- Ukrainian crews put wire netting around howitzers in 2023, and both sides used metal netting around guns and equipment [src:kyivindependent-2023-farrell-lancet][src:rferl-2023-lancet-counteroffensive].
- Camouflage netting on a metal frame was rated the most effective defence. Several net layers protect against the Lancet-1, but the Lancet-3 is harder because its warhead can be set off remotely after it hits the net [src:kyivpost-2023-tuzov-lancet-overview]. In one case two Lancets failed against a T-72 because a camouflage net stopped them [src:kyivpost-2023-tuzov-lancet-overview].
- By November 2023 a newer Lancet pre-detonated its shaped charge to defeat nets and cages [src:euromaidan-2023-shandra-lancet-new-model].
- RUSI's September 2023 recommendation: "The protection of guns from Lancet-3M and other loitering munitions is also becoming a critical priority" [src:rusi-2023-watling-stormbreak].

---

## 4. Russian FPVs and drops

- **Before the counteroffensive.** The knowledge base dates RF FPVs to the Bakhmut period (December 2022 – May 2023) and their mass use to the Avdiivka period from October 2023 [src:justsecurity-2026-bidochko-drone-superpower][src:rusi-2025-watling-reynolds-third-year]. A Russian state defence enterprise announced in May 2023 that it planned to make up to 3,000 FPVs a month (reported by Reuters) [src:reuters-2023-russia-upper-hand-drones].
- **At Vuhledar (February 2023).** No fetched source documents Russian FPVs or drone drops at Vuhledar. The only Russian drone weapon an Atlantic Council analysis noted there was the occasional Lancet [src:dfrlab-2023-vuhledar-war-report].
- **In the June–August counteroffensive.** Russia used FPVs with Lancets against the lead elements of Ukrainian assaults [src:rusi-2023-watling-stormbreak]. Kofman and Lee list "improvised commercial first-person-view" drones among the Russian strike assets [src:warontherocks-2023-kofman-lee-three-months]. When Ukrainian units had to resupply by day through narrow minefield breaches, Russian troops attacked the resupply routes "using FPV loitering munitions" and tank-fired ATGMs [src:rusi-2024-watling-offensive-lessons].
- **By September–October 2023.** A 47th Brigade soldier said in September that "Russia has the edge in FPV drones", using them on vehicles, artillery and dugouts [src:kyivindependent-2023-farrell-47th-surovikin-line]. A 65th Brigade sergeant near Robotyne said in October that Russia was increasingly using FPVs against Ukrainian equipment, and reported a reusable FPV that drops its charge and returns [src:euromaidan-2023-shandra-robotyne-drones-jamming]. He also said both sides flew Mavic "wedding drones" to reconnoitre and adjust artillery [src:euromaidan-2023-shandra-robotyne-drones-jamming].
- **By November 2023.** A Ukrainian platoon commander near Bakhmut estimated Russian FPV numbers in his sector at "about double" Ukraine's, a snapshot Reuters says is hard to verify [src:reuters-2023-russia-upper-hand-drones]. A Western analyst told Reuters that 2023 "was critical to the Russian efforts to field this tech in large numbers" [src:reuters-2023-russia-upper-hand-drones].
- **Frequencies.** Russian FPVs used the 850–930 MHz band through 2023 to May 2024 [src:euromaidan-2024-martyniuk-ew-backpacks].

**So the answer to "late 2023?" is yes, with a lead-in.** Russian FPVs were present in small numbers in spring 2023, were part of the anti-armour kill chain in June–August, and were seen as outnumbering Ukraine's in some sectors by September–November 2023 [src:reuters-2023-russia-upper-hand-drones][src:rusi-2023-watling-stormbreak][src:kyivindependent-2023-farrell-47th-surovikin-line].

---

## 5. Electronic warfare against drones

### 5.1 Density and doctrine (spring–summer 2023)

- About one major EW system per 10 km of front, usually about 7 km behind the front line, "usually aimed at controlling and defeating UAVs" (RUSI estimate, spring 2023) [src:rusi-2023-watling-meatgrinder].
- Crews were "weapons free" with "minimal interest" in deconflicting their effects [src:rusi-2023-watling-meatgrinder].
- Counter-UAS kit was being pushed down to platoon level, "directional jammers and arrays for hijacking UAVs", with at least one system per platoon in Russian line infantry by Ukrainian assessment [src:rusi-2023-watling-meatgrinder].
- Russia also made "extensive use of navigational interference in the battle area", which contributed to Ukrainian drone losses of about 10,000 a month [src:rusi-2023-watling-meatgrinder].
- Eight Mi-17s with EW suites were believed to operate over southern Ukraine, attacking Ukrainian command and control from altitude to greater depth than ground systems [src:rusi-2023-watling-meatgrinder].
- In summer 2023 Russia moved away from large single platforms because Ukraine targeted emitters: Zhitel was used "much more subtly", antennas were mounted on light vehicles or dismounted, and Pole-21 was treated as "disposable" wide-area cover against drone strikes [src:rusi-2023-watling-stormbreak].
- Russian units reset their frequencies every 24 hours, deconflicted UAV orbits, and synchronised EW with manoeuvre, so that "electronic protection would dip" when precise coordinates were needed and then come back up [src:rusi-2024-watling-offensive-lessons].

### 5.2 System by system

| system | what it jams or does | range | where and when deployed | sources |
|---|---|---|---|---|
| **Shipovnik-Aero** (truck-mounted) | Takes over drones, has a "sophisticated range of effects for downing UAVs", and geolocates operators for artillery; low signature, can imitate other emitters | about 10 km | across the front in 2023; rated "particularly effective" by Ukrainian sources | [src:rusi-2023-watling-meatgrinder][src:euromaidan-2023-voichuk-economist-ew] |
| **Pole-21** | GNSS jammer: "creates a dome" that suppresses GPS and radio-navigation-reliant equipment; built against cruise missiles, guided bombs and UAVs | Russia claims 60 km; Ukrainian intelligence estimates "over 25 km" | treated as disposable wide-area cover by summer 2023; one destroyed in the Tavria sector (announced 5 Nov 2023), and a "growing number" of reported kills since the summer | [src:pravda-2023-pole21-destroyed][src:rusi-2023-watling-stormbreak][src:cnn-2023-sebastian-ew-push] |
| **R-330Zh Zhitel** | GPS, Inmarsat, Thuraya and GSM; geolocates satcom, cell and drone control points | up to about 30 km | used "more subtly" in 2023 after losses | [src:militarnyi-2026-pryhodko-lasars-zhitel][src:rusi-2023-watling-stormbreak] |
| **Leer-3** (Orlan-10-borne) | Detects and jams cellphone traffic and can inject fake texts; believed to cover 30 MHz–3 GHz | no Ukrainian or Western measured range found | one system per brigade | [src:armada-2022-withington-master-servant][src:rusi-2022-zabrodskyi-preliminary-lessons] |
| **Krasukha-4** | Airborne radar and UAV control links | 300 km claimed, unverified | hidden under nets in forest belts in Zaporizhzhia; one killed near Urozhaine in November 2023 | [src:militarnyi-2025-krasukha-4-leak][src:militarnyi-2023-kushnikov-krasukha-4-zaporizhzhia] |
| **Silok-01 / Silok-M1** | Detects drones, fixes their coordinates and cuts control, telemetry and communications | ">4 km" (Militarnyi); 3–4 km (Jamestown) | one captured by the 128th Brigade in 2022, which judged it "effective only at Russian training grounds" | [src:militarnyi-2022-silok-01-captured][src:jamestown-2022-lapaiev-ew-hype] |
| **Platoon jammers and hijacking arrays** | Directional jamming of drone links; drone takeover | not stated | at least one per platoon in line infantry (spring 2023) | [src:rusi-2023-watling-meatgrinder] |
| **"Trench jamming stations"** (suitcase-size) | Cut Ukrainian drones off from their operators so they "fall down in scores" (soldier's claim) | not stated | in Russian trenches around Robotyne, summer–autumn 2023 | [src:euromaidan-2023-shandra-robotyne-drones-jamming] |
| **Volnorez** (vehicle cone jammer) | A "dome" over a vehicle against FPVs | not stated | from autumn 2023 | [src:militarnyi-2024-kushnikov-volnorez] |
| **Torn-MDM** | Interception and near-real-time decryption of Ukrainian Motorola radios | detects 1.5–3,000 MHz and locates sources up to 70 km (Jamestown) | front-wide; Ukrainian radio receivers suppressed out to about 10 km beyond the front line | [src:rusi-2023-watling-meatgrinder][src:jamestown-2022-lapaiev-ew-hype] |
| **EW helicopters** | Mi-17s with EW suites; EW-equipped escort helicopters with directional pods against radar fly with Ka-52 groups | depth from altitude | eight believed in southern Ukraine (spring 2023); escorts seen in the summer 2023 offensive | [src:rusi-2023-watling-meatgrinder][src:rusi-2023-watling-stormbreak] |

### 5.3 What jamming did to each kind of link

- **DJI OcuSync (control and video together).** Early Russian EW focused on jamming the drones' control and video links, and DJI's OcuSync "was not built for dense jamming"; a jammer that attacks both signals often leaves the drone "unrecoverable" [src:ieee-2024-chulilla-hackers-war]. A Ukrainian operator near Bakhmut in March 2023: "When the drone loses control, it descends and they fire at it" [src:kyivpost-2023-afp-bakhmut-commandos]. A drone instructor on the southern front said in March 2023 that Russian EW could force Ukraine's commercial drones to land [src:kyivindependent-2023-query-war-from-above]. Shipovnik-Aero could go further and take control of a drone [src:euromaidan-2023-voichuk-economist-ew].
- **GNSS.** RUSI's 2022 data shows Ukrainian drone missions failing through jamming of the link, dazzling of sensors, or "denial of its navigational systems from determining the accurate location of a target" [src:rusi-2022-zabrodskyi-preliminary-lessons]. A drone that loses both its pilot link and its orientation crashes or drifts until its battery dies, and spoofing can trick a drone into thinking it is higher than it is [src:ieee-2025-pultarova-killer-drones]. The same GNSS jamming cut Excalibur's hit rate from 55% in January 2023 to 6% in August 2023, according to Ukrainian data relayed by Kyiv Post [src:kyivpost-2024-chiu-gps-weapons].
- **Analogue FPV.** From 2022 to 2024 "the main vector of attack was the control link" (typically 900 MHz ELRS-type radios), and dedicated video jamming "first appeared in summer 2024" [src:armada-2026-remler-segre-video-jamming]. Control-link jamming has favourable geometry for the defender: the drone's receiver flies towards the jammer while its transmitter stays near the operator [src:armada-2026-remler-segre-video-jamming]. So in 2023 the analogue *video* picture was mostly lost to distance and terrain, not to dedicated jammers [src:armada-2026-remler-segre-video-jamming].
- **Ukrainian fixed-wing and heavy bombers.** In June 2023, converted agricultural bombers were "fairly binary in their viability": with Russian EW active they "could not get in and usually were not committed" [src:rusi-2023-watling-stormbreak].

### 5.4 Does it ramp with depth? Is it patchy?

**It ramps with depth.**
- The jammers sit in layers: platoon jammers in the first line, and the major systems about 7 km back [src:rusi-2023-watling-meatgrinder].
- Russian suppression of Ukrainian radios reached about 10 km beyond the front line [src:rusi-2023-watling-meatgrinder].
- RUSI found drones more useful in the defence than the attack, partly because "using them in the close against advancing troops means they are in closer proximity to their control antennae than to enemy jamming" [src:rusi-2024-watling-offensive-lessons].
- This favoured the 72nd's quadcopters over the Vuhledar approaches and worked against the 47th's drones flying south over the Russian belts. That is an inference from RUSI's general finding; no source states it for these two battles.

**It is patchy in place and time.**
- A Ukrainian unit near Huliaipole lost three Mavics in two weeks in January 2023, when a Mavic had lasted six months near Donetsk a few months earlier [src:euromaidan-2025-zoria-jamming-fails].
- One southern-front drone team lost five Mavic-3s in one week of March 2023, against two the week before [src:kyivindependent-2023-query-war-from-above].
- At times "over 2,000 Ukrainian drones have been disabled in a single week" [src:euromaidan-2023-voichuk-economist-ew].
- Russian EW was switched on and off, dipping for Russian precision fires and lifting for Russian aviation [src:rusi-2024-watling-offensive-lessons][src:rusi-2023-watling-stormbreak].
- Ukrainian commanders also limited their own electronic protection to keep their drones flying, "giving freedom to enemy ISR and fires" [src:rusi-2024-watling-offensive-lessons].

### 5.5 Ukrainian countermeasures in 2022–2023

- **Frequency changes.** When jammed, an Asgard Group pilot near Bakhmut switched "to a frequency the Russians cannot easily exploit", and the picture returned within seconds (AP, September 2023) [src:ap-2023-army-of-drones-duct-tape]. The unit took three months of trial and error to learn to evade the jammers [src:ap-2023-army-of-drones-duct-tape]. FPV video moved from 5.8 GHz towards 1.2 GHz, which penetrates obstacles better and was less jammed (reported by early 2024) [src:ieee-2024-chulilla-hackers-war].
- **Power and repeaters.** Builders gained video transmitters of up to 5 W, and volunteer engineers made "two-way, two-signal repeaters" that "more than double the practical range" (reported by early 2024) [src:ieee-2024-chulilla-hackers-war].
- **Custom firmware.** Volunteers patched DJI firmware and "closed the backdoors that allowed the drones to be tracked by Aeroscope" [src:ieee-2024-chulilla-hackers-war]. See §6 for Remote ID.
- **Mission design.** Crews who pre-programmed flight paths "shielded by terrain" lived longer. Some flew without transmitting and downloaded images on recovery, at the cost of slower targeting [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **Platform change.** By April 2023 Ukrainian units had begun shifting to modular FPVs that were easier to adapt to EW [src:ieee-2024-chulilla-hackers-war].
- **Hunting the jammers.** Ukrainian drone units hunted Russian EW antennas with the aim of a "cat-and-mouse" game, and Fedorov called programming drones to strike EW "on a large scale" the key task (November 2023) [src:cnn-2023-sebastian-ew-push].
- **Altitude.** No fetched source describes flying higher as a 2023 counter to jamming; see §10.

### 5.6 Where sources disagree

- **How good Russian EW was.** In April 2022 Jamestown judged that Russian EW had "few if any recorded successes" [src:jamestown-2022-lapaiev-ew-hype], and the 128th Brigade dismissed the captured Silok [src:militarnyi-2022-silok-01-captured]. By 2023 RUSI and Zaluzhnyi's essay describe Russian EW as a leading Ukrainian weakness [src:rusi-2023-watling-meatgrinder][src:euromaidan-2023-voichuk-economist-ew]. The difference is mostly time: the 2022 judgment predates the static front.
- **EW on 8 June 2023.** ISW reported that "multiple Russian sources" said Russian EW "severely interfered" with Ukrainian command signals, GPS devices and UAV controls during the 8 June attacks [src:isw-2023-06-08-roca]. A week later ISW was still describing these as milbloggers' claims of "alleged superior" EW [src:isw-2023-06-15-roca]. This is a Russian claim relayed by ISW, not an observed effect.
- **Robotyne drone losses.** A 65th Brigade sergeant said the Delta map showed "many hundreds" of Ukrainian drones downed during the Robotyne assault [src:euromaidan-2023-shandra-robotyne-drones-jamming]. That is one soldier's unverified figure.

---

## 6. Finding the operator

| date | development | source |
|---|---|---|
| 12–16 Mar 2022 | Fedorov writes to DJI that Russia uses AeroScope, "an extended version ... taken from Syria", at "up to 50 km" (Ukrainian claim). DJI replies that AeroScope is built into all recent DJI drones and "cannot be turned off" | [src:dronedj-2022-singh-fedorov-dji-letter] |
| early May 2022 | Ukrainian operators obtain a program ("CIAJeepDoors", loaded via a USB box) that switches Remote ID off without overwriting firmware | [src:c4isrnet-2022-skove-cloak-drones] |
| about June 2022 | DJI ships firmware in which Remote ID can no longer be switched off; some older models become detectable again (per a researcher and a Ukrainian officer) | [src:c4isrnet-2022-skove-cloak-drones] |
| Jul 2022 | Aerorozvidka: "unfortunately, examples exist" of Ukrainians killed by Russian AeroScope users; it built its own R18 drones partly to evade the DJI sensor | [src:rferl-2022-aeroscope-aerorozvidka] |
| Aug–Oct 2022 | An instructor says operator casualties were initially high because of poor security; the Karlson unit says Russia still uses AeroScope and can begin targeting "in as little as five or six minutes", with artillery or Grad following | [src:c4isrnet-2022-skove-cloak-drones] |
| 2 Mar 2023 | Ruhr University Bochum and CISPA publish that DJI's DroneID broadcasts the drone's and the pilot's location, and that "the transmitted data is not encrypted" (Mini 2, Mavic Air 2, Mavic 3 among the models tested) | [src:rub-2023-droneid-vulnerabilities] |
| Mar 2023 | A southern-front instructor warns that some DJI models "can spot other drones of the same model in the area and show the original take-off position" | [src:kyivindependent-2023-query-war-from-above] |
| spring 2023 | Shipovnik-Aero geolocates operators "for artillery targeting"; fires cued by EW detection take about 20–30 min | [src:euromaidan-2023-voichuk-economist-ew][src:rusi-2023-watling-meatgrinder] |
| 2022–2023 | Russians also struck Ukrainian UAV ground control stations directly | [src:rusi-2022-zabrodskyi-preliminary-lessons] |
| early 2024 | Ukrainian operators say Russia "specifically target[s]" drone operators with glide bombs and MLRS (via ISW) | [src:interfax-2024-isw-ew-since-fall-2023] |

**Two speeds of operator hunting.** AeroScope and DroneID gave a direct position for the pilot, and the reported time to targeting was 5–6 minutes [src:c4isrnet-2022-skove-cloak-drones]. Direction-finding by EW gave a slower chain of about 20–30 minutes [src:rusi-2023-watling-meatgrinder]. These come from different sources and periods, so they are not a controlled comparison.

**Ukrainian adaptations.** Ukrainian units disabled Remote ID or patched it out [src:c4isrnet-2022-skove-cloak-drones][src:ieee-2024-chulilla-hackers-war], moved to non-DJI drones [src:rferl-2022-aeroscope-aerorozvidka], and practised basic site security such as hiding vehicles and sandbagging headquarters [src:kyivindependent-2023-query-war-from-above]. CNAS noted in early 2024 that operators were being hunted with drone-tracking software [src:cnas-2024-pettyjohn-evolution]. The Russian side reflashes its own DJIs with war firmware too [src:euromaidan-2025-martyniuk-drone-firmware].

---

## 7. Vuhledar and Robotyne specifically

### 7.1 Vuhledar, 6 February 2023 (e4)

- **What is documented.** The Ukrainian defence ran on drones: a drone feed spotted each column, and shells were "adjusted by" drones or by observers on the tallest buildings [src:nyt-2023-kramer-epic-tank-battle][src:kyivpost-2023-korshak-vuhledar-marines]. The 72nd flew DJI Matrice-class quadcopters and small drop drones [src:militarnyi-2023-vuhledar-vehicles][src:cnn-2023-lister-vuhledar-turkeys]. The volume of Ukrainian drone footage of the 6–10 February columns implies that Russian EW did not deny Ukrainian ISR over the approaches; this is an inference, not a sourced statement [src:forbes-2023-axe-vuhledar-halt][src:isw-2023-02-09-roca].
- **The Russian drone and EW side.** The only Russian drone weapon an Atlantic Council analysis noted at Vuhledar was the "occasional" Lancet [src:dfrlab-2023-vuhledar-war-report]. Russian commentators called the attacking tanks "blind, deaf" and without situational awareness [src:cnn-2023-lister-vuhledar-turkeys]. No fetched source names a Russian EW system at Vuhledar.
- **Nearby and same-season evidence.** Russian EW took three Mavics in two weeks near Huliaipole in January 2023 [src:euromaidan-2025-zoria-jamming-fails]. A southern-front drone team lost five Mavic-3s in one week in March 2023 [src:kyivindependent-2023-query-war-from-above]. RUSI's front-wide picture for December 2022 – May 2023 is about one major EW system per 10 km and about 10,000 Ukrainian drones lost a month [src:rusi-2023-watling-meatgrinder].
- **Reading.** At Vuhledar the Ukrainian player should be the side with the better drone picture, facing Russian artillery and TOS-1A fire on the town, rather than a Lancet or FPV swarm. This reading rests on the absence of documented Russian drone strikes there [src:dfrlab-2023-vuhledar-war-report][src:isw-2023-01-27-roca].

### 7.2 South of Mala Tokmachka and Robotyne, 7–8 June 2023 and after (e5)

- **The kill chain.** The 47th met the full Russian kill chain. Drones spotted the columns, and artillery, ATGMs, Ka-52s, Lancets and FPVs followed; "a tank or armored vehicle will become immobilized after hitting a mine and then be destroyed by attack helicopters or drones" [src:warontherocks-2023-kofman-lee-three-months]. On 7 June the second Ukrainian group was found by drones and shelled [src:euromaidan-2023-shandra-leopards-mala-tokmachka].
- **Lancet tempo.** Monthly Lancet strike videos were near their 2023 peak (135 in July, 126 in August) [src:forbes-2024-hambling-lancet-blast]. A Lancet-3 was filmed on an abandoned Leopard 2A6 after 8 June, and a Lancet hit an 82nd Brigade Stryker east of Robotyne on 15 August [src:popularmechanics-2023-roblin-minefield-losses][src:kyivpost-2023-york-82nd-in-action].
- **EW.** Russian sources claimed EW "severely interfered" with Ukrainian GPS devices and drone controls on 8 June (reported by ISW) [src:isw-2023-06-08-roca]. Russian trench jammers along the Robotyne belt reportedly brought Ukrainian drones down "in scores" [src:euromaidan-2023-shandra-robotyne-drones-jamming]. Ukrainian night-bomber drones were committed only when Russian EW relaxed [src:rusi-2023-watling-stormbreak].
- **Smoke vs. drones.** Only 3% of Ukrainian fire missions were smoke, because smoke blinded the drone feeds that headquarters used to run the fight [src:rusi-2023-watling-stormbreak].
- **The helicopter tell.** Ka-52 strikes came from 8–10 km and were "often heralded by the lifting of GPS jamming among Russian formations", because both armies use many of the same navigation-dependent platforms [src:rusi-2023-watling-stormbreak]. Helicopter groups often flew with an EW escort carrying directional pods aimed at radar [src:rusi-2023-watling-stormbreak].
- **Other tells.** RUSI reports two more signatures: Russian frequencies reset every 24 hours, and Russian electronic protection dipped when precise coordinates were needed [src:rusi-2024-watling-offensive-lessons]. No source found reports a GPS-jamming tell before Lancet launches or artillery.
- **Autumn.** By September–October 2023 Russia had the FPV edge in the 47th's sector, and Russian FPVs hunted equipment and dugouts around Robotyne [src:kyivindependent-2023-farrell-47th-surovikin-line][src:euromaidan-2023-shandra-robotyne-drones-jamming]. At the end of November Russian milbloggers said the weather was hampering vehicles and artillery but not drones [src:isw-2023-11-30-roca].

---

## 8. How it changed, 2024–2025

- **Late 2023 – 2024: more EW, more FPVs.**
  - Ukrainian drone operators said Russia "significantly increased electronic warfare (EW) jamming since fall 2023" [src:interfax-2024-isw-ew-since-fall-2023].
  - About 100 drones of both sides could operate at once within a 10 km radius, and one brigade detected 70–90 Russian FPVs a day (early 2024, via ISW) [src:interfax-2024-isw-ew-since-fall-2023].
  - Russian FPV control bands spread from 850–930 MHz to 720–1020 MHz, and Ukrainian vehicle jammers went from one module to seven within 2024 [src:euromaidan-2024-martyniuk-ew-backpacks][src:euromaidan-2025-zoria-jamming-fails].
  - Russian vehicles gained Volnorez domes (autumn 2023) and then "turtle" jammer boxes (April 2024) [src:militarnyi-2024-kushnikov-volnorez][src:kyivpost-2024-turtle-tanks].
- **2024: video jamming and denser ISR.**
  - Dedicated video jamming appeared in summer 2024, and dedicated video-jamming platforms in the second half of 2024 [src:armada-2026-remler-segre-video-jamming].
  - Russian Orlan and Zala orbits reached 1,000–1,500 a day by August 2024 [src:rusi-2025-watling-reynolds-third-year].
  - 60–80% of Ukrainian FPVs failed to reach their targets [src:rusi-2025-watling-reynolds-third-year].
- **Late 2024 – 2025: fibre and operator hunting.**
  - Russia fielded fibre-optic FPVs, immune to jamming, from August 2024 in Kursk [src:kyivindependent-2025-farrell-fiber-optic].
  - A Ukrainian EW commander dates the "collapse" of conventional front-line jamming to the end of 2024 [src:euromaidan-2025-zoria-jamming-fails].
  - Rubikon (founded August 2024) put over 25% of its strikes on Ukrainian drone teams by September 2025 [src:rferl-2025-rubicon].
  - The drone kill zone deepened to 10–15 km [src:kyivindependent-2025-goncharova-drone-line-launch].
  - Ukraine answered with terminal autonomy for the last ~500 m [src:kyivpost-2025-orlova-tfl1] and interceptor drones against Russian reconnaissance UAVs [src:kse-2025-drone-innovations].
- **Lancet.** Lancet strike videos fell after August 2023 (59 in December) [src:forbes-2024-hambling-lancet-blast]. The Air Force counted 10,604 Lancets destroyed by August 2026 [src:euromaidan-2026-murdoch-445000-intercepted].

---

## 9. Game/sim relevance

- **Two different drone weathers.**
  - **Vuhledar (6 Feb 2023).** Ukrainian quadcopters should mostly work over their own approaches: they fly near their own antennas and are rarely jammed in the close [src:rusi-2024-watling-offensive-lessons]. The player should still lose roughly a few drones per team per week to EW and accidents [src:kyivindependent-2023-query-war-from-above][src:euromaidan-2025-zoria-jamming-fails]. Russian Lancet strikes should be rare, with no Russian FPVs [src:dfrlab-2023-vuhledar-war-report].
  - **Robotyne (June–August 2023).** Russian Lancets should be near their 2023 peak, a few strikes a day across the theatre, aimed at guns, lead vehicles and immobilised or abandoned vehicles [src:forbes-2024-hambling-lancet-blast][src:rusi-2023-watling-stormbreak][src:rferl-2023-lancet-counteroffensive]. Russian FPVs should rise from June towards an edge by September [src:kyivindependent-2023-farrell-47th-surovikin-line].
- **EW as a depth-ramped, switchable field.** Model a thin Russian platoon-jammer belt at the line and major emitters about 7 km back at about one per 10 km [src:rusi-2023-watling-meatgrinder]. Link loss should grow as a Ukrainian drone flies south over Russian lines [src:rusi-2024-watling-offensive-lessons].
  - Each Russian emitter should have an on/off schedule: dips before Russian precision strikes, and GNSS jamming lifted before Ka-52 runs. Expose these as readable tells [src:rusi-2024-watling-offensive-lessons][src:rusi-2023-watling-stormbreak].
  - Heavy-bomber sorties should be all-or-nothing on whether EW is up [src:rusi-2023-watling-stormbreak].
- **Link-specific damage (2023).** Jamming of a DJI link makes the drone descend or land, where it can be shot down or taken over. GNSS loss makes it drift and its target coordinates wrong [src:kyivpost-2023-afp-bakhmut-commandos][src:ieee-2024-chulilla-hackers-war][src:rusi-2022-zabrodskyi-preliminary-lessons]. Analogue FPVs in 2023 lose the *control* link near Russian jammers, not the video. Dedicated video jamming should not exist before summer 2024 [src:armada-2026-remler-segre-video-jamming].
- **The operator as a target.** Each DJI sortie should leave a detectable pilot position. With AeroScope or DroneID, the time to incoming fire is about 5–6 minutes; with EW direction-finding it is about 20–30 [src:c4isrnet-2022-skove-cloak-drones][src:rusi-2023-watling-meatgrinder]. Moving after launch and patched firmware should reduce this exposure [src:c4isrnet-2022-skove-cloak-drones][src:ieee-2024-chulilla-hackers-war].
- **Counter-battery clock.** The player's guns should be detected and fired on within minutes. Ukrainian crews displaced 2–15 minutes after opening fire [src:rusi-2023-watling-stormbreak]. Under Lancet-3M threat, guns should not be able to follow the breach forward [src:rusi-2024-watling-offensive-lessons].
- **Nets as a cheap, partial counter.** Nets should defeat most Lancet-1s and reduce damage from Lancet-3 net-triggered detonations. That benefit should shrink once the pre-detonating Lancet arrives (November 2023) [src:kyivpost-2023-tuzov-lancet-overview][src:euromaidan-2023-shandra-lancet-new-model].
- **What the player's own drones should do.** Spot and adjust fire (Vuhledar's kill zone) [src:nyt-2023-kramer-epic-tank-battle]. Hunt Russian EW emitters such as Pole-21 [src:cnn-2023-sebastian-ew-push]. Night-bomb reserves when EW lapses [src:rusi-2023-watling-stormbreak]. Using smoke should cost the player its own drone picture [src:rusi-2023-watling-stormbreak].

---

## 10. Open questions/gaps

- **Russian drones and EW at Vuhledar itself.** No fetched source names a Russian EW system, Orlan sortie or drone strike other than "occasional" Lancets at Vuhledar in January–February 2023. The Russian side of the drone fight there is reconstructed from the front-wide picture.
- **Orlan sound and visual signature.** No English-language source found describes how audible the Orlan-10 is, or at what distance it is seen.
- **Altitude as a counter-jamming technique.** No 2023 source found describes Ukrainian pilots flying higher to keep the link. The Guardian's April 2023 report on the range of Ukrainian drones shrinking near Bakhmut could not be fetched (domain blocked) and is known only through secondary summaries, so it is not cited.
- **Lancet effect on armour at Robotyne.** No confirmed Lancet kill on a Bradley or Leopard was found. Filmed hits on Leopards (June and July 2023) come from Russian channels with unconfirmed outcomes. The only volume series (LostArmour) is Russian and is used here only as Forbes relays it.
- **Measured jammer ranges for 2023 tactical systems.** Trench and platoon jammers have no published ranges. Leer-3's 30 km range is a Russian Ministry of Defence claim found only in non-qualifying outlets, and the Pole-21 range is disputed (60 km Russian claim against more than 25 km Ukrainian estimate).
- **Russian FPV numbers in 2023.** Only planning figures (3,000 a month announced) and a sector snapshot (about 2:1) exist.
- **GPS-jamming tells other than for aviation.** RUSI is the only source for the helicopter tell and the EW dip, and no independent corroboration was found.
- **Zala and Supercam specifications.** Range and endurance figures for ZALA 421-16E and Supercam S350 were not found in a qualifying source.

---

## 11. Sources

New (in `db/staging/sources/11-drones-ru-ew-2023.yaml`):
- **rferl-2023-lancet-counteroffensive** — Reuters/RFE/RL, Jul 2023: Lancet range, speed, cost, a Ukrainian gunner's account of rising use, interception claims.
- **kyivpost-2023-tuzov-lancet-overview** — Lancet variants and specs, Orlan repeater, nets and remote detonation.
- **forbes-2024-hambling-lancet-blast** — LostArmour monthly Lancet counts for 2023; the "~30%" Ukrainian estimate; the ZOMZ hypothesis.
- **pravda-2023-ukmod-lancet-step-change** — UK Defence Intelligence: Lancet a "step change", used in counter-battery.
- **unn-2026-bashynskyi-orlan** — Orlan-10 specs and Ukrainian counters (Ukrainian explainer).
- **pravda-2022-orlan-stormtroopers** — Orlan-10 specs; MANPADS kill; 10 downed in a day.
- **armada-2022-withington-master-servant** — Orlan three-ship structure, 930 MHz link, Leer-3, "within ten minutes".
- **unian-2017-orlan-toretsk** — Orlan group roles and working heights.
- **pravda-2023-supercam-mykolaiv** — Supercam S350 role; two downed in March 2023.
- **euromaidan-2023-shandra-robotyne-drones-jamming** — 65th Brigade sergeant on Russian drones, FPVs and trench jammers at Robotyne.
- **pravda-2023-pole21-destroyed** — Pole-21 role, range dispute, kill announced by Tarnavskyi.
- **cnn-2023-sebastian-ew-push** — Pole-21 hunting; Fedorov on EW and drones (Nov 2023).
- **jamestown-2022-lapaiev-ew-hype** — Early-war Russian EW underperformance; Silok-M1 and Torn-MDM figures.
- **militarnyi-2022-silok-01-captured** — Silok-01 captured and judged ineffective.
- **ieee-2024-chulilla-hackers-war** — OcuSync vulnerability; AeroScope backdoors closed; 1.2 GHz video, 5 W transmitters, repeaters.
- **ieee-2025-pultarova-killer-drones** — What a drone does when it loses link and orientation; spoofing.
- **armada-2026-remler-segre-video-jamming** — Control-link vs video jamming; video jamming only from summer 2024.
- **kyivindependent-2023-query-war-from-above** — Southern-front drone team: Mavic losses, forced landings, take-off position exposed.
- **kyivpost-2023-afp-bakhmut-commandos** — Jammed drone "descends and they fire at it".
- **ap-2023-army-of-drones-duct-tape** — Frequency switching to evade Russian jamming.
- **reuters-2023-russia-upper-hand-drones** — Russian FPV scale-up in 2023; 2:1 sector snapshot.
- **interfax-2024-isw-ew-since-fall-2023** — Russian EW up since fall 2023; operator targeting; FPV counts.
- **dronedj-2022-singh-fedorov-dji-letter** — Fedorov's AeroScope letter and DJI's reply.
- **rferl-2022-aeroscope-aerorozvidka** — AeroScope used against Ukrainian operators; Aerorozvidka's response.
- **c4isrnet-2022-skove-cloak-drones** — Remote ID disabling; 5–6 minutes to targeting.
- **rub-2023-droneid-vulnerabilities** — DroneID broadcasts the pilot's location unencrypted.

Existing (knowledge base):
- **rusi-2022-zabrodskyi-preliminary-lessons** — 2022 drone attrition, Orlan-10, 3–5 and 20–30 minute fire times, Leer-3.
- **rusi-2023-watling-meatgrinder** — Orlan complexes, EW density, Shipovnik-Aero, platoon jammers, Mi-17 EW.
- **rusi-2023-watling-stormbreak** — 2023 offensive: binary bomber drones, GPS-jamming tell, disposable Pole-21, Lancet-3M.
- **rusi-2024-watling-offensive-lessons** — Lancets pin the guns; FPVs on resupply routes; EW dips and frequency resets; defender's drone advantage.
- **rusi-2025-watling-reynolds-third-year** — 2024 picture: Orlan and Zala orbits, FPV failure rates, interceptors.
- **kyivindependent-2023-farrell-lancet** — Lancet spotters, nets, Leopard video, rarity of interceptions.
- **euromaidan-2023-shandra-lancet-new-model** — Pre-detonating Lancet; weight.
- **oryx-2022-lancet-kill-list** — Lancet/Kub hits and misses to March 2023.
- **euromaidan-2026-murdoch-445000-intercepted** — Lancets destroyed by August 2026.
- **euromaidan-2023-voichuk-economist-ew** — Shipovnik-Aero; 2,000 drones a week.
- **euromaidan-2025-zoria-jamming-fails** — Huliaipole Mavic losses (Jan 2023); collapse of jamming (end 2024).
- **militarnyi-2026-pryhodko-lasars-zhitel**, **militarnyi-2023-kushnikov-krasukha-4-zaporizhzhia**, **militarnyi-2025-krasukha-4-leak**, **militarnyi-2024-kushnikov-volnorez**, **wikipedia-bukovel** — EW system details.
- **euromaidan-2024-martyniuk-ew-backpacks**, **kyivpost-2024-turtle-tanks**, **kyivindependent-2025-farrell-fiber-optic**, **rferl-2025-rubicon**, **kyivindependent-2025-goncharova-drone-line-launch**, **kyivpost-2025-orlova-tfl1**, **kse-2025-drone-innovations**, **euromaidan-2025-martyniuk-drone-firmware**, **cnas-2024-pettyjohn-evolution** — 2024–2025 changes.
- **kyivpost-2024-chiu-gps-weapons** — Excalibur under Russian GNSS jamming.
- **justsecurity-2026-bidochko-drone-superpower** — Dating of the FPV introduction and boom.
- **warontherocks-2023-kofman-lee-three-months**, **kyivindependent-2023-farrell-47th-surovikin-line**, **kyivpost-2023-york-82nd-in-action**, **popularmechanics-2023-roblin-minefield-losses**, **euromaidan-2023-shandra-leopards-mala-tokmachka**, **guardian-2023-beaumont-wintour-mala-tokmachka**, **isw-2023-06-08-roca**, **isw-2023-06-15-roca**, **isw-2023-11-30-roca** — Robotyne chapter sources.
- **dfrlab-2023-vuhledar-war-report**, **nyt-2023-kramer-epic-tank-battle**, **kyivpost-2023-korshak-vuhledar-marines**, **cnn-2023-lister-vuhledar-turkeys**, **militarnyi-2023-vuhledar-vehicles**, **forbes-2023-axe-vuhledar-halt**, **isw-2023-02-09-roca**, **isw-2023-01-27-roca** — Vuhledar chapter sources.
