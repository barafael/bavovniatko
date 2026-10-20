//! Aircraft over the field (design/hostomel.md, phase 2): the helicopter stream that brings the VDV to Hostomel,
//! and the transports behind it. A sortie flies in low along its route; a transport (Mi-8) slows, flares, settles on
//! its landing zone and puts its men out over the ramp, then lifts and goes; a gunship (Ka-52, Mi-24) circles the
//! field and dives on what it can see of ours with rockets and its gun, until it has fired them off. Anything can be
//! hit (`AirHit`): a hit aircraft smokes and turns for home, a killed one falls, and burns where it lands, a wreck on
//! the runway as much as any crater.
//!
//! Only a diorama whose script flies sorties (`Act::Air`) has any of this in its world: the Record of the others
//! must not change. Robotyne's Ka-52s on station beyond reach are `robotyne`'s own.

use crate::fx::{self, Emitter, EmitterKind, FxQueue};
use crate::geo;
use crate::units::{Models, Palette, Part, Radio, SState, Side, Soldier, Stats};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use serde::Deserialize;

pub struct AirPlugin;

impl Plugin for AirPlugin {
    fn build(&self, app: &mut App) {
        if !flies() {
            return;
        }
        app.insert_resource(AirRng(SmallRng::seed_from_u64(crate::dev::seeded(2402))))
            .init_resource::<AirTally>()
            .add_message::<AirHit>()
            .add_systems(Update, (fly, fight, take_hits, fall).chain().run_if(crate::running).in_set(crate::Step::Fire).in_set(AirSystems).after(crate::director::DirectorSystems));
    }
}

/// The aircraft's systems, for what shoots at them to run before.
#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AirSystems;

/// Whether the day's script flies anything.
pub fn flies() -> bool {
    crate::data::sim_data().script.iter().any(|(_, a)| matches!(a, crate::director::Act::Air(_)))
}

/// What flies.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AirKind {
    /// The Ka-52 "Alligator": the stream's lead gunships, armoured, two crew side by side.
    Ka52,
    /// The Mi-24/35: gunships, and a cabin for eight.
    Mi24,
    /// The Mi-8AMTSh: the transports, sixteen paratroopers each.
    Mi8,
    /// The Il-76MD: the airlift that was to land on the runway once it was taken.
    Il76,
}

impl AirKind {
    /// How much it takes: a 23 mm burst on a Mi-8 is one, an Igla's warhead two.
    pub fn hp(self) -> f32 {
        match self {
            AirKind::Ka52 => 3.0,
            AirKind::Mi24 => 3.0,
            AirKind::Mi8 => 2.0,
            AirKind::Il76 => 5.0,
        }
    }

    /// Cruise over the field, m/s.
    fn cruise(self) -> f32 {
        match self {
            AirKind::Ka52 | AirKind::Mi24 => 60.0,
            AirKind::Mi8 => 55.0,
            AirKind::Il76 => 85.0,
        }
    }

    /// Height above the ground it flies at: the stream came in at treetop height under the cloud.
    fn agl(self) -> f32 {
        match self {
            AirKind::Il76 => 350.0,
            AirKind::Ka52 | AirKind::Mi24 => 45.0,
            AirKind::Mi8 => 30.0,
        }
    }

    pub fn gunship(self) -> bool {
        matches!(self, AirKind::Ka52 | AirKind::Mi24)
    }

    /// Men it brings.
    pub fn troops(self) -> u32 {
        match self {
            AirKind::Mi8 => 16,
            _ => 0,
        }
    }

    /// Rocket salvos it carries (S-8, eight to a salvo).
    fn salvos(self) -> u32 {
        match self {
            AirKind::Ka52 => 3,
            AirKind::Mi24 => 3,
            _ => 0,
        }
    }

    /// The chance it puts out flares when a missile comes for it (RUSI: the Ka-52s often did not).
    pub fn flares(self) -> f64 {
        match self {
            AirKind::Ka52 => 0.25,
            AirKind::Mi24 => 0.45,
            AirKind::Mi8 => 0.5,
            AirKind::Il76 => 0.8,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            AirKind::Ka52 => "Ka-52",
            AirKind::Mi24 => "Mi-24",
            AirKind::Mi8 => "Mi-8",
            AirKind::Il76 => "Il-76",
        }
    }

    /// The baked model's id (tools/bake).
    pub fn model(self) -> &'static str {
        match self {
            AirKind::Ka52 => "ka52",
            AirKind::Mi24 => "mi24",
            AirKind::Mi8 => "mi8",
            AirKind::Il76 => "il76",
        }
    }
}

