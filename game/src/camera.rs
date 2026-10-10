//! Two ways of seeing the field.
//!
//! - **The drone** (default): a Mavic/Matrice-class quadcopter hovering behind the Ukrainian lines, the way most
//!   of the world saw Vuhledar, as grey, grainy video with a crosshair and telemetry. Its battery drains.
//! - **The OP**: the observer on the roof of a nine-storey block at the town's southern edge, with binoculars.
//!   Closer to the noise, and inside the TOS-1A's target area.

use crate::dev_env::dev;
use crate::geo;
use crate::world::ObservationPost;
use bevy::{
    camera::{Exposure, ShadowLodOrigin},
    core_pipeline::{
        fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
        tonemapping::{Tonemapping, tonemapping},
        Core3dSystems,
    },
    ecs::{schedule::ScheduleConfigs, system::BoxedSystem},
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    pbr::{DistanceFog, FogFalloff},
    prelude::*,
    render::{extract_component::ExtractComponent, render_resource::ShaderType, view::Msaa},
    shader::ShaderRef,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FullscreenMaterialPlugin::<FeedEffect>::default())
            .init_resource::<View>()
            .init_resource::<Shake>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_launcher.after(spawn_camera))
            .add_systems(Update, (switch_view, drone_control, op_control, ground_control, apply_view, update_feed, cursor_grab, show_launcher).chain());
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Mode {
    #[default]
    Drone,
    Op,
    /// Eye level at the south edge of a tree line, behind an ATGM launcher.
    Ground,
}

#[derive(Resource)]
pub struct View {
    pub mode: Mode,
    pub drone_pos: Vec3,
    pub drone_yaw: f32,
    pub gimbal: f32,
    pub drone_fov: f32,
    pub op_yaw: f32,
    pub op_pitch: f32,
    pub op_fov: f32,
    pub battery: f32,
    /// Seconds the feed is down (drone lost or swapping battery).
    pub feed_down: f32,
    pub drones_left: u32,
    /// Signal degradation from Russian EW, 0..1.
    pub jamming: f32,
    pub op_alive: bool,
    // Drone flight state: it is an aircraft, not a camera rig.
    pub drone_vel: Vec3,
    pub yaw_target: f32,
    pub gimbal_target: f32,
    pub fov_target: f32,
    pub climb: f32,
    /// Visual attitude disturbance from gusts and manoeuvres (yaw, pitch, roll).
    pub tilt: Vec3,
    // Ground view.
    pub ground_team: Option<Entity>,
    pub ground_eye: Vec3,
    pub ground_yaw: f32,
    pub ground_pitch: f32,
    pub sight: bool,
    /// Range to whatever the sight's crosshair is on.
    pub ground_range: Option<f32>,
    /// Javelin: the vehicle the seeker is tracking and for how long (it locks after 2.5 s).
    pub lock: Option<(Entity, f32)>,
}

impl Default for View {
    fn default() -> Self {
        Self {
            mode: Mode::Drone,
            drone_pos: Vec3::new(150.0, 0.0, -700.0),
            drone_yaw: std::f32::consts::PI,
            gimbal: -0.62,
            drone_fov: 52f32.to_radians(),
            op_yaw: 3.6,
            op_pitch: -0.07,
            op_fov: 55f32.to_radians(),
            battery: 1.0,
            feed_down: 0.0,
            drones_left: 3,
            jamming: 0.0,
            op_alive: true,
            drone_vel: Vec3::ZERO,
            yaw_target: std::f32::consts::PI,
            gimbal_target: -0.62,
            fov_target: 52f32.to_radians(),
            climb: 0.0,
            tilt: Vec3::ZERO,
            ground_team: None,
            ground_eye: Vec3::ZERO,
            ground_yaw: std::f32::consts::PI,
            ground_pitch: 0.0,
            sight: false,
            ground_range: None,
            lock: None,
        }
    }
}

impl View {
    /// Height above ground the drone holds.
    pub const DRONE_AGL: f32 = 190.0;

    pub fn zoom(&self) -> f32 {
        match self.mode {
            Mode::Drone => 52f32.to_radians() / self.drone_fov,
            Mode::Op => 55f32.to_radians() / self.op_fov,
            Mode::Ground => if self.sight { Self::SIGHT_ZOOM } else { 1.0 },
        }
    }

