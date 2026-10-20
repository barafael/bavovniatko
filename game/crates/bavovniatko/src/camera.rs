//! Three ways of seeing the field.
//!
//! - **The drone** (default): a Mavic/Matrice-class quadcopter hovering behind the Ukrainian lines, the way most
//!   of the world saw Vuhledar, as grey, grainy video with a crosshair and telemetry. Its battery drains.
//! - **The OP**: the observer on the roof of a nine-storey block at the town's southern edge, with binoculars.
//!   Closer to the noise, and inside the TOS-1A's target area.
//! - **The tree line**: eye level behind an ATGM team's launcher, and through its sight.

use crate::dev::{self, dev};
use crate::geo;
use crate::units::{SState, Side, Soldier, Vehicle, vehicle_yaw};
pub use bav_look::feed::{FeedSetup, HeadlessTarget};
pub use bav_sim::player::{Fpv, MainCamera, ObservationPost, Operator, Rth, Shake, View};
use bevy::{
    camera::{Exposure, ShadowLodOrigin},
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    pbr::{DistanceFog, FogFalloff},
    prelude::*,
    render::view::Msaa,
};
pub use bav_core::vocab::{Mode, Target};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<View>()
            .init_resource::<Shake>()
            .add_systems(Startup, spawn_camera.before(FeedSetup))
            .add_systems(Startup, (spawn_launcher, spawn_igla).after(spawn_camera))
            .add_systems(Update, (switch_view, igla_view, drone_control, fpv_control, op_control, ground_control, igla_control, apply_view, crate::feed::update_feed, cursor_grab, show_launcher).chain().in_set(crate::Step::Camera));
    }
}

/// Yaw and pitch that look along `d` (yaw 0 faces -Z).
pub fn look_angles(d: Vec3) -> (f32, f32) {
    (f32::atan2(-d.x, -d.z), f32::atan2(d.y, d.xz().length()))
}

/// The middle of the densest group among these men: the one with the most others within 60 m, and those
/// others. The mean of everyone often lands on empty field between groups.
fn densest(pts: &[Vec3]) -> Option<Vec3> {
    const NEAR: f32 = 60.0;
    let best = pts.iter().max_by_key(|p| pts.iter().filter(|q| q.xz().distance(p.xz()) < NEAR).count())?;
    let group: Vec<Vec3> = pts.iter().copied().filter(|q| q.xz().distance(best.xz()) < NEAR).collect();
    Some(group.iter().copied().sum::<Vec3>() / group.len() as f32)
}

/// Where a [`Target`] is now: the world as the camera sees it.
pub trait Locate {
    fn locate(self, near: Vec3, vehicles: &Query<(&Vehicle, &Transform)>, men: &Query<(&Soldier, &Transform)>) -> Option<Vec3>;
}

impl Locate for Target {
    fn locate(self, near: Vec3, vehicles: &Query<(&Vehicle, &Transform)>, men: &Query<(&Soldier, &Transform)>) -> Option<Vec3> {
        match self {
            Target::Point(p) => Some(geo::ground(p)),
            Target::Vehicle(idx) => vehicles.iter().filter(|(v, _)| v.idx == idx && v.side == crate::diorama::diorama().attacker()).min_by_key(|(v, _)| v.wave).map(|(_, t)| t.translation),
            Target::Wave(wave, idx) => vehicles.iter().filter(|(v, _)| v.side == crate::diorama::diorama().attacker() && v.wave == wave && v.idx == idx).map(|(_, t)| t.translation).next(),
            Target::Men => {
                let pts: Vec<Vec3> = men
                    .iter()
                    .filter(|(s, t)| s.side == crate::diorama::diorama().attacker() && s.state != SState::Flee && t.translation.distance(near) < 1400.0)
                    .map(|(_, t)| t.translation)
                    .collect();
                densest(&pts)
            }
            Target::MenNear(p) => {
                let at = geo::ground(p);
                let pts: Vec<Vec3> = men
                    .iter()
                    .filter(|(s, t)| s.side == crate::diorama::diorama().attacker() && s.state != SState::DugIn && t.translation.distance(at) < 500.0)
                    .map(|(_, t)| t.translation)
                    .collect();
                Some(densest(&pts).unwrap_or(at))
            }
            Target::Hit | Target::Pool | Target::Grass => None,
        }
    }
}


pub fn spawn_camera(mut commands: Commands, mut view: ResMut<View>) {
    view.drone_pos.y = geo::height(view.drone_pos.x, view.drone_pos.z) + View::DRONE_AGL;
    let cam = commands.spawn((
        Camera3d::default(),
        MainCamera,
        Projection::Perspective(PerspectiveProjection { fov: view.drone_fov, near: 0.5, far: 12_000.0, ..default() }),
        Transform::from_translation(view.drone_pos),
        match dev::aa() {
            dev::Aa::Msaa4 => Msaa::Sample4,
            dev::Aa::Msaa2 => Msaa::Sample2,
            dev::Aa::Smaa | dev::Aa::Off => Msaa::Off,
        },
        Tonemapping::AgX,
        Exposure { ev100: 11.25 },
        DistanceFog {
            color: Color::srgb(0.70, 0.72, 0.75),
            directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.15),
            directional_light_exponent: 12.0,
            // A frosty, clear morning ("visibility good"): the haze takes the far belts and the horizon, not the
            // field a drone works at a kilometre or so.
            falloff: FogFalloff::from_visibility_colors(6000.0, Color::srgb(0.48, 0.5, 0.54), Color::srgb(0.72, 0.74, 0.77)),
        },
        // Point-light shadows are detailed around the drone (no camera renders to a window: bav_look::feed shows the
        // picture).
        ShadowLodOrigin,
        // A soft shadow filter. The hardware 2×2 comparison was the cheapest, but zoomed in, a shadow texel is several
        // pixels across and its stair-stepped edge showed, and crawled whenever the sun or the gimbal moved. This one
        // reads a few more texels a pixel for an edge with no steps in it.
        bevy::light::ShadowFilteringMethod::Gaussian,
    )).id();
    if dev::aa() == dev::Aa::Smaa {
        commands.entity(cam).insert(bevy::anti_alias::smaa::Smaa::default());
    }
}

