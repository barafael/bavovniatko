//! Air defence at our posts (design/hostomel.md, phase 4): the Igla teams and the ZU-23-2 the National Guard's rear
//! echelon had at Hostomel. The teams fire by themselves at what comes within reach; the player can take one and
//! fire it (`Mode::Manpads`): shoulder the tube, find the heat of a helicopter in the ring until the seeker's growl
//! turns to the steady tone, and press. The missile flies its own way after that, and the helicopter may put out
//! flares (the Ka-52s often did not, RUSI). A ZU-23-2 fires bursts at anything low within two kilometres.

use crate::air::{AirHit, Aircraft, Flight};
use crate::allot::AirArm;
use crate::fx::{self, FxQueue};
use crate::geo;
use crate::player::View;
use crate::units::Radio;
use bav_core::vocab::Mode;
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct ManpadsPlugin;

/// Whether the day has any air defence of ours at its posts (an `Air` unit in its catalog): only then is any of this
/// in its world, the Igla's view among it.
pub fn present() -> bool {
    crate::data::sim_data().catalog.iter().any(|u| matches!(u.what, crate::allot::UnitKind::Air(_)))
}

impl Plugin for ManpadsPlugin {
    fn build(&self, app: &mut App) {
        if !present() {
            return;
        }
        app.insert_resource(ManpadsRng(SmallRng::seed_from_u64(crate::dev::seeded(9338))))
            .add_systems(Update, (player_igla, auto_iglas, zu23, fly_iglas).chain().run_if(crate::running).in_set(crate::Step::Fire).before(crate::air::AirSystems));
    }
}

/// The seeker holds a target this long before the tone is steady and the tube will fire on it.
pub const LOCK: f32 = 1.0;
/// Below this the warhead has not armed; beyond the other the motor is spent.
pub const MIN_RANGE: f32 = 500.0;
pub const MAX_RANGE: f32 = 5200.0;
/// The seeker's field: a target within this angle of the ring is in it.
pub const SEEKER_COS: f32 = 0.99926; // 2.2°

#[derive(Resource)]
struct ManpadsRng(SmallRng);

/// An Igla team: its missiles, and how long until the next tube is ready.
#[derive(Component)]
pub struct Igla {
    pub missiles: u32,
    pub reload: f32,
}

/// A ZU-23-2: its bursts, and how long until the next.
#[derive(Component)]
pub struct Zu23 {
    pub bursts: u32,
    pub timer: f32,
}

/// An Igla in flight: what it is after (an aircraft, or a flare it took instead), how fast, how far it has come.
#[derive(Component)]
pub struct IglaMissile {
    target: Option<Entity>,
    decoy: Option<Vec3>,
    vel: Vec3,
    age: f32,
    travelled: f32,
    /// Where the target was a moment ago, for the lead.
    seen: Option<Vec3>,
    /// The flares, if the target puts them out: when.
    flares_at: Option<f32>,
    /// Whether the seeker has lost its target against the ground: a helicopter at treetop height is hard to hold
    /// against warm earth and trees. Decided once it is under way.
    lost: Option<bool>,
    from: &'static str,
}

/// Give a post its air defence.
pub fn arm(commands: &mut Commands, post: Entity, arm: AirArm, k: usize) {
    match arm {
        AirArm::Igla(n) => commands.entity(post).insert(Igla { missiles: n, reload: 5.0 + k as f32 * 3.0 }),
        AirArm::Zu23(n) => commands.entity(post).insert(Zu23 { bursts: n, timer: 2.0 + k as f32 }),
    };
}

/// Whether an aircraft is in the air for the gunners (not waiting off the map, not already falling).
fn flying(a: &Aircraft) -> bool {
    !matches!(a.flight, Flight::Waiting(_) | Flight::Falling)
}

