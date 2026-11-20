//! The road into Myrnohrad as it looks (design/myrnohrad.md, phase 1): the town's buildings raised from the map
//! (tools/osm) after months of shelling, many roofs gone and some ruins still smouldering, the villages' houses the
//! map leaves out (`bav_core::data::HouseSpec`), the Tsentralna mine's headframe over its shaft, the burnt-out cars
//! on the road, the observers on a nine-storey roof; and the light at 48.3 N on the parts' days: the November fog
//! lifting before noon, a clear December day to sunset, and a January night with no moon.

use crate::meshes::MeshBuilder;
use crate::world::Daylight;
use bav_core::geo::{self, hash01, smoothstep};
use bav_sim::fx::{Emitter, EmitterKind};
use bevy::{camera::Exposure, pbr::DistanceFog, prelude::*};

/// The Tsentralna mine's shaft, the map's mineshaft node (OpenStreetMap; design/myrnohrad.md, the map).
const SHAFT: Vec2 = Vec2::new(1475.0, -248.0);

/// The set: the map's buildings and the villages' houses, the headframe, the cars, the smouldering ruins, and the
/// observers' post where the part's file puts it.
pub fn spawn_set(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut paint: ResMut<Assets<crate::baked::VehicleMaterial>>,
) {
    let look = crate::data::look_data();
    let snow = !bav_core::diorama::diorama().summer();
    let shelled = look.ruined.map(|(ruined, seed)| crate::buildings::Shelled { ruined, seed });
    let mut roofs = MeshBuilder::new();
    crate::buildings::raise(&mut commands, &mut meshes, &mut images, &mut materials, &mut roofs, crate::buildings::Town { snow, hangar: &mut crate::hostomel::hangar, shelled });
    headframe(&mut roofs, snow);
    // The observers: on the roof of a nine-storey block, the town's high ground.
    let ((x, z), h) = bav_core::data::data().op.expect("Myrnohrad's parts place the observers");
    let at = Vec2::new(x, z);
    commands.insert_resource(bav_sim::player::ObservationPost { eye: geo::ground(at) + Vec3::Y * h, building: at });
    let solid = materials.add(StandardMaterial { perceptual_roughness: 0.95, ..default() });
    commands.spawn((Mesh3d(meshes.add(roofs.build())), MeshMaterial3d(solid)));

    // The cars the drones caught on the road and in the yards: Ladas and pickups, burnt out.
    if !look.cars.is_empty() {
        let kinds: Vec<crate::baked::Baked> = ["lada", "l200"].iter().map(|&id| crate::baked::load(id, crate::models::livery(id), &mut meshes, &mut images, &mut paint)).collect();
        for (k, &((x, z), yaw)) in look.cars.iter().enumerate() {
            let m = &kinds[(hash01(k as u32, 41) < 0.3) as usize];
            let mut e = commands.spawn((bav_sim::units::burnt(&m.hull), Transform::from_translation(geo::ground(Vec2::new(x, z))).with_rotation(Quat::from_rotation_y(yaw))));
            bav_sim::units::dress_charred(&mut e, &m.hull);
        }
    }

    // Some of the ruins still smoulder: a thin grey thread from inside the walls.
    if let Some(s) = shelled {
        let lay = geo::layout();
        let ruins = lay.buildings.iter().enumerate().filter(|(i, b)| s.ruined(*i) && b.foot.len() >= 3 && geo::in_bounds(b.foot[0]));
        for (_, b) in ruins.filter(|(i, _)| hash01(*i as u32, s.seed.wrapping_add(5)) < 0.5).take(look.smouldering as usize) {
            let mid = b.foot.iter().copied().sum::<Vec2>() / b.foot.len() as f32;
            commands.spawn((Transform::from_translation(geo::ground(mid) + Vec3::Y * 1.5), Emitter::new(EmitterKind::Smoulder, 0.8, 1.0e5, Vec3::Y)));
        }
    }
}

