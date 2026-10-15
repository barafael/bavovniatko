<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# 09 — Bevy rendering and simulation prior art

Research pack for **bavovniatko**: what exists in the Bevy ecosystem (as of Bevy 0.19, surveyed
2026-10-02) for the engine problems this game raises — landscapes, sea, weather, clouds, rather
large maps, and rapid camera changes between drone, A-50/other AWACS, and ground AGV views. This
pack records prior art and techniques, not history.

## 0. Decisions this pack supports

- **Target: Bevy 0.19 stable** (0.19.1, 2026-08-13) [src:bevy-releases] [src:bevy-0.19-release].
  Bevy 0.20.0 shipped on 2026-10-08, after this survey, following release candidates of 15 and
  28 September; the plugins below were checked against 0.19, so 0.19.1 stays pinned until they
  follow [src:crates-io-bevy].
- **World architecture: per-chapter dioramas plus a 2D country map**, not one continuous
  country-scale 3D world. Dioramas are 10–100 km; local origins keep `f32` transforms precise
  enough (a 32-bit float has about 1.6 cm of resolution at 200 km from origin), so floating-origin
  machinery is not a v1 dependency [src:big-space].
- **Visual target: sim-first, drab utilitarian.** Sensor looks (FLIR, night vision, FPV feed
  artefacts, radar scope) are first-class build items, not polish. No ready-made plugin exists for
  any of them; each is one fullscreen post-process system + shader, which Bevy 0.19 made simple
  (render passes are now plain ECS systems in the `Core3d`/`Core2d` schedules) [src:bevy-0.19-release].
- **Integration:** loaded into the knowledge base as topic 09 on 2026-10-10; this file is now generated from it.

## 1. Engine baseline (Bevy 0.19)

Facts from the 0.19 release notes and the 0.19.1 example tree that this design leans on [src:bevy-0.19-release] [src:bevy-examples-tree] [src:bevy-releases]:

- **Large-scene rendering.** GPU-driven multi-draw indirect with GPU-side batch-set unpacking,
  GPU-checked visibility ranges, batched depth-only prepasses, sparse mesh-uniform uploads, and
  GPU light clustering. Official `many_cubes` numbers: 1.6M cube entities with PBR materials went
  from 49.5 ms to 18.8 ms per frame with culling (116k in view), and 93.1 ms to 41.2 ms unculled.
  The `large_scenes/bevy_city` example generates 55k rendered entities by default.
- **Custom post-processing is now ECS.** The old render graph is gone; render passes are regular
  systems added to `Core3d`/`Core2d` with normal `.before()`/`.after()` ordering. Built-in
  **Vignette** and **LensDistortion** (barrel/pincushion) components were added in 0.19 — the base
  of the FPV-drone camera look.
- **Lighting.** Contact shadows (short-range screen-space ray march, per-light toggle),
  physically based screen-space reflections, rectangular area lights, parallax-corrected
  cubemap probes, partial bindless (significant Metal wins).
- **Atmosphere.** Bevy 0.19 carries the built-in Bruneton-style atmosphere used by bevy-aqua and
  bevy_weather: an `Atmosphere` component with a `ScatteringMedium` (e.g. `ScatteringMedium::earth`),
  `AtmosphereEnvironmentMapLight` for scene lighting from the sky, and `AtmosphereSettings`. The
  sky composites between the opaque and transparent passes as
  `destination * transmittance + inscattering`, in physical radiance multiplied by camera exposure
  (both plugins calibrate around EV100 12–13) [src:bevy-aqua] [src:bevy-weather].
- **Fog inventory** (all built-in examples): `fog.rs` (distance fog), `fog_volumes.rs` (local fog
  boxes), `volumetric_fog.rs`, `scrolling_fog.rs`, `atmospheric_fog.rs` [src:bevy-examples-tree].
  Volumetric fog requires WebGPU on the web; WebGL2 cannot run it [src:bevy-weather].
- **Solari**, the real-time path-traced renderer, improved in 0.19 (temporal stability, mirrors,
  non-metallic materials). Relevant later, not for v1 [src:bevy-0.19-release].