fn switch_view(keys: Res<ButtonInput<KeyCode>>, mut view: ResMut<View>, teams: Query<(Entity, &crate::fire::Post, &crate::fire::AtgmTeam)>) {
    // In the goggles the pilot cannot look anywhere else until the FPV is gone.
    if view.mode == Mode::Fpv {
        return;
    }
    if keys.just_pressed(KeyCode::Digit1) {
        view.mode = Mode::Drone;
    }
    if keys.just_pressed(KeyCode::Digit2) && view.op_alive {
        view.mode = Mode::Op;
    }
    if keys.just_pressed(KeyCode::Digit3) {
        // Into a tree line with an ATGM team; pressing again moves on to the next team still in action.
        let mut live: Vec<(Entity, Vec2)> = teams.iter().filter(|(_, p, t)| p.side == Side::Ua && t.atgm > 0 && p.strength >= 2).map(|(e, p, _)| (e, p.pos)).collect();
        live.sort_by(|a, b| a.1.x.total_cmp(&b.1.x));
        if !live.is_empty() {
            let i = match (view.mode, view.ground_team) {
                (Mode::Ground, Some(cur)) => live.iter().position(|(e, _)| *e == cur).map(|i| (i + 1) % live.len()).unwrap_or(0),
                _ => live.iter().position(|(e, _)| Some(*e) == view.ground_team).unwrap_or(0),
            };
            let (e, p) = live[i];
            enter_ground(&mut view, e, p);
        }
    }
    if keys.just_pressed(KeyCode::Tab) {
        view.mode = match view.mode {
            Mode::Drone if view.op_alive => Mode::Op,
            _ => Mode::Drone,
        };
    }
    if !view.op_alive && view.mode == Mode::Op {
        view.mode = Mode::Drone;
    }
}

/// Kneel in the trench at the south edge of the team's tree line, facing the fields.
pub fn enter_ground(view: &mut View, team: Entity, pos: Vec2) {
    let at = pos + Vec2::new(1.5, 8.5);
    view.mode = Mode::Ground;
    view.ground_team = Some(team);
    // Eye on the launcher's sight, set up on the trench parapet.
    view.ground_eye = geo::ground(at) + Vec3::Y * 1.4;
    // Face the nearest road's kill zone.
    let kz = geo::layout().routes.iter().map(|r| r.line.point(r.deploy_s)).min_by(|a, b| a.distance(at).total_cmp(&b.distance(at))).unwrap_or(at + Vec2::Y);
    let d = kz - at;
    view.ground_yaw = vehicle_yaw(d);
    view.ground_pitch = -0.01;
    view.sight = false;
    view.lock = None;
}

/// The ground view's gunner tracks the nearest live attacker vehicle within 3.5 km, easing the head onto him
/// (`BAV_VIEW=ground`, and the film's ground shots each frame).
pub fn track_nearest(view: &mut View, vehicles: &Query<(&Vehicle, &Transform)>) {
    let eye = view.ground_eye;
    let Some((_, tf)) = vehicles
        .iter()
        .filter(|(v, t)| v.side == crate::diorama::diorama().attacker() && v.state.alive() && t.translation.distance(eye) < 3500.0)
        .min_by(|a, b| a.1.translation.distance(eye).total_cmp(&b.1.translation.distance(eye)))
    else {
        return;
    };
    let (yaw, pitch) = look_angles(tf.translation + Vec3::Y * 1.2 - eye);
    view.ground_yaw += crate::units::wrap(yaw - view.ground_yaw) * 0.3;
    view.ground_pitch += (pitch - view.ground_pitch) * 0.3;
}

/// What the operator's hands do this frame: the sticks, the gimbal wheel, the zoom rocker. Read from the keys and
/// the mouse, or from the scripted flight (`BAV_FLY`, and the film's flown shots).
#[derive(Default, Clone, Copy)]
struct Stick {
    /// Forward and sideways, -1..1.
    fwd: f32,
    right: f32,
    /// Turn (left positive) and climb, -1..1.
    yaw: f32,
    climb: f32,
    /// Sport mode (shift).
    sport: bool,
    /// Cine mode, slow and smooth for filming (the film's flown shots only).
    cine: bool,
    /// Gimbal tilt rate, -1..1 (up positive).
    gimbal: f32,
    /// Zoom, in wheel notches (in positive).
    zoom: f32,
    /// The mouse's yaw and tilt this frame, in radians.
    pan: f32,
    tilt: f32,
}

/// A scripted flight the sticks read: legs of game time with what the sticks do. `DevPlugin` fills it from
/// `BAV_FLY`; the film's tour replaces it per flown shot.
#[derive(Resource, Default, Clone)]
pub struct Flight(pub Vec<(f32, f32, String)>);

