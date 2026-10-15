//! Mala Tokmachka, 8 June 2023, as it looks: the farm by the N08 and its water tower (the observers' post), the
//! power line, last summer's combine, the berms on the crest, the Ka-52s' rotors, and the June light. The battle's
//! side is `bav_sim::robotyne`.

use crate::meshes::{self, MeshBuilder};
use crate::world::Skirt;
use bav_core::geo::robotyne::n08_z;
use bav_core::geo::{self, smoothstep};
use bav_sim::player::{MainCamera, ObservationPost};
use bav_sim::robotyne::{DUG_IN, HeliModels, Rotor};
use bevy::{camera::Exposure, pbr::FogFalloff, prelude::*};

pub struct RobotyneLookPlugin;

impl Plugin for RobotyneLookPlugin {
    fn build(&self, app: &mut App) {
        if bav_core::diorama::diorama() != bav_core::diorama::Diorama::Robotyne {
            return;
        }
        app.add_systems(Update, spin_rotors.in_set(bav_core::schedule::Step::Effects));
    }
}

/// The observers' water tower, by the farm on the N08.
const TOWER: Vec2 = Vec2::new(-1150.0, -1470.0);

pub fn spawn_set(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, mut images: ResMut<Assets<Image>>) {
    let solid = materials.add(StandardMaterial { perceptual_roughness: 0.95, ..default() });
    let mut set = MeshBuilder::new();
    // The kolkhoz by the highway: long sheds, two of them roofless, a row of silos, and the water tower.
    let g = geo::ground(TOWER);
    set.append(&meshes::water_tower(), Transform::from_translation(g - Vec3::Y * 0.3));
    for (k, (x, z, len, ruin)) in [(-1260.0f32, -1430.0f32, 72.0f32, 0.0f32), (-1230.0, -1395.0, 60.0, 0.7), (-1080.0, -1415.0, 54.0, 0.35), (-1300.0, -1500.0, 40.0, 1.0)].into_iter().enumerate() {
        let p = Vec2::new(x, z);
        let rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2 + (k as f32 - 1.5) * 0.02);
        set.append(&meshes::farm_shed(len, ruin), Transform::from_translation(geo::ground(p) - Vec3::Y * 0.3).with_rotation(rot));
    }
    for k in 0..4 {
        let p = Vec2::new(-1020.0 + k as f32 * 9.0, -1470.0);
        let mut silo = MeshBuilder::new();
        silo.cylinder(Vec3::ZERO, Vec3::new(0.0, 14.0, 0.0), 3.6, 10, [0.62, 0.61, 0.58]);
        silo.frustum(Vec3::new(0.0, 14.0, 0.0), Vec3::new(0.0, 1.6, 0.0), 3.6, 0.4, 10, [0.45, 0.46, 0.45], false);
        if k == 2 {
            silo.boxc(Vec3::new(2.8, 9.0, -2.0), Vec3::new(2.0, 3.0, 1.0), [0.08, 0.08, 0.08]);
        }
        set.append(&silo, Transform::from_translation(geo::ground(p) - Vec3::Y * 0.3));
    }
    // Last summer's combine, burned where it stood; a car shot up on the highway.
    set.append(&meshes::combine(), Transform::from_translation(geo::ground(Vec2::new(-860.0, -1180.0))).with_rotation(Quat::from_rotation_y(0.4)));
    let car = geo::ground(Vec2::new(380.0, n08_z(380.0) + 2.0));
    set.cuboid(car + Vec3::Y * 0.6, Vec3::new(0.85, 0.45, 2.1), Quat::from_rotation_y(1.3), [0.18, 0.16, 0.15]);
    // The power line along the N08, half its poles down.
    let n08 = &geo::layout().roads[0].line;
    let mut s = 0.0;
    let mut k = 0u32;
    while s < n08.length() {
        k += 1;
        let p = n08.point(s) + n08.normal(s) * 14.0;
        let lean = if geo::hash01(k, 77) < 0.35 { (geo::hash01(k, 78) - 0.5) * 1.4 } else { 0.0 };
        if geo::hash01(k, 79) > 0.15 {
            set.append(&meshes::pole(lean), Transform::from_translation(geo::ground(p)).with_rotation(Quat::from_rotation_y(n08.tangent(s).to_angle())));
        }
        s += 45.0;
    }
    // The crest: log roofs over the dugouts, and earth berms where the tanks are dug in.
    for p in &geo::layout().ru_positions {
        let at = geo::ground(*p + Vec2::new(5.0, 14.0));
        set.boxc(at + Vec3::Y * 0.2, Vec3::new(5.0, 0.4, 5.0), [0.30, 0.25, 0.19]);
    }
    for (p, _) in DUG_IN {
        let at = geo::ground(p - Vec2::new(0.0, 5.0));
        set.extrude_x(&[Vec2::new(-3.0, -0.2), Vec2::new(3.0, -0.2), Vec2::new(0.0, 1.6)], 3.2, at, [0.42, 0.36, 0.27]);
    }
    commands.spawn((Mesh3d(meshes.add(set.build())), MeshMaterial3d(solid)));
    commands.insert_resource(ObservationPost { eye: g + Vec3::new(0.0, 18.4, 2.7), building: TOWER });

    let b = crate::baked::load("ka52", Color::WHITE, &mut meshes, &mut images, &mut materials);
    let heli = HeliModels { body: b.hull, rotor: b.turret, rotor_at: b.turret_at };
    commands.insert_resource(heli);
}

fn spin_rotors(time: Res<Time>, mut q: Query<&mut Transform, With<Rotor>>) {
    for mut tf in &mut q {
        tf.rotate_y(time.delta_secs() * 28.0);
    }
}

