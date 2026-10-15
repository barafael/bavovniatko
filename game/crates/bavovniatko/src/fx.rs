//! Particles, flashes, fire and smoke. A small CPU particle system: camera-facing quads that share a handful of
//! materials (one per kind × alpha step), so everything batches.

use crate::camera::MainCamera;
use crate::diorama::diorama;
use crate::geo::{fbm, value_noise};
use crate::units::Side;
use bevy::{asset::RenderAssetUsages, light::NotShadowCaster, mesh::{Indices, PrimitiveTopology}, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use strum::VariantArray;

pub struct FxPlugin;

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FxRng(SmallRng::seed_from_u64(crate::dev::seeded(20230206))))
            .insert_resource(Wind(day_wind()))
            .init_resource::<Smoke>()
            .insert_resource(FxLight(1.0))
            .add_systems(Startup, setup_fx)
            .add_systems(Update, (update_particles, update_emitters, grass_fires, update_flashes, light_particles).chain().in_set(crate::Step::Effects));
    }
}

#[derive(Resource)]
pub struct FxRng(pub SmallRng);

/// Wind drifts every plume the same way, as the vector it blows towards (north is -z). ERA5 at the dioramas
/// (design/next-items.md §2.1): on 6 February from the north-east, 4 m/s at dawn and 5.5 by midday, so towards
/// the south-west; on 8 June from the east at night turning south-east by day, about 2 m/s at dawn, so towards
/// the west-north-west. It is the look's alone: the gameplay smoke (`Smoke`) does not drift.
#[derive(Resource)]
pub struct Wind(pub Vec3);

/// The day's wind (see `Wind`); the ground painter reads it too, for the drifts in the belts' lee.
pub fn day_wind() -> Vec3 {
    if crate::diorama::diorama().summer() { Vec3::new(-2.0, 0.0, -0.7) } else { Vec3::new(-3.2, 0.0, 3.2) }
}

/// Smoke on the ground thick enough to hide a vehicle from a sight: grenades and smoke shells. (The particles are
/// what you see; this is what a gunner cannot see through.)
#[derive(Resource, Default)]
pub struct Smoke {
    clouds: Vec<(Vec2, f32, f64)>,
}

impl Smoke {
    pub fn add(&mut self, at: Vec2, radius: f32, until: f64) {
        self.clouds.push((at, radius, until));
    }

    /// Does smoke lie across the line from `a` to `b` at game time `now`?
    pub fn blocks(&self, a: Vec2, b: Vec2, now: f64) -> bool {
        self.clouds.iter().any(|&(c, r, until)| {
            if now > until {
                return false;
            }
            let ab = b - a;
            let t = if ab.length_squared() > 0.0 { ((c - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
            (a + ab * t).distance(c) < r
        })
    }

    /// Is `p` inside smoke?
    pub fn covers(&self, p: Vec2, now: f64) -> bool {
        self.blocks(p, p, now)
    }
}

/// What a particle is made of: each kind has its colour, and in [`update_particles`] its own way of moving with
/// the wind (§2.2 of design/next-items.md: hot smoke rises before it bends; dust stays low; fog goes as a sheet).
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::VariantArray)]
pub enum Kind {
    /// Diesel, rubber and oil burning: jet-black, oily, strongly buoyant.
    BlackSmoke,
    GreySmoke,
    WhiteSmoke,
    /// A diesel-into-the-exhaust screen: a milky fog that neither rises nor sinks, moving with the wind as a sheet.
    Fog,
    /// Smoke-grenade smoke: grey-black, hugging the ground, fattening in humid air.
    Hc,
    /// Pure white: coolant steam, white phosphorus.
    Steam,
    /// Frozen earth: dark clods and the column of a winter burst.
    Dirt,
    /// Dry summer earth: the khaki column and the hanging haze of a June burst.
    Dust,
    /// Lifted snow (in summer, the pale dust a track throws).
    Snow,
    Fire,
    /// Burning propellant: the white-yellow jets and sparks of a cook-off.
    Propellant,
    Flash,
    /// A Ukrainian tracer (strontium red).
    Tracer,
    /// A Russian tracer (barium green).
    TracerGreen,
    Spark,
}

impl Kind {
    /// How fully and how soon the wind carries this kind: fog and grenade smoke go with it at once, hot smoke
    /// rises first and bends over, dust and clods barely feel it.
    fn wind(self) -> (f32, f32) {
        match self {
            Kind::Fog | Kind::Hc | Kind::Steam | Kind::WhiteSmoke => (1.0, 2.0),
            Kind::BlackSmoke | Kind::GreySmoke => (0.9, 0.5),
            Kind::Snow | Kind::Dust => (0.6, 1.0),
            Kind::Dirt => (0.3, 1.0),
            _ => (0.0, 0.0),
        }
    }

    /// How much the air's eddies wobble it.
    fn turbulence(self) -> f32 {
        match self {
            Kind::BlackSmoke | Kind::GreySmoke | Kind::WhiteSmoke | Kind::Steam => 1.0,
            Kind::Fog | Kind::Hc => 0.5,
            Kind::Snow | Kind::Dust => 0.4,
            _ => 0.0,
        }
    }
}

const STEPS: usize = 8;

/// How bright the scene is lit, for the unlit particles (1 in daylight). Smoke that ignores the light glows white
/// in the blue hour; this dims it with everything else.
#[derive(Resource)]
pub struct FxLight(pub f32);

#[derive(Resource)]
pub struct FxAssets {
    /// The effects' own random stream: what a puff looks like never touches what the day decides.
    pub rng: SmallRng,
    /// Four puff shapes, one per quadrant of the atlas, so no two puffs are the same stamp.
    quads: [Handle<Mesh>; 4],
    mats: Vec<Handle<StandardMaterial>>,
    /// Each non-emissive material's own colour, to scale by [`FxLight`].
    base: Vec<Option<Color>>,
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

/// Four puffs in a 2×2 atlas: three round ones of different grain, one wispy and torn.
fn puff_texture() -> Image {
    const N: usize = 64;
    let mut d = vec![0u8; 4 * N * N * 4];
    for (q, (seed, scale, power, tear)) in [(5u32, 2.5f32, 1.4f32, 0.35f32), (13, 3.5, 1.2, 0.4), (29, 1.8, 1.7, 0.3), (47, 4.0, 1.0, 0.6)].into_iter().enumerate() {
        let (ox, oy) = ((q % 2) * N, (q / 2) * N);
        for y in 0..N {
            for x in 0..N {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5) / N as f32 * 2.0 - 1.0;
                let r = p.length();
                let n = fbm(p.x * scale, p.y * scale, 3, seed) * tear;
                let a = (1.0 - (r + n).clamp(0.0, 1.0)).powf(power);
                let o = ((oy + y) * 2 * N + ox + x) * 4;
                d[o..o + 4].copy_from_slice(&[255, 255, 255, (a * 255.0) as u8]);
            }
        }
    }
    Image::new(Extent3d { width: 2 * N as u32, height: 2 * N as u32, depth_or_array_layers: 1 }, TextureDimension::D2, d, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default())
}

/// A unit quad showing one quadrant of the atlas.
fn quad(q: usize) -> Mesh {
    let (u0, v0) = ((q % 2) as f32 * 0.5, (q / 2) as f32 * 0.5);
    let mut m = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    m.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.5, 0.5, 0.0], [-0.5, 0.5, 0.0]]);
    m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4]);
    m.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[u0, v0 + 0.5], [u0 + 0.5, v0 + 0.5], [u0 + 0.5, v0], [u0, v0]]);
    m.insert_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));
    m
}


