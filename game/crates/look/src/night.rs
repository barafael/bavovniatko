//! The night's lights (design/hostomel.md, phase 6), where a fight runs into the dark: what has been burning since
//! the afternoon, the buildings the bombs set alight, the flares and illumination rounds (`bav_sim::flares`) with the
//! ground lit under them, and the muzzles lighting their surroundings for a frame. Burning wrecks glow already
//! (`fx::fire_glow`). The point lights are shadowless, and only the brightest near the camera are drawn (`budget`):
//! the 780M's clustered lighting is paid per light (design/perf.md).
//!
//! Only where the file has a fight into the dark (its posts carry flares): each light is an entity, and an entity
//! more would change another day.

use crate::fx::{FxAssets, FxLight, Kind, Spawn, flash, spawn};
use bav_core::geo;
use bav_sim::flares::{Flare, FlareKind};
use bav_sim::fx::{Emitter, EmitterKind, FxCall, FxQueue};
use bav_sim::player::MainCamera;
use bevy::prelude::*;

pub struct NightPlugin;

impl Plugin for NightPlugin {
    fn build(&self, app: &mut App) {
        if !bav_sim::fight::spec().is_some_and(|f| f.flares > 0) {
            return;
        }
        app.add_systems(Startup, light_fires.in_set(bav_core::schedule::Setup::Dress))
            .add_systems(Update, muzzles.in_set(crate::Step::Effects).before(crate::fx::play_requests))
            .add_systems(Update, (bombed_buildings, flare_lights, budget).chain().in_set(crate::Step::Effects).after(crate::fx::light_particles));
    }
}

/// How many of the point lights are drawn at once.
const LIGHTS: usize = 24;

/// What has been burning since the afternoon (`fires` in the file): so many flames side by side at each.
fn light_fires(mut commands: Commands) {
    for &((x, z), n) in &bav_core::data::data().fires {
        for k in 0..n {
            let at = Vec2::new(x, z) + Vec2::new((k as f32 - (n as f32 - 1.0) * 0.5) * 7.0, (k % 2) as f32 * 4.0);
            commands.spawn((Transform::from_translation(geo::ground(at)), Emitter::new(EmitterKind::WreckFire, 18.0, 1.0e5, Vec3::Y)));
        }
    }
}

/// A heavy bomb on a building sets it burning, on its roof, for ten minutes or so: one fire a building.
fn bombed_buildings(mut commands: Commands, mut landed: MessageReader<bav_sim::fire::ShellLanded>, mut burning: Local<Vec<usize>>) {
    let lay = geo::layout();
    for s in landed.read() {
        if s.kind != bav_sim::fire::ShellKind::Himars {
            continue;
        }
        let Some(i) = lay.buildings.iter().position(|b| geo::inside(&b.foot, s.at)) else { continue };
        if burning.contains(&i) {
            continue;
        }
        burning.push(i);
        let b = &lay.buildings[i];
        let mid = b.foot.iter().copied().sum::<Vec2>() / b.foot.len() as f32;
        commands.spawn((Transform::from_translation(geo::ground(mid) + Vec3::Y * b.height * 0.7), Emitter::new(EmitterKind::WreckFire, 20.0, 600.0, Vec3::Y)));
    }
}

/// A rifle or a machine gun firing in the dark lights the ground round it for a frame: one flash a muzzle a frame.
fn muzzles(mut commands: Commands, queue: Res<FxQueue>, light: Res<FxLight>, mut fx: ResMut<FxAssets>, mut lit: Local<Vec<Vec3>>) {
    if light.0 > 0.75 {
        return;
    }
    lit.clear();
    for call in &queue.0 {
        let FxCall::Tracer { from, to, .. } = *call else { continue };
        if lit.iter().any(|p| p.distance_squared(from) < 4.0) {
            continue;
        }
        lit.push(from);
        flash(&mut commands, from, 1.5e5, 14.0, 0.05);
        spawn(&mut commands, &mut fx, Spawn { kind: Kind::Flash, pos: from + (to - from).normalize_or_zero() * 0.6, vel: Vec3::ZERO, life: 0.05, size0: 0.5, size1: 0.7, alpha: 1.0, ..default() });
    }
}

