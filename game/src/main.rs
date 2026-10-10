//! Vuhledar, 6 February 2023: the first diorama of bavovniatko.
//!
//! The history is in research/chapters/c08-vuhledar.md. Russian columns drive the same dirt roads into the
//! 72nd Brigade's minefields, ATGM teams in the tree lines and drone-spotted artillery. The world runs by itself;
//! the player, a Ukrainian drone operator and fire controller, matters at the margin.

mod audio;
mod camera;
mod dev_env;
mod director;
mod fire;
mod fx;
mod geo;
mod hud;
mod meshes;
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
    if dev_env::dev().headless {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: None, exit_condition: ExitCondition::DontExit, ..default() })
                .set(AudioPlugin { global_volume: GlobalVolume::new(Volume::SILENT), ..default() })
                .disable::<WinitPlugin>(),
        );
        // Filming runs on a fixed game step, so there is no reason to wait for wall-clock frames.
        let wait = if dev_env::dev().showcase.is_some() { 0.0 } else { 1.0 / 60.0 };
        app.add_plugins(ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f64(wait)));
    } else {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Vuhledar, 6 February 2023".into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }));
    }
    // The two shaders live in the binary, so the game runs from anywhere with no asset folder.
    bevy::asset::embedded_asset!(app, "shaders/feed.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/ground.wgsl");
    app
        .init_state::<Phase>()
        .init_resource::<audio::AmbienceLevels>()
        .add_plugins((terrain::TerrainPlugin, world::WorldPlugin, camera::CameraPlugin, fx::FxPlugin, audio::AudioPlugin))
        .add_plugins((units::UnitsPlugin, fire::FirePlugin, director::DirectorPlugin, hud::HudPlugin))
        .add_observer(units::on_wrecked)
        .add_systems(Update, drone_view_audio)
        .add_plugins((dev::DevPlugin, showcase::ShowcasePlugin))
        .run();
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Phase {
    #[default]
    Briefing,
    Playing,
    Debrief,
}

/// The battle systems run only while the day is being played.
pub fn running(phase: Res<State<Phase>>) -> bool {
    *phase.get() == Phase::Playing
}

fn drone_view_audio(view: Res<camera::View>, mut levels: ResMut<audio::AmbienceLevels>) {
    levels.drone_view = view.mode == camera::Mode::Drone;
}

/// Development helpers, driven by the `VUHLEDAR_*` switches in [`dev_env`]. F12 saves a screenshot.
mod dev {
    use crate::camera::{Mode, View};
    use crate::dev_env::dev;
    use crate::units::{SState, Side, Soldier, VState, Vehicle};
    use bevy::{prelude::*, render::view::screenshot::{Screenshot, save_to_disk}};

    pub struct DevPlugin;