impl Stick {
    fn read(keys: &ButtonInput<KeyCode>, buttons: &ButtonInput<MouseButton>, motion: &AccumulatedMouseMotion, scroll: &AccumulatedMouseScroll, view: &View) -> Self {
        let axis = |a, b| keys.pressed(a) as u8 as f32 - keys.pressed(b) as u8 as f32;
        let mut s = Self { fwd: axis(KeyCode::KeyW, KeyCode::KeyS), right: axis(KeyCode::KeyD, KeyCode::KeyA), yaw: axis(KeyCode::KeyQ, KeyCode::KeyE), climb: axis(KeyCode::KeyR, KeyCode::KeyF), sport: keys.pressed(KeyCode::ShiftLeft), zoom: scroll.delta.y, ..default() };
        if buttons.pressed(MouseButton::Right) || buttons.pressed(MouseButton::Middle) {
            s.pan = -motion.delta.x * 0.0025 / view.zoom().sqrt();
            s.tilt = -motion.delta.y * 0.0025 / view.zoom().sqrt();
        }
        s
    }

    /// The scripted leg at game time `t`, if one is running: `w a s d q e r f` as on the keys, `+` sport, `-` cine, `u`/`n` gimbal
    /// up/down, `z`/`x` zoom in/out.
    fn scripted(t: f32, dt: f32, flight: &Flight) -> Option<Self> {
        let (_, _, spec) = flight.0.iter().find(|(from, to, _)| (*from..*to).contains(&t))?;
        let mut s = Self::default();
        for c in spec.chars() {
            match c {
                'w' => s.fwd += 1.0,
                's' => s.fwd -= 1.0,
                'd' => s.right += 1.0,
                'a' => s.right -= 1.0,
                'q' => s.yaw += 1.0,
                'e' => s.yaw -= 1.0,
                'r' => s.climb += 1.0,
                'f' => s.climb -= 1.0,
                '+' => s.sport = true,
                '-' => s.cine = true,
                'u' => s.gimbal += 1.0,
                'n' => s.gimbal -= 1.0,
                'z' => s.zoom += 4.0 * dt,
                'x' => s.zoom -= 4.0 * dt,
                _ => {}
            }
        }
        Some(s)
    }
}

