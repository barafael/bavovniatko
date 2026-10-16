//! The settings overlay (Esc or F10): the picture's qualities and the frame rate, changed while playing and kept
//! between runs in `$XDG_CONFIG_HOME/bavovniatko/settings.txt` (`~/.config/...`).
//!
//! The arrow keys pick a row and change it, Enter steps it on, Esc or F10 closes. Everything applies at once: the
//! camera's anti-aliasing and shadow filter, the sun's shadow maps and cascades, the window, and the look's own
//! settings (`bav_look::LookSettings`: the 3D picture's size, the haze, the exposure, the video look, the particle
//! budget). Only a windowed game reads or writes the file: headless runs (shots, the showcase, the same-day check)
//! keep the picture as designed. The `BAV_*` switches still win over the file for the run they are set on.

use crate::fx::{FxAssets, MAX_PARTICLES};
use bav_core::dev::{self, Aa};
use bav_look::{LookSettings, VideoLook};
use bevy::{
    anti_alias::smaa::Smaa,
    light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, ShadowFilteringMethod},
    prelude::*,
    render::view::Msaa,
    window::{MonitorSelection, PresentMode, PrimaryWindow, WindowMode},
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // Headless there is no window to play in: the overlay, the cap and the file are for the player.
        if dev::dev().headless {
            return;
        }
        app.insert_resource(Settings::load())
            .init_resource::<FrameRate>()
            .add_systems(Startup, spawn_overlay)
            .add_systems(Update, (settings_input, apply, show).chain().in_set(crate::Step::Screen))
            .add_systems(Last, frame_cap);
    }
}

/// A frame-rate cap: the main loop waits out the rest of each frame.
const CAPS: [Option<u32>; 8] = [Some(30), Some(60), Some(90), Some(120), Some(144), Some(165), Some(240), None];
/// The 3D picture's size against the screen's: `None` is the automatic cap near 1920x1200 (`bav_look::feed`).
const SCALES: [Option<f32>; 6] = [None, Some(0.5), Some(0.67), Some(0.75), Some(0.85), Some(1.0)];
const AAS: [Aa; 4] = [Aa::Off, Aa::Msaa2, Aa::Msaa4, Aa::Smaa];
/// Each sun cascade's shadow map, in texels; 0 is no sun shadows.
const SHADOW_MAPS: [u32; 4] = [0, 1024, 2048, 4096];
/// How far the sun's shadows reach: (cascades, the first's far bound, the last's), m.
const SHADOW_REACH: [(usize, f32, f32); 3] = [(2, 350.0, 1400.0), (3, 450.0, 2800.0), (4, 450.0, 4500.0)];
/// How far the haze lets you see, against the diorama's own.
const HAZES: [f32; 4] = [0.6, 1.0, 1.5, 2.5];
const EXPOSURES: [f32; 5] = [-1.0, -0.5, 0.0, 0.5, 1.0];
const VIDEOS: [VideoLook; 3] = [VideoLook::Full, VideoLook::Light, VideoLook::Off];
const PARTICLES: [f32; 3] = [0.35, 0.65, 1.0];

/// The player's settings. Each is an index into its table above, so the overlay steps through them and the file
/// stores small numbers.
#[derive(Resource, Clone, Copy, PartialEq, Debug)]
pub struct Settings {
    cap: usize,
    vsync: bool,
    fullscreen: bool,
    counter: bool,
    scale: usize,
    aa: usize,
    shadow_map: usize,
    shadow_reach: usize,
    soft_shadows: bool,
    haze: usize,
    exposure: usize,
    video: usize,
    particles: usize,
}

impl Default for Settings {
    /// The picture as designed (game/design/perf.md), at 60 frames a second.
    fn default() -> Self {
        Self {
            cap: 1,
            vsync: true,
            fullscreen: false,
            counter: false,
            scale: 0,
            aa: 1,
            shadow_map: 2,
            shadow_reach: 1,
            soft_shadows: true,
            haze: 1,
            exposure: 2,
            video: 0,
            particles: 2,
        }
    }
}

