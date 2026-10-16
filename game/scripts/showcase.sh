#!/bin/sh
# Film the showcase: a directed tour of the day at 1920x1080 with its sound, rebuilt offline.
#   scripts/showcase.sh <out.mp4>                            Vuhledar
#   BAV_DIORAMA=robotyne scripts/showcase.sh <out.mp4>  Mala Tokmachka
#   scripts/showcase.sh --film [out.mp4]                     the film of both dioramas (design/showcase.md)
#   scripts/showcase.sh --scout [out.mp4]                    the film at 640x360 with contact sheets, for retiming
#   scripts/showcase.sh --trailer [out.mp4]                  the five-minute trailer of both (design/trailer.md)
#   scripts/showcase.sh --trailer-scout [out.mp4]            the trailer at 640x360 with contact sheets
# Headless and silent while it runs. Needs ffmpeg, ImageMagick and python3 with numpy.
set -e
cd "$(dirname "$0")/.."
mode=
case "${1:-}" in
  --film|--scout) mode=${1#--}; tour=film; shift ;;
  --trailer) mode=film; tour=trailer; shift ;;
  --trailer-scout) mode=scout; tour=trailer; shift ;;
esac
out=$(realpath -m "${1:-devlog/videos/showcase.mp4}")
# The film's work dir holds ~4000 1080p frames (tens of GB): on the disk, never on a /tmp tmpfs, and never in
# target/, which a cargo clean empties.
mkdir -p devlog
work=$(mktemp -d "$PWD/devlog/filmwork.XXXXXX")
cargo build -q --release
# The mixer needs numpy: the venv in .venv if there is one (python3 -m venv .venv; .venv/bin/pip install numpy).
py=python3; [ -x .venv/bin/python ] && py=$PWD/.venv/bin/python
if [ -z "$mode" ]; then
BAV_HEADLESS=1 BAV_SKIP_BRIEFING=1 BAV_NO_BATTERY=1 BAV_OPERATOR=off BAV_RES=1920x1080 BAV_SHOWCASE=$work \
  timeout 3000 ./target/release/bavovniatko 2>&1 | grep -E 'panicked|ERROR' -A3 | grep -v rodio | head -5 || true
lead=3
place='VUHLEDAR'; date='6 FEBRUARY 2023'
if [ "${BAV_DIORAMA:-}" = robotyne ]; then place='SOUTH OF MALA TOKMACHKA'; date='8 JUNE 2023'; fi
magick -size 1920x1080 xc:black -font DejaVu-Sans -fill '#e8e8ea' -gravity center \
  -pointsize 64 -annotate +0-60 "$place" -pointsize 34 -annotate +0+20 "$date" \
  -fill '#9a9ca0' -pointsize 22 -annotate +0+90 'bavovniatko: a diorama of the Russo-Ukrainian war' "$work/title.png"
$py scripts/showcase_mix.py "$work" "$work/sound.wav" $lead
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

# The film (design/showcase.md): both dioramas with BAV_TOUR=film, cards over black between the shots,
# each part's sound rebuilt offline, then title, part one, part two and end card concatenated.
name=film; [ "$tour" = trailer ] && name=trailer
if [ "$mode" = scout ]; then
  out=$(realpath -m "${1:-devlog/film/$(date +%Y%m%d-%H%M)-$name-scout.mp4}")
  res=640x360; crf=30; preset=veryfast
else
  out=$(realpath -m "${1:-devlog/videos/$(date +%Y%m%d-%H%M)-$name.mp4}")
  res=1920x1080; crf=27; preset=slow
  # The trailer is for showing on a big screen: less grain lost to the encoder (crf 20 made 1.6 GB of it).
  [ "$tour" = trailer ] && crf=24
