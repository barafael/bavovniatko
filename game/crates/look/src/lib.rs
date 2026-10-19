//! The picture and the sound. The ground and its paint, the sky and the light, the shelterbelts, towns and sets,
//! particles, fire and smoke, the video feed, the models, and the synthesised sound. It reads the battle
//! (`bav_sim`) and plays what the battle queues, from its own random stream; nothing here changes how a day goes.

pub use bav_core::schedule::{Step, running, sim_dt};
pub use bav_core::{dev, diorama, geo, schedule, text, vocab};
pub use bav_sim::{director, fire, player, units};

pub mod audio;
pub mod baked;
pub mod feed;
pub mod fx;
pub mod hostomel;
pub mod meshes;
pub mod models;
pub mod robotyne;
pub mod sky;
pub mod terrain;
pub mod trees;
pub mod world;

use bevy::prelude::*;

/// What the player has set of the picture (the settings overlay, in the game crate). The defaults are the picture
/// as designed; headless runs (shots, the showcase, the regression runs) never change them.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct LookSettings {
    /// The 3D picture's size against the screen's (0.25..1); `None` caps it near 1920x1200 (`feed::picture`).
    pub picture_scale: Option<f32>,
    /// How far the haze lets you see, against the diorama's own visibility.
    pub haze: f32,
    /// Exposure, in stops over the camera's.
    pub exposure: f32,
    pub video: VideoLook,
    /// The share of the particle budget smoke, dust and fire may use.
    pub particles: f32,
}

impl Default for LookSettings {
    fn default() -> Self {
        Self { picture_scale: None, haze: 1.0, exposure: 0.0, video: VideoLook::Full, particles: 1.0 }
    }
}

/// How much of the drone camera's video look the feed lays over the picture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoLook {
    /// Compression, grain and the small sensor's colour, as designed.
    Full,
    /// The same with a third of the grain.
    Light,
    /// The picture as rendered.
    Off,
}

/// The shaders, embedded so the game runs from anywhere with no asset folder.
pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LookSettings>();
        bevy::asset::embedded_asset!(app, "shaders/feed.wgsl");
        bevy::asset::embedded_asset!(app, "shaders/ground.wgsl");
        bevy::asset::embedded_asset!(app, "shaders/sky.wgsl");
        bevy::asset::embedded_asset!(app, "shaders/vehicle.wgsl");
    }
}
