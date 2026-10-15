//! Mala Tokmachka, 8 June 2023: the second diorama. Robotyne is Vuhledar from the other side.
//!
//! The history is in db/research/chapters/c09-robotyne.md (§3, §5, §6 set piece 1). Before dawn the 47th Mechanised
//! Brigade goes south behind Leopard 2R and BMR-2 mine-clearing vehicles, in two groups of nine and eleven, up a
//! long open slope towards the 291st Regiment's first line on the crest. The barrier minefields are hundreds of
//! metres deep, with anti-tank mines stacked two high; the lanes are too shallow, the columns stack up on them,
//! and drones, guns, ATGMs and Ka-52s firing from beyond reach do the rest. Four more Bradleys go back in for the
//! men. Most crews get out through the hatches and the lowered ramps.
//!
//! The box is the same size as Vuhledar's. +X is east, +Z south: the attack climbs from the N08 highway in the
//! north (about 70 m) to the crest in the south (about 110 m), and the fields between are a grid of tree lines.

use crate::camera::MainCamera;
use crate::baked::Part;
use crate::fire::Missile;
use crate::fx::{self, FxAssets, FxRng};
use crate::geo::{self, smoothstep};
use crate::meshes::{self, MeshBuilder};
use crate::units::{Models, Radio, Side, VKind, Vehicle, VehicleSpec};
use crate::world::{ObservationPost, Palette, Skirt};
use bevy::{camera::Exposure, pbr::FogFalloff, prelude::*};
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub use bav_core::geo::robotyne::{TRENCH_Z, UA_LINE_Z};
use bav_core::geo::robotyne::n08_z;

// ---------------------------------------------------------------------------------------------------------
// The set: the farm by the N08 and its water tower, the power line, last summer's combine, the Russian dug-in
// tanks on the crest.


pub struct RobotynePlugin;

