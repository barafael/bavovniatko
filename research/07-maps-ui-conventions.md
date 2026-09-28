# 07 — Map and UI conventions of the Russo-Ukrainian war

Research date: 2026-09-28. Scope: how war maps of the 2022– full-scale invasion present information (DeepStateMap first), the military symbology standards behind "real" tactical displays, what Ukrainian military software is publicly known to look like, the visual language of drone/OSINT footage, and whether any of this data can legally be used by the game.

---

## 1. Overview

- **DeepStateMap is the reference.** It is Ukrainian, run by the DeepState UA NGO, fed largely by Ukrainian military sources, and deliberately published 2–3 days late for operational security [src:wikipedia-deepstatemap-live] [src:kyivindependent-2025-farrell-front-line-mapping]. Its visual grammar is simple. Semi-transparent area fills show control status (occupied, liberated, recently liberated, unknown/grey, pre-2022 occupied). Point icons mark Russian units, airfields and HQs, and arrows show "directions of attack" [src:deepstate-api-history]. Ukrainian positions are never shown [src:kyivpost-2025-ashcroft-inside-deepstate].
- **ISW/CTP is the Western reference.** Its legend is epistemic rather than just territorial. It separates *assessed* (evidence-backed) from *claimed* (asserted, unverified) and, since December 2025, draws a dedicated *infiltration areas* layer [src:isw-arcgis-control-of-terrain]. Most Western outlets (e.g., the BBC) redraw ISW/CTP data [src:bbc-2022-ukraine-in-maps].
- **The front line has stopped being a line.** From 2025 on, drone "kill zones" and infiltration by small Russian groups turned the grey zone from a thin no-man's-land into a deep contact zone with overlapping positions. That forced DeepState (2026) and ISW (Dec 2025) to redefine their uncertainty categories [src:onlineua-2026-deepstate-legend] [src:kyivindependent-2025-farrell-front-line-mapping] [src:warontherocks-2026-maurin-front-line]. This is directly relevant to eras e8-drone-kill-zone and e9-counteroffensive-26.
- **Mapping is political.** DeepState has clashed publicly with Ukrainian commanders over encirclements and breakthroughs (Makarivka, Dec 2024; Dobropillia, Aug 2025). Flag-planting photo-ops by both sides are an information-war tactic that careful mappers refuse to map [src:devua-2024-deepstate-makarivka] [src:kyivindependent-2025-farrell-front-line-mapping].
- **Tactical UIs are a different world.** Delta (MoD situational awareness), Kropyva (Army SOS artillery/mapping tablet app), and the "Vezha" drone-stream module with Avengers AI are the Ukrainian military's own digital layers [src:wikipedia-delta] [src:militarnyi-2022-delta-unveiled] [src:forbes-2022-hambling-kropyva]. The Army of Drones Bonus "e-points" system turned video-verified strikes into a points economy with unit leaderboards and a marketplace [src:kyivpost-2026-korshak-epoints] [src:defensenews-2026-ruitenberg-epoints] [src:united24-2026-petriv-army-of-drones].
- **Data rights are the main blocker.** DeepState has no documented API or open licence, although undocumented JSON endpoints exist and community scrapers use them. ISW's ArcGIS layers are public but carry "exclusive intellectual property… written consent" terms. ACLED requires a corporate licence for commercial use [src:deepstate-api-history] [src:isw-arcgis-control-of-terrain] [src:acled-eula]. Only the symbology renderer milsymbol is permissively (MIT) licensed [src:github-milsymbol].

---

## 2. DeepStateMap in depth

### 2.1 Who runs it, history
- **Operator:** DeepState UA (ДіпСтейт), a Ukrainian NGO co-founded by childhood friends **Roman Pohorilyi** (law background) and **Ruslan Mykula** (marketing) [src:wikipedia-deepstatemap-live] [src:kyivpost-2025-ashcroft-inside-deepstate].
- **Origins:** It began in February 2020 as a Telegram channel on world news and politics. The team experimented with an updatable conflict map during the 2021 Taliban offensive, and before the full-scale invasion it switched to tracking Russian forces on Ukraine's borders [src:wikipedia-deepstatemap-live].
- **Launch:** The map went live on **24 February 2022** (e1-invasion). It first ran on Google Maps. Google blocked the account in late March 2022, so DeepState built its own platform, and data from before 3 April 2022 was lost [src:wikipedia-deepstatemap-live]. The site's own version history starts on 2022-04-03, which matches this [src:deepstate-api-history].
- **Team and security:** 100+ paid staff and volunteers work from an undisclosed location in Kyiv. Staff are vetted, including with polygraphs, and access is tiered so that the most sensitive material stays with the co-founders [src:kyivpost-2025-ashcroft-inside-deepstate]. About 60 people work on verification [src:wikipedia-deepstatemap-live].
- **Reach:**
  - Its busiest days came during the September 2022 Izium/Lyman liberation (e3) [src:wikipedia-deepstatemap-live].
  - It passed 1 billion total views, announced in February 2024 [src:wikipedia-deepstatemap-live].
  - The Telegram channel had 800k+ subscribers in 2025. The audience is about 70% Ukrainian and 30% international [src:kyivpost-2025-ashcroft-inside-deepstate].