- **BSN scenes** (new scene notation) make spawning large generated content (fields of buildings,
  belts, villages) far less boilerplate; asset-driven `.bsn` loaders are still ahead [src:bevy-0.19-release].

## 2. Landscape / terrain

| prior art | bevy | what it gives us | source |
|---|---|---|---|
| bevy-clipmap 1.0.5 | 0.19 (README table) | Hoppe-style GPU geometry clipmaps: one huge heightmap rendered at adaptive LOD. Ships a Python pipeline (16-bit PNG → KTX2, plus horizon maps for terrain self-shadowing; horizon maps cost 360·W·H·4 bytes — 1.4 GB at 1k, prohibitive past ~8k) | [src:bevy-clipmap] [src:hoppe-2004-gpugcm] |
| bevytiles 0.1.0 | 0.19 | Geo-spatial tile streaming engine (a port of raytiles), built for "UAV simulations, flight-planning, lightweight GIS, digital sand tables": `WorldConfig::from_lat_lon`, quadtree LOD policy (snapshot-tested against the C++ original), Terrarium heightmap decode + ancestor synthesis to zoom ≤ 22, uint16 bilinear `ground_height()` CPU queries, worker-thread tile fetch + disk cache (wasm via `fetch`), large-world rebase convention via `TerrainAnchor`. Streams Esri imagery and Mapzen/AWS terrain tiles — those terms must be respected; the architecture is what we want, fed by our own baked tiles | [src:bevytiles] |
| bevy_3d_tiles 0.4.4 | native+wasm | OGC 3D Tiles streaming: packed `.3tz` ranged reader, per-tile SSE traversal, glTF/b3dm, KTX2 + meshopt, ECEF georeferencing, Google Photorealistic 3D Tiles. crates.io lists it with no stable channel (preview) — reference, not dependency | [src:bevy-3d-tiles] |
| terraink 0.0.2 | n/a | Small Bevy renderer for DEMs (GeoTIFF, Terrarium PNG) and Mapbox vector tiles — reference for the Copernicus GLO-30 ingest path | [src:terraink] |
| bevy_mesh_terrain 0.15.11 | 0.15 (lagged) | Ergonomic chunked heightmap terrain with splat mapping; companion `bevy_regions` (u16 region-paint texture, 0.16) and `bevy_foliage_paint`. Good code to mine for chunking and splatting patterns | [src:bevy-mesh-terrain] [src:bevy-regions] |
| bevy_symbios_ground 0.5.0 | 0.19-era | Active (2026-08) algorithmic terrain engine integration; watch, small adoption | [src:bevy-symbios-ground] |
| bevy_a5 0.1.2 | 0.19-era | A5 geospatial pentagonal cells for floating-origin use and spatial queries — alternative spatial partitioning if dioramas ever merge | [src:bevy-a5] |
| built-ins | 0.19 | `visibility_range.rs` (per-mesh LOD swaps, now GPU-checked), `occlusion_culling.rs`, `meshlet.rs` | [src:bevy-examples-tree] |

**Fit for our measured landscape** (belt-bounded blocks ≈ 0.55 × 1.2 km, fields 38–65 ha median,
gullies from DEM flow accumulation — research/06 §8): bake a tile pyramid per diorama from
Copernicus GLO-30 + CC BY land-cover layers (WorldCover/Dynamic World/Hansen), render via clipmap
(single-scene, cheap) or the bevytiles quadtree model (when one scene spans two height sources),
and keep a CPU heightfield service for placement and line-of-sight, the way bevytiles'
`ground_height()` works. The procedural references in research/06 §6 (Parish & Müller, OBB
parcel splitting, terrain-aware villages, cost-routed roads) stay the generation literature.

## 3. Vegetation (treelines, grass, fields)

