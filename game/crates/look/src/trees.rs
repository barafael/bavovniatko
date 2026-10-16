//! The belts' trees as pictures: one atlas, drawn once at start and the same every run, for the kinds of tree the
//! shelterbelts of the south were planted with (black locust, elm, ash, poplar, walnut), a shell-broken one, and
//! two shrubs.
//!
//! In June a crown is a loose ball of leafy patches, each a card facing its own way: from any side and from above
//! it is a lumpy mass with gaps, never the flat star that crossed cards of a whole crown make from the drone's
//! height. A patch is drawn as clumps of foliage, each a ragged ball lit from the sky, the nearest clump at a
//! texel winning: lit tops, dark undersides, gaps, and no detail finer than a clump's leafy edge (the old
//! single-texel holes were the belts' speckle). In February a crown is bare limbs and twigs on crossed cards,
//! with one across the top.
//!
//! The mips keep each cell's coverage, so a crown far away stays as thick as it is near instead of thinning out.

use crate::geo::{hash01, value_noise};
use bevy::{image::ImageSamplerDescriptor, prelude::*};

/// The atlas's side in texels, and its cells per side.
const ATLAS: usize = 1024;
const CELLS: usize = 4;
const CELL: usize = ATLAS / CELLS;

/// The kinds of tree and shrub in the belts. In June each has two cells of leafy patches (its index, and eight
/// on); in February a cell of its bare crown seen from the side (its index), and one of four seen from above.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Black locust (Robinia), the commonest belt tree of the steppe: an open, irregular crown of separate clumps.
    Robinia,
    /// Elm: a broad crown, widest high up.
    Elm,
    /// Ash: an airy oval.
    Ash,
    /// Poplar: a tall narrow column, planted in rows along the roads and belt ends.
    Poplar,
    /// Walnut: a broad dense dome.
    Walnut,
    /// Shell-broken: a thin crown of what leaves are left; bare, a split stem and a few limbs.
    Broken,
    /// Hawthorn, sloe, elder: a low wide mound.
    Shrub,
    /// Locust suckers and wild rose along the margins: upright and uneven.
    Thicket,
}

impl Kind {
    fn side_cell(self) -> usize {
        self as usize
    }

    fn top_cell(self) -> usize {
        8 + match self {
            Kind::Robinia | Kind::Ash | Kind::Broken => 0,
            Kind::Elm | Kind::Walnut => 1,
            Kind::Poplar => 2,
            Kind::Shrub | Kind::Thicket => 3,
        }
    }

    /// The crown's width over its height.
    pub fn aspect(self) -> f32 {
        match self {
            Kind::Robinia => 0.85,
            Kind::Elm => 0.95,
            Kind::Ash => 0.75,
            Kind::Poplar => 0.32,
            Kind::Walnut => 1.15,
            Kind::Broken => 0.7,
            Kind::Shrub => 1.6,
            Kind::Thicket => 1.0,
        }
    }
}

/// Half a sine over 0..1: the outlines are built from it.
fn arch(y: f32) -> f32 {
    (std::f32::consts::PI * y.clamp(0.0, 1.0)).sin()
}

/// A kind's bare crown outline: its half-width at a height (0 at the crown's foot, 1 at its top), as a share of
/// the crown's half-width.
fn outline(kind: Kind) -> fn(f32) -> f32 {
    match kind {
        Kind::Robinia => |y| arch(y * 0.92 + 0.04).powf(0.6) * (0.75 + 0.25 * y),
        Kind::Elm => |y| arch(0.5 + (y - 0.62) * 1.25).powf(0.5) * (0.55 + 0.45 * y.min(0.75) / 0.75),
        Kind::Ash => |y| arch(y).powf(0.75),
        Kind::Poplar => |y| arch(y * 0.96 + 0.02).powf(0.4),
        Kind::Walnut => |y| arch(0.25 + y * 0.75).powf(0.5),
        Kind::Broken => |y| if y < 0.6 { arch(y / 0.6).powf(0.6) * 0.8 } else { 0.0 },
        Kind::Shrub => |y| arch(0.5 + y * 0.5).powf(0.5),
        Kind::Thicket => |y| arch(y * 0.85 + 0.15).powf(0.7),
    }
}