- **Funding:** Mostly donations, plus Telegram ads and merchandise [src:kyivpost-2025-ashcroft-inside-deepstate]. Wikipedia also records partial government funding [src:wikipedia-deepstatemap-live].
- **Apps:**
  - Android (March 2023) and iOS (April 2023, after Apple initially resisted) [src:wikipedia-deepstatemap-live].
  - "DeepStateMap 2.0" (2 July 2024) overhauled the graphics and added offline mode, drawing tools, keyboard shortcuts, expanded weapon-range overlays and coordinate copying [src:wikipedia-deepstatemap-live].
- **Tech stack (from the live page source):** Leaflet with a PixiJS WebGL overlay, Highcharts for statistics charts, fabric.js for the drawing tools, and leaflet.pattern for patterned fills [src:deepstate-map-live].

### 2.2 Methodology and sourcing
- **Inputs:**
  - Geolocated photos and video, cross-checked against multiple sources [src:wikipedia-deepstatemap-live].
  - Direct reports from Ukrainian soldiers and units, "from frontline soldiers to command bunkers." Units routinely disagree: "Everyone explains things their own way… some see only their sector" (Pohorilyi) [src:kyivpost-2025-ashcroft-inside-deepstate] [src:kyivindependent-2025-farrell-front-line-mapping].
  - Official MoD and General Staff information [src:wikipedia-deepstatemap-live].
- **Russian claims are excluded** as a matter of policy [src:wikipedia-deepstatemap-live].
- **Deliberate delay:** the map typically lags reality by **2–3 days** for OPSEC, and sensitive sectors are held back by agreement with the military [src:wikipedia-deepstatemap-live] [src:kyivindependent-2025-farrell-front-line-mapping]. Ukrainian gains are sometimes withheld until operations finish. The July 2026 figures were published with pending Ukrainian areas "for operational security" [src:kyivpost-2026-zavadska-deepstate-july].
- **Anti-emotional rule:** Russian flag photos do not move the line. "We cannot react emotionally or quickly, or when we see Russians with flags there, we draw…" (Pohorilyi) [src:kyivindependent-2025-farrell-front-line-mapping].

### 2.3 Update cadence
The site's history endpoint listed **1,771 published map versions** between 3 April 2022 and 27 September 2026. The median gap between versions is about 24 hours: 420 versions in 2022, then about 360 a year, and 27–31 a month in 2026 [src:deepstate-api-history]. Each version carries a short changelog in Ukrainian and (from May 2022) English, with links to the affected settlements, e.g. "The Ukrainian Armed Forces liberated Ridkodub, Nove and Katerynivka…" (27 Sep 2026) [src:deepstate-api-history]. The live map is best understood as **one curated daily snapshot plus a news-style change log**, not a real-time feed.

### 2.4 Colour scheme and legend
Wikipedia describes the legend as follows [src:wikipedia-deepstatemap-live]:
- **red:** Russian-occupied
- **green:** liberated more than two weeks ago
- **blue:** liberated within the last two weeks, or held during the Kursk incursion
- **pink:** occupied before 2022 (Crimea, parts of Donetsk and Luhansk oblasts)
- **grey:** unknown status
- **light red:** "controversial" Russian occupations outside Ukraine

The live GeoJSON (fetched 2026-09-28) states the actual style values. Every area uses about 30% fill opacity with a solid outline in the same colour [src:deepstate-api-history]:

| Feature name (EN) / internal key | Stated fill | In words |
|---|---|---|
| Occupied / `geoJSON.status.occupied` | `#a52714` | dark brick red |
| Liberated / `geoJSON.status.dismissed` (older polygons keep date labels such as "Liberated 27–31.03") | `#0f9d58` | green |
| Liberated, recent | `#0288d1`, `#01579b` | light and dark blue |
| Unknown status / `geoJSON.status.unknown` | `#bcaaa4`, `#bdbdbd` | warm grey and neutral grey |
| CADR and CALR (ORDLO), Occupied Crimea, Tuzla / `geoJSON.territories.ordlo`, `.crimea` | `#880e4f` | deep crimson-magenta ("pink" at 30% opacity) |
| Transnistria, Abkhazia, Tskhinvali, and trolling entries such as "temporarily occupied" Karelia, East Prussia, Southern Kurils | `#ff5252` | bright red |

Notes:
- The `styleUrl` strings (`#poly-A52714-2000-77-nodesc`) follow a Google-Maps/KML naming pattern. This is my inference and consistent with the map's Google Maps origins.
- The labels have a Ukrainian wartime voice. Examples: "shoigists 'storm-z'" and "Pro-Russian clown Fico", and the site metadata writes "russia" in lower case [src:deepstate-api-history] [src:deepstate-map-live].

### 2.5 The grey zone and uncertainty
- **Old meaning:** grey meant "territory where the situation requires clarification," in practice the thin no-man's-land between the two sides' first lines [src:onlineua-2026-deepstate-legend] [src:kyivindependent-2025-farrell-front-line-mapping].
- **2025 shift:** Ukraine had too few troops to man positions densely, and Russia attacked with small infiltration squads instead of mechanised assaults. The result on DeepState was "increasingly jagged and unusual formations," with grey now depicting a wide contact zone of overlapping positions [src:kyivindependent-2025-farrell-front-line-mapping].
- **2026 redefinition:** DeepState explained that grey now represents **areas where Russian forces have infiltrated**. Positions "may effectively overlap or intermix," and grey should not be read as no-man's-land, as either side's control, or as a precise front [src:onlineua-2026-deepstate-legend]. Internally the polygons are still coded `geoJSON.status.unknown` [src:deepstate-api-history].
- **How deep the zone is:** one analyst describes the contested zone near Kotlyne growing from about 1 km deep (Jan 2025) to "five to eight times wider." He argues that a front "thickened into a depth of 12 kilometers" can no longer be the thing being fought over [src:warontherocks-2026-maurin-front-line].
- **Beyond grey:** the only other uncertainty device is time. The two-week blue "recently liberated" state and the dated "Liberated dd.mm" labels encode how old a change is [src:wikipedia-deepstatemap-live] [src:deepstate-api-history].

