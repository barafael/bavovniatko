# 06 — Terrain and geodata

Research pack for **bavovniatko**, covering the procedurally generated front-line landscape of eastern and southern Ukraine. Accessed 2026-09-28. Citations use the form `[src:<id>]` and resolve to `sources/06-terrain-geodata.yaml`. Era ids follow the canonical list (e1–e9).

## 1. Overview

The front from Kharkiv to Kherson is mostly open steppe and forest-steppe farmland on chernozem (chornozem, "black earth"). Large fields are separated by planted windbreak belts, called *lisosmuhy* in Ukrainian (sing. *lisosmuha*) and *lesopolosy* in Russian [src:eou-shelterbelt] [src:cepa-2023-kallberg-treelines]. Natural woodland is scarce and occurs "mainly along rivers banks and in gullies" (balkas) [src:tarasov-usda-luhansk-wind-erosion]. The exceptions are the sandy pine forests on the left-bank terraces of the Siverskyi Donets (Lyman, Kreminna and Serebrianka) [src:wikipedia-siverskyi-donets] [src:kyivindependent-2022-ponomarenko-donbas-woods]. The Donbas adds mining towns with terrikons (spoil heaps) 40 m to over 100 m high, coke and chemical plants, and Soviet apartment blocks [src:euromaidan-2023-havrylets-avdiivka] [src:euromaidan-2024-toretsk-highrises].

On this ground the treeline is the basic tactical unit. It provides cover, a firing position, an infiltration corridor and an artillery target. Its value swings with the seasons: foliage in summer, bare branches and snow in winter, and mud (rasputitsa, Ukrainian *bezdorizhzhia*) in spring and autumn [src:cepa-2023-kallberg-treelines] [src:rferl-2026-lohinov-spring-battlefield] [src:united24-2026-brizard-winter-camouflage] [src:spire-2025-soil-moisture]. In e8–e9 drone saturation has widened the contested zone from about 1 km to several km, and survival now depends on concealment more than on firepower [src:wotr-2026-maurin-front-line].

For procedural generation, the landscape is regular. It consists of a rectilinear Soviet-era field and belt grid laid over a dendritic network of gullies and river valleys, with villages strung along the valleys. Several open datasets can calibrate this (OSM, Copernicus DEM, ESA WorldCover, Sentinel-2 and Dynamic World) (§5).

## 2. Landscape elements

### 2.1 Shelterbelts (lisosmuhy)

**Origin.** Ukrainian shelterbelts date from 1809, when Vasyl Lomykovsky planted trees around fields in Poltava gubernia. They were studied by Dokuchaev and Vysotsky in the 1890s and planted systematically in 1938–39 and 1948–52 [src:eou-shelterbelt]. The 1948 "Great Stalin Plan for the Transformation of Nature" drove the largest wave. By a Ukrainian NGO's count, 440,000 ha of field-protective belts were planted in the ten years from 1950, and another 154,500 ha in the 20 years from 1971 [src:uncg-2022-shamina-stalin-plan]. Ukraine had about 400,000 ha of shelterbelts in the early 1990s [src:unfccc-ukr-agroforestry-brief]. After the collective farms were dissolved, many belts ended up with unclear ownership and outside the land cadastre, and they degraded. The government's 2020 rules and the national Technology Needs Assessment target reconstruction of about 85,000 ha by 2030 [src:unfccc-ukr-agroforestry-brief]. The front-line belts in e1–e9 are therefore mostly 50–75-year-old Soviet plantings, often gappy or fragmented.

**Dimensions.**
- Width. A studied Left-Bank forest-steppe oak windbreak is **15 m** wide, planted as oak "nests" at 5 × 3 m with 5 m alleys and companion and shrub rows [src:maliuha-2023-windbreaks]. A Luhansk soil-erosion study found about **10 m** to be the optimal width for trapping blown soil [src:tarasov-usda-luhansk-wind-erosion]. The TNA brief describes belts as mixed species "in a few rows" [src:unfccc-ukr-agroforestry-brief].
- Height. Mature belts reach **19.6–23.4 m** [src:maliuha-2023-windbreaks].
- Spacing. In the Oleshky district of Kherson oblast, the distance between belts is **350–780 m, averaging 550 m** (search-only) [src:ffm-oleshky-shelterbelts].
- Orientation. In Luhansk oblast the main harmful (deflation) winds come from the east [src:tarasov-usda-luhansk-wind-erosion]. Main belts are laid across the prevailing wind, so they tend to run roughly north–south there. Orientation elsewhere should come from OSM rather than be assumed (§10).
- Species. Oak, pine, birch and poplar form the tall layer, with linden, maple, apple and pear lower down [src:eou-shelterbelt]. Oak-led belts with companion trees and a shrub understorey are common [src:maliuha-2023-windbreaks].