/// One cell being drawn: colour, alpha and the nearest surface's depth at each texel.
struct Canvas {
    rgb: Vec<[f32; 3]>,
    a: Vec<f32>,
    z: Vec<f32>,
}

impl Canvas {
    fn new() -> Self {
        Canvas { rgb: vec![[0.0; 3]; CELL * CELL], a: vec![0.0; CELL * CELL], z: vec![f32::MIN; CELL * CELL] }
    }

    /// A texel in the crown's own units: x across (0 at the centre, the crown `aspect` wide), y up (0 at the foot
    /// of the cell, 1 at its top). The cell keeps a margin so the mips do not bleed into the next.
    fn at(col: usize, row: usize, aspect: f32) -> Vec2 {
        let m = 6.0;
        let span = CELL as f32 - 2.0 * m;
        Vec2::new(((col as f32 + 0.5 - m) / span - 0.5) * aspect, 1.0 - (row as f32 + 0.5 - m) / span)
    }

    /// The texel range a disc of radius `r` around `p` covers (in crown units).
    fn span(p: Vec2, r: f32, aspect: f32) -> (usize, usize, usize, usize) {
        let m = 6.0;
        let span = CELL as f32 - 2.0 * m;
        let col = |x: f32| (x / aspect + 0.5) * span + m;
        let row = |y: f32| (1.0 - y) * span + m;
        let cl = |v: f32| v.clamp(0.0, CELL as f32 - 1.0) as usize;
        (cl(col(p.x - r) - 1.0), cl(col(p.x + r) + 1.0), cl(row(p.y + r) - 1.0), cl(row(p.y - r) + 1.0))
    }

    /// Put a texel if it is nearer than what is there.
    fn put(&mut self, i: usize, z: f32, c: [f32; 3], a: f32) {
        if z > self.z[i] {
            self.z[i] = z;
            self.rgb[i] = c;
            self.a[i] = self.a[i].max(a);
        } else {
            self.a[i] = self.a[i].max(a * 0.5);
        }
    }

    /// A limb from `p0` to `p1`, `w0` thick tapering to `w1`, at depth `z`, in colour `c`.
    fn limb(&mut self, p0: Vec2, p1: Vec2, w0: f32, w1: f32, z: f32, c: [f32; 3], aspect: f32) {
        let (c0, c1, r0, r1) = Canvas::span(p0.min(p1) + (p0.max(p1) - p0.min(p1)) * 0.5, (p1 - p0).length() * 0.5 + w0, aspect);
        let d = p1 - p0;
        let len2 = d.length_squared().max(1e-8);
        for row in r0..=r1 {
            for col in c0..=c1 {
                let q = Canvas::at(col, row, aspect);
                let t = ((q - p0).dot(d) / len2).clamp(0.0, 1.0);
                let w = w0 + (w1 - w0) * t;
                let dist = (q - (p0 + d * t)).length();
                // Coverage over about a texel's width, so a thin twig is a faint line rather than a broken one.
                let px = aspect.max(1.0) / CELL as f32;
                let a = ((w * 0.5 + px * 0.5 - dist) / px).clamp(0.0, 1.0);
                if a > 0.0 {
                    let lit = 0.8 + 0.2 * (q.x / aspect + 0.5);
                    self.put(row * CELL + col, z, [c[0] * lit, c[1] * lit, c[2] * lit], a);
                }
            }
        }
    }
}