    impl Plugin for DevPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Update, (auto_shot, key_shot, env_view, env_look, record, log_stats, warp, scripted_fire, frame_times, env_ground));
        }
    }

    fn shot_of(target: Option<&crate::camera::HeadlessTarget>) -> Screenshot {
        match target {
            Some(t) => Screenshot::image(t.0.clone()),
            None => Screenshot::primary_window(),
        }
    }

    /// `VUHLEDAR_SHOT=<path>`: save a screenshot after `VUHLEDAR_SHOT_AT` seconds (default 6) and exit.
    fn auto_shot(mut commands: Commands, time: Res<Time<Real>>, target: Option<Res<crate::camera::HeadlessTarget>>, mut done: Local<u8>, mut exit: MessageWriter<AppExit>) {
        let Some(path) = dev().shot.clone() else { return };
        let t = time.elapsed_secs();
        if *done == 0 && t > dev().shot_at {
            commands.spawn(shot_of(target.as_deref())).observe(save_to_disk(path));
            *done = 1;
        } else if *done == 1 && t > dev().shot_at + 1.5 {
            exit.write(AppExit::Success);
            *done = 2;
        }
    }

    /// `VUHLEDAR_CAM="x,z,agl,yaw,pitch,fov"` places the drone; `VUHLEDAR_VIEW=op` starts on the roof;
    /// `VUHLEDAR_SPEED` scales game time.
    fn env_view(mut view: ResMut<View>, mut vtime: ResMut<Time<Virtual>>, mut done: Local<bool>) {
        if *done {
            return;
        }
        *done = true;
        if let Some(s) = dev().speed {
            vtime.set_relative_speed(s);
        }
        if dev().view.as_deref() == Some("op") {
            view.mode = Mode::Op;
        }
        if let Some(v) = dev().cam {
            view.drone_pos = Vec3::new(v[0], crate::geo::height(v[0], v[1]) + v[2], v[1]);
            set_drone_aim(&mut view, v[3].to_radians(), v[4].to_radians());
            view.drone_fov = v[5].to_radians();
            view.fov_target = view.drone_fov;
        }
    }

    /// Point the drone and make that its slew target too, so its flight controller does not fight back.
    fn set_drone_aim(view: &mut View, yaw: f32, gimbal: f32) {
        view.drone_yaw = yaw;
        view.gimbal = gimbal;
        view.yaw_target = yaw;
        view.gimbal_target = gimbal;
    }

    /// `VUHLEDAR_LOOK=<idx>`: keep the drone's gimbal on the Russian vehicle with that column index (first wave
    /// still alive); `VUHLEDAR_LOOK=men` follows the middle of the Russian infantry near the drone.
    fn env_look(mut view: ResMut<View>, q: Query<(&Vehicle, &Transform)>, men: Query<(&Soldier, &Transform)>, mut aim: Local<Option<Vec3>>) {
        let Some(look) = dev().look.as_deref() else { return };
        let target = if look == "men" {
            let me = view.drone_pos;
            let pts: Vec<Vec3> = men
                .iter()
                .filter(|(s, t)| s.side == Side::Ru && s.state != SState::Flee && t.translation.distance(me) < 1400.0)
                .map(|(_, t)| t.translation)
                .collect();
            if pts.is_empty() {
                return;
            }
            pts.iter().copied().sum::<Vec3>() / pts.len() as f32
        } else {
            let Ok(idx) = look.parse::<u32>() else { return };
            let Some((_, tf)) = q.iter().filter(|(v, _)| v.idx == idx).min_by_key(|(v, _)| v.wave) else { return };
            tf.translation
        };
        // Ease the aim so the gimbal pans smoothly.
        let a = aim.get_or_insert(target);
        *a = a.lerp(target, 0.03);
        let d = *a - view.drone_pos;
        set_drone_aim(&mut view, f32::atan2(-d.x, -d.z), f32::atan2(d.y, Vec2::new(d.x, d.z).length()));
    }

    /// `VUHLEDAR_WARP_TO=<game s>`: run at 3x (the most the simulation's step cap allows at 60 fps) until then.
    fn warp(script: Res<crate::director::Script>, mut vtime: ResMut<Time<Virtual>>, mut done: Local<bool>) {
        let Some(to) = dev().warp_to else { return };
        if *done {
            return;
        }
        if script.t < to {
            vtime.set_relative_speed(3.0);
        } else {
            vtime.set_relative_speed(1.0);
            *done = true;
        }
    }

    /// `VUHLEDAR_REC=<dir>`: from game time `VUHLEDAR_REC_FROM` save `VUHLEDAR_REC_SECS` seconds of frames at 15 fps
    /// as `<dir>/f00000.png`…, then exit. `scripts/record.sh` encodes them.
    fn record(
        mut commands: Commands,
        time: Res<Time<Real>>,
        script: Res<crate::director::Script>,
        target: Option<Res<crate::camera::HeadlessTarget>>,
        mut n: Local<u32>,
        mut next: Local<f32>,
        mut exit: MessageWriter<AppExit>,
    ) {
        let Some(dir) = dev().rec.as_deref() else { return };
        if script.t < dev().rec_from {
            return;
        }
        let t = time.elapsed_secs();
        if *n == 0 && *next == 0.0 {
            *next = t;
        }
        if t < *next {
            return;
        }
        if *n >= (dev().rec_secs * 15.0) as u32 {
            if t > *next + 2.0 {
                exit.write(AppExit::Success);
            }
            return;
        }
        *next += 1.0 / 15.0;
        commands.spawn(shot_of(target.as_deref())).observe(save_to_disk(format!("{dir}/f{:05}.png", *n)));
        *n += 1;
    }

    /// `VUHLEDAR_LOG_STATS=1`: print the tally once a game minute.
    fn log_stats(script: Res<crate::director::Script>, stats: Res<crate::units::Stats>, fc: Res<crate::fire::FireControl>, soldiers: Query<&Soldier>, vehicles: Query<&Vehicle>, mut last: Local<u32>) {
        if !dev().log_stats || script.t as u32 / 60 == *last {
            return;
        }
        *last = script.t as u32 / 60;
        let moving = vehicles.iter().filter(|v| v.state.mobile()).count();
        let men = soldiers.iter().filter(|s| s.side == Side::Ru).count();
        info!(
            "t={:>4.0} {} committed={} destroyed={} abandoned={} withdrawn={} moving={} men_alive={} sent={} back={} dead={} foothold={} mines={} atgm={} art={} shells={} brigade={} ua_losses={}",
            script.t, crate::director::clock_text(script.t), stats.vehicles_committed, stats.destroyed, stats.abandoned, stats.withdrawn, moving, men, stats.sent, stats.returned, stats.ru_dead, stats.foothold, stats.mine_hits, stats.atgm_kills, stats.artillery_kills, fc.shells, fc.brigade_shells, stats.ua_losses
        );
    }

    /// `VUHLEDAR_FIRE="100:he,130:raam,160:himars"`: at those game seconds, call that strike on the nearest stalled
    /// Russian vehicle, as a player would from the drone.
    fn scripted_fire(
        script: Res<crate::director::Script>,
        time: Res<Time>,
        view: Res<View>,
        mut fc: ResMut<crate::fire::FireControl>,
        mut radio: ResMut<crate::units::Radio>,
        q: Query<(&Vehicle, &Transform)>,
        mut done: Local<usize>,
    ) {
        let plan = &dev().fire;
        while *done < plan.len() && script.t >= plan[*done].0 {
            let strike = plan[*done].1;
            *done += 1;
            let me = Vec2::new(view.drone_pos.x, view.drone_pos.z);
            let flat = |t: &Transform| Vec2::new(t.translation.x, t.translation.z);
            let Some((_, tf)) = q.iter().filter(|(v, _)| v.state.alive() && v.state != VState::Advancing).min_by(|a, b| flat(a.1).distance(me).total_cmp(&flat(b.1).distance(me))) else { continue };
            let now = time.elapsed_secs_f64();
            fc.missions.push(strike.mission(flat(tf), 12.0, now, true));
            radio.say(now, format!("(scripted) {strike:?} on {:.0}, {:.0}", tf.translation.x, tf.translation.z));
        }
    }

    /// `VUHLEDAR_DIAG=1`: log the mean and worst frame time every ten seconds.
    fn frame_times(time: Res<Time<Real>>, mut acc: Local<(f32, f32, u32, f32)>) {
        if !dev().diag {
            return;
        }
        let dt = time.delta_secs();
        acc.0 += dt;
        acc.1 = acc.1.max(dt);
        acc.2 += 1;
        acc.3 += dt;
        if acc.3 > 10.0 {
            info!("frames: mean {:.1} ms, worst {:.1} ms", acc.0 / acc.2 as f32 * 1000.0, acc.1 * 1000.0);
            *acc = (0.0, 0.0, 0, 0.0);
        }
    }

    /// `VUHLEDAR_VIEW=ground`: start in the tree line nearest the dacha road with a `VUHLEDAR_TEAM` launcher
    /// (default Stugna-P), sight up, tracking the nearest moving Russian vehicle the way a gunner would.
    /// `VUHLEDAR_AUTO_ATGM=<s>` launches then.
    fn env_ground(mut view: ResMut<View>, teams: Query<(Entity, &crate::fire::UaPosition)>, q: Query<(&Vehicle, &Transform)>, mut started: Local<bool>) {
        if !dev().ground_view() {
            return;
        }
        if !*started {
            let want = dev().team.unwrap_or(crate::fire::Atgm::Stugna);
            let kz = Vec2::new(930.0, -658.0);
            let Some((e, p)) = teams.iter().filter(|(_, p)| p.atgm > 0 && p.weapon == Some(want)).min_by(|a, b| a.1.pos.distance(kz).total_cmp(&b.1.pos.distance(kz))) else {
                return;
            };
            crate::camera::enter_ground(&mut view, e, p.pos);
            *started = true;
        }
        view.sight = true;
        let eye = view.ground_eye;
        let Some((_, tf)) = q
            .iter()
            .filter(|(v, t)| v.side == Side::Ru && v.state.alive() && t.translation.distance(eye) < 3500.0)
            .min_by(|a, b| a.1.translation.distance(eye).total_cmp(&b.1.translation.distance(eye)))
        else {
            return;
        };
        let d = tf.translation + Vec3::Y * 1.2 - eye;
        let yaw = f32::atan2(-d.x, -d.z);
        let pitch = f32::atan2(d.y, Vec2::new(d.x, d.z).length());
        view.ground_yaw += crate::units::wrap(yaw - view.ground_yaw) * 0.3;
        view.ground_pitch += (pitch - view.ground_pitch) * 0.3;
    }

    fn key_shot(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>, mut n: Local<u32>) {
        if keys.just_pressed(KeyCode::F12) {
            *n += 1;
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("vuhledar-{:03}.png", *n)));
        }
    }
}
