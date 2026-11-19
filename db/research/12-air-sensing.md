<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# 12 — Sensing and reporting the air threat, 2022–2026

Scope: how Ukraine detects, reports, classifies and fuses long-range and reconnaissance drones over the rear and the cities: Shahed-136/Geran-2, Gerbera and Parodiya decoys, jet Geran-3/4/5, and Orlan/ZALA/Supercam scouts. It covers the citizen app ePPO, the acoustic networks (Sky Fortress, Zvook), radars, visual and thermal observation, RF sensing, how the picture is fused and passed on, the signatures of the drones, and the attacker's own sensing of the defence. The effectors and their kill shares are in the sibling draft `air-effectors.md`; frontline FPV sensing (RF detectors, Zvook NW0, counter-FPV radars) is in `frontline-counter-drone.md`. This draft repeats neither except where the sensor is the point.

Accessed 2026-11-18. English and Ukrainian-language sources, Ukrainian outlets paired with Western analysis. Russian claims appear only as Ukrainian or Western outlets report them, and are marked as claims. **Reported** means a figure a named party states (a count, a spec, a measured rate); **estimate** means a figure the source itself presents as approximate or derived. **Vendor claim** means a maker's own figure. Much of this field is secret: Ukrainian sources told the Telegraph in 2024 that the acoustic project was "too secret to discuss in detail" [src:telegraph-2024-barnes-phones-on-poles], so most numbers come from a small number of officials, vendors and one US general.

---

## 1. Summary

- **Radar cannot see low fliers, so Ukraine built people and microphones into its air picture.** Both ePPO and the acoustic networks were built in 2022 to fill radar's low-altitude blind zone, which the Earth's curvature and terrain create [src:zaxid-2022-eppo-launch][src:espreso-2022-zvook][src:united24-2025-brizard-sky-fortress]. A Ukrainian outlet puts a typical radar's range against a target at 100 m altitude at 40–50 km, depending on terrain [src:defenceexpress-2024-shcherbak-russian-acoustic].
- **ePPO turns a phone into a bearing sensor.** Launched on 13 October 2022 by the volunteer group Tekhnari, it sends the phone's geolocation and compass heading when the user points it along the target's flight and presses one button. Reports can also come from sound [src:zaxid-2022-eppo-launch][src:dou-2022-gurska-eppo-interview]. Users log in through Diia "so that enemies and fools don't interfere with gunners with false targets" [src:zaxid-2022-eppo-launch]. A report reaches air-defence maps in 2–7 seconds, according to the deputy defence minister [src:ukrinform-2024-eppo-80pct]. Reported scale grew from 600,000 downloads and 200,000 active users (Nov 2024) to more than 850,000 verified users and more than 4 million reports (2025–26, secondary) [src:babel-2024-perepechko-eppo-80pct][src:swj-2026-woods-citizen-sensors].
- **The acoustic net is thousands of cheap, passive nodes.** Sky Fortress counts rose in successive reports from about 6,000 (Feb 2024) to 8,000 (Mar 2024), 9,500 (Jul 2024) and 14,000 (Jul 2025), at about $400–1,000 a node [src:twz-2024-trevithick-networked-microphones][src:militarnyi-2024-kushnikov-hecker-acoustic][src:aviationweek-2024-osborne-low-cost-kills][src:united24-2025-brizard-sky-fortress]. The separate Zvook net uses fewer, longer-range mirror sensors on 10–12 m towers. It had 210 nodes covering about 5% of Ukraine in 2024, and it reports detection ranges of about 5 km for drones and 7 km for cruise missiles, with data in Delta within about 12 seconds (vendor claims) [src:telegraph-2024-barnes-phones-on-poles][src:thedefensenews-2026-kumar-zvook-batt26].
- **The famous result is one raid.** The 84-drone raid with 80 downed by fire teams using acoustic tracks, told by Gen. James Hecker in 2024, is repeated in every later source and is the only quantified field result [src:militarnyi-2024-kushnikov-hecker-acoustic][src:aviationweek-2024-osborne-low-cost-kills]. United24 adds that about 20% of targets entering Ukraine are tracked by acoustic data alone [src:united24-2025-brizard-sky-fortress].
- **Russia answered the sensors by flying high, low and fake.** From 2024–25, Shaheds cruised above 2 km, where Ihnat said they are "more visible to our radars" but out of gun reach. Others flew low (up to about 200 m) in radar blind spots, and decoys made up about 40% of waves [src:kyivindependent-2025-ihnat-high-altitude][src:gwara-2025-kulish-decoy-waves]. Decoys carry Luneburg lenses to look like Shaheds on radar, radars merge 3–4 close targets into one, and decoys that vanish are logged as "lost from location" [src:isis-2024-anokhin-decoy-drones][src:gwara-2025-kulish-decoy-waves][src:euromaidan-2025-zoria-shaheds-evade-kyiv].
- **The sensors are also targets.** Oryx has visually confirmed 152 Ukrainian radars and communications systems lost, including 32 36D6 (16 "Tin Shield" and 16 ST-68U), 30 P-18-family, 2 TRML-4D (IRIS-T), 2 AN/MPQ-53 (Patriot) and 2 RADA ieMHR [src:oryx-ukrainian-losses]. Decoys "identify positions" of air defence for later strikes, and from April 2026 guided Shaheds hunt the mobile fire groups that fire on them [src:gwara-2025-kulish-decoy-waves][src:euromaidan-2026-mukhina-guided-shahed-mfg].

---

## 2. ePPO: the crowd as a sensor

### 2.1 Origin and timeline

