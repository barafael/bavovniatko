//! `VUHLEDAR_SHOWCASE=<dir>`: a directed tour of the day for a showcase video. Headless only.
//!
//! The game runs on a fixed step (1/30 s of game time per rendered frame while filming, 1/20 s while skipping
//! ahead), so frames and game time stay locked however slowly frames render. Each filmed frame is saved to
//! `<dir>/f00000.png`…, `<dir>/segments.txt` maps game time to video time, and the audio module logs every sound
//! it plays to `<dir>/audio.log`, so `scripts/showcase.py` can rebuild the soundtrack offline.

use crate::camera::{Mode, View};
use crate::units::{Side, SState, Soldier, Vehicle};
use crate::world::ObservationPost;
use bevy::{prelude::*, render::view::screenshot::{Screenshot, save_to_disk}, time::TimeUpdateStrategy};
use std::{io::Write, time::Duration};

pub struct ShowcasePlugin;

impl Plugin for ShowcasePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dir) = std::env::var("VUHLEDAR_SHOWCASE") {
            std::fs::create_dir_all(&dir).ok();
            app.insert_resource(Tour { dir, shot: 0, frame: 0, video_t: 0.0, aim: None, fired: Vec::new(), shot_frames: 0 })
                .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(STEP_SKIP)))
                .add_systems(Update, direct);
        }
    }
}

const STEP_FILM: f64 = 1.0 / 30.0;
const STEP_SKIP: f64 = 1.0 / 20.0;

#[derive(Clone, Copy)]
enum Aim {
    Point(Vec2),
    Vehicle(u32),
    Men,
    /// The OP looking at a point (x, z) on the ground.
    OpAt(Vec2),
}

struct Shot {
    from: f32,
    secs: f32,
    pos0: Vec3,
    pos1: Vec3,
    aim: Aim,
    fov: (f32, f32),
    /// Fire missions to call (game time, kind) before or during the shot, on the nearest stalled vehicle.
    fire: &'static [(f32, &'static str)],
    debrief: bool,
}

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
    vec![
        // Dawn over the January wrecks on the dacha road, before the first column.
        s(4.0, 6.0, (1150.0, 200.0, 70.0), (1220.0, 300.0, 60.0), Aim::Point(Vec2::new(1440.0, 530.0)), (26.0, 20.0)),
        // The 155th's column in single file behind its smoke screen.
        s(22.0, 8.0, (1000.0, -250.0, 170.0), (1000.0, -250.0, 170.0), Aim::Vehicle(3), (16.0, 11.0)),
        // From behind the tree line: the ATGM teams open up.
        s(68.0, 9.0, (1500.0, -300.0, 90.0), (1490.0, -320.0, 90.0), Aim::Vehicle(0), (22.0, 16.0)),
        // The kill zone: mines, wrecks, crews in the snow, a fire mission landing among them.
        Shot { fire: &[(80.0, "he"), (90.0, "he")], ..s(97.0, 10.0, (750.0, -500.0, 120.0), (760.0, -480.0, 120.0), Aim::Vehicle(2), (5.0, 4.2)) },
        // The TOS-1A salvo landing on the town's southern blocks.
        s(164.0, 8.0, (250.0, -760.0, 140.0), (230.0, -780.0, 140.0), Aim::Point(Vec2::new(-290.0, -1150.0)), (34.0, 30.0)),
        // Afternoon: infantry on foot across the field, the battalion mortars working over them.
        s(1186.0, 7.0, (-640.0, -760.0, 90.0), (-630.0, -740.0, 90.0), Aim::Men, (7.0, 5.5)),
        // Dusk over the dacha road: everything that came down it today is still there.
        s(1470.0, 4.0, (950.0, -820.0, 200.0), (980.0, -800.0, 200.0), Aim::Point(Vec2::new(1130.0, -80.0)), (34.0, 30.0)),
        // The debrief.
        Shot { debrief: true, ..s(1474.0, 4.0, (980.0, -800.0, 200.0), (980.0, -800.0, 200.0), Aim::Point(Vec2::new(1130.0, -80.0)), (30.0, 30.0)) },
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

#[allow(clippy::too_many_arguments)]
fn direct(
    mut commands: Commands,
    mut tour: ResMut<Tour>,
    mut strategy: ResMut<TimeUpdateStrategy>,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    target: Option<Res<crate::camera::HeadlessTarget>>,
    op: Option<Res<ObservationPost>>,
    mut view: ResMut<View>,
    mut next_phase: ResMut<NextState<crate::Phase>>,
    mut fc: ResMut<crate::fire::FireControl>,
    vehicles: Query<(&Vehicle, &Transform)>,
    men: Query<(&Soldier, &Transform)>,
    mut exit: MessageWriter<AppExit>,
) {
    let list = shots();
    let Some(shot) = list.get(tour.shot) else {
        exit.write(AppExit::Success);
        return;
    };
    let t = script.t;
    let si = tour.shot;
    // Fire missions for this shot, called at their own times (possibly before the shot starts).
    for (k, (at, kind)) in shot.fire.iter().enumerate() {
        if t >= *at && !tour.fired.contains(&(si, k)) {
            tour.fired.push((si, k));
            let me = Vec2::new(view.drone_pos.x, view.drone_pos.z);
            if let Some((_, tf)) = vehicles
                .iter()
                .filter(|(v, _)| v.state.alive() && !matches!(v.state, crate::units::VState::Advancing))
                .min_by(|a, b| Vec2::new(a.1.translation.x, a.1.translation.z).distance(me).total_cmp(&Vec2::new(b.1.translation.x, b.1.translation.z).distance(me)))
            {
                let target = Vec2::new(tf.translation.x, tf.translation.z);
                let now = time.elapsed_secs_f64();
                let kind_ = if *kind == "raam" { crate::fire::ShellKind::Raam } else { crate::fire::ShellKind::He155 };
                fc.missions.push(crate::fire::Mission { target, kind: kind_, rounds: 4, sigma: 12.0, interval: 3.0, next: now + 1.0, flight: 24.0, player: true });
            }
        }
    }
    if t < shot.from {
        // Skip ahead, unfilmed. Slow to the filming step just before the shot so it starts on time.
        let step = if shot.from - t > 1.0 { STEP_SKIP } else { STEP_FILM };
        *strategy = TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(step));
        tour.aim = None;
        place_camera(&mut view, shot, 0.0, &mut tour.aim, op.as_deref(), &vehicles, &men);
        return;
    }
    *strategy = TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(STEP_FILM));
    let total = (shot.secs as f64 / STEP_FILM).round() as u32;
    let k = tour.shot_frames as f32 / total as f32;
    if shot.debrief {
        next_phase.set(crate::Phase::Debrief);
    }
    place_camera(&mut view, shot, k, &mut tour.aim, op.as_deref(), &vehicles, &men);
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

