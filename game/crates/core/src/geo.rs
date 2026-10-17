//! The diorama's ground truth: terrain height, roads, shelterbelts, minefields and places.
//!
//! World units are metres. +X is east, +Z is south (so the town on its ridge lies at negative Z and the
//! Kashlahach valley at positive Z). The layout compresses the dossier geography (db/research/chapters/
//! c08-vuhledar.md §3) into a 4.6 × 3.6 km box: Vuhledar's southern blocks on the ridge in the north, open
//! fields cut by shelterbelts falling 40 m to the river, Pavlivka to the south-west and Mykilske to the
//! south-east, and the dirt roads the columns drove up.

use bevy::math::{Vec2, Vec3};
use std::sync::OnceLock;

pub const HALF_X: f32 = 2304.0;
pub const HALF_Z: f32 = 1792.0;
pub const CHUNK: f32 = 256.0;
pub const CHUNKS_X: usize = (2.0 * HALF_X / CHUNK) as usize;
pub const CHUNKS_Z: usize = (2.0 * HALF_Z / CHUNK) as usize;
/// Ground texels per 256 m chunk: half-metre painting. One metre blurred past the film's close shots (fov 7–9
/// from 100–200 m), and the painting cost is amortised over every run that reads a saved ground.
pub const TEX_PER_CHUNK: usize = 512;

// ---------------------------------------------------------------------------------------------------------
// Cheap deterministic value noise (no allocation, usable from any thread).

fn hash2(x: i32, z: i32, seed: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x8da6_b343) ^ (z as u32).wrapping_mul(0xd816_3841) ^ seed.wrapping_mul(0xcb1a_b31f);
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1_e995);
    h ^= h >> 15;
    (h & 0x00ff_ffff) as f32 / 16_777_215.0
}

pub fn value_noise(x: f32, z: f32, seed: u32) -> f32 {
    let (xi, zi) = (x.floor() as i32, z.floor() as i32);
    let (fx, fz) = (x - xi as f32, z - zi as f32);
    let (ux, uz) = (fx * fx * (3.0 - 2.0 * fx), fz * fz * (3.0 - 2.0 * fz));
    let a = hash2(xi, zi, seed);
    let b = hash2(xi.wrapping_add(1), zi, seed);
    let c = hash2(xi, zi.wrapping_add(1), seed);
    let d = hash2(xi.wrapping_add(1), zi.wrapping_add(1), seed);
    let ab = a + (b - a) * ux;
    let cd = c + (d - c) * ux;
    (ab + (cd - ab) * uz) * 2.0 - 1.0
}

pub fn fbm(x: f32, z: f32, octaves: u32, seed: u32) -> f32 {
    let (mut sum, mut amp, mut freq, mut norm) = (0.0, 1.0, 1.0, 0.0);
    for o in 0..octaves {
        sum += value_noise(x * freq, z * freq, seed + o * 17) * amp;
        norm += amp;
        amp *= 0.5;
        freq *= 2.03;
    }
    sum / norm
}

pub fn hash01(i: u32, seed: u32) -> f32 {
    hash2(i as i32, (i >> 16) as i32 ^ 0x5f3, seed)
}

pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

// ---------------------------------------------------------------------------------------------------------
// Polylines

#[derive(Clone, Debug)]
pub struct Polyline {
    pub pts: Vec<Vec2>,
    /// Cumulative arc length at each point.
    pub cum: Vec<f32>,
}

impl Polyline {
    pub fn new(pts: Vec<Vec2>) -> Self {
        let mut cum = vec![0.0];
        for w in pts.windows(2) {
            cum.push(cum.last().unwrap() + w[0].distance(w[1]));
        }
        Self { pts, cum }
    }

    /// A smooth version: Chaikin corner cutting, then resampled so vehicles turn gently.
    pub fn smoothed(pts: Vec<Vec2>, iterations: usize) -> Self {
        let mut p = pts;
        for _ in 0..iterations {
            let mut q = vec![p[0]];
            for w in p.windows(2) {
                q.push(w[0].lerp(w[1], 0.25));
                q.push(w[0].lerp(w[1], 0.75));
            }
            q.push(*p.last().unwrap());
            p = q;
        }
        Self::new(p)
    }

    pub fn length(&self) -> f32 {
        *self.cum.last().unwrap()
    }

    fn segment_at(&self, s: f32) -> usize {
        let s = s.clamp(0.0, self.length());
        match self.cum.binary_search_by(|c| c.partial_cmp(&s).unwrap()) {
            Ok(i) => i.min(self.pts.len() - 2),
            Err(i) => (i.max(1) - 1).min(self.pts.len() - 2),
        }
    }

