//! Vuhledar, 6 February 2023: the first diorama of bavovniatko.
//!
//! The history is in research/chapters/c08-vuhledar.md. Russian columns drive the same dirt roads into the
//! 72nd Brigade's minefields, ATGM teams in the tree lines and drone-spotted artillery. The world runs by itself;
//! the player, a Ukrainian drone operator and fire controller, matters at the margin.

mod audio;
mod camera;
mod director;
mod fire;
mod fx;
mod geo;
mod hud;
mod meshes;
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

/// `VUHLEDAR_HEADLESS=1`: no window and no sound; the camera renders offscreen on the GPU (for screenshots).
pub fn headless() -> bool {
    std::env::var("VUHLEDAR_HEADLESS").is_ok()
}

fn main() {
    let mut app = App::new();
    if headless() {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: None, exit_condition: ExitCondition::DontExit, ..default() })
                .set(AudioPlugin { global_volume: GlobalVolume::new(Volume::SILENT), ..default() })
                .disable::<WinitPlugin>(),
        )
        .add_plugins(ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f64(1.0 / 60.0)));
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
        .add_plugins(dev::DevPlugin)
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

/// Development helpers: `VUHLEDAR_SHOT=<path>` saves a screenshot after `VUHLEDAR_SHOT_AT` seconds (default 6)
/// and exits; `VUHLEDAR_VIEW=op` starts in the OP view; F12 saves a screenshot.
mod dev {
    use bevy::{prelude::*, render::view::screenshot::{Screenshot, save_to_disk}};

    pub struct DevPlugin;