/// One aircraft of a script's `Act::Air`: what, from where (beyond the box), by which points, and, for a transport,
/// where it lands and where its men go; for a gunship, the middle of the field it works over.
#[derive(Deserialize, Debug, Clone)]
pub struct Sortie {
    pub kind: AirKind,
    pub from: (f32, f32),
    #[serde(default)]
    pub via: Vec<(f32, f32)>,
    /// A transport's landing zone; a gunship's orbit centre.
    pub at: (f32, f32),
    /// Where a transport's men go once out of it.
    #[serde(default)]
    pub goal: Option<(f32, f32)>,
    /// Seconds after the act's time it takes off from `from` (so a wave strings out).
    #[serde(default)]
    pub delay: f32,
}

/// The models, by kind, as the look loads them: the body, the rotor and where it turns.
#[derive(Resource)]
pub struct AirModels {
    pub kinds: Vec<(AirKind, Vec<Part>, Vec<Part>, Vec3)>,
}

impl AirModels {
    fn get(&self, k: AirKind) -> Option<&(AirKind, Vec<Part>, Vec<Part>, Vec3)> {
        self.kinds.iter().find(|m| m.0 == k)
    }
}

#[derive(Resource)]
struct AirRng(SmallRng);

/// The day's count of what flew and what fell.
#[derive(Resource, Default, Debug)]
pub struct AirTally {
    pub flown: u32,
    pub down: Vec<(AirKind, &'static str)>,
    /// Turned for home hit, or by the fire around them.
    pub driven_off: u32,
    pub landed: u32,
    pub men_landed: u32,
}

/// What an aircraft is doing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flight {
    /// Waiting for its delay at the start of its route.
    Waiting(f32),
    /// Along its route to the field.
    Inbound,
    /// A transport slowing onto its landing zone.
    Approach,
    /// On the ground, putting its men out (one every so often), then waiting to lift.
    Landed { left: f32, out: f32 },
    /// Going home, the way it came; out of the box and gone.
    Outbound,
    /// A gunship circling the field, between runs.
    Orbit { until: f32 },
    /// Diving on a target: fires at `fire_at` metres.
    Run { target: Vec2 },
    /// Hit hard enough: falling.
    Falling,
}

#[derive(Component)]
pub struct Aircraft {
    pub kind: AirKind,
    pub hp: f32,
    pub flight: Flight,
    route: Vec<Vec2>,
    leg: usize,
    speed: f32,
    yaw: f32,
    lz: Vec2,
    goal: Vec2,
    troops: u32,
    salvos: u32,
    /// Falling: the velocity it has, and its spin.
    vel: Vec3,
    spin: f32,
    /// Hit at all: smoking, and it will not stay.
    pub damaged: bool,
    /// Which way round it orbits, and where on the circle it is.
    side: f32,
    angle: f32,
}

/// A rotor: spun by the look.
#[derive(Component)]
pub struct AirRotor;

/// What is left of an aircraft on the ground: an obstacle on the runway, and cover.
#[derive(Component)]
pub struct AirWreck {
    pub kind: AirKind,
    pub at: Vec2,
}

/// Something hit an aircraft: how hard, with what, by whom.
#[derive(Message, Clone, Copy)]
pub struct AirHit {
    pub target: Entity,
    pub damage: f32,
    pub weapon: &'static str,
    pub from: &'static str,
}

/// Put a sortie in the air (the director, on its act's time).
pub fn launch(commands: &mut Commands, models: &AirModels, s: &Sortie) {
    let Some((_, body, rotor, rotor_at)) = models.get(s.kind) else {
        warn!("air: no model for the {}", s.kind.name());
        return;
    };
    let from = Vec2::new(s.from.0, s.from.1);
    let at = Vec2::new(s.at.0, s.at.1);
    let mut route: Vec<Vec2> = s.via.iter().map(|&(x, z)| Vec2::new(x, z)).collect();
    route.push(at);
    let yaw = crate::units::vehicle_yaw(route[0] - from);
    let p = Vec3::new(from.x, geo::height(from.x, from.y).max(150.0) + s.kind.agl(), from.y);
    let mut e = commands.spawn((
        crate::units::skin(body),
        Transform::from_translation(p).with_rotation(Quat::from_rotation_y(yaw)),
        Aircraft {
            kind: s.kind,
            hp: s.kind.hp(),
            flight: if s.delay > 0.0 { Flight::Waiting(s.delay) } else { Flight::Inbound },
            route,
            leg: 0,
            speed: s.kind.cruise(),
            yaw,
            lz: at,
            goal: s.goal.map_or(at, |(x, z)| Vec2::new(x, z)),
            troops: s.kind.troops(),
            salvos: s.kind.salvos(),
            vel: Vec3::ZERO,
            spin: 0.0,
            damaged: false,
            side: if (from.x * 0.37).sin() > 0.0 { 1.0 } else { -1.0 },
            angle: 0.0,
        },
        Visibility::Hidden,
    ));
    if !rotor.is_empty() {
        let rotor = rotor.clone();
        let at = *rotor_at;
        e.with_children(|c| crate::units::dress(&mut c.spawn((crate::units::skin(&rotor), Transform::from_translation(at), AirRotor)), &rotor));
    }
    crate::units::dress(&mut e, body);
}