### 2.6 Symbols and layers
- **Point features in the live data** [src:deepstate-api-history]:
  - About 60 "Direction of attack" arrows (`arrow_8`…`arrow_16` icon variants, apparently orientations).
  - About 260 Russian unit markers (`icon=enemy`), keyed by echelon: army, division, regiment, brigade, battalion. Separate keys cover BARS volunteer detachments, Storm-Z/Storm-V, Akhmat and Belarusian units (`icon=iblorussia`).
  - About 44 airfields/airports, including in Belarus and Russia.
  - About 15 headquarters.
  - Landmarks: Crimean bridge, the cruiser *Moskva*, capitals.
- **Toggles described in secondary sources:**
  - Russian HQs, airfields, railways and fortifications.
  - A losses statistics chart and a weather layer.
  - Background radiation.
  - Weapon-system range rings.
  - Fires detected in the last 48 hours (NASA FIRMS).
  - Distance measurement and history/compare tools.
  - A removed "pathogen mode" (blurred troop-concentration view) [src:wikipedia-deepstatemap-live] [src:onlineua-2026-deepstate-legend].
- **No friendly units, by design.** DeepState shows "only enemy positions and Russian defences" [src:kyivpost-2025-ashcroft-inside-deepstate].
- **No NATO symbology.** DeepState uses its own icon set, not APP-6 frames [src:deepstate-api-history].

### 2.7 Monthly statistics (km²)
DeepState publishes monthly tallies of territory changes. They are widely quoted by Ukrainian and Western media:
- **July 2026:** Russia advanced **36 km²** confirmed (possibly 88 km² once delayed reporting catches up). Ukraine regained at least **49 km² and 83 km²** in two areas that were pending publication [src:kyivpost-2026-zavadska-deepstate-july].
- **May 2026:** Russia took about 130 km² and Ukraine regained about 250 km², a net gain of about 120 km² for Ukraine despite 7,000+ combat engagements [src:kyivpost-2026-zavadska-deepstate-july].
- **Western comparison:** ISW put Russia's 2025 gains at about 4,700 km² against Russia's claimed 6,000 km² [src:bbc-2022-ukraine-in-maps]. The gap between claimed and assessed numbers is itself a UI motif.

### 2.8 Relationship with the military, and controversies
- **Memorandum:** DeepState signed a Memorandum of Cooperation on data exchange with the Ministry of Defence on 13 March 2024 (e6) [src:wikipedia-deepstatemap-live]. Commander-in-Chief Syrskyi has praised the map [src:wikipedia-deepstatemap-live].
- **Too fast or too slow:** in 2023 some units objected that fast updates exposed their positions, while others complained about delays [src:wikipedia-deepstatemap-live].
- **Makarivka, December 2024 (e7):** DeepState warned on 17 December of a near-encirclement of the Makarivka garrison. On 23 December the Khortytsia group denied it. MP Mariana Bezuhla alleged that Syrskyi was moving against the team, while AFU leadership (per journalist Yurii Butusov) denied any conflict. Pohorilyi said: "not all commanders-in-chief like the truth" [src:devua-2024-deepstate-makarivka].
- **Dobropillia, August 2025 (e8):**
  - The General Staff spoke of scattered sabotage groups, while DeepState mapped a deep Russian penetration after a day of checking with units [src:kyivindependent-2025-farrell-front-line-mapping].
  - A battalion commander said the map prompted faster reaction from higher command than official channels [src:kyivindependent-2025-farrell-front-line-mapping].
  - Pohorilyi: "it is nonsense when projects like ours are needed to talk about these problems" [src:kyivindependent-2025-farrell-front-line-mapping].
- **Commercial reuse:** in November 2025 Polymarket overlaid the map without permission. DeepState complained and the overlay was removed [src:wikipedia-deepstatemap-live].

---

## 3. Other maps compared

