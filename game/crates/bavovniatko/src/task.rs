//! The day's task, where its file states one (`objective`, `bav_core::data::Goal`; Myrnohrad's parts): the figures its
//! goals read, the same the debrief's rows show, the tally's line while the day runs, and the verdict the debrief opens
//! with. It reads the day's count and never acts on the battle.

use crate::units::Side;
use bav_core::data::{Condition, Count, Goal};
use bevy::prelude::*;

/// The day's goals (none: no task).
pub fn goals() -> &'static [Goal] {
    &bav_core::data::data().objective
}

/// What the goals read of the day.
#[derive(bevy::ecs::system::SystemParam)]
pub struct TaskView<'w, 's> {
    kz: Option<Res<'w, bav_sim::killzone::KzTally>>,
    op: Res<'w, bav_sim::player::Operator>,
    seep: Option<Res<'w, bav_sim::seep::Seep>>,
    casualties: Option<Res<'w, bav_sim::evac::Casualties>>,
    field: Option<Res<'w, bav_sim::fight::Field>>,
    posts: Query<'w, 's, (&'static crate::fire::Post, Option<&'static bav_sim::fight::Ammo>)>,
}

impl TaskView<'_, '_> {
    /// The day's figures as they stand.
    pub fn count(&self) -> Count {
        let defender = crate::diorama::diorama().defender();
        Count {
            cars_home: self.kz.as_deref().map_or(0, |k| k.home),
            team_wounded: 2 * self.op.hits + self.seep.as_deref().map_or(0, |s| s.crews_hit),
            wounded_waiting: self.casualties.as_deref().map_or(0, |c| c.count()),
            wounded_out: self.casualties.as_deref().map_or(0, |c| c.out),
            post_least: self.posts.iter().filter(|(p, _)| p.side == defender && p.strength > 0).filter_map(|(_, a)| a.map(|a| a.0)).min(),
            ours: self.field.as_deref().map_or(Vec::new(), |f| f.zones.iter().filter(|z| z.held == Some(Side::Ua)).map(|z| z.key.clone()).collect()),
        }
    }

    /// A goal's figure against what it asks.
    pub fn figure(&self, test: &Condition, c: &Count) -> String {
        match test {
            Condition::CarsHome(n) => format!("{} of {n}", c.cars_home),
            Condition::TeamWounded(n) => format!("{} wounded, at most {n}", c.team_wounded),
            Condition::WoundedWaiting(n) => format!("{} waiting, at most {n}", c.wounded_waiting),
            Condition::WoundedOut(n) => format!("{} of {n}", c.wounded_out),
            Condition::Held(keys) => {
                let zones = self.field.as_deref().map_or(&[][..], |f| &f.zones[..]);
                let lost: Vec<String> = zones
                    .iter()
                    .filter(|z| keys.contains(&z.key) && z.held != Some(Side::Ua))
                    .map(|z| format!("{} {}", z.name.0, if z.held.is_some() { "theirs" } else { "contested" }))
                    .collect();
                if lost.is_empty() { "ours".to_string() } else { lost.join(", ") }
            }
            Condition::PostsFed(n) => format!("the leanest post {} volleys, at least {n}", c.post_least.unwrap_or(0)),
        }
    }

    /// The tally's line, where the day has a task: each goal and its figure so far.
    pub fn line(&self) -> String {
        if goals().is_empty() {
            return String::new();
        }
        let c = self.count();
        format!("\nTask   {}", goals().iter().map(|g| format!("{}: {}", g.name.0, self.figure(&g.test, &c))).collect::<Vec<_>>().join("   "))
    }

    /// The verdict: each goal's name, its figure, whether it was met, and what that meant.
    pub fn verdict(&self) -> Vec<(&'static Goal, String, bool)> {
        let c = self.count();
        goals().iter().map(|g| (g, self.figure(&g.test, &c), g.test.met(&c))).collect()
    }
}
