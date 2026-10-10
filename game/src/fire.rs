//! Fire: the Ukrainian tree-line positions and ATGM teams, artillery (the player's battery, the brigade's
//! pre-registered fires, RAAM, a HIMARS launcher), and the Russian return fire.
//!
//! The kill zone (dossier §5, Lt. Bayak): a drone spots the column, the road is left clear and the fields are
//! mined, ATGM teams in the tree lines open fire, panicking drivers swerve onto the shoulders, and artillery
//! then works over the stalled column and the crews climbing out.

use crate::audio::{self, Pending, Sfx};
use crate::camera::{self, MainCamera, Mode, Shake, View};
use crate::fx::{self, FxAssets, FxRng};
use crate::geo;
use crate::terrain::{Paint, PaintQueue};
use crate::units::{self, Blast, Cause, DirectHit, Mine, Minefield, Models, Radio, SState, Side, Soldier, Stats, Vehicle, VState};
use crate::world::{ObservationPost, Palette};
use bevy::{light::NotShadowCaster, prelude::*, window::PrimaryWindow};
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FireRng(SmallRng::seed_from_u64(72)))
            .init_resource::<FireControl>()
            .add_systems(Startup, spawn_positions.after(units::setup_models).after(crate::world::spawn_belts))
            .add_systems(
                Update,
                (player_fire, run_missions, land_shells, atgm_teams, fly_missiles, ua_small_arms, russian_fire, pre_registered_fires, ua_tank, ua_mortars)
                    .chain()
                    .run_if(crate::running),
            )
            .add_systems(Update, (aim_gizmo, atgm_mode_key));
    }
}