| Map | Operator (country) | Style | Legend categories | Update cadence | Data access / licence | Source id |
|---|---|---|---|---|---|---|
| **DeepStateMap** | DeepState UA NGO (Ukraine) | Semi-transparent control polygons, custom point icons, attack arrows; Leaflet web plus apps | Occupied, liberated, recently liberated, unknown status (grey/infiltration), pre-2022 occupied, foreign occupied; RU units, airfields, HQs | About daily versions, deliberately 2–3 days behind | Undocumented JSON endpoints; no open licence found; complains about unauthorised commercial reuse | deepstate-map-live, deepstate-api-history |
| **ISW / CTP** | Institute for the Study of War and AEI Critical Threats Project (USA) | Static daily PNG maps in the ROCA plus an ArcGIS StoryMap/web map | Assessed Russian-controlled terrain, assessed Russian advances, assessed Russian infiltration areas, claimed Russian territory, claimed Ukrainian counteroffensives, pre-24 Feb 2022 occupied, control of terrain in Russia, Russian field fortifications; separate partisan layers | Daily (features stamped `pub_date` 2026-09-27); time-lapse archive monthly | Public ArcGIS Feature Services, but "exclusive intellectual property of ISW… written consent" | isw-arcgis-control-of-terrain, isw-cta-roca-2026-09-26 |
| **Black Bird Group** | Finnish NGO (Finland) | Two-sided front lines plus movement arrows over an OpenFreeMap/OSM basemap; NATO red/blue | Russian-held territory, front line (Russian side), front line (Ukrainian side), RU movements, UA movements | "Updated regularly" (near daily); weekly Friday snapshot feeds ACLED | "Public product"; non-profit/public-facing adaptations welcomed; no formal licence text | blackbirdgroup-map |
| **UA Control Map** | Pseudonymous (unverified) | Geolocations, front lines, unit positions; clickable per-feature source links | Not verified | Not verified | Web "data viewer"; terms not verified | kyivindependent-2025-farrell-front-line-mapping |
| **Liveuamap** | Liveuamap, founded by two Dnipro software engineers (Ukraine; since Feb 2014) | Event-pin map fed by an algorithmically filtered, human-verified social media stream; archive by day [src:wikipedia-liveuamap] | Event icons by type/side (details not verified) | Continuous | Terms not verified (site blocked fetch) | wikipedia-liveuamap |
| **GeoConfirmed** | Volunteer OSINT collective (international) | Every marker is one geolocated clip with sources and a public geolocation breakdown; timeline playback; ORBAT trees | Per-event, filterable by date, equipment, unit, faction | Continuous (883 new Ukraine events in the 30 days before fetch) | Advertises a "documented, versioned, free to use" public API | geoconfirmed-home |
| **ACLED Ukraine Conflict Monitor** | ACLED (international NGO) | Scaled event circles plus oblast choropleth; front line from Black Bird Group | Political-violence event types; infrastructure tags (energy, health, education, residential); civilian fatalities (conservative) | Weekly situation updates | Registration; EULA: non-commercial by default, corporate licence for businesses, no raw redistribution, attribution mandatory | acled-ukraine-conflict-monitor, acled-eula |
| **BBC "Ukraine in maps"** | BBC Visual Journalism (UK) | Explainer maps with narrative text | Redraws ISW/CTP daily assessments of Russian control | Periodic (first published 24 Feb 2022, last updated Sep 2026) | © BBC | bbc-2022-ukraine-in-maps |
| **NYT "Maps: Tracking the Russian Invasion of Ukraine"** | New York Times (USA) | Long-running interactive, updated through the war [src:nyt-2022-ukraine-maps] | Not verified (fetch blocked) | Periodic | © NYT | nyt-2022-ukraine-maps |
| **Suriyak Maps** | Pseudonymous (unverified) | Not verified | Not verified | Not verified | Not verified | — (no verified source; see gaps) |
| **Andrew Perpetua** | Independent analyst (unverified) | Known for geolocation-based tallies; map not verified | Not verified | Not verified | Not verified | — |
| **AMK Mapping** | Pseudonymous (unverified) | Not verified | Not verified | Not verified | Not verified | — |

Notes:
- **Assessed vs claimed.** ISW's two epistemic tiers ("assessed" = evidence-backed, "claimed" = asserted but unverified) are the clearest model of *confidence as a map layer*. In the web-map JSON:
  - Russian-controlled terrain and Russian advances share a translucent red fill (RGBA 239,0,0,71) with a red outline.
  - Infiltration areas are a patterned picture-fill with a **dashed** red outline.
  - Claimed Russian territory is translucent orange (255,170,0,126).
  - Claimed Ukrainian counteroffensives are translucent light blue (115,223,255,147) with a blue outline [src:isw-arcgis-control-of-terrain].
  - The infiltration feature service was created on 20 December 2025 (e8) [src:isw-arcgis-control-of-terrain].
- **ISW methodology** (per search excerpts of the daily ROCA):
  - ISW uses only publicly available information and no classified material.
  - It depicts "the furthest assessed extent of Russian advances until open-source evidence" shows otherwise, and warns this may underestimate Ukrainian advances.
  - It notes that a porous front with intermingled positions complicates control of terrain [src:isw-cta-roca-2026-09-26].
- **Partisans.** ISW keeps two partisan products: "reported but not confirmed Ukrainian partisan warfare" (feature service, 2022–) and a curated "Verified Ukrainian Partisan Attacks" StoryMap that only includes high-confidence events [src:isw-arcgis-control-of-terrain].
- **Black Bird Group on control:** "When we paint an area as Russian or Ukrainian-controlled, it's more like here the Ukrainians or Russians generally have more control than the other side." It also values being able to say its map is based on open sources [src:kyivindependent-2025-farrell-front-line-mapping].

---

## 4. Symbology standards and renderers

