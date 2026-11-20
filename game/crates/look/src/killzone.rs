//! The kill zone's look (`bav_sim::killzone`): their FPVs and our cars on the road wear the baked models (the
//! seven-inch quad, the L200, the Lada); our EW domes stand as masts with a green lamp that goes dark when an FPV
//! knocks one out; and the nets hang over their stretch of road as a gallery of poles under a sagging mesh (c15 §5;
//! the Kyiv Independent's photographs of the Pokrovsk road, February 2026). The lamp goes amber while we have the
//! domes switched off. Their relays stand on their ground as guyed masts with a red lamp, and their launch points
//! show the same once our direction-finders have them; a lamp goes dark while our fire has it out.

use crate::meshes::MeshBuilder;
use bav_core::geo;
use bav_sim::ew::{Dome, DomeDown, DomeOff};
use bav_sim::killzone::{Crew, KillzoneModels, Located, Net, Relay, Struck};
use bav_sim::units::Palette;
use bevy::prelude::*;

pub struct KillzoneLookPlugin;

impl Plugin for KillzoneLookPlugin {
    fn build(&self, app: &mut App) {
        if !bav_sim::killzone::present() {
            return;
        }
        app.add_systems(Startup, load.in_set(bav_core::schedule::Setup::Kit))
            .add_systems(Startup, (dress_domes, dress_nets, dress_points).in_set(bav_core::schedule::Setup::Dress))
            .add_systems(Update, (dome_lamps, point_lamps).chain().in_set(crate::Step::Effects));
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
    lit: Handle<StandardMaterial>,
    dark: Handle<StandardMaterial>,
    off: Handle<StandardMaterial>,
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
    let off = materials.add(StandardMaterial { base_color: Color::srgb(1.0, 0.65, 0.1), emissive: LinearRgba::rgb(6.0, 3.0, 0.3), unlit: true, ..default() });
    for (e, tf) in &domes {
        commands.spawn((Mesh3d(mast.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(tf.translation)));
        commands.spawn((Mesh3d(lamp.clone()), MeshMaterial3d(lit.clone()), Transform::from_translation(tf.translation + Vec3::Y * 6.2), DomeLamp(e), bevy::light::NotShadowCaster));
    }
    commands.insert_resource(DomeMats { lit, dark, off });
}

/// A dome's lamp burns green while it jams, amber while we have it switched off, and goes dark when it is knocked
/// out.
fn dome_lamps(mats: Option<Res<DomeMats>>, domes: Query<(Has<DomeDown>, Has<DomeOff>)>, mut lamps: Query<(&DomeLamp, &mut MeshMaterial3d<StandardMaterial>)>) {
    let Some(mats) = mats else { return };
    for (lamp, mut mat) in &mut lamps {
        let Ok((down, off)) = domes.get(lamp.0) else { continue };
        let want = if down {
            &mats.dark
        } else if off {
            &mats.off
        } else {
            &mats.lit
        };
        if mat.0 != *want {
            mat.0 = want.clone();
        }
    }
}

/// A lamp on one of their masts: a relay's, or a launch point's once we have located it.
#[derive(Component)]
struct PointLamp(Entity);

#[derive(Resource)]
struct PointKit {
    mast: Handle<Mesh>,
    lamp: Handle<Mesh>,
    lit: Handle<StandardMaterial>,
    dark: Handle<StandardMaterial>,
}

/// Their relays stand as masts with a red lamp on their ground; a launch point shows the same once our
/// direction-finders have it (`Located`), a pickup's mast among the ruins or the trees.
fn dress_points(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, palette: Res<Palette>, relays: Query<(Entity, &Transform), With<Relay>>) {
    let mut m = MeshBuilder::new();
    m.cylinder(Vec3::ZERO, Vec3::Y * 9.0, 0.07, 6, [0.28, 0.28, 0.27]);
    for k in 0..3 {
        let a = k as f32 * std::f32::consts::TAU / 3.0;
        let d = Vec3::new(a.cos(), 0.0, a.sin());
        // Guy wires, from the ground to near the top.
        m.cylinder(d * 4.0, Vec3::Y * 8.0 - d * 4.0, 0.015, 3, [0.15, 0.15, 0.15]);
    }
    m.cylinder(Vec3::Y * 8.6, Vec3::Y * 2.4, 0.03, 4, [0.12, 0.12, 0.12]);
    m.boxc(Vec3::new(0.0, 0.25, 0.0), Vec3::new(0.6, 0.5, 0.6), [0.25, 0.27, 0.20]);
    let kit = PointKit {
        mast: meshes.add(m.build()),
        lamp: meshes.add(Sphere::new(0.3).mesh().ico(1).expect("an icosphere")),
        lit: materials.add(StandardMaterial { base_color: Color::srgb(1.0, 0.2, 0.15), emissive: LinearRgba::rgb(30.0, 3.0, 2.0), unlit: true, ..default() }),
        dark: materials.add(StandardMaterial { base_color: Color::srgb(0.12, 0.05, 0.05), ..default() }),
    };
    for (e, tf) in &relays {
        mast(&mut commands, &kit, &palette, e, tf.translation);
    }
    commands.insert_resource(kit);
}

fn mast(commands: &mut Commands, kit: &PointKit, palette: &Palette, e: Entity, at: Vec3) {
    commands.spawn((Mesh3d(kit.mast.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(at)));
    commands.spawn((Mesh3d(kit.lamp.clone()), MeshMaterial3d(kit.lit.clone()), Transform::from_translation(at + Vec3::Y * 9.3), PointLamp(e), bevy::light::NotShadowCaster));
}

/// Their launch crews that we located this frame.
type JustLocated = (Added<Located>, With<Crew>);

/// A launch point we have just located gets its mast; each lamp goes dark while our fire has it out.
fn point_lamps(
    mut commands: Commands,
    kit: Option<Res<PointKit>>,
    palette: Res<Palette>,
    located: Query<(Entity, &Transform), JustLocated>,
    struck: Query<Has<Struck>>,
    mut lamps: Query<(&PointLamp, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Some(kit) = kit else { return };
    for (e, tf) in &located {
        mast(&mut commands, &kit, &palette, e, tf.translation);
    }
    for (lamp, mut mat) in &mut lamps {
        let want = if struck.get(lamp.0).unwrap_or(false) { &kit.dark } else { &kit.lit };
        if mat.0 != *want {
            mat.0 = want.clone();
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