**Tactical role.**
- Positions and reserves. Treelines hold forward trenches, while reserves, artillery and logistics sit 8–15 km back in treelines, woods and towns [src:cepa-2023-kallberg-treelines].
- Infiltration corridors. Russian small-group assaults follow the shelterbelts, a pattern analysed in a 2026 report by the Ukrainian soldier-analyst Hlib Parfonov [src:saratoga-2026-parfonov-treelines]. In spring 2026 Russian forces tried to use new foliage to move infantry along treelines on foot, on motorcycles or on quad bikes [src:rferl-2026-lohinov-spring-battlefield].
- Survival. Near Pokrovsk in winter 2024–25, an exposed 80 × 30 m trench cut into the 1 km-long CHAI treeline was destroyed. Two concealed fighting holes in the undergrowth held for 10 days [src:wotr-2026-maurin-front-line].
- Foliage. Leaves protected positions from drones and precision fires in summer 2023. Autumn leaf-fall and then snow exposed them [src:cepa-2023-kallberg-treelines].
- Being shredded. Sustained barrages reduced some treelines to "piles of split wood and tree branches" [src:cepa-2023-kallberg-treelines].

### 2.2 Fields

- **Geometry.** The southern battlefield is "flat and open ... large fields are separated by treelines" [src:cepa-2023-kallberg-treelines]. With belts 350–780 m apart [src:ffm-oleshky-shelterbelts] and treelines about 1 km long [src:wotr-2026-maurin-front-line], a typical block is roughly **0.5 km × 1 km or more, about 40–100+ ha**. This is a *derived estimate* and still needs calibrating (§10).
- **Size class.** That puts the typical block in the "large" (16–100 ha) and "very large" (>100 ha) classes of the global field-size map. Large fields dominate post-Soviet countries (per a search summary of the study) [src:lesiv-2019-field-size].
- **National count.** Ukraine has more than 5 million fields of at least 1 ha (2021–2023) [src:sadeh-2025-ukraine-field-boundaries].
- **Abandonment.** Fields near the front are often abandoned and hard to delineate, and in the game they should become weedy fallow [src:sadeh-2025-ukraine-field-boundaries].
- **Tactical meaning.** Open fields are kill zones watched by drones. Movement happens along or through the belts, and crossing a field between belts is the dangerous phase [src:wotr-2026-maurin-front-line] [src:rferl-2026-lohinov-spring-battlefield].
- **Roads.** Dirt tracks run beside the belts, and paved roads become the only routes for heavy vehicles in mud season [src:spire-2025-soil-moisture]. Field tracks alongside belts are a design assumption to verify in OSM (`highway=track` next to `natural=tree_row`).

### 2.3 Landforms

- **Balkas and yary (gullies and ravines).** Balkas and yary cut the steppe interfluves. They host most of the natural woodland [src:tarasov-usda-luhansk-wind-erosion] and dissect the high right bank of the Siverskyi Donets [src:wikipedia-siverskyi-donets]. For the game they are covered approach routes and natural anti-vehicle obstacles. No dedicated military source on balka fighting was retrieved (§10).
- **Chalk hills.** The right bank of the Siverskyi Donets is "usually high, sometimes with chalk cliffs" (the Sviatohirsk / Holy Mountains area). The left bank is low, with swamps, lakes and oxbows [src:wikipedia-siverskyi-donets]. This asymmetry means the high bank dominates and the low bank floods.
- **Pine forests on sand.** The left-bank terraces carry pine forest [src:wikipedia-siverskyi-donets]. Near Lyman in 2022 the forest positions were sandy trenches, with the enemy about 300 m away behind tall pines [src:kyivindependent-2022-ponomarenko-donbas-woods] (e2).
- **Serebrianka forest.** The Serebriansky forest SW of Kreminna mixes forest, swamp and meadow. The protected reserve covers 107.1 ha within a much larger pine forest. It was on the front from autumn 2022 until it fell by December 2025, according to Wikipedia [src:wikipedia-serebriansky-forest] (e3–e8).
- **Terrikons (terykony).** These are conical coal-waste heaps and act as artificial high ground. The Avdiivka terrikon is about 40 m high [src:euromaidan-2023-havrylets-avdiivka]. Its defenders "saw every Russian move from the heights", and the steep slope stopped assaults [src:euromaidan-2023-avdiivka-spoil-tip] (e6). Toretsk has two waste dumps over 100 m high [src:euromaidan-2024-toretsk-highrises] (e7), and terrikons also shaped fire control around Pokrovsk and Selydove [src:wikipedia-battle-of-pokrovsk].
- **Flat steppe (Zaporizhzhia).** The terrain is flat to gently rolling, so observation lines are long [src:cepa-2023-kallberg-treelines] (e5).

