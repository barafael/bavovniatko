//! The feed as a video pipeline. The drone's camera renders into an image; a second, flat camera shows that
//! image through the feed shader on a screen-sized quad, with the interface drawn over it. So the picture can
//! freeze on its last frame and catch up with a jump, or fall to a lower bit rate, the way a jammed digital link
//! does (design/next-items.md §2.6), while the interface stays crisp: the controller draws that, the video does
//! not carry it.

use bav_core::vocab::Mode;
use bav_sim::player::{MainCamera, Shake, View};
use bevy::render::render_resource::ShaderType;
use crate::dev::{self, dev};
use bevy::{
    camera::{ImageRenderTarget, RenderTarget, visibility::RenderLayers},
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

/// The feed's startup: after the game has spawned its camera.
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FeedSetup;

/// Which look the feed shader gives the picture (matched by value in feed.wgsl).
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
    /// The picture as rendered (`BAV_OFF=feed`).
    Raw = 4,
    /// An FPV's analogue goggles.
    Fpv = 5,
}

/// What the feed shader is told each frame.
#[derive(Clone, Copy, ShaderType, Default, Debug)]
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
    /// How much colour the drone camera keeps: the February snow came through nearly grey, June stays green.
    pub saturation: f32,
    /// The link's bit rate, 1 clean: lower is softer, blockier.
    pub bitrate: f32,
    /// Which sight, in mode 3: 0 the Stugna-P panel's TV picture, 1 the Javelin CLU's thermal.
    pub sight: f32,
}

/// The offscreen image the finished picture lands in when running headless (made at the feed's setup).
#[derive(Resource)]
pub struct HeadlessTarget(pub Handle<Image>);

pub struct FeedPlugin;

impl Plugin for FeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FeedMaterial>::default())
            .init_resource::<Link>()
            .add_systems(Startup, setup.in_set(FeedSetup))
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
        "embedded://bav_look/shaders/feed.wgsl".into()
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

/// The 3D picture's most pixels: an integrated GPU fills about this many in time, so a larger screen gets a
/// smaller picture, scaled up by the feed pass (the interface stays sharp).
const PICTURE_PIXELS: f32 = 1920.0 * 1200.0;

/// The 3D picture for a screen of `pw` x `ph` physical and `lw` logical pixels wide: its size, and the scale
/// factor that keeps its logical size the window's, so the cursor and the HUD's markers land where they are seen.
fn picture(pw: u32, ph: u32, lw: f32) -> (u32, u32, f32) {
    let s = dev().scale.unwrap_or_else(|| (PICTURE_PIXELS / (pw * ph).max(1) as f32).sqrt()).clamp(0.25, 1.0);
    let (w, h) = (((pw as f32 * s).round() as u32).max(1), ((ph as f32 * s).round() as u32).max(1));
    (w, h, w as f32 / lw.max(1.0))
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
    let (w, h, scale_factor) = picture(pw, ph, lw);
    info!("feed: the 3D picture is {w}x{h} on a {pw}x{ph} screen");
    let feed = images.add(target(w, h));
    // The drone's camera paints the picture; the flat camera shows it and carries the interface.
    commands.entity(*cam).remove::<IsDefaultUiCamera>().insert(RenderTarget::Image(ImageRenderTarget { handle: feed.clone(), scale_factor }));
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
fn resize(
    mut resized: MessageReader<WindowResized>,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    feed: Option<Res<FeedImage>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    screen: Option<Single<&Mesh2d, With<Screen>>>,
    mut cam: Query<&mut RenderTarget, With<MainCamera>>,
) {
    if resized.read().last().is_none() {
        return;
    }
    let (Some(window), Some(feed), Some(screen)) = (window, feed, screen) else { return };
    let (w, h, scale_factor) = picture(window.physical_width(), window.physical_height(), window.width());
    if let Some(mut img) = images.get_mut(&feed.0)
        && (img.width() != w || img.height() != h)
    {
        *img = target(w, h);
    }
    for mut t in &mut cam {
        *t = RenderTarget::Image(ImageRenderTarget { handle: feed.0.clone(), scale_factor });
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
    fpv: Res<bav_sim::player::Fpv>,
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
        Mode::Fpv => FeedMode::Fpv,
    } as u32;
    fx.grain = match view.mode {
        Mode::Drone | Mode::Fpv => 1.0,
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
    if view.mode == Mode::Fpv {
        // Analogue: the picture thins into snow as the signal goes; gone, it is all snow.
        fx.glitch = 1.0 - fpv.signal + if fpv.control { 0.0 } else { 0.15 };
        fx.down = if fpv.flying { 0.0 } else { 1.0 };
        cam.is_active = fpv.flying;
    }
}
