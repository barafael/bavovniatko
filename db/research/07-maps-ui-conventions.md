<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# 07 — Map and UI conventions of the Russo-Ukrainian war

Research date: 2026-09-28. Scope: how war maps of the 2022– full-scale invasion present information (DeepStateMap first), the military symbology standards behind "real" tactical displays, what Ukrainian military software is publicly known to look like, the visual language of drone/OSINT footage, and whether any of this data can legally be used by the game.

---

## 1. Overview

- **DeepStateMap is the reference.** It is Ukrainian, run by the DeepState UA NGO, fed largely by Ukrainian military sources, and deliberately published 2–3 days late for operational security [src:wikipedia-deepstatemap-live] [src:kyivindependent-2025-farrell-front-line-mapping]. Its visual grammar is simple. Semi-transparent area fills show control status (occupied, liberated, recently liberated, unknown/grey, pre-2022 occupied). Point icons mark Russian units, airfields and HQs, and arrows show "directions of attack" [src:deepstate-api-history]. Ukrainian positions are never shown [src:kyivpost-2025-ashcroft-inside-deepstate].
- **ISW/CTP is the Western reference.** Its legend is epistemic rather than just territorial. It separates *assessed* (evidence-backed) from *claimed* (asserted, unverified) and, since December 2025, draws a dedicated *infiltration areas* layer [src:isw-arcgis-control-of-terrain]. Most Western outlets (e.g., the BBC) redraw ISW/CTP data [src:bbc-2022-ukraine-in-maps].
- **The front line has stopped being a line.** From 2025 on, drone "kill zones" and infiltration by small Russian groups turned the grey zone from a thin no-man's-land into a deep contact zone with overlapping positions. That forced DeepState (2026) and ISW (Dec 2025) to redefine their uncertainty categories [src:onlineua-2026-deepstate-legend] [src:kyivindependent-2025-farrell-front-line-mapping] [src:warontherocks-2026-maurin-front-line]. This is directly relevant to eras e8-drone-kill-zone and e9-counteroffensive-26.
- **Mapping is political.** DeepState has clashed publicly with Ukrainian commanders over encirclements and breakthroughs (Makarivka, Dec 2024; Dobropillia, Aug 2025). Flag-planting photo-ops by both sides are an information-war tactic that careful mappers refuse to map [src:devua-2024-deepstate-makarivka] [src:kyivindependent-2025-farrell-front-line-mapping].
- **Tactical UIs are a different world.** Delta (MoD situational awareness), Kropyva (Army SOS artillery/mapping tablet app), and the "Vezha" drone-stream module with Avengers AI are the Ukrainian military's own digital layers [src:wikipedia-delta] [src:militarnyi-2022-delta-unveiled] [src:forbes-2022-hambling-kropyva]. The Army of Drones Bonus "e-points" system turned video-verified strikes into a points economy with unit leaderboards and a marketplace [src:kyivpost-2026-korshak-epoints] [src:defensenews-2026-ruitenberg-epoints] [src:united24-2026-petriv-army-of-drones].
- **One consistent territory metric exists.** DeepState's undocumented history endpoint serves the full GeoJSON of every map version since 3 April 2022. Differencing month-end versions reproduces DeepState's own published monthly figures to within a few km² in most months, and gives Russian gains and Ukrainian recaptures separately (§2.7) [src:deepstate-api-history] [src:euromaidan-2026-tril-2025-4336].
- **Data rights are the main blocker.** DeepState has no documented API or open licence, although undocumented JSON endpoints exist and community scrapers use them. ISW's ArcGIS layers are public but carry "exclusive intellectual property… written consent" terms. ACLED requires a corporate licence for commercial use [src:deepstate-api-history] [src:isw-arcgis-control-of-terrain] [src:acled-eula]. GeoConfirmed's free API is scoped to "research, journalism, and analytical use", and Liveuamap allows reuse "with reference" but sells its API [src:geoconfirmed-openapi] [src:liveuamap-about-terms]. Only the symbology renderer milsymbol is permissively (MIT) licensed [src:github-milsymbol].

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
The site's history endpoint listed **1,771 published map versions** between 3 April 2022 and 27 September 2026, and `/api/history/{id}/geojson` returns the complete map for any of them [src:deepstate-api-history]. The median gap between versions is about 24 hours: 420 versions in 2022, then about 360 a year, and 27–31 a month in 2026 [src:deepstate-api-history]. Each version carries a short changelog in Ukrainian and (from May 2022) English, with links to the affected settlements, e.g. "The Ukrainian Armed Forces liberated Ridkodub, Nove and Katerynivka…" (27 Sep 2026) [src:deepstate-api-history]. The live map is best understood as **one curated daily snapshot plus a news-style change log**, not a real-time feed.

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

### 2.7 Territory time series (km²)
DeepState publishes a monthly "Russia advanced X km²" figure, widely quoted by Ukrainian and Western media. No complete published series was found, so the series below was **computed** from DeepState's own map versions and checked against every published monthly figure that could be fetched.

