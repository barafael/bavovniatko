//! Particles, flashes, fire and smoke. A small CPU particle system: camera-facing quads that share a handful of
//! materials (one per kind × alpha step), so everything batches.

use crate::camera::MainCamera;
use bevy::{asset::RenderAssetUsages, light::NotShadowCaster, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use strum::VariantArray;

pub struct FxPlugin;

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FxRng(SmallRng::seed_from_u64(20230206)))
            .insert_resource(Wind(Vec3::new(3.5, 0.0, -1.2)))
            .add_systems(Startup, setup_fx)
            .add_systems(Update, (update_particles, update_emitters, update_flashes).chain().in_set(crate::Step::Effects));
    }
}

#[derive(Resource)]
pub struct FxRng(pub SmallRng);

/// Wind drifts every plume the same way: east-north-east at a few m/s (ERA5: up to 26 km/h on 7–8 Feb).
#[derive(Resource)]
pub struct Wind(pub Vec3);

#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::VariantArray)]
pub enum Kind {
    BlackSmoke,
    GreySmoke,
    WhiteSmoke,
    Dirt,
    Snow,
    Fire,
    Flash,
    Tracer,
    Spark,
}

const STEPS: usize = 8;

#[derive(Resource)]
pub struct FxAssets {
    quad: Handle<Mesh>,
    mats: Vec<Handle<StandardMaterial>>,
    pub crater_flash: Handle<StandardMaterial>,
    pub sphere: Handle<Mesh>,
    count: usize,
}

impl FxAssets {
    fn mat(&self, k: Kind, alpha: f32) -> Handle<StandardMaterial> {
        let ki = k as usize;
        let ai = ((alpha.clamp(0.0, 1.0) * (STEPS - 1) as f32).round() as usize).min(STEPS - 1);
        self.mats[ki * STEPS + ai].clone()
    }
}

fn puff_texture() -> Image {
    const N: usize = 64;
    let mut d = Vec::with_capacity(N * N * 4);
    for y in 0..N {
        for x in 0..N {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5) / N as f32 * 2.0 - 1.0;
            let r = p.length();
            let n = crate::geo::fbm(p.x * 2.5, p.y * 2.5, 3, 5) * 0.35;
            let a = (1.0 - (r + n).clamp(0.0, 1.0)).powf(1.4);
            d.extend_from_slice(&[255, 255, 255, (a * 255.0) as u8]);
        }
    }
    Image::new(Extent3d { width: N as u32, height: N as u32, depth_or_array_layers: 1 }, TextureDimension::D2, d, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default())
}

fn setup_fx(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, mut images: ResMut<Assets<Image>>) {
    let tex = images.add(puff_texture());
    let mut mats = Vec::new();
    for &k in Kind::VARIANTS {
        for a in 0..STEPS {
            let alpha = (a as f32 + 1.0) / STEPS as f32;
            // Unlit, pre-shaded for an overcast sky: billboards lit by a low sun look wrong from behind.
            let (rgb, emissive): ([f32; 3], f32) = match k {
                Kind::BlackSmoke => ([0.035, 0.032, 0.03], 0.0),
                Kind::GreySmoke => ([0.30, 0.30, 0.30], 0.0),
                Kind::WhiteSmoke => ([0.62, 0.63, 0.64], 0.0),
                Kind::Dirt => ([0.13, 0.11, 0.09], 0.0),
                Kind::Snow => ([0.70, 0.71, 0.73], 0.0),
                Kind::Fire => ([1.0, 0.55, 0.15], 6.0),
                Kind::Flash => ([1.0, 0.85, 0.6], 30.0),
                Kind::Tracer => ([1.0, 0.7, 0.3], 20.0),
                Kind::Spark => ([1.0, 0.6, 0.2], 12.0),
            };
            let c = Color::srgba(rgb[0], rgb[1], rgb[2], alpha);
            mats.push(materials.add(StandardMaterial {
                base_color: c,
                base_color_texture: Some(tex.clone()),
                emissive: if emissive > 0.0 { LinearRgba::from(Color::srgb(rgb[0], rgb[1], rgb[2])) * emissive * alpha } else { LinearRgba::BLACK },
                alpha_mode: if emissive > 0.0 { AlphaMode::Add } else { AlphaMode::Blend },
                unlit: true,
                double_sided: true,
                cull_mode: None,
                perceptual_roughness: 1.0,
                fog_enabled: true,
                ..default()
            }));
        }
    }
    let quad = meshes.add(Rectangle::new(1.0, 1.0));
    let sphere = meshes.add(Sphere::new(1.0).mesh().ico(2).unwrap());
    let crater_flash = materials.add(StandardMaterial { base_color: Color::srgb(1.0, 0.8, 0.5), emissive: LinearRgba::rgb(40.0, 22.0, 8.0), unlit: true, ..default() });
    commands.insert_resource(FxAssets { quad, mats, crater_flash, sphere, count: 0 });
}