fn setup_fx(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, mut images: ResMut<Assets<Image>>) {
    let tex = images.add(puff_texture());
    let mut mats = Vec::new();
    let mut base = Vec::new();
    for &k in Kind::VARIANTS {
        for a in 0..STEPS {
            let alpha = (a as f32 + 1.0) / STEPS as f32;
            // Unlit, pre-shaded for an overcast sky: billboards lit by a low sun look wrong from behind.
            let (rgb, emissive): ([f32; 3], f32) = match k {
                // The burning-vehicle columns read dark grey in the footage, never pitch black: the drone's
                // camera lifts them, and stacked puffs crush towards black on their own.
                Kind::BlackSmoke => ([0.11, 0.105, 0.10], 0.0),
                Kind::GreySmoke => ([0.38, 0.38, 0.38], 0.0),
                Kind::WhiteSmoke => ([0.62, 0.63, 0.64], 0.0),
                Kind::Fog => ([0.68, 0.69, 0.70], 0.0),
                Kind::Hc => ([0.44, 0.44, 0.45], 0.0),
                Kind::Steam => ([0.84, 0.85, 0.86], 0.0),
                Kind::Dirt => ([0.13, 0.11, 0.09], 0.0),
                Kind::Dust => ([0.44, 0.38, 0.28], 0.0),
                // In summer the same particles are dust: pale, warm, and visible from kilometres.
                Kind::Snow => if crate::diorama::diorama().summer() { ([0.60, 0.55, 0.46], 0.0) } else { ([0.70, 0.71, 0.73], 0.0) },
                Kind::Fire => ([1.0, 0.55, 0.15], 6.0),
                Kind::Propellant => ([1.0, 0.95, 0.75], 10.0),
                Kind::Flash => ([1.0, 0.85, 0.6], 30.0),
                Kind::Tracer => ([1.0, 0.45, 0.3], 20.0),
                Kind::TracerGreen => ([0.55, 1.0, 0.45], 20.0),
                Kind::Spark => ([1.0, 0.6, 0.2], 12.0),
            };
            let c = Color::srgba(rgb[0], rgb[1], rgb[2], alpha);
            base.push((emissive == 0.0).then_some(c));
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
    let quads = [meshes.add(quad(0)), meshes.add(quad(1)), meshes.add(quad(2)), meshes.add(quad(3))];
    let sphere = meshes.add(Sphere::new(1.0).mesh().ico(2).unwrap());
    let crater_flash = materials.add(StandardMaterial { base_color: Color::srgb(1.0, 0.8, 0.5), emissive: LinearRgba::rgb(40.0, 22.0, 8.0), unlit: true, ..default() });
    commands.insert_resource(FxAssets { rng: SmallRng::seed_from_u64(0x2023_0206), quads, mats, base, crater_flash, sphere, count: 0 });
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
    /// The puff's own turn about the line of sight, and how fast it keeps turning.
    spin: f32,
    spin_rate: f32,
    /// Mirrored, so the atlas's four shapes become eight.
    mirror: f32,
    /// Its own eddy: where in the turbulence field it samples.
    eddy: f32,
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
    let (shape, spin, spin_rate, mirror, eddy) = if s.streak > 0.0 {
        (0, 0.0, 0.0, 1.0, 0.0)
    } else {
        let r = &mut fx.rng;
        (r.random_range(0..4), r.random_range(0.0..std::f32::consts::TAU), r.random_range(-0.25..0.25), if r.random_bool(0.5) { 1.0 } else { -1.0 }, r.random_range(0.0..100.0))
    };
    commands.spawn((
        Mesh3d(fx.quads[shape].clone()),
        MeshMaterial3d(fx.mat(s.kind, s.alpha)),
        Transform::from_translation(s.pos).with_scale(Vec3::splat(s.size0)),
        NotShadowCaster,
        Particle { kind: s.kind, vel: s.vel, age: 0.0, life: s.life, size0: s.size0, size1: s.size1, alpha: s.alpha, drag: s.drag, gravity: s.gravity, buoyancy: s.buoyancy, step: usize::MAX, streak: s.streak, spin, spin_rate, mirror, eddy },
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
    let t = time.elapsed_secs();
    let cam_pos = cam.translation();
    let cam_rot = cam.compute_transform().rotation;
    // The wind comes and goes: a gust envelope shared by every plume (the drone feels the same one).
    let gust = 0.75 + 0.5 * value_noise(t * 0.07, 3.3, 63) + 0.3 * (value_noise(t * 0.5, 9.1, 64) - 0.5);
    let mut alive = 0;
    for (e, mut p, mut tf, mut mat) in &mut q {
        p.age += dt;
        if p.age >= p.life {
            commands.entity(e).despawn();
            continue;
        }
        alive += 1;
        let k = p.age / p.life;
        let (wind_k, wind_rate) = p.kind.wind();
        let windward = wind.0 * gust * wind_k * (1.0 - (-p.age * wind_rate).exp());
        // Hot smoke rises, then its heat is spent and it goes with the wind (a plume's bend-over).
        let buoyancy = p.buoyancy * (-p.age / 20.0).exp();
        let vel = p.vel;
        p.vel = vel * (1.0 - p.drag * dt).max(0.0) + Vec3::Y * (buoyancy - p.gravity) * dt;
        // The air's eddies: a slow field each particle samples at its own place, so plumes writhe.
        let turb = p.kind.turbulence();
        let eddy = if turb > 0.0 {
            let (x, z) = (tf.translation.x * 0.04 + p.eddy, tf.translation.z * 0.04);
            Vec3::new(value_noise(x + t * 0.25, z, 71) - 0.5, (value_noise(x, z + t * 0.2, 72) - 0.5) * 0.6, value_noise(x - t * 0.25, z + 50.0, 73) - 0.5) * 1.4 * turb
        } else {
            Vec3::ZERO
        };
        tf.translation += (p.vel + windward + eddy) * dt;
        let size = p.size0 + (p.size1 - p.size0) * (1.0 - (1.0 - k) * (1.0 - k));
        // Fade in fast; fade out slowly, so old smoke thins into a wide veil rather than vanishing.
        let a = p.alpha * (k / 0.05).min(1.0) * (1.0 - k).powf(1.8) * (1.0 - 0.45 * k);
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
            tf.rotation = cam_rot * Quat::from_rotation_z(p.spin + p.age * p.spin_rate);
            tf.scale = Vec3::new(size * p.mirror, size, 1.0);
        }
    }
    fx.count = alive;
}


fn light_particles(light: Res<FxLight>, fx: Res<FxAssets>, mut materials: ResMut<Assets<StandardMaterial>>, mut last: Local<Option<f32>>) {
    if last.is_some_and(|l| (l - light.0).abs() < 0.01) {
        return;
    }
    *last = Some(light.0);
    for (h, c) in fx.mats.iter().zip(&fx.base) {
        if let (Some(c), Some(mut m)) = (c, materials.get_mut(h)) {
            let s = c.to_srgba();
            m.base_color = Color::srgba(s.red * light.0, s.green * light.0, s.blue * light.0, s.alpha);
        }
    }
}

/// A long-lived source: a burning wreck's fire and smoke column, a vehicle's exhaust smoke screen.
#[derive(Component)]
pub struct Emitter {
    pub kind: EmitterKind,
    pub rate: f32,
    /// Seconds to the next puff; set it ahead to delay the first.
    pub timer: f32,
    pub remaining: f32,
    pub secs: f32,
    pub offset: Vec3,
}

impl Emitter {
    /// `rate` puffs a second for `secs` seconds, from `offset` in the owner's frame.
    pub fn new(kind: EmitterKind, rate: f32, secs: f32, offset: Vec3) -> Self {
        Self { kind, rate, timer: 0.0, remaining: secs, secs, offset }
    }

    /// The same, starting after `delay` seconds.
    pub fn after(delay: f32, kind: EmitterKind, rate: f32, secs: f32, offset: Vec3) -> Self {
        Self { timer: delay, remaining: secs + delay, ..Self::new(kind, rate, secs, offset) }
    }

    /// Seconds since it started.
    fn age(&self) -> f32 {
        self.secs - self.remaining
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EmitterKind {
    /// The whole vehicle burning: flames twice the hull's height and a sooty column.
    WreckFire,
    Smoulder,
    /// Diesel injected into the exhaust: a milky fog pouring out low behind a moving vehicle.
    SmokeScreen,
    Exhaust,
    /// A fire in the engine bay: white oil and coolant steam, going grey, from the rear deck.
    EngineSmoke,
    /// A smoke grenade burning on the ground: grey-black puffs that merge into a low wall.
    Hc,
    /// A white-phosphorus wedge or a smoke canister on the ground: a pure white bank that rises a little.
    Wp,
    /// Fuel burning in a pool on the ground beside a hull: low flames, a thinner black plume, steam off the
    /// snow it melts.
    PoolFire,
}

fn update_emitters(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxAssets>,
    mut q: Query<(Entity, &mut Emitter, &GlobalTransform)>,
) {
    let dt = crate::sim_dt(&time);
    let load = fx.count as f32 / MAX_PARTICLES as f32;
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
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
            let r = &mut rng;
            let base = gt.transform_point(em.offset);
            let jitter = Vec3::new(r.random_range(-1.0..1.0), 0.0, r.random_range(-1.0..1.0));
            match em.kind {
                EmitterKind::WreckFire => {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::BlackSmoke, pos: base + jitter + Vec3::Y * 1.5, vel: Vec3::Y * r.random_range(3.0..6.0), life: r.random_range(45.0..80.0), size0: 3.0, size1: r.random_range(40.0..65.0), alpha: 0.5, drag: 0.15, buoyancy: 0.4, ..default() });
                    // Flames big enough to read in daylight: the footage shows a hull as a torch.
                    if r.random_bool(0.8) {
                        spawn(&mut commands, &mut fx, Spawn { kind: Kind::Fire, pos: base + jitter * 1.2 + Vec3::Y * r.random_range(0.8..2.2), vel: Vec3::Y * r.random_range(2.0..5.0), life: r.random_range(0.5..1.0), size0: r.random_range(2.0..4.0), size1: 1.2, alpha: 0.95, ..default() });
                    }
                }
                EmitterKind::EngineSmoke => {
                    // White steam first, grey as the oil takes, black once the fuel and rubber are in it.
                    let kind = match em.age() {
                        a if a < 45.0 => Kind::Steam,
                        a if a < 120.0 => Kind::GreySmoke,
                        _ => Kind::BlackSmoke,
                    };
                    spawn(&mut commands, &mut fx, Spawn { kind, pos: base + jitter * 0.6, vel: Vec3::Y * r.random_range(1.5..3.0), life: r.random_range(12.0..20.0), size0: 1.0, size1: r.random_range(7.0..12.0), alpha: 0.6, drag: 0.3, buoyancy: 0.25, ..default() });
                    if r.random_bool(0.3) {
                        spawn(&mut commands, &mut fx, Spawn { kind: Kind::Fire, pos: base + jitter * 0.4, vel: Vec3::Y * 1.5, life: r.random_range(0.3..0.6), size0: 0.8, size1: 0.4, alpha: 0.9, ..default() });
                    }
                }
                EmitterKind::Smoulder => {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::GreySmoke, pos: base + jitter + Vec3::Y, vel: Vec3::Y * r.random_range(1.0..2.5), life: r.random_range(30.0..50.0), size0: 1.5, size1: r.random_range(15.0..28.0), alpha: 0.45, drag: 0.2, buoyancy: 0.2, ..default() });
                }
                EmitterKind::SmokeScreen => {
                    // A fog, not a fire: it neither rises nor sinks, and the wind lays it out as a ribbon.
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::Fog, pos: base + jitter * 0.6, vel: Vec3::new(r.random_range(-0.8..0.8), r.random_range(0.1..0.5), r.random_range(-0.8..0.8)), life: r.random_range(60.0..100.0), size0: 2.5, size1: r.random_range(16.0..26.0), alpha: 0.7, drag: 0.4, ..default() });
                }
                EmitterKind::Hc => {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::Hc, pos: base + jitter * 0.8 + Vec3::Y * 0.5, vel: Vec3::new(r.random_range(-1.0..1.0), r.random_range(0.4..1.0), r.random_range(-1.0..1.0)), life: r.random_range(50.0..80.0), size0: 1.5, size1: r.random_range(7.0..10.0), alpha: 0.6, drag: 0.5, ..default() });
                }
                EmitterKind::Wp => {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::Steam, pos: base + jitter + Vec3::Y * 0.5, vel: Vec3::new(r.random_range(-1.5..1.5), r.random_range(0.5..1.5), r.random_range(-1.5..1.5)), life: r.random_range(60.0..100.0), size0: 3.0, size1: r.random_range(20.0..32.0), alpha: 0.75, drag: 0.5, buoyancy: 0.3, ..default() });
                }
                EmitterKind::Exhaust => {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::GreySmoke, pos: base, vel: Vec3::Y * 1.0, life: 2.5, size0: 0.6, size1: 2.5, alpha: 0.35, ..default() });
                }
                EmitterKind::PoolFire => {
                    // 2–6 m² of diesel: flames 1.5–3 m, licking low over the ground.
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::Fire, pos: base + jitter * 1.1 + Vec3::Y * r.random_range(0.3..0.9), vel: Vec3::Y * r.random_range(1.5..3.5), life: r.random_range(0.4..0.8), size0: r.random_range(1.2..2.2), size1: 0.8, alpha: 0.95, ..default() });
                    if r.random_bool(0.35) {
                        spawn(&mut commands, &mut fx, Spawn { kind: Kind::BlackSmoke, pos: base + jitter + Vec3::Y * 1.5, vel: Vec3::Y * r.random_range(2.0..4.0), life: r.random_range(25.0..45.0), size0: 1.5, size1: r.random_range(16.0..26.0), alpha: 0.35, drag: 0.2, buoyancy: 0.3, ..default() });
                    }
                    // In the first minutes the snow round it melts and steams.
                    if !diorama().summer() && em.age() < 180.0 && r.random_bool(0.15) {
                        let a = r.random_range(0.0..std::f32::consts::TAU);
                        spawn(&mut commands, &mut fx, Spawn { kind: Kind::Steam, pos: base + Vec3::new(a.cos(), 0.2, a.sin()) * 2.2, vel: Vec3::Y * r.random_range(0.4..1.0), life: r.random_range(3.0..6.0), size0: 0.6, size1: 2.5, alpha: 0.4, drag: 0.5, ..default() });
                    }
                }
            }
        }
    }
    fx.rng = rng;
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