/// One row of the overlay: its label, how many values it has, and how it reads and writes its index.
struct Row {
    label: &'static str,
    get: fn(&Settings) -> usize,
    set: fn(&mut Settings, usize),
    values: fn(usize) -> String,
    count: usize,
}

const fn row(label: &'static str, count: usize, get: fn(&Settings) -> usize, set: fn(&mut Settings, usize), values: fn(usize) -> String) -> Row {
    Row { label, get, set, values, count }
}

fn on_off(i: usize) -> String {
    if i == 1 { "on".into() } else { "off".into() }
}

const ROWS: [Row; 13] = [
    row("Frame rate cap", CAPS.len(), |s| s.cap, |s, i| s.cap = i, |i| CAPS[i].map_or("off".into(), |c| format!("{c} fps"))),
    row("Vertical sync", 2, |s| s.vsync as usize, |s, i| s.vsync = i == 1, on_off),
    row("Display", 2, |s| s.fullscreen as usize, |s, i| s.fullscreen = i == 1, |i| if i == 1 { "fullscreen".into() } else { "window".into() }),
    row("Frame rate counter", 2, |s| s.counter as usize, |s, i| s.counter = i == 1, on_off),
    row("Render scale", SCALES.len(), |s| s.scale, |s, i| s.scale = i, |i| SCALES[i].map_or("auto (up to 1920x1200)".into(), |k| format!("{:.0}%", k * 100.0))),
    row("Anti-aliasing", AAS.len(), |s| s.aa, |s, i| s.aa = i, |i| ["off", "MSAA 2x", "MSAA 4x", "SMAA"][i].into()),
    row("Shadows", SHADOW_MAPS.len(), |s| s.shadow_map, |s, i| s.shadow_map = i, |i| ["off", "low (1024)", "medium (2048)", "high (4096)"][i].into()),
    row("Shadow distance", SHADOW_REACH.len(), |s| s.shadow_reach, |s, i| s.shadow_reach = i, |i| format!("{} ({:.1} km)", ["near", "normal", "far"][i], SHADOW_REACH[i].2 / 1000.0)),
    row("Shadow edges", 2, |s| s.soft_shadows as usize, |s, i| s.soft_shadows = i == 1, |i| if i == 1 { "soft".into() } else { "hard".into() }),
    row("View distance", HAZES.len(), |s| s.haze, |s, i| s.haze = i, |i| ["hazy", "as on the day", "far", "clear"][i].into()),
    row("Brightness", EXPOSURES.len(), |s| s.exposure, |s, i| s.exposure = i, |i| format!("{:+.1} stop", EXPOSURES[i])),
    row("Video look", VIDEOS.len(), |s| s.video, |s, i| s.video = i, |i| ["drone camera", "light grain", "off (as rendered)"][i].into()),
    row("Smoke and fire", PARTICLES.len(), |s| s.particles, |s, i| s.particles = i, |i| ["low", "medium", "full"][i].into()),
];

impl Settings {
    fn path() -> Option<PathBuf> {
        let base = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
        Some(base.join("bavovniatko").join("settings.txt"))
    }

    /// The file's settings over the defaults, then the `BAV_*` switches over both.
    fn load() -> Self {
        let mut s = Self::default();
        if let Some(text) = Self::path().and_then(|p| std::fs::read_to_string(p).ok()) {
            for line in text.lines() {
                let Some((k, v)) = line.split_once('=') else { continue };
                let (k, v) = (k.trim(), v.trim());
                if let (Some(row), Ok(i)) = (ROWS.iter().find(|r| key(r.label) == k), v.parse::<usize>())
                    && i < row.count
                {
                    (row.set)(&mut s, i);
                }
            }
        }
        let d = dev::dev();
        if d.aa.is_some() || dev::off(dev::Part::Msaa) {
            s.aa = AAS.iter().position(|a| *a == dev::aa()).unwrap_or(s.aa);
        }
        if let Some(k) = d.scale {
            s.scale = SCALES.iter().position(|x| x.is_some_and(|x| (x - k).abs() < 0.01)).unwrap_or(s.scale);
        }
        if let Some(m) = d.shadow_map {
            s.shadow_map = SHADOW_MAPS.iter().position(|x| *x == m).unwrap_or(s.shadow_map);
        }
        if dev::off(dev::Part::Shadows) {
            s.shadow_map = 0;
        }
        if dev::off(dev::Part::Feed) {
            s.video = 2;
        }
        s
    }