- **Standards family.** NATO APP-6 (STANAG 2019) and US MIL-STD-2525 are aligned standards. APP-6A (1999) is equivalent to MIL-STD-2525A. Later editions are APP-6B (2008) and APP-6C (2011), and APP-6E is listed as current [src:wikipedia-nato-joint-military-symbology]. milsymbol supports MIL-STD-2525C/D/E, FM 1-02.2 and APP-6 B/D/E [src:github-milsymbol].
- **Affiliation is encoded twice, by frame shape and by colour** [src:wikipedia-nato-joint-military-symbology]:
  - **Friend:** rectangle, **blue**
  - **Hostile:** diamond, **red**
  - **Neutral:** square, **green**
  - **Unknown:** quatrefoil, **yellow**
  - Further states: pending, assumed friend, suspect, and exercise variants.
  - Because the shape carries affiliation too, symbols work in monochrome.
- **Frame modifiers** [src:wikipedia-nato-joint-military-symbology]:
  - Solid frame = present; **dashed frame = planned/anticipated**.
  - Closed frames = land and sea surface; bottom-open = air/space; top-open = subsurface.
- **Symbol anatomy** [src:wikipedia-nato-joint-military-symbology] [src:spatialillusions-battle-staff-tools]:
  - Frame, central icon, optional fill.
  - Modifiers/amplifiers arranged around the frame: unique designation, additional information, altitude/depth, combat effectiveness, headquarters/task force/dummy, echelon/mobility.
- **Echelons.** Coded in the SIDC and drawn above the frame: Team/Crew, Squad, Section, Platoon/detachment, Company/battery/troop, Battalion/squadron, Regiment/group, Brigade, Division, Corps, Army, Army Group/front, Region/Theatre, Command [src:github-milsymbol].
- **SIDC formats.** Older standards use letter-based symbol identification codes, 15 characters in the APP-6A lineage [src:wikipedia-nato-joint-military-symbology]. Current 2525D/E and APP-6D/E use numeric codes; milsymbol's README example is a 30-digit SIDC. milsymbol implements both schemes [src:github-milsymbol].
- **Renderers:**
  - **milsymbol** (Måns Beckman / spatialillusions):
    - Pure JavaScript, **MIT licence**, SVG or Canvas output.
    - Filled/unfilled and framed/unframed symbols, text fields, movement indicators.
    - Since v3.0 it renders to MIL-STD-2525E/APP-6E appearance for every SIDC version. The latest release is v3.0.4 (March 2026).
    - Integrations include Leaflet, OpenLayers and Cesium [src:github-milsymbol].
    - This is the safe choice for game assets, e.g. pre-render SVGs to a texture atlas at build time.
  - **Spatial Illusions "Battle Staff Tools"** (the "unit generator"):
    - A browser tool for MIL-STD-2525E/APP-6E and 2525C/APP-6B symbols with PNG/SVG export, ORBAT building and a map.
    - © 2025 Måns Beckman. Free on spatialillusions.com with watermarks and donation prompts; offline use or use on other sites needs a paid licence file [src:spatialillusions-battle-staff-tools].
    - Treat its exports as **not** cleared for redistribution in a game.
- **Who follows NATO conventions:**
  - Black Bird Group explicitly uses the NATO convention "red is the Russian adversary, blue is Ukrainian" [src:blackbirdgroup-map].
  - Delta was "built according to NATO standards" [src:militarnyi-2022-delta-unveiled].
  - DeepState does not use APP-6 frames [src:deepstate-api-history].

---

## 5. Ukrainian military UIs (public glimpses)

- **Delta (Дельта)**:
  - A situational-awareness and battlefield-management system developed by the MoD's Centre for Innovation and Development of Defence Technologies, with the Aerorozvidka NGO [src:wikipedia-delta].
  - It fuses reports from reconnaissance units, drones, sensors, satellite imagery, partner intelligence and vetted civilians, and maps geolocated data in real time "along with pictures of enemy assets" [src:wikipedia-delta].
  - Cloud-native; it runs on PCs, laptops, tablets and phones [src:wikipedia-delta].
  - Timeline:
    - First tested in 2017 in a NATO context [src:wikipedia-delta].
    - Broadly operational in August 2022 [src:wikipedia-delta].
    - Publicly unveiled at the NATO TIDE Sprint (Virginia Beach, October 2022). The developers cited years of TIDE Hackathon/TIDE Sprint/CWIX participation, and the system integrates the eVorog and "STOP Russian War" chatbots [src:militarnyi-2022-delta-unveiled].
    - Full deployment approved on 4 February 2023 (e4) [src:wikipedia-delta].
- **Vezha and Avengers (drone "streams"):**
  - Vezha is Delta's video-stream module. The Avengers AI running inside it auto-detects and classifies enemy vehicles in drone and stationary-camera feeds: about 12,000 targets a week, about 70% of visible equipment, about 2.2 s per detection [src:wikipedia-delta].
  - It was shown at NATO TIDE Sprint (Helsinki, February 2025) and the London Defence Conference (May 2025) [src:wikipedia-delta].
  - This is the backbone of the "streams" wall that Ukrainian HQs watch. No verified public screenshot description was found (see gaps).
- **Kropyva (Кропива, "Nettle")**:
  - Proprietary intelligence-mapping software from the volunteer organisation Army SOS (since 2014), running on Android tablets [src:forbes-2022-hambling-kropyva].
  - It maps battle lines and targets, calculates artillery fire missions, and takes drone data to compute fire corrections. It is used "from divisional command right down to individual vehicles" and supplied as a rugged system compatible with NATO-standard secure communications [src:forbes-2022-hambling-kropyva].
