//! The northern shoulder of the Avdiivka salient as it looks (design/avdiivka.md, phase 1): the buildings of
//! Stepove and the coke plant raised from the map (tools/osm), the observers on the spoil heap, and the light of
//! a clear October day at 48.2 N, from the first light before the columns to noon; and in the third part a
//! January afternoon under cloud and snow.

use crate::meshes::MeshBuilder;
use crate::world::Skirt;
use bav_core::geo::{self, smoothstep};
use bevy::{camera::Exposure, pbr::DistanceFog, prelude::*};

/// The set: the map's buildings, and the observers' post where the part's file puts it (the heap's top).
pub fn spawn_set(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut roofs = MeshBuilder::new();
    let snow = !bav_core::diorama::diorama().summer();
    crate::buildings::raise(&mut commands, &mut meshes, &mut images, &mut materials, &mut roofs, snow, &mut crate::hostomel::hangar);
    // The observers: on the heap's first terrace, where the 110th saw every Russian move from the heights.
    let ((x, z), h) = bav_core::data::data().op.expect("Avdiivka's parts place the observers");
    let at = Vec2::new(x, z);
    commands.insert_resource(bav_sim::player::ObservationPost { eye: geo::ground(at) + Vec3::Y * h, building: at });
    plant(&mut roofs);
    if !roofs.is_empty() {
        let solid = materials.add(StandardMaterial { perceptual_roughness: 0.95, ..default() });
        commands.spawn((Mesh3d(meshes.add(roofs.build())), MeshMaterial3d(solid)));
    }
}

/// The coke plant's skyline along the box's south-west end, placed by eye inside its area (the map has its
/// footprints but none of its heights): the coke batteries, long and low with their coal towers and quench
/// towers over them, the chimneys that marked the plant from anywhere on the salient, and the fuel farm's tanks.
/// Bombed for months by the day, its shapes stand but blackened.
fn plant(m: &mut MeshBuilder) {
    let soot = [0.30, 0.29, 0.28];
    let brick = [0.42, 0.33, 0.28];
    let steel = [0.36, 0.37, 0.38];
    let at = |x: f32, z: f32| geo::ground(Vec2::new(x, z)) - Vec3::Y * 0.5;
    // Two batteries along the plant's long axis (+x), each a wall of ovens with its coal tower at one end and the
    // quench tower off its side.
    for (x, z) in [(-1500.0, -600.0), (-1750.0, -280.0)] {
        let g = at(x, z);
        m.boxc(g + Vec3::Y * 7.0, Vec3::new(190.0, 14.0, 16.0), brick);
        m.boxc(g + Vec3::Y * 14.5, Vec3::new(190.0, 1.4, 20.0), steel);
        m.boxc(g + Vec3::new(-105.0, 23.0, 0.0), Vec3::new(13.0, 46.0, 13.0), soot);
        m.boxc(g + Vec3::new(60.0, 15.0, 24.0), Vec3::new(11.0, 30.0, 11.0), steel);
    }
    // The chimneys: tapering stacks of brick and concrete.
    for (x, z, h) in [(-1400.0, -720.0, 80.0), (-1770.0, -470.0, 70.0), (-2020.0, -200.0, 92.0), (-1300.0, -1000.0, 60.0), (-2200.0, -880.0, 75.0)] {
        let g = at(x, z);
        m.cylinder(g, Vec3::Y * h, 4.2, 12, brick);
        m.cylinder(g + Vec3::Y * (h - 0.5), Vec3::Y * 2.0, 4.6, 12, soot);
    }
    // The fuel farm's tanks, where the map has them.
    for (x, z) in [(-1999.0, -821.0), (-2065.0, -767.0), (-1992.0, -785.0), (-2037.0, -813.0), (-2028.0, -776.0), (-2075.0, -802.0)] {
        m.cylinder(at(x, z), Vec3::Y * 12.0, 14.0, 16, steel);
    }
}

