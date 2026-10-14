#!/bin/sh
# How far the day's outcome swings with chance alone: the day run on several random streams (VUHLEDAR_SEED),
# each to its end, and the final tallies side by side. One run says little; the band of a few is the test a
# balance change has to pass (the target bands are in design/next-items.md §3 A).
#   scripts/band.sh [diorama] [seeds...]     default: vuhledar, seeds 0 1 2 3 (0 is the day itself)
# Two days run at a time; each takes about as long as scripts/same-day.sh.
set -e
cd "$(dirname "$0")/.."
dio=${1:-vuhledar}
[ $# -gt 0 ] && shift
seeds=${*:-0 1 2 3}
cargo build -q --release
out=target/band/$dio
mkdir -p "$out"
run() {
  dir=$out/seed-$1
  rm -rf "$dir" && mkdir -p "$dir"
  VUHLEDAR_DIORAMA=$dio VUHLEDAR_SEED=$1 VUHLEDAR_HEADLESS=1 VUHLEDAR_SKIP_BRIEFING=1 VUHLEDAR_NO_BATTERY=1 VUHLEDAR_RES=320x180 \
    VUHLEDAR_LOG_STATS=1 VUHLEDAR_SHOWCASE="$(realpath "$dir")" timeout 3000 ./target/release/vuhledar 2>&1 \
    | sed 's/\x1b\[[0-9;]*m//g' | grep -o 't=.*' > "$dir/tally.txt" || true
  rm -f "$dir"/f*.png
}
n=0
for s in $seeds; do
  run "$s" &
  n=$((n + 1))
  [ $((n % 2)) -eq 0 ] && wait
done
wait
for s in $seeds; do
  echo "$dio seed $s: $(grep committed= "$out/seed-$s/tally.txt" | tail -1 | sed 's/ overlap_frames.*//; s/ hidden=[0-9]*//')"
done