**Method (computed, queried 2026-09-28).**
- Input: the last published map version of each month from `/api/history/public`, each downloaded via `/api/history/{id}/geojson` (55 versions, 3 Apr 2022 to 27 Sep 2026) [src:deepstate-api-history].
- Russian-held area = all polygons filled `#a52714` ("Occupied"), minus a fixed pre-2022 mask (ORDLO plus Crimea and Tuzla, 43,572 km², taken from the latest version). Everything is clipped to Ukraine's land: the geoBoundaries border intersected with Natural Earth 10 m land. Areas are measured in a Lambert azimuthal equal-area projection centred on Ukraine [src:geoboundaries-ukr-adm0] [src:naturalearth-10m-land].
- Russian gain = area newly red since the previous month-end; Ukrainian recapture = area red at the previous month-end but not now. A 150 m morphological opening removes redraw slivers (it changes monthly totals by under 5%). Net = change in total red area.
- Grey ("unknown status") is tracked separately and **not** counted as Russian.

**Validation.** DeepState's headline monthly number is the net change in red area. The computed net matches it closely:

| Month | DeepState published | Computed net |
|---|---|---|
| Oct 2024 | 490 [src:euromaidan-2024-mukhina-october-490] | 502 |
| Nov 2024 | 700+ [src:euromaidan-2025-shandra-gains-plummet] | 729 |
| Mar 2025 | 133 [src:euromaidan-2025-shandra-gains-plummet] | 134 |
| Sep 2025 | 259 [src:euromaidan-2025-mukhina-september-259] | 258 |
| Nov 2025 | "some 630" [src:euromaidan-2026-zoria-february-126] | 503 (unexplained gap) |
| Dec 2025 | 445 [src:euromaidan-2026-tril-cost-per-km] | 445 |
| Jan / Feb 2026 | 245 / 126 [src:euromaidan-2026-zoria-february-126] | 245 / 126 |
| May 2026 | 14 [src:euromaidan-2026-zoria-may-14] | 14 |
| Jul 2026 | 36 [src:kyivpost-2026-zavadska-deepstate-july]; 27 per Radio Svoboda [src:euromaidan-2026-tril-cost-per-km] | 36 |
| Year 2025 | 4,336 [src:euromaidan-2026-tril-2025-4336] | 4,336 |
| Year 2024 | 3,600+ (Mil.in.ua summary) [src:euromaidan-2025-hrudka-2024-3600] | 3,301 |

**Series** (km²; gain and recapture are sums of monthly gross changes; "grey" is the grey area at the end of the period, outside the pre-2022 zone):

| Period | Era | RU gain | UA recapture | Net RU | Grey at end | Notes |
|---|---|---|---|---|---|---|
| May–Jun 2022 | e2 | 4,614 | 1,643 | +2,985 | 723 | Popasna, Sievierodonetsk; computed |
| Jul–Sep 2022 | e2/e3 | 1,461 | 11,304 | −9,822 | 1,708 | Kharkiv counteroffensive in Sep (10,902 recaptured); computed |
| Oct–Dec 2022 | e3/e4 | 643 | 7,361 | −6,716 | 420 | Lyman, Kherson right bank (4,671 in Nov); computed |
| 2023 Q1 | e4 | 528 | 58 | +481 | 380 | Bakhmut, Vuhledar; computed |
| 2023 Q2 | e4/e5 | 77 | 350 | −275 | 381 | Counteroffensive starts in June; computed |
| 2023 Q3 | e5 | 138 | 294 | −177 | 358 | Robotyne; computed |
| 2023 Q4 | e5/e6 | 156 | 53 | +111 | 494 | Avdiivka; computed |
| 2024 Q1 | e6 | 211 | 18 | +198 | 538 | computed |
| 2024 Q2 | e6 | 542 | 13 | +536 | 626 | Vovchansk push in May (304); computed |
| Jul 2024 | e6 | 180 | 3 | +181 | 645 | computed |
| Aug 2024 | e7 | 364 | 4 | +363 | 654 | Pokrovsk axis; computed |
| Sep 2024 | e7 | 401 | 6 | +397 | 699 | computed |
| Oct 2024 | e7 | 501 | 1 | +502 | 732 | DeepState 490 |
| Nov 2024 | e7 | 728 | 1 | +729 | 740 | Peak month after 2022; DeepState 700+, ISW about 627 [src:euromaidan-2025-kravchuk-assaults-may] |
| Dec 2024 | e7 | 397 | 6 | +394 | 727 | computed |
| Jan 2025 | e7 | 322 | 1 | +326 | 740 | computed |
| Feb 2025 | e7 | 193 | 4 | +192 | 742 | computed |
| Mar 2025 | e7/e8 | 142 | 11 | +134 | 775 | DeepState 133, UK MoD 143, ISW about 203 |
| Apr 2025 | e8 | 188 | 5 | +187 | 867 | computed |
| May 2025 | e8 | 441 | 5 | +439 | 927 | computed |
| Jun 2025 | e8 | 550 | 0 | +556 | 976 | computed |
| Jul 2025 | e8 | 565 | 7 | +564 | 1,008 | computed |
| Aug 2025 | e8 | 472 | 12 | +464 | 982 | Dobropillia breach; computed |
| Sep 2025 | e8 | 278 | 23 | +258 | 1,190 | DeepState 259 |
| Oct 2025 | e8 | 300 | 36 | +267 | 1,296 | computed |
| Nov 2025 | e8 | 509 | 9 | +503 | 1,350 | DeepState "some 630" |
| Dec 2025 | e8 | 463 | 21 | +445 | 1,370 | DeepState 445 |
| Jan 2026 | e8 | 241 | 1 | +245 | 1,475 | DeepState 245 |
| Feb 2026 | e9 | 171 | 48 | +126 | 1,567 | Counteroffensive from 29 Jan–9 Feb (see 00 §3 e9); DeepState 126 |
| Mar 2026 | e9 | 201 | 45 | +160 | 1,544 | computed (Radio Svoboda gives 160 for April) |
| Apr 2026 | e9 | 151 | 13 | +141 | 1,550 | computed |
| May 2026 | e9 | 78 | 66 | +14 | 1,589 | DeepState 14; Kyiv Post quotes 130 lost and 250 regained [src:kyivpost-2026-zavadska-deepstate-july] |
| Jun 2026 | e9 | 85 | 2 | +84 | 1,635 | computed |
| Jul 2026 | e9 | 86 | 52 | +36 | 1,796 | DeepState 36; Ukrainian gains withheld for OPSEC |
| Aug 2026 | e9 | 132 | 127 | +5 | 1,688 | Vivaldi near Lyman; computed |
| 1–27 Sep 2026 | e9 | 142 | 90 | +54 | 1,627 | computed |

