//! Mala Tokmachka, 8 June 2023: what the battle needs beyond the shared kit: the 291st's tanks dug in on the crest.
//! (Its Ka-52s on station beyond reach are `air`'s, `Flight::Station`.) The ground is `game/dioramas/robotyne.ron`; the set and the light are
//! `bav_look::robotyne`.
//!
//! The history is in db/research/chapters/c09-robotyne.md (§3, §5, §6 set piece 1). Before dawn the 47th Mechanised
//! Brigade goes south behind Leopard 2R and BMR-2 mine-clearing vehicles, in two groups of nine and eleven, up a
//! long open slope towards the 291st Regiment's first line on the crest. The barrier minefields are hundreds of
//! metres deep, with anti-tank mines stacked two high; the lanes are too shallow, the columns stack up on them,
//! and drones, guns, ATGMs and Ka-52s firing from beyond reach do the rest. Four more Bradleys go back in for the
//! men. Most crews get out through the hatches and the lowered ramps.

use crate::geo;
use crate::units::{Models, Palette, Side, VKind, VehicleSpec};
use bevy::prelude::*;

/// Tanks dug in hull-down on the defenders' line (the 291st's T-72B3s behind the crest trench), facing north.
pub fn spawn_dug_in(commands: &mut Commands, models: &Models, palette: &Palette) {
    for (k, &((x, z), yaw)) in crate::data::sim_data().dug_in.iter().enumerate() {
        let p = Vec2::new(x, z);
        let spec = VehicleSpec { side: Side::Ru, kind: VKind::T72, route: 0, s: 0.0, lat: 0.0, wave: 0, idx: 100 + k as u32, plow: false, smoke: false, rescue: false, column_pace: f32::INFINITY };
        let e = crate::units::spawn_vehicle_at(commands, models, palette, spec, p, yaw);
        commands.entity(e).insert(Transform::from_translation(geo::ground(p) - Vec3::Y * 1.1).with_rotation(Quat::from_rotation_y(yaw)));
    }
}