    pub fn point(&self, s: f32) -> Vec2 {
        let i = self.segment_at(s);
        let seg = self.cum[i + 1] - self.cum[i];
        let t = if seg > 0.0 { ((s - self.cum[i]) / seg).clamp(0.0, 1.0) } else { 0.0 };
        self.pts[i].lerp(self.pts[i + 1], t)
    }

    /// Whether the segment `a`–`b` crosses the line.
    pub fn crosses(&self, a: Vec2, b: Vec2) -> bool {
        self.pts.windows(2).any(|w| crossing(a, b, w[0], w[1]).is_some())
    }

    pub fn tangent(&self, s: f32) -> Vec2 {
        let i = self.segment_at(s);
        (self.pts[i + 1] - self.pts[i]).normalize_or_zero()
    }

    /// Right-hand normal (in the XZ plane, looking along the tangent).
    pub fn normal(&self, s: f32) -> Vec2 {
        let t = self.tangent(s);
        Vec2::new(-t.y, t.x)
    }

    /// Signed lateral offset of `p` from the line (positive to the right), and the arc length of the closest point.
    pub fn lateral(&self, p: Vec2) -> (f32, f32) {
        let (d, s) = self.project(p);
        let side = (p - self.point(s)).dot(self.normal(s)).signum();
        (d * side, s)
    }

    /// `lateral`, searching only the stretch within `window` of the arc length `near`: a vehicle that knows
    /// roughly where it is on its route, so a bend that doubles back cannot claim it.
    pub fn lateral_near(&self, p: Vec2, near: f32, window: f32) -> (f32, f32) {
        let (d, s) = self.project_in(p, self.segment_at(near - window), self.segment_at(near + window));
        let side = (p - self.point(s)).dot(self.normal(s)).signum();
        (d * side, s)
    }

    /// The point at arc length `s`, carried on straight past either end.
    pub fn point_beyond(&self, s: f32) -> Vec2 {
        let len = self.length();
        if s < 0.0 {
            self.point(0.0) + self.tangent(0.0) * s
        } else if s > len {
            self.point(len) + self.tangent(len) * (s - len)
        } else {
            self.point(s)
        }
    }

    /// Distance from `p` to the polyline, and the arc length of the closest point.
    pub fn project(&self, p: Vec2) -> (f32, f32) {
        self.project_in(p, 0, self.pts.len() - 2)
    }

    fn project_in(&self, p: Vec2, first: usize, last: usize) -> (f32, f32) {
        let mut best = (f32::MAX, 0.0);
        for i in first..=last {
            let (a, b) = (self.pts[i], self.pts[i + 1]);
            let ab = b - a;
            let len2 = ab.length_squared();
            let t = if len2 > 0.0 { ((p - a).dot(ab) / len2).clamp(0.0, 1.0) } else { 0.0 };
            let d = p.distance(a + ab * t);
            if d < best.0 {
                best = (d, self.cum[i] + t * len2.sqrt());
            }
        }
        best
    }
}

// ---------------------------------------------------------------------------------------------------------
// Layout

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub enum RoadKind {
    Asphalt,
    Dirt,
    Track,
}

#[derive(Clone, Debug)]
pub struct Road {
    pub line: Polyline,
    pub width: f32,
    pub kind: RoadKind,
}

#[derive(Clone, Debug)]
pub struct Belt {
    pub line: Polyline,
    pub width: f32,
    /// Arc-length intervals with no trees (road crossings, old gaps).
    pub gaps: Vec<(f32, f32)>,
}

impl Belt {
    /// Whether the arc length `s` falls in a gap, the gap narrowed by `margin` at each end (widened, if negative).
    pub fn in_gap(&self, s: f32, margin: f32) -> bool {
        self.gaps.iter().any(|&(g0, g1)| s > g0 + margin && s < g1 - margin)
    }

    /// Where the segment `a`–`b` crosses the belt's line: the fraction along `a`–`b`, and whether it crosses in a gap.
    pub fn crossings(&self, a: Vec2, b: Vec2) -> impl Iterator<Item = (f32, bool)> + '_ {
        let pts = &self.line.pts;
        (0..pts.len() - 1).filter_map(move |i| {
            let t = crossing(a, b, pts[i], pts[i + 1])?;
            let s = self.line.cum[i] + pts[i].distance(pts[i + 1]) * ((a + (b - a) * t) - pts[i]).length() / pts[i].distance(pts[i + 1]).max(1e-3);
            Some((t, self.in_gap(s, 0.0)))
        })
    }
}

pub use bevy::math::Rect;

