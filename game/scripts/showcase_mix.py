#!/usr/bin/env python3
"""Rebuild the showcase soundtrack offline from the game's audio log.

Usage: showcase_mix.py [--bed out.wav secs] [--gain-db db] <dir> <out.wav> <lead-in seconds>

<dir> holds what a VUHLEDAR_SHOWCASE run wrote: sfx-*.wav (the synthesised sounds), audio.log (every sound the
game played, and the ambience levels each frame) and segments.txt (video time, game time and length per shot).
Each shot's sounds are placed at their video time and cut at the shot's end, as an editor would.

--bed renders just the wind-and-rumble bed of so many seconds (the title and end cards sit on one). In the
main mode, a bed is laid under every span in <dir>/cards.txt (the film's cards), and when cards are present the
filmed segments are level-matched to about -20 dBFS RMS before the soft clip, so the film's two parts sit at
the same loudness; --gain-db nudges the whole thing as a last correction.
"""
import math
import sys
import wave
from pathlib import Path

import numpy as np

RATE = 44100
argv = sys.argv[1:]
gain_db = 0.0
bed_mode = False
while argv and argv[0].startswith("--"):
    if argv[0] == "--gain-db":
        gain_db, argv = float(argv[1]), argv[2:]
    elif argv[0] == "--bed":
        bed_mode, argv = True, argv[1:]
    else:
        argv = argv[1:]
# `--bed <dir> <secs> <out.wav>` renders just the wind-and-rumble bed; else `<dir> <out.wav> <lead-in>`.
d = argv[0]
if bed_mode:
    bed_secs, out = float(argv[1]), argv[2]
else:
    out, lead = argv[1], float(argv[2])

def load(name):
    with wave.open(f"{d}/sfx-{name}.wav") as w:
        return np.frombuffer(w.readframes(w.getnframes()), dtype=np.int16).astype(np.float32) / 32767.0

def bed(n, fade=0.3):
    """Wind and far-off guns, wrapped to any length, with short edge fades."""
    mix = np.zeros(n, dtype=np.float32)
    for name, level in (("Wind", 0.25), ("Rumble", 0.3)):
        lp = load(name)
        mix += lp[np.arange(n) % len(lp)] * level
    f = min(n // 2, int(fade * RATE))
    mix[:f] *= np.linspace(0, 1, f)
    mix[-f:] *= np.linspace(1, 0, f)
    return mix * 10 ** (gain_db / 20)

if bed_mode:
    pcm = (np.clip(bed(int(bed_secs * RATE)), -1, 1) * 32767).astype(np.int16)
    with wave.open(out, "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(RATE)
        w.writeframes(pcm.tobytes())
    print(f"bed {bed_secs:.1f} s -> {out}")
    sys.exit(0)

segs = [tuple(map(float, l.split())) for l in Path(f"{d}/segments.txt").read_text().splitlines() if l.strip()]
total = segs[-1][0] + segs[-1][2] + lead
mix = np.zeros(int(total * RATE) + RATE, dtype=np.float32)
bank = {}

def seg_of(t):
    for vs, gs, ln in segs:
        if gs - 1e-3 <= t < gs + ln:
            return vs, gs, ln
    return None

amb = []
log = Path(f"{d}/audio.log").read_text().splitlines()
for line in log:
    p = line.split()
    if p[0] == "sfx":
        t, name, vol, speed = float(p[1]), p[2], float(p[3]), float(p[4])
        s = seg_of(t)
        if not s:
            continue
        vs, gs, ln = s
        if name not in bank:
            bank[name] = load(name)
        src = bank[name]
        if abs(speed - 1.0) > 1e-3:
            idx = np.arange(0, len(src) - 1, speed)
            src = np.interp(idx, np.arange(len(src)), src).astype(np.float32)
        start = int((lead + vs + (t - gs)) * RATE)
        end = min(int((lead + vs + ln) * RATE), start + len(src))  # hard cut at the shot's end
        if end <= start:
            continue
        chunk = src[: end - start] * vol
        fade = min(len(chunk), int(0.03 * RATE))
        chunk[-fade:] *= np.linspace(1, 0, fade)
        mix[start:end] += chunk
    elif p[0] == "amb":
        amb.append((float(p[1]), float(p[2]), int(p[3])))

# Ambience: wind and the distant war all along; engines and the drone's whine as the game had them.
loops = {k: load(k) for k in ("Wind", "Rumble", "Engines", "Rotor")}
amb_t = np.array([a[0] for a in amb]) if amb else np.zeros(1)
for vs, gs, ln in segs:
    a, b = int((lead + vs) * RATE), int((lead + vs + ln) * RATE)
    n = b - a
    tt = gs + np.arange(n) / RATE
    eng = np.interp(tt, amb_t, [x[1] for x in amb]) if amb else np.zeros(n)
    drone = np.interp(tt, amb_t, [x[2] for x in amb]) if amb else np.ones(n)
    off = int(gs * RATE)
    for name, level in (("Wind", 0.18 + 0.12 * (1 - drone)), ("Rumble", 0.35 * np.ones(n)), ("Engines", eng), ("Rotor", 0.05 * drone)):
        lp = loops[name]
        idx = (off + np.arange(n)) % len(lp)
        seg = lp[idx] * level
        f = min(n // 2, int(0.05 * RATE))
        seg[:f] *= np.linspace(0, 1, f)
        seg[-f:] *= np.linspace(1, 0, f)
        mix[a:b] += seg

# The lead-in under the title card: wind and far-off guns, fading in.
n = int(lead * RATE)
mix[:n] += (loops["Wind"][:n] * 0.25 + loops["Rumble"][:n] * 0.3) * np.linspace(0, 1, n)

# A bed under each card, so the cuts between shots never land on silence.
cards = []
cards_txt = Path(f"{d}/cards.txt")
if cards_txt.exists():
    for line in cards_txt.read_text().splitlines():
        if line.strip():
            head = line.split("\t")[0]
            vt, secs = head.split()
            cards.append((float(vt), float(secs)))
for vt, secs in cards:
    a, b = int((lead + vt) * RATE), int((lead + vt + secs) * RATE)
    mix[a:b] += bed(b - a)

# Loudness: the drone is far from most of it, so bring it up and soft-clip the peaks. The film's parts are
# level-matched over their filmed segments first, so both days sit at the same loudness; --gain-db nudges the
# whole thing as a last correction.
if cards:
    mask = np.zeros(len(mix), dtype=bool)
    for vs, gs, ln in segs:
        mask[int((lead + vs) * RATE):int((lead + vs + ln) * RATE)] = True
    rms = math.sqrt(float(np.mean(mix[mask] ** 2))) + 1e-9
    mix *= 10 ** ((-20.0 - 20 * math.log10(rms)) / 20)
peak = np.percentile(np.abs(mix), 99.9) + 1e-6
mix = np.tanh(mix / peak * 0.9) * 0.95
# A pure output gain: before the clip it would be squeezed away by the tanh.
if gain_db:
    mix *= 10 ** (gain_db / 20)
pcm = (np.clip(mix, -1, 1) * 32767).astype(np.int16)
with wave.open(out, "wb") as w:
    w.setnchannels(1)
    w.setsampwidth(2)
    w.setframerate(RATE)
    w.writeframes(pcm.tobytes())
print(f"mixed {total:.1f} s, {sum(1 for l in log if l.startswith('sfx'))} sounds logged"
      + (f", {len(cards)} cards" if cards else ""))