impl Plugin for RobotynePlugin {
    fn build(&self, app: &mut App) {
        if crate::diorama::diorama() != crate::diorama::Diorama::Robotyne {
            return;
        }
        app.insert_resource(HeliRng(SmallRng::seed_from_u64(crate::dev::seeded(52))))
            .add_systems(Update, fly_helicopters.run_if(crate::running).in_set(crate::Step::Fire))
            .add_systems(Update, spin_rotors.in_set(crate::Step::Effects));
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

/// The 291st's T-72B3s, dug in hull-down on the crest behind the trench, facing north.
const DUG_IN: [(Vec2, f32); 2] = [(Vec2::new(-150.0, TRENCH_Z + 22.0), 0.15), (Vec2::new(1050.0, TRENCH_Z + 18.0), -0.1)];

pub fn spawn_dug_in(commands: &mut Commands, models: &Models, palette: &Palette) {
    for (k, (p, yaw)) in DUG_IN.into_iter().enumerate() {
        let spec = VehicleSpec { side: Side::Ru, kind: VKind::T72, route: 0, s: 0.0, lat: 0.0, wave: 0, idx: 100 + k as u32, plow: false, smoke: false, rescue: false };
        let e = crate::units::spawn_vehicle_at(commands, models, palette, spec, p, yaw);
        commands.entity(e).insert(Transform::from_translation(geo::ground(p) - Vec3::Y * 1.1).with_rotation(Quat::from_rotation_y(yaw)));
    }
}

// ---------------------------------------------------------------------------------------------------------
// The Ka-52s: on station 4 km out over the Russian side, popping up to fire Vikhrs and dropping back. Nothing
// the brigade has reaches them (dossier c09 §5: "one of the most influential Russian weapons systems").

#[derive(Resource)]
pub struct HeliModels {
    body: Vec<Part>,
    rotor: Vec<Part>,
    rotor_at: Vec3,
}

#[derive(Resource)]
struct HeliRng(SmallRng);

#[derive(Component)]
pub struct Helicopter {
    home: Vec2,
    timer: f32,
    shots: u32,
    age: f32,
    phase: f32,
}

#[derive(Component)]
struct Rotor;

pub fn spawn_helicopter(commands: &mut Commands, h: &HeliModels, at: Vec2) {
    let phase = (at.x * 0.013).sin() * 3.0;
    let mut e = commands.spawn((
        crate::units::skin(&h.body),
        Transform::from_translation(geo::ground(at) + Vec3::Y * 20.0),
        Helicopter { home: at, timer: 10.0 + phase.abs() * 3.0, shots: 6, age: 0.0, phase },
    ));
    e.with_children(|c| crate::units::dress(&mut c.spawn((crate::units::skin(&h.rotor), Transform::from_translation(h.rotor_at), Rotor)), &h.rotor));
    crate::units::dress(&mut e, &h.body);
}

#[allow(clippy::too_many_arguments)]
fn fly_helicopters(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<HeliRng>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut radio: ResMut<Radio>,
    smoke: Res<fx::Smoke>,
    models: Res<Models>,
    palette: Res<Palette>,
    vehicles: Query<(Entity, &Vehicle, &Transform), Without<Helicopter>>,
    mut helis: Query<(Entity, &mut Helicopter, &mut Transform)>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    for (e, mut h, mut tf) in &mut helis {
        h.age += dt;
        h.timer -= dt;
        let leaving = h.shots == 0;
        let mut p = tf.translation;
        if leaving {
            // Out of missiles: away, low, back to Berdiansk.
            let away = h.home.normalize_or_zero();
            p += Vec3::new(away.x, 0.0, away.y) * 45.0 * dt;
            if p.xz().length() > 9000.0 {
                commands.entity(e).despawn();
                continue;
            }
        }
        // Hover low behind the ridge; pop up for a few seconds to fire.
        let pop = (1.0 - (h.timer.min(4.0) / 4.0)).max(0.0);
        let want_y = geo::height(p.x, p.z) + 22.0 + 5.0 * (h.age * 0.35 + h.phase).sin() + 35.0 * pop;
        p.y += (want_y - p.y) * (1.0 - (-dt * 1.2).exp());
        p.x = if leaving { p.x } else { h.home.x + 40.0 * (h.age * 0.11 + h.phase).sin() };
        tf.translation = p;
        let face = Vec2::new(0.0, 200.0) - p.xz();
        tf.rotation = Quat::from_rotation_y(crate::units::vehicle_yaw(face)) * Quat::from_rotation_x(-0.08 - 0.06 * pop);
        if leaving || h.timer > 0.0 {
            continue;
        }
        // A target: something of ours that the smoke does not hide, preferably something that cannot move.
        let from = p.xz();
        let mut best: Option<(Entity, f32)> = None;
        for (ve, v, vt) in &vehicles {
            if v.side != Side::Ua || !v.state.alive() || smoke.blocks(from, vt.translation.xz(), now) {
                continue;
            }
            let score = vt.translation.xz().distance(from) - if v.state.mobile() { 0.0 } else { 1500.0 } + rng.random_range(0.0..800.0);
            if best.is_none_or(|b| score < b.1) {
                best = Some((ve, score));
            }
        }
        h.timer = rng.random_range(18.0..30.0);
        let Some((target, _)) = best else { continue };
        h.shots -= 1;
        let fwd = tf.rotation * Vec3::NEG_Z;
        let muzzle = p + fwd * 3.0 - Vec3::Y * 0.6;
        fx::flash(&mut commands, muzzle, 4.0e6, 80.0, 0.15);
        for _ in 0..6 {
            fx::spawn(&mut commands, &mut fx, fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: muzzle, vel: -fwd * fxr.0.random_range(3.0..9.0), life: 6.0, size0: 1.5, size1: 8.0, alpha: 0.5, drag: 1.0, ..default() });
        }
        commands.spawn((
            Mesh3d(models.missile.clone()),
            MeshMaterial3d(palette.vertex.clone()),
            Transform::from_translation(muzzle).looking_to(fwd, Vec3::Y),
            Missile { target, launched_at: muzzle, speed: 450.0, trail: 0.0, p_hit: Some(0.62), top_attack: false },
            bevy::light::NotShadowCaster,
        ));
        if rng.random_bool(0.5) {
            radio.say(now, "Launch from the helicopter! Missile inbound!");
        }
    }
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
    mut fx_light: ResMut<fx::FxLight>,
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
