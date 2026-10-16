//! `BAV_SHOWCASE=<dir>`: a directed tour of the day for a showcase video. Headless only.
//!
//! The game runs on a fixed step (1/30 s of game time per rendered frame while filming, 1/20 s while skipping
//! ahead), so frames and game time stay locked however slowly frames render. Each filmed frame is saved to
//! `<dir>/f00000.png`…, `<dir>/segments.txt` maps game time to video time, and the audio module logs every sound
//! it plays to `<dir>/audio.log`, so the xtask (`xtask/src/mix.rs`) can rebuild the soundtrack offline.
//!
//! `BAV_TOUR=film` films the showcase film's two tours instead (design/showcase.md): shots typed with
//! cards before them, the interface switched per shot, two ground shots from an ATGM team, and two flown by the
//! stick script. Unset, the minute tours film exactly as they always did.

use crate::camera::{Flight, Locate, Mode, Target, View, enter_ground, look_angles, track_nearest};
use crate::fire::{Post, Strike, StrikeMission};
use crate::units::Radio;
use bav_sim::player::{Fpv, Operator};
use crate::units::{Soldier, Vehicle};
use crate::world::ObservationPost;
use bevy::{prelude::*, render::view::screenshot::{Screenshot, save_to_disk}, time::TimeUpdateStrategy};
use std::{io::Write, sync::LazyLock, time::Duration};

mod reel;

pub struct ShowcasePlugin;

impl Plugin for ShowcasePlugin {
    fn build(&self, app: &mut App) {
        if let Some(dir) = crate::dev::dev().showcase.clone() {
            std::fs::create_dir_all(&dir).ok();
            app.insert_resource(Tour { dir, shot: 0, frame: 0, video_t: 0.0, aim: None, fired: Vec::new(), shot_frames: 0, ground: None })
                .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(step_skip())))
                .add_systems(Update, direct.in_set(crate::Step::Control).before(crate::dev::DevSystems));
            if crate::dev::dev().tour == Some(crate::dev::TourKind::Reel) {
                app.add_systems(Update, log_events.in_set(crate::Step::Screen));
            }
        }
    }
}

const STEP_FILM: f64 = 1.0 / 30.0;
const STEP_SKIP: f64 = 1.0 / 20.0;

/// The step between shots. The reel skips on the filming step: the battle is integrated on its steps, so a coarser
/// skip would make the day depend on the shot list, and every retimed shot would change what it films.
fn step_skip() -> f64 {
    if matches!(crate::dev::dev().tour, Some(crate::dev::TourKind::Reel)) { STEP_FILM } else { STEP_SKIP }
}

#[derive(Clone, Copy)]
enum Aim {
    /// The drone, following a target.
    Drone(Target),
    /// A defender ATGM team's ground view, the team with missiles left nearest the point; the bool is the sight.
    Ground(Vec2, bool),
    /// The drone placed once at its first filmed frame, then flown by the shot's stick script.
    Fly(Target),
    /// The OP looking at a point (x, z) on the ground.
    #[expect(dead_code, reason = "the film's R9 may yet become the water tower; the camera supports it")]
    Op(Vec2),
    /// Through the FPV's goggles if it is up (the reel launches it on its own schedule); the view is left to it.
    Goggles,
}

use Aim::Drone;
use Target::{Men, MenNear, Point, Vehicle as Column, Wave};

/// A card the script renders over black before the shot: a title, a smaller line, a dim note.
struct Card {
    title: &'static str,
    line: &'static str,
    note: &'static str,
    secs: f32,
}

struct Shot {
    from: f32,
    secs: f32,
    pos0: Vec3,
    pos1: Vec3,
    aim: Aim,
    fov: (f32, f32),
    /// Fire missions to call (game time, strike) before or during the shot, on the nearest stalled vehicle.
    fire: &'static [(f32, Strike)],
    /// Fire missions called on a point (game time, strike, (x, z)): a jammer, a trench, a lane.
    fire_at: &'static [(f32, Strike, (f32, f32))],
    debrief: bool,
    /// A card over black before the shot; its frames are reserved, its time goes in the segment's front.
    card: Option<Card>,
    /// The interface in shot: the atmosphere shots go without.
    hud: bool,
    /// Stick-script legs (seconds into the shot, what the sticks do) for a flown shot.
    fly: &'static [(f32, f32, &'static str)],
    /// A ground shot fires its missile this many seconds in.
    launch: Option<f32>,
}

