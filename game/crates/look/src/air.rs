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
        app.add_systems(Startup, load).add_systems(Update, spin.in_set(bav_core::schedule::Step::Effects));
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
}

/// Rotors turn while their aircraft flies; a wreck's are still.
fn spin(time: Res<Time>, mut q: Query<(&mut Transform, &ChildOf), With<AirRotor>>, flying: Query<(), With<Aircraft>>) {
    for (mut tf, parent) in &mut q {
        if flying.contains(parent.parent()) {
            tf.rotate_y(time.delta_secs() * 30.0);
        }
    }
}
