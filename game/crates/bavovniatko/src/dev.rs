//! The development systems the `BAV_*` switches drive (the switches themselves are `bav_core::dev`). F12 saves a
//! screenshot.

pub use bav_core::dev::*;
use crate::camera::{HeadlessTarget, View, look_angles};
use crate::fire::{FireControl, Post};
use crate::units::{Soldier, Stats, VState, Vehicle};
use bav_core::vocab::{Atgm, Mode};
use crate::camera::Locate;
use crate::fire::StrikeMission;
use bevy::{diagnostic::DiagnosticsStore, prelude::*, render::view::screenshot::{Screenshot, save_to_disk}};

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(crate::camera::Flight(dev().fly.clone()))
            .add_systems(Startup, lineup.after(crate::Setup::Kit));
        app.add_systems(Update, (auto_shot, key_shot, env_view, env_look, record, log_stats, warp, env_debrief, scripted_fire, frame_times, env_ground, era_test).chain().in_set(crate::Step::Control).in_set(DevSystems));
        if dev().field {
            app.add_systems(Startup, field_overlay_setup).add_systems(Update, field_overlay.in_set(crate::Step::Screen));
        }
    }
}

/// `BAV_FIELD=1`: the field drawn over the picture, worked out once: ground-following line strips, and flat rings.
#[derive(Resource, Default)]
struct FieldLines {
    strips: Vec<(Vec<Vec3>, Color)>,
    rings: Vec<(Vec3, f32, Color)>,
}

/// `BAV_FIELD=1`: the field as the map table will know it. Each position a ring (green ours, yellow in the grey
/// zone, red theirs; hides and ruins larger, trenches and hides doubled) with a tick the way it faces; the road heads,
/// blue; the grey zone's edges; and the tracked routes from each road head to every other position of ours. The
/// positions are listed by number in the log, for `BAV_ALLOT` (`stugna@37`) until the map table exists.
fn field_overlay_setup(mut commands: Commands, mut store: ResMut<GizmoConfigStore>) {
    use bav_core::field::{Ground, Mover, PositionKind, field, routes};
    let (config, _) = store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 3.0;
    let f = field();
    let ours = f.positions.iter().filter(|p| p.ground == Ground::Ours).count();
    info!("field: {} positions ({ours} ours), {} road heads, {} route nodes, grey zone z {:.0}..{:.0}", f.positions.len(), f.road_heads.len(), routes().graph.nodes.len(), f.grey.0, f.grey.1);
    for (i, p) in f.positions.iter().enumerate() {
        info!("field: position {i}: {} ({}) at {:.0},{:.0}, {:?}", p.kind.name().0, p.kind.name().1, p.at.x, p.at.y, p.ground);
    }
    let lift = |p: Vec2| crate::geo::ground(p) + Vec3::Y * 2.0;
    // A strip that follows the ground, a point every 20 m.
    let on_ground = |pts: &[Vec2]| -> Vec<Vec3> {
        let mut out = vec![lift(pts[0])];
        for w in pts.windows(2) {
            let n = (w[0].distance(w[1]) / 20.0).ceil().max(1.0) as usize;
            out.extend((1..=n).map(|i| lift(w[0].lerp(w[1], i as f32 / n as f32))));
        }
        out
    };
    let mut lines = FieldLines::default();
    let yellow = Color::srgb(1.0, 0.9, 0.2);
    for p in &f.positions {
        let c = match p.ground {
            Ground::Ours => Color::srgb(0.3, 1.0, 0.4),
            Ground::Grey => yellow,
            Ground::Theirs => Color::srgb(1.0, 0.25, 0.2),
        };
        let r = match p.kind {
            PositionKind::Hide => 16.0,
            PositionKind::Ruin | PositionKind::Trench => 12.0,
            PositionKind::Treeline => 8.0,
        };
        lines.rings.push((lift(p.at), r, c));
        if matches!(p.kind, PositionKind::Trench | PositionKind::Hide) {
            lines.rings.push((lift(p.at), r * 0.6, c));
        }
        lines.strips.push((on_ground(&[p.at, p.at + p.facing * 25.0]), c));
    }
    for h in &f.road_heads {
        for r in [40.0, 25.0] {
            lines.rings.push((lift(h.at), r, Color::srgb(0.3, 0.5, 1.0)));
        }
    }
    for z in [f.grey.0, f.grey.1] {
        lines.strips.push((on_ground(&[Vec2::new(-crate::geo::HALF_X, z), Vec2::new(crate::geo::HALF_X, z)]), yellow));
    }
    let colours = [Color::srgb(0.2, 0.9, 1.0), Color::srgb(1.0, 0.4, 1.0), Color::srgb(1.0, 1.0, 1.0)];
    for h in 0..f.road_heads.len() {
        for p in f.positions.iter().filter(|p| p.ground == Ground::Ours).step_by(2) {
            if let Some(path) = routes().path(h, Mover::Tracked, p.at) {
                lines.strips.push((on_ground(&path.pts), colours[h % colours.len()]));
            }
        }
    }
    commands.insert_resource(lines);
}

