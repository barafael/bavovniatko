//! Procedural models. Everything in the diorama is built here from primitives: flat-shaded, vertex-coloured,
//! low-poly, and meant to be seen the way the war was seen, through a drone's grainy video feed.

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

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

    /// Multiply every vertex colour (for charring, dirt, variation).
    pub fn tint(&mut self, f: [f32; 3]) {
        for c in &mut self.col {
            c[0] *= f[0];
            c[1] *= f[1];
            c[2] *= f[2];
        }
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

pub const RU_GREEN: Rgb = [0.40, 0.43, 0.33];
pub const UA_GREEN: Rgb = [0.34, 0.36, 0.25];
pub const MUD: Rgb = [0.33, 0.29, 0.24];
pub const TRACK: Rgb = [0.13, 0.13, 0.12];
pub const STEEL: Rgb = [0.20, 0.21, 0.19];
pub const ERA: Rgb = [0.30, 0.32, 0.23];
pub const DRUM: Rgb = [0.22, 0.25, 0.18];

// ---------------------------------------------------------------------------------------------------------
// Vehicles. Local frame: forward is -Z, up is +Y, origin on the ground at the hull centre.

fn running_gear(m: &mut MeshBuilder, half_len: f32, track_x: f32, wheels: u32, wheel_r: f32, skirt: bool, body: Rgb) {
    for side in [-1.0f32, 1.0] {
        // Track run.
        m.boxc(Vec3::new(side * track_x, 0.45, 0.0), Vec3::new(0.58, 0.9, half_len * 2.0), TRACK);
        // Road wheels just proud of the track.
        for i in 0..wheels {
            let z = -half_len + 0.6 + i as f32 * (2.0 * half_len - 1.2) / (wheels - 1) as f32;
            m.cylinder(Vec3::new(side * (track_x + 0.30), wheel_r + 0.05, z), Vec3::new(side * 0.06, 0.0, 0.0), wheel_r, 8, STEEL);
        }
        if skirt {
            m.boxc(Vec3::new(side * (track_x + 0.33), 0.78, -0.2), Vec3::new(0.05, 0.42, half_len * 1.8), body);
        }
        // Fender.
        m.boxc(Vec3::new(side * track_x, 0.95, 0.0), Vec3::new(0.66, 0.05, half_len * 2.05), body);
    }
}

fn era_grid(m: &mut MeshBuilder, origin: Vec3, rot: Quat, cols: u32, rows: u32, step: Vec2) {
    for r in 0..rows {
        for c in 0..cols {
            let off = Vec3::new((c as f32 - (cols - 1) as f32 * 0.5) * step.x, 0.0, (r as f32 - (rows - 1) as f32 * 0.5) * step.y);
            m.cuboid(origin + rot * off, Vec3::new(step.x * 0.42, 0.05, step.y * 0.42), rot, ERA);
        }
    }
}

pub struct VehicleModel {
    pub hull: MeshBuilder,
    pub turret: MeshBuilder,
    /// Turret ring position in the hull frame.
    pub turret_at: Vec3,
}

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

/// T-80BV / T-72B3 family (and, recoloured, the Ukrainian T-64BV).
pub fn tank(body: Rgb, plow: bool, ukrainian: bool) -> VehicleModel {
    let mut h = MeshBuilder::new();
    running_gear(&mut h, 3.2, 1.38, 6, 0.36, true, body);
    // Hull: low, with a sharp glacis.
    let prof = [Vec2::new(3.45, 0.55), Vec2::new(3.45, 1.40), Vec2::new(-2.35, 1.45), Vec2::new(-3.55, 0.98), Vec2::new(-3.3, 0.55)];
    h.extrude_x(&prof, 1.12, Vec3::ZERO, body);
    // Glacis ERA.
    let glacis = Quat::from_rotation_x(-0.37);
    era_grid(&mut h, Vec3::new(0.0, 1.22, -2.95), glacis, 6, 3, Vec2::new(0.34, 0.34));
    // Engine deck grilles, exhaust.
    h.boxc(Vec3::new(0.0, 1.45, 2.4), Vec3::new(1.8, 0.06, 1.4), [0.18, 0.19, 0.15]);
    if !ukrainian {
        // Fuel drums on the rear.
        for x in [-0.55f32, 0.55] {
            h.cylinder(Vec3::new(x - 0.5, 1.15, 3.62), Vec3::new(1.0, 0.0, 0.0), 0.3, 8, DRUM);
        }
    } else {
        h.boxc(Vec3::new(0.0, 1.1, 3.5), Vec3::new(2.0, 0.5, 0.2), [0.25, 0.26, 0.2]);
    }
    if plow {
        h.append(&kmt7(), Transform::IDENTITY);
    }

    let mut t = MeshBuilder::new();
    let tr = if ukrainian { Vec3::new(1.05, 0.62, 1.15) } else { Vec3::new(1.12, 0.68, 1.3) };
    t.dome(Vec3::new(0.0, 0.0, 0.15), tr, 4, 12, body);
    // Turret-front ERA "brow".
    for side in [-1.0f32, 1.0] {
        let rot = Quat::from_rotation_y(side * 0.55) * Quat::from_rotation_x(-1.1);
        era_grid(&mut t, Vec3::new(side * 0.55, 0.42, -0.85), rot, 3, 2, Vec2::new(0.3, 0.3));
    }
    // Gun: 125 mm with thermal sleeve and fume extractor.
    t.cylinder(Vec3::new(0.0, 0.38, -0.95), Vec3::new(0.0, 0.0, -5.3), 0.085, 8, STEEL);
    t.cylinder(Vec3::new(0.0, 0.38, -2.4), Vec3::new(0.0, 0.0, -0.8), 0.13, 8, STEEL);
    t.frustum(Vec3::new(0.0, 0.38, -0.7), Vec3::new(0.0, 0.0, -0.4), 0.22, 0.16, 8, body, false);
    // Commander's cupola, MG, snorkel.
    t.cylinder(Vec3::new(0.35, 0.6, 0.35), Vec3::new(0.0, 0.18, 0.0), 0.32, 8, body);
    t.cylinder(Vec3::new(0.35, 0.86, 0.2), Vec3::new(0.0, 0.0, -1.0), 0.03, 4, STEEL);
    t.cylinder(Vec3::new(-0.7, 0.35, 1.25), Vec3::new(1.4, 0.0, 0.0), 0.09, 6, body);
    VehicleModel { hull: h, turret: t, turret_at: Vec3::new(0.0, 1.42, -0.35) }
}

/// BMP-1 / BMP-2: the sharp ribbed nose, the long glacis, the small turret.
pub fn bmp(body: Rgb, bmp2: bool) -> VehicleModel {
    let mut h = MeshBuilder::new();
    running_gear(&mut h, 3.0, 1.25, 6, 0.33, false, body);
    let prof = [Vec2::new(3.35, 0.5), Vec2::new(3.35, 1.62), Vec2::new(-1.4, 1.75), Vec2::new(-3.45, 0.72), Vec2::new(-3.1, 0.45)];
    h.extrude_x(&prof, 1.15, Vec3::ZERO, body);
    // Nose ribs.
    for i in 0..4 {
        h.boxc(Vec3::new(0.0, 0.85 + i as f32 * 0.15, -3.1 + i as f32 * 0.32), Vec3::new(2.2, 0.04, 0.06), body);
    }
    // Rear doors and roof hatches.
    for x in [-0.55f32, 0.55] {
        h.boxc(Vec3::new(x, 1.05, 3.38), Vec3::new(0.85, 0.95, 0.06), body);
        h.boxc(Vec3::new(x, 1.66, 1.9), Vec3::new(0.7, 0.05, 1.6), [0.23, 0.25, 0.18]);
    }
    let mut t = MeshBuilder::new();
    t.frustum(Vec3::ZERO, Vec3::new(0.0, 0.48, 0.0), 0.95, 0.62, 10, body, true);
    if bmp2 {
        t.cylinder(Vec3::new(0.0, 0.3, -0.6), Vec3::new(0.0, 0.0, -2.6), 0.05, 6, STEEL);
        t.cylinder(Vec3::new(0.0, 0.62, 0.0), Vec3::new(0.0, 0.0, -1.3), 0.08, 6, [0.25, 0.27, 0.2]);
    } else {
        t.cylinder(Vec3::new(0.0, 0.3, -0.6), Vec3::new(0.0, 0.0, -1.6), 0.07, 6, STEEL);
        t.cylinder(Vec3::new(0.0, 0.5, -2.2), Vec3::new(0.0, 0.0, -0.5), 0.06, 6, body);
    }
    VehicleModel { hull: h, turret: t, turret_at: Vec3::new(0.0, 1.62, -0.55) }
}

/// MT-LB: the flat tractor-carrier.
pub fn mtlb(body: Rgb) -> VehicleModel {
    let mut h = MeshBuilder::new();
    running_gear(&mut h, 2.9, 1.2, 6, 0.3, false, body);
    let prof = [Vec2::new(3.2, 0.45), Vec2::new(3.2, 1.55), Vec2::new(-2.4, 1.6), Vec2::new(-3.25, 1.05), Vec2::new(-3.2, 0.45)];
    h.extrude_x(&prof, 1.35, Vec3::ZERO, body);
    h.boxc(Vec3::new(0.0, 1.62, 1.5), Vec3::new(2.4, 0.06, 2.6), [0.22, 0.24, 0.18]);
    let mut t = MeshBuilder::new();
    t.frustum(Vec3::ZERO, Vec3::new(0.0, 0.35, 0.0), 0.42, 0.34, 8, body, true);
    t.cylinder(Vec3::new(0.0, 0.2, -0.3), Vec3::new(0.0, 0.0, -0.9), 0.025, 4, STEEL);
    VehicleModel { hull: h, turret: t, turret_at: Vec3::new(-0.7, 1.6, -1.6) }
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

/// Three crossed vertical cards carrying the branch texture.
pub fn crown_cards(m: &mut MeshBuilder, at: Vec3, w: f32, h: f32, yaw: f32, c: Rgb) {
    for k in 0..3 {
        let a = yaw + k as f32 * std::f32::consts::PI / 3.0;
        let d = Vec3::new(a.cos(), 0.0, a.sin()) * (w * 0.5);
        let (p0, p1) = (at - d, at + d);
        let up = Vec3::Y * h;
        m.poly_uv(&[p0, p1, p1 + up, p0 + up], &[Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0), Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0)], c);
    }
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

impl MeshBuilder {
    /// A polygon with both faces, for wedges and plates whose winding is not worth reasoning about.
    pub fn poly2(&mut self, pts: &[Vec3], c: Rgb) {
        self.poly(pts, c);
        let rev: Vec<Vec3> = pts.iter().rev().copied().collect();
        self.poly(&rev, c);
    }
}

/// The Leopard 2 hull and running gear (2A4 and 2A6 alike): long, high-sided, heavy skirts.
fn leopard_hull(body: Rgb) -> MeshBuilder {
    let mut h = MeshBuilder::new();
    running_gear(&mut h, 3.55, 1.5, 7, 0.36, true, body);
    let prof = [Vec2::new(3.75, 0.5), Vec2::new(3.75, 1.55), Vec2::new(-2.55, 1.62), Vec2::new(-3.9, 1.12), Vec2::new(-3.65, 0.5)];
    h.extrude_x(&prof, 1.2, Vec3::ZERO, body);
    // The 2A6's thick front skirt sections, and the engine deck.
    for side in [-1.0f32, 1.0] {
        h.boxc(Vec3::new(side * 1.84, 0.92, -2.9), Vec3::new(0.12, 0.62, 1.4), body);
    }
    h.boxc(Vec3::new(0.0, 1.63, 2.5), Vec3::new(2.2, 0.05, 1.8), [0.2, 0.22, 0.17]);
    h.boxc(Vec3::new(0.0, 1.15, 3.8), Vec3::new(2.3, 0.6, 0.12), [body[0] * 0.85, body[1] * 0.85, body[2] * 0.85]);
    h
}

/// Leopard 2A6: the arrowhead of spaced armour on the turret front, and the long L55 gun.
pub fn leopard2a6(body: Rgb) -> VehicleModel {
    let h = leopard_hull(body);
    let mut t = MeshBuilder::new();
    let prof = [Vec2::new(2.7, 0.0), Vec2::new(2.7, 0.66), Vec2::new(-1.2, 0.76), Vec2::new(-1.45, 0.08), Vec2::new(-1.4, 0.0)];
    t.extrude_x(&prof, 1.62, Vec3::ZERO, body);
    // The wedge: two sloped plates meeting ahead of the turret face.
    let (l0, r0, tip0) = (Vec3::new(-1.62, 0.06, -1.35), Vec3::new(1.62, 0.06, -1.35), Vec3::new(0.0, 0.1, -2.75));
    let (l1, r1, tip1) = (Vec3::new(-1.62, 0.74, -1.25), Vec3::new(1.62, 0.74, -1.25), Vec3::new(0.0, 0.58, -2.6));
    for (a, b, c, d) in [(l0, tip0, tip1, l1), (tip0, r0, r1, tip1)] {
        t.poly2(&[a, b, c, d], body);
    }
    t.poly2(&[l1, tip1, r1], [body[0] * 0.92, body[1] * 0.92, body[2] * 0.92]);
    // Mantlet and the 120 mm L55, 6.6 m of barrel with its thermal sleeve and fume extractor.
    t.boxc(Vec3::new(0.0, 0.4, -1.55), Vec3::new(0.7, 0.55, 0.5), body);
    t.cylinder(Vec3::new(0.0, 0.42, -1.7), Vec3::new(0.0, 0.0, -6.6), 0.075, 8, body);
    t.cylinder(Vec3::new(0.0, 0.42, -3.6), Vec3::new(0.0, 0.0, -0.9), 0.12, 8, body);
    // Commander's PERI sight, gunner's sight, smoke dischargers, the stowage basket on the bustle.
    t.cylinder(Vec3::new(-0.7, 0.72, -0.4), Vec3::new(0.0, 0.42, 0.0), 0.17, 8, body);
    t.boxc(Vec3::new(-0.7, 1.2, -0.4), Vec3::new(0.32, 0.22, 0.32), [0.18, 0.19, 0.16]);
    t.boxc(Vec3::new(0.95, 0.82, -1.0), Vec3::new(0.45, 0.24, 0.5), body);
    for side in [-1.0f32, 1.0] {
        for k in 0..4 {
            t.cylinder(Vec3::new(side * 1.55, 0.5, 0.2 + k as f32 * 0.18), Vec3::new(side * 0.25, 0.08, -0.05), 0.05, 5, STEEL);
        }
    }
    t.boxc(Vec3::new(0.0, 0.5, 2.85), Vec3::new(2.8, 0.55, 0.35), [0.25, 0.26, 0.2]);
    VehicleModel { hull: h, turret: t, turret_at: Vec3::new(0.0, 1.6, -0.1) }
}

/// A full-width mine plow on arms ahead of the hull: Pearson (Leopard 2R) or a dozer-plow (Wisent).
fn full_plow(h: &mut MeshBuilder, z: f32, width: f32) {
    h.cuboid(Vec3::new(0.0, 0.45, z), Vec3::new(width * 0.5, 0.42, 0.12), Quat::from_rotation_x(-0.35), STEEL);
    for k in 0..12 {
        let x = (k as f32 / 11.0 - 0.5) * width * 0.95;
        h.cuboid(Vec3::new(x, 0.1, z - 0.35), Vec3::new(0.05, 0.12, 0.35), Quat::from_rotation_x(0.5), STEEL);
    }
    for side in [-1.0f32, 1.0] {
        h.boxc(Vec3::new(side * 1.2, 0.8, z + 0.8), Vec3::new(0.16, 0.16, 1.7), STEEL);
    }
}

/// The full-width plow on its own, for a hull that comes from elsewhere.
pub fn full_plow_only(z: f32, width: f32) -> MeshBuilder {
    let mut h = MeshBuilder::new();
    full_plow(&mut h, z, width);
    h
}

/// Leopard 2R: a 2A4 hull with a fixed armoured superstructure instead of a turret, a full-width plow and
/// the lane-marking rack (all six Finnish vehicles went to Ukraine).
pub fn leopard2r(body: Rgb) -> VehicleModel {
    let mut h = leopard_hull(body);
    full_plow(&mut h, -4.4, 4.2);
    let mut t = MeshBuilder::new();
    let prof = [Vec2::new(2.2, 0.0), Vec2::new(2.2, 0.95), Vec2::new(-0.6, 1.0), Vec2::new(-1.6, 0.3), Vec2::new(-1.6, 0.0)];
    t.extrude_x(&prof, 1.35, Vec3::ZERO, body);
    t.cylinder(Vec3::new(0.6, 1.0, 0.4), Vec3::new(0.0, 0.3, 0.0), 0.3, 8, body);
    // Lane markers in their rack at the rear.
    for k in 0..6 {
        t.cylinder(Vec3::new(-1.2 + k as f32 * 0.12, 0.6, 2.25), Vec3::new(0.0, 1.1, 0.0), 0.025, 4, [0.85, 0.6, 0.2]);
    }
    VehicleModel { hull: h, turret: t, turret_at: Vec3::new(0.0, 1.6, 0.2) }
}

/// Wisent 1: a Leopard 1 hull with a dozer-plow and a crane, for clearing and recovery.
pub fn wisent(body: Rgb) -> VehicleModel {
    let mut h = MeshBuilder::new();
    running_gear(&mut h, 3.3, 1.42, 7, 0.34, true, body);
    let prof = [Vec2::new(3.5, 0.5), Vec2::new(3.5, 1.5), Vec2::new(-2.4, 1.55), Vec2::new(-3.6, 1.05), Vec2::new(-3.4, 0.5)];
    h.extrude_x(&prof, 1.12, Vec3::ZERO, body);
    full_plow(&mut h, -4.1, 3.8);
    let mut t = MeshBuilder::new();
    t.boxc(Vec3::new(0.0, 0.5, 0.0), Vec3::new(2.3, 1.0, 2.6), body);
    // The crane folded back along the hull.
    t.cuboid(Vec3::new(0.85, 1.15, 0.4), Vec3::new(0.14, 0.14, 2.3), Quat::from_rotation_x(0.12), [0.62, 0.55, 0.22]);
    VehicleModel { hull: h, turret: t, turret_at: Vec3::new(0.0, 1.55, -0.3) }
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

/// BMR-2: a T-55 hull with its turret gone, an armoured cab, a heavy machine gun, and KMT-7 rollers ahead.
pub fn bmr2(body: Rgb) -> VehicleModel {
    let mut h = MeshBuilder::new();
    running_gear(&mut h, 3.0, 1.35, 5, 0.4, false, body);
    let prof = [Vec2::new(3.1, 0.5), Vec2::new(3.1, 1.45), Vec2::new(-2.2, 1.5), Vec2::new(-3.25, 1.0), Vec2::new(-3.05, 0.5)];
    h.extrude_x(&prof, 1.05, Vec3::ZERO, body);
    h.append(&kmt_rollers(), Transform::IDENTITY);
    let mut t = MeshBuilder::new();
    t.frustum(Vec3::ZERO, Vec3::new(0.0, 0.55, 0.0), 0.75, 0.6, 8, body, true);
    t.cylinder(Vec3::new(0.0, 0.62, -0.2), Vec3::new(0.0, 0.0, -1.4), 0.03, 5, STEEL);
    VehicleModel { hull: h, turret: t, turret_at: Vec3::new(0.0, 1.5, -0.6) }
}

/// M2A2 ODS Bradley: a tall box with bolted appliqué tiles, the 25 mm turret with its TOW box, a rear ramp.
pub fn bradley(body: Rgb) -> VehicleModel {
    let mut h = MeshBuilder::new();
    running_gear(&mut h, 3.0, 1.35, 6, 0.32, false, body);
    let prof = [Vec2::new(3.25, 0.5), Vec2::new(3.25, 2.0), Vec2::new(-1.75, 2.04), Vec2::new(-3.3, 1.3), Vec2::new(-3.3, 0.55)];
    h.extrude_x(&prof, 1.42, Vec3::ZERO, body);
    // Appliqué armour tiles along the sides, standing proud of the hull.
    for side in [-1.0f32, 1.0] {
        for row in 0..2 {
            for k in 0..6 {
                let z = -2.3 + k as f32 * 0.9;
                h.boxc(Vec3::new(side * 1.5, 1.2 + row as f32 * 0.5, z), Vec3::new(0.08, 0.44, 0.84), [body[0] * 0.95, body[1] * 0.95, body[2] * 0.93]);
            }
        }
    }
    // The rear ramp's outline and the trim vane on the nose.
    h.boxc(Vec3::new(0.0, 1.15, 3.28), Vec3::new(1.7, 1.4, 0.05), [body[0] * 0.8, body[1] * 0.8, body[2] * 0.8]);
    h.cuboid(Vec3::new(0.0, 1.45, -3.0), Vec3::new(1.3, 0.04, 0.35), Quat::from_rotation_x(-0.8), body);
    let mut t = MeshBuilder::new();
    let tprof = [Vec2::new(1.2, 0.0), Vec2::new(1.2, 0.68), Vec2::new(-0.7, 0.72), Vec2::new(-1.2, 0.25), Vec2::new(-1.2, 0.0)];
    t.extrude_x(&tprof, 0.95, Vec3::ZERO, body);
    t.cylinder(Vec3::new(0.15, 0.35, -1.2), Vec3::new(0.0, 0.0, -2.4), 0.045, 6, STEEL);
    t.cylinder(Vec3::new(0.15, 0.35, -1.2), Vec3::new(0.0, 0.0, -0.5), 0.1, 6, body);
    // The twin TOW launcher box on the turret's left side.
    t.boxc(Vec3::new(-1.15, 0.45, 0.05), Vec3::new(0.42, 0.48, 1.45), [body[0] * 0.9, body[1] * 0.9, body[2] * 0.9]);
    t.boxc(Vec3::new(0.55, 0.85, 0.1), Vec3::new(0.5, 0.25, 0.45), [0.2, 0.22, 0.18]);
    VehicleModel { hull: h, turret: t, turret_at: Vec3::new(0.25, 2.02, -0.35) }
}

/// Ka-52 "Alligator": side-by-side cockpit, stub wings with launchers, twin fins, and two coaxial rotors
/// (returned separately, so they can spin).
pub fn ka52() -> (MeshBuilder, MeshBuilder) {
    let c: Rgb = [0.27, 0.29, 0.28];
    let glass: Rgb = [0.08, 0.1, 0.12];
    let mut m = MeshBuilder::new();
    m.boxc(Vec3::new(0.0, 0.2, -1.6), Vec3::new(1.9, 1.7, 3.6), c);
    m.cuboid(Vec3::new(0.0, 0.55, -3.1), Vec3::new(0.9, 0.55, 0.9), Quat::from_rotation_x(0.6), glass);
    m.boxc(Vec3::new(0.0, 0.3, 1.2), Vec3::new(1.6, 1.6, 3.0), c);
    m.frustum(Vec3::new(0.0, 0.5, 2.6), Vec3::new(0.0, 0.0, 5.8), 0.55, 0.25, 6, c, true);
    for side in [-1.0f32, 1.0] {
        m.cuboid(Vec3::new(side * 0.9, 1.2, 8.2), Vec3::new(0.06, 0.9, 0.6), Quat::from_rotation_z(side * -0.2), c);
        // Stub wings and their pods.
        m.boxc(Vec3::new(side * 2.2, -0.1, -0.4), Vec3::new(2.8, 0.12, 1.1), c);
        for k in 0..2 {
            m.cylinder(Vec3::new(side * (1.6 + k as f32 * 1.2), -0.45, 0.4), Vec3::new(0.0, 0.0, -1.8), 0.22, 6, [0.22, 0.24, 0.22]);
        }
    }
    m.cylinder(Vec3::new(0.0, 1.0, -0.4), Vec3::new(0.0, 1.6, 0.0), 0.18, 6, c);
    let mut r = MeshBuilder::new();
    for level in [0.0f32, 0.9] {
        for k in 0..3 {
            let a = k as f32 * std::f32::consts::TAU / 3.0 + level;
            let d = Vec3::new(a.cos(), 0.0, a.sin());
            r.cuboid(Vec3::new(0.0, level, 0.0) + d * 3.6, Vec3::new(3.6, 0.03, 0.22), Quat::from_rotation_y(-a), [0.18, 0.19, 0.18]);
        }
    }
    (m, r)
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
