//! Set dressing: the shelterbelts, the town's ruined panel blocks, the dachas, the power line, and the sky.

use crate::geo::{self, CHUNK, CHUNKS_X, HALF_X, HALF_Z, hash01};
use crate::meshes::{self, MeshBuilder};
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    light::CascadeShadowConfigBuilder,
    prelude::*,
    render::render_resource::{Extent3d, Face, TextureDimension, TextureFormat},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.70, 0.72, 0.74)))
            .insert_resource(GlobalAmbientLight { color: Color::srgb(0.80, 0.86, 1.0), brightness: 450.0, ..default() })
            .insert_resource(bevy::light::DirectionalLightShadowMap { size: 4096 })
            .add_systems(Startup, (spawn_light, spawn_belts, spawn_town.after(spawn_belts)))
            .add_systems(Update, daylight);
    }
}

/// Shared materials for vertex-coloured models.
#[derive(Resource, Clone)]
pub struct Palette {
    pub vertex: Handle<StandardMaterial>,
    pub charred: Handle<StandardMaterial>,
    pub branches: Handle<StandardMaterial>,
}

/// The steppe beyond the diorama: a coarse skirt out to 9 km so the box never shows its edge.
fn spawn_skirt(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) {
    let mut m = MeshBuilder::new();
    let n = 48;
    let ring = |r: f32, i: u32| {
        let a = i as f32 / n as f32 * std::f32::consts::TAU;
        Vec2::new(a.cos(), a.sin()) * r
    };
    let inner_at = |i: u32| {
        // Project the ring direction onto the map rectangle, slightly inside the edge.
        let d = ring(1.0, i);
        let t = ((HALF_X - 2.0) / d.x.abs().max(1e-4)).min((HALF_Z - 2.0) / d.y.abs().max(1e-4));
        d * t
    };
    for i in 0..n {
        let rings = [inner_at(i), inner_at(i + 1)];
        let mid = [ring(5000.0, i), ring(5000.0, i + 1)];
        let far = [ring(9000.0, i), ring(9000.0, i + 1)];
        // Carry each edge height straight out to the horizon, a little lower, so the steppe stays flat.
        let edge = [geo::height(rings[0].x, rings[0].y), geo::height(rings[1].x, rings[1].y)];
        let h = |p: Vec2| {
            let k = if p.distance(mid[0]) < 1.0 || p.distance(far[0]) < 1.0 { 0 } else { 1 };
            if p.distance(rings[0]) < 1.0 { edge[0] - 0.3 } else if p.distance(rings[1]) < 1.0 { edge[1] - 0.3 } else { edge[k] - 1.5 - (p.length() / 9000.0) * 6.0 }
        };
        let snow = [0.70, 0.72, 0.74];
        for (a, b) in [(rings, mid), (mid, far)] {
            let p = [a[0], a[1], b[1], b[0]].map(|q| Vec3::new(q.x, h(q), q.y));
            m.poly(&[p[0], p[3], p[2], p[1]], snow);
        }
    }
    commands.spawn((Mesh3d(meshes.add(m.build())), MeshMaterial3d(materials.add(StandardMaterial { perceptual_roughness: 0.95, ..default() }))));
}

fn spawn_light(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    spawn_skirt(&mut commands, &mut meshes, &mut materials);
    // An overcast February morning: a weak, low sun from the south-east through high cloud.
    commands.spawn((
        DirectionalLight { illuminance: 5200.0, shadow_maps_enabled: true, color: Color::srgb(1.0, 0.96, 0.9), shadow_depth_bias: 0.25, shadow_normal_bias: 3.5, ..default() },
        Transform::from_xyz(0.6, 0.45, 0.8).looking_at(Vec3::ZERO, Vec3::Y),
        // Long, hard winter shadows matter at drone range: the first cascade covers the near kilometre.
        CascadeShadowConfigBuilder { num_cascades: 2, first_cascade_far_bound: 900.0, maximum_distance: 2800.0, ..default() }.build(),
    ));
}