fn drone_control(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    script: Res<crate::director::Script>,
    flight: Res<Flight>,
    ew: Res<bav_sim::ew::Ew>,
    mut view: ResMut<View>,
) {
    let dt = time.delta_secs();
    // The battery drains whether or not you are watching. A Matrice gives you about 35 minutes; we compress.
    if view.feed_down <= 0.0 && !dev().no_battery {
        view.battery -= dt / 600.0;
        if view.battery <= 0.0 {
            view.feed_down = 25.0; // fly back, swap, relaunch
            view.battery = 1.0;
        }
    } else {
        view.feed_down -= dt;
    }
    // Their EW, once it has found the link, holds it for a while.
    view.jamming = (view.jamming - dt * 0.03).max(0.0);
    let t = time.elapsed_secs();
    // Two links (design/next-items.md §2.6): the video is the feed's own; the control link fails in spells past
    // the heavier jamming, and after six seconds without it the drone flies itself home and stays on that
    // course until the operator cancels it (H) with the link back. The satellite fix walks under jamming, and
    // the position readout with it.
    let jam = view.jamming;
    view.control = jam < 0.55 || geo::value_noise(t * 0.7, 9.0, 65) > (jam - 0.35) * 1.4;
    view.lost_for = if view.control { 0.0 } else { view.lost_for + dt };
    if view.lost_for > 6.0 && view.rth.is_none() && view.feed_down <= 0.0 {
        view.rth = Some(Rth::Brake(1.5));
    }
    if view.feed_down > 0.0 || (keys.just_pressed(KeyCode::KeyH) && view.control) {
        view.rth = None;
    }
    // GNSS jamming goes with the rest, except where they lift it for their own helicopters.
    let gnss_jam = if ew.gnss_lifted(time.elapsed_secs_f64()) { 0.0 } else { jam };
    view.gnss = 1.0 - (gnss_jam * 1.4 - 0.2).clamp(0.0, 1.0);
    view.gnss_error = Vec2::new(geo::fbm(t * 0.3, 4.0, 2, 66), geo::fbm(t * 0.3 + 9.0, 5.0, 2, 67)) * 120.0 * (1.0 - view.gnss);
    // Gusts: the position hold fights a wind that comes and goes. Mostly a few m/s, now and then a shove.
    let gust = Vec3::new(geo::fbm(t * 0.21, 1.3, 3, 61), 0.0, geo::fbm(t * 0.21 + 40.0, 7.1, 3, 62)) * 2.2
        * (1.0 + 2.5 * (geo::value_noise(t * 0.07, 3.3, 63) - 0.35).max(0.0));
    if (view.mode != Mode::Drone && view.rth.is_none()) || view.locked {
        return;
    }
    let stick = Stick::scripted(script.t, dt, &flight).unwrap_or_else(|| Stick::read(&keys, &buttons, &motion, &scroll, &view));
    // A Mavic 3: 15 m/s in its normal mode, 21 in sport, and a gentle 7 in cine for steady filming.
    let (max_speed, max_accel) = if stick.sport { (21.0, 6.0) } else if stick.cine { (7.0, 2.5) } else { (15.0, 4.0) };
    let yaw = view.drone_yaw;
    let fwd = Vec3::new(-yaw.sin(), 0.0, -yaw.cos());
    let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin());
    let horizontal = Vec3::new(view.drone_vel.x, 0.0, view.drone_vel.z);
    if horizontal.length() > 1.0 {
        view.track = horizontal.normalize();
    }
    let ground_here = geo::height(view.drone_pos.x, view.drone_pos.z);
    // Where the drone wants to go: the sticks, or its own way home.
    let (desired, climb_cmd) = match view.rth {
        None => ((fwd * stick.fwd + right * stick.right).normalize_or_zero() * max_speed, stick.climb * if stick.sport { 8.0 } else if stick.cine { 3.0 } else { 5.0 }),
        Some(Rth::Brake(left)) => {
            view.rth = Some(if left > dt { Rth::Brake(left - dt) } else { Rth::Back { from: view.drone_pos } });
            (Vec3::ZERO, 0.0)
        }
        Some(Rth::Back { from }) => {
            if from.distance(view.drone_pos) > 50.0 {
                view.rth = Some(Rth::Home);
            }
            (-view.track * 5.0, 0.0)
        }
        Some(Rth::Home) => {
            let home = geo::layout().home;
            let to = home - view.drone_pos.xz();
            let agl = view.drone_pos.y - ground_here;
            let climb = (120.0 - agl).clamp(-3.0, 3.0);
            (if to.length() > 20.0 { Vec3::new(to.x, 0.0, to.y).normalize() * 10.0 } else { Vec3::ZERO }, climb)
        }
    };
    let accel = if desired == Vec3::ZERO && view.hold.is_none() && horizontal.length() > 1.0 {
        // Sticks centred at speed: it brakes hard to a stop first, and holds where it stopped.
        (-horizontal * 1.6 - gust * 0.08).clamp_length_max(6.0)
    } else if desired == Vec3::ZERO {
        // Sticks centred: the position hold brings the aircraft back to where it stopped and keeps it there to
        // within a metre, as a GPS hold does; the gust shoves it about that point, slowly.
        let here = view.drone_pos.xz();
        let off = *view.hold.get_or_insert(here) - here;
        (Vec3::new(off.x, 0.0, off.y) * 0.8 - horizontal * 1.6 - gust * 0.08).clamp_length_max(max_accel)
    } else {
        view.hold = None;
        // Under way, the controller only half-cancels the gust before it is felt.
        ((desired - horizontal) * 1.3 - gust * 0.35).clamp_length_max(max_accel)
    };
    let mut vel = horizontal + accel * dt;
    // Vertical: climb or sink towards the commanded rate, gently.
    view.climb += (climb_cmd - view.climb).clamp(-2.0 * dt, 2.0 * dt);
    vel.y = view.climb;
    view.drone_vel = vel;
    let step = if view.hold.is_some() { vel } else { Vec3::new(vel.x + gust.x * 0.25, vel.y, vel.z + gust.z * 0.25) } * dt;
    view.drone_pos += step;
    // Keep the drone over Ukrainian-held ground, mostly: it may creep forward, at a price in jamming.
    view.drone_pos.x = view.drone_pos.x.clamp(-geo::HALF_X + 50.0, geo::HALF_X - 50.0);
    view.drone_pos.z = view.drone_pos.z.clamp(-geo::HALF_Z + 50.0, 1400.0);
    // Their EW (bav_sim::ew): near their jammers, and past the jam line towards their rear emitters, the link
    // degrades; once it has the link it holds it a while (the slow decay above).
    let field = ew.jam_at(view.drone_pos);
    if field > view.jamming {
        view.jamming = (view.jamming + dt * 0.25).min(field);
    }
    let ground = geo::height(view.drone_pos.x, view.drone_pos.z);
    // The field overlay (`BAV_FIELD`) wants the whole box in one look: the ceiling lifts for it.
    let ceiling = if dev().field { 6000.0 } else { 700.0 };
    view.drone_pos.y = view.drone_pos.y.clamp(ground + 25.0, ground + ceiling);
    // Yaw and gimbal slew towards their targets, slower the further in the zoom is (as the DJI app does, and as the
    // mouse already did); the zoom motor takes its time.
    let gain = dt / view.zoom();
    view.yaw_target += stick.yaw * gain * 0.6 + stick.pan;
    view.gimbal_target = (view.gimbal_target + stick.gimbal * gain * 0.4 + stick.tilt).clamp(-1.55, 0.1);
    let dyaw = crate::units::wrap(view.yaw_target - view.drone_yaw);
    view.drone_yaw += (dyaw * (1.0 - (-dt * 4.0).exp())).clamp(-1.2 * dt, 1.2 * dt);
    let dg = view.gimbal_target - view.gimbal;
    view.gimbal += (dg * (1.0 - (-dt * 6.0).exp())).clamp(-1.5 * dt, 1.5 * dt);
    if stick.zoom != 0.0 {
        view.fov_target = (view.fov_target * (1.0 - stick.zoom * 0.12)).clamp(3f32.to_radians(), 70f32.to_radians());
    }
    let lf = view.drone_fov.ln() + (view.fov_target.ln() - view.drone_fov.ln()) * (1.0 - (-dt * 3.0).exp());
    view.drone_fov = lf.exp();
    // What the gimbal cannot quite hide: a little lag in the turns and under acceleration. The gusts it hides.
    let target_tilt = Vec3::new(-dyaw.clamp(-0.3, 0.3) * 0.01, accel.dot(fwd) * -0.001, accel.dot(right) * 0.001);
    let cur = view.tilt;
    view.tilt = cur + (target_tilt - cur) * (1.0 - (-dt * 5.0).exp());
}

