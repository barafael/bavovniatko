#!/bin/sh
# The Overpass extract a diorama's ground is made from (tools/osm/osm2ron.py): every way of the kinds the game
# draws, in a box a little wider than the diorama's. Usage: tools/osm/fetch.sh hostomel
set -e
cd "$(dirname "$0")"
case "$1" in
  hostomel) bbox="50.575,30.15,50.635,30.235" ;;
  *) echo "no box for $1" >&2; exit 1 ;;
esac
q="[out:json][timeout:120];(way[\"aeroway\"]($bbox);way[\"building\"]($bbox);way[\"landuse\"]($bbox);way[\"natural\"]($bbox);way[\"railway\"]($bbox);way[\"highway\"~\"motorway|trunk|primary|secondary|tertiary|unclassified|track|service|residential\"]($bbox);way[\"waterway\"]($bbox););out geom;"
mkdir -p data
curl -sf -m 180 -A "bavovniatko-osm2ron/0.1" https://overpass-api.de/api/interpreter --data-urlencode "data=$q" -o "data/$1.json"
echo "data/$1.json: $(wc -c < "data/$1.json") bytes"