#[derive(Resource)]
pub struct FireRng(pub SmallRng);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AtgmMode {
    /// Teams wait until the column is committed (the lead vehicle is deep in the kill zone).
    Auto,
    /// Hold fire unless a vehicle is within 400 m.
    Hold,
    /// Fire at anything in range.
    Free,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShellKind {
    He155,
    Mortar120,
    Raam,
    Himars,
    Ru152,
    Tos,
    TankHe,
    Cannon30,
}

#[derive(Clone, Copy, Debug)]
pub struct Mission {
    pub target: Vec2,
    pub kind: ShellKind,
    pub rounds: u32,
    pub sigma: f32,
    pub interval: f32,
    pub next: f64,
    pub flight: f32,
    pub player: bool,
}

#[derive(Resource)]
pub struct FireControl {
    pub shells: u32,
    pub raam: u32,
    pub himars: u32,
    pub brigade_shells: u32,
    /// The battalion's 120 mm mortars, which work over infantry in the open on their own.
    pub mortar_bombs: u32,
    pub missions: Vec<Mission>,
    pub atgm_mode: AtgmMode,
    /// The battery can work one player mission at a time.
    pub battery_busy_until: f64,
    pub last_call: Option<(Vec2, f64)>,
    pub warned_low: bool,
}

impl Default for FireControl {
    fn default() -> Self {
        // Zaluzhnyi: artillery ammunition is the binding constraint. ~6,000 RAAM shells reached Ukraine in late
        // 2022, but a battery on one morning gets a handful of missions.
        Self { shells: 96, raam: 4, himars: 2, brigade_shells: 140, mortar_bombs: 160, missions: Vec::new(), atgm_mode: AtgmMode::Auto, battery_busy_until: 0.0, last_call: None, warned_low: false }
    }
}

/// A Ukrainian position in a tree line: riflemen, and maybe an ATGM team.
#[derive(Component)]
pub struct UaPosition {
    pub pos: Vec2,
    pub strength: u32,
    pub atgm: u32,
    pub reload: f32,
    pub fire_timer: f32,
    pub weapon: &'static str,
    pub figures: Vec<Entity>,
}

#[derive(Component)]
pub struct Shell {
    pub at: Vec2,
    pub kind: ShellKind,
    pub land: f64,
    pub whistled: bool,
    pub by: Side,
}

#[derive(Component)]
pub struct Missile {
    pub target: Entity,
    pub launched_at: Vec3,
    pub speed: f32,
    pub from: Entity,
    pub trail: f32,
}

#[derive(Component)]
pub struct UaTank {
    pub timer: f32,
}

fn spawn_positions(mut commands: Commands, models: Res<Models>, palette: Res<Palette>) {
    let lay = geo::layout();
    let weapons = ["Stugna-P", "Javelin", "Stugna-P", "Javelin", "Stugna-P", "Stugna-P", "", "", ""];
    for (i, p) in lay.ua_positions.iter().enumerate() {
        let atgm = if weapons[i].is_empty() { 0 } else if weapons[i] == "Javelin" { 4 } else { 6 };
        let mut figures = Vec::new();
        // A few figures in the trench line under the trees, mostly hidden.
        for k in 0..3 {
            let at = *p + Vec2::new(k as f32 * 3.0 - 3.0, (k % 2) as f32 * 1.5);
            let e = commands
                .spawn((
                    Mesh3d(models.ua_body.clone()),
                    MeshMaterial3d(palette.vertex.clone()),
                    Transform::from_translation(geo::ground(at) - Vec3::Y * 0.9).with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                    NotShadowCaster,
                ))
                .id();
            figures.push(e);
        }
        commands.spawn((
            UaPosition { pos: *p, strength: 6, atgm, reload: 4.0 + i as f32 * 7.0, fire_timer: i as f32, weapon: weapons[i], figures },
            Transform::from_translation(geo::ground(*p)),
        ));
    }
    // The Wanderer (NYT): a T-64 in a hide at the town edge, firing on coordinates from a drone pilot.
    let hide = Vec2::new(-150.0, -1150.0);
    let yaw = units::vehicle_yaw(Vec2::new(0.2, 1.0));
    commands
        .spawn((
            Mesh3d(models.ua_tank.0.clone()),
            MeshMaterial3d(palette.vertex.clone()),
            Transform::from_translation(geo::ground(hide)).with_rotation(Quat::from_rotation_y(yaw)),
            UaTank { timer: 20.0 },
        ))
        .with_child((Mesh3d(models.ua_tank.1.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(models.ua_tank.2)));
}

fn atgm_mode_key(keys: Res<ButtonInput<KeyCode>>, mut fc: ResMut<FireControl>, time: Res<Time>, mut radio: ResMut<Radio>) {
    if keys.just_pressed(KeyCode::Space) {
        fc.atgm_mode = match fc.atgm_mode {
            AtgmMode::Auto => AtgmMode::Hold,
            AtgmMode::Hold => AtgmMode::Free,
            AtgmMode::Free => AtgmMode::Auto,
        };
        let line = match fc.atgm_mode {
            AtgmMode::Auto => "All teams: wait until the column is committed, then fire.",
            AtgmMode::Hold => "All teams: hold fire. Let them come.",
            AtgmMode::Free => "All teams: fire at will.",
        };
        radio.say(time.elapsed_secs_f64(), line);
    }
}

// ---------------------------------------------------------------------------------------------------------
// Player fire control

fn observed_sigma(view: &View, target: Vec2) -> f32 {
    // Adjusted fire: a drone over the target halves the dispersion; the OP on the roof helps close in.
    match view.mode {
        Mode::Drone => {
            let d = Vec2::new(view.drone_pos.x, view.drone_pos.z).distance(target);
            if view.feed_down > 0.0 { 40.0 } else { 9.0 + d * 0.006 }
        }
        Mode::Op => 14.0 + 0.004 * (target.y + 1200.0).abs(),
    }
}

#[allow(clippy::too_many_arguments)]
fn player_fire(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    view: Res<View>,
    mut fc: ResMut<FireControl>,
    mut radio: ResMut<Radio>,
    mut pending: ResMut<Pending>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let now = time.elapsed_secs_f64();
    let Some(hit) = camera::cursor_ground(&window, cam.0, cam.1) else { return };
    let target = Vec2::new(hit.x, hit.z);
    let sigma = observed_sigma(&view, target);
    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    let (lat, lon) = geo::latlon(target);
    if ctrl {
        if fc.himars == 0 {
            radio.say(now, "HIMARS: nothing left for you today.");
            return;
        }
        fc.himars -= 1;
        fc.missions.push(Mission { target, kind: ShellKind::Himars, rounds: 2, sigma: 3.0, interval: 4.0, next: now + 2.0, flight: 55.0, player: true });
        radio.say(now, format!("HIMARS, two rockets, {lat:.4} {lon:.4}. Flight time about a minute."));
    } else if shift {
        if fc.raam == 0 {
            radio.say(now, "No more RAAM.");
            return;
        }
        fc.raam -= 1;
        fc.missions.push(Mission { target, kind: ShellKind::Raam, rounds: 3, sigma: sigma * 1.2, interval: 5.0, next: now + 3.0, flight: 28.0, player: true });
        radio.say(now, format!("RAAM, three shells, {lat:.4} {lon:.4}. Mines going down on that lane."));
    } else {
        if fc.shells < 4 {
            radio.say(now, "Battery: out of shells. Nothing to send.");
            return;
        }
        if now < fc.battery_busy_until {
            radio.say(now, "Battery still firing the last mission. Wait.");
            return;
        }
        // Repeated missions on the same spot get adjusted onto it.
        let adjust = fc.last_call.is_some_and(|(p, t)| p.distance(target) < 60.0 && now - t < 120.0);
        let sigma = if adjust { sigma * 0.6 } else { sigma };
        fc.shells -= 4;
        fc.battery_busy_until = now + 18.0;
        fc.last_call = Some((target, now));
        fc.missions.push(Mission { target, kind: ShellKind::He155, rounds: 4, sigma, interval: 3.5, next: now + 2.0, flight: 24.0, player: true });
        radio.say(now, format!("Fire mission, {lat:.4} {lon:.4}, four rounds. {}", if adjust { "Adjusting from the last." } else { "Shot, over." }));
        if fc.shells <= 32 && !fc.warned_low {
            fc.warned_low = true;
            radio.say(now, format!("Battery: {} rounds left. That's all we get today.", fc.shells));
        }
    }
    audio::play_now(&mut pending, now, Sfx::Radio, 0.25);
}

fn aim_gizmo(
    mut gizmos: Gizmos,
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    view: Res<View>,
    keys: Res<ButtonInput<KeyCode>>,
    fc: Res<FireControl>,
    time: Res<Time>,
) {
    if let Some(hit) = camera::cursor_ground(&window, cam.0, cam.1) {
        let t = Vec2::new(hit.x, hit.z);
        let sigma = observed_sigma(&view, t);
        let color = if keys.pressed(KeyCode::ControlLeft) {
            Color::srgba(1.0, 0.3, 0.2, 0.8)
        } else if keys.pressed(KeyCode::ShiftLeft) {
            Color::srgba(0.4, 0.9, 1.0, 0.8)
        } else {
            Color::srgba(1.0, 0.85, 0.3, 0.7)
        };
        let iso = Isometry3d::new(hit + Vec3::Y * 0.8, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
        gizmos.circle(iso, sigma * 2.0, color);
        gizmos.circle(iso, 1.5, color);
    }
    // Pending missions: where the rounds will land.
    let now = time.elapsed_secs_f64();
    for m in &fc.missions {
        if m.player {
            let blink = ((now * 4.0) as i64 % 2) as f32;
            let iso = Isometry3d::new(geo::ground(m.target) + Vec3::Y, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
            gizmos.circle(iso, m.sigma * 2.0, Color::srgba(1.0, 0.4, 0.2, 0.3 + 0.5 * blink));
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// Shells

fn run_missions(mut commands: Commands, time: Res<Time>, mut fc: ResMut<FireControl>, mut rng: ResMut<FireRng>) {
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    let mut done = Vec::new();
    for (i, m) in fc.missions.iter_mut().enumerate() {
        while m.rounds > 0 && now >= m.next {
            m.rounds -= 1;
            m.next += m.interval as f64 * rng.random_range(0.7..1.3);
            let off = Vec2::new(gauss(rng), gauss(rng)) * m.sigma;
            let by = if matches!(m.kind, ShellKind::Ru152 | ShellKind::Tos | ShellKind::TankHe | ShellKind::Cannon30) { Side::Ru } else { Side::Ua };
            commands.spawn(Shell { at: m.target + off, kind: m.kind, land: now + m.flight as f64 * rng.random_range(0.95..1.05), whistled: false, by });
        }
        if m.rounds == 0 {
            done.push(i);
        }
    }
    for i in done.into_iter().rev() {
        fc.missions.remove(i);
    }
}

pub fn gauss(rng: &mut SmallRng) -> f32 {
    let u: f32 = rng.random_range(1e-6..1.0);
    let v: f32 = rng.random_range(0.0..1.0);
    (-2.0 * u.ln()).sqrt() * (std::f32::consts::TAU * v).cos()
}

#[allow(clippy::too_many_arguments)]
fn land_shells(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut paint: ResMut<PaintQueue>,
    mut pending: ResMut<Pending>,
    mut blasts: MessageWriter<Blast>,
    mut mf: ResMut<Minefield>,
    mut shake: ResMut<Shake>,
    mut view: ResMut<View>,
    mut radio: ResMut<Radio>,
    op: Option<Res<ObservationPost>>,
    (models, palette, mut stats): (Res<Models>, Res<Palette>, ResMut<Stats>),
    cam: Single<&GlobalTransform, With<MainCamera>>,
    mut positions: Query<&mut UaPosition>,
    mut q: Query<(Entity, &mut Shell)>,
) {
    let now = time.elapsed_secs_f64();
    let listener = cam.translation();
    let rng = &mut fxr.0;
    for (e, mut s) in &mut q {
        let p = geo::ground(s.at);
        if !s.whistled && now > s.land - 1.4 {
            s.whistled = true;
            if listener.distance(p) < 400.0 && s.kind != ShellKind::Cannon30 {
                audio::play_at(&mut pending, now - listener.distance(p) as f64 / 343.0, listener, p, Sfx::Whistle, 0.6);
            }
        }
        if now < s.land {
            continue;
        }
        commands.entity(e).despawn();
        let snow = true;
        let (size, radius, cause, sfx, crater) = match s.kind {
            ShellKind::He155 => (1.2, 15.0, Cause::Artillery, Sfx::BlastNear, 3.2),
            ShellKind::Mortar120 => (0.8, 12.0, Cause::Artillery, Sfx::BlastNear, 2.0),
            ShellKind::Ru152 => (1.1, 14.0, Cause::Artillery, Sfx::BlastNear, 3.0),
            ShellKind::Himars => (2.2, 30.0, Cause::Himars, Sfx::BlastNear, 5.5),
            ShellKind::Tos => (1.6, 30.0, Cause::Thermobaric, Sfx::Thermobaric, 4.5),
            ShellKind::TankHe => (0.8, 9.0, Cause::TankGun, Sfx::BlastNear, 2.0),
            ShellKind::Cannon30 => (0.18, 2.0, Cause::TankGun, Sfx::BlastNear, 0.0),
            ShellKind::Raam => (0.5, 0.0, Cause::Artillery, Sfx::BlastFar, 0.0),
        };
        match s.kind {
            ShellKind::Raam => {
                // The base ejects nine mines over the lane; they lie on the snow, visible from the drone.
                fx::spawn(&mut commands, &mut fx, fx::Spawn { kind: fx::Kind::GreySmoke, pos: p + Vec3::Y * 60.0, vel: Vec3::Y * -2.0, life: 4.0, size0: 3.0, size1: 8.0, alpha: 0.5, ..default() });
                audio::play_at(&mut pending, now, listener, p + Vec3::Y * 60.0, sfx, 0.4);
                for _ in 0..9 {
                    let mp = s.at + Vec2::new(gauss(rng), gauss(rng)) * 22.0;
                    let ent = commands
                        .spawn((Mesh3d(models.raam.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(mp)), NotShadowCaster))
                        .id();
                    mf.add(Mine { pos: mp, live: true, raam: true, entity: Some(ent) });
                }
                continue;
            }
            ShellKind::Tos => fx::thermobaric(&mut commands, &mut fx, rng, p),
            ShellKind::Cannon30 => {
                fx::spawn(&mut commands, &mut fx, fx::Spawn { kind: fx::Kind::Dirt, pos: p, vel: Vec3::Y * 6.0, life: 0.8, size0: 0.5, size1: 2.0, alpha: 0.8, gravity: 9.0, ..default() });
            }
            _ => fx::explosion(&mut commands, &mut fx, rng, p, size, snow),
        }
        if crater > 0.0 {
            paint.push(Paint::Crater { at: s.at, radius: crater * rng.random_range(0.8..1.2), seed: rng.random() });
        }
        if s.kind != ShellKind::Cannon30 {
            audio::play_at(&mut pending, now, listener, p, sfx, size);
        }
        let dist = listener.distance(p);
        if dist < 250.0 {
            shake.trauma = (shake.trauma + size * (1.0 - dist / 250.0)).min(1.0);
            if dist < 60.0 {
                shake.flash = (shake.flash + 0.5).min(1.0);
            }
        }
        blasts.write(Blast { at: p, radius, cause, by: s.by });
        // Russian fire on the tree lines and the town hurts the defenders.
        if s.by == Side::Ru {
            for mut pos in &mut positions {
                let d = pos.pos.distance(s.at);
                if pos.strength > 0 && d < radius * 1.6 && rng.random_bool((0.5 * (1.0 - d / (radius * 1.6))) as f64) {
                    pos.strength -= 1;
                    stats.ua_losses += 1;
                    if let Some(f) = pos.figures.pop() {
                        commands.entity(f).despawn();
                    }
                    if pos.strength == 0 {
                        radio.say(now, format!("We've lost the position at {:.0}, {:.0}. Nobody answering.", pos.pos.x, pos.pos.y));
                    } else if rng.random_bool(0.4) {
                        radio.say(now, "Three hundred. Need evacuation from the tree line.");
                    }
                }
            }
            if let Some(op) = op.as_ref()
                && view.op_alive
                && op.building.distance(s.at) < if s.kind == ShellKind::Tos { 40.0 } else { 15.0 }
                && rng.random_bool(0.04)
            {
                view.op_alive = false;
                radio.say(now, "The OP on the roof is gone. TOS took the building. No eyes from the high-rise.");
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// ATGM teams

#[allow(clippy::too_many_arguments)]
fn atgm_teams(
    mut commands: Commands,
    time: Res<Time>,
    fc: Res<FireControl>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut pending: ResMut<Pending>,
    mut radio: ResMut<Radio>,
    models: Res<Models>,
    palette: Res<Palette>,
    cam: Single<&GlobalTransform, With<MainCamera>>,
    vehicles: Query<(Entity, &Vehicle, &Transform)>,
    missiles: Query<&Missile>,
    mut teams: Query<(Entity, &mut UaPosition)>,
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs_f64();
    let lay = geo::layout();
    // Is any column committed: its lead vehicle past the deploy point, or stopped in the kill zone?
    let committed: Vec<usize> = (0..lay.routes.len())
        .filter(|&r| vehicles.iter().any(|(_, v, _)| v.route == r && v.state.alive() && v.s > lay.routes[r].deploy_s - 350.0))
        .collect();
    let targeted: Vec<Entity> = missiles.iter().map(|m| m.target).collect();
    for (team, mut pos) in &mut teams {
        if pos.atgm == 0 || pos.strength < 2 {
            continue;
        }
        pos.reload -= dt;
        if pos.reload > 0.0 {
            continue;
        }
        let range = if pos.weapon == "Javelin" { 2500.0 } else { 4500.0 };
        let mut best: Option<(Entity, f32, Vec3)> = None;
        for (e, v, tf) in &vehicles {
            if !v.state.alive() || v.side != Side::Ru || targeted.contains(&e) {
                continue;
            }
            let vp = Vec2::new(tf.translation.x, tf.translation.z);
            let d = vp.distance(pos.pos);
            let allowed = match fc.atgm_mode {
                AtgmMode::Free => d < range,
                AtgmMode::Hold => d < 400.0,
                AtgmMode::Auto => d < range.min(2500.0) && (committed.contains(&v.route) || d < 700.0),
            };
            if !allowed {
                continue;
            }
            // Prefer tanks, then moving vehicles, then the nearest.
            let score = d - if v.kind.is_tank() { 600.0 } else { 0.0 } - if v.state.mobile() { 400.0 } else { -2000.0 };
            if best.is_none_or(|b| score < b.1) {
                best = Some((e, score, tf.translation));
            }
        }
        let Some((target, _, tp)) = best else { continue };
        // Each team takes its own time to acquire and track: a few seconds, rarely all at once.
        if !fxr.0.random_bool((dt / 6.0).min(1.0) as f64) {
            continue;
        }
        pos.atgm -= 1;
        pos.reload = if pos.weapon == "Javelin" { 25.0 } else { 40.0 };
        let from = geo::ground(pos.pos) + Vec3::Y * 1.2;
        let dir = (tp - from).normalize();
        fx::muzzle(&mut commands, &mut fx, &mut fxr.0, from, dir, false);
        for _ in 0..8 {
            fx::spawn(&mut commands, &mut fx, fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: from - dir * 2.0, vel: -dir * fxr.0.random_range(4.0..12.0) + Vec3::Y, life: 4.0, size0: 1.0, size1: 5.0, alpha: 0.6, drag: 1.5, ..default() });
        }
        audio::play_at(&mut pending, now, cam.translation(), from, Sfx::AtgmLaunch, 0.9);
        let speed = if pos.weapon == "Javelin" { 140.0 } else { 190.0 };
        commands.spawn((
            Mesh3d(models.missile.clone()),
            MeshMaterial3d(palette.vertex.clone()),
            Transform::from_translation(from).looking_to(dir, Vec3::Y),
            Missile { target, launched_at: from, speed, from: team, trail: 0.0 },
            NotShadowCaster,
        ));
        if fxr.0.random_bool(0.5) {
            radio.say(now, format!("{}: launch.", pos.weapon));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn fly_missiles(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut hits: MessageWriter<DirectHit>,
    mut paint: ResMut<PaintQueue>,
    targets: Query<&Transform, (With<Vehicle>, Without<Missile>)>,
    mut q: Query<(Entity, &mut Missile, &mut Transform)>,
) {
    let dt = time.delta_secs().min(0.05);
    for (e, mut m, mut tf) in &mut q {
        let Ok(t) = targets.get(m.target) else {
            commands.entity(e).despawn();
            continue;
        };
        let aim = t.translation + Vec3::Y * 1.3;
        let to = aim - tf.translation;
        let d = to.length();
        // Fly a little high, then down onto the target (wire-guided line of sight, with a gentle arc).
        let lift = Vec3::Y * (d / 300.0).min(1.0) * 4.0;
        let dir = (to + lift).normalize();
        tf.translation += dir * m.speed * dt;
        tf.look_to(dir, Vec3::Y);
        m.trail += dt;
        if m.trail > 0.03 {
            m.trail = 0.0;
            fx::spawn(&mut commands, &mut fx, fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: tf.translation, vel: Vec3::Y * 0.3, life: 3.5, size0: 0.4, size1: 2.4, alpha: 0.55, drag: 0.5, ..default() });
            fx::spawn(&mut commands, &mut fx, fx::Spawn { kind: fx::Kind::Flash, pos: tf.translation, vel: Vec3::ZERO, life: 0.05, size0: 0.8, size1: 0.6, alpha: 1.0, ..default() });
        }
        if d < 3.0 {
            commands.entity(e).despawn();
            let rng = &mut fxr.0;
            // Long wire-guided shots miss more often: the gunner holds the crosshair for 15-25 s under fire.
            let range = m.launched_at.distance(aim);
            if rng.random_bool((0.88 - range / 9000.0).clamp(0.4, 0.88) as f64) {
                hits.write(DirectHit { target: m.target, cause: Cause::Atgm });
                fx::explosion(&mut commands, &mut fx, rng, aim, 0.7, false);
            } else {
                // A miss: it ploughs into the snow beyond.
                let miss = Vec2::new(aim.x, aim.z) + Vec2::new(gauss(rng), gauss(rng)) * 12.0;
                fx::explosion(&mut commands, &mut fx, rng, geo::ground(miss), 0.6, true);
                paint.push(Paint::Crater { at: miss, radius: 1.2, seed: rng.random() });
            }
            let _ = m.from;
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// Small arms and the Russian return fire

#[allow(clippy::too_many_arguments)]
fn ua_small_arms(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut stats: ResMut<Stats>,
    mut pending: ResMut<Pending>,
    models: Res<Models>,
    palette: Res<Palette>,
    cam: Single<&GlobalTransform, With<MainCamera>>,
    mut positions: Query<&mut UaPosition>,
    mut soldiers: Query<(Entity, &mut Soldier, &Transform)>,
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs_f64();
    let rng = &mut fxr.0;
    let mut dead: Vec<Entity> = Vec::new();
    for mut pos in &mut positions {
        if pos.strength == 0 {
            continue;
        }
        pos.fire_timer -= dt;
        if pos.fire_timer > 0.0 {
            continue;
        }
        pos.fire_timer = rng.random_range(1.5..3.5);
        let from = geo::ground(pos.pos) + Vec3::Y * 0.8;
        let mut target: Option<(Entity, f32, Vec3)> = None;
        for (e, s, tf) in &soldiers {
            if s.side != Side::Ru || dead.contains(&e) {
                continue;
            }
            let d = Vec2::new(tf.translation.x, tf.translation.z).distance(pos.pos);
            if d < 750.0 && target.is_none_or(|t| d < t.1) {
                target = Some((e, d, tf.translation));
            }
        }
        let Some((e, d, tp)) = target else { continue };
        for k in 0..5 {
            let spread = Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(0.0..1.5), rng.random_range(-3.0..3.0)) * (d / 300.0);
            if k % 2 == 0 {
                fx::tracer(&mut commands, &mut fx, from, tp + spread, 700.0);
            }
        }
        audio::play_at(&mut pending, now, cam.translation(), from, Sfx::MachineGun, 0.5);
        let Ok((_, mut s, tf)) = soldiers.get_mut(e) else { continue };
        let prone = if matches!(s.state, SState::Prone | SState::DugIn) { 0.35 } else { 1.0 };
        let p_kill = (0.42 * (1.0 - d / 800.0) * prone * (pos.strength as f32 / 6.0).sqrt()).clamp(0.02, 0.6);
        if rng.random_bool(p_kill as f64) {
            units::kill_soldier(&mut commands, &models, &palette, &mut stats, e, &s, tf, rng);
            dead.push(e);
        } else {
            s.fear += 0.7;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn russian_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut pending: ResMut<Pending>,
    cam: Single<&GlobalTransform, With<MainCamera>>,
    positions: Query<&UaPosition>,
    turrets: Query<&GlobalTransform, With<units::Turret>>,
    mut vehicles: Query<(&mut Vehicle, &Transform)>,
    soldiers: Query<(&Soldier, &Transform)>,
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs_f64();
    let rng = &mut fxr.0;
    let live: Vec<Vec2> = positions.iter().filter(|p| p.strength > 0).map(|p| p.pos).collect();
    let tree_line = |rng: &mut SmallRng, from: Vec2| -> Vec2 {
        // Fire at a known position if one is near, else into the nearest tree line ahead.
        if !live.is_empty() && rng.random_bool(0.5) {
            let p = live[rng.random_range(0..live.len())];
            if p.distance(from) < 2500.0 {
                return p + Vec2::new(gauss(rng), gauss(rng)) * 30.0;
            }
        }
        Vec2::new(from.x + rng.random_range(-400.0..400.0), -660.0 + rng.random_range(-15.0..15.0))
    };
    for (mut v, tf) in &mut vehicles {
        if !matches!(v.state, VState::Advancing | VState::Deploying | VState::Overwatch | VState::Halted | VState::Immobilized) || v.crew == 0 {
            continue;
        }
        v.fire_timer -= dt;
        if v.aim.is_none() {
            let from = Vec2::new(tf.translation.x, tf.translation.z);
            v.aim = Some(tree_line(rng, from));
        }
        if v.fire_timer > 0.0 {
            continue;
        }
        let from2 = Vec2::new(tf.translation.x, tf.translation.z);
        let aim = v.aim.unwrap();
        let muzzle = turrets.get(v.turret).map(|t| t.transform_point(Vec3::new(0.0, 0.38, -6.2))).unwrap_or(tf.translation + Vec3::Y * 2.0);
        let dir = (geo::ground(aim) - muzzle).normalize();
        if v.kind.is_tank() {
            v.fire_timer = rng.random_range(14.0..26.0);
            fx::muzzle(&mut commands, &mut fx, rng, muzzle, dir, true);
            audio::play_at(&mut pending, now, cam.translation(), muzzle, Sfx::TankGun, 1.0);
            let d = from2.distance(aim);
            commands.spawn(Shell { at: aim + Vec2::new(gauss(rng), gauss(rng)) * (8.0 + d * 0.01), kind: ShellKind::TankHe, land: now + d as f64 / 900.0, whistled: true, by: Side::Ru });
        } else {
            v.fire_timer = rng.random_range(6.0..14.0);
            for k in 0..6 {
                let hit = aim + Vec2::new(gauss(rng), gauss(rng)) * 12.0;
                if k % 2 == 0 {
                    fx::tracer(&mut commands, &mut fx, muzzle, geo::ground(hit) + Vec3::Y, 900.0);
                }
                commands.spawn(Shell { at: hit, kind: ShellKind::Cannon30, land: now + 1.5 + k as f64 * 0.18, whistled: true, by: Side::Ru });
            }
            fx::muzzle(&mut commands, &mut fx, rng, muzzle, dir, false);
            audio::play_at(&mut pending, now, cam.translation(), muzzle, Sfx::Cannon30, 0.8);
        }
        if rng.random_bool(0.3) {
            v.aim = None;
        }
    }
    // Russian infantry fire back from the snow: green tracers into the tree line, mostly harmless.
    let mut n = 0;
    for (s, tf) in &soldiers {
        if s.side != Side::Ru || !matches!(s.state, SState::Prone | SState::DugIn) || n > 6 {
            continue;
        }
        if rng.random_bool((dt * 0.25) as f64) {
            n += 1;
            let from = tf.translation + Vec3::Y * 0.4;
            let to = geo::ground(tree_line(rng, Vec2::new(from.x, from.z))) + Vec3::Y * 2.0;
            fx::tracer(&mut commands, &mut fx, from, to, 700.0);
            audio::play_at(&mut pending, now, cam.translation(), from, Sfx::MachineGun, 0.3);
        }
    }
}

/// The brigade's pre-registered fires: when a column stalls on a known point, the guns fire on their own.
#[allow(clippy::too_many_arguments)]
fn pre_registered_fires(time: Res<Time>, mut fc: ResMut<FireControl>, mut radio: ResMut<Radio>, mut last: Local<Vec<f64>>, vehicles: Query<(&Vehicle, &Transform)>, soldiers: Query<(&Soldier, &Transform)>) {
    let now = time.elapsed_secs_f64();
    let trps = &geo::layout().trps;
    last.resize(trps.len(), -1000.0);
    for (i, trp) in trps.iter().enumerate() {
        if now - last[i] < 50.0 || fc.brigade_shells < 6 {
            continue;
        }
        // Stalled: knocked out, dismounting or firing from the spot, or halted behind a blockage for a while.
        let stalled = vehicles
            .iter()
            .filter(|(v, t)| {
                v.side == Side::Ru
                    && v.state.alive()
                    && (matches!(v.state, VState::Immobilized | VState::Dismounting | VState::Overwatch) || (v.state == VState::Halted && v.timer > 6.0))
                    && Vec2::new(t.translation.x, t.translation.z).distance(*trp) < 140.0
            })
            .count();
        let men = soldiers.iter().filter(|(s, t)| s.side == Side::Ru && Vec2::new(t.translation.x, t.translation.z).distance(*trp) < 140.0).count();
        if stalled >= 1 || men >= 6 {
            last[i] = now;
            fc.brigade_shells -= 6;
            fc.missions.push(Mission { target: *trp, kind: ShellKind::He155, rounds: 6, sigma: 35.0, interval: 2.5, next: now + 1.0, flight: 22.0, player: false });
            radio.say(now, format!("Brigade guns: firing on registered point {}.", i + 1));
        }
    }
}

/// The Wanderer: the T-64 crew in the hide fires on stalled columns at long range, on a drone pilot's word.
#[allow(clippy::too_many_arguments)]
fn ua_tank(mut commands: Commands, time: Res<Time>, mut fx: ResMut<FxAssets>, mut fxr: ResMut<FxRng>, mut pending: ResMut<Pending>, cam: Single<&GlobalTransform, With<MainCamera>>, vehicles: Query<(&Vehicle, &Transform)>, mut q: Query<(&mut UaTank, &Transform)>) {
    let now = time.elapsed_secs_f64();
    let rng = &mut fxr.0;
    for (mut t, tf) in &mut q {
        t.timer -= time.delta_secs();
        if t.timer > 0.0 {
            continue;
        }
        t.timer = rng.random_range(18.0..30.0);
        let from = Vec2::new(tf.translation.x, tf.translation.z);
        let targets: Vec<Vec2> = vehicles.iter().filter(|(v, _)| v.state.alive() && !matches!(v.state, VState::Advancing)).map(|(_, t)| Vec2::new(t.translation.x, t.translation.z)).filter(|p| p.distance(from) < 4500.0).collect();
        if targets.is_empty() {
            continue;
        }
        let aim = targets[rng.random_range(0..targets.len())];
        let muzzle = tf.translation + Vec3::Y * 1.9 + (tf.rotation * Vec3::NEG_Z) * 5.0;
        fx::muzzle(&mut commands, &mut fx, rng, muzzle, (geo::ground(aim) - muzzle).normalize(), true);
        audio::play_at(&mut pending, now, cam.translation(), muzzle, Sfx::TankGun, 1.2);
        let d = from.distance(aim);
        commands.spawn(Shell { at: aim + Vec2::new(gauss(rng), gauss(rng)) * (6.0 + d * 0.006), kind: ShellKind::TankHe, land: now + d as f64 / 850.0, whistled: true, by: Side::Ua });
    }
}

/// The battalion mortars: when Russian infantry bunch up within range in the open, a few bombs on them.
fn ua_mortars(time: Res<Time>, mut fc: ResMut<FireControl>, mut rng: ResMut<FireRng>, mut next: Local<f64>, soldiers: Query<(&Soldier, &Transform)>) {
    let now = time.elapsed_secs_f64();
    if now < *next || fc.mortar_bombs < 3 {
        return;
    }
    let rng = &mut rng.0;
    let men: Vec<Vec2> = soldiers
        .iter()
        .filter(|(s, t)| s.side == Side::Ru && s.state != SState::Flee && t.translation.z < 700.0 && t.translation.z > -900.0)
        .map(|(_, t)| Vec2::new(t.translation.x, t.translation.z))
        .collect();
    if men.len() < 3 {
        *next = now + 5.0;
        return;
    }
    // Aim at the densest group: the man with the most others within 40 m.
    let target = *men.iter().max_by_key(|p| men.iter().filter(|q| q.distance(**p) < 40.0).count()).unwrap();
    let rounds = rng.random_range(2..5);
    fc.mortar_bombs -= rounds;
    fc.missions.push(Mission { target, kind: ShellKind::Mortar120, rounds, sigma: 22.0, interval: 2.5, next: now, flight: 16.0, player: false });
    *next = now + rng.random_range(14.0..26.0);
}
