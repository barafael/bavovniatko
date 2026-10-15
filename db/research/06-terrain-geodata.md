<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# 06 — Terrain and geodata

Research pack for **bavovniatko**, covering the procedurally generated front-line landscape of eastern and southern Ukraine. Accessed 2026-09-28. Citations use the form `[src:<id>]` and resolve to `sources/06-terrain-geodata.yaml`. Era ids follow the canonical list (e1–e9).

## 1. Overview

The front from Kharkiv to Kherson is mostly open steppe and forest-steppe farmland on chernozem (chornozem, "black earth"). Large fields are separated by planted windbreak belts, called *lisosmuhy* in Ukrainian (sing. *lisosmuha*) and *lesopolosy* in Russian [src:eou-shelterbelt] [src:cepa-2023-kallberg-treelines]. Natural woodland is scarce and occurs "mainly along rivers banks and in gullies" (balkas) [src:tarasov-usda-luhansk-wind-erosion]. The exceptions are the sandy pine forests on the left-bank terraces of the Siverskyi Donets (Lyman, Kreminna and Serebrianka) [src:wikipedia-siverskyi-donets] [src:kyivindependent-2022-ponomarenko-donbas-woods]. The Donbas adds mining towns with terrikons (spoil heaps) 40 m to over 100 m high, coke and chemical plants, and Soviet apartment blocks [src:euromaidan-2023-havrylets-avdiivka] [src:euromaidan-2024-toretsk-highrises].

On this ground the treeline is the basic tactical unit. It provides cover, a firing position, an infiltration corridor and an artillery target. Its value swings with the seasons: foliage in summer, bare branches and snow in winter, and mud (rasputitsa, Ukrainian *bezdorizhzhia*) in spring and autumn [src:cepa-2023-kallberg-treelines] [src:rferl-2026-lohinov-spring-battlefield] [src:united24-2026-brizard-winter-camouflage] [src:spire-2025-soil-moisture]. In e8–e9 drone saturation has widened the contested zone from about 1 km to several km, and survival now depends on concealment more than on firepower [src:wotr-2026-maurin-front-line].

For procedural generation, the landscape is regular. It consists of a rectilinear Soviet-era field and belt grid laid over a dendritic network of gullies and river valleys, with villages strung along the valleys. Measured in five 10 × 10 km front-line boxes (§2.7), belt-bounded blocks are typically about 0.5–0.6 km × 1.0–1.3 km (40–65 ha median), and small villages are linear strips a few hundred metres wide. Several open datasets can calibrate this (OSM, Copernicus DEM, ESA WorldCover, Sentinel-2 and Dynamic World) (§5).

## 2. Landscape elements

### 2.1 Shelterbelts (lisosmuhy)

**Origin.** Ukrainian shelterbelts date from 1809, when Vasyl Lomykovsky planted trees around fields in Poltava gubernia. They were studied by Dokuchaev and Vysotsky in the 1890s and planted systematically in 1938–39 and 1948–52 [src:eou-shelterbelt]. The 1948 "Great Stalin Plan for the Transformation of Nature" drove the largest wave. By a Ukrainian NGO's count, 440,000 ha of field-protective belts were planted in the ten years from 1950, and another 154,500 ha in the 20 years from 1971 [src:uncg-2022-shamina-stalin-plan]. Ukraine had about 400,000 ha of shelterbelts in the early 1990s [src:unfccc-ukr-agroforestry-brief]. After the collective farms were dissolved, many belts ended up with unclear ownership and outside the land cadastre, and they degraded. The government's 2020 rules and the national Technology Needs Assessment target reconstruction of about 85,000 ha by 2030 [src:unfccc-ukr-agroforestry-brief]. The front-line belts in e1–e9 are therefore mostly 50–75-year-old Soviet plantings, often gappy or fragmented.

**Dimensions.**
- Width and rows. Front-line belts are "typically three to four rows of trees deep" and "rarely more than twenty to thirty meters wide", with **10–30 m gaps** left for farm machinery [src:saratoga-2026-parfonov-treelines]. Belts mapped as narrow wood strips in OSM have a median canopy width of **20–27 m** in the sample boxes (§2.7) [src:osm-overpass-2026-landscape-sample]. Canopy width overstates the planted width: Ukrainian foresters convert remote-sensed canopy width to stem-to-stem width with a factor of 0.46 [src:matsala-2024-war-protective-plantations]. A studied Left-Bank forest-steppe oak windbreak is **15 m** wide, planted as oak "nests" at 5 × 3 m with 5 m alleys and companion and shrub rows [src:maliuha-2023-windbreaks]. A Luhansk soil-erosion study found about **10 m** to be the optimal width for trapping blown soil [src:tarasov-usda-luhansk-wind-erosion].
- Height. Mature belts reach **19.6–23.4 m** [src:maliuha-2023-windbreaks].
- Spacing. Parfonov describes a dense grid at "intervals of 300 to 400 meters longitudinally and 1,000 to 2,000 meters laterally" [src:saratoga-2026-parfonov-treelines]. The measurements put the main belts somewhat further apart. At Huliaipole the median is **549 m** (p10–p90 379–990 m) between main belts and about **1.1–1.3 km** between cross-belts. At Robotyne it is about 0.5–0.6 km and 1.0–1.2 km (§2.7) [src:osm-overpass-2026-landscape-sample] [src:eox-2021-s2cloudless]. In the Oleshky district of Kherson oblast the distance between belts is 350–780 m, averaging 550 m (search-only) [src:ffm-oleshky-shelterbelts]. With belts about 20 m tall, main belts therefore stand roughly 25–30 tree heights apart. This is a derived figure: Soviet design norms stated as multiples of height were not found.
- Density. Kherson oblast alone had about 30,000 km of windbreaks, roughly 1 km per km² of oblast [src:sudnik-2006-windbreaks-southern-ukraine]. A full two-way grid at the measured spacings implies about 2.5–3 km of belt per km² of cropland. The best-mapped OSM boxes reach 1.35 km/km² (§2.7).
- Orientation. In Luhansk oblast the main harmful (deflation) winds come from the east [src:tarasov-usda-luhansk-wind-erosion]. Main belts are laid across the prevailing wind, so they tend to run roughly north–south there. The measured grids are near-cardinal at Huliaipole (N–S main belts, E–W cross-belts) and Robotyne (roughly E–W and N–S, 10° off). They are oblique (WNW–ESE and NNE–SSW) east of Kupiansk and irregular in the dissected ground south of Chasiv Yar (§2.7).
- Species. Oak, pine, birch and poplar form the tall layer, with linden, maple, apple and pear lower down [src:eou-shelterbelt]. Oak-led belts with companion trees and a shrub understorey are common [src:maliuha-2023-windbreaks].

