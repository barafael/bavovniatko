#!/bin/sh
# Check that a change leaves the simulation alone. The day is deterministic (main.rs, `Step`), so filming it gives the
# same sound log, film segments and minute-by-minute tally every time the code is the same.
#   scripts/same-day.sh <name>            film the day into target/same-day/<name> (headless, 320x180, frames dropped)
#   scripts/same-day.sh <name> <earlier>  ...and compare it with an earlier run
# Takes about five minutes.
set -e
cd "$(dirname "$0")/.."
cargo build -q --release
out=target/same-day/$1
rm -rf "$out" && mkdir -p "$out"
VUHLEDAR_HEADLESS=1 VUHLEDAR_SKIP_BRIEFING=1 VUHLEDAR_NO_BATTERY=1 VUHLEDAR_RES=320x180 VUHLEDAR_LOG_STATS=1 \
  VUHLEDAR_SHOWCASE="$(realpath "$out")" timeout 3000 ./target/release/vuhledar 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | grep -o 't=.*' > "$out/tally.txt" || true
rm -f "$out"/f*.png
echo "$1: $(wc -l < "$out/audio.log") sounds, $(tail -1 "$out/tally.txt")"
[ -n "$2" ] || exit 0
same=0
for f in audio.log segments.txt tally.txt; do
  if cmp -s "$out/$f" "target/same-day/$2/$f"; then echo "  $f: same"; else echo "  $f: DIFFERENT"; same=1; fi
done
exit $same