/// An approach route: the polyline the columns drive, where they deploy, and where their troops go.
#[derive(Clone, Debug)]
pub struct Route {
    pub line: Polyline,
    /// Arc length where columns leave single file and fan out to dismount.
    pub deploy_s: f32,
    /// What the dismounts attack (on a holding route, the position they take).
    pub objective: Vec2,
    /// A column's way to a position (design/command.md): it fans out at the end, puts its men into the position
    /// and holds there, whichever side it is. The layout's routes are attack routes.
    pub hold: bool,
}

/// A minefield strip along a road's shoulders, the 72nd's design: the road stays clear, the fields beside it
/// are seeded so a column cannot turn round (NYT, Bayak).
#[derive(Clone, Debug)]
pub struct ShoulderField {
    pub road: usize,
    pub s0: f32,
    pub s1: f32,
    pub inner: f32,
    pub outer: f32,
    pub density: f32,
}

/// A barrier minefield across open ground, the Surovikin line's kind: hundreds of metres deep, anti-tank mines
/// often stacked two high, anti-personnel mines among them, and no lanes left open.
#[derive(Clone, Debug)]
pub struct Barrier {
    pub z0: f32,
    pub z1: f32,
    /// Anti-tank mines per square metre.
    pub at: f32,
    /// Share of anti-tank mines stacked two high (a plow or roller does not survive one).
    pub stacked: f32,
    /// Anti-personnel mines per square metre.
    pub ap: f32,
}

pub struct Layout {
    pub roads: Vec<Road>,
    pub belts: Vec<Belt>,
    pub routes: Vec<Route>,
    pub shoulder_fields: Vec<ShoulderField>,
    /// Belts of mines in open field (in front of the town edge).
    pub field_mines: Vec<(Polyline, f32)>,
    /// Barrier minefields across the whole width of the box.
    pub barriers: Vec<Barrier>,
    pub river: Option<Polyline>,
    pub town: Rect,
    pub dachas: Rect,
    pub pavlivka: Rect,
    pub mykilske: Rect,
    /// Ukrainian positions in the tree lines: ATGM teams and rifle positions.
    pub ua_positions: Vec<Vec2>,
    /// Russian positions: trenches and ATGM teams.
    pub ru_positions: Vec<Vec2>,
    /// Pre-registered artillery target points on the approach roads.
    pub trps: Vec<Vec2>,
    /// Where earlier fighting left wrecks.
    pub old_wrecks: Vec<(Vec2, f32)>,
    /// Which way the attack goes along z: -1 north (Vuhledar), +1 south (Robotyne).
    pub advance: f32,
    /// The defenders' forward line (z): attackers alive at or past it hold a foothold.
    pub foothold_z: f32,
    /// The defenders' main line (z), the tree line or trench the attackers' guns work over.
    pub defence_z: f32,
    /// The defenders' trench, where one is painted across the field.
    pub trench_z: Option<f32>,
    /// The attackers' start line, where they leave from a tree line of their own.
    pub start_z: Option<f32>,
    /// The drone's base, and the z beyond which it flies into the enemy's jamming.
    pub home: Vec2,
    pub jam_z: f32,
    /// Where the defenders' mortars reach (a z band in front of their line).
    pub mortar_band: (f32, f32),
}

static LAYOUT: OnceLock<Layout> = OnceLock::new();

/// The diorama's layout, built from its file (`crate::data`).
pub fn layout() -> &'static Layout {
    LAYOUT.get_or_init(|| crate::data::layout(crate::data::data()))
}

pub fn v(x: f32, z: f32) -> Vec2 {
    Vec2::new(x, z)
}