**Tactical role.**
- Positions and reserves. Treelines hold forward trenches, while reserves, artillery and logistics sit 8–15 km back in treelines, woods and towns [src:cepa-2023-kallberg-treelines].
- Infiltration corridors. The Ukrainian soldier-analyst Hlib Parfonov bases his 2026 report on Russian tactical guidance and his own front-line service. It finds the *longitudinal* attack to be "the predominant method in current practice, employed by both Russian and Ukrainian forces": troops enter a belt from its flank and fight along its interior. The *perpendicular* attack across the open field is "operationally unviable in the majority of cases" because of mines, anti-tank weapons, artillery and exposure [src:saratoga-2026-parfonov-treelines]. The assault group is up to platoon strength and fights in depth along the belt:
  - scouts 7–9 m apart across the belt's width;
  - the main body up to 30 m behind;
  - a consolidation group 100–150 m further back;
  - a reserve up to 450 m from the point.

  The whole group is 300–500 m deep. Supporting guns sit 8–12 km back and mortars 2–3 km back, and belt intersections are avoided as command-post sites because they are obvious aiming points [src:saratoga-2026-parfonov-treelines]. In spring 2026 Russian forces tried to use new foliage to move infantry along treelines on foot, on motorcycles or on quad bikes [src:rferl-2026-lohinov-spring-battlefield].
- Survival. Near Pokrovsk in winter 2024–25, an exposed 80 × 30 m trench cut into the 1 km-long CHAI treeline was destroyed. Two concealed fighting holes in the undergrowth held for 10 days [src:wotr-2026-maurin-front-line].
- Foliage. Leaves protected positions from drones and precision fires in summer 2023. Autumn leaf-fall and then snow exposed them [src:cepa-2023-kallberg-treelines]. Parfonov writes that summer canopy "substantially degrade[s]" FPV effectiveness. Foliage conceals both sides from late spring to early autumn, and bare branches expose both from late autumn to early spring [src:saratoga-2026-parfonov-treelines].
- Being shredded. Sustained barrages reduced some treelines to "piles of split wood and tree branches" [src:cepa-2023-kallberg-treelines]. Shredded belts become "tangles of fallen timber" that are impassable and useless to both sides, effectively grey zones [src:saratoga-2026-parfonov-treelines].

### 2.2 Fields

- **Geometry.** The southern battlefield is "flat and open ... large fields are separated by treelines" [src:cepa-2023-kallberg-treelines]. The OSM farmland polygons in the five sample boxes give median field sizes of **38–65 ha**, with a p10–p90 of about 6–15 ha to 97–172 ha. The median minimum bounding rectangle is **420–625 m × 920–1,350 m** and the median aspect ratio is **1.8–2.0** (§2.7) [src:osm-overpass-2026-landscape-sample]. The fragmented Chasiv Yar box is smaller, at a 14 ha median. In the two cleanly gridded boxes (Robotyne, Huliaipole) a belt-bounded block is about **0.5–0.6 × 1.0–1.3 km**, and a block is sometimes split into two or three crop fields along its long axis.
- **Size class.** That puts the typical field in the "large" (16–100 ha) class of the global field-size map, with a tail in the "very large" (>100 ha) class. Large fields dominate post-Soviet countries (per a search summary of the study) [src:lesiv-2019-field-size].
- **National count.** Ukraine has more than 5 million fields of at least 1 ha (2021–2023) [src:sadeh-2025-ukraine-field-boundaries].
- **Abandonment.** Fields near the front are often abandoned and hard to delineate, and in the game they should become weedy fallow [src:sadeh-2025-ukraine-field-boundaries]. Nationally, cultivated area fell by 2.5 Mha (−8.5%) from 2021 to 2024. By 2024, 7% of cropland (about 2.2 Mha) was abandoned, and 1.1–1.7 Mha of it, "primarily along frontlines", may be permanently lost. Up to 0.47 Mha was recultivated in 2024 alone, far more than official demining covers [src:wagner-2025-cropland-abandonment].
- **Tactical meaning.** Open fields are kill zones watched by drones. Movement happens along or through the belts, and crossing a field between belts is the dangerous phase [src:wotr-2026-maurin-front-line] [src:rferl-2026-lohinov-spring-battlefield].
- **Roads.** Paved roads become the only routes for heavy vehicles in mud season [src:spire-2025-soil-moisture]. Primary to tertiary roads run at **0.13–0.22 km per km²** in the sample boxes. Mapped `highway=track` density is 0.3–0.9 km/km² where tracks are mapped at all (Huliaipole has almost none in OSM) [src:osm-overpass-2026-landscape-sample]. Only 4–24% of mapped track length lies within 40 m of a mapped belt, so "a track along every belt" is not supported by OSM. The imagery shows tracks along some belt edges and across fields, which OSM maps incompletely (§2.7).

### 2.3 Landforms

