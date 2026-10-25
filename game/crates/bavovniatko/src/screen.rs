//! The screen's part of a diorama's file (`game/dioramas/<name>.ron`, read by `bav_core::data`): the tally's and the
//! fire panel's lines, the debrief's rows, and where the development views start. The words are the place's; the
//! code fills in the figures and adds what the day has (aircraft, areas, a runway, the drone's grenades) where the
//! battle has it, whatever the place.

use crate::units::Side;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug)]
pub struct ScreenData {
    pub hud: HudSpec,
    pub debrief: DebriefSpec,
    /// `BAV_VIEW=ground` starts with the ATGM team nearest this point.
    pub ground_team: (f32, f32),
}

/// The two counters' lines, as templates: `{name}` is a figure the HUD fills in ([`TALLY_KEYS`], [`STOCK_KEYS`]).
#[derive(Deserialize, Debug)]
pub struct HudSpec {
    /// The tally under the clock, top right.
    pub tally: String,
    /// What is left to fire, bottom left.
    pub stock: String,
}

/// What the tally's template can name: the date and the clock; the vehicles and the men; what is in the air and on
/// the ground where the day has aircraft, areas, a fight and a runway. `air` and `zones` are whole phrases, empty
/// where the day has none.
pub const TALLY_KEYS: &[&str] = &["date", "clock", "committed", "out", "withdrawn", "sent", "returned", "air", "zones", "air_down", "landed", "men_out", "captured", "taken", "runway"];

/// What the fire panel's template can name. `vog`, `ad` and `pull_out` are whole phrases, empty where the day has
/// no grenades, no air defence left, or nobody to pull out; `speed` is the time's speed when it is not 1.
pub const STOCK_KEYS: &[&str] = &["shells", "cluster", "raam", "himars", "smoke", "illum", "fpv", "vog", "ad", "teams", "missiles", "atgm_mode", "order", "pull_out", "speed"];

/// The end of the day: the heading's place (none: the diorama's own), the rows of its account, and the table of
/// its columns (none: no table).
#[derive(Deserialize, Debug)]
pub struct DebriefSpec {
    #[serde(default)]
    pub place: Option<(String, String)>,
    pub rows: Vec<Row>,
    #[serde(default)]
    pub columns: Option<Columns>,
}

/// One line of the debrief's account, or several.
#[derive(Deserialize, Debug)]
pub enum Row {
    /// A label and its figure. A label that starts with spaces is set dim, under the one before.
    Line(String, Figure),
    /// These rows, where the day has the thing.
    If(Gate, Vec<Row>),
    /// A row per area of the field: who holds it at the end, the label with `{area}` for its name.
    Areas(String),
    /// A row per hit on the duel's tank (each the same hit again counted once), or this line when nothing reached it.
    TankHits(String),
    /// A row per run the Bradley pair made, at its time.
    Runs,
}

/// What a set of rows needs the day to have.
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Gate {
    /// Aircraft have come over.
    Flew,
    /// The field has areas held, or fought for.
    Field,
    /// An airfield, its runway counted.
    Runway,
    /// The drone carries grenades.
    Drops,
}

