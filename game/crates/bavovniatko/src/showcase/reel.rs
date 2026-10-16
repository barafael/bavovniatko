//! The action reel's tour (`BAV_TOUR=reel`, `BAV_REEL=<file>`): its shots, read from a file `cargo xtask trailer`
//! writes after an events pass over the same day (design/trailer.md). The day does not depend on the shots (the
//! tour skips on its filming step and calls nothing on the camera's position), so the day of the events pass is the
//! day of every filming run.
//!
//! ```text
//! shot <from> <secs> <x0> <z0> <agl0> <x1> <z1> <agl1> <aim x> <aim z> <fov0> <fov1>
//!                                                 the drone, sliding from one place to the other, looking at a point
//! goggles <from> <secs>                           through the FPV's goggles, if it is up
//! ground <from> <secs> <x> <z> <sight 0|1>        an ATGM team's ground view, the team nearest the point
//! end <game s>                                    the run ends (an events pass has no shots before it)
//! ```
//!
//! What the player does in the day (fire missions on points, the FPV's launch) is not the reel's: the xtask gives
//! it as `BAV_FIRE` and `BAV_FPV`, the same in the events pass and every filming run.

use super::{Aim, Shot, shot};
use bav_core::vocab::Target::Point;
use bevy::prelude::*;

/// The reel's shots, in time order.
pub(super) fn load() -> Vec<Shot> {
    let path = crate::dev::dev().reel.clone().expect("BAV_TOUR=reel needs BAV_REEL=<file>");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("BAV_REEL {path}: {e}"));
    parse(&text).unwrap_or_else(|e| panic!("BAV_REEL {path}: {e}"))
}

fn parse(text: &str) -> Result<Vec<Shot>, String> {
    let mut shots = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let w: Vec<&str> = line.split_whitespace().collect();
        let Some(&kind) = w.first() else { continue };
        let f = |i: usize| w.get(i).and_then(|s| s.parse::<f32>().ok()).ok_or_else(|| format!("line {}: {line}", n + 1));
        match kind {
            "shot" => shots.push(shot(f(1)?, f(2)?, (f(3)?, f(4)?, f(5)?), (f(6)?, f(7)?, f(8)?), Aim::Drone(Point(Vec2::new(f(9)?, f(10)?))), (f(11)?, f(12)?))),
            "goggles" => shots.push(shot(f(1)?, f(2)?, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Aim::Goggles, (54.0, 54.0))),
            "ground" => shots.push(shot(f(1)?, f(2)?, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Aim::Ground(Vec2::new(f(3)?, f(4)?), f(5)? > 0.5), (54.0, 54.0))),
            // The run's last frame: one frame from somewhere, so the tour has something to wait for.
            "end" => shots.push(shot(f(1)?, 1.0 / 30.0, (0.0, 0.0, 400.0), (0.0, 0.0, 400.0), Aim::Drone(Point(Vec2::ZERO)), (30.0, 30.0))),
            _ => return Err(format!("line {}: what is {kind}?", n + 1)),
        }
    }
    shots.sort_by(|a, b| a.from.total_cmp(&b.from));
    Ok(shots)
}