- **Army of Drones Bonus / e-points (е-бали):**
  - Launched in 2023 on Mykhailo Fedorov's initiative. Every strike must be **video-verified** before points are awarded [src:kyivpost-2026-korshak-epoints].
  - Published price list [src:kyivpost-2026-korshak-epoints]:

    | Result | Points |
    |---|---|
    | Soldier killed | 12 |
    | Soldier wounded | 8 |
    | Drone operator | 25 |
    | Tank | 40 |
    | MLRS | 50 |
    | Manned helicopter | 100 |
    | Prisoner taken alive | 120 |

  - Units spend points on drones, EW and other kit in the **Brave1 marketplace** [src:defensenews-2026-ruitenberg-epoints].
  - 2025 totals: 819,737 verified strikes, including about 240k on personnel [src:united24-2026-petriv-army-of-drones] [src:defensenews-2026-ruitenberg-epoints].
  - A public **top-10 unit leaderboard** is led by the 414th "Birds of Madyar" brigade [src:united24-2026-petriv-army-of-drones].
  - 2026 changes extend points to air defence, army aviation and snipers, and add multipliers for longer-range strikes [src:united24-2026-petriv-army-of-drones].
  - This is a real, state-run gamification of the war, and a design (and ethics) reference for any score screen.

---

## 6. Visual language of drone and OSINT footage (UI inspiration)

- **FPV on-screen display (OSD):**
  - The blocky white telemetry text over FPV strike clips comes from flight-controller OSDs such as Betaflight's. Elements sit on a **character grid**: 30×16 for PAL and 30×13 for NTSC on MAX7456 analogue chips, and a default 53×20 for HD digital systems. Fonts are uploaded as glyph sets [src:betaflight-osd-tab].
  - Standard elements include timers, alarms, warnings and a post-flight statistics screen [src:betaflight-osd-tab].
  - A game HUD built on a monospace glyph grid reads instantly as "drone feed."
- **Signal breakup:** analogue static and freeze at impact or under EW is the other signature of FPV footage. This is an observation, not sourced here.
- **Thermal:** night drone and sight footage is usually monochrome "white-hot" or "black-hot," with colour palettes (e.g. "ironbow") as alternatives. No fetchable source was obtained this session, so see gaps.
- **Geolocation proofs:** GeoConfirmed publishes a public "geolocation breakdown" for each event. This is the OSINT genre of annotated side-by-side frame and satellite comparisons with matched landmarks [src:geoconfirmed-home].
- **AI detections:** Avengers adds machine classifications of vehicles over live streams [src:wikipedia-delta]. In the game this could appear as auto-tagged boxes in a "Vezha" feed panel.
- **Map overlays:**
  - DeepState's attack arrows, and Black Bird's paired RU/UA front lines with movement arrows [src:deepstate-api-history] [src:blackbirdgroup-map].
  - ISW's dashed outline for infiltration (dashed = uncertain/anticipated, echoing APP-6) [src:isw-arcgis-control-of-terrain] [src:wikipedia-nato-joint-military-symbology].

---

## 7. Data access and licensing

- **DeepState:**
  - No documented public API or open licence was found. The web app itself calls JSON endpoints [src:deepstate-api-history]:
    - `…/api/history/public`: a list of every map version with UA/EN changelog text and timestamps.
    - `…/api/history/last`: a GeoJSON FeatureCollection with KML-style `fill`/`stroke`/`styleUrl` properties and trilingual names (`Ukrainian /// English /// i18n key`).
  - Community scrapers:
    - **cyterat/deepstate-map-data** (GPL-3.0 code): daily GeoJSON since 2024-07-08, merging "Occupied", "CADR and CALR" and "Occupied Crimea" into one occupied multipolygon, plus a consolidated gzip history [src:github-cyterat-deepstate-map-data].
    - Others found on GitHub include a KML exporter and a GIF generator (not individually cited).
  - The scraper's GPL covers the *code*, not DeepState's data. DeepState has objected to unauthorised commercial overlays (Polymarket) [src:wikipedia-deepstatemap-live].
  - **For a commercial game, ask DeepState for permission** or use it only as an internal reference.
- **ISW/CTP:**
  - The layers are public ArcGIS Feature Services, queryable via REST, e.g. `VIEW_RussiaCoTinUkraine_V2`, `View_AssessedRussianInfiltrationAreasinUkraine_V4`, `VIEW_ClaimedUkrainianCounteroffensives_V2`, `RUAF_Field_Fortifications_Polylines`.
  - Every item states: "This geodata is the exclusive intellectual property of the Institute for the Study of War (ISW). You may not use this geodata without the written consent of ISW." [src:isw-arcgis-control-of-terrain]
- **ACLED:**
  - Non-commercial use is covered by a royalty-free, non-exclusive licence. Commercial businesses need a corporate licence and governments a public-sector licence.
  - No direct access to raw data may be given. Derivatives must be "transformative," and attribution is mandatory [src:acled-eula].
  - The monitor needs a login for curated files [src:acled-ukraine-conflict-monitor].
- **GeoConfirmed:** its homepage advertises a free public API. Its licence terms were not reviewed [src:geoconfirmed-home].
- **Black Bird Group:** presented as a public product and welcomed in non-profit or public-facing adaptations. There is no formal licence, so ask [src:blackbirdgroup-map].
- **Symbology:** milsymbol is MIT and fine to ship [src:github-milsymbol]. Battle Staff Tools is proprietary [src:spatialillusions-battle-staff-tools].
- **Recommendation:** ship the game with **self-authored, simplified historical control polygons** (e.g. digitised by hand at era granularity, informed by these maps). Credit DeepState, ISW and Black Bird as inspiration, and request explicit permission before bundling any of their geometry.