/// The light of the day, from the sun computed for the place and date (`bav_core::sun`): a clear, dry October
/// morning with hard shadows (parts I and II; the second day's overcast is the sky switch), or January's flat
/// grey over snow (part III). The sun's own direction is floored at 12 degrees so the steppe does not shadow
/// itself black at dawn; its true elevation sets the brightness.
#[allow(clippy::too_many_arguments)]
pub fn daylight(
    script: Res<bav_sim::director::Script>,
    mut sun: Query<(&mut DirectionalLight, &mut Transform)>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut clear: ResMut<ClearColor>,
    mut cam: Query<(&mut DistanceFog, &mut Exposure), With<bav_sim::player::MainCamera>>,
    skirt: Option<Res<Skirt>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sky_dome: crate::sky::Sky,
    mut last: Local<Option<Color>>,
    look: Res<crate::LookSettings>,
) {
    let dbg = crate::dev::dev().daydbg;
    if dbg == Some(crate::dev::DayDebug::Off) {
        return;
    }
    let minutes = bav_sim::director::clock_minutes(script.t);
    let winter = !bav_core::diorama::diorama().summer();
    let weather = crate::dev::dev().sky.unwrap_or(if winter { crate::sky::Weather::Overcast } else { crate::sky::Weather::Sun });
    let overcast = weather == crate::sky::Weather::Overcast;
    let (elev, azim) = bav_core::sun::position(minutes);
    let azim = geo::map_heading(azim).to_radians();
    let twilight = smoothstep(-12.0, 0.0, elev);
    let day = smoothstep(0.0, 25.0, elev);
    // A camera's auto-exposure flattens the day: first light still reads as light grey, only cooler.
    let light = 0.70 + 0.30 * day.powf(0.5);
    let mix3 = |a: [f32; 3], b: [f32; 3], t: f32| [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
    let [k_sun, k_amb, ev_off] = crate::dev::dev().light;
    // Where the sun really is, for the sky, and the light's direction in the sun's steps, floored.
    let e = elev.to_radians();
    let sun_at = Vec3::new(azim.sin() * e.cos(), e.sin(), -azim.cos() * e.cos());
    let (e_step, a_step) = bav_core::sun::position(crate::world::sun_hours(minutes) * 60.0);
    let (e_step, a_step) = (e_step.max(12.0).to_radians(), geo::map_heading(a_step).to_radians());
    for (mut l, mut tf) in &mut sun {
        l.illuminance = if overcast { (3500.0 * day + 400.0 * twilight) * k_sun } else { 14000.0 * (0.75 + 0.25 * day) * light * twilight * k_sun };
        let dir = Vec3::new(a_step.sin() * e_step.cos(), e_step.sin(), -a_step.cos() * e_step.cos());
        let to = Transform::from_translation(dir).looking_at(Vec3::ZERO, Vec3::Y);
        if dbg != Some(crate::dev::DayDebug::NoSun) && tf.rotation != to.rotation {
            *tf = to;
        }
        // Low autumn sun: warm through the morning haze, white by noon; under cloud a cool grey key.
        let c = if overcast { mix3([0.60, 0.65, 0.80], [0.93, 0.93, 0.94], day) } else { mix3([1.0, 0.78, 0.55], [1.0, 0.95, 0.88], day) };
        l.color = Color::srgb(c[0], c[1], c[2]);
    }
    ambient.brightness = (if overcast { 520.0 } else { 420.0 }) * light * k_amb;
    let (sky, zenith) = if overcast {
        (Color::srgb(0.66 * light, 0.68 * light, 0.70 * light), Color::srgb(0.52 * light, 0.55 * light, 0.59 * light))
    } else {
        // A dry October sky: pale at the horizon, a deep blue overhead, warm at dawn.
        let hz = mix3([0.80, 0.66, 0.52], [0.74, 0.79, 0.86], day);
        let zn = mix3([0.40, 0.44, 0.62], [0.36, 0.50, 0.76], day);
        (Color::srgb(hz[0] * light, hz[1] * light, hz[2] * light), Color::srgb(zn[0] * light, zn[1] * light, zn[2] * light))
    };
    clear.0 = sky;
    let sun_colour = if overcast { Color::srgb(0.9, 0.9, 0.9) } else { Color::srgb(1.0, 0.90, 0.72) };
    sky_dome.set(weather, minutes / 60.0, zenith, sky, sun_at, sun_colour, if overcast { 0.4 } else { 1.5 * day.powf(0.3) }, 0.5 + 0.5 * day);
    for (mut f, mut ex) in &mut cam {
        f.color = sky;
        // Dry air, far horizons; the haze thickens under January's cloud.
        let vis = if overcast { 5000.0 } else { 7000.0 };
        let (near, far) = if overcast { (Color::srgb(0.50, 0.51, 0.53), Color::srgb(0.70, 0.71, 0.72)) } else { (Color::srgb(0.52, 0.52, 0.50), Color::srgb(0.76, 0.76, 0.74)) };
        f.falloff = bevy::pbr::FogFalloff::from_visibility_colors(vis * look.haze, near, far);
        // The camera opens up at first light and under cloud: dawn still reads, only cooler.
        ex.ev100 = (if overcast { 10.2 } else { 10.0 }) + 1.0 * day - look.exposure + ev_off;
    }
    let changed = last.is_none_or(|l| (l.to_srgba().red - sky.to_srgba().red).abs() > 0.004);
    if changed && let Some(sk) = skirt.as_ref() && let Some(mut m) = materials.get_mut(&sk.0) {
        m.base_color = sky;
        *last = Some(sky);
    }
}
