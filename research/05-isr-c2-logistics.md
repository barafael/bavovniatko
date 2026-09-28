# 05 — ISR, C2 and Logistics

Research notes for *bavovniatko*. Scope: intelligence, surveillance and reconnaissance (ISR); command and control (C2); and logistics in the Russo-Ukrainian war, 2022 to September 2026. We look at it from the Ukrainian side. Russian capabilities are described as Ukrainian and Western sources report them. Inline source tags point to `sources/05-isr-c2-logistics.yaml`.

---

## 1. Overview

The war turned into a contest of **sensors → decision → strike** loops. Whichever side closes the loop faster, and denies the other its loop, holds ground at lower cost. Four long trends run through all nine eras.

1. **Battlefield transparency grew, and the "kill zone" got deeper.**
   - **2022:** Surveillance came mainly from Orlan-10 drones, Bayraktar TB2s, commercial and allied satellites, and intercepted radio traffic. Artillery did the killing. Russian units with their own drones could put fire on a target 3–5 minutes after spotting it [src:rusi-2022-zabrodskyi-preliminary-lessons].
   - **Early 2025:** RUSI described a dense drone network. The battlefield was fully visible within 3 km of the line of contact, coverage thinned out to about 15 km, and commanders could task reconnaissance out to about 40 km [src:rusi-2025-watling-tactical-developments].
   - **Mid-2026:** Ukrainian commanders put the zone of regular drone strikes at **20–25+ km on both sides**. One corps commander expects 30 km by the end of 2026 [src:pravda-2026-levytska-magyar-kill-zone] [src:euromaidanpress-2026-vivdych-kill-zone-corps].
2. **Mass became a liability.**
   - Assault groups shrank: from BTG columns (2022), to Ukrainian assault detachments of 20 or fewer, to Russian infiltration teams of 1–5 people (2025–26) [src:rusi-2025-watling-tactical-developments] [src:kyivindependent-2025-farrell-kostiantynivka] [src:rusi-2025-watling-combined-arms].
   - Vehicles hide in dug-in shelters and dart out to fire. Troops walk the last 10–15+ km [src:rbcukraine-2026-perch-kill-zone-foot].
3. **Ukrainian C2 went digital and cloud-based, with Starlink as the backbone.**
   - The tools are Kropyva, GIS Arta, Delta and its modules (Vezha, Avengers AI, Mission Control, Delta Tube), plus the ePoints bonus scheme. All of them ride on Starlink and cellular links [src:csis-2024-bondar-delta-cjadc2] [src:newamerica-2023-zikusoka-gis-arta].
   - Russia built a similar system: Strelets, drone "reconnaissance-fire circuits", and smuggled Starlink terminals. Russian C2 has been reported as more centralised and more fragile [src:rusi-2023-watling-meatgrinder].
   - That fragility was exposed in **February 2026**. SpaceX's whitelist cut off unregistered Starlink terminals, Russian front-line C2 broke down, and Ukraine made its fastest gains since 2023 [src:osw-2026-wilk-starlink-cutoff] [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
4. **Logistics became the main target.**
   - 2022: HIMARS against depots [src:kyivindependent-2022-ponomarenko-himars].
   - 2024: long-range drones against arsenals such as Toropets and Tikhoretsk [src:twz-2024-altman-ammo-depots].
   - 2025: Russian drone units such as Rubikon hunting Ukrainian supply roads [src:kyivindependent-2025-farrell-kostiantynivka].
   - 2026: a Ukrainian "middle strike" / "Logistical Lockdown" campaign at 25–200 km depth [src:kyivindependent-2026-rushton-middle-strike].
   - Both sides now net their roads, move at night, and hand the last mile to ground robots (UGVs, Ukrainian *NRK*). Ukraine logged **25,143 UGV missions in August 2026 alone** [src:united24-2026-kabachynskyi-ugv-25000].

---

## 2. Per-era notes

