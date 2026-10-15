<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

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
   - That fragility was exposed in **February 2026**. A speed cap (late January), then SpaceX's whitelist (enforced 5 February), cut off unregistered Starlink terminals. Russian front-line C2 broke down, and Ukraine made its fastest gains since 2023 [src:mod-2026-starlink-whitelist-operational] [src:osw-2026-wilk-starlink-cutoff] [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
   - Russia recovered partially over the summer through mesh radios, other satellites, relays and laundered whitelist registrations. The edge narrowed but did not close [src:isw-2026-03-18-roca] [src:euromaidanpress-2026-axe-quiet-counteroffensive].
4. **Logistics became the main target.**
   - 2022: HIMARS against depots [src:kyivindependent-2022-ponomarenko-himars].
   - 2024: long-range drones against arsenals such as Toropets and Tikhoretsk [src:twz-2024-altman-ammo-depots].
   - 2025: Russian drone units such as Rubikon hunting Ukrainian supply roads [src:kyivindependent-2025-farrell-kostiantynivka].
   - 2026: a Ukrainian "middle strike" / "Logistics Lockdown" campaign at 20–200 km depth. It cut M-14 freight by about 71% and caused fuel rationing in Crimea [src:kyivindependent-2026-rushton-middle-strike] [src:euromaidanpress-2026-crimea-island].
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
- **Commercial SAR:** The "People's Satellite" was bought from $13.5 million crowdfunded in three days in June 2022 (the "People's Bayraktar" money, redirected when Baykar donated the drones). The ICEYE contract signed on 18 August 2022 gives one dedicated SAR satellite plus tasking across the constellation. Imagery reached HUR from September; it sees through cloud and at night, at up to 0.25 m resolution [src:euromaidanpress-2026-peoples-satellite].
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
- **Satellite gap:** On 7 March 2025 the US suspended Ukraine's GEGD access to Maxar imagery during the intelligence-sharing pause [src:kyivindependent-2025-zadorozhnyy-maxar]. The pause lasted about a week. Intelligence sharing resumed after the 11 March US–Ukraine meeting in Jeddah, and DNI Gabbard confirmed its end [src:euromaidanpress-2025-us-resumes-intel].

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
- **UGVs scale up:** 67 units used UGVs in November 2025, flying 2,900+ missions [src:mod-2026-ugv-9000-march]. Ukrainian road netting covered routes in five oblasts by October 2025 [src:euromaidanpress-2026-tril-net-tunnel-pace].
- **Satellite ISR diversifies after the March 2025 scare** [src:euromaidanpress-2026-ft-europe-intel] [src:euromaidanpress-2026-peoples-satellite]:
  - HUR runs its own platform for French CSO-3 imagery, under a deal signed in February 2025.
  - Japan shares SAR imagery with HUR, its first such sharing with any foreign state. Germany and Italy provide radar imagery.
  - NATO's APSS ("Aquila") pools data from 18 nations' satellites.
  - ICEYE (54 SAR satellites) has become the linchpin. A German-funded contract has supplied SAR data to the MoD since November 2024, and Rheinmetall–ICEYE plan to build satellites in Germany.
  - The crowdfunded "People's Satellite" had delivered 5,900+ SAR images to HUR by May 2026.
  - Budanov still said in December 2025 that Kyiv "remained critically dependent" on US satellite imagery and early warning. Macron's January 2026 claim that France supplies two-thirds of Ukraine's intelligence is disputed.
- **Russia's C2 software races to shorten its kill chain** [src:csis-2026-bondar-russian-c2]:
  - Unmanned systems carry out up to 80% of Russian fire missions.
  - The volunteer-built Glaz/Groza complex turns a one-click mark on a drone feed into a full fire-mission package for guns and mortars. It cuts detection-to-impact from hours to minutes.
  - Belousov announced the "Svod" tactical situational-awareness complex in August 2025, and an MoD council on a unified drone-management system met in September 2025. Earlier, a spyware-laced copy of the AlpineQuest mapping app had leaked Russian unit locations (early 2025).
- **Starlink on Russian mid-range drones:** from late December 2025, Rubikon and other units flew Starlink-equipped BM-35 and Molniya-2 drones deep into the Ukrainian rear. They hit trains and traffic on the E-50 highway, much like the battlefield air interdiction (BAI) that enabled their advances at Pokrovsk and Huliaipole [src:isw-2026-02-01-roca] [src:isw-2026-02-05-roca].

### e9-counteroffensive-26 (Feb 2026–present)
- **Starlink cutoff, in phases.** The dates in the sources differ because they describe different steps:
  - *Trigger (late December 2025 – January 2026):* Rubikon and other units had fitted Starlink to BM-35 and Molniya-2 drones for strikes deep in the Ukrainian rear, against trains and vehicles on the E-50 Pokrovsk–Pavlohrad highway [src:isw-2026-02-01-roca] [src:isw-2026-02-05-roca]. On 29 January Fedorov said the MoD had contacted SpaceX within hours of Starlink-guided Russian drones appearing over Ukrainian cities [src:mod-2026-spacex-starlink-russian-uavs].
  - *First step (late January – 1 February):* a speed-based cutoff for terminals moving faster than about 75–90 km/h, which hit Starlink-equipped Shaheds and Molniyas [src:twz-2026-altman-rogoway-starlink-scramble] [src:isw-2026-02-01-roca]. On 1 February Musk wrote on X that the steps had worked, and Fedorov announced that only authorised terminals would be allowed [src:mod-2026-starlink-authorization-system].
  - *Registration (2 February):* a Cabinet resolution set up the whitelist. Civilians register at administrative service centres, businesses via Diia and the military via Delta [src:kyivindependent-2026-myronyshena-starlink-whitelist].
  - *Enforcement (5 February):* unregistered terminals went dark at about 03:00 Kyiv time. The same day Fedorov said Russian terminals were blocked and the whitelist was being updated once a day [src:kyivindependent-2026-starlink-catastrophe] [src:mod-2026-starlink-whitelist-operational].
  - ISW and outlets relaying it date the block to "1 February", meaning the first step. OSW and the Kyiv Independent date it to 5 February, meaning enforcement. United24's "early December" is an error [src:isw-2026-02-10-roca] [src:osw-2026-wilk-starlink-cutoff] [src:united24-2026-litnarovych-starlink-gains].