fn shot(from: f32, secs: f32, p0: (f32, f32, f32), p1: (f32, f32, f32), aim: Aim, fov: (f32, f32)) -> Shot {
    Shot {
        from,
        secs,
        pos0: Vec3::new(p0.0, p0.1, p0.2),
        pos1: Vec3::new(p1.0, p1.1, p1.2),
        aim,
        fov,
        fire: &[],
        fire_at: &[],
        debrief: false,
        card: None,
        hud: true,
        fly: &[],
        launch: None,
    }
}

/// The tour, shot by shot: the minute tours, or the film's.
static SHOTS: LazyLock<Vec<Shot>> = LazyLock::new(|| {
    match crate::dev::dev().tour {
        Some(crate::dev::TourKind::Film) => shots_film(),
        Some(crate::dev::TourKind::Reel) => reel::load().shots,
        None => shots_minute(),
    }
});

fn shots_minute() -> Vec<Shot> {
    let s = shot;
    if crate::diorama::diorama() == crate::diorama::Diorama::Robotyne {
        return vec![
            // The blue hour: our guns' preparation walking along the crest, the column not yet moving.
            s(20.0, 6.0, (100.0, -1100.0, 150.0), (140.0, -1070.0, 150.0), Drone(Point(Vec2::new(0.0, 700.0))), (22.0, 17.0)),
            // First light: the first group moves off down the western track behind the plow, raising dust.
            s(150.0, 7.0, (-300.0, -1250.0, 90.0), (-285.0, -1190.0, 90.0), Drone(Column(0)), (26.0, 18.0)),
            // The plow in the forward minefield: the lane, the markers, the first mine.
            s(470.0, 8.0, (-380.0, -720.0, 110.0), (-370.0, -700.0, 110.0), Drone(Column(0)), (9.0, 7.0)),
            // The second group bunching behind its stopped plow on the eastern track.
            s(600.0, 7.0, (420.0, -800.0, 140.0), (440.0, -780.0, 140.0), Drone(Point(Vec2::new(665.0, -360.0))), (12.0, 9.0)),
            // The men in the field, between the burning vehicles.
            s(690.0, 7.0, (-260.0, -700.0, 120.0), (-250.0, -690.0, 120.0), Drone(Point(Vec2::new(-492.0, -330.0))), (9.0, 7.0)),
            // Smoke on the wrecks, and the Bradleys that went back in for the men.
            Shot { fire: &[(745.0, Strike::Smoke)], ..s(760.0, 8.0, (-250.0, -800.0, 130.0), (-240.0, -790.0, 130.0), Drone(Point(Vec2::new(-492.0, -360.0))), (11.0, 8.0)) },
            // Late morning: the row of wrecks along the track, still smoking.
            s(1350.0, 5.0, (-650.0, -950.0, 240.0), (-630.0, -930.0, 240.0), Drone(Point(Vec2::new(-492.0, -330.0))), (16.0, 13.0)),
            Shot { debrief: true, ..s(1355.0, 4.0, (-630.0, -930.0, 240.0), (-630.0, -930.0, 240.0), Drone(Point(Vec2::new(-492.0, -330.0))), (13.0, 13.0)) },
        ];
    }
    vec![
        // Dawn over the January wrecks on the dacha road, before the first column.
        s(4.0, 6.0, (1150.0, 200.0, 70.0), (1220.0, 300.0, 60.0), Drone(Point(Vec2::new(1440.0, 530.0))), (26.0, 20.0)),
        // The 155th's column in single file behind its smoke screen.
        s(22.0, 8.0, (1000.0, -250.0, 170.0), (1000.0, -250.0, 170.0), Drone(Column(3)), (16.0, 11.0)),
        // From behind the tree line: the ATGM teams open up.
        s(68.0, 9.0, (1500.0, -300.0, 90.0), (1490.0, -320.0, 90.0), Drone(Column(0)), (22.0, 16.0)),
        // The kill zone: mines, wrecks, crews in the snow, a fire mission landing among them.
        Shot { fire: &[(80.0, Strike::He), (90.0, Strike::He)], ..s(97.0, 10.0, (750.0, -500.0, 120.0), (760.0, -480.0, 120.0), Drone(Column(2)), (5.0, 4.2)) },
        // The TOS-1A salvo landing on the town's southern blocks.
        s(164.0, 8.0, (250.0, -760.0, 140.0), (230.0, -780.0, 140.0), Drone(Point(Vec2::new(-290.0, -1150.0))), (34.0, 30.0)),
        // Afternoon: infantry on foot across the field, the battalion mortars working over them.
        s(1186.0, 7.0, (-640.0, -760.0, 90.0), (-630.0, -740.0, 90.0), Drone(Men), (7.0, 5.5)),
        // Dusk over the dacha road: everything that came down it today is still there.
        s(1470.0, 4.0, (950.0, -820.0, 200.0), (980.0, -800.0, 200.0), Drone(Point(Vec2::new(1130.0, -80.0))), (34.0, 30.0)),
        // The debrief.
        Shot { debrief: true, ..s(1474.0, 4.0, (980.0, -800.0, 200.0), (980.0, -800.0, 200.0), Drone(Point(Vec2::new(1130.0, -80.0))), (30.0, 30.0)) },
    ]
}

