# 01 — Drones and uncrewed systems

Research base for *bavovniatko*. The player is the Ukrainian side. Accessed 2026-09-28 to 2026-09-30. Sources are listed in `sources/01-drones-uncrewed.yaml` and cited inline as `[src:<id>]`. Figures from officials and manufacturers are claims, and the text names who makes each one.

## 1. Overview

Drones went from being scarce and quickly lost to being the main killer on the battlefield.

- **2022.** Drones were mostly scouts: commercial quadcopters, Ukrainian fixed-wing ISR (intelligence, surveillance and reconnaissance) and a few Bayraktar TB2s. Around 90% of Ukrainian UAVs were lost, and only about a third of missions succeeded [src:rusi-2022-zabrodskyi-preliminary-lessons]. The Shahed campaign against cities began in late September [src:kaggle-2026-ivaniuk-missile-attacks].
- **2023.** Russia's Orlan-10 plus artillery "reconnaissance-fires circuit" and the Lancet loitering munition dominated; by autumn Lancets were hitting jets about 65 km from Russian-held ground [src:kyivindependent-2023-farrell-lancet]. Ukraine was losing about 10,000 UAVs a month to Russian electronic warfare (EW) [src:rusi-2023-watling-meatgrinder].
- **2024.** The FPV (first-person-view kamikaze quadcopter) boom arrived. RUSI's fieldwork found tactical UAVs caused 60–70% of damaged or destroyed Russian systems, even though 60–80% of FPVs never reached their target [src:rusi-2025-watling-reynolds-third-year]. Shahed-type launches roughly tripled on 2023, to about 11,000, most of them after August [src:kaggle-2026-ivaniuk-missile-attacks].
- **2025.** A "kill zone" 10–15 km deep formed on both sides of the line [src:rbc-2026-fedorov-drone-line][src:osw-2025-game-of-drones]. Several things drove it:
  - the Drone Line project, announced in February [src:kyivindependent-2025-goncharova-drone-line-launch];
  - fiber-optic FPVs with 40+ km reach [src:ukrinform-2025-fedorov-fiber-optic-40km];
  - Russia's Rubikon center hunting Ukrainian drone teams and logistics [src:rferl-2025-rubicon];
  - over 50,000 Shahed-type drones launched at Ukraine in the year [src:isis-2026-anokhin-shahed-2025];
  - mass-produced interceptor drones, at about 950 a day by December [src:kmu-2025-950-interceptors].
- **2026.** Three things changed the picture:
  - Russia lost Starlink in early February (1 Feb per ISW, 5 Feb per Kyiv Independent) [src:isw-2026-feb-26-assessment][src:kyivindependent-2026-starlink-catastrophe]. This grounded its deeper-reaching strike drones [src:euromaidan-2026-axe-starlink-counterattacks], but not its fiber-optic FPVs, so the close kill zone kept growing, to 20–25 km in some sectors by July [src:euromaidanpress-2026-vivdych-kill-zone-corps].
  - Ukrainian ground robots (UGVs) took over a large share of front-line logistics [src:united24-2026-ugv-missions].
  - Russia moved to jet-powered Gerans, which are much harder to intercept [src:united24-2026-place-jet-gerans].

Ukrainian officials now claim drones cause 70–80% of Russian casualties [src:osw-2025-game-of-drones]. In December 2025, Ukrainian drone units reportedly "neutralised" about as many Russians as Russia recruited [src:euromaidan-2026-murdoch-syrskyi-december].

## 2. Per-era notes

### e1-invasion (Feb–Apr 2022)
- Both sides used commercial and adapted quadcopters at tactical level and fixed-wing ISR at medium altitude: Ukraine's SKIF, Leleka and Furia, and Russia's Orlan-10. Ukraine flew TB2s and Russia flew Orion MALE drones (medium-altitude, long-endurance), committed only under favourable conditions [src:rusi-2022-zabrodskyi-preliminary-lessons].
- Ukrainian battalions had been receiving Furia, Leleka and PD-1 UAVs since 2015. Linked to the Kropyva fire-control software, this sped up artillery engagements [src:rusi-2022-zabrodskyi-preliminary-lessons]. The makers date formal adoption to 2019 (Leleka) and 2020 (Furia, after state trials) [src:deviro-2026-leleka][src:athlonavia-2026-furia].
- Attrition was extreme. About 90% of Ukrainian UAVs were destroyed. A quadcopter survived about 3 flights and a fixed-wing about 6, and only about 1/3 of missions succeeded [src:rusi-2022-zabrodskyi-preliminary-lessons].
- FPV kamikaze drones barely existed yet: Ukraine had about 3,000–5,000 FPVs in 2022 [src:justsecurity-2026-bidochko-drone-superpower].