| date | event | source |
|---|---|---|
| early May 2022 | Andriy Kosiak proposes the idea; a prototype is written in days; the volunteer group Tekhnari (about 10 people) merges with IT WAR UA | [src:dou-2022-gurska-eppo-interview] |
| May–Sep 2022 | Coordinated with Operational Command South, its air-defence command and the Navy's air defence; APK tested by "several hundred" trusted people; beta about August | [src:dou-2022-gurska-eppo-interview] |
| 13 Oct 2022 | Public launch on Google Play (Android); iOS "soon" | [src:zaxid-2022-eppo-launch] |
| 22 Oct 2022 | First claimed success: a Kalibr missing from radar (terrain masking) shot down in the south from citizen reports, with "four minutes" of warning | [src:dou-2022-gurska-eppo-interview] |
| Oct 2022 | First week of Shahed waves: Suldin says that on one night 19 of 20 drones were shot down, and people who heard but did not see them helped "draw" their tracks | [src:dou-2022-gurska-eppo-interview] |
| Nov 2024 | 600,000+ downloads, about 200,000 active users; AI predicts trajectories; "It's coming at you" warnings to users | [src:ukrinform-2024-eppo-80pct][src:babel-2024-perepechko-eppo-80pct] |
| Dec 2025 | 4 million+ reports in total (secondary, citing the Snake Island Institute) | [src:swj-2026-woods-citizen-sensors] |
| 2026 | 850,000+ Diia-verified users (secondary) | [src:swj-2026-woods-citizen-sensors] |

### 2.2 How a report works

- **What the user does.** The user chooses the type (drone, missile, aircraft, helicopter), points the phone in the direction the target is moving and presses a large red button [src:zaxid-2022-eppo-launch][src:ukrinform-2024-eppo-80pct].
- **What the phone sends.** It sends the geolocation and the heading from the phone's compass [src:dou-2022-gurska-eppo-interview][src:kyivpost-2024-brown-smartphone-apps]. A developer said "Geolocation is the only data type the ePPO app asks from a person" [src:dou-2022-gurska-eppo-interview]. The direction is the target's flight direction, not a bearing *to* the target. The 2022 instructions say to point "in the direction the target is moving" [src:zaxid-2022-eppo-launch]; a 2024 account says users point the phone "at an approaching object" [src:kyivpost-2024-brown-smartphone-apps]. The two descriptions differ, and no source says which one the server assumes.
- **Sound as well as sight.** Drones were reported by people who heard but did not see them [src:dou-2022-gurska-eppo-interview]. A 2026 account calls sound-based reports especially useful for Shahed-type drones [src:swj-2026-woods-citizen-sensors].
- **Processing.** Reports are processed on a server and passed to the military, and the developers say geolocation is "not stored anywhere" [src:dou-2022-gurska-eppo-interview]. Several reports along one path gave a commander enough points to direct a unit [src:dou-2022-gurska-eppo-interview]. Reports are aggregated in real time to estimate direction and speed, and users along the predicted path get a warning about 10 minutes before overflight [src:swj-2026-woods-citizen-sensors]. Public outputs show only location and type; some data is restricted to military use [src:swj-2026-woods-citizen-sensors].
- **Who sees it, and how fast.** Reports appear "on the air defense maps of all officers in 2–7 seconds" (Deputy Defence Minister Chernohorenko, Nov 2024) [src:ukrinform-2024-eppo-80pct]. The developers explained the cost of delay in 2022: a missile at 800 km/h flies 200 m a second, so a minute's delay is kilometres of error [src:dou-2022-gurska-eppo-interview].
- **Its place in the system.** "ePPO is a supplementary detection system, not the primary one" [src:dou-2022-gurska-eppo-interview]. The developers valued rural reports most: ten users among fishermen and farmers on the approach routes were worth more than a thousand in a big city [src:dou-2022-gurska-eppo-interview].

### 2.3 Trust, false reports and attacks on the app

- **Identity as the filter.** Diia login was the stated defence against false reports from the enemy and pranksters [src:zaxid-2022-eppo-launch][src:dou-2022-gurska-eppo-interview]. State IT bodies reviewed the code [src:dou-2022-gurska-eppo-interview].
- **No public trust model.** No fetched source describes duplicate merging, per-user trust scores, outlier rejection or a measured false-report rate. A 2026 Western analysis only warns in general terms of "data poisoning" sending interceptors to the wrong place, and recommends outlier analytics [src:swj-2026-woods-citizen-sensors].
- **Russian pressure.** The only documented attacks are DDoS attacks on IT WAR's services, with two short outages; the source does not say ePPO was among them [src:dou-2022-gurska-eppo-interview]. No fetched source documents spoofed ePPO reports.
- **The 80% claim.** Chernohorenko said ePPO users help "close Ukrainian airspace by 80%". No method is given [src:babel-2024-perepechko-eppo-80pct][src:ukrinform-2024-eppo-80pct].
- **Russia copied it.** Russia's "Radar" app, developed by All-Russia People's Front activists and pushed to millions via the state portal on 20–21 September 2023, has a near-identical point-and-press interface. Its developers claimed 50,000+ downloads in a month and two drones downed over Bryansk Oblast (Russian claim, relayed from the Washington Post) [src:caliber-2023-russia-radar-app].

### 2.4 Other human reporting

- **Air wardens and Telegram.** Thousands of volunteer monitors, mostly retired military or local officials in villages, report aircraft seen or heard with a direction, by phone, text or to the nearest unit. By early 2024 national and local Telegram channels were the main real-time source for both the military and the public (Kyiv Post analysis) [src:kyivpost-2025-korshak-how-ukraine-shoots-shaheds].
- **Long warning from launch.** Early warning often comes hours before a strike, when bombers fly to launch areas, and Russian crews have been seen loading launch racks in Crimea and southern Russia [src:kyivpost-2025-korshak-how-ukraine-shoots-shaheds]. Ballistic warning is short: on 2 July 2026 "two missiles from Kursk Oblast" (01:58) was followed by "approaching Kyiv" two minutes later [src:uaf-2026-kpszsu-alerts-1-2-july].
- **The gunner's ear.** In 2026 mobile fire groups still learned of drones mainly by hearing them; warnings often came "when we already hear it" [src:frontliner-2026-derkachov-mfg-modernise]. An Igla operator says propeller Shaheds give warning by engine noise, while jet drones and cruise missiles can be almost overhead before they are heard [src:kyivindependent-2026-moroziuk-jet-drone-crews].

---

## 3. Acoustic networks

### 3.1 Two families

