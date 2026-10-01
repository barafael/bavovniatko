<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# 02 — Electronic warfare and the measure → countermeasure cycle

*Research base for bavovniatko. Accessed 2026-09-28. Russian capabilities are described only as reported by Ukrainian and Western sources. Source metadata: `sources/02-ew-countermeasures.yaml`.*

## 1. Overview

Electronic warfare (EW; Ukrainian **REB**, *radioelektronna borotba*) has set the terms of the drone war since 2022. From the start, RUSI's Ukrainian-sourced data showed that about 90% of UAS employed were lost, with EW as the main counter-UAS tool, and that precision weapons "can be defeated by EW" [src:rusi-2022-zabrodskyi-preliminary-lessons]. The same pattern has repeated ever since:

1. **A new capability arrives and dominates.** Examples: GMLRS, Excalibur, commercial quadcopters, analogue FPVs, Starlink, Shahed, fibre-optic FPVs, interceptor drones, mesh-controlled jet Gerans.
2. **The other side finds the physical-layer weakness.** Examples: the GNSS signal, the control or video frequency, the satellite link, the emitter signature, the cable, speed.
3. **The counter is cheap, spreads fast, and forces a counter-counter.** Examples: moving frequencies, CRPA antennas, fibre, autonomy, whitelists, mesh relays, nets, jet engines, digital video.

**Who is ahead changes by domain and by era.**
- **2023: Russia led the tactical EW fight.** It fielded about one major EW system per 10 km of front, and Ukraine lost about 10,000 UAVs a month [src:rusi-2023-watling-meatgrinder]. Zaluzhnyi counted about 60 types of modern Russian EW, and some weeks more than 2,000 Ukrainian drones were disabled [src:euromaidan-2023-voichuk-economist-ew]. Russia also blunted US GPS-guided munitions: Excalibur's hit rate fell from 55% to 6% [src:kyivpost-2024-chiu-gps-weapons].
- **2024–25: fibre and CRPA shifted the edge back to Russia.** Russia fielded fibre-optic FPVs first, from August 2024 in Kursk, and moved to multi-element CRPA antennas (Kometa-M) to beat Ukrainian GNSS jamming [src:kyivindependent-2025-farrell-fiber-optic; src:defenseexpress-2025-kometa-m-16]. A Ukrainian EW company commander dates the "collapse" of conventional frontline jamming to the end of 2024 [src:euromaidan-2025-zoria-jamming-fails].
- **2025: Ukraine answered in several ways at once.**
  - Mass production: EW output rose 340-fold in 2024, from more than 140 makers [src:militarnyi-2025-pryhodko-ew-340x].
  - GNSS jamming and spoofing (Lima, the backbone of the national Pokrova system) [src:kyivpost-2026-lima; src:militarnyi-2026-khomenko-pokrova-lima].
  - Interceptor drones: first against reconnaissance UAVs, then against Shaheds [src:kse-2025-drone-innovations].
  - Terminal autonomy (TFL-1) [src:kyivpost-2025-orlova-tfl1].
  - Nets over logistics roads [src:euromaidan-2025-murdoch-net-tunnels].
- **February 2026 was the biggest single swing.** The Starlink whitelist cut Russia off from the satellite link that had become the backbone of its drone and assault C2 [src:ukrinform-2026-liskovych-starlink-c2; src:atlanticcouncil-2026-spencer-starlink-crisis]. Biletsky put the drop in Russian drone effectiveness at 20–40% in the first two weeks [src:isw-2026-feb-26-assessment].
- **Mid-2026: Russia rebuilt its links on the ground and began to in space.** Mesh relay chains now control Gerans 150–175 km out [src:militarnyi-2026-khomenko-mesh-175km], and the Rassvet constellation gives two windows of more than an hour a day over Ukraine [src:euromaidan-2026-vivdych-rassvet-windows]. In the strike war, jet Gerans outran the cheap propeller interceptors [src:united24-2026-place-jet-gerans].

**How fast the cycle runs.** At the tactical radio-frequency level, change is monthly or faster:
- Ukrainian vehicle jammers got by with one standard 900 MHz module until spring 2024, then needed four to five by summer and seven by autumn [src:euromaidan-2025-zoria-jamming-fails].
- Captured Russian FPV chips cover 300–1100 MHz, so a band change is a firmware update, not new hardware [src:pravda-2025-fpv-ew-shield].
- The "Firmware 1001" used to fly DJI quadcopters in war conditions went through more than 40 versions in two years, about one every two to three weeks. Russian units reflash thousands of DJIs every month [src:euromaidan-2025-martyniuk-drone-firmware].
- Detector makers say they must update hardware monthly [src:united24-2026-brizard-detectors].

At the materiel level (cages, antennas, fibre spools, interceptors), the lag is typically 3–9 months. At the systemic level (Starlink, US GPS weapons, mesh networks, satellite constellations), it is 6–24 months. KSE describes Ukraine's "development – combat testing – modification" loop as running in months, with innovation cycles "measured in weeks" [src:kse-2025-drone-innovations]. RUSI's lesson for everyone is that holding a technological edge "requires the ability to rapidly update systems in the field" [src:rusi-2024-watling-offensive-lessons].

## 2. Measure → countermeasure chain

Lag is the approximate time from the measure appearing at scale to the counter appearing at scale. It is estimated from the dates in the cited sources.

