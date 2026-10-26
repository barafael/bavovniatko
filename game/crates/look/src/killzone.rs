//! The kill zone's look (`bav_sim::killzone`): their FPVs and our cars on the road wear the baked models (the
//! seven-inch quad, the L200, the Lada); our EW domes stand as masts with a green lamp that goes dark when an FPV
//! knocks one out; and the nets hang over their stretch of road as a gallery of poles under a sagging mesh (c15 §5;
//! the Kyiv Independent's photographs of the Pokrovsk road, February 2026).

use crate::meshes::MeshBuilder;
use bav_core::geo;
use bav_sim::ew::{Dome, DomeDown};
use bav_sim::killzone::{KillzoneModels, Net};
use bav_sim::units::Palette;
use bevy::prelude::*;

pub struct KillzoneLookPlugin;

impl Plugin for KillzoneLookPlugin {
    fn build(&self, app: &mut App) {
        if !bav_sim::killzone::present() {
            return;
        }
        app.add_systems(Startup, load.in_set(bav_core::schedule::Setup::Kit))
            .add_systems(Startup, (dress_domes, dress_nets).in_set(bav_core::schedule::Setup::Dress))
            .add_systems(Update, dome_lamps.in_set(crate::Step::Effects));
    }
}

fn load(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut paint: ResMut<Assets<crate::baked::VehicleMaterial>>) {
    let mut get = |id: &str| crate::baked::load(id, crate::models::livery(id), &mut meshes, &mut images, &mut paint).hull;
    commands.insert_resource(KillzoneModels { fpv: get("fpv"), l200: get("l200"), lada: get("lada") });
}

#[derive(Component)]
struct DomeLamp(Entity);

#[derive(Resource)]
struct DomeMats {
    dark: Handle<StandardMaterial>,
}

/// Each dome a mast of whip antennas over its box of electronics and a battery, and a lamp on top.
fn dress_domes(mut commands: Commands, palette: Res<Palette>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, domes: Query<(Entity, &Transform), With<Dome>>) {
    let mut m = MeshBuilder::new();
    m.cylinder(Vec3::ZERO, Vec3::Y * 6.0, 0.06, 6, [0.30, 0.31, 0.29]);
    for k in 0..4 {
        let a = k as f32 * std::f32::consts::FRAC_PI_2;
        let d = Vec3::new(a.cos(), 0.0, a.sin()) * 0.35;
        m.cylinder(Vec3::Y * 5.2 + d, Vec3::Y * 1.6 + d * 0.3, 0.02, 4, [0.12, 0.12, 0.12]);
    }
    m.boxc(Vec3::new(-0.45, 0.0, 0.4), Vec3::new(0.45, 0.6, 1.1), [0.27, 0.30, 0.22]);
    let mast = meshes.add(m.build());
    let lamp = meshes.add(Sphere::new(0.25).mesh().ico(1).expect("an icosphere"));
    let lit = materials.add(StandardMaterial { base_color: Color::srgb(0.2, 1.0, 0.3), emissive: LinearRgba::rgb(2.0, 30.0, 4.0), unlit: true, ..default() });
    let dark = materials.add(StandardMaterial { base_color: Color::srgb(0.05, 0.12, 0.06), ..default() });
    for (e, tf) in &domes {
        commands.spawn((Mesh3d(mast.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(tf.translation)));
        commands.spawn((Mesh3d(lamp.clone()), MeshMaterial3d(lit.clone()), Transform::from_translation(tf.translation + Vec3::Y * 6.2), DomeLamp(e), bevy::light::NotShadowCaster));
    }
    commands.insert_resource(DomeMats { dark });
}

/// A dome's lamp burns while it jams, and goes dark when it is knocked out.
fn dome_lamps(mats: Option<Res<DomeMats>>, down: Query<(), Added<DomeDown>>, mut lamps: Query<(&DomeLamp, &mut MeshMaterial3d<StandardMaterial>)>) {
    let Some(mats) = mats else { return };
    for (lamp, mut mat) in &mut lamps {
        if down.contains(lamp.0) && mat.0 != mats.dark {
            mat.0 = mats.dark.clone();
        }
    }
}

/// The nets: poles every ten metres either side of the road, four metres high, and the mesh between them sagging
/// over it.
fn dress_nets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, palette: Res<Palette>, nets: Query<&Net>) {
    let mut poles = MeshBuilder::new();
    let mut mesh = MeshBuilder::new();
    let wood = [0.30, 0.24, 0.17];
    let green = [0.22, 0.27, 0.18];
    for net in &nets {
        for w in net.line.windows(2) {
            let (a, b) = (w[0], w[1]);
            let len = a.distance(b);
            let dir = (b - a) / len.max(0.01);
            let side = dir.perp() * 5.0;
            let n = (len / 10.0).ceil().max(1.0) as usize;
            for k in 0..=n {
                let p = a + dir * (len * k as f32 / n as f32);
                for s in [side, -side] {
                    let g = geo::ground(p + s);
                    poles.cylinder(g, Vec3::Y * 4.2, 0.08, 5, wood);
                }
                if k < n {
                    let q = a + dir * (len * (k + 1) as f32 / n as f32);
                    let top = |p: Vec2, s: Vec2, sag: f32| geo::ground(p + s) + Vec3::Y * (4.0 - sag);
                    // Two panels to a bay, sagging to the middle of the road.
                    let (mp, mq) = (geo::ground(p) + Vec3::Y * 3.4, geo::ground(q) + Vec3::Y * 3.4);
                    mesh.quad(top(p, side, 0.0), mp, mq, top(q, side, 0.0), green);
                    mesh.quad(mp, top(p, -side, 0.0), top(q, -side, 0.0), mq, green);
                }
            }
        }
    }
    if !poles.is_empty() {
        commands.spawn((Mesh3d(meshes.add(poles.build())), MeshMaterial3d(palette.vertex.clone())));
        let netting = materials.add(StandardMaterial { base_color: Color::srgba(1.0, 1.0, 1.0, 0.55), alpha_mode: AlphaMode::Blend, double_sided: true, cull_mode: None, perceptual_roughness: 1.0, ..default() });
        commands.spawn((Mesh3d(meshes.add(mesh.build())), MeshMaterial3d(netting), bevy::light::NotShadowCaster));
    }
}
