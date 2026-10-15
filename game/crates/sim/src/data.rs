//! The battle's part of a diorama's file (`game/dioramas/<name>.ron`, read by `bav_core::data`): what each side
//! brings to the day and the day's script. The script runs whether or not the player does anything.

use crate::director::Act;
use crate::units::VKind;
use bav_core::vocab::Atgm;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug)]
pub struct SimData {
    /// What the player's fires have for the day.
    pub stock: Stock,
    /// The weapon at each position, in the layout's order (none: riflemen only).
    pub ua_weapons: Vec<Option<Atgm>>,
    #[serde(default)]
    pub ru_weapons: Vec<Option<Atgm>>,
    /// A Ukrainian tank in a hide, firing on coordinates from a drone pilot: where, and which way it faces.
    #[serde(default)]
    pub wanderer: Option<((f32, f32), (f32, f32))>,
    /// Tanks dug in hull-down on the defenders' line: where, and which way they face.
    #[serde(default)]
    pub dug_in: Vec<((f32, f32), f32)>,
    /// What the old wrecks were, in turn.
    pub old_wreck_kinds: Vec<VKind>,
    /// The defending brigade's guns on their registered points: every so many seconds, so many rounds.
    pub registered_fire: (f64, u32),
    /// The vehicles the player can send in for the wounded (none: nothing gets there by day).
    #[serde(default)]
    pub evacuation: Option<crate::evac::EvacSpec>,
    /// Where the drone team flies from, and how often an hour (of the day's clock) the enemy finds it while a drone
    /// is up: often in early 2023, when DJI's AeroScope gave the pilot away; rarely south of Mala Tokmachka.
    pub operator: OperatorSpec,
    /// The strike drones the player has (none: the drone is eyes only).
    #[serde(default)]
    pub fpv: Option<FpvSpec>,
    /// The day: at each game second, what happens.
    pub script: Vec<(f32, Act)>,
}

/// The player's fires for the day, and the defending brigade's own.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Stock {
    /// 155 mm rounds for the player's missions.
    pub shells: u32,
    pub raam: u32,
    pub himars: u32,
    pub smoke: u32,
    /// The defending brigade's guns, which fire on their registered points by themselves.
    pub brigade_shells: u32,
    /// The battalion's 120 mm mortars, which work over infantry in the open on their own.
    pub mortar_bombs: u32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct OperatorSpec {
    pub pos: (f32, f32),
    pub found_per_hour: f32,
}

/// FPVs: how many for the day, and how long one flies (s).
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct FpvSpec {
    pub count: u32,
    pub endurance: f32,
}

static DATA: OnceLock<SimData> = OnceLock::new();

/// The battle's part of the diorama on the table.
pub fn sim_data() -> &'static SimData {
    DATA.get_or_init(bav_core::data::read)
}