/// The Tsentralna mine over its shaft: the steel headframe, about 45 m to its sheaves, braced back by its raking
/// strut towards the winding house; the shaft house at its foot and the skip gallery climbing towards the heap. Shelled
/// for months by the day, it stands, sooted.
fn headframe(m: &mut MeshBuilder, snow: bool) {
    let steel = [0.33, 0.29, 0.27];
    let brick = [0.42, 0.31, 0.26];
    let roof = if snow { [0.78, 0.79, 0.80] } else { [0.30, 0.31, 0.32] };
    let g = geo::ground(SHAFT) - Vec3::Y * 0.5;
    // The winding house stands away from the heap; the strut leans towards it.
    let heap = Vec2::new(1608.0, -478.0);
    let back = (SHAFT - heap).normalize();
    let side = back.perp();
    let flat = |v: Vec2| Vec3::new(v.x, 0.0, v.y);
    let beam = |m: &mut MeshBuilder, a: Vec3, b: Vec3, r: f32| m.cylinder(a, b - a, r, 6, steel);
    // Four legs, 7 m apart at the foot and 5 m at the head, ring beams and crossed braces every 8 m.
    let top = 42.0;
    let leg = |k: usize, y: f32| {
        let half = 3.5 - 1.0 * (y / top);
        let (sx, sz) = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)][k % 4];
        g + flat(back * sx * half + side * sz * half) + Vec3::Y * y
    };
    for k in 0..4 {
        beam(m, leg(k, 0.0), leg(k, top), 0.45);
    }
    let mut y = 0.0;
    while y < top {
        let y1 = (y + 8.0).min(top);
        for k in 0..4 {
            beam(m, leg(k, y1), leg(k + 1, y1), 0.22);
            beam(m, leg(k, y), leg(k + 1, y1), 0.15);
            beam(m, leg(k + 1, y), leg(k, y1), 0.15);
        }
        y = y1;
    }
    // The raking strut: two legs from the head down to the ground 30 m back.
    for s in [-1.0, 1.0] {
        let foot = g + flat(back * 30.0 + side * s * 4.0);
        beam(m, foot, leg(if s < 0.0 { 1 } else { 2 }, top - 2.0), 0.4);
        beam(m, foot, leg(if s < 0.0 { 1 } else { 2 }, top * 0.5), 0.25);
    }
    // The head: its platform and the two sheave wheels, turned to the winding house.
    let head = g + Vec3::Y * (top + 0.6);
    m.boxc(head, Vec3::new(8.0, 1.2, 8.0), steel);
    for s in [-1.2, 1.2] {
        let c = head + Vec3::Y * 3.4 + flat(side * s);
        m.cylinder(c - flat(side) * 0.2, flat(side) * 0.4, 3.0, 20, [0.26, 0.24, 0.23]);
    }
    // The shaft house round the headframe's foot, and the winding house behind it.
    m.boxc(g + Vec3::Y * 6.0, Vec3::new(16.0, 12.0, 16.0), brick);
    m.boxc(g + Vec3::Y * 12.2, Vec3::new(16.6, 0.5, 16.6), roof);
    let wh = g + flat(back * 52.0);
    m.cuboid(wh + Vec3::Y * 8.0, Vec3::new(15.0, 8.0, 10.0), Quat::from_rotation_y(-back.y.atan2(back.x)), brick);
    m.cuboid(wh + Vec3::Y * 16.3, Vec3::new(15.6, 0.4, 10.6), Quat::from_rotation_y(-back.y.atan2(back.x)), roof);
    // The skip gallery from the shaft house up towards the heap, on three trestles.
    let (a, b) = (g + Vec3::Y * 10.0, g + flat(-back * 140.0) + Vec3::Y * 28.0);
    let dir = (b - a).normalize();
    let mid = (a + b) * 0.5;
    m.cuboid(mid, Vec3::new(1.8, 1.6, a.distance(b) * 0.5), Quat::from_rotation_arc(Vec3::Z, dir), [0.40, 0.38, 0.36]);
    for t in [0.3, 0.6, 0.9] {
        let p = a.lerp(b, t);
        let foot = Vec3::new(p.x, geo::ground(Vec2::new(p.x, p.z)).y - 0.5, p.z);
        beam(m, foot, p, 0.35);
    }
}

