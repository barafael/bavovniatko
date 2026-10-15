#!/bin/sh
# How far the day's outcome swings with chance alone: the day run on several random streams (BAV_SEED),
# each to its end, and the final tallies side by side. One run says little; the band of a few is the test a
# balance change has to pass (the target bands are in design/next-items.md §3 A).
#   scripts/band.sh [diorama] [seeds...]     default: vuhledar, seeds 0 1 2 3 (0 is the day itself)
# Two days run at a time, built with the fast profile; each takes about as long as scripts/same-day.sh.
set -e
cd "$(dirname "$0")/.."
dio=${1:-vuhledar}
[ $# -gt 0 ] && shift
seeds=${*:-0 1 2 3}
cargo build -q --profile fast
# BAND_TAG=<name> keeps a variant apart (target/band/<diorama>-<name>); the switches it varies go in the environment.
out=target/band/$dio${BAND_TAG:+-$BAND_TAG}
mkdir -p "$out"
run() {
  dir=$out/seed-$1
  rm -rf "$dir" && mkdir -p "$dir"
  BAV_DIORAMA=$dio BAV_SEED=$1 BAV_HEADLESS=1 BAV_SKIP_BRIEFING=1 BAV_NO_BATTERY=1 BAV_RES=320x180 \
    BAV_LOG_STATS=1 BAV_SHOWCASE="$(realpath "$dir")" timeout 3000 ./target/fast/bavovniatko 2>&1 \
    | sed 's/\x1b\[[0-9;]*m//g' > "$dir/out.txt" || true
  grep -o 't=.*' "$dir/out.txt" > "$dir/tally.txt" || true
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
  echo "$dio seed $s: $(grep committed= "$out/seed-$s/tally.txt" | tail -1 | sed 's/ overlap_frames.*//; s/ hidden=[0-9]*//') $(grep -o 'wounded=[0-9/]*' "$out/seed-$s/tally.txt" | tail -1)"
done
