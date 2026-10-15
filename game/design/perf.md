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

## Next levers, not pulled yet

- **The CPU, now the limit at 1080p.** Fewer entities: the 9000-particle cap; tree belts merged into fewer meshes
  where they are not already. Also drop Bevy plugins the game never uses (picking, `ui_widgets`).
- **The feed camera's 2D pass** at full resolution, about 0.6 ms at 2560x1600.
- **Worst frames** of 15–20 ms remain: spawn bursts (shell fans, cook-offs) and ground painting. They want a
  frame-time trace of their own.
