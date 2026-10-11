//! Which diorama is on the table: `vuhledar` (the default) or `robotyne`, chosen by the first command-line
//! argument or `VUHLEDAR_DIORAMA`. The two places share the kit (terrain, vehicles, mines, fire, the drone), and
//! differ in season, ground, forces, and which way the war is going. The player is Ukrainian in both.

use crate::units::Side;
use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Diorama {
    /// 6 February 2023: Russian columns into the 72nd Brigade's minefields (research/chapters/c08-vuhledar.md).
    Vuhledar,
    /// 8 June 2023: the 47th Brigade's breach south of Mala Tokmachka, into the Russian minefields
    /// (research/chapters/c09-robotyne.md §6, set piece 1).
    Robotyne,
}

static DIORAMA: OnceLock<Diorama> = OnceLock::new();

/// The diorama this run plays, fixed at startup.
pub fn diorama() -> Diorama {
    *DIORAMA.get_or_init(|| {
        let pick = std::env::args().nth(1).or_else(|| std::env::var("VUHLEDAR_DIORAMA").ok());
        pick.and_then(|s| s.trim().parse().ok()).unwrap_or(Diorama::Vuhledar)
    })
}

impl Diorama {
    /// Whose columns drive into whose minefields.
    pub fn attacker(self) -> Side {
        match self {
            Diorama::Vuhledar => Side::Ru,
            Diorama::Robotyne => Side::Ua,
        }
    }

    pub fn defender(self) -> Side {
        match self.attacker() {
            Side::Ru => Side::Ua,
            Side::Ua => Side::Ru,
        }
    }

    pub fn summer(self) -> bool {
        self == Diorama::Robotyne
    }

    pub fn title(self) -> &'static str {
        match self {
            Diorama::Vuhledar => "Vuhledar, 6 February 2023",
            Diorama::Robotyne => "Mala Tokmachka, 8 June 2023",
        }
    }

    /// The date as the tally shows it.
    pub fn date(self) -> &'static str {
        match self {
            Diorama::Vuhledar => "06 FEB 2023",
            Diorama::Robotyne => "08 JUN 2023",
        }
    }

    /// Clock time (minutes after midnight) when the day starts.
    pub fn day_start_min(self) -> f32 {
        match self {
            Diorama::Vuhledar => 7.0 * 60.0 + 40.0,
            Diorama::Robotyne => 3.0 * 60.0 + 50.0,
        }
    }

    /// Game minutes per real second.
    pub fn compression(self) -> f32 {
        match self {
            Diorama::Vuhledar => 0.36,
            Diorama::Robotyne => 0.30,
        }
    }
}