fn place_camera(view: &mut View, shot: &Shot, k: f32, aim: &mut Option<Vec3>, op: Option<&ObservationPost>, vehicles: &Query<(&Vehicle, &Transform)>, men: &Query<(&Soldier, &Transform)>) {
    let ease = k * k * (3.0 - 2.0 * k);
    let fov = (shot.fov.0 + (shot.fov.1 - shot.fov.0) * ease).to_radians();
    if let Aim::OpAt(p) = shot.aim {
        view.mode = Mode::Op;
        view.op_fov = fov;
        if let Some(op) = op {
            let d = crate::geo::ground(p) - op.eye;
            view.op_yaw = f32::atan2(-d.x, -d.z);
            view.op_pitch = f32::atan2(d.y, Vec2::new(d.x, d.z).length());
        }
        return;
    }
    view.mode = Mode::Drone;
    view.drone_fov = fov;
    let p = shot.pos0.lerp(shot.pos1, ease);
    view.drone_pos = Vec3::new(p.x, crate::geo::height(p.x, p.y) + p.z, p.y);
    let target = match shot.aim {
        Aim::Point(q) => Some(crate::geo::ground(q)),
        Aim::Vehicle(idx) => vehicles.iter().filter(|(v, _)| v.idx == idx && v.side == Side::Ru).min_by_key(|(v, _)| v.wave).map(|(_, t)| t.translation),
        Aim::Men => {
            let pts: Vec<Vec3> = men
                .iter()
                .filter(|(s, t)| s.side == Side::Ru && s.state != SState::Flee && t.translation.distance(view.drone_pos) < 1400.0)
                .map(|(_, t)| t.translation)
                .collect();
            (!pts.is_empty()).then(|| pts.iter().copied().sum::<Vec3>() / pts.len() as f32)
        }
        Aim::OpAt(_) => None,
    };
    let Some(target) = target else { return };
    let a = aim.get_or_insert(target);
    *a = a.lerp(target, 0.06);
    let d = *a - view.drone_pos;
    view.drone_yaw = f32::atan2(-d.x, -d.z);
    view.gimbal = f32::atan2(d.y, Vec2::new(d.x, d.z).length());
}

pub fn append(path: &str, line: &str) {
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}