fn field_overlay(mut gizmos: Gizmos, lines: Res<FieldLines>) {
    let flat = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    for (pts, c) in &lines.strips {
        gizmos.linestrip(pts.iter().copied(), *c);
    }
    for &(at, r, c) in &lines.rings {
        gizmos.circle(Isometry3d::new(at, flat), r, c);
    }
}

/// `BAV_LINEUP="x,z,heading"`: every vehicle model side by side, 10 m apart, and behind them the same burned
/// out, turrets turned a little so the split shows. The baked models no vehicle kind drives yet (the Ka-52, the
/// aircraft, Hostomel's set) follow, spaced by their own size.
fn lineup(mut commands: Commands, models: Res<crate::units::Models>, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<crate::baked::VehicleMaterial>>) {
    use crate::units::{VKind, VehicleMeshes, burnt, dress, dress_charred, skin};
    use bevy::camera::primitives::MeshAabb;
    use strum::VariantArray;
    const SPARE: [&str; 9] = ["ka52", "mi8", "mi24", "il76", "an225", "su24", "zu23", "btr80", "bmd2"];
    let Some([x, z, heading]) = dev().lineup else { return };
    let rot = Quat::from_rotation_y(heading.to_radians());
    let across = rot * Vec3::X;
    let back = rot * Vec3::Z;
    let spare: Vec<VehicleMeshes> = SPARE
        .iter()
        .map(|&id| {
            let b = crate::baked::load(id, bav_look::models::livery(id), &mut meshes, &mut images, &mut materials);
            VehicleMeshes { hull: b.hull, plow_hull: None, turret: b.turret, turret_at: b.turret_at, muzzle: b.muzzle }
        })
        .collect();
    // Each model's place along the row and the depth of its row, from its footprint (a rotor's disc included).
    let footprint = |m: &VehicleMeshes| {
        let (mut lo, mut hi) = (Vec3::MAX, Vec3::MIN);
        for (parts, off) in [(&m.hull, Vec3::ZERO), (&m.turret, m.turret_at)] {
            for b in parts.iter().filter_map(|p| meshes.get(&p.mesh)?.compute_aabb()) {
                lo = lo.min(Vec3::from(b.min()) + off);
                hi = hi.max(Vec3::from(b.max()) + off);
            }
        }
        (hi.x.max(-lo.x), hi.z - lo.z)
    };
    let fixed: Vec<&VehicleMeshes> = VKind::VARIANTS.iter().map(|k| models.vehicle(*k)).chain([&models.ua_tank]).collect();
    let mut all: Vec<(&VehicleMeshes, f32, f32)> = fixed.iter().enumerate().map(|(i, m)| (*m, i as f32 * 10.0, 14.0)).collect();
    let mut edge = (fixed.len() as f32 - 0.5) * 10.0;
    for m in &spare {
        let (half, length) = footprint(m);
        all.push((m, edge + half.max(2.5), (length + 4.0).max(14.0)));
        edge += 2.0 * half.max(2.5) + 4.0;
    }
    for (i, (m, side, depth)) in all.into_iter().enumerate() {
        for row in 0..2 {
            let at = Vec3::new(x, 0.0, z) + across * side + back * (row as f32 * depth);
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
                if dev().lineup_fire {
                    use crate::fx::{Emitter, EmitterKind};
                    match i % 3 {
                        0 => {
                            e.insert(Emitter::new(EmitterKind::WreckFire, 5.0, 1.0e5, Vec3::Y * 1.5));
                        }
                        1 => {
                            e.insert(Emitter::new(EmitterKind::Smoulder, 1.2, 1.0e3, Vec3::Y));
                        }
                        _ => {
                            commands.spawn((Transform::from_translation(at.transform_point(Vec3::X * 3.5)), Emitter::new(EmitterKind::PoolFire, 7.0, 1.0e5, Vec3::ZERO)));
                        }
                    }
                }
            }
        }
    }
}