/// The film's tours (design/showcase.md): 250–600 m off and 100–200 m up, gimbal 45–70° down, fov 7–14°, the
/// horizon out of frame, no placed shot below fov 7, cards as hard cuts between shots. The `from` times are
/// first estimates from the beat table and the day-7 log; the scout retimes what the sheets disagree with.
fn shots_film() -> Vec<Shot> {
    let f = shot;
    let card = |title: &'static str, line: &'static str, note: &'static str, secs: f32| Card { title, line, note, secs };
    if crate::diorama::diorama() == crate::diorama::Diorama::Robotyne {
        return vec![
            // R1. The guns in the dark: 155 mm on the crest trench, where prep mission 8 lands 61–71. Drop if the
            //      sheet reads black.
            Shot {
                card: Some(card("Four months later. The other side of the minefield.", "South of МАЛА ТОКМАЧКА (MALA TOKMACHKA), 8 June 2023", "The 47th Mechanised Brigade goes forward into the Russian minefields. Now the columns are ours.", 5.0)),
                hud: false,
                ..f(60.0, 6.0, (-760.0, -120.0, 200.0), (-770.0, -100.0, 200.0), Drone(Point(Vec2::new(-930.0, 700.0))), (10.0, 8.0))
            },
            // R2. First light: the first group moves off down the western track; nine vehicles pass under the camera.
            f(132.0, 9.0, (-400.0, -1150.0, 160.0), (-410.0, -1115.0, 160.0), Drone(Column(3)), (14.0, 10.0)),
            // R3. Ground, eye level: the Leopard 2R with its plow passes the overwatch position, the head following.
            f(152.0, 10.0, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Aim::Ground(Vec2::new(-520.0, crate::geo::layout().start_z.expect("Mala Tokmachka has a start line")), false), (54.0, 54.0)),
            // R4. The plow in the forward minefield: the lane, the first mine (a double, under the plow).
            f(318.0, 12.0, (-420.0, -470.0, 140.0), (-430.0, -440.0, 140.0), Drone(Column(0)), (8.0, 7.0)),
            // R5. The second group, eleven, on the eastern track.
            Shot {
                card: Some(card("05:29", "The second group, eleven vehicles, on the eastern track.", "", 2.5)),
                ..f(400.0, 9.0, (720.0, -700.0, 150.0), (700.0, -660.0, 150.0), Drone(Wave(2, 0)), (12.0, 9.0))
            },
            // R6. Stopped vehicles under the Ka-52s; crews out. The helicopters arrive from t=430 by the script.
            f(455.0, 12.0, (-560.0, -430.0, 150.0), (-570.0, -400.0, 150.0), Drone(Column(1)), (9.0, 7.0)),
            // R7. The men in the field between the burning vehicles.
            f(560.0, 10.0, (-540.0, -380.0, 110.0), (-550.0, -355.0, 110.0), Drone(MenNear(Vec2::new(-492.0, -300.0))), (8.0, 7.0)),
            // R8. Flown: trailing the rescue Bradleys up the lane, smoke blooming ahead (called at 646 for 672–680).
            //     Fallback if the framing fails: placed (-430,-420,130) → (-440,-390,130), Wave(3,0), 9→7, from 690.
            Shot {
                card: Some(card("07:02", "Four Bradleys go back in for the men.", "", 2.5)),
                fire: &[(646.0, Strike::Smoke)],
                fly: &[(0.0, 14.0, "w-")],
                ..f(668.0, 14.0, (-497.0, -780.0, 120.0), (0.0, 0.0, 0.0), Aim::Fly(Wave(3, 0)), (12.0, 12.0))
            },
            // R9. Walking back: the men go down the slope past the wrecks. The owner's option: the water tower.
            Shot {
                card: Some(card("08:50", "Everyone back to the start line.", "", 2.5)),
                ..f(1010.0, 8.0, (-560.0, -700.0, 120.0), (-570.0, -675.0, 120.0), Drone(MenNear(Vec2::new(-495.0, -600.0))), (9.0, 8.0))
            },
            // R10. Late morning: the row of wrecks along the track, still smoking. "Quiet on the lanes" in-shot.
            f(1300.0, 8.0, (-600.0, -520.0, 200.0), (-590.0, -480.0, 200.0), Drone(Point(Vec2::new(-492.0, -330.0))), (14.0, 12.0)),
            // R11. The ledger.
            Shot { debrief: true, hud: false, ..f(1308.0, 7.0, (-590.0, -480.0, 200.0), (-590.0, -480.0, 200.0), Drone(Point(Vec2::new(-492.0, -330.0))), (12.0, 12.0)) },
        ];
    }
    vec![
        // V1. Dawn: the January wrecks on the dacha road, near-overhead, ends before the 06:45 banner.
        Shot {
            card: Some(card("ВУГЛЕДАР (VUHLEDAR)", "6 February 2023", "Russian columns attack the 72nd Mechanised Brigade. The roads are left clear; the fields beside them are mined.", 5.0)),
            hud: false,
            ..f(6.0, 7.0, (1270.0, 250.0, 140.0), (1250.0, 215.0, 140.0), Drone(Point(Vec2::new(1228.0, 203.0))), (12.0, 9.0))
        },
        // V2. The column in single file behind its smoke; the net's report and the first jamming land in-shot.
        f(15.0, 9.0, (1130.0, 70.0, 170.0), (1110.0, 10.0, 170.0), Drone(Column(2)), (14.0, 10.0)),
        // V3. Ground: the Stugna-P sight on the lead tank, the launch at +4 s. The fault of every earlier cut
        //     (missiles as specks) answered from the sight.
        Shot { launch: Some(4.0), ..f(62.0, 12.0, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Aim::Ground(Vec2::new(930.0, -658.0), true), (54.0, 54.0)) },
        // V4. The kill zone, wide: the column stacks at the deploy point. Slide to 82 if the lead is not there yet.
        f(76.0, 10.0, (1110.0, -380.0, 170.0), (1090.0, -410.0, 170.0), Drone(Column(1)), (11.0, 8.0)),
        // V5. The kill zone, close: mines, crews in the snow, 155 mm landing (fires at 76 and 84, lands 102/110);
        //     fates and bail-outs.
        Shot { fire: &[(76.0, Strike::He), (84.0, Strike::He)], ..f(100.0, 10.0, (1060.0, -250.0, 140.0), (1040.0, -290.0, 140.0), Drone(Column(2)), (8.0, 7.0)) },
        // V6. TOS-1A on the southern blocks, rockets landing 165–175; from the drone, no parapet.
        f(163.0, 10.0, (-110.0, -900.0, 160.0), (-130.0, -925.0, 160.0), Drone(Point(Vec2::new(-290.0, -1170.0))), (20.0, 16.0)),
        // V7. The second column passes the morning's wrecks at the middle belt; `Vehicle(0)` would pick wave 1's wreck.
        Shot {
            card: Some(card("08:40", "A second column. The same road.", "", 2.5)),
            ..f(470.0, 10.0, (1190.0, -50.0, 150.0), (1170.0, -100.0, 150.0), Drone(Wave(2, 0)), (10.0, 8.0))
        },
        // V8. Alga's BMP-1s in the shoulder mines on the Pavlivka road.
        Shot {
            card: Some(card("10:20", "The Alga battalion, from Павлівка (Pavlivka).", "", 2.5)),
            ..f(790.0, 10.0, (-740.0, -240.0, 150.0), (-760.0, -280.0, 150.0), Drone(Wave(3, 1)), (10.0, 7.0))
        },
        // V9. Flown: the operator pushes in over the third column at its wrecks — hover, fly in sport, zoom, hover.
        //     Fallback if the framing fails: placed (1120,-150,150) → (1100,-190,150), Wave(4,0), 12→8.
        Shot {
            fly: &[(3.0, 9.0, "w+"), (9.0, 11.0, "z")],
            ..f(1036.0, 12.0, (1250.0, -80.0, 160.0), (0.0, 0.0, 0.0), Aim::Fly(Wave(4, 0)), (20.0, 20.0))
        },
        // V10. Infantry on foot under the mortars: 100 m up, a man about 120 px tall at 1080p.
        Shot {
            card: Some(card("13:20", "No more armour. Infantry, on foot.", "", 2.5)),
            ..f(1190.0, 10.0, (-900.0, 300.0, 100.0), (-910.0, 275.0, 100.0), Drone(MenNear(Vec2::new(-950.0, 250.0))), (9.0, 7.0))
        },
        // V11. Late afternoon over the dacha road: everything that came down it is still there.
        f(1440.0, 8.0, (1060.0, -630.0, 220.0), (1090.0, -570.0, 220.0), Drone(Point(Vec2::new(1040.0, -330.0))), (22.0, 18.0)),
        // V12. The ledger, before the day's end.
        Shot { debrief: true, hud: false, ..f(1448.0, 7.0, (1090.0, -570.0, 220.0), (1090.0, -570.0, 220.0), Drone(Point(Vec2::new(1040.0, -330.0))), (18.0, 18.0)) },
    ]
}

