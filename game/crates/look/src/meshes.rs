//! Procedural models. Everything in the diorama but the vehicles (baked.rs) is built here from primitives: flat-shaded, vertex-coloured,
//! low-poly, and meant to be seen the way the war was seen, through a drone's grainy video feed.

use bevy::{
    asset::RenderAssetUsages,
    image::{ImageSampler, ImageSamplerDescriptor},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

/// An sRGB RGBA8 texture of `w`×`h` from its bytes, sampled as `sampler` says.
pub fn rgba8_image(w: usize, h: usize, data: Vec<u8>, sampler: ImageSamplerDescriptor) -> Image {
    let mut img = Image::new(Extent3d { width: w as u32, height: h as u32, depth_or_array_layers: 1 }, TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default());
    img.sampler = ImageSampler::Descriptor(sampler);
    img
}

/// The mip chain of a square RGBA8 image, level 0 first and each next level a 2×2 box average of the last, down
/// to 1×1: what the GPU samples far away instead of the full image (no shimmer, and far cheaper to read).
pub fn with_mips(n: usize, mut data: Vec<u8>) -> (Vec<u8>, u32) {
    let mut levels = 1u32;
    let (mut size, mut at) = (n, 0usize);
    while size > 1 {
        let half = size / 2;
        let mut next = vec![0u8; half * half * 4];
        for y in 0..half {
            for x in 0..half {
                for c in 0..4 {
                    let px = |dx: usize, dy: usize| data[at + ((2 * y + dy) * size + 2 * x + dx) * 4 + c] as u32;
                    next[(y * half + x) * 4 + c] = ((px(0, 0) + px(1, 0) + px(0, 1) + px(1, 1) + 2) / 4) as u8;
                }
            }
        }
        at += size * size * 4;
        data.extend_from_slice(&next);
        size = half;
        levels += 1;
    }
    (data, levels)
}

/// Rebuild the mip levels of a square RGBA8 image whose level 0 has changed (`data` holds the whole chain).
pub fn refresh_mips(n: usize, data: &mut [u8]) {
    let (mut size, mut at) = (n, 0usize);
    while size > 1 {
        let half = size / 2;
        let next_at = at + size * size * 4;
        for y in 0..half {
            for x in 0..half {
                for c in 0..4 {
                    let px = |dx: usize, dy: usize| data[at + ((2 * y + dy) * size + 2 * x + dx) * 4 + c] as u32;
                    let v = ((px(0, 0) + px(1, 0) + px(0, 1) + px(1, 1) + 2) / 4) as u8;
                    data[next_at + (y * half + x) * 4 + c] = v;
                }
            }
        }
        at = next_at;
        size = half;
    }
}

/// A square RGBA8 texture with its full mip chain, sampled trilinearly as `sampler` says (its mipmap filter is
/// set to linear). `srgb`: colour (the ground) or plain data (noise).
pub fn rgba8_mipped(n: usize, data: Vec<u8>, sampler: ImageSamplerDescriptor, srgb: bool) -> Image {
    let format = if srgb { TextureFormat::Rgba8UnormSrgb } else { TextureFormat::Rgba8Unorm };
    // Made from level 0 (the size Image::new checks), then given the whole chain.
    let mut img = Image::new(Extent3d { width: n as u32, height: n as u32, depth_or_array_layers: 1 }, TextureDimension::D2, data[..n * n * 4].to_vec(), format, RenderAssetUsages::default());
    let (data, levels) = with_mips(n, data);
    img.data = Some(data);
    img.texture_descriptor.mip_level_count = levels;
    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor { mipmap_filter: bevy::image::ImageFilterMode::Linear, ..sampler });
    img
}

#[derive(Default)]
pub struct MeshBuilder {
    pos: Vec<[f32; 3]>,
    nor: Vec<[f32; 3]>,
    col: Vec<[f32; 4]>,
    uv: Vec<[f32; 2]>,
    idx: Vec<u32>,
}