// ---------------------------------------------------------------------------------------------------------
// The light: 8 June at 47.5 N. Sunrise 04:40 in the north-east, 79% cloud (ERA5, dossier §3). The column moves
// off in the blue before dawn and reaches the minefield as the sky colours behind it.

#[allow(clippy::too_many_arguments)]
pub fn daylight(
    script: Res<crate::director::Script>,
    mut sun: Query<(&mut DirectionalLight, &mut Transform)>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut clear: ResMut<ClearColor>,
    mut cam: Query<(&mut DistanceFog, &mut Exposure), With<MainCamera>>,
    skirt: Option<Res<Skirt>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fx_light: ResMut<crate::fx::FxLight>,
    mut sky_dome: crate::sky::Sky,
    mut last: Local<Option<Color>>,
) {
    let dbg = crate::dev::dev().daydbg;
    if dbg == Some(crate::dev::DayDebug::Off) {
        return;
    }
    let h = crate::director::clock_minutes(script.t) / 60.0;
    let (rise, noon, set, top) = (4.67f32, 12.62f32, 20.5f32, 65.5f32);
    let elev = top * (std::f32::consts::PI * (h - rise) / (set - rise)).sin();
    // Azimuth from north, clockwise: about 53 degrees at sunrise, south at solar noon.
    let azim = (53.0 + (h - rise) / (noon - rise) * 127.0).to_radians();
    let s = |e0: f32, e1: f32, x: f32| smoothstep(e0, e1, x);
    let twilight = s(-9.0, 1.0, elev);
    let day = s(0.0, 30.0, elev);
    let dawn = s(-5.0, 2.0, elev) * (1.0 - s(4.0, 16.0, elev));
    let mix3 = |a: [f32; 3], b: [f32; 3], t: f32| [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
    let e = elev.max(10.0).to_radians();
    let [k_sun, k_amb, ev_off] = crate::dev::dev().light;
    let up = s(-1.0, 7.0, elev);
    for (mut l, mut tf) in &mut sun {
        // Before sunrise the light is the glow of the sky in the north-east: weak, blue, soft.
        l.illuminance = (14000.0 * up * (0.6 + 0.4 * day) + 2500.0 * (1.0 - up) * (0.4 + 0.6 * twilight)) * k_sun;
        let dir = Vec3::new(azim.sin() * e.cos(), e.sin(), -azim.cos() * e.cos());
        if dbg != Some(crate::dev::DayDebug::NoSun) {
            *tf = Transform::from_translation(dir).looking_at(Vec3::ZERO, Vec3::Y);
        }
        let warm = [1.0, 0.62 + 0.33 * s(2.0, 20.0, elev), 0.38 + 0.52 * s(2.0, 25.0, elev)];
        let c = mix3([0.55, 0.65, 1.0], warm, up);
        l.color = Color::srgb(c[0], c[1], c[2]);
    }
    ambient.brightness = (1820.0 + 600.0 * twilight - 1000.0 * day) * k_amb;
    ambient.color = Color::srgb(0.55 + 0.30 * twilight, 0.62 + 0.24 * twilight, 0.92 + 0.04 * twilight);
    // Blue hour, then a pale dawn under the cloud, then the grey-white of an overcast June morning.
    let mut sky = mix3([0.22, 0.27, 0.38], [0.60, 0.60, 0.65], twilight);
    sky = mix3(sky, [0.74, 0.62, 0.56], dawn * 0.6);
    sky = mix3(sky, [0.72, 0.74, 0.77], day);
    let sky = Color::srgb(sky[0], sky[1], sky[2]);
    clear.0 = sky;
    fx_light.0 = 0.4 + 0.6 * s(-4.0, 12.0, elev);
    // The sky itself: deep blue before dawn, a milky veil by day with the sun a hazy disc through it.
    let sun_at = {
        let e = elev.to_radians();
        Vec3::new(azim.sin() * e.cos(), e.sin(), -azim.cos() * e.cos())
    };
    let z = mix3([0.10, 0.14, 0.28], [0.52, 0.60, 0.78], twilight);
    let z = mix3(z, [0.58, 0.66, 0.80], day);
    sky_dome.set(crate::sky::Weather::Veil, h, Color::srgb(z[0], z[1], z[2]), sky, sun_at, Color::srgb(1.0, 0.78 + 0.18 * day, 0.55 + 0.35 * day), 2.0 * s(-3.0, 10.0, elev), 0.3 + 0.7 * s(-4.0, 12.0, elev));
    for (mut f, mut ex) in &mut cam {
        f.color = sky;
        // Clearer than February's haze, though the morning was 79% cloud.
        f.falloff = FogFalloff::from_visibility_colors(4300.0, Color::srgb(0.48, 0.5, 0.54), Color::srgb(0.72, 0.74, 0.77));
        // The haze towards the sun glows at dawn.
        f.directional_light_color = Color::srgba(1.0, 0.62, 0.36, 0.15 + 0.45 * dawn);
        // A camera's exposure follows the light: the blue hour reads dark, but not black.
        ex.ev100 = 8.8 + 2.45 * s(-6.0, 14.0, elev) + ev_off;
    }
    let changed = last.is_none_or(|l| (l.to_srgba().red - sky.to_srgba().red).abs() > 0.004 || (l.to_srgba().blue - sky.to_srgba().blue).abs() > 0.004);
    if changed && let Some(sk) = skirt.as_ref() && let Some(mut m) = materials.get_mut(&sk.0) {
        m.base_color = sky;
        *last = Some(sky);
    }
}