### 2.4 Water

- **Siverskyi Donets.** The river is 1,053 km long, about 950 km of it in Ukraine. It has an asymmetric valley and a wet floodplain with willow, birch and alder. Russian crossing attempts above Lysychansk failed in May 2022 [src:wikipedia-siverskyi-donets] (e2). Other named front rivers include the Vovcha and the Kazennyi Torets near Pokrovsk [src:wikipedia-battle-of-pokrovsk]. The Oskil, Mokri Yaly and Kinska were not source-covered (§10).
- **Kakhovka reservoir (e5 onward).** When the dam was destroyed on 6 June 2023, the reservoir (created by flooding 709,900 ha, including the Velykyi Luh "Great Meadow" of up to 80,000 ha) drained [src:uncg-2022-shamina-stalin-plan]. Snowmelt in March 2024 partly re-flooded it. The bed became soft ground that "no equipment, including amphibious vehicles" can cross [src:defenceexpress-2024-kakhovka-refill]. By May 2026 more than 2,000 km² of the old bed held a young willow-dominated forest. The old reservoir was 3.6 km wide at Kamianska Sich and 13.5 km at Novovorontsovka [src:uanimals-2026-kakhovka-forest].
- **In the game.** The Kakhovka area is a terrain state that changes with era: open water, then mudflat (e5), then wetland and scrub (e6–e7), then willow forest (e8–e9). The Dnipro stays a hard barrier throughout.

### 2.5 Settlements

- **Private-sector houses.** Single-storey private houses with gardens make up districts such as Zabalka in Toretsk, which sits "in unfavorable terrain" below the waste heaps [src:euromaidan-2024-toretsk-highrises].
- **Resorts and cottages.** Holiday resorts, children's camps and cottages in the Donbas pine woods served as positions [src:kyivindependent-2022-ponomarenko-donbas-woods]. They can stand in for dacha settlements.
- **Khrushchevky.** These Soviet blocks were built 1956 to the mid-1970s, usually 4–5 storeys (no lift required at 5 storeys or fewer), in panel or brick, with basements, including Ukrainian design series [src:wikipedia-khrushchevka]. Toretsk's high-rise centre was demolished by Ukrainian engineers to deny it to Russian troops [src:euromaidan-2024-toretsk-highrises].
- **Linear street villages.** Villages strung along one street down a valley or balka are a key procedural motif, but no source describing them was retrieved (§10).

### 2.6 Industry and mining

- **Avdiivka coke plant.** The plant was the "gateway to the city from the northeast". The industrial zone (*promzona*, "promka") near Yasynuvata-2 station was the first line of defence, with reinforced-concrete positions linked by trenches and tunnels less than 10 km from Donetsk [src:euromaidan-2023-havrylets-avdiivka] (e6).
- **Mining towns.** Toretsk, Pokrovsk, Myrnohrad and Selydove pair mine shafts and waste heaps with apartment blocks and private-house districts [src:euromaidan-2024-toretsk-highrises] [src:wikipedia-battle-of-pokrovsk].
- **In the game.** Industrial and mining towns are dense, concrete and tunnelled, so they are high-cover, high-defence tiles. Terrikons act as line-of-sight dominators.

## 3. Seasons and weather

| season/condition | ground | foliage/visibility | tactical effect | source id |
|---|---|---|---|---|
| Spring thaw (Mar–Apr) | "Liquid earth": chernozem mud over still-frozen subsoil, fields impassable | Leafless belts, little cover | Off-road vehicle movement stops and heavy traffic keeps to paved roads. Infantry and quads keep moving but are slowed | [src:spire-2025-soil-moisture] [src:rferl-2026-lohinov-spring-battlefield] |
| Late spring (mid-Apr–May) | Ground hardens (mechanised assaults resumed near Pokrovsk by mid-April 2025) | Foliage expanding | Infiltration along treelines rises. Defenders prepare treelines before the leaves come in | [src:spire-2025-soil-moisture] [src:rferl-2026-lohinov-spring-battlefield] |
| Summer (Jun–Sep) | Dry and firm. Dust is likely but no source was retrieved | Full canopy | Leaves hide positions from drones and precision fires | [src:cepa-2023-kallberg-treelines] |
| Autumn (Oct–Nov) | Rains bring the autumn rasputitsa and armies shift to the defensive (Nov 2023) | Leaf-fall exposes positions | Detection rises and mobility falls | [src:cepa-2023-kallberg-treelines] [src:spire-2025-soil-moisture] |
| Winter with snow | Frozen ground (as cold as −20 °C in 2026) | Bare dark treelines against white snow | Tracks in snow lead drones to positions, and movement on snow is readily detected. Troops wear white ponchos and walk single file | [src:united24-2026-brizard-winter-camouflage] [src:cepa-2023-kallberg-treelines] |
| Winter without snow | Frozen or wet, grey | "Everything looks grey" | Camouflage patterns are harder to match | [src:united24-2026-brizard-winter-camouflage] |
| Winter/early-spring wind | — | Leafless belts are more permeable | Blown soil moves mainly in this period (a detail for dust effects) | [src:tarasov-usda-luhansk-wind-erosion] |
| Fog | — | Drones blinded | A window for reinforcement and infiltration (Pokrovsk, Feb 2025). Ukrainian drone work was hampered (Aug 2025) | [src:wikipedia-battle-of-pokrovsk] |
| Night and cold | — | Thermal imaging, more effective in the cold | Troops use thermal ponchos and heated batteries | [src:wotr-2026-maurin-front-line] [src:united24-2026-brizard-winter-camouflage] |
| Rain (drone grounding) | — | — | Not sourced (§10) | — |

