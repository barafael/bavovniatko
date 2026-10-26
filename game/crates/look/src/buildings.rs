//! The buildings of a map (tools/osm) raised from their footprints, for every diorama drawn from one: walls with a
//! facade each kind, flat roofs, and the helpers that measure and triangulate a footprint.

use crate::meshes::MeshBuilder;
use bav_core::geo::{self, BuildingKind, hash01};
use bevy::{
    image::{ImageAddressMode, ImageSamplerDescriptor},
    prelude::*,
};

/// How a building's walls read: which facade texture, and its scale (metres per repeat across and up).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Wall {
    /// Five-storey flats and offices: panels, a window a bay a floor.
    Block,
    /// Workshops and stores: brick or sheet, a band of high windows.
    Works,
    /// Houses: plaster, a few small windows.
    House,
}

const WALLS: [Wall; 3] = [Wall::Block, Wall::Works, Wall::House];

/// A builder for a hangar's footprint: the walls' mesh, the roofs' mesh, the footprint, its base and its height.
pub type HangarBuilder = dyn FnMut(&mut MeshBuilder, &mut MeshBuilder, &[Vec2], f32, f32);

/// How a town after months of shelling stands (Myrnohrad's): a share of its buildings gutted, their roofs gone and
/// their walls broken off at uneven heights over a floor of rubble, and its houses under pitched roofs. None:
/// every building whole, its roof flat, as the other sets have them.
#[derive(Clone, Copy, Debug)]
pub struct Shelled {
    /// The share of buildings without their roofs.
    pub ruined: f32,
    /// Drawn from this, so the same buildings are ruined every day.
    pub seed: u32,
}

impl Shelled {
    /// Whether the `i`th building is a ruin.
    pub fn ruined(&self, i: usize) -> bool {
        hash01(i as u32, self.seed) < self.ruined
    }
}

/// The map's buildings raised from their footprints: one mesh per facade, spawned here, and their roofs added to
/// `roofs` for the caller to finish (the set's own pieces go in the same mesh). A hangar goes through `hangar`.
/// `snow` whitens the roofs (February); else they are the sheet and felt they are. `shelled`: the town after the
/// shelling, its houses' roofs pitched.
#[allow(clippy::too_many_arguments)]
pub fn raise(commands: &mut Commands, meshes: &mut Assets<Mesh>, images: &mut Assets<Image>, materials: &mut Assets<StandardMaterial>, roofs: &mut MeshBuilder, snow: bool, hangar: &mut HangarBuilder, shelled: Option<Shelled>) {
    let lay = geo::layout();
    let mut walls: Vec<MeshBuilder> = WALLS.iter().map(|_| MeshBuilder::new()).collect();
    for (i, b) in lay.buildings.iter().enumerate() {
        let n = b.foot.len();
        if n < 3 {
            continue;
        }
        // Stand on the lowest ground under the footprint, the walls a metre into it on slopes.
        let base = b.foot.iter().map(|p| geo::ground(*p).y).fold(f32::MAX, f32::min) - 0.3;
        if b.kind == BuildingKind::Hangar {
            hangar(&mut walls[1], roofs, &b.foot, base, b.height);
            continue;
        }
        let wall = match b.kind {
            BuildingKind::Block => Wall::Block,
            BuildingKind::House => Wall::House,
            BuildingKind::Shed | BuildingKind::Industrial => Wall::Works,
            BuildingKind::Hangar => unreachable!(),
            // Untagged: the big ones are the airfield's workshops, the small ones houses and huts.
            BuildingKind::Other => {
                if geo_area(&b.foot) > 400.0 {
                    if b.height > 12.0 { Wall::Block } else { Wall::Works }
                } else {
                    Wall::House
                }
            }
        };
        let w = WALLS.iter().position(|x| *x == wall).unwrap();
        let top = base + 0.3 + b.height;
        // The outline counter-clockwise seen from above (+y), so each wall faces out.
        let foot: Vec<Vec2> = if signed_area(&b.foot) > 0.0 { b.foot.clone() } else { b.foot.iter().rev().copied().collect() };
        let shade = 0.88 + 0.12 * hash01(i as u32, 7);
        let mut u = hash01(i as u32, 8) * 12.0;
        let ruin = shelled.filter(|s| s.ruined(i));
        for k in 0..n {
            let (a, c) = (foot[k], foot[(k + 1) % n]);
            let len = a.distance(c);
            let (s, t) = scale(wall);
            // A ruin's walls break off at heights of their own, some down to a stub.
            let top = match ruin {
                Some(r) => {
                    let h = hash01(i as u32 * 64 + k as u32, r.seed.wrapping_add(1));
                    if h < 0.2 { base + 1.5 } else { top - (top - base) * (0.1 + 0.45 * h) }
                }
                None => top,
            };
            let uv = [Vec2::new(u / s, 0.0), Vec2::new((u + len) / s, 0.0), Vec2::new((u + len) / s, -(top - base) / t), Vec2::new(u / s, -(top - base) / t)];
            let quad = [Vec3::new(a.x, base, a.y), Vec3::new(c.x, base, c.y), Vec3::new(c.x, top, c.y), Vec3::new(a.x, top, a.y)];
            walls[w].poly_uv(&quad, &uv, [shade; 3]);
            if ruin.is_some() {
                // The inside of the wall, seen from above through the missing roof: sooted.
                walls[w].poly_uv(&[quad[1], quad[0], quad[3], quad[2]], &[uv[1], uv[0], uv[3], uv[2]], [shade * 0.45; 3]);
            }
            u += len;
        }
        if ruin.is_some() {
            // No roof: the floors fallen in, a heap of rubble and burnt timber inside the walls.
            let tris = triangulate(&foot);
            let pts: Vec<Vec3> = foot.iter().map(|p| Vec3::new(p.x, base + 1.0, p.y)).collect();
            let c = if snow { [0.55, 0.55, 0.56] } else { [0.24, 0.22, 0.21] };
            roofs.tris(&pts, &vec![Vec2::ZERO; n], &tris, Vec3::Y, c);
            continue;
        }
        if shelled.is_some() && wall == Wall::House && n == 4 {
            gable(roofs, &foot, top, snow, i);
            continue;
        }
        // Flat roofs under thin snow; the houses' pitched roofs read flat from the drone's height.
        let tris = triangulate(&foot);
        let pts: Vec<Vec3> = foot.iter().map(|p| Vec3::new(p.x, top, p.y)).collect();
        let top_colour = match (snow, wall) {
            (true, Wall::House) => [0.62, 0.58, 0.55],
            (true, _) => [0.70, 0.71, 0.72],
            // Bare roofs: the houses' slate and tin, the works' felt and sheet.
            (false, Wall::House) => [0.36, 0.30, 0.27],
            (false, _) => [0.40, 0.40, 0.39],
        };
        roofs.tris(&pts, &vec![Vec2::ZERO; n], &tris, Vec3::Y, top_colour);
    }
    for (wall, m) in WALLS.iter().zip(walls) {
        if m.is_empty() {
            continue;
        }
        let tex = images.add(facade(*wall));
        let mat = materials.add(StandardMaterial { base_color_texture: Some(tex), perceptual_roughness: 0.92, ..default() });
        commands.spawn((Mesh3d(meshes.add(m.build())), MeshMaterial3d(mat)));
    }
}

