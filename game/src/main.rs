//! Vuhledar, 6 February 2023: the first diorama of bavovniatko.
//!
//! The history is in research/chapters/c08-vuhledar.md. Russian columns drive the same dirt roads into the
//! 72nd Brigade's minefields, ATGM teams in the tree lines and drone-spotted artillery. The world runs by itself;
//! the player, a Ukrainian drone operator and fire controller, matters at the margin.

mod audio;
mod camera;
mod dev;
mod diorama;
mod director;
mod fire;
mod fx;
mod geo;
mod hud;
mod meshes;
mod robotyne;
mod showcase;
mod terrain;
mod units;
mod world;

use bevy::{
    app::ScheduleRunnerPlugin,
    audio::{AudioPlugin, GlobalVolume, Volume},
    prelude::*,
    window::{ExitCondition, PresentMode},
    winit::WinitPlugin,
};

fn main() {
    let mut app = App::new();
    // VUHLEDAR_HEADLESS: no window and no sound; the camera renders offscreen on the GPU (for screenshots).
    if dev::dev().headless {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: None, exit_condition: ExitCondition::DontExit, ..default() })
                .set(AudioPlugin { global_volume: GlobalVolume::new(Volume::SILENT), ..default() })
                .disable::<WinitPlugin>(),
        );
        // Filming runs on a fixed game step, and measuring wants every frame the machine can make, so neither
        // waits for wall-clock frames.
        let wait = if dev::dev().showcase.is_some() || dev::dev().diag { 0.0 } else { 1.0 / 60.0 };
        app.add_plugins(ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f64(wait)));
    } else {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: diorama::diorama().title().into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }));
    }
    if dev::dev().diag {
        app.add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin);
    }
    // The two shaders live in the binary, so the game runs from anywhere with no asset folder.
    bevy::asset::embedded_asset!(app, "shaders/feed.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/ground.wgsl");
    app.configure_sets(Update, (Step::Control, Step::Camera, Step::Script, Step::Fire, Step::Units, Step::Effects, Step::Screen).chain())
        .init_state::<Phase>()
        .init_resource::<audio::AmbienceLevels>()
        .add_plugins((terrain::TerrainPlugin, world::WorldPlugin, camera::CameraPlugin, fx::FxPlugin, audio::AudioPlugin))
        .add_plugins((units::UnitsPlugin, fire::FirePlugin, director::DirectorPlugin, hud::HudPlugin))
        .add_observer(units::on_wrecked)
        .add_systems(Update, drone_view_audio.in_set(Step::Camera))
        .add_plugins((dev::DevPlugin, showcase::ShowcasePlugin, robotyne::RobotynePlugin))
        .run();
}

/// Each frame's game systems run in this order, so a day plays out the same way every time it gets the same
/// input (the showcase films it twice alike): what steers the frame (showcase, dev switches, keys), the camera,
/// the day's script, the fire, the units, then what only shows or plays the result.
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Step {
    Control,
    Camera,
    Script,
    Fire,
    Units,
    Effects,
    Screen,
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Phase {
    #[default]
    Briefing,
    Playing,
    Debrief,
}

/// The longest simulation step: a slow frame is simulated as several short ones' worth, not one long jump.
pub const MAX_STEP: f32 = 0.05;

/// This frame's simulation step.
pub fn sim_dt(time: &Time) -> f32 {
    time.delta_secs().min(MAX_STEP)
}

/// The battle systems run only while the day is being played.
pub fn running(phase: Res<State<Phase>>) -> bool {
    *phase.get() == Phase::Playing
}

fn drone_view_audio(view: Res<camera::View>, mut levels: ResMut<audio::AmbienceLevels>) {
    levels.drone_view = view.mode == camera::Mode::Drone;
}
