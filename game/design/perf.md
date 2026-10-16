# Frame time: what it costs, measured

*Written 2026-10-15. The machine is the owner's: a Framework Laptop 16 with only its Radeon 780M (RADV
PHOENIX), a 2560x1600 panel at 165 Hz, the desktop at 1.25 scale. Everything below is a headless run of the
`fast` or `release` build. Each run starts with `BAV_WARP_TO=240` and is measured over 30 s from there
(`BAV_DIAG=1`, the drone over the field, `BAV_OPERATOR=off`). Other sessions' builds share the machine, so a
single run is good to about ±1.5 ms. Compare rows taken the same evening.*

## How to measure

- **Frame and GPU times.** `BAV_DIAG=1` logs the mean and worst frame and the GPU time of each pass every 10 s.
  Bevy's diagnostics do not time the shadow pass, so its cost shows only in the frame. `BAV_OFF=` measures it.
- **Leaving parts out.** `BAV_OFF=shadows,msaa,feed,trees,terrain` leaves parts out, one row per part.
- **The CPU.** Build with `--features bevy/trace_chrome` into its own target dir (the trace file is gigabytes),
  then sum the spans over the last seconds.
- **The four knobs.** `BAV_SCALE`, `BAV_AA`, `BAV_SHADOW_MAP` and `BAV_CASCADES` change the picture's cost
  without a rebuild (README, "Development").

## What was done

| step | Vuhledar | Robotyne | at |
|---|---|---|---|
| start (machine loaded) | 20.6 ms | 18.9 ms | 1920x1080 |
| ground detail noise from a mipped texture, not 45 hashes per pixel | 16.5 | 15.4 | 1920x1080 |
| shadow filter `Hardware2x2` | 14.9 | 14.4 | 1920x1080 |
| the same, machine quieter (the baseline for the rest) | 18.2 | 18.0 | 2560x1600 |
| the 3D picture capped near 1920x1200 (`BAV_SCALE`) | 15.4 | 13.6 | 2560x1600 |
| shadows: three 2048 maps from 450 m, not two 4096 from 900 m | 12.1 | 12.5 | 2560x1600 |
| MSAA 2x, not 4x | **9.6** | **9.7** | 2560x1600 |

The same build at 1920x1080 is at 9.1 and 8.4 ms (11.3 and 13.5 before the last three steps).

Other settings measured on the way:

| setting | Vuhledar | Robotyne |
|---|---|---|
| two 2048 maps from 900 m | 11.4 | 11.4 |
| two 3072 maps from 900 m | 13.2 | 13.0 |
| two 2048 maps, SMAA instead of MSAA | 10.6 | 9.9 |
| two 2048 maps, MSAA 2x | 9.8 | 8.9 |

Choices:

- **Two 2048 maps** are the cheapest, but the tree belts' streaks of shade go short and soft. Three from 450 m
  look like the 4096 pair in side-by-side crops.
- **MSAA 2x** keeps the tree cards' alpha-to-coverage. Under the feed's grain it is hard to tell from 4x.
- **SMAA** makes the cards fall back to a hard alpha mask and costs more than 2x.

The 3D picture is a render target the feed pass samples. A smaller target costs nothing in sharpness of the
interface, which the flat camera draws at full resolution. Its scale factor keeps its logical size the window's,
which also fixed the cursor at a desktop scale of 1.25: before, a click for a fire mission or an evacuation landed
off the cursor, and the error grew towards the screen's edges.

## Where the frame goes now (1920x1080, from the trace)

The main world and the render world overlap (pipelined rendering). The frame is whichever is longer, plus the
extract between them:

- **The main world** is about 11 ms on a loaded machine. Our own systems are about 1.4 ms of it: the particles
  0.44 and the ground painting 0.34. The rest is Bevy's per-system cost across about 800 systems: visibility,
  transforms, the material specialisation checks and the executor itself.
- **The render world** is about 13 ms:
  - command encoding 3.6 ms (the 3D camera 2.7, the feed camera 0.8);
  - 15 queue submits a frame, 1.7 ms;
  - the prepare and queue systems, about 7 ms.

## The second round (2026-10-16)

With the machine quiet (load 2), the defaults at 2560x1600 run at **7.7 ms** in both dioramas. At that size:
- Halving the 3D picture gives 6.4 ms.
- Halving it and leaving the shadows out gives 3.9 ms.

So the sun's cascades are still the largest part, and the CPU floor is under 4 ms. Bevy's diagnostics do not time
the shadow pass; this split is the only measure of it.

Some measures and fixes:
- **Hitches.** `BAV_DIAG` now logs every frame over 25 ms with its game time. The 15–20 ms worst frames were
  rare. The 60–90 ms ones came at the end of the run, from saving the screenshot, not from the day.
- **CPU time per frame.** `BAV_DIAG` also logs the process's own CPU time per frame, all threads, from
  `/proc/self/stat`. Other sessions' builds stretch the wall clock (load 20 doubled the frame), but this barely.
- **Interleaved runs.** Under load, single runs could not tell variants apart. The variants below ran
  interleaved, three rounds, each over 30 ten-second windows (medians). Vuhledar at 2560x1600, load about 20:

| variant | frame | CPU, all threads | GPU 3D pass |
|---|---|---|---|
| defaults | 14.6 ms | 33.0 ms | 7.13 ms |
| `BAV_OFF=vshadows` (vehicles cast no shadow) | 14.9 | 30.9 | 7.09 |
| `BAV_OFF=particles` (2,200 particles at this moment) | 14.4 | 27.1 | 7.13 |
| without gamepads, animation and UI widget plugins | 14.8 | 30.2 | 7.00 |
| `BAV_OFF=trees` | **13.4** | 26.1 | **6.02** |

What this settles:
- **The vehicles' 30,000-triangle models** cost nothing measurable in the cascades. Low-poly shadow stand-ins
  would not pay; they were not built.
- **Dropping unused default plugins** gains nothing. That was tried and left out. `GltfPlugin` and the picking
  plugins cannot go anyway: PBR and the UI need their resources.
- **The particles** are about 6 ms of CPU a frame across threads. That does not show while the GPU is the
  limit, but it does when the CPU is (1080p, quiet machine). They now move across the task pool and are
  despawned afterwards in the query's order, so the ids the battle's later spawns get do not depend on thread
  timing.
- **Standing soldiers' legs** are no longer rewritten every frame, which sent every man through transform
  propagation.
- **The tree belts** are the one measurable lever left: about 1.1 ms of GPU and 1.2 ms of frame here. That is the
  alpha-tested cards in the main pass and in the three cascades.

### Where the trees' cost lies

Interleaved again, two rounds, both dioramas (medians of 20 windows, load 10-13):

| variant | Robotyne frame | GPU 3D pass | Vuhledar frame | GPU 3D pass |
|---|---|---|---|---|
| defaults | 11.6 ms | 5.16 ms | 13.35 ms | 6.70 ms |
| `BAV_OFF=treeshadows` (crowns cast none) | 8.8 | 4.31 | 11.65 | 6.29 |
| `BAV_OFF=trees` | 8.0 | 3.79 | 9.75 | 5.14 |

Most of the belts' cost is their crowns' shadows: alpha-tested cards drawn into three cascades.

**Tried and dropped: shrubs casting no shadow.** The shrub storey is about a third of the crown cards, and its
shade falls inside the belt's own. So it got a mesh of its own that cast none, and side by side the band of shade
looked the same. But three interleaved rounds (30 windows each, alternating which went first) showed no gain:

| | Robotyne | Vuhledar |
|---|---|---|
| frame, before | 9.8 ms | 10.4 ms |
| frame, after | 10.4 ms | 11.2 ms |

The cost is in the trees' cards, not the shrubs'; the extra mesh per chunk may even have cost a little. The change
was reverted.

## The third round: the belts' shade baked (2026-10-16)

The owner found the crown shadows odd-looking anyway, so they were replaced rather than thinned:

- **No shadow maps from trees.** Neither crowns nor trunks cast into the sun's cascades.
- **A baked occluder map.** At start, `terrain::occluder_map` bakes the canopy into a 2 m map over the whole
  diorama: crown underside, top and density, as a soft, slightly holed disc per crown and shrub. It also bakes
  a flag for every texel within about 100 m of a crown. The bake takes 150 ms.
- **The ground shades itself.** Near a belt, the ground shader walks that map towards wherever the sun is now,
  in twelve growing steps out to about 90 m. It then takes out the sun's direct light, using Bevy's own Burley
  diffuse term times what the shadow map already let through. Away from the belts it reads one texel and stops.
  The shade follows the sun all day, its edge soft and grainy; vehicles keep their real shadows. One loss:
  things standing in a belt's shade are not darkened, since only the ground reads the map.
- **Debug view.** `BAV_SHADE_DEBUG=1` shows the shade term alone; `BAV_OFF=shade` leaves it out.

The first version walked 20 steps on every ground pixel and included the terrain. It cost 2.5 ms more in the
main pass than the crown shadows had saved (GPU 3D pass 3.1 → 5.6 ms). Skipping open field and dropping the
terrain fixed that.

On a quiet machine, interleaved against the committed build, 30 windows each, at 2560x1600:

| | Robotyne | Vuhledar |
|---|---|---|
| crown cards in the cascades | 7.70 ms | 8.30 ms |
| baked shade | **7.30 ms** | **7.85 ms** |

The main pass costs about 0.7 ms more; the shadow pass saves more than that. Under load the crown shadows had
measured 1.7–2.8 ms; on a quiet machine the net gain is about 0.4 ms. The look changed more than the frame.

**The painted ground is cached.** It is kept in `~/.cache/bavovniatko/ground/<diorama>-<key>/` (about 70 MB of
PNGs). The key hashes the painting's code and the diorama's file. Loaded across the cores it takes 64 ms instead
of 2.7 s of painting, so a start is about half as long. Seed 0 of both days replays identically from the cache.

## Next levers, not pulled yet

- **Shade on what stands in the belts.** The men and vehicles there could read the same map.
- **The feed camera's 2D pass** at full resolution, about 0.6 ms at 2560x1600.
- **Worst frames.** Rare ones of 15–20 ms remain; the hitch log names their game time for a trace of their own.