- **Balkas and yary (gullies and ravines).** Balkas and yary cut the steppe interfluves. They host most of the natural woodland [src:tarasov-usda-luhansk-wind-erosion] and dissect the high right bank of the Siverskyi Donets [src:wikipedia-siverskyi-donets]. In the Sentinel-2 sample mosaics, wooded balkas show up as dendritic strips several kilometres long that cut diagonally across the belt grid. Villages sit along them or at their heads (W of Pokrovsk, E of Kupiansk, E of Huliaipole) [src:eox-2021-s2cloudless]. Ravines are also mapped as their own class of protective plantation [src:matsala-2024-war-protective-plantations]. Tactically, a gully separates one "elevated platform" (a flat interfluve) from the next. At Rivnopil in June 2023 Russian positions were vulnerable because a gully cut them off from the next platform they held [src:euromaidan-2023-rivnopil-heights] (e5). Assault troops use "folds in the ground" to approach a belt [src:saratoga-2026-parfonov-treelines]. For the game, balkas are covered approach routes, natural anti-vehicle obstacles and boundaries between defensible platforms. No source gives typical balka widths or depths (§10).
- **Chalk hills.** The right bank of the Siverskyi Donets is "usually high, sometimes with chalk cliffs" (the Sviatohirsk / Holy Mountains area). The left bank is low, with swamps, lakes and oxbows [src:wikipedia-siverskyi-donets]. This asymmetry means the high bank dominates and the low bank floods. The forests of the Sviati Hory national park on the chalk hills were about 80% devastated by 2024 (NGL.media estimate) [src:euromaidan-2024-ngl-forest-destruction].
- **Pine forests on sand.** The left-bank terraces carry pine forest [src:wikipedia-siverskyi-donets]. Near Lyman in 2022 the forest positions were sandy trenches, with the enemy about 300 m away behind tall pines [src:kyivindependent-2022-ponomarenko-donbas-woods] (e2).
- **Serebrianka forest.** The Serebriansky forest SW of Kreminna mixes forest, swamp and meadow. The protected reserve covers 107.1 ha within a much larger pine forest [src:wikipedia-serebriansky-forest]. It was contested from autumn 2022 (e3–e8). DeepState's changelog records back-and-forth fighting in "Serebryanske forestry": Russian gains in June and August 2023, Ukrainian recoveries in November 2023 and June 2024, then Russian advances almost weekly from 5 August to 29 September 2025. Sampling DeepState's polygons over the forest shows the occupied share rising from 15% (1 Aug 2025) to 64% (1 Sep), 90% (1 Oct) and 100% (20 Dec 2025). The village of Serebrianka was occupied on 16 Dec and Siversk on 23 Dec 2025. The forest was still fully occupied on 27 Sep 2026 [src:deepstate-api-history]. This confirms Wikipedia's "fully captured by December 2025" [src:wikipedia-serebriansky-forest]. ISW's pages returned 403 and were not read.
- **Terrikons (terykony).** These are conical coal-waste heaps and act as artificial high ground. The Avdiivka terrikon is about 40 m high [src:euromaidan-2023-havrylets-avdiivka]. Its defenders "saw every Russian move from the heights", and the steep slope stopped assaults [src:euromaidan-2023-avdiivka-spoil-tip] (e6). Toretsk has two waste dumps over 100 m high [src:euromaidan-2024-toretsk-highrises] (e7), and terrikons also shaped fire control around Pokrovsk and Selydove [src:wikipedia-battle-of-pokrovsk].
- **Flat steppe (Zaporizhzhia).** The terrain is flat to gently rolling, so observation lines are long [src:cepa-2023-kallberg-treelines] (e5).

### 2.4 Water

- **Siverskyi Donets.** The river is 1,053 km long, about 950 km of it in Ukraine. It has an asymmetric valley and a wet floodplain with willow, birch and alder. Russian crossing attempts above Lysychansk failed in May 2022 [src:wikipedia-siverskyi-donets] (e2). Other named front rivers include the Vovcha and the Kazennyi Torets near Pokrovsk [src:wikipedia-battle-of-pokrovsk].
- **Oskil (Kupiansk).** Large pine massifs line the Oskil as well as the Donets [src:matsala-2024-war-protective-plantations]. To take Kupiansk, Russia had to cross the Oskil. The pre-war bridge was destroyed, and Ukrainian aircraft "typically" strike the floating spans whenever Russian engineers build pontoons [src:euromaidan-2025-mukhina-oskil-crossing] (e7). Russian troops instead entered Kupiansk through the disused Shebelynka–Ostrohozhsk gas pipeline, "a walk of about 10 km in darkness" that exits in a forest. Ukraine retook about 90% of the city by December 2025 after disabling the route. By September 2026 the pipe was an escape route, according to Ukraine's Khartia corps [src:euromaidan-2026-zoria-kupiansk-pipeline] (e8–e9). Pipelines, culverts and utility tunnels are therefore terrain objects in their own right.
- **Mokri Yaly (Velyka Novosilka).** The river is 147 km long, with a 2,660 km² basin. It runs north from the Azov Upland through Velyka Novosilka to the Vovcha, and it was a German defensive line in 1943 [src:wikipedia-mokri-yaly]. In June 2023 Russian forces blew up the hydro facility near Novodarivka "to slow down our advance", flooding both banks, according to the Ukrainian spokesman Valeriy Shershen [src:euromaidan-2023-mokri-yaly-dam] (e5). Small dams on steppe rivers are thus flood triggers.
- **Kinska (Orikhiv).** In August 2026 Russian groups took eastern Mala Tokmachka and "crossed north over the Kinska River", attacking Ukrainian trenches "on the tactical heights beyond" [src:euromaidan-2026-axe-mala-tokmachka] (e9). The pattern is a small river with heights on the far bank.
- **Kakhovka reservoir (e5 onward).** When the dam was destroyed on 6 June 2023, the reservoir (created by flooding 709,900 ha, including the Velykyi Luh "Great Meadow" of up to 80,000 ha) drained [src:uncg-2022-shamina-stalin-plan]. Snowmelt in March 2024 partly re-flooded it. The bed became soft ground that "no equipment, including amphibious vehicles" can cross [src:defenceexpress-2024-kakhovka-refill]. By May 2026 more than 2,000 km² of the old bed held a young willow-dominated forest. The old reservoir was 3.6 km wide at Kamianska Sich and 13.5 km at Novovorontsovka [src:uanimals-2026-kakhovka-forest].
- **In the game.** The Kakhovka area is a terrain state that changes with era: open water, then mudflat (e5), then wetland and scrub (e6–e7), then willow forest (e8–e9). The Dnipro stays a hard barrier throughout.

### 2.5 Settlements