/// Bare winter branches: recursive strokes into an alpha mask.
fn branch_texture() -> Image {
    const N: usize = 256;
    let mut a = vec![0u8; N * N];
    fn stroke(a: &mut [u8], p: Vec2, dir: Vec2, len: f32, w: f32, depth: u32, seed: u32) {
        let steps = (len * 2.0) as i32;
        let mut q = p;
        for _ in 0..steps {
            q += dir * 0.5;
            let r = w.max(0.6);
            let (x0, x1) = ((q.x - r) as i32, (q.x + r) as i32 + 1);
            let (y0, y1) = ((q.y - r) as i32, (q.y + r) as i32 + 1);
            for y in y0.max(0)..y1.min(N as i32) {
                for x in x0.max(0)..x1.min(N as i32) {
                    if Vec2::new(x as f32, y as f32).distance(q) <= r {
                        a[y as usize * N + x as usize] = 255;
                    }
                }
            }
        }
        if depth == 0 {
            return;
        }
        let kids = 2 + (hash01(seed, 3) * 2.0) as u32;
        for k in 0..kids {
            let t = 0.35 + 0.6 * hash01(seed * 7 + k, 4);
            let start = p + dir * len * t;
            let ang = (hash01(seed * 11 + k, 5) - 0.5) * 1.6;
            let nd = Vec2::from_angle(ang).rotate(dir).normalize();
            stroke(a, start, nd, len * (0.45 + 0.25 * hash01(seed + k, 6)), w * 0.62, depth - 1, seed * 5 + k + 1);
        }
    }
    for t in 0..3 {
        let x = 100.0 + t as f32 * 28.0;
        stroke(&mut a, Vec2::new(x, N as f32 - 1.0), Vec2::new((t as f32 - 1.0) * 0.15, -1.0).normalize(), 120.0, 3.2, 5, t + 1);
    }
    let mut data = Vec::with_capacity(N * N * 4);
    for v in a {
        data.extend_from_slice(&[255, 255, 255, v]);
    }
    let mut img = Image::new(
        Extent3d { width: N as u32, height: N as u32, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::linear());
    img
}

pub fn spawn_belts(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let vertex = materials.add(StandardMaterial { perceptual_roughness: 0.9, ..default() });
    let charred = materials.add(StandardMaterial { base_color: Color::srgb(0.22, 0.2, 0.19), perceptual_roughness: 1.0, ..default() });
    let branches = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(branch_texture())),
        alpha_mode: AlphaMode::Mask(0.5),
        double_sided: true,
        cull_mode: None,
        perceptual_roughness: 1.0,
        ..default()
    });
    commands.insert_resource(Palette { vertex: vertex.clone(), charred, branches: branches.clone() });

    // Bucket trees by ground chunk so each bucket is one trunk mesh and one crown mesh.
    let n_chunks = CHUNKS_X * geo::CHUNKS_Z;
    let mut trunks: Vec<MeshBuilder> = (0..n_chunks).map(|_| MeshBuilder::new()).collect();
    let mut crowns: Vec<MeshBuilder> = (0..n_chunks).map(|_| MeshBuilder::new()).collect();
    let chunk_of = |p: Vec2| {
        let cx = ((p.x + HALF_X) / CHUNK).clamp(0.0, CHUNKS_X as f32 - 1.0) as usize;
        let cz = ((p.y + HALF_Z) / CHUNK).clamp(0.0, geo::CHUNKS_Z as f32 - 1.0) as usize;
        cz * CHUNKS_X + cx
    };
    let trunk_meshes: Vec<MeshBuilder> = (0..6).map(|k| meshes::trunk(7.0 + k as f32 * 1.6, 0.14 + k as f32 * 0.03)).collect();

    let mut id = 0u32;
    for (bi, belt) in geo::layout().belts.iter().enumerate() {
        let len = belt.line.length();
        let mut s = 0.0;
        while s < len {
            id += 1;
            s += 2.6 + hash01(id, 1) * 2.4;
            if belt.gaps.iter().any(|&(a, b)| s > a && s < b) {
                continue;
            }
            let rows = 3;
            for row in 0..rows {
                id += 1;
                if hash01(id, 2) < 0.18 {
                    continue; // dead gaps, shell-split trunks
                }
                let lat = (row as f32 - (rows - 1) as f32 * 0.5) * belt.width / rows as f32 + (hash01(id, 3) - 0.5) * 2.5;
                let p2 = belt.line.point(s) + belt.line.normal(s) * lat;
                let p = geo::ground(p2) - Vec3::Y * 0.2;
                let ci = chunk_of(p2);
                // Shelling has split and thinned the belts closest to the fighting.
                let shattered = hash01(id, 9) < 0.12 + 0.18 * (bi < 2) as i32 as f32;
                let k = (hash01(id, 4) * 6.0) as usize % 6;
                let mut xf = Transform::from_translation(p).with_rotation(Quat::from_rotation_y(hash01(id, 5) * 6.28));
                let h = 7.0 + k as f32 * 1.6;
                if shattered {
                    xf.scale = Vec3::new(1.0, 0.35 + 0.3 * hash01(id, 6), 1.0);
                    xf.rotation *= Quat::from_rotation_z((hash01(id, 7) - 0.5) * 0.5);
                }
                trunks[ci].append(&trunk_meshes[k], xf);
                if !shattered {
                    let w = 6.0 + hash01(id, 8) * 4.5;
                    let tone = 0.2 + hash01(id, 10) * 0.1;
                    meshes::crown_cards(&mut crowns[ci], p + Vec3::Y * (h * 0.3), w, h * 0.78, hash01(id, 11) * 3.0, [tone, tone * 0.93, tone * 0.86]);
                }
            }
            // Scrub between the rows.
            if hash01(id, 12) < 0.45 {
                let p2 = belt.line.point(s) + belt.line.normal(s) * ((hash01(id, 13) - 0.5) * belt.width * 1.2);
                let tone = 0.3 + hash01(id, 14) * 0.1;
                meshes::crown_cards(&mut crowns[chunk_of(p2)], geo::ground(p2) - Vec3::Y * 0.2, 3.0, 2.6, hash01(id, 15) * 3.0, [tone * 1.05, tone, tone * 0.85]);
            }
        }
    }
    for (t, c) in trunks.into_iter().zip(crowns) {
        if !t.is_empty() {
            commands.spawn((Mesh3d(meshes.add(t.build())), MeshMaterial3d(vertex.clone())));
        }
        if !c.is_empty() {
            commands.spawn((Mesh3d(meshes.add(c.build())), MeshMaterial3d(branches.clone()), bevy::light::NotShadowCaster));
        }
    }
}