/// The player's Igla: the growl while something hot is in the ring, the tone once it is held, and the shot.
#[allow(clippy::too_many_arguments)]
fn player_igla(
    mut commands: Commands,
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut view: ResMut<View>,
    mut fx: ResMut<FxQueue>,
    mut radio: ResMut<Radio>,
    mut sound: crate::sound::Sound,
    mut teams: Query<(&crate::fire::Post, &mut Igla)>,
    mut tone: Local<f32>,
) {
    if view.mode != Mode::Manpads {
        return;
    }
    let now = time.elapsed_secs_f64();
    let Some(team) = view.ground_team else { return };
    let Ok((post, mut igla)) = teams.get_mut(team) else { return };
    // The seeker's voice: a growl that quickens as it holds, steady once locked.
    if view.sight && let Some((_, held)) = view.lock {
        *tone -= time.delta_secs();
        if *tone <= 0.0 {
            sound.now(crate::sound::Sfx::Beep, if held >= LOCK { 0.35 } else { 0.2 });
            *tone = if held >= LOCK { 0.07 } else { 0.32 - 0.2 * (held / LOCK) };
        }
    }
    if !view.sight || !buttons.just_pressed(MouseButton::Left) || igla.missiles == 0 || igla.reload > 0.0 || post.strength < 2 {
        return;
    }
    igla.missiles -= 1;
    igla.reload = 12.0;
    let fwd = Quat::from_euler(EulerRot::YXZ, view.ground_yaw, view.ground_pitch, 0.0) * Vec3::NEG_Z;
    let from = view.ground_eye + fwd * 1.2;
    // Without the tone the missile goes where it is pointed and finds nothing.
    let target = view.lock.filter(|(_, t)| *t >= LOCK).map(|(e, _)| e);
    launch(&mut commands, &mut fx, from, fwd, target, "our Igla");
    sound.at(from, crate::sound::Sfx::AtgmLaunch, 1.2);
    radio.say2(now, "Пуск!", "Launch!");
    view.lock = None;
    info!("igla: the player fires, {} left, locked: {}, at {now:.0} s", igla.missiles, target.is_some());
}

fn launch(commands: &mut Commands, fx: &mut FxQueue, from: Vec3, dir: Vec3, target: Option<Entity>, by: &'static str) {
    fx.flash(from, 1.5e6, 30.0, 0.1);
    for k in 0..8 {
        fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: from - dir * k as f32 * 0.6, vel: -dir * 3.0 + Vec3::Y * 0.5, life: 3.0, size0: 0.6, size1: 3.0, alpha: 0.5, drag: 1.0, ..default() });
    }
    commands.spawn((
        Transform::from_translation(from).looking_to(dir, Vec3::Y),
        IglaMissile { target, decoy: None, vel: dir * 60.0, age: 0.0, travelled: 0.0, seen: None, flares_at: None, lost: None, from: by },
    ));
}

/// The teams the player is not holding fire by themselves at a helicopter in reach, now and then.
#[allow(clippy::type_complexity)]
fn auto_iglas(
    mut commands: Commands,
    time: Res<Time>,
    view: Res<View>,
    mut rng: ResMut<ManpadsRng>,
    mut fx: ResMut<FxQueue>,
    mut sound: crate::sound::Sound,
    mut teams: Query<(Entity, &crate::fire::Post, &mut Igla)>,
    aircraft: Query<(Entity, &Aircraft, &Transform)>,
) {
    let dt = crate::sim_dt(&time);
    for (e, post, mut igla) in &mut teams {
        igla.reload = (igla.reload - dt).max(0.0);
        let held = view.mode == Mode::Manpads && view.ground_team == Some(e);
        if held || igla.missiles == 0 || igla.reload > 0.0 || post.strength < 2 {
            continue;
        }
        let eye = geo::ground(post.pos) + Vec3::Y * 1.6;
        let mut in_reach: Vec<(Entity, Vec3)> = aircraft
            .iter()
            .filter(|(_, a, t)| flying(a) && (MIN_RANGE..3000.0).contains(&t.translation.distance(eye)) && geo::line_of_sight(post.pos, t.translation.xz()))
            .map(|(e, _, t)| (e, t.translation))
            .collect();
        // In a fixed order (by place, not by entity id, which is not the day's own).
        in_reach.sort_by(|a, b| a.1.x.total_cmp(&b.1.x).then(a.1.z.total_cmp(&b.1.z)));
        if in_reach.is_empty() || !rng.0.random_bool((0.12 * dt as f64).min(1.0)) {
            continue;
        }
        let (target, at) = in_reach[rng.0.random_range(0..in_reach.len())];
        igla.missiles -= 1;
        igla.reload = rng.0.random_range(25.0..45.0);
        let dir = (at - eye).normalize();
        launch(&mut commands, &mut fx, eye + dir, dir, Some(target), "an Igla team");
        sound.at(eye, crate::sound::Sfx::AtgmLaunch, 1.0);
    }
}