Per era (computed net Russian change): e2 (May–Aug 2022) +3,782; e3 (Sep–Nov 2022) −17,533, with 18,256 recaptured; e4 (Dec 2022–May 2023) +703; e5 (Jun–Nov 2023) −441; e6 (Dec 2023–Jul 2024) +989, about 124 a month; e7 (Aug 2024–Mar 2025) +3,037, about 380 a month; e8 (Apr 2025–Jan 2026) +3,928, about 393 a month; e9 (Feb–Sep 2026) +620, about 78 a month, against 444 km² recaptured. Russian-held territory including the pre-2022 zone rose from 107,553 km² (end 2022) to 116,195 km² (27 Sep 2026), about 19.3% of Ukraine; DeepState's own total at the end of 2025 was 116,165 km² against the computed 115,330 [src:euromaidan-2026-tril-2025-4336]. The routed Radio Svoboda figures (450–550 km² a month at the 2025 peak, 27 in July 2026) fit the series for May–Aug 2025 (439–564) but not for July 2026, where DeepState's own report and the computation both give 36 [src:euromaidan-2026-tril-cost-per-km].

**Caveats.**
- **Publication dates, not event dates.** Versions lag events by 2–3 days, and Ukrainian gains are sometimes released weeks later: DeepState's July 2026 report added 25 and 27 km² reclaimed in earlier months [src:kyivpost-2026-zavadska-deepstate-july], and AMK Mapping described a September 2026 DeepState release near Lyman as "OPSEC release part 3", covering advances "primarily... during May and June" [src:amk-mapping-control-map]. So e9 recaptures are pushed into Aug–Sep, and Ukrainian gains in May–July are understated.
- **April 2022 is unusable.** DeepState rebuilt the map after Google blocked it, and the 3 April version is incomplete (computed April "gain" of 15,376 km² is an artefact). Crimea and ORDLO were not drawn as polygons until mid-2022, hence the fixed pre-2022 mask.
- **Grey zone.** Red to grey counts as a Ukrainian recapture and grey to red as a Russian gain. Grey tripled from about 400 km² (2023) to 1,600–1,800 km² (mid-2026), so in e9 part of the "recapture" is reclassification. Counting red plus grey as Russian gives August 2026 a net Ukrainian gain of about 103 km².
- **Gross vs net.** Monthly gross sums exceed year-on-year differencing because of back-and-forth churn. For 2023, differencing gives 699 km² gained and 539 recaptured; DeepState's rounded 2023 figures were about 540 and 430 [src:euromaidan-2025-hrudka-2024-3600].
- **Other trackers differ by method.** ISW counted +104 km² of assessed Russian control plus 628 km² of infiltration zone for Jan–May 2026 [src:euromaidan-2026-zoria-may-14]. The Russian project Slivochny Kapriz, which credits any area where Russian troops were seen, gives 550–600 km² a month for 2025 [src:euromaidan-2026-tril-cost-per-km]. Ukrainian-held ground inside Russia (Kursk, 2024–25) is outside this metric.
- **Totals.** Coastline generalisation makes totals about 0.7% lower than DeepState's. Changes are unaffected.
- **Western comparison.** ISW put Russia's 2025 gains at about 4,700 km² against Russia's claimed 6,000 km² [src:bbc-2022-ukraine-in-maps]. The gap between claimed and assessed numbers is itself a UI motif.

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
| **ISW / CTP** | Institute for the Study of War and AEI Critical Threats Project (USA) | Static daily PNG maps in the ROCA plus an ArcGIS StoryMap/web map | Assessed Russian-controlled terrain, assessed Russian advances, assessed Russian infiltration areas, claimed Russian territory, claimed Ukrainian counteroffensives, pre-24 Feb 2022 occupied, control of terrain in Russia, Russian field fortifications; separate partisan layers | Daily (features stamped `pub_date` 2026-09-27); time-lapse archive monthly | Public ArcGIS Feature Services, but "exclusive intellectual property of ISW… written consent" | isw-arcgis-control-of-terrain, isw-2026-09-26-assessment |
| **Black Bird Group** | Finnish NGO (Finland) | Two-sided front lines plus movement arrows over an OpenFreeMap/OSM basemap; NATO red/blue | Russian-held territory, front line (Russian side), front line (Ukrainian side), RU movements, UA movements | "Updated regularly" (near daily); weekly Friday snapshot feeds ACLED | "Public product"; non-profit/public-facing adaptations welcomed; no formal licence text | blackbirdgroup-map |
| **UA Control Map** | Pseudonymous (unverified) | Geolocations, front lines, unit positions; clickable per-feature source links | Not verified | Not verified | Web "data viewer"; terms not verified | kyivindependent-2025-farrell-front-line-mapping |
| **Liveuamap** | Liveuamap, founded by two Dnipro software engineers in Feb 2014; now Liveuamap LLC (Virginia, USA) with EU-hosted infrastructure | Event-pin map fed by AI web crawlers, fact-checked by analysts, curated by editors; archive by day [src:wikipedia-liveuamap] [src:liveuamap-about-terms] | Pictogram per event type (shahed, bomb, drone, explosion, air defence, casualties, speech…) in a side colour; see notes | Continuous | Reuse of "data and maps… with reference to liveuamap.com"; paid API from $150/month | liveuamap-about-terms, liveuamap-promo-api, liveuamap-home-2026-09 |
| **GeoConfirmed** | Volunteer OSINT collective (international) | Every marker is one geolocated clip with sources and a public geolocation breakdown; timeline playback; ORBAT trees | 116 icons = object type × state (active / recently destroyed / older destroyed), coloured by faction; filterable by date, equipment, unit, faction | Continuous (883 new Ukraine events in the 30 days before fetch) | Unauthenticated read API with KMZ/CSV/GeoJSON export; "freely available for research, journalism, and analytical use"; no licence text | geoconfirmed-home, geoconfirmed-openapi |
| **ACLED Ukraine Conflict Monitor** | ACLED (international NGO) | Scaled event circles plus oblast choropleth; front line from Black Bird Group | Political-violence event types; infrastructure tags (energy, health, education, residential); civilian fatalities (conservative) | Weekly situation updates | Registration; EULA: non-commercial by default, corporate licence for businesses, no raw redistribution, attribution mandatory | acled-ukraine-conflict-monitor, acled-eula |
| **BBC "Ukraine in maps"** | BBC Visual Journalism (UK) | Explainer maps with narrative text | Redraws ISW/CTP daily assessments of Russian control | Periodic (first published 24 Feb 2022, last updated Sep 2026) | © BBC | bbc-2022-ukraine-in-maps |
| **NYT "Maps: Tracking the Russian Invasion of Ukraine"** | New York Times (USA) | Long-running interactive, updated through the war [src:nyt-2022-ukraine-maps] | Not verified (fetch blocked) | Periodic | © NYT | nyt-2022-ukraine-maps |
| **Suriyak Maps** | Anonymous X/Telegram account (since March 2017), apparently Spanish-speaking; also maps Syria, Yemen, Libya, the Sahel and Gaza | Static map images posted to X (110k followers) and Telegram (47k) | Control plus war-zone / grey / unknown bands (per Wikipedia editors); legend not verified | Frequent posts | None stated; social-media images | suriyak-telegram, wikipedia-rsn-2024-suriyakmaps |
| **Andrew Perpetua** | Andrew Perpetua with @Gaspo_sk ("UA map", ukrdailyupdate.com) | Leaflet web map built on geolocated footage; daily date slider, measuring tools, clustered event badges | Event badges by type (drone, Shahed, Lancet, helicopter, TOS, flag, death, explosion, abandoned…) in dark blue, dark red, gold and purple | Daily (latest map date 26 Sep 2026) | No licence; asks for screenshots rather than embedding | perpetua-ukrdailyupdate-map |
| **AMK Mapping** | Pseudonymous analyst (X, Telegram with 60k subscribers); covers Ukraine and the Middle East | Google My Maps with toggleable layers; publicly exportable KML | Confirmed Russian control (brick red), confirmed Ukrainian control (indigo), infiltration zone (black), confirmed Russian advances (yellow), confirmed Ukrainian advances (light blue), direction definitions, points of interest, "Never forget" | Near daily (last update 28 Sep 2026) | No licence; Google My Maps terms | amk-mapping-control-map |

