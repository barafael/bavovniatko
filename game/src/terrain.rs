//! The ground: chunked terrain meshes carrying a painted ground texture (one texel per metre). The texture is
//! the battle's memory. Roads, furrows under thin snow and belts are painted at start; every crater, scorch
//! and track mark is stamped in afterwards and never cleaned up.

use crate::geo::{self, CHUNK, CHUNKS_X, CHUNKS_Z, HALF_X, HALF_Z, RoadKind, TEX_PER_CHUNK, fbm, value_noise};
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageSampler, ImageSamplerDescriptor},
    mesh::{Indices, PrimitiveTopology},
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, Extent3d, TextureDimension, TextureFormat},
    shader::ShaderRef,
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<GroundMaterial>::default())
            .init_resource::<PaintQueue>()
            .add_systems(Startup, spawn_terrain)
            .add_systems(PostUpdate, apply_paint);
    }
}

/// The ground shader adds what a 1 m texel cannot hold: furrows, stubble rows and grain in the snow.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct GroundDetail {
    /// x: furrow strength, y: snow grain strength.
    #[uniform(100)]
    params: Vec4,
}

impl MaterialExtension for GroundDetail {
    fn fragment_shader() -> ShaderRef {
        "embedded://vuhledar/shaders/ground.wgsl".into()
    }
}

pub type GroundMaterial = ExtendedMaterial<StandardMaterial, GroundDetail>;

#[derive(Resource)]
pub struct Ground {
    images: Vec<Handle<Image>>,
}

#[derive(Clone, Copy)]
pub enum Paint {
    Crater { at: Vec2, radius: f32, seed: u32 },
    Scorch { at: Vec2, radius: f32 },
    Track { at: Vec2, dir: Vec2, gauge: f32 },
    Footprints { at: Vec2 },
}

#[derive(Resource, Default)]
pub struct PaintQueue {
    pub ops: Vec<Paint>,
    timer: f32,
}

impl PaintQueue {
    pub fn push(&mut self, p: Paint) {
        self.ops.push(p);
    }
}

const W: usize = CHUNKS_X * TEX_PER_CHUNK;
const H: usize = CHUNKS_Z * TEX_PER_CHUNK;

type Px = [f32; 3];

const SNOW: Px = [0.80, 0.82, 0.84];
const EARTH: Px = [0.36, 0.31, 0.26];
const DARK_EARTH: Px = [0.20, 0.17, 0.15];

