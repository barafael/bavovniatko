//! Development switches, read from `BAV_*` environment variables once at startup (see game/README.md,
//! "Development"). The systems they drive live with the game.

use crate::vocab::{Atgm, Mode, Strike, Target, Weather};
use bevy::prelude::*;
use std::{str::FromStr, sync::OnceLock};

/// `BAV_DAYDBG`: daylight debugging.
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum DayDebug {
    /// Leave the light as it was at startup.
    Off,
    /// Animate the light but leave the sun where it is.
    NoSun,
}

/// `BAV_OFF`: parts of the picture to leave out, to measure what each costs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Part {
    /// The sun's shadow cascades.
    Shadows,
    Msaa,
    /// The video-feed post-process.
    Feed,
    Trees,
    /// The ground chunks.
    Terrain,
}

/// Whether `BAV_OFF` leaves this part out.
pub fn off(part: Part) -> bool {
    dev().off.contains(&part)
}

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
    /// Start in this view: `op` or `ground`.
    pub view: Option<Mode>,
    /// Ground view: which kind of ATGM team to join.
    pub team: Option<Atgm>,
    /// Ground view: launch the player's missile at this game time.
    pub auto_atgm: Option<f64>,
    pub daydbg: Option<DayDebug>,
    pub shot: Option<String>,
    pub shot_at: f32,
    /// Drone placement: x, z, height above ground, yaw, pitch, fov (degrees).
    pub cam: Option<[f32; 6]>,
    /// Keep the drone's gimbal on a vehicle (column index) or `men`.
    pub look: Option<Target>,
    pub speed: Option<f32>,
    pub warp_to: Option<f32>,
    /// `BAV_SEED=<n>`: run the day on other random streams (0, or unset, is the day itself).
    pub seed: u64,
    /// Go straight to the debrief once the warp (if any) arrives, to lay the screen out without playing the day.
    pub debrief: bool,
    pub rec: Option<String>,
    pub rec_from: f32,
    pub rec_secs: f32,
    /// Scripted fire missions: game time and strike.
    pub fire: Vec<(f32, Strike)>,
    /// Scripted drone flight: legs of game time with what the sticks do (see `camera::Stick::scripted`).
    pub fly: Vec<(f32, f32, String)>,
    /// Log frame times and the GPU time of every render pass; headless, also lift the frame cap.
    pub diag: bool,
    pub off: Vec<Part>,
    /// A directory of ground chunk PNGs: read at start if complete, written after painting if not.
    pub ground: Option<String>,
    pub log_stats: bool,
    /// Park every vehicle model in a row here (x, z, heading in degrees), live and burned out, to look them over.
    pub lineup: Option<[f32; 3]>,
    /// `BAV_ERA_AT=<game s>`: set off the reactive armour on every lineup hull that has it, once.
    pub era_at: Option<f32>,
    /// Light tuning without a rebuild: multipliers for the sun and the ambient light, and an exposure offset.
    pub light: [f32; 3],
    /// `BAV_SKY`: the February weather, `overcast` (the date's) or `sun` (the footage's).
    pub sky: Option<Weather>,
    /// `BAV_TOUR`: which shot list the showcase films, `film` (design/showcase.md) or the minute tours.
    pub tour: Option<TourKind>,
}

/// `BAV_TOUR`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum TourKind {
    /// The showcase film's tour, one per diorama.
    Film,
}

fn var(k: &str) -> Option<String> {
    std::env::var(k).ok().filter(|s| !s.is_empty())
}

fn parse<T: FromStr>(k: &str) -> Option<T> {
    var(k).and_then(|s| s.trim().parse().ok())
}