/// A branching limb system: from `p` along `dir` for `len`, `w` thick, splitting `depth` more times. `spread` is
/// how far the children turn, `up` how much they bend back towards the sky.
#[allow(clippy::too_many_arguments)]
fn branch(cv: &mut Canvas, p: Vec2, dir: Vec2, len: f32, w: f32, depth: u32, seed: u32, spread: f32, up: f32, z: f32, c: [f32; 3], aspect: f32, keep: &dyn Fn(Vec2) -> bool) {
    // A limb is a few kinked segments, not a ruled line.
    let segs = 3;
    let mut q = p;
    let mut d = dir;
    for k in 0..segs {
        let turn = (hash01(seed * 13 + k, 21) - 0.5) * 0.5;
        d = (Vec2::from_angle(turn).rotate(d) + Vec2::Y * up * 0.3).normalize();
        let next = q + d * len / segs as f32;
        if !keep(next) {
            break;
        }
        let t0 = k as f32 / segs as f32;
        let t1 = (k + 1) as f32 / segs as f32;
        cv.limb(q, next, w * (1.0 - 0.45 * t0), w * (1.0 - 0.45 * t1), z, c, aspect);
        q = next;
    }
    if depth == 0 {
        return;
    }
    let kids = 2 + (hash01(seed, 3) * 2.0) as u32;
    for k in 0..kids {
        let t = 0.35 + 0.6 * hash01(seed * 7 + k, 4);
        let start = p + (q - p) * t;
        let side = if k % 2 == 0 { 1.0 } else { -1.0 };
        let ang = side * spread * (0.4 + 0.6 * hash01(seed * 11 + k, 5));
        let nd = (Vec2::from_angle(ang).rotate(d) + Vec2::Y * up).normalize();
        let zk = z + (hash01(seed * 17 + k, 9) - 0.5) * 0.2;
        let paler = c.map(|v| (v * 1.05).min(0.75));
        branch(cv, start, nd, len * (0.5 + 0.25 * hash01(seed + k, 6)), w * 0.62, depth - 1, seed * 5 + k + 1, spread, up, zk, paler, aspect, keep);
    }
}

/// A leafy patch (or a crown seen from above): clumps over a disc, lit from above the cell. Each kind's patches
/// have its own leaf: locust's open and ragged, walnut's big and dense, poplar's small and close.
fn leafy_patch(kind: Kind, seed: u32) -> Canvas {
    let mut cv = Canvas::new();
    let (clumps, radius, ragged): (u32, (f32, f32), f32) = match kind {
        Kind::Robinia => (45, (0.09, 0.15), 0.35),
        Kind::Elm => (70, (0.09, 0.15), 0.15),
        Kind::Ash => (55, (0.06, 0.1), 0.3),
        Kind::Poplar => (80, (0.06, 0.09), 0.1),
        Kind::Walnut => (60, (0.11, 0.18), 0.1),
        Kind::Broken => (25, (0.07, 0.12), 0.4),
        Kind::Shrub => (50, (0.1, 0.17), 0.2),
        Kind::Thicket => (40, (0.08, 0.14), 0.3),
    };
    for k in 0..clumps {
        let id = seed * 7919 + k;
        let ang = hash01(id, 1) * std::f32::consts::TAU;
        let r = radius.0 + (radius.1 - radius.0) * hash01(id, 3);
        let rad = hash01(id, 2).sqrt() * (0.5 - r - ragged * 0.3 * hash01(id, 7)).max(0.0);
        let c = Vec2::new(ang.cos() * rad, 0.5 + ang.sin() * rad);
        let zc = (1.0 - rad / 0.4) * 0.3 + hash01(id, 4) * 0.15;
        let tint = 0.92 + 0.16 * hash01(id, 6);
        let (c0, c1, r0, r1) = Canvas::span(c, r * 1.2, 1.0);
        for row in r0..=r1 {
            for col in c0..=c1 {
                let q = Canvas::at(col, row, 1.0);
                let d = q - c;
                let rough = value_noise(q.x * 38.0, q.y * 38.0, seed + 40) * 0.5 + value_noise(q.x * 90.0, q.y * 90.0, seed + 41) * 0.25;
                let rr = r * (0.88 + 0.22 * rough);
                let dd = d.length();
                // Sprays of leaves: lighter and darker at a few texels, and gaps through the clump's rim (never a
                // single texel, which is what sparkled).
                let leaf = value_noise(q.x * 24.0 + 3.1, q.y * 24.0, seed + 42) * 0.6 + value_noise(q.x * 55.0, q.y * 55.0 + 1.7, seed + 43) * 0.4;
                if dd > rr || (dd > rr * 0.55 && leaf < -0.28) {
                    continue;
                }
                let n2 = d / rr;
                let nz = (1.0 - n2.length_squared()).max(0.0).sqrt();
                let z = zc + nz * r;
                let lam = (0.35 * n2.y + 0.85 * nz).max(0.0);
                let b = (0.5 + 0.5 * lam) * (0.68 + 0.32 * (z / 0.5).clamp(0.0, 1.0)) * tint * (0.86 + 0.28 * leaf);
                let k = lam * (z / 0.45).clamp(0.0, 1.0);
                let col_ = [b * (0.8 + 0.25 * k), b * (0.93 + 0.1 * k), b * (1.05 - 0.23 * k)];
                let a = ((rr - dd) / (1.5 / CELL as f32)).clamp(0.0, 1.0);
                cv.put(row * CELL + col, z, col_, a);
            }
        }
    }
    cv
}

