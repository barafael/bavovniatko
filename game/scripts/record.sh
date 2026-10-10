#!/bin/sh
# Record a short headless clip into devlog/videos/.
#   scripts/record.sh <name> "<cam x,z,agl,yaw,pitch,fov>" <from-game-s> <seconds> ["note"] [look: idx|men] [view: op]
# Runs offscreen on the GPU, silent, no window. Appends a line to devlog/videos.md.
set -e
cd "$(dirname "$0")/.."
mkdir -p devlog/videos
name=$1; cam=$2; from=${3:-5}; secs=${4:-10}; note=${5:-}; look=${6:-}; view=${7:-}
tmp=$(mktemp -d)
VUHLEDAR_HEADLESS=1 VUHLEDAR_SKIP_BRIEFING=1 VUHLEDAR_CAM="$cam" VUHLEDAR_LOOK="$look" VUHLEDAR_VIEW="$view" VUHLEDAR_WARP_TO=$from VUHLEDAR_NO_BATTERY=1 VUHLEDAR_FIRE="${FIRE:-}" \
  VUHLEDAR_REC=$tmp VUHLEDAR_REC_FROM=$from VUHLEDAR_REC_SECS=$secs timeout 900 cargo run -q --release 2>&1 | grep -E 'panicked|ERROR' -A3 | head -5 || true
f=$(date +%Y%m%d-%H%M)-$name.mp4
ffmpeg -loglevel error -y -framerate 15 -i $tmp/f%05d.png -vf scale=1280:-2 -c:v libx264 -crf 30 -preset slow -pix_fmt yuv420p devlog/videos/$f
rm -rf $tmp
[ -f devlog/videos.md ] || printf '# Video history\n\nShort headless clips from development, oldest first. Keep them all.\n\n| when | file | what |\n|---|---|---|\n' > devlog/videos.md
echo "| $(date '+%Y-%m-%d %H:%M') | [$f](videos/$f) | $note |" >> devlog/videos.md
echo "recorded devlog/videos/$f ($(du -h devlog/videos/$f | cut -f1))"
