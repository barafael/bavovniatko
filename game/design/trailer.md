# The trailer: the action of both dioramas (2026-10-16)

The owner asked for a trailer of the two dioramas: everything the game can show, cinematic, at a resolution fit
for a demo. The first cut (about 5:20, a shot list timed by hand like the showcase film's) was half empty: the
day moved under its shots every time the battle changed, and the quiet shots of wrecks and observers' posts ran
long. The owner's note: just the action, but more of it; the order of the battle's events does not matter.

So the battle picks the shots. The trailer is an action reel, about two and a quarter minutes: a title, a card and
the action of Vuhledar, a card and the action of Mala Tokmachka, an end card. No narrator and no music; the
radio net, the hit feed and the sound of the guns carry it.

```sh
cargo xtask trailer            # 1920x1080, x264 crf 24, into devlog/videos/
cargo xtask trailer --scout    # 640x360 with a contact sheet, into devlog/film/
```

## How it is made (`xtask/src/reel.rs`, `crates/bavovniatko/src/showcase/reel.rs`)

1. **The day.** What the player does is fixed per day (`reel::DAYS`): fire missions on points (155 mm on the
   columns, RAAM and HIMARS at Vuhledar; the jammer and HIMARS on a dug-in tank south of Mala Tokmachka), the
   FPV launched at the western dug-in T-72, and the Bradleys sent for the wounded half a minute after each call (`BAV_EVAC=30`). The reel tour (`BAV_TOUR=reel`, `BAV_REEL=<file>`) makes them in every run;
   it skips between shots on its filming step and calls nothing on the camera's position, so the day does not
   depend on the shots, and every run is the same day.
2. **The events pass.** Each day runs once, filming nothing, at 320x180, both days at once. The tour logs every
   blast (cause, place, radius) and every hit on a vehicle (weapon, place) to `events.txt`.
3. **The moments.** Every heavy event is the heart of a candidate, the events within 4 s and 150 m its body,
   weighted: a vehicle hit or a mine strike 6, the FPV 9, HIMARS 5, a cook-off 4, a TOS rocket 1.6, a shell 0.7,
   the tanks' own gunfire (thousands of small splashes) next to nothing. Only a vehicle hit, HIMARS, a cook-off,
   the FPV or a TOS salvo of four or more is a heart: in the first scout, shells on open ground and on the crest
   trench made half the moments, and nothing showed in them. The twelve best of each day that are not the same
   scene as a better one (within 14 s and 280 m) are filmed: the drone about 130 m off and 240 m up, the gimbal
   some 60 degrees down (at 46 the tree lines hid the vehicles along the tracks), from one of eight sides in turn, pushing in a little, the field of view holding the
   moment's spread; from 1.8 s before its first blast to 2.4 s after its last, 4 to 5.5 s. Besides them, the
   FPV's last ten seconds through its goggles. (A Stugna-P launch through the gunner's sight was tried: the hit is
   a speck on the horizon, ten seconds of nothing.)
4. **The runs.** Moments that overlap in time cannot be filmed in one run; first fit in time order shares them
   among as few runs as it takes. Each run's sound is rebuilt from its log and levelled to -18 LUFS, and each
   moment is cut out of its run.
5. **The cut.** Each day's moments in an order of their own, not the battle's: the second best opens, then from
   the least up, the best last; the FPV's dive closes Mala Tokmachka.

The events passes' logs and events are kept in `devlog/film/reel-events-*.{log,txt}`, the runs' logs in
`devlog/film/reel-<day>-<run>.log`.