/// `BAV_ERA_AT` with `BAV_LINEUP`: the reactive armour of the lineup's T-80, T-72 and Bradley going off
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

/// `BAV_SHOT=<path>`: save a screenshot after `BAV_SHOT_AT` seconds (default 6) and exit. With
/// `BAV_WARP_TO`, the seconds count from when the warp arrives, so a shot lands at a chosen game time.
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

/// `BAV_CAM="x,z,agl,yaw,pitch,fov"` places the drone; `BAV_VIEW=op` starts on the roof;
/// `BAV_SPEED` scales game time.
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

/// `BAV_LOOK=<idx>|men`: keep the drone's gimbal on that [`Target`].
/// What `BAV_LOOK=hit|pool|grass` aims at: the last hit, the pool fires, the grass fires, the missiles in the air.
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

/// `BAV_WARP_TO=<game s>`: run at 3x (the most the simulation's step cap allows at 60 fps) until then.
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

/// `BAV_DEBRIEF=1`: end the day as soon as the warp arrives (or at once), so the debrief can be looked at.
fn env_debrief(script: Res<crate::director::Script>, phase: Res<State<crate::Phase>>, mut next: ResMut<NextState<crate::Phase>>) {
    if dev().debrief && *phase.get() == crate::Phase::Playing && script.t >= dev().warp_to.unwrap_or(0.0) {
        next.set(crate::Phase::Debrief);
    }
}

/// `BAV_REC=<dir>`: from game time `BAV_REC_FROM` save `BAV_REC_SECS` seconds of frames at 15 fps
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

/// `BAV_LOG_STATS=1`: print the tally once a game minute, with how often two hulls overlapped since the
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

/// `BAV_FIRE="100:he,130:raam,160:himars"`: at those game seconds, call that strike on the nearest stalled
/// Russian vehicle, as a player would from the drone.
fn scripted_fire(mut commands: Commands, script: Res<crate::director::Script>, time: Res<Time>, view: Res<View>, q: Query<(&Vehicle, &Transform)>, mut done: Local<usize>) {
    let plan = &dev().fire;
    while *done < plan.len() && script.t >= plan[*done].0 {
        let (_, strike, point) = plan[*done];
        *done += 1;
        let now = time.elapsed_secs_f64();
        let called = match point {
            Some((x, z)) => {
                let at = Vec2::new(x, z);
                strike.call(&mut commands, at, 12.0, now);
                Some(at)
            }
            None => strike.on_nearest_stalled(&mut commands, &q, view.drone_pos.xz(), now),
        };
        if let Some(at) = called {
            info!("scripted: {strike:?} on {:.0},{:.0}, at game {:.0} s", at.x, at.y, script.t);
        }
    }
}

/// Every entity, those with a mesh, and the particles.
type Counts<'w, 's> = (Query<'w, 's, ()>, Query<'w, 's, (), With<Mesh3d>>, Query<'w, 's, (), With<crate::fx::Particle>>);