fi
# The trailer's Mala Tokmachka answers every call for evacuation half a minute later, as a player would: nothing
# goes in for the wounded unless someone sends it.
extra=; [ "$tour" = trailer ] && extra=BAV_EVAC=30
mkdir -p "$(dirname "$out")" devlog/film
w=${res%x*}; h=${res#*x}
font="$PWD/crates/bavovniatko/src/fonts/LiberationMono-Regular.ttf"
s=$(python3 -c "print($h/1080)")
t1=$(python3 -c "print(round(54*$s))"); t2=$(python3 -c "print(round(30*$s))"); t3=$(python3 -c "print(round(22*$s))")
y1=$(python3 -c "print(round(70*$s))"); y2=$(python3 -c "print(round(10*$s))"); y3=$(python3 -c "print(round(80*$s))")

# 1. The two parts, one after the other: they share the GPU. About 8–13 min per part at 1080p, measured on the
#    first full run (2026-10-12); the scout's 640x360 parts film in 3–4 min.
for part in vuhledar robotyne; do
  mkdir -p "$work/$part"
  start=$(date +%s)
  [ $part = robotyne ] && part_env=$extra || part_env=
  env $part_env BAV_DIORAMA=$part BAV_TOUR=$tour BAV_HEADLESS=1 BAV_SKIP_BRIEFING=1 BAV_NO_BATTERY=1 BAV_OPERATOR=off \
    BAV_RES=$res BAV_SHOWCASE="$work/$part" \
    timeout 4000 ./target/release/bavovniatko 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | grep -v 'Screenshot saved' > "devlog/film/$name-$part.log" || true
  grep -E 'panicked|ERROR' -A3 "devlog/film/$name-$part.log" | grep -v rodio | head -5 || true
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

# 3. Each part's sound, level-matched: mixed once and measured (EBU R128, integrated), then mixed again with the
#    gain that brings it to -18 LUFS, so the parts sit within a loudness point whatever the day's bursts did. Then
#    identical encodes so the concat needs no re-encode.
for part in vuhledar robotyne; do
  $py scripts/showcase_mix.py "$work/$part" "$work/$part.wav" 0
  i=$(ffmpeg -i "$work/$part.wav" -af ebur128 -f null - 2>&1 | sed -n 's/^ *I: *\(-*[0-9.]*\) LUFS/\1/p' | tail -1)
  gain=$(python3 -c "print(round(-18.0 - ($i), 1))")
  echo "level $part: $i LUFS, gain $gain dB"
  $py scripts/showcase_mix.py --gain-db "$gain" "$work/$part" "$work/$part.wav" 0
  ffmpeg -loglevel error -y -framerate 30 -i "$work/$part/f%05d.png" -i "$work/$part.wav" \
    -c:v libx264 -crf $crf -preset $preset -pix_fmt yuv420p -c:a aac -b:a 160k -shortest "$work/$part.mp4"
done
for part in vuhledar robotyne; do
  ffmpeg -i "$work/$part.mp4" -map 0:a -af ebur128 -f null - 2>&1 | grep -E '^ *I:' | tail -1 | sed "s/^/loudness $part /" || true
done

# 4. Title and end card over a wind bed; fade in on the title, fade out on the end. The end card reads both
#    parts' ledgers: four rows each, then the closing lines verbatim.
$py scripts/showcase_mix.py --bed "$work/vuhledar" 5 "$work/bed-title.wav"
$py scripts/showcase_mix.py --bed "$work/robotyne" 10 "$work/bed-end.wav"
if [ "$tour" = trailer ]; then
  title2='One minefield, seen from both sides'
  title3='Two days of the Russo-Ukrainian war, simulated, and seen through a Ukrainian drone.'
else
  title2='Two dioramas of the Russo-Ukrainian war'
  title3='Everything here is simulated, and seen through a Ukrainian drone.'
fi
magick -size $res xc:black -font "$font" -fill '#e8e8ea' -gravity center \
  -pointsize $t1 -annotate +0-$y1 'BAVOVNIATKO' \
  -pointsize $t2 -annotate +0+$y2 "$title2" \
  -fill '#9a9ca0' -pointsize $t3 -annotate +0+$y3 "$title3" \
  "$work/title.png"
python3 - "$work" <<'EOF'
import sys
work = sys.argv[1]
def rows(part):
    out = []
    for line in open(f"{work}/{part}/ledger.txt", encoding="utf-8"):
        f = line.rstrip("\n").split("\t")
        if f[0] != "closing" and len(out) < 4:
            # A row's note in parentheses would not fit the card's column: the label only.
            out.append(f"{f[0].split(' (')[0].strip()[:46]:<46} {f[1]:>8}" if len(f) > 1 else f[0])
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
# The end card's rows at 24 pt at 1080p: 22 read small on the first trailer scout, and at 26 the longest closing
# line runs off the card.
t4=$(python3 -c "print(round(24*$s))")
magick -size $res xc:black -font "$font" -pointsize $t4 -fill '#e8e8ea' -gravity center \
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
      "devlog/film/$(basename "$out" .mp4)-$part-sheet-%02d.png"
  done
fi

if [ "$mode" = film ] && [ "$tour" = trailer ]; then
  mkdir -p devlog
  echo "| $(date '+%Y-%m-%d %H:%M') | [$(basename "$out")](videos/$(basename "$out")) | The trailer (design/trailer.md): title, Vuhledar, the intertitle, Mala Tokmachka, the end card from both ledgers. |" >> devlog/videos.md
elif [ "$mode" = film ]; then
  mkdir -p devlog
  echo "| $(date '+%Y-%m-%d %H:%M') | [$(basename "$out")](videos/$(basename "$out")) | The showcase film (design/showcase.md): title, Vuhledar with its cards, the intertitle, Mala Tokmachka, the end card from both ledgers. |" >> devlog/videos.md
fi
rm -rf "$work"
echo "film: $out ($(du -h "$out" | cut -f1))"