/// Turn `yaw` towards `want` by at most `rate * dt`.
fn turn(yaw: f32, want: f32, rate: f32, dt: f32) -> f32 {
    let d = (want - yaw + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
    yaw + d.clamp(-rate * dt, rate * dt)
}

/// The flying: along the route, onto the landing zone and off it, round the field, and home.
#[allow(clippy::too_many_arguments)]
fn fly(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<AirRng>,
    mut tally: ResMut<AirTally>,
    mut stats: ResMut<Stats>,
    mut radio: ResMut<Radio>,
    models: Res<Models>,
    palette: Res<Palette>,
    mut q: Query<(Entity, &mut Aircraft, &mut Transform, &mut Visibility)>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    for (e, mut a, mut tf, mut vis) in &mut q {
        if a.flight == Flight::Falling {
            continue;
        }
        let mut p = tf.translation;
        let here = p.xz();
        let (mut want_speed, mut want_agl, mut aim) = (a.kind.cruise(), a.kind.agl(), None::<Vec2>);
        match a.flight {
            Flight::Waiting(left) => {
                a.flight = if left - dt <= 0.0 {
                    tally.flown += 1;
                    *vis = Visibility::Inherited;
                    Flight::Inbound
                } else {
                    Flight::Waiting(left - dt)
                };
                continue;
            }
            Flight::Inbound => {
                let to = a.route[a.leg];
                aim = Some(to);
                let last = a.leg + 1 == a.route.len();
                if a.kind.gunship() && last && here.distance(to) < 900.0 {
                    a.angle = (here - to).to_angle();
                    a.flight = Flight::Orbit { until: rng.0.random_range(8.0..16.0) };
                } else if !a.kind.gunship() && last && here.distance(to) < 600.0 {
                    a.flight = Flight::Approach;
                } else if !last && here.distance(to) < 120.0 {
                    a.leg += 1;
                }
            }
            Flight::Approach => {
                // Slow with the distance left, down to the ground over the last hundred metres.
                let d = here.distance(a.lz);
                aim = Some(a.lz);
                want_speed = (d * 0.12).clamp(1.5, a.kind.cruise());
                want_agl = (d * 0.25).clamp(0.0, a.kind.agl());
                if d < 6.0 {
                    a.flight = Flight::Landed { left: 22.0 + rng.0.random_range(0.0..8.0), out: 1.5 };
                    tally.landed += 1;
                }
            }
            Flight::Landed { left, out } => {
                want_speed = 0.0;
                want_agl = 0.0;
                let mut out = out - dt;
                if out <= 0.0 && a.troops > 0 {
                    // One man down the ramp, out to the side, then on to the goal.
                    a.troops -= 1;
                    out = 0.7;
                    let back = (Quat::from_rotation_y(a.yaw) * Vec3::Z).xz() * 9.0;
                    let at = here + back + Vec2::new(rng.0.random_range(-3.0..3.0), rng.0.random_range(-3.0..3.0));
                    let goal = a.goal + Vec2::new(rng.0.random_range(-40.0..40.0), rng.0.random_range(-40.0..40.0));
                    crate::units::spawn_soldier(&mut commands, &models, &palette, Side::Ru, at, goal, a.lz, 1.0, None, &mut rng.0);
                    stats.sent += 1;
                    tally.men_landed += 1;
                }
                let left = left - dt;
                a.flight = if left <= 0.0 && a.troops == 0 { Flight::Outbound } else { Flight::Landed { left, out } };
            }
            Flight::Outbound => {
                let home = a.route.first().copied().unwrap_or(Vec2::ZERO);
                let away = (here - a.lz).normalize_or(Vec2::Y);
                aim = Some(if a.route.len() > 1 { home } else { here + away * 1000.0 });
                if here.length() > 7000.0 || (a.route.len() > 1 && here.distance(home) < 150.0) {
                    commands.entity(e).despawn();
                    continue;
                }
                // Climbing away from the field before it turns.
                want_agl = a.kind.agl().max(40.0);
            }
            Flight::Orbit { until } => {
                // A wide circle round the field, low.
                a.angle += a.side * dt * want_speed / 750.0;
                aim = Some(a.lz + Vec2::from_angle(a.angle + a.side * 0.4) * 750.0);
                a.flight = Flight::Orbit { until: until - dt };
                // Out of rockets, home; else `fight` picks the next run's target.
                if until - dt <= 0.0 && (a.salvos == 0 || a.damaged) {
                    a.flight = Flight::Outbound;
                }
            }
            Flight::Run { target } => {
                aim = Some(target);
                want_agl = 60.0;
            }
            Flight::Falling => unreachable!(),
        }
        if let Some(to) = aim {
            let want = crate::units::vehicle_yaw(to - here);
            a.yaw = turn(a.yaw, want, 0.5, dt);
        }
        a.speed += (want_speed - a.speed).clamp(-6.0 * dt, 4.0 * dt);
        // A model faces -Z (tools/bake): forward is the yaw's turn of that.
        let rot = Quat::from_rotation_y(a.yaw);
        p += rot * Vec3::NEG_Z * a.speed * dt;
        let ground = geo::height(p.x, p.z);
        let target_y = ground + want_agl;
        p.y += (target_y - p.y) * (1.0 - (-dt * 0.9).exp());
        if matches!(a.flight, Flight::Landed { .. }) {
            p.y = ground + 0.1;
        }
        // Nose down as it speeds up, up as it flares.
        let pitch = -(a.speed / a.kind.cruise()).clamp(0.0, 1.0) * 0.12 + if a.flight == Flight::Approach { 0.08 } else { 0.0 };
        tf.translation = p;
        tf.rotation = rot * Quat::from_rotation_x(pitch);
        if a.damaged && a.flight != Flight::Outbound && !matches!(a.flight, Flight::Landed { .. }) {
            a.flight = Flight::Outbound;
            tally.driven_off += 1;
            radio.say(now, "It's hit! Smoking, turning away.");
        }
    }
}

/// The gunships' runs: from the orbit onto something of ours they can see, rockets at a kilometre, the gun closer,
/// and away.
#[allow(clippy::too_many_arguments)]
fn fight(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<AirRng>,
    mut fx: ResMut<FxQueue>,
    mut radio: ResMut<Radio>,
    spotted: Option<Res<crate::staff::Spotted>>,
    posts: Query<&crate::fire::Post>,
    soldiers: Query<(&Soldier, &Transform), Without<Aircraft>>,
    mut q: Query<(&mut Aircraft, &Transform)>,
) {
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    for (mut a, tf) in &mut q {
        let here = tf.translation.xz();
        match a.flight {
            Flight::Orbit { until } if until <= 0.0 && a.salvos > 0 && !a.damaged => {
                // Command's staff: what they have spotted; the Record: our posts as they are, and men in the open.
                let mut targets: Vec<Vec2> = match spotted.as_ref().filter(|_| crate::staff::rule("air")) {
                    Some(s) => s.positions(),
                    None => posts.iter().filter(|p| p.side == Side::Ua && p.strength > 0).map(|p| p.pos).collect(),
                };
                targets.extend(soldiers.iter().filter(|(s, _)| s.side == Side::Ua && s.state != SState::DugIn).map(|(_, t)| t.translation.xz()));
                if targets.is_empty() {
                    a.flight = Flight::Orbit { until: rng.random_range(10.0..20.0) };
                    continue;
                }
                let target = targets[rng.random_range(0..targets.len())];
                a.flight = Flight::Run { target };
            }
            Flight::Run { target } => {
                let d = here.distance(target);
                if d < 1100.0 && a.salvos > 0 {
                    a.salvos -= 1;
                    // A salvo of eight: they land over a few seconds, spread along the line of flight.
                    let fwd = tf.rotation * Vec3::NEG_Z;
                    fx.flash(tf.translation + fwd * 3.0, 3.0e6, 60.0, 0.2);
                    fx.sound(tf.translation, crate::sound::Sfx::Ripple, 1.4);
                    for k in 0..8 {
                        // Unguided from a moving helicopter at a kilometre: a beaten zone, not a point.
                        let off = Vec2::new(rng.random_range(-45.0..45.0), rng.random_range(-45.0..45.0)) + fwd.xz().normalize_or_zero() * rng.random_range(-60.0..60.0);
                        commands.spawn((crate::fire::Shell { at: target + off, kind: crate::fire::ShellKind::Rocket, land: now + d as f64 / 600.0 + k as f64 * 0.12, by: Side::Ru }, crate::fire::Whistled));
                        fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: tf.translation + fwd * (6.0 + k as f32), vel: -fwd * 4.0, life: 3.0, size0: 1.0, size1: 4.0, alpha: 0.4, drag: 1.0, ..default() });
                    }
                    if rng.random_bool(0.4) {
                        radio.say(now, "Helicopter rockets on the apron! Get down!");
                    }
                    // Break away and back to the circle.
                    a.flight = Flight::Orbit { until: rng.random_range(18.0..32.0) };
                } else if d < 1100.0 {
                    a.flight = Flight::Orbit { until: 5.0 };
                }
            }
            _ => {}
        }
    }
}

