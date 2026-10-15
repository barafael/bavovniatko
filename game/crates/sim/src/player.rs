//! Where the player is and what they hold: the drone in flight with its battery, link and position fix, the
//! observers' post, the ATGM team's sight, and the camera that shows it. The battle reads it (a drone over a
//! target tightens the sheaf, the TOS can take the OP); the game's camera flies it and puts the picture there.

use crate::diorama::{Diorama, diorama};
use crate::geo;
use bav_core::vocab::Mode;
use bevy::prelude::*;

/// The camera that shows the field, and the listener every sound is heard at.
#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct View {
    pub mode: Mode,
    pub drone_pos: Vec3,
    pub drone_yaw: f32,
    pub gimbal: f32,
    pub drone_fov: f32,
    pub op_yaw: f32,
    pub op_pitch: f32,
    pub op_fov: f32,
    pub battery: f32,
    /// Seconds the feed is down (drone lost or swapping battery).
    pub feed_down: f32,
    pub drones_left: u32,
    /// Signal degradation from Russian EW, 0..1.
    pub jamming: f32,
    pub op_alive: bool,
    // Drone flight state: it is an aircraft, not a camera rig.
    pub drone_vel: Vec3,
    pub yaw_target: f32,
    pub gimbal_target: f32,
    pub fov_target: f32,
    pub climb: f32,
    /// Visual attitude disturbance from manoeuvres (yaw, pitch, roll).
    pub tilt: Vec3,
    /// Where the position hold keeps the drone while the sticks are centred.
    pub hold: Option<Vec2>,
    // Ground view.
    pub ground_team: Option<Entity>,
    pub ground_eye: Vec3,
    pub ground_yaw: f32,
    pub ground_pitch: f32,
    pub sight: bool,
    /// Range to whatever the sight's crosshair is on.
    pub ground_range: Option<f32>,
    /// Javelin: the vehicle the seeker is tracking and for how long (it locks after 2.5 s).
    pub lock: Option<(Entity, f32)>,
    /// The view was placed by hand this frame (the showcase): the flight controller and the ground view's own
    /// controls stand down.
    pub locked: bool,
    /// The showcase's ground shot fires this frame, beside the mouse and `BAV_AUTO_ATGM`.
    pub launch: bool,
    /// The control link (the video link is the feed's own): up, and for how long it has been down.
    pub control: bool,
    pub lost_for: f32,
    /// The drone flying itself home after six seconds without control; it stays on that course until cancelled.
    pub rth: Option<Rth>,
    /// The satellite fix, 1 clean, and the error it puts on the position readout.
    pub gnss: f32,
    pub gnss_error: Vec2,
    /// The last direction it was flying, to back down when the link goes.
    pub track: Vec3,
}

/// The return-to-home sequence of a DJI drone without its controller: brake and hover, back 50 m along the
/// track it came on, then straight home at the return height, and hover there.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rth {
    Brake(f32),
    Back { from: Vec3 },
    Home,
}

impl Default for View {
    fn default() -> Self {
        // At Vuhledar over the town looking south across the fields; at Mala Tokmachka over the forward tree
        // line, looking south up the slope to the crest.
        let drone_pos = match diorama() {
            Diorama::Vuhledar => Vec3::new(150.0, 0.0, -700.0),
            Diorama::Robotyne => Vec3::new(80.0, 0.0, -1150.0),
        };
        Self {
            mode: Mode::Drone,
            drone_pos,
            drone_yaw: std::f32::consts::PI,
            gimbal: -0.62,
            drone_fov: 52f32.to_radians(),
            op_yaw: 3.6,
            op_pitch: -0.07,
            op_fov: 55f32.to_radians(),
            battery: 1.0,
            feed_down: 0.0,
            drones_left: 3,
            jamming: 0.0,
            op_alive: true,
            drone_vel: Vec3::ZERO,
            yaw_target: std::f32::consts::PI,
            gimbal_target: -0.62,
            fov_target: 52f32.to_radians(),
            climb: 0.0,
            tilt: Vec3::ZERO,
            hold: None,
            ground_team: None,
            ground_eye: Vec3::ZERO,
            ground_yaw: std::f32::consts::PI,
            ground_pitch: 0.0,
            sight: false,
            ground_range: None,
            lock: None,
            locked: false,
            launch: false,
            control: true,
            lost_for: 0.0,
            rth: None,
            gnss: 1.0,
            gnss_error: Vec2::ZERO,
            track: Vec3::NEG_Z,
        }
    }
}

impl View {
    /// Height above ground the drone holds.
    pub const DRONE_AGL: f32 = 190.0;

    pub fn zoom(&self) -> f32 {
        match self.mode {
            Mode::Drone => 52f32.to_radians() / self.drone_fov,
            Mode::Op => 55f32.to_radians() / self.op_fov,
            Mode::Ground => if self.sight { Self::SIGHT_ZOOM } else { 1.0 },
        }
    }

    pub const GROUND_FOV: f32 = 0.95;
    /// Magnification of the ATGM sight.
    pub const SIGHT_ZOOM: f32 = 10.0;
}

/// Camera shake and white-out from nearby blasts; decays on its own.
#[derive(Resource, Default)]
pub struct Shake {
    pub trauma: f32,
    pub flash: f32,
}

/// Where the observers watch from: the roof of the town's front-row block at Vuhledar, the water tower south of
/// Mala Tokmachka. Each diorama's set inserts it at startup, so it is always there once the day runs.
#[derive(Resource)]
pub struct ObservationPost {
    pub eye: Vec3,
    pub building: Vec2,
}

/// Ray from the cursor into the terrain.
pub fn cursor_ground(window: &Window, camera: &Camera, cam_tf: &GlobalTransform) -> Option<Vec3> {
    let cursor = window.cursor_position()?;
    let ray = camera.viewport_to_world(cam_tf, cursor).ok()?;
    geo::raycast(ray.origin, *ray.direction, 9000.0)
}