/// A shell burst (`size` 1 is a 155 mm round; `ground` for a burst in the field rather than on a hull). In
/// February a dark column of frozen clods with a white skirt of snow and a pale haze that clears in half a minute;
/// in June a khaki column of dry earth and a brown haze that hangs for minutes.
pub fn explosion(commands: &mut Commands, fx: &mut FxAssets, at: Vec3, size: f32, ground: bool) {
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    let rng = &mut rng;
    let summer = diorama().summer();
    let earth = if summer { Kind::Dust } else { Kind::Dirt };
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
    // 1. Spikes: earth thrown out in fast jets, the splash silhouette of a hit.
    for _ in 0..(14.0 * size) as u32 {
        let dir = Vec3::new(rng.random_range(-0.65..0.65), 1.0, rng.random_range(-0.65..0.65)).normalize();
        spawn(commands, fx, Spawn { kind: Kind::Dirt, pos: at + Vec3::Y * 0.5, vel: dir * rng.random_range(30.0..62.0) * size.sqrt(), life: rng.random_range(0.6..1.2), size0: 0.7 * size, size1: 1.4 * size, alpha: 0.95, drag: 0.9, gravity: 9.8, streak: 4.5, ..default() });
    }
    // 2. The column: earth heaving up 20–35 m and collapsing back; frozen it is dark and chunky, dry it is wide.
    for _ in 0..(14.0 * size) as u32 {
        let spread = if summer { 0.5 } else { 0.35 };
        let dir = Vec3::new(rng.random_range(-spread..spread), 1.0, rng.random_range(-spread..spread)).normalize();
        spawn(commands, fx, Spawn { kind: earth, pos: at + Vec3::Y * 0.5, vel: dir * rng.random_range(16.0..38.0) * size.sqrt(), life: rng.random_range(1.8..3.4), size0: 2.2 * size, size1: if summer { 11.0 } else { 9.0 } * size, alpha: 0.95, drag: 1.4, gravity: 9.0, ..default() });
    }
    // 3. Clods: bits of earth arcing out and falling, pocking the ground round the crater.
    for _ in 0..(18.0 * size) as u32 {
        let a = rng.random_range(0.0..std::f32::consts::TAU);
        let v = Vec3::new(a.cos(), rng.random_range(0.6..1.6), a.sin()).normalize() * rng.random_range(10.0..28.0) * size.sqrt();
        spawn(commands, fx, Spawn { kind: Kind::Dirt, pos: at + Vec3::Y, vel: v, life: rng.random_range(1.4..2.8), size0: 0.45, size1: 0.35, alpha: 1.0, drag: 0.15, gravity: 9.8, streak: 2.0, ..default() });
    }
    // 4. The skirt: snow (or dust) racing out low along the ground.
    for _ in 0..(14.0 * size) as u32 {
        let a = rng.random_range(0.0..std::f32::consts::TAU);
        let v = Vec3::new(a.cos(), rng.random_range(0.05..0.25), a.sin()) * rng.random_range(14.0..30.0) * size.sqrt();
        let kind = if !ground { Kind::GreySmoke } else if summer { Kind::Dust } else { Kind::Snow };
        spawn(commands, fx, Spawn { kind, pos: at + Vec3::Y * 0.8, vel: v, life: rng.random_range(2.5..5.0), size0: 2.5 * size, size1: 12.0 * size, alpha: 0.75, drag: 2.2, gravity: 1.0, ..default() });
    }
    // 5. The haze: in winter pale snow dust and grey smoke that clears in half a minute; in June a brown dust
    // cloud that hangs for minutes and drifts off with the wind.
    let (n, life, kinds): (u32, std::ops::Range<f32>, [Kind; 2]) = if summer { ((10.0 * size) as u32, 60.0..120.0, [Kind::Dust, Kind::GreySmoke]) } else { ((8.0 * size) as u32, 20.0..40.0, [Kind::Snow, Kind::GreySmoke]) };
    for _ in 0..n {
        let k = if rng.random_bool(0.3) { kinds[1] } else { kinds[0] };
        spawn(commands, fx, Spawn { kind: k, pos: at + Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(2.0..9.0), rng.random_range(-3.0..3.0)), vel: Vec3::new(rng.random_range(-2.0..2.0), rng.random_range(1.0..3.5), rng.random_range(-2.0..2.0)), life: rng.random_range(life.clone()), size0: 6.0 * size, size1: (if summer { 40.0 } else { 32.0 }) * size, alpha: if k == Kind::GreySmoke { 0.4 } else { 0.5 }, drag: 0.6, buoyancy: 0.15, ..default() });
    }
    for _ in 0..(8.0 * size) as u32 {
        let dir = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.3..1.2), rng.random_range(-1.0..1.0)).normalize();
        spawn(commands, fx, Spawn { kind: Kind::Spark, pos: at + Vec3::Y, vel: dir * rng.random_range(25.0..60.0), life: rng.random_range(0.3..0.8), size0: 0.25, size1: 0.1, alpha: 1.0, drag: 0.5, gravity: 9.8, streak: 6.0, ..default() });
    }
    fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
}


