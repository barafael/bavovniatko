//! The wings' look (`bav_sim::wings`): their Orlans and ZALAs wear the baked models (flightclubua's Orlan-10,
//! Viktor Zhuravlev's ZALA 421-16E), and our interceptors the seven-inch quad their FPVs wear.

use bav_sim::wings::WingModels;
use bevy::prelude::*;

pub struct WingsLookPlugin;

impl Plugin for WingsLookPlugin {
    fn build(&self, app: &mut App) {
        if !bav_sim::wings::present() {
            return;
        }
        app.add_systems(Startup, load.in_set(bav_core::schedule::Setup::Kit));
    }
}

fn load(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut paint: ResMut<Assets<crate::baked::VehicleMaterial>>) {
    let mut get = |id: &str| crate::baked::load(id, crate::models::livery(id), &mut meshes, &mut images, &mut paint).hull;
    commands.insert_resource(WingModels { orlan: get("orlan"), zala: get("zala"), fpv: get("fpv") });
}
