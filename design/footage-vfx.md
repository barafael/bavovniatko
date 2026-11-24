# What the footage says about the game's effects

*Notes, written 2026-10-11, from the reference footage in `research/footage-ref/`.*

> The smoke is white, and it lies down.

---

All 34 clips in `research/footage-ref/` were watched as contact sheets. They include drone spotter feeds, Orlan ISR, FPV, thermal and unit documentaries, from Skybyn 2022 to Kurakhove 2024. Each clip was compared with what the game draws now, and the key frames were checked one by one.

The footage itself stays out of git (see `research/footage-ref/README.md`). Citations here are `clip id @ m:ss`, and the clip's folder is in `CATALOG.md`.

The findings are ranked by how many clips agree and how far the game is from them.

## 1. Wreck smoke: the biggest mismatch

**What the game draws.** Tall, thin, black columns, identical and evenly spaced, all leaning the same way.

**What the footage shows.**
- **Colour and shape.** Burning vehicles give white to light-grey smoke that billows in cauliflower lobes.
  - It stays low, about two vehicle lengths high, then shears off downwind into a long cone that keeps widening, often ten or more vehicle lengths (AkGUe9yZGOU @ 0:27, KNNtzedEWg0 @ 0:31–0:36).
- **Black.** Black appears only as a dark core at the base or as a brief flare when fuel catches (dcYOjbyttvM @ 0:48, WQBisqWiITA @ 0:17).
- **Flame.** A small, saturated orange flame shows at the base even in daylight (CdbWTdRDrwQ @ 0:13, 4JQarq-IUEQ @ 1:13).
- **Variety.** No two wrecks look alike: the size, tint, lean and pulse differ.
- **Moving hits.** A vehicle hit on the move keeps rolling and drags a dense, low white trail 3–5 vehicle lengths long (rG46MZ-4XBY @ 1:04).

**In the game** (`fx.rs`):
- Swap the smoke colour ramp so it runs from a dark core to white.
- Lower the smoke and add a strong downwind shear and spread that grows with age.
- Vary each wreck's emitter (rate, size, tint).
- Add a small emissive flame sprite at the base.
- Give a vehicle that is still moving when hit a trail emitter.

## 2. Craters, tracks and snow

**What the game draws.** Ringed crater stamps.

**What the footage shows.**
- **Craters** (rG46MZ-4XBY @ 0:30, hVhmtEsaUQM @ 0:45, juNPLv_JJoE @ 0:00):
  - hundreds of small, soft, irregular dark splotches, a few vehicle lengths apart;
  - most are smaller than a vehicle;
  - there are no rings.
- **Fresh hits on snow** leave large, lopsided sooty fans instead (hVhmtEsaUQM @ 0:26, yrrso5JDR5I @ 0:28).
- **Snow** is never uniform: it is patchy, with mud and burnt scrub showing through (rG46MZ-4XBY @ 0:30, WUKVnvTrRdw @ 0:00).
- **Track marks** are the strongest marks on snow: a dense web of crisp black paired lines that records every turn and reverse (hVhmtEsaUQM @ 1:45, WUKVnvTrRdw @ 0:31).

**In the game** (`terrain.rs` ground paint, `ground.wgsl`):
- Replace the ringed stamp with a soft noisy blot, small by default.
- Add a directional soot fan for fresh hits on snow.
- Paint persistent paired track lines under every moving tracked vehicle.
- Break up the snow with noise so mud shows through.

## 3. Artillery impacts

**What the footage shows** (rG46MZ-4XBY @ 1:30 and 2:08, CS8uBrqrx2A @ 0:37, w6C1sGmQKAs @ 0:39 and 1:46, juNPLv_JJoE @ 0:09):
- **Shape.** A lumpy cauliflower puff 1–1.5 vehicle lengths wide at first, growing to 2–4.
- **Colour.** It takes the colour of the ground: white on snow, tan, beige or pink on summer earth, grey-brown on mud.
- **Flash.** At drone range there is no visible flash, or a single orange frame at most.
- **No ring.** There is no shockwave ring; at most a low ring of dust along the ground.
- **Lifetime.** The puff lingers 5–30 s. A barrage merges into a drifting haze over the field that casts a shadow on the ground.
- **Hits on vehicles and ammunition** (AkGUe9yZGOU @ 0:00, rG46MZ-4XBY @ 1:45–1:50, WUKVnvTrRdw @ 0:14, CdbWTdRDrwQ @ 0:30, GS7OxTrGJrM @ 0:30):
  - A fireball turns into a white cloud with orange glowing inside within about a second.
  - A delayed fan of white sparks follows.
  - Secondary explosions are clipped, pure-white balls.