- **Private-sector houses.** Single-storey private houses with gardens make up districts such as Zabalka in Toretsk, which sits "in unfavorable terrain" below the waste heaps [src:euromaidan-2024-toretsk-highrises].
- **Resorts and cottages.** Holiday resorts, children's camps and cottages in the Donbas pine woods served as positions [src:kyivindependent-2022-ponomarenko-donbas-woods]. They can stand in for dacha settlements.
- **Khrushchevky.** These Soviet blocks were built 1956 to the mid-1970s, usually 4–5 storeys (no lift required at 5 storeys or fewer), in panel or brick, with basements, including Ukrainian design series [src:wikipedia-khrushchevka]. Toretsk's high-rise centre was demolished by Ukrainian engineers to deny it to Russian troops [src:euromaidan-2024-toretsk-highrises].
- **Village size and shape (measured).** Microsoft ML building footprints were clustered in the five sample boxes (§2.7) [src:microsoft-global-ml-building-footprints]. The boxes hold **815–3,833 buildings per 100 km²** and **5–11 settlements of ≥20 buildings per 100 km²**. The median footprint is 45–56 m², which fits single-storey houses plus outbuildings.
  - Small settlements (<300 buildings, n = 33) are linear: median 71 buildings, 755 m long × 254 m wide, elongation (PCA axis ratio) 4.1 (p10–p90 1.8–10.6), about 100 buildings per km of length. Half have an elongation of 4 or more, the one- or two-street villages along a balka or stream.
  - Large villages (≥300 buildings, n = 10) are compact multi-street grids: median 595 buildings, 2.2 × 1.0 km, elongation 2.3.
  - OSM `place=village|hamlet` nodes number only 2–5 per 100 km² in the same boxes, because OSM place tagging is incomplete [src:osm-overpass-2026-landscape-sample].

### 2.6 Industry and mining

- **Avdiivka coke plant.** The plant was the "gateway to the city from the northeast". The industrial zone (*promzona*, "promka") near Yasynuvata-2 station was the first line of defence, with reinforced-concrete positions linked by trenches and tunnels less than 10 km from Donetsk [src:euromaidan-2023-havrylets-avdiivka] (e6).
- **Mining towns.** Toretsk, Pokrovsk, Myrnohrad and Selydove pair mine shafts and waste heaps with apartment blocks and private-house districts [src:euromaidan-2024-toretsk-highrises] [src:wikipedia-battle-of-pokrovsk].
- **In the game.** Industrial and mining towns are dense, concrete and tunnelled, so they are high-cover, high-defence tiles. Terrikons act as line-of-sight dominators.

### 2.7 Measured landscape statistics

**Method.** OSM data was queried through the Overpass API on 2026-09-28 for five 10 × 10 km boxes. The query covered `natural=tree_row`, `natural=wood`/`landuse=forest`, `landuse=farmland`, `highway=*`, `landuse=residential`, places and buildings, and the data is © OpenStreetMap contributors (ODbL) [src:osm-overpass-2026-landscape-sample]. The analysis projected the data to local metres in python/numpy. Belts are `tree_row` lines plus wood polygons with a mean width under 60 m and a length over 6× the width. Spacing is the perpendicular distance from each belt's midpoint to the nearest overlapping parallel belt (±15°). Fields are farmland polygons of at least 1 ha, measured by area and by minimum-area bounding rectangle. Settlements are Microsoft building footprints (release 2026-02-03) clustered on a 100 m grid [src:microsoft-global-ml-building-footprints]. As a satellite check, OSM was overlaid on 10 m Sentinel-2 cloudless 2021 mosaics (about 5 × 5 km per box). A black-top-hat detector for narrow dark lines then measured belt spacing independently [src:eox-2021-s2cloudless].

Bounding boxes (S, W, N, E): **Pokrovsk W** 48.22, 36.8325, 48.31, 36.9675; **Robotyne** 47.43, 35.7485, 47.52, 35.8815; **Kupiansk E** 49.70, 37.7005, 49.79, 37.8395; **Huliaipole E** 47.64, 36.2783, 47.73, 36.4117; **Chasiv Yar S** 48.47, 37.7822, 48.56, 37.9178.

| metric | Pokrovsk W | Robotyne | Kupiansk E | Huliaipole E | Chasiv Yar S |
|---|---|---|---|---|---|
| Mapped belts, km per km² (strip count, median width) | 0.28 (25, 27 m) | 1.34 (151, 26 m) | 0.62 (81, 23 m) | 1.35 (172, 27 m) | 0.67 (106, 20 m) |
| Belt axes (share of belt length within ±20°) | NNW–SSE 68% | ~E–W 42% / ~N–S 34% | WNW–ESE 54% / NNE–SSW 34% | N–S 55% / E–W 29% | irregular: ENE 34% / NNW 29% |
| Spacing, main belts: OSM median (p10–p90) / Sentinel-2 median | too few mapped / noisy | 1,154 (271–1,790) / ~1,000 m | 1,116 (482–2,341) / noisy | **549 (379–990) / 547 m** | 871 (396–2,883) / noisy |
| Spacing, cross-belts: OSM / Sentinel-2 | — | 1,134 (OSM misses belts) / ~610 m | 1,094 / noisy | 769 / 775 m (visually 1.1–1.3 km) | 601 / noisy |
| Farmland polygons ≥1 ha: n, median ha (p10–p90) | 78, 65 (15–172) | 109, 39 (8–97) | 47, 38 (6–109) | 119, 48 (15–126) | 52, 14 (3–72) |
| Field MBR median short × long, m | 625 × 1,348 | 533 × 969 | 416 × 919 | 609 × 1,050 | 386 × 678 |
| Field aspect ratio median (p10–p90) | 2.0 (1.2–4.3) | 1.8 (1.1–3.6) | 1.8 (1.1–4.9) | 1.9 (1.2–3.1) | 1.8 (1.2–3.4) |
| Farmland share of box mapped | 67% | 57% | 26% | 72% | 24% |
| Tracks / primary–tertiary roads, km per km² | 0.55 / 0.13 | 0.34 / 0.17 | 0.72 / 0.20 | 0.02 (unmapped) / 0.22 | 0.92 / 0.17 |
| MS buildings; settlements ≥20 buildings | 2,758; 8 | 815; 5 | 3,621; 9 | 3,833; 10 | 1,526; 11 |
| OSM buildings as share of MS buildings | 1.8% | 8.2% | 18% | 0.8% | 5.8% |