#[derive(Resource)]
struct Tour {
    dir: String,
    shot: usize,
    frame: u32,
    video_t: f64,
    aim: Option<Vec3>,
    fired: Vec<(usize, usize)>,
    /// Frames filmed in the current shot (shots are timed by frames: the debrief stops the game clock).
    shot_frames: u32,
    /// The shot whose ground view is entered, so it happens once.
    ground: Option<usize>,
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn direct(
    mut commands: Commands,
    mut tour: ResMut<Tour>,
    mut strategy: ResMut<TimeUpdateStrategy>,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    target: Option<Res<crate::camera::HeadlessTarget>>,
    op: Res<ObservationPost>,
    mut view: ResMut<View>,
    mut next_phase: ResMut<NextState<crate::Phase>>,
    mut fc: ResMut<crate::fire::FireControl>,
    mut flight: ResMut<Flight>,
    mut hud: ResMut<crate::hud::HudHidden>,
    (vehicles, men): (Query<(&Vehicle, &Transform)>, Query<(&Soldier, &Transform)>),
    (mut fpv, operator, mut radio): (ResMut<Fpv>, Res<Operator>, ResMut<Radio>),
    teams: Query<(Entity, &Post)>,
    mut exit: MessageWriter<AppExit>,
) {
    let t = script.t;
    // The reel's interventions are the day's, whatever is being filmed: every run makes them at their times.
    if crate::dev::dev().tour == Some(crate::dev::TourKind::Reel) {
        let now = time.elapsed_secs_f64();
        for (k, &(at, kind, p)) in reel::PLAN.fires.iter().enumerate() {
            if t >= at && !tour.fired.contains(&(usize::MAX, k)) {
                tour.fired.push((usize::MAX, k));
                fc.missions.push(kind.mission(p, 12.0, now, true));
                info!("showcase: {kind:?} called on {:.0},{:.0}, at game {t:.0} s", p.x, p.y);
            }
        }
        if let Some((at, p)) = reel::PLAN.fpv
            && t >= at
            && !tour.fired.contains(&(usize::MAX, usize::MAX))
        {
            tour.fired.push((usize::MAX, usize::MAX));
            fpv.auto = Some(p);
            bav_sim::drones::launch_fpv(&mut fpv, &mut view, &operator, &mut radio, now);
        }
    }
    let Some(shot) = SHOTS.get(tour.shot) else {
        exit.write(AppExit::Success);
        return;
    };
    let si = tour.shot;
    hud.set_if_neq(crate::hud::HudHidden(!shot.hud));
    // Fire missions for this shot, called at their own times (possibly before the shot starts).
    for (k, &(at, kind)) in shot.fire.iter().enumerate() {
        if t >= at && !tour.fired.contains(&(si, k)) {
            tour.fired.push((si, k));
            let at = kind.on_nearest_stalled(&mut fc, &vehicles, view.drone_pos.xz(), time.elapsed_secs_f64());
            info!("showcase: {kind:?} called on {at:?}, at game {t:.0} s");
        }
    }
    for (k, &(at, kind, (x, z))) in shot.fire_at.iter().enumerate() {
        if t >= at && !tour.fired.contains(&(si, 100 + k)) {
            tour.fired.push((si, 100 + k));
            fc.missions.push(kind.mission(Vec2::new(x, z), 12.0, time.elapsed_secs_f64(), true));
            info!("showcase: {kind:?} called on {x:.0},{z:.0}, at game {t:.0} s");
        }
    }
    // A ground shot enters its team's tree line on the shot's first frame, and during the skip, so the camera
    // is already there when filming starts.
    if let Aim::Ground(near, sight) = shot.aim
        && tour.ground != Some(si)
        && let Some((e, p)) = teams.iter().filter(|(_, p)| p.atgm > 0).min_by(|a, b| a.1.pos.distance(near).total_cmp(&b.1.pos.distance(near)))
    {
        enter_ground(&mut view, e, p.pos);
        view.sight = sight;
        tour.ground = Some(si);
    }
    if t < shot.from {
        // Skip ahead, unfilmed. Slow to the filming step just before the shot so it starts on time.
        let step = if shot.from - t > 1.0 { step_skip() } else { STEP_FILM };
        *strategy = TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(step));
        tour.aim = None;
        place_camera(&mut view, shot, 0.0, &mut tour.aim, &op, &vehicles, &men);
        if matches!(shot.aim, Aim::Ground(..)) {
            track_nearest(&mut view, &vehicles);
        }
        return;
    }
    *strategy = TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(STEP_FILM));
    let total = (shot.secs as f64 / STEP_FILM).round() as u32;
    let k = tour.shot_frames as f32 / total as f32;
    // A flown shot is placed once at its first filmed frame; from there the stick script flies it and the
    // placement touches nothing.
    if let Aim::Fly(_) = shot.aim {
        if tour.shot_frames == 0 {
            place_fly(&mut view, shot, &mut flight, &vehicles, &men);
        }
    } else if let Aim::Goggles = shot.aim {
        view.locked = false;
        if fpv.flying {
            view.mode = crate::camera::Mode::Fpv;
        }
    } else {
        place_camera(&mut view, shot, k, &mut tour.aim, &op, &vehicles, &men);
    }
    // The debrief shot ends the day, once: re-setting the state every frame would re-fire `OnEnter` (and
    // `show_debrief`) every frame.
    if shot.debrief && tour.shot_frames == 0 {
        next_phase.set(crate::Phase::Debrief);
    }
    // The ground shot's launch: on the one filmed frame where the shot's clock crosses it.
    let step = STEP_FILM as f32;
    let clock = tour.shot_frames as f32 * step;
    view.launch = shot.launch.is_some_and(|at| clock >= at && clock - step < at);
    if tour.shot_frames == 0 {
        // A card goes before the shot: the script renders it into the frames reserved here, so the segment,
        // its sounds and its bed all start after the card.
        if let Some(card) = &shot.card {
            append(&format!("{}/cards.txt", tour.dir), &format!("{:.4} {}\t{}\t{}\t{}\n", tour.video_t, card.secs, card.title, card.line, card.note));
            tour.video_t += card.secs as f64;
            tour.frame += (card.secs as f64 / STEP_FILM).round() as u32;
        }
        // Segment start: video time, game (virtual) time, length.
        let line = format!("{:.4} {:.4} {:.4}\n", tour.video_t, time.elapsed_secs_f64(), shot.secs);
        append(&format!("{}/segments.txt", tour.dir), &line);
    }
    if let Some(target) = target.as_ref() {
        commands.spawn(Screenshot::image(target.0.clone())).observe(save_to_disk(format!("{}/f{:05}.png", tour.dir, tour.frame)));
    }
    tour.frame += 1;
    tour.shot_frames += 1;
    tour.video_t += STEP_FILM;
    if tour.shot_frames >= total {
        tour.shot += 1;
        tour.shot_frames = 0;
        tour.aim = None;
        tour.ground = None;
    }
}