    pub const GROUND_FOV: f32 = 0.95;
    /// Magnification of the ATGM sight.
    pub const SIGHT_ZOOM: f32 = 10.0;
}

/// Camera shake and white-out from nearby blasts; decays on its own.
#[derive(Resource, Default)]
pub struct Shake {
    pub trauma: f32,
    pub flash: f32,
}

#[derive(Component)]
pub struct MainCamera;

/// Which look the post-process gives the picture (matched by value in feed.wgsl).
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FeedMode {
    /// The drone's compressed video.
    Drone = 0,
    /// The OP's binoculars.
    Binoculars = 1,
    /// The naked eye in the tree line.
    Eye = 2,
    /// Through the ATGM sight.
    Sight = 3,
}

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default)]
pub struct FeedEffect {
    pub time: f32,
    /// A [`FeedMode`].
    pub mode: u32,
    pub grain: f32,
    /// Signal breakup, 0..1.
    pub glitch: f32,
    pub flash: f32,
    pub zoom: f32,
    pub down: f32,
    pub _pad: f32,
}

impl FullscreenMaterial for FeedEffect {
    fn fragment_shader() -> ShaderRef {
        "embedded://vuhledar/shaders/feed.wgsl".into()
    }

    // After tonemapping, so grain and compression live in display space like a real video feed.
    fn schedule_configs(system: ScheduleConfigs<BoxedSystem>) -> ScheduleConfigs<BoxedSystem> {
        system.in_set(Core3dSystems::PostProcess).after(tonemapping)
    }
}

/// The offscreen image the camera renders into when running headless.
#[derive(Resource)]
pub struct HeadlessTarget(pub Handle<Image>);

fn spawn_camera(mut commands: Commands, mut view: ResMut<View>, mut images: ResMut<Assets<Image>>) {
    view.drone_pos.y = geo::height(view.drone_pos.x, view.drone_pos.z) + View::DRONE_AGL;
    let mut cam = commands.spawn((
        Camera3d::default(),
        MainCamera,
        Projection::Perspective(PerspectiveProjection { fov: view.drone_fov, near: 0.5, far: 12_000.0, ..default() }),
        Transform::from_translation(view.drone_pos),
        Msaa::Sample4,
        Tonemapping::AgX,
        Exposure { ev100: 11.25 },
        DistanceFog {
            color: Color::srgb(0.70, 0.72, 0.75),
            directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.15),
            directional_light_exponent: 12.0,
            falloff: FogFalloff::from_visibility_colors(3000.0, Color::srgb(0.48, 0.5, 0.54), Color::srgb(0.72, 0.74, 0.77)),
        },
        FeedEffect { grain: 1.0, zoom: 1.0, ..default() },
        IsDefaultUiCamera,
        // Point-light shadows are detailed around the drone (headless, no camera renders to a window).
        ShadowLodOrigin,
    ));
    if dev().headless {
        let (w, h) = dev().res.unwrap_or((1600, 900));
        let mut img = Image::new_target_texture(w, h, bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb, None);
        img.texture_descriptor.usage |= bevy::render::render_resource::TextureUsages::COPY_SRC;
        let h = images.add(img);
        cam.insert(bevy::camera::RenderTarget::Image(h.clone().into()));
        commands.insert_resource(HeadlessTarget(h));
    }
}

fn switch_view(keys: Res<ButtonInput<KeyCode>>, mut view: ResMut<View>, teams: Query<(Entity, &crate::fire::UaPosition)>) {
    if keys.just_pressed(KeyCode::Digit1) {
        view.mode = Mode::Drone;
    }
    if keys.just_pressed(KeyCode::Digit2) && view.op_alive {
        view.mode = Mode::Op;
    }
    if keys.just_pressed(KeyCode::Digit3) {
        // Into a tree line with an ATGM team; pressing again moves on to the next team still in action.
        let mut live: Vec<(Entity, Vec2)> = teams.iter().filter(|(_, p)| p.atgm > 0 && p.strength >= 2).map(|(e, p)| (e, p.pos)).collect();
        live.sort_by(|a, b| a.1.x.total_cmp(&b.1.x));
        if !live.is_empty() {
            let i = match (view.mode, view.ground_team) {
                (Mode::Ground, Some(cur)) => live.iter().position(|(e, _)| *e == cur).map(|i| (i + 1) % live.len()).unwrap_or(0),
                _ => live.iter().position(|(e, _)| Some(*e) == view.ground_team).unwrap_or(0),
            };
            let (e, p) = live[i];
            enter_ground(&mut view, e, p);
        }
    }
    if keys.just_pressed(KeyCode::Tab) {
        view.mode = match view.mode {
            Mode::Drone if view.op_alive => Mode::Op,
            _ => Mode::Drone,
        };
    }
    if !view.op_alive && view.mode == Mode::Op {
        view.mode = Mode::Drone;
    }
}

