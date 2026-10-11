//! `VUHLEDAR_SHOWCASE=<dir>`: a directed tour of the day for a showcase video. Headless only.
//!
//! The game runs on a fixed step (1/30 s of game time per rendered frame while filming, 1/20 s while skipping
//! ahead), so frames and game time stay locked however slowly frames render. Each filmed frame is saved to
//! `<dir>/f00000.png`…, `<dir>/segments.txt` maps game time to video time, and the audio module logs every sound
//! it plays to `<dir>/audio.log`, so `scripts/showcase.py` can rebuild the soundtrack offline.

use crate::camera::{Mode, Target, View, look_angles};
use crate::fire::Strike;
use crate::units::{Soldier, Vehicle};
use crate::world::ObservationPost;
use bevy::{prelude::*, render::view::screenshot::{Screenshot, save_to_disk}, time::TimeUpdateStrategy};
use std::{io::Write, sync::LazyLock, time::Duration};

pub struct ShowcasePlugin;

impl Plugin for ShowcasePlugin {
    fn build(&self, app: &mut App) {
        if let Some(dir) = crate::dev::dev().showcase.clone() {
            std::fs::create_dir_all(&dir).ok();
            app.insert_resource(Tour { dir, shot: 0, frame: 0, video_t: 0.0, aim: None, fired: Vec::new(), shot_frames: 0 })
                .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(STEP_SKIP)))
                .add_systems(Update, direct.in_set(crate::Step::Control).before(crate::dev::DevSystems));
        }
    }
}

const STEP_FILM: f64 = 1.0 / 30.0;
const STEP_SKIP: f64 = 1.0 / 20.0;

#[derive(Clone, Copy)]
enum Aim {
    /// The drone, following a target.
    Drone(Target),
    /// The OP looking at a point (x, z) on the ground.
    #[expect(dead_code, reason = "no shot in the current tour looks through the OP, but the camera supports it")]
    Op(Vec2),
}

use Aim::Drone;
use Target::{Men, Point, Vehicle as Column};

struct Shot {
    from: f32,
    secs: f32,
    pos0: Vec3,
    pos1: Vec3,
    aim: Aim,
    fov: (f32, f32),
    /// Fire missions to call (game time, strike) before or during the shot, on the nearest stalled vehicle.
    fire: &'static [(f32, Strike)],
    debrief: bool,
}

/// The tour, shot by shot.
static SHOTS: LazyLock<Vec<Shot>> = LazyLock::new(shots);

fn shots() -> Vec<Shot> {
    let s = |from: f32, secs: f32, p0: (f32, f32, f32), p1: (f32, f32, f32), aim: Aim, fov: (f32, f32)| Shot {
        from,
        secs,
        pos0: Vec3::new(p0.0, p0.1, p0.2),
        pos1: Vec3::new(p1.0, p1.1, p1.2),
        aim,
        fov,
        fire: &[],
        debrief: false,
    };
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
}

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
    vehicles: Query<(&Vehicle, &Transform)>,
    men: Query<(&Soldier, &Transform)>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(shot) = SHOTS.get(tour.shot) else {
        exit.write(AppExit::Success);
        return;
    };
    let t = script.t;
    let si = tour.shot;
    // Fire missions for this shot, called at their own times (possibly before the shot starts).
    for (k, &(at, kind)) in shot.fire.iter().enumerate() {
        if t >= at && !tour.fired.contains(&(si, k)) {
            tour.fired.push((si, k));
            kind.on_nearest_stalled(&mut fc, &vehicles, view.drone_pos.xz(), time.elapsed_secs_f64());
        }
    }
    if t < shot.from {
        // Skip ahead, unfilmed. Slow to the filming step just before the shot so it starts on time.
        let step = if shot.from - t > 1.0 { STEP_SKIP } else { STEP_FILM };
        *strategy = TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(step));
        tour.aim = None;
        place_camera(&mut view, shot, 0.0, &mut tour.aim, &op, &vehicles, &men);
        return;
    }
    *strategy = TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(STEP_FILM));
    let total = (shot.secs as f64 / STEP_FILM).round() as u32;
    let k = tour.shot_frames as f32 / total as f32;
    if shot.debrief {
        next_phase.set(crate::Phase::Debrief);
    }
    place_camera(&mut view, shot, k, &mut tour.aim, &op, &vehicles, &men);
    if tour.shot_frames == 0 {
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
    }
}

fn place_camera(view: &mut View, shot: &Shot, k: f32, aim: &mut Option<Vec3>, op: &ObservationPost, vehicles: &Query<(&Vehicle, &Transform)>, men: &Query<(&Soldier, &Transform)>) {
    let ease = k * k * (3.0 - 2.0 * k);
    let fov = (shot.fov.0 + (shot.fov.1 - shot.fov.0) * ease).to_radians();
    let target = match shot.aim {
        Aim::Op(p) => {
            view.mode = Mode::Op;
            view.op_fov = fov;
            (view.op_yaw, view.op_pitch) = look_angles(crate::geo::ground(p) - op.eye);
            return;
        }
        Aim::Drone(target) => target,
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

pub fn append(path: &str, line: &str) {
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}