fn place_camera(view: &mut View, shot: &Shot, k: f32, aim: &mut Option<Vec3>, op: &ObservationPost, vehicles: &Query<(&Vehicle, &Transform)>, men: &Query<(&Soldier, &Transform)>) {
    let ease = k * k * (3.0 - 2.0 * k);
    let fov = (shot.fov.0 + (shot.fov.1 - shot.fov.0) * ease).to_radians();
    // The view is placed by hand this frame: the flight controller and the ground view's own controls stand down.
    view.locked = true;
    let target = match shot.aim {
        Aim::Op(p) => {
            view.mode = Mode::Op;
            view.op_fov = fov;
            (view.op_yaw, view.op_pitch) = look_angles(crate::geo::ground(p) - op.eye);
            return;
        }
        // The ground view was placed by `enter_ground`; the gunner's head tracks.
        Aim::Ground(..) => return,
        // The FPV flies itself; the drone waits where the last shot left it.
        Aim::Goggles => {
            view.locked = false;
            return;
        }
        Aim::Drone(target) | Aim::Fly(target) => target,
    };
    view.mode = Mode::Drone;
    view.drone_fov = fov;
    let p = shot.pos0.lerp(shot.pos1, ease);
    view.drone_pos = Vec3::new(p.x, crate::geo::height(p.x, p.y) + p.z, p.y);
    let Some(target) = target.locate(view.drone_pos, vehicles, men) else { return };
    let a = aim.get_or_insert(target);
    *a = a.lerp(target, 0.06);
    (view.drone_yaw, view.gimbal) = look_angles(*a - view.drone_pos);
}

