#!/bin/sh
# Record the trailer (design/trailer.md): about five minutes of both dioramas, 1920x1080 with sound.
#   scripts/trailer.sh [out.mp4]            the trailer, into devlog/videos/ (about 8 min on the 8060S)
#   scripts/trailer.sh --scout [out.mp4]    the same at 640x360 with contact sheets, into devlog/film/ (about 4 min)
# The filming is scripts/showcase.sh --trailer; this sets up what it needs, and afterwards checks the game's logs
# for the moments the shots are timed on. The day is deterministic, but a change to the battle moves them: a
# warning here means a shot in crates/bavovniatko/src/showcase/trailer.rs needs retiming (scout, then look).
set -e
cd "$(dirname "$0")/.."
mode=--trailer
if [ "${1:-}" = --scout ]; then
  mode=--trailer-scout
  shift
fi
for tool in ffmpeg ffprobe magick; do
  command -v $tool >/dev/null || { echo "trailer: needs $tool" >&2; exit 1; }
done
# The sound mixer needs numpy, from a venv of the game's own (gitignored).
if ! [ -x .venv/bin/python ] || ! .venv/bin/python -c 'import numpy' 2>/dev/null; then
  echo "trailer: setting up .venv with numpy"
  python3 -m venv .venv
  .venv/bin/pip install -q numpy
fi

scripts/showcase.sh $mode "$@" | tee devlog/film/trailer-run.txt
out=$(sed -n 's/^film: \(.*\.mp4\) (.*$/\1/p' devlog/film/trailer-run.txt | tail -1)

# The moments the shots wait for: (log line, from, to) in game seconds, as the shot list has them.
check() {
  log=devlog/film/trailer-$1.log
  t=$(grep -m1 "$2" "$log" | sed -n 's/.*at \([0-9]*\) s$/\1/p')
  if [ -z "$t" ]; then
    echo "  MISSING  $1: $4"
  elif [ "$t" -lt "$3" ] || [ "$t" -gt "${5:-999999}" ]; then
    echo "  MOVED    $1: $4 at $t s, the shot expects $3-${5:-} s"
  else
    echo "  ok       $1: $4 at $t s"
  fi
}
echo "trailer: the shots' moments"
check robotyne 'jammer at -350,950 knocked out' 359 'the jammer shelled out, in its shot' 372
check robotyne 'fpv: in on' 642 'the FPV hit, in the dive shot' 654
check robotyne 'rescue: Bradley 3/0 stopped' 507 'the first Bradley stopping for the wounded' 521
grep -q 'fpv: into the ground' devlog/film/trailer-robotyne.log && echo "  MISSED   robotyne: the FPV went into the ground"
grep -q 'panicked' devlog/film/trailer-*.log && echo "  PANIC    see devlog/film/trailer-*.log"
echo "trailer: $out, $(ffprobe -v error -show_entries format=duration -of csv=p=0 "$out" | cut -d. -f1) s"
