//! The feed as a video pipeline. The drone's camera renders into an image; a second, flat camera shows that
//! image through the feed shader on a screen-sized quad, with the interface drawn over it. So the picture can
//! freeze on its last frame and catch up with a jump, or fall to a lower bit rate, the way a jammed digital link
//! does (design/next-items.md §2.6), while the interface stays crisp: the controller draws that, the video does
//! not carry it.

use crate::camera::{FeedEffect, FeedMode, HeadlessTarget, MainCamera, Mode, Shake, View};
use crate::dev::{self, dev};
use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
    render::{
        render_resource::{AsBindGroup, TextureFormat, TextureUsages},
        view::Msaa,
    },
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d},
    window::{PrimaryWindow, WindowResized},
};
use rand::Rng;

pub struct FeedPlugin;

impl Plugin for FeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FeedMaterial>::default())
            .init_resource::<Link>()
            .add_systems(Startup, setup.after(crate::camera::spawn_camera))
            .add_systems(Update, resize.in_set(crate::Step::Control));
    }
}

/// The screen-sized quad's material: the drone camera's picture, and the look the shader gives it.
#[derive(Asset, AsBindGroup, TypePath, Clone)]
pub struct FeedMaterial {
    #[uniform(0)]
    pub fx: FeedEffect,
    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}

impl Material2d for FeedMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bavovniatko/shaders/feed.wgsl".into()
    }
}

/// The video link's state: a freeze in progress, and when to roll for the next.
#[derive(Resource, Default)]
pub struct Link {
    pub frozen_until: f64,
    next_roll: f64,
    next_beep: f64,
}

/// The quad the picture is shown on.
#[derive(Component)]
pub struct Screen;

/// The image the drone's camera renders into.
#[derive(Resource)]
pub struct FeedImage(Handle<Image>);

fn target(w: u32, h: u32) -> Image {
    Image::new_target_texture(w.max(1), h.max(1), TextureFormat::Rgba8UnormSrgb, None)
}

fn setup(
    mut commands: Commands,
    cam: Single<Entity, With<MainCamera>>,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FeedMaterial>>,
) {
    let (pw, ph, lw, lh) = match (&window, dev().headless) {
        (Some(w), false) => (w.physical_width(), w.physical_height(), w.width(), w.height()),
        _ => {
            let (w, h) = dev().res.unwrap_or((1600, 900));
            (w, h, w as f32, h as f32)
        }
    };
    let feed = images.add(target(pw, ph));
    // The drone's camera paints the picture; the flat camera shows it and carries the interface.
    commands.entity(*cam).remove::<IsDefaultUiCamera>().insert(RenderTarget::Image(feed.clone().into()));
    let mut flat = commands.spawn((Camera2d, Camera { order: 1, clear_color: ClearColorConfig::Custom(Color::BLACK), ..default() }, IsDefaultUiCamera, RenderLayers::layer(1), Msaa::Off));
    if dev().headless {
        let mut img = target(pw, ph);
        img.texture_descriptor.usage |= TextureUsages::COPY_SRC;
        let fin = images.add(img);
        flat.insert(RenderTarget::Image(fin.clone().into()));
        commands.insert_resource(HeadlessTarget(fin));
    }
    let fx = FeedEffect { grain: 1.0, zoom: 1.0, bitrate: 1.0, saturation: if crate::diorama::diorama().summer() { 0.68 } else { 0.45 }, ..default() };
    commands.spawn((Mesh2d(meshes.add(Rectangle::new(lw, lh))), MeshMaterial2d(materials.add(FeedMaterial { fx, image: feed.clone() })), Transform::default(), RenderLayers::layer(1), Screen));
    commands.insert_resource(FeedImage(feed));
}