pub type Rgb = [f32; 3];

fn lin(c: Rgb) -> [f32; 4] {
    let l = Color::srgb(c[0], c[1], c[2]).to_linear();
    [l.red, l.green, l.blue, 1.0]
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.idx.is_empty()
    }

    /// A flat polygon (convex, counter-clockwise when seen from the front).
    pub fn poly(&mut self, pts: &[Vec3], c: Rgb) {
        self.poly_uv(pts, &vec![Vec2::ZERO; pts.len()], c);
    }

    pub fn poly_uv(&mut self, pts: &[Vec3], uvs: &[Vec2], c: Rgb) {
        if pts.len() < 3 {
            return;
        }
        let n = (pts[1] - pts[0]).cross(pts[2] - pts[0]).normalize_or_zero();
        let base = self.pos.len() as u32;
        let col = lin(c);
        for (p, uv) in pts.iter().zip(uvs) {
            self.pos.push(p.to_array());
            self.nor.push(n.to_array());
            self.col.push(col);
            self.uv.push(uv.to_array());
        }
        for i in 1..pts.len() as u32 - 1 {
            self.idx.extend_from_slice(&[base, base + i, base + i + 1]);
        }
    }

    /// A flat polygon with a normal of its own at each corner (a card standing in for something rounder).
    pub fn poly_uv_normals(&mut self, pts: &[Vec3], uvs: &[Vec2], normals: &[Vec3], c: Rgb) {
        let base = self.pos.len() as u32;
        let col = lin(c);
        for ((p, uv), n) in pts.iter().zip(uvs).zip(normals) {
            self.pos.push(p.to_array());
            self.nor.push(n.to_array());
            self.col.push(col);
            self.uv.push(uv.to_array());
        }
        for i in 1..pts.len() as u32 - 1 {
            self.idx.extend_from_slice(&[base, base + i, base + i + 1]);
        }
    }

    pub fn quad(&mut self, a: Vec3, b: Vec3, c: Vec3, d: Vec3, col: Rgb) {
        self.poly(&[a, b, c, d], col);
    }

    /// An oriented box.
    pub fn cuboid(&mut self, center: Vec3, half: Vec3, rot: Quat, c: Rgb) {
        let corner = |sx: f32, sy: f32, sz: f32| center + rot * (half * Vec3::new(sx, sy, sz));
        let p = [
            corner(-1., -1., -1.), corner(1., -1., -1.), corner(1., 1., -1.), corner(-1., 1., -1.),
            corner(-1., -1., 1.), corner(1., -1., 1.), corner(1., 1., 1.), corner(-1., 1., 1.),
        ];
        self.quad(p[4], p[5], p[6], p[7], c); // +z
        self.quad(p[1], p[0], p[3], p[2], c); // -z
        self.quad(p[5], p[1], p[2], p[6], c); // +x
        self.quad(p[0], p[4], p[7], p[3], c); // -x
        self.quad(p[3], p[7], p[6], p[2], c); // +y
        self.quad(p[0], p[1], p[5], p[4], c); // -y
    }

    pub fn boxc(&mut self, center: Vec3, size: Vec3, c: Rgb) {
        self.cuboid(center, size * 0.5, Quat::IDENTITY, c);
    }

    /// Extrude a side profile (points in (z, y), convex, any winding) across x from -hw to +hw.
    pub fn extrude_x(&mut self, profile: &[Vec2], hw: f32, offset: Vec3, c: Rgb) {
        let mut prof: Vec<Vec2> = profile.to_vec();
        // Make the winding counter-clockwise in (z, y).
        let area: f32 = prof.iter().zip(prof.iter().cycle().skip(1)).map(|(a, b)| a.x * b.y - b.x * a.y).sum();
        if area < 0.0 {
            prof.reverse();
        }
        let left: Vec<Vec3> = prof.iter().map(|p| offset + Vec3::new(-hw, p.y, p.x)).collect();
        let right: Vec<Vec3> = prof.iter().map(|p| offset + Vec3::new(hw, p.y, p.x)).collect();
        // Seen from +x, (z, y) counter-clockwise appears clockwise, so the +x cap is reversed.
        let mut r = right.clone();
        r.reverse();
        self.poly(&left, c);
        self.poly(&r, c);
        let n = prof.len();
        for i in 0..n {
            let j = (i + 1) % n;
            self.quad(left[i], right[i], right[j], left[j], c);
        }
    }

    /// A frustum along `axis` from `base`, radii r0 → r1.
    pub fn frustum(&mut self, base: Vec3, axis: Vec3, r0: f32, r1: f32, segs: u32, c: Rgb, caps: bool) {
        let len = axis.length();
        let dir = axis / len;
        let u = dir.any_orthonormal_vector();
        let w = dir.cross(u);
        let ring = |r: f32, t: f32, i: u32| {
            let a = i as f32 / segs as f32 * std::f32::consts::TAU;
            base + dir * t + (u * a.cos() + w * a.sin()) * r
        };
        for i in 0..segs {
            let (a0, a1) = (ring(r0, 0.0, i), ring(r0, 0.0, i + 1));
            let (b0, b1) = (ring(r1, len, i), ring(r1, len, i + 1));
            self.quad(a0, a1, b1, b0, c);
        }
        if caps {
            let bottom: Vec<Vec3> = (0..segs).rev().map(|i| ring(r0, 0.0, i)).collect();
            let top: Vec<Vec3> = (0..segs).map(|i| ring(r1, len, i)).collect();
            self.poly(&bottom, c);
            self.poly(&top, c);
        }
    }

    pub fn cylinder(&mut self, base: Vec3, axis: Vec3, r: f32, segs: u32, c: Rgb) {
        self.frustum(base, axis, r, r, segs, c, true);
    }

    /// The upper half of an ellipsoid (a turret).
    pub fn dome(&mut self, center: Vec3, radii: Vec3, rings: u32, segs: u32, c: Rgb) {
        let p = |ri: u32, si: u32| {
            let phi = ri as f32 / rings as f32 * std::f32::consts::FRAC_PI_2;
            let th = si as f32 / segs as f32 * std::f32::consts::TAU;
            center + Vec3::new(phi.cos() * th.cos(), phi.sin(), phi.cos() * th.sin()) * radii
        };
        for ri in 0..rings {
            for si in 0..segs {
                let (a, b, cc, d) = (p(ri, si), p(ri, si + 1), p(ri + 1, si + 1), p(ri + 1, si));
                if ri + 1 == rings {
                    self.poly(&[a, d, b], c);
                } else {
                    self.quad(a, d, cc, b, c);
                }
            }
        }
        let bottom: Vec<Vec3> = (0..segs).map(|si| p(0, si)).collect();
        self.poly(&bottom, c);
    }

    pub fn append(&mut self, other: &MeshBuilder, xf: Transform) {
        let base = self.pos.len() as u32;
        for i in 0..other.pos.len() {
            self.pos.push(xf.transform_point(Vec3::from(other.pos[i])).to_array());
            self.nor.push((xf.rotation * Vec3::from(other.nor[i])).to_array());
            self.col.push(other.col[i]);
            self.uv.push(other.uv[i]);
        }
        self.idx.extend(other.idx.iter().map(|i| i + base));
    }

    pub fn build(self) -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.pos)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.nor)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.col)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uv)
            .with_inserted_indices(Indices::U32(self.idx))
    }
}