impl DevEnv {
    fn load() -> Self {
        let cam = var("BAV_CAM").and_then(|c| {
            let v: Vec<f32> = c.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            <[f32; 6]>::try_from(v).ok()
        });
        let fire = var("BAV_FIRE")
            .map(|plan| {
                plan.split(',')
                    .filter_map(|item| {
                        let (t, k) = item.split_once(':')?;
                        Some((t.trim().parse().ok()?, k.trim().parse().ok()?))
                    })
                    .collect()
            })
            .unwrap_or_default();
        // `BAV_FLY="3-9:w+,9-12:q,12-20:nz"`: from 3 to 9 s fly forward in sport mode, then turn left, then tilt
        // the gimbal down while zooming in.
        let fly = var("BAV_FLY")
            .map(|plan| {
                plan.split(',')
                    .filter_map(|leg| {
                        let (span, spec) = leg.split_once(':')?;
                        let (from, to) = span.split_once('-')?;
                        Some((from.trim().parse().ok()?, to.trim().parse().ok()?, spec.trim().to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();
        Self {
            headless: var("BAV_HEADLESS").is_some(),
            res: var("BAV_RES").and_then(|r| {
                let (w, h) = r.split_once('x')?;
                Some((w.parse().ok()?, h.parse().ok()?))
            }),
            skip_briefing: var("BAV_SKIP_BRIEFING").is_some(),
            showcase: var("BAV_SHOWCASE"),
            no_battery: var("BAV_NO_BATTERY").is_some(),
            view: parse("BAV_VIEW"),
            team: parse("BAV_TEAM"),
            auto_atgm: parse("BAV_AUTO_ATGM"),
            daydbg: parse("BAV_DAYDBG"),
            sky: parse("BAV_SKY"),
            shot: var("BAV_SHOT"),
            shot_at: parse("BAV_SHOT_AT").unwrap_or(6.0),
            cam,
            look: parse("BAV_LOOK"),
            speed: parse("BAV_SPEED"),
            warp_to: parse("BAV_WARP_TO"),
            seed: parse("BAV_SEED").unwrap_or(0),
            debrief: var("BAV_DEBRIEF").is_some(),
            rec: var("BAV_REC"),
            rec_from: parse("BAV_REC_FROM").unwrap_or(5.0),
            rec_secs: parse("BAV_REC_SECS").unwrap_or(10.0),
            fire,
            fly,
            diag: var("BAV_DIAG").is_some(),
            off: var("BAV_OFF").map(|s| s.split(',').filter_map(|p| p.trim().parse().ok()).collect()).unwrap_or_default(),
            ground: var("BAV_GROUND"),
            log_stats: var("BAV_LOG_STATS").is_some(),
            era_at: parse("BAV_ERA_AT"),
            lineup: var("BAV_LINEUP").and_then(|c| <[f32; 3]>::try_from(c.split(',').filter_map(|s| s.trim().parse().ok()).collect::<Vec<f32>>()).ok()),
            light: var("BAV_LIGHT")
                .and_then(|c| <[f32; 3]>::try_from(c.split(',').filter_map(|s| s.trim().parse().ok()).collect::<Vec<f32>>()).ok())
                .unwrap_or([1.0, 1.0, 0.0]),
            tour: parse("BAV_TOUR"),
        }
    }
}

static DEV: OnceLock<DevEnv> = OnceLock::new();

/// A simulation stream's seed under `BAV_SEED`: the stream's own seed for the day itself, mixed with the
/// run number otherwise, so `scripts/band.sh` can see how far the day's outcome swings with chance alone.
pub fn seeded(base: u64) -> u64 {
    match dev().seed {
        0 => base,
        s => base ^ s.wrapping_mul(0x9E37_79B9_7F4A_7C15),
    }
}

/// The development switches, parsed on first use.
pub fn dev() -> &'static DevEnv {
    DEV.get_or_init(DevEnv::load)
}

/// The development systems run as one set in `Step::Control`; the showcase steers before them, the keys after.
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DevSystems;

/// Append a line to a file, creating it: the showcase's sound log and segments, written as the day runs.
pub fn append(path: &str, line: &str) {
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}
