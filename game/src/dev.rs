//! Development switches, read from `VUHLEDAR_*` environment variables once at startup (see README.md,
//! "Development"), and the helpers they drive. F12 saves a screenshot.

use crate::camera::{HeadlessTarget, Mode, Target, View, look_angles};
use crate::fire::{Atgm, FireControl, Strike, Post};
use crate::units::{Radio, Soldier, Stats, VState, Vehicle};
use bevy::{diagnostic::DiagnosticsStore, prelude::*, render::view::screenshot::{Screenshot, save_to_disk}};
use std::{str::FromStr, sync::OnceLock};

/// `VUHLEDAR_DAYDBG`: daylight debugging.
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum DayDebug {
    /// Leave the light as it was at startup.
    Off,
    /// Animate the light but leave the sun where it is.
    NoSun,
}

/// `VUHLEDAR_OFF`: parts of the picture to leave out, to measure what each costs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Part {
    /// The sun's shadow cascades.
    Shadows,
    Msaa,
    /// The video-feed post-process.
    Feed,
    Trees,
    /// The ground chunks.
    Terrain,
}

/// Whether `VUHLEDAR_OFF` leaves this part out.
pub fn off(part: Part) -> bool {
    dev().off.contains(&part)
}

#[derive(Debug, Default)]
pub struct DevEnv {
    /// No window, no sound; render offscreen on the GPU.
    pub headless: bool,
    /// Offscreen render size.
    pub res: Option<(u32, u32)>,
    pub skip_briefing: bool,
    /// Film the showcase tour into this directory.
    pub showcase: Option<String>,
    pub no_battery: bool,
    /// Start in this view: `op` or `ground`.
    pub view: Option<Mode>,
    /// Ground view: which kind of ATGM team to join.
    pub team: Option<Atgm>,
    /// Ground view: launch the player's missile at this game time.
    pub auto_atgm: Option<f64>,
    pub daydbg: Option<DayDebug>,
    pub shot: Option<String>,
    pub shot_at: f32,
    /// Drone placement: x, z, height above ground, yaw, pitch, fov (degrees).
    pub cam: Option<[f32; 6]>,
    /// Keep the drone's gimbal on a vehicle (column index) or `men`.
    pub look: Option<Target>,
    pub speed: Option<f32>,
    pub warp_to: Option<f32>,
    /// `VUHLEDAR_SEED=<n>`: run the day on other random streams (0, or unset, is the day itself).
    pub seed: u64,
    /// Go straight to the debrief once the warp (if any) arrives, to lay the screen out without playing the day.
    pub debrief: bool,
    pub rec: Option<String>,
    pub rec_from: f32,
    pub rec_secs: f32,
    /// Scripted fire missions: game time and strike.
    pub fire: Vec<(f32, Strike)>,
    /// Scripted drone flight: legs of game time with what the sticks do (see `camera::Stick::scripted`).
    pub fly: Vec<(f32, f32, String)>,
    /// Log frame times and the GPU time of every render pass; headless, also lift the frame cap.
    pub diag: bool,
    pub off: Vec<Part>,
    /// A directory of ground chunk PNGs: read at start if complete, written after painting if not.
    pub ground: Option<String>,
    pub log_stats: bool,
    /// Park every vehicle model in a row here (x, z, heading in degrees), live and burned out, to look them over.
    pub lineup: Option<[f32; 3]>,
    /// `VUHLEDAR_ERA_AT=<game s>`: set off the reactive armour on every lineup hull that has it, once.
    pub era_at: Option<f32>,
    /// Light tuning without a rebuild: multipliers for the sun and the ambient light, and an exposure offset.
    pub light: [f32; 3],
/// `VUHLEDAR_SKY`: the February weather, `overcast` (the date's) or `sun` (the footage's).
pub sky: Option<crate::sky::Weather>,
    /// `VUHLEDAR_TOUR`: which shot list the showcase films, `film` (design/showcase.md) or the minute tours.
    pub tour: Option<TourKind>,
}

/// `VUHLEDAR_TOUR`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum TourKind {
    /// The showcase film's tour, one per diorama.
    Film,
}

fn var(k: &str) -> Option<String> {
    std::env::var(k).ok().filter(|s| !s.is_empty())
}

fn parse<T: FromStr>(k: &str) -> Option<T> {
    var(k).and_then(|s| s.trim().parse().ok())
}

