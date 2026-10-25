# Myrnohrad: map data

The roads, the disused mine railway, tree rows, land cover (the two mines' spoil heaps among it) and building
footprints in `osm.ron` are made by `tools/osm/osm2ron.py` from an Overpass extract (`tools/osm/data/myrnohrad.json`,
2026-10-25) of OpenStreetMap.

Map data © OpenStreetMap contributors, available under the Open Database License (ODbL):
https://www.openstreetmap.org/copyright

OpenStreetMap shows the place as it is now: every railway in the box tagged disused, the villages' houses not
mapped. The script keeps the disused lines, which were there on the day, and the rest is checked by eye against
what the sources describe (design/myrnohrad.md).