/// The light under a flare, kept apart from the battle's entity.
#[derive(Component)]
struct FlareLight {
    of: Entity,
}

/// Each flare burning: a hard white light swinging under its canopy, the star itself, and its trail of smoke.
fn flare_lights(mut commands: Commands, time: Res<Time>, mut fx: ResMut<FxAssets>, flares: Query<(Entity, &Flare, &Transform), Without<FlareLight>>, mut lights: Query<(Entity, &FlareLight, &mut PointLight, &mut Transform)>, mut lit: Local<Vec<Entity>>, mut puff: Local<f32>) {
    let t = time.elapsed_secs();
    let s = |e: Entity| (e.to_bits() % 97) as f32;
    for (e, l, mut light, mut tf) in &mut lights {
        let Ok((_, f, ftf)) = flares.get(l.of) else {
            commands.entity(e).despawn();
            lit.retain(|&x| x != l.of);
            continue;
        };
        let full = match f.kind {
            FlareKind::Hand => 5.0e7,
            FlareKind::Shell => 3.0e8,
        };
        // The magnesium sputters; the canopy swings the star a few metres either way.
        light.intensity = full * (0.85 + 0.15 * geo::value_noise(t * 9.0 + s(l.of), 1.0, 41));
        let sway = Vec3::new((t * 0.9 + s(l.of)).sin(), 0.0, (t * 0.7 + s(l.of)).cos()) * if f.kind == FlareKind::Shell { 4.0 } else { 1.5 };
        tf.translation = ftf.translation + sway;
    }
    *puff -= crate::sim_dt(&time);
    let trail = *puff <= 0.0;
    if trail {
        *puff = 0.3;
    }
    for (e, f, tf) in &flares {
        let star = if f.kind == FlareKind::Shell { 7.0 } else { 3.0 };
        spawn(&mut commands, &mut fx, Spawn { kind: Kind::Flash, pos: tf.translation, vel: Vec3::ZERO, life: 0.06, size0: star, size1: star, alpha: 1.0, ..default() });
        if trail {
            spawn(&mut commands, &mut fx, Spawn { kind: Kind::WhiteSmoke, pos: tf.translation + Vec3::Y * 1.5, vel: Vec3::Y * 0.5, life: 12.0, size0: 1.0, size1: 6.0, alpha: 0.25, drag: 0.5, ..default() });
        }
        if !lit.contains(&e) {
            lit.push(e);
            let range = if f.kind == FlareKind::Shell { 1300.0 } else { 450.0 };
            commands.spawn((PointLight { intensity: 0.0, range, color: Color::srgb(1.0, 0.96, 0.88), shadow_maps_enabled: false, ..default() }, Transform::from_translation(tf.translation), FlareLight { of: e }));
        }
    }
}

/// Only the lights that matter most are drawn: the brightest for their distance from the camera, so many at most.
/// (A blast's fireball carries a dark light of its own for its timing, on its mesh: that is left alone.)
fn budget(cam: Single<&GlobalTransform, With<MainCamera>>, mut lights: Query<(&PointLight, &Transform, &mut Visibility), Without<Mesh3d>>) {
    let at = cam.translation();
    let mut all: Vec<(f32, Mut<Visibility>)> = lights.iter_mut().map(|(l, tf, v)| (l.intensity / (1.0 + tf.translation.distance_squared(at)), v)).collect();
    if all.len() > LIGHTS {
        all.sort_by(|a, b| b.0.total_cmp(&a.0));
    }
    for (k, (_, mut v)) in all.into_iter().enumerate() {
        v.set_if_neq(if k < LIGHTS { Visibility::Inherited } else { Visibility::Hidden });
    }
}