| | Sky Fortress (Skyfortress) | Zvook |
|---|---|---|
| builder | NGO of Ukrainian engineers, anonymous; in the Brave1 pool; Ajax Systems named for a 12,500-sensor fundraising tranche | Lviv volunteers; founders include Respeecher co-founder Dmytro Bielievtsov, i3 Engineering CEO Pavlo Tsiupka and serviceman Marian Sulym |
| start | 2022, against low cruise missiles | 2022; 40+ stations by Dec 2022 |
| node | first generation: an Android phone in a box on a pole about 6 ft (1.8 m) high; later custom CPU and sound card | microphone at the focus of a parabolic acoustic mirror, on telecom towers at 10–12 m |
| count | about 6,000 (Feb 2024) → 8,000 (Mar 2024) → 9,500 (Jul 2024) → 14,000 (Jul 2025); 15,000 third-generation nodes funded by NATO allies | 40+ (Dec 2022) → 210 (Mar 2024), "nearly 300" later (search snippet only) |
| node cost | about $500; $400–500; $400–1,000 | about £400 to make; about $500 |
| network cost | "less than two Patriot missiles" (Hecker); $5–54m (analyst range) | covers Ukraine at 1,000 m and below "for under €50 million" (company) |
| coverage | nationwide, no area figure | about 5% of Ukraine (2024), border entry routes; 8,000 nodes would cover the country (company) |
| range | not published | drones about 5 km, cruise missiles about 6–7 km, ballistic about 10 km (company) |
| output | flight routes to fire teams on iPads | location, direction, speed, altitude and class into Delta, within about 12 s (company) |
| sources | [src:united24-2025-brizard-sky-fortress][src:telegraph-2024-barnes-phones-on-poles][src:twz-2024-trevithick-networked-microphones][src:militarnyi-2024-kushnikov-hecker-acoustic][src:aviationweek-2024-osborne-low-cost-kills][src:kyivpost-2025-korshak-how-ukraine-shoots-shaheds] | [src:algorithmwatch-2022-andriushchenko-zvook][src:espreso-2022-zvook][src:telegraph-2024-barnes-phones-on-poles][src:kyivpost-2024-brown-smartphone-apps][src:thedefensenews-2026-kumar-zvook-batt26][src:euromaidan-2026-mukhina-zvook-hydrophones] |

**Where sources disagree.** The Sky Fortress counts rise over time and may also mix systems; the Kyiv Post calls the network "Sky Watch" and gives 10,000–15,000 as an analysts' estimate [src:kyivpost-2025-korshak-how-ukraine-shoots-shaheds]. Zvook's drone range is "about 3 miles" (about 5 km) in the Telegraph and 5 km in a later trade report, and its cruise-missile range is 4 miles (about 6.4 km) against 7 km [src:telegraph-2024-barnes-phones-on-poles][src:thedefensenews-2026-kumar-zvook-batt26]. The 12 seconds is described both as delivery time into Delta and as "warning time", which are different measures, and it is a company figure.

### 3.2 How it works

- **Location from loudness and geometry.** Computers compare the relative strength of the engine sound across nodes and triangulate the drone's position and heading [src:kyivpost-2025-korshak-how-ukraine-shoots-shaheds]. Combining several stations gives speed and direction [src:militarnyi-2024-kushnikov-hecker-acoustic][src:algorithmwatch-2022-andriushchenko-zvook]. A Ukrainian defence outlet judges acoustic accuracy "significantly lower than radar", but says it gives a dense field at low altitude where radar is weak [src:defenceexpress-2024-shcherbak-russian-acoustic].
- **Always on.** Phones are always recording and send over local mobile networks to a central system [src:telegraph-2024-barnes-phones-on-poles].
- **Classification by machine learning.** Zvook classes targets as helicopter, jet aircraft, cruise missile or drone [src:espreso-2022-zvook]. It reached about 50% accuracy in early tests. A cow's moo was once taken for a missile engine (the "cow problem"), and real missiles were sometimes classed as harmless, which the team says it fixed [src:algorithmwatch-2022-andriushchenko-zvook]. Missile sounds made up about 0.1% of the training data, and a distant and a close missile sound very different (the "domain shift") [src:algorithmwatch-2022-andriushchenko-zvook]. By 2026 the company claimed a false-positive rate of about 1.6% [src:thedefensenews-2026-kumar-zvook-batt26].
- **Spoofing the sound.** Russian attempts to change drone noise signatures cut Sky Fortress accuracy by only 3% [src:united24-2025-brizard-sky-fortress].
- **Passive.** The nodes emit nothing, so they do not give away their position [src:thedefensenews-2026-kumar-zvook-batt26].
- **To the guns.** Acoustic data is fused with radar data and sent to mobile firing teams with tablets and anti-aircraft guns [src:united24-2025-brizard-sky-fortress]. Hecker: "In six hours, they teach a fighter to look at a tablet that displays the direction of the drones' movement" [src:militarnyi-2024-kushnikov-hecker-acoustic]. Zvook's CEO: "It's an auxiliary system. It's not replacing radar but making it more efficient" [src:telegraph-2024-barnes-phones-on-poles].
- **The national frame.** The Telegraph reports that the national air-defence network "Virazh" combines at least 40 kinds of sensor network [src:telegraph-2024-barnes-phones-on-poles].
- **Russia copies it.** In September 2024 Beskrestnov photographed a Russian four-microphone sensor with a processing and transmission unit on a mobile-phone tower, sending by mobile network or radio [src:defenceexpress-2024-shcherbak-russian-acoustic].

### 3.3 Results and limits

| measure | value | kind | source |
|---|---|---|---|
| One raid, acoustically tracked | 84 drones in, all tracked, 80 downed by fire teams | reported by Hecker (US) | [src:militarnyi-2024-kushnikov-hecker-acoustic][src:aviationweek-2024-osborne-low-cost-kills] |
| Share of targets tracked by acoustics alone | about 20% | reported (United24, unattributed) | [src:united24-2025-brizard-sky-fortress] |
| Accuracy loss from Russian noise changes | 3% | reported | [src:united24-2025-brizard-sky-fortress] |
| Zvook false positives | about 1.6% | vendor claim | [src:thedefensenews-2026-kumar-zvook-batt26] |
| Zvook data into Delta | about 12 s | vendor claim | [src:thedefensenews-2026-kumar-zvook-batt26] |

