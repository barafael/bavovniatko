# Avdiivka: map data

The roads, railway and sidings, tree rows, land cover (the spoil heap among it) and building footprints in
`osm.ron` are made by `tools/osm/osm2ron.py` from an Overpass extract (`tools/osm/data/avdiivka.json`,
2026-10-21) of OpenStreetMap.

Map data © OpenStreetMap contributors, available under the Open Database License (ODbL):
https://www.openstreetmap.org/copyright

OpenStreetMap shows the place as it is now: the plant in ruins, the railway disused. The script keeps the
disused lines, which were there on the day, and the rest is checked by eye against what the sources describe
(design/avdiivka.md).