#[derive(Component)]
pub struct Particle {
    kind: Kind,
    vel: Vec3,
    age: f32,
    life: f32,
    size0: f32,
    size1: f32,
    alpha: f32,
    drag: f32,
    gravity: f32,
    buoyancy: f32,
    step: usize,
    /// Stretch along velocity (tracers, sparks).
    streak: f32,
}

pub const MAX_PARTICLES: usize = 9000;

pub struct Spawn {
    pub kind: Kind,
    pub pos: Vec3,
    pub vel: Vec3,
    pub life: f32,
    pub size0: f32,
    pub size1: f32,
    pub alpha: f32,
    pub drag: f32,
    pub gravity: f32,
    pub buoyancy: f32,
    pub streak: f32,
}

impl Default for Spawn {
    fn default() -> Self {
        Self { kind: Kind::GreySmoke, pos: Vec3::ZERO, vel: Vec3::ZERO, life: 3.0, size0: 1.0, size1: 3.0, alpha: 0.8, drag: 0.5, gravity: 0.0, buoyancy: 0.0, streak: 0.0 }
    }
}

pub fn spawn(commands: &mut Commands, fx: &mut FxAssets, s: Spawn) {
    if fx.count >= MAX_PARTICLES {
        return;
    }
    fx.count += 1;
    commands.spawn((
        Mesh3d(fx.quad.clone()),
        MeshMaterial3d(fx.mat(s.kind, s.alpha)),
        Transform::from_translation(s.pos).with_scale(Vec3::splat(s.size0)),
        NotShadowCaster,
        Particle { kind: s.kind, vel: s.vel, age: 0.0, life: s.life, size0: s.size0, size1: s.size1, alpha: s.alpha, drag: s.drag, gravity: s.gravity, buoyancy: s.buoyancy, step: usize::MAX, streak: s.streak },
    ));
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    wind: Res<Wind>,
    mut fx: ResMut<FxAssets>,
    cam: Single<&GlobalTransform, With<MainCamera>>,
    mut q: Query<(Entity, &mut Particle, &mut Transform, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let dt = crate::sim_dt(&time);
    let cam_pos = cam.translation();
    let cam_rot = cam.compute_transform().rotation;
    let mut alive = 0;
    for (e, mut p, mut tf, mut mat) in &mut q {
        p.age += dt;
        if p.age >= p.life {
            commands.entity(e).despawn();
            continue;
        }
        alive += 1;
        let t = p.age / p.life;
        let windward = wind.0 * (1.0 - (-p.age * 0.5).exp()) * (p.buoyancy > 0.0 || p.kind == Kind::WhiteSmoke) as i32 as f32;
        let vel = p.vel;
        p.vel = vel * (1.0 - p.drag * dt).max(0.0) + Vec3::Y * (p.buoyancy - p.gravity) * dt;
        tf.translation += (p.vel + windward) * dt;
        let size = p.size0 + (p.size1 - p.size0) * (1.0 - (1.0 - t) * (1.0 - t));
        // Fade in fast; fade out slowly, so old smoke thins into a wide veil rather than vanishing.
        let a = p.alpha * (t / 0.05).min(1.0) * (1.0 - t).powf(1.8) * (1.0 - 0.45 * t);
        let step = ((a * (STEPS - 1) as f32).round() as usize).min(STEPS - 1);
        if step != p.step {
            p.step = step;
            mat.0 = fx.mat(p.kind, a);
        }
        if p.streak > 0.0 && p.vel.length_squared() > 1.0 {
            // Stretched along the motion, facing the camera as best it can.
            let dir = p.vel.normalize();
            let to_cam = (cam_pos - tf.translation).normalize_or_zero();
            let side = dir.cross(to_cam).normalize_or_zero();
            let normal = side.cross(dir);
            tf.rotation = Quat::from_mat3(&Mat3::from_cols(side, dir, normal));
            tf.scale = Vec3::new(size, size * p.streak, 1.0);
        } else {
            tf.rotation = cam_rot;
            tf.scale = Vec3::splat(size);
        }
    }
    fx.count = alive;
}

/// A long-lived source: a burning wreck's fire and smoke column, a vehicle's exhaust smoke screen.
#[derive(Component)]
pub struct Emitter {
    pub kind: EmitterKind,
    pub rate: f32,
    pub timer: f32,
    pub remaining: f32,
    pub offset: Vec3,
}

impl Emitter {
    /// `rate` puffs a second for `secs` seconds, from `offset` in the owner's frame.
    pub fn new(kind: EmitterKind, rate: f32, secs: f32, offset: Vec3) -> Self {
        Self { kind, rate, timer: 0.0, remaining: secs, offset }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EmitterKind {
    WreckFire,
    Smoulder,
    SmokeScreen,
    Exhaust,
}

fn update_emitters(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxAssets>,
    mut rng: ResMut<FxRng>,
    mut q: Query<(Entity, &mut Emitter, &GlobalTransform)>,
) {
    let dt = crate::sim_dt(&time);
    let load = fx.count as f32 / MAX_PARTICLES as f32;
    for (e, mut em, gt) in &mut q {
        em.remaining -= dt;
        if em.remaining <= 0.0 {
            if em.kind == EmitterKind::WreckFire {
                // The fire burns down to a smoulder that lasts the rest of the day.
                em.kind = EmitterKind::Smoulder;
                em.rate = 1.2;
                em.remaining = 900.0;
            } else {
                commands.entity(e).remove::<Emitter>();
                continue;
            }
        }
        em.timer -= dt;
        let rate = em.rate * (1.0 - load * 0.8);
        while em.timer <= 0.0 {
            em.timer += 1.0 / rate.max(0.05);
            let r = &mut rng.0;
            let base = gt.transform_point(em.offset);
            let jitter = Vec3::new(r.random_range(-1.0..1.0), 0.0, r.random_range(-1.0..1.0));
            match em.kind {
                EmitterKind::WreckFire => {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::BlackSmoke, pos: base + jitter + Vec3::Y * 1.5, vel: Vec3::Y * r.random_range(3.0..6.0), life: r.random_range(45.0..80.0), size0: 2.5, size1: r.random_range(35.0..60.0), alpha: 0.62, drag: 0.15, buoyancy: 0.35, ..default() });
                    if r.random_bool(0.7) {
                        spawn(&mut commands, &mut fx, Spawn { kind: Kind::Fire, pos: base + jitter * 0.8 + Vec3::Y * r.random_range(0.8..2.0), vel: Vec3::Y * r.random_range(1.5..3.5), life: r.random_range(0.4..0.9), size0: r.random_range(1.2..2.4), size1: 0.6, alpha: 0.9, ..default() });
                    }
                }
                EmitterKind::Smoulder => {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::GreySmoke, pos: base + jitter + Vec3::Y, vel: Vec3::Y * r.random_range(1.0..2.5), life: r.random_range(30.0..50.0), size0: 1.5, size1: r.random_range(15.0..28.0), alpha: 0.45, drag: 0.2, buoyancy: 0.2, ..default() });
                }
                EmitterKind::SmokeScreen => {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::WhiteSmoke, pos: base + jitter, vel: Vec3::new(r.random_range(-1.0..1.0), r.random_range(0.3..1.0), r.random_range(-1.0..1.0)), life: r.random_range(60.0..100.0), size0: 3.0, size1: r.random_range(35.0..60.0), alpha: 0.7, drag: 0.3, ..default() });
                }
                EmitterKind::Exhaust => {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::GreySmoke, pos: base, vel: Vec3::Y * 1.0, life: 2.5, size0: 0.6, size1: 2.5, alpha: 0.35, ..default() });
                }
            }
        }
    }
}

