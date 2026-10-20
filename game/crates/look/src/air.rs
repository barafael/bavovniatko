//! The aircraft's look (`bav_sim::air`): their models, by kind, and their rotors turning. A kind whose model is not
//! baked yet flies as the Ka-52, so the stream can be flown before the bake has every airframe.

use bav_sim::air::{AirKind, AirModels, AirRotor, Aircraft};
use bevy::prelude::*;

pub struct AirLookPlugin;

impl Plugin for AirLookPlugin {
    fn build(&self, app: &mut App) {
        if !bav_sim::air::flies() {
            return;
        }
        app.add_systems(Startup, load).add_systems(Update, (spin, dress_zu23, traverse_zu23).in_set(bav_core::schedule::Step::Effects));
    }
}

fn load(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut paint: ResMut<Assets<crate::baked::VehicleMaterial>>) {
    let kinds = [AirKind::Ka52, AirKind::Mi24, AirKind::Mi8, AirKind::Il76]
        .into_iter()
        .map(|k| {
            let id = if crate::baked::IDS.contains(&k.model()) { k.model() } else { "ka52" };
            let b = crate::baked::load(id, crate::models::livery(id), &mut meshes, &mut images, &mut paint);
            (k, b.hull, b.turret, b.turret_at)
        })
        .collect();
    commands.insert_resource(AirModels { kinds });
    let z = crate::baked::load("zu23", crate::models::livery("zu23"), &mut meshes, &mut images, &mut paint);
    commands.insert_resource(Zu23Model { base: z.hull, guns: z.turret, ring: z.turret_at });
}

/// The ZU-23-2's model: its platform, and its guns and seats, which traverse.
#[derive(Resource)]
struct Zu23Model {
    base: Vec<bav_sim::units::Part>,
    guns: Vec<bav_sim::units::Part>,
    ring: Vec3,
}

/// The guns that turn.
#[derive(Component)]
struct Zu23Guns;

/// A ZU-23-2 at its post, beside the figures, its guns facing east (where the helicopters come from).
fn dress_zu23(mut commands: Commands, model: Option<Res<Zu23Model>>, posts: Query<(Entity, &bav_sim::fire::Post), Added<bav_sim::manpads::Zu23>>) {
    let Some(m) = model else { return };
    for (_, p) in &posts {
        let at = p.pos + Vec2::new(6.0, -4.0);
        let mut e = commands.spawn((bav_sim::units::skin(&m.base), Transform::from_translation(bav_core::geo::ground(at)).with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))));
        let guns = m.guns.clone();
        let ring = m.ring;
        e.with_children(|c| bav_sim::units::dress(&mut c.spawn((bav_sim::units::skin(&guns), Transform::from_translation(ring), Zu23Guns)), &guns));
        bav_sim::units::dress(&mut e, &m.base);
    }
}

/// The guns follow the nearest aircraft in reach (the picture only: the burst is the battle's).
fn traverse_zu23(time: Res<Time>, mut guns: Query<(&mut Transform, &GlobalTransform), With<Zu23Guns>>, aircraft: Query<&GlobalTransform, With<Aircraft>>) {
    for (mut tf, gt) in &mut guns {
        let p = gt.translation();
        let Some(t) = aircraft.iter().map(|a| a.translation()).filter(|a| a.distance(p) < 2500.0).min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))) else { continue };
        let d = t - p;
        // In the platform's frame (turned -90°): yaw about its up.
        let want = bav_sim::units::vehicle_yaw(d.xz()) + std::f32::consts::FRAC_PI_2;
        let (cur, _, _) = tf.rotation.to_euler(EulerRot::YXZ);
        let delta = (want - cur + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
        tf.rotation = Quat::from_rotation_y(cur + delta.clamp(-1.5 * time.delta_secs(), 1.5 * time.delta_secs()));
    }
}

/// Rotors turn while their aircraft flies; a wreck's are still.
fn spin(time: Res<Time>, mut q: Query<(&mut Transform, &ChildOf), With<AirRotor>>, flying: Query<(), With<Aircraft>>) {
    for (mut tf, parent) in &mut q {
        if flying.contains(parent.parent()) {
            tf.rotate_y(time.delta_secs() * 30.0);
        }
    }
}
