#!/bin/sh
# Film the showcase: a directed tour of the day at 1920x1080 with its sound, rebuilt offline.
#   scripts/showcase.sh <out.mp4>
# Headless and silent while it runs. Needs ffmpeg, ImageMagick and python3 with numpy.
set -e
cd "$(dirname "$0")/.."
out=$(realpath -m "${1:-devlog/videos/showcase.mp4}")
work=$(mktemp -d)
cargo build -q --release
VUHLEDAR_HEADLESS=1 VUHLEDAR_SKIP_BRIEFING=1 VUHLEDAR_NO_BATTERY=1 VUHLEDAR_RES=1920x1080 VUHLEDAR_SHOWCASE=$work \
  timeout 3000 ./target/release/vuhledar 2>&1 | grep -E 'panicked|ERROR' -A3 | grep -v rodio | head -5 || true
lead=3
magick -size 1920x1080 xc:black -font DejaVu-Sans -fill '#e8e8ea' -gravity center \
  -pointsize 64 -annotate +0-60 'VUHLEDAR' -pointsize 34 -annotate +0+20 '6 FEBRUARY 2023' \
  -fill '#9a9ca0' -pointsize 22 -annotate +0+90 'bavovniatko: a diorama of the Russo-Ukrainian war' "$work/title.png"
python3 scripts/showcase_mix.py "$work" "$work/sound.wav" $lead
ffmpeg -loglevel error -y \
  -loop 1 -t $lead -framerate 30 -i "$work/title.png" \
  -framerate 30 -i "$work/f%05d.png" \
  -i "$work/sound.wav" \
  -filter_complex "[0:v]fade=t=in:st=0:d=0.8,fade=t=out:st=2.4:d=0.6,format=yuv420p[t];[1:v]format=yuv420p[g];[t][g]concat=n=2:v=1:a=0[v]" \
  -map "[v]" -map 2:a -c:v libx264 -crf 27 -preset slow -tune grain -c:a aac -b:a 160k -shortest "$out"
rm -rf "$work"
echo "showcase: $out ($(du -h "$out" | cut -f1))"