/// A house's pitched roof over a four-cornered footprint: the ridge along the longer side, a third of the short
/// side's width high, the gable ends closed in plaster. Tin, slate or tile, or snow on them.
fn gable(m: &mut MeshBuilder, foot: &[Vec2], top: f32, snow: bool, i: usize) {
    let (a, b, c, d) = if foot[0].distance(foot[1]) >= foot[1].distance(foot[2]) { (foot[0], foot[1], foot[2], foot[3]) } else { (foot[1], foot[2], foot[3], foot[0]) };
    let rise = b.distance(c) * 0.33;
    let at = |p: Vec2, y: f32| Vec3::new(p.x, y, p.y);
    let (ra, rb) = (at((a + d) * 0.5, top + rise), at((b + c) * 0.5, top + rise));
    let (a, b, c, d) = (at(a, top), at(b, top), at(c, top), at(d, top));
    let colour = if snow {
        [0.80, 0.81, 0.83]
    } else {
        [[0.38, 0.30, 0.26], [0.42, 0.44, 0.42], [0.47, 0.30, 0.24], [0.30, 0.33, 0.36]][(hash01(i as u32, 9) * 4.0) as usize % 4]
    };
    // Both ways round: the roofs' material culls back faces, and an outline may come either way.
    let mut both = |pts: &[Vec3], col: [f32; 3]| {
        m.poly(pts, col);
        m.poly(&pts.iter().rev().copied().collect::<Vec<_>>(), col);
    };
    both(&[a, b, rb, ra], colour);
    both(&[c, d, ra, rb], colour);
    both(&[b, c, rb], [0.70, 0.66, 0.58]);
    both(&[d, a, ra], [0.70, 0.66, 0.58]);
}

fn scale(w: Wall) -> (f32, f32) {
    match w {
        Wall::Block => (12.0, 8.4),
        Wall::Works => (12.0, 6.0),
        Wall::House => (8.0, 3.0),
    }
}