| prior art | bevy | notes | source |
|---|---|---|---|
| bevy_feronia 0.8.4 | 0.17 (lagged) | The strongest scatter system: declarative scatter layers, LOD-matched items, wind simulation shaders (explicitly inspired by Sucker Punch's Ghost of Tsushima grass/wind GDC talks), heightmap baking, ordered scattering (rocks → trees → grass). main branch pins bevy_scene/pbr/… 0.17.2 — mine the WGSL, do not depend yet | [src:bevy-feronia] |
| warbler_grass 0.6.1 | ~0.13/0.14 (outdated) | The classic dense GPU billboard grass; outdated but the canonical approach | [src:warbler-grass] |
| bevy_procedural_tree 0.4.0 | 0.17-era | Procedural 3D tree meshes — candidate for oak/pine/poplar/birch belt variety (species per research/06 §2.1) | [src:bevy-procedural-tree] |
| built-ins | 0.19 | 0.19 GPU-driven batching makes plain instanced-mesh vegetation viable at extreme counts | [src:bevy-0.19-release] |

**Recommended shape:** a shelterbelt = one instanced mesh batch (a few hundred tree blobs) plus
canopy cross-billboards with a wind vertex shader; damage states (intact → thinned → split wood →
impassable deadfall, research/06 §4) are mesh/texture swaps driven by era and hotspot intensity.
This is small custom work informed by feronia's shaders; nothing to import wholesale.

## 4. Water: sea, rivers, reservoirs

| prior art | bevy | what it gives us | source |
|---|---|---|---|
| bevy-aqua 0.1.3 | 0.19 | Camera-centred ocean, the most complete water stack in the ecosystem: five displacement cascades with smooth LOD blending; analytic (Crest-style) or Tessendorf spectral (FFT) waves; persistent whitecaps + shoreline foam; `BedHeightMap` terrain bed for shoaling and shallow-water optics; **bounded water bodies** (`WaterBody` + `WaterShape::Circle/Polygon/corridor/river`) with per-body `WaterOptics` presets — i.e. the Dnipro, the Kakhovka reach, and steppe rivers as first-class objects; GPU `WaveQuery` surface probes (256/frame, ~1 frame readback latency) for buoyancy and sea-drone gameplay; planar reflections (opt-in screen-space), procedural caustics, opt-in underwater volume scattering + underside shading, budgeted Hanabi spray; WebGPU/wasm demos. Constraints, documented by the author: one active `Camera3d` is the supported view path (multi-camera/PiP untested); one horizontal level per body (tilted water rejected); underwater is a homogeneous medium with near-plane-crossing limits; a known bevy 0.19.1 planar-mirror normal-polarity bug | [src:bevy-aqua] |
| bevy_water 0.18.1 | 0.18 (lagged) | Long-lived (since bevy 0.8) tileable dynamic ocean material: vertex waves, tileable for endless ocean, `get_wave_point()` for floating objects, imports `bevy_pbr` lighting/shadows. Camera-agnostic and simple — the fallback where multi-view scenes break aqua | [src:bevy-water] |

**Chapter mapping.** c02/c13 (Snake Island, Black Sea drone war): aqua open ocean; underwater
feature for sea-drone POV. c04/c10 (Kherson crossings, Krynky): aqua river corridor over a Dnipro
bed heightmap. The Kakhovka tile changes state by era (open water → mudflat → willow forest,
research/06 §2.4): swap `WaterBody` shapes and optics per era, with the reservoir reach removed
after e5. Bevy's built-in `transmission.rs` example covers shallow river refraction for minor
streams without aqua [src:bevy-examples-tree].

## 5. Sky, clouds, weather

| prior art | bevy | what it gives us | source |
|---|---|---|---|
| bevy_weather 0.2 | 0.19 | The standout, and almost a spec of our needs: day/night from real solar geometry (latitude, axial tilt, date → seasons and day length); Bevy's Bruneton atmosphere wired and quality-scaled; procedural stars (magnitude distribution, colour temperature, sidereal drift), Milky Way, phase-correct moon, meteors; raymarched volumetric clouds through a curved planetary shell (light marching, Henyey–Greenstein phase, powder, wind advection) with **cloud shadows projected through a light cookie** (`DirectionalLightTexture`, needs `pbr_light_features`/binding arrays, degrades gracefully); volumetric fog + god rays driven by weather; GPU-resident rain/snow (one draw call each); Poisson-scheduled lightning with speed-of-sound thunder delay; **`GroundConditions`** — persistent wetness/snow (ground stays wet after rain; snow melts into wetness near freezing) — i.e. rasputitsa as data; **deterministic: weather is a pure function of clock + seed**, with `forecast()` — same sky after save/load, on every machine, which suits a replayable sim; climates (temperate/oceanic/arid/tropical/mediterranean/polar/desert) or field-by-field manual control; `WeatherTime` can be paused and scrubbed (`time_of_day` from any timeline); `DayPhaseChanged` and `LightningStrike` messages; quality tiers Potato→Ultra (cloud steps 12→128, light march 0→8, particles 1.2k→60k). Cost notes: nearly all cost is the per-pixel cloud raymarch; broken (transitional) sky is the expensive case; biggest levers are `steps`, `light_steps = Some(0)` (analytic light), `detail_distance`, and cloud-shadow texture resolution (CPU-rebuilt: ~1.5 ms@64, 6 ms@128, 20 ms@256, spread over frames) | [src:bevy-weather] |
| bevy-volumetric-clouds 0.2 | 0.18 (lagged) | Horizon Zero Dawn-method raymarched clouds (Schneider/Vos + Hillaire references); limits: own simple sky (no Bevy atmosphere integration), no depth-buffer integration (ground views only), fixed 1920×1080 render target | [src:bevy-volumetric-clouds] [src:guerrilla-hzd-cloudscapes] [src:frostbite-2016-sky-clouds] |
| bevy_fast_clouds 0.19 | 0.19 | Wavelet-volume clouds that skip provably invisible work (stripe directions parallel to the ray cancel; sub-pixel octaves drop): fog-volume-style rasterized boxes, `LodPolicy` by distance; planet-approach demo. Very new (2026-09), tiny adoption — an alternative if bevy_weather's cloud cost bites | [src:bevy-fast-clouds] |
| bevy_hanabi 0.19.0 | 0.19 | GPU particle system for smoke columns, dust, artillery blooms, tracers; RON asset loader available | [src:bevy-hanabi] [src:bevy-hanabi-loader] |
| bevy_stage 0.2.0 | 0.19-era | glTF levels with artist-authored metadata, streaming sectors and LOD (same author as bevy_weather) — sector-streaming reference | [src:bevy-stage] |

**Implementation notes worth keeping** (from bevy_weather's architecture discussion)
[src:bevy-weather]: stars/galaxy/moon are drawn in the opaque pass at the far plane with depth
writes off, so the atmosphere extinguishes them correctly; clouds must be drawn in the transparent
pass *after* the atmosphere with their own aerial perspective (otherwise overcast reads brighter
than clear); the sky renders for **every 3D camera** — minimap and render-to-texture cameras must
be scoped out with render layers; fog visibility is `DistanceFog` + a matching sky fade (blend
toward colour, so white-out not black-out), with volumetric fog only adding god rays; hand the
sun raw solar spectrum (pre-reddened light double-counts Rayleigh extinction and turns sunsets
olive).

**Weather → gameplay mapping** (research/06 §3): fog events ground drones and open infiltration
windows (Pokrovsk, Nov 2025) → fog volumes + detection modifiers; rasputitsa → `GroundConditions`
wetness gates off-road mobility Mar–Apr and Oct–Nov; snow makes tracks persistent and detectable;
icing penalises fibre-optic FPVs; foliage season gates treeline concealment Jun–Sep. The seasonal
table lives in our sim layer; bevy_weather supplies the sky and the ground-state accumulators.

## 6. Large maps, coordinates, streaming

| prior art | bevy | notes | source |
|---|---|---|---|
| big_space 0.13 | 0.19 | THE floating-origin crate: world partitioned into nestable integer grids (i8–i128), absolute coordinates without drift (unlike camera-relative recentering), plain `Transform` compatibility, spatial hashing + partitioning, proton-to-observable-universe demo. 37k downloads, active. At the 2026-10-02 access the latest release was 0.12 (bevy 0.18); 0.13.0 for bevy 0.19 and 0.14.0-rc.0 for bevy 0.20 followed on 2026-10-08 | [src:big-space] [src:crates-io-big-space] |
| big_space_solari 0.2 | 0.19-era | Companion: resets ray-traced lighting temporal history when a floating origin recenters; optional scale-bias keeps ray-traced geometry artefact-free at large coordinates — the general lesson: recentering must reset temporal GPU effects | [src:big-space-solari] |
| chunky-bevy 0.18 | ~0.18 | Simple chunk management (load-in/load-out) | [src:chunky-bevy] |
| bevy_spatial 0.11 | ~0.16 (lagged) | Entity tracking in spatial indices (KD-tree) for nearest-neighbour sim queries | [src:bevy-spatial] |
| utm 0.1.6 | n/a | lat/lon ↔ UTM in ~300 lines, MIT, `no_std`-capable — converting our OSM/GLO-30 sample boxes to local metres | [src:utm-crate] |
| bevytiles rebase | 0.19 | `TerrainAnchor` pattern: `absolute = user − world_offset` for large worlds without floating origin | [src:bevytiles] |

**Decision recorded in §0:** diorama-local origins make big_space unnecessary for v1. It becomes
relevant only if dioramas merge into one continuous world later — in which case also budget for
the temporal-effect-reset lesson [src:big-space-solari].

## 7. Cameras: drone, AWACS, AGV, rapid cuts, sensor looks

**Primitives, all in the 0.19.1 example tree** [src:bevy-examples-tree]:

- `split_screen.rs` — two cameras, two viewports (order + `Viewport`): AWACS main + tactical inset.
- `render_to_texture.rs` + `camera/2d_on_ui.rs` — render a camera to an `Image`, composite into UI:
  the **PiP drone-feed** pattern (a UAV video tile inside the command view), with per-camera render
  layers scoping what each camera sees.
- `camera_sub_view.rs` — different rendering per sub-view region (EO/IR split feed reference).
- `orthographic.rs` — top-down AWACS/radar projection; `custom_projection.rs`,
  `projection_zoom.rs` — sensor-zoom FOV work.
- `camera/free_camera_controller.rs`, `camera_orbit.rs`, `first_person_view_model.rs` —
  free/orbit/FPV controllers.
- Post chain per camera: `Vignette`, `LensDistortion` (built-ins), `color_grading.rs`,
  `tonemapping.rs`, `auto_exposure.rs`, `motion_blur.rs`, `post_processing.rs` (custom pass
  template) [src:bevy-0.19-release] [src:bevy-examples-tree].

**Camera-controller prior art:** bevy_rts_camera 0.14 (bevy 0.19, MIT/Apache, ~100★) —
pan/zoom/rotate, `Ground`-marker ground-following, smoothing, edge pan; the ground-command /
drone-operator station view [src:bevy-rts-camera].

**Map/overlay UIs for the AWACS and Delta-style views:**

- milsymbol 0.3.0 — native Rust port of milsymbol.js: MIL-STD-2525 / APP-6 symbol generation
  (2026-09, brand new). Matches research/07's recommendation (milsymbol, MIT). An alternative,
  `milsymbol-rs`, wraps the JS library [src:milsymbol-rust] [src:milsymbol-rs-js].
- bevy_geo_tiles 0.2.1 / bevy_map_viewer 0.2.1 — slippy-map (xyz raster) tiles in Bevy: briefing
  screens, the 2D country map background [src:bevy-geo-tiles] [src:bevy-map-viewer].
- evroon's bevy-adsb — renders live ADS-B aircraft traffic; useful reference for the A-50 hunt's
  air-picture feel [src:evroon-bevy-open-world].

**Sensor looks: a confirmed gap, all custom, all small on 0.19.** Searches found no thermal/FLIR
or night-vision plugins for Bevy. Each is one fullscreen post-process system + WGSL shader added
to `Core3d` after the main pass:

- **FLIR white-hot:** luminance remap + white-hot/black-hot LUT + blur + sensor noise; emissive
  "heat" tagged objects drive it; per-camera so only the sensor view wears it.
- **Night vision:** green gain curve + halo + noise.
- **FPV feed:** built-in `LensDistortion` (barrel, GoPro-like) + `Vignette` + scanlines/interlace
  + grain + occasional glitch; telemetry HUD as UI. rusty-rotor (WIP, tiny) is the only FPV-drone
  prior art found [src:rusty-rotor-fpv].
- **Radar scope:** orthographic top-down + gizmo range rings/sweep + scanline post + milsymbol
  overlay; the A-50 chapter view.

**Multi-view caveat:** every extra camera re-renders the scene it sees. Scope scenes with render
layers per camera, keep feed resolutions low, and test bevy-aqua early in a two-camera scene — its
author documents single-active-`Camera3d` as the supported path [src:bevy-aqua] [src:bevy-weather].

## 8. Whole-game prior art

| project | licence | lesson | source |
|---|---|---|---|
| Digital Extinction (archived 2024) | **AGPL-3.0** | The most complete Bevy RTS: heightmap terrain (`de_terrain` crate), spatial index (`de_index`), pathfinding, minimap, lobby/multiplayer infra. **AGPL: study the architecture, never copy code or assets** | [src:digital-extinction-game] |
| evroon/bevy-open-world | demo code | bevy-osm (OSM buildings/roads rendering), bevy-terrain (quadtree), bevy-adsb (live traffic), bevy-earth (day/night globe), bevy-flight-sim (instruments), bevy-night-sky, bevy-skybox. Author states these are demonstrations, not published crates | [src:evroon-bevy-open-world] |
| iron-curtain (2026) | GPL-3.0 + modding exception | Rust-native C&C-style RTS engine on Bevy; young, watch | [src:iron-curtain] |
| rusty-rotor (2025, WIP) | MIT | FPV drone simulator prototype in Bevy; thin but the only FPV prior art found | [src:rusty-rotor-fpv] |

## 9. Licences and data terms

- All recommended plugins are MIT and/or Apache-2.0 (bevy-aqua, bevy_weather, bevy-clipmap,
  bevytiles, bevy_rts_camera, big_space, bevy_hanabi, utm) — checked per repository
  [src:bevy-aqua] [src:bevy-weather] [src:bevy-clipmap] [src:bevytiles] [src:bevy-rts-camera]
  [src:big-space] [src:bevy-hanabi] [src:utm-crate].
- **Do not ship Esri imagery or Mapzen/AWS terrain tiles** without checking their terms; if we
  borrow bevytiles, we feed it our own baked tiles — consistent with research/06 §8 (prefer CC BY
  sources: WorldCover, Dynamic World, Hansen; Copernicus DEM with attribution).
- Digital Extinction is AGPL-3.0: architecture reading only.
- Google Photorealistic 3D Tiles (via bevy_3d_tiles) has its own terms — not applicable to v1.

## 10. Game/sim relevance — recommended stack

| subsystem | choice | fallback/reference |
|---|---|---|
| Engine | Bevy 0.19.x pinned | 0.20.0 (released 2026-10-08) adopted once the plugins follow |
| Terrain | Baked tile pyramid per diorama (GLO-30 + CC BY layers); clipmap-style renderer; CPU height service (bevytiles' `ground_height()` pattern) | bevytiles architecture; bevy-clipmap; bevy_mesh_terrain code |
| Vegetation | Custom instanced belts + wind billboards; era/hotspot damage states | bevy_feronia WGSL; bevy_procedural_tree meshes |
| Water | bevy-aqua (ocean, river corridors, wave queries, underwater) | bevy_water for multi-camera scenes |
| Sky/weather | bevy_weather (deterministic driver, clouds, precipitation, `GroundConditions`) + built-in atmosphere/fog | bevy-volumetric-clouds, bevy_fast_clouds as alternatives |
| FX | bevy_hanabi | built-in particles |
| Large maps | Diorama-local origins; `f32`; per-camera render layers | big_space (if merging worlds later) |
| Cameras | Switching + PiP via examples' patterns; bevy_rts_camera for ground station | custom_projection/zoom for sensors |
| Sensor looks | Custom `Core3d` post systems: FLIR, NVG, FPV, radar | `post_processing.rs` as template |
| Map UI | milsymbol (APP-6) + bevy_geo_tiles background | bevy_map_viewer |
| Coordinates | utm for lat/lon → local metres | bevy_a5 (future) |

## 11. Terms

- **clipmap** — nested concentric grid rings around the camera giving terrain LOD (Losasso & Hoppe)
- **floating origin** — keeping the render origin near the camera; world position lives elsewhere (big_space: integer grid cells)
- **Tessendorf waves** — FFT spectral ocean synthesis (bevy-aqua's "spectral" mode)
- **Gerstner waves** — sum-of-trochoids analytic waves (bevy-aqua's "analytic" mode)
- **shoaling** — wave behaviour change over shallow bed (aqua `BedHeightMap`)
- **Terrarium tiles** — PNG elevation encoding (RGB → metres), the Mapzen/AWS terrain-tile format
- **slippy map** — the xyz z/x/y raster tile scheme of OSM-style web maps
- **3D Tiles / b3dm / .3tz** — OGC streaming scene format (bevy_3d_tiles)
- **APP-6 / MIL-STD-2525** — NATO military symbology standards (milsymbol)
- **Bruneton atmosphere** — precomputed atmospheric scattering model (Bevy's built-in sky)
- **HZD cloudscapes** — Guerilla Games' Horizon Zero Dawn volumetric cloud method
- **BSN** — Bevy Scene Notation, the 0.19 scene system
- **PiP** — picture-in-picture: a secondary camera rendered to a texture inside the main view

## 12. Open questions / gaps

- **big_space on bevy 0.19** — resolved: 0.13.0 for bevy 0.19 was published on 2026-10-08. Only
  matters if we merge dioramas [src:crates-io-big-space].
- **bevy-aqua multi-camera/PiP** — single supported camera; must be tested with a drone-feed PiP
  before committing c13 (Black Sea) to aqua; bevy_water is the fallback [src:bevy-aqua].
- **bevy_feronia port** — pinned to bevy 0.17; our wind-shader needs may be met by mining its
  WGSL instead [src:bevy-feronia].
- **bevy-clipmap published version** — README documents 1.0.5 for bevy 0.19 while crates.io
  showed 1.0.4 at access; verify on adoption [src:bevy-clipmap].
- **bevy_fast_clouds maturity** — one maintainer, first release Sep 2026; re-evaluate if
  bevy_weather's cloud cost is a problem on target hardware [src:bevy-fast-clouds].
- **milsymbol coverage** — verify the Rust port renders the APP-6 subset research/07 needs
  (including Ukrainian-specific marks) before building the AWACS overlay [src:milsymbol-rust].
- **No thermal/NVG/FPV/radar plugins exist** — confirmed by search; these four post-process
  systems are our own build items [src:bevy-0.19-release].
- **Cloud raymarch cost on min-spec** — bevy_weather's own benchmark mode (`--bench`) should run
  on our minimum GPU before locking quality tiers [src:bevy-weather].
- **Horizon-map sizes** — bevy-clipmap's self-shadowing horizon maps are disk-hungry; decide
  whether dioramas need them or shadow maps suffice [src:bevy-clipmap].

## 13. Sources

See `sources/09-bevy-rendering.yaml`. Grouped by id prefix: `bevy-*` (engine and
plugin repositories, all fetched 2026-10-02), `hoppe-2004-gpugcm` / `guerrilla-hzd-cloudscapes` /
`frostbite-2016-sky-clouds` (rendering literature), `digital-extinction` / `iron-curtain` /
`rusty-rotor` / `evroon-bevy-open-world` (whole-game prior art), `utm-crate` / `milsymbol-rust`
(non-engine utilities).
