//! `BAV_SHOWCASE=<dir>`: a directed tour of the day for a showcase video. Headless only.
//!
//! The game runs on a fixed step (1/30 s of game time per rendered frame while filming, 1/20 s while skipping
//! ahead), so frames and game time stay locked however slowly frames render. Each filmed frame is saved to
//! `<dir>/f00000.png`…, `<dir>/segments.txt` maps game time to video time, and the audio module logs every sound
//! it plays to `<dir>/audio.log`, so the xtask (`xtask/src/mix.rs`) can rebuild the soundtrack offline.
//!
//! The shots are the day's shots file's (`dioramas/shots/<id>.ron`): its minute tour, or with `BAV_TOUR=film` its
//! part of the showcase film (design/showcase.md), shots with cards before them, the interface switched per shot,
//! ground shots from an ATGM team, and shots flown by the stick script.

use crate::camera::{Flight, Locate, Mode, Target, View, enter_ground, look_angles, track_nearest};
use crate::fire::{AtgmTeam, Post, Strike, StrikeMission};
use bav_sim::air::Aircraft;
use bav_sim::player::{Fpv, TriggerPull};
use crate::units::{Soldier, Vehicle};
use crate::world::ObservationPost;
use bevy::{prelude::*, render::view::screenshot::{Screenshot, save_to_disk}, time::TimeUpdateStrategy};
use serde::Deserialize;
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
            if is_reel() {
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
    if is_reel() { STEP_FILM } else { STEP_SKIP }
}

fn is_reel() -> bool {
    crate::dev::dev().tour == Some(crate::dev::TourKind::Reel)
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
    Op(Vec2),
    /// Through the FPV's goggles if it is up (the reel launches it on its own schedule); the view is left to it.
    Goggles,
    /// The drone, following the aircraft in the air nearest the point (x, z), or the point when there is none.
    Air(Vec2),
    /// The drone, following the one of a kind of Myrnohrad's things nearest the point (x, z), or the point when
    /// there is none.
    Near(Thing, Vec2),
    /// The Vampire's camera under its belly (key 8); the drone's while it is not up.
    Bomber,
}

/// What a Myrnohrad shot follows (design/myrnohrad.md, phase 10): a robot (on its run or lost where it stands), a car
/// of ours, one of their wings, the Vampire, a burrow (a found one before any other).
#[derive(Clone, Copy, Deserialize)]
enum Thing {
    Robot,
    Car,
    Wing,
    Vampire,
    Burrow,
}

