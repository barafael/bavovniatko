# 02 — Electronic warfare and the measure → countermeasure cycle

*Research base for bavovniatko. Accessed 2026-09-28. Russian capabilities are described only as reported by Ukrainian and Western sources. Source metadata: `sources/02-ew-countermeasures.yaml`.*

## 1. Overview

Electronic warfare (EW; Ukrainian **REB**, *radioelektronna borotba*) has set the terms of the drone war since 2022. From the start, RUSI's Ukrainian-sourced data showed that about 90% of UAS employed were lost, with EW as the main counter-UAS tool, and that precision weapons "can be defeated by EW" [src:rusi-2022-zabrodskyi-preliminary-lessons]. The same pattern has repeated ever since:

1. **A new capability arrives and dominates.** Examples: GMLRS, Excalibur, commercial quadcopters, analogue FPVs, Starlink, Shahed, fibre-optic FPVs, interceptor drones.
2. **The other side finds the physical-layer weakness.** Examples: the GNSS signal, the control or video frequency, the satellite link, the emitter signature, the cable, speed.
3. **The counter is cheap, spreads fast, and forces a counter-counter.** Examples: moving frequencies, CRPA antennas, fibre, autonomy, whitelists, nets, jet engines.

**Who is ahead changes by domain and by era.**
- **2023: Russia led the tactical EW fight.** It fielded about one major EW system per 10 km of front, and Ukraine lost about 10,000 UAVs a month [src:rusi-2023-watling-meatgrinder]. It also blunted US GPS-guided munitions: Excalibur's hit rate fell from 55% to 6% [src:kyivpost-2024-chiu-gps-weapons].
- **2024–25: fibre and CRPA shifted the edge back to Russia.** Russia fielded fibre-optic FPVs first, from August 2024 in Kursk, and moved to multi-element CRPA antennas (Kometa-M) to beat Ukrainian GNSS jamming [src:kyivindependent-2025-farrell-fiber-optic; src:defenseexpress-2025-kometa-m-16].
- **2025: Ukraine answered in several ways at once.**
  - GNSS jamming and spoofing (Lima) [src:kyivpost-2026-lima].
  - Interceptor drones: first against reconnaissance UAVs, then against Shaheds [src:kse-2025-drone-innovations].
  - Terminal autonomy (TFL-1) [src:kyivpost-2025-orlova-tfl1].
  - Nets over logistics roads [src:euromaidan-2025-murdoch-net-tunnels].
- **February 2026 was the biggest single swing.** The Starlink whitelist cut Russia off from the satellite link that had become the backbone of its drone and assault C2 [src:ukrinform-2026-liskovych-starlink-c2; src:atlanticcouncil-2026-spencer-starlink-crisis]. Russia's answer in the strike war is jet-powered Gerans, which outran the cheap propeller interceptors [src:united24-2026-place-jet-gerans].

**How fast the cycle runs.** At the tactical radio-frequency level, participants describe new frequency variants appearing "every month and every week" [src:pravda-2025-fpv-ew-shield], and detector makers say they must update hardware monthly [src:united24-2026-brizard-detectors]. At the materiel level (cages, antennas, fibre spools, interceptors), the lag is typically 3–9 months. At the systemic level (Starlink, US GPS weapons), it is 6–24 months. KSE describes Ukraine's "development – combat testing – modification" loop as running in months, with innovation cycles "measured in weeks" [src:kse-2025-drone-innovations]. RUSI's lesson for everyone is that holding a technological edge "requires the ability to rapidly update systems in the field" [src:rusi-2024-watling-offensive-lessons].

## 2. Measure → countermeasure chain

Lag is the approximate time from the measure appearing at scale to the counter appearing at scale. It is estimated from the dates in the cited sources.