/// A bare crown seen from the side: the trunk's top, limbs, and the twigs as a fine fringe at their ends.
fn bare_side(kind: Kind, seed: u32) -> Canvas {
    let aspect = kind.aspect();
    let mut cv = Canvas::new();
    let half = aspect * 0.5;
    // Inside the kind's outline (a little beyond it), so each kind keeps its shape bare.
    let outline = outline(kind);
    let keep = move |p: Vec2| p.y < 1.02 && p.y > -0.1 && p.x.abs() < (outline)(p.y.clamp(0.0, 1.0)).max(0.12) * half * 1.15;
    // Limbs, then their branches and twigs down to a fraction of a texel: drawn as coverage, the twigs come out
    // as the fine grey haze a bare crown is from far off, not as a skeleton.
    let (limbs, spread, up, w, len) = match kind {
        Kind::Poplar => (6, 0.25, 0.9, 0.022, 0.62),
        Kind::Elm => (5, 0.7, 0.5, 0.03, 0.58),
        Kind::Walnut => (5, 0.9, 0.25, 0.035, 0.52),
        Kind::Robinia => (5, 0.8, 0.25, 0.028, 0.55),
        Kind::Ash => (4, 0.55, 0.45, 0.028, 0.58),
        Kind::Broken => (2, 0.7, 0.2, 0.03, 0.35),
        Kind::Shrub => (9, 0.8, 0.35, 0.02, 0.45),
        Kind::Thicket => (7, 0.5, 0.55, 0.02, 0.55),
    };
    let bark = [0.5, 0.46, 0.43];
    for k in 0..limbs {
        let x0 = (hash01(seed + k, 30) - 0.5) * half * 0.25;
        let lean = if kind == Kind::Poplar { 0.15 } else { 1.2 };
        let dir = Vec2::new((hash01(seed + k, 31) - 0.5) * lean * aspect, 1.0).normalize();
        branch(&mut cv, Vec2::new(x0, -0.05), dir, len, w, 5, seed * 3 + k, spread, up, (hash01(seed + k, 34) - 0.5) * 0.4, bark, aspect, &keep);
    }
    if kind == Kind::Broken {
        // The split stem: pale wood where the top was torn off.
        cv.limb(Vec2::new(0.0, -0.05), Vec2::new(0.02, 0.5), 0.05, 0.035, 1.0, [0.78, 0.72, 0.62], aspect);
    }
    cv
}

/// A bare crown seen from above: limbs radiating from the stem.
fn bare_top(cell: usize, seed: u32) -> Canvas {
    let mut cv = Canvas::new();
    let n = match cell {
        2 => 6,
        3 => 10,
        _ => 7,
    };
    let keep = |p: Vec2| (p - Vec2::new(0.0, 0.5)).length() < 0.45;
    for k in 0..n {
        let ang = (k as f32 + hash01(seed + k, 1) * 0.6) / n as f32 * std::f32::consts::TAU;
        let dir = Vec2::from_angle(ang);
        branch(&mut cv, Vec2::new(0.0, 0.5), dir, 0.2, 0.025, 5, seed * 3 + k, 0.7, 0.0, hash01(seed + k, 2), [0.5, 0.47, 0.44], 1.0, &keep);
    }
    cv
}