/// Facade texture for the panel blocks: grey panels, a 4 × 3 grid of windows, most blown out and sooted.
fn facade_texture(seed: u32) -> Image {
    const W: usize = 192;
    const H: usize = 135; // 12 m × 8.4 m at 16 px/m
    let mut d = vec![0u8; W * H * 4];
    for y in 0..H {
        for x in 0..W {
            let (fx, fy) = (x as f32 / 16.0, y as f32 / 16.0);
            let panel = 0.56 + 0.05 * geo::value_noise(fx * 0.8, fy * 0.8, seed) + 0.03 * geo::value_noise(fx * 6.0, fy * 6.0, seed + 1);
            let mut c = [panel, panel * 0.98, panel * 0.95];
            // Panel seams.
            if (fx % 3.0) < 0.07 || (fy % 2.8) < 0.07 {
                c = [c[0] * 0.75, c[1] * 0.75, c[2] * 0.75];
            }
            // Windows: 1.4 × 1.5 m, one per 3 m bay per floor.
            let (bx, by) = ((fx / 3.0).floor(), (fy / 2.8).floor());
            let (lx, ly) = (fx - bx * 3.0, fy - by * 2.8);
            let win = (lx > 0.8 && lx < 2.2) && (ly > 0.7 && ly < 2.2);
            let id = (bx as u32) * 7 + (by as u32) * 13 + seed * 101;
            let burned = hash01(id, 33) < 0.55;
            if win {
                c = if burned { [0.05, 0.05, 0.05] } else if hash01(id, 34) < 0.5 { [0.18, 0.2, 0.22] } else { [0.32, 0.30, 0.26] };
            } else if burned && lx > 0.6 && lx < 2.4 && ly > 2.2 {
                // Soot above a burned-out window.
                let t = 1.0 - ((ly - 2.2) / 0.6).clamp(0.0, 1.0);
                let k = 1.0 - 0.6 * t;
                c = [c[0] * k, c[1] * k, c[2] * k];
            }
            // Shell scars.
            let scar = geo::value_noise(fx * 1.3, fy * 1.3, seed + 9);
            if scar > 0.55 {
                c = [c[0] * 0.6, c[1] * 0.58, c[2] * 0.55];
            }
            let o = (y * W + x) * 4;
            d[o] = (c[0] * 255.0) as u8;
            d[o + 1] = (c[1] * 255.0) as u8;
            d[o + 2] = (c[2] * 255.0) as u8;
            d[o + 3] = 255;
        }
    }
    let mut img = Image::new(Extent3d { width: W as u32, height: H as u32, depth_or_array_layers: 1 }, TextureDimension::D2, d, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default());
    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..ImageSamplerDescriptor::linear()
    });
    img
}