Notes:
- **Assessed vs claimed.** ISW's two epistemic tiers ("assessed" = evidence-backed, "claimed" = asserted but unverified) are the clearest model of *confidence as a map layer*. In the web-map JSON:
  - Russian-controlled terrain and Russian advances share a translucent red fill (RGBA 239,0,0,71) with a red outline.
  - Infiltration areas are a patterned picture-fill with a **dashed** red outline.
  - Claimed Russian territory is translucent orange (255,170,0,126).
  - Claimed Ukrainian counteroffensives are translucent light blue (115,223,255,147) with a blue outline [src:isw-arcgis-control-of-terrain].
  - The infiltration feature service was created on 20 December 2025 (e8) [src:isw-arcgis-control-of-terrain].
- **ISW methodology:**
  - Each daily ROCA pairs a theatre map with per-sector zoom maps. The text runs axis by axis, each with a stated "Russian objective" and the "assessed objective" of named Russian armies. Control changes are backed by "geolocated footage published on" a given date, and every report ends with the note that ISW "uses only publicly available information" and receives no classified material [src:isw-2026-09-26-assessment].
  - Earlier ROCAs (seen only in search excerpts in pass 1) state that the map shows "the furthest assessed extent of Russian advances" until open-source evidence shows otherwise, that this may underestimate Ukrainian advances, and that a porous front complicates control of terrain.