// ---------------------------------------------------------------------------------------------------------
// Palette

pub const STEEL: Rgb = [0.20, 0.21, 0.19];

// ---------------------------------------------------------------------------------------------------------
// Vehicles. Local frame: forward is -Z, up is +Y, origin on the ground at the hull centre. The vehicles are real
// models (baked.rs); what is built here goes on them.

/// KMT-7 mine plow: two toothed blades ahead of the tracks, on arms from the hull's nose.
pub fn kmt7() -> MeshBuilder {
    let mut h = MeshBuilder::new();
    for side in [-1.0f32, 1.0] {
        h.boxc(Vec3::new(side * 1.38, 0.25, -4.25), Vec3::new(0.85, 0.35, 0.25), STEEL);
        for t in 0..4 {
            h.boxc(Vec3::new(side * 1.38 + (t as f32 - 1.5) * 0.2, 0.08, -4.45), Vec3::new(0.06, 0.2, 0.35), STEEL);
        }
        h.boxc(Vec3::new(side * 1.1, 0.75, -3.75), Vec3::new(0.12, 0.12, 0.9), STEEL);
    }
    h
}

// ---------------------------------------------------------------------------------------------------------
// People. Body and legs separate so the legs can swing.

pub struct FigureModel {
    pub body: MeshBuilder,
    pub leg: MeshBuilder,
    /// Hip position of the left leg (mirror for the right).
    pub hip: Vec3,
}