## 4. Battle damage over time

- **Treelines.** Treelines go from intact to thinned to "piles of split wood" under sustained barrages [src:cepa-2023-kallberg-treelines]. Forests such as Serebrianka were "severely damaged" over three years [src:wikipedia-serebriansky-forest].
- **Villages.** Towns get erased over time. Marinka (about 9,400 residents) went from damaged in May 2022 to "few buildings left standing" by November 2022 and "not a single surviving house" by December 2022, and the battle lasted about 20 months [src:wikipedia-battle-of-marinka-2022]. Toretsk's centre was levelled into "battles among ruins" [src:euromaidan-2024-toretsk-highrises].
- **Craters.** In 2014 imagery of Donetsk oblast, 22,000+ craters were found across 858 km². That is about 26 craters per km², covering 0.14% of the area. It is a *low* pre-2022 baseline, and e4 (Bakhmut) and e6 (Avdiivka) hotspots should be far denser [src:umd-2023-duncan-craters].
- **Landscape transformation.** The Kakhovka reservoir went from water to mud to forest (§2.4). Front-line fields were abandoned [src:sadeh-2025-ukraine-field-boundaries].
- **Satellite view across years.** Open 10 m imagery gives a year-by-year record:
  - Sentinel-2 (L2A from 2017, 5-day revisit) [src:copernicus-sentinel-2]
  - Dynamic World (from 2015, per scene) [src:google-dynamic-world]
  - Hansen tree-cover loss (annual to 2024/25, 30 m) [src:umd-hansen-gfc]
  - ESA WorldCover 2021, a pre-invasion baseline [src:esa-2021-worldcover]
  - Both DEMs predate 2022 (SRTM in 2000, Copernicus in 2011–2015), so they show pre-war terrain [src:usgs-srtmgl1] [src:copernicus-dem-glo30]

  A 10–25 m shelterbelt is only 1–2 pixels wide at 10 m, so treeline loss is detectable only coarsely.
- **Contested zone.** The contested zone near Pokrovsk was about 1 km deep on 1 Jan 2025 and 5–8× wider by 2026 [src:wotr-2026-maurin-front-line] (e7 to e9). Damage and fallow should therefore spread across a widening belt, not a thin line.

## 5. Open geodata catalog