- **Partisans.** ISW keeps two partisan products: "reported but not confirmed Ukrainian partisan warfare" (feature service, 2022–) and a curated "Verified Ukrainian Partisan Attacks" StoryMap that only includes high-confidence events [src:isw-arcgis-control-of-terrain].
- **Reputation of the one-person maps** is contested and only documented in Wikipedia editors' discussions. Suriyak was ruled unreliable for anonymity and unclear method (2021), then split editors between "partisan" and "very neutral… among the more conservative sources" (2024). One editor's 2024 ranking put ISW first, DeepState and Andrew Perpetua tied second (Perpetua "mostly based on geolocated footage… a bit cautious in reporting Russian gains"), and Suriyak, MilitaryLand, Rybar and Liveuamap lower. [src:wikipedia-rsn-2024-suriyakmaps] [src:wikipedia-talk-2024-territorial-control-mappers]. Perpetua's page carries a fundraiser for Ukraine's Khartiia corps [src:perpetua-ukrdailyupdate-map]. Treat all three as low-to-medium, single-team sources.
- **AMK's legend is a close relative of DeepState's.** It uses the same Google-palette reds and blues (`#A52714` for Russian control, `#0288D1` for Ukrainian advances) and adds explicit *advance* layers and a black infiltration zone. That is the "change this period" layer DeepState only implies with its two-week blue [src:amk-mapping-control-map].
- **Liveuamap icon semantics.** The site does not document its legend. In a captured feed, each event carries a code of the form pictogram-colour index: Russian strikes are `shahed-1`, `bomb-1` and `destroy-1`; Ukrainian drone strikes on Russia, explosions in Kursk, Ukrainian air defence, casualties in Ukraine and Zelensky statements are `drone-2`, `explode-2`, `aa-2`, `medicine-2` and `speech-2`; a US Congress item is `speech-10`. The page loads red, blue, green, white and black icon sets. The reading "1 = red = Russian, 2 = blue = Ukrainian" is an inference from 19 events [src:liveuamap-home-2026-09].
- **GeoConfirmed's icon grammar** is the most game-ready. Every object type has three states: active, "recently destroyed/damaged [visible in footage]" and "older destroyed". Colour is the faction *doing* the act ("colour of who is shelling"; a killed civilian is drawn "in the colour of who is assessed as doing the attack"; brown = UXO). Types range from tanks, APCs and trucks to suicide and multipurpose UGVs, short- and long-range one-way attack drones, interceptions, trenches, mines and CBRN incidents. Faction colours for Ukraine: Ukraine `#0051CA`, Ukraine civilian `#3184FF`, Russia `#E00000`, Russia civilian `#FF6666`, North Korea `#6A1D00`, Belarus `#400080` [src:geoconfirmed-openapi].
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
    - NATO exercises: at CWIX 2023 it exchanged data over **Link 16** and integrated with 15 systems of 10 countries [src:militarnyi-2023-delta-link16]. At CWIX 2024 it linked to Poland's TOPAZ artillery fire-control system [src:euromaidan-2024-looijen-delta-cwix]. At REPMUS 24 (Portugal, September 2024) it coordinated 50+ uncrewed underwater, surface, land and air vehicles, with "separation of own and enemy forces" [src:militarnyi-2024-delta-repmus].
  - **What the UI is, in words.** Officials call it "Google for the military, because after one login you get access to different tools" [src:euromaidan-2024-looijen-delta-cwix]. The published modules are [src:euromaidan-2025-zoria-delta-scales]:
    - **Deltamonitor:** a live digital map of friendly and enemy positions.
    - **Target Hub:** targets are created, assigned to a "performer" unit and tracked directly on the map.
    - **Vezha:** drone video streamed to command centres. MoD photos show a control-room wall of live feeds.
    - **Mission Control:** UAV responsibility zones and flight paths, linking drone operators to higher HQs.
    - Secure chat.
  - The MoD claims Delta supports targeting of 2,000+ enemy assets a day [src:euromaidan-2025-zoria-delta-scales]. No source describes its symbol set. "Built according to NATO standards" [src:militarnyi-2022-delta-unveiled] and Link 16 interoperability suggest APP-6-compatible symbology, but that is an inference.
- **Vezha and Avengers (drone "streams"):**
  - Vezha is Delta's video-stream module. The Avengers AI running inside it auto-detects and classifies enemy vehicles in drone and stationary-camera feeds: about 12,000 targets a week, about 70% of visible equipment, about 2.2 s per detection [src:wikipedia-delta].
  - It was shown at NATO TIDE Sprint (Helsinki, February 2025) and the London Defence Conference (May 2025) [src:wikipedia-delta].
  - This is the backbone of the "streams" wall that Ukrainian HQs watch [src:euromaidan-2025-zoria-delta-scales].
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
- **Russia's side as reported.** Photos from Russian command posts show **ASTRAS**, apparently a domestic replacement for Discord after Russia blocked it. Its interface looks "similar to civilian instant messengers" (text, voice, possibly files), which is a chat tool rather than a map-centric system like Delta [src:militarnyi-2025-astras].

---

## 6. Visual language of drone and OSINT footage (UI inspiration)