/// Where the things a [`Thing`] names are, for the shots that follow them.
#[derive(bevy::ecs::system::SystemParam)]
struct Things<'w, 's> {
    robots: Query<'w, 's, &'static Transform, With<bav_sim::robots::Robot>>,
    cars: Query<'w, 's, &'static Transform, With<bav_sim::killzone::Car>>,
    wings: Query<'w, 's, &'static Transform, With<bav_sim::wings::Wing>>,
    vampires: Query<'w, 's, &'static bav_sim::robots::Vampire>,
    burrows: Query<'w, 's, (&'static bav_sim::seep::Burrow, Has<bav_sim::seep::Found>)>,
}

impl Things<'_, '_> {
    /// The nearest of a kind to `p`, a burrow found before one not.
    fn near(&self, thing: Thing, p: Vec2) -> Option<Vec3> {
        let by = |a: &Vec3, b: &Vec3| a.xz().distance(p).total_cmp(&b.xz().distance(p));
        match thing {
            Thing::Robot => self.robots.iter().map(|t| t.translation).min_by(by),
            Thing::Car => self.cars.iter().map(|t| t.translation).min_by(by),
            Thing::Wing => self.wings.iter().map(|t| t.translation).min_by(by),
            Thing::Vampire => self.vampires.iter().map(|v| v.pos).min_by(by),
            Thing::Burrow => self.burrows.iter().min_by(|a, b| b.1.cmp(&a.1).then(a.0.at.distance(p).total_cmp(&b.0.at.distance(p)))).map(|(b, _)| crate::geo::ground(b.at)),
        }
    }
}

/// A card the script renders over black before the shot: a title, a smaller line, a dim note.
#[derive(Deserialize)]
struct Card {
    title: String,
    line: String,
    note: String,
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
    fire: Vec<(f32, Strike)>,
    debrief: bool,
    /// A card over black before the shot; its frames are reserved, its time goes in the segment's front.
    card: Option<Card>,
    /// The interface in shot: the atmosphere shots go without.
    hud: bool,
    /// Stick-script legs (seconds into the shot, what the sticks do) for a flown shot.
    fly: Vec<(f32, f32, String)>,
    /// A ground shot fires its missile this many seconds in.
    launch: Option<f32>,
    /// The drone on its thermal channel for the shot (a night's shots).
    thermal: bool,
}

fn shot(from: f32, secs: f32, p0: (f32, f32, f32), p1: (f32, f32, f32), aim: Aim, fov: (f32, f32)) -> Shot {
    Shot {
        from,
        secs,
        pos0: Vec3::new(p0.0, p0.1, p0.2),
        pos1: Vec3::new(p1.0, p1.1, p1.2),
        aim,
        fov,
        fire: Vec::new(),
        debrief: false,
        card: None,
        hud: true,
        fly: Vec::new(),
        launch: None,
        thermal: false,
    }
}

/// A day's tours as its file has them (`dioramas/shots/<id>.ron`): the minute tour and the film's part (none: no
/// film).
#[derive(Deserialize)]
struct ShotsFile {
    minute: Vec<ShotSpec>,
    #[serde(default)]
    film: Vec<ShotSpec>,
}

/// A shot as the file writes it ([`Shot`]'s fields; `pos` is the drone's (x, z, height above the ground) at the
/// shot's start and end).
#[derive(Deserialize)]
struct ShotSpec {
    from: f32,
    secs: f32,
    pos: ((f32, f32, f32), (f32, f32, f32)),
    aim: AimSpec,
    fov: (f32, f32),
    #[serde(default)]
    fire: Vec<(f32, Strike)>,
    #[serde(default)]
    debrief: bool,
    #[serde(default)]
    card: Option<Card>,
    #[serde(default = "yes")]
    hud: bool,
    #[serde(default)]
    fly: Vec<(f32, f32, String)>,
    #[serde(default)]
    launch: Option<f32>,
    #[serde(default)]
    thermal: bool,
}

fn yes() -> bool {
    true
}

/// [`Aim`] as the file writes it, points as (x, z).
#[derive(Deserialize)]
enum AimSpec {
    Drone(TargetSpec),
    Ground((f32, f32), bool),
    Fly(TargetSpec),
    Op((f32, f32)),
    Goggles,
    Air((f32, f32)),
    Near(Thing, (f32, f32)),
    Bomber,
}

/// [`Target`] as the file writes it.
#[derive(Deserialize)]
enum TargetSpec {
    Point((f32, f32)),
    Vehicle(u32),
    Wave(u32, u32),
    Men,
    MenNear((f32, f32)),
}

impl From<TargetSpec> for Target {
    fn from(t: TargetSpec) -> Target {
        match t {
            TargetSpec::Point(p) => Target::Point(p.into()),
            TargetSpec::Vehicle(i) => Target::Vehicle(i),
            TargetSpec::Wave(w, i) => Target::Wave(w, i),
            TargetSpec::Men => Target::Men,
            TargetSpec::MenNear(p) => Target::MenNear(p.into()),
        }
    }
}

impl From<ShotSpec> for Shot {
    fn from(s: ShotSpec) -> Shot {
        let aim = match s.aim {
            AimSpec::Drone(t) => Aim::Drone(t.into()),
            AimSpec::Ground(p, sight) => Aim::Ground(p.into(), sight),
            AimSpec::Fly(t) => Aim::Fly(t.into()),
            AimSpec::Op(p) => Aim::Op(p.into()),
            AimSpec::Goggles => Aim::Goggles,
            AimSpec::Air(p) => Aim::Air(p.into()),
            AimSpec::Near(thing, p) => Aim::Near(thing, p.into()),
            AimSpec::Bomber => Aim::Bomber,
        };
        Shot { fire: s.fire, debrief: s.debrief, card: s.card, hud: s.hud, fly: s.fly, launch: s.launch, thermal: s.thermal, ..shot(s.from, s.secs, s.pos.0, s.pos.1, aim, s.fov) }
    }
}

/// The tour, shot by shot: the day's minute tour, or its part of the film (`BAV_TOUR=film`), from its shots file; or
/// the reel's.
static SHOTS: LazyLock<Vec<Shot>> = LazyLock::new(|| {
    let film = match crate::dev::dev().tour {
        Some(crate::dev::TourKind::Reel) => return reel::load(),
        Some(crate::dev::TourKind::Film) => true,
        None => false,
    };
    let (name, text) = bav_core::data::shots_source().unwrap_or_else(|| panic!("{} has no shots file to film", crate::diorama::diorama()));
    let file: ShotsFile = bav_core::data::parse(text).unwrap_or_else(|e| panic!("{name}: {e}"));
    // A day with no film films nothing rather than another day's.
    let shots = if film { file.film } else { file.minute };
    shots.into_iter().map(Shot::from).collect()
});
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
    mut flight: ResMut<Flight>,
    mut hud: ResMut<crate::hud::HudHidden>,
    (vehicles, men, aircraft): (Query<(&Vehicle, &Transform)>, Query<(&Soldier, &Transform)>, Query<(&Transform, &Aircraft)>),
    (fpv, things): (Res<Fpv>, Things),
    teams: Query<(Entity, &Post, &AtgmTeam)>,
    mut exit: MessageWriter<AppExit>,
    mut pulls: MessageWriter<TriggerPull>,
) {
    let t = script.t;
    let Some(shot) = SHOTS.get(tour.shot) else {
        exit.write(AppExit::Success);
        return;
    };
    let si = tour.shot;
    hud.set_if_neq(crate::hud::HudHidden(!shot.hud));
    // A shot on the thermal channel puts the drone on it, and the shot after takes it off again (`BAV_THERMAL` keeps it
    // on, `thermal::toggle`).
    if shot.thermal {
        view.thermal = true;
    } else if si > 0 && SHOTS[si - 1].thermal {
        view.thermal = false;
    }
    // Fire missions for this shot, called at their own times (possibly before the shot starts).
    for (k, &(at, kind)) in shot.fire.iter().enumerate() {
        if t >= at && !tour.fired.contains(&(si, k)) {
            tour.fired.push((si, k));
            let at = kind.on_nearest_stalled(&mut commands, &vehicles, view.drone_pos.xz(), time.elapsed_secs_f64());
            info!("showcase: {kind:?} called on {at:?}, at game {t:.0} s");
        }
    }
    // A ground shot enters its team's tree line on the shot's first frame, and during the skip, so the camera
    // is already there when filming starts.
    if let Aim::Ground(near, sight) = shot.aim
        && tour.ground != Some(si)
        && let Some((e, p, _)) = teams.iter().filter(|(_, _, t)| t.atgm > 0).min_by(|a, b| a.1.pos.distance(near).total_cmp(&b.1.pos.distance(near)))
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
        place_camera(&mut view, shot, 0.0, &mut tour.aim, &op, &vehicles, &men, &aircraft, &things);
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
        place_camera(&mut view, shot, k, &mut tour.aim, &op, &vehicles, &men, &aircraft, &things);
    }
    // The debrief shot ends the day, once: re-setting the state every frame would re-fire `OnEnter` (and
    // `show_debrief`) every frame.
    if shot.debrief && tour.shot_frames == 0 {
        next_phase.set(crate::Phase::Debrief);
    }
    // The ground shot's launch: on the one filmed frame where the shot's clock crosses it.
    let step = STEP_FILM as f32;
    let clock = tour.shot_frames as f32 * step;
    if shot.launch.is_some_and(|at| clock >= at && clock - step < at) {
        pulls.write(TriggerPull);
    }
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