/// Fly the FPV: it goes where it is pointed. The mouse points it (and turns it); W/S are the throttle; A/D bank it
/// sideways, and it levels itself when let go. Its analogue video thins with range and behind the ground; near
/// their jammers the control link drops out in spells, and without it the drone falls. `BAV_FPV` flies it by
/// itself at the vehicle nearest a point.
#[allow(clippy::too_many_arguments)]
fn fpv_control(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    motion: Res<AccumulatedMouseMotion>,
    mut fpv: ResMut<Fpv>,
    mut view: ResMut<View>,
    op: Res<Operator>,
    ew: Res<bav_sim::ew::Ew>,
    vehicles: Query<(&Vehicle, &Transform)>,
) {
    let dt = time.delta_secs();
    if !fpv.flying {
        // After it goes in: static in the goggles for a moment, then back to the drone.
        if view.mode == Mode::Fpv {
            fpv.gone += dt;
            if fpv.gone > 1.6 {
                view.mode = Mode::Drone;
            }
        }
        return;
    }
    let t = time.elapsed_secs();
    let p = fpv.pos;
    // The video: 5.8 GHz analogue, good to a few kilometres in the open, thin behind a crest.
    let eye = geo::ground(op.pos) + Vec3::Y * 3.0;
    let seen = {
        let n = (eye.distance(p) / 20.0).ceil().max(1.0) as usize;
        (1..n).all(|i| {
            let q = eye.lerp(p, i as f32 / n as f32);
            geo::height(q.x, q.z) < q.y
        })
    };
    let range = 1.0 - geo::smoothstep(2500.0, 5000.0, eye.xz().distance(p.xz()));
    fpv.signal = (range * if seen { 1.0 } else { 0.4 }).clamp(0.0, 1.0);
    // The control link: their jammers sit past the jam line; in 2023 they took the control, not the video.
    let jam = ew.jam_at(p);
    fpv.battery -= dt;
    // At the crest the link is gone about a third of the time, in spells.
    fpv.control = fpv.battery > 0.0 && (jam < 0.2 || geo::value_noise(t * 1.3, 4.0, 91) > -1.0 + 0.72 * jam);
    fpv.lost_for = if fpv.control { 0.0 } else { fpv.lost_for + dt };
    let mut bank = 0.0;
    if fpv.control {
        if let Some((_, (x, z))) = dev().fpv {
            let at = Vec2::new(x, z);
            // The test pilot: at the vehicle nearest the point, full throttle.
            if let Some((_, tf)) = vehicles.iter().filter(|(v, _)| v.side == Side::Ru && v.state.alive()).min_by(|a, b| a.1.translation.xz().distance(at).total_cmp(&b.1.translation.xz().distance(at))) {
                // At height until close, then the dive, steep enough to come in over a dug-in tank's berm: low, a
                // link dropout is the ground. Along the dive the range stays under three times the height, so it holds.
                let close = tf.translation.xz().distance(p.xz()) < 3.0 * (p.y - tf.translation.y).max(10.0) + 1.0;
                let (yaw, pitch) = look_angles(tf.translation + Vec3::Y * if close { 2.4 } else { 40.0 } - p);
                fpv.aim_yaw = yaw;
                fpv.aim_pitch = pitch.clamp(-1.4, 0.5);
                fpv.throttle = 1.0;
            }
        } else {
            let axis = |a, b| keys.pressed(a) as u8 as f32 - keys.pressed(b) as u8 as f32;
            fpv.aim_yaw -= motion.delta.x * 0.003 + axis(KeyCode::KeyE, KeyCode::KeyQ) * dt * 1.5;
            fpv.aim_pitch = (fpv.aim_pitch - motion.delta.y * 0.003).clamp(-1.4, 0.5);
            fpv.throttle = (fpv.throttle + axis(KeyCode::KeyW, KeyCode::KeyS) * dt * 0.7).clamp(0.0, 1.0);
            bank = axis(KeyCode::KeyD, KeyCode::KeyA);
        }
    }
    // The airframe follows where it is pointed, as fast as it can turn; it banks into turns and levels itself.
    let dyaw = crate::units::wrap(fpv.aim_yaw - fpv.yaw);
    fpv.yaw += dyaw.clamp(-2.5 * dt, 2.5 * dt);
    let dp = fpv.aim_pitch - fpv.pitch;
    fpv.pitch += dp.clamp(-2.0 * dt, 2.0 * dt);
    let roll_target = (-dyaw * 0.8).clamp(-0.6, 0.6) - bank * 0.5;
    fpv.roll += (roll_target - fpv.roll) * (1.0 - (-dt * 6.0).exp());
    let fwd = Vec3::new(-fpv.yaw.sin() * fpv.pitch.cos(), fpv.pitch.sin(), -fpv.yaw.cos() * fpv.pitch.cos());
    let right = Vec3::new(fpv.yaw.cos(), 0.0, -fpv.yaw.sin());
    if fpv.lost_for < 0.8 {
        // (A short dropout: the receiver holds the last command.) A 7-inch quad with an RPG warhead slung under
        // it is no racer: 5 m/s idling forward, cruising at 15–25, about 28 flat out, a little more in a dive.
        let speed = 5.0 + 23.0 * fpv.throttle + (-fpv.pitch).max(0.0) * 5.0;
        let want = fwd * speed + right * bank * 5.0;
        let dv = want - fpv.vel;
        fpv.vel += dv.clamp_length_max(10.0 * dt);
    } else {
        // No link for long enough: the failsafe cuts the motors and it falls.
        fpv.throttle = 0.0;
        fpv.vel.y -= 9.81 * dt;
        fpv.vel *= 1.0 - 0.2 * dt;
    }
    let v = fpv.vel;
    fpv.pos += v * dt;
    fpv.pos.x = fpv.pos.x.clamp(-geo::HALF_X + 20.0, geo::HALF_X - 20.0);
    fpv.pos.z = fpv.pos.z.clamp(-geo::HALF_Z + 20.0, geo::HALF_Z - 20.0);
}