- **FPV on-screen display (OSD):**
  - The blocky white telemetry text over FPV strike clips comes from flight-controller OSDs such as Betaflight's. Elements sit on a **character grid**: 30×16 for PAL and 30×13 for NTSC on MAX7456 analogue chips, and a default 53×20 for HD digital systems. Fonts are uploaded as glyph sets [src:betaflight-osd-tab].
  - Standard elements include timers, alarms, warnings and a post-flight statistics screen [src:betaflight-osd-tab].
  - A game HUD built on a monospace glyph grid reads instantly as "drone feed."
- **Signal breakup:** analogue static and freeze at impact or under EW is the other signature of FPV footage. This is an observation, not sourced here.
- **Thermal:** night drone and sight footage is usually monochrome "white-hot" (hot = white) or "black-hot" (hot = black). The DJI Mavic 3T, a commercial thermal drone, records thermal video at 640×512 and 30 fps and offers White Hot, Black Hot, Tint, Iron Red, Hot Iron, Arctic, Medical, Fulgurite and two Rainbow palettes [src:dji-mavic3-enterprise-specs]. "Ironbow" is FLIR's name for the iron-red family. Which palette Ukrainian units prefer is not sourced; the monochrome modes dominate published strike clips (observation).
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
    - `…/api/history/{id}/geojson`: the full map of any listed version (the basis of §2.7).
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
- **GeoConfirmed:**
  - The API spec states that read endpoints need no authentication and that the data is "freely available for research, journalism, and analytical use". It offers per-conflict KMZ, CSV and GeoJSON exports.
  - There is no licence field and no mention of commercial use. It asks for a descriptive User-Agent and fair use, and to "get in touch" for bulk or near-real-time needs [src:geoconfirmed-openapi].
  - For a commercial game, ask. Its icon *grammar* (type × state × actor colour) is an idea and can be reimplemented freely.
- **Liveuamap:** the terms (Jan 2023 snapshot) allow use of "our data and maps, including map tiles and areas polygons partially, or in the whole in your work, with reference to liveuamap.com". Third-party images and text stay under their platforms' terms [src:liveuamap-about-terms]. The API is paid: Pro at $150/month for 200 requests/day, Enterprise from $1,000/month [src:liveuamap-promo-api].
- **One-person maps:** AMK Mapping's Google My Map exports as KML [src:amk-mapping-control-map], and Andrew Perpetua's map asks for screenshots rather than embeds [src:perpetua-ukrdailyupdate-map]. Neither states a licence, so treat both as reference only. Suriyak publishes images [src:suriyak-telegram].
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
- **Calibrate front movement with §2.7.** Typical monthly net Russian gains are about 120 km² (e6), about 380–390 km² (e7–e8, peaking at about 730 in November 2024) and under 100 km² (e9), with Ukrainian counteroffensives recapturing 4,700–10,900 km² in a single month in autumn 2022. The grey zone's area roughly tripled between 2023 and 2026 [src:deepstate-api-history] [src:euromaidan-2026-tril-2025-4336].
- **Statistics screen.** A monthly km² gained/lost panel, as in DeepState's monthly reports [src:kyivpost-2026-zavadska-deepstate-july], showing *gross* gains and recaptures, which DeepState's single net number hides. Add "claimed vs assessed" comparisons [src:bbc-2022-ukraine-in-maps] and a note when figures are revised, since trackers differ by method (DeepState, ISW and Slivochny Kapriz give different numbers for the same months) [src:euromaidan-2026-zoria-may-14] [src:euromaidan-2026-tril-cost-per-km].
- **Event icons with state and actor.** Borrow GeoConfirmed's grammar: each map object is active, recently destroyed or older destroyed, and coloured by the side that acted. "Recent" vs "older" doubles as a fog-of-war decay timer [src:geoconfirmed-openapi]. AMK-style "advances this period" layers show change directly [src:amk-mapping-control-map]. An optional e-points-style results ledger would show video-verified strikes, a points economy and a Brave1-like shop [src:kyivpost-2026-korshak-epoints] [src:defensenews-2026-ruitenberg-epoints]. Handle the leaderboard with care: it scores human casualties.
- **Drone-feed panels.** Picture-in-picture "Vezha" streams with a character-grid OSD, white-hot/black-hot thermal (640×512 is a realistic sensor resolution) and AI detection tags, as a diegetic way to deliver intel [src:betaflight-osd-tab] [src:wikipedia-delta] [src:dji-mavic3-enterprise-specs].
- **A Delta-like command UI.** Split the player's tools the way Delta does: a situational map (Deltamonitor), a target list with task assignment (Target Hub), a stream wall (Vezha) and drone airspace deconfliction (Mission Control) [src:euromaidan-2025-zoria-delta-scales]. The Russian side, as reported, works through a messenger-style tool (ASTRAS), which could justify slower, chattier enemy C2 [src:militarnyi-2025-astras].
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
- **net change (DeepState monthly figure)** — Change in total Russian-occupied area over a month. It hides gross gains and recaptures, which §2.7 separates.
- **OPSEC release** — A delayed batch of Ukrainian gains published on DeepState after an operation's security window has passed.
- **CoT** — Control of terrain, ISW's term for the territorial layer.
- **flag operation** — Planting a flag in a contested settlement for a photo, to create an impression of capture.
- **OSGT (ОСУВ)** — Operational-strategic group of troops (e.g. Khortytsia, later Dnipro), whose statements sometimes contradict mappers.
- **APP-6 / MIL-STD-2525** — NATO and US military symbology standards.
- **SIDC** — Symbol identification code, the letter or numeric string that fully specifies a military symbol.
- **amplifier / modifier** — Text or graphic fields around or inside an APP-6 frame (designation, echelon, HQ staff, etc.).
- **echelon** — Unit size indicator (squad … brigade … army) drawn above the frame.
- **Delta (Дельта)** — MoD situational-awareness and battle-management system.
- **Deltamonitor / Target Hub / Mission Control** — Delta modules for the live map, strike-task assignment, and UAV zones and flight paths.
- **ASTRAS** — Russian messenger-style command-post software, reported as a Delta analogue.
- **Vezha (Вежа)** — Delta's video-stream module.
- **Avengers** — MoD AI for automatic detection in drone video.
- **Kropyva (Кропива)** — Army SOS tablet app for mapping and artillery fire calculation.
- **e-points (е-бали) / Army of Drones Bonus** — MoD programme awarding points for video-verified strikes, spendable on the Brave1 marketplace.
- **Brave1 Market** — Ukrainian government defence-tech marketplace where units buy equipment.
- **OSD** — On-screen display, the telemetry overlay on an FPV video feed.
- **white-hot / black-hot** — Monochrome thermal palettes where hotter objects appear white or black. Colour palettes include iron red (FLIR "ironbow"), arctic and rainbow.
- **geolocation** — Establishing where footage was shot by matching features to maps or satellite imagery.

