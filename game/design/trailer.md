# The trailer: both dioramas in five minutes (2026-10-16)

The owner asked for a trailer of the two dioramas: about five minutes, everything the game can show now, cinematic,
at a resolution fit for a demo. It is the showcase film ([`showcase.md`](showcase.md)) grown by what the dioramas
gained since 2026-10-12: the observer's roof, RAAM and HIMARS at Vuhledar; their EW as a field (a jammer's mast
and lamp, one shelled out, the GNSS tell before the Ka-52s), the FPV flown through its goggles and the Bradleys
sent for the wounded south of Mala Tokmachka; and the hit feed on every operator's shot.

The film's decisions hold: one minefield seen from both sides, Vuhledar then Mala Tokmachka; cards and the radio
net, no narrator and no music; English cards with the Ukrainian place names; the interface off for the atmosphere
shots; the ledgers on the end card. So do its rules for every shot (250–600 m off, 100–200 m up, gimbal 45–70°
down, fov 7–14°, the horizon out of frame, cards as hard cuts).

```sh
scripts/trailer.sh             # 1920x1080, x264 crf 24, into devlog/videos/
scripts/trailer.sh --scout     # 640x360 with contact sheets, into devlog/film/
```

`trailer.sh` sets up the numpy venv the mixer needs, films through `showcase.sh --trailer`, and then checks the
game's logs for the moments the shots wait for (the jammer shelled out, the first Bradley stopping, the FPV's hit):
each is `ok`, `MOVED` (with where it went) or `MISSING`. The day is deterministic, so a warning means a change to
the battle moved it, and the shot in `trailer.rs` wants retiming: scout, read the sheets and the warning, move the
shot. Each part's sound is measured and mixed again to -18 LUFS, so the parts stay level whatever the day does.

`BAV_TOUR=trailer` selects the shot lists in `crates/bavovniatko/src/showcase/trailer.rs`. Two kinds of shot
are new to it: `Aim::Fpv(point)` launches the FPV at the shot's first frame (if it is not already up) and lets
it fly itself at the enemy vehicle nearest the point (`Fpv::auto`, which `BAV_FPV` sets too), the view left to
it; and `fire_at`, a mission called on a point rather than on the nearest stalled vehicle (the jammer, the RAAM
lane). The Mala Tokmachka part runs with `BAV_EVAC=30`: the Bradleys go only when sent, and the trailer sends
them as a player would.

## The cut (about 5:20)

Title 5 s; Vuhledar about 160 s with its cards; Mala Tokmachka about 150 s with the intertitle; end card 10 s.

What the scouts taught: a shot's missions are called only while it is the next shot, so a mission must be
timed after the shot before it ends; the second column at Vuhledar is slow (its lead near 1250,160 at 580 s);
from close in over Mala Tokmachka's burning vehicles the drone sees only smoke; and the FPV's autopilot came
in short into a dug-in tank's berm or long past it until its dive started at three times its height. And the
tour skipped between shots on a coarser step than it films on, so every change to the shot list changed the
day: the trailer skips on the filming step, and a scout now films the same day as the full run.

| part | shots |
|---|---|
| Вугледар | dawn over the January wrecks · the column behind its smoke · the Stugna-P sight and launch · the kill zone wide · close, 155 mm landing · TOS-1A on the town · *08:40* the second column past the wrecks · RAAM onto its lane · HIMARS on its stalled tail · the lead onto the scattered mines · *10:20* Alga on the Pavlivka road · the observers' roof · flown over the third column · *13:20* infantry under the mortars · late afternoon · the ledger |
| Мала Токмачка | a jammer's lamp behind the crest, in the dark · first light, the first group · eye level at the start line · the plow in the forward minefield · 155 mm on the jammer, then the GNSS tell · *05:29* the second group · under the Ka-52s · *06:22* a Bradley for the wounded · *06:47* the FPV out over the column, and the dive · *08:50* the pull-back from the water tower · the wrecks still smoking · the ledger |