**Reading the table.**
- Huliaipole E is the clean reference grid. OSM and Sentinel-2 agree within 1% on the 550 m main-belt spacing, and the imagery shows cross-belts about 1.1–1.3 km apart. That gives **blocks of about 0.55 × 1.2 km**, split into 40–50 ha crop fields.
- Robotyne is the same pattern rotated about 10°, with main belts at about 0.5–0.6 km and cross-belts at 1.0–1.2 km.
- East of Kupiansk the forest-steppe fields are larger and less regular, and wooded balkas cut across them.
- South of Chasiv Yar the ground is dissected, with large woods and fragmented fields.

**Completeness caveat.**
- OSM is uneven. West of Pokrovsk almost no belts are mapped: they exist only as the gaps between farmland polygons, which the imagery shows follow the belts closely. In the best boxes about two-thirds of the belts visible in Sentinel-2 are mapped, so belt densities and OSM spacings are lower bounds and upper bounds respectively.
- OSM buildings are 1–18% complete, so building counts must come from Microsoft footprints, not OSM.
- The Sentinel-2 detector works only in clean grids. Crop stripes, tracks and (south of Chasiv Yar) wartime earthworks cause false detections.
- The imagery is from 2021 and so shows the pre-invasion landscape.
- Farmland polygons sometimes split one belt block into several fields, which biases field sizes low.

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
| Fog | — | Drones blinded | A window for reinforcement and infiltration: Avdiivka, morning fog, Nov 2023; Pokrovsk, Feb and Aug 2025. In thick fog on 7–12 Nov 2025 more than 300 Russian troops got inside Pokrovsk | [src:euromaidan-2023-avdiivka-rain-fog] [src:wikipedia-battle-of-pokrovsk] [src:euromaidan-2025-axe-foggy-advance] |
| Night and cold | — | Thermal imaging, more effective in the cold | Troops use thermal ponchos and heated batteries | [src:wotr-2026-maurin-front-line] [src:united24-2026-brizard-winter-camouflage] |
| Rain and wet weather | Mud (see autumn and spring) | Light quadcopters and FPVs degraded | Rain and frost "significantly hampered" reconnaissance and caused "substantial losses" of Mavics near Kupiansk (Dec 2024). Russia switched to mechanised assaults to exploit it. Wet weather "disadvantaged" light FPVs at Pokrovsk (2025). Assaults along belts are timed to rain, fog, darkness and pre-dawn | [src:euromaidan-2024-mukhina-kupiansk-weather] [src:aljazeera-2025-psaropoulos-pokrovsk] [src:saratoga-2026-parfonov-treelines] |
| Icing and freezing rain | Frozen | Iced branches | Fibre-optic FPVs suffer: ice stops the spool unwinding, iced fibre snaps or sticks to tree limbs, and the SFP modules (rated 0–70 °C) lose 20–50%. A Kyiv training centre grounded its fibre drones in mid-January 2026 | [src:euromaidan-2026-kossov-icy-fibre] |

## 4. Battle damage over time

- **Treelines.** Treelines go from intact to thinned to "piles of split wood" under sustained barrages [src:cepa-2023-kallberg-treelines], and finally to impassable "tangles of fallen timber" [src:saratoga-2026-parfonov-treelines]. A Sentinel-2 study of front-line hromadas in Kharkiv and Donetsk oblasts quantified the loss [src:matsala-2024-war-protective-plantations]:
  - Protective forest damaged: 11.5% by 2022 and 18.1% by 2023 (145 ± 42 km²).
  - Shelterbelts damaged: 15.8% by 2022 and 21.5% by 2023.
  - Where the loss fell: the area-average loss of cropland-protection function was only 1.9% then 2.7%, but it reached **up to 57%** in hotspots near Izium/Balakliia, Vuhledar, Avdiivka and Bakhmut. Bakhmutska hromada was the worst, and landscapes more than 50 km from battles were "mostly unaffected".
  - "Low" damage outnumbers "high" damage, and spectral regrowth of herbs hides broken trees, so optical imagery undercounts damage. Undamaged front-line belts should be treated as possibly mined.
