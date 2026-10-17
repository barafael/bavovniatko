//! What the battle makes heard: each sound queued at the moment it is made, from where, and when it reaches the
//! listener (the camera) at 343 m/s. `bav_look::audio` synthesises the sounds and plays them as they fall due;
//! the mix levels the battle sets ([`AmbienceLevels`]) are here too.

use crate::player::MainCamera;
use bevy::{ecs::system::SystemParam, prelude::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::VariantArray)]
pub enum Sfx {
    BlastNear,
    BlastFar,
    Thermobaric,
    MineBlast,
    TankGun,
    Cannon30,
    MachineGun,
    AtgmLaunch,
    Whistle,
    CookOff,
    Radio,
    /// The controller's warning tone.
    Beep,
    /// Reactive armour blocks going off: a hard, bright crack.
    EraCrack,
    /// A gun battery firing, heard from kilometres off: the report that comes before (or after) its shells land.
    GunReport,
    /// Rockets leaving their launcher one after another.
    Ripple,
    /// A Ka-52's Vikhr leaving the rail.
    VikhrLaunch,
}

#[derive(Resource, Default)]
pub struct Pending {
    /// (when it is heard, what, volume, playback speed).
    pub queue: Vec<(f64, Sfx, f32, f32)>,
}

/// What a system needs to make a sound: the clock, the queue, and where the listener (the camera) is.
#[derive(SystemParam)]
pub struct Sound<'w, 's> {
    time: Res<'w, Time>,
    pending: ResMut<'w, Pending>,
    camera: Single<'w, 's, &'static GlobalTransform, With<MainCamera>>,
}

impl Sound<'_, '_> {
    pub fn listener(&self) -> Vec3 {
        self.camera.translation()
    }

    /// Play `sfx` from `pos`, heard at the camera after the sound's travel time, attenuated with distance.
    pub fn at(&mut self, pos: Vec3, sfx: Sfx, loudness: f32) {
        let now = self.time.elapsed_secs_f64();
        self.at_time(now, pos, sfx, loudness);
    }

    /// As [`Sound::at`], for a sound made at game time `t`.
    pub fn at_time(&mut self, t: f64, pos: Vec3, sfx: Sfx, loudness: f32) {
        let d = self.listener().distance(pos);
        let delay = d as f64 / 343.0;
        // Distant blasts lose their crack: switch to the far variant.
        let sfx = if sfx == Sfx::BlastNear && d > 900.0 { Sfx::BlastFar } else { sfx };
        let vol = loudness * (80.0 / (80.0 + d)).powf(1.2);
        if vol < 0.004 {
            return;
        }
        let speed = 1.0 - (d / 6000.0).min(0.15);
        self.pending.queue.push((t + delay, sfx, vol, speed));
    }

    /// Play without delay (radio, UI).
    pub fn now(&mut self, sfx: Sfx, vol: f32) {
        let now = self.time.elapsed_secs_f64();
        self.pending.queue.push((now, sfx, vol, 1.0));
    }
}

/// What the player's view does to the ambience: the rotor whine is only there in the drone view, and jamming
/// thins the world behind a hiss. (What is near the camera, engines and fires, the look finds for itself.)
#[derive(Resource, Default)]
pub struct AmbienceLevels {
    pub drone_view: bool,
    /// The jamming on the drone's link, 0..1, and whether its picture is frozen this frame.
    pub jam: f32,
    pub frozen: bool,
}