/// An intact facade (the morning of the first day): panels and windows, brick and a band of high windows, or
/// plaster with small windows.
fn facade(w: Wall) -> Image {
    let (sw, sh) = scale(w);
    let (wpx, hpx) = ((sw * 16.0) as usize, (sh * 16.0) as usize);
    let mut d = vec![0u8; wpx * hpx * 4];
    for y in 0..hpx {
        for x in 0..wpx {
            let (fx, fy) = (x as f32 / 16.0, y as f32 / 16.0);
            let n = geo::value_noise(fx * 0.9, fy * 0.9, 61) * 0.5 + 0.5;
            let c = match w {
                Wall::Block => {
                    let panel = 0.60 + 0.05 * n;
                    let (lx, ly) = (fx % 3.0, fy % 2.8);
                    let win = lx > 0.8 && lx < 2.2 && ly > 0.8 && ly < 2.3;
                    let lit = hash01((fx / 3.0) as u32 * 31 + (fy / 2.8) as u32 * 17, 62);
                    if win {
                        if lit < 0.3 { [0.30, 0.31, 0.32] } else { [0.14, 0.16, 0.18] }
                    } else if lx < 0.06 || ly < 0.06 {
                        [panel * 0.78; 3]
                    } else {
                        [panel, panel * 0.98, panel * 0.94]
                    }
                }
                Wall::Works => {
                    let brick = [0.52 + 0.06 * n, 0.40 + 0.04 * n, 0.33 + 0.03 * n];
                    let band = fy > 3.4 && fy < 4.9 && (fx % 4.0) > 0.4;
                    if band { [0.20, 0.22, 0.24] } else if (fy * 4.0) % 1.0 < 0.08 { [brick[0] * 0.8, brick[1] * 0.8, brick[2] * 0.8] } else { brick }
                }
                Wall::House => {
                    let plaster = [0.74 + 0.05 * n, 0.70 + 0.05 * n, 0.62 + 0.04 * n];
                    let (lx, ly) = (fx % 4.0, fy % 3.0);
                    if lx > 1.4 && lx < 2.6 && ly > 0.9 && ly < 2.2 { [0.16, 0.17, 0.18] } else { plaster }
                }
            };
            let o = (y * wpx + x) * 4;
            d[o] = (c[0] * 255.0) as u8;
            d[o + 1] = (c[1] * 255.0) as u8;
            d[o + 2] = (c[2] * 255.0) as u8;
            d[o + 3] = 255;
        }
    }
    crate::meshes::rgba8_image(wpx, hpx, d, ImageSamplerDescriptor { address_mode_u: ImageAddressMode::Repeat, address_mode_v: ImageAddressMode::Repeat, ..ImageSamplerDescriptor::linear() })
}

fn signed_area(p: &[Vec2]) -> f32 {
    let mut s = 0.0;
    for k in 0..p.len() {
        let (a, b) = (p[k], p[(k + 1) % p.len()]);
        s += a.x * b.y - b.x * a.y;
    }
    // +z is south: counter-clockwise seen from above is clockwise in (x, z).
    -s * 0.5
}

fn geo_area(p: &[Vec2]) -> f32 {
    signed_area(p).abs()
}

/// Ear clipping of a simple outline, counter-clockwise from above.
fn triangulate(p: &[Vec2]) -> Vec<[usize; 3]> {
    let mut idx: Vec<usize> = (0..p.len()).collect();
    let mut out = Vec::new();
    // Counter-clockwise seen from above (+y) with +z south: the turn a -> b -> c is convex when this is negative.
    let convex = |a: Vec2, b: Vec2, c: Vec2| (b - a).perp_dot(c - b) < 0.0;
    let mut guard = 0;
    while idx.len() > 3 && guard < 10_000 {
        guard += 1;
        let m = idx.len();
        let mut cut = false;
        for k in 0..m {
            let (ia, ib, ic) = (idx[(k + m - 1) % m], idx[k], idx[(k + 1) % m]);
            let (a, b, c) = (p[ia], p[ib], p[ic]);
            if !convex(a, b, c) {
                continue;
            }
            let ear = idx.iter().all(|&j| j == ia || j == ib || j == ic || !in_triangle(p[j], a, b, c));
            if ear {
                out.push([ia, ic, ib]);
                idx.remove(k);
                cut = true;
                break;
            }
        }
        if !cut {
            break;
        }
    }
    if idx.len() == 3 {
        out.push([idx[0], idx[2], idx[1]]);
    } else {
        // Not simple (the map's outlines sometimes cross themselves): a fan, as before.
        for k in 1..idx.len() - 1 {
            out.push([idx[0], idx[k + 1], idx[k]]);
        }
    }
    // Every triangle facing up (+y): with +z south, that is a negative turn in (x, z).
    for t in &mut out {
        if (p[t[1]] - p[t[0]]).perp_dot(p[t[2]] - p[t[0]]) > 0.0 {
            t.swap(1, 2);
        }
    }
    out
}

fn in_triangle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let d1 = (b - a).perp_dot(p - a);
    let d2 = (c - b).perp_dot(p - b);
    let d3 = (a - c).perp_dot(p - c);
    let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(neg && pos)
}