/// The ZU-23-2: a burst at the nearest aircraft within two kilometres, every few seconds while it has rounds.
#[allow(clippy::too_many_arguments)]
fn zu23(
    time: Res<Time>,
    mut rng: ResMut<ManpadsRng>,
    mut fx: ResMut<FxQueue>,
    mut sound: crate::sound::Sound,
    mut hits: MessageWriter<AirHit>,
    mut guns: Query<(&crate::fire::Post, &mut Zu23)>,
    aircraft: Query<(Entity, &Aircraft, &Transform)>,
) {
    let dt = crate::sim_dt(&time);
    for (post, mut gun) in &mut guns {
        gun.timer -= dt;
        if gun.timer > 0.0 || gun.bursts == 0 || post.strength < 2 {
            continue;
        }
        let at = geo::ground(post.pos) + Vec3::Y * 1.5;
        let Some((target, a, tp)) = aircraft
            .iter()
            .filter(|(_, a, t)| flying(a) && t.translation.distance(at) < 2000.0 && geo::line_of_sight(post.pos, t.translation.xz()))
            .min_by(|x, y| x.2.translation.distance(at).total_cmp(&y.2.translation.distance(at)).then(x.2.translation.x.total_cmp(&y.2.translation.x)))
            .map(|(e, a, t)| (e, a, t.translation))
        else {
            gun.timer = 1.0;
            continue;
        };
        gun.bursts -= 1;
        gun.timer = rng.0.random_range(2.5..4.0);
        let d = tp.distance(at);
        // Tracers, a stream of them up towards it, falling a little behind.
        let dir = (tp - at).normalize();
        for k in 0..10 {
            let miss = Vec3::new(rng.0.random_range(-1.0..1.0), rng.0.random_range(-1.0..1.0), rng.0.random_range(-1.0..1.0)) * d * 0.01;
            fx.spawn(fx::Spawn { kind: fx::Kind::Tracer, pos: at + dir * (k as f32 * 4.0), vel: (dir * d + miss * 3.0).normalize() * 950.0, life: (d / 950.0).min(2.5), size0: 0.4, size1: 0.4, alpha: 1.0, ..default() });
        }
        fx.flash(at, 4.0e5, 15.0, 0.08);
        sound.at(at, crate::sound::Sfx::Cannon30, 1.0);
        // Slow and low is easiest; on the ground, easier still.
        let slow = if matches!(a.flight, Flight::Landed { .. } | Flight::Approach) { 1.6 } else { 1.0 };
        let p = (0.2 * (1.0 - d / 2200.0) * slow).clamp(0.0, 0.9);
        if rng.0.random_bool(p as f64) {
            hits.write(AirHit { target, damage: rng.0.random_range(0.8..1.4), weapon: "ZU-23-2", from: "the base" });
        }
    }
}