/// A short point light for blasts and muzzle flashes.
#[derive(Component)]
pub struct Flash {
    pub age: f32,
    pub life: f32,
    pub intensity: f32,
}

fn update_flashes(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Flash, &mut PointLight, Option<&mut Transform>)>) {
    for (e, mut f, mut light, tf) in &mut q {
        f.age += time.delta_secs();
        if f.age >= f.life {
            commands.entity(e).despawn();
            continue;
        }
        let k = 1.0 - f.age / f.life;
        light.intensity = f.intensity * k * k;
        if let Some(mut tf) = tf
            && tf.scale.x > 0.01
        {
            tf.scale *= 0.9;
        }
    }
}

pub fn flash(commands: &mut Commands, pos: Vec3, intensity: f32, range: f32, life: f32) {
    commands.spawn((
        PointLight { intensity, range, color: Color::srgb(1.0, 0.75, 0.45), shadow_maps_enabled: false, ..default() },
        Transform::from_translation(pos),
        Flash { age: 0.0, life, intensity },
    ));
}

// ---------------------------------------------------------------------------------------------------------
// Composite effects

pub fn explosion(commands: &mut Commands, fx: &mut FxAssets, rng: &mut SmallRng, at: Vec3, size: f32, snow: bool) {
    flash(commands, at + Vec3::Y * 2.0, 4.0e7 * size, 120.0 * size, 0.35);
    commands.spawn((
        Mesh3d(fx.sphere.clone()),
        MeshMaterial3d(fx.crater_flash.clone()),
        Transform::from_translation(at + Vec3::Y).with_scale(Vec3::splat(3.0 * size)),
        NotShadowCaster,
        PointLight { intensity: 0.0, range: 1.0, ..default() },
        Flash { age: 0.0, life: 0.12, intensity: 0.0 },
    ));
    // 0. The fireball: a fraction of a second of white-orange heat at the burst.
    for _ in 0..(6.0 * size) as u32 {
        let d = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.0..1.0), rng.random_range(-1.0..1.0));
        spawn(commands, fx, Spawn { kind: Kind::Fire, pos: at + Vec3::Y * 1.5 + d * size, vel: d * rng.random_range(4.0..12.0), life: rng.random_range(0.12..0.3), size0: 3.0 * size, size1: 6.5 * size, alpha: 1.0, drag: 3.0, ..default() });
    }
    // 1. Spikes: frozen earth thrown out in fast dark jets, the splash silhouette of a hit in snow.
    for _ in 0..(14.0 * size) as u32 {
        let dir = Vec3::new(rng.random_range(-0.65..0.65), 1.0, rng.random_range(-0.65..0.65)).normalize();
        spawn(commands, fx, Spawn { kind: Kind::Dirt, pos: at + Vec3::Y * 0.5, vel: dir * rng.random_range(30.0..62.0) * size.sqrt(), life: rng.random_range(0.6..1.2), size0: 0.7 * size, size1: 1.4 * size, alpha: 0.95, drag: 0.9, gravity: 9.8, streak: 4.5, ..default() });
    }
    // 2. The column: dark earth heaving up and collapsing back.
    for _ in 0..(14.0 * size) as u32 {
        let dir = Vec3::new(rng.random_range(-0.35..0.35), 1.0, rng.random_range(-0.35..0.35)).normalize();
        spawn(commands, fx, Spawn { kind: Kind::Dirt, pos: at + Vec3::Y * 0.5, vel: dir * rng.random_range(16.0..38.0) * size.sqrt(), life: rng.random_range(1.8..3.4), size0: 2.2 * size, size1: 9.0 * size, alpha: 0.95, drag: 1.4, gravity: 9.0, ..default() });
    }
    // 3. Clods: bits of earth arcing out and falling, pocking the snow round the crater.
    for _ in 0..(18.0 * size) as u32 {
        let a = rng.random_range(0.0..std::f32::consts::TAU);
        let v = Vec3::new(a.cos(), rng.random_range(0.6..1.6), a.sin()).normalize() * rng.random_range(10.0..28.0) * size.sqrt();
        spawn(commands, fx, Spawn { kind: Kind::Dirt, pos: at + Vec3::Y, vel: v, life: rng.random_range(1.4..2.8), size0: 0.45, size1: 0.35, alpha: 1.0, drag: 0.15, gravity: 9.8, streak: 2.0, ..default() });
    }
    // 4. The skirt: snow and dirt racing out low along the ground.
    for _ in 0..(14.0 * size) as u32 {
        let a = rng.random_range(0.0..std::f32::consts::TAU);
        let v = Vec3::new(a.cos(), rng.random_range(0.05..0.25), a.sin()) * rng.random_range(14.0..30.0) * size.sqrt();
        spawn(commands, fx, Spawn { kind: if snow { Kind::Snow } else { Kind::GreySmoke }, pos: at + Vec3::Y * 0.8, vel: v, life: rng.random_range(2.5..5.0), size0: 2.5 * size, size1: 12.0 * size, alpha: 0.75, drag: 2.2, gravity: 1.0, ..default() });
    }
    // 5. The cloud that hangs for half a minute: pale snow dust and grey smoke, drifting off with the wind.
    for _ in 0..(8.0 * size) as u32 {
        let k = if rng.random_bool(0.3) { Kind::GreySmoke } else { Kind::Snow };
        spawn(commands, fx, Spawn { kind: k, pos: at + Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(2.0..9.0), rng.random_range(-3.0..3.0)), vel: Vec3::new(rng.random_range(-2.0..2.0), rng.random_range(1.0..3.5), rng.random_range(-2.0..2.0)), life: rng.random_range(22.0..40.0), size0: 6.0 * size, size1: 32.0 * size, alpha: if k == Kind::Snow { 0.5 } else { 0.4 }, drag: 0.6, buoyancy: 0.15, ..default() });
    }
    for _ in 0..(8.0 * size) as u32 {
        let dir = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.3..1.2), rng.random_range(-1.0..1.0)).normalize();
        spawn(commands, fx, Spawn { kind: Kind::Spark, pos: at + Vec3::Y, vel: dir * rng.random_range(25.0..60.0), life: rng.random_range(0.3..0.8), size0: 0.25, size1: 0.1, alpha: 1.0, drag: 0.5, gravity: 9.8, streak: 6.0, ..default() });
    }
}

