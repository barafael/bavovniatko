//! Antonov airport at Hostomel, 24 February 2022, as it looks (design/hostomel.md, phase 1): the buildings raised
//! from their footprints on the map (tools/osm), the Antonov hangar under its barrel roof, and the light of a late
//! February day at 50.6 N, from an overcast late morning to the dark after the counterattack.

use crate::meshes::MeshBuilder;
use crate::world::Daylight;
use bav_core::geo::{self, BuildingKind, smoothstep};
use bav_sim::player::MainCamera;
use bevy::{camera::Exposure, pbr::FogFalloff, prelude::*};

/// The set: the map's buildings, the Antonov hangar under its barrel roof, the Mriya in it, the control tower, and
/// the observers' post.
pub fn spawn_set(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<StandardMaterial>>, mut paint: ResMut<Assets<crate::baked::VehicleMaterial>>) {
    let lay = geo::layout();
    let mut roofs = MeshBuilder::new();
    crate::buildings::raise(&mut commands, &mut meshes, &mut images, &mut materials, &mut roofs, crate::buildings::Town { snow: true, hangar: &mut hangar, shelled: None });
    // The Mriya in the Antonov hangar, as she stood that morning, ready to fly since 22:00 the night before
    // (c01 §6 set piece 4): along the hangar's long axis, under the vault.
    if let Some(h) = lay.buildings.iter().find(|b| b.kind == BuildingKind::Hangar) {
        let mid = h.foot.iter().copied().sum::<Vec2>() / h.foot.len() as f32;
        let axis = (0..h.foot.len()).map(|k| h.foot[(k + 1) % h.foot.len()] - h.foot[k]).max_by(|a, b| a.length().total_cmp(&b.length())).unwrap_or(Vec2::X);
        let m = crate::baked::load("an225", crate::models::livery("an225"), &mut meshes, &mut images, &mut paint);
        let mut e = commands.spawn((bav_sim::units::skin(&m.hull), Transform::from_translation(geo::ground(mid)).with_rotation(Quat::from_rotation_y(bav_sim::units::vehicle_yaw(axis)))));
        bav_sim::units::dress(&mut e, &m.hull);
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
pub(crate) fn hangar(walls: &mut MeshBuilder, roofs: &mut MeshBuilder, foot: &[Vec2], base: f32, height: f32) {
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
    // The doors are the whole of one end, open: the end towards the apron.
    let apron = Vec2::new(700.0, 500.0);
    let open_lo = (axis * lo.x + across * (lo.y + hi.y) * 0.5).distance(apron) < (axis * hi.x + across * (lo.y + hi.y) * 0.5).distance(apron);
    for (x, flip) in [(lo.x, false), (hi.x, true)] {
        if (x == lo.x) == open_lo {
            continue;
        }
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

/// The light under a low deck of cloud, from the late morning to full dark: the sun a brightness behind the cloud
/// and never a hard shadow, the ambient carrying the day; after sunset the blue hour, then the night, which the
/// camera opens up to (a drone's sensor), and the fires and flares carry it from then on (`FxLight`).
pub fn daylight(
    day: Daylight,
    mut cam: Query<(&mut DistanceFog, &mut Exposure), With<MainCamera>>,
    mut fx_light: ResMut<crate::fx::FxLight>,
) {
    let Daylight { script, mut sun, mut ambient, mut clear, skirt, mut materials, mut sky_dome, mut last, look } = day;
    let dbg = crate::dev::dev().daydbg;
    if dbg == Some(crate::dev::DayDebug::Off) {
        return;
    }
    let minutes = bav_sim::director::clock_minutes(script.t);
    let (elev, azim) = bav_core::sun::position(minutes);
    let azim = bav_core::geo::map_heading(azim);
    let s = |e0: f32, e1: f32, x: f32| smoothstep(e0, e1, x);
    let twilight = s(-12.0, 0.0, elev);
    let day = s(0.0, 25.0, elev);
    let mix3 = |a: [f32; 3], b: [f32; 3], t: f32| [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
    let [k_sun, k_amb, ev_off] = crate::dev::dev().light;
    // The light's direction in the sun's steps, so shadows of things standing still hold still.
    let (e_step, a_step) = bav_core::sun::position(crate::world::sun_hours(minutes) * 60.0);
    let a_step = bav_core::geo::map_heading(a_step);
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
    sky_dome.set(crate::sky::Weather::Overcast, minutes / 60.0, Color::srgb(z[0], z[1], z[2]), sky_c, crate::sky::SkySun { dir: sun_at, colour: Color::srgb(1.0, 0.9, 0.8), strength: 0.6 * day }, 0.15 + 0.85 * twilight);
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