fn op_control(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
    mut view: ResMut<View>,
) {
    if view.mode != Mode::Op {
        return;
    }
    let dt = time.delta_secs();
    if buttons.pressed(MouseButton::Right) || buttons.pressed(MouseButton::Middle) {
        view.op_yaw -= motion.delta.x * 0.003 / view.zoom();
        view.op_pitch = (view.op_pitch - motion.delta.y * 0.003 / view.zoom()).clamp(-0.9, 0.4);
    }
    if keys.pressed(KeyCode::KeyA) { view.op_yaw += dt * 0.5 / view.zoom() }
    if keys.pressed(KeyCode::KeyD) { view.op_yaw -= dt * 0.5 / view.zoom() }
    if keys.pressed(KeyCode::KeyW) { view.op_pitch += dt * 0.3 / view.zoom() }
    if keys.pressed(KeyCode::KeyS) { view.op_pitch -= dt * 0.3 / view.zoom() }
    if scroll.delta.y != 0.0 {
        view.op_fov = (view.op_fov * (1.0 - scroll.delta.y * 0.12)).clamp(5f32.to_radians(), 70f32.to_radians());
    }
}

fn ground_control(
    time: Res<Time>,
    vehicles: Query<(Entity, &Vehicle, &Transform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    mut view: ResMut<View>,
    camera: Single<&GlobalTransform, With<MainCamera>>,
) {
    if view.mode != Mode::Ground {
        return;
    }
    // Mouse look is direct here (the cursor is captured); the right button brings the eye to the sight. A view
    // placed by hand (the showcase's ground shots) keeps its own sight.
    if dev().view != Some(Mode::Ground) && !view.locked {
        view.sight = buttons.pressed(MouseButton::Right);
    }
    let k = 0.0022 / view.zoom();
    view.ground_yaw -= motion.delta.x * k;
    view.ground_pitch = (view.ground_pitch - motion.delta.y * k).clamp(-0.5, 0.35);
    let gt = camera.compute_transform();
    let terrain = geo::raycast(gt.translation, *gt.forward(), 5500.0).map(|p| p.distance(gt.translation));
    // The Javelin seeker: whichever vehicle sits within ~0.4° of the crosshair, held there, locks.
    let fwd = *gt.forward();
    let on = vehicles
        .iter()
        .filter(|(_, v, _)| v.side == Side::Ru && v.state.alive())
        .map(|(e, _, t)| (e, (t.translation + Vec3::Y * 1.2 - gt.translation).normalize().dot(fwd), t.translation.distance(gt.translation)))
        .filter(|(_, c, _)| *c > (0.007f32).cos())
        .max_by(|a, b| a.1.total_cmp(&b.1));
    // The range readout shows the vehicle under the crosshair if there is one, else the ground.
    view.ground_range = on.map(|(_, _, d)| d).or(terrain);
    let on = on.filter(|(_, _, d)| *d < crate::fire::Atgm::Javelin.range() + 100.0).map(|(e, _, _)| e);
    view.lock = match (on, view.lock) {
        (Some(e), Some((l, t))) if e == l && view.sight => Some((e, t + time.delta_secs())),
        (Some(e), _) if view.sight => Some((e, 0.0)),
        _ => None,
    };
}

fn apply_view(
    time: Res<Time>,
    view: Res<View>,
    fpv: Res<Fpv>,
    op: Res<ObservationPost>,
    mut shake: ResMut<Shake>,
    camera: Single<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    let (mut tf, mut proj) = camera.into_inner();
    let t = time.elapsed_secs();
    let dt = time.delta_secs();
    let (pos, rot, fov) = match view.mode {
        Mode::Drone => {
            // The gimbal steadies the airframe's motion; what gets through is its own small, aimless stabilisation
            // error, the same angle at any zoom, so it shows only zoomed in: the picture swims, it never rocks.
            let tremor = Vec3::new(geo::fbm(t * 0.9, 3.0, 2, 71), geo::fbm(t * 0.9, 5.0, 2, 72), geo::fbm(t * 0.9, 9.0, 2, 73)) * 0.00025;
            let tl = view.tilt + tremor;
            let rot = Quat::from_euler(EulerRot::YXZ, view.drone_yaw + tl.x, view.gimbal + tl.y, tl.z);
            (view.drone_pos, rot, view.drone_fov)
        }
        Mode::Op => {
            let eye = op.eye;
            // Hand-held binoculars: breathing sway, worse when zoomed in.
            let z = view.zoom();
            let sway = Vec2::new((t * 0.9).sin() + 0.5 * (t * 2.3).sin(), (t * 1.1).cos() + 0.4 * (t * 2.9).sin()) * 0.0015 * z.min(6.0);
            let rot = Quat::from_euler(EulerRot::YXZ, view.op_yaw + sway.x / z, view.op_pitch + sway.y / z, 0.0);
            (eye, rot, view.op_fov)
        }
        Mode::Fpv => {
            // Strapped to a small, stiff frame: the motors' buzz shakes the picture a little.
            let buzz = Vec2::new((t * 47.0).sin() + (t * 71.0).sin(), (t * 53.0).cos() + (t * 83.0).sin()) * 0.0012 * (0.3 + fpv.throttle);
            let rot = Quat::from_euler(EulerRot::YXZ, fpv.yaw + buzz.x, fpv.pitch + buzz.y, fpv.roll);
            (fpv.pos, rot, 1.3)
        }
        Mode::Manpads => {
            // Standing, the tube on the shoulder: the whole body sways, and more as the arms tire of it.
            let breath = (t * 1.3).sin();
            let sway = Vec2::new((t * 0.7).sin() + 0.5 * (t * 1.9).sin(), (t * 0.9).cos() + 0.4 * (t * 2.4).sin()) * if view.sight { 0.0012 } else { 0.004 };
            let eye = view.ground_eye + Vec3::Y * breath * 0.015;
            let rot = Quat::from_euler(EulerRot::YXZ, view.ground_yaw + sway.x, view.ground_pitch + sway.y, 0.0);
            (eye, rot, View::GROUND_FOV / view.zoom())
        }
        Mode::Ground => {
            // Kneeling, breathing; the sight on its tripod is steadier, but the gunner's hands are not.
            let breath = (t * 1.4).sin();
            let sway = if view.sight {
                Vec2::new((t * 0.8).sin() + 0.6 * (t * 2.1).sin(), (t * 1.0).cos() + 0.5 * (t * 2.6).sin()) * 0.00025
            } else {
                Vec2::new((t * 0.6).sin(), breath) * 0.004
            };
            let eye = view.ground_eye + Vec3::Y * breath * 0.012;
            let rot = Quat::from_euler(EulerRot::YXZ, view.ground_yaw + sway.x, view.ground_pitch + sway.y, 0.0);
            (eye, rot, if view.sight { View::GROUND_FOV / View::SIGHT_ZOOM } else { View::GROUND_FOV })
        }
    };
    // Shake from nearby blasts.
    let tr = shake.trauma * shake.trauma;
    let jitter = Vec3::new((t * 53.0).sin(), (t * 61.0 + 2.0).sin(), (t * 47.0 + 4.0).sin()) * tr * match view.mode { Mode::Op => 0.6, Mode::Ground | Mode::Manpads => 0.35, Mode::Drone | Mode::Fpv => 0.25 };
    let rjit = Quat::from_euler(EulerRot::YXZ, (t * 37.0).sin() * tr * 0.02, (t * 43.0).sin() * tr * 0.02, (t * 29.0).sin() * tr * 0.03);
    tf.translation = pos + jitter;
    tf.rotation = rot * rjit;
    if let Projection::Perspective(p) = proj.as_mut() {
        p.fov = fov;
        p.near = if matches!(view.mode, Mode::Ground | Mode::Fpv | Mode::Manpads) { 0.1 } else { 0.5 };
    }
    shake.trauma = (shake.trauma - dt * 0.9).max(0.0);
    shake.flash = (shake.flash - dt * 2.5).max(0.0);
}

/// The launcher on its tripod in front of the eye in the ground view: a Stugna-P tube and its sight box.
#[derive(Component)]
struct Launcher;

fn spawn_launcher(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, camera: Single<Entity, With<MainCamera>>) {
    let mut m = crate::meshes::MeshBuilder::new();
    m.cylinder(Vec3::new(0.0, 0.0, 0.25), Vec3::new(0.0, 0.0, -1.25), 0.075, 10, [0.30, 0.33, 0.24]);
    m.cylinder(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, -0.06), 0.085, 10, [0.18, 0.19, 0.16]);
    m.boxc(Vec3::new(-0.16, 0.02, 0.05), Vec3::new(0.16, 0.18, 0.32), [0.20, 0.22, 0.18]);
    m.boxc(Vec3::new(-0.16, 0.02, -0.12), Vec3::new(0.1, 0.1, 0.03), [0.05, 0.06, 0.07]);
    for (x, z) in [(-0.35f32, 0.25f32), (0.35, 0.25), (0.0, -0.45)] {
        m.cuboid(Vec3::new(x * 0.5, -0.32, z * 0.5), Vec3::new(0.02, 0.32, 0.02), Quat::from_rotation_x(z * 0.6) * Quat::from_rotation_z(-x * 0.6), [0.15, 0.15, 0.14]);
    }
    let e = commands
        .spawn((
            Mesh3d(meshes.add(m.build())),
            MeshMaterial3d(materials.add(StandardMaterial { perceptual_roughness: 0.8, ..default() })),
            Transform::from_xyz(0.32, -0.38, -0.55),
            Visibility::Hidden,
            bevy::light::NotShadowCaster,
            Launcher,
        ))
        .id();
    commands.entity(*camera).add_child(e);
}

