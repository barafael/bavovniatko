//! The Allotment (наряд сил): the player's side for the day, chosen from the diorama's catalog of Units
//! (підрозділи) and Support (підтримка) under its allotment points (design/command.md).
//!
//! In the Record (літопис) the allotment is the day's own, the `record` in the diorama's file, and the battle is
//! the one the archive describes. In Command (командування) the player picks; until the map table exists,
//! `BAV_ALLOT` names the picks (`stugna@0,javelin@3,155*6,himars*2`: a unit at a position, or so many of an item),
//! and without it Command plays the record's allotment too. Everything on the player's side that used to be read
//! straight from the file (the posts and their weapons, the tank in its hide, the stock of rounds and missions,
//! the FPVs, the vehicles for the wounded) is read from here, so the Record is Command with the record's picks.

use crate::units::VKind;
use bav_core::{diorama::Play, vocab::Atgm};
use bevy::math::Vec2;
use serde::Deserialize;
use std::{collections::HashMap, sync::OnceLock};

/// A Unit: something on the map, placed or sent.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum UnitKind {
    /// Riflemen at a position in a tree line or a trench, and maybe an ATGM team with them.
    Post(Option<Atgm>),
    /// A tank in a hide behind the line, firing on coordinates from a drone pilot.
    Hide,
    /// A vehicle for the columns (the script's in the Record; ordered in with Supply in Command).
    Vehicle(VKind),
    /// A vehicle held back to go in for the wounded when the player sends it.
    Evacuation,
}

/// Support: off the map, called rather than placed.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum SupportKind {
    /// So many 155 mm rounds for the player's missions.
    Shells(u32),
    /// A RAAM mission: remote mines scattered onto a lane.
    Raam,
    /// A HIMARS mission: two guided rockets.
    Himars,
    /// A smoke mission.
    Smoke,
    /// So many rounds for the brigade's guns on their registered points, which fire by themselves.
    Brigade(u32),
    /// So many 120 mm bombs for the battalion's mortars, which work over infantry in the open by themselves.
    Mortar(u32),
    /// A strike drone.
    Fpv,
}

/// One entry of the catalog: what it is, called what, what it costs in allotment points (and later in Supply),
/// and how many there were that day.
#[derive(Deserialize, Debug, Clone)]
pub struct Entry<K> {
    pub id: String,
    /// (English, Ukrainian).
    pub name: (String, String),
    pub what: K,
    pub cost: u32,
    /// What a column of it costs in Supply during the battle (Command, design/command.md phase 6).
    #[serde(default)]
    pub supply: u32,
    pub available: u32,
}

/// A pick: so many of an entry, or one unit at a position (an index into the field's positions,
/// `bav_core::field`: the diorama's own posts first, then its hides, then the tree lines and ruins).
#[derive(Deserialize, Debug, Clone)]
pub struct Pick {
    pub id: String,
    #[serde(default = "one")]
    pub count: u32,
    #[serde(default)]
    pub at: Option<usize>,
}

fn one() -> u32 {
    1
}

/// The diorama's allotment rules: the points, the Supply that comes in during the battle, and the day's own picks.
#[derive(Deserialize, Debug, Clone)]
pub struct AllotmentSpec {
    pub points: u32,
    /// Supply at the start, and how much comes in each minute of the day's clock (Command, phase 6).
    #[serde(default)]
    pub supply: (u32, f32),
    /// The day as it was.
    pub record: Vec<Pick>,
}

/// The rounds and missions the player's side has for the day.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Stock {
    pub shells: u32,
    pub raam: u32,
    pub himars: u32,
    pub smoke: u32,
    pub brigade_shells: u32,
    pub mortar_bombs: u32,
}

/// The player's side as allotted: what the battle builds it from.
#[derive(Debug, Clone, Default)]
pub struct Allotment {
    /// The posts, in the layout's position order: where, and the weapon (none: riflemen only).
    pub posts: Vec<(Vec2, Option<Atgm>)>,
    /// Tanks in their hides: where, and which way they face.
    pub hides: Vec<(Vec2, Vec2)>,
    /// The vehicles for the columns, by kind.
    pub vehicles: HashMap<VKind, u32>,
    pub stock: Stock,
    pub fpvs: u32,
    /// Vehicles held back for the wounded.
    pub evacuation: u32,
    /// What it cost, of how many points.
    pub spent: u32,
    pub points: u32,
}

static ALLOTMENT: OnceLock<Allotment> = OnceLock::new();

/// The player's side for this run.
pub fn allotment() -> &'static Allotment {
    ALLOTMENT.get_or_init(|| {
        let d = crate::data::sim_data();
        let place = bav_core::diorama::diorama();
        let record = build(&d.allotment.record).unwrap_or_else(|e| panic!("{place} record allotment: {e}"));
        check_record(&record);
        let a = match (bav_core::diorama::play(), bav_core::dev::dev().allot.as_deref()) {
            (Play::Command, Some(spec)) => parse(spec).and_then(|picks| build(&picks)).unwrap_or_else(|e| panic!("{place} allotment (BAV_ALLOT): {e}")),
            _ => record,
        };
        bevy::log::info!("allotment ({}): {} of {} points, {} posts, {} hides, {:?}", bav_core::diorama::play(), a.spent, a.points, a.posts.len(), a.hides.len(), a.stock);
        a
    })
}

