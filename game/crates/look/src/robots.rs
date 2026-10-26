//! The robots' look (`bav_sim::robots`): the heavy bomber's model, where the day has one. (The ground robot is a
//! vehicle like any, built with the others: `models`.)

use bav_sim::robots::BomberModel;
use bevy::prelude::*;

pub struct RobotsLookPlugin;

impl Plugin for RobotsLookPlugin {
    fn build(&self, app: &mut App) {
        if !bav_sim::robots::spec().is_some_and(|s| s.bomber.is_some()) {
            return;
        }
        app.add_systems(Startup, load.in_set(bav_core::schedule::Setup::Kit));
    }
}

/// The Vampire: the octocopter dressed as the night bomber (tools/bake, `vampire`), in its own paint.
fn load(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut paint: ResMut<Assets<crate::baked::VehicleMaterial>>) {
    let b = crate::baked::load("vampire", crate::models::livery("vampire"), &mut meshes, &mut images, &mut paint);
    commands.insert_resource(BomberModel(b.hull));
}
