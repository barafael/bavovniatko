//! The days the tasks film, from the game's own files: `dioramas/index.ron` lists the dioramas in order, and the
//! shots file each names (`dioramas/shots/<id>.ron`) has its tour's title card and, for a day in the trailer and the
//! film, its `reel`. A day with no shots file has no tour. The shots themselves are the game's to read.

use serde::Deserialize;
use std::path::Path;

/// A diorama of the index: its id, and its shots file.
#[derive(Deserialize)]
struct Entry {
    id: String,
    #[serde(default)]
    shots: Option<String>,
}

/// What the tasks read of a shots file.
#[derive(Deserialize)]
struct ShotsFile {
    card: (String, String),
    #[serde(default)]
    reel: Option<Reel>,
}

/// A day of the trailer and the film: the card before its moments (title, line, note), when its events pass ends,
/// and its switches: what the player does in it (`BAV_FIRE`, `BAV_FPV`, the same in the events pass and every
/// filming run) and the day's own.
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Reel {
    pub card: (String, String, String),
    pub end: f32,
    pub env: Vec<(String, String)>,
}

/// A day with a tour: its id (as `BAV_DIORAMA` names it), its title card (the place, the date), and its day in the
/// trailer, if it has one.
pub struct Day {
    pub part: String,
    pub card: (String, String),
    pub reel: Option<Reel>,
}

/// RON as the dioramas are written: with implicit `Some`.
fn parse<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME).from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Every diorama's id, in the index's order.
pub fn ids(root: &Path) -> Result<Vec<String>, String> {
    let index: Vec<Entry> = parse(&root.join("dioramas/index.ron"))?;
    Ok(index.into_iter().map(|e| e.id).collect())
}

/// The days that have a tour, in the index's order.
pub fn days(root: &Path) -> Result<Vec<Day>, String> {
    let dir = root.join("dioramas");
    let index: Vec<Entry> = parse(&dir.join("index.ron"))?;
    let mut days = Vec::new();
    for e in index {
        let Some(shots) = e.shots else { continue };
        let file: ShotsFile = parse(&dir.join(shots))?;
        days.push(Day { part: e.id, card: file.card, reel: file.reel });
    }
    if days.is_empty() {
        return Err("dioramas/index.ron names no shots file: there is nothing to film".into());
    }
    Ok(days)
}

#[cfg(test)]
mod tests {
    /// The trailer's days read as they were written in code: Vuhledar, then Mala Tokmachka, their cards, ends and
    /// switches.
    #[test]
    fn the_reel_reads() {
        let days = super::days(&crate::root()).unwrap();
        let reel: Vec<(&str, &super::Reel)> = days.iter().filter_map(|d| d.reel.as_ref().map(|r| (d.part.as_str(), r))).collect();
        let s = |a: &str, b: &str| (a.to_string(), b.to_string());
        assert_eq!(reel.len(), 2);
        assert_eq!(reel[0].0, "vuhledar");
        assert_eq!(*reel[0].1, super::Reel { card: ("ВУГЛЕДАР (VUHLEDAR)".into(), "6 February 2023".into(), "Russian columns into the 72nd Brigade's minefields.".into()), end: 1440.0, env: vec![s("BAV_FIRE", "86:he@1200;90,480:raam@1233;150,505:himars@1335;350,700:he@1150;30,860:he@-870;-10")] });
        assert_eq!(reel[1].0, "robotyne");
        assert_eq!(*reel[1].1, super::Reel { card: ("МАЛА ТОКМАЧКА (MALA TOKMACHKA)".into(), "8 June 2023".into(), "The 47th Brigade into the Russian minefields.".into()), end: 1300.0, env: vec![s("BAV_FIRE", "334:he@-350;950,640:himars@1050;720"), s("BAV_FPV", "590:-150;724"), s("BAV_EVAC", "30")] });
        let card = |id: &str| days.iter().find(|d| d.part == id).map(|d| d.card.clone()).unwrap();
        assert_eq!(card("vuhledar"), s("VUHLEDAR", "6 FEBRUARY 2023"));
        assert_eq!(card("robotyne"), s("SOUTH OF MALA TOKMACHKA", "8 JUNE 2023"));
        assert_eq!(card("hostomel-1"), s("HOSTOMEL, ANTONOV AIRPORT", "24 FEBRUARY 2022, THE LANDING"));
        assert_eq!(card("hostomel-2"), s("HOSTOMEL, ANTONOV AIRPORT", "24 FEBRUARY 2022, DUSK"));
    }
}