---

## 10. Open questions and gaps

- **ISW's methodology page** still returns 403, and the Wayback copy of the 26 Sep 2026 ROCA does not repeat the "furthest assessed extent" sentence. That wording rests on pass-1 search excerpts. ISW's historic 2022 legend (e.g., "Ukrainian counteroffensives" on static maps) was not checked visually.
- **November 2025 and 2024-total mismatches.** DeepState was quoted at "some 630" km² for November 2025 and 3,600+ km² for 2024, against 503 and 3,301 computed. May 2026 also has conflicting DeepState quotations (14 net per RFE/RL; 130 lost and 250 regained per Kyiv Post). DeepState's own Telegram monthly posts, which would settle these, were not fetched.
- **Event-dated series.** The computed series follows publication dates. Reassigning OPSEC-delayed Ukrainian gains (e.g., Vivaldi, May–June 2026) to their event months would need DeepState's changelog text parsed per settlement.
- **Suriyak's legend and cadence** are known only from editors' descriptions. Its maps are images on X and Telegram (X was not fetchable). Andrew Perpetua's map data file did not load outside the web app, so its polygon legend (as opposed to its event badges) is unverified.
- **Liveuamap's legend** is not documented by the site. The side-colour reading comes from 19 captured events, and the live site blocks fetches.
- **UA Control Map** legend and terms are unverified (the site returned only a 226-byte redirect stub).
- **NYT "Maps: Tracking the Russian Invasion"** is blocked directly and through the Wayback Machine. Its methodology is unknown. The Kyiv Independent map pages were not fetched.
- **DeepState's official terms of use** were not found. It is unclear whether the `/api/history/*` endpoints are tolerated for third-party use, so ask before any bulk use.
- **Delta's visual design** (basemap, symbol set, colours) is not described in any fetched source. The NATO ACT "DELTA at CWIX24" article is still behind a Cloudflare challenge.
- **Thermal and FPV visuals.** No source was found on which palette Ukrainian units prefer, or on analogue "signal loss" artefacts under EW. FLIR's palette guides are behind Cloudflare.
- **GeoConfirmed commercial use** is not addressed by its API terms. Written permission is needed before shipping any of its data.
- **Date conflict:** the Makarivka encirclement dispute is dated December 2024 by dev.ua. A Wikipedia summary placed it in December 2023. dev.ua's date is used.

---

## 11. Sources