---

## 8. Game and sim relevance

- **Two map modes, two epistemologies.**
  - A strategic "DeepState-style" layer: semi-transparent occupied/liberated/grey fills, a daily-snapshot rhythm, a changelog feed ("Defence Forces liberated X"), enemy-only unit icons and attack arrows.
  - A tactical layer in APP-6 symbology rendered with milsymbol: blue rectangles for friendly and red diamonds for hostile, with dashed frames for unconfirmed contacts.
  - The player should *feel* that the strategic map is a lagged public picture and the tactical map is their own (partial) sensor picture.
- **Model uncertainty explicitly.**
  - Use ISW's assessed/claimed/infiltration tiers or DeepState's grey zone as first-class terrain states: patterned fill and dashed outline for infiltration, a separate hue for "claimed."
  - Let grey-zone *depth* grow in later eras (e8/e9), from about 1 km to 5–12 km of contested depth [src:warontherocks-2026-maurin-front-line] [src:onlineua-2026-deepstate-legend].
  - Freshness can use DeepState's two-week blue "recently liberated" state [src:wikipedia-deepstatemap-live].
- **OPSEC delay as a mechanic.** DeepState publishes 2–3 days late and withholds Ukrainian gains until operations finish [src:kyivindependent-2025-farrell-front-line-mapping] [src:kyivpost-2026-zavadska-deepstate-july]. The game could show the player's successes on the public map only after a delay, and model Russian "flag operations" that create false claims the player must not over-react to.
- **Statistics screen.** A monthly km² gained/lost panel, as in DeepState's monthly reports [src:kyivpost-2026-zavadska-deepstate-july], with "claimed vs assessed" comparisons [src:bbc-2022-ukraine-in-maps]. An optional e-points-style results ledger would show video-verified strikes, a points economy and a Brave1-like shop [src:kyivpost-2026-korshak-epoints] [src:defensenews-2026-ruitenberg-epoints]. Handle the leaderboard with care: it scores human casualties.
- **Drone-feed panels.** Picture-in-picture "Vezha" streams with a character-grid OSD, white-hot thermal and AI detection tags, as a diegetic way to deliver intel [src:betaflight-osd-tab] [src:wikipedia-delta].
- **Command friction.** Tension between the map and commanders (Makarivka, Dobropillia) can become an event system: HQ reports versus independent-map reports that the player must reconcile [src:devua-2024-deepstate-makarivka] [src:kyivindependent-2025-farrell-front-line-mapping].

---

## 9. Terms

- **grey zone (сіра зона)** — Contested or unclear-control area. On DeepState, since 2026, it means areas of Russian infiltration with intermixed positions, not simple no-man's-land.
- **liberated (звільнено)** — DeepState status for areas Ukraine retook. Its internal key is `dismissed`, and it is blue for about two weeks, then green.
- **ORDLO / CADR and CALR** — "Certain Areas of Donetsk/Luhansk Regions": the parts of Donbas occupied since 2014, drawn separately from post-2022 occupation.
- **TOT** — Temporarily occupied territories, the official Ukrainian term. DeepState applies it ironically to Karelia, East Prussia, the Kurils and others.
- **direction of attack (напрямок удару)** — DeepState arrow icon for Russian offensive axes.
- **assessed vs claimed** — ISW's two evidence tiers: backed by open-source evidence, or asserted by a party without confirmation.
- **infiltration area** — ISW layer (since Dec 2025) for areas where Russian small groups operate without assessed control.
- **ROCA** — ISW/CTP daily *Russian Offensive Campaign Assessment*.
- **CoT** — Control of terrain, ISW's term for the territorial layer.
- **flag operation** — Planting a flag in a contested settlement for a photo, to create an impression of capture.
- **OSGT (ОСУВ)** — Operational-strategic group of troops (e.g. Khortytsia, later Dnipro), whose statements sometimes contradict mappers.
- **APP-6 / MIL-STD-2525** — NATO and US military symbology standards.
- **SIDC** — Symbol identification code, the letter or numeric string that fully specifies a military symbol.
- **amplifier / modifier** — Text or graphic fields around or inside an APP-6 frame (designation, echelon, HQ staff, etc.).
- **echelon** — Unit size indicator (squad … brigade … army) drawn above the frame.
- **Delta (Дельта)** — MoD situational-awareness and battle-management system.
- **Vezha (Вежа)** — Delta's video-stream module.
- **Avengers** — MoD AI for automatic detection in drone video.
- **Kropyva (Кропива)** — Army SOS tablet app for mapping and artillery fire calculation.
- **e-points (е-бали) / Army of Drones Bonus** — MoD programme awarding points for video-verified strikes, spendable on the Brave1 marketplace.
- **Brave1 Market** — Ukrainian government defence-tech marketplace where units buy equipment.
- **OSD** — On-screen display, the telemetry overlay on an FPV video feed.
- **white-hot / black-hot** — Monochrome thermal palettes where hotter objects appear white or black.
- **geolocation** — Establishing where footage was shot by matching features to maps or satellite imagery.

---

## 10. Open questions and gaps