type Tube<'w, 's> = Query<'w, 's, (&'static mut Visibility, &'static mut Transform), (With<IglaTube>, Without<Launcher>)>;

fn show_launcher(view: Res<View>, mut q: Query<&mut Visibility, With<Launcher>>, mut tube: Tube) {
    for mut v in &mut q {
        v.set_if_neq(if view.mode == Mode::Ground && !view.sight { Visibility::Inherited } else { Visibility::Hidden });
    }
    // On the shoulder, the tube comes up beside the eye when it is aimed.
    for (mut v, mut tf) in &mut tube {
        v.set_if_neq(if view.mode == Mode::Manpads { Visibility::Inherited } else { Visibility::Hidden });
        tf.translation = if view.sight { Vec3::new(0.16, -0.12, 0.0) } else { Vec3::new(0.26, -0.32, -0.1) };
    }
}

/// 5: out with an Igla team (again for the next team with missiles left), standing at its post, looking where the
/// helicopters are, or east, where they came from.
fn igla_view(keys: Res<ButtonInput<KeyCode>>, mut view: ResMut<View>, teams: Query<(Entity, &crate::fire::Post, &bav_sim::manpads::Igla)>, aircraft: Query<&Transform, With<bav_sim::air::Aircraft>>) {
    if view.mode == Mode::Fpv || !keys.just_pressed(KeyCode::Digit5) {
        return;
    }
    let mut live: Vec<(Entity, Vec2)> = teams.iter().filter(|(_, p, t)| t.missiles > 0 && p.strength >= 2).map(|(e, p, _)| (e, p.pos)).collect();
    live.sort_by(|a, b| a.1.x.total_cmp(&b.1.x).then(a.1.y.total_cmp(&b.1.y)));
    if live.is_empty() {
        return;
    }
    let i = match (view.mode, view.ground_team) {
        (Mode::Manpads, Some(cur)) => live.iter().position(|(e, _)| *e == cur).map(|i| (i + 1) % live.len()).unwrap_or(0),
        _ => 0,
    };
    let (e, p) = live[i];
    view.mode = Mode::Manpads;
    view.ground_team = Some(e);
    view.ground_eye = geo::ground(p + Vec2::new(2.0, 2.0)) + Vec3::Y * 1.65;
    let toward = aircraft.iter().map(|t| t.translation.xz()).min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))).unwrap_or(p + Vec2::X);
    view.ground_yaw = vehicle_yaw(toward - p);
    view.ground_pitch = 0.08;
    view.lock = None;
}