    impl Plugin for DevPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Update, (auto_shot, key_shot, env_view, env_look, record, log_stats, warp, scripted_fire, frame_times));
        }
    }

    fn auto_shot(mut commands: Commands, time: Res<Time<Real>>, target: Option<Res<crate::camera::HeadlessTarget>>, mut done: Local<u8>, mut exit: MessageWriter<AppExit>) {
        let Ok(path) = std::env::var("VUHLEDAR_SHOT") else { return };
        let at: f32 = std::env::var("VUHLEDAR_SHOT_AT").ok().and_then(|s| s.parse().ok()).unwrap_or(6.0);
        let t = time.elapsed_secs();
        if *done == 0 && t > at {
            let shot = match target.as_ref() {
                Some(t) => Screenshot::image(t.0.clone()),
                None => Screenshot::primary_window(),
            };
            commands.spawn(shot).observe(save_to_disk(path));
            *done = 1;
        } else if *done == 1 && t > at + 1.5 {
            exit.write(AppExit::Success);
            *done = 2;
        }
    }

    /// `VUHLEDAR_CAM="x,z,agl,yaw_deg,pitch_deg,fov_deg"` places the drone; `VUHLEDAR_VIEW=op` picks the OP.
    fn env_view(mut view: ResMut<crate::camera::View>, mut vtime: ResMut<Time<Virtual>>, mut done: Local<bool>) {
        if *done {
            return;
        }
        *done = true;
        if let Some(s) = std::env::var("VUHLEDAR_SPEED").ok().and_then(|s| s.parse::<f32>().ok()) {
            vtime.set_relative_speed(s);
        }
        if std::env::var("VUHLEDAR_VIEW").as_deref() == Ok("op") {
            view.mode = crate::camera::Mode::Op;
        }
        if let Ok(c) = std::env::var("VUHLEDAR_CAM") {
            let v: Vec<f32> = c.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            if v.len() == 6 {
                view.drone_pos = Vec3::new(v[0], crate::geo::height(v[0], v[1]) + v[2], v[1]);
                view.drone_yaw = v[3].to_radians();
                view.gimbal = v[4].to_radians();
                view.drone_fov = v[5].to_radians();
            }
        }
    }

    /// `VUHLEDAR_LOOK=<idx>`: keep the drone's gimbal on the Russian vehicle with that column index (first wave
    /// still alive), from wherever `VUHLEDAR_CAM` put it.
    /// `VUHLEDAR_LOOK=men` instead follows the middle of the Russian infantry still advancing.
    fn env_look(mut view: ResMut<crate::camera::View>, q: Query<(&crate::units::Vehicle, &Transform)>, men: Query<(&crate::units::Soldier, &Transform)>, mut aim: Local<Option<Vec3>>) {
        let Ok(look) = std::env::var("VUHLEDAR_LOOK") else { return };
        let target = if look == "men" {
            let me = view.drone_pos;
            let pts: Vec<Vec3> = men
                .iter()
                .filter(|(s, t)| s.side == crate::units::Side::Ru && s.state != crate::units::SState::Flee && t.translation.distance(me) < 1400.0)
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
        view.drone_yaw = f32::atan2(-d.x, -d.z);
        view.gimbal = f32::atan2(d.y, Vec2::new(d.x, d.z).length());
    }

    /// `VUHLEDAR_REC=<dir>`: from `VUHLEDAR_REC_FROM` seconds (default 5) save `VUHLEDAR_REC_SECS` seconds
    /// (default 10) of frames at 15 fps as `<dir>/f00000.png`…, then exit. `scripts/record.sh` encodes them.
    /// `VUHLEDAR_WARP_TO=<game s>`: run at 3x (the most the simulation's step cap allows at 60 fps) until then.
    fn warp(script: Res<crate::director::Script>, mut vtime: ResMut<Time<Virtual>>, mut done: Local<bool>) {
        let Some(to) = std::env::var("VUHLEDAR_WARP_TO").ok().and_then(|s| s.parse::<f32>().ok()) else { return };
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

    fn record(
        mut commands: Commands,
        time: Res<Time<Real>>,
        script: Res<crate::director::Script>,
        target: Option<Res<crate::camera::HeadlessTarget>>,
        mut n: Local<u32>,
        mut next: Local<f32>,
        mut exit: MessageWriter<AppExit>,
    ) {
        let Ok(dir) = std::env::var("VUHLEDAR_REC") else { return };
        let env = |k: &str, d: f32| std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d);
        // Start at a game time (script clock), then take frames at 15 per real second.
        let (from, secs) = (env("VUHLEDAR_REC_FROM", 5.0), env("VUHLEDAR_REC_SECS", 10.0));
        if script.t < from {
            return;
        }
        if *n == 0 && *next == 0.0 {
            *next = time.elapsed_secs();
        }
        let t = time.elapsed_secs();
        if t < *next {
            return;
        }
        if *n >= (secs * 15.0) as u32 {
            if t > *next + 2.0 {
                exit.write(AppExit::Success);
            }
            return;
        }
        *next += 1.0 / 15.0;
        let shot = match target.as_ref() {
            Some(t) => Screenshot::image(t.0.clone()),
            None => Screenshot::primary_window(),
        };
        commands.spawn(shot).observe(save_to_disk(format!("{dir}/f{:05}.png", *n)));
        *n += 1;
    }

    /// `VUHLEDAR_LOG_STATS=1`: print the tally once a game minute.
    fn log_stats(script: Res<crate::director::Script>, stats: Res<crate::units::Stats>, fc: Res<crate::fire::FireControl>, soldiers: Query<&crate::units::Soldier>, vehicles: Query<&crate::units::Vehicle>, mut last: Local<u32>) {
        if std::env::var("VUHLEDAR_LOG_STATS").is_err() || script.t as u32 / 60 == *last {
            return;
        }
        *last = script.t as u32 / 60;
        let moving = vehicles.iter().filter(|v| v.state.mobile()).count();
        let men = soldiers.iter().filter(|s| s.side == crate::units::Side::Ru).count();
        info!(
            "t={:>4.0} {} committed={} destroyed={} abandoned={} withdrawn={} moving={} men_alive={} sent={} back={} dead={} foothold={} mines={} atgm={} art={} shells={} brigade={} ua_losses={}",
            script.t, crate::director::clock_text(script.t), stats.vehicles_committed, stats.destroyed, stats.abandoned, stats.withdrawn, moving, men, stats.sent, stats.returned, stats.ru_dead, stats.foothold, stats.mine_hits, stats.atgm_kills, stats.artillery_kills, fc.shells, fc.brigade_shells, stats.ua_losses
        );
    }

    /// `VUHLEDAR_FIRE="100:he,130:raam,160:himars"`: at those game seconds, call that mission on the nearest
    /// stalled Russian vehicle (as a player would, from the drone).
    fn scripted_fire(
        script: Res<crate::director::Script>,
        time: Res<Time>,
        view: Res<crate::camera::View>,
        mut fc: ResMut<crate::fire::FireControl>,
        mut radio: ResMut<crate::units::Radio>,
        q: Query<(&crate::units::Vehicle, &Transform)>,
        mut done: Local<Vec<usize>>,
    ) {
        use crate::fire::{Mission, ShellKind};
        let Ok(plan) = std::env::var("VUHLEDAR_FIRE") else { return };
        let now = time.elapsed_secs_f64();
        for (i, item) in plan.split(',').enumerate() {
            let mut it = item.split(':');
            let (Some(t), Some(kind)) = (it.next().and_then(|t| t.parse::<f32>().ok()), it.next()) else { continue };
            if script.t < t || done.contains(&i) {
                continue;
            }
            done.push(i);
            let me = Vec2::new(view.drone_pos.x, view.drone_pos.z);
            let Some((_, tf)) = q
                .iter()
                .filter(|(v, _)| v.state.alive() && !matches!(v.state, crate::units::VState::Advancing))
                .min_by(|a, b| Vec2::new(a.1.translation.x, a.1.translation.z).distance(me).total_cmp(&Vec2::new(b.1.translation.x, b.1.translation.z).distance(me)))
            else {
                continue;
            };
            let target = Vec2::new(tf.translation.x, tf.translation.z);
            let m = match kind {
                "raam" => Mission { target, kind: ShellKind::Raam, rounds: 3, sigma: 15.0, interval: 5.0, next: now + 3.0, flight: 28.0, player: true },
                "himars" => Mission { target, kind: ShellKind::Himars, rounds: 2, sigma: 3.0, interval: 4.0, next: now + 2.0, flight: 55.0, player: true },
                _ => Mission { target, kind: ShellKind::He155, rounds: 4, sigma: 12.0, interval: 3.5, next: now + 2.0, flight: 24.0, player: true },
            };
            fc.missions.push(m);
            radio.say(now, format!("(scripted) {kind} mission on {:.0}, {:.0}", target.x, target.y));
        }
    }

    /// `VUHLEDAR_DIAG=1`: log the mean and worst frame time every ten seconds.
    fn frame_times(time: Res<Time<Real>>, mut acc: Local<(f32, f32, u32, f32)>) {
        if std::env::var("VUHLEDAR_DIAG").is_err() {
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

    fn key_shot(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>, mut n: Local<u32>) {
        if keys.just_pressed(KeyCode::F12) {
            *n += 1;
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("vuhledar-{:03}.png", *n)));
        }
    }
}
