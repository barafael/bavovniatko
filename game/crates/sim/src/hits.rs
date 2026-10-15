//! The hit feed's record: every hit on a vehicle, and every missile or drone that missed, as the battle decided
//! it: who fired what from where, at what, and what it did. The HUD shows the newest on the right. It is a
//! record only; nothing in the battle reads it.

use crate::diorama::Side;
use crate::units::{Kill, VKind, Vehicle};
use bevy::prelude::*;

/// One hit (or near thing), as the feed tells it.
#[derive(Clone, Debug)]
pub struct HitEntry {
    /// Game time (elapsed seconds) when it happened.
    pub at: f64,
    /// What hit: "Stugna-P", "Kornet", "155 mm", "a mine", "FPV".
    pub weapon: &'static str,
    /// From where, or whose: "the forward tree line", "the crest", "your drone team"; empty when it is plain.
    pub from: &'static str,
    pub target: Entity,
    /// "T-72B3 (dug in)", "Bradley".
    pub target_name: String,
    /// Whose vehicle was hit.
    pub side: Side,
    /// What it did, built up as the battle decides it: "from above: penetrated", then "; the ammunition went up".
    pub result: String,
}

/// The last twenty.
#[derive(Resource, Default)]
pub struct HitLog {
    pub entries: Vec<HitEntry>,
}

impl HitLog {
    pub fn open(&mut self, at: f64, weapon: &'static str, from: &'static str, target: Entity, v: &Vehicle, result: impl Into<String>) {
        self.entries.push(HitEntry { at, weapon, from, target, target_name: name_of(v), side: v.side, result: result.into() });
        if self.entries.len() > 20 {
            self.entries.remove(0);
        }
    }

    /// Add to the newest entry for this vehicle, if it is recent (the same hit's consequences).
    pub fn add(&mut self, at: f64, target: Entity, more: &str) -> bool {
        match self.entries.iter_mut().rev().find(|h| h.target == target && at - h.at < 2.0) {
            Some(h) => {
                h.result.push_str(more);
                true
            }
            None => false,
        }
    }
}

/// A vehicle as the feed names it.
pub fn name_of(v: &Vehicle) -> String {
    let what = v.kind.name();
    if v.idx >= 100 {
        format!("{what} (dug in)")
    } else if v.rescue {
        format!("{what} (sent for the wounded)")
    } else if !v.state.mobile() {
        format!("{what} (stopped)")
    } else if v.idx == 0 {
        format!("{what} (leading)")
    } else {
        what.to_string()
    }
}

/// How a wreck died, as the feed says it.
pub fn fate(how: Kill) -> &'static str {
    match how {
        Kill::BrewUp => "; the ammunition went up",
        Kill::Torch => "; burning",
        Kill::Quiet => "; knocked out",
    }
}

impl VKind {
    pub fn name(self) -> &'static str {
        match self {
            VKind::T80 => "T-80BV",
            VKind::T72 => "T-72B3",
            VKind::Bmp1 => "BMP-1",
            VKind::Bmp2 => "BMP-2",
            VKind::Mtlb => "MT-LB",
            VKind::Leo2a6 => "Leopard 2A6",
            VKind::Leo2r => "Leopard 2R",
            VKind::Wisent => "Wisent",
            VKind::Bmr2 => "BMR-2",
            VKind::Bradley => "Bradley",
        }
    }
}
