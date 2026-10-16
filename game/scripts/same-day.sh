#!/bin/sh
# Check that a change leaves the simulation alone. The day is deterministic (main.rs, `Step`), so filming it gives the
# same sound log, film segments and minute-by-minute tally every time the code is the same.
#   scripts/same-day.sh <name>            film the day into devlog/same-day/<name> (headless, 320x180, frames dropped)
#   scripts/same-day.sh <name> <earlier>  ...and compare it with an earlier run
# Takes about five minutes.
set -e
cd "$(dirname "$0")/.."
cargo build -q --release
out=devlog/same-day/$1
rm -rf "$out" && mkdir -p "$out"
BAV_HEADLESS=1 BAV_SKIP_BRIEFING=1 BAV_NO_BATTERY=1 BAV_RES=320x180 BAV_LOG_STATS=1 \
  BAV_SHOWCASE="$(realpath "$out")" timeout 3000 ./target/release/bavovniatko 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | grep -o 't=.*' > "$out/tally.txt" || true
rm -f "$out"/f*.png
echo "$1: $(wc -l < "$out/audio.log") sounds, $(tail -1 "$out/tally.txt")"
[ -n "$2" ] || exit 0
same=0
for f in audio.log segments.txt tally.txt; do
  if cmp -s "$out/$f" "devlog/same-day/$2/$f"; then echo "  $f: same"; else echo "  $f: DIFFERENT"; same=1; fi
done
exit $same