/// A flown shot's one placement: the drone set down at `pos0` looking at the shot's target, its legs installed
/// in game time, and the aircraft handed to the flight controller.
fn place_fly(view: &mut View, shot: &Shot, flight: &mut Flight, vehicles: &Query<(&Vehicle, &Transform)>, men: &Query<(&Soldier, &Transform)>) {
    let Aim::Fly(target) = shot.aim else { return };
    view.mode = Mode::Drone;
    view.drone_fov = shot.fov.0.to_radians();
    view.fov_target = view.drone_fov;
    let p = shot.pos0;
    view.drone_pos = Vec3::new(p.x, crate::geo::height(p.x, p.y) + p.z, p.y);
    if let Some(at) = target.locate(view.drone_pos, vehicles, men) {
        let (yaw, gimbal) = look_angles(at - view.drone_pos);
        view.drone_yaw = yaw;
        view.gimbal = gimbal;
        view.yaw_target = yaw;
        view.gimbal_target = gimbal;
    }
    view.drone_vel = Vec3::ZERO;
    view.hold = None;
    view.tilt = Vec3::ZERO;
    view.rth = None;
    view.locked = false;
    flight.0 = shot.fly.iter().map(|&(a, b, s)| (shot.from + a, shot.from + b, s.to_string())).collect();
}

