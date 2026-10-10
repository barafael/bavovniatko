#!/usr/bin/env python3
"""Rebuild the showcase soundtrack offline from the game's audio log.

Usage: showcase_mix.py <dir> <out.wav> <lead-in seconds>

<dir> holds what a VUHLEDAR_SHOWCASE run wrote: sfx-*.wav (the synthesised sounds), audio.log (every sound the
game played, and the ambience levels each frame) and segments.txt (video time, game time and length per shot).
Each shot's sounds are placed at their video time and cut at the shot's end, as an editor would.
"""
import sys, wave
import numpy as np

RATE = 44100
d, out, lead = sys.argv[1], sys.argv[2], float(sys.argv[3])

def load(name):
    with wave.open(f"{d}/sfx-{name}.wav") as w:
        return np.frombuffer(w.readframes(w.getnframes()), dtype=np.int16).astype(np.float32) / 32767.0

segs = [tuple(map(float, l.split())) for l in open(f"{d}/segments.txt") if l.strip()]
total = segs[-1][0] + segs[-1][2] + lead
mix = np.zeros(int(total * RATE) + RATE, dtype=np.float32)
bank = {}

def seg_of(t):
    for vs, gs, ln in segs:
        if gs - 1e-3 <= t < gs + ln:
            return vs, gs, ln
    return None

amb = []
for line in open(f"{d}/audio.log"):
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

# Loudness: the drone is far from most of it, so bring it up and soft-clip the peaks.
peak = np.percentile(np.abs(mix), 99.9) + 1e-6
mix = np.tanh(mix / peak * 0.9) * 0.95
pcm = (np.clip(mix, -1, 1) * 32767).astype(np.int16)
with wave.open(out, "wb") as w:
    w.setnchannels(1)
    w.setsampwidth(2)
    w.setframerate(RATE)
    w.writeframes(pcm.tobytes())
print(f"mixed {total:.1f} s, {sum(1 for l in open(d + '/audio.log') if l.startswith('sfx'))} sounds logged")