impl DevEnv {
    fn load() -> Self {
        let cam = var("VUHLEDAR_CAM").and_then(|c| {
            let v: Vec<f32> = c.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            <[f32; 6]>::try_from(v).ok()
        });
        let fire = var("VUHLEDAR_FIRE")
            .map(|plan| {
                plan.split(',')
                    .filter_map(|item| {
                        let (t, k) = item.split_once(':')?;
                        Some((t.trim().parse().ok()?, k.trim().parse().ok()?))
                    })
                    .collect()
            })
            .unwrap_or_default();
        // `VUHLEDAR_FLY="3-9:w+,9-12:q,12-20:nz"`: from 3 to 9 s fly forward in sport mode, then turn left, then tilt
        // the gimbal down while zooming in.
        let fly = var("VUHLEDAR_FLY")
            .map(|plan| {
                plan.split(',')
                    .filter_map(|leg| {
                        let (span, spec) = leg.split_once(':')?;
                        let (from, to) = span.split_once('-')?;
                        Some((from.trim().parse().ok()?, to.trim().parse().ok()?, spec.trim().to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();
        Self {
            headless: var("VUHLEDAR_HEADLESS").is_some(),
            res: var("VUHLEDAR_RES").and_then(|r| {
                let (w, h) = r.split_once('x')?;
                Some((w.parse().ok()?, h.parse().ok()?))
            }),
            skip_briefing: var("VUHLEDAR_SKIP_BRIEFING").is_some(),
            showcase: var("VUHLEDAR_SHOWCASE"),
            no_battery: var("VUHLEDAR_NO_BATTERY").is_some(),
            view: parse("VUHLEDAR_VIEW"),
            team: parse("VUHLEDAR_TEAM"),
            auto_atgm: parse("VUHLEDAR_AUTO_ATGM"),
            daydbg: parse("VUHLEDAR_DAYDBG"),
            sky: parse("VUHLEDAR_SKY"),
            shot: var("VUHLEDAR_SHOT"),
            shot_at: parse("VUHLEDAR_SHOT_AT").unwrap_or(6.0),
            cam,
            look: parse("VUHLEDAR_LOOK"),
            speed: parse("VUHLEDAR_SPEED"),
            warp_to: parse("VUHLEDAR_WARP_TO"),
            seed: parse("VUHLEDAR_SEED").unwrap_or(0),
            debrief: var("VUHLEDAR_DEBRIEF").is_some(),
            rec: var("VUHLEDAR_REC"),
            rec_from: parse("VUHLEDAR_REC_FROM").unwrap_or(5.0),
            rec_secs: parse("VUHLEDAR_REC_SECS").unwrap_or(10.0),
            fire,
            fly,
            diag: var("VUHLEDAR_DIAG").is_some(),
            off: var("VUHLEDAR_OFF").map(|s| s.split(',').filter_map(|p| p.trim().parse().ok()).collect()).unwrap_or_default(),
            ground: var("VUHLEDAR_GROUND"),
            log_stats: var("VUHLEDAR_LOG_STATS").is_some(),
            era_at: parse("VUHLEDAR_ERA_AT"),
            lineup: var("VUHLEDAR_LINEUP").and_then(|c| <[f32; 3]>::try_from(c.split(',').filter_map(|s| s.trim().parse().ok()).collect::<Vec<f32>>()).ok()),
            light: var("VUHLEDAR_LIGHT")
                .and_then(|c| <[f32; 3]>::try_from(c.split(',').filter_map(|s| s.trim().parse().ok()).collect::<Vec<f32>>()).ok())
                .unwrap_or([1.0, 1.0, 0.0]),
            tour: parse("VUHLEDAR_TOUR"),
        }
    }
}

static DEV: OnceLock<DevEnv> = OnceLock::new();

/// The development switches, parsed on first use.
/// A simulation stream's seed under `VUHLEDAR_SEED`: the stream's own seed for the day itself, mixed with the
/// run number otherwise, so `scripts/band.sh` can see how far the day's outcome swings with chance alone.
pub fn seeded(base: u64) -> u64 {
    match dev().seed {
        0 => base,
        s => base ^ s.wrapping_mul(0x9E37_79B9_7F4A_7C15),
    }
}

pub fn dev() -> &'static DevEnv {
    DEV.get_or_init(DevEnv::load)
}

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(crate::camera::Flight(dev().fly.clone()))
            .add_systems(Startup, lineup.after(crate::units::setup_models));
        app.add_systems(Update, (auto_shot, key_shot, env_view, env_look, record, log_stats, warp, env_debrief, scripted_fire, frame_times, env_ground, era_test).chain().in_set(crate::Step::Control).in_set(DevSystems));
    }
}

#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DevSystems;

/// `VUHLEDAR_LINEUP="x,z,heading"`: every vehicle model side by side, 10 m apart, and behind them the same burned
/// out, turrets turned a little so the split shows.
fn lineup(mut commands: Commands, models: Res<crate::units::Models>, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    use crate::units::{VKind, VehicleMeshes, burnt, dress, dress_charred, skin};
    use strum::VariantArray;
    let Some([x, z, heading]) = dev().lineup else { return };
    let rot = Quat::from_rotation_y(heading.to_radians());
    let across = rot * Vec3::X;
    let back = rot * Vec3::Z;
    let b = crate::baked::load("ka52", Color::WHITE, &mut meshes, &mut images, &mut materials);
    let ka52 = VehicleMeshes { hull: b.hull, plow_hull: None, turret: b.turret, turret_at: b.turret_at, muzzle: b.muzzle };
    let all: Vec<&VehicleMeshes> = VKind::VARIANTS.iter().map(|k| models.vehicle(*k)).chain([&models.ua_tank, &ka52]).collect();
    for (i, m) in all.into_iter().enumerate() {
        for row in 0..2 {
            let at = Vec3::new(x, 0.0, z) + across * (i as f32 * 10.0) + back * (row as f32 * 14.0);
            let at = crate::geo::ground(at.xz());
            let at = Transform::from_translation(at).with_rotation(rot);
            let turret = Transform::from_translation(m.turret_at).with_rotation(Quat::from_rotation_y(row as f32 * 0.5));
            if row == 0 {
                let hull = m.plow_hull.as_ref().unwrap_or(&m.hull);
                let mut e = commands.spawn((skin(hull), at));
                dress(&mut e, hull);
                e.with_children(|c| dress(&mut c.spawn((skin(&m.turret), turret)), &m.turret));
            } else {
                let mut e = commands.spawn((burnt(&m.hull), at));
                dress_charred(&mut e, &m.hull);
                e.with_children(|c| dress_charred(&mut c.spawn((burnt(&m.turret), turret)), &m.turret));
            }
        }
    }
}

/// `VUHLEDAR_ERA_AT` with `VUHLEDAR_LINEUP`: the reactive armour of the lineup's T-80, T-72 and Bradley going off
/// at a chosen moment, to look at the pops on their own (in the day they come with the warhead's blast).
fn era_test(mut commands: Commands, script: Res<crate::director::Script>, mut fx: ResMut<crate::fx::FxAssets>, mut done: Local<bool>) {
    use crate::units::VKind;
    use strum::VariantArray;
    let (Some(at), Some([x, z, heading])) = (dev().era_at, dev().lineup) else { return };
    if *done || script.t < at {
        return;
    }
    *done = true;
    let rot = Quat::from_rotation_y(heading.to_radians());
    for (i, k) in VKind::VARIANTS.iter().enumerate() {
        if k.era() {
            let p = crate::geo::ground((Vec3::new(x, 0.0, z) + rot * Vec3::X * (i as f32 * 10.0)).xz());
            crate::fx::era_pops(&mut commands, &mut fx, &Transform::from_translation(p).with_rotation(rot), false);
        }
    }
}

fn shot_of(target: Option<&HeadlessTarget>) -> Screenshot {
    match target {
        Some(t) => Screenshot::image(t.0.clone()),
        None => Screenshot::primary_window(),
    }
}

/// `VUHLEDAR_SHOT=<path>`: save a screenshot after `VUHLEDAR_SHOT_AT` seconds (default 6) and exit. With
/// `VUHLEDAR_WARP_TO`, the seconds count from when the warp arrives, so a shot lands at a chosen game time.
fn auto_shot(mut commands: Commands, time: Res<Time<Real>>, script: Res<crate::director::Script>, target: Option<Res<HeadlessTarget>>, mut done: Local<u8>, mut from: Local<Option<f32>>, mut exit: MessageWriter<AppExit>) {
    let Some(path) = dev().shot.clone() else { return };
    let t = time.elapsed_secs();
    if from.is_none() && dev().warp_to.is_none_or(|w| script.t >= w) {
        *from = Some(t);
    }
    let Some(t0) = *from else { return };
    if *done == 0 && t - t0 > dev().shot_at {
        commands.spawn(shot_of(target.as_deref())).observe(save_to_disk(path));
        *done = 1;
    } else if *done == 1 && t - t0 > dev().shot_at + 1.5 {
        exit.write(AppExit::Success);
        *done = 2;
    }
}

/// `VUHLEDAR_CAM="x,z,agl,yaw,pitch,fov"` places the drone; `VUHLEDAR_VIEW=op` starts on the roof;
/// `VUHLEDAR_SPEED` scales game time.
fn env_view(mut view: ResMut<View>, mut vtime: ResMut<Time<Virtual>>, mut done: Local<bool>) {
    if *done {
        return;
    }
    *done = true;
    if let Some(s) = dev().speed {
        vtime.set_relative_speed(s);
    }
    if dev().view == Some(Mode::Op) {
        view.mode = Mode::Op;
    }
    if let Some(v) = dev().cam {
        view.drone_pos = Vec3::new(v[0], crate::geo::height(v[0], v[1]) + v[2], v[1]);
        set_drone_aim(&mut view, v[3].to_radians(), v[4].to_radians());
        view.drone_fov = v[5].max(1.0).to_radians();
        view.fov_target = view.drone_fov;
    }
}

/// Point the drone and make that its slew target too, so its flight controller does not fight back.
fn set_drone_aim(view: &mut View, yaw: f32, gimbal: f32) {
    view.drone_yaw = yaw;
    view.gimbal = gimbal;
    view.yaw_target = yaw;
    view.gimbal_target = gimbal;
}

/// `VUHLEDAR_LOOK=<idx>|men`: keep the drone's gimbal on that [`Target`].
/// What `VUHLEDAR_LOOK=hit|pool|grass` aims at: the last hit, the pool fires, the grass fires, the missiles in the air.
type FireLooks<'w, 's> = (
    Res<'w, crate::units::LastHit>,
    Query<'w, 's, (&'static crate::fx::Emitter, &'static GlobalTransform)>,
    Query<'w, 's, &'static crate::fx::GrassFire>,
    Query<'w, 's, &'static crate::fire::Missile>,
);

fn env_look(
    mut view: ResMut<View>,
    vehicles: Query<(&Vehicle, &Transform)>,
    men: Query<(&Soldier, &Transform)>,
    fires: FireLooks,
    mut aim: Local<Option<Vec3>>,
) {
    let (hit, pools, grass, missiles) = fires;
    let located = match dev().look {
        // The hull the newest missile in the air is flying at, so the shot is on it when it strikes; else the last hit.
        Some(crate::camera::Target::Hit) => missiles.iter().filter_map(|m| vehicles.get(m.target).ok()).last().map(|(_, t)| t.translation).or(hit.0),
        Some(crate::camera::Target::Pool) => pools.iter().filter(|(e, _)| e.kind == crate::fx::EmitterKind::PoolFire).last().map(|(_, t)| t.translation()),
        Some(crate::camera::Target::Grass) => grass.iter().filter_map(|g| g.centre()).last().map(crate::geo::ground),
        Some(t) => t.locate(view.drone_pos, &vehicles, &men),
        None => None,
    };
    let Some(target) = located else { return };
    // Ease the aim so the gimbal pans smoothly.
    let a = aim.get_or_insert(target);
    *a = a.lerp(target, 0.03);
    let (yaw, pitch) = look_angles(*a - view.drone_pos);
    set_drone_aim(&mut view, yaw, pitch);
}

/// `VUHLEDAR_WARP_TO=<game s>`: run at 3x (the most the simulation's step cap allows at 60 fps) until then.
fn warp(script: Res<crate::director::Script>, mut vtime: ResMut<Time<Virtual>>, mut done: Local<bool>) {
    let Some(to) = dev().warp_to else { return };
    if *done {
        return;
    }
    if script.t < to {
        vtime.set_relative_speed(3.0);
    } else {
        vtime.set_relative_speed(1.0);
        *done = true;
    }
}

/// `VUHLEDAR_DEBRIEF=1`: end the day as soon as the warp arrives (or at once), so the debrief can be looked at.
fn env_debrief(script: Res<crate::director::Script>, phase: Res<State<crate::Phase>>, mut next: ResMut<NextState<crate::Phase>>) {
    if dev().debrief && *phase.get() == crate::Phase::Playing && script.t >= dev().warp_to.unwrap_or(0.0) {
        next.set(crate::Phase::Debrief);
    }
}

/// `VUHLEDAR_REC=<dir>`: from game time `VUHLEDAR_REC_FROM` save `VUHLEDAR_REC_SECS` seconds of frames at 15 fps
/// as `<dir>/f00000.png`…, then exit. `scripts/record.sh` encodes them.
fn record(
    mut commands: Commands,
    time: Res<Time<Real>>,
    script: Res<crate::director::Script>,
    target: Option<Res<HeadlessTarget>>,
    mut n: Local<u32>,
    mut next: Local<f32>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(dir) = dev().rec.as_deref() else { return };
    if script.t < dev().rec_from {
        return;
    }
    let t = time.elapsed_secs();
    if *n == 0 && *next == 0.0 {
        *next = t;
    }
    if t < *next {
        return;
    }
    if *n >= (dev().rec_secs * 15.0) as u32 {
        if t > *next + 2.0 {
            exit.write(AppExit::Success);
        }
        return;
    }
    *next += 1.0 / 15.0;
    commands.spawn(shot_of(target.as_deref())).observe(save_to_disk(format!("{dir}/f{:05}.png", *n)));
    *n += 1;
}

/// `VUHLEDAR_LOG_STATS=1`: print the tally once a game minute, with how often two hulls overlapped since the
/// last line (hull centres closer than 4 m: certainly through each other; closer than 6.5 m: touching or nearly).
fn log_stats(
    view: Res<View>,
    script: Res<crate::director::Script>,
    stats: Res<Stats>,
    fc: Res<FireControl>,
    order: Res<crate::units::ColumnOrder>,
    soldiers: Query<&Soldier>,
    vehicles: Query<(&Vehicle, &Transform)>,
    mut last: Local<u32>,
    mut overlap: Local<(u32, u32, u32, u32)>,
) {
    if !dev().log_stats {
        return;
    }
    let pos: Vec<(Vec2, VState)> = vehicles.iter().filter(|(v, _)| v.side == crate::diorama::diorama().attacker()).map(|(v, t)| (t.translation.xz(), v.state)).collect();
    let (mut through, mut touching) = (0, 0);
    let mut closest = (f32::MAX, VState::Advancing, VState::Advancing);
    for i in 0..pos.len() {
        for j in i + 1..pos.len() {
            let d = pos[i].0.distance(pos[j].0);
            through += (d < 4.0) as u32;
            touching += (d < 6.5) as u32;
            if d < closest.0 {
                closest = (d, pos[i].1, pos[j].1);
            }
        }
    }
    overlap.0 += 1;
    overlap.1 += (through > 0) as u32;
    overlap.2 = overlap.2.max(through);
    overlap.3 += (touching > 0) as u32;
    if script.t as u32 / 60 == *last {
        return;
    }
    *last = script.t as u32 / 60;
    let moving = vehicles.iter().filter(|(v, _)| v.state.mobile()).count();
    let men = soldiers.iter().filter(|s| s.side == crate::diorama::diorama().attacker()).count();
    info!(
        "t={:>4.0} {} committed={} destroyed={} abandoned={} withdrawn={} moving={} men_alive={} sent={} back={} dead={} foothold={} mines={} atgm={} art={} shells={} brigade={} ua_losses={} hidden={} overlap_frames={}/{} max_pairs={} touching_frames={} closest={:.1}m({:?},{:?}) view={:?} drone=({:.0},{:.0},{:.0}) yaw={:.2} fov={:.3} jam={:.2} rth={:?}",
        script.t, crate::director::clock_text(script.t), stats.vehicles_committed, stats.destroyed, stats.abandoned, stats.withdrawn, moving, men, stats.sent, stats.returned, stats.attacker_dead, stats.foothold, stats.mine_hits, stats.atgm_kills, stats.artillery_kills, fc.shells, fc.brigade_shells, stats.defender_losses,
        stats.hidden, overlap.1, overlap.0, overlap.2, overlap.3, closest.0, closest.1, closest.2,
        view.mode, view.drone_pos.x, view.drone_pos.y, view.drone_pos.z, view.drone_yaw, view.drone_fov, view.jamming, view.rth
    );
    *overlap = (0, 0, 0, 0);
    if crate::diorama::diorama() == crate::diorama::Diorama::Robotyne {
        info!("     picked_up={} ru_vehicles={} vehicle_advance={:.0} foot_advance={:.0} order={:?} overwatch_lost={} wounded={}/{} deaths={:?}", stats.picked_up, stats.defender_vehicles, stats.vehicle_advance.max(-9999.0), stats.max_advance.max(-9999.0), *order, stats.overwatch_losses, stats.wounded_out, stats.wounded, stats.deaths);
    }
}

/// `VUHLEDAR_FIRE="100:he,130:raam,160:himars"`: at those game seconds, call that strike on the nearest stalled
/// Russian vehicle, as a player would from the drone.
fn scripted_fire(script: Res<crate::director::Script>, time: Res<Time>, view: Res<View>, mut fc: ResMut<FireControl>, mut radio: ResMut<Radio>, q: Query<(&Vehicle, &Transform)>, mut done: Local<usize>) {
    let plan = &dev().fire;
    while *done < plan.len() && script.t >= plan[*done].0 {
        let strike = plan[*done].1;
        *done += 1;
        let now = time.elapsed_secs_f64();
        if let Some(at) = strike.on_nearest_stalled(&mut fc, &q, view.drone_pos.xz(), now) {
            radio.say(now, format!("(scripted) {strike:?} on {:.0}, {:.0}", at.x, at.y));
        }
    }
}

/// `VUHLEDAR_DIAG=1`: every ten seconds, log the mean and worst frame time and the GPU time of each render pass
/// (from `RenderDiagnosticsPlugin`, averaged over its last 120 frames), largest first.
fn frame_times(time: Res<Time<Real>>, diagnostics: Res<DiagnosticsStore>, mut acc: Local<(f32, f32, u32, f32)>) {
    if !dev().diag {
        return;
    }
    let dt = time.delta_secs();
    acc.0 += dt;
    acc.1 = acc.1.max(dt);
    acc.2 += 1;
    acc.3 += dt;
    if acc.3 > 10.0 {
        info!("frames: mean {:.1} ms, worst {:.1} ms", acc.0 / acc.2 as f32 * 1000.0, acc.1 * 1000.0);
        let mut passes: Vec<(&str, f64)> = diagnostics
            .iter()
            .filter_map(|d| {
                let name = d.path().as_str().strip_prefix("render/")?.strip_suffix("/elapsed_gpu")?;
                Some((name, d.average()?))
            })
            .collect();
        passes.sort_by(|a, b| b.1.total_cmp(&a.1));
        info!("gpu ms: {}", passes.iter().map(|(name, ms)| format!("{name} {ms:.2}")).collect::<Vec<_>>().join(", "));
        *acc = (0.0, 0.0, 0, 0.0);
    }
}

/// `VUHLEDAR_VIEW=ground`: start in the tree line nearest the dacha road with a `VUHLEDAR_TEAM` launcher
/// (default Stugna-P), sight up, tracking the nearest moving attacker vehicle the way a gunner would.
/// `VUHLEDAR_AUTO_ATGM=<s>` launches then.
fn env_ground(mut view: ResMut<View>, teams: Query<(Entity, &Post)>, q: Query<(&Vehicle, &Transform)>, mut started: Local<bool>) {
    if dev().view != Some(Mode::Ground) {
        return;
    }
    if !*started {
        let want = dev().team.unwrap_or(Atgm::Stugna);
        let kz = match crate::diorama::diorama() {
            crate::diorama::Diorama::Vuhledar => Vec2::new(930.0, -658.0),
            crate::diorama::Diorama::Robotyne => Vec2::new(-100.0, crate::robotyne::UA_LINE_Z),
        };
        let Some((e, p)) = teams.iter().filter(|(_, p)| p.atgm > 0 && p.weapon == Some(want)).min_by(|a, b| a.1.pos.distance(kz).total_cmp(&b.1.pos.distance(kz))) else {
            return;
        };
        crate::camera::enter_ground(&mut view, e, p.pos);
        *started = true;
    }
    view.sight = true;
    crate::camera::track_nearest(&mut view, &q);
}

fn key_shot(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>, mut n: Local<u32>) {
    if keys.just_pressed(KeyCode::F12) {
        *n += 1;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("vuhledar-{:03}.png", *n)));
    }
}
