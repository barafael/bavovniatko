//! The drone's thermal channel (design/hostomel.md, phase 6), where the diorama has one (`thermal` in its file).
//! The picture has no heat of its own, so what is hot carries it: a small marker on every man, on a vehicle's
//! engine deck, on an aircraft's engines, and over a fire, in a colour nothing else in the scene is (magenta). The
//! markers are hidden until the channel is on (key H, from the drone); the feed's thermal mode reads the colour as
//! heat and draws it white over a world flattened to one grey.
//!
//! Only where the file asks for it: every marker is an entity, and an entity more would change another day.

use bav_core::vocab::Mode;
use bav_sim::fx::{Emitter, EmitterKind};
use bav_sim::player::View;
use bav_sim::units::{Soldier, Vehicle};
use bevy::{light::NotShadowCaster, prelude::*};

pub struct ThermalPlugin;

impl Plugin for ThermalPlugin {
    fn build(&self, app: &mut App) {
        if !bav_core::data::data().thermal {
            return;
        }
        app.add_systems(Startup, setup)
            .add_systems(Update, (toggle, mark_men, mark_posts, mark_vehicles, mark_aircraft, mark_fires, show).chain().in_set(bav_core::schedule::Step::Effects));
    }
}

#[derive(Resource)]
struct HeatKit {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

/// A marker of heat.
#[derive(Component)]
struct Heat;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(HeatKit {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(StandardMaterial { base_color: Color::srgb(0.85, 0.0, 0.85), unlit: true, ..default() }),
    });
}

/// H: the drone's camera between its visible and its thermal channel.
fn toggle(keys: Res<ButtonInput<KeyCode>>, mut view: ResMut<View>) {
    if view.mode == Mode::Drone && keys.just_pressed(KeyCode::KeyH) {
        view.thermal = !view.thermal;
    }
    if crate::dev::dev().thermal && view.mode == Mode::Drone {
        view.thermal = true;
    }
}

fn marker(kit: &HeatKit, at: Vec3, size: Vec3) -> impl Bundle {
    (Mesh3d(kit.mesh.clone()), MeshMaterial3d(kit.material.clone()), Transform::from_translation(at).with_scale(size), Visibility::Hidden, NotShadowCaster, Heat)
}

/// A man: his body, warm through the coat.
fn mark_men(mut commands: Commands, kit: Res<HeatKit>, men: Query<Entity, Added<Soldier>>) {
    for e in &men {
        commands.entity(e).with_child(marker(&kit, Vec3::Y * 0.9, Vec3::new(0.45, 1.4, 0.3)));
    }
}

/// A post: its men in their holes, a warm smudge a few metres long (the figures are not soldiers of their own).
fn mark_posts(mut commands: Commands, kit: Res<HeatKit>, posts: Query<Entity, Added<bav_sim::fire::Post>>) {
    for e in &posts {
        commands.entity(e).with_child(marker(&kit, Vec3::Y * 0.5, Vec3::new(7.0, 0.6, 2.0)));
    }
}

/// A vehicle: its engine deck, at the back.
fn mark_vehicles(mut commands: Commands, kit: Res<HeatKit>, vehicles: Query<Entity, Added<Vehicle>>) {
    for e in &vehicles {
        commands.entity(e).with_child(marker(&kit, Vec3::new(0.0, 1.7, 2.2), Vec3::new(2.2, 0.5, 1.8)));
    }
}

/// An aircraft: its engines under the rotor, its exhausts.
fn mark_aircraft(mut commands: Commands, kit: Res<HeatKit>, aircraft: Query<Entity, Added<bav_sim::air::Aircraft>>) {
    for e in &aircraft {
        commands.entity(e).with_child(marker(&kit, Vec3::new(0.0, 2.6, 0.5), Vec3::new(1.6, 0.9, 3.0)));
    }
}

/// A fire: a burning wreck, a pool of fuel.
fn mark_fires(mut commands: Commands, kit: Res<HeatKit>, fires: Query<(Entity, &Emitter), Added<Emitter>>) {
    for (e, em) in &fires {
        if matches!(em.kind, EmitterKind::WreckFire | EmitterKind::PoolFire) {
            commands.entity(e).with_child(marker(&kit, Vec3::Y * 1.5, Vec3::new(4.0, 3.0, 4.0)));
        }
    }
}

/// The markers show only on the thermal channel.
fn show(view: Res<View>, mut q: Query<&mut Visibility, With<Heat>>) {
    let want = if view.thermal && view.mode == Mode::Drone { Visibility::Inherited } else { Visibility::Hidden };
    for mut v in &mut q {
        v.set_if_neq(want);
    }
}
