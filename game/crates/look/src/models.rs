//! The battle's models built at startup: the baked vehicles with the hand-built tops, plows and rollers, the
//! figures in each side's season, and the small things (RAAM mines, missiles, lane markers, a thrown track). The
//! battle holds them as handles (`bav_sim::units::Models`) and dresses what it spawns with them.

use crate::baked;
use crate::meshes::{self, WEST_GREEN};
use bav_core::diorama::diorama;
use bav_sim::units::{FigureMeshes, Livery, Models, Part, VKind, VehicleMaterial, VehicleMeshes};
use bevy::{platform::collections::HashMap, prelude::*};
use strum::VariantArray;

/// Each model's paint, by the side that drove it and the season. Vuhledar, February: the Russian 4BO green gone
/// dull, mud-caked, with what is left of a winter whitewash; the 72nd Brigade's T-64s plain green. Mala
/// Tokmachka, June: the Leopards and Bradleys in NATO three-tone under the steppe's dust, the BMR-2 green.
pub fn livery(id: &str) -> Livery {
    let summer = diorama().summer();
    let seed = baked::IDS.iter().position(|&i| i == id).unwrap_or(0) as f32 * 7.31;
    let (scheme, whitewash) = match id {
        "t80" | "t72" | "bmp1" | "bmp2" | "mtlb" => (0.0, if summer { 0.0 } else { 0.7 }),
        "leo2a6" | "leo2r" | "bradley" => (1.0, 0.0),
        "t64" => (2.0, 0.0),
        _ => (3.0, 0.0),
    };
    Livery { scheme: Vec4::new(scheme, whitewash, summer as u32 as f32, seed), ..default() }
}