/// Thermobaric: a big orange fireball, then a dense grey-black mushroom.
pub fn thermobaric(commands: &mut Commands, fx: &mut FxAssets, rng: &mut SmallRng, at: Vec3) {
    flash(commands, at + Vec3::Y * 6.0, 2.5e8, 400.0, 0.7);
    for _ in 0..18 {
        let dir = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.0..0.9), rng.random_range(-1.0..1.0)).normalize();
        spawn(commands, fx, Spawn { kind: Kind::Fire, pos: at + Vec3::Y * 3.0, vel: dir * rng.random_range(10.0..22.0), life: rng.random_range(0.6..1.3), size0: 8.0, size1: 18.0, alpha: 1.0, drag: 2.5, ..default() });
    }
    for _ in 0..12 {
        spawn(commands, fx, Spawn { kind: Kind::BlackSmoke, pos: at + Vec3::Y * 5.0, vel: Vec3::new(rng.random_range(-6.0..6.0), rng.random_range(4.0..14.0), rng.random_range(-6.0..6.0)), life: rng.random_range(10.0..18.0), size0: 10.0, size1: 30.0, alpha: 0.75, drag: 0.5, buoyancy: 0.6, ..default() });
    }
    explosion(commands, fx, rng, at, 1.6, true);
}

pub fn muzzle(commands: &mut Commands, fx: &mut FxAssets, rng: &mut SmallRng, at: Vec3, dir: Vec3, big: bool) {
    let s = if big { 1.0 } else { 0.3 };
    flash(commands, at, 6.0e6 * s, 60.0 * s, 0.08);
    spawn(commands, fx, Spawn { kind: Kind::Flash, pos: at + dir * s, vel: dir * 5.0, life: 0.08, size0: 3.0 * s, size1: 4.0 * s, alpha: 1.0, ..default() });
    if big {
        for _ in 0..6 {
            spawn(commands, fx, Spawn { kind: Kind::GreySmoke, pos: at + dir * 2.0, vel: dir * rng.random_range(5.0..15.0) + Vec3::Y * 1.5, life: rng.random_range(2.0..4.0), size0: 1.5, size1: 6.0, alpha: 0.6, drag: 1.5, ..default() });
            spawn(commands, fx, Spawn { kind: Kind::Snow, pos: at - Vec3::Y * 1.5, vel: Vec3::new(rng.random_range(-6.0..6.0), 1.0, rng.random_range(-6.0..6.0)), life: 1.5, size0: 2.0, size1: 6.0, alpha: 0.6, drag: 1.5, ..default() });
        }
    }
}

pub fn tracer(commands: &mut Commands, fx: &mut FxAssets, from: Vec3, to: Vec3, speed: f32) {
    let d = to - from;
    let len = d.length();
    spawn(commands, fx, Spawn { kind: Kind::Tracer, pos: from, vel: d / len * speed, life: len / speed, size0: 0.35, size1: 0.35, alpha: 1.0, drag: 0.0, streak: 18.0, ..default() });
}

/// A track-throw of snow behind a moving vehicle.
pub fn spray(commands: &mut Commands, fx: &mut FxAssets, rng: &mut SmallRng, at: Vec3, vel: Vec3) {
    spawn(commands, fx, Spawn { kind: Kind::Snow, pos: at + Vec3::Y * 0.5, vel: -vel * 0.2 + Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.5..1.5), rng.random_range(-1.0..1.0)), life: rng.random_range(1.5..2.5), size0: 1.2, size1: 4.0, alpha: 0.45, drag: 1.0, ..default() });
}