| date / era | measure (who) | countermeasure (who) | lag | source id |
|---|---|---|---|---|
| Feb 2022 / e1 | Russian EW and strikes threaten Ukrainian C2 (RU) | Starlink terminals requested 26 Feb and first shipment 28 Feb; more than 5,000 by 6 Apr (UA/SpaceX) | days | wikipedia-starlink-war |
| 2022 / e1–e2 | Commercial quadcopters and TB2 for recon and fires (UA) | Dense Russian EW, which becomes the main counter-UAS; about 90% of UAS lost (RU) | immediate | rusi-2022-zabrodskyi-preliminary-lessons |
| Mar–Sep 2022 / e1–e3 | Big Russian EW platforms (Zhitel) jam GPS, satcom and GSM (RU) | Ukraine hunts emitters: Zhitels killed at the end of March, in May and in September 2022, and by Excalibur in March 2023 (UA) | months | militarnyi-2022-kushnikov-zhitel-kharkiv; wikipedia-zhitel |
| 2022–23 / e1–e5 | Ukrainian Starlink (UA) | Russian attempts to jam terminal uplinks give only "minor, locally restricted" effects (RU) | failed | militarnyi-2026-stepanets-ew-vs-starlink |
| Jul 2022 / e2 | GMLRS/HIMARS "devastates" Russian C2 and logistics (UA) | HQs pulled back and hardened, SHORAD linked to long-range radar, EW; a "proportion" of GMLRS intercepted by spring 2023; US modifies GMLRS software (RU → US) | about 6–9 months | rusi-2023-watling-meatgrinder; wikipedia-zhitel; rusi-2024-watling-offensive-lessons |
| Dec 2022–Aug 2023 / e4–e5 | Excalibur GPS-guided 155 mm shells (UA/US) | Russian GNSS jamming and spoofing: hit rate 55% (Jan 2023) → 6% (Aug 2023); cost per successful strike $300k → $1.9M (RU) | about 6 months | kyivpost-2024-chiu-gps-weapons |
| Feb 2023 / e4 | JDAM-ER glide bombs (UA/US) | Jamming causes misses of 19 m to about 3/4 mile (RU). Counter-counter: upgraded kits, add-on seekers, home-on-jam (US) | weeks | kyivpost-2024-chiu-gps-weapons; eurosd-2024-withington-excalibur |
| Jan 2023 / e4 | Mavic reconnaissance near Huliaipole (UA) | Russian EW: three Mavics lost in two weeks, where one had lasted six months near Donetsk in late 2022 (RU) | – | euromaidan-2025-zoria-jamming-fails |
| 2023 / e4 | Mass Ukrainian UAV use | About one major EW system per 10 km of front; Shipovnik-Aero (about 10 km, takes over drones and geolocates operators); platoon-level jammers and hijacking arrays; about 10,000 UAVs lost a month (RU) | already in place | rusi-2023-watling-meatgrinder; euromaidan-2023-voichuk-economist-ew |
| 2023 / e4–e5 | Ukrainian counter-battery and anti-emitter strikes on big EW platforms (UA) | Russia uses Zhitel more subtly, disperses antennas on light platforms, and treats Pole-21 as "disposable"; Krasukha-4 hidden under nets in forest belts, but the second one is still killed in Nov 2023 (RU → UA) | months | rusi-2023-watling-stormbreak; militarnyi-2023-kushnikov-krasukha-4-zaporizhzhia |
| Summer 2023 / e5 | Russian EW makes Ukrainian bomber-drone use "binary" (RU) | Ukraine opens EW gaps with artillery SEAD and planned pauses in its own jamming (UA) | weeks–months | rusi-2023-watling-stormbreak; rusi-2025-watling-third-year |
| Mid-2023 → Aug 2024 / e5–e7 | Ukrainian FPVs kill armour (UA) | Cope cages *(mangaly)* (mid-2023), then factory cages, then "turtle" boxes with jammers (Apr 2024), then the "super turtle" T-80 (Aug 2024) (RU) | about 3 months to cages, 9–12 to turtles | kyivpost-2024-turtle-tanks |
| Autumn 2023 → Aug 2024 / e6–e7 | Ukrainian FPVs against vehicles (UA) | Volnorez magnet-mounted cone jammer puts a "dome" over each vehicle (RU); one captured with its documents in Kursk (UA) | months | militarnyi-2024-kushnikov-volnorez |
| 2023–May 2024 / e6 | Russian FPVs on 850–930 MHz (RU) | Ukrainian backpack, vehicle and trench jammers (Kvertus, 720–1050 MHz; Obrii) (UA). Counter-counter: Russia moves to 720–1020 MHz and mixes bands, so vehicle jammers need 1 → 4–5 → 7 modules within 2024 (RU) | months each step | euromaidan-2024-martyniuk-ew-backpacks; euromaidan-2025-zoria-jamming-fails |
| Feb 2024 / e6 | Russians use smuggled Starlink (83rd Air Assault Bde, near Klishchiivka) (RU) | GUR exposes it; SpaceX deactivations, with a claimed block in May 2024 that proved only partial (UA/SpaceX) | months (only fully solved Feb 2026) | aljazeera-2024-gur-starlink; wikipedia-starlink-war |
| Sep 2022 → 2025 / e3–e8 | Shahed/Geran waves; high-flying routes mapping Ukrainian EW and air defence (RU) | Distributed defence: helicopters, mobile fire groups, Gepard and Skynex guns, MANPADS, "vast majority intercepted" (UA). Counter-counter: nav hardening and waves routed to avoid EW (RU) | months | rusi-2025-watling-third-year; rusi-2023-watling-stormbreak; militarnyi-2025-gepard-repair; militarnyi-2025-shumlianskyi-skynex-shaheds |
| Early 2024 / e6–e7 | GNSS-guided Shaheds, missiles and KABs (RU) | Pokrova national GNSS-spoofing system (early 2024); its Lima component (Cascade Systems, Night Watch) is 85% of Pokrova by end-2024 and more than 90% by 2026; more than 400 Lima systems, a claimed 20,500+ Shaheds jammed (UA) | years (maker claim) | militarnyi-2026-khomenko-pokrova-lima; kyivpost-2026-lima; euromaidan-2023-voichuk-economist-ew |
| 2024 / e6–e7 | Ukrainian EW jams KAB guidance (UA) | Kometa-4 → Kometa-8 CRPA on KABs; KABs effective again (RU) | months | forbes-2026-hambling-lima-quant |
| 2024 / e6–e7 | Ukrainian Starlink for drones and C2 (UA) | First purpose-built Russian anti-Starlink jammers (prototypes of "Peresvet") (RU) | about 2 years | militarnyi-2026-stepanets-ew-vs-starlink |
| Aug 2024 / e7 | Russian Orlan/Zala orbits: 1,000–1,500 a day (RU) | Radar-cued interceptor UAVs: several hundred kills a month (Oct 2024), more than 1,000 a month (summer 2025) (UA) | about 3–12 months | rusi-2025-watling-third-year; kse-2025-drone-innovations |
| Aug 2024 / e7 | Russian fibre-optic FPV "KVN" in Kursk, immune to jamming (RU) | Ukrainian Silkworm spool (Feb 2025), about 1 in 10 teams on fibre by May 2025; shotguns, nets, heated tungsten wire, net guns, daytime and foot logistics (UA) | about 6–9 months | kyivindependent-2025-farrell-fiber-optic; united24-2025-barkhush-fiber-counters |
| Jan–Jun 2025 / e7–e8 | Ukrainian GNSS jamming (UA) | Kometa-M CRPA: 12 elements on UMPK (Apr 2025), 16 on Shahed and Iskander-K (from Mar 2025); in theory one jammer is needed per element, but a new CRPA series resists more jammers than it has elements (a new 8-element needed 19 jammers; a 16-element resisted 104) (RU) | months | defenseexpress-2025-kometa-m-16; united24-2025-khomenko-kometa-umpk; forbes-2026-hambling-lima-quant |
| Feb 2025 / e8 | Ukrainian FPVs and bombers on Russian supply roads (UA) | Russian net tunnels, e.g. about 2 km between Bakhmut and Chasiv Yar (RU) | months | defenseexpress-2025-russian-net-tunnels |
| Early 2025 / e8 | Shahed campaign (RU) | First interceptor-drone Shahed kills (UA) | about 2.5 years after the first Shaheds | kse-2025-drone-innovations |
| Apr 2025 / e8 | Russian trench EW domes (RU) | FPV with a 12-band emitter detector that homes on and strikes jammers, i.e. "emit and die" (UA) | months | militarnyi-2025-kushnikov-ew-hunter-fpv |
| May 2025 / e8 | Ukrainian thermal-camera drones at night (UA) | Russian assault groups in anti-thermal ponchos at dawn changeover; only partly effective (RU) | – | militarnyi-2025-yan-russian-thermal-ponchos |
| Mid-2025 / e8 | Ukrainian recon UAVs (UA) | Russian interceptor drones (RU) | about 12 months behind Ukraine | kse-2025-drone-innovations |
| 2025 / e8 | Standalone jammers overwhelmed by frequency diversity: 400–490 MHz, 720–1020 MHz, 2.1–2.3 GHz, plus hopping (RU) | Networked EW (Kvertus Atlas, Jul 2025: Mirage nodes reach 20 km directional, 8 km omni) (UA) | months | pravda-2025-fpv-ew-shield; defensepost-2025-encarnacion-atlas |
| Sep 2025 / e8 | Jamming kills the RF link in the last mile (RU) | TFL-1 machine-vision terminal guidance for the last about 500 m, mass-produced with Vyriy (UA) | about 1 year from trials | kyivpost-2025-orlova-tfl1 |
| Sep 2025 / e8 | Fibre FPVs strike roads 25–30 km deep (RU) | Zaporizhzhia net tunnels: first 6.4 km, with hundreds of km planned (UA) | about 12 months after KVN | euromaidan-2025-murdoch-net-tunnels |
| 2025 → Sep 2026 / e8–e9 | Ukrainian GNSS jamming and interceptors against Gerans (UA) | Mesh radio control: XK-F358 COFDM modems and relay towers; control range doubles to 150–175 km (RU). Ukraine did not systematically counter mesh for about a year (UA) | about 12 months, counter still open | euromaidan-2026-vivdych-xk-f358; militarnyi-2026-khomenko-mesh-175km; euromaidan-2026-mukhina-geran-mesh |
| 1–5 Feb 2026 / e9 | Russian drones and C2 on smuggled Starlink, including Starlink Minis on BM-35/Molniya/Shahed (RU) | Whitelist plus a 75–90 km/h speed cap: Russian C2 "collapsed"; Russian drone effectiveness down 20–40% in two weeks; more than 200 km² retaken in 5 days (UA/SpaceX) | about 2 years after GUR exposure | kyivindependent-2026-myronyshena-starlink-whitelist; ukrinform-2026-liskovych-starlink-c2; isw-2026-feb-26-assessment; atlanticcouncil-2026-spencer-starlink-crisis |
| Feb–Sep 2026 / e9 | Starlink loss (RU) | Workarounds: bribed Ukrainian registrations, legacy radio, Ubiquiti links (about 80% of Russian frontline links), mesh relays, fibre, and Rassvet satellites (first 16 launched Mar 2026) (RU) | weeks to partial; 3–5 years to full (Biletsky) | ukrinform-2026-liskovych-starlink-c2; isw-2026-feb-08-assessment; isw-2026-feb-26-assessment; militarnyi-2026-ubiquiti-80; euromaidan-2026-thomas-isw-rassvet |
| Apr 2026 / e9 | New-series Kometa CRPA on KABs and Shaheds (RU) | Lima-Quant: KAB effectiveness "dropped to zero" on about 700 km of front; CRPA suppressed at 50 km (developer claims). Friction: brigade commanders ban it because it blinds their own Mavics (UA) | about 12 months | forbes-2026-hambling-lima-quant |
| Jun–Aug 2026 / e9 | Ukrainian drones on Starlink (UA) | Volna Kupol Garant satellite-uplink jammer (14–14.5 GHz, about 20 km² per unit), announced in serial production (RU). Counter: SpaceX spots the interference, partner ELINT locates it, and Ukraine strikes the units (UA) | weeks | militarnyi-2026-pryhodko-volna-kupol-garant; militarnyi-2026-shumlianskyi-volna-mass-production |
| 2026 / e9 | Ukrainian detectors read analogue video (UA) | Russians move to non-standard bands, e.g. Molniya video at 4.1–4.5 GHz and control hopping across 300–600 MHz; then digital video (Sep 2026), which analogue detectors cannot see (RU). Chuika 4.0 widened to 4.5 GHz (Sep 2026) (UA) | about monthly; digital counter still open | united24-2026-brizard-detectors; euromaidan-2026-mukhina-molniya; euromaidan-2026-mukhina-molniya-digital; militarnyi-2026-khomenko-chuika-4 |
| 2026 / e9 | Russian jamming of RF FPVs (RU) | National Guard fibre share rises from about 20% to 70% of its drones (UA) | about 18 months after KVN | euromaidan-2026-vivdych-ng-fibre-70 |
| Feb–May 2026 / e9 | Massed piston Shaheds (RU) | Interceptor drones: about 6,300 sorties and more than 1,500 kills (Feb); more than 3,000 Sting kills in May; more than 40% of Shahed kills on 24 May (UA) | – | isis-2026-anokhin-shahed-monthly; pravda-2026-sting-3000; militarnyi-2026-pryhodko-interceptors |
| 2026 / e9 | Propeller interceptors and mobile fire groups (UA) | Jet Gerans (Geran-3/4/5, 300–600 km/h): interception falls from 90%+ to about 60%; mobile fire groups "can no longer be relied upon" against them (RU) | about 12 months | united24-2026-place-jet-gerans; pravda-2026-ihnat-jet-drones |
| Jul–Sep 2026 / e9 | Ukrainian GNSS jamming of Gerans (RU target) | 32-element CRPA recovered; 20-element expected on drones in Sep 2026 and 24 in 2027; Geran-5 "K5" reportedly navigates by optical terrain matching (unconfirmed) (RU) | months | militarnyi-2026-khomenko-crpa-32; euromaidan-2026-mukhina-crpa-20; militarnyi-2026-leonov-geran-5-optical |
| Jul–Sep 2026 / e9 | Jet Gerans (RU) | Jet-killer interceptors (Sting 2.0, LITAVR+); "speed alone" fails without machine-vision lock, loiter time and agility (UA) | months, still open | united24-2026-place-jet-gerans; euromaidan-2026-mukhina-jet-interceptors |
| Jul–Aug 2026 / e9 | Ukrainian FPVs against tanks (UA) | Arena-M active protection adapted to FPVs on T-72B3A (RU). Counter: saturate it; 7 drones empty the launchers and a kill takes 15–20 FPVs. Ukrainian APS stop 60–80% of FPVs in tests (UA) | days | militarnyi-2026-pryhodko-aps-80 |
| Aug 2026 / e9 | Ukrainian interceptor video links (UA) | "Shtora" image-spoofing system aimed at drone video feeds (RU). Counter: interceptors that hop video channels and bands (Last Shadow T200) (UA) | months | militarnyi-2026-khomenko-t200-shtora |
| May–Sep 2026 / e9 | Russian drone dominance near Lyman (RU) | Operation Vivaldi: 9 months of EW preparation, Russian drones "blinded", Rubicon hit, then UGVs and infantry (UA) | 9 months of preparation | euobserver-2026-vasilko-vivaldi; mwi-2026-rose-vivaldi; kyivindependent-2026-farrell-vivaldi |

