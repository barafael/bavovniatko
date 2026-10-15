//! What the battle leaves on the ground, queued for the painter (`bav_look::terrain` stamps it into the ground
//! texture), and the craters it keeps for the drivers: a hull that drives into one slows down.

use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Paint {
    Crater { at: Vec2, radius: f32, seed: u32 },
    Scorch { at: Vec2, radius: f32 },
    Track { at: Vec2, dir: Vec2, gauge: f32 },
    Footprints { at: Vec2 },
    /// A mine plow's strip of turned earth, with spoil thrown to both sides.
    Plow { at: Vec2, dir: Vec2 },
    /// Snow melted by a fire on the ground: wet black earth, a grey slush rim. (In June, the grass burned.)
    Melt { at: Vec2, radius: f32 },
}

#[derive(Resource, Default)]
pub struct PaintQueue {
    pub ops: Vec<Paint>,
    /// The painter's own pacing (`bav_look::terrain`).
    pub timer: f32,
    /// Every crater deep enough to slow a track, kept for the drivers.
    pub craters: Craters,
}

impl PaintQueue {
    pub fn push(&mut self, p: Paint) {
        if let Paint::Crater { at, radius, .. } = p
            && radius >= 1.5
        {
            self.craters.add(at);
        }
        self.ops.push(p);
    }
}

/// Where shells have cratered the ground (centres, on a coarse grid), so a vehicle driving into one slows down.
#[derive(Default)]
pub struct Craters {
    grid: std::collections::HashMap<(i32, i32), Vec<Vec2>>,
}

impl Craters {
    const CELL: f32 = 16.0;

    fn add(&mut self, at: Vec2) {
        let c = ((at.x / Self::CELL).floor() as i32, (at.y / Self::CELL).floor() as i32);
        self.grid.entry(c).or_default().push(at);
    }

    /// Is a crater centre within `r` of `p`? (`r` at most a cell.)
    pub fn near(&self, p: Vec2, r: f32) -> bool {
        self.nearest(p, r).is_some()
    }

    /// The nearest crater centre within `r` of `p`, if any (`r` at most a cell).
    pub fn nearest(&self, p: Vec2, r: f32) -> Option<Vec2> {
        let (cx, cz) = ((p.x / Self::CELL).floor() as i32, (p.y / Self::CELL).floor() as i32);
        let mut best: Option<(f32, Vec2)> = None;
        for dz in -1..=1 {
            for dx in -1..=1 {
                for c in self.grid.get(&(cx + dx, cz + dz)).into_iter().flatten() {
                    let d = c.distance(p);
                    if d < r && best.is_none_or(|b| d < b.0) {
                        best = Some((d, *c));
                    }
                }
            }
        }
        best.map(|b| b.1)
    }
}