/// The atlas: each cell drawn, then its mips with each cell's coverage kept.
pub fn atlas(summer: bool) -> Image {
    let mut level0 = vec![[0.0f32; 4]; ATLAS * ATLAS];
    for cell in 0..CELLS * CELLS {
        let seed = 100 + cell as u32 * 37 + if summer { 0 } else { 1000 };
        let kinds = [Kind::Robinia, Kind::Elm, Kind::Ash, Kind::Poplar, Kind::Walnut, Kind::Broken, Kind::Shrub, Kind::Thicket];
        let cv = match (cell, summer) {
            // June: two patches of each kind's leaf.
            (_, true) => leafy_patch(kinds[cell % 8], seed),
            (0..8, false) => bare_side(kinds[cell], seed),
            (8..12, false) => bare_top(cell - 8, seed),
            _ => continue,
        };
        // Under the transparent texels, the crown's own darker tone: the mips average towards it, not to black.
        let (mut sum, mut wsum) = ([0.0f32; 3], 0.0f32);
        for i in 0..CELL * CELL {
            for (s, v) in sum.iter_mut().zip(cv.rgb[i]) {
                *s += v * cv.a[i];
            }
            wsum += cv.a[i];
        }
        let mean = sum.map(|s| s / wsum.max(1.0) * 0.8);
        let (ox, oy) = ((cell % CELLS) * CELL, (cell / CELLS) * CELL);
        for row in 0..CELL {
            for col in 0..CELL {
                let i = row * CELL + col;
                let c = if cv.a[i] > 0.0 { cv.rgb[i] } else { mean };
                level0[(oy + row) * ATLAS + ox + col] = [c[0], c[1], c[2], cv.a[i]];
            }
        }
    }
    // The mip chain: colour averaged by alpha, and alpha averaged, then distributed: each level's alpha is
    // dithered to 0 or 1 within its cell (error diffusion), so that as many of its texels are over Bevy's cut (it
    // sharpens alpha-to-coverage around 0.5) as the average says. Averaged alone, a crown would thin out into
    // nothing far away; scaled up instead, a bare one would go solid. (Yuksel, "Alpha Distribution for Alpha
    // Testing", 2018.)
    let mut data: Vec<u8> = Vec::new();
    let push = |data: &mut Vec<u8>, lv: &[[f32; 4]]| {
        for p in lv {
            data.extend_from_slice(&[(p[0].clamp(0.0, 1.0) * 255.0).round() as u8, (p[1].clamp(0.0, 1.0) * 255.0).round() as u8, (p[2].clamp(0.0, 1.0) * 255.0).round() as u8, (p[3].clamp(0.0, 1.0) * 255.0).round() as u8]);
        }
    };
    push(&mut data, &level0);
    let mut levels = 1u32;
    let (mut n, mut prev) = (ATLAS, level0);
    while n > 1 {
        let h = n / 2;
        let mut next = vec![[0.0f32; 4]; h * h];
        for y in 0..h {
            for x in 0..h {
                let ps = [prev[2 * y * n + 2 * x], prev[2 * y * n + 2 * x + 1], prev[(2 * y + 1) * n + 2 * x], prev[(2 * y + 1) * n + 2 * x + 1]];
                let wa: f32 = ps.iter().map(|p| p[3]).sum();
                let mut o = [0.0f32; 4];
                for c in 0..3 {
                    o[c] = if wa > 0.0 { ps.iter().map(|p| p[c] * p[3]).sum::<f32>() / wa } else { ps.iter().map(|p| p[c]).sum::<f32>() * 0.25 };
                }
                o[3] = wa * 0.25;
                next[y * h + x] = o;
            }
        }
        let mut shown = next.clone();
        let cs = h / CELLS;
        for cell in 0..CELLS * CELLS {
            let (ox, oy) = ((cell % CELLS) * cs, (cell / CELLS) * cs);
            // The error still to place, with a column of margin either side and a row below.
            let w = cs + 2;
            let mut err = vec![0.0f32; w * (cs + 1)];
            for row in 0..cs {
                // Serpentine, so the error does not drift into diagonal streaks.
                let dir: isize = if row % 2 == 1 { -1 } else { 1 };
                for k in 0..cs {
                    let col = if dir < 0 { cs - 1 - k } else { k } as isize;
                    let at = |r: usize, c: isize| r * w + (c + 1) as usize;
                    let i = (oy + row) * h + ox + col as usize;
                    let v = next[i][3] + err[at(row, col)];
                    let o = if v >= 0.5 { 1.0 } else { 0.0 };
                    shown[i][3] = o;
                    let q = v - o;
                    err[at(row, col + dir)] += q * 7.0 / 16.0;
                    err[at(row + 1, col - dir)] += q * 3.0 / 16.0;
                    err[at(row + 1, col)] += q * 5.0 / 16.0;
                    err[at(row + 1, col + dir)] += q / 16.0;
                }
            }
        }
        push(&mut data, &shown);
        levels += 1;
        n = h;
        prev = next;
    }
    let mut img = crate::meshes::rgba8_image(ATLAS, ATLAS, data[..ATLAS * ATLAS * 4].to_vec(), ImageSamplerDescriptor::linear());
    img.data = Some(data);
    img.texture_descriptor.mip_level_count = levels;
    img.sampler = bevy::image::ImageSampler::Descriptor(ImageSamplerDescriptor { mipmap_filter: bevy::image::ImageFilterMode::Linear, ..ImageSamplerDescriptor::linear() });
    img
}