## 3. Per-era notes

### e1-invasion (Feb–Apr 2022)
- **EW was already the main counter-UAS tool.** Ukraine lost about 90% of the UAS it employed, and RUSI calls EW "the primary means of CUAS" [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **Russian EW hurt its own side too.** Russian EW "rarely deconflict[s]", which causes fratricide and forces effects to be used one after another [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **Big Russian systems came in with the columns.** A Krasukha-4 was filmed at Makariv (Kyiv oblast) in March 2022 [src:militarnyi-2023-kushnikov-krasukha-4-zaporizhzhia], and the first Zhitel kill came at the end of March [src:militarnyi-2022-kushnikov-zhitel-kharkiv].
- **Starlink arrived within days.** The first terminals landed on 28 February 2022, two days after Fedorov's request, and more than 5,000 were in country by 6 April [src:wikipedia-starlink-war]. For the game, this is Ukraine's first "EW-resistant comms" card.
- **Ukraine already had domestic EW.** Bukovel-AD has been in service since 2016 against Orlan-10 [src:wikipedia-bukovel].

### e2-donbas-artillery (May–Aug 2022)
- **GMLRS arrived and devastated Russian C2 and logistics** in July 2022. Dispersed Russian air defence could not intercept it at first [src:rusi-2023-watling-meatgrinder]. Russian jamming later forced a US software change to GMLRS, and it widened JDAM error from about 5 m to about 30 m [src:wikipedia-zhitel].
- **Ukraine was already hunting emitters.** A second Zhitel was destroyed in May 2022 [src:militarnyi-2022-kushnikov-zhitel-kharkiv].

### e3-counteroffensives-22 (Sep–Nov 2022)
- **More emitter kills.** Another Zhitel burned in Kharkiv oblast in September 2022, and a Borisoglebsk-2 station was captured there [src:militarnyi-2022-kushnikov-zhitel-kharkiv].
- **Starlink limits.** Musk refused Starlink coverage around Crimea in September 2022 [src:wikipedia-starlink-war].
- **Software as a weak point.** RUSI's later review of the 2022–23 offensives says software-defined systems were "susceptible to targeted electronic warfare interference". It adds that Russia over time built "hard counters" to Excalibur and GMLRS [src:rusi-2024-watling-offensive-lessons].
- **The Shahed campaign began.** Ukraine's layered "distributed defence" against it is described in [src:rusi-2025-watling-third-year].

### e4-bakhmut (Dec 2022–May 2023)
- **Russian EW density (spring 2023):**
  - About one major system per 10 km of front, sited about 7 km back [src:rusi-2023-watling-meatgrinder].
  - "Weapons free", with no deconfliction.
  - Shipovnik-Aero rated especially effective because of its low signature and its ability to imitate other emitters. It reaches about 10 km, can take control of drones and geolocates their operators for artillery [src:euromaidan-2023-voichuk-economist-ew].
  - Ukrainian UAV losses about 10,000 a month.
- **Where Russian EW arrived, drones died fast.** Near Huliaipole in January 2023, one unit lost three Mavics in two weeks. Near Donetsk a few months earlier, a Mavic had lasted six months [src:euromaidan-2025-zoria-jamming-fails].
- **Comms interception.** Ukrainian Motorola 256-bit traffic was reportedly decrypted in near real time, most likely by Torn-MDM [src:rusi-2023-watling-meatgrinder].
- **GPS weapons lost their edge.** JDAM-ER (from February 2023) missed by 19 m to about 3/4 mile, and Excalibur accuracy began to collapse [src:kyivpost-2024-chiu-gps-weapons].

### e5-counteroffensive-23 (Jun–Nov 2023)
- **Excalibur bottomed out.** It hit the target only 6% of the time by August 2023 [src:kyivpost-2024-chiu-gps-weapons].
- **Ukrainian drone use depended on Russian EW.** If Russian EW was active, UAVs "could not get in" [src:rusi-2023-watling-stormbreak].
- **Jamming as a tell.** Russian aviation strikes were heralded by the *lifting* of Russian GPS jamming, an exploitable signature [src:rusi-2023-watling-stormbreak].
- **Russian EW adapted to being hunted** [src:rusi-2023-watling-stormbreak]:
  - Zhitel used more subtly.
  - Antennas dispersed on light platforms.
  - Pole-21 treated as expendable wide-area cover.
  - Shahed navigation hardened.
  - Krasukha-4 hidden under nets in forest belts. Drones still found one near Urozhaine in November 2023, only the second recorded loss of the type [src:militarnyi-2023-kushnikov-krasukha-4-zaporizhzhia].
- **The EW gap in numbers.** Zaluzhnyi's November 2023 essay counts about 60 types of modern Russian EW against Ukraine's mostly Soviet-era kit. It names Pokrova as a nationwide GNSS spoofer that could deny satellite navigation over most of Ukraine [src:euromaidan-2023-voichuk-economist-ew].
- **Cope cages** *(mangaly)* **appeared on Russian tanks** from mid-2023 [src:kyivpost-2024-turtle-tanks].

### e6-avdiivka-attrition (Oct 2023–Jul 2024)
- **Jammers in every trench.** The FPV boom brought mass trench and backpack jammers. Kvertus went from dozens a month (2022) to thousands (2024) [src:euromaidan-2024-martyniuk-ew-backpacks]. In December 2023, commanders who had refused EW kit that September began signing for it "without even reading" the forms [src:euromaidan-2025-zoria-jamming-fails].
- **Frequency moves began.** Russian FPVs moved from 850–930 MHz to 720–1020 MHz, although 65–70% still used standard bands [src:euromaidan-2024-martyniuk-ew-backpacks]. Vehicle jammers went from one module in spring 2024 to seven by autumn, which strained power supply and weight [src:euromaidan-2025-zoria-jamming-fails].
- **Industry scaled.** EW output grew 340-fold in 2024 over 2023, and the number of makers went from about 10 to more than 140 [src:militarnyi-2025-pryhodko-ew-340x].
- **Russia fielded vehicle "domes".** The Volnorez cone jammer appeared in autumn 2023 [src:militarnyi-2024-kushnikov-volnorez].
- **Western framing.** CNAS called EW "the most effective way to stop drones" and noted operators being hunted with drone-tracking software [src:cnas-2024-pettyjohn-evolution].
- **Russian Starlink use confirmed.** GUR called it "systemic" in February 2024 [src:aljazeera-2024-gur-starlink].
- **Turtle tanks.** Russian "turtle tanks" with onboard jammers appeared in April 2024 [src:kyivpost-2024-turtle-tanks].
- **Pokrova went live in early 2024**, and its Lima component deployed from July 2024 [src:militarnyi-2026-khomenko-pokrova-lima; src:kyivpost-2026-lima].
- **Russian firmware logistics became a target.** Russian units reflash thousands of DJIs every month from central servers. The IT Army claims it disrupted those servers in February 2024 [src:euromaidan-2025-martyniuk-drone-firmware].

### e7-kursk-pokrovsk (Aug 2024–Mar 2025)
- **Fibre-optic drones arrived.** Russia's KVN fibre FPV appeared in Kursk in August 2024 [src:kyivindependent-2025-farrell-fiber-optic].
  - The Atlantic Council claims Ukraine lost 25% more vehicles than Russia in Kursk.
  - Russia kept a lead in fibre FPVs through 2025 [src:atlanticcouncil-2026-sutea-fiber-optic].
  - One EW commander calls the end of 2024 the point where conventional EW "began to collapse". Russian FPV reach grew from about 3 km to 10 km by spring 2025 [src:euromaidan-2025-zoria-jamming-fails].
- **RUSI snapshot (Feb 2025)** [src:rusi-2025-watling-third-year]:
  - 60–80% of Ukrainian FPVs fail to reach their targets.
  - Most vehicles carry jammers.
  - Navigational jamming is "ubiquitous".
  - Fibre FPVs have about 10 km range.
  - Kh-101 upgraded for visual terrain tracking to beat terminal-phase EW.
- **Ukraine's interceptor drones against Russian reconnaissance UAVs scaled up** [src:rusi-2025-watling-third-year; src:kse-2025-drone-innovations].
- **A 16-element Chinese CRPA was found** on Russian drones in January 2025 [src:united24-2025-khomenko-kometa-umpk].
- **Captured kit.** A Volnorez vehicle jammer was taken with its documentation in Kursk (Aug 2024) [src:militarnyi-2024-kushnikov-volnorez].

### e8-drone-kill-zone (Mar 2025–Jan 2026)
- **The CRPA ladder climbed.** Russian CRPA went to 12 elements on UMPK (April 2025) and 16 elements on Shahed and Iskander-K [src:defenseexpress-2025-kometa-m-16]. A new series of Russian CRPA resists more jammers than it has elements, which made KABs effective again until Lima-Quant (see e9) [src:forbes-2026-hambling-lima-quant].
- **Ukraine hunted jammers directly.** It tested EW-hunting FPVs in April 2025 [src:militarnyi-2025-kushnikov-ew-hunter-fpv] and fielded the networked Atlas EW system in July 2025 [src:defensepost-2025-encarnacion-atlas].
- **Frequencies spread out.** One sector showed a 20% / 30% / 50% split across the 400 MHz, 900 MHz and 2.1–2.3 GHz bands [src:pravda-2025-fpv-ew-shield].
  - Captured Russian chips cover 300–1100 MHz, so Ukrainian units pre-order jammers for bands the enemy can reach by firmware update.
  - Dome jammers protect 200–300 m; directional ones reach several km.
  - Trench EW weighs 20–50 kg plus a generator.
  - Russian ELINT satellites map Ukrainian emitters.
  - Kvertus says 6,000 Mirage nodes and 300 Azimuth receivers would cover the 1,300 km front for over UAH 5bn; only 10% was funded.
  - Industry sources agree drones "hold the initiative" and blame the institutional speed of deployment, not technology.
- **Big vehicle EW pulled back.** Bukovel-AD, Nota, Ai-Petri, Damba and Beton were forced back from the front by longer-range drones and fibre. One commander compares them to helmets: "potentially lifesaving … but offering no guarantees" [src:euromaidan-2025-zoria-jamming-fails].
- **Fibre: Russia ahead, Ukraine catching up.** Ukraine had about 1 in 10 teams on fibre by May 2025 [src:kyivindependent-2025-farrell-fiber-optic].
- **Nets became infrastructure.** Russia built net tunnels in February 2025 [src:defenseexpress-2025-russian-net-tunnels], and Ukraine followed in Zaporizhzhia from September 2025 [src:euromaidan-2025-murdoch-net-tunnels].
- **Improvised counters to fibre drones** [src:united24-2025-barkhush-fiber-counters]:
  - Tungsten "hot wire" that snaps the cable.
  - Shotgun teams.
  - Net guns.
  - Nets, with the caveat that "waiting" drones *(zhduny)* can hide under them.
- **Thermal masking.** Russian groups tried anti-thermal ponchos at the dawn changeover between thermal and day drones (May 2025). They reduced thermal contrast only partly, and movement gave the men away [src:militarnyi-2025-yan-russian-thermal-ponchos].
- **Autonomy went to mass production.** TFL-1 terminal autonomy entered mass production in September 2025 [src:kyivpost-2025-orlova-tfl1].
- **Interceptors turned on Shaheds.** Interceptor drones scored their first Shahed kills in early 2025, and Russia fielded its own interceptors from mid-2025 [src:kse-2025-drone-innovations].
- **Gun air defence.** Skynex batteries (four towed 35 mm guns with AHEAD airburst, about 4 km range) shot Shaheds over point targets [src:militarnyi-2025-shumlianskyi-skynex-shaheds]. Gepard remained "one of the key" AA guns, and Ukraine began repairing them at home in August 2025 [src:militarnyi-2025-gepard-repair].
- **Drones dominate the front.** 80–85% of frontline targets were engaged by drones [src:kse-2025-drone-innovations].
- **Russian mesh began.** Russian mesh networks for Geran control were documented in 2025 at about half their 2026 range [src:militarnyi-2026-khomenko-mesh-175km]. Ukrainian radio makers moved to mesh too: four Himera handhelds gave one brigade 45 km of signal across forest [src:euromaidan-2026-kossov-himera-mesh].

### e9-counteroffensive-26 (Feb 2026–present)
- **Starlink whitelist.** It was ordered on 2 February and Russian terminals were blocked by 5 February [src:kyivindependent-2026-myronyshena-starlink-whitelist; src:ukrinform-2026-liskovych-starlink-c2]. ISW dates the block to 1 February [src:isw-2026-feb-26-assessment].
  - Beskrestnov: Russia's "entire C2 system has collapsed".
  - ISW, 8 Feb: Russian milbloggers say units need radio and satellite donations and run communications "on the ground". A Ukrainian brigade reports degraded Russian C2 and drone operations, but small-group assaults continue. ISW expects Russia to struggle to sustain its battlefield air interdiction (BAI) campaign [src:isw-2026-feb-08-assessment].
  - ISW, 26 Feb: Biletsky says Russian drone effectiveness fell by roughly 20–40% in two weeks. He expects partial recovery through other satcom within one to two months, but not Starlink-level efficiency for three to five years. ISW has observed reduced tempo and depth of Russian tactical and mid-range strikes [src:isw-2026-feb-26-assessment].
  - More than 200 km² retaken within 5 days, per the Atlantic Council [src:atlanticcouncil-2026-spencer-starlink-crisis].
  - Wikipedia's figure for the southern front is much smaller, "ten to twelve kilometers" [src:wikipedia-starlink-war], and ties it to an April start, so treat the gains as contested.
- **Russian recovery, Feb–Sep 2026.** It is partial and uneven, and it is moving to terrestrial mesh first and satellites second:
  - **Terrestrial links.** Ubiquiti radios already made up about 80% of Russian frontline links before the cut [src:militarnyi-2026-ubiquiti-80].
  - **Mesh.** Unsanctioned Chinese XK-F358 mesh modems (about $7,000–9,000) fly on Geran-5 and Gerbera [src:euromaidan-2026-vivdych-xk-f358]. Relay chains reach 150–175 km, about twice the 2025 range [src:militarnyi-2026-khomenko-mesh-175km]. Beskrestnov says nobody in the armed forces worked systematically on countering this mesh for a year [src:euromaidan-2026-mukhina-geran-mesh].
  - **Fibre.** Russian fibre FPVs reached Kharkiv's outskirts on 25 February, about 21 km from the border. Chinese optical fibre for Russian buyers rose from 16 to 40 yuan/km after Ukraine struck the Saransk fibre plant [src:isw-2026-feb-26-assessment].
  - **Rassvet.** 16 satellites were launched in March and 16 more in July. Only the 12 good first-batch satellites are at about 550 km, giving two daily windows over Ukraine totalling up to 90 minutes (ISW, 31 Aug) [src:euromaidan-2026-thomas-isw-rassvet]. Coverage grew from 6–10 minute passes in March to two windows of more than an hour each by late August [src:euromaidan-2026-vivdych-rassvet-windows]. Russia needs 200–300 satellites for 24/7 cover, and Soyuz-2.1b cadence caps it at about 128 in four years (ISW) [src:euromaidan-2026-thomas-isw-rassvet].
  - **"Kupol".** The reported "Kupol" system is the Volna Kupol Garant jammer. It is a counter to *Ukrainian* Starlink, not a replacement for Russia's (see below) [src:militarnyi-2026-shumlianskyi-volna-mass-production].
  - **Yamal/Express GEO terminals.** No fetched source was found.
- **Russia jams Ukraine's Starlink.** Volna Kupol Garant jams the 14–14.5 GHz uplink over about 20 km² per unit. It was reported deployed in June and announced as in serial production in August, with the Russian claim that Starlink is "shut down everywhere we need" [src:militarnyi-2026-pryhodko-volna-kupol-garant; src:militarnyi-2026-shumlianskyi-volna-mass-production].
  - A Ukrainian satcom engineer measures the effect of such systems as 0–35% packet loss inside a cell about 17 km across. That breaks unhardened drone video, but it is not a blackout [src:militarnyi-2026-stepanets-ew-vs-starlink].
  - Russian authors concede that the strategic Tobol and Tirada-2 satellite jammers have not been seen working against Starlink in combat [src:militarnyi-2026-glazyev-starlink-scenarios].
- **GNSS war at the top end.**
  - **Lima-Quant (April 2026).** Its developers claim KABs "dropped to zero" effectiveness on about 700 km of front, with 869 KABs in a month causing minor injuries to 8 soldiers, and 41 of 42 Kinzhals neutralised [src:forbes-2026-hambling-lima-quant].
  - **Night Watch (July 2026).** It claims 61 Kinzhals countered and says EW accounts for about half of all "intercepted" Shaheds and missiles [src:militarnyi-2026-kozatskyi-lima-half].
  - **Russia keeps climbing.** A 32-element CRPA (16 L1 plus 16 L2 elements) was recovered [src:militarnyi-2026-khomenko-crpa-32]. 20-element arrays were expected in September 2026 and 24-element ones in 2027 [src:euromaidan-2026-mukhina-crpa-20]. A Geran-5 with optical terrain matching is reported but unconfirmed [src:militarnyi-2026-leonov-geran-5-optical].
- **Interceptors took over the Shahed fight.** In February 2026 they flew about 6,300 sorties and made more than 1,500 kills; near Kyiv more than 70% of Shaheds fell to interceptors [src:isis-2026-anokhin-shahed-monthly].
  - Sting alone claimed more than 3,000 Shahed and Gerbera kills in May. The MoD counted 7,476 of 8,150 launches intercepted that month (91.7%) [src:pravda-2026-sting-3000].
  - They took more than 40% of Shahed kills on 24 May 2026 [src:militarnyi-2026-pryhodko-interceptors].
  - The Shahed hit rate fell to about 6.7% in May 2026 [src:isis-2026-anokhin-shahed-monthly]. Beskrestnov says 92–96% of piston Shaheds on deep-rear raids are stopped, which pushes Russia to jets for those raids [src:isis-2026-anokhin-shahed-monthly].
- **Guns: useful but fragile.** One Skynex battery claims about 35 Shaheds and 2 cruise missiles, 12 targets in a single engagement [src:militarnyi-2026-kozatskyi-skynex-12]. An internal report says two Skynex lost three of eight guns to faults within minutes on 1 April 2026 and let a Shahed through [src:militarnyi-2026-khomenko-skynex-issues].
- **Russia's answer was jet Gerans** [src:united24-2026-place-jet-gerans; src:euromaidan-2026-mukhina-jet-interceptors]:
  - About 3,000 Geran-4/5 a month, per HUR.
  - Interception rate against jets about 60%.
  - The first "ready" jet interceptors failed in combat.
  - Ihnat: at up to 500 km/h the jets outrun 300 km/h interceptors, so mobile fire groups and interceptors "can no longer be relied upon" against them and missiles must be used [src:pravda-2026-ihnat-jet-drones].
- **Radio-frequency adaptation is continuous.**
  - Molniya uses frequency hopping [src:euromaidan-2026-mukhina-molniya], and detectors need monthly updates [src:united24-2026-brizard-detectors].
  - Chuika 4.0 was widened to 4.5 GHz for Molniya video [src:militarnyi-2026-khomenko-chuika-4].
  - Molniyas are now switching to digital video, which the analogue detectors cannot see [src:euromaidan-2026-mukhina-molniya-digital].
  - Russia's "Shtora" spoofs video feeds [src:militarnyi-2026-khomenko-t200-shtora].
- **Ukraine moved to cable.** The National Guard now flies 70% of its drones on fibre, up from about 20% [src:euromaidan-2026-vivdych-ng-fibre-70].
- **Active protection returned to armour.** Russia's FPV-adapted Arena-M was first used in July 2026. It stopped several FPVs: 7 drones emptied it, and killing the tank took 15–20 FPVs. Ukrainian APS stop 60–80% of FPVs in tests [src:militarnyi-2026-pryhodko-aps-80].
- **Emitter hunting went deep.** Only 24 Zhitels have been visually confirmed destroyed or damaged in the whole war [src:militarnyi-2026-pryhodko-lasars-zhitel]. One National Guard unit killed two in 2026, the second 55 km behind the front [src:euromaidan-2026-mukhina-zhitel-55km].
- **Operation Vivaldi near Lyman** began with nine months of EW preparation to blind Russian drones. It then struck the Rubicon unit and Russian logistics 50 km and then 120 km deep, and then used UGVs [src:euobserver-2026-vasilko-vivaldi; src:mwi-2026-rose-vivaldi; src:kyivindependent-2026-farrell-vivaldi].

## 4. Key figures

| figure | value | unit | era | source id |
|---|---|---|---|---|
| UAS employed that are lost | 90 | % | e1-invasion | rusi-2022-zabrodskyi-preliminary-lessons |
| Russian EW density | ~1 major system per 10 | km of front | e4-bakhmut | rusi-2023-watling-meatgrinder |
| Ukrainian UAV losses | ~10,000 per month; >2,000 in some weeks | count | e4–e5 | rusi-2023-watling-meatgrinder; euromaidan-2023-voichuk-economist-ew |
| Types of modern Russian EW fielded (Zaluzhnyi) | ~60 | types | e5-counteroffensive-23 | euromaidan-2023-voichuk-economist-ew |
| Shipovnik-Aero range | ~10 | km | e4-bakhmut | euromaidan-2023-voichuk-economist-ew |
| Zhitel jamming radius | up to ~30 | km | e1 → e9 | militarnyi-2026-pryhodko-lasars-zhitel |
| Zhitels confirmed destroyed or damaged (Feb 2022–May 2026) | 24 | systems | e1 → e9 | militarnyi-2026-pryhodko-lasars-zhitel |
| Deepest reported drone kill of a Zhitel | 55 | km behind the front | e9-counteroffensive-26 | euromaidan-2026-mukhina-zhitel-55km |
| Krasukha-4 claimed range (unverified) | 300 | km | e1 → e8 | militarnyi-2025-krasukha-4-leak |
| Excalibur hit rate, Jan 2023 → Aug 2023 | 55 → 6 | % | e4-bakhmut → e5-counteroffensive-23 | kyivpost-2024-chiu-gps-weapons |
| Cost per successful Excalibur strike | 0.3 → 1.9 | USD million | e5-counteroffensive-23 | kyivpost-2024-chiu-gps-weapons |
| JDAM-ER miss distance under jamming | 19 m to ~1,200 m (3/4 mile) | m | e4-bakhmut | kyivpost-2024-chiu-gps-weapons |
| JDAM error, unjammed → jammed | ~5 → ~30 | m | e5-counteroffensive-23 | wikipedia-zhitel |
| Russian FPV control band shift | 850–930 → 720–1020 | MHz | e6-avdiivka-attrition | euromaidan-2024-martyniuk-ew-backpacks |
| Jammer modules needed per vehicle, spring → autumn 2024 | 1 → 4–5 → 7 | modules | e6 → e7 | euromaidan-2025-zoria-jamming-fails |
| Russian FPV chip tunable range | 300–1100 | MHz | e8-drone-kill-zone | pravda-2025-fpv-ew-shield |
| Dome (omni) jammer protection radius | 200–300 | m | e8-drone-kill-zone | pravda-2025-fpv-ew-shield |
| Trench EW weight | 20–50 (plus generator) | kg | e8-drone-kill-zone | pravda-2025-fpv-ew-shield |
| DJI war firmware ("Firmware 1001") versions | >40 in 2 years | versions | e6 → e7 | euromaidan-2025-martyniuk-drone-firmware |
| Kvertus backpack jammer cost | 7,000 | USD | e6-avdiivka-attrition | euromaidan-2024-martyniuk-ew-backpacks |
| Ukrainian EW output growth, 2023 → 2024 | 340 | × | e6 → e7 | militarnyi-2025-pryhodko-ew-340x |
| Ukrainian EW makers | ~10 → >140 | companies | e6 → e7 | militarnyi-2025-pryhodko-ew-340x |
| Ukrainian FPVs failing to reach target | 60–80 | % | e7-kursk-pokrovsk | rusi-2025-watling-third-year |
| Share of damaged or destroyed Russian systems hit by tactical UAVs | 60–70 | % | e7-kursk-pokrovsk | rusi-2025-watling-third-year |
| Russian Orlan/Zala orbits (Aug 2024) | 1,000–1,500 | per day | e7-kursk-pokrovsk | rusi-2025-watling-third-year |
| Human counter-UAS efficiency (versus automated turrets) | ~25 | % | e7-kursk-pokrovsk | rusi-2025-watling-third-year |
| Fibre FPV range | ~10 (RUSI, Feb 2025); 10–15, with 20 in test (KI, May 2025); 30+ and ~40 for Birds of Magyar (AC, 2026) | km | e7 → e9 | rusi-2025-watling-third-year; kyivindependent-2025-farrell-fiber-optic; atlanticcouncil-2026-sutea-fiber-optic |
| Ukrainian drone teams using fibre (May 2025) | ~1 in 10 | ratio | e8-drone-kill-zone | kyivindependent-2025-farrell-fiber-optic |
| National Guard drones on fibre | ~20 → 70 | % | e8 → e9 | euromaidan-2026-vivdych-ng-fibre-70 |
| Fibre FPV threat depth from front | 25–30 | km | e8-drone-kill-zone | euromaidan-2025-murdoch-net-tunnels |
| Kometa-M CRPA elements | 4 (2022) → 8 → 12 (Apr 2025) → 16 (2025) → 32 recovered (2026) → 20 on drones (Sep 2026) → 24 planned (2027) | elements | e7 → e9 | defenseexpress-2025-kometa-m-16; forbes-2026-hambling-lima-quant; militarnyi-2026-khomenko-crpa-32; euromaidan-2026-mukhina-crpa-20 |
| Jammers needed against the new-series 8-element CRPA | 19 (16-element: >104) | jammers | e8 → e9 | forbes-2026-hambling-lima-quant |
| Lima: Shaheds jammed (maker claim) | 20,500+ | drones | e7 → e9 | kyivpost-2026-lima |
| Lima share of Pokrova | 85 (end 2024) → 95 (early 2025) → >90 (2026) | % | e7 → e9 | militarnyi-2026-khomenko-pokrova-lima |
| EW share of "intercepted" air targets (Night Watch claim) | ~50 | % | e9-counteroffensive-26 | militarnyi-2026-kozatskyi-lima-half |
| Kinzhals neutralised by Lima (claims) | 41 of 42 (Apr 2026); 61 (Jul 2026) | missiles | e9-counteroffensive-26 | forbes-2026-hambling-lima-quant; militarnyi-2026-kozatskyi-lima-half |
| GNSS-denied drift | ~2 km per 100 km (Lima); ~1.7 km per 100 km of EW cover for laser-gyro missiles (Night Watch) | km | e9-counteroffensive-26 | kyivpost-2026-lima; militarnyi-2026-kozatskyi-lima-half |
| Interceptor kills: Oct 2024 → summer 2025 | several hundred → >1,000 | per month | e7 → e8 | kse-2025-drone-innovations |
| Frontline targets engaged by UAVs | 80–85 | % | e8-drone-kill-zone | kse-2025-drone-innovations |
| TFL-1 autonomous terminal leg | ~500 | m | e8-drone-kill-zone | kyivpost-2025-orlova-tfl1 |
| TFL-1 effectiveness gain (maker claim) | 2–4 | × | e8-drone-kill-zone | kyivpost-2025-orlova-tfl1 |
| Kvertus Mirage node reach | 20 (directional) / 8 (omni) | km | e8-drone-kill-zone | pravda-2025-fpv-ew-shield |
| Russian smuggled Starlink terminals (claim) | 50,000+ | terminals | e9-counteroffensive-26 | ukrinform-2026-liskovych-starlink-c2 |
| Drop in Russian drone effectiveness after the Starlink cut (Biletsky) | 20–40 | % in 2 weeks | e9-counteroffensive-26 | isw-2026-feb-26-assessment |
| Territory retaken within 5 days of the Starlink cut (contested) | 200+ | km² | e9-counteroffensive-26 | atlanticcouncil-2026-spencer-starlink-crisis |
| Ubiquiti share of Russian frontline radio links | ~80 | % | e8 → e9 | militarnyi-2026-ubiquiti-80 |
| Russian mesh drone-control range | about half the 2026 figure (2025) → 150–175 (2026) | km | e8 → e9 | militarnyi-2026-khomenko-mesh-175km |
| Rassvet coverage over Ukraine | 6–10 min passes (Mar) → 2 windows up to 90 min total (ISW) or >1 h each (Aug) | per day | e9-counteroffensive-26 | euromaidan-2026-thomas-isw-rassvet; euromaidan-2026-vivdych-rassvet-windows |
| Rassvet satellites working / needed | 12 of 32 launched / 200–300 | satellites | e9-counteroffensive-26 | euromaidan-2026-thomas-isw-rassvet |
| Volna Kupol Garant coverage | ~20 km², up to 16 km | per unit | e9-counteroffensive-26 | militarnyi-2026-pryhodko-volna-kupol-garant |
| Starlink packet loss under Russian jamming | 0–35 | % | e9-counteroffensive-26 | militarnyi-2026-stepanets-ew-vs-starlink |
| Interceptor sorties and kills (Feb 2026) | ~6,300 sorties / >1,500 kills | count | e9-counteroffensive-26 | isis-2026-anokhin-shahed-monthly |
| Shahed kills by interceptors (24 May 2026) | >40 | % | e9-counteroffensive-26 | militarnyi-2026-pryhodko-interceptors |
| Shahed-type launches intercepted (May 2026, MoD) | 7,476 of 8,150 (91.7%) | count | e9-counteroffensive-26 | pravda-2026-sting-3000 |
| Shahed/Geran hit rate (May 2026) | ~6.7 | % | e9-counteroffensive-26 | isis-2026-anokhin-shahed-monthly |
| Interception rate: piston Shahed vs jet Geran | 90+ vs ~60 | % | e9-counteroffensive-26 | united24-2026-place-jet-gerans |
| Skynex battery kills to date (one battery) | ~35 Shaheds + 2 cruise missiles | count | e9-counteroffensive-26 | militarnyi-2026-kozatskyi-skynex-12 |
| Skynex guns failing in one engagement | 3 of 8 | guns | e9-counteroffensive-26 | militarnyi-2026-khomenko-skynex-issues |
| Geran-4/5 production (HUR claim) | ~3,000 | per month | e9-counteroffensive-26 | united24-2026-place-jet-gerans |
| Jet share of Shahed-type launches (May 2026) | 0.5–1.5 | % | e9-counteroffensive-26 | isis-2026-anokhin-shahed-monthly |
| FPVs to kill an Arena-M tank | 15–20 (7 to empty the APS) | drones | e9-counteroffensive-26 | militarnyi-2026-pryhodko-aps-80 |
| Ukrainian APS interception of FPVs (tests) | 60–80 | % | e9-counteroffensive-26 | militarnyi-2026-pryhodko-aps-80 |
| Chuika 4.0 bands | 1,080–2,200 / 2,860–4,500 / 4,860–6,040 / 6,040–8,800 | MHz | e9-counteroffensive-26 | militarnyi-2026-khomenko-chuika-4 |
| EW preparation before Vivaldi | 9 | months | e9-counteroffensive-26 | euobserver-2026-vasilko-vivaldi |

The figures below are contested; the table shows the ranges, and this is who claims what:
- **FPV success rates.** RUSI's 60–80% failure rate comes from field interviews; makers' 2–4× gains come from marketing. No 2026 equivalent exists. The nearest 2026 data points are the fibre share (70% in the National Guard) and "drones per kill" against APS-protected tanks (15–20).
- **Excalibur.** The 55% → 6% drop is the Washington Post's reading of internal Ukrainian data. Some secondary summaries say "70% → 6%", and we could not find that in RUSI's text.
- **Starlink cut date.** 1 February (ISW, 26 Feb), 2 February (the cabinet resolution) and 5 February (terminals blocked; Ukrinform, and ISW as relayed by Euromaidan). The whitelist was phased, so these are all defensible.
- **Starlink-cut gains.** Reports range from 10–12 km (Wikipedia, southern front) to more than 200 km² (Atlantic Council). These use different units, depth of advance and area retaken, so they don't directly conflict. They still need a common measure, such as DeepStateMap's monthly km² statistics. Biletsky's 20–40% effectiveness drop is the only measured effect on Russian drone output.
- **Russian anti-Starlink effect.** A Russian industry source claims Volna has "shut down Starlink" wherever needed. A Ukrainian engineer measures 0–35% packet loss.
- **Lima claims.** The Kinzhal counts (41 of 42 in April, 61 by July) and "KABs to zero" all come from the developer and the Night Watch unit that fields Lima. Hambling flags them as unverified.
- **Interceptor shares.** Figures vary with scope: one night, the Kyiv region, or a month. Official "shot down/suppressed" totals lump kinetic and EW kills together [src:isis-2026-anokhin-shahed-monthly].
- **Rassvet numbers.** ISW counts 12 working satellites giving up to 90 minutes a day in total; Euromaidan's tracking-based count is two windows of more than an hour each. Both sources give a working altitude of about 550 km [src:euromaidan-2026-thomas-isw-rassvet; src:euromaidan-2026-vivdych-rassvet-windows].

## 5. Russia's side as reported

**Systems.** Ukrainian and Western sources name:
- **Krasukha-4 (1RL257):** two KamAZ-6350 vehicles, three steerable dishes; jams airborne radar and UAV control links. Its 300 km range is unverified. Only two recorded losses by November 2023 [src:militarnyi-2025-krasukha-4-leak; src:militarnyi-2023-kushnikov-krasukha-4-zaporizhzhia]. No fetched source covers Krasukha-2 in Ukraine.
- **R-330Zh Zhitel:** jams GPS, Inmarsat, Thuraya and GSM out to about 30 km, and geolocates satcom, cell towers and drone control points. 24 confirmed losses by May 2026 [src:militarnyi-2026-pryhodko-lasars-zhitel; src:eurosd-2024-withington-excalibur].
- **Shipovnik-Aero:** low-signature, imitates other emitters, reaches about 10 km, takes over UAVs and geolocates operators [src:rusi-2023-watling-meatgrinder; src:euromaidan-2023-voichuk-economist-ew].
- **Borisoglebsk-2:** a multi-function EW complex; one R-934BMV station was captured in Kharkiv oblast in 2022 [src:militarnyi-2022-kushnikov-zhitel-kharkiv].
- **Torn-MDM:** communications interception [src:rusi-2023-watling-meatgrinder].
- **Pole-21:** wide-area GNSS denial, now treated as expendable [src:rusi-2023-watling-stormbreak].
- **Tobol, Tirada-2:** strategic satellite-communications jammers. Russian authors say they "have not been seen in actual combat" against Starlink [src:militarnyi-2026-glazyev-starlink-scenarios].
- **Volna Kupol Garant:** a trailer-mounted Starlink uplink jammer (14–14.5 GHz, 120 kg, about 20 km² per unit, about $1.5M). In serial production per Russian claims, and struck by Ukraine [src:militarnyi-2026-pryhodko-volna-kupol-garant; src:militarnyi-2026-shumlianskyi-volna-mass-production; src:militarnyi-2026-glazyev-starlink-scenarios].
- **Platoon- and vehicle-level counter-UAS:** directional jammers and UAV-hijacking arrays [src:rusi-2023-watling-meatgrinder], and the Volnorez magnet-mounted cone jammer ("dome") [src:militarnyi-2024-kushnikov-volnorez]. Russian trench-jammer families beyond Volnorez have no fetched spec source.
- **Shtora:** an image-spoofing system aimed at drone video feeds [src:militarnyi-2026-khomenko-t200-shtora].
- **Arena-M:** an active protection system adapted to FPVs and first used on a T-72B3A in July 2026 [src:militarnyi-2026-pryhodko-aps-80].

**Doctrine.** Russian EW teams are "weapons free", with little interest in deconfliction [src:rusi-2023-watling-meatgrinder]. After 2023, Russia shifted from big Soviet-style platforms to dispersed, disposable antennas because Ukraine targets emitters [src:rusi-2023-watling-stormbreak]. By 2025, navigational jamming was "ubiquitous" and most vehicles carried jammers [src:rusi-2025-watling-third-year]. Russia uses ELINT satellites to map Ukrainian emitters and route strikes around EW-saturated zones [src:pravda-2025-fpv-ew-shield]. Ukrainian experts credit Russia with methodical, almost monthly upgrades and with scaling: "They know how to scale" [src:euromaidan-2026-mukhina-crpa-20].

**Adaptations Russia led:**
- **Fibre-optic FPVs.** KVN, developed through the Ushkuinik accelerator, and the Rubicon unit [src:kyivindependent-2025-farrell-fiber-optic].
- **Multi-element CRPA (Kometa-M)** on Shaheds, UMPK glide-bomb kits and Iskander-K [src:defenseexpress-2025-kometa-m-16]. It then moved to a new CRPA series that beats jammer-count arithmetic [src:forbes-2026-hambling-lima-quant].
- **Mesh radio control** of Gerans and Gerbera via Chinese COFDM modems and relay towers [src:euromaidan-2026-vivdych-xk-f358; src:militarnyi-2026-khomenko-mesh-175km].
- **Vehicle cages and "turtles"** [src:kyivpost-2024-turtle-tanks].
- **Road nets** [src:defenseexpress-2025-russian-net-tunnels].
- **Kh-101 improvements** for EW-resistant terminal navigation [src:rusi-2025-watling-third-year].
- **Frequency hopping on Molniya** [src:euromaidan-2026-mukhina-molniya], then digital video [src:euromaidan-2026-mukhina-molniya-digital].
- **Jet Gerans** [src:united24-2026-place-jet-gerans].

**Adaptations Russia followed.** Interceptor drones came about a year after Ukraine's [src:kse-2025-drone-innovations].

**Dependencies.** Starlink had become critical to Russian UAV and assault C2 before February 2026 [src:ukrinform-2026-liskovych-starlink-c2]. Its domestic alternatives are reported as inferior: Gazprom Space Systems and the Rassvet project [src:atlanticcouncil-2026-spencer-starlink-crisis; src:ukrinform-2026-liskovych-starlink-c2]. As of September 2026, Rassvet gives only scheduled windows, and Beskrestnov calls a finished Rassvet "a small catastrophe" because it would give Russia a permanent satellite link to its long-range weapons [src:euromaidan-2026-mukhina-geran-mesh]. Russia's terrestrial fallback depends on Western and Chinese radios: Ubiquiti and the XK-F358 [src:militarnyi-2026-ubiquiti-80; src:euromaidan-2026-vivdych-xk-f358].

## 6. Game/sim relevance

- **Tech tree with obsolescence timers.** Each item has an effectiveness curve that decays once the enemy fields a counter. The lags from section 2 give plausible decay times:
  - Frequency moves: weeks. A band change can be a firmware push, so model it as a software event that needs no factory time [src:pravda-2025-fpv-ew-shield].
  - Cages and antennas: 3–9 months.
  - Systemic counters (Starlink, GPS weapons, mesh, satellite constellations): 6–24 months.
  - Excalibur's fall from 55% to 6% is a ready-made decay curve.
- **EW bubbles as map layers.** Model GNSS-denial fields (Pole-21, Zhitel, Lima/Pokrova) separately from control- and video-link jamming domes (trench, vehicle and backpack jammers). Each field has a band set. A drone with a matching band, or a fibre, autonomy, mesh or CRPA upgrade, ignores the corresponding bubble. Dome radius is about 200–300 m for omni jammers and several km for directional ones [src:pravda-2025-fpv-ew-shield].
- **Friendly-fire toggle.** Friendly jamming also blocks your own drones, so let players schedule "EW pauses" to launch strikes [src:rusi-2025-watling-third-year]. Lima-Quant's makers say brigade commanders switch it off because it blinds their own Mavics, a direct trade-off between strategic protection and tactical ISR [src:forbes-2026-hambling-lima-quant].
- **CRPA arms race as a ladder.** Each Russian CRPA tier (4, 8, 12, 16, 20, 24) needs more jammers or a new Ukrainian technique. The late tiers break the "one jammer per element" rule [src:forbes-2026-hambling-lima-quant; src:euromaidan-2026-mukhina-crpa-20]. The end state is optical terrain matching, which ignores GNSS jamming altogether [src:militarnyi-2026-leonov-geran-5-optical].
- **Emission signatures.** Every emitter (jammer, drone ground station, Starlink dish, Volna uplink jammer) adds to a detectable signature. Direction finding and EW-hunting FPVs let the enemy strike emitters, so "emit and die" is a trade-off rather than a free buff [src:militarnyi-2025-kushnikov-ew-hunter-fpv; src:rusi-2023-watling-stormbreak]. Big emitters are rare and valuable: 24 Zhitels confirmed hit in four years, some 55 km deep [src:militarnyi-2026-pryhodko-lasars-zhitel; src:euromaidan-2026-mukhina-zhitel-55km].
- **Comms backbone as a strategic toggle.** Starlink access is an event-driven, side-specific modifier. Its removal in February 2026 should cut enemy drone effectiveness by roughly 20–40% for weeks, with partial recovery over one to two months as workarounds come online [src:isw-2026-feb-26-assessment]. The recovery paths are separate techs:
  - Terrestrial mesh: 150–175 km range, but needs relay towers you can strike [src:militarnyi-2026-khomenko-mesh-175km].
  - Fibre.
  - Rassvet: model it as scheduled satellite windows that lengthen with each launch [src:euromaidan-2026-thomas-isw-rassvet].
- **Satcom jamming is partial.** Model anti-Starlink jammers as packet loss (0–35%) in a cell about 17 km across rather than as a blackout. Unhardened drone video fails; hardened C2 degrades [src:militarnyi-2026-stepanets-ew-vs-starlink].
- **Detector blindness.** Analogue-video detectors (Chuika) warn infantry only while the enemy uses analogue video in covered bands. Digital video removes the warning until a bearing-only detector is researched [src:euromaidan-2026-mukhina-molniya-digital; src:militarnyi-2026-khomenko-chuika-4].
- **Fibre vs nets vs autonomy** are three counters to EW, each with its own cost:
  - **Fibre:** limited range, snag risk, visible cable. Adoption can reach 70% of a force's drones [src:euromaidan-2026-vivdych-ng-fibre-70].
  - **Nets:** fixed infrastructure, which burns and can be bypassed.
  - **Autonomy:** needs machine-vision research, and the unit costs 10–20% more.
- **Active protection as ammunition.** APS should hold a finite load that saturation attacks deplete: 7 FPVs emptied an Arena-M, and the kill took 15–20 [src:militarnyi-2026-pryhodko-aps-80].
- **Gun air defence has a reliability roll.** Guns are cheap per kill and good for point defence (about 4 km), but can fail mid-engagement [src:militarnyi-2025-shumlianskyi-skynex-shaheds; src:militarnyi-2026-khomenko-skynex-issues].
- **Strike-war economics.** Interceptor effectiveness against piston Shaheds should drop sharply when jet Gerans enter the enemy mix. Mobile fire groups and propeller interceptors should become near-useless against jets until a jet-interceptor tech with machine-vision lock is researched [src:euromaidan-2026-mukhina-jet-interceptors; src:pravda-2026-ihnat-jet-drones].
- **Thermal masking is partial.** Anti-thermal ponchos should reduce detection probability at range, not grant invisibility, and moving cancels most of the benefit [src:militarnyi-2025-yan-russian-thermal-ponchos].

## 7. Terms

- **REB** *(radioelektronna borotba)* — electronic warfare. **REBivets** — an EW operator.
- **okopnyi REB** — "trench EW": a small jammer protecting a dugout or position.
- **kupol** — "dome": a jammer's protective bubble over a position or vehicle. It is also part of the name of the Russian Starlink jammer "Volna Kupol Garant".
- **ryukzak REB** — backpack EW jammer, e.g. Kvertus.
- **optovolokno / "optyka"** — fibre-optic drone, controlled by an unspooling cable and immune to RF jamming.
- **zhdun (pl. zhduny)** — "waiter": an ambush drone that lands near a road and waits for a target; it can hide under nets.
- **mangal** — "barbecue grill": slang for a cope cage on a vehicle.
- **cherepakha** — "turtle": a Russian vehicle fully boxed in against drones.
- **sitka / antydronova sitka** — net / anti-drone net; **tunel** — a net tunnel over a road.
- **mobilni vohnevi hrupy** — mobile fire groups: truck-mounted machine-gun teams hunting Shaheds.
- **perekhoplyuvach** — interceptor (drone).
- **moped** — Ukrainian slang for a Shahed/Geran, from its engine sound.
- **Chuika** — "intuition, sixth sense": a BlueBird drone-video detector.
- **Pokrova** — "Intercession" or "protective veil": Ukraine's nationwide GNSS-spoofing system. Lima is its main component.
- **CRPA** — controlled reception pattern antenna: a multi-element anti-jam GNSS antenna (Russian Kometa-M).
- **PRFH** — pseudo-random frequency hopping.
- **mesh (MESH)** — a radio network in which every node relays for the others, so links survive the loss of any node. Used for Russian Geran control and for Ukrainian tactical radios.
- **analogue vs digital video** — analogue FPV video can be picked up and viewed by cheap detectors; digital video can only be sensed as a signal, not viewed.
- **Shtora** — "curtain": a Russian image-spoofing system aimed at drone video feeds.
- **APS** — active protection system: radar-cued launchers that fire at incoming munitions or drones (Russian Arena-M).
- **GNSS jamming / spoofing** — drowning out, or faking, satellite navigation signals.
- **INS** — inertial navigation, the fallback when GNSS is denied; it drifts over time.
- **last mile / terminal guidance** — machine-vision homing for the final few hundred metres after the RF link drops.
- **emit and die** — any detectable emission invites direction finding and a strike.
- **whitelist** — the Feb 2026 Starlink registry; unregistered terminals were cut off.
- **kill zone** *(zona urazhennia)* — the drone-dominated band behind the line of contact.

## 8. Open questions/gaps

- **Russian systems still thin.** Krasukha-2 in Ukraine, Pole-21 specs and Russian trench-jammer families beyond Volnorez have no fetched spec source. Militarnyi and Euromaidan site searches returned only loss reports; Defense Express's search is script-driven and could not be queried.
- **Ukrainian systems.** Nota has no spec source beyond being named among the large vehicle systems pulled back from the front. Bukovel specs still rest on Wikipedia.
- **Gun and passive defence over time.** No series of kill shares by weapon type (mobile fire groups, Gepard, Skynex, interceptors, EW) was found for 2022–26. We have only snapshots: EW about 50% (a Night Watch claim), interceptors more than 40% on one night, and jets outrunning mobile fire groups in 2026. Official totals lump kinetic and EW kills.
- **Digital links.** The Russian switch to digital video is documented for Molniya only, and the Ukrainian digital or encrypted video share is unquantified.
- **Firmware cadence.** It is quantified for DJI war firmware (more than 40 versions in two years) but not for DELTA, Kropyva or Ukrainian FPV firmware.
- **Current FPV loss rates.** There is no 2026 equivalent of RUSI's 60–80% failure figure. RUSI's publication listings are script-driven and no 2026 RUSI tactical report was found.
- **Starlink-cut effects.** The ISW 8 and 26 February assessments are now included (via Wayback). A common km² measure of the gains is still missing.
- **Russian recovery.** Terminal types after the cut (Yamal/Express GEO terminals, bribed whitelist registrations at scale) have no fetched source. How much of Russia's pre-cut drone depth has returned by September 2026 is not measured.

## 9. Sources

- **rusi-2022-zabrodskyi-preliminary-lessons** — 2022 baseline: 90% of UAS lost; EW as the main counter-UAS tool.
- **rusi-2023-watling-meatgrinder** — Russian EW density, Shipovnik-Aero, 10,000 UAVs lost a month.
- **rusi-2023-watling-stormbreak** — "Binary" UAV use under EW; dispersed and disposable Russian EW; GPS-jamming tell.
- **rusi-2024-watling-offensive-lessons** — Hard counters to Excalibur and GMLRS; the need for field updates.
- **rusi-2025-watling-third-year** — 60–80% FPV failure; fibre; interceptors; EW pauses; Shahed defence.
- **kyivpost-2024-chiu-gps-weapons** — Excalibur 55% → 6%; JDAM-ER misses.
- **eurosd-2024-withington-excalibur** — Technical GPS-jamming explainer; M-code and home-on-jam.
- **wikipedia-zhitel** — JDAM error under jamming; GMLRS software fix; Excalibur kill of a Zhitel (navigation only).
- **wikipedia-bukovel** — Ukrainian Bukovel-AD counter-UAS (navigation only).
- **wikipedia-starlink-war** — Starlink timeline from 2022 to 2026 (navigation only).
- **aljazeera-2024-gur-starlink** — GUR confirms "systemic" Russian Starlink use (Feb 2024).
- **kyivindependent-2026-myronyshena-starlink-whitelist** — The Feb 2026 whitelist resolution.
- **ukrinform-2026-liskovych-starlink-c2** — Collapse of Russian C2; speed cap; workarounds.
- **isw-2026-feb-08-assessment** — ISW on early Starlink-cut effects; BAI under strain.
- **isw-2026-feb-26-assessment** — Biletsky's 20–40% drop; recovery timeline; fibre to Kharkiv.
- **atlanticcouncil-2026-spencer-starlink-crisis** — Effects of the Starlink cut; 200+ km² in 5 days.
- **militarnyi-2026-ubiquiti-80** — Ubiquiti as 80% of Russian frontline links.
- **euromaidan-2026-vivdych-xk-f358** — Chinese mesh modem behind Russian jam-resistant drones.
- **militarnyi-2026-khomenko-mesh-175km** — Russian mesh control range 150–175 km.
- **euromaidan-2026-mukhina-geran-mesh** — Ukraine's year-long lag on countering mesh; Rassvet warning.
- **euromaidan-2026-thomas-isw-rassvet** — ISW on Rassvet's stalled second batch and coverage windows.
- **euromaidan-2026-vivdych-rassvet-windows** — Rassvet coverage growth, March to August 2026.
- **militarnyi-2026-stepanets-ew-vs-starlink** — Physics of Starlink jamming; 0–35% packet loss.
- **militarnyi-2026-pryhodko-volna-kupol-garant** — Volna Kupol Garant specs and weaknesses.
- **militarnyi-2026-shumlianskyi-volna-mass-production** — Russian claim of serial production.
- **militarnyi-2026-glazyev-starlink-scenarios** — Tobol and Tirada-2 not seen in combat (Russian authors).
- **militarnyi-2025-krasukha-4-leak** — Krasukha-4 internals; unverified 300 km range.
- **militarnyi-2023-kushnikov-krasukha-4-zaporizhzhia** — Second Krasukha-4 loss; camouflage in forest belts.
- **militarnyi-2022-kushnikov-zhitel-kharkiv** — 2022 Zhitel kills; Borisoglebsk-2 capture.
- **militarnyi-2026-pryhodko-lasars-zhitel** — Zhitel specs; 24 confirmed losses.
- **euromaidan-2026-mukhina-zhitel-55km** — Zhitel killed 55 km behind the front.
- **euromaidan-2023-voichuk-economist-ew** — Zaluzhnyi's 60 EW types; Shipovnik-Aero 10 km; Pokrova.
- **militarnyi-2024-kushnikov-volnorez** — Volnorez vehicle dome jammer.
- **kyivindependent-2025-farrell-fiber-optic** — Russia's fibre lead and Ukraine's catch-up (May 2025).
- **atlanticcouncil-2026-sutea-fiber-optic** — Fibre retrospective: ranges and counters.
- **euromaidan-2026-vivdych-ng-fibre-70** — National Guard fibre share 20% → 70%.
- **united24-2025-barkhush-fiber-counters** — Unit-level counters to fibre drones.
- **euromaidan-2025-murdoch-net-tunnels** — Zaporizhzhia road net tunnels.
- **defenseexpress-2025-russian-net-tunnels** — Russian net tunnels near Bakhmut.
- **kyivpost-2024-turtle-tanks** — Cope cage → turtle → super turtle chain.
- **militarnyi-2026-pryhodko-aps-80** — Arena-M against FPVs; Ukrainian APS 60–80% in tests.
- **defenseexpress-2025-kometa-m-16** — The Kometa-M CRPA ladder, up to 16 elements.
- **united24-2025-khomenko-kometa-umpk** — 12-channel Kometa on UMPK glide bombs.
- **forbes-2026-hambling-lima-quant** — New-series CRPA; Lima-Quant claims; friendly-fire bans.
- **militarnyi-2026-khomenko-crpa-32** — 32-element CRPA recovered from a Shahed.
- **euromaidan-2026-mukhina-crpa-20** — 20-element CRPA due Sep 2026, 24 in 2027.
- **militarnyi-2026-leonov-geran-5-optical** — Reported optical terrain-matching Geran-5 (unconfirmed).
- **kyivpost-2026-lima** — Lima GNSS jammer/spoofer claims.
- **militarnyi-2026-khomenko-pokrova-lima** — Pokrova structure; Lima share of it.
- **militarnyi-2026-kozatskyi-lima-half** — EW as half of air-target suppression; 61 Kinzhals (claims).
- **militarnyi-2025-pryhodko-ew-340x** — 340-fold EW output growth; 140+ makers.
- **euromaidan-2024-martyniuk-ew-backpacks** — Backpack and trench jammers; FPV band shift.
- **euromaidan-2025-zoria-jamming-fails** — An EW commander's 2022–25 account; module escalation; big EW pulled back.
- **defensepost-2025-encarnacion-atlas** — Networked EW (Kvertus Atlas).
- **pravda-2025-fpv-ew-shield** — FPV band split; firmware-tunable chips; jammer radii; Atlas costing.
- **euromaidan-2025-martyniuk-drone-firmware** — DJI firmware cadence (40+ versions in two years).
- **united24-2026-brizard-detectors** — Chuika and ZORKO detectors; monthly updates.
- **militarnyi-2026-khomenko-chuika-4** — Chuika 4.0 bands widened for Molniya.
- **euromaidan-2026-mukhina-molniya** — Molniya frequency hopping defeats jamming.
- **euromaidan-2026-mukhina-molniya-digital** — Molniya digital video blinds analogue detectors.
- **militarnyi-2026-khomenko-t200-shtora** — Russian Shtora video spoofing; channel-hopping interceptor.
- **euromaidan-2026-kossov-himera-mesh** — Ukrainian mesh radios under jamming.
- **kyivpost-2025-orlova-tfl1** — TFL-1 terminal autonomy in mass production.
- **militarnyi-2025-kushnikov-ew-hunter-fpv** — EW-hunting FPV ("emit and die").
- **militarnyi-2025-yan-russian-thermal-ponchos** — Anti-thermal ponchos: partial concealment only.
- **kse-2025-drone-innovations** — Interceptor scaling, the adaptation loop, drone share of engagements.
- **militarnyi-2026-pryhodko-interceptors** — More than 40% of Shahed kills by interceptors (24 May 2026).
- **pravda-2026-sting-3000** — Sting's May 2026 kills; 91.7% monthly interception.
- **isis-2026-anokhin-shahed-monthly** — Monthly Shahed data; interceptors; CRPA; jet share; mesh relays.
- **militarnyi-2025-shumlianskyi-skynex-shaheds** — Skynex battery composition and AHEAD rounds.
- **militarnyi-2026-kozatskyi-skynex-12** — One Skynex battery's kill tally.
- **militarnyi-2026-khomenko-skynex-issues** — Skynex failures in a 1 April 2026 engagement.
- **militarnyi-2025-gepard-repair** — Gepard's role; domestic repair.
- **pravda-2026-ihnat-jet-drones** — Jets outrun mobile fire groups and interceptors.
- **united24-2026-place-jet-gerans** — Jet Geran speeds, production, interception drop.
- **euromaidan-2026-mukhina-jet-interceptors** — Why the first jet interceptors failed.
- **kyivindependent-2026-farrell-vivaldi** — Operation Vivaldi overview.
- **mwi-2026-rose-vivaldi** — Vivaldi as drone and EW dominance; Rubicon hit.
- **euobserver-2026-vasilko-vivaldi** — Nine-month EW preparation for Vivaldi.
- **cnas-2024-pettyjohn-evolution** — Early-2024 Western view: EW as the best counter-drone tool.