- **First effects:**
  - The General Staff reported fewer Russian assaults, and none at all in some sectors. Brigades reported fewer FPV sorties and fewer Molniya strikes on the rear [src:isw-2026-02-05-roca] [src:isw-2026-02-06-roca].
  - Russian units fell back on radios, cables, Wi-Fi bridges and cellular links, then brought in Yamal/Express satellite terminals and mesh networks [src:osw-2026-wilk-starlink-cutoff] [src:ukrinform-2026-starlink-c2].
  - Roskomnadzor throttled Telegram on 9–10 February, although units used it for horizontal coordination and air-defence cueing. Peskov said he could not imagine troops using messengers at the front, and milbloggers called that a lie [src:isw-2026-02-10-roca] [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
  - Rubikon stopped posting location details [src:euromaidanpress-2026-zoria-isw-rubikon-starlink]. Russian logistics units reportedly dropped Starlink-linked UGVs for crewed trucks and motorcycles [src:futuradoctrina-2026-ryan-starlink-surprise].
  - On 26 February Brig. Gen. Biletskyi (3rd Army Corps) said the cutoff had cut Russian drone effectiveness by roughly 20–40% in two weeks. He expected a partial recovery through other satellite links within one to two months, but not a return to Starlink-level efficiency for three to five years [src:isw-2026-feb-26-assessment].
- **Russian recovery, March–September:** partial. See §5 for the details [src:isw-2026-03-18-roca].
- **Territorial effect:**
  - More than 200 km² in the first five days (Atlantic Council), or 201 km² in one week per ISW data [src:atlanticcouncil-2026-spencer-russian-comms-crisis] [src:united24-2026-litnarovych-starlink-gains].
  - Zelensky claimed 745 km² by 12 August 2026 [src:wikipedia-2026-ukrainian-counteroffensive].
  - Ukraine's own terminals had been pre-registered and were largely unaffected [src:osw-2026-wilk-starlink-cutoff].
- **Operation Vivaldi (north of Lyman, end of May–September 2026)** is the best-documented Ukrainian ISR/C2 operation of the year:
  - *Secrecy:* the 3rd Army Corps attacked the Russian salient north of Lyman from about the end of May under an information blackout. Its commander, Biletskyi, confirmed the operation only on 15 September [src:hromadske-2026-lisova-vivaldi-phase1] [src:wikipedia-2026-ukrainian-counteroffensive]. SSO, border-guard and HUR units fought under the same tactical command [src:euromaidanpress-2026-zoria-vivaldi-targeting].
  - *Area:* the corps claims 75 km² cleared of infiltrators and 10 km² de-occupied in "season" one, then 40 km² more in season two (20 September: Shandryholove, Derylove, Drobysheve), for 125+ km² in all. Outside mappers put it higher: Molin at over 200 km², others at up to about 240 km². ISW judged the gains larger than disclosed but gave no figure. The "240 km²" is a mappers' estimate, not an ISW confirmation [src:kyivindependent-2026-york-vivaldi-gains] [src:euromaidanpress-2026-isw-vivaldi-fortress-belt] [src:hromadske-2026-lisova-vivaldi-phase1].
  - *Reconnaissance aimed at C2:* brigade and corps reconnaissance mapped the Russian command nodes whose loss would break control of the sector. Heavy bomber drones then dropped kamikaze UGVs onto them, more than 10 km behind the line. Russia had to pull troops back to guard its command posts, and those troops were then missing from the defence [src:euromaidanpress-2026-zoria-vivaldi-targeting] [src:euromaidan-2026-mukhina-vivaldi-ugv-drop].
  - *Counter-ISR:* an air-defence and EW "dome" over the assault area blinded Russian drones so they "could not read the operation's design". Artillery and strike drones hit command posts, communications nodes, crossings and reserves. Assault groups moved through ravines and gullies [src:euromaidan-2026-mukhina-vivaldi-ugv-drop] [src:kyivindependent-2026-york-vivaldi-gains].
  - *Counter-drone:* in the preparatory phase the corps identified Rubikon as the unit most able to isolate the area and struck it. A Russian commander pulled Rubikon to the rear to recover just as the assault began. The corps says Russian drone units on the axis are at 70–90% strength [src:euromaidanpress-2026-zoria-vivaldi-targeting].
  - *Logistics:* the same intelligence effort found a diesel pipeline from Voronezh Oblast into occupied Luhansk Oblast. The corps says it covered about 50% of the daily fuel needs of Russian forces on the axis. Strikes on both ends forced Russia to dismantle it, and fuel now goes by truck on a 400 km run. Russia also moved the 20th Army / 144th Division repair base into Voronezh Oblast, lengthening the repair route to 250 km. Drone units could not refuel the generators that charge their batteries, and EW ran in reduced mode. ISW corroborated the supply problems [src:euromaidanpress-2026-zoria-vivaldi-targeting] [src:euromaidanpress-2026-isw-vivaldi-fortress-belt].
  - *Claims:* 3,000+ Russian casualties and 12,000+ drones in season one (corps figures, unverified). ISW assessed that Vivaldi made Russia's plan to encircle the Donetsk fortress belt from the north unfeasible [src:euromaidanpress-2026-isw-vivaldi-fortress-belt].
  - *Starlink:* no source ties Vivaldi to the Starlink cutoff directly. Biletskyi's corps is the one that reported a 20–40% drop in Russian drone effectiveness in February [src:isw-2026-feb-26-assessment].
- **Kill zone at 20–25+ km:**
  - Brovdi ("Magyar") puts it at 25+ km on both sides. He bases this on how regularly and densely strikes land, as tracked in the situational awareness system, not on the maximum range of individual drones [src:pravda-2026-levytska-magyar-kill-zone].
  - Brig. Gen. Lasiichuk (7th Air Assault Corps, Pokrovsk axis) gives 20–25 km now and expects 30 km by the end of 2026. Drones cause 70–80% of damage [src:euromaidanpress-2026-vivdych-kill-zone-corps].
  - The 107th TDF Brigade commander describes the distance troops must walk growing from 2 km to 3, 5, 10 and now 15+ km. Up to 20 km is "logistically difficult", and some trips to positions take three days [src:rbcukraine-2026-perch-kill-zone-foot].
  - On the Kramatorsk–Kostiantynivka road, vehicles run at up to 120 km/h under nets. Routine supply and rotations go on foot, and at least ten wrecked UGVs line the route [src:kyivindependent-2026-korovayny-road-kill-zone].
  - No strike-density map is public. The MoD's Mission Control does sort every mission by type and by depth band (20+ km, 50+ km) and records own losses by depth, so the data exists inside Delta [src:mod-2026-mission-control-cost-exchange]. The only open proxies are unit tallies (K-2: 258 strikes in April, 344 in May) and OSINT counts of strike videos [src:euromaidanpress-2026-crimea-island].
- **Ukrainian C2 at scale:**
  - Mission Control launched on 27 January 2026 as the national drone-management module inside Delta, replacing paper reports (forms 5.31/5.32) [src:mod-2026-mission-control-launch] [src:mod-2026-delta-6600-targets].
  - In June 2026 Delta logged 200,000+ drone strikes (6,600+ per day) and 75,000+ video streams per day. Confirmed strikes rose from 105,800 in January to 200,200 in June [src:mod-2026-delta-6600-targets].
  - The Avengers Labs dataset holds 5 million labelled images, and the auto-detector analyses 100,000+ drone streams per month [src:pravda-2026-levytska-avengers-labs].
- **UGVs** [src:united24-2026-kabachynskyi-ugv-25000] [src:united24-2026-kosoy-ugv-kill-zone] [src:mod-2026-ugv-9000-march] [src:mod-2026-ugv-triple]:
  - Monthly missions, from Delta data: 2,900+ (November 2025), then for January to August 2026 7,511 → 7,960 → 9,072 → 11,028 → 14,059 → 16,664 → 19,940 → **25,143**. The January–August total is about 112,000, 3.3 times the January rate by August. Most missions are logistics.
  - Units using UGVs: the two United24 counts do not conflict; they are two different MoD measures. Units active in a given month went from 67 (November 2025) to 167 (March 2026). Units that used UGVs at all went from 117 (2025) to 230 (2026 so far).
  - Plan: 25,000 UGVs contracted in the first half of 2026. One robot carries about 500 kg, the load of about 10 soldiers. The General Staff credits UGVs with up to 30% fewer personnel casualties (a claim) [src:euromaidanpress-2026-ugv-playbook].
  - Doctrine lags: about 200 models are in use, "basically every one" is modified by the unit, and best practice goes out of date in about six months [src:euromaidanpress-2026-ugv-playbook].
  - **Attrition:** a Khartia UGV commander says a robot used to last 12–15 missions, then 8–9, and now 3–4 near Kupiansk. The causes are mines, FPV and fibre-optic drones, and Russian crews assigned specifically to hunt UGVs. At 10–15 km/h most robots are too slow to cross logistics routes safely [src:euromaidanpress-2026-ugv-persian-elephants]. On the Kostiantynivka supply routes, where UGVs are the main resupply method, analysts counted at least 50 destroyed logistics UGVs by May [src:euromaidanpress-2026-axe-ugv-kostiantynivka]. RUSI observed a two to three hour wait at night between a UGV reaching the rendezvous and the casualty reaching the medical post [src:rusi-2025-watling-combined-arms]. No army-wide loss rate is published.
- **Road netting:** 1,066 km built between January and July 2026, reaching 9.2 km/day in June, with a target of 4,000 km by the end of 2026. Euromaidan Press says Russian reconnaissance and FPV drones work 15–30 km deep [src:euromaidanpress-2026-mukhina-net-tunnels-1000km] [src:euromaidanpress-2026-tril-net-tunnel-pace].
- **"Middle strike" / "Logistics Lockdown"** [src:kyivindependent-2026-rushton-middle-strike] [src:jamestown-2026-lapaiev-mid-range-drones] [src:atlanticcouncil-2026-kirichenko-drones-logistics]:
  - Ukrainian drones hit targets 20–200 km deep: fuel trucks, convoys, bridges and the R-280 "Novorossiya" highway (the M-14) to Crimea. The bands are roughly FPVs to 20 km, Hornet-type AI drones to 150 km, and FP-1/FP-2 to 200 km [src:euromaidanpress-2026-crimea-island].
  - The MoD launched the "Logistics Lockdown" programme on 27 May with an extra UAH 5 billion (about $112 million), paid through ePoints to the best-performing middle-strike units. The MoD claims a fourfold rise in destroyed Russian logistics at operational depth over the preceding months. It says Russian assaults along the line fall as logistics losses rise, and credits this jointly to middle strike and the Starlink loss [src:mod-2026-logistics-lockdown-launch]. The Kyiv Independent calls it "Logistical Lockdown".
  - Geolocated strikes rose from 55 (April) to 130+ (May). 125 trucks were hit and 80+ destroyed. Russia restricted traffic on the R-280 by decree on 21 May.
  - Brovdi said on 9 June that freight traffic on the M-14 from Rostov had fallen by 71%. From strike videos, OSINT analyst Clément Molin counted about ten Russian trucks hit per day by late May. The K-2 unit alone logged 258 strikes in April and 344 in May [src:euromaidanpress-2026-crimea-island].
  - By mid-June, Russian-installed officials said no intact bridges remained at Crimea's land entrances. Chonhar was unusable and pontoon replacements were struck [src:euromaidanpress-2026-crimea-island]. In July the MoD listed 14 rail and road bridges, overpasses and pontoon crossings hit in Donetsk, Luhansk, Kherson and Zaporizhzhia oblasts and in Crimea, plus S-400, Buk and Pantsir systems and radars that protected the routes [src:mod-2026-middle-strike-july-results].
  - Gasoline rationing (20 litres) began in Crimea on 30 May, and Sevastopol sold out on 31 May. ISW judged middle strike the catalyst: long-range refinery strikes cut production, and middle strike stopped the remaining fuel reaching occupied Ukraine [src:isw-2026-06-04-gasoline-synergy].
  - Russia responded with camouflage, decoys, mobile fire teams and road netting. Fuel drivers were ordered into civilian clothes, and military fuel was moved in ambulances and bread vans [src:atlanticcouncil-2026-kirichenko-drones-logistics] [src:euromaidanpress-2026-crimea-island].
  - The programme outlived its author. Zelenskyy dismissed Fedorov in July and appointed Drapatyi Commander-in-Chief. On 22 July acting minister Khmara, after meeting Drapatyi, said deep strike, middle strike and UGVs would be scaled up. Parliament confirmed Khmara as minister on 19 August [src:mod-2026-khmara-drapatyi-priorities] [src:euromaidanpress-2026-khmara-appointment].
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
| e9-counteroffensive-26 | 20–25+ km on both sides (30 km expected); Russian drones to 15–30 km, Molniya motherships to 40–50 km; Ukrainian bands: FPV to ~20 km, AI middle-strike drones to 150 km, FP-1/2 to 200 km | Troops walk 15+ km (up to 3 days); 25k UGV missions a month, robots lasting 3–4 missions in the worst sectors; 1,000+ km of nets; vehicles at 120 km/h under nets; M-14/R-280 freight down ~71%; Crimea land bridges cut; Russian fuel trucked 400 km on the Lyman axis | pravda-2026-levytska-magyar-kill-zone; euromaidanpress-2026-vivdych-kill-zone-corps; rbcukraine-2026-perch-kill-zone-foot; kyivindependent-2026-rushton-middle-strike; isw-2026-03-18-roca; euromaidanpress-2026-crimea-island; euromaidanpress-2026-ugv-persian-elephants |

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
| Russian fire missions flown by unmanned systems | up to 80 | % | e8-drone-kill-zone | csis-2026-bondar-russian-c2 |
| Ukrainian People's Satellite SAR images delivered (Sep 2022–May 2026) | 5,900+ | images | e8-drone-kill-zone | euromaidanpress-2026-peoples-satellite |
| Starlink speed cap on terminals | ~75–90 | km/h | e9-counteroffensive-26 | twz-2026-altman-rogoway-starlink-scramble |
| Russian drone effectiveness lost after the cutoff (3rd Corps, first two weeks) | 20–40 | % | e9-counteroffensive-26 | isw-2026-feb-26-assessment |
| Russian Molniya strike depth after partial recovery (March 2026) | 40–50 | km | e9-counteroffensive-26 | isw-2026-03-18-roca |
| Terminals one suspect registered for Russia through the whitelist | 1,000+ | terminals | e9-counteroffensive-26 | euromaidanpress-2026-sbu-starlink-schemes |
| Rassvet satellites in orbit / needed / planned end-2027 | 32 / 200–250 / 292 | satellites | e9-counteroffensive-26 | euromaidan-2026-vivdych-rassvet-windows |
| Volna Kupol Garant footprint / cost / deployed (July) | ~20 km² / $1.5–1.8M / 10 | — | e9-counteroffensive-26 | euromaidanpress-2026-kupol-jammers-deployed; euromaidanpress-2026-kupol-third-destroyed |
| Units using UGVs in the month, Nov 2025 → Mar 2026 | 67 → 167 | units | e9-counteroffensive-26 | mod-2026-ugv-9000-march |
| UGV lifespan, earlier → mid-2026 near Kupiansk | 12–15 → 3–4 | missions | e9-counteroffensive-26 | euromaidanpress-2026-ugv-persian-elephants |
| Logistics Lockdown extra funding (May 2026) | UAH 5bn (~$112M) | — | e9-counteroffensive-26 | mod-2026-logistics-lockdown-launch |
| M-14 (R-280) freight traffic fall by early June 2026 | ~71 | % | e9-counteroffensive-26 | euromaidanpress-2026-crimea-island |
| Russian trucks hit (OSINT count from videos, late May 2026) | ~10 | per day | e9-counteroffensive-26 | euromaidanpress-2026-crimea-island |
| Bridges and crossings hit by middle strike (July 2026) | 14 | structures | e9-counteroffensive-26 | mod-2026-middle-strike-july-results |
| Vivaldi area, corps claim (seasons 1+2) / mappers' estimates | 125+ / 200–240 | km² | e9-counteroffensive-26 | kyivindependent-2026-york-vivaldi-gains; hromadske-2026-lisova-vivaldi-phase1 |
| Russian fuel on the Lyman axis via the destroyed pipeline / replacement truck run | ~50% / 400 km | — | e9-counteroffensive-26 | euromaidanpress-2026-zoria-vivaldi-targeting |

---

## 5. Russia's side as reported

**C2.**
- **2022:** Russia went in with insecure analogue radios, missing encryption keys and stolen civilian phones. Traffic was mostly units reporting their positions, which Ukraine exploited [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **2023 fixes** [src:rusi-2023-watling-meatgrinder]:
  - Higher HQs were dispersed and linked by microwave links, relay vehicles and field cable, often tied into captured Ukrainian civilian telecoms.
  - Strelets was used for reconnaissance and fire correction by elite units.
  - Below battalion level, traffic stayed mostly in the clear.
- **2025–26 software:** Russia has dropped the goal of one end-to-end automated C2 system (the Sozvezdie-M2 / UTLCS lineage) in favour of task-specific tools: Glaz/Groza for drone-to-fires, ZOV Maps, and the Svod situational-awareness complex. Up to 80% of fire missions are now flown by unmanned systems [src:csis-2026-bondar-russian-c2]. At headquarters level, photos from Russian command posts show ASTRAS, a domestic messenger-style tool (text, voice, probably files) that apparently replaced the blocked Discord. It is a chat system, not a map-centric one like Delta [src:militarnyi-2025-astras].
- **Structure:** Russian C2 is top-down. Drones are kept over Russian forces for "combat management", and horizontal coordination between neighbouring units is weak [src:rusi-2025-watling-tactical-developments] [src:rusi-2023-watling-meatgrinder].
- **2024–26 dependencies:** Smuggled Starlink terminals (bought via Dubai and Central Asia; one analyst claims 50,000+) and Telegram for coordination and fundraising [src:wikipedia-starlink-russo-ukrainian-war] [src:ukrinform-2026-starlink-c2] [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
- **February 2026 losses:**
  - The whitelist removed Starlink, then the Kremlin throttled Telegram.
  - Replacements were Yamal/Express satellite terminals, Gazprom Space Systems ("far less reliable"), mesh networks, radios and cable [src:osw-2026-wilk-starlink-cutoff] [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
  - Reported side effects: friendly fire (12 killed in one Zaporizhzhia incident) and attempts to get Ukrainians to register terminals for them [src:atlanticcouncil-2026-spencer-russian-comms-crisis].
  - Forecasts of recovery time ranged from about six months (a Ukrainian USF brigade commander) to one or two months for partial recovery and three to five years for full recovery (Biletskyi) [src:euromaidanpress-2026-zoria-isw-rubikon-starlink] [src:isw-2026-feb-26-assessment].
- **Recovery, March–September 2026: partial, and faster than hoped.**
  - *March:* units put large antennas and repeaters on high-rises, for example in Myrnohrad. Those emitters exposed positions and drew Ukrainian strikes [src:isw-2026-03-10-roca]. By mid-March, Molniya "motherships" were again carrying FPVs 40–50 km into the Ukrainian rear near Kupiansk. They were controlled over radio relays, fibre-optic internet links and Kometa satellites: slower than Starlink, but a partial substitute [src:isw-2026-03-18-roca].
  - *Mesh radio:* the second adaptation was mesh radio. Chinese XK-F358 modems (about $7,000–9,000 each, unsanctioned) link Gerans and Gerberas in self-healing networks that keep working after losing about 80% of their nodes, over 100+ km or up to 600 km in relay chains. They spread onto Geran-2s from July 2025 and to other long-range drones after the cutoff. Frontelligence Insight finds that fibre makes up about 60% of Russian FPVs in key sectors [src:euromaidanpress-2026-unjammable-russian-drones]. By September, relay chains kept drones under control 150–175 km from the farthest ground station, twice the 2025 range [src:militarnyi-2026-khomenko-mesh-175km]. About 80% of Russian frontline point-to-point links and drone relays are Ubiquiti radios [src:militarnyi-2026-ubiquiti-80].
  - *Laundering through the whitelist:* Russia paid Ukrainians, often recruited through Telegram, to register terminals in their own names. SBU cases include a Kyiv network that activated 76 terminals for the FSB and a Dnipro man suspected of registering 1,000+ (July 2026) [src:euromaidanpress-2026-sbu-starlink-schemes].
  - *Rassvet:* Bureau 1440's LEO constellation launched 16 satellites in March 2026, with six to ten minute windows over Ukraine, and 16 more in July. By late August it had two stable windows of more than an hour a day (ISW: up to about 90 minutes in total), although the July batch stalled at 300–400 km [src:euromaidan-2026-thomas-isw-rassvet]. Ukrainian estimates say 200–250 satellites are needed for round-the-clock service. Russia plans 292 by the end of 2027, but by Ukrainian calculation needs five to eight years at Soyuz-2.1b launch rates. Ukraine has asked the ITU to exclude its territory, and Flamingo missiles struck the Soyuz assembly plant in Samara in August [src:euromaidan-2026-vivdych-rassvet-windows].
  - *Result:* Euromaidan Press, citing ISW, says Russia "gradually restored command and control through the summer". Ukrainian attacks met more resistance and the grey zone re-stabilised by August. ISW credits the Feb–Aug counteroffensive to five factors, of which the Starlink cutoff is only one [src:euromaidanpress-2026-axe-quiet-counteroffensive]. The advantage faded but did not vanish: in June Euromaidan Press still described Russian drones as "hobbled" without Starlink [src:euromaidanpress-2026-kupol-jammers-deployed].
- **Attacking Ukraine's Starlink instead:** the Volna Kupol Garant is not a Russian substitute but a jammer against Ukrainian Starlink [src:euromaidanpress-2026-kupol-jammers-deployed] [src:euromaidanpress-2026-kupol-third-destroyed]:
  - Six trailers of dishes jam eight 62.5 MHz channels in the 14–14.5 GHz band, deafening a passing satellite over about 20 km² (roughly 2.5 km around the site). Each costs about $1.5–1.8 million.
  - Ukraine identified it in mid-June 2026. By July Reuters counted ten on the front, and a third destroyed one was confirmed at Gelendzhik in August.
  - Its effect is interference, not a blackout: a satcom engineer puts it at 0–35% packet loss inside a cell about 17 km across. That is enough to break unhardened drone links [src:militarnyi-2026-stepanets-ew-vs-starlink].
  - Its power makes it a beacon for strikes. AI-terminal drones such as the Hornet need no live link and are unaffected.
  - Russian proposals now range up to anti-satellite weapons. A Starlink ground station in Poland burned on 23 September; Warsaw suspects sabotage [src:euromaidanpress-2026-starlink-hunt-widens].

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
- **In 2026** Russia faced the Ukrainian middle-strike campaign on the R-280 and routes into Crimea. Reported effects: driver shortages, higher shipping costs, fuel rationing, R-280 traffic restrictions, and depots moved deeper into Russia [src:kyivindependent-2026-rushton-middle-strike] [src:jamestown-2026-lapaiev-mid-range-drones].
  - *Crimea:* RUSI reports that no heavy trucks can cross the Kerch Bridge. All three Kerch Strait ferries have been struck out of service. Freight moves to Crimea in long escorted convoys on secondary roads, and carriers struggle to get insurance [src:rusi-2026-ferris-logistics-targeting]. The bridge itself was hit in 2022, 2023 and 2025 but always reopened [src:wikipedia-crimean-bridge].
  - *Rail:* Russia has brought back Soviet-style armoured trains to escort resupply and repair crews, and drones have disabled some. BARS reserve units act as mobile fire teams guarding railways in Zaporizhzhia [src:rusi-2026-ferris-logistics-targeting].
  - *Fuel types matter:* motorcycles, ATVs and light trucks run on gasoline, which ran short in occupied Ukraine from late May 2026. Armour, Urals and generators run on diesel, which is still cushioned by Russia's export surplus even though output fell about 10% in April and again in May [src:isw-2026-06-04-gasoline-synergy]. On the Lyman axis a diesel pipeline supplied about half the fuel until Ukraine destroyed it. Its replacement is a 400 km truck run [src:euromaidanpress-2026-zoria-vivaldi-targeting].
  - *Depot distances:* no systematic 2026 figure exists. The examples are the repair base moved to Voronezh Oblast (a 250 km route) and logistics elements for the Borova and Lyman directions pulled across the border [src:euromaidanpress-2026-zoria-vivaldi-targeting] [src:euromaidanpress-2026-isw-vivaldi-fortress-belt]. The rail/road split is not measured in open sources. The last detailed description is still RUSI's 2022 pattern: rail to railheads, then trucks. Middle strike now reaches both legs [src:rusi-2022-zabrodskyi-preliminary-lessons] [src:mod-2026-middle-strike-july-results].

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
  - 2025–26 figures are thin and not independent. Russia's Glaz/Groza cut its drone-to-artillery loop "from hours to minutes" (CSIS, from Russian developer channels). Euromaidan Press says Delta cut Ukraine's detection-to-strike time "from roughly 20 minutes to a few" (unattributed). RUSI gives slower timings for everything else: 24–48 hours for each of the first three attack phases, and 2–3 hours from UGV rendezvous to medical post [src:csis-2026-bondar-russian-c2] [src:euromaidanpress-2026-hryfon-zakhid] [src:rusi-2025-watling-combined-arms].

  Latency, together with guns displacing within 2–15 minutes, gives a natural "shoot and scoot" mechanic.
- **Supply reliability model.** Treat each leg of a supply route as a hazard.
  - Rear leg: rail or trucks, exposed to deep strikes.
  - Mid leg: nets, night movement, fast vehicles.
  - Last mile: foot, bomber drones, UGVs.

  A delivery succeeds with some probability that depends on the kill-zone depth, net coverage, weather and UGV availability. Ukrainian UGV missions (7.5k → 25k per month in 2026) and net kilometres should be buildable upgrades. Units should resupply when conditions allow, not when they run low.
- **Concentration penalty.** The bigger and slower a formation in the kill zone, the higher its strike probability. That pushes the player toward small assault groups (20 or fewer), dispersion, decoys and reversionary positions. Russian AI behaviour should shift from BTG columns (e1) to 1–5 person infiltration and motorcycle swarms (e8–e9).
- **C2 as a network with single points of failure.** Starlink, Telegram and HQ nodes can be cut by events, deep strikes or EW.
  - The Starlink whitelist should be a *phased* scripted event, not a switch: speed cap (about 29 Jan–1 Feb), then enforcement (5 Feb), then Telegram throttling (9–10 Feb).
  - Russian drone effectiveness drops by about 20–40% at first. It recovers partially over one to two months through relays, mesh radios and other satellites, and more over the summer.
  - A laundering mechanic (bought whitelist registrations) and Rassvet coverage windows that grow over time let Russia climb back.
  - Substitute emitters (big antennas, repeaters) and Kupol jammers are loud and targetable, so both recovery and counter-jamming cost Russia exposure.
- **Targeting the enemy's C2 is its own mission type.** Vivaldi is the template: reconnaissance maps the command nodes, air-dropped kamikaze UGVs and strike drones hit them, and the enemy diverts troops to guard its HQs. Meanwhile an EW/air-defence "dome" denies the enemy's drones a picture of the attack [src:euromaidanpress-2026-zoria-vivaldi-targeting].
- **UGVs wear out.** Give each robot a per-mission loss probability that rises with local FPV and mine density: about 12–15 missions of life early in 2026, 3–4 in the hottest sectors by summer [src:euromaidanpress-2026-ugv-persian-elephants].
- **Fuel types and pipelines.** Russian light vehicles (bikes, ATVs, trucks) burn gasoline; armour and generators burn diesel. Model separate stocks and fixed nodes (pipelines, bridges, ferries) whose loss adds hundreds of kilometres of truck haul [src:isw-2026-06-04-gasoline-synergy] [src:euromaidanpress-2026-zoria-vivaldi-targeting].
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
- **middle strike** — Ukrainian drone strikes at about 20–200 km depth, against logistics, air defence and command posts.
- **Logistics Lockdown** — the MoD programme (27 May 2026) funding middle-strike units through ePoints; the Kyiv Independent calls it "Logistical Lockdown". Launched under Fedorov and continued under Khmara.
- **whitelist** (*bilyi spysok*, білий список) — the registry of verified Starlink terminals; unregistered terminals stop working in Ukraine from 5 Feb 2026.
- **BAI** — battlefield air interdiction: ISW's term for Russia's drone campaign against Ukrainian supply routes and vehicles in the near rear.
- **Molniya** (Молния, "lightning") — cheap Russian fixed-wing strike drone, also used as an FPV "mothership".
- **mesh modem** — a radio in which every drone relays for the others, so the network survives losing nodes; e.g. the Chinese XK-F358 on Gerans.
- **Rassvet** (Рассвет, "dawn") — Bureau 1440's Russian LEO satellite constellation, the planned Starlink substitute.
- **Volna Kupol Garant** — Russian dish-array jammer that deafens Starlink satellites over about 20 km².
- **Glaz/Groza** (Глаз/Гроза, "eye/thunderstorm") — Russian volunteer-built drone-to-fires software; **Svod** is the Russian MoD's tactical situational-awareness complex.
- **People's Satellite** — the crowdfunded ICEYE SAR satellite whose imagery goes to HUR.
- **Rubikon / Rubicon** — Russia's elite Centre for Advanced Unmanned Technologies.
- **GEGD** — the US government programme that gave Ukraine access to commercial Maxar imagery; suspended for about a week in March 2025.

---

## 8. Open questions and gaps

- **How far Russia's C2 recovered, in numbers.** The pattern is clear: a 20–40% drop in February, partial substitutes by mid-March, and "gradually restored" C2 by August. But nobody has published a measured comparison of Russian drone sorties or strike depth before and after. ISW's daily assessments stop discussing Starlink after March. We sampled the April–May assessments and found no further Starlink passages, and could not read June–September systematically because Wayback throttles access.
- **Vivaldi's full scope.** Only two of four "seasons" are public. The size of the area is disputed (125+ km² claimed against mappers' 200–240 km²), and losses are corps claims.
- **Jet-powered Gerans and interceptor drones.** Not covered here in depth; see 01/02/03. Their effect on Ukrainian rail and energy logistics in 2026 still needs a figure.
- **Kill-zone depth varies by sector.** The estimates (15, 20–25, 25+, 15–30 km) come from different commanders and methods. Mission Control holds strike data by depth band, but no map or dataset is public.
- **UGV loss rates army-wide.** We have one commander's lifespan figures (12–15 → 3–4 missions) and road counts (10+, 50+ wrecks), but no MoD loss or failure share. The "90% of a regiment's deliveries by UGV" claim is still unverified.
- **Detection-to-strike latency in 2025–26.** Still no independent measurement of the Ukrainian loop. The available figures are Russian developer claims (via CSIS), unattributed media figures and MoD claims. No RUSI or CSIS fieldwork number for 2025–26 turned up in the RUSI and CSIS catalogues.
- **Russian rail versus truck share and depot distances in 2026.** No systematic figure. We have only anecdotes: a 400 km fuel run, a repair base moved 250 km back, M-14 freight down 71%.
- **Figures that conflict:**
  - The start of the southern counteroffensive: the Air Assault Forces and Euromaidan Press give 29 January [src:euromaidanpress-2026-axe-quiet-counteroffensive], and ISW first observed counterattacks on 9 February. The often-quoted 11 February traces back to Wikipedia (see 00 §3 e9).
  - Jamestown converts UAH 5 billion to "$1.12 million". The MoD figure is UAH 5bn, which Euromaidan Press gives as about $112 million, so Jamestown slipped by a factor of 100 [src:mod-2026-logistics-lockdown-launch].

---

## 9. Sources

Key sources are marked ★.

- defenseexpress-2022-aerorozvidka-convoy — Aerorozvidka's night thermal-drone ambushes on the Kyiv column; claims only partly verified.
- kyivindependent-2022-ponomarenko-himars — why Russian rail/truck logistics were vulnerable to HIMARS in 2022.
- ★ rusi-2022-zabrodskyi-preliminary-lessons — RUSI, based on General Staff data: Russian C2 failures, rail logistics, Kropyva, kill-chain times, deception.
- ★ rusi-2023-watling-meatgrinder — Russian recon-fire circuits, Strelets, EW density, UAV losses, HQ displacement and wired C2.
- newamerica-2023-zikusoka-gis-arta — GIS Arta "Uber for artillery", 30–45 s claim, Starlink bearer.
- rusi-2023-watling-stormbreak — 2023 offensive: Russian precision fires, gun displacement times, drone feeds driving command.
- kyivpost-2023-decoys — Metinvest decoys, costs, thermal and radio signatures.
- euromaidanpress-2024-martyniuk-decoy-makers — volunteer and industry decoy makers; the signatures decoys must copy.
- militarnyi-2024-avengers-12000 — MoD claim: Avengers detects 12,000 pieces of equipment per week.
- twz-2024-altman-ammo-depots — Maxar imagery of the Toropets and Tikhoretsk depot strikes.
- ★ csis-2024-bondar-delta-cjadc2 — the best overview of Delta's architecture and modules.
- ★ rusi-2025-watling-tactical-developments — the 3/15/40 km transparency figures, rear casualties, resupply and rotation practice.
- defenseexpress-2025-russian-net-tunnels — Russian road net tunnels near Bakhmut and Chasiv Yar.
- kyivindependent-2025-zadorozhnyy-maxar — US suspension of Ukraine's GEGD access to Maxar imagery, March 2025.
- euromaidanpress-2025-us-resumes-intel — end of the March 2025 US intelligence pause.
- frontelligence-2025-motorcycle-assaults — Ukrainian OSINT on Russian motorcycle assault and logistics groups.
- ★ kyivindependent-2025-farrell-kostiantynivka — Rubikon's effect on Ukrainian logistics; bomber-drone resupply; infiltration.
- mod-2025-delta-avengers-ecosystem — MoD claims: Avengers 2.2 s / 70%; Delta 2,000 targets per day.
- ★ rusi-2025-watling-combined-arms — FPV range band, UGV casevac, phased attacks, Russian hunting of C2/EW.
- militarnyi-2025-astras — ASTRAS, Russian messenger-style command-post software (photo-based).
- mod-2026-mission-control-launch — launch of the national Mission Control module (January 2026).
- militarnyi-2026-ubiquiti-80 — Ubiquiti radios as ~80% of Russian frontline links.
- euromaidanpress-2026-ft-europe-intel — European and Japanese satellite ISR replacing US dependence.
- mod-2026-spacex-starlink-russian-uavs — 29 Jan: MoD and SpaceX start work on Russian Starlink drones.
- isw-2026-02-01-roca — ISW: Musk and Fedorov on the first steps; the speed cap; Starlink on Russian mid-range drones.
- mod-2026-starlink-authorization-system — 1 Feb: the whitelist is announced.
- kyivindependent-2026-myronyshena-starlink-whitelist — the 2 Feb Cabinet resolution and registration channels.
- isw-2026-02-05-roca — ISW: whitelist enforced; expected hit to Russian BAI.
- kyivindependent-2026-starlink-catastrophe — the 5 Feb switch-off and mixed battlefield effects.
- ★ mod-2026-starlink-whitelist-operational — 5 Feb: Russian terminals blocked; the whitelist is live.
- isw-2026-02-06-roca — ISW: fewer Russian assaults and FPV sorties after the block.
- twz-2026-altman-rogoway-starlink-scramble — the speed-based kill switch and early Russian workarounds.
- ★ csis-2026-bondar-russian-c2 — Russian C2 software (Glaz/Groza, Svod); up to 80% of Russian fire missions unmanned.
- isw-2026-02-10-roca — ISW: Telegram throttled 9–10 Feb; the origin of the "1 February" date.
- ★ osw-2026-wilk-starlink-cutoff — Starlink whitelist mechanics and Russian substitutes.
- ukrinform-2026-starlink-c2 — Ukrainian expert views on the Russian C2 collapse; 50k-terminal smuggling claim.
- euromaidanpress-2026-zoria-isw-rubikon-starlink — ISW: the Starlink block degrades Rubikon.
- futuradoctrina-2026-ryan-starlink-surprise — Mick Ryan: Russian uses of Starlink, including UGVs; lessons about surprise.
- united24-2026-litnarovych-starlink-gains — 201 km² in one week per ISW data; contains a date error.
- euromaidanpress-2026-tril-net-tunnel-pace — pace and budget of Ukrainian net-tunnel building in early 2026.
- ★ isw-2026-feb-26-assessment — ISW: Biletskyi's 20–40% cut in Russian drone effectiveness; recovery forecast.
- ★ atlanticcouncil-2026-spencer-russian-comms-crisis — Starlink cut plus Telegram throttling; MAX; friendly fire.
- isw-2026-03-10-roca — ISW: Russian antennas and repeaters exposed; drone-operator hunting.
- ★ isw-2026-03-18-roca — ISW: Russian partial recovery, Molniya motherships 40–50 km deep.
- ★ mod-2026-ugv-9000-march — primary UGV counts: 67 → 167 active units; monthly missions.
- kyivindependent-2026-hodunova-ukrzaliznytsia — scale of the Russian campaign against Ukrainian rail.
- united24-2026-kosoy-ugv-kill-zone — UGV procurement, payloads, unit counts.
- ★ pravda-2026-levytska-magyar-kill-zone — Brovdi (Magyar): kill zone 25+ km, defined by strike density.
- euromaidanpress-2026-axe-ugv-kostiantynivka — UGVs as the main resupply for Kostiantynivka; 50+ wrecked on its routes.
- euromaidanpress-2026-peoples-satellite — the crowdfunded ICEYE "People's Satellite"; 5,900+ SAR images.
- ★ mod-2026-logistics-lockdown-launch — the Logistics Lockdown programme, UAH 5bn, MoD effect claims.
- ★ kyivindependent-2026-rushton-middle-strike — the middle-strike campaign by numbers; "Logistical Lockdown".
- kyivindependent-2026-korovayny-road-kill-zone — on the ground on a netted supply road, June 2026.
- ★ isw-2026-06-04-gasoline-synergy — ISW: middle strike plus refinery strikes produce fuel shortages in occupied Ukraine.
- jamestown-2026-lapaiev-mid-range-drones — mid-range drones against the R-280; effects on Russian logistics.
- euromaidanpress-2026-ugv-playbook — UGV doctrine gaps; General Staff claim of up to 30% fewer casualties.
- fpri-2026-lee-putiata-rubicon — Rubikon's structure and growth from documents.
- atlanticcouncil-2026-kirichenko-drones-logistics — Ukrainian drone interdiction in the south; Russian countermeasures.
- euromaidanpress-2026-crimea-island — M-14 freight down 71%, truck-strike rates, Crimea bridges, strike-depth bands.
- carnegie-2026-vakulenko-refineries — Ukrainian refinery strikes and Russian output, 2025–26.
- euromaidanpress-2026-vivdych-kill-zone-corps — 7th Corps commander: 20–25 km now, 30 km expected.
- rusi-2026-ferris-logistics-targeting — Kerch Bridge closed to heavy trucks, armoured trains, BARS teams.
- euromaidanpress-2026-kupol-jammers-deployed — Volna Kupol Garant: ten on the front, 20 km² footprint.
- euromaidanpress-2026-mukhina-net-tunnels-1000km — 1,066 km of nets; Russian drones reach 15–30 km.
- mod-2026-khmara-drapatyi-priorities — the new leadership keeps middle strike, deep strike and UGVs.
- ★ mod-2026-delta-6600-targets — Delta statistics for June 2026: strikes, streams, map objects.
- euromaidanpress-2026-unjammable-russian-drones — Russia's mesh-radio and fibre adaptation (Frontelligence).
- euromaidanpress-2026-kupol-third-destroyed — third Kupol jammer destroyed; ~$1.8M per unit.
- mod-2026-middle-strike-july-results — July 2026 middle-strike results: 14 bridges and crossings, air defences.
- pravda-2026-levytska-avengers-labs — Avengers Labs dataset; 100k+ streams per month.
- euromaidanpress-2026-sbu-starlink-schemes — Russian laundering of whitelist registrations through paid Ukrainians.
- rbcukraine-2026-perch-kill-zone-foot — 107th TDF Brigade: distance on foot grew from 2 km to 15+ km.
- euromaidanpress-2026-axe-quiet-counteroffensive — Russian C2 "gradually restored" by summer; ISW's five factors behind the 745 km².
- mod-2026-mission-control-cost-exchange — Mission Control depth bands and cost-exchange claims.
- euromaidanpress-2026-khmara-appointment — Khmara confirmed as defence minister, 19 Aug 2026.
- ★ euromaidanpress-2026-ugv-persian-elephants — UGV lifespan collapsing from 12–15 to 3–4 missions.
- euromaidan-2026-thomas-isw-rassvet — ISW on Rassvet: up to about 90 minutes of daily coverage; July batch stalled.
- militarnyi-2026-khomenko-mesh-175km — Russian mesh relay chains controlling drones out to 150–175 km.
- militarnyi-2026-stepanets-ew-vs-starlink — what Russian Starlink jamming can and cannot do (0–35% packet loss).
- mod-2026-ugv-triple — 25,000+ UGV missions in August; ~112,000 in 2026.
- euromaidanpress-2026-hryfon-zakhid — unattributed claim that Delta cut detection-to-strike from ~20 min to a few (low).
- ★ united24-2026-kabachynskyi-ugv-25000 — monthly UGV mission series for 2026.
- hromadske-2026-lisova-vivaldi-phase1 — Vivaldi confirmed on 15 Sep; 75 km² in phase one; Molin 200+ km².
- euromaidanpress-2026-isw-vivaldi-fortress-belt — ISW on Vivaldi: gains larger than disclosed; Russian logistics problems corroborated.
- kyivindependent-2026-york-vivaldi-gains — Vivaldi season two; Rubikon withdrawn.
- euromaidan-2026-mukhina-vivaldi-ugv-drop — Vivaldi: air-dropped kamikaze UGVs, the EW "dome", 125+ km² (corps claim).
- ★ euromaidanpress-2026-zoria-vivaldi-targeting — Vivaldi's reconnaissance against C2 nodes, Rubikon and the diesel pipeline.
- euromaidanpress-2026-starlink-hunt-widens — Russian anti-Starlink proposals; Polish ground-station fire.
- euromaidan-2026-vivdych-rassvet-windows — Rassvet growth from 6-minute passes to hour-long windows; launch-rate limits.
- wikipedia-2026-ukrainian-counteroffensive — axes of the 2026 counteroffensive, Vivaldi, territory claims (navigation only).
- wikipedia-crimean-bridge — dates of the 2022, 2023 and 2025 bridge attacks (navigation only).
- wikipedia-starlink-russo-ukrainian-war — timeline of Starlink use on both sides (navigation only).
