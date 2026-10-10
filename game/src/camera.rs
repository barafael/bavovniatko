//! Two ways of seeing the field.
//!
//! - **The drone** (default): a Mavic/Matrice-class quadcopter hovering behind the Ukrainian lines, the way most
//!   of the world saw Vuhledar, as grey, grainy video with a crosshair and telemetry. Its battery drains.
//! - **The OP**: the observer on the roof of a nine-storey block at the town's southern edge, with binoculars.
//!   Closer to the noise, and inside the TOS-1A's target area.

use crate::geo;
use crate::world::ObservationPost;
use bevy::{
    anti_alias::fxaa::Fxaa,
    camera::Exposure,
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
            .add_systems(Update, (switch_view, drone_control, op_control, apply_view, update_feed).chain());
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Mode {
    #[default]
    Drone,
    Op,
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
        }
    }
}

/// Camera shake and white-out from nearby blasts; decays on its own.
#[derive(Resource, Default)]
pub struct Shake {
    pub trauma: f32,
    pub flash: f32,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default)]
pub struct FeedEffect {
    pub time: f32,
    /// 0 = drone feed, 1 = OP binoculars.
    pub mode: f32,
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
            falloff: FogFalloff::from_visibility_colors(3800.0, Color::srgb(0.48, 0.5, 0.54), Color::srgb(0.72, 0.74, 0.77)),
        },
        FeedEffect { grain: 1.0, zoom: 1.0, ..default() },
        IsDefaultUiCamera,
    ));
    if crate::headless() {
        let mut img = Image::new_target_texture(1600, 900, bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb, None);
        img.texture_descriptor.usage |= bevy::render::render_resource::TextureUsages::COPY_SRC;
        let h = images.add(img);
        cam.insert(bevy::camera::RenderTarget::Image(h.clone().into()));
        commands.insert_resource(HeadlessTarget(h));
    }
    let _ = Fxaa::default();
}

