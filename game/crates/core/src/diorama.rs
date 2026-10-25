//! Which diorama is on the table: one of `game/dioramas/index.ron`'s ids (`vuhledar`, the first, by default;
//! `robotyne`, `hostomel-1`, `avdiivka-3`, …), chosen by the first command-line argument or `BAV_DIORAMA`. The places
//! share the kit (terrain, vehicles, mines, fire, the drone) and differ in what their files say: the season, the
//! ground, the forces, the set, and which way the war is going. The player is Ukrainian in all of them.

use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub enum Side {
    Ru,
    Ua,
}

/// A diorama, by its place in the index. What it is (its id, its files, its tours' shots) is the index's, built into
/// the binary by bav-core's build script; what it holds is its files'.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Diorama(usize);

/// An entry of the index as the build script writes it: the id, the files (name, text) merged in order into one
/// record, and the shots file of its tours.
pub(crate) struct Builtin {
    pub id: &'static str,
    pub files: &'static [(&'static str, &'static str)],
    pub shots: Option<(&'static str, &'static str)>,
}

include!(concat!(env!("OUT_DIR"), "/dioramas.rs"));

impl std::fmt::Display for Diorama {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id())
    }
}

impl std::str::FromStr for Diorama {
    type Err = String;

    /// An id of the index, in any case.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BUILTIN.iter().position(|b| b.id.eq_ignore_ascii_case(s)).map(Diorama).ok_or_else(|| format!("no diorama {s:?} in dioramas/index.ron"))
    }
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
        pick.and_then(|s| s.trim().parse().ok()).unwrap_or(Diorama::ALL[0])
    })
}

impl Diorama {
    /// Every diorama, in the index's order (the order the READMEs give them).
    pub const ALL: [Diorama; COUNT] = {
        let mut all = [Diorama(0); COUNT];
        let mut i = 0;
        while i < COUNT {
            all[i] = Diorama(i);
            i += 1;
        }
        all
    };

    /// Its id, as the command line and `BAV_DIORAMA` name it.
    pub fn id(self) -> &'static str {
        BUILTIN[self.0].id
    }

    /// Its built-in files, (name, text), in the order they are merged.
    pub fn files(self) -> &'static [(&'static str, &'static str)] {
        BUILTIN[self.0].files
    }

    /// Its built-in shots file (`dioramas/shots/`), if it has tours.
    pub fn shots(self) -> Option<(&'static str, &'static str)> {
        BUILTIN[self.0].shots
    }

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

    /// The season the place is painted for.
    pub fn season(self) -> crate::data::Season {
        crate::data::data().season
    }

    /// Whether the ground is bare of snow: summer's and autumn's dust, fields and leaves rather than winter's snow.
    pub fn summer(self) -> bool {
        self.season() != crate::data::Season::Winter
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