fn mix(a: Px, b: Px, t: f32) -> Px {
    let t = t.clamp(0.0, 1.0);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

fn tex_of(p: Vec2) -> (f32, f32) {
    (p.x + HALF_X, p.y + HALF_Z)
}

/// Furrow direction per field block, as the alpha code the ground shader reads (1..=255; 0 = no furrows).
fn furrow_code(x: f32, z: f32) -> u8 {
    let bx = ((x + HALF_X) / 1200.0).floor();
    let bz = ((z + 760.0) / 550.0).floor();
    let t = geo::hash01((bx as i32 * 7 + bz as i32 * 13 + 100) as u32, 5);
    1 + (t * 254.0) as u8
}

/// Base ground colour at a world point: snow over ploughed fields, thinner in patches where the wind scoured it.
fn base_colour(x: f32, z: f32) -> Px {
    // Thin snow, patchy at every scale: the wind strips it off the crests, weeks of shelling churned the rest.
    // Earth shows through in dark patches (compare the February drone footage).
    let broad = fbm(x / 45.0, z / 45.0, 3, 3);
    let fine = fbm(x / 5.0, z / 5.0, 3, 4);
    // More exposed earth in the fought-over ground south of the first tree line.
    let churn = 0.18 * (1.0 - ((z + 700.0) / 250.0).clamp(0.0, 1.0)).max(0.0) - 0.12 * ((z + 650.0) / 900.0).clamp(0.0, 1.0);
    // Mostly white; small, sharp-edged dark patches where the thresholded noise dips.
    let v = 0.62 + broad * 0.45 + fine * 0.5 - churn * 1.2;
    let cover = ((v - 0.18) / 0.14).clamp(0.0, 1.0);
    let earth = mix([0.27, 0.25, 0.22], [0.44, 0.42, 0.39], value_noise(x / 4.0, z / 4.0, 12) * 0.5 + 0.5);
    let mut col = mix(earth, SNOW, cover);
    let n = value_noise(x / 3.0, z / 3.0, 9) * 0.02;
    col = [col[0] + n, col[1] + n, col[2] + n * 1.2];
    col
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<GroundMaterial>>,
) {
    let mut px = vec![[0.0f32; 3]; W * H];
    // Pixels where something other than ploughed field lies (roads, belts, craters, villages): no furrows.
    let mut bare = vec![false; W * H];

    // Base: paint rows in parallel.
    let rows_per = H.div_ceil(16);
    std::thread::scope(|scope| {
        for (ci, band) in px.chunks_mut(rows_per * W).enumerate() {
            scope.spawn(move || {
                for (i, p) in band.iter_mut().enumerate() {
                    let row = ci * rows_per + i / W;
                    let col = i % W;
                    let x = col as f32 + 0.5 - HALF_X;
                    let z = row as f32 + 0.5 - HALF_Z;
                    *p = base_colour(x, z);
                }
            });
        }
    });

    let lay = geo::layout();
    let stamp = |px: &mut Vec<Px>, bare: &mut Vec<bool>, p: Vec2, r: f32, f: &dyn Fn(f32, Px, Vec2) -> Px| {
        let (cx, cz) = tex_of(p);
        let (x0, x1) = ((cx - r).floor().max(0.0) as usize, ((cx + r).ceil() as usize).min(W - 1));
        let (z0, z1) = ((cz - r).floor().max(0.0) as usize, ((cz + r).ceil() as usize).min(H - 1));
        for zz in z0..=z1 {
            for xx in x0..=x1 {
                let d = Vec2::new(xx as f32 + 0.5 - cx, zz as f32 + 0.5 - cz);
                let dl = d.length() / r;
                if dl <= 1.0 {
                    let i = zz * W + xx;
                    px[i] = f(dl, px[i], Vec2::new(xx as f32 - HALF_X, zz as f32 - HALF_Z));
                    bare[i] = true;
                }
            }
        }
    };

    // The river: ice and reeds.
    let river = &lay.river;
    let mut s = 0.0;
    while s < river.length() {
        let p = river.point(s);
        stamp(&mut px, &mut bare, p, 26.0, &|d, c, w| {
            let reeds = value_noise(w.x / 2.0, w.y / 2.0, 3) * 0.5 + 0.5;
            if d < 0.35 { mix([0.63, 0.67, 0.70], [0.5, 0.55, 0.58], value_noise(w.x / 6.0, w.y / 6.0, 8)) } else { mix(c, [0.50, 0.45, 0.36], (1.0 - d) * reeds * 1.4) }
        });
        s += 3.0;
    }

    // Shelterbelts: leaf litter, scrub and trampled snow under the trees.
    for belt in &lay.belts {
        let mut s = 0.0;
        while s < belt.line.length() {
            let p = belt.line.point(s);
            let r = belt.width * 0.65;
            stamp(&mut px, &mut bare, p, r, &|d, c, w| {
                let n = value_noise(w.x / 3.0, w.y / 3.0, 21) * 0.5 + 0.5;
                mix(c, mix([0.42, 0.39, 0.35], [0.30, 0.27, 0.24], n), (1.0 - d * d) * 0.85)
            });
            s += 2.5;
        }
    }

    // Roads: churned mud on the dirt roads (columns have been using them for weeks), asphalt on the T-0509.
    for road in &lay.roads {
        let mut s = 0.0;
        let r = road.width * 0.5 + 1.5;
        while s < road.line.length() {
            let p = road.line.point(s);
            let nrm = road.line.normal(s);
            let kind = road.kind;
            stamp(&mut px, &mut bare, p, r, &|d, c, w| {
                let lat = (w - p).dot(nrm).abs();
                match kind {
                    RoadKind::Asphalt => {
                        let edge_snow = value_noise(w.x / 4.0, w.y / 4.0, 5) * 0.5 + 0.5;
                        if d < 0.8 { mix([0.29, 0.29, 0.30], SNOW, edge_snow * 0.25) } else { mix(c, [0.55, 0.53, 0.50], 0.6) }
                    }
                    RoadKind::Dirt | RoadKind::Track => {
                        // In snow a dirt road is two dark ruts with trampled grey between, not a solid band.
                        let rut = (1.0 - ((lat - 1.4).abs() / 0.55).min(1.0)).powf(0.7);
                        let trampled = [0.60, 0.58, 0.55];
                        let base = mix(c, trampled, (1.0 - d.powi(3)) * if kind == RoadKind::Dirt { 0.7 } else { 0.4 });
                        let mud = mix(EARTH, DARK_EARTH, 0.5 + value_noise(w.x / 2.5, w.y / 2.5, 6) * 0.5);
                        mix(base, mud, rut * if kind == RoadKind::Dirt { 0.85 } else { 0.5 })
                    }
                }
            });
            s += 1.0;
        }
    }

    // Villages and the town: trampled, dirty snow, gardens.
    for (rect, dirt) in [(lay.town, 0.45), (lay.dachas, 0.35), (lay.pavlivka, 0.3), (lay.mykilske, 0.3)] {
        let (a, b) = (tex_of(rect.min), tex_of(rect.max));
        for zz in (a.1.max(0.0) as usize)..(b.1 as usize).min(H) {
            for xx in (a.0.max(0.0) as usize)..(b.0 as usize).min(W) {
                let w = Vec2::new(xx as f32 - HALF_X, zz as f32 - HALF_Z);
                let n = fbm(w.x / 25.0, w.y / 25.0, 3, 31) * 0.5 + 0.5;
                let i = zz * W + xx;
                px[i] = mix(px[i], [0.50, 0.48, 0.45], dirt * n);
                bare[i] = true;
            }
        }
    }

    // Weeks of shelling before this morning: old craters, densest along the belts and roads.
    let mut k = 0u32;
    for (wi, (p, _)) in lay.old_wrecks.iter().enumerate() {
        for j in 0..10 {
            let a = geo::hash01(wi as u32 * 100 + j, 41) * std::f32::consts::TAU;
            let d = geo::hash01(wi as u32 * 100 + j, 42) * 70.0;
            crater(&mut px, &mut bare, *p + Vec2::from_angle(a) * d, 3.0 + geo::hash01(j, 43) * 3.0, k);
            k += 1;
        }
    }
    for belt in &lay.belts {
        let n = (belt.line.length() / 35.0) as u32;
        for _ in 0..n {
            let s = geo::hash01(k, 51) * belt.line.length();
            let off = (geo::hash01(k, 52) - 0.5) * 70.0;
            let p = belt.line.point(s) + belt.line.normal(s) * off;
            crater(&mut px, &mut bare, p, 2.0 + geo::hash01(k, 53) * 3.5, k);
            k += 1;
        }
    }
    // Scattered harassing fire in clusters (a battery's sheaf lands as a group).
    for _ in 0..90 {
        let c = Vec2::new((geo::hash01(k, 61) * 2.0 - 1.0) * HALF_X, (geo::hash01(k, 62) * 1.6 - 1.0) * HALF_Z);
        k += 1;
        for _ in 0..(2 + (geo::hash01(k, 64) * 5.0) as u32) {
            let p = c + Vec2::new(geo::hash01(k, 65) - 0.5, geo::hash01(k, 66) - 0.5) * 90.0;
            crater(&mut px, &mut bare, p, 1.0 + geo::hash01(k, 63).powi(2) * 3.0, k);
            k += 1;
        }
    }

    // Upload as chunk textures and meshes.
    let mat_template = StandardMaterial { perceptual_roughness: 0.93, reflectance: 0.3, ..default() };
    let detail = GroundDetail { params: Vec4::new(1.0, 1.0, 0.0, 0.0) };
    let mut handles = Vec::new();
    let verts_per_side = 65usize;
    let step = CHUNK / (verts_per_side - 1) as f32;
    for cz in 0..CHUNKS_Z {
        for cx in 0..CHUNKS_X {
            let mut data = Vec::with_capacity(TEX_PER_CHUNK * TEX_PER_CHUNK * 4);
            for row in 0..TEX_PER_CHUNK {
                for col in 0..TEX_PER_CHUNK {
                    let i = (cz * TEX_PER_CHUNK + row) * W + cx * TEX_PER_CHUNK + col;
                    let (x, z) = ((cx * TEX_PER_CHUNK + col) as f32 - HALF_X, (cz * TEX_PER_CHUNK + row) as f32 - HALF_Z);
                    let code = if bare[i] { 0 } else { furrow_code(x, z) };
                    data.extend_from_slice(&to_srgb8(px[i], code));
                }
            }
            let mut image = Image::new(
                Extent3d { width: TEX_PER_CHUNK as u32, height: TEX_PER_CHUNK as u32, depth_or_array_layers: 1 },
                TextureDimension::D2,
                data,
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
            );
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::linear());
            let handle = images.add(image);
            handles.push(handle.clone());

            // Mesh: heights sampled in world space so chunk edges match exactly.
            let x0 = -HALF_X + cx as f32 * CHUNK;
            let z0 = -HALF_Z + cz as f32 * CHUNK;
            let mut pos = Vec::with_capacity(verts_per_side * verts_per_side);
            let mut nor = Vec::with_capacity(verts_per_side * verts_per_side);
            let mut uv = Vec::with_capacity(verts_per_side * verts_per_side);
            for j in 0..verts_per_side {
                for i in 0..verts_per_side {
                    let (x, z) = (x0 + i as f32 * step, z0 + j as f32 * step);
                    pos.push([x, geo::height(x, z), z]);
                    nor.push(geo::normal(x, z).to_array());
                    uv.push([i as f32 / (verts_per_side - 1) as f32, j as f32 / (verts_per_side - 1) as f32]);
                }
            }
            let mut idx = Vec::new();
            for j in 0..verts_per_side as u32 - 1 {
                for i in 0..verts_per_side as u32 - 1 {
                    let a = j * verts_per_side as u32 + i;
                    let b = a + 1;
                    let c = a + verts_per_side as u32;
                    let d = c + 1;
                    idx.extend_from_slice(&[a, c, b, b, c, d]);
                }
            }
            let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
                .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, nor)
                .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
                .with_inserted_indices(Indices::U32(idx));
            commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(materials.add(ExtendedMaterial { base: StandardMaterial { base_color_texture: Some(handle), ..mat_template.clone() }, extension: detail.clone() })),
                Transform::IDENTITY,
                Name::new(format!("ground {cx},{cz}")),
            ));
        }
    }
    commands.insert_resource(Ground { images: handles });
}