/// A leafy crown as patches: cards placed through the ellipsoid of the crown, each about three quarters of the
/// crown across and turned its own way, its picture's top kept towards the sky. The seed is the crown's turn, so
/// every crown is its own arrangement.
#[allow(clippy::too_many_arguments)]
fn patches(m: &mut crate::meshes::MeshBuilder, kind: Kind, mid: Vec3, w: f32, h: f32, yaw: f32, c: crate::meshes::Rgb, normal: &dyn Fn(Vec3) -> Vec3) {
    let n = if matches!(kind, Kind::Shrub | Kind::Thicket | Kind::Broken) { 3 } else { 5 };
    let seed = (yaw * 10_000.0) as u32;
    let half = Vec3::new(w * 0.5, h * 0.5, w * 0.5);
    for k in 0..n {
        let id = seed.wrapping_mul(31) + k;
        // Where in the crown: spread through it, a little more of them high and outside where the light is.
        let a = hash01(id, 1) * std::f32::consts::TAU;
        let y = if kind == Kind::Poplar { (k as f32 + 0.5) / n as f32 * 2.0 - 1.0 } else { hash01(id, 2) * 1.6 - 0.65 };
        let out = hash01(id, 3).sqrt() * 0.55 * (1.0 - y * y * 0.4).max(0.0);
        let p = mid + Vec3::new(a.cos() * out, y * 0.5, a.sin() * out) * half;
        // Patches are as big as the crown's width allows (a poplar's span its height in a few).
        let size = if kind == Kind::Poplar { w * 1.1 } else { w.min(h * 1.3) * (0.72 + 0.22 * hash01(id, 4)) };
        // Which way it faces: any way round, tilted up between 15 and 80 degrees. The drone looks down on the
        // belts, and a patch it saw edge-on would leave a hole in the crown; the first faces straight up.
        let fa = hash01(id, 5) * std::f32::consts::TAU;
        let fe = if k == 0 { 1.5 } else { 0.26 + 1.1 * hash01(id, 6) };
        let face = Vec3::new(fa.cos() * fe.cos(), fe.sin(), fa.sin() * fe.cos());
        let up = (Vec3::Y - face * face.y).normalize_or(Vec3::new(fa.sin(), 0.0, -fa.cos()));
        let right = up.cross(face).normalize_or(Vec3::X);
        let (r, u) = (right * size * 0.5, up * size * 0.5);
        let pts = [p - r - u, p + r - u, p + r + u, p - r + u];
        let cell = kind.side_cell() + if hash01(id, 7) < 0.5 { 0 } else { 8 };
        let uvs = [cell_uv(cell, 0.0, 1.0), cell_uv(cell, 1.0, 1.0), cell_uv(cell, 1.0, 0.0), cell_uv(cell, 0.0, 0.0)];
        m.poly_uv_normals(&pts, &uvs, &pts.map(normal), c);
    }
}