/// Charges cooking off in a burning hull: white-yellow jets of propellant through the hatches and the turret
/// ring, sparks, a flash. `strength` 1 is a tank's carousel going; a Bradley's rounds are a fraction of it.
pub fn cookoff(commands: &mut Commands, fx: &mut FxAssets, at: Vec3, strength: f32) {
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    let rng = &mut rng;
    flash(commands, at + Vec3::Y * 3.0, 5.0e7 * strength, 120.0, 0.4);
    for _ in 0..(6.0 * strength).ceil() as u32 {
        spawn(commands, fx, Spawn { kind: Kind::Propellant, pos: at + Vec3::Y * 2.0, vel: Vec3::new(rng.random_range(-2.5..2.5), rng.random_range(12.0..28.0) * strength.sqrt(), rng.random_range(-2.5..2.5)), life: rng.random_range(0.5..1.1), size0: rng.random_range(1.5..2.5), size1: rng.random_range(4.0..7.0) * strength.sqrt(), alpha: 1.0, drag: 1.2, ..default() });
    }
    for _ in 0..(15.0 * strength) as u32 {
        let dir = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.6..1.5), rng.random_range(-1.0..1.0)).normalize();
        spawn(commands, fx, Spawn { kind: Kind::Spark, pos: at + Vec3::Y * 2.0, vel: dir * rng.random_range(15.0..40.0), life: rng.random_range(0.8..2.5), size0: 0.3, size1: 0.12, alpha: 1.0, drag: 0.4, gravity: 9.8, streak: 5.0, ..default() });
    }
    for _ in 0..3 {
        spawn(commands, fx, Spawn { kind: Kind::WhiteSmoke, pos: at + Vec3::Y * 2.5, vel: Vec3::new(rng.random_range(-2.0..2.0), rng.random_range(4.0..9.0), rng.random_range(-2.0..2.0)), life: rng.random_range(6.0..12.0), size0: 2.0, size1: 12.0, alpha: 0.6, drag: 0.6, buoyancy: 0.5, ..default() });
    }
    fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
}

