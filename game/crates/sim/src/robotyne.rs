//! Mala Tokmachka, 8 June 2023: what the battle needs beyond the shared kit. The 291st's tanks dug in on the crest,
//! and the Ka-52s on station beyond reach. The ground is `game/dioramas/robotyne.ron`; the set and the light are
//! `bav_look::robotyne`.
//!
//! The history is in db/research/chapters/c09-robotyne.md (§3, §5, §6 set piece 1). Before dawn the 47th Mechanised
//! Brigade goes south behind Leopard 2R and BMR-2 mine-clearing vehicles, in two groups of nine and eleven, up a
//! long open slope towards the 291st Regiment's first line on the crest. The barrier minefields are hundreds of
//! metres deep, with anti-tank mines stacked two high; the lanes are too shallow, the columns stack up on them,
//! and drones, guns, ATGMs and Ka-52s firing from beyond reach do the rest. Four more Bradleys go back in for the
//! men. Most crews get out through the hatches and the lowered ramps.

use crate::fire::Missile;
use crate::fx::{self, FxQueue, FxRng};
use crate::geo;
use crate::units::{Models, Palette, Part, Radio, Side, VKind, Vehicle, VehicleSpec};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct RobotynePlugin;

impl Plugin for RobotynePlugin {
    fn build(&self, app: &mut App) {
        if crate::diorama::diorama() != crate::diorama::Diorama::Robotyne {
            return;
        }
        app.insert_resource(HeliRng(SmallRng::seed_from_u64(crate::dev::seeded(52))))
            .add_systems(Update, fly_helicopters.run_if(crate::running).in_set(crate::Step::Fire));
        // Command: the air defence on call, if there is any (nothing of it in the Record's world).
        if crate::allot::at_table() {
            app.insert_resource(AdRng(SmallRng::seed_from_u64(crate::dev::seeded(1972)))).add_systems(Update, air_defence.after(fly_helicopters).run_if(crate::running).in_set(crate::Step::Fire));
        }
    }
}

/// Tanks dug in hull-down on the defenders' line (the 291st's T-72B3s behind the crest trench), facing north.
pub fn spawn_dug_in(commands: &mut Commands, models: &Models, palette: &Palette) {
    for (k, &((x, z), yaw)) in crate::data::sim_data().dug_in.iter().enumerate() {
        let p = Vec2::new(x, z);
        let spec = VehicleSpec { side: Side::Ru, kind: VKind::T72, route: 0, s: 0.0, lat: 0.0, wave: 0, idx: 100 + k as u32, plow: false, smoke: false, rescue: false, column_pace: f32::INFINITY };
        let e = crate::units::spawn_vehicle_at(commands, models, palette, spec, p, yaw);
        commands.entity(e).insert(Transform::from_translation(geo::ground(p) - Vec3::Y * 1.1).with_rotation(Quat::from_rotation_y(yaw)));
    }
}

// ---------------------------------------------------------------------------------------------------------
// The Ka-52s: on station 4 km out over the Russian side, popping up to fire Vikhrs and dropping back. Nothing
// the brigade has reaches them (dossier c09 §5: "one of the most influential Russian weapons systems").

#[derive(Resource)]
pub struct HeliModels {
    pub body: Vec<Part>,
    pub rotor: Vec<Part>,
    pub rotor_at: Vec3,
}

#[derive(Resource)]
struct HeliRng(SmallRng);

#[derive(Component)]
pub struct Helicopter {
    home: Vec2,
    timer: f32,
    shots: u32,
    age: f32,
    phase: f32,
}

/// A rotor: the picture spins it.
#[derive(Component)]
pub struct Rotor;

pub fn spawn_helicopter(commands: &mut Commands, h: &HeliModels, at: Vec2) {
    let phase = (at.x * 0.013).sin() * 3.0;
    let mut e = commands.spawn((
        crate::units::skin(&h.body),
        Transform::from_translation(geo::ground(at) + Vec3::Y * 20.0),
        Helicopter { home: at, timer: 10.0 + phase.abs() * 3.0, shots: 6, age: 0.0, phase },
    ));
    e.with_children(|c| crate::units::dress(&mut c.spawn((crate::units::skin(&h.rotor), Transform::from_translation(h.rotor_at), Rotor)), &h.rotor));
    crate::units::dress(&mut e, &h.body);
}