pub fn figure(coat: Rgb, trousers: Rgb, helmet: Rgb, tape: Option<Rgb>) -> FigureModel {
    let mut b = MeshBuilder::new();
    // Torso with body armour, pack.
    b.boxc(Vec3::new(0.0, 1.22, 0.0), Vec3::new(0.46, 0.6, 0.28), coat);
    b.boxc(Vec3::new(0.0, 1.25, 0.2), Vec3::new(0.34, 0.42, 0.18), [coat[0] * 0.8, coat[1] * 0.8, coat[2] * 0.8]);
    // Head and helmet.
    b.boxc(Vec3::new(0.0, 1.66, -0.02), Vec3::new(0.2, 0.22, 0.22), [0.62, 0.5, 0.42]);
    b.dome(Vec3::new(0.0, 1.71, 0.0), Vec3::new(0.15, 0.14, 0.16), 2, 8, helmet);
    // Arms forward, holding a rifle.
    for side in [-1.0f32, 1.0] {
        let rot = Quat::from_rotation_x(-0.9) * Quat::from_rotation_z(side * 0.15);
        b.cuboid(Vec3::new(side * 0.25, 1.28, -0.16), Vec3::new(0.06, 0.28, 0.06), rot, coat);
    }
    b.boxc(Vec3::new(0.05, 1.2, -0.38), Vec3::new(0.05, 0.08, 0.85), [0.1, 0.1, 0.09]);
    if let Some(t) = tape {
        for side in [-1.0f32, 1.0] {
            b.boxc(Vec3::new(side * 0.27, 1.38, 0.0), Vec3::new(0.05, 0.07, 0.12), t);
        }
    }
    let mut l = MeshBuilder::new();
    // Hanging from the hip at the origin.
    l.boxc(Vec3::new(0.0, -0.45, 0.0), Vec3::new(0.15, 0.9, 0.17), trousers);
    l.boxc(Vec3::new(0.0, -0.88, -0.06), Vec3::new(0.14, 0.08, 0.28), [0.08, 0.08, 0.07]);
    FigureModel { body: b, leg: l, hip: Vec3::new(-0.11, 0.92, 0.0) }
}

/// A body lying in the snow: one mesh, no animation.
pub fn fallen(coat: Rgb, trousers: Rgb) -> MeshBuilder {
    let mut m = MeshBuilder::new();
    m.boxc(Vec3::new(0.0, 0.14, -0.3), Vec3::new(0.46, 0.26, 0.62), coat);
    m.boxc(Vec3::new(0.02, 0.12, -0.78), Vec3::new(0.22, 0.2, 0.24), [0.3, 0.32, 0.27]);
    m.cuboid(Vec3::new(-0.12, 0.1, 0.4), Vec3::new(0.08, 0.08, 0.45), Quat::from_rotation_y(0.15), trousers);
    m.cuboid(Vec3::new(0.14, 0.1, 0.38), Vec3::new(0.08, 0.08, 0.45), Quat::from_rotation_y(-0.25), trousers);
    m.cuboid(Vec3::new(0.38, 0.08, -0.45), Vec3::new(0.06, 0.06, 0.3), Quat::from_rotation_y(0.9), coat);
    m
}

