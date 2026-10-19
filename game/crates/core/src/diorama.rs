//! Which diorama is on the table: `vuhledar` (the default), `robotyne`, `hostomel-1` or `hostomel-2`, chosen by the first command-line
//! argument or `BAV_DIORAMA`. The two places share the kit (terrain, vehicles, mines, fire, the drone), and
//! differ in season, ground, forces, and which way the war is going. The player is Ukrainian in both.

use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub enum Side {
    Ru,
    Ua,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Diorama {
    /// 6 February 2023: Russian columns into the 72nd Brigade's minefields (db/research/chapters/c08-vuhledar.md).
    Vuhledar,
    /// 8 June 2023: the 47th Brigade's breach south of Mala Tokmachka, into the Russian minefields
    /// (db/research/chapters/c09-robotyne.md §6, set piece 1).
    Robotyne,
    /// 24 February 2022, Antonov airport at Hostomel, morning: the VDV come in by helicopter onto the National
    /// Guard's rear echelon (db/research/chapters/c01-march-on-kyiv.md; design/hostomel.md, sub-chapter I).
    #[strum(serialize = "hostomel-1")]
    HostomelLanding,
    /// The same day at dusk: the counterattack that retakes the field and craters the runway (sub-chapter II).
    #[strum(serialize = "hostomel-2")]
    HostomelCounter,
}

/// How the day is played (design/command.md).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Play {
    /// The Record (Літопис): the day as it was. The forces, the stock and the script are the archive's; the player
    /// matters at the margin.
    #[default]
    Record,
    /// Command (Командування): the player's side is the player's Allotment (наряд сил), chosen from what was there
    /// that day under its points.
    Command,
}

static PLAY: OnceLock<Play> = OnceLock::new();

/// How this run plays the day, fixed at startup: `BAV_PLAY=record|command`; unset, Command when `BAV_ALLOT` gives its
/// picks, else the Record.
pub fn play() -> Play {
    *PLAY.get_or_init(|| {
        let allotted = std::env::var_os("BAV_ALLOT").is_some_and(|v| !v.is_empty());
        std::env::var("BAV_PLAY").ok().and_then(|s| s.trim().parse().ok()).unwrap_or(if allotted { Play::Command } else { Play::Record })
    })
}

static DIORAMA: OnceLock<Diorama> = OnceLock::new();

/// The diorama this run plays, fixed at startup.
pub fn diorama() -> Diorama {
    *DIORAMA.get_or_init(|| {
        // The first argument names it unless that is a flag (a test harness passes its own).
        let pick = std::env::args().nth(1).filter(|a| !a.starts_with('-')).or_else(|| std::env::var("BAV_DIORAMA").ok());
        pick.and_then(|s| s.trim().parse().ok()).unwrap_or(Diorama::Vuhledar)
    })
}

impl Diorama {
    /// Whose columns drive into whose minefields.
    pub fn attacker(self) -> Side {
        crate::data::data().attacker
    }

    pub fn defender(self) -> Side {
        match self.attacker() {
            Side::Ru => Side::Ua,
            Side::Ua => Side::Ru,
        }
    }

    pub fn summer(self) -> bool {
        crate::data::data().summer
    }

    /// "Vuhledar (Вугледар), 6 February 2023".
    pub fn title(self) -> String {
        let d = crate::data::data();
        format!("{} ({}), {}", d.place.0, d.place.1, d.date)
    }

    /// The date as the tally shows it.
    pub fn tally_date(self) -> &'static str {
        &crate::data::data().tally_date
    }

    /// Clock time (minutes after midnight) when the day starts.
    pub fn day_start_min(self) -> f32 {
        crate::data::data().day_start_min
    }

    /// Game minutes per real second.
    pub fn compression(self) -> f32 {
        crate::data::data().compression
    }
}
