#!/bin/sh
# Take a headless screenshot into devlog/shots/ and log it in devlog/shots.md.
#   scripts/shot.sh <name> "<cam x,z,agl,yaw,pitch,fov>" <game-s> ["note"] [diorama] [look: idx|men] [view: op|ground]
# The game warps to <game-s> (3x), waits two seconds there, and shoots. FIRE="100:he" calls scripted fire missions;
# BIN=target/debug/vuhledar shoots with the dev build.
set -e
cd "$(dirname "$0")/.."
mkdir -p devlog/shots
name=$1; cam=$2; at=${3:-5}; note=${4:-}; dio=${5:-vuhledar}; look=${6:-}; view=${7:-}
f=$(date +%Y%m%d-%H%M)-$name
tmp=$(mktemp -d)
VUHLEDAR_DIORAMA=$dio VUHLEDAR_HEADLESS=1 VUHLEDAR_SKIP_BRIEFING=1 VUHLEDAR_CAM="$cam" VUHLEDAR_LOOK="$look" VUHLEDAR_VIEW="$view" \
  VUHLEDAR_WARP_TO=$at VUHLEDAR_SHOT_AT=${WAIT:-2} VUHLEDAR_NO_BATTERY=1 VUHLEDAR_FIRE="${FIRE:-}" VUHLEDAR_LOG_STATS=1 \
  VUHLEDAR_SHOT=$tmp/shot.png timeout 900 ${BIN:-./target/release/vuhledar} 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | grep -E 'panicked|ERROR|t=|picked_up' -A2 | tail -4 || true
ffmpeg -loglevel error -y -i $tmp/shot.png -q:v 3 devlog/shots/$f.jpg
rm -rf $tmp
echo "| $(date '+%Y-%m-%d %H:%M') | [$f.jpg](shots/$f.jpg) | $note |" >> devlog/shots.md
echo "devlog/shots/$f.jpg"
