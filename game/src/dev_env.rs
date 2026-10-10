//! Development switches, read from `VUHLEDAR_*` environment variables once at startup.
//! See README.md, "Development".

use crate::fire::{Atgm, Strike};
use std::{str::FromStr, sync::OnceLock};

#[derive(Debug, Default)]
pub struct DevEnv {
    /// No window, no sound; render offscreen on the GPU.
    pub headless: bool,
    /// Offscreen render size.
    pub res: Option<(u32, u32)>,
    pub skip_briefing: bool,
    /// Film the showcase tour into this directory.
    pub showcase: Option<String>,
    pub no_battery: bool,
    /// Start view: `op` or `ground`.
    pub view: Option<String>,
    /// Ground view: which kind of ATGM team to join.
    pub team: Option<Atgm>,
    /// Ground view: launch the player's missile at this game time.
    pub auto_atgm: Option<f64>,
    /// Daylight debugging: `off` or `nosun`.
    pub daydbg: Option<String>,
    pub shot: Option<String>,
    pub shot_at: f32,
    /// Drone placement: x, z, height above ground, yaw, pitch, fov (degrees).
    pub cam: Option<[f32; 6]>,
    /// Keep the drone's gimbal on a vehicle (column index) or `men`.
    pub look: Option<String>,
    pub speed: Option<f32>,
    pub warp_to: Option<f32>,
    pub rec: Option<String>,
    pub rec_from: f32,
    pub rec_secs: f32,
    /// Scripted fire missions: game time and strike.
    pub fire: Vec<(f32, Strike)>,
    pub diag: bool,
    pub log_stats: bool,
}

fn var(k: &str) -> Option<String> {
    std::env::var(k).ok()
}

fn num<T: FromStr>(k: &str) -> Option<T> {
    var(k).and_then(|s| s.trim().parse().ok())
}

impl DevEnv {
    fn load() -> Self {
        let cam = var("VUHLEDAR_CAM").and_then(|c| {
            let v: Vec<f32> = c.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            <[f32; 6]>::try_from(v).ok()
        });
        let fire = var("VUHLEDAR_FIRE")
            .map(|plan| {
                plan.split(',')
                    .filter_map(|item| {
                        let (t, k) = item.split_once(':')?;
                        Some((t.trim().parse().ok()?, Strike::from_str(k.trim()).ok()?))
                    })
                    .collect()
            })
            .unwrap_or_default();
        Self {
            headless: var("VUHLEDAR_HEADLESS").is_some(),
            res: var("VUHLEDAR_RES").and_then(|r| {
                let (w, h) = r.split_once('x')?;
                Some((w.parse().ok()?, h.parse().ok()?))
            }),
            skip_briefing: var("VUHLEDAR_SKIP_BRIEFING").is_some(),
            showcase: var("VUHLEDAR_SHOWCASE"),
            no_battery: var("VUHLEDAR_NO_BATTERY").is_some(),
            view: var("VUHLEDAR_VIEW").filter(|s| !s.is_empty()),
            team: var("VUHLEDAR_TEAM").and_then(|s| Atgm::from_str(&s).ok()),
            auto_atgm: num("VUHLEDAR_AUTO_ATGM"),
            daydbg: var("VUHLEDAR_DAYDBG"),
            shot: var("VUHLEDAR_SHOT"),
            shot_at: num("VUHLEDAR_SHOT_AT").unwrap_or(6.0),
            cam,
            look: var("VUHLEDAR_LOOK").filter(|s| !s.is_empty()),
            speed: num("VUHLEDAR_SPEED"),
            warp_to: num("VUHLEDAR_WARP_TO"),
            rec: var("VUHLEDAR_REC"),
            rec_from: num("VUHLEDAR_REC_FROM").unwrap_or(5.0),
            rec_secs: num("VUHLEDAR_REC_SECS").unwrap_or(10.0),
            fire,
            diag: var("VUHLEDAR_DIAG").is_some(),
            log_stats: var("VUHLEDAR_LOG_STATS").is_some(),
        }
    }

    pub fn ground_view(&self) -> bool {
        self.view.as_deref() == Some("ground")
    }
}

static DEV: OnceLock<DevEnv> = OnceLock::new();

/// The development switches, parsed on first use.
pub fn dev() -> &'static DevEnv {
    DEV.get_or_init(DevEnv::load)
}