| dataset | what | resolution | coverage | license | url source id |
|---|---|---|---|---|---|
| OpenStreetMap | Vectors for `landuse=farmland`, `natural=tree_row` (explicitly includes shelterbelts), `natural=wood`/`landuse=forest`, highways, places and buildings | vector | global, incl. Ukraine | ODbL 1.0 (share-alike on derived DBs) | [src:osm-copyright] [src:osm-wiki-tree-row] |
| Copernicus DEM GLO-30 | DSM (includes trees and buildings), 2011–2015 | 30 m (also 90 m); vertical <4 m LE90 | global | free license, attribution DLR/Airbus/Copernicus | [src:copernicus-dem-glo30] |
| SRTM GL1 v003 | radar DEM, Feb 2000 | 1″ (~30 m) | 60°N–56°S | US gov., any use | [src:usgs-srtmgl1] |
| ESA WorldCover v200 | 11-class land cover, 2021 (v100: 2020) | 10 m | global | CC BY 4.0 | [src:esa-2021-worldcover] |
| Sentinel-2 L2A | multispectral imagery, 5-day revisit | 10/20/60 m | global land, 2017– | Copernicus Sentinel terms (free and open) | [src:copernicus-sentinel-2] |
| Dynamic World V1 | per-scene 9-class land-cover probabilities | 10 m | global, 2015– | CC BY 4.0 | [src:google-dynamic-world] |
| Hansen GFC v1.12 (behind Global Forest Watch) | tree cover in 2000 and annual loss 2001–2024 | ~30 m | global | CC BY 4.0 | [src:umd-hansen-gfc] |
| NASA Harvest Ukraine field boundaries | >5 M field polygons ≥1 ha, 2021–2023 | from 3 m PlanetScope | Ukraine | CC BY-NC-ND 4.0 (calibration only, not shippable) | [src:sadeh-2025-ukraine-field-boundaries] |
| Fields of The World | 1.63 M field polygons + S2 chips | 10 m | 24 countries, **not Ukraine** | CC BY 4.0 plus per-country source licenses | [src:ftw-2024-fields-of-the-world] |
| AI4Boundaries | 2.5 M parcels + imagery | 10 m S2 / 1 m ortho | 7 EU regions, **not Ukraine** | CC BY 4.0 | [src:jrc-2023-ai4boundaries] |
| EuroCrops | CAP crop parcels | vector | 17 EU countries, **no Ukraine subset** | CC BY-SA 4.0 | [src:eurocrops] |
| Overture Maps | conflated buildings, transportation, base land use and water | vector | global | ODbL (buildings, transport, base); CDLA-P 2.0 (places) | [src:overture-maps-attribution] |
| Microsoft Global ML Building Footprints | 1.4 B building polygons (174 M with height), imagery 2014–2024 | vector | global | CDLA Permissive 2.0 per repo (Overture lists ODbL; check) | [src:microsoft-global-ml-building-footprints] |
| Global field-size map (Geo-Wiki) | field-size classes | global grid | global | not recorded | [src:lesiv-2019-field-size] |

## 6. Procedural-generation references

- **Parish & Müller 2001, *Procedural Modeling of Cities*.** An extended L-system grows road networks under global goals (pattern templates such as a grid, density maps) and local constraints (water, slope), then subdivides blocks into lots [src:parish-muller-2001-cities]. The kolkhoz landscape suits this with a strong grid goal: straight belt roads at fixed spacing, snapped to the belt grid.
- **Vanegas et al. 2012, *Procedural Generation of Parcels*.** Recursive oriented-bounding-box splitting and straight-skeleton subdivision of blocks [src:vanegas-2012-parcels]. OBB splitting reproduces rectangular field blocks between belts and narrow house plots along a village street.
- **Emilien et al. 2012, *Procedural generation of villages on arbitrary terrains*.** A terrain-aware village growth model covering road seeds, parcels and buildings driven by slope and accessibility [src:emilien-2012-villages]. It fits linear villages following valleys and balkas.
- **Galin et al. 2010, *Procedural Generation of Roads*.** Weighted shortest-path road routing across terrain, with slope, water and vegetation costs and bridges [src:galin-2010-roads]. It suits inter-village roads crossing gullies and floodplains.
- **Smelik et al. 2014, *A Survey on Procedural Modelling for Virtual Worlds*.** A map of the field: terrain, vegetation, water, roads and settlements [src:smelik-2014-procedural-survey].
- **Patel 2010, *Polygon Map Generation*.** Voronoi and Lloyd-relaxed polygons with elevation, moisture and rivers [src:patel-2010-polygon-maps]. It suits irregular macro features (woods, catchments, balka networks), less so the rectilinear field grid.
- **Gap.** No paper was retrieved specifically on procedural *hedgerow or shelterbelt* placement or *agricultural field-pattern* synthesis. The web-search budget ran out (§10).

## 7. Key figures