// ---------------------------------------------------------------------------------------------------------
// Buildings

/// A Soviet panel block: `sections` entrances of 15 m, `floors` storeys, 12 m deep. UVs in metres for the
/// tiled facade texture (12 m × 8.4 m per tile). Damaged sections are cut down to a ragged stump.
pub fn panel_block(sections: u32, floors: u32, damage: &[f32]) -> (MeshBuilder, MeshBuilder) {
    let mut facade = MeshBuilder::new();
    let mut solid = MeshBuilder::new();
    let (sec_w, depth, floor_h) = (15.0f32, 12.0f32, 2.8f32);
    let total = sections as f32 * sec_w;
    for s in 0..sections {
        let x0 = -total * 0.5 + s as f32 * sec_w;
        let x1 = x0 + sec_w;
        let dmg = damage.get(s as usize).copied().unwrap_or(0.0);
        let full = floors as f32 * floor_h;
        let hgt = full * (1.0 - dmg * 0.75);
        let (z0, z1) = (-depth * 0.5, depth * 0.5);
        let uvq = |a: Vec2, b: Vec2| [Vec2::new(a.x / 12.0, -a.y / 8.4), Vec2::new(b.x / 12.0, -a.y / 8.4), Vec2::new(b.x / 12.0, -b.y / 8.4), Vec2::new(a.x / 12.0, -b.y / 8.4)];
        // Front (-z) and back (+z) facades.
        facade.poly_uv(&[Vec3::new(x1, 0.0, z0), Vec3::new(x0, 0.0, z0), Vec3::new(x0, hgt, z0), Vec3::new(x1, hgt, z0)], &uvq(Vec2::new(x1, 0.0), Vec2::new(x0, hgt)), [1.0; 3]);
        facade.poly_uv(&[Vec3::new(x0, 0.0, z1), Vec3::new(x1, 0.0, z1), Vec3::new(x1, hgt, z1), Vec3::new(x0, hgt, z1)], &uvq(Vec2::new(x0, 0.0), Vec2::new(x1, hgt)), [0.9; 3]);
        if s == 0 {
            facade.poly_uv(&[Vec3::new(x0, 0.0, z0), Vec3::new(x0, 0.0, z1), Vec3::new(x0, hgt, z1), Vec3::new(x0, hgt, z0)], &uvq(Vec2::new(0.0, 0.0), Vec2::new(depth, hgt)), [0.85; 3]);
        }
        if s == sections - 1 {
            facade.poly_uv(&[Vec3::new(x1, 0.0, z1), Vec3::new(x1, 0.0, z0), Vec3::new(x1, hgt, z0), Vec3::new(x1, hgt, z1)], &uvq(Vec2::new(0.0, 0.0), Vec2::new(depth, hgt)), [0.85; 3]);
        }
        // Roof, or a ragged stump with exposed slabs.
        if dmg < 0.05 {
            // Flat roofs under February snow; the lift housings grey concrete.
            solid.boxc(Vec3::new((x0 + x1) * 0.5, hgt + 0.15, 0.0), Vec3::new(sec_w, 0.3, depth), [0.74, 0.75, 0.77]);
            solid.boxc(Vec3::new((x0 + x1) * 0.5, hgt + 1.1, 0.0), Vec3::new(3.0, 1.8, 3.0), [0.55, 0.54, 0.52]);
        } else {
            for k in 0..5 {
                let fx = x0 + 1.5 + k as f32 * 3.0;
                let dh = (crate::geo::hash01(s * 13 + k, floors) * 2.0 - 1.0) * floor_h * 1.4 * dmg;
                solid.boxc(Vec3::new(fx, hgt + dh * 0.5, 0.0), Vec3::new(3.0, (dh.abs() + 0.3).max(0.3), depth * (0.6 + 0.3 * k as f32 % 0.4)), [0.36, 0.35, 0.33]);
            }
            // Rubble spilling out front and back.
            for side in [-1.0f32, 1.0] {
                let r = 2.0 + dmg * 6.0;
                solid.extrude_x(&[Vec2::new(0.0, 0.0), Vec2::new(side * r, 0.0), Vec2::new(0.0, r * 0.5)], sec_w * 0.45, Vec3::new((x0 + x1) * 0.5, 0.0, side * depth * 0.5), [0.42, 0.40, 0.37]);
            }
        }
    }
    (facade, solid)
}