fn to_srgb8(c: Px, code: u8) -> [u8; 4] {
    [(c[0].clamp(0.0, 1.0) * 255.0) as u8, (c[1].clamp(0.0, 1.0) * 255.0) as u8, (c[2].clamp(0.0, 1.0) * 255.0) as u8, code]
}

fn from_srgb8(b: &[u8]) -> Px {
    [b[0] as f32 / 255.0, b[1] as f32 / 255.0, b[2] as f32 / 255.0]
}

/// Crater shape: a dark wet core, a raised rim of earth, and thrown dirt splashed over the snow in rays.
fn crater_colour(d: f32, c: Px, w: Vec2, centre: Vec2, seed: u32) -> Px {
    let ang = (w - centre).to_angle();
    let rays = (value_noise(ang * 3.0, (seed % 997) as f32, 71) * 0.5 + 0.5).powf(1.5);
    // Wobble the radius so no crater is a perfect disc.
    let d = d * (1.0 + 0.25 * value_noise(ang * 2.0 + (seed % 997) as f32, 0.5, 72));
    if d < 0.45 {
        mix(DARK_EARTH, [0.12, 0.10, 0.09], value_noise(w.x, w.y, seed) * 0.5 + 0.5)
    } else if d < 0.6 {
        mix(c, EARTH, 0.9)
    } else {
        // Thrown clods: a speckle of dark dots over the snow, densest near the rim and along the rays.
        let throw = (1.0 - (d - 0.6) / 0.4).powf(1.2) * (0.35 + 0.65 * rays);
        let clod = geo::hash01((w.x.floor() as i32 * 7919 + w.y.floor() as i32 * 104_729) as u32, seed % 991);
        if clod < throw * 0.75 { mix(c, [0.16, 0.13, 0.11], 0.85) } else { mix(c, [0.48, 0.43, 0.38], throw * 0.35) }
    }
}