- **Forests.** Between Kharkiv and Luhansk, 24,180 ± 4,715 ha (9.3%) of forest was damaged in April–September 2022, mostly by fires after shelling [src:matsala-2024-war-forest-fire-risk] (e1–e3). Intense outgoing barrages temporarily defoliated nearby oak woods. A journalistic investigation counts over 60,000 ha of forest destroyed nationally in two years, including Russian logging for fortifications [src:euromaidan-2024-ngl-forest-destruction]. Serebrianka was "severely damaged" over three years [src:wikipedia-serebriansky-forest].
- **Villages.** Towns get erased over time. Marinka (about 9,400 residents) went from damaged in May 2022 to "few buildings left standing" by November 2022 and "not a single surviving house" by December 2022, and the battle lasted about 20 months [src:wikipedia-battle-of-marinka-2022]. Toretsk's centre was levelled into "battles among ruins" [src:euromaidan-2024-toretsk-highrises].
- **Craters.** In 2014 imagery of Donetsk oblast, 22,000+ craters were found across 858 km². That is about 26 craters per km², covering 0.14% of the area. It is a *low* pre-2022 baseline, and e4 (Bakhmut) and e6 (Avdiivka) hotspots should be far denser [src:umd-2023-duncan-craters]. For scale in e1, about one month of fighting in Kyinska hromada (Chernihiv oblast) left 4,914 craters, 2,912 of them on arable land, 0.5–13.8 m across [src:bonchkovskyi-2023-kyinska-craters]. No 2023–26 crater-density study for Donbas or Zaporizhzhia was found.
- **Landscape transformation.** The Kakhovka reservoir went from water to mud to forest (§2.4). Front-line fields were abandoned [src:sadeh-2025-ukraine-field-boundaries]. By 2024, 7% of Ukraine's cropland was abandoned, mostly along the front, and fallow grew most in territory retaken from occupation [src:wagner-2025-cropland-abandonment].
- **Satellite view across years.** Open 10 m imagery gives a year-by-year record:
  - Sentinel-2 (L2A from 2017, 5-day revisit) [src:copernicus-sentinel-2]
  - Dynamic World (from 2015, per scene) [src:google-dynamic-world]
  - Hansen tree-cover loss (annual to 2024/25, 30 m) [src:umd-hansen-gfc]
  - ESA WorldCover 2021, a pre-invasion baseline [src:esa-2021-worldcover]
  - Both DEMs predate 2022 (SRTM in 2000, Copernicus in 2011–2015), so they show pre-war terrain [src:usgs-srtmgl1] [src:copernicus-dem-glo30]

  A 10–25 m shelterbelt is only 1–2 pixels wide at 10 m, so treeline loss is detectable only coarsely. It is nonetheless workable: the study above mapped belts from 10 m pixels by patch shape (perimeter-to-area ratio above 0.07, at least 0.3 ha) [src:matsala-2024-war-protective-plantations].
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
| Microsoft Global ML Building Footprints | 1.4 B building polygons (174 M with height), imagery 2014–2024; in the sample boxes about 5–125× more buildings than OSM | vector | global | CDLA Permissive 2.0 per repo (Overture lists ODbL; check) | [src:microsoft-global-ml-building-footprints] |
| EOxCloudless (Sentinel-2 cloudless) 2021 | cloud-free RGB mosaic, pre-invasion; WMTS tiles | 10 m | global | EOX Commercial Attribution-RestrictedUse (paid licence for commercial use); reference and analysis only | [src:eox-2021-s2cloudless] |
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
| Belt grid, front-line steppe (Parfonov) | 300–400 × 1,000–2,000 | m | saratoga-2026-parfonov-treelines |
| Belt rows / width / machinery gaps (Parfonov) | 3–4 / ≤20–30 / 10–30 | rows / m / m | saratoga-2026-parfonov-treelines |
| Main-belt spacing, Huliaipole E, OSM median (p10–p90); Sentinel-2 | 549 (379–990); 547 | m | osm-overpass-2026-landscape-sample; eox-2021-s2cloudless |
| Cross-belt spacing, Huliaipole E / Robotyne (imagery) | ~1,100–1,300 / ~1,000–1,200 | m | eox-2021-s2cloudless |
| Mapped belt canopy width, OSM strips (box medians) | 20–27 | m | osm-overpass-2026-landscape-sample |
| Mapped belt density, best-mapped boxes | 1.34–1.35 | km per km² | osm-overpass-2026-landscape-sample |
| Windbreak length, Kherson oblast | ~30,000 | km | sudnik-2006-windbreaks-southern-ukraine |
| Farmland polygon median area, 4 grid boxes (Chasiv Yar S 14) | 38–65 | ha | osm-overpass-2026-landscape-sample |
| Farmland MBR median short × long | 420–625 × 920–1,350 | m | osm-overpass-2026-landscape-sample |
| Farmland aspect ratio, median | 1.8–2.0 | — | osm-overpass-2026-landscape-sample |
| Track / primary–tertiary road density (where mapped) | 0.3–0.9 / 0.13–0.22 | km per km² | osm-overpass-2026-landscape-sample |
| Buildings per 100 km² (MS footprints, 5 boxes) | 815–3,833 | buildings | microsoft-global-ml-building-footprints |
| Settlements ≥20 buildings per 100 km² | 5–11 | settlements | microsoft-global-ml-building-footprints |
| Small settlement (<300 bldg) median size / elongation | 71 bldg, 755 × 254 m / 4.1 | — | microsoft-global-ml-building-footprints |
| Large village (≥300 bldg) median size / elongation | 595 bldg, 2.2 × 1.0 km / 2.3 | — | microsoft-global-ml-building-footprints |
| OSM building completeness vs MS, sample boxes | 0.8–18 | % | osm-overpass-2026-landscape-sample |
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
| Craters, Kyinska hromada, ~1 month of 2022 fighting | 4,914 (2,912 on arable land), 0.5–13.8 m across | craters | bonchkovskyi-2023-kyinska-craters |
| Shelterbelts damaged, E Ukraine front hromadas, 2022 → 2023 | 15.8 → 21.5 | % | matsala-2024-war-protective-plantations |
| Cropland-protection loss, area average / hotspot max (2023) | 2.7 / 57 | % | matsala-2024-war-protective-plantations |
| Forest damaged Apr–Sep 2022, Kharkiv–Luhansk AOI | 24,180 ± 4,715 (9.3%) | ha | matsala-2024-war-forest-fire-risk |
| Cropland abandoned in Ukraine, 2024 | 7 (~2.2 Mha) | % | wagner-2025-cropland-abandonment |
| Serebrianka forest occupied share, 1 Aug → 1 Sep → 1 Oct → 20 Dec 2025 | 15 → 64 → 90 → 100 | % | deepstate-api-history |
| Kupiansk gas-pipeline infiltration route | ~10 | km | euromaidan-2026-zoria-kupiansk-pipeline |
| Mokri Yaly length / basin | 147 / 2,660 | km / km² | wikipedia-mokri-yaly |
| Parfonov assault group depth along a belt | 300–500 | m | saratoga-2026-parfonov-treelines |
| Infiltration group size, Pokrovsk (Jul–Aug 2025) | ~50 (30 of 150 got through) | soldiers | wikipedia-battle-of-pokrovsk |
| Winter low, 2025–26 | −20 | °C | united24-2026-brizard-winter-camouflage |
| Khrushchevka height | 4–5 | storeys | wikipedia-khrushchevka |
| WorldCover / Dynamic World / Sentinel-2 resolution | 10 | m | esa-2021-worldcover |
| Copernicus DEM / SRTM resolution | 30 | m | copernicus-dem-glo30 |
| Copernicus DEM absolute vertical accuracy | <4 | m (LE90) | copernicus-dem-glo30 |

## 8. Game/sim relevance