/// Reactive armour blocks going off under a hit (§2.3 of design/next-items.md): a chain of two to six bright
/// white-orange flashes a metre or two across on the hull and turret front (the roof under a top attack), a
/// pale puff of dust and smoke from each, and the plates flung off. ERA does not burn.
pub fn era_pops(commands: &mut Commands, fx: &mut FxAssets, hull: &Transform, top: bool) {
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    let rng = &mut rng;
    let n = rng.random_range(2..=6);
    for _ in 0..n {
        // Blocks on the glacis and the turret cheeks (the front is -z in the hull's frame), or on the roof.
        let local = if top {
            Vec3::new(rng.random_range(-1.2..1.2), rng.random_range(2.2..2.6), rng.random_range(-1.8..0.6))
        } else {
            Vec3::new(rng.random_range(-1.6..1.6), rng.random_range(1.0..2.3), rng.random_range(-3.4..-1.8))
        };
        let at = hull.transform_point(local);
        let out = (at - hull.translation).with_y(0.0).normalize_or_zero() + Vec3::Y * 0.3;
        flash(commands, at, 2.0e7, 60.0, 0.1);
        spawn(commands, fx, Spawn { kind: Kind::Flash, pos: at, vel: Vec3::ZERO, life: rng.random_range(0.08..0.16), size0: rng.random_range(2.2..3.4), size1: 1.5, alpha: 1.0, ..default() });
        spawn(commands, fx, Spawn { kind: Kind::Fire, pos: at + out * 0.4, vel: out * rng.random_range(5.0..10.0), life: rng.random_range(0.15..0.3), size0: rng.random_range(1.5..2.5), size1: 3.0, alpha: 1.0, drag: 3.0, ..default() });
        // The block's dust and explosive smoke: a pale puff a few metres across, gone in seconds.
        for _ in 0..2 {
            spawn(commands, fx, Spawn { kind: Kind::WhiteSmoke, pos: at + out * 0.6, vel: out * rng.random_range(2.0..5.0) + Vec3::Y * rng.random_range(0.5..2.0), life: rng.random_range(3.0..5.0), size0: 1.5, size1: rng.random_range(4.5..7.0), alpha: 0.7, drag: 0.8, ..default() });
        }
        for _ in 0..rng.random_range(1..=3) {
            let v = (out + Vec3::new(rng.random_range(-0.5..0.5), rng.random_range(0.2..0.9), rng.random_range(-0.5..0.5))) * rng.random_range(6.0..14.0);
            spawn(commands, fx, Spawn { kind: Kind::Dirt, pos: at, vel: v, life: rng.random_range(1.0..1.8), size0: 0.35, size1: 0.3, alpha: 1.0, drag: 0.2, gravity: 9.8, ..default() });
        }
    }
    fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
}