/// Kneel in the trench at the south edge of the team's tree line, facing the fields.
pub fn enter_ground(view: &mut View, team: Entity, pos: Vec2) {
    let at = pos + Vec2::new(1.5, 8.5);
    view.mode = Mode::Ground;
    view.ground_team = Some(team);
    // Eye on the launcher's sight, set up on the trench parapet.
    view.ground_eye = geo::ground(at) + Vec3::Y * 1.4;
    // Face the nearest road's kill zone.
    let kz = geo::layout().routes.iter().map(|r| r.line.point(r.deploy_s)).min_by(|a, b| a.distance(at).total_cmp(&b.distance(at))).unwrap_or(at + Vec2::Y);
    let d = kz - at;
    view.ground_yaw = f32::atan2(-d.x, -d.y);
    view.ground_pitch = -0.01;
    view.sight = false;
    view.lock = None;
}

fn drone_control(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mut view: ResMut<View>,
) {
    let dt = time.delta_secs();
    // The battery drains whether or not you are watching. A Matrice gives you about 35 minutes; we compress.
    if view.feed_down <= 0.0 && !dev().no_battery {
        view.battery -= dt / 600.0;
        if view.battery <= 0.0 {
            view.feed_down = 25.0; // fly back, swap, relaunch
            view.battery = 1.0;
        }
    } else {
        view.feed_down -= dt;
    }
    view.jamming = (view.jamming - dt * 0.1).max(0.0);
    let t = time.elapsed_secs();
    // Gusts: the position hold fights a wind that comes and goes. Mostly a few m/s, now and then a shove.
    let gust = Vec3::new(geo::fbm(t * 0.21, 1.3, 3, 61), 0.0, geo::fbm(t * 0.21 + 40.0, 7.1, 3, 62)) * 2.2
        * (1.0 + 2.5 * (geo::value_noise(t * 0.07, 3.3, 63) - 0.35).max(0.0));
    if view.mode != Mode::Drone || dev().showcase.is_some() {
        return;
    }
    let shift = keys.pressed(KeyCode::ShiftLeft);
    // A Mavic-class drone: about 7 m/s for steady filming, 17 m/s in sport mode.
    let (max_speed, max_accel) = if shift { (17.0, 5.0) } else { (7.0, 2.5) };
    let yaw = view.drone_yaw;
    let fwd = Vec3::new(-yaw.sin(), 0.0, -yaw.cos());
    let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin());
    let mut stick = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) { stick += fwd }
    if keys.pressed(KeyCode::KeyS) { stick -= fwd }
    if keys.pressed(KeyCode::KeyD) { stick += right }
    if keys.pressed(KeyCode::KeyA) { stick -= right }
    let desired = stick.normalize_or_zero() * max_speed;
    let horizontal = Vec3::new(view.drone_vel.x, 0.0, view.drone_vel.z);
    // The flight controller only half-cancels the gust before it is felt.
    let accel = ((desired - horizontal) * 1.3 - gust * 0.35).clamp_length_max(max_accel);
    let mut vel = horizontal + accel * dt;
    // Vertical: climb or sink towards the commanded rate, gently.
    let climb_cmd = if keys.pressed(KeyCode::KeyR) { 1.0 } else if keys.pressed(KeyCode::KeyF) { -1.0 } else { 0.0 } * if shift { 6.0 } else { 3.0 };
    view.climb += (climb_cmd - view.climb).clamp(-2.0 * dt, 2.0 * dt);
    vel.y = view.climb;
    view.drone_vel = vel;
    let step = Vec3::new(vel.x + gust.x * 0.25, vel.y, vel.z + gust.z * 0.25) * dt;
    view.drone_pos += step;
    // Keep the drone over Ukrainian-held ground, mostly: it may creep forward, at a price in jamming.
    view.drone_pos.x = view.drone_pos.x.clamp(-geo::HALF_X + 50.0, geo::HALF_X - 50.0);
    view.drone_pos.z = view.drone_pos.z.clamp(-geo::HALF_Z + 50.0, 1400.0);
    if view.drone_pos.z > -100.0 {
        view.jamming = (view.jamming + dt * 0.05 * ((view.drone_pos.z + 100.0) / 500.0)).min(1.0);
    }
    let ground = geo::height(view.drone_pos.x, view.drone_pos.z);
    view.drone_pos.y = view.drone_pos.y.clamp(ground + 25.0, ground + 700.0);
    // Yaw and gimbal slew towards their targets; the zoom motor takes its time.
    if keys.pressed(KeyCode::KeyQ) { view.yaw_target += dt * 0.6 }
    if keys.pressed(KeyCode::KeyE) { view.yaw_target -= dt * 0.6 }
    if buttons.pressed(MouseButton::Right) || buttons.pressed(MouseButton::Middle) {
        view.yaw_target -= motion.delta.x * 0.0025 / view.zoom().sqrt();
        view.gimbal_target = (view.gimbal_target - motion.delta.y * 0.0025 / view.zoom().sqrt()).clamp(-1.55, 0.1);
    }
    let dyaw = crate::units::wrap(view.yaw_target - view.drone_yaw);
    view.drone_yaw += (dyaw * (1.0 - (-dt * 4.0).exp())).clamp(-1.2 * dt, 1.2 * dt);
    let dg = view.gimbal_target - view.gimbal;
    view.gimbal += (dg * (1.0 - (-dt * 6.0).exp())).clamp(-1.5 * dt, 1.5 * dt);
    if scroll.delta.y != 0.0 {
        view.fov_target = (view.fov_target * (1.0 - scroll.delta.y * 0.12)).clamp(3f32.to_radians(), 70f32.to_radians());
    }
    let lf = view.drone_fov.ln() + (view.fov_target.ln() - view.drone_fov.ln()) * (1.0 - (-dt * 3.0).exp());
    view.drone_fov = lf.exp();
    // What the gimbal cannot quite hide: a little lag in the turns, a tremor in the gusts.
    let target_tilt = Vec3::new(-dyaw.clamp(-0.3, 0.3) * 0.01, accel.dot(fwd) * -0.002, accel.dot(right) * 0.002) + gust * 0.0012;
    let cur = view.tilt;
    view.tilt = cur + (target_tilt - cur) * (1.0 - (-dt * 5.0).exp());
}