#[allow(clippy::too_many_arguments)]
fn place_camera(view: &mut View, shot: &Shot, k: f32, aim: &mut Option<Vec3>, op: &ObservationPost, vehicles: &Query<(&Vehicle, &Transform)>, men: &Query<(&Soldier, &Transform)>, aircraft: &Query<(&Transform, &Aircraft)>, things: &Things) {
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
        Aim::Air(p) => {
            view.mode = Mode::Drone;
            view.drone_fov = fov;
            let at = shot.pos0.lerp(shot.pos1, ease);
            view.drone_pos = Vec3::new(at.x, crate::geo::height(at.x, at.y) + at.z, at.y);
            let target = aircraft.iter().filter(|(_, a)| !matches!(a.flight, bav_sim::air::Flight::Waiting(_))).map(|(t, _)| t.translation).min_by(|a, b| a.xz().distance(p).total_cmp(&b.xz().distance(p))).unwrap_or(crate::geo::ground(p));
            let a = aim.get_or_insert(target);
            *a = a.lerp(target, 0.06);
            (view.drone_yaw, view.gimbal) = look_angles(*a - view.drone_pos);
            return;
        }
        // The Vampire's own camera, which its controls place; the drone waits where it is until the Vampire is up.
        Aim::Bomber => {
            if !things.vampires.is_empty() {
                view.mode = Mode::Bomber;
            }
            view.locked = false;
            return;
        }
        Aim::Near(thing, p) => {
            view.mode = Mode::Drone;
            view.drone_fov = fov;
            let at = shot.pos0.lerp(shot.pos1, ease);
            view.drone_pos = Vec3::new(at.x, crate::geo::height(at.x, at.y) + at.z, at.y);
            let target = things.near(thing, p).unwrap_or(crate::geo::ground(p));
            let a = aim.get_or_insert(target);
            *a = a.lerp(target, 0.06);
            (view.drone_yaw, view.gimbal) = look_angles(*a - view.drone_pos);
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
    flight.0 = shot.fly.iter().map(|(a, b, s)| (shot.from + a, shot.from + b, s.clone())).collect();
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
    mut file: Local<Option<std::io::BufWriter<std::fs::File>>>,
) {
    if file.is_none() {
        *file = std::fs::File::create(format!("{}/events.txt", tour.dir)).ok().map(std::io::BufWriter::new);
    }
    let Some(out) = file.as_mut() else { return };
    let t = script.t;
    for b in blasts.read() {
        let _ = writeln!(out, "{t:.2} blast {:?} {:.0} {:.0} {:.1}", b.cause, b.at.x, b.at.z, b.radius);
    }
    for h in log.entries.iter().filter(|h| h.at > *last) {
        let Ok(tf) = places.get(h.target) else { continue };
        let _ = writeln!(out, "{t:.2} hit {} {:.0} {:.0} {:?}", h.weapon.replace(' ', "_"), tf.translation.x, tf.translation.z, h.side);
    }
    *last = log.entries.iter().map(|h| h.at).fold(*last, f64::max);
    // A frame's lines at once: the run can end on any frame.
    let _ = out.flush();
}
