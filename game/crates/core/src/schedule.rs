//! The order of a frame, the phases of a day, and the simulation step.

use bevy::prelude::*;

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
