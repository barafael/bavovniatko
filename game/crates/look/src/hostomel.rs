//! Antonov airport at Hostomel, 24 February 2022, as it looks (design/hostomel.md, phase 1): the buildings raised
//! from their footprints on the map (tools/osm), the Antonov hangar under its barrel roof, and the light of a late
//! February day at 50.6 N, from an overcast late morning to the dark after the counterattack.

use crate::meshes::MeshBuilder;
use crate::world::Skirt;
use bav_core::geo::{self, BuildingKind, hash01, smoothstep};
use bav_sim::player::MainCamera;
use bevy::{
    camera::Exposure,
    image::{ImageAddressMode, ImageSamplerDescriptor},
    pbr::FogFalloff,
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

/// The buildings, one mesh per facade, and the roofs in one more.
pub fn spawn_set(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let lay = geo::layout();
    let mut walls: Vec<MeshBuilder> = WALLS.iter().map(|_| MeshBuilder::new()).collect();
    let mut roofs = MeshBuilder::new();
    for (i, b) in lay.buildings.iter().enumerate() {
        let n = b.foot.len();
        if n < 3 {
            continue;
        }
        // Stand on the lowest ground under the footprint, the walls a metre into it on slopes.
        let base = b.foot.iter().map(|p| geo::ground(*p).y).fold(f32::MAX, f32::min) - 0.3;
        if b.kind == BuildingKind::Hangar {
            hangar(&mut walls[1], &mut roofs, &b.foot, base, b.height);
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
        for k in 0..n {
            let (a, c) = (foot[k], foot[(k + 1) % n]);
            let len = a.distance(c);
            let (s, t) = scale(wall);
            let uv = [Vec2::new(u / s, 0.0), Vec2::new((u + len) / s, 0.0), Vec2::new((u + len) / s, -(top - base) / t), Vec2::new(u / s, -(top - base) / t)];
            walls[w].poly_uv(&[Vec3::new(a.x, base, a.y), Vec3::new(c.x, base, c.y), Vec3::new(c.x, top, c.y), Vec3::new(a.x, top, a.y)], &uv, [shade; 3]);
            u += len;
        }
        // Flat roofs under thin snow; the houses' pitched roofs read flat from the drone's height.
        let tris = triangulate(&foot);
        let pts: Vec<Vec3> = foot.iter().map(|p| Vec3::new(p.x, top, p.y)).collect();
        let snow = if wall == Wall::House { [0.62, 0.58, 0.55] } else { [0.70, 0.71, 0.72] };
        roofs.tris(&pts, &vec![Vec2::ZERO; n], &tris, Vec3::Y, snow);
    }
    for (wall, m) in WALLS.iter().zip(walls) {
        if m.is_empty() {
            continue;
        }
        let tex = images.add(facade(*wall));
        let mat = materials.add(StandardMaterial { base_color_texture: Some(tex), perceptual_roughness: 0.92, ..default() });
        commands.spawn((Mesh3d(meshes.add(m.build())), MeshMaterial3d(mat)));
    }
    // The control tower by the apron, where the observers are: a concrete shaft and the glazed cab on it. The map
    // does not have it; where it stands is placed by eye from the apron's layout (design/hostomel.md).
    let g = geo::ground(TOWER);
    roofs.cylinder(g - Vec3::Y, Vec3::Y * (TOWER_H + 1.0), 2.6, 10, [0.70, 0.70, 0.68]);
    roofs.boxc(g + Vec3::Y * (TOWER_H + 1.6), Vec3::new(7.5, 3.2, 7.5), [0.18, 0.22, 0.26]);
    roofs.boxc(g + Vec3::Y * (TOWER_H + 3.4), Vec3::new(8.4, 0.4, 8.4), [0.72, 0.73, 0.74]);
    // The observers: in the tower, or where the part's file puts them (at dusk the tower is the enemy's).
    let op = match bav_core::data::data().op {
        Some(((x, z), h)) => {
            let at = Vec2::new(x, z);
            bav_sim::player::ObservationPost { eye: geo::ground(at) + Vec3::Y * h, building: at }
        }
        None => bav_sim::player::ObservationPost { eye: g + Vec3::new(0.0, TOWER_H + 1.8, 4.2), building: TOWER },
    };
    commands.insert_resource(op);
    let solid = materials.add(StandardMaterial { perceptual_roughness: 0.95, ..default() });
    commands.spawn((Mesh3d(meshes.add(roofs.build())), MeshMaterial3d(solid)));
}

/// The control tower: where, and the height of its cab's floor.
const TOWER: Vec2 = Vec2::new(840.0, 380.0);
const TOWER_H: f32 = 24.0;

/// The Antonov hangar: a barrel vault over the footprint's long axis, its gable ends closed by walls of sheet.
fn hangar(walls: &mut MeshBuilder, roofs: &mut MeshBuilder, foot: &[Vec2], base: f32, height: f32) {
    // The footprint's box along its longest edge.
    let (mut best, mut axis) = (0.0, Vec2::X);
    for k in 0..foot.len() {
        let d = foot[(k + 1) % foot.len()] - foot[k];
        if d.length() > best {
            best = d.length();
            axis = d.normalize();
        }
    }
    let across = axis.perp();
    let (mut lo, mut hi) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
    for p in foot {
        let q = Vec2::new(p.dot(axis), p.dot(across));
        lo = lo.min(q);
        hi = hi.max(q);
    }
    let at = |a: f32, b: f32, y: f32| {
        let p = axis * a + across * b;
        Vec3::new(p.x, base + y, p.y)
    };
    // The vault spans the short side; its springing line is a third of the height up.
    let (span_lo, span_hi) = (lo.y, hi.y);
    let spring = height * 0.35;
    let segs = 16;
    let arc = |k: usize| {
        let th = std::f32::consts::PI * k as f32 / segs as f32;
        let b = span_lo + (span_hi - span_lo) * (1.0 - th.cos()) * 0.5;
        (b, spring + (height - spring) * th.sin())
    };
    for k in 0..segs {
        let (b0, y0) = arc(k);
        let (b1, y1) = arc(k + 1);
        roofs.poly(&[at(lo.x, b0, y0), at(lo.x, b1, y1), at(hi.x, b1, y1), at(hi.x, b0, y0)], [0.58, 0.60, 0.62]);
    }
    // The long walls up to the springing line, and the gables: the doors are the whole of one end.
    let uv = |w: f32, h: f32| [Vec2::ZERO, Vec2::new(w / 12.0, 0.0), Vec2::new(w / 12.0, -h / 8.4), Vec2::new(0.0, -h / 8.4)];
    let len = hi.x - lo.x;
    walls.poly_uv(&[at(hi.x, span_lo, 0.0), at(lo.x, span_lo, 0.0), at(lo.x, span_lo, spring), at(hi.x, span_lo, spring)], &uv(len, spring), [0.9; 3]);
    walls.poly_uv(&[at(lo.x, span_hi, 0.0), at(hi.x, span_hi, 0.0), at(hi.x, span_hi, spring), at(lo.x, span_hi, spring)], &uv(len, spring), [0.9; 3]);
    for (x, flip) in [(lo.x, false), (hi.x, true)] {
        let mut pts = vec![at(x, span_lo, 0.0), at(x, span_hi, 0.0)];
        pts.extend((0..=segs).rev().map(|k| {
            let (b, y) = arc(k);
            at(x, b, y)
        }));
        if flip {
            pts.reverse();
        }
        roofs.poly(&pts, [0.46, 0.47, 0.48]);
    }
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

/// The light under a low deck of cloud, from the late morning to full dark: the sun a brightness behind the cloud
/// and never a hard shadow, the ambient carrying the day; after sunset the blue hour, then the night, which the
/// camera opens up to (a drone's sensor), and the fires and flares carry it from then on (`FxLight`).
#[allow(clippy::too_many_arguments)]
pub fn daylight(
    script: Res<bav_sim::director::Script>,
    mut sun: Query<(&mut DirectionalLight, &mut Transform)>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut clear: ResMut<ClearColor>,
    mut cam: Query<(&mut DistanceFog, &mut Exposure), With<MainCamera>>,
    skirt: Option<Res<Skirt>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fx_light: ResMut<crate::fx::FxLight>,
    mut sky_dome: crate::sky::Sky,
    mut last: Local<Option<Color>>,
    look: Res<crate::LookSettings>,
) {
    let dbg = crate::dev::dev().daydbg;
    if dbg == Some(crate::dev::DayDebug::Off) {
        return;
    }
    let minutes = bav_sim::director::clock_minutes(script.t);
    let (elev, azim) = bav_core::sun::position(minutes);
    let s = |e0: f32, e1: f32, x: f32| smoothstep(e0, e1, x);
    let twilight = s(-12.0, 0.0, elev);
    let day = s(0.0, 25.0, elev);
    let mix3 = |a: [f32; 3], b: [f32; 3], t: f32| [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
    let [k_sun, k_amb, ev_off] = crate::dev::dev().light;
    // The light's direction in the sun's steps, so shadows of things standing still hold still.
    let (e_step, a_step) = bav_core::sun::position(crate::world::sun_hours(minutes) * 60.0);
    let e = e_step.max(8.0).to_radians();
    let a = a_step.to_radians();
    for (mut l, mut tf) in &mut sun {
        // Through 93% cloud the sun is a weak, diffuse key: enough to model the shapes, too soft to cut shadows.
        l.illuminance = (3200.0 * day + 400.0 * twilight) * k_sun;
        let dir = Vec3::new(a.sin() * e.cos(), e.sin(), -a.cos() * e.cos());
        let to = Transform::from_translation(dir).looking_at(Vec3::ZERO, Vec3::Y);
        if dbg != Some(crate::dev::DayDebug::NoSun) && tf.rotation != to.rotation {
            *tf = to;
        }
        let c = mix3([0.55, 0.62, 0.85], [0.96, 0.95, 0.94], day);
        l.color = Color::srgb(c[0], c[1], c[2]);
    }
    // The cloud is the light: a grey dome that dims through the blue hour into night.
    // (A little ambient is left at night: the glow of Kyiv and Irpin on the cloud's underside.)
    ambient.brightness = (140.0 + 620.0 * twilight + 900.0 * day) * k_amb;
    let a_col = mix3([0.30, 0.36, 0.60], [0.80, 0.84, 0.92], twilight);
    ambient.color = Color::srgb(a_col[0], a_col[1], a_col[2]);
    let mut sky = mix3([0.03, 0.04, 0.07], [0.30, 0.33, 0.42], twilight);
    sky = mix3(sky, [0.66, 0.68, 0.71], day);
    let sky_c = Color::srgb(sky[0], sky[1], sky[2]);
    clear.0 = sky_c;
    fx_light.0 = 0.3 + 0.7 * s(-6.0, 10.0, elev);
    let sun_at = {
        let (e, a) = (elev.to_radians(), azim.to_radians());
        Vec3::new(a.sin() * e.cos(), e.sin(), -a.cos() * e.cos())
    };
    let z = mix3([0.02, 0.03, 0.05], [0.24, 0.27, 0.36], twilight);
    let z = mix3(z, [0.58, 0.60, 0.64], day);
    sky_dome.set(crate::sky::Weather::Overcast, minutes / 60.0, Color::srgb(z[0], z[1], z[2]), sky_c, sun_at, Color::srgb(1.0, 0.9, 0.8), 0.6 * day, 0.15 + 0.85 * twilight);
    for (mut f, mut ex) in &mut cam {
        f.color = sky_c;
        // Drizzle under a low deck: the far edge of the field greys out.
        f.falloff = FogFalloff::from_visibility_colors(4500.0 * look.haze, Color::srgb(0.46, 0.48, 0.52), Color::srgb(0.66, 0.68, 0.71));
        f.directional_light_color = Color::NONE;
        // The sensor opens up as the light goes, but the night still reads as night.
        // A drone's low-light mode opens up further: grainy, but the field is there.
        ex.ev100 = 6.3 + 4.5 * s(-14.0, 12.0, elev) + ev_off - look.exposure;
    }
    let changed = last.is_none_or(|l| (l.to_srgba().red - sky_c.to_srgba().red).abs() > 0.004);
    if changed && let Some(sk) = skirt.as_ref() && let Some(mut m) = materials.get_mut(&sk.0) {
        m.base_color = sky_c;
        *last = Some(sky_c);
    }
}