/// A figure of the day's account.
#[derive(Deserialize, Debug, Clone)]
pub enum Figure {
    Committed,
    /// Destroyed and abandoned.
    Out,
    Destroyed,
    Abandoned,
    MineHits,
    AtgmKills,
    ArtilleryKills,
    TurretTosses,
    Withdrawn,
    Sent,
    Returned,
    /// Men picked up in the field by an evacuation vehicle.
    PickedUp,
    AttackerDead,
    Wounded,
    WoundedOut,
    /// Our wounded still in the field.
    WoundedLying,
    /// How close any attacker came to the town's edge, in metres.
    Closest,
    DefenderLosses,
    DefenderVehicles,
    OverwatchLosses,
    /// What is left of what there was: "n of m".
    Shells,
    Smoke,
    /// The evacuation vehicles sent, of all there were.
    Evacuations,
    /// How far short of the trench the furthest vehicle stopped, and the furthest man on foot got.
    VehicleReach,
    FootReach,
    /// The enemy's positions silenced, of all of them.
    PositionsSilenced,
    /// Their dug-in tanks knocked out, of all of them.
    DugInKnockedOut,
    /// Aircraft flown, shot down (all of them; of a kind; by a weapon), hit and turned away.
    Flown,
    Down,
    DownKind(bav_sim::air::AirKind),
    DownBy(String),
    DrivenOff,
    /// Paratroopers landed on the airfield.
    MenLanded,
    /// Our men who walked out, were taken, and when the field stopped holding.
    MenOut,
    Captured,
    HeldUntil,
    /// The areas a side holds at the end, by name.
    Held(Side),
    /// The runway's longest whole stretch, and the airlift.
    Runway,
    Airlift,
    IglasLeft,
    /// The drone's grenades dropped, of all, and the men they killed.
    Drops,
    /// The duel's tank: what became of it; the other hulls knocked out; the trench; the pair's 25 mm fired; the FPVs
    /// left.
    TankFate,
    OtherHulls,
    Trench,
    Fired25,
    FpvsLeft,
}

/// The table of columns: its three heads, the columns' names by number from 1, and the name of one past them, with
/// `{n}` for its number.
#[derive(Deserialize, Debug)]
pub struct Columns {
    pub heads: (String, String, String),
    #[serde(default)]
    pub names: Vec<String>,
    pub other: String,
}

impl Columns {
    pub fn name(&self, i: usize) -> String {
        self.names.get(i - 1).cloned().unwrap_or_else(|| self.other.replace("{n}", &i.to_string()))
    }
}

/// A template with its `{name}`s filled in, each by what `value` gives it, in one pass (a figure's own text is not
/// read again).
pub fn fill(template: &str, value: impl Fn(&str) -> String) -> String {
    let mut out = String::with_capacity(template.len() + 64);
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let close = rest[open..].find('}').map_or(rest.len(), |k| open + k);
        out.push_str(&value(&rest[open + 1..close]));
        rest = &rest[(close + 1).min(rest.len())..];
    }
    out.push_str(rest);
    out
}

/// The names a template uses that are not in `keys`.
fn unknown<'a>(template: &'a str, keys: &[&str]) -> Vec<&'a str> {
    let mut bad = Vec::new();
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        let close = rest[open..].find('}').map_or(rest.len(), |k| open + k);
        let key = &rest[open + 1..close];
        if !keys.contains(&key) {
            bad.push(key);
        }
        rest = &rest[(close + 1).min(rest.len())..];
    }
    bad
}

impl ScreenData {
    /// What in the file the screen cannot fill in.
    fn check(&self) -> Result<(), String> {
        for (what, template, keys) in [("hud.tally", &self.hud.tally, TALLY_KEYS), ("hud.stock", &self.hud.stock, STOCK_KEYS)] {
            let bad = unknown(template, keys);
            if !bad.is_empty() {
                return Err(format!("{what} names {bad:?}, which is none of {keys:?}"));
            }
        }
        Ok(())
    }
}

static DATA: OnceLock<ScreenData> = OnceLock::new();

/// The screen's part of the diorama on the table.
pub fn screen() -> &'static ScreenData {
    DATA.get_or_init(|| {
        let d: ScreenData = bav_core::data::read();
        d.check().unwrap_or_else(|e| panic!("{}: {e}", bav_core::data::source().0));
        d
    })
}

#[cfg(test)]
mod tests {
    /// Every diorama's screen reads, and its templates name only what the HUD fills in.
    #[test]
    fn every_screen_reads() {
        for d in bav_core::diorama::Diorama::ALL {
            let s: super::ScreenData = bav_core::data::parse(&bav_core::data::builtin_text(d)).unwrap_or_else(|e| panic!("{d}: {e}"));
            s.check().unwrap_or_else(|e| panic!("{d}: {e}"));
        }
    }

    #[test]
    fn templates_fill_once() {
        let t = super::fill("a {x} b {y}", |k| if k == "x" { "{y}".into() } else { "2".into() });
        assert_eq!(t, "a {y} b 2");
    }
}