/// Fuel from a holed tank burning in a pool beside the hull, one burning hull in three (a fire that grew
/// quietly, one in eight): flames 1.5–3 m for 10–25 minutes, starting when the fuel has run out. In February
/// it melts a black ring in the snow; in June it may set the field alight (`GrassFire`).
pub fn pool_fire(commands: &mut Commands, fx: &mut FxAssets, paint: &mut crate::terrain::PaintQueue, hull: &Transform, violent: bool) {
    if !fx.rng.random_bool(if violent { 0.35 } else { 0.12 }) {
        return;
    }
    let side = if fx.rng.random_bool(0.5) { 1.0 } else { -1.0 };
    let local = Vec3::new(side * fx.rng.random_range(2.4..3.6), 0.0, fx.rng.random_range(-1.5..2.5));
    let at = crate::geo::ground(hull.transform_point(local).xz());
    let delay = fx.rng.random_range(10.0..60.0);
    let secs = fx.rng.random_range(600.0..1500.0);
    commands.spawn((Transform::from_translation(at), Emitter::after(delay, EmitterKind::PoolFire, 3.0, secs, Vec3::ZERO)));
    info!("pool fire at {:.0},{:.0}", at.x, at.z);
    paint.push(crate::terrain::Paint::Melt { at: at.xz(), radius: fx.rng.random_range(2.2..3.2) });
    if diorama().summer() && fx.rng.random_bool(0.6) {
        let speed = fx.rng.random_range(0.15..0.6);
        info!("grass fire from {:.0},{:.0}", at.x, at.z);
        commands.spawn(GrassFire { origin: at.xz(), front: Vec::new(), start_in: delay + fx.rng.random_range(20.0..90.0), speed, secs: fx.rng.random_range(420.0..1100.0), t: 0.0 });
    }
}

/// A grass fire walking downwind from a burning pool in June: a low front of flames 0.3–1 m high, advancing at
/// 0.1–1 m/s and widening as it goes, a pale smoke off it, black stubble behind. Each point of the front goes
/// out in time, on a road or track, or when the fire has burned its course.
#[derive(Component)]
pub struct GrassFire {
    origin: Vec2,
    /// The burning points: where each is and how long it has to burn.
    front: Vec<(Vec2, f32)>,
    start_in: f32,
    speed: f32,
    secs: f32,
    t: f32,
}

impl GrassFire {
    /// The middle of the burning front, once it has caught.
    pub fn centre(&self) -> Option<Vec2> {
        (!self.front.is_empty()).then(|| self.front.iter().map(|(p, _)| *p).sum::<Vec2>() / self.front.len() as f32)
    }
}

fn grass_fires(mut commands: Commands, time: Res<Time>, wind: Res<Wind>, mut fx: ResMut<FxAssets>, mut paint: ResMut<crate::terrain::PaintQueue>, mut q: Query<(Entity, &mut GrassFire)>) {
    let dt = crate::sim_dt(&time);
    let down = wind.0.xz().normalize_or_zero();
    let across = down.perp();
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    for (e, mut g) in &mut q {
        if g.start_in > 0.0 {
            g.start_in -= dt;
            if g.start_in <= 0.0 {
                let o = g.origin;
                g.front = (0..5).map(|k| (o + across * (k as f32 - 2.0) * 1.5, rng.random_range(240.0..900.0))).collect();
                info!("grass fire caught at {:.0},{:.0} after {:.0} s", o.x, o.y, time.elapsed_secs());
            }
            continue;
        }
        g.t += dt;
        let (speed, done) = (g.speed, g.t > g.secs);
        let n = g.front.len();
        for i in 0..n {
            let (p, life) = g.front[i];
            // Downwind, wavering, the ends of the front fanning out.
            let fan = (i as f32 - (n as f32 - 1.0) * 0.5) / (n as f32).max(1.0);
            let step = (down + across * (fan * 0.6 + (value_noise(p.x / 7.0, p.y / 7.0, 91)) * 0.5)) * speed * rng.random_range(0.5..1.5);
            let q = p + step * dt;
            let on_road = crate::geo::layout().roads.iter().any(|r| r.line.project(q).0 < r.width * 0.5 + 0.5);
            g.front[i] = (q, if on_road || done { 0.0 } else { life - dt });
            if (q / 1.2).floor() != (p / 1.2).floor() {
                paint.push(crate::terrain::Paint::Scorch { at: q - down * 0.6, radius: 1.4 });
            }
            if rng.random_bool((dt * 2.5).min(1.0) as f64) {
                let at = crate::geo::ground(q);
                spawn(&mut commands, &mut fx, Spawn { kind: Kind::Fire, pos: at + Vec3::Y * 0.3, vel: Vec3::Y * rng.random_range(0.8..2.0), life: rng.random_range(0.3..0.6), size0: rng.random_range(0.6..1.2), size1: 0.4, alpha: 0.9, ..default() });
                if rng.random_bool(0.5) {
                    spawn(&mut commands, &mut fx, Spawn { kind: Kind::WhiteSmoke, pos: at + Vec3::Y * 0.8, vel: Vec3::Y * rng.random_range(1.0..2.0), life: rng.random_range(15.0..30.0), size0: 1.0, size1: rng.random_range(8.0..14.0), alpha: 0.3, drag: 0.4, buoyancy: 0.2, ..default() });
                }
            }
        }
        // The front widens: now and then a point splits where the grass is thick.
        if g.front.len() < 14 && rng.random_bool((dt * 0.05).min(1.0) as f64) {
            let k = rng.random_range(0..g.front.len().max(1));
            if let Some(&(p, life)) = g.front.get(k) {
                g.front.push((p + across * rng.random_range(-1.5..1.5), life * rng.random_range(0.6..1.0)));
            }
        }
        g.front.retain(|&(_, life)| life > 0.0);
        if g.front.is_empty() {
            commands.entity(e).despawn();
        }
    }
    fx.rng = rng;
}