fn crater(px: &mut [Px], bare: &mut [bool], at: Vec2, radius: f32, seed: u32) {
    let r = radius * 3.2;
    let (cx, cz) = tex_of(at);
    let (x0, x1) = ((cx - r).floor().max(0.0) as usize, ((cx + r).ceil() as usize).min(W - 1));
    let (z0, z1) = ((cz - r).floor().max(0.0) as usize, ((cz + r).ceil() as usize).min(H - 1));
    for zz in z0..=z1 {
        for xx in x0..=x1 {
            let w = Vec2::new(xx as f32 + 0.5 - HALF_X, zz as f32 + 0.5 - HALF_Z);
            let d = w.distance(at) / r;
            if d <= 1.0 {
                let i = zz * W + xx;
                px[i] = crater_colour(d, px[i], w, at, seed);
                bare[i] = d < 0.75;
            }
        }
    }
}

/// Apply queued paint at most a few times a second, re-uploading only the chunks that changed.
fn apply_paint(time: Res<Time>, mut queue: ResMut<PaintQueue>, ground: Option<Res<Ground>>, mut images: ResMut<Assets<Image>>) {
    let Some(ground) = ground else { return };
    queue.timer -= time.delta_secs();
    if queue.ops.is_empty() || queue.timer > 0.0 {
        return;
    }
    queue.timer = 0.2;
    let ops = std::mem::take(&mut queue.ops);
    let mut seed = 1000u32;
    for op in ops {
        seed = seed.wrapping_add(7919);
        let (at, r) = match op {
            Paint::Crater { at, radius, .. } => (at, radius * 3.2),
            Paint::Scorch { at, radius } => (at, radius),
            Paint::Track { at, gauge, .. } => (at, gauge * 0.5 + 0.8),
            Paint::Footprints { at } => (at, 0.8),
        };
        let (cx, cz) = tex_of(at);
        let (x0, x1) = ((cx - r).floor().max(0.0) as i64, ((cx + r).ceil() as i64).min(W as i64 - 1));
        let (z0, z1) = ((cz - r).floor().max(0.0) as i64, ((cz + r).ceil() as i64).min(H as i64 - 1));
        if x0 > x1 || z0 > z1 {
            continue;
        }
        // Visit each overlapped chunk once, so each image is fetched (and flagged for upload) once per op.
        let (x0, x1, z0, z1) = (x0 as usize, x1 as usize, z0 as usize, z1 as usize);
        for chz in z0 / TEX_PER_CHUNK..=z1 / TEX_PER_CHUNK {
            for chx in x0 / TEX_PER_CHUNK..=x1 / TEX_PER_CHUNK {
                let chunk = chz * CHUNKS_X + chx;
                let Some(mut img) = images.get_mut(&ground.images[chunk]) else { continue };
                let Some(data) = img.data.as_mut() else { continue };
                for zz in z0.max(chz * TEX_PER_CHUNK)..=z1.min((chz + 1) * TEX_PER_CHUNK - 1) {
                    for xx in x0.max(chx * TEX_PER_CHUNK)..=x1.min((chx + 1) * TEX_PER_CHUNK - 1) {
                        let w = Vec2::new(xx as f32 + 0.5 - HALF_X, zz as f32 + 0.5 - HALF_Z);
                        let d = w.distance(at) / r;
                        if d > 1.0 {
                            continue;
                        }
                        let o = ((zz % TEX_PER_CHUNK) * TEX_PER_CHUNK + xx % TEX_PER_CHUNK) * 4;
                        let c = from_srgb8(&data[o..o + 4]);
                        let n = match op {
                            Paint::Crater { at, seed: s, .. } => crater_colour(d, c, w, at, s),
                            Paint::Scorch { .. } => mix(c, [0.10, 0.09, 0.08], (1.0 - d) * 0.8 * (0.6 + 0.4 * value_noise(w.x / 2.0, w.y / 2.0, seed))),
                            Paint::Track { dir, gauge, .. } => {
                                // Tracks compact the snow to a fixed tone; they never darken what is already darker.
                                let lat = (w - at).perp_dot(dir).abs();
                                let on_track = ((lat - gauge * 0.5).abs() < 0.5) as i32 as f32;
                                let track = [0.55, 0.52, 0.48];
                                let lum = |p: Px| p[0] * 0.3 + p[1] * 0.59 + p[2] * 0.11;
                                if lum(c) > lum(track) { mix(c, track, on_track * 0.6) } else { c }
                            }
                            Paint::Footprints { .. } => if c[0] > 0.62 { mix(c, [0.62, 0.60, 0.58], 0.3 * (1.0 - d)) } else { c },
                        };
                        // Craters and scorch wipe out the furrows; tracks and footprints don't.
                        let code = match op {
                            Paint::Crater { .. } | Paint::Scorch { .. } if d < 0.7 => 0,
                            _ => data[o + 3],
                        };
                        data[o..o + 4].copy_from_slice(&to_srgb8(n, code));
                    }
                }
            }
        }
    }
}