    fn save(&self) {
        let Some(path) = Self::path() else { return };
        let text: String = ROWS.iter().map(|r| format!("{} = {}\n", key(r.label), (r.get)(self))).collect();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(path, text);
    }
}

/// A row's key in the file: its label in snake case.
fn key(label: &str) -> String {
    label.to_lowercase().replace([' ', '-'], "_")
}

/// The overlay's state: open or not, and the row picked.
#[derive(Component)]
struct Overlay {
    open: bool,
    row: usize,
}
#[derive(Component)]
struct OverlayText;
/// The interface's font, for the rows made as the overlay changes.
#[derive(Resource)]
struct OverlayFont(Handle<Font>);
#[derive(Component)]
struct Counter;

/// Frames as the player sees them: a smoothed rate, and the worst frame of the last second.
#[derive(Resource, Default)]
struct FrameRate {
    mean: f32,
    worst: f32,
    worst_window: f32,
    window_left: f32,
}

fn spawn_overlay(mut commands: Commands, assets: Res<AssetServer>) {
    let font: Handle<Font> = assets.load(crate::text::FONT);
    commands.insert_resource(OverlayFont(font.clone()));
    let tf = |size: f32| TextFont { font: FontSource::Handle(font.clone()), font_size: FontSize::Px(size), ..default() };
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: percent(12),
                left: percent(50),
                margin: UiRect::left(px(-330)),
                width: px(660),
                padding: UiRect::all(px(18)),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.05, 0.06, 0.96)),
            GlobalZIndex(10),
            Overlay { open: dev::dev().settings_open, row: 0 },
        ))
        .with_children(|p| {
            p.spawn((Text::new(""), tf(16.0), TextColor(Color::srgba(0.95, 0.96, 0.97, 0.92)), OverlayText));
        });
    commands.spawn((
        Text::new(""),
        tf(14.0),
        TextColor(Color::srgba(0.95, 0.96, 0.97, 0.92)),
        TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.8) },
        Node { position_type: PositionType::Absolute, top: px(14), left: percent(50), margin: UiRect::left(px(-60)), ..default() },
        Visibility::Hidden,
        Counter,
    ));
}

fn settings_input(keys: Res<ButtonInput<KeyCode>>, phase: Res<State<crate::Phase>>, mut overlay: Single<&mut Overlay>, mut settings: ResMut<Settings>) {
    // At the debrief Esc ends the game (hud::debrief_input): there only F10 opens the overlay.
    let esc = keys.just_pressed(KeyCode::Escape) && *phase.get() != crate::Phase::Debrief;
    if esc || keys.just_pressed(KeyCode::F10) {
        overlay.open = !overlay.open;
    }
    if !overlay.open {
        return;
    }
    let reset = ROWS.len();
    if keys.just_pressed(KeyCode::ArrowDown) {
        overlay.row = (overlay.row + 1) % (ROWS.len() + 1);
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        overlay.row = (overlay.row + ROWS.len()) % (ROWS.len() + 1);
    }
    if overlay.row == reset {
        if keys.just_pressed(KeyCode::Enter) {
            *settings = Settings::default();
        }
        return;
    }
    let r = &ROWS[overlay.row];
    let step = if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::Enter) {
        1
    } else if keys.just_pressed(KeyCode::ArrowLeft) {
        r.count - 1
    } else {
        return;
    };
    let mut s = *settings;
    let next = ((r.get)(&s) + step) % r.count;
    (r.set)(&mut s, next);
    *settings = s;
}