| figure | value | unit | source id |
|---|---|---|---|
| Shelterbelts in Ukraine, early 1990s | ~400,000 | ha | unfccc-ukr-agroforestry-brief |
| Field-protective belts planted 1950–1960 | 440,000 | ha | uncg-2022-shamina-stalin-plan |
| Field-protective belts planted 1971–1991 | 154,500 | ha | uncg-2022-shamina-stalin-plan |
| Shelterbelt width (studied oak windbreak) | 15 | m | maliuha-2023-windbreaks |
| Optimal belt width for soil trapping (Luhansk) | ~10 | m | tarasov-usda-luhansk-wind-erosion |
| Mature belt height | 19.6–23.4 | m | maliuha-2023-windbreaks |
| Distance between belts (Oleshky, Kherson oblast) | 350–780 (mean 550) | m | ffm-oleshky-shelterbelts |
| Example treeline length (CHAI, near Pokrovsk) | ~1 | km | wotr-2026-maurin-front-line |
| Exposed trench footprint (HOMAR) | 80 × 30 | m | wotr-2026-maurin-front-line |
| Contested-zone depth, Pokrovsk salient, 1 Jan 2025 → 2026 | ~1 → 5–8× wider | km | wotr-2026-maurin-front-line |
| Depth of reserves/artillery hidden in treelines | 8–15 | km behind front | cepa-2023-kallberg-treelines |
| Field-size classes: large / very large | 16–100 / >100 | ha | lesiv-2019-field-size |
| Fields ≥1 ha in Ukraine (2021–23) | >5,000,000 | fields | sadeh-2025-ukraine-field-boundaries |
| Avdiivka terrikon height | ~40 | m | euromaidan-2023-havrylets-avdiivka |
| Toretsk waste dumps height | >100 | m | euromaidan-2024-toretsk-highrises |
| Enemy line distance, Lyman pine woods (May 2022) | ~300 | m | kyivindependent-2022-ponomarenko-donbas-woods |
| Serebriansky reserve area | 107.1 | ha | wikipedia-serebriansky-forest |
| Siverskyi Donets length (in Ukraine) | 1,053 (~950) | km | wikipedia-siverskyi-donets |
| Kakhovka former bed now vegetated | >2,000 | km² | uanimals-2026-kakhovka-forest |
| Land flooded to create Kakhovka reservoir | 709,900 | ha | uncg-2022-shamina-stalin-plan |
| Marinka pre-war population / battle length | ~9,400 / ~20 | people / months | wikipedia-battle-of-marinka-2022 |
| Crater density, Donetsk oblast 2014 (baseline) | ~26 (22,000 in 858 km²) | craters/km² | umd-2023-duncan-craters |
| Crater area share (2014 baseline) | 0.14 | % | umd-2023-duncan-craters |
| Infiltration group size, Pokrovsk (Jul–Aug 2025) | ~50 (30 of 150 got through) | soldiers | wikipedia-battle-of-pokrovsk |
| Winter low, 2025–26 | −20 | °C | united24-2026-brizard-winter-camouflage |
| Khrushchevka height | 4–5 | storeys | wikipedia-khrushchevka |
| WorldCover / Dynamic World / Sentinel-2 resolution | 10 | m | esa-2021-worldcover |
| Copernicus DEM / SRTM resolution | 30 | m | copernicus-dem-glo30 |
| Copernicus DEM absolute vertical accuracy | <4 | m (LE90) | copernicus-dem-glo30 |

## 8. Game/sim relevance

- **Belt grid.** Generate main belts perpendicular to a per-map prevailing-wind axis (east winds in Luhansk give N–S belts [src:tarasov-usda-luhansk-wind-erosion]). Sample spacing from about U(350, 780) m, mean 550 [src:ffm-oleshky-shelterbelts]. Set width to 10–25 m (1–4 tree rows plus a shrub row) and height to 15–23 m [src:maliuha-2023-windbreaks]. Add longer-spaced cross-belts; their spacing is unsourced, so calibrate from OSM `natural=tree_row` in Donetsk and Zaporizhzhia oblasts. Add a *gap* probability for degraded Soviet belts [src:unfccc-ukr-agroforestry-brief].
- **Fields and roads.** Subdivide belt-bounded blocks of about 0.5 × 1–2 km (40–100+ ha) [src:lesiv-2019-field-size] with OBB splits [src:vanegas-2012-parcels]. Put dirt tracks along belt edges, and paved roads only between settlements, routed by terrain cost [src:galin-2010-roads]. Near the front, flip fields to fallow and weeds with probability rising by era [src:sadeh-2025-ukraine-field-boundaries].
- **Terrain skeleton.** Derive balkas and rivers from DEM flow accumulation (Copernicus GLO-30 or SRTM) [src:copernicus-dem-glo30]. Make right banks high and gullied with chalk outcrops, and left banks low with wetlands and pine-on-sand terraces [src:wikipedia-siverskyi-donets]. Place natural woods only in balkas and floodplains [src:tarasov-usda-luhansk-wind-erosion]. Villages follow valleys [src:emilien-2012-villages]. Mining towns get 1–3 terrikons of 40–100+ m as line-of-sight dominators [src:euromaidan-2024-toretsk-highrises].
- **Seasonal modifiers** (a monthly table):
  - Treeline concealment: high in Jun–Sep, dropping in Oct–Nov, low in winter [src:cepa-2023-kallberg-treelines].
  - Snow: tracks become persistent, detectable trails [src:united24-2026-brizard-winter-camouflage].
  - Cold: thermal detection bonus [src:united24-2026-brizard-winter-camouflage].
  - Mud: off-road vehicle mobility near zero in Mar–early Apr and in Oct–Nov, with a random dry-out date around mid-April [src:spire-2025-soil-moisture].
  - Fog: random events that ground drones and open infiltration windows [src:wikipedia-battle-of-pokrovsk].