#[derive(Resource)]
pub struct ObservationPost {
    pub eye: Vec3,
    pub building: Vec2,
}

fn spawn_town(mut commands: Commands, palette: Res<Palette>, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let facades: Vec<Handle<StandardMaterial>> = (0..3)
        .map(|k| materials.add(StandardMaterial { base_color_texture: Some(images.add(facade_texture(k + 1))), perceptual_roughness: 0.95, ..default() }))
        .collect();
    let solid = materials.add(StandardMaterial { perceptual_roughness: 0.95, ..default() });
    let lay = geo::layout();
    let mut cards_for_town = MeshBuilder::new();

    // Rows of 5- and 9-storey blocks on the ridge. The southern row, facing the fields, took the worst of it.
    let mut op_eye = Vec3::ZERO;
    let mut op_building = Vec2::ZERO;
    let mut id = 0u32;
    for (ri, z) in [-1225.0f32, -1330.0, -1450.0, -1580.0, -1700.0].into_iter().enumerate() {
        let mut x = lay.town.min.x + 120.0;
        while x < lay.town.max.x - 60.0 {
            id += 1;
            let sections = 3 + (hash01(id, 1) * 4.0) as u32;
            let len = sections as f32 * 15.0;
            // The observers' block: the front-row block spanning x = -300, nine storeys, still standing.
            let is_op = ri == 0 && x <= -300.0 && x + len >= -300.0;
            let floors = if is_op || hash01(id, 2) < 0.55 { 9 } else { 5 };
            let cx = x + len * 0.5;
            // A few blocks stand end-on, but not in the two front rows, where they would cross the rows.
            let turned = ri >= 2 && sections <= 4 && hash01(id, 4) < 0.2;
            let rot = if is_op { 0.0 } else { (hash01(id, 3) - 0.5) * 0.08 + if turned { std::f32::consts::FRAC_PI_2 } else { 0.0 } };
            let exposure = if ri == 0 { 0.8 } else { 0.45 - ri as f32 * 0.07 };
            let damage: Vec<f32> = (0..sections)
                .map(|s| {
                    let r = hash01(id * 10 + s, 5);
                    // The OP block is hit only at its far end.
                    if is_op && s + 1 < sections { 0.0 } else if r < exposure * 0.5 { r / (exposure * 0.5) } else { 0.0 }
                })
                .collect();
            let (facade, body) = meshes::panel_block(sections, floors, &damage);
            let ground_y = (0..=4).map(|k| geo::height(cx - len * 0.5 + k as f32 * len / 4.0, z)).fold(f32::MAX, f32::min);
            let xf = Transform::from_xyz(cx, ground_y - 0.5, z).with_rotation(Quat::from_rotation_y(rot));
            commands.spawn((Mesh3d(meshes.add(facade.build())), MeshMaterial3d(facades[(id % 3) as usize].clone()), xf));
            commands.spawn((Mesh3d(meshes.add(body.build())), MeshMaterial3d(solid.clone()), xf));
            if is_op {
                // At the southern parapet, between the roof housings of the first two sections.
                op_eye = Vec3::new(x + 3.0, ground_y - 0.5 + 9.0 * 2.8 + 1.6, z + 3.9);
                op_building = Vec2::new(cx, z);
            }
            x += len + 30.0 + hash01(id, 6) * 40.0;
        }
    }
    if op_eye == Vec3::ZERO {
        op_eye = Vec3::new(-300.0, geo::height(-300.0, -1225.0) + 27.0, -1230.0);
        op_building = Vec2::new(-300.0, -1225.0);
    }
    info!("OP at {op_eye:?}");
    commands.insert_resource(ObservationPost { eye: op_eye, building: op_building });

    // The observers' roof: a parapet at the southern edge, sandbags, broken slabs, an old antenna.
    let roof = op_eye.y - 1.6;
    let mut r = MeshBuilder::new();
    let pz = op_eye.z + 2.15;
    r.boxc(Vec3::new(op_eye.x + 12.0, roof + 0.55, pz), Vec3::new(30.0, 1.1, 0.3), [0.5, 0.49, 0.47]);
    for k in 0..7 {
        let x = op_eye.x + 2.5 + k as f32 * 0.6;
        r.cuboid(Vec3::new(x, roof + 1.25, pz - 0.1), Vec3::new(0.32, 0.14, 0.22), Quat::from_rotation_y((k as f32 * 1.7).sin() * 0.2), [0.42, 0.40, 0.33]);
        if k % 2 == 0 {
            r.cuboid(Vec3::new(x + 0.3, roof + 1.5, pz - 0.1), Vec3::new(0.32, 0.13, 0.22), Quat::from_rotation_y((k as f32).cos() * 0.25), [0.40, 0.38, 0.31]);
        }
    }
    for k in 0..14 {
        let a = Vec3::new(op_eye.x + 2.0 + hash01(k, 91) * 18.0, roof + 0.35, op_eye.z - 1.5 - hash01(k, 92) * 9.0);
        r.cuboid(a, Vec3::new(0.3 + hash01(k, 93) * 0.8, 0.12 + hash01(k, 94) * 0.2, 0.3 + hash01(k, 95) * 0.6), Quat::from_rotation_y(hash01(k, 96) * 3.0) * Quat::from_rotation_x((hash01(k, 97) - 0.5) * 0.6), [0.38, 0.37, 0.35]);
    }
    r.cylinder(Vec3::new(op_eye.x + 6.0, roof, op_eye.z - 3.0), Vec3::new(0.3, 4.5, 0.2), 0.04, 4, [0.25, 0.25, 0.25]);
    r.boxc(Vec3::new(op_eye.x + 3.5, roof + 1.0, op_eye.z - 4.0), Vec3::new(2.4, 2.0, 2.4), [0.43, 0.42, 0.40]);
    commands.spawn((Mesh3d(meshes.add(r.build())), MeshMaterial3d(solid.clone())));

    // Courtyards: bare trees between the rows, and burnt-out cars.
    let mut yard = MeshBuilder::new();
    let mut cars = MeshBuilder::new();
    for k in 0..1400u32 {
        let x = lay.town.min.x + 60.0 + hash01(k, 81) * (lay.town.max.x - lay.town.min.x - 120.0);
        let rows = [-1225.0f32, -1330.0, -1450.0, -1580.0, -1700.0];
        let row = rows[(hash01(k, 82) * 5.0) as usize % 5];
        let z = row + if hash01(k, 83) < 0.5 { 1.0 } else { -1.0 } * (14.0 + hash01(k, 84) * 30.0);
        if z < -HALF_Z + 20.0 {
            continue;
        }
        let p = geo::ground(Vec2::new(x, z)) - Vec3::Y * 0.2;
        if k % 9 == 0 {
            let yaw = hash01(k, 85) * 6.28;
            let rot = Quat::from_rotation_y(yaw);
            let c = if hash01(k, 86) < 0.7 { [0.2, 0.17, 0.15] } else { [0.45, 0.42, 0.38] };
            cars.cuboid(p + Vec3::Y * 0.6, Vec3::new(0.85, 0.45, 2.1), rot, c);
            cars.cuboid(p + Vec3::Y * 1.2 + rot * Vec3::new(0.0, 0.0, 0.2), Vec3::new(0.75, 0.3, 1.0), rot, [c[0] * 0.8, c[1] * 0.8, c[2] * 0.8]);
        } else {
            let h = 6.0 + hash01(k, 87) * 7.0;
            let tone = 0.22 + hash01(k, 88) * 0.08;
            yard.append(&meshes::trunk(h, 0.15 + hash01(k, 89) * 0.1), Transform::from_translation(p));
            meshes::crown_cards(&mut cards_for_town, p + Vec3::Y * h * 0.3, 5.0 + hash01(k, 90) * 4.0, h * 0.8, hash01(k, 98) * 3.0, [tone, tone * 0.94, tone * 0.88]);
        }
    }
    commands.spawn((Mesh3d(meshes.add(yard.build())), MeshMaterial3d(solid.clone())));
    commands.spawn((Mesh3d(meshes.add(cars.build())), MeshMaterial3d(solid.clone())));
    commands.spawn((Mesh3d(meshes.add(cards_for_town.build())), MeshMaterial3d(palette.branches.clone()), bevy::light::NotShadowCaster));

    // Dachas: garden plots, most houses already wrecked; and the villages down in the valley.
    let mut dachas = MeshBuilder::new();
    let mut place_village = |rect: geo::Rect, spacing: f32, ruin: f32, seed: u32, out: &mut MeshBuilder| {
        let mut z = rect.min.y + spacing * 0.5;
        let mut k = 0;
        while z < rect.max.y {
            let mut x = rect.min.x + spacing * 0.5;
            while x < rect.max.x {
                k += 1;
                let h = hash01(k + seed, 1);
                if h < 0.8 {
                    let p = Vec2::new(x + (hash01(k + seed, 2) - 0.5) * spacing * 0.4, z + (hash01(k + seed, 3) - 0.5) * spacing * 0.4);
                    if geo::in_bounds(p) && !geo::layout().roads.iter().any(|r| r.line.project(p).0 < r.width + 6.0) {
                        let m = meshes::dacha(k + seed, hash01(k + seed, 4) < ruin);
                        out.append(&m, Transform::from_translation(geo::ground(p) - Vec3::Y * 0.1).with_rotation(Quat::from_rotation_y((hash01(k + seed, 5) - 0.5) * 0.3)));
                    }
                }
                x += spacing;
            }
            z += spacing;
        }
    };
    place_village(lay.dachas, 26.0, 0.7, 0, &mut dachas);
    place_village(lay.pavlivka, 34.0, 0.6, 500, &mut dachas);
    place_village(lay.mykilske, 34.0, 0.5, 900, &mut dachas);
    commands.spawn((Mesh3d(meshes.add(dachas.build())), MeshMaterial3d(solid.clone())));

    // The power line along the T-0509.
    let road = &lay.roads[0].line;
    let mut poles = MeshBuilder::new();
    let mut s = 0.0;
    let mut k = 0;
    while s < road.length() {
        k += 1;
        let p = road.point(s) + road.normal(s) * 11.0;
        let lean = if hash01(k, 77) < 0.2 { (hash01(k, 78) - 0.5) * 0.7 } else { 0.0 };
        if hash01(k, 79) > 0.08 {
            poles.append(&meshes::pole(lean), Transform::from_translation(geo::ground(p)).with_rotation(Quat::from_rotation_y(road.tangent(s).to_angle())));
        }
        s += 42.0;
    }
    commands.spawn((Mesh3d(meshes.add(poles.build())), MeshMaterial3d(solid.clone())));
    let _ = Face::Back;
}