/// The ammunition going all at once: an orange-white fireball 15–25 m across for a second, burning charges
/// raining out to 50 m, and a sooty mushroom that climbs for twenty seconds.
pub fn fireball(commands: &mut Commands, fx: &mut FxAssets, at: Vec3) {
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    let rng = &mut rng;
    flash(commands, at + Vec3::Y * 5.0, 2.5e8, 350.0, 0.9);
    for _ in 0..12 {
        let d = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.2..1.0), rng.random_range(-1.0..1.0));
        spawn(commands, fx, Spawn { kind: Kind::Fire, pos: at + Vec3::Y * 4.0 + d * 3.0, vel: d * rng.random_range(6.0..14.0), life: rng.random_range(0.6..1.1), size0: 8.0, size1: 22.0, alpha: 1.0, drag: 2.0, ..default() });
    }
    for _ in 0..40 {
        let dir = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.4..1.4), rng.random_range(-1.0..1.0)).normalize();
        spawn(commands, fx, Spawn { kind: Kind::Propellant, pos: at + Vec3::Y * 3.0, vel: dir * rng.random_range(15.0..45.0), life: rng.random_range(1.0..3.0), size0: 0.5, size1: 0.3, alpha: 1.0, drag: 0.3, gravity: 9.8, streak: 5.0, ..default() });
    }
    for _ in 0..8 {
        spawn(commands, fx, Spawn { kind: Kind::BlackSmoke, pos: at + Vec3::Y * 6.0 + Vec3::new(rng.random_range(-3.0..3.0), 0.0, rng.random_range(-3.0..3.0)), vel: Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(6.0..14.0), rng.random_range(-3.0..3.0)), life: rng.random_range(15.0..25.0), size0: 8.0, size1: 45.0, alpha: 0.7, drag: 0.4, buoyancy: 1.2, ..default() });
    }
    fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
    explosion(commands, fx, at, 1.0, false);
}

/// A TOS-1A rocket (§2.7 of design/next-items.md): a two-frame flash, an orange fireball 20–25 m across that
/// darkens inside two seconds, a ring of lifted snow or dust racing out along the ground, a sooty head on a
/// grey-brown stem that climbs for half a minute, and haze that hangs; over the town, pale concrete dust.
pub fn thermobaric(commands: &mut Commands, fx: &mut FxAssets, at: Vec3) {
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    let rng = &mut rng;
    let summer = diorama().summer();
    flash(commands, at + Vec3::Y * 8.0, 4.0e8, 500.0, 0.6);
    commands.spawn((
        Mesh3d(fx.sphere.clone()),
        MeshMaterial3d(fx.crater_flash.clone()),
        Transform::from_translation(at + Vec3::Y * 6.0).with_scale(Vec3::splat(9.0)),
        NotShadowCaster,
        PointLight { intensity: 0.0, range: 1.0, ..default() },
        Flash { age: 0.0, life: 0.25, intensity: 0.0 },
    ));
    // The fireball: it rolls up and darkens.
    for _ in 0..22 {
        let d = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.1..1.0), rng.random_range(-1.0..1.0)).normalize();
        spawn(commands, fx, Spawn { kind: Kind::Fire, pos: at + Vec3::Y * 5.0 + d * 4.0, vel: d * rng.random_range(4.0..10.0) + Vec3::Y * 3.0, life: rng.random_range(1.0..1.7), size0: 10.0, size1: 26.0, alpha: 1.0, drag: 2.0, ..default() });
    }
    // The ground ring: lifted snow or dust racing out 20–50 m in a fraction of a second.
    for k in 0..24 {
        let a = k as f32 / 24.0 * std::f32::consts::TAU + rng.random_range(-0.1..0.1);
        let v = Vec3::new(a.cos(), 0.08, a.sin()) * rng.random_range(38.0..52.0);
        spawn(commands, fx, Spawn { kind: if summer { Kind::Dust } else { Kind::Snow }, pos: at + Vec3::Y * 1.0, vel: v, life: rng.random_range(1.0..1.8), size0: 4.0, size1: 16.0, alpha: 0.8, drag: 1.8, ..default() });
    }
    // The head: black and sooty, climbing fast at first; the stem grey-brown beneath it.
    for _ in 0..10 {
        spawn(commands, fx, Spawn { kind: Kind::BlackSmoke, pos: at + Vec3::Y * 8.0 + Vec3::new(rng.random_range(-4.0..4.0), 0.0, rng.random_range(-4.0..4.0)), vel: Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(8.0..16.0), rng.random_range(-3.0..3.0)), life: rng.random_range(40.0..90.0), size0: 12.0, size1: 48.0, alpha: 0.7, drag: 0.4, buoyancy: 1.5, ..default() });
    }
    for _ in 0..6 {
        spawn(commands, fx, Spawn { kind: if summer { Kind::Dust } else { Kind::GreySmoke }, pos: at + Vec3::Y * 3.0, vel: Vec3::new(rng.random_range(-2.0..2.0), rng.random_range(3.0..7.0), rng.random_range(-2.0..2.0)), life: rng.random_range(25.0..50.0), size0: 8.0, size1: 30.0, alpha: 0.6, drag: 0.5, buoyancy: 0.4, ..default() });
    }
    // Concrete and plaster dust off the facades, low and pale, hanging over the block.
    for _ in 0..8 {
        let a = rng.random_range(0.0..std::f32::consts::TAU);
        spawn(commands, fx, Spawn { kind: Kind::Snow, pos: at + Vec3::Y * 2.0, vel: Vec3::new(a.cos() * rng.random_range(3.0..6.0), rng.random_range(0.5..2.0), a.sin() * rng.random_range(3.0..6.0)), life: rng.random_range(50.0..90.0), size0: 6.0, size1: 35.0, alpha: 0.45, drag: 0.6, ..default() });
    }
    fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
    explosion(commands, fx, at, 0.8, true);
}