/// The Record's vehicles are the script's: our columns in the day's script are made of exactly what the record
/// allots (a file whose two halves disagree stops here, not in a day that quietly differs).
fn check_record(record: &Allotment) {
    let d = crate::data::sim_data();
    let mut scripted: HashMap<VKind, u32> = HashMap::new();
    if bav_core::diorama::diorama().attacker() == bav_core::diorama::Side::Ua {
        for (_, act) in &d.script {
            if let crate::director::Act::Column { kinds, rescue: false, .. } = act {
                for k in kinds {
                    *scripted.entry(*k).or_default() += 1;
                }
            }
        }
    }
    assert_eq!(record.vehicles, scripted, "the record allotment's vehicles are not the script's columns");
}

/// `stugna@0,javelin@3,155*6,himars*2`: an id, then `@position` or `*count`.
fn parse(spec: &str) -> Result<Vec<Pick>, String> {
    spec.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (id, at, count) = if let Some((id, at)) = s.split_once('@') {
                (id, Some(at.parse::<usize>().map_err(|e| format!("{s}: {e}"))?), 1)
            } else if let Some((id, n)) = s.split_once('*') {
                (id, None, n.parse::<u32>().map_err(|e| format!("{s}: {e}"))?)
            } else {
                (s, None, 1)
            };
            Ok(Pick { id: id.to_string(), count, at })
        })
        .collect()
}

/// The side from the picks, checked against the catalog: each id known, no more of one than there were, units
/// that need a place given one (and no place given twice), and the whole within the points.
pub fn build(picks: &[Pick]) -> Result<Allotment, String> {
    let d = crate::data::sim_data();
    let positions = &bav_core::field::field().positions;
    let mut a = Allotment { points: d.allotment.points, ..Default::default() };
    // What stands at each position: a post (with its weapon) or a tank.
    let mut held: Vec<Option<UnitKind>> = vec![None; positions.len()];
    let mut taken: HashMap<&str, u32> = HashMap::new();
    for p in picks {
        let unit = d.catalog.iter().find(|u| u.id == p.id);
        let support = d.support.iter().find(|s| s.id == p.id);
        let (cost, available) = match (unit, support) {
            (Some(u), _) => (u.cost, u.available),
            (_, Some(s)) => (s.cost, s.available),
            _ => return Err(format!("no {} in the catalog", p.id)),
        };
        let n = taken.entry(p.id.as_str()).or_default();
        *n += p.count;
        if *n > available {
            return Err(format!("{} of {}, but there were {}", n, p.id, available));
        }
        a.spent += cost * p.count;
        if let Some(u) = unit {
            match u.what {
                UnitKind::Post(_) | UnitKind::Hide => {
                    let at = p.at.ok_or_else(|| format!("{} needs a position (@n)", p.id))?;
                    let pos = positions.get(at).ok_or_else(|| format!("{}@{at}: there are {} positions", p.id, positions.len()))?;
                    let kind = pos.kind.name().0;
                    if pos.ground != bav_core::field::Ground::Ours {
                        return Err(format!("{}@{at}: the {kind} at {:.0},{:.0} is not on our side of the grey zone", p.id, pos.at.x, pos.at.y));
                    }
                    let fits = if matches!(u.what, UnitKind::Hide) { pos.kind.holds_vehicles() } else { pos.kind.holds_men() };
                    if !fits {
                        return Err(format!("{}@{at}: a {kind} does not hold it", p.id));
                    }
                    if held[at].is_some() {
                        return Err(format!("position {at} given twice"));
                    }
                    held[at] = Some(u.what);
                }
                UnitKind::Vehicle(kind) => *a.vehicles.entry(kind).or_default() += p.count,
                UnitKind::Evacuation => a.evacuation += p.count,
            }
        } else if let Some(s) = support {
            let st = &mut a.stock;
            match s.what {
                SupportKind::Shells(r) => st.shells += r * p.count,
                SupportKind::Raam => st.raam += p.count,
                SupportKind::Himars => st.himars += p.count,
                SupportKind::Smoke => st.smoke += p.count,
                SupportKind::Brigade(r) => st.brigade_shells += r * p.count,
                SupportKind::Mortar(r) => st.mortar_bombs += r * p.count,
                SupportKind::Fpv => a.fpvs += p.count,
            }
        }
    }
    if a.spent > a.points {
        return Err(format!("{} points, of {}", a.spent, a.points));
    }
    // In the positions' order, which for the Record's posts is the layout's, as it always was: their order sets their
    // reload and fire timers.
    for (h, pos) in held.iter().zip(positions) {
        match h {
            Some(UnitKind::Post(weapon)) => a.posts.push((pos.at, *weapon)),
            Some(UnitKind::Hide) => a.hides.push((pos.at, pos.facing)),
            _ => {}
        }
    }
    Ok(a)
}
