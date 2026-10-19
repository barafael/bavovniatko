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
    /// The defending brigade's guns on their registered points: every so many seconds, so many rounds.
    pub registered_fire: (f64, u32),
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
    /// The strike drones of the day (how many is the Allotment's; none: the drone is eyes only).
    #[serde(default)]
    pub fpv: Option<FpvSpec>,
    /// What Space orders: the ATGM teams' fire discipline, or the column.
    pub space_key: crate::fire::SpaceKey,
    /// What Shift+click calls: RAAM mines or smoke.
    pub shift_fire: crate::fire::ShiftFire,
    /// The enemy's guns, and what they fire on (none: they have none).
    #[serde(default)]
    pub ru_guns: Option<crate::director::RuGuns>,
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

/// FPVs: how long one flies (s).
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct FpvSpec {
    pub endurance: f32,
}

static DATA: OnceLock<SimData> = OnceLock::new();

/// The battle's part of the diorama on the table.
pub fn sim_data() -> &'static SimData {
    DATA.get_or_init(bav_core::data::read)
}