/// The missiles: a boost to speed, then steering on the target's lead (or the flare it took instead), a hit inside
/// a few metres once armed, and out of motor after seventeen seconds.
#[allow(clippy::too_many_arguments)]
fn fly_iglas(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<ManpadsRng>,
    mut fx: ResMut<FxQueue>,
    mut hits: MessageWriter<AirHit>,
    mut q: Query<(Entity, &mut IglaMissile, &mut Transform), Without<Aircraft>>,
    aircraft: Query<(&Aircraft, &Transform)>,
) {
    let dt = crate::sim_dt(&time);
    for (e, mut m, mut tf) in &mut q {
        m.age += dt;
        let speed = (60.0 + m.age * 900.0).min(570.0);
        let p0 = tf.translation;
        // The target, if it is still in the air, and the seeker still has it.
        let mut tgt = m.target.and_then(|t| aircraft.get(t).ok()).filter(|(a, _)| a.flight != Flight::Falling).map(|(a, t)| (a.kind, t.translation));
        if m.lost.is_none()
            && m.age > 0.5
            && let Some((_, tp)) = tgt
        {
            let agl = tp.y - geo::height(tp.x, tp.z);
            m.lost = Some(rng.0.random_bool(if agl < 40.0 { 0.35 } else { 0.2 }));
        }
        if m.lost == Some(true) {
            tgt = None;
        }
        // The flares: put out once, a second after launch, when the kind does; the seeker may take one.
        if m.flares_at.is_none() && m.age > 0.05 {
            m.flares_at = Some(if let Some((kind, _)) = tgt { if rng.0.random_bool(kind.flares()) { 1.0 } else { f32::MAX } } else { f32::MAX });
        }
        if let (Some(at), Some((_, tp))) = (m.flares_at, tgt)
            && m.age >= at
            && m.decoy.is_none()
        {
            for k in 0..6 {
                fx.spawn(fx::Spawn { kind: fx::Kind::Spark, pos: tp + Vec3::Y * -2.0, vel: Vec3::new((k as f32 - 2.5) * 6.0, -4.0, rng.0.random_range(-5.0..5.0)), life: 3.0, size0: 1.6, size1: 0.6, alpha: 1.0, gravity: 3.0, ..default() });
            }
            if rng.0.random_bool(0.45) {
                m.decoy = Some(tp + Vec3::Y * -6.0);
            }
            m.flares_at = Some(f32::MAX);
        }
        let aim = match (m.decoy, tgt) {
            (Some(d), _) => Some(d),
            (None, Some((_, tp))) => {
                // Lead: where it will be by the time the missile gets there.
                let v = m.seen.map_or(Vec3::ZERO, |s| (tp - s) / dt.max(1e-3));
                m.seen = Some(tp);
                let t_go = tp.distance(p0) / speed;
                Some(tp + v * t_go.min(4.0))
            }
            _ => None,
        };
        let mut dir = m.vel.normalize_or(*tf.forward());
        if let Some(aim) = aim
            && m.age > 0.25
        {
            let want = (aim - p0).normalize_or(dir);
            let ang = dir.angle_between(want);
            let max = 0.9 * dt;
            if ang > 1e-4 {
                dir = dir.slerp(want, (max / ang).min(1.0)).normalize();
            }
        } else {
            dir = (dir - Vec3::Y * 0.01 * dt).normalize();
        }
        m.vel = dir * speed;
        let p1 = p0 + m.vel * dt;
        m.travelled += p0.distance(p1);
        tf.translation = p1;
        tf.look_to(dir, Vec3::Y);
        if m.age < 9.0 {
            fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: p0, vel: Vec3::Y * 0.3, life: 5.0, size0: 0.5, size1: 2.4, alpha: 0.45, drag: 1.0, ..default() });
        }
        // A hit: the target passes within a few metres of the segment flown, once the warhead has armed.
        if let (Some((_, tp)), None) = (tgt, m.decoy) {
            let seg = p1 - p0;
            let t = ((tp - p0).dot(seg) / seg.length_squared().max(1e-6)).clamp(0.0, 1.0);
            if (p0 + seg * t).distance(tp) < 4.0 && m.travelled > MIN_RANGE {
                fx.explosion(tp, 0.5, false);
                // The 1.2 kg warhead against a helicopter: often enough it comes down, often it only smokes and limps off.
                hits.write(AirHit { target: m.target.unwrap(), damage: rng.0.random_range(1.8..3.6), weapon: "Igla", from: m.from });
                commands.entity(e).despawn();
                continue;
            }
        }
        if m.age > 17.0 || p1.y < geo::height(p1.x, p1.z) {
            fx.explosion(p1, 0.3, p1.y < geo::height(p1.x, p1.z) + 1.0);
            commands.entity(e).despawn();
        }
    }
}

/// The aircraft in the seeker's ring, if any, from an eye looking along `fwd`: the one nearest the ring's centre,
/// in reach and in the air. (The camera holds it; held long enough, the tone is steady.)
pub fn in_ring(eye: Vec3, fwd: Vec3, aircraft: impl Iterator<Item = (Entity, Vec3, bool)>) -> Option<(Entity, f32)> {
    aircraft
        .filter(|(_, _, up)| *up)
        .map(|(e, p, _)| (e, (p - eye).normalize().dot(fwd), p.distance(eye)))
        .filter(|(_, c, d)| *c > SEEKER_COS && *d < MAX_RANGE)
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _, d)| (e, d))
}

/// For the camera and the HUD: whether an aircraft is up for the seeker.
pub fn up(a: &Aircraft) -> bool {
    flying(a)
}
