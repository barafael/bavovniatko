//! The battle. Everything that decides how a day goes: the columns and the men, mines, fire of every kind, the
//! day's script, the Ka-52s, and where the player's drone and posts are. It runs in the frame's `Step` order and
//! draws only from its own random streams, so a day replays byte for byte; it never draws a picture or plays a
//! sound. It asks for effects, ground marks and sounds through queues (`fx::FxQueue`, `paint::PaintQueue`,
//! `sound::Pending`) that `bav_look` plays, and it dresses what it spawns with model handles the look builds.

pub use bav_core::schedule::{MAX_STEP, Phase, Step, running, sim_dt};
pub use bav_core::{dev, diorama, geo, schedule, text, vocab};

pub mod allot;
pub mod columns;
pub mod data;
pub mod director;
pub mod drones;
pub mod evac;
pub mod ew;
pub mod fire;
pub mod fx;
pub mod hits;
pub mod orders;
pub mod paint;
pub mod player;
pub mod robotyne;
pub mod sound;
pub mod supply;
pub mod units;

use bevy::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};

/// The battle's shared resources: its effects stream and the queues the picture and the sound drain. The battle's
/// systems come with their own plugins (`units`, `fire`, `director`, `robotyne`).
pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(fx::FxRng(SmallRng::seed_from_u64(dev::seeded(20230206))))
            .insert_resource(fx::Wind(fx::day_wind()))
            .init_resource::<fx::Smoke>()
            .init_resource::<fx::FxQueue>()
            .init_resource::<paint::PaintQueue>()
            .init_resource::<sound::Pending>()
            .init_resource::<sound::AmbienceLevels>()
            .init_resource::<hits::HitLog>();
    }
}
