//! The battle's part of a diorama's file (`game/dioramas/<name>.ron`, read by `bav_core::data`): what each side
//! brings to the day and the day's script. The script runs whether or not the player does anything. The player's
//! side is an Allotment (`allot`): the file's catalog and support, and the day's own picks (the Record).

use crate::director::Act;
use crate::units::VKind;
use bav_core::vocab::Atgm;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug)]
pub struct SimData {
    /// The Units the player's side could have had that day (design/command.md).
    pub catalog: Vec<crate::allot::Entry<crate::allot::UnitKind>>,
    /// The Support: fires and strike drones, called rather than placed.
    pub support: Vec<crate::allot::Entry<crate::allot::SupportKind>>,
    /// The points, the Supply, and the day's own Allotment (the Record).
    pub allotment: crate::allot::AllotmentSpec,
    /// The weapon at each of the enemy's positions, in the layout's order (none: riflemen only).
    #[serde(default)]
    pub ru_weapons: Vec<Option<Atgm>>,
    /// Tanks dug in hull-down on the defenders' line: where, and which way they face.
    #[serde(default)]
    pub dug_in: Vec<((f32, f32), f32)>,
    /// What the old wrecks were, in turn.
    pub old_wreck_kinds: Vec<VKind>,
    /// The defending brigade's guns on their registered points: every so many seconds, so many rounds (none: no
    /// registered fire).
    #[serde(default)]
    pub registered_fire: Option<(f64, u32)>,
    /// The vehicles that can go in for the wounded: what kind, from where (how many is the Allotment's; none:
    /// nothing gets there by day).
    #[serde(default)]
    pub evacuation: Option<crate::evac::EvacSpec>,
    /// Where the drone team flies from, and how often an hour (of the day's clock) the enemy finds it while a drone
    /// is up: often in early 2023, when DJI's AeroScope gave the pilot away; rarely south of Mala Tokmachka.
    pub operator: OperatorSpec,
    /// Their EW: jammers on their line, the rear ramp, the Ka-52 tell (none: only the scripted bursts).
    #[serde(default)]
    pub ew: Option<crate::ew::EwSpec>,
    /// Their drones over the ground: the crews, the roads they watch, our domes and nets, our runs on the road (none:
    /// no kill zone).
    #[serde(default)]
    pub killzone: Option<crate::killzone::KillzoneSpec>,
    /// Their reconnaissance wings as aircraft and our interceptors against them (none: the staff's Orlan is a strip,
    /// and there are no interceptors).
    #[serde(default)]
    pub wings: Option<crate::wings::WingsSpec>,
    /// The strike drones of the day (how many is the Allotment's; none: the drone is eyes only).
    #[serde(default)]
    pub fpv: Option<FpvSpec>,
    /// What Space orders: the ATGM teams' fire discipline, or the column.
    pub space_key: crate::fire::SpaceKey,
    /// What Shift+click calls: RAAM mines, smoke or cluster (none: Shift+click is a plain fire mission).
    #[serde(default)]
    pub shift_fire: Option<crate::fire::ShiftFire>,
    /// The enemy's guns, and what they fire on (none: they have none).
    #[serde(default)]
    pub ru_guns: Option<crate::director::RuGuns>,
    /// What an earlier part of the day left on the ground (`air::Aftermath`): the Record's own, which Command replaces
    /// with what the player's morning left.
    #[serde(default)]
    pub aftermath: Option<crate::air::Aftermath>,
    /// The infantry fight for a place (`fight`): ammunition, pulling out, capture, the field's areas.
    #[serde(default)]
    pub fight: Option<crate::fight::FightSpec>,
    /// The field's areas, held by whoever has men in them, where the day has them without the fight (Avdiivka's;
    /// Hostomel's are the fight's own, `fight.zones`).
    #[serde(default)]
    pub areas: Vec<crate::fight::ZoneSpec>,
    /// The Bradleys dug in (`bradley`): what is in their launchers and how far their gunners open fire (none: a full
    /// launcher, and the gun's whole reach).
    #[serde(default)]
    pub bradley: Option<BradleySpec>,
    /// The ground robots and the heavy bomber (`robots`; none: the day has neither).
    #[serde(default)]
    pub robots: Option<crate::robots::RobotsSpec>,
    /// Command's Enemy staff: its budgets for the day (`staff`).
    #[serde(default)]
    pub staff: crate::staff::StaffSpec,
    /// The day: at each game second, what happens.
    pub script: Vec<(f32, Act)>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct OperatorSpec {
    pub pos: (f32, f32),
    pub found_per_hour: f32,
}

/// The Bradleys dug in: TOWs in each launcher, and the range inside which the gunners open fire.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct BradleySpec {
    pub tows: u32,
    pub range: f32,
    /// When the gunners left to themselves first open fire (`bradley::Opens`): at what they see, or not before
    /// their run.
    #[serde(default)]
    pub opens: crate::bradley::Opens,
    /// How far a run takes a Bradley out of its hide towards what it goes for (m).
    #[serde(default = "run_length")]
    pub run: f32,
}

fn run_length() -> f32 {
    150.0
}

/// FPVs: how long one flies (s), and what share of the day's are heavy enough to fly in fog (`fight::Seeing`): the
/// light ones are useless in the wet (c15 §3). All of them where the file does not say.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct FpvSpec {
    pub endurance: f32,
    #[serde(default = "all")]
    pub heavy: f32,
}

fn all() -> f32 {
    1.0
}

static DATA: OnceLock<SimData> = OnceLock::new();

/// The battle's part of the diorama on the table.
pub fn sim_data() -> &'static SimData {
    DATA.get_or_init(bav_core::data::read)
}

#[cfg(test)]
mod tests {
    /// Every diorama's battle reads: the forces, the stock and the script.
    #[test]
    fn every_battle_reads() {
        for d in bav_core::diorama::Diorama::ALL {
            let _: super::SimData = bav_core::data::parse(&bav_core::data::builtin_text(d)).unwrap_or_else(|e| panic!("{d}: {e}"));
        }
    }
}