fn switch_view(keys: Res<ButtonInput<KeyCode>>, mut view: ResMut<View>) {
    if keys.just_pressed(KeyCode::Digit1) {
        view.mode = Mode::Drone;
    }
    if keys.just_pressed(KeyCode::Digit2) && view.op_alive {
        view.mode = Mode::Op;
    }
    if keys.just_pressed(KeyCode::Tab) {
        view.mode = if view.mode == Mode::Drone && view.op_alive { Mode::Op } else { Mode::Drone };
    }
    if !view.op_alive && view.mode == Mode::Op {
        view.mode = Mode::Drone;
    }
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
    if view.feed_down <= 0.0 && std::env::var("VUHLEDAR_NO_BATTERY").is_err() {
        view.battery -= dt / 600.0;
        if view.battery <= 0.0 {
            view.feed_down = 25.0; // fly back, swap, relaunch
            view.battery = 1.0;
        }
    } else {
        view.feed_down -= dt;
    }
    view.jamming = (view.jamming - dt * 0.1).max(0.0);
    if view.mode != Mode::Drone {
        return;
    }
    let shift = keys.pressed(KeyCode::ShiftLeft);
    let speed = if shift { 120.0 } else { 45.0 } / view.zoom().sqrt();
    let yaw = view.drone_yaw;
    let fwd = Vec3::new(-yaw.sin(), 0.0, -yaw.cos());
    let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin());
    let mut mv = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) { mv += fwd }
    if keys.pressed(KeyCode::KeyS) { mv -= fwd }
    if keys.pressed(KeyCode::KeyD) { mv += right }
    if keys.pressed(KeyCode::KeyA) { mv -= right }
    view.drone_pos += mv.normalize_or_zero() * speed * dt;
    // Keep the drone over Ukrainian-held ground, mostly: it may creep forward, at a price in jamming.
    view.drone_pos.x = view.drone_pos.x.clamp(-geo::HALF_X + 50.0, geo::HALF_X - 50.0);
    view.drone_pos.z = view.drone_pos.z.clamp(-geo::HALF_Z + 50.0, 1400.0);
    if view.drone_pos.z > -100.0 {
        view.jamming = (view.jamming + dt * 0.05 * ((view.drone_pos.z + 100.0) / 500.0)).min(1.0);
    }
    if keys.pressed(KeyCode::KeyQ) { view.drone_yaw += dt * 0.7 }
    if keys.pressed(KeyCode::KeyE) { view.drone_yaw -= dt * 0.7 }
    let mut agl = view.drone_pos.y - geo::height(view.drone_pos.x, view.drone_pos.z);
    if keys.pressed(KeyCode::KeyR) { agl += dt * 30.0 }
    if keys.pressed(KeyCode::KeyF) { agl -= dt * 30.0 }
    if !keys.pressed(KeyCode::KeyR) && !keys.pressed(KeyCode::KeyF) {
        agl += (View::DRONE_AGL - agl).clamp(-1.0, 1.0) * dt * 2.0 * (agl < 60.0) as i32 as f32;
    }
    agl = agl.clamp(40.0, 700.0);
    view.drone_pos.y = geo::height(view.drone_pos.x, view.drone_pos.z) + agl;
    if buttons.pressed(MouseButton::Right) || buttons.pressed(MouseButton::Middle) {
        view.drone_yaw -= motion.delta.x * 0.003 / view.zoom().sqrt();
        view.gimbal = (view.gimbal - motion.delta.y * 0.003 / view.zoom().sqrt()).clamp(-1.55, 0.1);
    }
    if scroll.delta.y != 0.0 {
        view.drone_fov = (view.drone_fov * (1.0 - scroll.delta.y * 0.12)).clamp(3f32.to_radians(), 70f32.to_radians());
    }
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
            // Hover wobble: a quadcopter in a 7 m/s February wind.
            let wob = Vec3::new((t * 0.7).sin() * 0.25, (t * 0.5).sin() * 0.15, (t * 0.9 + 1.0).sin() * 0.25);
            let rot = Quat::from_euler(EulerRot::YXZ, view.drone_yaw + (t * 0.31).sin() * 0.002, view.gimbal + (t * 0.43).sin() * 0.0015, (t * 0.6).sin() * 0.002);
            (view.drone_pos + wob, rot, view.drone_fov)
        }
        Mode::Op => {
            let eye = op.map(|o| o.eye).unwrap_or(Vec3::new(-300.0, 215.0, -1230.0));
            // Hand-held binoculars: breathing sway, worse when zoomed in.
            let z = view.zoom();
            let sway = Vec2::new((t * 0.9).sin() + 0.5 * (t * 2.3).sin(), (t * 1.1).cos() + 0.4 * (t * 2.9).sin()) * 0.0015 * z.min(6.0);
            let rot = Quat::from_euler(EulerRot::YXZ, view.op_yaw + sway.x / z, view.op_pitch + sway.y / z, 0.0);
            (eye, rot, view.op_fov)
        }
    };
    // Shake from nearby blasts.
    let tr = shake.trauma * shake.trauma;
    let jitter = Vec3::new((t * 53.0).sin(), (t * 61.0 + 2.0).sin(), (t * 47.0 + 4.0).sin()) * tr * if view.mode == Mode::Op { 0.6 } else { 0.25 };
    let rjit = Quat::from_euler(EulerRot::YXZ, (t * 37.0).sin() * tr * 0.02, (t * 43.0).sin() * tr * 0.02, (t * 29.0).sin() * tr * 0.03);
    tf.translation = pos + jitter;
    tf.rotation = rot * rjit;
    if let Projection::Perspective(p) = proj.as_mut() {
        p.fov = fov;
    }
    shake.trauma = (shake.trauma - dt * 0.9).max(0.0);
    shake.flash = (shake.flash - dt * 2.5).max(0.0);
}

fn update_feed(time: Res<Time>, view: Res<View>, shake: Res<Shake>, mut fx: Single<&mut FeedEffect>) {
    fx.time = time.elapsed_secs() % 1000.0;
    fx.mode = if view.mode == Mode::Drone { 0.0 } else { 1.0 };
    fx.grain = if view.mode == Mode::Drone { 1.0 } else { 0.45 };
    fx.glitch = if view.mode == Mode::Drone { view.jamming.max(if view.battery < 0.08 { 0.3 } else { 0.0 }) } else { 0.0 };
    fx.flash = shake.flash;
    fx.zoom = view.zoom();
    fx.down = if view.mode == Mode::Drone && view.feed_down > 0.0 { 1.0 } else { 0.0 };
}

/// Ray from the cursor into the terrain.
pub fn cursor_ground(window: &Window, camera: &Camera, cam_tf: &GlobalTransform) -> Option<Vec3> {
    let cursor = window.cursor_position()?;
    let ray = camera.viewport_to_world(cam_tf, cursor).ok()?;
    geo::raycast(ray.origin, *ray.direction, 9000.0)
}