/// `BAV_DIAG=1`: every ten seconds, log the mean and worst frame time, the GPU time of each render pass (from
/// `RenderDiagnosticsPlugin`, averaged over its last 120 frames), largest first, and how many entities, meshes
/// and particles there are. A frame over 25 ms is logged on its own with the game time, to find what made it.
fn frame_times(
    time: Res<Time<Real>>,
    script: Res<crate::director::Script>,
    diagnostics: Res<DiagnosticsStore>,
    counts: Counts,
    mut acc: Local<(f32, f32, u32, f32)>,
    mut cpu_was: Local<f64>,
) {
    if !dev().diag {
        return;
    }
    let dt = time.delta_secs();
    if dt > 0.025 && dev().warp_to.is_none_or(|w| script.t >= w) {
        info!("hitch: {:.1} ms at {:.1} s", dt * 1000.0, script.t);
    }
    acc.0 += dt;
    acc.1 = acc.1.max(dt);
    acc.2 += 1;
    acc.3 += dt;
    if acc.3 > 10.0 {
        // The process's CPU time over the window, all threads: the work a frame costs, which other load on the
        // machine stretches far less than the wall clock.
        let cpu = cpu_secs();
        let per_frame = (cpu - *cpu_was) / acc.2 as f64 * 1000.0;
        *cpu_was = cpu;
        info!("frames: mean {:.1} ms, worst {:.1} ms, cpu {per_frame:.2} ms", acc.0 / acc.2 as f32 * 1000.0, acc.1 * 1000.0);
        let mut passes: Vec<(&str, f64)> = diagnostics
            .iter()
            .filter_map(|d| {
                let name = d.path().as_str().strip_prefix("render/")?.strip_suffix("/elapsed_gpu")?;
                Some((name, d.average()?))
            })
            .collect();
        passes.sort_by(|a, b| b.1.total_cmp(&a.1));
        info!("gpu ms: {}", passes.iter().map(|(name, ms)| format!("{name} {ms:.2}")).collect::<Vec<_>>().join(", "));
        info!("entities {}, meshes {}, particles {}", counts.0.iter().len(), counts.1.iter().len(), counts.2.iter().len());
        *acc = (0.0, 0.0, 0, 0.0);
    }
}

/// User plus system CPU seconds this process has used, from `/proc/self/stat` (0 where there is none).
fn cpu_secs() -> f64 {
    let Ok(stat) = std::fs::read_to_string("/proc/self/stat") else { return 0.0 };
    let Some((_, rest)) = stat.rsplit_once(')') else { return 0.0 };
    let f: Vec<&str> = rest.split_whitespace().collect();
    // After the command name: state is field 3, utime 14 and stime 15 (1-based), in clock ticks of 1/100 s.
    let tick = |i: usize| f.get(i - 3).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
    (tick(14) + tick(15)) / 100.0
}

/// `BAV_VIEW=ground`: start in the tree line nearest the dacha road with a `BAV_TEAM` launcher
/// (default Stugna-P), sight up, tracking the nearest moving attacker vehicle the way a gunner would.
/// `BAV_AUTO_ATGM=<s>` launches then.
fn env_ground(mut view: ResMut<View>, teams: Query<(Entity, &Post, &crate::fire::AtgmTeam)>, q: Query<(&Vehicle, &Transform)>, mut started: Local<bool>) {
    if dev().view != Some(Mode::Ground) {
        return;
    }
    if !*started {
        let want = dev().team.unwrap_or(Atgm::Stugna);
        let kz = match crate::diorama::diorama() {
            crate::diorama::Diorama::Vuhledar => Vec2::new(930.0, -658.0),
            crate::diorama::Diorama::Robotyne => Vec2::new(-100.0, crate::geo::layout().start_z.expect("Mala Tokmachka has a start line")),
        };
        let Some((e, p, _)) = teams.iter().filter(|(_, _, t)| t.atgm > 0 && t.weapon == want).min_by(|a, b| a.1.pos.distance(kz).total_cmp(&b.1.pos.distance(kz))) else {
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
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}-{:03}.png", crate::diorama::diorama(), *n)));
    }
}
