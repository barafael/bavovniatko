//! The ground: chunked terrain meshes carrying a painted ground texture (one texel per metre). The texture is
//! the battle's memory. Roads, furrows under thin snow and belts are painted at start; every crater, scorch
//! and track mark is stamped in afterwards and never cleaned up.
//!
//! The ground is 252 chunks of 256×256 texels, painted straight into their 8-bit textures with no larger buffer
//! in between, at start and during the day alike. `VUHLEDAR_GROUND=<dir>` saves them as one PNG each and reads
//! them back on the next run: the shape a persisted day's ground takes (design/next.md).

use crate::dev::dev;
use crate::diorama::{Diorama, diorama};
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
use std::path::{Path, PathBuf};
use std::time::Instant;

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

/// The chunk textures, in chunk order (x fastest). Painting changes them in place; Bevy uploads a changed one.
#[derive(Resource)]
pub struct Ground {
    images: Vec<Handle<Image>>,
}

impl Ground {
    /// Each chunk's texture as `<dir>/ground-<cx>-<cz>.png`.
    pub fn save(&self, images: &Assets<Image>, dir: &Path) -> Result<(), String> {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        for (i, handle) in self.images.iter().enumerate() {
            let image = images.get(handle).ok_or("a ground chunk is not loaded")?.clone();
            let image = image.try_into_dynamic().map_err(|e| e.to_string())?;
            image.save(chunk_path(dir, i)).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

fn chunk_path(dir: &Path, i: usize) -> PathBuf {
    dir.join(format!("ground-{}-{}.png", i % CHUNKS_X, i / CHUNKS_X))
}

/// The chunks `Ground::save` wrote, if all of them are there.
fn load(dir: &Path) -> Option<Chunks> {
    (0..CHUNKS_X * CHUNKS_Z)
        .map(|i| {
            let image = image::open(chunk_path(dir, i)).ok()?.into_rgba8();
            (image.width() as usize == TEX_PER_CHUNK && image.height() as usize == TEX_PER_CHUNK).then(|| image.into_raw())
        })
        .collect()
}

#[derive(Clone, Copy)]
pub enum Paint {
    Crater { at: Vec2, radius: f32, seed: u32 },
    Scorch { at: Vec2, radius: f32 },
    Track { at: Vec2, dir: Vec2, gauge: f32 },
    Footprints { at: Vec2 },
    /// A mine plow's strip of turned earth, with spoil thrown to both sides.
    Plow { at: Vec2, dir: Vec2 },
}

#[derive(Resource, Default)]
pub struct PaintQueue {
    pub ops: Vec<Paint>,
    timer: f32,
    /// Every crater deep enough to slow a track, kept for the drivers.
    pub craters: Craters,
}

impl PaintQueue {
    pub fn push(&mut self, p: Paint) {
        if let Paint::Crater { at, radius, .. } = p
            && radius >= 1.5
        {
            self.craters.add(at);
        }
        self.ops.push(p);
    }
}

/// Where shells have cratered the ground (centres, on a coarse grid), so a vehicle driving into one slows down.
#[derive(Default)]
pub struct Craters {
    grid: std::collections::HashMap<(i32, i32), Vec<Vec2>>,
}

impl Craters {
    const CELL: f32 = 16.0;

    fn add(&mut self, at: Vec2) {
        let c = ((at.x / Self::CELL).floor() as i32, (at.y / Self::CELL).floor() as i32);
        self.grid.entry(c).or_default().push(at);
    }

    /// Is a crater centre within `r` of `p`? (`r` at most a cell.)
    pub fn near(&self, p: Vec2, r: f32) -> bool {
        self.nearest(p, r).is_some()
    }

    /// The nearest crater centre within `r` of `p`, if any (`r` at most a cell).
    pub fn nearest(&self, p: Vec2, r: f32) -> Option<Vec2> {
        let (cx, cz) = ((p.x / Self::CELL).floor() as i32, (p.y / Self::CELL).floor() as i32);
        let mut best: Option<(f32, Vec2)> = None;
        for dz in -1..=1 {
            for dx in -1..=1 {
                for c in self.grid.get(&(cx + dx, cz + dz)).into_iter().flatten() {
                    let d = c.distance(p);
                    if d < r && best.is_none_or(|b| d < b.0) {
                        best = Some((d, *c));
                    }
                }
            }
        }
        best.map(|b| b.1)
    }
}

const W: usize = CHUNKS_X * TEX_PER_CHUNK;
const H: usize = CHUNKS_Z * TEX_PER_CHUNK;
const CHUNK_BYTES: usize = TEX_PER_CHUNK * TEX_PER_CHUNK * 4;

/// A texel's colour as the painting works with it; the texture holds it as 8-bit sRGB plus the furrow code.
type Px = [f32; 3];

/// The chunk textures while they are painted at start, before they become images.
type Chunks = Vec<Vec<u8>>;

const SNOW: Px = [0.80, 0.82, 0.84];
const EARTH: Px = [0.36, 0.31, 0.26];
const DARK_EARTH: Px = [0.20, 0.17, 0.15];

/// The colours the battle paints with, by season: earth thrown on snow in February, on grass in June.
struct Pal {
    earth: Px,
    dark_earth: Px,
    core: Px,
    clod: Px,
    throw: Px,
    track: Px,
    plow: Px,
}

const WINTER: Pal = Pal { earth: EARTH, dark_earth: DARK_EARTH, core: [0.12, 0.10, 0.09], clod: [0.16, 0.13, 0.11], throw: [0.48, 0.43, 0.38], track: [0.55, 0.52, 0.48], plow: [0.30, 0.26, 0.22] };
const SUMMER: Pal = Pal { earth: [0.50, 0.43, 0.33], dark_earth: [0.30, 0.25, 0.19], core: [0.20, 0.16, 0.12], clod: [0.31, 0.26, 0.20], throw: [0.58, 0.52, 0.42], track: [0.46, 0.44, 0.32], plow: [0.44, 0.36, 0.26] };

fn pal() -> &'static Pal {
    if diorama().summer() { &SUMMER } else { &WINTER }
}

fn mix(a: Px, b: Px, t: f32) -> Px {
    let t = t.clamp(0.0, 1.0);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

fn tex_of(p: Vec2) -> (f32, f32) {
    (p.x + HALF_X, p.y + HALF_Z)
}

fn to_srgb8(c: Px, code: u8) -> [u8; 4] {
    [(c[0].clamp(0.0, 1.0) * 255.0) as u8, (c[1].clamp(0.0, 1.0) * 255.0) as u8, (c[2].clamp(0.0, 1.0) * 255.0) as u8, code]
}

fn from_srgb8(b: &[u8]) -> Px {
    [b[0] as f32 / 255.0, b[1] as f32 / 255.0, b[2] as f32 / 255.0]
}

/// The texels of one chunk inside a disc of radius `r` around `at` (or a rectangle: `r` infinite).
struct Texels {
    chunk: usize,
    /// Global texel coordinates, inclusive.
    x0: usize,
    x1: usize,
    z0: usize,
    z1: usize,
    at: Vec2,
    r: f32,
}

impl Texels {
    /// Each texel's bytes, its world position (the texel's centre) and its distance from `at` as a fraction of `r`.
    fn each(&self, data: &mut [u8], mut f: impl FnMut(&mut [u8], Vec2, f32)) {
        for zz in self.z0..=self.z1 {
            for xx in self.x0..=self.x1 {
                let w = Vec2::new(xx as f32 + 0.5 - HALF_X, zz as f32 + 0.5 - HALF_Z);
                let d = w.distance(self.at) / self.r;
                if d > 1.0 {
                    continue;
                }
                let o = ((zz % TEX_PER_CHUNK) * TEX_PER_CHUNK + xx % TEX_PER_CHUNK) * 4;
                f(&mut data[o..o + 4], w, d);
            }
        }
    }
}

/// The chunks a texel rectangle overlaps, each with its part of it: the one loop every stamp goes through.
fn cover(x0: i64, z0: i64, x1: i64, z1: i64, at: Vec2, r: f32) -> Vec<Texels> {
    let (x0, z0) = (x0.max(0), z0.max(0));
    let (x1, z1) = (x1.min(W as i64 - 1), z1.min(H as i64 - 1));
    if x0 > x1 || z0 > z1 {
        return vec![];
    }
    let (x0, x1, z0, z1) = (x0 as usize, x1 as usize, z0 as usize, z1 as usize);
    let mut out = vec![];
    for chz in z0 / TEX_PER_CHUNK..=z1 / TEX_PER_CHUNK {
        for chx in x0 / TEX_PER_CHUNK..=x1 / TEX_PER_CHUNK {
            out.push(Texels {
                chunk: chz * CHUNKS_X + chx,
                x0: x0.max(chx * TEX_PER_CHUNK),
                x1: x1.min((chx + 1) * TEX_PER_CHUNK - 1),
                z0: z0.max(chz * TEX_PER_CHUNK),
                z1: z1.min((chz + 1) * TEX_PER_CHUNK - 1),
                at,
                r,
            });
        }
    }
    out
}

/// The texels within `r` metres of `at`, by chunk.
fn disc(at: Vec2, r: f32) -> Vec<Texels> {
    let (cx, cz) = tex_of(at);
    cover((cx - r).floor() as i64, (cz - r).floor() as i64, (cx + r).ceil() as i64, (cz + r).ceil() as i64, at, r)
}

/// The texels of a world rectangle, by chunk.
fn rect(min: Vec2, max: Vec2) -> Vec<Texels> {
    let (a, b) = (tex_of(min), tex_of(max));
    cover(a.0 as i64, a.1 as i64, b.0 as i64 - 1, b.1 as i64 - 1, Vec2::ZERO, f32::INFINITY)
}

/// Furrow direction per field block, as the alpha code the ground shader reads (1..=255; 0 = no furrows).
fn furrow_code(x: f32, z: f32) -> u8 {
    if diorama().summer() {
        // Only the sown fields have rows: last year's unharvested sunflowers, and this year's few crops.
        let (kind, t) = summer_field(x, z);
        return if matches!(kind, 3..=5) { 1 + (t * 254.0) as u8 } else { 0 };
    }
    let bx = ((x + HALF_X) / 1200.0).floor();
    let bz = ((z + 760.0) / 550.0).floor();
    let t = geo::hash01((bx as i32 * 7 + bz as i32 * 13 + 100) as u32, 5);
    1 + (t * 254.0) as u8
}

/// The field block a point is in (between the summer tree lines), as a kind of cover and a row direction.
/// 0 lush weeds, 1 dry fallow grass, 2 weeds in flower, 3 last year's sunflowers never harvested, 4 young
/// sunflowers on bare earth, 5 green wheat. Near the front almost nothing has been sown since 2022.
fn summer_field(x: f32, z: f32) -> (u32, f32) {
    let bx = ((x + 1650.0) / 1150.0).floor();
    let bz = ((z + 1450.0) / 550.0).floor();
    let id = (bx as i32 * 7 + bz as i32 * 13 + 300) as u32;
    let h = geo::hash01(id, 7);
    let rear = z < -1100.0;
    let kind = if rear && h < 0.5 { 5 } else if rear && h < 0.75 { 4 } else { (h * 4.0) as u32 % 4 };
    (kind, geo::hash01(id, 8))
}

fn summer_colour(x: f32, z: f32) -> Px {
    let (kind, _) = summer_field(x, z);
    let broad = fbm(x / 60.0, z / 60.0, 3, 23) * 0.5 + 0.5;
    let fine = value_noise(x / 3.0, z / 3.0, 24) * 0.5 + 0.5;
    let lush: Px = [0.42, 0.50, 0.25];
    let dry: Px = [0.60, 0.58, 0.40];
    let mut c = match kind {
        0 => mix(lush, dry, broad * 0.5),
        1 => mix(dry, lush, broad * 0.45),
        2 => {
            let bloom = ((fbm(x / 6.0, z / 6.0, 3, 25) - 0.15) * 3.0).clamp(0.0, 1.0);
            mix(mix(lush, dry, broad * 0.3), [0.68, 0.64, 0.33], bloom * 0.45)
        }
        3 => mix([0.42, 0.37, 0.29], lush, broad * 0.55),
        4 => mix([0.53, 0.45, 0.34], [0.46, 0.51, 0.28], broad * 0.3),
        _ => mix([0.49, 0.56, 0.28], [0.58, 0.59, 0.34], broad * 0.4),
    };
    // A year of shelling between the lines: the grass grows back thin and patchy over churned earth.
    let churn = (1.0 - ((z - 150.0).abs() / 650.0)).clamp(0.0, 1.0);
    let bare = ((fbm(x / 9.0, z / 9.0, 3, 26) + 0.25 - churn * 0.55) * 4.0).clamp(0.0, 1.0);
    c = mix([0.49, 0.42, 0.32], c, 0.35 + 0.65 * bare);
    let n = (fine - 0.5) * 0.06;
    [c[0] + n, c[1] + n, c[2] + n * 0.6]
}

/// Base ground colour at a world point: snow over ploughed fields, thinner in patches where the wind scoured it.
fn base_colour(x: f32, z: f32) -> Px {
    if diorama().summer() {
        return summer_colour(x, z);
    }
    // Thin snow, patchy at every scale: the wind strips it off the crests, weeks of shelling churned the rest.
    // Earth shows through in dark patches (compare the February drone footage).
    let broad = fbm(x / 45.0, z / 45.0, 3, 3);
    let fine = fbm(x / 5.0, z / 5.0, 3, 4);
    // More exposed earth in the fought-over ground south of the first tree line.
    let churn = 0.18 * (1.0 - ((z + 700.0) / 250.0).clamp(0.0, 1.0)).max(0.0) - 0.12 * ((z + 650.0) / 900.0).clamp(0.0, 1.0);
    // Mostly white; small dark patches where the noise dips. The texel holds how much snow lies here as a
    // gradient a few metres wide; the ground shader cuts the patch's ragged edge below the texel from that.
    let v = 0.62 + broad * 0.45 + fine * 0.5 - churn * 1.2;
    let cover = ((v - 0.12) / 0.30).clamp(0.0, 1.0);
    let earth = mix([0.27, 0.25, 0.22], [0.44, 0.42, 0.39], value_noise(x / 4.0, z / 4.0, 12) * 0.5 + 0.5);
    let mut col = mix(earth, SNOW, cover);
    let n = value_noise(x / 3.0, z / 3.0, 9) * 0.02;
    col = [col[0] + n, col[1] + n, col[2] + n * 1.2];
    col
}

/// `f(i)` for every `i` below `n`, on all cores, the results in order.
fn in_parallel<T: Send>(n: usize, f: impl Fn(usize) -> T + Sync) -> Vec<T> {
    let per = n.div_ceil(std::thread::available_parallelism().map_or(4, |t| t.get()));
    let mut out = Vec::with_capacity(n);
    std::thread::scope(|scope| {
        let f = &f;
        let workers: Vec<_> = (0..n).step_by(per).map(|start| scope.spawn(move || (start..(start + per).min(n)).map(f).collect::<Vec<T>>())).collect();
        for worker in workers {
            out.extend(worker.join().expect("a terrain worker panicked"));
        }
    });
    out
}

/// One chunk's base ground, with the furrow code of each texel.
fn base_chunk(chunk: usize) -> Vec<u8> {
    let (cx, cz) = (chunk % CHUNKS_X, chunk / CHUNKS_X);
    let mut data = Vec::with_capacity(CHUNK_BYTES);
    for row in 0..TEX_PER_CHUNK {
        for col in 0..TEX_PER_CHUNK {
            let (x, z) = ((cx * TEX_PER_CHUNK + col) as f32 + 0.5 - HALF_X, (cz * TEX_PER_CHUNK + row) as f32 + 0.5 - HALF_Z);
            data.extend_from_slice(&to_srgb8(base_colour(x, z), furrow_code(x, z)));
        }
    }
    data
}

/// Every chunk's texture, painted from scratch: the base ground on all cores, then the river, the belts, the
/// roads, the villages, the old craters and, at Robotyne, the trenches, stamped in that order. A stamp wipes
/// the furrows under it; a crater's rim gives them back.
fn paint() -> Chunks {
    let mut chunks = in_parallel(CHUNKS_X * CHUNKS_Z, base_chunk);
    let lay = geo::layout();
    let stamp = |chunks: &mut Chunks, p: Vec2, r: f32, f: &dyn Fn(f32, Px, Vec2) -> Px| {
        for t in disc(p, r) {
            t.each(&mut chunks[t.chunk], |px, w, d| px.copy_from_slice(&to_srgb8(f(d, from_srgb8(px), w), 0)));
        }
    };

    // The river: ice and reeds.
    let no_river = geo::Polyline::new(vec![Vec2::ZERO, Vec2::ZERO]);
    let river = lay.river.as_ref().unwrap_or(&no_river);
    let mut s = 0.0;
    while s < river.length() {
        let p = river.point(s);
        stamp(&mut chunks, p, 26.0, &|d, c, w| {
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
            let summer = diorama().summer();
            stamp(&mut chunks, p, r, &|d, c, w| {
                let n = value_noise(w.x / 3.0, w.y / 3.0, 21) * 0.5 + 0.5;
                // Leaf litter and trampled snow in winter; deep shade and nettles in June.
                let under = if summer { mix([0.25, 0.29, 0.15], [0.18, 0.21, 0.11], n) } else { mix([0.42, 0.39, 0.35], [0.30, 0.27, 0.24], n) };
                mix(c, under, (1.0 - d * d) * 0.85)
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
            let summer = diorama().summer();
            stamp(&mut chunks, p, r, &|d, c, w| {
                let lat = (w - p).dot(nrm).abs();
                if summer {
                    // June: grey asphalt with dry verges; dirt tracks of pale dust with two darker ruts.
                    return match kind {
                        RoadKind::Asphalt => if d < 0.8 { mix([0.34, 0.34, 0.33], [0.42, 0.41, 0.39], value_noise(w.x / 5.0, w.y / 5.0, 5) * 0.5 + 0.5) } else { mix(c, [0.58, 0.55, 0.42], 0.5) },
                        RoadKind::Dirt | RoadKind::Track => {
                            let rut = (1.0 - ((lat - 1.4).abs() / 0.6).min(1.0)).powf(0.7);
                            let dust = mix([0.63, 0.58, 0.47], [0.57, 0.52, 0.42], value_noise(w.x / 2.0, w.y / 2.0, 6) * 0.5 + 0.5);
                            let base = mix(c, dust, (1.0 - d.powi(3)) * if kind == RoadKind::Dirt { 0.85 } else { 0.55 });
                            mix(base, [0.47, 0.41, 0.32], rut * 0.6)
                        }
                    };
                }
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
    for (area, dirt) in [(lay.town, 0.45), (lay.dachas, 0.35), (lay.pavlivka, 0.3), (lay.mykilske, 0.3)] {
        for t in rect(area.min, area.max) {
            t.each(&mut chunks[t.chunk], |px, w, _| {
                let n = fbm(w.x / 25.0, w.y / 25.0, 3, 31) * 0.5 + 0.5;
                px.copy_from_slice(&to_srgb8(mix(from_srgb8(px), [0.50, 0.48, 0.45], dirt * n), 0));
            });
        }
    }

    // Weeks of shelling before this morning: old craters, densest along the belts and roads.
    let mut k = 0u32;
    for (wi, (p, _)) in lay.old_wrecks.iter().enumerate() {
        for j in 0..10 {
            let a = geo::hash01(wi as u32 * 100 + j, 41) * std::f32::consts::TAU;
            let d = geo::hash01(wi as u32 * 100 + j, 42) * 70.0;
            crater(&mut chunks, *p + Vec2::from_angle(a) * d, 3.0 + geo::hash01(j, 43) * 3.0, k);
            k += 1;
        }
    }
    for belt in &lay.belts {
        let n = (belt.line.length() / 35.0) as u32;
        for _ in 0..n {
            let s = geo::hash01(k, 51) * belt.line.length();
            let off = (geo::hash01(k, 52) - 0.5) * 70.0;
            let p = belt.line.point(s) + belt.line.normal(s) * off;
            crater(&mut chunks, p, 2.0 + geo::hash01(k, 53) * 3.5, k);
            k += 1;
        }
    }
    // Scattered harassing fire in clusters (a battery's sheaf lands as a group).
    for _ in 0..90 {
        let c = Vec2::new((geo::hash01(k, 61) * 2.0 - 1.0) * HALF_X, (geo::hash01(k, 62) * 1.6 - 1.0) * HALF_Z);
        k += 1;
        for _ in 0..(2 + (geo::hash01(k, 64) * 5.0) as u32) {
            let p = c + Vec2::new(geo::hash01(k, 65) - 0.5, geo::hash01(k, 66) - 0.5) * 90.0;
            crater(&mut chunks, p, 1.0 + geo::hash01(k, 63).powi(2) * 3.0, k);
            k += 1;
        }
    }

    if diorama() == Diorama::Robotyne {
        // A year of artillery between the lines: no-man's-land is pocked from the forward minefield to the crest.
        for _ in 0..240 {
            let c = Vec2::new((geo::hash01(k, 67) * 2.0 - 1.0) * HALF_X, -480.0 + geo::hash01(k, 68) * 1250.0);
            k += 1;
            for _ in 0..(1 + (geo::hash01(k, 69) * 4.0) as u32) {
                let p = c + Vec2::new(geo::hash01(k, 70) - 0.5, geo::hash01(k, 71) - 0.5) * 70.0;
                crater(&mut chunks, p, 1.0 + geo::hash01(k, 72).powi(2) * 3.2, k);
                k += 1;
            }
        }
        // The Russian first line: a zigzag trench along the crest tree line, spoil thrown forward, communication
        // trenches back into the trees, dugouts at each platoon position. A short Ukrainian trench in the forward
        // tree line opposite.
        let trench = |chunks: &mut Chunks, a: Vec2, b: Vec2, spoil_side: f32| {
            let n = (a.distance(b) / 0.8) as usize;
            let nrm = (b - a).normalize_or_zero().perp();
            for i in 0..=n {
                let p = a.lerp(b, i as f32 / n.max(1) as f32);
                stamp(chunks, p + nrm * spoil_side * 1.8, 1.6, &|d, c, _| mix(c, [0.55, 0.48, 0.37], (1.0 - d) * 0.8));
                stamp(chunks, p, 0.8, &|_, _, w| mix([0.15, 0.12, 0.09], [0.22, 0.18, 0.13], value_noise(w.x, w.y, 8) * 0.5 + 0.5));
            }
        };
        let tz = crate::robotyne::TRENCH_Z;
        let mut x = -HALF_X + 30.0;
        let mut side = 1.0;
        while x < HALF_X - 30.0 {
            let a = Vec2::new(x, tz + side * 6.0);
            let b = Vec2::new(x + 22.0, tz - side * 6.0);
            trench(&mut chunks, a, b, -1.0);
            x += 22.0;
            side = -side;
        }
        for (i, p) in lay.ru_positions.iter().enumerate() {
            trench(&mut chunks, *p, *p + Vec2::new(8.0 * (i as f32 - 3.0).signum(), 30.0), 1.0);
            stamp(&mut chunks, *p + Vec2::new(5.0, 14.0), 3.0, &|_, _, _| [0.17, 0.14, 0.11]);
        }
        for p in &lay.ua_positions {
            trench(&mut chunks, *p + Vec2::new(-25.0, 7.0), *p + Vec2::new(25.0, 9.0), -1.0);
        }
    }
    chunks
}

/// Crater shape: a dark wet core, a raised rim of earth, and thrown dirt splashed over the snow in rays.
fn crater_colour(d: f32, c: Px, w: Vec2, centre: Vec2, seed: u32) -> Px {
    let ang = (w - centre).to_angle();
    let rays = (value_noise(ang * 3.0, (seed % 997) as f32, 71) * 0.5 + 0.5).powf(1.5);
    // Wobble the radius so no crater is a perfect disc.
    let d = d * (1.0 + 0.25 * value_noise(ang * 2.0 + (seed % 997) as f32, 0.5, 72));
    let pal = pal();
    if d < 0.45 {
        mix(pal.dark_earth, pal.core, value_noise(w.x, w.y, seed) * 0.5 + 0.5)
    } else if d < 0.6 {
        mix(c, pal.earth, 0.9)
    } else {
        // Thrown clods: a speckle of dark dots over the snow, densest near the rim and along the rays.
        let throw = (1.0 - (d - 0.6) / 0.4).powf(1.2) * (0.35 + 0.65 * rays);
        let clod = geo::hash01((w.x.floor() as i32 * 7919 + w.y.floor() as i32 * 104_729) as u32, seed % 991);
        if clod < throw * 0.75 { mix(c, pal.clod, 0.85) } else { mix(c, pal.throw, throw * 0.35) }
    }
}

/// An old crater: its core and rim wipe the furrows; under its thrown earth they stay.
fn crater(chunks: &mut Chunks, at: Vec2, radius: f32, seed: u32) {
    let summer = diorama().summer();
    for t in disc(at, radius * 3.2) {
        t.each(&mut chunks[t.chunk], |px, w, d| {
            let c = crater_colour(d, from_srgb8(px), w, at, seed);
            // In June last year's craters are half grown over.
            let c = if summer { mix(c, from_srgb8(px), 0.45) } else { c };
            px.copy_from_slice(&to_srgb8(c, if d < 0.75 { 0 } else { furrow_code(w.x, w.y) }));
        });
    }
}

/// Vertices per chunk side, and the grid they are read from: one more vertex all round for the normals.
const SIDE: usize = 65;
const GRID: usize = SIDE + 2;

/// One chunk's mesh. Its heights are sampled once on the grid, and the normals come from the grid's central
/// differences, so chunk edges match exactly.
fn chunk_mesh(chunk: usize, indices: &[u32]) -> Mesh {
    let (cx, cz) = (chunk % CHUNKS_X, chunk / CHUNKS_X);
    let step = CHUNK / (SIDE - 1) as f32;
    let (x0, z0) = (-HALF_X + cx as f32 * CHUNK - step, -HALF_Z + cz as f32 * CHUNK - step);
    let mut h = Vec::with_capacity(GRID * GRID);
    for j in 0..GRID {
        for i in 0..GRID {
            h.push(geo::height(x0 + i as f32 * step, z0 + j as f32 * step));
        }
    }
    let at = |i: usize, j: usize| h[j * GRID + i];
    let (mut pos, mut nor, mut uv) = (Vec::with_capacity(SIDE * SIDE), Vec::with_capacity(SIDE * SIDE), Vec::with_capacity(SIDE * SIDE));
    for j in 1..=SIDE {
        for i in 1..=SIDE {
            pos.push([x0 + i as f32 * step, at(i, j), z0 + j as f32 * step]);
            nor.push(Vec3::new(at(i - 1, j) - at(i + 1, j), 2.0 * step, at(i, j - 1) - at(i, j + 1)).normalize().to_array());
            uv.push([(i - 1) as f32 / (SIDE - 1) as f32, (j - 1) as f32 / (SIDE - 1) as f32]);
        }
    }
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, nor)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
        .with_inserted_indices(Indices::U32(indices.to_vec()))
}

/// The triangles of a chunk's vertex grid, the same for every chunk.
fn grid_indices() -> Vec<u32> {
    let mut idx = Vec::with_capacity((SIDE - 1) * (SIDE - 1) * 6);
    for j in 0..SIDE as u32 - 1 {
        for i in 0..SIDE as u32 - 1 {
            let a = j * SIDE as u32 + i;
            let (b, c) = (a + 1, a + SIDE as u32);
            idx.extend_from_slice(&[a, c, b, b, c, c + 1]);
        }
    }
    idx
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<GroundMaterial>>,
) {
    let t0 = Instant::now();
    let dir = dev().ground.as_deref().map(Path::new);
    let saved = dir.and_then(load);
    let from_file = saved.is_some();
    let chunks = saved.unwrap_or_else(paint);
    let t1 = Instant::now();
    let indices = grid_indices();
    let chunk_meshes = in_parallel(CHUNKS_X * CHUNKS_Z, |chunk| chunk_mesh(chunk, &indices));
    info!("ground: {} in {:.0} ms, meshes in {:.0} ms", if from_file { "loaded" } else { "painted" }, (t1 - t0).as_secs_f32() * 1000.0, t1.elapsed().as_secs_f32() * 1000.0);

    let mat_template = StandardMaterial { perceptual_roughness: 0.93, reflectance: 0.3, ..default() };
    let detail = GroundDetail { params: Vec4::new(1.0, 1.0, if diorama().summer() { 1.0 } else { 0.0 }, 0.0) };
    let mut handles = Vec::with_capacity(CHUNKS_X * CHUNKS_Z);
    for (chunk, (data, mesh)) in chunks.into_iter().zip(chunk_meshes).enumerate() {
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
        if crate::dev::off(crate::dev::Part::Terrain) {
            continue; // the ground still shapes everything; only its picture is left out
        }
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(ExtendedMaterial { base: StandardMaterial { base_color_texture: Some(handle), ..mat_template.clone() }, extension: detail.clone() })),
            Transform::IDENTITY,
            Name::new(format!("ground {},{}", chunk % CHUNKS_X, chunk / CHUNKS_X)),
            // The ground receives every shadow but casts none: the 252 chunks under the sun's two cascades were
            // half the frame (design/rewrites.md, step 5), and the shadow one hill throws on the next is not
            // visible from the drone or the OP.
            bevy::light::NotShadowCaster,
        ));
    }
    let ground = Ground { images: handles };
    if let (Some(dir), false) = (dir, from_file) {
        match ground.save(&images, dir) {
            Ok(()) => info!("ground: saved to {}", dir.display()),
            Err(e) => warn!("ground: not saved to {}: {e}", dir.display()),
        }
    }
    commands.insert_resource(ground);
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
    let pal = pal();
    let summer = diorama().summer();
    for op in ops {
        seed = seed.wrapping_add(7919);
        let (at, r) = match op {
            Paint::Crater { at, radius, .. } => (at, radius * 3.2),
            Paint::Scorch { at, radius } => (at, radius),
            Paint::Track { at, gauge, .. } => (at, gauge * 0.5 + 0.8),
            Paint::Footprints { at } => (at, 0.8),
            Paint::Plow { at, .. } => (at, 3.2),
        };
        // Each overlapped chunk is fetched (and so flagged for upload) once per op.
        for t in disc(at, r) {
            let Some(mut img) = images.get_mut(&ground.images[t.chunk]) else { continue };
            let Some(data) = img.data.as_mut() else { continue };
            t.each(data, |px, w, d| {
                let c = from_srgb8(px);
                let n = match op {
                    Paint::Crater { at, seed: s, .. } => crater_colour(d, c, w, at, s),
                    Paint::Scorch { .. } => mix(c, [0.10, 0.09, 0.08], (1.0 - d) * 0.8 * (0.6 + 0.4 * value_noise(w.x / 2.0, w.y / 2.0, seed))),
                    Paint::Track { dir, gauge, .. } => {
                        // Tracks compact the snow to a fixed tone; they never darken what is already darker.
                        // In June they crush the grass flat and grind the dust: a fixed tone either way.
                        let lat = (w - at).perp_dot(dir).abs();
                        let on_track = ((lat - gauge * 0.5).abs() < 0.5) as i32 as f32;
                        let track = pal.track;
                        let lum = |p: Px| p[0] * 0.3 + p[1] * 0.59 + p[2] * 0.11;
                        if summer || lum(c) > lum(track) { mix(c, track, on_track * 0.6) } else { c }
                    }
                    Paint::Footprints { .. } => {
                        if summer { mix(c, [0.52, 0.51, 0.38], 0.12 * (1.0 - d)) } else if c[0] > 0.62 { mix(c, [0.62, 0.60, 0.58], 0.3 * (1.0 - d)) } else { c }
                    }
                    Paint::Plow { dir, .. } => {
                        // Turned earth across the plow's width, a ridge of spoil at each edge.
                        let lat = (w - at).perp_dot(dir).abs();
                        if lat < 2.1 { mix(c, pal.plow, 0.85) } else { mix(c, pal.dark_earth, 0.7 * (1.0 - (lat - 2.1) / 1.1).clamp(0.0, 1.0)) }
                    }
                };
                // Craters and scorch wipe out the furrows; tracks and footprints don't.
                let code = match op {
                    Paint::Crater { .. } | Paint::Scorch { .. } if d < 0.7 => 0,
                    Paint::Plow { .. } => 0,
                    _ => px[3],
                };
                px.copy_from_slice(&to_srgb8(n, code));
            });
        }
    }
}
