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
pub mod meshes;
pub mod models;
pub mod robotyne;
pub mod sky;
pub mod terrain;
pub mod world;

use bevy::prelude::*;

/// The shaders, embedded so the game runs from anywhere with no asset folder.
pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        bevy::asset::embedded_asset!(app, "shaders/feed.wgsl");
        bevy::asset::embedded_asset!(app, "shaders/ground.wgsl");
        bevy::asset::embedded_asset!(app, "shaders/sky.wgsl");
    }
}