/// Hits: a hit aircraft smokes and turns for home; one hit past what it takes falls.
fn take_hits(
    mut commands: Commands,
    time: Res<Time>,
    mut hits: MessageReader<AirHit>,
    mut tally: ResMut<AirTally>,
    mut log: ResMut<crate::hits::HitLog>,
    mut radio: ResMut<Radio>,
    mut rng: ResMut<AirRng>,
    mut q: Query<(&mut Aircraft, &Transform)>,
) {
    let now = time.elapsed_secs_f64();
    for h in hits.read() {
        let Ok((mut a, tf)) = q.get_mut(h.target) else { continue };
        if a.flight == Flight::Falling {
            continue;
        }
        a.hp -= h.damage;
        if a.hp <= 0.0 {
            a.flight = Flight::Falling;
            a.vel = tf.rotation * Vec3::NEG_Z * a.speed;
            a.spin = rng.0.random_range(0.8..2.2) * if rng.0.random_bool(0.5) { 1.0 } else { -1.0 };
            commands.entity(h.target).insert(Emitter::new(EmitterKind::WreckFire, 30.0, 60.0, Vec3::new(0.0, 1.0, 1.0)));
            tally.down.push((a.kind, h.weapon));
            log.note(now, h.weapon, h.from, a.kind.name(), Side::Ru, "shot down");
            radio.say2(now, format!("Мінус! {} падає!", a.kind.name()), format!("Got it! The {} is going down!", a.kind.name()));
            info!("air: a {} shot down by {} at {:.0},{:.0}, at {now:.0} s", a.kind.name(), h.weapon, tf.translation.x, tf.translation.z);
        } else if !a.damaged {
            a.damaged = true;
            commands.entity(h.target).insert(Emitter::new(EmitterKind::EngineSmoke, 8.0, 300.0, Vec3::new(0.0, 1.5, 1.5)));
            log.note(now, h.weapon, h.from, a.kind.name(), Side::Ru, "hit, smoking");
        }
    }
}