fn op_control(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
    mut view: ResMut<View>,
) {
    if view.mode != Mode::Op {
        return;
    }
    let dt = time.delta_secs();
    if buttons.pressed(MouseButton::Right) || buttons.pressed(MouseButton::Middle) {
        view.op_yaw -= motion.delta.x * 0.003 / view.zoom();
        view.op_pitch = (view.op_pitch - motion.delta.y * 0.003 / view.zoom()).clamp(-0.9, 0.4);
    }
    if keys.pressed(KeyCode::KeyA) { view.op_yaw += dt * 0.5 / view.zoom() }
    if keys.pressed(KeyCode::KeyD) { view.op_yaw -= dt * 0.5 / view.zoom() }
    if keys.pressed(KeyCode::KeyW) { view.op_pitch += dt * 0.3 / view.zoom() }
    if keys.pressed(KeyCode::KeyS) { view.op_pitch -= dt * 0.3 / view.zoom() }
    if scroll.delta.y != 0.0 {
        view.op_fov = (view.op_fov * (1.0 - scroll.delta.y * 0.12)).clamp(5f32.to_radians(), 70f32.to_radians());
    }
}

fn ground_control(
    time: Res<Time>,
    vehicles: Query<(Entity, &crate::units::Vehicle, &Transform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    mut view: ResMut<View>,
    camera: Single<&GlobalTransform, With<MainCamera>>,
) {
    if view.mode != Mode::Ground {
        return;
    }
    // Mouse look is direct here (the cursor is captured); the right button brings the eye to the sight.
    if !dev().ground_view() {
        view.sight = buttons.pressed(MouseButton::Right);
    }
    let k = 0.0022 / view.zoom();
    view.ground_yaw -= motion.delta.x * k;
    view.ground_pitch = (view.ground_pitch - motion.delta.y * k).clamp(-0.5, 0.35);
    let gt = camera.compute_transform();
    let terrain = geo::raycast(gt.translation, *gt.forward(), 5500.0).map(|p| p.distance(gt.translation));
    // The Javelin seeker: whichever vehicle sits within ~0.4° of the crosshair, held there, locks.
    let fwd = *gt.forward();
    let on = vehicles
        .iter()
        .filter(|(_, v, _)| v.side == crate::units::Side::Ru && v.state.alive())
        .map(|(e, _, t)| (e, (t.translation + Vec3::Y * 1.2 - gt.translation).normalize().dot(fwd), t.translation.distance(gt.translation)))
        .filter(|(_, c, _)| *c > (0.007f32).cos())
        .max_by(|a, b| a.1.total_cmp(&b.1));
    // The range readout shows the vehicle under the crosshair if there is one, else the ground.
    view.ground_range = on.map(|(_, _, d)| d).or(terrain);
    let on = on.filter(|(_, _, d)| *d < crate::fire::Atgm::Javelin.range() + 100.0).map(|(e, _, _)| e);
    view.lock = match (on, view.lock) {
        (Some(e), Some((l, t))) if e == l && view.sight => Some((e, t + time.delta_secs())),
        (Some(e), _) if view.sight => Some((e, 0.0)),
        _ => None,
    };
}

fn apply_view(
    time: Res<Time>,
    view: Res<View>,
    op: Option<Res<ObservationPost>>,
    mut shake: ResMut<Shake>,
    camera: Single<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    let (mut tf, mut proj) = camera.into_inner();
    let t = time.elapsed_secs();
    let dt = time.delta_secs();
    let (pos, rot, fov) = match view.mode {
        Mode::Drone => {
            // The gimbal steadies most of the airframe's motion; a tremor gets through, worse when zoomed.
            let z = view.zoom().sqrt();
            let tremor = Vec3::new((t * 3.1).sin() * 0.0004, (t * 2.7 + 1.0).sin() * 0.0004, (t * 2.3).sin() * 0.0006) * z;
            let tl = view.tilt + tremor;
            let rot = Quat::from_euler(EulerRot::YXZ, view.drone_yaw + tl.x, view.gimbal + tl.y, tl.z);
            (view.drone_pos, rot, view.drone_fov)
        }
        Mode::Op => {
            let eye = op.map(|o| o.eye).unwrap_or(Vec3::new(-300.0, 215.0, -1230.0));
            // Hand-held binoculars: breathing sway, worse when zoomed in.
            let z = view.zoom();
            let sway = Vec2::new((t * 0.9).sin() + 0.5 * (t * 2.3).sin(), (t * 1.1).cos() + 0.4 * (t * 2.9).sin()) * 0.0015 * z.min(6.0);
            let rot = Quat::from_euler(EulerRot::YXZ, view.op_yaw + sway.x / z, view.op_pitch + sway.y / z, 0.0);
            (eye, rot, view.op_fov)
        }
        Mode::Ground => {
            // Kneeling, breathing; the sight on its tripod is steadier, but the gunner's hands are not.
            let breath = (t * 1.4).sin();
            let sway = if view.sight {
                Vec2::new((t * 0.8).sin() + 0.6 * (t * 2.1).sin(), (t * 1.0).cos() + 0.5 * (t * 2.6).sin()) * 0.00025
            } else {
                Vec2::new((t * 0.6).sin(), breath) * 0.004
            };
            let eye = view.ground_eye + Vec3::Y * breath * 0.012;
            let rot = Quat::from_euler(EulerRot::YXZ, view.ground_yaw + sway.x, view.ground_pitch + sway.y, 0.0);
            (eye, rot, if view.sight { View::GROUND_FOV / View::SIGHT_ZOOM } else { View::GROUND_FOV })
        }
    };
    // Shake from nearby blasts.
    let tr = shake.trauma * shake.trauma;
    let jitter = Vec3::new((t * 53.0).sin(), (t * 61.0 + 2.0).sin(), (t * 47.0 + 4.0).sin()) * tr * match view.mode { Mode::Op => 0.6, Mode::Ground => 0.35, Mode::Drone => 0.25 };
    let rjit = Quat::from_euler(EulerRot::YXZ, (t * 37.0).sin() * tr * 0.02, (t * 43.0).sin() * tr * 0.02, (t * 29.0).sin() * tr * 0.03);
    tf.translation = pos + jitter;
    tf.rotation = rot * rjit;
    if let Projection::Perspective(p) = proj.as_mut() {
        p.fov = fov;
        p.near = if view.mode == Mode::Ground { 0.1 } else { 0.5 };
    }
    shake.trauma = (shake.trauma - dt * 0.9).max(0.0);
    shake.flash = (shake.flash - dt * 2.5).max(0.0);
}

fn update_feed(time: Res<Time>, view: Res<View>, shake: Res<Shake>, mut fx: Single<&mut FeedEffect>) {
    fx.time = time.elapsed_secs() % 1000.0;
    fx.mode = match view.mode {
        Mode::Drone => FeedMode::Drone,
        Mode::Op => FeedMode::Binoculars,
        Mode::Ground if view.sight => FeedMode::Sight,
        Mode::Ground => FeedMode::Eye,
    } as u32;
    fx.grain = match view.mode { Mode::Drone => 1.0, Mode::Op => 0.45, Mode::Ground => 0.3 };
    fx.glitch = if view.mode == Mode::Drone { view.jamming.max(if view.battery < 0.08 { 0.3 } else { 0.0 }) } else { 0.0 };
    fx.flash = shake.flash * if view.mode == Mode::Ground { 0.35 } else { 1.0 };
    fx.zoom = view.zoom();
    fx.down = if view.mode == Mode::Drone && view.feed_down > 0.0 { 1.0 } else { 0.0 };
}

/// Ray from the cursor into the terrain.
pub fn cursor_ground(window: &Window, camera: &Camera, cam_tf: &GlobalTransform) -> Option<Vec3> {
    let cursor = window.cursor_position()?;
    let ray = camera.viewport_to_world(cam_tf, cursor).ok()?;
    geo::raycast(ray.origin, *ray.direction, 9000.0)
}

/// The launcher on its tripod in front of the eye in the ground view: a Stugna-P tube and its sight box.
#[derive(Component)]
struct Launcher;

fn spawn_launcher(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, camera: Single<Entity, With<MainCamera>>) {
    let mut m = crate::meshes::MeshBuilder::new();
    m.cylinder(Vec3::new(0.0, 0.0, 0.25), Vec3::new(0.0, 0.0, -1.25), 0.075, 10, [0.30, 0.33, 0.24]);
    m.cylinder(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, -0.06), 0.085, 10, [0.18, 0.19, 0.16]);
    m.boxc(Vec3::new(-0.16, 0.02, 0.05), Vec3::new(0.16, 0.18, 0.32), [0.20, 0.22, 0.18]);
    m.boxc(Vec3::new(-0.16, 0.02, -0.12), Vec3::new(0.1, 0.1, 0.03), [0.05, 0.06, 0.07]);
    for (x, z) in [(-0.35f32, 0.25f32), (0.35, 0.25), (0.0, -0.45)] {
        m.cuboid(Vec3::new(x * 0.5, -0.32, z * 0.5), Vec3::new(0.02, 0.32, 0.02), Quat::from_rotation_x(z * 0.6) * Quat::from_rotation_z(-x * 0.6), [0.15, 0.15, 0.14]);
    }
    let e = commands
        .spawn((
            Mesh3d(meshes.add(m.build())),
            MeshMaterial3d(materials.add(StandardMaterial { perceptual_roughness: 0.8, ..default() })),
            Transform::from_xyz(0.32, -0.38, -0.55),
            Visibility::Hidden,
            bevy::light::NotShadowCaster,
            Launcher,
        ))
        .id();
    commands.entity(*camera).add_child(e);
}

fn show_launcher(view: Res<View>, mut q: Query<&mut Visibility, With<Launcher>>) {
    for mut v in &mut q {
        v.set_if_neq(if view.mode == Mode::Ground && !view.sight { Visibility::Inherited } else { Visibility::Hidden });
    }
}

/// Capture the cursor in the ground view (mouse look); give it back elsewhere.
fn cursor_grab(view: Res<View>, mut last: Local<Option<Mode>>, mut cursor: Query<&mut bevy::window::CursorOptions, With<bevy::window::PrimaryWindow>>) {
    if *last == Some(view.mode) {
        return;
    }
    *last = Some(view.mode);
    for mut c in &mut cursor {
        let grab = view.mode == Mode::Ground;
        c.grab_mode = if grab { bevy::window::CursorGrabMode::Locked } else { bevy::window::CursorGrabMode::None };
        c.visible = !grab;
    }
}
