# Licence for the content and data

The **code** in this repository is dual-licensed under the MIT licence ([LICENSE-MIT](LICENSE-MIT)) or the Apache
License 2.0 ([LICENSE-APACHE](LICENSE-APACHE)), at your option. That covers `db/tools/`, `db/schema/`, `db/checks/`,
`db/fixes/`, `db/dbctl.sh`, `web/` and the other scripts.

The **content and data** are licensed under the
[Creative Commons Attribution 4.0 International licence (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/legalcode),
**except for the parts listed below**. Content and data means:

- the research texts (`research/`), the design documents (`design/`) and the briefs in `db/staging/`;
- the knowledge base: the claims, their relations, observations and entities in `db/dump/`, `db/staging/` and
  `db/vocab/`;
- the game feed (`game-data/`) and the explorer's generated dataset.

Copyright © 2026 Rafael Bachmann. To attribute, write: *"bavovniatko knowledge base, CC BY 4.0,
https://github.com/barafael/bavovniatko"*. If you used OpenStreetMap-derived data, add the attribution in section 1.

## What this licence does not cover

### 1. OpenStreetMap-derived geometry: ODbL 1.0

These came from OpenStreetMap (via Nominatim, Photon and Overpass):

- the place geometries and centroids (the `geometry` and `centroid` fields in `db/dump/place.jsonl` and the
  explorer's dataset);
- the geocoding cache in `db/staging/entities/geocode.jsonl`.

They are **© OpenStreetMap contributors** and available under the
[Open Database License 1.0 (ODbL)](https://opendatacommons.org/licenses/odbl/1-0/), not CC BY. If you publicly use
an adapted version of this geometry, you must offer it under the ODbL as well. See
https://www.openstreetmap.org/copyright.

Figures *computed* from OpenStreetMap data, such as distances, field sizes and shelterbelt spacing, are produced
works under the ODbL. They are covered by this CC BY licence, and they also require the attribution
"© OpenStreetMap contributors".

### 2. Quotations and third-party works

- **Quotations** from sources (the `quote` field of citations, and quoted passages in the research texts) remain the
  work of their authors and publishers. They are reproduced as short quotations for the purpose of citation, and
  this licence grants no rights in them.
- **Linked works** are not part of this repository and not covered by its licence. That includes cited articles,
  reports, maps, images, videos and satellite imagery, which are linked by URL only (the `media` and `source`
  records). The private offline archive of those pages (`db/tools/archive.py`) is not distributed.
- **Names, logos and trademarks** of publishers and organisations appear for identification only.

### 3. Computed data and attributions

Some claims were computed from open datasets. Their attribution notices apply in addition to CC BY:

- **Weather** (temperature, wind, precipitation, cloud, humidity): Open-Meteo historical weather API
  ([open-meteo.com](https://open-meteo.com/), CC BY 4.0), based on ERA5 reanalysis. *Contains modified Copernicus
  Climate Change Service information.* Neither the European Commission nor ECMWF is responsible for any use made of
  it.
- **Elevation**: SRTM 30 m, via Open Topo Data. SRTM is a NASA/USGS product in the public domain.
- **Frontline areas and their changes**: computed by this project from DeepStateMap's published map history
  ([deepstatemap.live](https://deepstatemap.live/)). Only the derived figures are included, not DeepState's map
  geometry. Credit DeepState as the source of the underlying maps.
- **Sun and moon times**: computed with PyEphem (no data licence).

## No warranty, and how to read the data

The content is provided "as is", without warranty of any kind. It concerns an ongoing war, and many figures are
contested. Claims record **what sources say**, with their claimants and their disagreements; they are not
established facts. Claims were extracted from the research texts by AI agents and checked by tools and spot
reviews. Always check the cited sources before relying on a claim.

## Third-party software

The explorer website bundles third-party software under its own licences, listed on its *About* page and in
`licenses/` on the site (generated at build time by `web/notices.py`). This includes SurrealDB, which is licensed
under the **Business Source License 1.1**, not an open-source licence.
