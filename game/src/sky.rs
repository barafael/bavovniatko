//! The sky: a dome round the camera with a procedural cloud deck, a sun that shows through thin cloud, and a
//! horizon matched to the fog. The day's light (`world::daylight`, `robotyne::daylight`) sets its state each
//! frame; the weather of the day is a preset (design/next-items.md §2.1): the unbroken low stratus of
//! 6 February 2023 as ERA5 has it, the broken cloud and hard sun of the week's footage, and June's cirrus veil.

use crate::camera::MainCamera;
use bevy::{
    light::NotShadowCaster,
    pbr::{Material, MaterialPlugin},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SkyMaterial>::default())
            .add_systems(Startup, spawn_dome)
            .add_systems(Update, follow_camera.in_set(crate::Step::Effects));
    }
}

/// What the sky shader is told (matched by field order in sky.wgsl).
#[derive(ShaderType, Clone, Copy, Debug)]
pub struct SkyParams {
    /// Zenith and horizon colours, linear.
    pub zenith: Vec4,
    pub horizon: Vec4,
    /// Towards the sun (xyz) and the disc's angular radius (w, radians).
    pub sun_dir: Vec4,
    /// The sun's colour, linear (rgb) and its strength (w).
    pub sun_col: Vec4,
    /// x: cover 0..1; y: the deck's base height in metres; z: how dark its underside is (0..1 of the horizon);
    /// w: how much of a thin veil it is (1: cirrus the sun shows through).
    pub cloud: Vec4,
    /// xy: the deck's drift so far in metres; z: the light on the deck's top (0..1); w: unused.
    pub drift: Vec4,
}

impl Default for SkyParams {
    fn default() -> Self {
        Self { zenith: Vec4::new(0.3, 0.35, 0.45, 1.0), horizon: Vec4::new(0.45, 0.47, 0.5, 1.0), sun_dir: Vec4::new(0.0, 1.0, 0.0, 0.03), sun_col: Vec4::new(1.0, 0.9, 0.8, 0.0), cloud: Vec4::new(1.0, 400.0, 0.8, 0.0), drift: Vec4::ZERO }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Clone)]
pub struct SkyMaterial {
    #[uniform(0)]
    pub params: SkyParams,
}

impl Material for SkyMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://vuhledar/shaders/sky.wgsl".into()
    }
}

/// The weather over a day: which deck, how high, whether the sun shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Weather {
    /// 6 February 2023 as ERA5 has it: an unbroken low stratus deck, base 300–500 m, no sun.
    Overcast,
    /// The week's footage (8–11 February): broken stratocumulus and a hard low sun.
    Sun,
    /// 8 June 2023: a cirrus veil the sun shows through, thinning to broken cloud by afternoon.
    Veil,
}

impl Weather {
    /// (cover, base height, underside darkness, veil) at this hour.
    pub fn cloud(self, hour: f32) -> Vec4 {
        match self {
            Weather::Overcast => Vec4::new(1.0, 350.0, 0.82, 0.0),
            Weather::Sun => Vec4::new(0.35, 1300.0, 0.78, 0.05),
            // The veil holds until early afternoon, then breaks into altocumulus with blue between.
            Weather::Veil => {
                let breaking = ((hour - 13.0) / 5.0).clamp(0.0, 1.0);
                Vec4::new(0.95 - 0.4 * breaking, 2500.0 + 2000.0 * breaking, 0.85, 1.0 - 0.7 * breaking)
            }
        }
    }
}

#[derive(Component)]
pub struct SkyDome;

fn spawn_dome(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<SkyMaterial>>) {
    // Seen from inside: the sphere's winding is turned so its inner faces are the front.
    let sphere = Sphere::new(9000.0).mesh().uv(48, 24).with_inverted_winding().expect("a sphere's winding inverts");
    commands.spawn((Mesh3d(meshes.add(sphere)), MeshMaterial3d(materials.add(SkyMaterial { params: SkyParams::default() })), Transform::default(), NotShadowCaster, SkyDome));
}

fn follow_camera(cam: Single<&GlobalTransform, With<MainCamera>>, mut dome: Query<&mut Transform, With<SkyDome>>) {
    for mut tf in &mut dome {
        tf.translation = cam.translation();
    }
}

/// The daylight's hand on the sky: colours, the sun, the weather at this hour, the drift of the deck.
#[derive(bevy::ecs::system::SystemParam)]
pub struct Sky<'w, 's> {
    materials: ResMut<'w, Assets<SkyMaterial>>,
    dome: Query<'w, 's, &'static MeshMaterial3d<SkyMaterial>, With<SkyDome>>,
    time: Res<'w, Time>,
    wind: Res<'w, crate::fx::Wind>,
}

impl Sky<'_, '_> {
    /// `horizon` is the fog's colour; `sun` points at the sun; `strength` is how much of it shows at all.
    pub fn set(&mut self, weather: Weather, hour: f32, zenith: Color, horizon: Color, sun: Vec3, sun_colour: Color, strength: f32, light: f32) {
        let Some(h) = self.dome.iter().next() else { return };
        let Some(mut m) = self.materials.get_mut(h) else { return };
        let lin = |c: Color| {
            let l = c.to_linear();
            Vec4::new(l.red, l.green, l.blue, 1.0)
        };
        let t = self.time.elapsed_secs();
        m.params = SkyParams {
            zenith: lin(zenith),
            horizon: lin(horizon),
            sun_dir: sun.normalize_or(Vec3::Y).extend(0.03),
            sun_col: lin(sun_colour).with_w(strength),
            cloud: weather.cloud(hour),
            drift: Vec4::new(self.wind.0.x * t * 0.4, self.wind.0.z * t * 0.4, light, 0.0),
        };
    }
}