/// A falling aircraft: on its way down, turning; where it hits, it burns.
fn fall(mut commands: Commands, time: Res<Time>, mut fx: ResMut<FxQueue>, mut q: Query<(Entity, &mut Aircraft, &mut Transform)>) {
    let dt = crate::sim_dt(&time);
    for (e, mut a, mut tf) in &mut q {
        if a.flight != Flight::Falling {
            continue;
        }
        a.vel.y -= 9.8 * dt;
        a.vel.x *= 1.0 - 0.3 * dt;
        a.vel.z *= 1.0 - 0.3 * dt;
        tf.translation += a.vel * dt;
        tf.rotate_y(a.spin * dt);
        tf.rotate_local_z(a.spin * 0.2 * dt);
        let ground = geo::height(tf.translation.x, tf.translation.z);
        if tf.translation.y <= ground + 0.5 {
            let at = Vec3::new(tf.translation.x, ground, tf.translation.z);
            fx.explosion(at, 1.4, true);
            fx.sound(at, crate::sound::Sfx::BlastNear, 1.6);
            tf.translation = at + Vec3::Y * 0.4;
            let (yaw, _, _) = tf.rotation.to_euler(EulerRot::YXZ);
            tf.rotation = Quat::from_rotation_y(yaw) * Quat::from_rotation_z(0.35);
            commands.entity(e).remove::<Aircraft>().insert((AirWreck { kind: a.kind, at: at.xz() }, Emitter::new(EmitterKind::WreckFire, 30.0, 900.0, Vec3::new(0.0, 1.0, 0.0))));
        }
    }
}