No fetched source measures the effect of wind, rain or traffic noise, bearing or position accuracy, or performance against jet Gerans. A NATO lessons-learned note (written with AI assistance, citing only press reports) lists vulnerability "to physical decoys and cyberattacks" as an open challenge [src:nato-2026-nsatu-acoustic-lessons].

---

## 4. Radars

### 4.1 What is in service against drones

| radar | role and figures | kind | source |
|---|---|---|---|
| Soviet-era surveillance radars (36D6, P-18 family, P-19, 5N63 etc.) | the national net; many lost, see §8 | Oryx loss list | [src:oryx-ukrainian-losses] |
| Saab Giraffe family (PS-90, PS-100, Giraffe 1X, AMB, Mk IV) | 360° sweep; up to 100 km and 20 km altitude depending on variant; detects "small, slow, low-altitude drones"; Giraffe 1X is pickup-mounted. Which variants and how many are in Ukraine is not published | reported (MoD, Jun 2026) | [src:united24-2026-kohanets-giraffe] |
| Gepard search and tracking radars | original search radar up to 15 km, the S-band MPDR-18 upgrade up to 18 km (the source's naming of the MPDR-12 and Albis radars is muddled); 40 Gepards by mid-2023. Shaheds "a very easy target" | reported | [src:militarnyi-2023-safronov-gepard] |
| Skynex | radar-guided 35 mm battery (radar type not in fetched sources); misled by Parodiya decoys that look like large aircraft | reported | [src:euromaidan-2025-zoria-shaheds-evade-kyiv][src:militarnyi-2025-shumlianskyi-skynex-shaheds] |
| RADA ieMHR (Israel) | about $1m each against about $120,000 for Ukrainian analogues (an MFG coordinator's estimate); 2 lost (Oryx) | estimate; Oryx | [src:kyivindependent-2025-post-autumn-rains][src:oryx-ukrainian-losses] |
| Robin Radar IRIS (Netherlands) | compact counter-drone radar, vehicle-mountable; a software "Long-Range Mode" tested in Ukraine raised detection from 5 to 12 km for drones above 180 km/h (Sep 2025) | vendor claim | [src:united24-2025-litnarovych-iris-range] |
| CD-T10 (Contra-Drone, Ukraine) | X-band; 15 km, 3 km altitude; Shaheds, FPVs and guided bombs; AI filters birds by radar cross-section, speed and movement; in test with mobile fire groups; deliveries end of 2026 | vendor claim | [src:euromaidan-2026-zoria-cd-t10] |
| Proposed 30 kg vehicle radar | giving each MFG coordinates, speed and altitude in real time; part of the January 2026 "anti-drone dome" of classic radars, compact radars and acoustic sensors | proposal (aviation expert Romanenko) | [src:frontliner-2026-derkachov-mfg-modernise] |

### 4.2 Limits

- **Low fliers.** Ground radar gives poor location data for Shaheds below the horizon. A 477 km/h jet Shahed was picked up at 300 m, and some glide at 100 m [src:kyivindependent-2025-post-shaheds-evolving]. A commander: flights "at very low altitudes, up to 200 meters ... are difficult to detect by radar due to blind spots" [src:gwara-2025-kulish-decoy-waves].
- **High fliers are easier.** Above 2 km, Shaheds "become more visible to our radars" (Ihnat) [src:kyivindependent-2025-ihnat-high-altitude].
- **Close targets merge.** "Radars cannot clearly distinguish between 3–4 simultaneous targets, showing them as one" [src:gwara-2025-kulish-decoy-waves].
- **Not enough radars.** Interceptor units say "We just need better radar", and that without durable, EW-protected sensors "it's very difficult" [src:c4isrnet-2026-livingstone-interceptor-economics]. A 2025 Kyiv mobile-defence commander wanted more over-the-horizon radar [src:kyivindependent-2025-post-shaheds-evolving].
- **Siting changed in 2026.** Before, units decided their own radar placement. From spring 2026 radars are placed on a coordinated plan built from past attack patterns, moved after strikes, and their mistakes analysed (Deputy Air Force Commander Pavlo Yelizarov) [src:euromaidan-2026-mukhina-airforce-tracking].

---

## 5. Eyes, thermal and IR, and RF

- **Thermal sights and searchlights.** MFG thermal imaging works at night "when spotlights cannot", and is linked to the crews' shared tablet application [src:kyivindependent-2025-post-shaheds-evolving]. Cloud cover hides drones from thermal sights [src:kyivindependent-2026-moroziuk-jet-drone-crews].
- **Weather.** A scope that sees about 2,000 m in clear weather sees roughly 100–200 m in rain and snow (Kyiv Oblast MFG coordinator). Low cloud, rain, snow and fog defeat visual acquisition even with thermal cameras (aviation expert Khrapchynskyi). Zelensky put the fall in air-defence effectiveness in overcast weather at "20–30%" (October 2025) [src:kyivindependent-2025-post-autumn-rains]. "Shaheds fly in any weather" [src:kyivindependent-2025-post-autumn-rains].
- **Helicopter FLIR.** Helicopters carry 30 kg FLIR turrets for round-the-clock search. Attack corridors 60–70 km wide are too broad for the few helicopters [src:euromaidan-2025-zoria-shaheds-evade-kyiv].
- **Aerostats.** In March 2025 the start-up Aerobavovna showed a tethered aerostat with an IR sensor under it that drops an interceptor when it sees a Shahed. The source is social-media images; no ranges are published [src:euromaidan-2025-martyniuk-aerostat-ir].
- **Cameras on buildings.** No fetched source documents a city network of rooftop thermal or IR cameras for Shahed detection. See §11.
- **The crew's own correction.** An MFG receives direction, altitude and distance, then finds the drone with its own observation kit and corrects for the difference between the reported and the real position. It fires only within its sector [src:kyivindependent-2026-moroziuk-jet-drone-crews].
- **RF.** A brigade EW chief watches Russian drone links on a spectrum analyser: "I can see the flow of data in real time" [src:kyivindependent-2026-moroziuk-jet-drone-crews]. Newer Gerans carry cameras and mesh radios [src:kyivindependent-2026-moroziuk-jet-drone-crews][src:euromaidan-2026-vivdych-xk-f358], and almost all Shahed-types aimed at Kyiv on 1–2 July 2026 used a new 3,900–4,100 MHz band (Beskrestnov) [src:isw-2026-07-02-roca]. A Gerbera without a modem or camera emits nothing to find this way [src:pravda-2026-konyk-gerbera-fuel-tanks]. No fetched source describes RF direction-finding used to *track* Shaheds for the air picture.

---

## 6. Fusing and distributing the picture

- **The chain, as crews describe it.** Radars and other sensors are the first link; data goes through command posts to the units. Each unit reports to its own combat command post, which merges information from higher command and neighbouring groups and gives orders [src:kyivindependent-2026-moroziuk-jet-drone-crews].
- **Apps.** Units share a radar-based mobile app (the Kyiv Independent spells it "Visage"; probably Virazh, see §11) that tracks incoming attacks and is integrated with Delta [src:kyivindependent-2025-post-shaheds-evolving]. Delta itself is Ukraine's cloud situational-awareness system, adopted across the forces in August 2024 [src:csis-2024-bondar-delta-cjadc2].
- **Cueing interceptors.** Most interceptions start from radar tracks, acoustic spotters and stitched feeds from Mission Control. Networked systems hand a track between crews as the target crosses sector boundaries [src:c4isrnet-2026-livingstone-interceptor-economics].
- **Latencies that are reported.** ePPO report to map: 2–7 s (official) [src:ukrinform-2024-eppo-80pct]. Zvook to Delta: about 12 s (vendor) [src:thedefensenews-2026-kumar-zvook-batt26]. A gunner's engagement window at low altitude: "five or six seconds" [src:kyivindependent-2025-post-shaheds-evolving]; targets cross an Igla operator's sector in as little as 20 seconds [src:kyivindependent-2026-moroziuk-jet-drone-crews]. No end-to-end figure (first detection to gun) was found.
- **Counting drones across regions.** In spring 2026 the Air Force began tracking every drone's route across regions. Before, drones that hit nothing in a region could "drop out" of its statistics, and regions had an incentive not to own drones that passed through [src:euromaidan-2026-mukhina-airforce-tracking].
- **Saturation.** One commander says Ukrainian SAM systems track about 10–20 targets at once [src:euromaidan-2025-shandra-shaheds-dive]. Russia mixes drone raids with feints such as MiG take-offs, so the picture must be coordinated nationally [src:kyivindependent-2025-post-shaheds-evolving].

---

## 7. Signatures, and telling decoys apart

### 7.1 The targets

| type | speed | altitude | sound / heat / RF | radar | sources |
|---|---|---|---|---|---|
| Shahed-136 / Geran-2 (piston) | about 200 km/h (2024); 200–210 km/h (2025) | low (100–200 m) at first; above 2 km from 2024–25, cruising 2,500–2,800 m, then a near-vertical dive from 2,000–2,500 m | two-stroke piston engine (MD550 replica) with a "lawnmower"/moped buzz; some carry cameras, Starlink or mesh modems | small; better seen high than low | [src:kyivindependent-2025-post-shaheds-evolving][src:euromaidan-2025-shandra-shaheds-dive][src:kyivpost-2025-korshak-how-ukraine-shoots-shaheds][src:kyivindependent-2025-ihnat-high-altitude] |
| Gerbera (decoy, scout, light strike) | 160 km/h | holds a stable altitude when it has no modem or camera | DLE60 60 cc two-stroke (Chinese hobby engine); some with a Topotek HD camera; two upward/rearward cameras seen in 2025 | Luneburg lens makes it look like a Shahed | [src:pravda-2026-konyk-gerbera-fuel-tanks][src:isis-2024-anokhin-decoy-drones][src:bi-2025-loh-gerbera-cameras][src:gwara-2025-kulish-decoy-waves] |
| Parodiya (decoy) | similar to a Shahed | n/s | DLE55RA 55 cc two-stroke | Luneburg lenses give it a Shahed-like radar cross-section; some devices mimic a large aircraft such as an Il-76 | [src:isis-2024-anokhin-decoy-drones][src:euromaidan-2025-zoria-shaheds-evade-kyiv][src:gwara-2025-kulish-decoy-waves] |
| Geran-3 (jet) | 380–400 km/h typical, record 477 km/h (Mar 2025); about 300 mph (480 km/h) | ceiling about 2 km (2025) | jet engine; later cameras and mesh | n/s | [src:kyivindependent-2025-post-shaheds-evolving][src:csis-2026-bondar-shahed-geran] |
| Geran-4/5 (jet) | 350–500 km/h; 300–600 km/h | some at 5–6 km | heard late; "I couldn't even see it with my eyes" | n/s | [src:militarnyi-2026-jet-geran-interceptors][src:united24-2026-place-jet-gerans][src:kyivindependent-2026-moroziuk-jet-drone-crews] |

No fetched qualifying source gives a radar cross-section number for any of these types, or a measured IR signature. The Gerbera's foam-over-plywood build [src:isis-2024-anokhin-decoy-drones] suggests a weak bare radar return, which is why the lens is fitted; that is an inference, not a source.

### 7.2 How decoys are told apart, and how often they are not

- **Speed.** A Gerbera at 160 km/h [src:pravda-2026-konyk-gerbera-fuel-tanks] is slower than a 2025 Shahed at 200 km/h or more [src:euromaidan-2025-shandra-shaheds-dive]. Parodiya flies at a Shahed-like speed and looks like one on radar [src:gwara-2025-kulish-decoy-waves].
- **Behaviour.** Decoys can circle over a city and go back to Russia or fly on [src:gwara-2025-kulish-decoy-waves]. Gerberas fitted with extra tanks have flown over Ukraine and back [src:pravda-2026-konyk-gerbera-fuel-tanks].
- **By what happens next.** The Air Force logs decoys that disappear without effect as "lost from location". On 6–7 May 2025, 64 of 187 drones were logged that way, against 81 Shaheds shot down [src:euromaidan-2025-zoria-shaheds-evade-kyiv]. In early 2025 about half of radar-detected Shahed-types were decoys, many logged as "Lost/Fell" [src:kyivindependent-2025-post-shaheds-evolving].
- **Not in time.** Defenders cannot be sure a drone has no warhead, so they spend interceptors on decoys [src:isis-2024-anokhin-decoy-drones]. Gerberas now sometimes carry about 10 kg of explosive [src:isis-2024-anokhin-decoy-drones].
- **Share of the wave.** About 40% decoys in May 2025 [src:kyivindependent-2025-ihnat-high-altitude]; 35–40% in 2026 [src:frontliner-2026-zvirynskyi-strike-intensity]; "roughly one-third" (early 2026) [src:c4isrnet-2026-livingstone-interceptor-economics].

### 7.3 Scouts

Orlan, ZALA and Supercam are the frontline sibling draft's subject. For the air picture: a Supercam has been found carrying a radio-signal detector that makes it evade, often by diving, when it senses a transmission (Beskrestnov, Sept 2025) [src:bi-2025-loh-gerbera-cameras]. Shaheds also fly faster and lower than Russian observation drones, which helps tell them apart on a track [src:kyivpost-2025-korshak-how-ukraine-shoots-shaheds].

---

## 8. The attacker's sensing: finding and hitting the defence

- **Waves as reconnaissance.** In 2023 Tu-95 packages used Shaheds as pathfinders to find gaps in Ukrainian air defence [src:rusi-2023-watling-meatgrinder]. Waves of 5–10 Shaheds come in from one sector at different altitudes. A Ukrainian commander says "There are only two goals: overload the Ukrainian air defense system and identify positions", and decoys map radar coverage by altitude [src:gwara-2025-kulish-decoy-waves]. Decoys reveal air-defence, radar and EW positions for later strikes [src:isis-2024-anokhin-decoy-drones]. An air-defence officer, 2026: "They're probing, feeling things out" [src:kyivindependent-2026-moroziuk-jet-drone-crews].
- **Cameras and online control.** In 2025 a Kyiv MFG commander said some Shaheds "react to light": they manoeuvre when the searchlights come on. Some are "steered online", and they "film our positions ... so later they try to fly around us" [src:kyivindependent-2025-post-shaheds-evolving].
- **Hunting the gun crews.** In April 2026 Beskrestnov reported "another hit by a guided Shahed on a mobile fire group crew. The group was working on one drone, while another at that moment was hunting the crew" [src:euromaidan-2026-mukhina-guided-shahed-mfg].
- **Lancets on radars.** A Lancet hit the TRML-4D radar of an IRIS-T SLM in a video of June 2023. The damage is unclear, but the truck stayed immobile afterwards, and "radars are notoriously fragile" [src:twz-2023-trevithick-iris-t-lancet].
- **Ballistic missiles on forward air defence.** When Ukraine pushed air defence forward, Russia found it and struck it, usually with 9M723 Iskanders [src:rusi-2025-watling-third-year].
- **Documented losses (Oryx, visually confirmed, as fetched 18 Nov 2026).** Radars and communications: 152 (113 destroyed, 24 damaged, 1 abandoned, 14 captured). They include 16 36D6 "Tin Shield", 16 36D6 ST-68U, 14 P-18 "Malakhit", 12 P-18, 4 P-18ML, 16 5N63S (S-300PS), 2 TRML-4D, 2 AN/MPQ-53, 1 AN/MPQ-64 Sentinel, 2 RADA ieMHR and 1 MR-18. Self-propelled AA guns: 51, including 1 Gepard. Oryx does not attribute causes in this list [src:oryx-ukrainian-losses].

---

## 9. How it changed, era by era

| era | threat | sensing | sources |
|---|---|---|---|
| e3-counteroffensives-22 (autumn 2022) | first Shahed waves, low, at night | radars with blind zones; ePPO launched 13 Oct; Zvook's first 40 stations; MFGs listening | [src:zaxid-2022-eppo-launch][src:espreso-2022-zvook][src:dou-2022-gurska-eppo-interview] |
| e4–e6 (2023–mid 2024) | waves grow; Shahed pathfinders for missile packages; Lancets on radars | acoustic net to 6,000–9,500 nodes, tablets for fire teams; Gepard radars | [src:rusi-2023-watling-meatgrinder][src:twz-2023-trevithick-iris-t-lancet][src:aviationweek-2024-osborne-low-cost-kills][src:militarnyi-2023-safronov-gepard] |
| e7-kursk-pokrovsk (late 2024–Mar 2025) | Gerbera and Parodiya decoys with Luneburg lenses (from Jul 2024); jet Geran-3 | radar must judge Shahed or decoy; ePPO 200,000 active users | [src:isis-2024-anokhin-decoy-drones][src:kyivindependent-2025-post-shaheds-evolving][src:ukrinform-2024-eppo-80pct] |
| e8-drone-kill-zone (2025) | Shaheds above 2 km and in low blind spots; camera Shaheds that avoid searchlights; ~40% decoys; weather exploited | Sky Fortress 14,000 nodes; IRIS long-range mode; Delta-linked air-picture app | [src:kyivindependent-2025-ihnat-high-altitude][src:united24-2025-brizard-sky-fortress][src:united24-2025-litnarovych-iris-range][src:kyivindependent-2025-post-autumn-rains] |
| e9-counteroffensive-26 (2026) | guided Shaheds hunting MFGs; jet Gerans at 5–6 km, heard late; mesh and new bands | coordinated radar siting; route tracking across regions; CD-T10 with MFGs; "anti-drone dome"; Giraffe | [src:euromaidan-2026-mukhina-guided-shahed-mfg][src:euromaidan-2026-mukhina-airforce-tracking][src:euromaidan-2026-zoria-cd-t10][src:united24-2026-kohanets-giraffe][src:kyivindependent-2026-moroziuk-jet-drone-crews] |

---

## 10. Game/sim relevance

Design observations, not facts. Each rests on the cited facts above.

- **ePPO is a direction-of-flight report, not just a bearing.** The 2022 instructions say to point the phone where the target is *going* [src:zaxid-2022-eppo-launch]. Model a phone report as the phone's position plus a heading cone and a type. The type comes from a menu and can be wrong. Some users will point *at* the drone instead [src:kyivpost-2024-brown-smartphone-apps], which gives a second, systematically different reading of the same report. That is a good source of honest noise.
- **Latency tiers.** ePPO report to map about 2–7 s; acoustic into Delta about 12 s; a gunner's window 5–20 s at low altitude [src:ukrinform-2024-eppo-80pct][src:thedefensenews-2026-kumar-zvook-batt26][src:kyivindependent-2025-post-shaheds-evolving][src:kyivindependent-2026-moroziuk-jet-drone-crews]. At 55 m/s (200 km/h), 7 s is about 400 m of drift; at 130 m/s (a jet Geran) it is about 900 m. So a fresh phone report already sits at the edge of the concept's "track" tier, and a jet report is only a "cue".
- **Ranges to tune from.** Acoustic node: 5 km for a Shahed (Zvook mirror on a 10–12 m mast, vendor claim) [src:thedefensenews-2026-kumar-zvook-batt26]. The pole phone has no published range, so a shorter guess is reasonable. Optical sight: about 2 km clear, 100–200 m in rain or snow [src:kyivindependent-2025-post-autumn-rains]. Small counter-drone radar: 5–15 km (vendor) [src:united24-2025-litnarovych-iris-range][src:euromaidan-2026-zoria-cd-t10]. Gepard search radar: 15–18 km [src:militarnyi-2023-safronov-gepard].
- **Altitude is a sensor trade.** Low Shaheds hide from radar but are heard and seen; high ones are seen by radar but are faint to the ear and out of gun reach [src:kyivindependent-2025-ihnat-high-altitude][src:gwara-2025-kulish-decoy-waves]. The attacker director can choose the band that the player's current sensor mix covers worst.
- **Decoys are a classification problem with a delayed answer.** On radar, Gerbera, Parodiya and Shahed should look alike (the lens); speed separates the Gerbera, not the Parodiya; sound separates nothing reliably (all are small two-strokes). The truth comes later, when the contact is "lost from location" with no impact [src:euromaidan-2025-zoria-shaheds-evade-kyiv]. That fits the debrief's two-worlds comparison exactly.
- **Radar merges.** 3–4 targets close together show as one plot [src:gwara-2025-kulish-decoy-waves]. Groups of 5–10 from one sector should sometimes fuse into fewer contacts.
- **Acoustic classifier errors.** A "cow problem" type of false positive, and a range-dependent sound (domain shift), are documented failure modes [src:algorithmwatch-2022-andriushchenko-zvook]. A tuned false-positive rate of about 1.6% is the vendor's number [src:thedefensenews-2026-kumar-zvook-batt26].
- **Jets break the ear.** Jet drones and cruise missiles can be overhead before they are heard [src:kyivindependent-2026-moroziuk-jet-drone-crews]. In the 2026 scenario, acoustic and human-ear observers should give little or late evidence on jets; radar and thermal carry them.
- **Shooting reveals you.** Firing, or switching on a searchlight, should raise the chance that a camera Shahed records the position, and later that a guided one comes for the crew [src:kyivindependent-2025-post-shaheds-evolving][src:euromaidan-2026-mukhina-guided-shahed-mfg]. Radars should be relocated after a raid, as the Air Force now does [src:euromaidan-2026-mukhina-airforce-tracking].
- **Region-boundary bookkeeping is historical.** The pre-2026 incentive to let drones "drop out" of a region's count [src:euromaidan-2026-mukhina-airforce-tracking] could be a debrief note: belief versus truth, even in the defender's statistics.

---

## 11. Open questions/gaps

- **ePPO internals.** No source describes how duplicates are merged, whether users carry trust scores, the measured false-report rate, the accuracy of phone compass headings in practice, or documented Russian spoofing of ePPO. The 850,000 users and 4 million reports figures come only from a secondary Western article citing the Snake Island Institute, which was not fetched.
- **Acoustic accuracy.** No bearing accuracy, position error, detection range for the pole-phone nodes, or effect of wind, rain and urban echo was found. The Sky Fortress counts (6,000 to 14,000) may mix systems and dates. Zvook's "nearly 300 systems" appears only in a search snippet (bonedo.de) and is not used.
- **Radar inventory.** Which Giraffe variants, how many Ground Master 200, RADA, IRIS and Ukrainian radars serve in the drone role, and their Shahed-specific ranges, are not public. The Kyiv Post's GM200 and Control Master 200 articles were seen only as search snippets.
- **The air-picture system's name.** The Kyiv Independent's "Visage" is probably Virazh (the Telegraph's national network, which combines at least 40 sensor types) or the Virazh-Planshet tablet system, which appears in a search snippet from Militarnyi. Neither was confirmed from a fetched page.
- **End-to-end latency.** No figure from first detection to gun or interceptor exists, only per-link numbers.
- **Radar cross-section and IR.** No qualifying source gives RCS values for the Shahed, Gerbera or jet Geran (0.01 m² for the Shahed appears only on non-qualifying sites), or any IR signature data.
- **City camera networks.** No fetched source documents rooftop thermal or IR camera networks for drone detection in Kyiv or elsewhere.
- **Causes of radar losses.** Oryx lists 152 radars and communications systems lost but not what killed each one. The Lancet strikes on P-18, 35D6M and an Israeli radar seen in search results come from Russian channels or non-qualifying sites.
- **RF tracking of Shaheds.** Spectrum monitoring of Geran links is described, but no source says RF direction-finding feeds the national air picture.
- **Pravda.** Ukrainska Pravda's pages returned HTTP 403 for this session, so its 2025 MFG-altitude article and 2026 interceptor explainer were seen only as search snippets and are not cited.

---

## 12. Sources

New (in `db/staging/sources/12-air-sensing.yaml`):
- **dou-2022-gurska-eppo-interview** — DOU interview with ePPO's developers: timeline, what a report sends, Diia, first successes, "supplementary system".
- **zaxid-2022-eppo-launch** — ePPO launch on 13 Oct 2022; point along the flight path; Diia against false targets.
- **ukrinform-2024-eppo-80pct** — Chernohorenko: 600,000 downloads, 200,000 active, 2–7 s to maps, AI trajectories.
- **babel-2024-perepechko-eppo-80pct** — The same announcement; the 80% claim.
- **kyivpost-2024-brown-smartphone-apps** — ePPO GPS and compass; Virazh; Zvook ranges and 12,000 plan.
- **swj-2026-woods-citizen-sensors** — 850,000 users, 4 million reports (secondary); 10-minute warning; data poisoning.
- **caliber-2023-russia-radar-app** — Russia's "Radar" copy of ePPO (relays the Washington Post).
- **twz-2024-trevithick-networked-microphones** — Hecker: about 6,000 acoustic sensors; the picture to fire teams.
- **militarnyi-2024-kushnikov-hecker-acoustic** — Hecker: 8,000 sensors, $500 each, tablets, 84/80.
- **telegraph-2024-barnes-phones-on-poles** — Sky Fortress phones on poles; Virazh 40 networks; Zvook 210 nodes, ranges, £400.
- **aviationweek-2024-osborne-low-cost-kills** — 9,500 sensors, $400–500, iPads, the 84/80 raid.
- **united24-2025-brizard-sky-fortress** — 14,000 sensors, 15,000 third generation, $400–1,000, 20% acoustic-only, fusion with radar.
- **algorithmwatch-2022-andriushchenko-zvook** — Zvook's founders, mirrors, training data, the cow problem.
- **espreso-2022-zvook** — Zvook's 40+ stations, 10–12 m masts, four target classes.
- **thedefensenews-2026-kumar-zvook-batt26** — Zvook's 5/7 km ranges, about 12 s into Delta, 1.6% false positives (vendor).
- **euromaidan-2026-mukhina-zvook-hydrophones** — Zvook covers Ukraine at 1,000 m and below for under €50m.
- **defenceexpress-2024-shcherbak-russian-acoustic** — Russia copies the acoustic net; acoustic vs radar accuracy; radar range at 100 m altitude.
- **nato-2026-nsatu-acoustic-lessons** — NATO lessons note (AI-assisted compilation of press reports).
- **kyivpost-2025-korshak-how-ukraine-shoots-shaheds** — Sky Watch triangulation; air wardens; Telegram; early warning from launch.
- **kyivindependent-2025-post-shaheds-evolving** — Jet speeds; radar low-altitude limits; Delta-linked app; tablets and thermal; camera Shaheds.
- **kyivindependent-2025-post-autumn-rains** — Weather effects on optics and thermal; RADA cost.
- **kyivindependent-2026-moroziuk-jet-drone-crews** — The reporting chain; jets heard late; 5–6 km altitude; spectrum monitoring.
- **kyivindependent-2025-ihnat-high-altitude** — Ihnat: above 2 km, visible to radar; 40% decoys.
- **euromaidan-2025-shandra-shaheds-dive** — Shahed cruise altitude and dive; SAM track capacity.
- **euromaidan-2025-zoria-shaheds-evade-kyiv** — Parodiya mimics large aircraft; helicopter FLIR; "lost from location" counts.
- **euromaidan-2026-mukhina-airforce-tracking** — Yelizarov: route tracking across regions; coordinated radar siting.
- **frontliner-2026-derkachov-mfg-modernise** — Crews warned by ear; 30 kg radars; anti-drone dome.
- **euromaidan-2026-zoria-cd-t10** — CD-T10 X-band radar (vendor figures).
- **united24-2025-litnarovych-iris-range** — Robin IRIS range from 5 to 12 km.
- **united24-2026-kohanets-giraffe** — Giraffe family in Ukraine (MoD).
- **militarnyi-2023-safronov-gepard** — Gepard radars and numbers.
- **isis-2024-anokhin-decoy-drones** — Gerbera and Parodiya construction, engines, Luneburg lenses, costs.
- **gwara-2025-kulish-decoy-waves** — Low blind spots; merged plots; decoys mapping the defence.
- **bi-2025-loh-gerbera-cameras** — Gerbera with two cameras; Supercam with an RF detector.
- **euromaidan-2026-mukhina-guided-shahed-mfg** — Guided Shaheds hunting MFG crews.
- **twz-2023-trevithick-iris-t-lancet** — Lancet hits an IRIS-T TRML-4D radar.
- **c4isrnet-2026-livingstone-interceptor-economics** — How interceptor crews are cued; the radar bottleneck.
- **euromaidan-2025-martyniuk-aerostat-ir** — Aerostat with an IR sensor and an interceptor.

Existing (knowledge base):
- **oryx-ukrainian-losses** — Visually confirmed Ukrainian radar and AA-gun losses.
- **rusi-2023-watling-meatgrinder** — Shaheds as pathfinders for missile packages.
- **rusi-2025-watling-third-year** — Forward air defence found and hit by Iskanders.
- **csis-2024-bondar-delta-cjadc2** — Delta's architecture and adoption.
- **csis-2026-bondar-shahed-geran** — Geran-3 jet speed.
- **pravda-2026-konyk-gerbera-fuel-tanks** — Gerbera specs; no modem means no alarm.
- **united24-2026-place-jet-gerans** — Jet Geran speeds.
- **militarnyi-2026-jet-geran-interceptors** — Geran-4 cruise speed.
- **euromaidan-2026-vivdych-xk-f358** — Mesh modems on Gerans and Gerberas.
- **isw-2026-07-02-roca** — New 3.9–4.1 GHz band on 1–2 July 2026.
- **uaf-2026-kpszsu-alerts-1-2-july** — Air Force alert timings for a ballistic salvo.
- **frontliner-2026-zvirynskyi-strike-intensity** — Decoy share in 2026.
- **militarnyi-2025-shumlianskyi-skynex-shaheds** — Skynex battery.