- `deepstate-map-live` — Live DeepStateMap page; page metadata and tech stack (Leaflet/PixiJS/Highcharts/fabric.js).
- `deepstate-api-history` — DeepState's undocumented JSON history endpoints; the legend categories, stated colours, icon types, 1,771-version update log and per-version GeoJSON used for the §2.7 series.
- `wikipedia-deepstatemap-live` — Encyclopedic history: founders, Google Maps block, colours, MoD memorandum, controversies, apps.
- `kyivpost-2025-ashcroft-inside-deepstate` — Profile of the team: sourcing from the military, OPSEC, vetting, colour code, audience.
- `kyivindependent-2025-farrell-front-line-mapping` — How the blurred front makes mapping harder and political; grey zone evolution; Dobropillia; other mappers.
- `onlineua-2026-deepstate-legend` — Ukrainian explainer of DeepState's legend and its 2026 grey-zone redefinition.
- `kyivpost-2026-zavadska-deepstate-july` — DeepState's July 2026 km² statistics, with the May 2026 comparison.
- `euromaidan-2024-mukhina-october-490` — DeepState: 490 km² in October 2024.
- `euromaidan-2025-hrudka-2024-3600` — DeepState via Mil.in.ua: 3,600+ km² lost in 2024; 2023 about 540 lost, 430 regained.
- `euromaidan-2025-shandra-gains-plummet` — DeepState: 700+ km² (Nov 2024), 133 km² (Mar 2025); UK MoD 143.
- `euromaidan-2025-kravchuk-assaults-may` — ISW: about 627 km² (Nov 2024), about 203 km² (Mar 2025).
- `euromaidan-2025-mukhina-september-259` — DeepState: 259 km² in September 2025.
- `euromaidan-2026-tril-2025-4336` — DeepState: 4,336 km² in 2025; 116,165 km² (19.25%) occupied.
- `euromaidan-2026-zoria-february-126` — DeepState: 126 (Feb 2026), 245 (Jan 2026), "some 630" (Nov 2025).
- `euromaidan-2026-zoria-may-14` — DeepState: 14 km² in May 2026; ISW assessed control vs infiltration zone.
- `euromaidan-2026-tril-cost-per-km` — Radio Svoboda on DeepState: 450–550 peak, 445, 160, 27; Slivochny Kapriz comparison.
- `geoboundaries-ukr-adm0` — Ukraine border (ODbL) used in the km² computation.
- `naturalearth-10m-land` — Public-domain land mask used in the km² computation.
- `devua-2024-deepstate-makarivka` — Makarivka encirclement dispute and alleged pressure on DeepState (Dec 2024).
- `github-cyterat-deepstate-map-data` — Community daily GeoJSON of DeepState occupied areas (GPL-3.0 code).
- `isw-arcgis-control-of-terrain` — ISW's own ArcGIS web map: legend layers, symbology, daily pub_date, licence text.
- `isw-2026-09-26-assessment` — Representative ISW/CTP daily Russian Offensive Campaign Assessment (read via Wayback): structure, evidence standard, sources note.
- `blackbirdgroup-map` — Black Bird Group's war map: legend, NATO red/blue convention, usage stance.
- `acled-ukraine-conflict-monitor` — ACLED monitor: event map, weekly updates, Black Bird front line.
- `acled-eula` — ACLED licence terms: non-commercial default, corporate licence, attribution.
- `geoconfirmed-home` — GeoConfirmed: verified geolocation markers, ORBATs, timeline, public API.
- `geoconfirmed-openapi` — GeoConfirmed API spec: unauthenticated exports, research-use scope, faction colours, icon catalogue.
- `wikipedia-liveuamap` — Liveuamap's origin (Dnipro, 2014) and algorithm-plus-human verification model.
- `liveuamap-about-terms` — Liveuamap about page and terms (Wayback): governance, method, reuse "with reference".
- `liveuamap-promo-api` — Liveuamap paid API tiers (Wayback).
- `liveuamap-home-2026-09` — Liveuamap feed snapshot (Wayback): pictogram-colour icon codes.
- `amk-mapping-control-map` — AMK Mapping's Google My Map: layers, colours, KML export; Telegram channel.
- `suriyak-telegram` — Suriyakmaps Telegram channel: scope, audience, image-based output.
- `wikipedia-rsn-2024-suriyakmaps` — Wikipedia reliability discussions of SuriyakMaps (2021, 2024).
- `perpetua-ukrdailyupdate-map` — Andrew Perpetua's "UA map": Leaflet, date slider, event badges.
- `wikipedia-talk-2024-territorial-control-mappers` — Wikipedia editors' 2024 comparison of war maps.
- `bbc-2022-ukraine-in-maps` — BBC explainer maps built on ISW/CTP data; ISW 2025 km² figure.
- `nyt-2022-ukraine-maps` — NYT long-running interactive map (search-only; fetch blocked).
- `wikipedia-nato-joint-military-symbology` — APP-6/MIL-STD-2525 overview: frames, colours, status, dimensions.
- `github-milsymbol` — milsymbol JS renderer (MIT): supported standards, SIDC schemes, echelon list.
- `spatialillusions-battle-staff-tools` — Spatial Illusions unit generator: features and proprietary licence model.
- `wikipedia-delta` — Delta system history, Vezha/Avengers video-AI module.
- `militarnyi-2022-delta-unveiled` — Delta's public unveiling at NATO TIDE Sprint 2022; NATO standards.
- `militarnyi-2023-delta-link16` — Delta exchanges data over Link 16 at CWIX 2023.
- `euromaidan-2024-looijen-delta-cwix` — Delta at CWIX 2024 (TOPAZ link); "Google for the military".
- `militarnyi-2024-delta-repmus` — Delta coordinates 50+ uncrewed vehicles at REPMUS 24.
- `euromaidan-2025-zoria-delta-scales` — Delta modules: Deltamonitor, Target Hub, Vezha, Mission Control.
- `militarnyi-2025-astras` — Russian ASTRAS messenger-style command-post software.
- `forbes-2022-hambling-kropyva` — Kropyva (Army SOS) tablet mapping and fire-control software.
- `kyivpost-2026-korshak-epoints` — Army of Drones Bonus e-points price list and video verification.
- `defensenews-2026-ruitenberg-epoints` — e-points spendable on the Brave1 marketplace; 2025 strike totals.
- `united24-2026-petriv-army-of-drones` — 2025 top-10 unit leaderboard; 2026 e-points expansion.
- `betaflight-osd-tab` — Betaflight OSD documentation: character-grid HUD on FPV feeds.
- `dji-mavic3-enterprise-specs` — DJI Mavic 3T thermal resolution and palette list.
- `warontherocks-2026-maurin-front-line` — Analysis of the front "thickening" into a deep contested zone.