/// The Igla's eye: mouse look, the right button shoulders the tube, and whatever hot thing sits in its ring is held
/// by the seeker, the longer the steadier (`bav_sim::manpads::LOCK`).
fn igla_control(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    mut view: ResMut<View>,
    camera: Single<&GlobalTransform, With<MainCamera>>,
    aircraft: Query<(Entity, &bav_sim::air::Aircraft, &Transform)>,
) {
    if view.mode != Mode::Manpads {
        return;
    }
    if dev().view != Some(Mode::Manpads) && !view.locked {
        view.sight = buttons.pressed(MouseButton::Right);
    }
    let k = 0.0022 / view.zoom();
    view.ground_yaw -= motion.delta.x * k;
    view.ground_pitch = (view.ground_pitch - motion.delta.y * k).clamp(-0.3, 1.2);
    let gt = camera.compute_transform();
    let ring = bav_sim::manpads::in_ring(gt.translation, *gt.forward(), aircraft.iter().map(|(e, a, t)| (e, t.translation, bav_sim::manpads::up(a))));
    view.ground_range = ring.map(|(_, d)| d);
    view.lock = match (ring, view.lock) {
        (Some((e, _)), Some((l, t))) if e == l && view.sight => Some((e, t + time.delta_secs())),
        (Some((e, _)), _) if view.sight => Some((e, 0.0)),
        _ => None,
    };
}

/// The Igla on the shoulder, along the line of sight on the right: the tube, its grip and launcher mechanism, the
/// seeker's cap off.
#[derive(Component)]
struct IglaTube;

fn spawn_igla(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, camera: Single<Entity, With<MainCamera>>) {
    let mut m = crate::meshes::MeshBuilder::new();
    // The 1.7 m tube, 72 mm across, olive; the battery-coolant unit and grip under it; the front sight's ring.
    // (Only what is in front of the eye: the rest of the tube runs back over the shoulder, out of view.)
    m.cylinder(Vec3::new(0.0, 0.0, -0.25), Vec3::new(0.0, 0.0, -1.25), 0.036, 12, [0.30, 0.34, 0.24]);
    m.cylinder(Vec3::new(0.0, 0.0, -1.45), Vec3::new(0.0, 0.0, -0.06), 0.04, 12, [0.24, 0.27, 0.20]);
    m.boxc(Vec3::new(0.0, -0.08, -0.35), Vec3::new(0.05, 0.1, 0.2), [0.16, 0.17, 0.15]);
    m.boxc(Vec3::new(-0.045, 0.05, -1.2), Vec3::new(0.01, 0.05, 0.01), [0.1, 0.1, 0.1]);
    let e = commands
        .spawn((
            Mesh3d(meshes.add(m.build())),
            MeshMaterial3d(materials.add(StandardMaterial { perceptual_roughness: 0.75, ..default() })),
            Transform::from_xyz(0.13, -0.11, -0.2),
            Visibility::Hidden,
            bevy::light::NotShadowCaster,
            IglaTube,
        ))
        .id();
    commands.entity(*camera).add_child(e);
}

/// Capture the cursor in the ground view (mouse look); give it back elsewhere.
fn cursor_grab(view: Res<View>, mut last: Local<Option<Mode>>, mut cursor: Query<&mut bevy::window::CursorOptions, With<bevy::window::PrimaryWindow>>) {
    if *last == Some(view.mode) {
        return;
    }
    *last = Some(view.mode);
    for mut c in &mut cursor {
        let grab = matches!(view.mode, Mode::Ground | Mode::Fpv | Mode::Manpads);
        c.grab_mode = if grab { bevy::window::CursorGrabMode::Locked } else { bevy::window::CursorGrabMode::None };
        c.visible = !grab;
    }
}