/// A dacha: a small house with a gable roof, or a collapsed one.
pub fn dacha(seed: u32, ruined: bool) -> MeshBuilder {
    let mut m = MeshBuilder::new();
    let hs = |k: u32| crate::geo::hash01(seed * 31 + k, 77);
    let w = 4.0 + hs(1) * 2.5;
    let d = 4.5 + hs(2) * 2.0;
    let wall_colours: [Rgb; 5] = [[0.55, 0.62, 0.66], [0.70, 0.68, 0.60], [0.52, 0.60, 0.48], [0.62, 0.42, 0.34], [0.75, 0.74, 0.72]];
    let wall = wall_colours[(hs(3) * 5.0) as usize % 5];
    let roof: Rgb = if hs(4) > 0.5 { [0.35, 0.36, 0.36] } else { [0.45, 0.30, 0.25] };
    if ruined {
        m.boxc(Vec3::new(0.0, 0.5, 0.0), Vec3::new(w, 1.0, d), [0.25, 0.23, 0.21]);
        m.cuboid(Vec3::new(0.3, 1.2, 0.2), Vec3::new(w * 0.55, 0.05, d * 0.5), Quat::from_rotation_z(0.25) * Quat::from_rotation_x(0.1), roof);
        m.boxc(Vec3::new(-w * 0.45, 1.2, 0.0), Vec3::new(0.2, 2.4, d * 0.7), [wall[0] * 0.5, wall[1] * 0.5, wall[2] * 0.5]);
    } else {
        m.boxc(Vec3::new(0.0, 1.3, 0.0), Vec3::new(w, 2.6, d), wall);
        let rh = 1.8;
        let p = [Vec2::new(-d * 0.55, 2.6), Vec2::new(d * 0.55, 2.6), Vec2::new(0.0, 2.6 + rh)];
        m.extrude_x(&p, w * 0.55, Vec3::ZERO, roof);
    }
    m
}

/// A winter shelterbelt tree: a trunk (the crown is a separate alpha-tested card set).
pub fn trunk(h: f32, r: f32) -> MeshBuilder {
    let mut m = MeshBuilder::new();
    m.frustum(Vec3::ZERO, Vec3::new(0.0, h * 0.75, 0.0), r, r * 0.35, 5, [0.30, 0.27, 0.24], false);
    m
}

/// A concrete power-line pole, maybe leaning.
pub fn pole(lean: f32) -> MeshBuilder {
    let mut m = MeshBuilder::new();
    let rot = Quat::from_rotation_z(lean);
    m.cuboid(rot * Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.12, 5.0, 0.12), rot, [0.55, 0.55, 0.53]);
    m.cuboid(rot * Vec3::new(0.0, 9.4, 0.0), Vec3::new(0.9, 0.06, 0.06), rot, [0.3, 0.3, 0.3]);
    m
}

// ---------------------------------------------------------------------------------------------------------
// Summer 2023: the Western armour of the 47th and 33rd Brigades, and what stood against it.

pub const WEST_GREEN: Rgb = [0.31, 0.34, 0.25];

