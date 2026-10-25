//! Fixed-wing sorties (design/avdiivka.md, phase 2): the Su-25's rocket run, as the "Grachi" flew it over Avdiivka's
//! fields (c11 §4: "Su-25s close in"). In low from the Russian side under the Igla's horizon, a pop-up a few
//! kilometres out, a shallow dive with a ripple of S-8 rockets on the target, a hard break away with flares, and
//! home low the way it came. It never hovers or circles: a pass is one run, and a second is another sortie.
//!
//! Anything that shoots at aircraft shoots at it (`manpads`), and a hit goes the helicopters' way (`air::take_hits`):
//! a hit Su-25 smokes and turns for home, a killed one falls and burns.

use crate::air::{AirRng, AirTally, Aircraft, Flight};
use crate::fx::{self, FxQueue};
use crate::geo;
use crate::units::{Radio, Side};
use bevy::prelude::*;
use rand::Rng;

/// Where a Su-25 is in its attack (`Flight::Attack`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pass {
    /// Nose up and climbing, still running at the target, to see it.
    PopUp,
    /// Rolled in, nose on the target, coming down a shallow slope to rocket range.
    Dive,
    /// Rockets gone: pulling round hard for home, flares going.
    Break,
}

/// A fixed-wing aircraft's own state beside its `Aircraft`: the target, the pass, the attitude it flies in.
#[derive(Component)]
pub struct Jet {
    pub pass: Pass,
    pub target: Vec2,
    /// The flight path's climb (+) or dive (-), radians; the bank into a turn.
    pitch: f32,
    roll: f32,
    /// S-8s left in the pods.
    pub rockets: u32,
    /// Flares left in the break, and seconds to the next pair.
    flares: (u32, f32),
}

impl Jet {
    pub fn new(target: Vec2) -> Self {
        Self { pass: Pass::PopUp, target, pitch: 0.0, roll: 0.0, rockets: ROCKETS, flares: (0, 0.0) }
    }
}

/// One pass's ripple: two B-8 pods' worth, the other two kept for the way home or the next sortie.
const ROCKETS: u32 = 20;
/// Ground distance from the target at which it pops up, and the slant range it fires at.
const POP_AT: f32 = 4200.0;
const FIRE_AT: f32 = 1900.0;
/// Height over the ground it runs in at, and leaves at.
const LOW: f32 = 45.0;

/// An S-8 in flight, for its smoke: from the pod to where it lands, over these game times. (The burst is the
/// `fire::Shell` it was fired with.)
#[derive(Component)]
pub struct RocketTrail {
    from: Vec3,
    to: Vec3,
    t0: f64,
    t1: f64,
    /// How far along the smoke has been laid, metres.
    laid: f32,
}

/// The compass point a direction comes from, as the net says it (English, Ukrainian).
fn from_where(dir: Vec2) -> (&'static str, &'static str) {
    let (e, n) = geo::east_north(dir.x as f64, dir.y as f64);
    let b = (e.atan2(n).to_degrees() + 360.0 + 22.5) % 360.0;
    [
        ("the north", "з півночі"),
        ("the north-east", "з північного сходу"),
        ("the east", "зі сходу"),
        ("the south-east", "з південного сходу"),
        ("the south", "з півдня"),
        ("the south-west", "з південного заходу"),
        ("the west", "із заходу"),
        ("the north-west", "з північного заходу"),
    ][(b / 45.0) as usize % 8]
}

/// What the rockets went into, as the feed names it: the nearest of our positions, or open ground.
fn target_name(at: Vec2) -> &'static str {
    let near = bav_core::field::field().positions.iter().filter(|p| p.at.distance(at) < 90.0).min_by(|a, b| a.at.distance(at).total_cmp(&b.at.distance(at)));
    match near.map(|p| p.kind) {
        Some(bav_core::field::PositionKind::Trench) => "the trench",
        Some(bav_core::field::PositionKind::Treeline) => "the tree line",
        Some(bav_core::field::PositionKind::Ruin) => "the ruins",
        Some(bav_core::field::PositionKind::Hide) => "a hide",
        None => "open ground",
    }
}