**In the game** (`fx.rs`):
- Take the impact tint from the ground paint under it.
- Draw the puff as several overlapping lobes.
- Lengthen the lifetime and slow the drift.
- Drop or shrink the flash, and add a spark fan for hits on vehicles.

## 4. Dust behind vehicles

**What the game draws.** Nothing: there is no dust behind moving vehicles.

**What the footage shows** (GS7OxTrGJrM @ 1:53, 7vgP_G-Qo2g @ 1:20 and 2:58, T1ii61dPQwU @ 0:37, UB4MzKsr2-Q @ 0:00):
- Every moving vehicle drags a rooster tail 3–5 vehicle lengths long.
- In summer a column's tails merge into one drifting wall of dust, and at range that wall is often what makes the column visible at all.
- In winter the dust is pale and short.

**In the game** (`fx.rs`):
- Add a per-vehicle emitter scaled by speed and season, with long-lived, wind-advected particles.

## 5. The drone feed

**What the game draws.** Isolated black squares as dropouts, a monospace on-screen display, and smooth zoom.

**What the footage shows.**
- **Exposure blowout.** A nearby blast blows the whole frame to yellow-white, which recovers over about a second with purple and teal tint swings (9Tw8PZz3PRo @ 0:15).
- **Distance haze.** A blue-grey to lavender veil grows with distance and lifts the blacks, with an overall teal cast (CS8uBrqrx2A @ 0:11).
- **Compression.** Real compression never makes isolated black squares. It makes low-contrast smearing, worst inside smoke and dust, and bright events break the image into heavy blocks.
- **Zoom** jumps in steps, blurs, then refocuses. It is never a smooth dolly.
- **Two looks, one per side.**
  - The Mavic feed is crisp and saturated, with little grain.
  - Russian Orlan and long-zoom feeds are soft and purple-hazed.
- **On-screen display** (muQehXfIvZQ @ 0:08, 9Tw8PZz3PRo @ 0:27, KNNtzedEWg0 @ 0:06, yrrso5JDR5I @ 0:21):
  - **DJI:** a thin sans-serif font, not monospace.
    - A yellow corner-bracket box with a crosshair and centre dot.
    - A right-edge stack: binoculars icon, zoom as `9,0x`, `AF`, REC.
    - `H 432m D --m` along the bottom.
  - **Mavic 3** adds a green-outlined `ZOOM 8.4X` at the top centre and an ISO, shutter, EV and AE panel at the top right.
  - **Orlan:** corner brackets, a thin cross and a `Z:17` counter.

**In the game** (`feed.wgsl`, `hud.rs`, `camera.rs`):
- Drive the exposure blowout from nearby blasts.
- Add depth-based haze.
- Replace the black squares with smear and blocking.
- Make zoom steps snap with a short blur.
- Restyle the on-screen display as DJI, with an Orlan variant for the Russian side.

## 6. Smaller things

- **Smoke grenades** (yctq_FoLa4A @ 1:20): an orange pellet fan blooms into a white wall within about a second, with corkscrew trails. Some clips also show coloured marker smoke.
- **Tank gun:** a whiteout frame, then an orange ball, then a brown dust cloud hanging in front of the barrel.
- **25 mm fire** (yrrso5JDR5I @ 0:03): the tracer is a faint, thin white line for 1–2 frames, and hits on armour are small orange sparkles.
- **Fires on the ground:**
  - cook-off jets sideways out of hatches;
  - red-hot coals after the fire;
  - burning fuel streaks on snow.
- **Thermal** (H7-NM9EY-cQ @ 1:08, w6C1sGmQKAs @ 2:32):
  - It is white-hot, with scanline banding.
  - Explosions bloom and leave warm smears.
  - Some feeds split IR and RGB.
- **FPV** (9Tw8PZz3PRo @ 0:40): fisheye, analog noise bands, a Betaflight on-screen display, and breakup on impact.
- **Summer ground** (bk5OG6JYmfI, 4JQarq-IUEQ):
  - Fields are pale sage under lavender haze.
  - Tree lines are dense, dark blue-green in summer and splintered grey trunks in winter.

## Gaps

- No clip shows an ATGM launch.
- bk5OG6JYmfI (Robotyne) is helmet-cam only.

## Order of work

1. **Wreck smoke:** white, low, sheared by the wind, varied, with flame at the base, plus trails from hits on the move. This is the most visible change.
2. **The ground:** small soft craters, track marks, patchy snow and vehicle dust. All of this extends paint and particle systems that already exist.
3. **Impacts:** tinted by the ground, lumpy and long-lived.
4. **The feed:** exposure blowout, distance haze, smearing instead of black squares, and a DJI-style on-screen display.