### e2-donbas-artillery (May–Aug 2022)
- The Orlan-10 was the Russian workhorse. Russian units with their own UAVs could fire within 3–5 minutes of detection, against 20–30 minutes when the request went through a fire-control headquarters. Even so, Russia did not have enough Orlans to sustain its loss rate in Donbas [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **Army of Drones** (Armiia Droniv) launched in July 2022, run by UNITED24, the General Staff and the Ministry of Digital Transformation. Within 9 months it had contracted 3,200 UAV complexes (over UAH 4bn, about $107m) and trained 7,000+ operators [src:pravda-2023-fedorov-army-of-drones].

### e3-counteroffensives-22 (Sep–Nov 2022)
- Lancet-3(M) use rose from about October 2022. By early March 2023, Oryx had logged 113 visually confirmed Kub/Lancet hits and 41 misses, mostly on artillery, radars and SAM launchers [src:oryx-2022-lancet-kill-list].
- **Shahed campaign begins.** The first Shahed-136/131 appears in Air Force reports on 28 Sep 2022. At least 409 were launched from September to December, with a peak of 253 in October and a biggest night of 35 (19 Dec); about 95% were reported shot down [src:kaggle-2026-ivaniuk-missile-attacks]. Early reports often gave only the number destroyed, so these are floors.
- **Shark.** Ukrspecsystems' Shark ISR drone was unveiled in October 2022 to spot for HIMARS under heavy EW. It has a 60 km link, flies for 2+ hours at a cruising speed of 70–90 km/h and weighs 10 kg [src:euromaidan-2022-shark-himars]. Its 2026 successor, the Shark-M, flies for up to 7 hours on a 180 km link [src:militarnyi-2026-shark-m].

### e4-bakhmut (Dec 2022–May 2023)
- **EW.** Russia had at least one major EW system per 10 km of front, and Ukrainian UAV losses stayed at about 10,000 a month [src:rusi-2023-watling-meatgrinder].
- **Russian fires.** Russian counter-battery fire shifted from area saturation to Lancet strikes. Orlan-10 "complexes" orbited at both axis and artillery-brigade level, and Orlan-30s laser-designated for Krasnopol guided shells [src:rusi-2023-watling-meatgrinder].
- **Shahed lull.** Launches fell to about 240 in Q1 2023 (45 in February), then jumped to 411 in May with the spring campaign on Kyiv [src:kaggle-2026-ivaniuk-missile-attacks].

### e5-counteroffensive-23 (Jun–Nov 2023)
- **Ukrainian heavy bombers.** Ukraine hit Russian reserves at night with converted agricultural drones dropping RPGs, the forerunners of the "Baba Yaga" heavy bombers.
  - In one strike, 5 drones carrying 4 RPGs each destroyed or badly damaged 7 tanks, and all 5 drones were lost.
  - The tactic was binary: it worked only when Russian EW lapsed [src:rusi-2023-watling-stormbreak].
- **Russian precision.** Russia struck the lead elements of Ukrainian assaults with Lancets and FPVs flown together with ISR drones. RUSI flagged the Lancet-3 to 3M improvements and noted that Ukrainian units had to train for up to 25 UAVs watching them at once [src:rusi-2023-watling-stormbreak].
- **Lancet range and variants.**
  - In autumn 2023 Lancets destroyed a MiG-29 and an Su-25 at Dovhyntseve airbase near Kryvyi Rih, about 65 km from Russian-held ground. Targets are found by Orlan-10 or Supercam and handed to a launcher team [src:kyivindependent-2023-farrell-lancet].
  - A newer model pre-detonates its shaped charge to defeat nets and cages. The Lancet-3 weighs about 12 kg, and one is estimated to cost about $35,000 [src:euromaidan-2023-shandra-lancet-new-model].
  - ZALA advertised the tube-launched **Izdeliye-53** with "fully autonomous" and swarm modes, and Russian channels claimed its first use on 21 Oct 2023. HUR spokesman Yusov said there was no evidence of mass use [src:kyivindependent-2023-farrell-lancet].
- **Shahed volumes.** Launches ran at about 200–500 a month (685 in Q2 and 916 in Q3), and the Air Force reported about 82–87% shot down [src:kaggle-2026-ivaniuk-missile-attacks].

### e6-avdiivka-attrition (Oct 2023–Jul 2024)
- **Production.** The FPV boom took off. Ukraine built about 2.2m UAVs of all types in 2024 [src:osw-2025-game-of-drones]; CSIS says "approximately 2 million", 96% of them domestic [src:csis-2025-bondar-ai-autonomy]. The defence minister put 2024 FPV receipts at about 1.2m, implied from his claim that 2025 was 2.5x higher [src:pravda-2025-hubina-3m-fpv].
- **AI guidance, first steps.** In 2024 Ukraine began buying 10,000 AI-enhanced drones, a small fraction of output [src:csis-2025-bondar-ai-autonomy].
- **Organization.** The Unmanned Systems Forces (Syly bezpilotnykh system, USF) became a separate branch in June 2024 [src:kyivindependent-2025-hodunova-usf-drone-line-group].
- **Shahed volumes.** Launches rose to 1,280 in Q4 2023 (a record of 625 in December) and 1,294 in Q1 2024, then dipped to 973 in Q2 2024. The biggest night of 2023 was 75 drones (25 Nov) [src:kaggle-2026-ivaniuk-missile-attacks]. CSIS, which uses the same Air Force data, counts over 14,700 one-way attack drones from 28 Sep 2022 to the end of 2024, about 90% shot down. It takes $35,000 as the midpoint unit cost (estimates range from $20k to $80k), which makes about $350,000 per target struck [src:csis-2025-hollenbeck-shahed-cost-effectiveness].
- **Naval drones.** HUR says Magura V5 boats hit the Ivanovets, Tsezar Kunikov, Sergey Kotov, Ivan Khurs and several landing and patrol craft (Feb–Mar 2024 for the first three), for over $500m in damage [src:euromaidan-2024-mukhina-hur-magura-ships].
- **Front-line picture (RUSI 2024 fieldwork)** [src:rusi-2025-watling-reynolds-third-year]:
  - Battlefield transparency is dense within 3 km of the line, thins out to 15 km, and targets can be tasked out to 40 km.
  - Armoured vehicles have to stay concealed or dug in within about 3 km.
  - Excavators rarely come closer than 7 km.
  - Dedicated UAV units cover 40–70 km sections of front.
  - Bomber drones do more damage than FPVs but mostly fly at night.
  - About 50% of Ukrainian casualties are taken in the rear, from FPVs, artillery and glide bombs.

### e7-kursk-pokrovsk (Aug 2024–Mar 2025)
- **Rubikon.** Russia founded the Rubikon center on 2 Aug 2024 [src:fpri-2026-lee-putiata-rubicon]. It systematically cut the supply lines to Ukrainian positions around Sudzha, which made the Kursk salient unsustainable; Ukraine withdrew in March 2025 [src:rferl-2025-rubicon].
- **Fiber-optic FPVs.** RUSI noted Russia's shift to fibre-optic-guided FPVs, which cannot be jammed but have about 10 km range, fly worse and snag on obstacles. Ukraine "does not yet widely employ" them [src:rusi-2025-watling-reynolds-third-year]. Ukraine's answer leaned on AI terminal guidance [src:osw-2025-game-of-drones].
- **Hit rates (CSIS, early 2025).** Most small FPV missions under jamming succeed only 10–15% of the time. Autonomous terminal navigation raises that to 70–80%, so one or two drones do the work of eight or nine. Lock-on range grew from 300 m to about 1 km (2 km in good conditions). CSIS stresses that true autonomy is "not yet present on the battlefield": a human still selects the target [src:csis-2025-bondar-ai-autonomy].
- **Russian ISR density.** In August 2024 Russia flew 1,000–1,500 Orlan-10/Zala orbits a day. Radar-cued interceptor UAVs began wearing them down [src:rusi-2025-watling-reynolds-third-year].
- **Shahed surge.** CSIS puts launches at about 130 a week before September 2024 and about 1,100 a week by March 2025 [src:csis-2025-jensen-drone-saturation]. In monthly Air Force figures, launches went from 426 in July 2024 to 1,334 in September, 2,464 in November and 4,044 in March 2025. From September to December 2024 Russia launched more than in the previous 23 months combined [src:kaggle-2026-ivaniuk-missile-attacks][src:csis-2025-hollenbeck-shahed-cost-effectiveness]. The reported shot-down share fell to 55–58% from Q4 2024 because the Air Force began counting drones "lost" to EW separately [src:kaggle-2026-ivaniuk-missile-attacks]. Ukraine's Sting interceptor first appeared in autumn 2024 [src:defenceexpress-2025-sting-effectiveness].
- **Naval drones.** HUR claims a missile-armed Magura V5 destroyed two Mi-8 helicopters on 31 Dec 2024, the first such kill [src:euromaidan-2025-kravchuk-magura-su30].

### e8-drone-kill-zone (Mar 2025–Jan 2026)
- **Kill zone and organization.**
  - The **Drone Line** (Liniia droniv) was announced by the Defence Ministry on 9 Feb 2025. It groups five elite units: the K-2, Achilles and Rarog regiments, Magyar's Birds (414th brigade) and the Border Guard's Phoenix. Its aim is a 10–15 km-deep kill zone [src:kyivindependent-2025-goncharova-drone-line-launch].
  - The sources that date it to "spring" or June describe later steps. The ministry's anniversary material dates the start of operations to March 2025 and puts the cost at $880m [src:euromaidan-2026-mukhina-drone-line-880m]. On 20 June 2025 the USF and the Drone Line units were placed under one command group [src:kyivindependent-2025-hodunova-usf-drone-line-group].
  - Robert "Magyar" Brovdi took command of the USF on 3 June 2025 [src:militarnyi-2025-brovdi-usf].
- **Range race.**
  - Ukrainian fiber-optic FPVs went from about 20 km to 40+ km between spring and July 2025, according to Fedorov [src:ukrinform-2025-fedorov-fiber-optic-40km]. ISW says Russian fibre FPVs reach up to 60 km [src:isw-2026-feb-26-assessment].
  - Russia's plywood **Molniya** fixed-wing drone strikes logistics 50+ km deep. Variants carry an FPV, run on fiber, or carry Starlink for reconnaissance [src:mezha-2026-molniya].
- **Rubikon.** In spring 2025 Rubikon had 7 units of 130–150 people. By Sep 2025, over 25% of its strikes were on Ukrainian drone teams and 15% on EW [src:rferl-2025-rubicon]. It grew to about 5,000 personnel by spring 2026 [src:fpri-2026-lee-putiata-rubicon].
- **Deep strike.**
  - **Operation Spiderweb (Pavutyna)**, 1 June 2025: 117 FPVs launched from trucks at bomber bases. The SBU and General Staff claim 41 aircraft hit and about $7bn in damage [src:kyivindependent-2025-york-spiderweb].
  - The Liutyi's range doubled to about 1,000 km, and it costs as little as $55k. Fewer than 30% reach the target area, and the SBU claims 160+ strikes on Russian oil facilities in 2025 [src:defensenews-2025-ap-long-range-drones].
- **Shahed war.**
  - Russia launched 54,538 Shahed-type drones in 2025, about 40% of them decoys. The record night was 7 Sep 2025, with 810 [src:isis-2026-anokhin-shahed-2025]. The compiled Air Force reports give a near-identical 54,800 [src:kaggle-2026-ivaniuk-missile-attacks].
  - July 2025 set a monthly record of 6,129, against 423 in July 2024 [src:kyivindependent-2025-zadorozhnyy-79000-shahed].
  - The hit rate rose from 2–3% early in the year to about 17% in December [src:isis-2026-anokhin-shahed-2025].
  - The jet Geran-3 appeared, and the Geran-4/5 followed in late 2025 [src:csis-2025-jensen-drone-saturation][src:isis-2026-anokhin-shahed-2025].
- **Interceptors.**
  - Sting reaches over 315 km/h for about $2,000. Its maker reports 60–90% efficiency and says 8 in 10 kills need only one drone [src:defenceexpress-2025-sting-effectiveness].
  - Deliveries reached about 950 interceptors a day in December 2025 [src:kmu-2025-950-interceptors].
- **Autonomy and swarms.** Swarmer's software had flown "over a hundred" swarm missions by September 2025, in which drones in a group decide which strikes first. Brave1's AI lead put a simple AI targeting kit at about $150 and said the share of UAVs hitting targets "is constantly decreasing" without it. Russia's V2U was seen flying in groups of six with onboard AI and a 4G modem [src:euromaidan-2025-kirichenko-ai-swarms].
- **Russian heavy bombers.** Russian hexacopter copies of the Vampire, also called "Baba Yaga", appeared in 2025. The 81st Brigade says it had shot down five by March 2026 [src:euromaidan-2026-zoria-russian-vampire-copies].
- **Naval drones.**
  - HUR claims a Magura V5 missile shot down an Su-30 near Novorossiysk on 2 May 2025 [src:euromaidan-2025-kravchuk-magura-su30].
  - An SBU source says SBU Sea Baby drones disabled the empty shadow-fleet tankers Kairos and Virat off Turkey on 28 Nov 2025, in a joint operation with the Navy [src:kyivindependent-2025-zadorozhnyy-kairos-virat].
  - The SBU says its underwater Sea Baby put a Kilo-class submarine out of service in Novorossiysk on 15 Dec 2025 [src:kyivindependent-2025-myronyshena-sub-sea-baby].
- **Gamified scoring.** Under **e-points** (ye-baly) and the "Army of Drones Bonus", units upload confirmed hits to DELTA and spend the points on Brave1 Market. Units had ordered over UAH 5bn this way since August 2025 [src:digitalstate-2025-brave1-epoints].
  - Prices move to steer behaviour. A killed soldier was worth 2 points, then 6 from April 2025 [src:united24-2025-khomenko-adb-launch], then 12 by January 2026, when a drone operator was worth 25 and a soldier captured alive 120 [src:kyivpost-2026-korshak-epoints].
  - By mid-2026 points also paid for reconnaissance, logistics and evacuation missions [src:euromaidanpress-2026-epoints-400-units].
- **Output.**
  - Ukraine obtained 3m FPVs and about 15,000 UGVs in 2025 [src:pravda-2025-hubina-3m-fpv]. OSW's projection was 4.5m UAVs of all types [src:osw-2025-game-of-drones].
  - Syrskyi claims USF units "neutralised" about 33,000 Russians (video-confirmed) in December 2025 [src:euromaidan-2026-murdoch-syrskyi-december].

### e9-counteroffensive-26 (Feb 2026–present)
- **Starlink cut-off.** SpaceX's whitelist cut Russian terminals in early February 2026. ISW dates the block to 1 Feb, and the Kyiv Independent reported the front-wide cut-off on 5 Feb [src:isw-2026-feb-26-assessment][src:kyivindependent-2026-starlink-catastrophe].
  - The larger Russian attack drones that struck 20–200 km deep via Starlink lost their link. FPVs on line-of-sight or mesh radio carried on [src:euromaidan-2026-axe-starlink-counterattacks].
  - Starlink was on Shaheds, Molniyas and the BM-35. One Ukrainian engineer estimates that only a "single-digit percentage" of Russian drones used it [src:euromaidan-2026-kossov-starlink-russian-drones].
  - The 3rd Army Corps commander Biletsky says the outage cut the effectiveness of Russia's drone campaign by roughly 20–40% in two weeks, with partial recovery expected in one to two months. ISW saw lower tempo and depth in Russian tactical and mid-range strikes, only partly offset by local SIM cards and relay drones [src:isw-2026-feb-26-assessment].
  - Ukrainian counterattacks began across nine vectors toward Huliaipole and in Zaporizhzhia and Dnipropetrovsk oblasts. One up-armoured M1A1 was still immobilised by FPVs [src:euromaidan-2026-axe-starlink-counterattacks].
  - Brigades gave mixed accounts of how much Russian assaults actually slowed [src:kyivindependent-2026-starlink-catastrophe]. The Kyiv Independent's front reporting noted that Russia's dispersed infantry assaults hardly needed Starlink. Russia also kept a "colossal" lead in fiber-optic drones reaching 50+ km [src:kyivindependent-2026-post-starlink-scramble].
- **Did the Russian kill zone shrink?** Only its deep layer did.
  - The satellite-linked strikes 20–200 km deep thinned [src:euromaidan-2026-axe-starlink-counterattacks][src:isw-2026-feb-26-assessment].
  - The fiber layer kept growing. A Russian fibre FPV reached Kharkiv's outskirts on 25 Feb [src:isw-2026-feb-26-assessment], and a ring-wing KVN variant (KVS) carries the same 3 kg payload up to 50 km [src:euromaidan-2026-axe-ring-fpv].
  - Frontelligence Insight's Tatarigami says about 60% of Russian FPVs in key sectors are fibre-optic. An OSINT tally cited by the same outlet puts Russian FPV use at about 10,000 a day [src:euromaidan-2026-axe-unjammable-russian-drones].
  - Ukraine is converging on fibre too: 70% of National Guard drones were fibre-optic by September 2026, up from about 20% [src:euromaidan-2026-vivdych-ng-fibre-70].
  - Fibre supply is a pressure point. ISW, relaying Russian industry sources, reports that Chinese suppliers raised the price for Russian buyers from 16 to 40 yuan per km between early 2025 and early 2026. The struck Saransk plant had made about 4m km a year [src:isw-2026-feb-26-assessment].
  - Russia's Rassvet satellite constellation (32 satellites by July 2026) went from 6–10 minute passes in March to windows of over an hour by late August. It has terminals small enough for Geran-type drones [src:euromaidan-2026-vivdych-rassvet-windows].
- **Kill-zone depth, 2026.** The 7th Air Assault Corps commander (Pokrovsk axis) described a kill zone of 20–25 km on both sides of the line in July 2026 and expects 30 km by the end of the year. In his sector drones cause 70–80% of damage [src:euromaidanpress-2026-vivdych-kill-zone-corps].
- **Drone Line results.** In March 2026, Fedorov claimed 1,000+ crews, 1 in 4 front-line kills and 10,500+ Russian troops struck, plus 33,000+ Russian UAVs destroyed by interceptors, and a 1.3:1 strike-drone advantage [src:rbc-2026-fedorov-drone-line]. In June Syrskyi claimed a 1.5:1 FPV advantage and 12.7% month-on-month output growth (April to May) [src:euromaidan-2026-mukhina-syrskyi-fpv-ratio].
- **USF claims.** Brovdi claimed 102,000 Russians killed or wounded in 12 months (to June 2026), with 360,000 targets hit in 1.7m sorties [src:euromaidan-2026-thomas-brovdi-102000]. That is about one hit per five sorties, a derived figure that mixes reconnaissance, bomber and strike sorties.
- **Leadership changes.** Fedorov was dismissed as defence minister on 14 July 2026, and Khmara was appointed on 19 August [src:euromaidan-2026-stanovych-fedorov-dismissal]. The scoring economy kept running: by September 2026 over 500 units were using DOT-Chain/Brave1 Market and the "Army of Drones Bonus", against 12 a year earlier [src:euromaidan-2026-mukhina-dot-chain-500-units].
- **Interceptors.**
  - Output reached up to 1,000 a day, and interceptors made about 70% of Shahed kills around Kyiv in February 2026 [src:justsecurity-2026-bidochko-drone-superpower].
  - Of 3,500+ drones downed in May 2026, USF interceptors claimed 1,200+ and helicopters 440+. Beskrestnov said interceptors now destroy about 50% of incoming Shahed-types, up from about 10% in winter. Syrskyi warned that Russia aims to raise the jet share to 50% [src:kyivpost-2026-syrsky-may-interceptors].
  - The ZIRKA interceptor (June 2026) flies at 340+ km/h, costs up to $2,000 and uses AI to detect and track its target automatically [src:militarnyi-2026-zirka].
- **Shahed volumes.** Monthly Shahed-type launches peaked at 8,161 in May 2026. The overall hit rate fell from about 16.5% (Aug 2025) to 6.7% (May 2026) [src:isis-2026-anokhin-shahed-monthly]. The Air Force reported 65,618 Shaheds and 10,604 Lancets destroyed from Feb 2022 to Aug 2026 [src:euromaidan-2026-murdoch-445000-intercepted]. From about 5 May 2026 the Air Force stopped consistently publishing Shahed counts, so later monthly figures are reconstructions. ISIS reconstructs August 2026 as 4,288 shot down or suppressed and about 716 hits (13.9%), with 57–62% of genuine Shahed/Geran use jet-powered [src:isis-2026-anokhin-shahed-monthly-sep].
- **Jet Gerans.**
  - Russia launched about 2,800 jet-powered drones in August 2026, up from about 350 in June. Only about 60% are intercepted, according to Air Force spokesman Ihnat [src:united24-2026-place-jet-gerans].
  - In September Ukraine downed 55% (more than 1,500 of over 2,730) [src:euromaidan-2026-zoria-jet-kill-rate-55].
  - HUR estimates Russia builds about 3,000 Geran-4/5 a month and has stopped making the Geran-3 [src:euromaidan-2026-thomas-hur-geran-production].
- **Autonomy.**
  - On 6 July 2026 a Molniya with an Nvidia Jetson Orin module picked its own target near a Zaporizhzhia gas station and killed three civilians. This is the first documented self-targeting Russian kill.
  - Jetson modules have turned up in the V2U, Lancet and Klin drones since June 2025 [src:euromaidan-2026-katola-molniya-self-targeting].
- **UGVs.** Monthly logistics and evacuation missions rose from 7,511 in January to 25,143 in August, and Ukraine plans 50,000+ UGVs in 2026 [src:united24-2026-ugv-missions].
- **Heavy bombers.** SkyFall says it makes 100,000 bombers a year at $8,500 each, about 16 times the cost of an FPV. A bomber drops three or four grenades a sortie and may fly a dozen sorties before it is lost, so it is cheaper per mission than a one-use FPV [src:euromaidan-2026-axe-bomber-drones].
- **Operation Vivaldi (Lyman, 3rd Army Corps).**
  - Heavy bombers air-dropped kamikaze UGVs more than 10 km behind Russian lines, and the corps claims 125+ km² cleared [src:euromaidan-2026-mukhina-vivaldi-ugv-drop].
  - The corps says Rubikon was pulled from the sector after losing operators, launch sites and logistics [src:kyivindependent-2026-farrell-rubicon-vivaldi].
- **Russian heavy bombers.** Russia is fielding a Baba Yaga equivalent called "Berdysh" and reportedly repairing captured Vampires [src:pravda-2026-berdysh-russian-heavy-drone].

## 3. Drone type catalog

"n/s" means not stated in the sources gathered here. Ranges are claimed maxima unless noted otherwise.

| type | examples | role | typical range | cost (approx.) | era introduced | EW vulnerability | source id |
|---|---|---|---|---|---|---|---|
| Recon quadcopter | DJI Mavic/Matrice ("mavik"), Autel | spotting, adjusting fire, grenade drops | a few km (line of sight) | n/s (commercial, mostly Chinese parts) | e1-invasion | high (RF link and GNSS) | rusi-2022-zabrodskyi-preliminary-lessons; osw-2025-game-of-drones |
| Fixed-wing ISR (UA) | Leleka-100 (Deviro): 80 km radius, 5 h, 340 km route. A1-CM Furia (Athlon Avia): ≥50 km, 4 h, 200 km route, 6 kg. Shark (Ukrspecsystems): 60 km link, 2+ h in 2022; Shark-M: 180 km link, 7 h, 420 km route in 2026. Also PD-1 and SKIF | ISR, artillery and HIMARS spotting, counter-battery cueing | 50–80 km (Leleka, Furia); 60–180 km link (Shark) | n/s | e1-invasion (in service from 2015; Shark from e3-counteroffensives-22) | medium; dual anti-jam links, GNSS-free modes claimed | rusi-2022-zabrodskyi-preliminary-lessons; deviro-2026-leleka; athlonavia-2026-furia; euromaidan-2022-shark-himars; militarnyi-2026-shark-m |
| Fixed-wing ISR (RU) | Orlan-10/30, Zala, Supercam, Eleron | ISR, artillery cueing, laser designation, EW payload (Leer-3) | operational depth; ~40 km taskable | n/s | e1-invasion | medium; hunted by interceptor UAVs from 2024–25 | rusi-2023-watling-meatgrinder; rusi-2025-watling-reynolds-third-year |
| MALE UCAV | Bayraktar TB2, Orion | strike/ISR | long | n/s | e1-invasion | low EW, high SAM vulnerability | rusi-2022-zabrodskyi-preliminary-lessons |
| RF FPV (analog, then digital) | 7–10" kamikaze quads | anti-personnel, anti-vehicle | ~10 km typical | $300–400 | e4-bakhmut (mass from e6-avdiivka-attrition) | high; 10–15% success under jamming | rusi-2025-watling-reynolds-third-year; justsecurity-2026-bidochko-drone-superpower; csis-2025-bondar-ai-autonomy |
| AI terminal-guidance FPV | machine-vision lock-on modules (e.g. ZIR, Skynode) | strike through jamming in final approach | lock-on at ~1 km (ZIR up to 3 km) | ~$150 per kit (Brave1) | e6-avdiivka-attrition (10,000 bought in 2024) | lower in terminal phase; 70–80% success claimed | csis-2025-bondar-ai-autonomy; euromaidan-2025-kirichenko-ai-swarms; osw-2025-game-of-drones |
| Fiber-optic FPV | Russian KVN and ring-wing KVS, Ukrainian types (optovolokno) | EW-proof strike, ambush, logistics interdiction | ~10 km (early 2025), then 40+ km (UA, Jul 2025) and 50–60 km (RU, 2026) | n/s | e7-kursk-pokrovsk | immune to RF jamming; cable snags or breaks | rusi-2025-watling-reynolds-third-year; ukrinform-2025-fedorov-fiber-optic-40km; isw-2026-feb-26-assessment; euromaidan-2026-axe-ring-fpv |
| Heavy bomber multicopter | Vampire/Baba Yaga (SkyFall, UA); Berdysh and Vampire copies (RU) | night bombing, mining, resupply, UGV air-drop; reusable (~12 sorties) | Vampire 20 km (with a ~15 kg payload); Berdysh 25 km (claimed) | ~$8,500 (Vampire) | e5-counteroffensive-23 (agricultural conversions) | medium; slow, hovers, mostly night | rusi-2023-watling-stormbreak; euromaidan-2026-zoria-russian-vampire-copies; euromaidan-2026-axe-bomber-drones; pravda-2026-berdysh-russian-heavy-drone; euromaidan-2026-mukhina-vivaldi-ugv-drop |
| Loitering munition | Lancet-1/3/3M (Izdeliye-51/52), Izdeliye-53 (claimed), Kub | counter-battery, SAM, radar and airfield hunting | ~65 km shown (2023) | ~$35,000 (Lancet, estimate) | e3-counteroffensives-22 | medium; nets and cages until pre-detonating warheads; FPV interceptors | oryx-2022-lancet-kill-list; kyivindependent-2023-farrell-lancet; euromaidan-2023-shandra-lancet-new-model |
| Cheap fixed-wing strike | Molniya / Molniya-2 | logistics strikes 20–50+ km deep; FPV carrier; AI self-targeting (2026) | ~30 km, 50+ km in use | ~UAH 35,000 | e8-drone-kill-zone | medium; fiber variants exist | mezha-2026-molniya; euromaidan-2026-katola-molniya-self-targeting |
| Long-range one-way attack (RU) | Shahed-136/Geran-2; Gerbera/Parody decoys | strikes on cities, energy, airfields | hundreds of km+ | $20k–50k; CSIS midpoint $35k | e3-counteroffensives-22 | GNSS jamming helps; ~90% shot down 2022–24; Starlink link lost in Feb 2026 | csis-2025-jensen-drone-saturation; csis-2025-hollenbeck-shahed-cost-effectiveness; isis-2026-anokhin-shahed-2025 |
| Jet one-way attack (RU) | Geran-3/4/5, S8000 Banderol | fast strike that shortens reaction time | up to ~1,000 km | n/s | e8-drone-kill-zone | beats slower interceptors; 55–60% intercepted (Aug–Sep 2026) | united24-2026-place-jet-gerans; euromaidan-2026-zoria-jet-kill-rate-55; isis-2026-anokhin-shahed-2025 |
| Long-range strike (UA) | Liutyi (An-196), others | refineries, airbases | ~1,000 km+ | $55k up to ~$400k | e6-avdiivka-attrition (scaled in e8-drone-kill-zone) | high attrition; <30% reach target area | defensenews-2025-ap-long-range-drones; justsecurity-2026-bidochko-drone-superpower |
| Covert short-range deep strike | Spiderweb truck-launched FPVs | strategic surprise | launched near target | n/s | e8-drone-kill-zone | used Russian mobile networks | kyivindependent-2025-york-spiderweb |
| Interceptor (anti-Shahed) | Sting, ZIRKA, P1-SUN | kill Shahed/Gerbera | ZIRKA 30 km radius | $1,000–2,500 | e7-kursk-pokrovsk (Sting, autumn 2024) | radar-cued; AI tracking | defenceexpress-2025-sting-effectiveness; militarnyi-2026-zirka; justsecurity-2026-bidochko-drone-superpower |
| Interceptor (anti-recon) | FPV-type interceptors vs Orlan/Zala/Supercam | counter-ISR | n/s | n/s | e7-kursk-pokrovsk | radar/electro-optically cued | rusi-2025-watling-reynolds-third-year |
| Relay / "mothership" | Molniya FPV carrier; Starlink-fitted RU drones; repeaters | range extension, carrying smaller drones | RU Starlink-linked drones 20–200 km | n/s | e8-drone-kill-zone | Starlink cut off Feb 2026 | mezha-2026-molniya; euromaidan-2026-axe-starlink-counterattacks |
| UGV (NRK) | logistics carts, TW-12.7 armed "Droid", kamikaze UGVs | logistics, casevac, mining, strike | short (RF/mesh) | n/s | e7-kursk-pokrovsk (scaled in e8-drone-kill-zone and e9-counteroffensive-26) | RF-dependent; RU UGVs lost Starlink | united24-2026-ugv-missions; pravda-2025-hubina-3m-fpv |
| Naval drone | Magura V5/V7 (HUR), Sea Baby and underwater Sea Baby (SBU) | anti-ship, anti-tanker, anti-air with missiles, submarine strike | n/s | n/s | e4-bakhmut to e6-avdiivka-attrition | n/s | euromaidan-2024-mukhina-hur-magura-ships; euromaidan-2025-kravchuk-magura-su30; kyivindependent-2025-zadorozhnyy-kairos-virat; kyivindependent-2025-myronyshena-sub-sea-baby |

## 4. Key figures

| figure | value | unit | era | source id |
|---|---|---|---|---|
| Ukrainian UAVs destroyed, Feb–Jul 2022 | ~90 | % | e1-invasion; e2-donbas-artillery | rusi-2022-zabrodskyi-preliminary-lessons |
| Quadcopter / fixed-wing life expectancy | ~3 / ~6 | flights | e1-invasion; e2-donbas-artillery | rusi-2022-zabrodskyi-preliminary-lessons |
| UAV missions judged successful | ~33 | % | e1-invasion; e2-donbas-artillery | rusi-2022-zabrodskyi-preliminary-lessons |
| Russian sensor-to-shooter time with organic UAV | 3–5 | minutes | e2-donbas-artillery | rusi-2022-zabrodskyi-preliminary-lessons |
| Ukrainian FPV output in 2022 | 3,000–5,000 | units | e1-invasion; e2-donbas-artillery; e3-counteroffensives-22 | justsecurity-2026-bidochko-drone-superpower |
| Army of Drones first 9 months | 3,200 complexes / >$107m / 7,000+ operators | mixed | e2-donbas-artillery; e3-counteroffensives-22; e4-bakhmut | pravda-2023-fedorov-army-of-drones |
| Lancet/Kub hits logged by Oryx (to Mar 2023) | 113 hits / 41 misses | strikes | e3-counteroffensives-22; e4-bakhmut | oryx-2022-lancet-kill-list |
| Lancets destroyed, Feb 2022–Aug 2026 (Air Force) | 10,604 | units | e3-counteroffensives-22 to e9-counteroffensive-26 | euromaidan-2026-murdoch-445000-intercepted |
| Ukrainian UAV losses | ~10,000 | per month | e4-bakhmut | rusi-2023-watling-meatgrinder |
| Russian major EW density | ≥1 per 10 km | systems/front | e4-bakhmut | rusi-2023-watling-meatgrinder |
| Shahed-type launches by year (Air Force reports, compiled) | ≥409 (Sep–Dec 2022), ~3,100 (2023), ~11,100 (2024), ~54,800 (2025), ~16,200 (Q1 2026) | drones | e3-counteroffensives-22 to e9-counteroffensive-26 | kaggle-2026-ivaniuk-missile-attacks |
| Shahed-type launches by quarter | 402 (Q4 22), 242, 685, 916, 1,280 (Q1–Q4 23), 1,294, 973, 2,550, 6,264 (Q1–Q4 24) | drones | e3-counteroffensives-22 to e7-kursk-pokrovsk | kaggle-2026-ivaniuk-missile-attacks |
| One-way attack drones, 28 Sep 2022–end 2024 | >14,700, ~90% shot down | drones | e3-counteroffensives-22 to e7-kursk-pokrovsk | csis-2025-hollenbeck-shahed-cost-effectiveness |
| Shahed cost per target struck (CSIS) | ~350,000 | USD | e3-counteroffensives-22 to e7-kursk-pokrovsk | csis-2025-hollenbeck-shahed-cost-effectiveness |
| Ukrainian UAV output in 2024 (all types) | ~2.2m (OSW), ~2m (CSIS); ~1.2m FPV received (implied from MoD) | units | e6-avdiivka-attrition | osw-2025-game-of-drones; csis-2025-bondar-ai-autonomy; pravda-2025-hubina-3m-fpv |
| Share of damaged/destroyed Russian systems caused by tactical UAVs | 60–70 | % | e6-avdiivka-attrition; e7-kursk-pokrovsk | rusi-2025-watling-reynolds-third-year |
| FPVs failing to reach their target | 60–80 | % | e6-avdiivka-attrition; e7-kursk-pokrovsk | rusi-2025-watling-reynolds-third-year |
| FPV mission success without / with autonomous terminal guidance | 10–15 / 70–80 | % | e7-kursk-pokrovsk; e8-drone-kill-zone | csis-2025-bondar-ai-autonomy |
| Drones per target without / with terminal guidance | 8–9 / 1–2 | drones | e7-kursk-pokrovsk; e8-drone-kill-zone | csis-2025-bondar-ai-autonomy |
| USF sorties vs targets hit (12 months to Jun 2026, claim) | 1.7m / 360,000 | sorties / targets | e8-drone-kill-zone; e9-counteroffensive-26 | euromaidan-2026-thomas-brovdi-102000 |
| Share of Russian casualties from drones (Ukrainian claims) | 70–80 | % | e8-drone-kill-zone | osw-2025-game-of-drones |
| Share of damage caused by drones, 7th Corps sector (Jul 2026) | 70–80 | % | e9-counteroffensive-26 | euromaidanpress-2026-vivdych-kill-zone-corps |
| Dense ISR transparency / diminishing / taskable depth | 3 / 15 / 40 | km | e6-avdiivka-attrition; e7-kursk-pokrovsk | rusi-2025-watling-reynolds-third-year |
| Russian Orlan/Zala orbits over Ukraine (Aug 2024) | 1,000–1,500 | per day | e7-kursk-pokrovsk | rusi-2025-watling-reynolds-third-year |
| Wire-guided FPV range (early 2025) | ~10 | km | e7-kursk-pokrovsk | rusi-2025-watling-reynolds-third-year |
| Fiber-optic FPV range | 20 (spring 2025), then 40+ (Jul 2025, UA claim); up to 60 (RU, ISW) | km | e8-drone-kill-zone; e9-counteroffensive-26 | ukrinform-2025-fedorov-fiber-optic-40km; isw-2026-feb-26-assessment |
| Fibre share of Russian FPVs in key sectors (2026) | ~60 | % | e9-counteroffensive-26 | euromaidan-2026-axe-unjammable-russian-drones |
| Drone Line kill-zone target | 10–15 | km deep | e8-drone-kill-zone; e9-counteroffensive-26 | kyivindependent-2025-goncharova-drone-line-launch; rbc-2026-fedorov-drone-line |
| Kill-zone depth, Pokrovsk axis (Jul 2026) | 20–25, with 30 planned by end-2026 | km | e9-counteroffensive-26 | euromaidanpress-2026-vivdych-kill-zone-corps |
| Molniya strike depth | 50+ | km | e8-drone-kill-zone; e9-counteroffensive-26 | mezha-2026-molniya |
| Shahed launches | ~130/wk (pre-Sep 2024), then ~1,100/wk (Mar 2025) | per week | e7-kursk-pokrovsk | csis-2025-jensen-drone-saturation |
| Shahed-type launches in 2025 | 54,538 (~40% decoys) | drones | e8-drone-kill-zone | isis-2026-anokhin-shahed-2025 |
| Shahed hit rate | 2–3% (early 2025), ~17% (Dec 2025), 6.7% (May 2026) | % | e8-drone-kill-zone; e9-counteroffensive-26 | isis-2026-anokhin-shahed-2025; isis-2026-anokhin-shahed-monthly |
| Shahed-type launches, record month | 8,161 (May 2026) | drones | e9-counteroffensive-26 | isis-2026-anokhin-shahed-monthly |
| Interceptor share of Shahed-type kills (Beskrestnov) | ~10 (winter 2025–26), then ~50 (May 2026) | % | e9-counteroffensive-26 | kyivpost-2026-syrsky-may-interceptors |
| Shahed unit cost | 20k–50k (midpoint 35k) | USD | e7-kursk-pokrovsk; e8-drone-kill-zone | csis-2025-jensen-drone-saturation; csis-2025-hollenbeck-shahed-cost-effectiveness |
| Russian production targets for 2025 (SZRU) | 2m FPV; 30,000 long-range; 30,000 decoys | units | e8-drone-kill-zone | kyivindependent-2025-zadorozhnyy-russia-2m-fpv |
| Russian Shahed-type production plan for 2025 (HUR) | 79,000 | units | e8-drone-kill-zone | kyivindependent-2025-zadorozhnyy-79000-shahed |
| Russian Geran-4/5 jet output (HUR, Aug 2026) | ~3,000 | per month | e9-counteroffensive-26 | euromaidan-2026-thomas-hur-geran-production |
| FPV unit cost | 300–400 | USD | e8-drone-kill-zone; e9-counteroffensive-26 | justsecurity-2026-bidochko-drone-superpower |
| Heavy bomber (Vampire) cost / output / sorties before loss | ~$8,500 / 100,000 a year / ~12 | mixed | e9-counteroffensive-26 | euromaidan-2026-axe-bomber-drones |
| Sting interceptor cost / efficiency | ~$2,000–2,500 / 60–90% | USD / % | e8-drone-kill-zone | defenceexpress-2025-sting-effectiveness; justsecurity-2026-bidochko-drone-superpower |
| Interceptor deliveries (Dec 2025) | ~950 | per day | e8-drone-kill-zone | kmu-2025-950-interceptors |
| FPVs obtained in 2025 | 3m | units | e8-drone-kill-zone | pravda-2025-hubina-3m-fpv |
| UGVs delivered in 2025 | ~15,000 | units | e8-drone-kill-zone | pravda-2025-hubina-3m-fpv |
| Spiderweb (claimed) | 117 drones; 41 aircraft hit; ~$7bn | mixed | e8-drone-kill-zone | kyivindependent-2025-york-spiderweb |
| Liutyi range / cost / arrival rate | ~1,000 km / from $55k / <30% reach target | mixed | e8-drone-kill-zone | defensenews-2025-ap-long-range-drones |
| Russians "neutralised" by USF (Dec 2025, Ukrainian claim) | ~33,000 | personnel/month | e8-drone-kill-zone | euromaidan-2026-murdoch-syrskyi-december |
| USF share of Ukrainian personnel | ~2.2 | % | e8-drone-kill-zone; e9-counteroffensive-26 | euromaidan-2026-murdoch-syrskyi-december |
| E-points spending since Aug 2025 | >5bn | UAH | e8-drone-kill-zone | digitalstate-2025-brave1-epoints |
| Units ordering through DOT-Chain/Brave1 Market | 12 (Sep 2025), then 500+ (Sep 2026) | units | e8-drone-kill-zone; e9-counteroffensive-26 | euromaidan-2026-mukhina-dot-chain-500-units |
| Drone Line crews / share of kills (Mar 2026 claim) | 1,000+ / 25% | crews / % | e9-counteroffensive-26 | rbc-2026-fedorov-drone-line |
| Drone advantage claims | 1.3:1 strike drones (Fedorov, Mar 2026); 1.5:1 FPV (Syrskyi, Jun 2026) | ratio | e9-counteroffensive-26 | rbc-2026-fedorov-drone-line; euromaidan-2026-mukhina-syrskyi-fpv-ratio |
| Russian UAVs destroyed by interceptors (Mar 2026 claim) | 33,000+ | per month | e9-counteroffensive-26 | rbc-2026-fedorov-drone-line |
| Interceptor share of Shahed kills, Kyiv region (Feb 2026) | ~70 | % | e9-counteroffensive-26 | justsecurity-2026-bidochko-drone-superpower |
| Russian drone effectiveness lost to the Starlink cut-off (Biletsky, first two weeks) | 20–40 | % | e9-counteroffensive-26 | isw-2026-feb-26-assessment |
| Share of Russian drones using Starlink (estimate) | single-digit | % | e8-drone-kill-zone | euromaidan-2026-kossov-starlink-russian-drones |
| Jet-drone launches, Jun to Aug 2026 | ~350, then ~2,800 | per month | e9-counteroffensive-26 | united24-2026-place-jet-gerans |
| Jet-drone interception rate | ~60 (Aug), 55 (Sep 2026) | % | e9-counteroffensive-26 | united24-2026-place-jet-gerans; euromaidan-2026-zoria-jet-kill-rate-55 |
| UGV logistics/casevac missions | 7,511 (Jan), then 25,143 (Aug 2026) | per month | e9-counteroffensive-26 | united24-2026-ugv-missions |
| UGV production plan for 2026 | 50,000+ | units | e9-counteroffensive-26 | united24-2026-ugv-missions |
| Rubikon personnel | ~1,450 (Mar 2025), then ~5,000 (spring 2026) | people | e8-drone-kill-zone; e9-counteroffensive-26 | fpri-2026-lee-putiata-rubicon |

## 5. Russia's side as reported

All of the following comes from Ukrainian or Western sources.

- **2022–23: recon plus artillery.**
  - Orlan-10 complexes cued massed artillery and later Krasnopol laser-guided shells.
  - Lancet became the main counter-battery tool. It reached about 65 km by autumn 2023, and ZALA kept modifying it for range and accuracy. The claimed autonomous, tube-launched Izdeliye-53 had not been seen in mass use by November 2023 [src:kyivindependent-2023-farrell-lancet].
  - Dense EW (at least one major system per 10 km) cost Ukraine about 10,000 UAVs a month [src:rusi-2022-zabrodskyi-preliminary-lessons][src:rusi-2023-watling-meatgrinder][src:oryx-2022-lancet-kill-list].
- **2024–25: centralization and fiber.**
  - **Rubikon** (founded Aug 2024) is a hybrid unit that does R&D, training and combat. It prioritises Ukrainian drone teams, EW and logistics over infantry [src:rferl-2025-rubicon].
  - Rubikon detachments grew from 149 to 474 people, mixing FPV, fixed-wing, Supercam/Lancet recon-strike and counter-UAS elements. It is short of skilled recruits, partly because it competes with the 50th "Varyag" brigade for talent [src:fpri-2026-lee-putiata-rubicon].
  - Russia moved to fiber-optic FPVs earlier and at larger scale than Ukraine [src:rusi-2025-watling-reynolds-third-year]. It gets most of its fibre directly from China [src:euromaidan-2026-axe-unjammable-russian-drones].
- **Production (Ukrainian intelligence estimates).**
  - Foreign intelligence (SZRU) said in June 2025 that Russia aimed to build 2m FPVs, 30,000 long-range drones and 30,000 decoys in 2025 [src:kyivindependent-2025-zadorozhnyy-russia-2m-fpv].
  - HUR (Skibitskyi) put the 2025 Shahed-type plan at 79,000 [src:kyivindependent-2025-zadorozhnyy-79000-shahed].
  - HUR put jet Geran-4/5 output at about 3,000 a month by August 2026 [src:euromaidan-2026-thomas-hur-geran-production].
  - Zelensky, citing HUR briefings on Russian documents, says Russia plans $12bn in 2027 for jet drones and loitering munitions [src:euromaidan-2026-zoria-jet-kill-rate-55]. He says North Korea has begun producing Shahed-type drones with Russian help [src:euromaidan-2026-zoria-nk-shahed].
  - Kyiv Post, citing Bloomberg, reports intelligence estimates that Russia plans up to 7.3m FPVs in 2026, after drone output rose 117% year on year in April 2026 [src:kyivpost-2026-syrsky-may-interceptors].
  - No Ukrainian or Western estimate of Russian FPV output for 2023–24 was found.
- **Force size.**
  - Syrskyi says Russian drone forces number about 80,000, with plans for 165,500 in 2026 and about 210,000 by 2030 [src:euromaidan-2026-murdoch-syrskyi-december].
  - FPRI cites a Russian USF target of 165,000 personnel by the end of 2026, with recruitment falling short [src:fpri-2026-lee-putiata-rubicon].
- **Cheap depth weapons.**
  - Molniya/Molniya-2 are plywood fixed-wing drones (about UAH 35k) used against logistics 50+ km deep, including FPV-carrier and Starlink recon versions [src:mezha-2026-molniya]. By July 2026 at least one carried a Jetson module that selected its own target [src:euromaidan-2026-katola-molniya-self-targeting].
  - Before February 2026, Russian Starlink-linked strike drones reached 20–200 km [src:euromaidan-2026-axe-starlink-counterattacks]. Afterwards Russia pushed fibre FPVs out to 50 km with the ring-wing KVS [src:euromaidan-2026-axe-ring-fpv].
- **Strategic strikes.**
  - Launches ran at about 5,000–6,000 Shahed-type drones a month through late 2025, with about 40% decoys [src:isis-2026-anokhin-shahed-2025], rising to a record 8,161 in May 2026 [src:isis-2026-anokhin-shahed-monthly].
  - Russia shifted to jet Geran-3/4/5 and the Banderol in 2026 [src:united24-2026-place-jet-gerans]. HUR says Geran-3 production has stopped in favour of the Geran-4/5 [src:euromaidan-2026-thomas-hur-geran-production].
  - Syrskyi says Russia produces over 400 long-range drones a day [src:euromaidan-2026-murdoch-syrskyi-december].
  - These production estimates vary a lot and are not independently confirmed.
- **Heavy bombers.** Russia lagged Ukraine here. It fielded hexacopter Vampire copies in 2025 [src:euromaidan-2026-zoria-russian-vampire-copies], has now unveiled the Berdysh (claimed 20 kg payload, 25 km range) and reportedly reuses captured Vampires [src:pravda-2026-berdysh-russian-heavy-drone].
- **Weak points in 2026.**
  - Losing Starlink hurt its long-reach drones, its UGVs and its command and control [src:euromaidan-2026-axe-starlink-counterattacks][src:isw-2026-feb-26-assessment]. The Rassvet constellation is the planned replacement [src:euromaidan-2026-vivdych-rassvet-windows].
  - Operation Vivaldi reportedly forced Rubikon out of the Lyman sector [src:kyivindependent-2026-farrell-rubicon-vivaldi].

## 6. Game/sim relevance

- **Kill-zone depth by era.** Model drone threat as a band with a density gradient that deepens over time:
  - 2022: sparse and short-range.
  - 2024: dense to about 3 km, thinning to 15 km, taskable to 40 km [src:rusi-2025-watling-reynolds-third-year].
  - 2025: 10–15 km lethal, with fiber-optic FPVs and Molniya reaching 40–50 km [src:ukrinform-2025-fedorov-fiber-optic-40km][src:mezha-2026-molniya].
  - 2026: 20–25 km on active axes, with 30 km expected by year-end [src:euromaidanpress-2026-vivdych-kill-zone-corps].

  Vehicles and logistics inside the band suffer attrition every turn unless they are dug in, move at night or are escorted by EW.
- **Split the band by link type.** A political event (the Starlink cut-off) removed Russia's satellite-linked deep layer but left its fibre layer intact. Biletsky's 20–40% effectiveness drop, with partial recovery over one to two months, is a usable event curve [src:isw-2026-feb-26-assessment][src:kyivindependent-2026-post-starlink-scramble].
- **The EW vs link-type triangle.** RF FPVs are cheap but jammable, with a 60–80% failure rate (RUSI) or 10–15% success (CSIS) [src:rusi-2025-watling-reynolds-third-year][src:csis-2025-bondar-ai-autonomy]. Fiber drones cannot be jammed but have limited range and snag risk. In frost, ice stops spools unwinding and iced cable snaps, so range and link reliability should fall with temperature [src:euromaidan-2026-kossov-icy-fibre]. AI terminal guidance raises success to a claimed 70–80% in the last metres [src:csis-2025-bondar-ai-autonomy]. Satellite links (Starlink) extend reach but can be revoked, as in February 2026 [src:kyivindependent-2026-starlink-catastrophe]. The USF's 360,000 targets hit in 1.7m sorties (about 1 in 5) is a sanity check on aggregate rates [src:euromaidan-2026-thomas-brovdi-102000].
- **Production and budget economy.** Drones are a consumable throughput resource, measured in millions per year [src:pravda-2025-hubina-3m-fpv], rather than a unit roster. Cost asymmetry drives air defence: a $2k interceptor against a $20–50k Shahed [src:csis-2025-jensen-drone-saturation][src:defenceexpress-2025-sting-effectiveness], and jet Gerans reset that balance [src:united24-2026-place-jet-gerans]. A reusable bomber at $8,500 for about a dozen sorties is cheaper per mission than one-use FPVs [src:euromaidan-2026-axe-bomber-drones].
- **Strategic-strike pressure curve.** The quarterly Shahed series (about 400 in Q4 2022, about 1,300 in Q4 2023, about 6,300 in Q4 2024, about 16,000 in Q4 2025) can drive the Russian strategic-strike track. The Air Force's shot-down share is also a ready-made air-defence difficulty setting: over 90% early, falling as volumes and jets rise [src:kaggle-2026-ivaniuk-missile-attacks][src:isis-2026-anokhin-shahed-monthly].
- **E-points as an in-game currency.** Verified kills uploaded to DELTA earn points that units spend in a Brave1 Market catalogue [src:digitalstate-2025-brave1-epoints]. This maps directly to a player economy, including the moral hazard of chasing points. The programme survived a change of defence minister [src:euromaidan-2026-mukhina-dot-chain-500-units].
- **Robots replacing people in the kill zone.** UGV logistics and casevac can remove soldiers from the last 10 km; the monthly mission curve for 2026 is a ready-made tech-adoption curve [src:united24-2026-ugv-missions]. Late-game options include UGVs air-dropped by heavy bombers [src:euromaidan-2026-mukhina-vivaldi-ugv-drop].
- **Counter-drone units as priority targets.** Rubikon-style units should AI-target the player's drone teams, EW and supply routes rather than infantry [src:rferl-2025-rubicon]. Destroying their operators and launch sites should degrade Russian drone density across a sector [src:kyivindependent-2026-farrell-rubicon-vivaldi].

## 7. Terms

- ptashky / ptakhy (пташки / птахи) — "little birds" / "birds"; soldiers' slang for drones (hence "Magyar's Birds", Ptakhy Madiara).
- mavik (мавік) — any small recon quadcopter, from DJI Mavic.
- FPV / fipivi — first-person-view kamikaze quadcopter.
- skyd (скид) — a drop munition released from a drone; "skydy" in the plural.
- Baba Yaga / Babka — the Slavic folk witch; Russian soldiers' name for Ukrainian heavy night bombers such as the Vampire, adopted by Ukrainians. Russia's own copies carry the same nickname.
- Vampir (Vampire) — a SkyFall heavy hexacopter bomber.
- optovolokno / optyka (оптоволокно) — fiber-optic (tethered) FPV.
- REB (РЕБ) — radio-electronic warfare, i.e. EW or jamming.
- sira zona (сіра зона) — the "grey zone" of contested ground between the lines, now roughly the drone kill zone.
- kill zone / zona urazhennia — the band where drones make movement lethal; the Drone Line target is 10–15 km.
- Liniia droniv — the Drone Line project.
- ye-baly / e-bally (Є-бали) — e-points earned for verified kills and spent on Brave1 Market.
- DELTA — Ukraine's situational-awareness system, used to verify e-point claims.
- Brave1 — the government defence-tech cluster and marketplace.
- SBS (Syly bezpilotnykh system) — the Unmanned Systems Forces (USF).
- NRK (наземний роботизований комплекс) — ground robotic complex, i.e. a UGV.
- perekhopliuvach (перехоплювач) — interceptor drone.
- shakhed / moped (мопед) — Shahed/Geran; "moped" after the sound of its engine.
- Liutyi (Лютий) — "fierce"; a long-range strike drone.
- Molniya (Блискавка, Blyskavka) — "lightning"; a Russian plywood fixed-wing strike drone.
- Rubikon (Рубікон) — Russia's Center for Advanced Unmanned Technologies.
- izdeliye (изделие, Ukrainian vyrib) — "product"; a Russian industrial designation, as in Izdeliye-53 (a Lancet successor).
- Madyar / Magyar — call sign of Robert Brovdi, USF commander from June 2025. "Madyar's Birds" and "Magyar's Birds" are the same unit, the 414th brigade.
- Pavutyna (Павутина) — "Spiderweb", the SBU's June 2025 operation against Russian bomber bases.

## 8. Open questions / gaps

- **Russian FPV production for 2023–24.** No Ukrainian or Western estimate was found; only the SZRU's 2025 target of 2m exists. Russian claims were deliberately not used. The long-range production figures conflict: 79,000 Shahed-types a year (HUR) against 400+ a day (Syrskyi). Searched Kyiv Independent, Euromaidan Press and Militarnyi. RUSI and CSIS site searches did not render.
- **Lancet strike counts after March 2023.** WarSpotting's Lancet tracker blocks scripted access, and its public API covers only Russian losses. Oryx stopped in March 2023, and the Russian LostArmour tally is excluded by the sourcing rules. The Air Force's 10,604 Lancets destroyed (to Aug 2026) is the only volume indicator. Lancet ranges after 2023 are reported only qualitatively.
- **Shahed data before mid-2024 and after May 2026.** The compiled Air Force series is a floor for 2022–23, because some early reports gave only the number destroyed. From May 2026 the Air Force stopped publishing consistent counts, so ISIS figures for the later months are reconstructions. The dataset licence (CC BY-NC-SA) means it should be cited, not bundled.
- **Kill-zone depth** is still commanders' estimates. No systematic measurement of the Russian band after February 2026 exists, beyond ISW's qualitative "reduced tempo and depth".
- **Casualty-share figures.** The 60–70% (systems, RUSI) and 70–80% (casualties, Ukrainian officials via OSW, and damage in the 7th Corps sector) figures measure different things; none is audited, and Ukrainian casualties from Russian drones are poorly quantified.
- **Hit rates.** These remain definition-dependent. RUSI's 60–80% failure rate, CSIS's 10–15% success without guidance, OSW's 30% → 70% accuracy and the USF's claim of roughly one target per five sorties are not comparable. No 2026 field study was found, and manufacturer interceptor rates (60–90%) are self-reported.
- **Heavy bombers.** Vampire specs come from press relaying SkyFall; the maker's own site did not resolve. The Russian "Kashchei" was searched again and not found in any English-language outlet, so it is dropped. The Berdysh specs are relayed from Russian Telegram.
- **Fixed-wing ISR costs** are not published by Deviro, Athlon Avia or Ukrspecsystems.
- **Swarms.** Swarm use is small-scale (Swarmer's "over a hundred" missions by Sep 2025; CSIS calls it experimental). No adoption share for AI terminal guidance beyond the 10,000 bought in 2024 was found.

## 9. Sources

- rusi-2022-zabrodskyi-preliminary-lessons — RUSI with Ukrainian co-authors; 2022 UAV attrition and Orlan kill-chain baseline.
- pravda-2023-fedorov-army-of-drones — Army of Drones' first 9 months (3,200 UAVs, 7,000 operators).
- oryx-2022-lancet-kill-list — OSINT tally of Lancet/Kub hits and misses to March 2023.
- rusi-2023-watling-meatgrinder — about 10,000 Ukrainian UAVs lost a month; Russian EW density; Lancet counter-battery.
- rusi-2023-watling-stormbreak — 2023 offensive; agricultural-drone night bombing; Russian Lancet/FPV integration.
- rusi-2025-watling-reynolds-third-year — the key 2024 data: 60–70% of kills, 60–80% FPV failure, transparency depths.
- csis-2025-jensen-drone-saturation — Shahed ramp from Sep 2024 to Mar 2025; costs; Geran-3.
- csis-2025-hollenbeck-shahed-cost-effectiveness — 14,700 one-way drones 2022–24; $35k midpoint; cost per target struck.
- csis-2025-bondar-ai-autonomy — Ukraine's AI autonomy: 10–15% vs 70–80% success, swarms experimental.
- kaggle-2026-ivaniuk-missile-attacks — compiled Air Force reports; monthly Shahed series 2022–26.
- isis-2026-anokhin-shahed-monthly — monthly Shahed launches and hit rates, Aug 2025–Jun 2026.
- isis-2026-anokhin-shahed-monthly-sep — updated series; Air Force reporting gap from May 2026; August 2026 reconstruction.
- kyivpost-2026-syrsky-may-interceptors — May 2026 interceptor kills; interceptor share; Russian FPV plans for 2026.
- osw-2025-game-of-drones — Ukrainian production 2024–25, casualty shares, Chinese parts, Drone Line.
- rferl-2025-rubicon — Rubikon's role in Kursk, its structure and its target mix.
- fpri-2026-lee-putiata-rubicon — Rubikon org chart and growth to 2026; Russian USF targets.
- ukrinform-2025-fedorov-fiber-optic-40km — Ukrainian fiber-optic range goes from 20 to 40+ km.
- militarnyi-2025-brovdi-usf — Magyar appointed USF commander; history of Magyar's Birds.
- kyivindependent-2025-goncharova-drone-line-launch — Drone Line announced 9 Feb 2025; five units; 10–15 km.
- kyivindependent-2025-hodunova-usf-drone-line-group — USF and Drone Line under one command group, 20 Jun 2025.
- euromaidan-2026-mukhina-drone-line-880m — Drone Line operating since March 2025; $880m.
- kyivindependent-2025-york-spiderweb — Operation Spiderweb facts and claims.
- defensenews-2025-ap-long-range-drones — Liutyi range and cost, success rate, the refinery campaign.
- defenceexpress-2025-sting-effectiveness — Sting interceptor speed, cost, efficiency.
- kmu-2025-950-interceptors — official figure of about 950 interceptors a day (Dec 2025).
- isis-2026-anokhin-shahed-2025 — full-year 2025 Shahed statistics and hit rates.
- kyivindependent-2025-zadorozhnyy-79000-shahed — HUR's 2025 Shahed plan; July 2025 record month.
- kyivindependent-2025-zadorozhnyy-russia-2m-fpv — SZRU: Russia targets 2m FPVs in 2025.
- kyivindependent-2023-farrell-lancet — Lancet at 65 km; Izdeliye-53 claims and HUR's rebuttal.
- euromaidan-2023-shandra-lancet-new-model — pre-detonating Lancet; weight; ~$35k.
- euromaidan-2026-murdoch-445000-intercepted — Air Force cumulative kills, including 10,604 Lancets.
- pravda-2025-hubina-3m-fpv — 3m FPVs and 15k UGVs obtained in 2025.
- digitalstate-2025-brave1-epoints — official account of e-points and Brave1 Market.
- united24-2025-khomenko-adb-launch — Army of Drones Bonus launch; kill points from 2 to 6.
- kyivpost-2026-korshak-epoints — e-points price list, January 2026.
- euromaidanpress-2026-epoints-400-units — e-points extended to recon, logistics and evacuation.
- euromaidan-2026-mukhina-dot-chain-500-units — 500+ units on DOT-Chain/Brave1 Market by Sep 2026.
- euromaidan-2026-stanovych-fedorov-dismissal — Fedorov dismissed 14 Jul 2026; Khmara appointed.
- mezha-2026-molniya — Russian Molniya drone: specs, variants, depth.
- euromaidan-2026-katola-molniya-self-targeting — first documented self-targeting Russian drone kill; Jetson modules.
- euromaidan-2025-kirichenko-ai-swarms — Swarmer missions; AI kit cost; V2U groups.
- euromaidan-2026-murdoch-syrskyi-december — Syrskyi's December 2025 drone kill and recruitment claims.
- kyivindependent-2026-starlink-catastrophe — Russian Starlink cut-off on 5 Feb 2026.
- isw-2026-feb-26-assessment — ISW: Starlink block from 1 Feb; 20–40% effect; fibre to 60 km.
- euromaidan-2026-axe-starlink-counterattacks — which Russian drones lost Starlink; the start of the southern counterattack.
- euromaidan-2026-kossov-starlink-russian-drones — Starlink-linked Russian drone types; single-digit share.
- kyivindependent-2026-post-starlink-scramble — Russian workarounds; fibre lead; Rassvet not yet flying.
- euromaidan-2026-vivdych-rassvet-windows — Rassvet coverage growth, Mar to Aug 2026.
- euromaidan-2026-axe-ring-fpv — Russian ring-wing fibre FPV to 50 km.
- euromaidan-2026-axe-unjammable-russian-drones — 60% fibre share; Chinese fibre supply.
- euromaidan-2026-vivdych-ng-fibre-70 — National Guard drones 70% fibre-optic.
- euromaidan-2026-kossov-icy-fibre — frost and icing degrade fibre drones.
- euromaidanpress-2026-vivdych-kill-zone-corps — 20–25 km kill zone, 30 km planned (7th Corps).
- justsecurity-2026-bidochko-drone-superpower — production trajectory, costs, interceptor share.
- rbc-2026-fedorov-drone-line — Drone Line results for March 2026 (Fedorov).
- euromaidan-2026-mukhina-syrskyi-fpv-ratio — Syrskyi's 1.5:1 FPV advantage; output growth.
- euromaidan-2026-thomas-brovdi-102000 — USF claims: 102k casualties, 360k targets, 1.7m sorties.
- militarnyi-2026-zirka — ZIRKA AI-guided interceptor.
- pravda-2026-berdysh-russian-heavy-drone — Russian Berdysh heavy bomber; captured Vampires.
- euromaidan-2026-zoria-russian-vampire-copies — Vampire specs; Russian hexacopter copies.
- euromaidan-2026-axe-bomber-drones — SkyFall output and cost; bomber reuse economics.
- euromaidan-2022-shark-himars — original Shark specs and role.
- militarnyi-2026-shark-m — Shark-M specs.
- deviro-2026-leleka — maker specs for Leleka.
- athlonavia-2026-furia — maker specs for A1-CM Furia.
- united24-2026-ugv-missions — UGV mission curve for Jan–Aug 2026; 50k production plan.
- kyivindependent-2026-farrell-rubicon-vivaldi — Rubikon pulled from the Lyman sector after Vivaldi.
- euromaidan-2026-mukhina-vivaldi-ugv-drop — UGVs air-dropped by heavy bombers in Operation Vivaldi.
- united24-2026-place-jet-gerans — jet Geran-3/4/5 specs; August 2026 surge; 60% interception.
- euromaidan-2026-zoria-jet-kill-rate-55 — September 2026 jet interception at 55%; $12bn claim.
- euromaidan-2026-thomas-hur-geran-production — HUR: 3,000 Geran-4/5 a month; Geran-3 ended.
- euromaidan-2026-zoria-nk-shahed — Zelensky: North Korea producing Shahed-type drones.
- euromaidan-2024-mukhina-hur-magura-ships — HUR list of ships hit by Magura V5.
- euromaidan-2025-kravchuk-magura-su30 — Magura V5: two Mi-8s (Dec 2024), Su-30 (May 2025).
- kyivindependent-2025-zadorozhnyy-kairos-virat — Sea Baby strikes on shadow-fleet tankers, Nov 2025.
- kyivindependent-2025-myronyshena-sub-sea-baby — first underwater-drone strike on a submarine, Dec 2025.
