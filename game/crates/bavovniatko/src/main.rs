//! Vuhledar, 6 February 2023: the first diorama of bavovniatko.
//!
//! The history is in db/research/chapters/c08-vuhledar.md. Russian columns drive the same dirt roads into the
//! 72nd Brigade's minefields, ATGM teams in the tree lines and drone-spotted artillery. The world runs by itself;
//! the player, a Ukrainian drone operator and fire controller, matters at the margin.

mod camera;
mod dev;
mod hud;
mod showcase;

pub use bav_core::schedule::{MAX_STEP, Phase, Setup, Step, running, sim_dt};
pub use bav_core::{diorama, geo, text};
pub use bav_look::{audio, baked, feed, fx, meshes, sky, terrain, world};
pub use bav_sim::{director, fire, player, robotyne, units};

use bevy::{
    app::ScheduleRunnerPlugin,
    audio::{AudioPlugin, GlobalVolume, Volume},
    prelude::*,
    window::{ExitCondition, PresentMode},
    winit::WinitPlugin,
};

fn main() {
    let mut app = App::new();
    // BAV_HEADLESS: no window and no sound; the camera renders offscreen on the GPU (for screenshots).
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
                title: diorama::diorama().title(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }));
    }
    if dev::dev().diag {
        app.add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin);
    }
    // The font lives in the binary, as the shaders do (bav_look), so the game runs from anywhere with no asset folder.
    bevy::asset::embedded_asset!(app, "fonts/LiberationMono-Regular.ttf");
    app.configure_sets(Update, (Step::Control, Step::Camera, Step::Script, Step::Fire, Step::Units, Step::Effects, Step::Screen).chain())
        .configure_sets(Startup, (Setup::Kit, Setup::Field, Setup::Dress).chain())
        .init_state::<Phase>()
        .add_plugins((bav_sim::SimPlugin, bav_look::ShadersPlugin))
        .add_plugins((terrain::TerrainPlugin, world::WorldPlugin, sky::SkyPlugin, camera::CameraPlugin, feed::FeedPlugin, fx::FxPlugin, audio::AudioPlugin))
        .add_plugins((units::UnitsPlugin, fire::FirePlugin, director::DirectorPlugin, hud::HudPlugin))
        .add_observer(units::on_wrecked)
        .add_systems(Update, drone_view_audio.in_set(Step::Camera))
        .add_plugins((dev::DevPlugin, showcase::ShowcasePlugin, robotyne::RobotynePlugin, bav_look::robotyne::RobotyneLookPlugin, bav_sim::evac::EvacPlugin, bav_sim::drones::DronesPlugin))
        .run();
}

fn drone_view_audio(view: Res<camera::View>, mut levels: ResMut<audio::AmbienceLevels>) {
    levels.drone_view = view.mode == camera::Mode::Drone;
}