pub fn append(path: &str, line: &str) {
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}

/// The reel's events, for `cargo xtask trailer` to find the action in: every blast and every hit on a vehicle,
/// with the game time, where, and how big, one a line in `<dir>/events.txt`:
/// `<game s> blast <cause> <x> <z> <radius>` and `<game s> hit <weapon> <x> <z> <side>`.
fn log_events(
    tour: Res<Tour>,
    script: Res<crate::director::Script>,
    mut blasts: MessageReader<crate::units::Blast>,
    log: Res<bav_sim::hits::HitLog>,
    places: Query<&Transform>,
    mut last: Local<f64>,
) {
    let path = format!("{}/events.txt", tour.dir);
    let t = script.t;
    for b in blasts.read() {
        append(&path, &format!("{t:.2} blast {:?} {:.0} {:.0} {:.1}\n", b.cause, b.at.x, b.at.z, b.radius));
    }
    for h in log.entries.iter().filter(|h| h.at > *last) {
        let Ok(tf) = places.get(h.target) else { continue };
        append(&path, &format!("{t:.2} hit {} {:.0} {:.0} {:?}\n", h.weapon.replace(' ', "_"), tf.translation.x, tf.translation.z, h.side));
    }
    *last = log.entries.iter().map(|h| h.at).fold(*last, f64::max);
}