- **Damage by era.** Make crater density per km² a function of era and hotspot, starting from a floor of about 26/km² [src:umd-2023-duncan-craters]. Treeline state goes from intact to thinned to split wood [src:cepa-2023-kallberg-treelines]. Village state goes from intact to damaged to razed, over about 8 months of sustained fire in the Marinka example [src:wikipedia-battle-of-marinka-2022]. The Kakhovka tile changes by era: water to mudflat (e5) to willow forest (e8–e9) [src:uanimals-2026-kakhovka-forest]. The contested-zone width grows from about 1 km (e7) to 5–8 km (e9) [src:wotr-2026-maurin-front-line].
- **Licensing for shipped assets.** If generated maps are *derived* from OSM or Overture, ODbL share-alike applies to the derived database [src:osm-copyright] [src:overture-maps-attribution]. Prefer CC BY sources (WorldCover, Dynamic World, Hansen) for baked data. Use NASA Harvest (NC-ND) only for offline statistics.

## 9. Terms

- **lisosmuha** (лісосмуга, pl. *lisosmuhy*) — shelterbelt, a planted windbreak strip between fields; Russian *lesopolosa*
- **polezakhysni lisovi smuhy** — "field-protective forest belts", the formal agronomic term (rendered in English sources)
- **Velykyi Luh** — "Great Meadow", the Dnipro floodplain drowned by the Kakhovka reservoir and now regrowing
- **balka** — a broad, usually dry, grassed or wooded gully or valley in the steppe
- **yar** — a steeper, younger ravine
- **terykon / terrikon** (терикон) — a conical coal-mine spoil heap
- **shakhta** — coal mine
- **promzona / promka** — industrial zone
- **khrushchovka** (Russian *khrushchevka*) — a 1956–1970s block of 4–5 storeys
- **pryvatnyi sektor** — "private sector", districts of single-family houses with gardens (term not verified in sources here)
- **dacha** — a summer cottage and garden plot
- **chornozem** (Russian *chernozem*) — black-earth soil
- **bezdorizhzhia** (бездоріжжя) — "roadlessness", the Ukrainian term for the mud season; Russian *rasputitsa*
- **kolhosp / radhosp** — Soviet collective farm and state farm (Russian *kolkhoz* / *sovkhoz*), the origin of the large-field grid
- **sira zona** — "gray zone", the contested ground between the two sides' positions
- **posadka** — soldiers' slang for a treeline (common usage; not verified in this pack)
- **zelenka** — slang for the foliage season or green cover (common usage; not verified in this pack)
- **DSM vs DTM** — a surface model including trees and buildings (Copernicus DEM) vs bare-earth terrain

## 10. Open questions / gaps

- **Field block dimensions.** No source was fetched that states typical east/south field sizes (e.g., 1–2 km blocks), and §2.2 is derived. The next step is to measure `landuse=farmland` polygons and `natural=tree_row` spacing in OSM for sample areas (Pokrovsk, Robotyne, Kupiansk), or to compute stats from the NASA Harvest polygons offline.
- **Soviet belt design norms.** Spacing in multiples of tree height, cross-belt spacing and number of rows were not retrieved. The only spacing figure is search-only [src:ffm-oleshky-shelterbelts]. Ukrainian-language norms, such as the 2020 Cabinet rules on field-protective belts, were not read.
- **Parfonov report.** The full *Fighting Through the Treelines* PDF (Google Drive) was not read and likely has the best tactical detail [src:saratoga-2026-parfonov-treelines].
- **Dataset corrections to the brief.** No EuroCrops Ukraine subset exists. FTW and AI4Boundaries exclude Ukraine. The Microsoft footprint licence is inconsistent (CDLA-P 2.0 in the repo vs ODbL at Overture).
- **Landforms and rivers not sourced.** Balka fighting, the Oskil, Mokri Yaly and Kinska valleys, and chalk hills beyond the Siverskyi Donets have no retrieved source. Linear-street-village morphology and village density per km² are also missing and could be computed from OSM `place=village`.
- **Weather effects.** Summer dust, rain grounding drones, and a quantified foliage-versus-detection relationship have no source.
- **Treeline loss over time.** No quantitative study of treeline or shelterbelt loss by year along the front was found. It could be derived from Hansen loss or Dynamic World, but belt widths are near the pixel size.
- **Crater baseline.** The crater figure is from 2014 imagery. No 2022–2026 crater-density study was fetched (a Bakhmut January 2023 imagery mention was search-only and is not cited).
- **Serebrianka.** The claim that the forest fell in December 2025 rests on Wikipedia and should be cross-checked with DeepState or ISW.
- **Process note.** The session's web-search quota ran out mid-research, so the procgen literature on field patterns and hedgerows is thinner than planned.