/// Turn `yaw` towards `want` by at most `rate * dt`; the turn it made, per second.
fn turn(yaw: &mut f32, want: f32, rate: f32, dt: f32) -> f32 {
    let d = (want - *yaw + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
    let step = d.clamp(-rate * dt, rate * dt);
    *yaw += step;
    step / dt.max(1e-4)
}

/// The flying of a fixed-wing pass: in low along the route, the pop-up, the dive and the ripple, the break with
/// flares, and out low.
#[allow(clippy::too_many_arguments)]
pub(crate) fn fly_jets(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<AirRng>,
    mut tally: ResMut<AirTally>,
    mut radio: ResMut<Radio>,
    mut fx: ResMut<FxQueue>,
    mut log: ResMut<crate::hits::HitLog>,
    mut q: Query<(Entity, &mut Aircraft, &mut Jet, &mut Transform, &mut Visibility)>,
    mut said: Local<f64>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    for (e, mut a, mut j, mut tf, mut vis) in &mut q {
        if a.flight == Flight::Falling {
            continue;
        }
        let mut p = tf.translation;
        let here = p.xz();
        let fwd = tf.rotation * Vec3::NEG_Z;
        // What it wants this moment: a point to turn to, a height over the ground (or a climb or dive of its own), a
        // speed, and how hard it turns.
        let (mut aim, mut agl, mut climb, mut speed, mut rate) = (None::<Vec2>, LOW, None::<f32>, a.kind.cruise(), 0.12);
        match a.flight {
            Flight::Waiting(left) => {
                a.flight = if left - dt <= 0.0 {
                    tally.flown += 1;
                    *vis = Visibility::Inherited;
                    info!("air: a Su-25 in from {:.0},{:.0} on {:.0},{:.0}, at {now:.0} s", here.x, here.y, j.target.x, j.target.y);
                    Flight::Inbound
                } else {
                    Flight::Waiting(left - dt)
                };
                continue;
            }
            Flight::Inbound => {
                let to = a.route[a.leg];
                aim = Some(to);
                if a.leg + 1 < a.route.len() && here.distance(to) < 400.0 {
                    a.leg += 1;
                }
                if a.leg + 1 == a.route.len() && here.distance(j.target) < POP_AT {
                    a.flight = Flight::Attack;
                    j.pass = Pass::PopUp;
                    // Once for a pair: the net hears the first of them.
                    if now - *said > 30.0 {
                        *said = now;
                        let (en, uk) = from_where(here - j.target);
                        radio.say2(now, format!("Грачі! Су-25, низько, {uk}!"), format!("Su-25s, low, from {en}!"));
                    }
                }
            }
            Flight::Attack => match j.pass {
                Pass::PopUp => {
                    aim = Some(j.target);
                    climb = Some(0.42);
                    speed = 170.0;
                    if p.y - geo::height(p.x, p.z) > 620.0 || here.distance(j.target) < 2600.0 {
                        j.pass = Pass::Dive;
                    }
                }
                Pass::Dive => {
                    aim = Some(j.target);
                    rate = 0.25;
                    speed = 215.0;
                    let t = geo::ground(j.target);
                    let flat = here.distance(j.target).max(1.0);
                    climb = Some(((t.y - p.y) / flat).atan().clamp(-0.5, 0.0));
                    let slant = p.distance(t);
                    let on = fwd.xz().normalize_or_zero().dot((j.target - here).normalize_or_zero()) > 0.9;
                    if slant < FIRE_AT && on && j.rockets > 0 {
                        ripple(&mut commands, &mut fx, rng, &tf, j.target, j.rockets, a.speed, now);
                        j.rockets = 0;
                        log.note(now, "S-8 rockets", "a Su-25", target_name(j.target), Side::Ua, "a rocket run");
                        if rng.random_bool(0.4) {
                            radio.say2(now, format!("НУРСи по {}! Лягай!", if target_name(j.target) == "the trench" { "окопу" } else { "позиції" }), format!("Rockets on {}! Get down!", target_name(j.target)));
                        }
                        info!("air: a Su-25's rockets on {:.0},{:.0} from {slant:.0} m, at {now:.0} s", j.target.x, j.target.y);
                    }
                    if j.rockets == 0 || flat < 900.0 {
                        j.pass = Pass::Break;
                        j.flares = (8, 0.0);
                        // It breaks the way home is nearer; straight behind it, the way it was given.
                        let s = fwd.xz().normalize_or_zero().perp_dot((a.goal - here).normalize_or_zero());
                        if s.abs() > 0.2 {
                            a.side = s.signum();
                        }
                    }
                }
                Pass::Break => {
                    // Round hard on the side it was given, climbing a little, until its nose is on the way home.
                    climb = Some(0.12);
                    speed = 200.0;
                    let home = a.goal - here;
                    aim = Some(here + Vec2::from_angle(a.side * 1.7).rotate(fwd.xz().normalize_or(Vec2::Y)) * 1000.0);
                    rate = 0.32;
                    if fwd.xz().normalize_or_zero().dot(home.normalize_or_zero()) > 0.9 {
                        a.flight = Flight::Outbound;
                    }
                }
            },
            Flight::Outbound => {
                aim = Some(a.goal);
                agl = LOW + 15.0;
                speed = 210.0;
                if here.distance(a.goal) < 500.0 || here.length() > 9000.0 {
                    commands.entity(e).despawn();
                    continue;
                }
            }
            _ => {}
        }
        // Flares in the break, a pair every half second from the tail, falling and burning out.
        if j.flares.0 > 0 {
            j.flares.1 -= dt;
            if j.flares.1 <= 0.0 {
                j.flares = (j.flares.0 - 1, 0.45);
                let right = tf.rotation * Vec3::X;
                for s in [-1.0, 1.0] {
                    let at = p - fwd * 6.0 + Vec3::Y * 0.5;
                    let vel = a.velocity(&tf) * 0.3 + right * s * 18.0 + Vec3::Y * 6.0;
                    fx.spawn(fx::Spawn { kind: fx::Kind::Spark, pos: at, vel, life: 3.5, size0: 2.2, size1: 0.8, alpha: 1.0, drag: 0.6, gravity: 4.0, streak: 2.0, ..default() });
                    fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: at, vel: vel * 0.5, life: 5.0, size0: 0.8, size1: 4.0, alpha: 0.45, drag: 0.8, ..default() });
                }
            }
        }
        // Hit: off the target and home, the way it came, low.
        if a.damaged && a.flight != Flight::Outbound {
            a.flight = Flight::Outbound;
            tally.driven_off += 1;
            radio.say2(now, "Є влучання! Грач димить, тягне додому.", "Hit him! The Su-25 is smoking, dragging home.");
        }
        let yaw_rate = aim.map_or(0.0, |to| {
            let want = crate::units::vehicle_yaw(to - here);
            let mut yaw = a.yaw;
            let r = turn(&mut yaw, want, rate, dt);
            a.yaw = yaw;
            r
        });
        // Its own climb or dive, or holding its height over the ground ahead.
        let ahead = p + fwd * 400.0;
        let floor = geo::height(p.x, p.z).max(geo::height(ahead.x, ahead.z));
        let want_pitch = climb.unwrap_or_else(|| ((floor + agl - p.y) * 0.012).clamp(-0.18, 0.25));
        j.pitch += (want_pitch - j.pitch).clamp(-0.6 * dt, 0.6 * dt);
        // Banked into the turn: a left turn (yaw growing) lifts the right wing.
        let want_roll = (yaw_rate * 3.2).clamp(-1.3, 1.3);
        j.roll += (want_roll - j.roll).clamp(-2.0 * dt, 2.0 * dt);
        a.speed += (speed - a.speed).clamp(-8.0 * dt, 8.0 * dt);
        let along = Quat::from_rotation_y(a.yaw) * Quat::from_rotation_x(j.pitch);
        p += along * Vec3::NEG_Z * a.speed * dt;
        p.y = p.y.max(geo::height(p.x, p.z) + 15.0);
        tf.translation = p;
        tf.rotation = along * Quat::from_rotation_z(j.roll);
    }
}