/// What the camera sees by in the fog: the channel the player is looking through, and how far the battle sees.
#[derive(bevy::ecs::system::SystemParam)]
pub struct FogSight<'w> {
    view: Res<'w, bav_sim::player::View>,
    seeing: Option<Res<'w, bav_sim::fight::Seeing>>,
}

/// The light from the sun computed for the place and date (`bav_core::sun`): November's grey morning in fog that
/// lifts before noon, December's low clear sun to its setting, and January's night over snow, which the camera opens
/// up to as Hostomel's does; the moon where it is up (`sun::moon`), and on the night of the frost it is not. The
/// sun's own direction is floored so the ground does not shadow itself black at dawn; its true elevation sets the
/// brightness. In the fog the camera sees as far as the battle does (`bav_sim::fight::Seeing`): the observers, the
/// drone's day channel and the FPV's goggles as far as a man, the drone's thermal channel further.
pub fn daylight(
    day: Daylight,
    mut cam: Query<(&mut DistanceFog, &mut Exposure), With<bav_sim::player::MainCamera>>,
    mut fx_light: ResMut<crate::fx::FxLight>,
    fog_sight: FogSight,
) {
    let Daylight { script, mut sun, mut ambient, mut clear, skirt, mut materials, mut sky_dome, mut last, look } = day;
    let FogSight { view, seeing } = fog_sight;
    let dbg = crate::dev::dev().daydbg;
    if dbg == Some(crate::dev::DayDebug::Off) {
        return;
    }
    let minutes = bav_sim::director::clock_minutes(script.t);
    let winter = !bav_core::diorama::diorama().summer();
    let fog = bav_core::data::fog_at(minutes);
    let weather = crate::dev::dev().sky.unwrap_or(if bav_core::data::data().overcast || fog.is_some() { crate::sky::Weather::Overcast } else { crate::sky::Weather::Sun });
    let overcast = weather == crate::sky::Weather::Overcast;
    let (elev, azim) = bav_core::sun::position(minutes);
    let azim = geo::map_heading(azim).to_radians();
    let s = smoothstep;
    let twilight = s(-12.0, 0.0, elev);
    let day = s(0.0, 25.0, elev);
    let light = 0.70 + 0.30 * day.powf(0.5);
    let mix3 = |a: [f32; 3], b: [f32; 3], t: f32| [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
    let [k_sun, k_amb, ev_off] = crate::dev::dev().light;
    // The moon, where it is up: how much light it gives, as a share of a full moon high in the sky.
    let moon = bav_core::sun::moon(minutes).filter(|m| m.0 > 0.0);
    let moonlight = moon.map_or(0.0, |(e, _, lit)| lit * s(0.0, 25.0, e));
    let e = elev.to_radians();
    let sun_at = Vec3::new(azim.sin() * e.cos(), e.sin(), -azim.cos() * e.cos());
    let (e_step, a_step) = bav_core::sun::position(crate::world::sun_hours(minutes) * 60.0);
    let (e_step, a_step) = match moon {
        // At night the key is the moon's, where it is up.
        Some((me, ma, _)) if twilight <= 0.0 => (me.max(12.0).to_radians(), geo::map_heading(ma).to_radians()),
        _ => (e_step.max(10.0).to_radians(), geo::map_heading(a_step).to_radians()),
    };
    for (mut l, mut tf) in &mut sun {
        let by_day = if overcast { 3400.0 * day + 400.0 * twilight } else { 14000.0 * (0.75 + 0.25 * day) * light * twilight };
        l.illuminance = (by_day + 250.0 * moonlight) * k_sun;
        let dir = Vec3::new(a_step.sin() * e_step.cos(), e_step.sin(), -a_step.cos() * e_step.cos());
        let to = Transform::from_translation(dir).looking_at(Vec3::ZERO, Vec3::Y);
        if dbg != Some(crate::dev::DayDebug::NoSun) && tf.rotation != to.rotation {
            *tf = to;
        }
        let c = if twilight <= 0.0 {
            [0.62, 0.70, 0.95]
        } else if overcast {
            mix3([0.60, 0.65, 0.80], [0.93, 0.93, 0.94], day)
        } else {
            mix3([1.0, 0.70, 0.48], [1.0, 0.94, 0.86], day)
        };
        l.color = Color::srgb(c[0], c[1], c[2]);
    }
    // At night a little is left: the snow gives back what there is, and the sky over the burning towns.
    let snow = if winter { 1.3 } else { 1.0 };
    ambient.brightness = (if overcast { 140.0 + 620.0 * twilight + 900.0 * day } else { 110.0 + 520.0 * twilight + 330.0 * day } + 160.0 * moonlight) * snow * k_amb;
    let a_col = mix3([0.30, 0.36, 0.60], [0.80, 0.84, 0.92], twilight);
    ambient.color = Color::srgb(a_col[0], a_col[1], a_col[2]);
    let (horizon, zenith) = if overcast {
        let h = mix3(mix3([0.03, 0.04, 0.07], [0.30, 0.33, 0.42], twilight), [0.66, 0.68, 0.70], day);
        let z = mix3(mix3([0.02, 0.03, 0.05], [0.24, 0.27, 0.36], twilight), [0.54, 0.57, 0.61], day);
        (h, z)
    } else {
        // A cold clear sky: deep blue overhead, warm and pale low down by the sun's setting.
        let h = mix3(mix3([0.03, 0.04, 0.08], [0.62, 0.48, 0.42], twilight), [0.74, 0.79, 0.86], day);
        let z = mix3(mix3([0.01, 0.02, 0.05], [0.22, 0.28, 0.48], twilight), [0.34, 0.48, 0.74], day);
        (h, z)
    };
    // In the fog the sky is the fog.
    let thick = fog.map_or(0.0, |v| 1.0 - s(400.0, 3000.0, v));
    let fog_grey = [0.62 * light, 0.63 * light, 0.64 * light];
    let horizon = mix3(horizon, fog_grey, thick);
    let zenith = mix3(zenith, fog_grey, thick * 0.8);
    let sky = Color::srgb(horizon[0], horizon[1], horizon[2]);
    clear.0 = sky;
    fx_light.0 = 0.3 + 0.7 * s(-6.0, 10.0, elev);
    let sun_colour = if overcast { Color::srgb(0.9, 0.9, 0.9) } else { Color::srgb(1.0, 0.88, 0.70) };
    let strength = if overcast { 0.4 * day } else { 1.5 * day.powf(0.3) } * (1.0 - thick);
    sky_dome.set(weather, minutes / 60.0, Color::srgb(zenith[0], zenith[1], zenith[2]), sky, crate::sky::SkySun { dir: sun_at, colour: sun_colour, strength }, 0.15 + 0.85 * twilight);
    for (mut f, mut ex) in &mut cam {
        f.color = sky;
        let eye = if view.thermal && view.mode == bav_core::vocab::Mode::Drone { bav_sim::fight::Eye::Thermal } else { bav_sim::fight::Eye::Man };
        let vis = seeing.as_ref().map(|s| s.through(eye)).filter(|m| m.is_finite()).unwrap_or(if overcast { 5000.0 } else { 7000.0 });
        let (near, far) = if thick > 0.0 { (Color::srgb(0.55, 0.56, 0.57), Color::srgb(0.70, 0.71, 0.72)) } else if overcast { (Color::srgb(0.50, 0.51, 0.53), Color::srgb(0.70, 0.71, 0.72)) } else { (Color::srgb(0.52, 0.52, 0.52), Color::srgb(0.76, 0.77, 0.78)) };
        f.falloff = bevy::pbr::FogFalloff::from_visibility_colors(vis * look.haze, near, far);
        if twilight <= 0.0 {
            f.directional_light_color = Color::NONE;
        }
        // The sensor opens up as the light goes (a drone's low-light mode: grainy, but the field is there).
        ex.ev100 = 6.3 + 3.7 * s(-14.0, 10.0, elev) + 1.0 * day + if overcast { 0.2 } else { 0.0 } - look.exposure + ev_off;
    }
    let changed = last.is_none_or(|l| (l.to_srgba().red - sky.to_srgba().red).abs() > 0.004);
    if changed && let Some(sk) = skirt.as_ref() && let Some(mut m) = materials.get_mut(&sk.0) {
        m.base_color = sky;
        *last = Some(sky);
    }
}