## 11. Sources

- `eou-shelterbelt` — Encyclopedia of Ukraine: shelterbelt history (1809, 1938–39, 1948–52), species.
- `uncg-2022-shamina-stalin-plan` — Ukrainian NGO: 1948 plan in Ukraine, belt areas planted, Kakhovka flooding and Velykyi Luh.
- `unfccc-ukr-agroforestry-brief` — Ukraine TNA brief: ~400k ha of belts, ownership and degradation, restoration target.
- `maliuha-2023-windbreaks` — Ukrainian forestry paper: 15 m width, 19.6–23.4 m height, oak-nest design.
- `tarasov-usda-luhansk-wind-erosion` — Luhansk study: east winds, ~10 m optimal width, woods in gullies, leafless winter.
- `ffm-oleshky-shelterbelts` — Kherson oblast belt spacing 350–780 m (search-only).
- `cepa-2023-kallberg-treelines` — treeline warfare, foliage vs leaf-fall, belts shredded (e5).
- `saratoga-2026-parfonov-treelines` — Ukrainian analyst's report on Russian shelterbelt infiltration (announcement only).
- `wotr-2026-maurin-front-line` — first-hand: 1 km treeline, trench vs concealed holes, contested zone widening (e7–e9).
- `kyivindependent-2022-ponomarenko-donbas-woods` — Lyman pine woods, sandy trenches, resorts as positions (e2).
- `euromaidan-2023-avdiivka-spoil-tip` — Avdiivka terrikon as an observation fortress (e6).
- `euromaidan-2023-havrylets-avdiivka` — 40 m terrikon, coke plant, promzona, concrete and tunnels (e6).
- `euromaidan-2024-toretsk-highrises` — Toretsk: >100 m waste dumps, private-house district, demolished high-rises (e7).
- `wikipedia-serebriansky-forest` — Serebrianka forest near Kreminna, 107.1 ha reserve, fighting 2022–25.
- `wikipedia-battle-of-marinka-2022` — Marinka razed over ~20 months (e1–e6).
- `wikipedia-battle-of-pokrovsk` — fog as a window, small-group infiltration, terrikons (e7–e8).
- `rferl-2026-lohinov-spring-battlefield` — spring mud, foliage cover, quad bikes (e9).
- `united24-2026-brizard-winter-camouflage` — snow tracks, bare treelines, thermals, −20 °C (e8–e9).
- `spire-2025-soil-moisture` — rasputitsa/bezdorizhzhia timing and effects 2022–2025.
- `wikipedia-siverskyi-donets` — river length, chalk right bank, swampy left bank, pine terraces.
- `uanimals-2026-kakhovka-forest` — >2,000 km² willow forest on the former reservoir bed (e5→e9).
- `defenceexpress-2024-kakhovka-refill` — snowmelt refill, impassable soft bed (e5–e6).
- `wikipedia-khrushchevka` — Soviet 4–5-storey block typology.
- `umd-2023-duncan-craters` — crater density baseline from 2014 VHR imagery.
- `sadeh-2025-ukraine-field-boundaries` — NASA Harvest 5M-field Ukraine dataset (CC BY-NC-ND).
- `lesiv-2019-field-size` — global field-size classes and map.
- `osm-copyright` — OSM ODbL licence.
- `osm-wiki-tree-row` — `natural=tree_row` semantics (includes shelterbelts).
- `copernicus-dem-glo30` — 30 m DSM, licence, accuracy.
- `usgs-srtmgl1` — 30 m SRTM (2000).
- `esa-2021-worldcover` — 10 m land cover 2021, CC BY 4.0.
- `copernicus-sentinel-2` — 10 m imagery archive, 5-day revisit.
- `google-dynamic-world` — 10 m near-real-time land cover, CC BY 4.0.
- `umd-hansen-gfc` — 30 m tree-cover loss (behind GFW), CC BY 4.0.
- `microsoft-global-ml-building-footprints` — 1.4B building footprints, licence caveat.
- `overture-maps-attribution` — Overture per-theme licences.
- `ftw-2024-fields-of-the-world` — field-boundary benchmark; no Ukraine.
- `jrc-2023-ai4boundaries` — EU field-boundary dataset; no Ukraine.
- `eurocrops` — EU crop parcels; no Ukraine subset.
- `parish-muller-2001-cities` — L-system road networks and lots.
- `emilien-2012-villages` — terrain-aware village generation.
- `galin-2010-roads` — terrain-cost road routing.
- `vanegas-2012-parcels` — OBB and skeleton parcel subdivision.
- `smelik-2014-procedural-survey` — survey of procedural world modelling.
- `patel-2010-polygon-maps` — Voronoi polygon map generation (game-dev blog).
