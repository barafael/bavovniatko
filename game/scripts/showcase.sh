#!/bin/sh
# Film the showcase: a directed tour of the day at 1920x1080 with its sound, rebuilt offline.
#   scripts/showcase.sh <out.mp4>                            Vuhledar
#   VUHLEDAR_DIORAMA=robotyne scripts/showcase.sh <out.mp4>  Mala Tokmachka
#   scripts/showcase.sh --film [out.mp4]                     the film of both dioramas (design/showcase.md)
#   scripts/showcase.sh --scout [out.mp4]                    the film at 640x360 with contact sheets, for retiming
# Headless and silent while it runs. Needs ffmpeg, ImageMagick and python3 with numpy.
set -e
cd "$(dirname "$0")/.."
mode=
case "${1:-}" in
  --film|--scout) mode=${1#--}; shift ;;
esac
out=$(realpath -m "${1:-devlog/videos/showcase.mp4}")
# The film's work dir holds ~4000 1080p frames (tens of GB): on the disk, never on a /tmp tmpfs.
work=$(mktemp -d "${TMPDIR:-$PWD/target}/filmwork.XXXXXX")
cargo build -q --release
if [ -z "$mode" ]; then
VUHLEDAR_HEADLESS=1 VUHLEDAR_SKIP_BRIEFING=1 VUHLEDAR_NO_BATTERY=1 VUHLEDAR_RES=1920x1080 VUHLEDAR_SHOWCASE=$work \
  timeout 3000 ./target/release/vuhledar 2>&1 | grep -E 'panicked|ERROR' -A3 | grep -v rodio | head -5 || true
lead=3
place='VUHLEDAR'; date='6 FEBRUARY 2023'
if [ "${VUHLEDAR_DIORAMA:-}" = robotyne ]; then place='SOUTH OF MALA TOKMACHKA'; date='8 JUNE 2023'; fi
magick -size 1920x1080 xc:black -font DejaVu-Sans -fill '#e8e8ea' -gravity center \
  -pointsize 64 -annotate +0-60 "$place" -pointsize 34 -annotate +0+20 "$date" \
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
exit 0
fi

# The film (design/showcase.md): both dioramas with VUHLEDAR_TOUR=film, cards over black between the shots,
# each part's sound rebuilt offline, then title, part one, part two and end card concatenated.
if [ "$mode" = scout ]; then
  out=$(realpath -m "${1:-target/film/$(date +%Y%m%d-%H%M)-scout.mp4}")
  res=640x360; crf=30; preset=veryfast
else
  out=$(realpath -m "${1:-devlog/videos/$(date +%Y%m%d-%H%M)-film.mp4}")
  res=1920x1080; crf=27; preset=slow
fi
mkdir -p "$(dirname "$out")" target/film
w=${res%x*}; h=${res#*x}
font="$PWD/src/fonts/LiberationMono-Regular.ttf"
s=$(python3 -c "print($h/1080)")
t1=$(python3 -c "print(round(54*$s))"); t2=$(python3 -c "print(round(30*$s))"); t3=$(python3 -c "print(round(22*$s))")
y1=$(python3 -c "print(round(70*$s))"); y2=$(python3 -c "print(round(10*$s))"); y3=$(python3 -c "print(round(80*$s))")

# 1. The two parts, one after the other: they share the GPU. About 8–13 min per part at 1080p, measured on the
#    first full run (2026-10-12); the scout's 640x360 parts film in 3–4 min.
for part in vuhledar robotyne; do
  mkdir -p "$work/$part"
  start=$(date +%s)
  VUHLEDAR_DIORAMA=$part VUHLEDAR_TOUR=film VUHLEDAR_HEADLESS=1 VUHLEDAR_SKIP_BRIEFING=1 VUHLEDAR_NO_BATTERY=1 \
    VUHLEDAR_RES=$res VUHLEDAR_SHOWCASE="$work/$part" \
    timeout 4000 ./target/release/vuhledar 2>&1 | grep -E 'panicked|ERROR' -A3 | grep -v rodio | head -5 || true
  echo "film: $part filmed in $(( ($(date +%s) - start) / 60 )) min at $res"
done

# 2. The cards: black frames with a title, a smaller line and a dim note, linked into the frame ranges the tour
#    reserved for them (before each shot's first frame).
for part in vuhledar robotyne; do
  [ -f "$work/$part/cards.txt" ] || continue
  while IFS= read -r line; do
    [ -n "$line" ] || continue
    head=${line%%$'\t'*}
    vt=${head%% *}; secs=${head#* }
    title=$(printf '%s' "$line" | cut -f2)
    sline=$(printf '%s' "$line" | cut -f3)
    note=$(printf '%s' "$line" | cut -f4)
    first=$(python3 -c "print(round($vt*30))"); last=$(python3 -c "print(round(($vt+$secs)*30))")
    if [ -n "$note" ]; then
      magick -size $res xc:black -font "$font" -fill '#e8e8ea' -gravity center \
        -pointsize $t1 -annotate +0-$y1 "$title" -pointsize $t2 -annotate +0+$y2 "$sline" \
        -fill '#9a9ca0' -pointsize $t3 -annotate +0+$y3 "$note" "$work/$part/card-$first.png"
    else
      magick -size $res xc:black -font "$font" -fill '#e8e8ea' -gravity center \
        -pointsize $t1 -annotate +0-$y1 "$title" -pointsize $t2 -annotate +0+$y2 "$sline" "$work/$part/card-$first.png"
    fi
    i=$first
    while [ $i -lt $last ]; do
      ln -f "$work/$part/card-$first.png" "$(printf '%s/%s/f%05d.png' "$work" "$part" "$i")"
      i=$((i+1))
    done
  done < "$work/$part/cards.txt"
done

# 3. Each part's sound (level-matched over the filmed segments, the film's parts within a loudness point), then
#    identical encodes so the concat needs no re-encode.
for part in vuhledar robotyne; do
  case $part in
    # The ebur128 check on the first scout put Robotyne 3 LU over part one: the close bursts.
    robotyne) gain=--gain-db\ -3 ;;
    *) gain= ;;
  esac
  python3 scripts/showcase_mix.py $gain "$work/$part" "$work/$part.wav" 0
  ffmpeg -loglevel error -y -framerate 30 -i "$work/$part/f%05d.png" -i "$work/$part.wav" \
    -c:v libx264 -crf $crf -preset $preset -pix_fmt yuv420p -c:a aac -b:a 160k -shortest "$work/$part.mp4"
done
for part in vuhledar robotyne; do
  ffmpeg -i "$work/$part.mp4" -map 0:a -af ebur128 -f null - 2>&1 | grep -E '^ *I:' | tail -1 | sed "s/^/loudness $part /" || true
done

# 4. Title and end card over a wind bed; fade in on the title, fade out on the end. The end card reads both
#    parts' ledgers: four rows each, then the closing lines verbatim.
python3 scripts/showcase_mix.py --bed "$work/vuhledar" 5 "$work/bed-title.wav"
python3 scripts/showcase_mix.py --bed "$work/robotyne" 10 "$work/bed-end.wav"
magick -size $res xc:black -font "$font" -fill '#e8e8ea' -gravity center \
  -pointsize $t1 -annotate +0-$y1 'BAVOVNIATKO' \
  -pointsize $t2 -annotate +0+$y2 'Two dioramas of the Russo-Ukrainian war' \
  -fill '#9a9ca0' -pointsize $t3 -annotate +0+$y3 'Everything here is simulated, and seen through a Ukrainian drone.' \
  "$work/title.png"
python3 - "$work" <<'EOF'
import sys
work = sys.argv[1]
def rows(part):
    out = []
    for line in open(f"{work}/{part}/ledger.txt", encoding="utf-8"):
        f = line.rstrip("\n").split("\t")
        if f[0] != "closing" and len(out) < 4:
            out.append(f"{f[0].strip()[:46]:<46} {f[1]:>8}" if len(f) > 1 else f[0])
    return out
def closings(part):
    return [line.rstrip("\n").split("\t", 1)[1] for line in open(f"{work}/{part}/ledger.txt", encoding="utf-8") if line.startswith("closing\t")]
lines = [f"{'ВУГЛЕДАР (VUHLEDAR)':<57} МАЛА ТОКМАЧКА (MALA TOKMACHKA)", ""]
for a, b in zip(rows("vuhledar"), rows("robotyne")):
    lines.append(f"{a}   {b}")
lines.append("")
lines += closings("vuhledar")
lines.append("")
lines += closings("robotyne")
open(f"{work}/end.txt", "w", encoding="utf-8").write("\n".join(lines))
EOF
magick -size $res xc:black -font "$font" -pointsize $t3 -fill '#e8e8ea' -gravity center \
  -annotate +0+0 "@$work/end.txt" "$work/end.png"

enc="-c:v libx264 -crf $crf -preset $preset -pix_fmt yuv420p -c:a aac -b:a 160k"
ffmpeg -loglevel error -y -loop 1 -t 5 -framerate 30 -i "$work/title.png" -i "$work/bed-title.wav" \
  -vf "fade=t=in:st=0:d=0.8,fade=t=out:st=4.4:d=0.6" $enc -shortest "$work/title.mp4"
ffmpeg -loglevel error -y -loop 1 -t 10 -framerate 30 -i "$work/end.png" -i "$work/bed-end.wav" \
  -vf "fade=t=in:st=0:d=0.5,fade=t=out:st=9:d=1" $enc -shortest "$work/end.mp4"
printf "file '%s'\nfile '%s'\nfile '%s'\nfile '%s'\n" "$work/title.mp4" "$work/vuhledar.mp4" "$work/robotyne.mp4" "$work/end.mp4" > "$work/list.txt"
ffmpeg -loglevel error -y -f concat -safe 0 -i "$work/list.txt" -c copy "$out"

# 5. The scout's contact sheets, one tile per second, for retiming the shots.
if [ "$mode" = scout ]; then
  for part in vuhledar robotyne; do
    ffmpeg -loglevel error -y -i "$work/$part.mp4" \
      -vf "fps=1,scale=240:-1,tile=8x15" \
      "target/film/$(basename "$out" .mp4)-$part-sheet-%02d.png"
  done
fi

if [ "$mode" = film ]; then
  echo "| $(date '+%Y-%m-%d %H:%M') | [$(basename "$out")](videos/$(basename "$out")) | The showcase film (design/showcase.md): title, Vuhledar with its cards, the intertitle, Mala Tokmachka, the end card from both ledgers. |" >> devlog/videos.md
fi
rm -rf "$work"
echo "film: $out ($(du -h "$out" | cut -f1))"