/// Cut gaps where roads cross belts, plus a few natural gaps.
pub fn cut_gaps(belts: &mut [Belt], roads: &[Road]) {
    for (bi, belt) in belts.iter_mut().enumerate() {
        let n = (belt.line.length() / 8.0) as usize;
        let mut in_gap: Option<f32> = None;
        for k in 0..=n {
            let s = k as f32 * 8.0;
            let p = belt.line.point(s);
            // Vehicles turning through a belt have worn the gap wider than the road; a farm track crosses where
            // the belt was left open for it.
            let near_road = roads.iter().any(|r| r.line.project(p).0 < r.width * 0.5 + if r.kind == RoadKind::Track { 18.0 } else { 12.0 });
            let natural = value_noise(s / 90.0, bi as f32 * 7.0, 99) > 0.62;
            let gap = near_road || natural;
            match (gap, in_gap) {
                (true, None) => in_gap = Some(s),
                (false, Some(g0)) => {
                    belt.gaps.push((g0, s));
                    in_gap = None;
                }
                _ => {}
            }
        }
        if let Some(g0) = in_gap {
            belt.gaps.push((g0, belt.line.length()));
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// Terrain height

/// Ground height in metres above sea level: the diorama's height terms (`crate::data`).
pub fn height(x: f32, z: f32) -> f32 {
    crate::data::height(x, z)
}

pub fn normal(x: f32, z: f32) -> Vec3 {
    let e = 1.5;
    let dx = height(x + e, z) - height(x - e, z);
    let dz = height(x, z + e) - height(x, z - e);
    Vec3::new(-dx, 2.0 * e, -dz).normalize()
}

pub fn ground(p: Vec2) -> Vec3 {
    Vec3::new(p.x, height(p.x, p.y), p.y)
}

pub fn in_bounds(p: Vec2) -> bool {
    p.x.abs() < HALF_X && p.y.abs() < HALF_Z
}

/// March a ray against the terrain. Returns the hit point.
pub fn raycast(origin: Vec3, dir: Vec3, max: f32) -> Option<Vec3> {
    let mut t = 0.0;
    let mut prev = 0.0;
    let step: f32 = 4.0;
    while t < max {
        let p = origin + dir * t;
        if p.y < height(p.x, p.z) {
            // Refine by bisection.
            let (mut lo, mut hi) = (prev, t);
            for _ in 0..20 {
                let mid = 0.5 * (lo + hi);
                let q = origin + dir * mid;
                if q.y < height(q.x, q.z) { hi = mid } else { lo = mid }
            }
            return Some(origin + dir * hi);
        }
        prev = t;
        t += step.max(t * 0.004);
    }
    None
}

/// Where the segments `a`–`b` and `c`–`d` cross, as the fraction along `a`–`b`.
fn crossing(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Option<f32> {
    let (r, s) = (b - a, d - c);
    let denom = r.perp_dot(s);
    if denom.abs() < 1e-6 {
        return None;
    }
    let t = (c - a).perp_dot(s) / denom;
    let u = (c - a).perp_dot(r) / denom;
    ((0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)).then_some(t)
}

/// Can a man at `a`, eye at a parapet 1.5 m up, see a hull at `b` (its turret roof, 2 m up)? A shelterbelt
/// between them hides it (outside its gaps), unless he stands at that belt's near edge and looks out of it, or
/// the hull stands at its edge in front of it; and the ground hides it where a crest rises between.
pub fn line_of_sight(a: Vec2, b: Vec2) -> bool {
    let len = a.distance(b);
    for belt in &layout().belts {
        for (t, gap) in belt.crossings(a, b) {
            // Standing in the belt (within 20 m of where the line meets it): looking out, not through. And a
            // hull at a belt's near edge is seen against the trees, not through them.
            if t * len < 20.0 || (1.0 - t) * len < 20.0 || gap {
                continue;
            }
            return false;
        }
    }
    let (ha, hb) = (height(a.x, a.y) + 1.5, height(b.x, b.y) + 2.0);
    let n = (len / 25.0).ceil().max(1.0) as usize;
    for i in 1..n {
        let f = i as f32 / n as f32;
        let p = a.lerp(b, f);
        if height(p.x, p.y) > ha + (hb - ha) * f + 0.3 {
            return false;
        }
    }
    true
}

/// Map position to WGS84. Vuhledar is anchored on the town node (47.7810 N, 37.2461 E) at (-900, -1500);
/// Robotyne's box on the fields about 4 km south of Mala Tokmachka (47.4967 N, 35.8968 E at the origin).
pub fn latlon(p: Vec2) -> (f64, f64) {
    let a = crate::data::data().anchor;
    let (lat0, lon0, x0, z0, m_per_lon) = (a.lat, a.lon, a.x, a.z, a.m_per_lon);
    let lat = lat0 - (p.y as f64 - z0) / 111_200.0;
    let lon = lon0 + (p.x as f64 - x0) / m_per_lon;
    (lat, lon)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// How much of each route each ATGM position can see (`BAV_DIORAMA` picks the layout).
    #[test]
    fn sight_down_the_lanes() {
        let lay = layout();
        for (side, posts) in [("UA", &lay.ua_positions), ("RU", &lay.ru_positions)] {
            for p in posts {
                let mut line = format!("{side} post ({:>6.0},{:>5.0}):", p.x, p.y);
                for r in &lay.routes {
                    let n = 40;
                    let seen = (0..n).filter(|&k| line_of_sight(*p, r.line.point(r.line.length() * k as f32 / n as f32))).count();
                    line += &format!("  route {:>2}/{n}", seen);
                }
                println!("{line}");
            }
        }
    }
}