/// The texture coordinates of a cell's corner (`u`, `v` 0..1 across it).
fn cell_uv(cell: usize, u: f32, v: f32) -> Vec2 {
    let inset = 1.0 / CELL as f32;
    let (u, v) = (inset + u * (1.0 - 2.0 * inset), inset + v * (1.0 - 2.0 * inset));
    Vec2::new(((cell % CELLS) as f32 + u) / CELLS as f32, ((cell / CELLS) as f32 + v) / CELLS as f32)
}

/// A crown from the kind's cells: in June leafy patches, in February three crossed upright cards and one across
/// its top. Its foot's centre is `at`, its width `w` and height `h` (the kind's aspect, give or take). The normals
/// point out of a ball round the crown's middle, leaning up, so the crown is lit like a mass of leaves (sunny
/// side, shade side, lit top) and not like flat boards; the cards are drawn from both faces with the same normals
/// (the material is not double-sided), so no card's back goes dark.
#[allow(clippy::too_many_arguments)]
pub fn crown(m: &mut crate::meshes::MeshBuilder, kind: Kind, at: Vec3, w: f32, h: f32, yaw: f32, flip: bool, c: crate::meshes::Rgb) {
    let mid = at + Vec3::Y * h * 0.5;
    let normal = |p: Vec3| {
        let d = (p - mid) / Vec3::new(w * 0.5, h * 0.5, w * 0.5);
        (d + Vec3::Y * 1.4).normalize_or(Vec3::Y)
    };
    if crate::diorama::diorama().summer() {
        patches(m, kind, mid, w, h, yaw, c, &normal);
        return;
    }
    let side = kind.side_cell();
    let (u0, u1) = if flip { (1.0, 0.0) } else { (0.0, 1.0) };
    for k in 0..3 {
        let a = yaw + k as f32 * std::f32::consts::PI / 3.0;
        let d = Vec3::new(a.cos(), 0.0, a.sin()) * (w * 0.5);
        let (p0, p1) = (at - d, at + d);
        let up = Vec3::Y * h;
        let pts = [p0, p1, p1 + up, p0 + up];
        let uvs = [cell_uv(side, u0, 1.0), cell_uv(side, u1, 1.0), cell_uv(side, u1, 0.0), cell_uv(side, u0, 0.0)];
        m.poly_uv_normals(&pts, &uvs, &pts.map(normal), c);
    }
    // The top card, a little below the crown's top so it sits inside the upright ones.
    let top = kind.top_cell();
    let y = at.y + h * if kind == Kind::Poplar { 0.75 } else { 0.62 };
    let r = w * if kind == Kind::Poplar { 0.45 } else { 0.5 };
    let rot = Quat::from_rotation_y(yaw * 1.7);
    let pts = [Vec3::new(-r, 0.0, -r), Vec3::new(r, 0.0, -r), Vec3::new(r, 0.0, r), Vec3::new(-r, 0.0, r)].map(|p| Vec3::new(at.x, y, at.z) + rot * p);
    let uvs = [cell_uv(top, 0.0, 0.0), cell_uv(top, 1.0, 0.0), cell_uv(top, 1.0, 1.0), cell_uv(top, 0.0, 1.0)];
    m.poly_uv_normals(&pts, &uvs, &pts.map(|p| (normal(p) + Vec3::Y).normalize()), c);
}