/// The window changed size: the picture and the quad follow it.
fn resize(mut resized: MessageReader<WindowResized>, window: Option<Single<&Window, With<PrimaryWindow>>>, feed: Option<Res<FeedImage>>, mut images: ResMut<Assets<Image>>, mut meshes: ResMut<Assets<Mesh>>, screen: Option<Single<&Mesh2d, With<Screen>>>) {
    if resized.read().last().is_none() {
        return;
    }
    let (Some(window), Some(feed), Some(screen)) = (window, feed, screen) else { return };
    let (pw, ph) = (window.physical_width(), window.physical_height());
    if let Some(mut img) = images.get_mut(&feed.0)
        && (img.width() != pw || img.height() != ph)
    {
        *img = target(pw, ph);
    }
    if let Some(mut mesh) = meshes.get_mut(&screen.0) {
        *mesh = Rectangle::new(window.width(), window.height()).into();
    }
}

/// What the shader is told this frame, and whether the picture moves at all. Under jamming the link climbs a
/// ladder: soft and smeary, then blocks and a halved frame rate, then freezes of a fraction of a second to two
/// seconds that catch up with a jump; past the jam line it is gone.
pub fn update_feed(
    time: Res<Time>,
    view: Res<View>,
    shake: Res<Shake>,
    mut link: ResMut<Link>,
    mut levels: ResMut<crate::audio::AmbienceLevels>,
    mut sound: crate::audio::Sound,
    mut fxa: ResMut<crate::fx::FxAssets>,
    mut cam: Single<&mut Camera, With<MainCamera>>,
    screen: Option<Single<&MeshMaterial2d<FeedMaterial>, With<Screen>>>,
    mut materials: ResMut<Assets<FeedMaterial>>,
    teams: Query<&crate::fire::Post>,
) {
    let Some(screen) = screen else { return };
    let Some(mut m) = materials.get_mut(&screen.0) else { return };
    let now = time.elapsed_secs_f64();
    let fx = &mut m.fx;
    fx.time = time.elapsed_secs() % 1000.0;
    fx.mode = match view.mode {
        _ if dev::off(dev::Part::Feed) => FeedMode::Raw,
        Mode::Drone => FeedMode::Drone,
        Mode::Op => FeedMode::Binoculars,
        Mode::Ground if view.sight => FeedMode::Sight,
        Mode::Ground => FeedMode::Eye,
    } as u32;
    fx.grain = match view.mode {
        Mode::Drone => 1.0,
        Mode::Op => 0.45,
        Mode::Ground => 0.3,
    };
    fx.sight = if view.ground_team.and_then(|e| teams.get(e).ok()).and_then(|p| p.weapon) == Some(crate::fire::Atgm::Javelin) { 1.0 } else { 0.0 };
    let jam = if view.mode == Mode::Drone { view.jamming } else { 0.0 };
    fx.glitch = jam.max(if view.mode == Mode::Drone && view.battery < 0.08 { 0.3 } else { 0.0 });
    fx.bitrate = 1.0 - jam;
    if now >= link.next_roll {
        link.next_roll = now + 0.25;
        let r = &mut fxa.rng;
        if jam > 0.3 && r.random_bool(((jam - 0.3) * 0.5) as f64) {
            link.frozen_until = now + r.random_range(0.3..2.0_f64) * (0.5 + jam as f64);
        }
    }
    let frozen = view.mode == Mode::Drone && (now < link.frozen_until || view.feed_down > 0.0);
    cam.is_active = !frozen;
    // What the operator hears of it: the controller's warning every two seconds, and the hiss and the dropouts.
    levels.jam = jam;
    levels.frozen = frozen;
    if view.mode == Mode::Drone && (jam > 0.3 || !view.control || view.rth.is_some()) && now >= link.next_beep {
        link.next_beep = now + 2.0;
        sound.now(crate::audio::Sfx::Beep, 0.25);
    }
    fx.flash = shake.flash * if view.mode == Mode::Ground { 0.35 } else { 1.0 };
    fx.zoom = view.zoom();
    fx.down = if view.mode == Mode::Drone && view.feed_down > 0.0 { 1.0 } else { 0.0 };
}