/// A gun firing: the flash, and for a tank gun the grey-white propellant ball that hangs five to ten seconds
/// (whiter in cold humid air) and the ring of snow or dust its blast lifts 15–20 m out.
pub fn muzzle(commands: &mut Commands, fx: &mut FxAssets, at: Vec3, dir: Vec3, big: bool) {
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    let rng = &mut rng;
    let s = if big { 1.0 } else { 0.3 };
    let summer = diorama().summer();
    flash(commands, at, 6.0e6 * s, 60.0 * s, 0.08);
    spawn(commands, fx, Spawn { kind: Kind::Flash, pos: at + dir * s, vel: dir * 5.0, life: 0.08, size0: 3.0 * s, size1: 4.0 * s, alpha: 1.0, ..default() });
    if big {
        for _ in 0..6 {
            spawn(commands, fx, Spawn { kind: if summer { Kind::GreySmoke } else { Kind::Steam }, pos: at + dir * 2.5, vel: dir * rng.random_range(4.0..12.0) + Vec3::Y * 1.0, life: rng.random_range(5.0..9.0), size0: 2.0, size1: rng.random_range(9.0..14.0), alpha: 0.65, drag: 1.2, ..default() });
        }
        for _ in 0..8 {
            let a = rng.random_range(0.0..std::f32::consts::TAU);
            spawn(commands, fx, Spawn { kind: if summer { Kind::Dust } else { Kind::Snow }, pos: at - Vec3::Y * 1.5 + dir * 3.0, vel: Vec3::new(a.cos(), 0.15, a.sin()) * rng.random_range(8.0..14.0), life: rng.random_range(2.0..3.0), size0: 2.0, size1: 7.0, alpha: 0.6, drag: 1.5, ..default() });
        }
    }
    fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
}


/// An ATGM's back-blast: a fan of snow or dust 10–20 m behind the tube for a few seconds.
pub fn backblast(commands: &mut Commands, fx: &mut FxAssets, from: Vec3, dir: Vec3) {
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    let rng = &mut rng;
    let dust = if diorama().summer() { Kind::Dust } else { Kind::Snow };
    for _ in 0..10 {
        let side = Vec3::new(-dir.z, 0.0, dir.x) * rng.random_range(-0.5..0.5);
        spawn(commands, fx, Spawn { kind: dust, pos: from - dir * 2.0, vel: (-dir + side) * rng.random_range(6.0..15.0) + Vec3::Y * 0.8, life: rng.random_range(3.0..5.0), size0: 1.5, size1: 9.0, alpha: 0.6, drag: 1.4, ..default() });
    }
    fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
}

/// A white-phosphorus round bursting 80 m over `at`: a shower of burning wedges falls trailing white streamers,
/// and each becomes a source on the ground; in a minute they are one pure-white bank.
pub fn phosphorus(commands: &mut Commands, fx: &mut FxAssets, at: Vec3) {
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    let rng = &mut rng;
    let top = at + Vec3::Y * 80.0;
    spawn(commands, fx, Spawn { kind: Kind::Steam, pos: top, vel: Vec3::ZERO, life: 6.0, size0: 4.0, size1: 14.0, alpha: 0.7, ..default() });
    for _ in 0..12 {
        let d = Vec3::new(rng.random_range(-1.0..1.0), -1.0, rng.random_range(-1.0..1.0)).normalize();
        let fall = 80.0 / -d.y;
        let land = at + Vec3::new(d.x * fall, 0.0, d.z * fall);
        let secs = (fall / 14.0).max(1.0);
        spawn(commands, fx, Spawn { kind: Kind::Steam, pos: top, vel: d * 14.0, life: secs, size0: 0.8, size1: 1.2, alpha: 1.0, drag: 0.0, streak: 8.0, ..default() });
        commands.spawn((Transform::from_translation(land), Emitter::after(secs, EmitterKind::Wp, 1.2, rng.random_range(60.0..90.0), Vec3::Y)));
    }
    fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
}

/// A tracer: red for the Ukrainian side (strontium), green for the Russian (barium).
pub fn tracer(commands: &mut Commands, fx: &mut FxAssets, from: Vec3, to: Vec3, speed: f32, side: Side) {
    let d = to - from;
    let len = d.length();
    let kind = if side == Side::Ua { Kind::Tracer } else { Kind::TracerGreen };
    spawn(commands, fx, Spawn { kind, pos: from, vel: d / len * speed, life: len / speed, size0: 0.35, size1: 0.35, alpha: 1.0, drag: 0.0, streak: 18.0, ..default() });
}


/// A track-throw of snow behind a moving vehicle; in summer, a dust plume that hangs over the track.
pub fn spray(commands: &mut Commands, fx: &mut FxAssets, at: Vec3, vel: Vec3) {
    let mut rng = std::mem::replace(&mut fx.rng, SmallRng::seed_from_u64(0));
    let rng = &mut rng;
    if crate::diorama::diorama().summer() {
        spawn(commands, fx, Spawn { kind: Kind::Snow, pos: at + Vec3::Y * 0.8, vel: -vel * 0.25 + Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.6..1.6), rng.random_range(-1.0..1.0)), life: rng.random_range(5.0..8.0), size0: 1.6, size1: rng.random_range(7.0..11.0), alpha: 0.2, drag: 0.8, ..default() });
        fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
        return;
    }
    spawn(commands, fx, Spawn { kind: Kind::Snow, pos: at + Vec3::Y * 0.5, vel: -vel * 0.2 + Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(0.5..1.5), rng.random_range(-1.0..1.0)), life: rng.random_range(1.5..2.5), size0: 1.2, size1: 4.0, alpha: 0.45, drag: 1.0, ..default() });
    fx.rng = std::mem::replace(rng, SmallRng::seed_from_u64(0));
}