- **ISW methodology page and ROCA text** could not be fetched (403). The methodology language comes from search excerpts, and the daily ROCA is `verified: search-only`. The legend itself was verified from ISW's own ArcGIS web-map JSON.
- **ISW's historic 2022 legend** (e.g., "Ukrainian counteroffensives," "reported partisan warfare" on static maps) was not checked visually. Only current layer names are confirmed.
- **Suriyak Maps, Andrew Perpetua, AMK Mapping:** no fetchable primary or reliable secondary source was obtained this session (the web-search budget ran out). Their operators, countries, legends, cadence and possible bias are **unverified**. Research separately before relying on them.
- **UA Control Map and Liveuamap** legend/icon semantics are unverified: the sites blocked fetches. Liveuamap's side-colour convention needs checking.
- **Kyiv Independent map pages** and **NYT "where the front line moved"** pieces were not fetched (NYT blocked). Their methodology and data sources are unknown.
- **DeepState's official terms of use** were not found. It is unclear whether the `/api/history/*` endpoints are tolerated for third-party use.
- **Delta/Vezha public screenshots:** no verified description of the UI appearance (map style, symbology, colour) was found. The NATO ACT "DELTA at CWIX24" article (July 2024) is behind a Cloudflare challenge.
- **Thermal palettes** (white-hot/black-hot/ironbow) and FPV "signal loss" visuals are described from general knowledge and need a citable source.
- **DeepState monthly km² series for 2022–2025** (e.g., peak Russian months in late 2024) was not collected. It could be computed from the cyterat daily GeoJSON history (2024-07 onwards) or the `/api/history` snapshots.
- **Date conflict:** the Makarivka encirclement dispute is dated December 2024 by dev.ua. A Wikipedia summary placed it in December 2023. dev.ua's date is used.

---

## 11. Sources

- `deepstate-map-live` — Live DeepStateMap page; page metadata and tech stack (Leaflet/PixiJS/Highcharts/fabric.js).
- `deepstate-api-history` — DeepState's undocumented JSON history endpoints; the legend categories, stated colours, icon types and 1,771-version update log.
- `wikipedia-deepstatemap-live` — Encyclopedic history: founders, Google Maps block, colours, MoD memorandum, controversies, apps.
- `kyivpost-2025-ashcroft-inside-deepstate` — Profile of the team: sourcing from the military, OPSEC, vetting, colour code, audience.
- `kyivindependent-2025-farrell-front-line-mapping` — How the blurred front makes mapping harder and political; grey zone evolution; Dobropillia; other mappers.
- `onlineua-2026-deepstate-legend` — Ukrainian explainer of DeepState's legend and its 2026 grey-zone redefinition.
- `kyivpost-2026-zavadska-deepstate-july` — DeepState's July 2026 km² statistics, with the May 2026 comparison.
- `devua-2024-deepstate-makarivka` — Makarivka encirclement dispute and alleged pressure on DeepState (Dec 2024).
- `github-cyterat-deepstate-map-data` — Community daily GeoJSON of DeepState occupied areas (GPL-3.0 code).
- `isw-arcgis-control-of-terrain` — ISW's own ArcGIS web map: legend layers, symbology, daily pub_date, licence text.
- `isw-cta-roca-2026-09-26` — Representative ISW/CTP daily Russian Offensive Campaign Assessment (search-only).
- `blackbirdgroup-map` — Black Bird Group's war map: legend, NATO red/blue convention, usage stance.
- `acled-ukraine-conflict-monitor` — ACLED monitor: event map, weekly updates, Black Bird front line.
- `acled-eula` — ACLED licence terms: non-commercial default, corporate licence, attribution.
- `geoconfirmed-home` — GeoConfirmed: verified geolocation markers, ORBATs, timeline, public API.
- `wikipedia-liveuamap` — Liveuamap's origin (Dnipro, 2014) and algorithm-plus-human verification model.
- `bbc-2022-ukraine-in-maps` — BBC explainer maps built on ISW/CTP data; ISW 2025 km² figure.
- `nyt-2022-ukraine-maps` — NYT long-running interactive map (search-only; fetch blocked).
- `wikipedia-nato-joint-military-symbology` — APP-6/MIL-STD-2525 overview: frames, colours, status, dimensions.
- `github-milsymbol` — milsymbol JS renderer (MIT): supported standards, SIDC schemes, echelon list.
- `spatialillusions-battle-staff-tools` — Spatial Illusions unit generator: features and proprietary licence model.
- `wikipedia-delta` — Delta system history, Vezha/Avengers video-AI module.
- `militarnyi-2022-delta-unveiled` — Delta's public unveiling at NATO TIDE Sprint 2022; NATO standards.
- `forbes-2022-hambling-kropyva` — Kropyva (Army SOS) tablet mapping and fire-control software.
- `kyivpost-2026-korshak-epoints` — Army of Drones Bonus e-points price list and video verification.
- `defensenews-2026-ruitenberg-epoints` — e-points spendable on the Brave1 marketplace; 2025 strike totals.
- `united24-2026-petriv-army-of-drones` — 2025 top-10 unit leaderboard; 2026 e-points expansion.
- `betaflight-osd-tab` — Betaflight OSD documentation: character-grid HUD on FPV feeds.
- `warontherocks-2026-maurin-front-line` — Analysis of the front "thickening" into a deep contested zone.