pub fn setup_models(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<VehicleMaterial>>) {
    // The real models (tools/bake), painted over by the vehicle shader.
    let baked: HashMap<&str, baked::Baked> = baked::IDS.iter().map(|&id| (id, baked::load(id, livery(id), &mut meshes, &mut images, &mut materials))).collect();
    // What is built in code on a model's hull wears its colours as vertex colours, under the same livery.
    let vertex = StandardMaterial { perceptual_roughness: 0.6, ..default() };
    let mut hand_paint: HashMap<(&str, u32), baked::Pair> = HashMap::new();
    let mut hand = |meshes: &mut Assets<Mesh>, id: &'static str, lift: f32, m: meshes::MeshBuilder| {
        let b = &baked[id];
        let (material, charred) = hand_paint
            .entry((id, lift.to_bits()))
            .or_insert_with(|| baked::pair(&mut materials, vertex.clone(), &livery(id), Vec4::new(b.half_width, b.deck, lift, 0.0), Vec4::new(0.0, 0.0, 1.0, 0.0)))
            .clone();
        vec![Part { mesh: meshes.add(m.build()), material, charred }]
    };
    let mut vehicles = HashMap::new();
    for &k in VKind::VARIANTS {
        // The model whose hull it takes, and the hand-built top for those without a model of their own: the
        // Leopard 2R and the Wisent (a Leopard 1 hull, near enough a 2A4's), and the BMR-2 (a T-55's, here a T-64's).
        let (id, top) = match k {
            VKind::T80 => ("t80", None),
            VKind::T72 => ("t72", None),
            VKind::Bmp1 => ("bmp1", None),
            VKind::Bmp2 => ("bmp2", None),
            VKind::Mtlb => ("mtlb", None),
            VKind::Leo2a6 => ("leo2a6", None),
            VKind::Leo2r => ("leo2r", Some(meshes::leopard2r_top(WEST_GREEN))),
            VKind::Wisent => ("leo2r", Some(meshes::wisent_top(WEST_GREEN))),
            VKind::Bmr2 => ("t64", Some(meshes::bmr2_top(WEST_GREEN))),
            VKind::Bradley => ("bradley", None),
        };
        let (turret, turret_at, muzzle) = match top {
            Some((t, at)) => (hand(&mut meshes, id, at.y, t), at, Vec3::new(0.0, 0.6, -1.6)),
            None => (baked[id].turret.clone(), baked[id].turret_at, baked[id].muzzle),
        };
        let extra = match k {
            VKind::Leo2r => hand(&mut meshes, id, 0.0, meshes::full_plow(-4.4, 4.2)),
            VKind::Wisent => hand(&mut meshes, id, 0.0, meshes::full_plow(-4.1, 3.8)),
            VKind::Bmr2 => hand(&mut meshes, id, 0.0, meshes::kmt_rollers()),
            _ => vec![],
        };
        // The carousel tanks may carry a KMT-7 at the head of a column.
        let plow = k.carousel().then(|| hand(&mut meshes, id, 0.0, meshes::kmt7()));
        let b = &baked[id];
        let plow_hull = plow.map(|p| b.hull.iter().cloned().chain(p).collect());
        let hull = b.hull.iter().cloned().chain(extra).collect();
        vehicles.insert(k, VehicleMeshes { hull, plow_hull, turret, turret_at, muzzle });
    }
    let t64 = &baked["t64"];
    let ua_tank = VehicleMeshes { hull: t64.hull.clone(), plow_hull: None, turret: t64.turret.clone(), turret_at: t64.turret_at, muzzle: t64.muzzle };
    let mut figure = |coat, trousers, helmet, tape| {
        let f = meshes::figure(coat, trousers, helmet, Some(tape));
        (FigureMeshes { body: meshes.add(f.body.build()), leg: meshes.add(f.leg.build()), fallen: meshes.add(meshes::fallen(coat, trousers).build()) }, f.hip)
    };
    let ((ru, hip), (ua, _)) = if diorama().summer() {
        // Summer: no coats. Russian flora pattern, greyish; Ukrainian multicam-like, browner; tape on the arms.
        (figure([0.40, 0.42, 0.33], [0.36, 0.38, 0.30], [0.33, 0.35, 0.28], [0.8, 0.3, 0.25]), figure([0.45, 0.43, 0.32], [0.40, 0.38, 0.29], [0.36, 0.36, 0.27], [0.85, 0.8, 0.3]))
    } else {
        (figure([0.36, 0.37, 0.31], [0.33, 0.34, 0.29], [0.3, 0.32, 0.26], [0.8, 0.3, 0.25]), figure([0.34, 0.36, 0.27], [0.30, 0.32, 0.24], [0.28, 0.31, 0.22], [0.85, 0.8, 0.3]))
    };
    let mut raam = meshes::MeshBuilder::new();
    raam.cylinder(Vec3::ZERO, Vec3::Y * 0.12, 0.08, 6, [0.22, 0.24, 0.17]);
    let mut missile = meshes::MeshBuilder::new();
    missile.cylinder(Vec3::new(0.0, 0.0, 0.6), Vec3::new(0.0, 0.0, -1.2), 0.07, 6, [0.3, 0.32, 0.25]);
    let mut marker = meshes::MeshBuilder::new();
    marker.cylinder(Vec3::ZERO, Vec3::Y * 1.3, 0.025, 4, [0.3, 0.3, 0.28]);
    marker.boxc(Vec3::new(0.12, 1.2, 0.0), Vec3::new(0.24, 0.16, 0.01), [0.95, 0.45, 0.1]);
    let mut strip = meshes::MeshBuilder::new();
    strip.boxc(Vec3::new(0.0, 0.05, 0.0), Vec3::new(0.55, 0.1, 5.5), [0.12, 0.12, 0.11]);
    let mut wheel = meshes::MeshBuilder::new();
    wheel.cylinder(Vec3::new(-0.1, 0.37, 0.0), Vec3::new(0.2, 0.0, 0.0), 0.37, 10, [0.16, 0.16, 0.15]);
    commands.insert_resource(Models { vehicles, ru, ua, hip, raam: meshes.add(raam.build()), missile: meshes.add(missile.build()), ua_tank, marker: meshes.add(marker.build()), track_strip: meshes.add(strip.build()), road_wheel: meshes.add(wheel.build()) });
}