| date / era | measure (who) | countermeasure (who) | lag | source id |
|---|---|---|---|---|
| Feb 2022 / e1 | Russian EW and strikes threaten Ukrainian C2 (RU) | Starlink terminals requested 26 Feb and first shipment 28 Feb; more than 5,000 by 6 Apr (UA/SpaceX) | days | wikipedia-starlink-war |
| 2022 / e1–e2 | Commercial quadcopters and TB2 for recon and fires (UA) | Dense Russian EW, which becomes the main counter-UAS; about 90% of UAS lost (RU) | immediate | rusi-2022-zabrodskyi-preliminary-lessons |
| May–Sep 2022 / e2–e3 | Big Russian EW platforms (Zhitel) jam GPS, satcom and GSM (RU) | Ukraine hunts emitters: Zhitel units killed by artillery (May 2022), TB2 (Sep 2022) and Excalibur (Mar 2023) (UA) | months | wikipedia-zhitel |
| Jul 2022 / e2 | GMLRS/HIMARS "devastates" Russian C2 and logistics (UA) | HQs pulled back and hardened, SHORAD linked to long-range radar, EW; a "proportion" of GMLRS intercepted by spring 2023; US modifies GMLRS software (RU → US) | about 6–9 months | rusi-2023-watling-meatgrinder; wikipedia-zhitel; rusi-2024-watling-offensive-lessons |
| Dec 2022–Aug 2023 / e4–e5 | Excalibur GPS-guided 155 mm shells (UA/US) | Russian GNSS jamming and spoofing: hit rate 55% (Jan 2023) → 6% (Aug 2023); cost per successful strike $300k → $1.9M (RU) | about 6 months | kyivpost-2024-chiu-gps-weapons |
| Feb 2023 / e4 | JDAM-ER glide bombs (UA/US) | Jamming causes misses of 19 m to about 3/4 mile (RU). Counter-counter: upgraded kits, add-on seekers, home-on-jam (US) | weeks | kyivpost-2024-chiu-gps-weapons; eurosd-2024-withington-excalibur |
| 2023 / e4 | Mass Ukrainian UAV use | About one major EW system per 10 km of front; Shipovnik-Aero; platoon-level jammers and hijacking arrays; about 10,000 UAVs lost a month (RU) | already in place | rusi-2023-watling-meatgrinder |
| 2023 / e4–e5 | Ukrainian counter-battery and anti-emitter strikes on big EW platforms (UA) | Russia uses Zhitel more subtly, disperses antennas on light platforms, and treats Pole-21 as "disposable" (RU) | months | rusi-2023-watling-stormbreak |
| Summer 2023 / e5 | Russian EW makes Ukrainian bomber-drone use "binary" (RU) | Ukraine opens EW gaps with artillery SEAD and planned pauses in its own jamming (UA) | weeks–months | rusi-2023-watling-stormbreak; rusi-2025-watling-third-year |
| Mid-2023 → Aug 2024 / e5–e7 | Ukrainian FPVs kill armour (UA) | Cope cages *(mangaly)* (mid-2023), then factory cages, then "turtle" boxes with jammers (Apr 2024), then the "super turtle" T-80 (Aug 2024) (RU) | about 3 months to cages, 9–12 to turtles | kyivpost-2024-turtle-tanks |
| 2023–May 2024 / e6 | Russian FPVs on 850–930 MHz (RU) | Ukrainian backpack and trench jammers (Kvertus, 720–1050 MHz; Obrii) (UA). Counter-counter: Russia moves to 720–1020 MHz and mixes bands (RU) | months each step | euromaidan-2024-martyniuk-ew-backpacks |
| Feb 2024 / e6 | Russians use smuggled Starlink (83rd Air Assault Bde, near Klishchiivka) (RU) | GUR exposes it; SpaceX deactivations, with a claimed block in May 2024 that proved only partial (UA/SpaceX) | months (only fully solved Feb 2026) | aljazeera-2024-gur-starlink; wikipedia-starlink-war |
| Sep 2022 → 2025 / e3–e8 | Shahed/Geran waves; high-flying routes mapping Ukrainian EW and air defence (RU) | Distributed defence: helicopters, mobile fire groups, SPAAGs, MANPADS, "vast majority intercepted" (UA). Counter-counter: nav hardening and waves routed to avoid EW (RU) | months | rusi-2025-watling-third-year; rusi-2023-watling-stormbreak |
| Jul 2024 / e7 | GNSS-guided Shaheds, missiles and KABs (RU) | Lima GNSS jamming and spoofing (Cascade Systems), more than 400 systems; a claimed 20,500+ Shaheds jammed (UA) | years (maker claim) | kyivpost-2026-lima |
| Aug 2024 / e7 | Russian Orlan/Zala orbits: 1,000–1,500 a day (RU) | Radar-cued interceptor UAVs: several hundred kills a month (Oct 2024), more than 1,000 a month (summer 2025) (UA) | about 3–12 months | rusi-2025-watling-third-year; kse-2025-drone-innovations |
| Aug 2024 / e7 | Russian fibre-optic FPV "KVN" in Kursk, immune to jamming (RU) | Ukrainian Silkworm spool (Feb 2025), about 1 in 10 teams on fibre by May 2025; shotguns, nets, heated tungsten wire, net guns, daytime and foot logistics (UA) | about 6–9 months | kyivindependent-2025-farrell-fiber-optic; united24-2025-barkhush-fiber-counters |
| Jan–Jun 2025 / e7–e8 | Ukrainian GNSS jamming (UA) | Kometa-M CRPA: 12 elements on UMPK (Apr 2025), 16 on Shahed and Iskander-K (from Mar 2025); in theory one jammer is needed per element (RU) | months | defenseexpress-2025-kometa-m-16; united24-2025-khomenko-kometa-umpk |
| Feb 2025 / e8 | Ukrainian FPVs and bombers on Russian supply roads (UA) | Russian net tunnels, e.g. about 2 km between Bakhmut and Chasiv Yar (RU) | months | defenseexpress-2025-russian-net-tunnels |
| Early 2025 / e8 | Shahed campaign (RU) | First interceptor-drone Shahed kills (UA) | about 2.5 years after the first Shaheds | kse-2025-drone-innovations |
| Apr 2025 / e8 | Russian trench EW domes (RU) | FPV with a 12-band emitter detector that homes on and strikes jammers, i.e. "emit and die" (UA) | months | militarnyi-2025-kushnikov-ew-hunter-fpv |
| Mid-2025 / e8 | Ukrainian recon UAVs (UA) | Russian interceptor drones (RU) | about 12 months behind Ukraine | kse-2025-drone-innovations |
| 2025 / e8 | Standalone jammers overwhelmed by frequency diversity: 400–490 MHz, 720–1020 MHz, 2.1–2.3 GHz, plus hopping (RU) | Networked EW (Kvertus Atlas, Jul 2025) (UA) | months | pravda-2025-fpv-ew-shield; defensepost-2025-encarnacion-atlas |
| Sep 2025 / e8 | Jamming kills the RF link in the last mile (RU) | TFL-1 machine-vision terminal guidance for the last about 500 m, mass-produced with Vyriy (UA) | about 1 year from trials | kyivpost-2025-orlova-tfl1 |
| Sep 2025 / e8 | Fibre FPVs strike roads 25–30 km deep (RU) | Zaporizhzhia net tunnels: first 6.4 km, with hundreds of km planned (UA) | about 12 months after KVN | euromaidan-2025-murdoch-net-tunnels |
| 2 and 5 Feb 2026 / e9 | Russian drones and C2 on smuggled Starlink, including Starlink Minis on BM-35/Molniya/Shahed (RU) | Whitelist plus a 75–90 km/h speed cap: Russian C2 "collapsed"; more than 200 km² retaken in 5 days (UA/SpaceX) | about 2 years after GUR exposure | kyivindependent-2026-myronyshena-starlink-whitelist; ukrinform-2026-liskovych-starlink-c2; atlanticcouncil-2026-spencer-starlink-crisis |
| Feb 2026 / e9 | Starlink loss (RU) | Workarounds: bribed Ukrainian registrations, legacy radio, mesh, fibre, Rassvet satellites (RU) | weeks, partial | ukrinform-2026-liskovych-starlink-c2 |
| 2026 / e9 | Ukrainian detectors read analogue video (UA) | Russians move to non-standard bands, e.g. Molniya video at 4.1–4.5 GHz and control hopping across 300–600 MHz (RU) | about monthly | united24-2026-brizard-detectors; euromaidan-2026-mukhina-molniya |
| Feb–May 2026 / e9 | Massed piston Shaheds (RU) | Interceptor drones: about 6,300 sorties and more than 1,500 kills (Feb); more than 40% of Shahed kills on 24 May (UA) | – | isis-2026-anokhin-shahed-monthly; militarnyi-2026-pryhodko-interceptors |
| 2026 / e9 | Propeller interceptors and mobile fire groups (UA) | Jet Gerans (Geran-3/4/5, 300–600 km/h): interception falls from 90%+ to about 60% (RU) | about 12 months | united24-2026-place-jet-gerans |
| Jul–Sep 2026 / e9 | Jet Gerans (RU) | Jet-killer interceptors (Sting 2.0, LITAVR+); "speed alone" fails without machine-vision lock, loiter time and agility (UA) | months, still open | united24-2026-place-jet-gerans; euromaidan-2026-mukhina-jet-interceptors |
| May–Sep 2026 / e9 | Russian drone dominance near Lyman (RU) | Operation Vivaldi: 9 months of EW preparation, Russian drones "blinded", Rubicon hit, then UGVs and infantry (UA) | 9 months of preparation | euobserver-2026-vasilko-vivaldi; mwi-2026-rose-vivaldi; kyivindependent-2026-farrell-vivaldi |