/// A full-width mine plow on arms ahead of the hull: Pearson (Leopard 2R) or a dozer-plow (Wisent).
pub fn full_plow(z: f32, width: f32) -> MeshBuilder {
    let mut h = MeshBuilder::new();
    h.cuboid(Vec3::new(0.0, 0.45, z), Vec3::new(width * 0.5, 0.42, 0.12), Quat::from_rotation_x(-0.35), STEEL);
    for k in 0..12 {
        let x = (k as f32 / 11.0 - 0.5) * width * 0.95;
        h.cuboid(Vec3::new(x, 0.1, z - 0.35), Vec3::new(0.05, 0.12, 0.35), Quat::from_rotation_x(0.5), STEEL);
    }
    for side in [-1.0f32, 1.0] {
        h.boxc(Vec3::new(side * 1.2, 0.8, z + 0.8), Vec3::new(0.16, 0.16, 1.7), STEEL);
    }
    h
}

/// Leopard 2R: on a 2A4 hull, a fixed armoured superstructure instead of a turret, and the lane-marking rack
/// (all six Finnish vehicles went to Ukraine). The superstructure and where it sits on the hull.
pub fn leopard2r_top(body: Rgb) -> (MeshBuilder, Vec3) {
    let mut t = MeshBuilder::new();
    let prof = [Vec2::new(2.2, 0.0), Vec2::new(2.2, 0.95), Vec2::new(-0.6, 1.0), Vec2::new(-1.6, 0.3), Vec2::new(-1.6, 0.0)];
    t.extrude_x(&prof, 1.35, Vec3::ZERO, body);
    t.cylinder(Vec3::new(0.6, 1.0, 0.4), Vec3::new(0.0, 0.3, 0.0), 0.3, 8, body);
    // Lane markers in their rack at the rear.
    for k in 0..6 {
        t.cylinder(Vec3::new(-1.2 + k as f32 * 0.12, 0.6, 2.25), Vec3::new(0.0, 1.1, 0.0), 0.025, 4, [0.85, 0.6, 0.2]);
    }
    (t, Vec3::new(0.0, 1.6, 0.2))
}

/// Wisent 1: on a Leopard 1 hull (here the 2A4's), an armoured box and a crane, for clearing and recovery.
pub fn wisent_top(body: Rgb) -> (MeshBuilder, Vec3) {
    let mut t = MeshBuilder::new();
    t.boxc(Vec3::new(0.0, 0.5, 0.0), Vec3::new(2.3, 1.0, 2.6), body);
    // The crane folded back along the hull.
    t.cuboid(Vec3::new(0.85, 1.15, 0.4), Vec3::new(0.14, 0.14, 2.3), Quat::from_rotation_x(0.12), [0.62, 0.55, 0.22]);
    (t, Vec3::new(0.0, 1.55, -0.3))
}

/// KMT-5-style mine rollers: a set of heavy discs ahead of each track on a frame.
pub fn kmt_rollers() -> MeshBuilder {
    let mut h = MeshBuilder::new();
    for side in [-1.0f32, 1.0] {
        for k in 0..3 {
            h.cylinder(Vec3::new(side * 1.35 - 0.3 + k as f32 * 0.22, 0.42, -4.7), Vec3::new(0.12, 0.0, 0.0), 0.42, 10, STEEL);
        }
        h.boxc(Vec3::new(side * 1.2, 0.75, -3.85), Vec3::new(0.14, 0.14, 1.5), STEEL);
    }
    h.boxc(Vec3::new(0.0, 0.75, -4.7), Vec3::new(2.5, 0.12, 0.12), STEEL);
    h
}

/// BMR-2: on a turretless T-55 hull (here the T-64's), an armoured cab with a heavy machine gun.
pub fn bmr2_top(body: Rgb) -> (MeshBuilder, Vec3) {
    let mut t = MeshBuilder::new();
    t.frustum(Vec3::ZERO, Vec3::new(0.0, 0.55, 0.0), 0.75, 0.6, 8, body, true);
    t.cylinder(Vec3::new(0.0, 0.62, -0.2), Vec3::new(0.0, 0.0, -1.4), 0.03, 5, STEEL);
    (t, Vec3::new(0.0, 1.5, -0.6))
}