/// Put the settings into the window, the camera, the sun and the look, when they change (and once at start).
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn apply(
    mut commands: Commands,
    settings: Res<Settings>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    camera: Single<Entity, With<crate::player::MainCamera>>,
    mut sun: Query<(&mut DirectionalLight, &mut bevy::light::CascadeShadowConfig)>,
    mut shadow_map: ResMut<DirectionalLightShadowMap>,
    mut look: ResMut<LookSettings>,
    mut fx: ResMut<FxAssets>,
    palette: Option<Res<crate::units::Palette>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut started: Local<bool>,
) {
    if !settings.is_changed() {
        return;
    }
    let s = *settings;
    // The first time is the start: what was loaded goes in, and only a change is written back.
    if *started {
        s.save();
    }
    *started = true;
    window.present_mode = if s.vsync { PresentMode::AutoVsync } else { PresentMode::AutoNoVsync };
    window.mode = if s.fullscreen { WindowMode::BorderlessFullscreen(MonitorSelection::Current) } else { WindowMode::Windowed };
    let aa = AAS[s.aa];
    let mut cam = commands.entity(*camera);
    cam.insert(match aa {
        Aa::Msaa2 => Msaa::Sample2,
        Aa::Msaa4 => Msaa::Sample4,
        Aa::Smaa | Aa::Off => Msaa::Off,
    });
    if aa == Aa::Smaa {
        cam.insert(Smaa::default());
    } else {
        cam.remove::<Smaa>();
    }
    cam.insert(if s.soft_shadows { ShadowFilteringMethod::Gaussian } else { ShadowFilteringMethod::Hardware2x2 });
    // The tree cards' edges are resolved by the multisampling where there is some, and cut hard where not.
    if let Some(palette) = palette
        && let Some(mut m) = materials.get_mut(&palette.branches)
    {
        let want = if matches!(aa, Aa::Msaa2 | Aa::Msaa4) { AlphaMode::AlphaToCoverage } else { AlphaMode::Mask(0.5) };
        if m.alpha_mode != want {
            m.alpha_mode = want;
        }
    }
    let map = SHADOW_MAPS[s.shadow_map];
    if map > 0 {
        shadow_map.size = map as usize;
    }
    let (num_cascades, first_cascade_far_bound, maximum_distance) = SHADOW_REACH[s.shadow_reach];
    for (mut light, mut cascades) in &mut sun {
        light.shadow_maps_enabled = map > 0;
        *cascades = CascadeShadowConfigBuilder { num_cascades, first_cascade_far_bound, maximum_distance, ..default() }.build();
    }
    let l = LookSettings { picture_scale: SCALES[s.scale], haze: HAZES[s.haze], exposure: EXPOSURES[s.exposure], video: VIDEOS[s.video], particles: PARTICLES[s.particles] };
    look.set_if_neq(l);
    fx.budget = (MAX_PARTICLES as f32 * l.particles) as usize;
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn show(
    time: Res<Time<Real>>,
    settings: Res<Settings>,
    mut rate: ResMut<FrameRate>,
    overlay: Single<(&Overlay, &mut Node)>,
    text: Single<Entity, With<OverlayText>>,
    counter: Single<(&mut Text, &mut Visibility), (With<Counter>, Without<OverlayText>)>,
    font: Res<OverlayFont>,
    window: Single<&Window, With<PrimaryWindow>>,
    look: Res<LookSettings>,
    mut commands: Commands,
    mut shown: Local<String>,
    mut tick: Local<f32>,
) {
    let dt = time.delta_secs();
    if dt > 0.0 {
        rate.mean = if rate.mean == 0.0 { dt } else { rate.mean + (dt - rate.mean) * 0.05 };
        rate.worst_window = rate.worst_window.max(dt);
        rate.window_left -= dt;
        if rate.window_left <= 0.0 {
            rate.worst = rate.worst_window;
            rate.worst_window = 0.0;
            rate.window_left = 1.0;
        }
    }
    let fps = if rate.mean > 0.0 { 1.0 / rate.mean } else { 0.0 };
    // The numbers move four times a second: faster is a blur.
    *tick -= dt;
    let fresh = *tick <= 0.0;
    if fresh {
        *tick = 0.25;
    }
    let (mut ctext, mut cvis) = counter.into_inner();
    *cvis = if settings.counter { Visibility::Inherited } else { Visibility::Hidden };
    if fresh {
        ctext.0 = format!("{fps:.0} fps  {:.1} ms", rate.mean * 1000.0);
    }
    let (o, mut node) = overlay.into_inner();
    node.display = if o.open { Display::Flex } else { Display::None };
    if !o.open || !(fresh || settings.is_changed() || shown.is_empty()) {
        return;
    }
    let (pw, ph) = (window.physical_width(), window.physical_height());
    let k = look.picture_scale.or(dev::dev().scale).unwrap_or_else(|| (1920.0 * 1200.0 / (pw * ph).max(1) as f32).sqrt()).clamp(0.25, 1.0);
    let mut lines: Vec<(String, bool)> = vec![
        ("SETTINGS".into(), false),
        (format!("{fps:.0} fps   {:.1} ms a frame, worst {:.1} ms in the last second", rate.mean * 1000.0, rate.worst * 1000.0), false),
        (format!("3D picture {}x{} on a {pw}x{ph} screen", (pw as f32 * k).round(), (ph as f32 * k).round()), false),
        (String::new(), false),
    ];
    for (i, r) in ROWS.iter().enumerate() {
        let picked = i == o.row;
        lines.push((format!("{} {:<20} < {} >", if picked { ">" } else { " " }, r.label, (r.values)((r.get)(&settings))), picked));
    }
    lines.push((format!("{} Reset to defaults", if o.row == ROWS.len() { ">" } else { " " }), o.row == ROWS.len()));
    lines.push((String::new(), false));
    lines.push(("Up/Down pick   Left/Right/Enter change   Esc or F10 close".into(), false));
    let key: String = lines.iter().map(|(l, p)| format!("{p}{l}\n")).collect();
    if *shown == key {
        return;
    }
    *shown = key;
    commands.entity(*text).despawn_children().with_children(|p| {
        for (l, picked) in lines {
            let colour = if picked { Color::srgb(0.95, 0.85, 0.55) } else { Color::srgba(0.95, 0.96, 0.97, 0.92) };
            p.spawn((TextSpan::new(format!("{l}\n")), TextFont { font: FontSource::Handle(font.0.clone()), font_size: FontSize::Px(16.0), ..default() }, TextColor(colour)));
        }
    });
}

/// The frame-rate cap: at the end of the frame, wait out what is left of it. Sleep most of the way and spin the
/// last millisecond, which a sleep overshoots.
fn frame_cap(settings: Res<Settings>, mut last: Local<Option<Instant>>) {
    let Some(fps) = CAPS[settings.cap] else {
        *last = None;
        return;
    };
    let target = Duration::from_secs_f64(1.0 / fps as f64);
    let now = Instant::now();
    let Some(prev) = *last else {
        *last = Some(now);
        return;
    };
    let deadline = prev + target;
    loop {
        let now = Instant::now();
        if now >= deadline {
            break;
        }
        let left = deadline - now;
        if left > Duration::from_micros(1500) {
            std::thread::sleep(left - Duration::from_millis(1));
        } else {
            std::hint::spin_loop();
        }
    }
    // Keep to the beat, but a frame that ran long starts the next from now, not from a deadline long past.
    let now = Instant::now();
    *last = Some(if now - deadline < target { deadline } else { now });
}