### e1-invasion (Feb–Apr 2022)
- **Ukrainian ISR at night:** Aerorozvidka, then a volunteer IT-born drone unit, worked with about 30 special-forces troops on quad bikes. They had thermal drones and night-vision goggles and ambushed the Russian column north of Kyiv at night. Their first targets were the column's forward supply depot and its lead vehicles [src:defenseexpress-2022-aerorozvidka-convoy]. The unit's own claims were only partly verified.
- **Russian C2 failed badly:**
  - There was no time to exchange radio encryption keys, and troops were poorly trained on their radios. Many fell back on stolen civilian phones.
  - Ukraine was therefore able to monitor much of Russian tactical traffic. Only 10–20% of brigade/BTG radio traffic was about combat management; most of it was units reporting their own locations.
  - Units reused the same routes and staging areas and relied on a few main supply routes [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **Deception:** Ukraine photographed bomb damage at its airfields, printed the pattern onto covers, and brought aircraft back to "destroyed" sites. Dummy air-defence sites soaked up Russian munitions. Russian satellite intelligence missed 3–4 Ukrainian military trains a day in March 2022 [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **Starlink:** First shipments arrived on 28 February 2022, and 5,000 terminals had been delivered by 6 April [src:wikipedia-starlink-russo-ukrainian-war].

### e2-donbas-artillery (May–Aug 2022)
- **Russian logistics depended on rail.**
  - Fuel moved by rail tank car. From 1 to 19 April 2022, 228 tank cars (13,600+ t) went to Rovenky station alone.
  - Trains unloaded 30–50 km from the line of contact. Each "field artillery depot" was a converted civilian building supplying units within 30–50 km.
  - Russian doctrine put main support elements up to 50 km back. After Smerch and Tochka-U strikes, stocks were pulled out to 50 km and later 100 km [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **HIMARS/GMLRS:** RUSI calls their arrival "the point where the Russian offensive on Donbas ended" [src:rusi-2022-zabrodskyi-preliminary-lessons].
  - The Kyiv Independent reported that 8–12 launchers wrecked most depots in Donbas and the south within weeks.
  - Moving depots past the ~85 km range would roughly double delivery times. Russia had already lost at least 1,254 trucks and fuel tankers (Oryx count) [src:kyivindependent-2022-ponomarenko-himars].
- **Ukrainian artillery C2 before the invasion:**
  - Kropyva cut artillery deployment time by 80%, time to engage an unplanned target by two-thirds, and time to open counter-battery fire by 90% [src:rusi-2022-zabrodskyi-preliminary-lessons].
  - GIS Arta ("Uber for artillery") later claimed 30–45 seconds from target acquisition to fire mission. It ran over radio, cellular and Starlink [src:newamerica-2023-zikusoka-gis-arta].
- **Russian kill chain:** 3–5 minutes with a unit's own drone. Via a fire-control HQ, 20–30 minutes at tactical level and about 48 hours at operational level [src:rusi-2022-zabrodskyi-preliminary-lessons].

### e3-counteroffensives-22 (Sep–Nov 2022)
- **Operational deception worked:**
  - RUSI notes that Russian forces reliably committed resources against Ukraine's telegraphed moves and missed its concealed ones [src:rusi-2022-zabrodskyi-preliminary-lessons]. That is the pattern of the loudly advertised Kherson push versus the Kharkiv breakthrough.
  - Russian battle-damage assessment was weak. Troops were not encouraged to report failures, and data was fused poorly between sensors.
  - Ukraine used decoys of prestige systems and false radio traffic. Russian cruise missiles hit dummy HIMARS in August 2022 [src:rusi-2022-zabrodskyi-preliminary-lessons] [src:kyivpost-2023-decoys].
- **Russian HQs pushed back:** Ukrainian MLRS strikes in July 2022 drove Russian headquarters beyond 120 km from the front line. This caused serious friction through the autumn [src:rusi-2023-watling-meatgrinder].
- **Crimean Bridge (Kerch):** A truck bomb on 8 October 2022 dropped road spans and set fuel tank cars on fire. The road bridge was fully reopened only in February 2023 and the rail bridge in May 2023 [src:wikipedia-crimean-bridge].

### e4-bakhmut (Dec 2022–May 2023)
- **Russia refined its "reconnaissance-strike complex"** [src:rusi-2023-watling-meatgrinder]:
  - Kill chain: 3–5 minutes from drone detection to fires, 20–30 minutes from an electronic-warfare (EW) detection.
  - Strelets (a reconnaissance/fire-control data system) was widely used by VDV (airborne), Wagner and recon specialists.
  - EW: about one major system per 10 km of front, sited about 7 km behind the line. Ukrainian drone losses were about **10,000 per month**.
  - Russian EW reportedly decrypted Ukrainian Motorola traffic in near real time.
- **Russian C2 hardened:**
  - HQs dispersed and were wired to forward command posts (CPs), often through the captured Ukrainian civilian telecoms network.
  - Brigade CPs sat about 20 km back, underground.
  - Below battalion level, much traffic was still sent in the clear on analogue radios [src:rusi-2023-watling-meatgrinder].

### e5-counteroffensive-23 (Jun–Nov 2023)
- **Russian precision fires against Ukrainian columns** [src:rusi-2023-watling-stormbreak]:
  - Russia judged massed-fire doctrine unworkable, partly because "the logistics enabling such a volume of fire is too vulnerable to detection and long-range precision strike".
  - It shifted to precision: Krasnopol laser-guided shells designated by drones, and Lancets and FPVs flown with reconnaissance drones against lead elements.
- **Ukrainian gun crews** had to displace 2–15 minutes after opening fire [src:rusi-2023-watling-stormbreak].
- **Drone feeds drove command:** Only 3% of Ukrainian fire missions were smoke, because smoke blinds the drone feeds that brigade HQs use to manage the fight. Commanders preferred to keep their own picture over concealing their troops [src:rusi-2023-watling-stormbreak].
- **Decoys:** Metinvest began building decoys in August–September 2023. A decoy M777 costs about $1,000; the real gun costs about $4M, and the Russian missile sent against it costs $3–6M. Later decoys added heat sources and radio emitters [src:kyivpost-2023-decoys].
- **Crimean Bridge again:** Uncrewed boats struck it on 17 July 2023, and a span collapsed [src:wikipedia-crimean-bridge].

### e6-avdiivka-attrition (Oct 2023–Jul 2024)
- **Russia started using Starlink:** Ukrainian Defence Intelligence (HUR) confirmed Russian use in February 2024. The terminals were reportedly bought through third countries [src:wikipedia-starlink-russo-ukrainian-war].
- **RUSI fieldwork in 2024** described a mature "mass observation" battlefield [src:rusi-2025-watling-tactical-developments]:
  - Russian combat groups try to keep **five drone orbits** over each axis of advance.
  - Each orbit streams over a satellite uplink to battalion and brigade CPs, where the intelligence officer, fires officer and commander's representative authorise strikes.
  - Neither side generally uses fire controllers inside combat units.
- **Artillery shortage:** One Ukrainian brigade held 18 km of front with four working howitzers. Drones filled the gap and caused 60–70% of damaged or destroyed Russian systems. Even so, 60–80% of Ukrainian FPVs fail to reach their target [src:rusi-2025-watling-tactical-developments].
- **Decoy production grew** among volunteers and industry [src:euromaidanpress-2024-martyniuk-decoy-makers]:
  - Decoy Stugna anti-tank launchers cost $30–100.
  - Metinvest had made 250+ radar and howitzer decoys.
  - Good decoys copy radar, thermal and acoustic signatures and simulate firing or explosions.
  - The oft-repeated claim is that at least ten Kalibr missiles were spent on fake HIMARS.

### e7-kursk-pokrovsk (Aug 2024–Mar 2025)
- **Transparency figures** [src:rusi-2025-watling-tactical-developments]:
  - 3 km of full visibility, thinning to 15 km, with tasking possible to 40 km. Satellites cue drones beyond that.
  - In August 2024 Russia flew **1,000–1,500 Orlan/Zala orbits per day**.
  - Ukrainian brigades reported that **about 50% of their casualties are taken in the rear**, during rotation and resupply.
  - Units rotate less than once a month. Resupply is timed to good conditions rather than to consumption. Excavators rarely come within 7 km of the front.
- **The Kursk incursion (August 2024)** came after up to a month of preparatory intelligence collection and shaping fires, then a small two-point breach. Ukrainian assault detachments "rarely operate in groups above 20 personnel" [src:rusi-2025-watling-tactical-developments].
- **Delta formally adopted** across the defence forces in August 2024 [src:csis-2024-bondar-delta-cjadc2]:
  - It began in 2016 as an Aerorozvidka project, and the Ministry of Defence (MoD) took it over in 2023.
  - It links eight situational awareness centres and runs on cloud servers with Starlink connectivity.
  - Modules include Delta Monitor (600,000 enemy objects reported per month) and Vezha (4,000+ reconnaissance objects per day). Mission Control was already handling about 106,000 drone missions per month in 2024.
  - Delta is also being linked to Link 16 and to Polish TOPAZ artillery fire control.
- **Avengers AI** detected about 12,000 pieces of enemy equipment per week (MoD claim) [src:militarnyi-2024-avengers-12000].
- **Deep strike on logistics (September 2024):** Ukrainian drones hit the Toropets arsenal (about 30,000 t of munitions per TWZ) and Tikhoretsk (2,000+ t, including North Korean missiles). Maxar imagery confirmed the damage [src:twz-2024-altman-ammo-depots].
- **Russian countermeasures:**
  - Anti-drone net tunnels over roads, for example a 2+ km stretch between Bakhmut and Chasiv Yar in February 2025.
  - Drone-detector "traffic lights" along supply routes [src:defenseexpress-2025-russian-net-tunnels].
  - The Russian move to fibre-optic FPVs began to undercut EW-based protection [src:rusi-2025-watling-tactical-developments].
- **Satellite gap:** On 7 March 2025 the US suspended Ukraine's GEGD access to Maxar imagery during the intelligence-sharing pause [src:kyivindependent-2025-zadorozhnyy-maxar].

### e8-drone-kill-zone (Mar 2025–Jan 2026)
- **Rubikon strangles Ukrainian roads:**
  - Russia's Rubikon centre was sent to the Kostiantynivka area in April 2025 with long-range fibre-optic FPVs. Brigades had to rebuild their supply chains.
  - Net tunnels went up. Heavy bomber drones (*Kazhan*, "Bat") flew food, water, fuel and grenades around the clock, up to 20 runs per crew per day [src:kyivindependent-2025-farrell-kostiantynivka].
  - Rubikon grew from about 1,450 personnel (March 2025) to about 5,000 (spring 2026). Its stated mission includes hitting logistics and drone teams deep behind Ukrainian lines [src:fpri-2026-lee-putiata-rubicon].
- **Russian infiltration:** Assault groups shrank to 1–5 people creeping along treelines (*posadky*) [src:kyivindependent-2025-farrell-kostiantynivka]. Motorcycle groups ran 6–8 bikes (6–16 troops) with 2–3 EW sets and were also used for resupply and casevac. Ukrainian feedback says only about one in several groups survives [src:frontelligence-2025-motorcycle-assaults].
- **How Ukrainian attacks are now built** [src:rusi-2025-watling-combined-arms]:
  - FPVs work best from −3 to +3 km of the forward line.
  - UGVs evacuate wounded from 2–5 km out, at night, to medical posts 7+ km back.
  - Good units attack in seven phases over 5–10 days, taking 5–10% casualties against about 50% for uncoordinated attacks.
  - Russia systematically uses drones to find Ukrainian EW, radar, CPs and drone pilots.
- **MoD digital claims (September 2025):** Avengers classifies 70% of enemy vehicles in as little as 2.2 seconds, and Delta enables "up to 2,000 enemy assets neutralised per day" [src:mod-2025-delta-avengers-ecosystem].
- **Deep strike and counter-strike:**
  - Ukrainian refinery strikes peaked in August–October 2025 [src:carnegie-2026-vakulenko-refineries].
  - The SBU set off underwater charges at the Crimean Bridge piers on 3 June 2025 [src:wikipedia-crimean-bridge].
  - Russia hit Ukrainian rail almost 1,200 times in 2025, more than in 2023 and 2024 combined [src:kyivindependent-2026-hodunova-ukrzaliznytsia].
- **UGVs scale up:** 67 units used UGVs in November 2025 [src:united24-2026-kosoy-ugv-kill-zone]. Ukrainian road netting covered routes in five oblasts by October 2025 [src:euromaidanpress-2026-tril-net-tunnel-pace].

### e9-counteroffensive-26 (Feb 2026–present)
- **Starlink cutoff:**
  - SpaceX switched to a whitelist that dropped unregistered terminals. ISW, as relayed by Euromaidan Press, gives 1 February; OSW gives 5 February; United24 says the whitelist began in "early December 2025", which looks like an error [src:euromaidanpress-2026-zoria-isw-rubikon-starlink] [src:osw-2026-wilk-starlink-cutoff] [src:united24-2026-litnarovych-starlink-gains].
  - Ukrinform reports that earlier speed caps (75–90 km/h) had already degraded Russian drones that relied on Starlink [src:ukrinform-2026-starlink-c2].
  - Russian units fell back on radios, cables, Wi-Fi bridges and cellular links. Russia rushed Yamal/Express satellite terminals and mesh networks to the front [src:osw-2026-wilk-starlink-cutoff] [src:ukrinform-2026-starlink-c2].
  - Days later the Kremlin throttled Telegram, which units had used for coordination, and pushed the state-run MAX messenger [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
  - Rubikon stopped posting location details [src:euromaidanpress-2026-zoria-isw-rubikon-starlink].
  - Russian logistics units reportedly had to drop Starlink-linked UGVs and return to crewed trucks and motorcycles [src:futuradoctrina-2026-ryan-starlink-surprise].
- **Territorial effect:**
  - More than 200 km² in the first five days (Atlantic Council), or 201 km² in one week per ISW data [src:atlanticcouncil-2026-spencer-russian-comms-crisis] [src:united24-2026-litnarovych-starlink-gains].
  - Zelensky claimed 745 km² by 12 August 2026 [src:wikipedia-2026-ukrainian-counteroffensive].
  - Ukraine's own terminals had been pre-registered and were largely unaffected [src:osw-2026-wilk-starlink-cutoff].
- **Operation Vivaldi (Lyman, late May–September 2026):** ISW confirmed about 240 km² taken in phase one [src:wikipedia-2026-ukrainian-counteroffensive]. We found no detailed ISR/C2 sourcing for it (see Gaps).
- **Kill zone at 20–25+ km:**
  - Brovdi ("Magyar") puts it at 25+ km on both sides. He bases this on how regularly and densely strikes land, as tracked in the situational awareness system, not on the maximum range of individual drones [src:pravda-2026-levytska-magyar-kill-zone].
  - Brig. Gen. Lasiichuk (7th Air Assault Corps, Pokrovsk axis) gives 20–25 km now and expects 30 km by the end of 2026. Drones cause 70–80% of damage [src:euromaidanpress-2026-vivdych-kill-zone-corps].
  - The 107th TDF Brigade commander describes the distance troops must walk growing from 2 km to 3, 5, 10 and now 15+ km. Up to 20 km is "logistically difficult", and some trips to positions take three days [src:rbcukraine-2026-perch-kill-zone-foot].
  - On the Kramatorsk–Kostiantynivka road, vehicles run at up to 120 km/h under nets. Routine supply and rotations go on foot, and at least ten wrecked UGVs line the route [src:kyivindependent-2026-korovayny-road-kill-zone].
- **Ukrainian C2 at scale:**
  - Mission Control launched on 27 January 2026 as the national drone-management module inside Delta, replacing paper reports (forms 5.31/5.32) [src:mod-2026-mission-control-launch] [src:mod-2026-delta-6600-targets].
  - In June 2026 Delta logged 200,000+ drone strikes (6,600+ per day) and 75,000+ video streams per day. Confirmed strikes rose from 105,800 in January to 200,200 in June [src:mod-2026-delta-6600-targets].
  - The Avengers Labs dataset holds 5 million labelled images, and the auto-detector analyses 100,000+ drone streams per month [src:pravda-2026-levytska-avengers-labs].
- **UGVs** [src:united24-2026-kabachynskyi-ugv-25000] [src:united24-2026-kosoy-ugv-kill-zone]:
  - Monthly missions, January to August 2026: 7,511 → 7,960 → 9,072 → 11,028 → 14,059 → 16,664 → 19,940 → **25,143**, for 111,000+ in total.
  - Units using UGVs: 117 (2025) → 230 (2026). A different count gives 67 (November 2025) → 167 (March 2026).
  - Plan: 25,000 UGVs contracted in the first half of 2026. One robot carries about 500 kg, the load of about 10 soldiers.
- **Road netting:** 1,066 km built between January and July 2026, reaching 9.2 km/day in June, with a target of 4,000 km by the end of 2026. Euromaidan Press says Russian reconnaissance and FPV drones work 15–30 km deep [src:euromaidanpress-2026-mukhina-net-tunnels-1000km] [src:euromaidanpress-2026-tril-net-tunnel-pace].
- **"Middle strike" / "Logistical Lockdown"** [src:kyivindependent-2026-rushton-middle-strike] [src:jamestown-2026-lapaiev-mid-range-drones] [src:atlanticcouncil-2026-kirichenko-drones-logistics]:
  - Ukrainian drones hit targets 25–200 km deep: fuel trucks, convoys and the R-280 "Novorossiya" highway to Crimea.
  - Geolocated strikes rose from 55 (April) to 130+ (May). 125 trucks were hit and 80+ destroyed. There were fuel shortages in Sevastopol and Melitopol, and Russia restricted traffic on the R-280 by decree on 21 May.
  - Russia responded with camouflage, decoys, mobile fire teams and road netting.
- **Russian strikes on Ukrainian rail:** 209 locomotives, 86 rail bridges and 50 stations were damaged in 2025 and the first quarter of 2026. In frontline oblasts, buses are replacing trains on some routes [src:kyivindependent-2026-hodunova-ukrzaliznytsia].
- **Refineries:** Russian output fell about 13% in April–May 2026, with a possible 28% hit after mid-June strikes on the Moscow area. There were shortages in Crimea and the occupied territories [src:carnegie-2026-vakulenko-refineries].

---

## 3. Kill-zone / transparency table

| era | typical kill-zone depth | what moves where, and how | source id |
|---|---|---|---|
| e1-invasion | No continuous zone. Ambushes along roads; Russian depots within 50 km in doctrine | BTG columns on a few main supply routes; Ukrainian drone and special-forces teams on quad bikes strike at night with thermals | rusi-2022-zabrodskyi-preliminary-lessons; defenseexpress-2022-aerorozvidka-convoy |
| e2-donbas-artillery | Artillery range, about 20–40 km; HIMARS reaches about 80–85 km | Russian rail railheads 30–50 km out; depots pushed to 50 then 100+ km; trucks shuttle the last leg | rusi-2022-zabrodskyi-preliminary-lessons; kyivindependent-2022-ponomarenko-himars |
| e3-counteroffensives-22 | Same, plus HQ displacement | Russian HQs beyond 120 km after July 2022 MLRS strikes; concealed Ukrainian concentration at Kharkiv | rusi-2023-watling-meatgrinder; rusi-2022-zabrodskyi-preliminary-lessons |
| e4-bakhmut | Drone-cued artillery at 3–5 min; glide bombs to about 70 km | Russian brigade CPs about 20 km back underground; EW sites about 7 km back; wired links | rusi-2023-watling-meatgrinder |
| e5-counteroffensive-23 | Minefields plus drone-designated precision fires on breach lanes | Ukrainian armoured columns stopped in minefields; guns displace 2–15 min after firing | rusi-2023-watling-stormbreak |
| e6-avdiivka-attrition | Full visibility within about 3 km; FPV threat growing | Rotations at night and in bad weather, more than a month apart; HMMWV/M113 dashes for ammo | rusi-2025-watling-tactical-developments |
| e7-kursk-pokrovsk | Full visibility 3 km, fading to 15 km; tasking to 40 km | About 50% of Ukrainian casualties in the rear; drone drops to positions; vehicles dug in within 3 km; Russians net roads | rusi-2025-watling-tactical-developments; defenseexpress-2025-russian-net-tunnels |
| e8-drone-kill-zone | About 10–20 km; fibre-optic FPVs reach supply roads | Foot movement over 5–10+ km; bomber-drone resupply; UGV casevac from 2–5 km; net tunnels; Russian 1–5 person infiltration | kyivindependent-2025-farrell-kostiantynivka; rusi-2025-watling-combined-arms |
| e9-counteroffensive-26 | 20–25+ km on both sides (30 km expected); Russian drones to 15–30 km; Ukrainian middle strike 25–200 km | Troops walk 15+ km (up to 3 days); 25k UGV missions a month; 1,000+ km of nets; vehicles at 120 km/h under nets; Russian R-280 traffic restricted | pravda-2026-levytska-magyar-kill-zone; euromaidanpress-2026-vivdych-kill-zone-corps; rbcukraine-2026-perch-kill-zone-foot; kyivindependent-2026-rushton-middle-strike |

---

## 4. Key-figures table

| figure | value | unit | era | source id |
|---|---|---|---|---|
| Russian tactical radio traffic that was combat management (March 2022) | 10–20 | % | e1-invasion | rusi-2022-zabrodskyi-preliminary-lessons |
| Russian detection→fires, own UAV / via fire-control HQ / operational | 3–5 / 20–30 / ~48 h | minutes | e2-donbas-artillery | rusi-2022-zabrodskyi-preliminary-lessons |
| Kropyva: cut in artillery deployment time / counter-battery response time | 80 / 90 | % | e2-donbas-artillery | rusi-2022-zabrodskyi-preliminary-lessons |
| GIS Arta target acquisition → fire mission | 30–45 | seconds | e2-donbas-artillery | newamerica-2023-zikusoka-gis-arta |
| Fuel moved by rail to Rovenky, 1–19 April 2022 | 228 tank cars / 13,600+ | tonnes | e1-invasion | rusi-2022-zabrodskyi-preliminary-lessons |
| Russian trucks and fuel tankers lost by July 2022 (Oryx) | 1,254+ | vehicles | e2-donbas-artillery | kyivindependent-2022-ponomarenko-himars |
| Russian HQ distance after July 2022 MLRS strikes | >120 | km | e3-counteroffensives-22 | rusi-2023-watling-meatgrinder |
| Ukrainian UAV losses (2023) | ~10,000 | per month | e4-bakhmut | rusi-2023-watling-meatgrinder |
| Russian EW density | 1 major system per 10 km, ~7 km back | — | e4-bakhmut | rusi-2023-watling-meatgrinder |
| Ukrainian howitzer displacement time after firing | 2–15 | minutes | e5-counteroffensive-23 | rusi-2023-watling-stormbreak |
| Decoy M777 vs real M777 | ~$1,000 vs ~$4M | USD | e5-counteroffensive-23 | kyivpost-2023-decoys |
| Russian Orlan/Zala orbits over Ukrainian positions (Aug 2024) | 1,000–1,500 | per day | e7-kursk-pokrovsk | rusi-2025-watling-tactical-developments |
| Share of Ukrainian brigade casualties taken in the rear | ~50 | % | e7-kursk-pokrovsk | rusi-2025-watling-tactical-developments |
| Ukrainian FPVs failing to reach target | 60–80 | % | e7-kursk-pokrovsk | rusi-2025-watling-tactical-developments |
| Avengers AI detections (Sept 2024) | ~12,000 | per week | e7-kursk-pokrovsk | militarnyi-2024-avengers-12000 |
| Toropets / Tikhoretsk munitions destroyed | ~30,000 / 2,000+ | tonnes | e7-kursk-pokrovsk | twz-2024-altman-ammo-depots |
| Heavy bomber-drone resupply runs | up to 20 | per crew per day | e8-drone-kill-zone | kyivindependent-2025-farrell-kostiantynivka |
| Avengers time to classify a vehicle | 2.2 | seconds | e8-drone-kill-zone | mod-2025-delta-avengers-ecosystem |
| Russian attacks on Ukrainian rail (2025) | ~1,200 | attacks | e8-drone-kill-zone | kyivindependent-2026-hodunova-ukrzaliznytsia |
| Rubikon personnel, March 2025 → spring 2026 | ~1,450 → ~5,000 | people | e8-drone-kill-zone | fpri-2026-lee-putiata-rubicon |
| Russian Starlink terminals smuggled (analyst claim) | 50,000+ | terminals | e9-counteroffensive-26 | ukrinform-2026-starlink-c2 |
| Ukrainian gains in the first week after the Starlink cutoff | 200–201 | km² | e9-counteroffensive-26 | atlanticcouncil-2026-spencer-russian-comms-crisis; united24-2026-litnarovych-starlink-gains |
| Kill-zone depth (Brovdi, May 2026) | 25+ | km each side | e9-counteroffensive-26 | pravda-2026-levytska-magyar-kill-zone |
| Kill-zone depth (Lasiichuk, July 2026) | 20–25 (→30 by end-2026) | km | e9-counteroffensive-26 | euromaidanpress-2026-vivdych-kill-zone-corps |
| Distance walked to positions | 15+ (up to 3 days) | km | e9-counteroffensive-26 | rbcukraine-2026-perch-kill-zone-foot |
| Delta-logged drone strikes (June 2026) | 200,000+ (6,600+/day) | strikes | e9-counteroffensive-26 | mod-2026-delta-6600-targets |
| Video streams through Delta | 75,000+ | per day | e9-counteroffensive-26 | mod-2026-delta-6600-targets |
| UGV missions (August 2026) | 25,143 | per month | e9-counteroffensive-26 | united24-2026-kabachynskyi-ugv-25000 |
| UGV missions (January–August 2026) | 111,377+ | missions | e9-counteroffensive-26 | united24-2026-kabachynskyi-ugv-25000 |
| UGV payload | ~500 | kg (≈10 soldiers' loads) | e9-counteroffensive-26 | united24-2026-kosoy-ugv-kill-zone |
| Anti-drone road nets built (January–July 2026) | 1,066 | km | e9-counteroffensive-26 | euromaidanpress-2026-mukhina-net-tunnels-1000km |
| Net-building target (end 2026) | 4,000 | km | e9-counteroffensive-26 | euromaidanpress-2026-mukhina-net-tunnels-1000km |
| Middle-strike geolocated strikes, April → May 2026 | 55 → 130+ | strikes | e9-counteroffensive-26 | kyivindependent-2026-rushton-middle-strike |
| Russian trucks hit / destroyed on key routes | 125 / 80+ | vehicles | e9-counteroffensive-26 | kyivindependent-2026-rushton-middle-strike |
| Locomotives damaged (2025 to Q1 2026) | 209 | locomotives | e9-counteroffensive-26 | kyivindependent-2026-hodunova-ukrzaliznytsia |
| Russian refinery output cut (April–May 2026) | ~13 (to ~28 after mid-June) | % | e9-counteroffensive-26 | carnegie-2026-vakulenko-refineries |

---

## 5. Russia's side as reported

**C2.**
- **2022:** Russia went in with insecure analogue radios, missing encryption keys and stolen civilian phones. Traffic was mostly units reporting their positions, which Ukraine exploited [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **2023 fixes** [src:rusi-2023-watling-meatgrinder]:
  - Higher HQs were dispersed and linked by microwave links, relay vehicles and field cable, often tied into captured Ukrainian civilian telecoms.
  - Strelets was used for reconnaissance and fire correction by elite units.
  - Below battalion level, traffic stayed mostly in the clear.
- **Structure:** Russian C2 is top-down. Drones are kept over Russian forces for "combat management", and horizontal coordination between neighbouring units is weak [src:rusi-2025-watling-tactical-developments] [src:rusi-2023-watling-meatgrinder].
- **2024–26 dependencies:** Smuggled Starlink terminals (bought via Dubai and Central Asia; one analyst claims 50,000+) and Telegram for coordination and fundraising [src:wikipedia-starlink-russo-ukrainian-war] [src:ukrinform-2026-starlink-c2] [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
- **February 2026 losses:**
  - The whitelist removed Starlink, then the Kremlin throttled Telegram.
  - Replacements were Yamal/Express satellite terminals, Gazprom Space Systems ("far less reliable"), mesh networks, radios and cable [src:osw-2026-wilk-starlink-cutoff] [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
  - Reported side effects: friendly fire (12 killed in one Zaporizhzhia incident) and attempts to get Ukrainians to register terminals for them [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
  - A Ukrainian brigade commander estimated Russia would need about six months to adapt [src:euromaidanpress-2026-zoria-isw-rubikon-starlink].
  - Wikipedia notes a Russian "Volna/Kupol Garant" jamming system announced in August 2026. This is unverified [src:wikipedia-starlink-russo-ukrainian-war].

**ISR.**
- Orlan-10 and Zala drones at operational depth; five orbits over each axis; Lancet and FPVs doubling as scouts [src:rusi-2025-watling-tactical-developments].
- Rubikon is Russia's centralised drone centre for new tactics and deep interdiction. By 2026 it had 17 detachments of about 474 people each, with FPV, fixed-wing FPV, Lancet/Supercam, Orlan and counter-UAS/EW teams [src:fpri-2026-lee-putiata-rubicon].

**Logistics.**
- **Rail-bound:** Railheads sit 30–50 km from the front, with depots in converted buildings serving a 30–50 km radius. Railway Troops rebuild bridges, for example the pontoon rail bridge at Kupiansk [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **Chronic truck shortage** [src:kyivindependent-2022-ponomarenko-himars].
- **Adaptations:**
  - Dispersed ammunition caches near the guns [src:rusi-2025-watling-tactical-developments].
  - Road net tunnels and drone-warning "traffic lights" [src:defenseexpress-2025-russian-net-tunnels].
  - Motorcycles, buggies and light vehicles for the last mile [src:frontelligence-2025-motorcycle-assaults].
- **In 2026** Russia faced the Ukrainian middle-strike campaign on the R-280 and routes into Crimea. Reported effects: driver shortages, higher shipping costs, fuel rationing, R-280 traffic restrictions, and depots moved deeper into Russia [src:kyivindependent-2026-rushton-middle-strike] [src:jamestown-2026-lapaiev-mid-range-drones]. The Crimean Bridge was hit in 2022, 2023 and 2025 but always reopened [src:wikipedia-crimean-bridge].

**Strikes on Ukrainian logistics.**
- Geran/Shahed and Gerbera drones fly in waves against airfields, training areas, transformer substations and industry [src:rusi-2025-watling-tactical-developments].
- The rail campaign targets locomotives and bridges [src:kyivindependent-2026-hodunova-ukrzaliznytsia].
- Rubikon-style FPV interdiction of roads reaches 15–30 km [src:euromaidanpress-2026-mukhina-net-tunnels-1000km].

---

## 6. Game/sim relevance

- **Visibility is a gradient, not fog of war.** Model detection probability as a function of distance from the line and sensor density:
  - near-certain within about 3 km;
  - falling off to 15–25 km;
  - "tasked only" beyond 40 km, with satellites cueing longer-range drones.

  Night, fog and rain reduce drone effectiveness; this is when rotations and infiltration happen. Scale the depth up by era, from about 3 km (2023) to 25+ km (2026). Sources: rusi-2025-watling-tactical-developments, pravda-2026-levytska-magyar-kill-zone.
- **Detection → strike latency as a core stat.**
  - About 3–5 minutes with an organic drone; 20–30 minutes through an HQ; hours to days at operational level (2022–23).
  - Ukrainian digital tools (GIS Arta at 30–45 s, Avengers detection at 2.2 s) shorten the loop.
  - Loss of a C2 bearer (Starlink-cutoff event, EW) lengthens it and can cause friendly fire.

  Latency, together with guns displacing within 2–15 minutes, gives a natural "shoot and scoot" mechanic.
- **Supply reliability model.** Treat each leg of a supply route as a hazard.
  - Rear leg: rail or trucks, exposed to deep strikes.
  - Mid leg: nets, night movement, fast vehicles.
  - Last mile: foot, bomber drones, UGVs.

  A delivery succeeds with some probability that depends on the kill-zone depth, net coverage, weather and UGV availability. Ukrainian UGV missions (7.5k → 25k per month in 2026) and net kilometres should be buildable upgrades. Units should resupply when conditions allow, not when they run low.
- **Concentration penalty.** The bigger and slower a formation in the kill zone, the higher its strike probability. That pushes the player toward small assault groups (20 or fewer), dispersion, decoys and reversionary positions. Russian AI behaviour should shift from BTG columns (e1) to 1–5 person infiltration and motorcycle swarms (e8–e9).
- **C2 as a network with single points of failure.** Starlink, Telegram and HQ nodes can be cut by events, deep strikes or EW. The February 2026 whitelist is a scripted event that sharply lowers Russian coordination and gives the player a counteroffensive window.
- **Deception mechanics.** Decoys are cheap (about $30–1,000) and pull expensive enemy munitions. Weak Russian battle-damage assessment means "destroyed" can be spoofed, and telegraphed moves draw reserves away from concealed ones.

---

## 7. Terms

- **nul / na nuli** (нуль, "zero" / "at the zero") — the zero line: the most forward positions.
- **sira zona** (сіра зона) — grey zone: contested ground between the lines. In 2025–26 it is increasingly used as a synonym for the kill zone.
- **kilzona** (кілзона) — kill zone: the belt where drones reliably strike anything that moves. It is measured by strike density, not the maximum range of a drone.
- **posadka** (посадка, plural *posadky*) — tree line or windbreak between fields; the main cover for infantry and for infiltration.
- **blindazh** (бліндаж) — dugout or covered bunker.
- **NRK** (НРК, *nazemnyi robotyzovanyi kompleks*) — ground robotic system, i.e. a UGV.
- **BpLA** (БпЛА) — uncrewed aerial vehicle (UAV).
- **REB** (РЕБ, *radioelektronna borotba*) — electronic warfare (EW), especially jammers.
- **RER** (РЕР, *radioelektronna rozvidka*) — radio-electronic reconnaissance; SIGINT/ELINT and direction finding.
- **skyd** (скид) — "drop": a munition dropped from a drone; *skydy* are bomber-drone strikes.
- **optovolokno** (оптоволокно) — fibre optic: FPVs guided through a thin cable that cannot be jammed.
- **ptashka** (пташка, "birdie") — slang for a drone.
- **mavik** — generic slang for a small quadcopter used for reconnaissance or drops, after the DJI Mavic.
- **Kazhan** (Кажан, "Bat") / **Baba Yaha** — heavy multirotor bomber drones, also used for resupply drops.
- **moped** (мопед) — Ukrainian slang for a Shahed/Geran drone, from the sound of its engine.
- **evak** (евак) — casualty evacuation.
- **maket** (макет) — decoy or mockup, e.g. a fake HIMARS or M777.
- **Kropyva** (Кропива, "nettle") — Ukrainian artillery and mapping app (from 2014).
- **GIS Arta** — Ukrainian "Uber for artillery" fire-allocation software.
- **Delta** — Ukrainian cloud-based situational awareness and battle-management system.
- **Vezha** (Вежа, "tower") — Delta's video-analysis platform.
- **Avengers** — MoD AI platform that detects targets in drone and camera video.
- **Mission Control** — Delta's drone-mission planning and reporting module.
- **ePoints / Army of Drones Bonus** — scheme that awards points to units for confirmed strikes.
- **Strelets** — Russian reconnaissance, fire-control and communications data system.
- **RFC / reconnaissance-fire circuit** — Russian term for the sensor-to-shooter kill chain.
- **middle strike** — Ukrainian drone strikes at about 25–200 km depth, against logistics, air defence and command posts.
- **Logistical Lockdown** — Fedorov's 2026 programme to scale up middle-strike interdiction.
- **Rubikon / Rubicon** — Russia's elite Centre for Advanced Unmanned Technologies.
- **GEGD** — the US government programme that gave Ukraine access to commercial Maxar imagery; suspended in March 2025.

---

## 8. Open questions and gaps

- **Operation Vivaldi ISR/C2 detail.** The sources confirm the operation and its roughly 240 km² phase-one gains. They say nothing about how drone overwatch, the Russian loss of Starlink or deception were used there. The web-search budget ran out before targeted searches were possible.
- **Russian C2 after the Starlink cutoff (March–September 2026).** How far did Yamal/Express terminals, mesh networks and the claimed "Kupol" system restore Russian drone and C2 performance? Was Ukraine's advantage temporary? We found no systematic assessment.
- **Jet-powered Gerans and interceptor drones.** Not covered here in depth; probably in the air-defence topic. Their effect on Ukrainian rail and energy logistics in 2026 needs a figure.
- **Kill-zone depth varies by sector.** The estimates (15, 20–25, 25+, 15–30 km) come from different commanders and methods. No dataset exists; Delta strike-density data is not public.
- **UGV loss and success rates.** We have mission counts but not the share that fail. The Kyiv Independent saw at least ten wrecked UGVs on one road [src:kyivindependent-2026-korovayny-road-kill-zone]. The "90% of a regiment's deliveries by UGV" claim appears only in secondary drone-news coverage and is not verified.
- **Detection-to-strike latency in 2025–26.** The latency figures we have are from 2022–23 (RUSI) and from vendor/MoD claims (GIS Arta, Avengers). There is no independent measurement for the drone era.
- **Satellite ISR after March 2025.** European and commercial alternatives, such as ICEYE SAR, were not researched here.
- **Figures that conflict:**
  - Starlink whitelist date: 1 February (ISW via EMP), 5 February (OSW), "early December 2025" (United24).
  - UGV user-unit counts differ between two United24 articles.
  - Jamestown converts 5 billion hryvnia to "$1.12 million". It is about $120 million, so the source has a conversion error.
- **Russian logistics metrics in 2025–26** (rail versus truck share, depot distances) are thin in Western and Ukrainian open sources.

---

## 9. Sources

Key sources are marked ★.

- ★ rusi-2022-zabrodskyi-preliminary-lessons — RUSI, based on General Staff data: Russian C2 failures, rail logistics, Kropyva, kill-chain times, deception.
- defenseexpress-2022-aerorozvidka-convoy — Aerorozvidka's night thermal-drone ambushes on the Kyiv column; claims only partly verified.
- kyivindependent-2022-ponomarenko-himars — why Russian rail/truck logistics were vulnerable to HIMARS in 2022.
- newamerica-2023-zikusoka-gis-arta — GIS Arta "Uber for artillery", 30–45 s claim, Starlink bearer.
- ★ rusi-2023-watling-meatgrinder — Russian recon-fire circuits, Strelets, EW density, UAV losses, HQ displacement and wired C2.
- rusi-2023-watling-stormbreak — 2023 offensive: Russian precision fires, gun displacement times, drone feeds driving command.
- kyivpost-2023-decoys — Metinvest decoys, costs, thermal and radio signatures.
- euromaidanpress-2024-martyniuk-decoy-makers — volunteer and industry decoy makers; the signatures decoys must copy.
- militarnyi-2024-avengers-12000 — MoD claim: Avengers detects 12,000 pieces of equipment per week.
- twz-2024-altman-ammo-depots — Maxar imagery of the Toropets and Tikhoretsk depot strikes.
- ★ csis-2024-bondar-delta-cjadc2 — the best overview of Delta's architecture and modules.
- ★ rusi-2025-watling-tactical-developments — the 3/15/40 km transparency figures, rear casualties, resupply and rotation practice.
- defenseexpress-2025-russian-net-tunnels — Russian road net tunnels near Bakhmut and Chasiv Yar.
- kyivindependent-2025-zadorozhnyy-maxar — US suspension of Ukraine's GEGD access to Maxar imagery, March 2025.
- frontelligence-2025-motorcycle-assaults — Ukrainian OSINT on Russian motorcycle assault and logistics groups.
- ★ kyivindependent-2025-farrell-kostiantynivka — Rubikon's effect on Ukrainian logistics; bomber-drone resupply; infiltration.
- mod-2025-delta-avengers-ecosystem — MoD claims: Avengers 2.2 s / 70%; Delta 2,000 targets per day.
- ★ rusi-2025-watling-combined-arms — FPV range band, UGV casevac, phased attacks, Russian hunting of C2/EW.
- mod-2026-mission-control-launch — launch of the national Mission Control module (January 2026).
- ★ osw-2026-wilk-starlink-cutoff — Starlink whitelist mechanics and Russian substitutes.
- ukrinform-2026-starlink-c2 — Ukrainian expert views on the Russian C2 collapse; 50k-terminal smuggling claim.
- euromaidanpress-2026-zoria-isw-rubikon-starlink — ISW: the Starlink block degrades Rubikon.
- futuradoctrina-2026-ryan-starlink-surprise — Mick Ryan: Russian uses of Starlink, including UGVs; lessons about surprise.
- united24-2026-litnarovych-starlink-gains — 201 km² in one week per ISW data; contains a date error.
- ★ atlanticcouncil-2026-spencer-russian-comms-crisis — Starlink cut plus Telegram throttling; MAX; friendly fire.
- euromaidanpress-2026-tril-net-tunnel-pace — pace and budget of Ukrainian net-tunnel building in early 2026.
- kyivindependent-2026-hodunova-ukrzaliznytsia — scale of the Russian campaign against Ukrainian rail.
- united24-2026-kosoy-ugv-kill-zone — UGV procurement, payloads, unit counts.
- ★ pravda-2026-levytska-magyar-kill-zone — Brovdi (Magyar): kill zone 25+ km, defined by strike density.
- ★ kyivindependent-2026-rushton-middle-strike — the middle-strike campaign by numbers; "Logistical Lockdown".
- kyivindependent-2026-korovayny-road-kill-zone — on the ground on a netted supply road, June 2026.
- jamestown-2026-lapaiev-mid-range-drones — mid-range drones against the R-280; effects on Russian logistics.
- fpri-2026-lee-putiata-rubicon — Rubikon's structure and growth from documents.
- atlanticcouncil-2026-kirichenko-drones-logistics — Ukrainian drone interdiction in the south; Russian countermeasures.
- carnegie-2026-vakulenko-refineries — Ukrainian refinery strikes and Russian output, 2025–26.
- euromaidanpress-2026-vivdych-kill-zone-corps — 7th Corps commander: 20–25 km now, 30 km expected.
- euromaidanpress-2026-mukhina-net-tunnels-1000km — 1,066 km of nets; Russian drones reach 15–30 km.
- ★ mod-2026-delta-6600-targets — Delta statistics for June 2026: strikes, streams, map objects.
- pravda-2026-levytska-avengers-labs — Avengers Labs dataset; 100k+ streams per month.
- rbcukraine-2026-perch-kill-zone-foot — 107th TDF Brigade: distance on foot grew from 2 km to 15+ km.
- ★ united24-2026-kabachynskyi-ugv-25000 — monthly UGV mission series for 2026.
- wikipedia-starlink-russo-ukrainian-war — timeline of Starlink use on both sides (navigation only).
- wikipedia-crimean-bridge — dates of the 2022, 2023 and 2025 bridge attacks (navigation only).
- wikipedia-2026-ukrainian-counteroffensive — axes of the 2026 counteroffensive, Vivaldi, territory claims (navigation only).