/// A Rozhnovsky water tower, the steppe village's landmark and its observers' perch: a brick shaft and a
/// steel tank. The eye sits in the tank's hatch at about 18 m.
pub fn water_tower() -> MeshBuilder {
    let mut m = MeshBuilder::new();
    m.frustum(Vec3::ZERO, Vec3::new(0.0, 14.5, 0.0), 1.5, 1.2, 10, [0.55, 0.36, 0.28], false);
    m.frustum(Vec3::new(0.0, 14.5, 0.0), Vec3::new(0.0, 1.0, 0.0), 1.2, 2.5, 12, [0.38, 0.40, 0.38], false);
    m.cylinder(Vec3::new(0.0, 15.5, 0.0), Vec3::new(0.0, 2.6, 0.0), 2.5, 12, [0.42, 0.44, 0.42]);
    m.frustum(Vec3::new(0.0, 18.1, 0.0), Vec3::new(0.0, 1.1, 0.0), 2.6, 0.2, 12, [0.30, 0.31, 0.30], false);
    // A shell hole through the tank, and the ladder.
    m.boxc(Vec3::new(1.9, 16.6, 1.2), Vec3::new(0.6, 0.9, 0.9), [0.06, 0.06, 0.06]);
    m.boxc(Vec3::new(0.0, 7.5, -1.45), Vec3::new(0.5, 15.0, 0.05), [0.2, 0.2, 0.2]);
    m
}

/// A long collective-farm shed, roof half fallen in.
pub fn farm_shed(len: f32, ruined: f32) -> MeshBuilder {
    let mut m = MeshBuilder::new();
    let wall: Rgb = [0.58, 0.56, 0.52];
    m.boxc(Vec3::new(0.0, 1.8, 0.0), Vec3::new(12.0, 3.6, len), wall);
    let keep = len * (1.0 - ruined);
    if keep > 1.0 {
        let p = [Vec2::new(-6.6, 3.6), Vec2::new(6.6, 3.6), Vec2::new(0.0, 5.6)];
        let mut roof = MeshBuilder::new();
        roof.extrude_x(&p, keep * 0.5, Vec3::ZERO, [0.42, 0.44, 0.43]);
        m.append(&roof, Transform::from_xyz(0.0, 0.0, -len * 0.5 + keep * 0.5).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)));
    }
    m
}

/// A combine harvester burned in last summer's field.
pub fn combine() -> MeshBuilder {
    let mut m = MeshBuilder::new();
    let c: Rgb = [0.23, 0.2, 0.18];
    m.boxc(Vec3::new(0.0, 2.0, 0.6), Vec3::new(2.8, 2.6, 6.0), c);
    m.boxc(Vec3::new(0.0, 3.6, -1.6), Vec3::new(1.6, 1.2, 1.6), [0.12, 0.12, 0.12]);
    m.boxc(Vec3::new(0.0, 0.8, -3.4), Vec3::new(6.4, 0.8, 1.4), [0.3, 0.22, 0.17]);
    for (x, z, r) in [(-1.5f32, -1.0f32, 0.85f32), (1.5, -1.0, 0.85), (-1.4, 2.6, 0.55), (1.4, 2.6, 0.55)] {
        m.cylinder(Vec3::new(x - 0.25, r, z), Vec3::new(0.5, 0.0, 0.0), r, 8, [0.08, 0.08, 0.08]);
    }
    m.cuboid(Vec3::new(1.6, 3.4, 1.8), Vec3::new(0.18, 0.18, 2.2), Quat::from_rotation_x(0.3) * Quat::from_rotation_y(0.4), c);
    m
}