#[allow(clippy::too_many_arguments)]
fn fly_helicopters(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<HeliRng>,
    mut fx: ResMut<FxQueue>,
    mut fxr: ResMut<FxRng>,
    mut radio: ResMut<Radio>,
    smoke: Res<fx::Smoke>,
    models: Res<Models>,
    palette: Res<Palette>,
    vehicles: Query<(Entity, &Vehicle, &Transform), Without<Helicopter>>,
    mut helis: Query<(Entity, &mut Helicopter, &mut Transform)>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    for (e, mut h, mut tf) in &mut helis {
        h.age += dt;
        h.timer -= dt;
        let leaving = h.shots == 0;
        let mut p = tf.translation;
        if leaving {
            // Out of missiles: away, low, back to Berdiansk.
            let away = h.home.normalize_or_zero();
            p += Vec3::new(away.x, 0.0, away.y) * 45.0 * dt;
            if p.xz().length() > 9000.0 {
                commands.entity(e).despawn();
                continue;
            }
        }
        // Hover low behind the ridge; pop up for a few seconds to fire.
        let pop = (1.0 - (h.timer.min(4.0) / 4.0)).max(0.0);
        let want_y = geo::height(p.x, p.z) + 22.0 + 5.0 * (h.age * 0.35 + h.phase).sin() + 35.0 * pop;
        p.y += (want_y - p.y) * (1.0 - (-dt * 1.2).exp());
        p.x = if leaving { p.x } else { h.home.x + 40.0 * (h.age * 0.11 + h.phase).sin() };
        tf.translation = p;
        let face = Vec2::new(0.0, 200.0) - p.xz();
        tf.rotation = Quat::from_rotation_y(crate::units::vehicle_yaw(face)) * Quat::from_rotation_x(-0.08 - 0.06 * pop);
        if leaving || h.timer > 0.0 {
            continue;
        }
        // A target: something of ours that the smoke does not hide, preferably something that cannot move.
        let from = p.xz();
        let mut best: Option<(Entity, f32)> = None;
        for (ve, v, vt) in &vehicles {
            if v.side != Side::Ua || !v.state.alive() || smoke.blocks(from, vt.translation.xz(), now) {
                continue;
            }
            let score = vt.translation.xz().distance(from) - if v.state.mobile() { 0.0 } else { 1500.0 } + rng.random_range(0.0..800.0);
            if best.is_none_or(|b| score < b.1) {
                best = Some((ve, score));
            }
        }
        h.timer = rng.random_range(18.0..30.0);
        let Some((target, _)) = best else { continue };
        h.shots -= 1;
        let fwd = tf.rotation * Vec3::NEG_Z;
        let muzzle = p + fwd * 3.0 - Vec3::Y * 0.6;
        fx.flash(muzzle, 4.0e6, 80.0, 0.15);
        // Heard through the effects' queue: a sound parameter here would change this system's place in the frame.
        fx.sound(muzzle, crate::sound::Sfx::VikhrLaunch, 1.5);
        for _ in 0..6 {
            fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: muzzle, vel: -fwd * fxr.0.random_range(3.0..9.0), life: 6.0, size0: 1.5, size1: 8.0, alpha: 0.5, drag: 1.0, ..default() });
        }
        commands.spawn((
            Mesh3d(models.missile.clone()),
            MeshMaterial3d(palette.vertex.clone()),
            Transform::from_translation(muzzle).looking_to(fwd, Vec3::Y),
            Missile { target, launched_at: muzzle, speed: 450.0, trail: 0.0, p_hit: Some(0.62), top_attack: false, weapon: "Vikhr", from: "a Ka-52" },
            bevy::light::NotShadowCaster,
        ));
        if rng.random_bool(0.5) {
            radio.say(now, "Launch from the helicopter! Missile inbound!");
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// Command's air defence on call (design/command.md phase 7): a battery behind our lines with a few missiles for the
// day. A Ka-52 is out of its reach while it hangs low behind the ridge; when one pops up to fire, the battery gets one
// shot at it. It may bring it down, or make it turn for home with its Vikhrs unfired; or the missile goes wide.

/// The air defence's own stream.
#[derive(Resource)]
struct AdRng(SmallRng);

/// A Ka-52 the battery has fired at during this pop-up (by the shots it had left then).
#[derive(Component)]
struct Engaged(u32);

/// Where the battery stands: behind our start line.
fn battery() -> Vec2 {
    geo::layout().home + Vec2::new(300.0, -500.0)
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn air_defence(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<AdRng>,
    mut fc: ResMut<crate::fire::FireControl>,
    mut fx: ResMut<FxQueue>,
    mut radio: ResMut<Radio>,
    mut helis: Query<(Entity, &mut Helicopter, &Transform, Option<&Engaged>)>,
) {
    if fc.air_defence == 0 {
        return;
    }
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    for (e, mut h, tf, engaged) in &mut helis {
        // Up to fire, and not yet fired at this time up.
        let up = h.shots > 0 && h.timer < 2.5;
        if !up || engaged.is_some_and(|g| g.0 == h.shots) || fc.air_defence == 0 {
            continue;
        }
        fc.air_defence -= 1;
        commands.entity(e).insert(Engaged(h.shots));
        let at = tf.translation;
        let from = geo::ground(battery()) + Vec3::Y * 3.0;
        fx.sound(from, crate::sound::Sfx::AtgmLaunch, 1.4);
        fx.flash(from, 2.0e6, 60.0, 0.12);
        // The missile's trail, the whole way up to it.
        for k in 0..40 {
            let p = from.lerp(at, k as f32 / 40.0);
            fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: p, vel: Vec3::Y * 0.4, life: 7.0 + k as f32 * 0.05, size0: 1.2, size1: 5.0, alpha: 0.35, drag: 1.0, ..default() });
        }
        let roll: f64 = rng.random();
        if roll < 0.3 {
            fx.explosion(at, 1.2, false);
            fx.sound(at, crate::sound::Sfx::BlastFar, 1.2);
            commands.entity(e).despawn();
            radio.say2(now, "Мінус «алігатор»! Ка-52 збито.", "One alligator down! The Ka-52 is hit.");
            info!("air defence: a Ka-52 brought down at {:.0},{:.0}, {} missiles left, at {now:.0} s", at.x, at.z, fc.air_defence);
        } else if roll < 0.7 {
            h.shots = 0;
            radio.say2(now, "ППО працює. Ка-52 розвернувся, йде геть.", "Air defence firing. The Ka-52 has turned away.");
            info!("air defence: a Ka-52 turned away, {} missiles left, at {now:.0} s", fc.air_defence);
        } else {
            radio.say2(now, "ППО: промах.", "Air defence: a miss.");
            info!("air defence: a miss, {} missiles left, at {now:.0} s", fc.air_defence);
        }
    }
}
