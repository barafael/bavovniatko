//! The game's small shared words: the views, what a camera follows, the ATGMs, the strikes the player can call,
//! the weather. Plain data, so the switches (`dev`), the battle and the picture can all name them.

use bevy::math::{Vec2, Vec4};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    #[default]
    Drone,
    Op,
    /// Eye level at the south edge of a tree line, behind an ATGM launcher.
    Ground,
}

/// What a camera follows: a point, a vehicle of the column, the infantry (`BAV_LOOK`, the showcase), one
/// wave's vehicle by column index, or the attacker's infantry near a point.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Target {
    /// A point on the ground (x, z).
    Point(Vec2),
    /// The attacker's vehicle with this column index, in the earliest wave that has one.
    Vehicle(u32),
    /// That wave's vehicle by column index (`w/i`), where several waves share a road.
    Wave(u32, u32),
    /// The densest group of the attacker's infantry still advancing within 1.4 km.
    Men,
    /// The densest group of the attacker's infantry on foot within 500 m of a point; the point itself when nobody
    /// is there, so a shot never holds a stale aim on an empty field.
    MenNear(Vec2),
    /// For test shots of the fire looks (`BAV_LOOK=hit|pool|grass`): the hull a missile is flying at (or was hit last), the newest
    /// pool fire, the grass fire's front. Located by `dev::env_look`, which can see them.
    Hit,
    Pool,
    Grass,
}

impl std::str::FromStr for Target {
    type Err = std::num::ParseIntError;

    /// `men`, a column index, or `w/i`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "men" {
            Ok(Target::Men)
        } else if s == "hit" {
            Ok(Target::Hit)
        } else if s == "pool" {
            Ok(Target::Pool)
        } else if s == "grass" {
            Ok(Target::Grass)
        } else if let Some((w, i)) = s.split_once('/') {
            Ok(Target::Wave(w.trim().parse()?, i.trim().parse()?))
        } else {
            s.parse().map(Target::Vehicle)
        }
    }
}

/// The ATGMs in the tree lines: Javelin and Stugna-P on the Ukrainian side (dossier c08 §4), Kornet on the
/// Russian (c09 §5).
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::Display, strum::EnumString, serde::Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum Atgm {
    /// Ukrainian, wire-guided (semi-automatic command to line of sight): the gunner holds the sight on the target.
    #[strum(to_string = "Stugna-P", serialize = "stugna")]
    Stugna,
    /// American, fire-and-forget: the seeker locks, the missile climbs and dives onto the roof.
    Javelin,
    /// Russian, laser beam-riding, long-range.
    Kornet,
}

impl Atgm {
    pub fn range(self) -> f32 {
        match self {
            Atgm::Stugna => 4500.0,
            Atgm::Javelin => 2500.0,
            Atgm::Kornet => 5500.0,
        }
    }
    pub fn reload(self) -> f32 {
        match self {
            Atgm::Stugna => 40.0,
            Atgm::Javelin => 25.0,
            Atgm::Kornet => 35.0,
        }
    }
    pub fn speed(self) -> f32 {
        match self {
            Atgm::Stugna => 190.0,
            Atgm::Javelin => 140.0,
            Atgm::Kornet => 250.0,
        }
    }
    /// Missiles carried by a team.
    pub fn load(self) -> u32 {
        match self {
            Atgm::Stugna => 6,
            Atgm::Javelin => 4,
            Atgm::Kornet => 4,
        }
    }
}

/// What the player can call down (and what the dev scripts can call: `BAV_FIRE="100:he,130:raam"`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Strike {
    /// 155 mm high explosive, four rounds.
    He,
    /// Remote anti-armour mines scattered onto a lane, three shells.
    Raam,
    /// Two guided rockets.
    Himars,
    /// Smoke shells: a screen on the ground that hides vehicles from the guns' and the ATGMs' sights.
    Smoke,
}

/// The weather over a day: which deck, how high, whether the sun shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Weather {
    /// 6 February 2023 as ERA5 has it: an unbroken low stratus deck, base 300–500 m, no sun.
    Overcast,
    /// The week's footage (8–11 February): broken stratocumulus and a hard low sun.
    Sun,
    /// 8 June 2023: a cirrus veil the sun shows through, thinning to broken cloud by afternoon.
    Veil,
}

impl Weather {
    /// (cover, base height, underside darkness, veil) at this hour.
    pub fn cloud(self, hour: f32) -> Vec4 {
        match self {
            Weather::Overcast => Vec4::new(1.0, 350.0, 0.82, 0.0),
            Weather::Sun => Vec4::new(0.35, 1300.0, 0.78, 0.05),
            // The veil holds until early afternoon, then breaks into altocumulus with blue between.
            Weather::Veil => {
                let breaking = ((hour - 13.0) / 5.0).clamp(0.0, 1.0);
                Vec4::new(0.95 - 0.4 * breaking, 2500.0 + 2000.0 * breaking, 0.85, 1.0 - 0.7 * breaking)
            }
        }
    }
}

