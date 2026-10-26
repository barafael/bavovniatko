//! The picture's part of a diorama's file (`game/dioramas/<name>.ron`, read by `bav_core::data`): which set is raised
//! over the ground, and what the map's woods are grown as. The season, the sun, the sky and the fires are the
//! place's own (`bav_core::data`), read where they are painted.

use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug)]
pub struct LookData {
    /// The set raised over the ground, with its own light.
    pub set: Set,
    #[serde(default)]
    pub woods: Woods,
    /// The town after months of shelling (`buildings::Shelled`): the share of its buildings gutted, and the seed
    /// that picks them. None: the buildings stand whole.
    #[serde(default)]
    pub ruined: Option<(f32, u32)>,
    /// How many of the ruins are still smouldering.
    #[serde(default)]
    pub smouldering: u32,
    /// Burnt-out cars where the drones caught them: where, and which way they face (radians).
    #[serde(default)]
    pub cars: Vec<((f32, f32), f32)>,
}

/// A set: its buildings, its observers' post and its day's light, each a builder of its own (`world`).
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Set {
    /// Vuhledar's ruined panel blocks on the ridge, the dachas and the power line, under February's hand-made light
    /// (`world`).
    PanelBlocks,
    /// The farm by the N08 and the observers' water tower, under the June dawn (`robotyne`).
    WaterTower,
    /// Antonov airport: the map's buildings, the hangar under its barrel roof with the Mriya in it, the tower; the light
    /// under cloud from late morning to full dark (`hostomel`).
    Antonov,
    /// The map's buildings, the coke plant's skyline and the observers on the heap, under the sun computed for the place
    /// and date (`avdiivka`).
    CokePlant,
    /// The map's buildings in ruins and the villages' houses, the Tsentralna mine's headframe and buildings over its
    /// shaft, the observers on a nine-storey roof, the burnt-out cars; the sun (and the moon) computed for the
    /// place and date, the fog through the morning, the dark of a winter night (`myrnohrad`).
    Headframe,
}

/// What the map's woods are grown as.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Woods {
    /// The steppe's: a wood on the map is a shelterbelt grown wide, of the belts' own kinds.
    #[default]
    Steppe,
    /// Pine on the sand (Hostomel's), with some ash and elm.
    Pine,
}

static DATA: OnceLock<LookData> = OnceLock::new();

/// The picture's part of the diorama on the table.
pub fn look_data() -> &'static LookData {
    DATA.get_or_init(bav_core::data::read)
}

#[cfg(test)]
mod tests {
    /// Every diorama names its set.
    #[test]
    fn every_look_reads() {
        for d in bav_core::diorama::Diorama::ALL {
            let _: super::LookData = bav_core::data::parse(&bav_core::data::builtin_text(d)).unwrap_or_else(|e| panic!("{d}: {e}"));
        }
    }
}