## 3. Per-era notes

### e1-invasion (Feb–Apr 2022)
- **EW was already the main counter-UAS tool.** Ukraine lost about 90% of the UAS it employed, and RUSI calls EW "the primary means of CUAS" [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **Russian EW hurt its own side too.** Russian EW "rarely deconflict[s]", which causes fratricide and forces effects to be used one after another [src:rusi-2022-zabrodskyi-preliminary-lessons].
- **Starlink arrived within days.** The first terminals landed on 28 February 2022, two days after Fedorov's request, and more than 5,000 were in country by 6 April [src:wikipedia-starlink-war]. For the game, this is Ukraine's first "EW-resistant comms" card.
- **Ukraine already had domestic EW.** Bukovel-AD has been in service since 2016 against Orlan-10 [src:wikipedia-bukovel].

### e2-donbas-artillery (May–Aug 2022)
- **GMLRS arrived and devastated Russian C2 and logistics** in July 2022. Dispersed Russian air defence could not intercept it at first [src:rusi-2023-watling-meatgrinder].
- **Ukraine was already hunting emitters.** A Zhitel was destroyed by drone-cued artillery in May 2022 [src:wikipedia-zhitel].

### e3-counteroffensives-22 (Sep–Nov 2022)
- **More emitter kills.** A TB2 destroyed another Zhitel in September 2022 [src:wikipedia-zhitel].
- **Starlink limits.** Musk refused Starlink coverage around Crimea in September 2022 [src:wikipedia-starlink-war].
- **Software as a weak point.** RUSI's later review of the 2022–23 offensives says software-defined systems were "susceptible to targeted electronic warfare interference". It adds that Russia over time built "hard counters" to Excalibur and GMLRS [src:rusi-2024-watling-offensive-lessons].
- **The Shahed campaign began.** Ukraine's layered "distributed defence" against it is described in [src:rusi-2025-watling-third-year].

### e4-bakhmut (Dec 2022–May 2023)
- **Russian EW density (spring 2023):**
  - About one major system per 10 km of front, sited about 7 km back [src:rusi-2023-watling-meatgrinder].
  - "Weapons free", with no deconfliction.
  - Shipovnik-Aero rated especially effective because of its low signature and its ability to imitate other emitters.
  - Ukrainian UAV losses about 10,000 a month.
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
- **Cope cages** *(mangaly)* **appeared on Russian tanks** from mid-2023 [src:kyivpost-2024-turtle-tanks].

### e6-avdiivka-attrition (Oct 2023–Jul 2024)
- **Jammers in every trench.** The FPV boom brought mass trench and backpack jammers. Kvertus went from dozens a month (2022) to thousands (2024) [src:euromaidan-2024-martyniuk-ew-backpacks].
- **Frequency moves began.** Russian FPVs moved from 850–930 MHz to 720–1020 MHz, although 65–70% still used standard bands [src:euromaidan-2024-martyniuk-ew-backpacks].
- **Western framing.** CNAS called EW "the most effective way to stop drones" and noted operators being hunted with drone-tracking software [src:cnas-2024-pettyjohn-evolution].
- **Russian Starlink use confirmed.** GUR called it "systemic" in February 2024 [src:aljazeera-2024-gur-starlink].
- **Turtle tanks.** Russian "turtle tanks" with onboard jammers appeared in April 2024 [src:kyivpost-2024-turtle-tanks].
- **Lima began deploying** in July 2024 [src:kyivpost-2026-lima].

### e7-kursk-pokrovsk (Aug 2024–Mar 2025)
- **Fibre-optic drones arrived.** Russia's KVN fibre FPV appeared in Kursk in August 2024 [src:kyivindependent-2025-farrell-fiber-optic].
  - The Atlantic Council claims Ukraine lost 25% more vehicles than Russia in Kursk.
  - Russia kept a lead in fibre FPVs through 2025 [src:atlanticcouncil-2026-sutea-fiber-optic].
- **RUSI snapshot (Feb 2025)** [src:rusi-2025-watling-third-year]:
  - 60–80% of Ukrainian FPVs fail to reach their targets.
  - Most vehicles carry jammers.
  - Navigational jamming is "ubiquitous".
  - Fibre FPVs have about 10 km range.
  - Kh-101 upgraded for visual terrain tracking to beat terminal-phase EW.
- **Ukraine's interceptor drones against Russian reconnaissance UAVs scaled up** [src:rusi-2025-watling-third-year; src:kse-2025-drone-innovations].
- **A 16-element Chinese CRPA was found** on Russian drones in January 2025 [src:united24-2025-khomenko-kometa-umpk].

### e8-drone-kill-zone (Mar 2025–Jan 2026)
- **The CRPA ladder climbed.** Russian CRPA went to 12 elements on UMPK (April 2025) and 16 elements on Shahed and Iskander-K [src:defenseexpress-2025-kometa-m-16].
- **Ukraine hunted jammers directly.** It tested EW-hunting FPVs in April 2025 [src:militarnyi-2025-kushnikov-ew-hunter-fpv] and fielded the networked Atlas EW system in July 2025 [src:defensepost-2025-encarnacion-atlas].
- **Frequencies spread out.** One sector showed a 20% / 30% / 50% split across the 400 MHz, 900 MHz and 2.1–2.3 GHz bands [src:pravda-2025-fpv-ew-shield].
- **Fibre: Russia ahead, Ukraine catching up.** Ukraine had about 1 in 10 teams on fibre by May 2025 [src:kyivindependent-2025-farrell-fiber-optic].
- **Nets became infrastructure.** Russia built net tunnels in February 2025 [src:defenseexpress-2025-russian-net-tunnels], and Ukraine followed in Zaporizhzhia from September 2025 [src:euromaidan-2025-murdoch-net-tunnels].
- **Improvised counters to fibre drones** [src:united24-2025-barkhush-fiber-counters]:
  - Tungsten "hot wire" that snaps the cable.
  - Shotgun teams.
  - Net guns.
  - Nets, with the caveat that "waiting" drones *(zhduny)* can hide under them.
- **Autonomy went to mass production.** TFL-1 terminal autonomy entered mass production in September 2025 [src:kyivpost-2025-orlova-tfl1].
- **Interceptors turned on Shaheds.** Interceptor drones scored their first Shahed kills in early 2025, and Russia fielded its own interceptors from mid-2025 [src:kse-2025-drone-innovations].
- **Drones dominate the front.** 80–85% of frontline targets were engaged by drones [src:kse-2025-drone-innovations].

### e9-counteroffensive-26 (Feb 2026–present)
- **Starlink whitelist.** It was ordered on 2 February and Russian terminals were blocked by 5 February [src:kyivindependent-2026-myronyshena-starlink-whitelist; src:ukrinform-2026-liskovych-starlink-c2].
  - Beskrestnov: Russia's "entire C2 system has collapsed".
  - More than 200 km² retaken within 5 days, per the Atlantic Council [src:atlanticcouncil-2026-spencer-starlink-crisis].
  - Wikipedia's figure for the southern front is much smaller, "ten to twelve kilometers" [src:wikipedia-starlink-war], and ties it to an April start, so treat the gains as contested.
- **Interceptors took over the Shahed fight.** In February 2026 they flew about 6,300 sorties and made more than 1,500 kills; near Kyiv more than 70% of Shaheds fell to interceptors [src:isis-2026-anokhin-shahed-monthly].
  - They took more than 40% of Shahed kills on 24 May 2026 [src:militarnyi-2026-pryhodko-interceptors].
  - The Shahed hit rate fell to about 6.7% in May 2026 [src:isis-2026-anokhin-shahed-monthly].
- **Russia's answer was jet Gerans** [src:united24-2026-place-jet-gerans; src:euromaidan-2026-mukhina-jet-interceptors]:
  - About 3,000 Geran-4/5 a month, per HUR.
  - Interception rate against jets about 60%.
  - The first "ready" jet interceptors failed in combat.
- **Radio-frequency adaptation is continuous.** Molniya uses frequency hopping [src:euromaidan-2026-mukhina-molniya], and detectors need monthly updates [src:united24-2026-brizard-detectors].
- **Operation Vivaldi near Lyman** began with nine months of EW preparation to blind Russian drones, then struck the Rubicon unit and Russian logistics 50 km and then 120 km deep, then used UGVs [src:euobserver-2026-vasilko-vivaldi; src:mwi-2026-rose-vivaldi; src:kyivindependent-2026-farrell-vivaldi].

## 4. Key figures

| figure | value | unit | era | source id |
|---|---|---|---|---|
| UAS employed that are lost | 90 | % | e1-invasion | rusi-2022-zabrodskyi-preliminary-lessons |
| Russian EW density | ~1 major system per 10 | km of front | e4-bakhmut | rusi-2023-watling-meatgrinder |
| Ukrainian UAV losses | ~10,000 | per month | e4-bakhmut | rusi-2023-watling-meatgrinder |
| Excalibur hit rate, Jan 2023 → Aug 2023 | 55 → 6 | % | e4-bakhmut → e5-counteroffensive-23 | kyivpost-2024-chiu-gps-weapons |
| Cost per successful Excalibur strike | 0.3 → 1.9 | USD million | e5-counteroffensive-23 | kyivpost-2024-chiu-gps-weapons |
| JDAM-ER miss distance under jamming | 19 m to ~1,200 m (3/4 mile) | m | e4-bakhmut | kyivpost-2024-chiu-gps-weapons |
| JDAM error, unjammed → jammed | ~5 → ~30 | m | e5-counteroffensive-23 | wikipedia-zhitel |
| Russian FPV control band shift | 850–930 → 720–1020 | MHz | e6-avdiivka-attrition | euromaidan-2024-martyniuk-ew-backpacks |
| Kvertus backpack jammer cost | 7,000 | USD | e6-avdiivka-attrition | euromaidan-2024-martyniuk-ew-backpacks |
| Ukrainian FPVs failing to reach target | 60–80 | % | e7-kursk-pokrovsk | rusi-2025-watling-third-year |
| Share of damaged or destroyed Russian systems hit by tactical UAVs | 60–70 | % | e7-kursk-pokrovsk | rusi-2025-watling-third-year |
| Russian Orlan/Zala orbits (Aug 2024) | 1,000–1,500 | per day | e7-kursk-pokrovsk | rusi-2025-watling-third-year |
| Human counter-UAS efficiency (versus automated turrets) | ~25 | % | e7-kursk-pokrovsk | rusi-2025-watling-third-year |
| Fibre FPV range | ~10 (RUSI, Feb 2025); 10–15, with 20 in test (KI, May 2025); 30+ and ~40 for Birds of Magyar (AC, 2026) | km | e7 → e9 | rusi-2025-watling-third-year; kyivindependent-2025-farrell-fiber-optic; atlanticcouncil-2026-sutea-fiber-optic |
| Ukrainian drone teams using fibre (May 2025) | ~1 in 10 | ratio | e8-drone-kill-zone | kyivindependent-2025-farrell-fiber-optic |
| Fibre FPV threat depth from front | 25–30 | km | e8-drone-kill-zone | euromaidan-2025-murdoch-net-tunnels |
| Kometa-M CRPA elements | 4 (2022) → 12 (Apr 2025) → 16 (2025) | elements | e8-drone-kill-zone | defenseexpress-2025-kometa-m-16 |
| Lima: Shaheds jammed (maker claim) | 20,500+ | drones | e7 → e9 | kyivpost-2026-lima |
| GNSS-denied drift | ~2 | km per 100 km | e9-counteroffensive-26 | kyivpost-2026-lima |
| Interceptor kills: Oct 2024 → summer 2025 | several hundred → >1,000 | per month | e7 → e8 | kse-2025-drone-innovations |
| Frontline targets engaged by UAVs | 80–85 | % | e8-drone-kill-zone | kse-2025-drone-innovations |
| TFL-1 autonomous terminal leg | ~500 | m | e8-drone-kill-zone | kyivpost-2025-orlova-tfl1 |
| TFL-1 effectiveness gain (maker claim) | 2–4 | × | e8-drone-kill-zone | kyivpost-2025-orlova-tfl1 |
| Russian smuggled Starlink terminals (claim) | 50,000+ | terminals | e9-counteroffensive-26 | ukrinform-2026-liskovych-starlink-c2 |
| Territory retaken within 5 days of the Starlink cut (contested) | 200+ | km² | e9-counteroffensive-26 | atlanticcouncil-2026-spencer-starlink-crisis |
| Interceptor sorties and kills (Feb 2026) | ~6,300 sorties / >1,500 kills | count | e9-counteroffensive-26 | isis-2026-anokhin-shahed-monthly |
| Shahed kills by interceptors (24 May 2026) | >40 | % | e9-counteroffensive-26 | militarnyi-2026-pryhodko-interceptors |
| Shahed/Geran hit rate (May 2026) | ~6.7 | % | e9-counteroffensive-26 | isis-2026-anokhin-shahed-monthly |
| Interception rate: piston Shahed vs jet Geran | 90+ vs ~60 | % | e9-counteroffensive-26 | united24-2026-place-jet-gerans |
| Geran-4/5 production (HUR claim) | ~3,000 | per month | e9-counteroffensive-26 | united24-2026-place-jet-gerans |
| Jet share of Shahed-type launches (May 2026) | 0.5–1.5 | % | e9-counteroffensive-26 | isis-2026-anokhin-shahed-monthly |
| EW preparation before Vivaldi | 9 | months | e9-counteroffensive-26 | euobserver-2026-vasilko-vivaldi |

The figures below are contested; the table shows the ranges, and this is who claims what:
- **FPV success rates.** RUSI's 60–80% failure rate comes from field interviews; makers' 2–4× gains come from marketing.
- **Excalibur.** The 55% → 6% drop is the Washington Post's reading of internal Ukrainian data. Some secondary summaries say "70% → 6%", and we could not find that in RUSI's text.
- **Starlink-cut gains.** Reports range from 10–12 km (Wikipedia, southern front) to more than 200 km² (Atlantic Council). These use different units, depth of advance and area retaken, so they don't directly conflict. They still need a common measure, such as DeepStateMap's monthly km² statistics.
- **Interceptor shares.** Figures vary with scope: one night, the Kyiv region, or a month.

## 5. Russia's side as reported

**Systems.** Ukrainian and Western sources name:
- **Shipovnik-Aero:** low-signature, imitates other emitters, can down UAVs [src:rusi-2023-watling-meatgrinder].
- **Torn-MDM:** communications interception [src:rusi-2023-watling-meatgrinder].
- **R-330Zh Zhitel:** jams GPS, satcom and GSM out to about 30 km [src:wikipedia-zhitel; src:eurosd-2024-withington-excalibur].
- **Pole-21:** wide-area GNSS denial, now treated as expendable [src:rusi-2023-watling-stormbreak].
- **Platoon-level counter-UAS:** directional jammers and UAV-hijacking arrays [src:rusi-2023-watling-meatgrinder].

**Doctrine.** Russian EW teams are "weapons free", with little interest in deconfliction [src:rusi-2023-watling-meatgrinder]. After 2023, Russia shifted from big Soviet-style platforms to dispersed, disposable antennas because Ukraine targets emitters [src:rusi-2023-watling-stormbreak]. By 2025, navigational jamming was "ubiquitous" and most vehicles carried jammers [src:rusi-2025-watling-third-year].

**Adaptations Russia led:**
- **Fibre-optic FPVs.** KVN, developed through the Ushkuinik accelerator, and the Rubicon unit [src:kyivindependent-2025-farrell-fiber-optic].
- **Multi-element CRPA (Kometa-M)** on Shaheds, UMPK glide-bomb kits and Iskander-K [src:defenseexpress-2025-kometa-m-16].
- **Vehicle cages and "turtles"** [src:kyivpost-2024-turtle-tanks].
- **Road nets** [src:defenseexpress-2025-russian-net-tunnels].
- **Kh-101 improvements** for EW-resistant terminal navigation [src:rusi-2025-watling-third-year].
- **Frequency hopping on Molniya** [src:euromaidan-2026-mukhina-molniya].
- **Jet Gerans** [src:united24-2026-place-jet-gerans].

**Adaptations Russia followed.** Interceptor drones came about a year after Ukraine's [src:kse-2025-drone-innovations].

**Dependencies.** Starlink had become critical to Russian UAV and assault C2 before February 2026 [src:ukrinform-2026-liskovych-starlink-c2]. Russia's domestic alternatives are reported as inferior: Gazprom Space Systems and the Rassvet project [src:atlanticcouncil-2026-spencer-starlink-crisis; src:ukrinform-2026-liskovych-starlink-c2].

**Gaps in this file.** Krasukha, Tobol (anti-satellite-link jamming) and Russian trench jammers are not covered by a fetched source here. See section 8.

## 6. Game/sim relevance

- **Tech tree with obsolescence timers.** Each item has an effectiveness curve that decays once the enemy fields a counter. The lags from section 2 give plausible decay times:
  - Frequency moves: weeks.
  - Cages and antennas: 3–9 months.
  - Systemic counters (Starlink, GPS weapons): 6–24 months.
  - Excalibur's fall from 55% to 6% is a ready-made decay curve.
- **EW bubbles as map layers.** Model GNSS-denial fields (Pole-21, Zhitel, Lima) separately from control- and video-link jamming domes (trench, vehicle and backpack jammers). Each field has a band set. A drone with a matching band, or a fibre, autonomy or CRPA upgrade, ignores the corresponding bubble. Friendly jamming also blocks your own drones, so let players schedule "EW pauses" to launch strikes [src:rusi-2025-watling-third-year].
- **Emission signatures.** Every emitter (jammer, drone ground station, Starlink dish) adds to a detectable signature. Direction finding and EW-hunting FPVs let the enemy strike emitters, so "emit and die" is a trade-off rather than a free buff [src:militarnyi-2025-kushnikov-ew-hunter-fpv; src:rusi-2023-watling-stormbreak].
- **Comms backbone as a strategic toggle.** Starlink access is an event-driven, side-specific modifier. Its removal in February 2026 should crash the enemy's drone tempo and C2 for weeks while workarounds come online [src:ukrinform-2026-liskovych-starlink-c2].
- **Fibre vs nets vs autonomy** are three counters to EW, each with its own cost:
  - **Fibre:** limited range, snag risk, visible cable.
  - **Nets:** fixed infrastructure, which burns and can be bypassed.
  - **Autonomy:** needs machine-vision research, and the unit costs 10–20% more.
- **Strike-war economics.** Interceptor effectiveness against piston Shaheds should drop sharply when jet Gerans enter the enemy mix, until a jet-interceptor tech with machine-vision lock is researched [src:euromaidan-2026-mukhina-jet-interceptors].

## 7. Terms

- **REB** *(radioelektronna borotba)* — electronic warfare. **REBivets** — an EW operator.
- **okopnyi REB** — "trench EW": a small jammer protecting a dugout or position.
- **kupol** — "dome": a jammer's protective bubble over a position or vehicle.
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
- **CRPA** — controlled reception pattern antenna: a multi-element anti-jam GNSS antenna (Russian Kometa-M).
- **PRFH** — pseudo-random frequency hopping.
- **GNSS jamming / spoofing** — drowning out, or faking, satellite navigation signals.
- **INS** — inertial navigation, the fallback when GNSS is denied; it drifts over time.
- **last mile / terminal guidance** — machine-vision homing for the final few hundred metres after the RF link drops.
- **emit and die** — any detectable emission invites direction finding and a strike.
- **whitelist** — the Feb 2026 Starlink registry; unregistered terminals were cut off.
- **kill zone** *(zona urazhennia)* — the drone-dominated band behind the line of contact.

## 8. Open questions/gaps

- **Russian systems without a fetched source.** Krasukha-2/4, Tobol (Starlink-jamming attempts) and Russian trench-jammer families (e.g. "Piranha"-class) are not covered by a fetched source. Add from Defense Express, Militarnyi or RUSI.
- **Ukrainian systems.** Nota and the Pokrova GNSS spoofer are only named in passing. There is no fetched spec source for either.
- **Gun and passive defences.** Gepard, Skynex and thermal-masking cloaks have no dedicated source here. Their share of Shahed kills over time is a gap.
- **Digital links.** Ukrainian and Russian moves to digital, encrypted and mesh video/control links (e.g. Russian digital FPV links) are under-documented. So is the effect on analogue-only detectors such as Chuika.
- **Firmware cadence.** Update cadence for Kropyva, DELTA and drone firmware is not quantified. We have "monthly" and "weekly" only as quotes.
- **Current FPV loss rates.** There is no 2026 equivalent of RUSI's 60–80% failure figure. How much fibre and autonomy have changed it is unknown.
- **Starlink-cut effects.** Figures vary widely (10–12 km versus 200+ km²), and ISW/CTP pages were blocked from fetching (403). The ISW assessments of 8 and 26 Feb 2026 should be added manually.
- **Russian workarounds.** How far Russia has recovered C2 since the February 2026 cut (Rassvet, mesh, fibre) is unclear as of September 2026.
- **Blocked sources.** Ukrainska Pravda's August 2025 EW piece is search-only (403), and Forbes (Hambling) on new Ukrainian anti-Kometa jammers (Apr 2026) was blocked and not included.

## 9. Sources

- **rusi-2022-zabrodskyi-preliminary-lessons** — 2022 baseline: 90% of UAS lost; EW as the main counter-UAS tool.
- **rusi-2023-watling-meatgrinder** — Russian EW density, Shipovnik-Aero, 10,000 UAVs lost a month.
- **rusi-2023-watling-stormbreak** — "Binary" UAV use under EW; dispersed and disposable Russian EW; GPS-jamming tell.
- **rusi-2024-watling-offensive-lessons** — Hard counters to Excalibur and GMLRS; the need for field updates.
- **rusi-2025-watling-third-year** — 60–80% FPV failure; fibre; interceptors; EW pauses; Shahed defence.
- **kyivpost-2024-chiu-gps-weapons** — Excalibur 55% → 6%; JDAM-ER misses.
- **eurosd-2024-withington-excalibur** — Technical GPS-jamming explainer; M-code and home-on-jam.
- **wikipedia-zhitel** — Zhitel capabilities and Ukrainian kills (navigation only).
- **wikipedia-bukovel** — Ukrainian Bukovel-AD counter-UAS (navigation only).
- **wikipedia-starlink-war** — Starlink timeline from 2022 to 2026 (navigation only).
- **aljazeera-2024-gur-starlink** — GUR confirms "systemic" Russian Starlink use (Feb 2024).
- **kyivindependent-2026-myronyshena-starlink-whitelist** — The Feb 2026 whitelist resolution.
- **ukrinform-2026-liskovych-starlink-c2** — Collapse of Russian C2; speed cap; workarounds.
- **atlanticcouncil-2026-spencer-starlink-crisis** — Effects of the Starlink cut; 200+ km² in 5 days.
- **kyivindependent-2025-farrell-fiber-optic** — Russia's fibre lead and Ukraine's catch-up (May 2025).
- **atlanticcouncil-2026-sutea-fiber-optic** — Fibre retrospective: ranges and counters.
- **united24-2025-barkhush-fiber-counters** — Unit-level counters to fibre drones.
- **euromaidan-2025-murdoch-net-tunnels** — Zaporizhzhia road net tunnels.
- **defenseexpress-2025-russian-net-tunnels** — Russian net tunnels near Bakhmut.
- **kyivpost-2024-turtle-tanks** — Cope cage → turtle → super turtle chain.
- **defenseexpress-2025-kometa-m-16** — The Kometa-M CRPA ladder, up to 16 elements.
- **united24-2025-khomenko-kometa-umpk** — 12-channel Kometa on UMPK glide bombs.
- **kyivpost-2026-lima** — Lima GNSS jammer/spoofer claims.
- **euromaidan-2024-martyniuk-ew-backpacks** — Backpack and trench jammers; FPV band shift.
- **defensepost-2025-encarnacion-atlas** — Networked EW (Kvertus Atlas).
- **united24-2026-brizard-detectors** — Chuika and ZORKO detectors; monthly updates.
- **pravda-2025-fpv-ew-shield** — FPV frequency split and adaptation pace (search-only).
- **euromaidan-2026-mukhina-molniya** — Molniya frequency hopping defeats jamming.
- **kyivpost-2025-orlova-tfl1** — TFL-1 terminal autonomy in mass production.
- **militarnyi-2025-kushnikov-ew-hunter-fpv** — EW-hunting FPV ("emit and die").
- **kse-2025-drone-innovations** — Interceptor scaling, the adaptation loop, drone share of engagements.
- **militarnyi-2026-pryhodko-interceptors** — More than 40% of Shahed kills by interceptors (24 May 2026).
- **isis-2026-anokhin-shahed-monthly** — Monthly Shahed data; interceptors; CRPA; jet share.
- **united24-2026-place-jet-gerans** — Jet Geran speeds, production, interception drop.
- **euromaidan-2026-mukhina-jet-interceptors** — Why the first jet interceptors failed.
- **kyivindependent-2026-farrell-vivaldi** — Operation Vivaldi overview.
- **mwi-2026-rose-vivaldi** — Vivaldi as drone and EW dominance; Rubicon hit.
- **euobserver-2026-vasilko-vivaldi** — Nine-month EW preparation for Vivaldi.
- **cnas-2024-pettyjohn-evolution** — Early-2024 Western view: EW as the best counter-drone tool.