/// The light over the day: grey dawn at 07:40, a flat overcast noon, and dusk closing in after 16:00, when the
/// burning wrecks start to show. Early February at 47.8 N: sunrise about 07:50, sunset about 17:40 (ERA5/dossier).
fn daylight(
    script: Res<crate::director::Script>,
    mut sun: Query<(&mut DirectionalLight, &mut Transform)>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut clear: ResMut<ClearColor>,
    mut fog: Query<&mut bevy::pbr::DistanceFog>,
) {
    let dbg = std::env::var("VUHLEDAR_DAYDBG").unwrap_or_default();
    if dbg == "off" {
        return;
    }
    let minutes = crate::director::DAY_START_MIN + script.t * crate::director::COMPRESSION;
    let h = minutes / 60.0;
    // Solar elevation, roughly: 0 at 07:50 and 17:40, 24 degrees at 12:45.
    let elev_true = (((h - 7.83) / (17.67 - 7.83)) * std::f32::consts::PI).sin().max(0.0) * 24f32.to_radians();
    // The light never drops below 12 degrees: at grazing incidence the steppe shadows itself into black, and the
    // footage of the day is late-morning light anyway. The true elevation still sets the brightness.
    let elev = elev_true.max(12f32.to_radians());
    let azim = (h - 12.75) / 5.0 * 1.2; // south-east in the morning, south-west after
    let day = (elev_true / 24f32.to_radians()).clamp(0.0, 1.0);
    // A camera's auto-exposure flattens the day: twilight over snow still reads light grey, only cooler.
    let light = 0.74 + 0.26 * day.powf(0.5);
    for (mut l, mut tf) in &mut sun {
        // A clear, frosty day (the 6 February footage shows hard sun): strong direct light, a cold blue sky fill.
        l.illuminance = 15000.0 * (0.8 + 0.2 * day) * light;
        let dir = Vec3::new(azim.sin() * elev.cos(), elev.sin().max(0.08), azim.cos() * elev.cos());
        if dbg != "nosun" {
            *tf = Transform::from_translation(dir).looking_at(Vec3::ZERO, Vec3::Y);
        }
        l.color = Color::srgb(1.0, 0.9 + 0.08 * day, 0.8 + 0.15 * day);
    }
    ambient.brightness = 450.0 * light;
    let sky = Color::srgb(0.70 * light, 0.75 * light + 0.01, 0.82 * light + 0.04);
    clear.0 = sky;
    for mut f in &mut fog {
        f.color = sky;
    }
}