/// The ripple: the rockets leave the pods a beat apart, and each lands in a beaten zone long along the line of
/// flight (unguided, from a diving aircraft at two kilometres) as an S-8's burst (`fire::ShellKind::Rocket`).
#[allow(clippy::too_many_arguments)]
fn ripple(commands: &mut Commands, fx: &mut FxQueue, rng: &mut rand::rngs::SmallRng, tf: &Transform, target: Vec2, n: u32, speed: f32, now: f64) {
    let fwd = tf.rotation * Vec3::NEG_Z;
    let right = tf.rotation * Vec3::X;
    let along = fwd.xz().normalize_or(Vec2::NEG_Y);
    let across = along.perp();
    fx.flash(tf.translation + fwd * 4.0, 3.0e6, 60.0, 0.25);
    fx.sound(tf.translation, crate::sound::Sfx::Ripple, 1.8);
    for k in 0..n {
        let t0 = now + k as f64 * 0.06;
        let pod = if k % 2 == 0 { 1.6 } else { -1.6 };
        let from = tf.translation + fwd * (3.0 + speed * k as f32 * 0.06) + right * pod - Vec3::Y * 0.8;
        let at = target + along * rng.random_range(-60.0..45.0) + across * rng.random_range(-28.0..28.0);
        let to = geo::ground(at);
        let t1 = t0 + (from.distance(to) / 620.0) as f64;
        commands.spawn((crate::fire::Shell { at, kind: crate::fire::ShellKind::Rocket, land: t1, by: Side::Ru }, crate::fire::Whistled));
        commands.spawn(RocketTrail { from, to, t0, t1, laid: 0.0 });
    }
}

/// The rockets' smoke: a puff every twenty-five metres along each one's line as it flies, and a spark at its head.
pub(crate) fn rocket_trails(mut commands: Commands, time: Res<Time>, mut fx: ResMut<FxQueue>, mut q: Query<(Entity, &mut RocketTrail)>) {
    let now = time.elapsed_secs_f64();
    for (e, mut r) in &mut q {
        if now < r.t0 {
            continue;
        }
        let u = (((now - r.t0) / (r.t1 - r.t0)) as f32).min(1.0);
        let len = r.from.distance(r.to);
        let dir = (r.to - r.from) / len.max(1.0);
        while r.laid < u * len {
            fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: r.from + dir * r.laid, vel: Vec3::Y * 0.3, life: 4.0, size0: 0.7, size1: 3.0, alpha: 0.4, drag: 1.0, ..default() });
            r.laid += 25.0;
        }
        fx.spawn(fx::Spawn { kind: fx::Kind::Fire, pos: r.from + dir * u * len, vel: dir * 20.0, life: 0.06, size0: 0.9, size1: 0.4, alpha: 1.0, ..default() });
        if u >= 1.0 {
            commands.entity(e).despawn();
        }
    }
}