- **Belt grid.**
  - Main belts: generate them perpendicular to a per-map prevailing-wind axis (east winds in Luhansk give N–S belts [src:tarasov-usda-luhansk-wind-erosion]). Snap to near-cardinal in the south (Huliaipole, Robotyne) and allow oblique grids in the forest-steppe (Kupiansk) [src:osm-overpass-2026-landscape-sample]. Sample the spacing around 550 m (p10–p90 about 380–990 m; Parfonov's lower 300–400 m is a valid tight variant) [src:osm-overpass-2026-landscape-sample] [src:saratoga-2026-parfonov-treelines].
  - Cross-belts: spacing about 1.0–1.3 km (Parfonov: 1–2 km) [src:eox-2021-s2cloudless] [src:saratoga-2026-parfonov-treelines].
  - Belt form: 3–4 tree rows, 15–30 m of canopy and 15–23 m tall [src:saratoga-2026-parfonov-treelines] [src:maliuha-2023-windbreaks]. Give each belt 10–30 m machinery gaps at field corners [src:saratoga-2026-parfonov-treelines] and a further *gap* probability for degraded Soviet belts [src:unfccc-ukr-agroforestry-brief].
  - Checksum: the resulting belt density is about 2.5–3 km per km² of cropland.
- **Fields and roads.** Belt-bounded blocks of about 0.55 × 1.2 km are OBB-split along the long axis into one to three crop fields. The target is a median field of about 40–65 ha, aspect about 1.8–2.0 and p90 about 100–170 ha [src:osm-overpass-2026-landscape-sample] [src:vanegas-2012-parcels]. Place dirt tracks on some belt edges and some field diagonals, at 0.3–0.9 km/km². Paved roads run only between settlements, at about 0.15–0.2 km/km², routed by terrain cost [src:osm-overpass-2026-landscape-sample] [src:galin-2010-roads]. Near the front, flip fields to fallow and weeds with probability rising by era, to about 7% nationally but most of the front band by e8 [src:wagner-2025-cropland-abandonment].
- **Terrain skeleton.** Derive balkas and rivers from DEM flow accumulation (Copernicus GLO-30 or SRTM) [src:copernicus-dem-glo30]. Make right banks high and gullied with chalk outcrops, and left banks low with wetlands and pine-on-sand terraces [src:wikipedia-siverskyi-donets]. Place natural woods only in balkas and floodplains [src:tarasov-usda-luhansk-wind-erosion]. Villages follow valleys and balka heads [src:emilien-2012-villages] [src:eox-2021-s2cloudless]. Seed 5–11 settlements per 100 km². Most are linear one- or two-street strips of about 70 houses, 0.75 × 0.25 km with an elongation of about 4. About one in four is a compact grid village of about 600 houses, 2 × 1 km [src:microsoft-global-ml-building-footprints]. Treat small dams, gas pipelines and culverts as special terrain objects: flood triggers and covered infiltration routes [src:euromaidan-2023-mokri-yaly-dam] [src:euromaidan-2026-zoria-kupiansk-pipeline]. Rivers such as the Kinska and Oskil pair a low crossing bank with heights beyond [src:euromaidan-2026-axe-mala-tokmachka]. Mining towns get 1–3 terrikons of 40–100+ m as line-of-sight dominators [src:euromaidan-2024-toretsk-highrises].
- **Seasonal modifiers** (a monthly table):
  - Treeline concealment: high in Jun–Sep, dropping in Oct–Nov, low in winter [src:cepa-2023-kallberg-treelines].
  - Snow: tracks become persistent, detectable trails [src:united24-2026-brizard-winter-camouflage].
  - Cold: thermal detection bonus [src:united24-2026-brizard-winter-camouflage].
  - Mud: off-road vehicle mobility near zero in Mar–early Apr and in Oct–Nov, with a random dry-out date around mid-April [src:spire-2025-soil-moisture].
  - Fog: random events, often in the morning, that ground drones and open infiltration windows. At Pokrovsk in November 2025 one fog spell let more than 300 troops through [src:wikipedia-battle-of-pokrovsk] [src:euromaidan-2025-axe-foggy-advance].
  - Rain and frost: raise the loss rate of light quadcopters (Mavic class) and FPVs, and let the attacker commit vehicles [src:euromaidan-2024-mukhina-kupiansk-weather].
  - Icing: an extra penalty for fibre-optic drones (spool jams, snapped cable, reduced range) [src:euromaidan-2026-kossov-icy-fibre].
  - Treeline assaults: the AI should favour the longitudinal attack along a belt, launched in bad visibility [src:saratoga-2026-parfonov-treelines].
- **Damage by era.** Make crater density per km² a function of era and hotspot, starting from a floor of about 26/km² [src:umd-2023-duncan-craters]. Treeline state goes from intact to thinned to split wood to impassable deadfall [src:cepa-2023-kallberg-treelines] [src:saratoga-2026-parfonov-treelines]. Calibrate it with the Matsala figures: about 16% of belts damaged after the first year of fighting and about 22% after two. Damage should concentrate in hotspot hexes near the main battles, with up to 57% loss of function there, and be near zero more than 50 km from the fighting [src:matsala-2024-war-protective-plantations]. Village state goes from intact to damaged to razed, over about 8 months of sustained fire in the Marinka example [src:wikipedia-battle-of-marinka-2022]. The Kakhovka tile changes by era: water to mudflat (e5) to willow forest (e8–e9) [src:uanimals-2026-kakhovka-forest]. The contested-zone width grows from about 1 km (e7) to 5–8 km (e9) [src:wotr-2026-maurin-front-line].
- **Licensing for shipped assets.** If generated maps are *derived* from OSM or Overture, ODbL share-alike applies to the derived database [src:osm-copyright] [src:overture-maps-attribution]. Prefer CC BY sources (WorldCover, Dynamic World, Hansen) for baked data. Use NASA Harvest (NC-ND) and EOxCloudless imagery only for offline statistics. The measured statistics in §2.7 are facts about OSM data. Using them as generator parameters does not ship an ODbL-derived database.

## 9. Terms

- **lisosmuha** (лісосмуга, pl. *lisosmuhy*) — shelterbelt, a planted windbreak strip between fields; Russian *lesopolosa*
- **lisnytstvo** (лісництво) — forestry district, a named managed forest block, as in "Serebrianske lisnytstvo" (DeepState's "Serebryanske forestry")
- **longitudinal vs perpendicular attack** — assaulting along a shelterbelt's interior from its flank, vs crossing the open field into it (Parfonov)
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

- **Soviet belt design norms.** Spacing as multiples of tree height and the official cross-belt rules were still not found. The measured grid (§2.7) and Parfonov now replace the search-only Oleshky figure as the basis. The forestry-journal site refused connections again, the MDPI Ukraine shelterbelt papers (Land 2025, Resources 2025) returned 403, and Ukrainian-language norms were out of scope.
- **Measurement coverage.** Five boxes is a small sample, and OSM completeness varies. The next step is to run the same method over the NASA Harvest field polygons offline (NC-ND, statistics only) or over a belt layer derived from WorldCover or Dynamic World.
- **Balka dimensions.** No source gives typical balka widths, depths or slope angles. Measure them from Copernicus GLO-30 along the sample boxes. Chalk hills beyond the Sviati Hory area are unsourced.
- **Weather effects.** Rain, fog and icing are now sourced qualitatively. Wind limits, summer dust and any *quantified* foliage-versus-detection relationship were not found. Parfonov says only that canopy "substantially" degrades FPV effectiveness, and a Resources 2025 paper mentions rising dust storms without figures.
- **Crater density 2023–26.** No crater-density study for Donbas or Zaporizhzhia after 2022 was found. The Kyinska hromada count (e1) lacks an area, so it gives no density. UNOSAT's product pages render by JavaScript and could not be listed.
- **Village destruction over time.** Beyond the Marinka narrative there is no settlement-level damage time series. Sentinel-1 coherence methods exist but were not applied.
- **Dataset corrections to the brief.** No EuroCrops Ukraine subset exists. FTW and AI4Boundaries exclude Ukraine. The Microsoft footprint licence is inconsistent (CDLA-P 2.0 in the repo vs ODbL at Overture).
- **Process note.** The web-search quota ran out, so later research used OpenAlex, publisher site searches, the DeepState endpoints, Overpass, Microsoft footprints and EOX tiles. ISW pages returned 403. The procgen literature on field patterns and hedgerows is still thin.

## 11. Sources

- `eou-shelterbelt` — Encyclopedia of Ukraine: shelterbelt history (1809, 1938–39, 1948–52), species.
- `uncg-2022-shamina-stalin-plan` — Ukrainian NGO: 1948 plan in Ukraine, belt areas planted, Kakhovka flooding and Velykyi Luh.
- `unfccc-ukr-agroforestry-brief` — Ukraine TNA brief: ~400k ha of belts, ownership and degradation, restoration target.
- `maliuha-2023-windbreaks` — Ukrainian forestry paper: 15 m width, 19.6–23.4 m height, oak-nest design.
- `tarasov-usda-luhansk-wind-erosion` — Luhansk study: east winds, ~10 m optimal width, woods in gullies, leafless winter.
- `ffm-oleshky-shelterbelts` — Kherson oblast belt spacing 350–780 m (search-only; corroborated by §2.7).
- `cepa-2023-kallberg-treelines` — treeline warfare, foliage vs leaf-fall, belts shredded (e5).
- `saratoga-2026-parfonov-treelines` — Ukrainian soldier-analyst's report (full PDF): belt grid, rows, gaps, longitudinal attack, assault-group depth, bad-weather timing.
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
- `osm-overpass-2026-landscape-sample` — OSM via Overpass, five 10 × 10 km boxes queried 2026-09-28 (ODbL): belts, fields, roads, places.
- `eox-2021-s2cloudless` — 10 m Sentinel-2 cloudless 2021 tiles, used as a completeness and spacing check; restricted licence.
- `deepstate-api-history` — DeepState changelog and per-version polygons; Serebrianka forest occupation timeline 2023–26.
- `matsala-2024-war-protective-plantations` — peer-reviewed: 18% of protective plantations and 21.5% of shelterbelts damaged by 2023; hotspots to 57%.
- `matsala-2024-war-forest-fire-risk` — peer-reviewed: 9.3% of the Kharkiv–Luhansk forests damaged in 2022, mainly shelling fires.
- `wagner-2025-cropland-abandonment` — peer-reviewed: 7% of cropland abandoned by 2024, mostly along the front.
- `bonchkovskyi-2023-kyinska-craters` — 4,914 craters from one month of fighting in a Chernihiv hromada (e1).
- `sudnik-2006-windbreaks-southern-ukraine` — 30,000 km of windbreaks in Kherson oblast; gaps and steppe colonisation.
- `euromaidan-2024-ngl-forest-destruction` — NGL.media: >60,000 ha of forest destroyed; Sviati Hory about 80% devastated.
- `euromaidan-2023-avdiivka-rain-fog` — Russian assaults timed to rain and morning fog (e6).
- `euromaidan-2024-mukhina-kupiansk-weather` — rain and frost cost Mavics; mechanised assaults exploit the weather (e7).
- `euromaidan-2025-axe-foggy-advance` — fog lets 300+ troops into Pokrovsk (e8).
- `euromaidan-2026-kossov-icy-fibre` — icing degrades fibre-optic drones (e9).
- `aljazeera-2025-psaropoulos-pokrovsk` — wet weather disadvantages light FPVs (e8).
- `euromaidan-2023-rivnopil-heights` — elevated platforms and a gully decide Rivnopil (e5).
- `wikipedia-mokri-yaly` — Mokri Yaly river length, basin, 1943 and 2023 fighting.
- `euromaidan-2023-mokri-yaly-dam` — dam blown near Novodarivka to slow the 2023 counteroffensive (e5).
- `euromaidan-2025-mukhina-oskil-crossing` — Oskil bridges destroyed, pontoons targeted (e7).
- `euromaidan-2026-zoria-kupiansk-pipeline` — a 10 km gas pipeline as an infiltration route into Kupiansk (e8–e9).
- `euromaidan-2026-axe-mala-tokmachka` — Russian crossing of the Kinska towards the heights near Orikhiv (e9).
