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

## Next levers, not pulled yet

- **The tree belts' cards.** Coarser crowns far off (a `VisibilityRange` LOD), or crowns that cast only into
  the near cascade.
- **The feed camera's 2D pass** at full resolution, about 0.6 ms at 2560x1600.
- **Worst frames.** Rare ones of 15–20 ms remain; the hitch log names their game time for a trace of their own.
