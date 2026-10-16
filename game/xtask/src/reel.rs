//! The trailer as an action reel (design/trailer.md): the battle picks the shots. An events pass films nothing and
//! logs every blast and every hit on a vehicle (`BAV_TOUR=reel`, `events.txt`); the action is found in it, each
//! moment framed by the drone a couple of seconds before its first blast and held until the smoke rises, and the
//! moments filmed in as many runs as it takes for none of a run's shots to overlap. The day does not depend on
//! the shots, so every run films the day the events pass saw. The reel is cut from the runs: Vuhledar, then Mala
//! Tokmachka, each building to its biggest moment, the FPV's dive last.

use std::{fs, path::Path};

/// A day of the reel: the diorama, its card, when its events pass ends, and its switches: what the player does in
/// it (`BAV_FIRE`, `BAV_FPV`, the same in the events pass and every filming run) and the day's own.
pub struct Day {
    pub part: &'static str,
    /// The card before the day's moments: (title, line, note).
    pub card: (&'static str, &'static str, &'static str),
    pub end: f32,
    pub env: &'static [(&'static str, &'static str)],
}

pub const DAYS: [Day; 2] = [
    Day {
        part: "vuhledar",
        card: ("ВУГЛЕДАР (VUHLEDAR)", "6 February 2023", "Russian columns into the 72nd Brigade's minefields."),
        end: 1440.0,
        // 155 mm on the first column in the kill zone; RAAM on the second column's lane, HIMARS on its tail; 155 mm on
        // the third at the middle belt and on Alga's at the Pavlivka road.
        env: &[("BAV_FIRE", "86:he@1200;90,480:raam@1233;150,505:himars@1335;350,700:he@1150;30,860:he@-870;-10")],
    },
    Day {
        part: "robotyne",
        card: ("МАЛА ТОКМАЧКА (MALA TOKMACHKA)", "8 June 2023", "The 47th Brigade into the Russian minefields."),
        end: 1300.0,
        // 155 mm on the jammer behind the crest; the FPV at the western dug-in T-72; HIMARS on the eastern one. The
        // Bradleys go for the wounded half a minute after each call, as a player would send them.
        env: &[("BAV_FIRE", "334:he@-350;950,640:himars@1050;720"), ("BAV_FPV", "590:-150;724"), ("BAV_EVAC", "30")],
    },
];

/// How many moments each day gets, and how far apart in time and place two must be to be different scenes.
const MOMENTS: usize = 12;
const SAME_SCENE: (f32, f32) = (14.0, 280.0);
/// A shot: from this long before the moment's first blast to this long after its last, within these bounds.
const LEAD: f32 = 1.8;
const TAIL: f32 = 2.4;
const SHOT: (f32, f32) = (4.0, 5.5);

/// What a logged event was (the game's `log_events`: `blast <Cause>`, `hit <weapon>`).
#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    /// The player's FPV going into a hull.
    FpvHit,
    /// A mine under a vehicle.
    MineHit,
    /// A tank's gun hitting a vehicle.
    GunHit,
    /// Anything else hitting a vehicle: an ATGM, a Vikhr, a shell.
    Hit,
    Himars,
    CookOff,
    FpvBlast,
    Mine,
    /// A TOS-1A rocket.
    Thermobaric,
    Shell,
    Atgm,
    /// The tanks' own gunfire: thousands of rounds, small splashes.
    TankGun,
}

impl Kind {
    fn parse(what: &str, tag: &str) -> Option<Kind> {
        Some(match (what, tag) {
            ("hit", "FPV") => Kind::FpvHit,
            ("hit", "a_mine") => Kind::MineHit,
            ("hit", "a_tank_gun") => Kind::GunHit,
            ("hit", _) => Kind::Hit,
            ("blast", "Himars") => Kind::Himars,
            ("blast", "CookOff") => Kind::CookOff,
            ("blast", "Fpv") => Kind::FpvBlast,
            ("blast", "Mine") => Kind::Mine,
            ("blast", "Thermobaric") => Kind::Thermobaric,
            ("blast", "Artillery") => Kind::Shell,
            ("blast", "Atgm") => Kind::Atgm,
            ("blast", "TankGun") => Kind::TankGun,
            _ => return None,
        })
    }

    /// How much it is worth watching: a vehicle hit most, a heavy blast a lot, a shell some, the tanks' own gunfire
    /// hardly at all.
    fn weight(self) -> f32 {
        match self {
            Kind::FpvHit => 9.0,
            Kind::MineHit | Kind::Hit | Kind::FpvBlast => 6.0,
            Kind::Himars => 5.0,
            Kind::CookOff => 4.0,
            Kind::GunHit => 3.0,
            Kind::Mine => 2.0,
            Kind::Thermobaric => 1.6,
            Kind::Atgm => 1.0,
            Kind::Shell => 0.7,
            Kind::TankGun => 0.05,
        }
    }

    /// Whether it can be the heart of a moment on its own (TOS rockets can, as a salvo: `moments`).
    fn heart(self) -> bool {
        // The FPV's hit is the goggles' shot; from the drone its dug-in tank is under the trees.
        self.weight() >= 4.0 && self != Kind::FpvHit
    }
}

/// One logged event.
#[derive(Clone, Copy, Debug)]
struct Event {
    t: f32,
    x: f32,
    z: f32,
    kind: Kind,
}

fn events(path: &Path) -> Result<Vec<Event>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut unknown = 0;
    let ev = text
        .lines()
        .filter_map(|l| {
            let w: Vec<&str> = l.split_whitespace().collect();
            let kind = Kind::parse(w.get(1)?, w.get(2)?);
            unknown += kind.is_none() as usize;
            Some(Event { t: w.first()?.parse().ok()?, x: w.get(3)?.parse().ok()?, z: w.get(4)?.parse().ok()?, kind: kind? })
        })
        .collect();
    if unknown > 0 {
        eprintln!("reel: {unknown} events of kinds the reel does not know, in {} (log_events and Kind::parse disagree)", path.display());
    }
    Ok(ev)
}

/// A moment to film: the drone's shot of it.
#[derive(Clone, Debug)]
pub struct Moment {
    pub from: f32,
    pub secs: f32,
    pub score: f32,
    /// The reel line that films it.
    pub line: String,
}

/// A candidate moment: its score, its first and last blasts, where its weight lies, and how far it spreads.
struct Candidate {
    score: f32,
    first: f32,
    last: f32,
    centre: (f32, f32),
    spread: f32,
}

/// The day's best moments, from its events pass: each heavy event in turn is the heart of a candidate, the events
/// within a few seconds and 150 m of it its body; the best candidates that are not the same scene as a better one
/// win. Only something worth seeing is a heart (a vehicle hit, HIMARS, a cook-off, a TOS salvo): shells on open
/// ground add to a moment but never make one, the first scout's lesson.
pub fn moments(events_txt: &Path) -> Result<Vec<Moment>, String> {
    let ev = events(events_txt)?;
    let near = |a: &Event, e: &Event| (e.t - a.t).abs() <= 4.0 && ((e.x - a.x).powi(2) + (e.z - a.z).powi(2)).sqrt() < 150.0;
    let mut candidates: Vec<Candidate> = Vec::new();
    for a in ev.iter().filter(|a| a.kind.heart() || a.kind == Kind::Thermobaric) {
        let body: Vec<&Event> = ev.iter().filter(|e| near(a, e)).collect();
        if !a.kind.heart() && body.iter().filter(|e| e.kind == Kind::Thermobaric).count() < 4 {
            continue;
        }
        let score: f32 = body.iter().map(|e| e.kind.weight()).sum();
        let w = score.max(1e-3);
        let centre = (body.iter().map(|e| e.x * e.kind.weight()).sum::<f32>() / w, body.iter().map(|e| e.z * e.kind.weight()).sum::<f32>() / w);
        let spread = body.iter().map(|e| ((e.x - centre.0).powi(2) + (e.z - centre.1).powi(2)).sqrt()).fold(0.0, f32::max).min(120.0);
        let (first, last) = body.iter().filter(|e| e.kind.weight() >= 0.5).fold((a.t, a.t), |(f, l), e| (f.min(e.t), l.max(e.t)));
        candidates.push(Candidate { score, first, last, centre, spread });
    }
    candidates.sort_by(|a, b| b.score.total_cmp(&a.score));
    let mut chosen: Vec<Candidate> = Vec::new();
    for c in candidates {
        let same = chosen.iter().any(|o| (o.first - c.first).abs() < SAME_SCENE.0 && ((o.centre.0 - c.centre.0).powi(2) + (o.centre.1 - c.centre.1).powi(2)).sqrt() < SAME_SCENE.1);
        if !same {
            chosen.push(c);
        }
        if chosen.len() == MOMENTS {
            break;
        }
    }
    let mut out: Vec<Moment> = chosen.iter().enumerate().map(|(i, c)| frame(i, c)).collect();
    out.extend(goggles(&ev));
    Ok(out)
}

/// The drone's shot of a moment: about 130 m off and 240 m up, the gimbal some 60 degrees down, from one of eight
/// sides in turn so the reel does not look from one side only, pushing in a little; the field of view holds the
/// moment's spread and a margin. Steep, because the tracks south of Mala Tokmachka run along the tree lines, and
/// at 46 degrees the second scout saw the crowns where the vehicles were.
fn frame(i: usize, c: &Candidate) -> Moment {
    let from = c.first - LEAD;
    let secs = (c.last + TAIL - from).clamp(SHOT.0, SHOT.1);
    let az = (i as f32 * 2.399 + 0.6) % std::f32::consts::TAU;
    let (dx, dz) = (az.sin() * 130.0, az.cos() * 130.0);
    let slant = (130.0f32.powi(2) + 240.0f32.powi(2)).sqrt();
    let fov = (2.0 * ((c.spread + 35.0) / slant).atan()).to_degrees().clamp(7.0, 18.0);
    let (x, z) = c.centre;
    let line = format!("shot {from:.2} {secs:.2} {:.0} {:.0} 250 {:.0} {:.0} 240 {x:.0} {z:.0} {:.1} {fov:.1}", x + dx * 1.08, z + dz * 1.08, x + dx, z + dz, fov * 1.1);
    Moment { from, secs, score: c.score, line }
}

/// The FPV's last seconds through its goggles, to the hit and the static after it.
fn goggles(ev: &[Event]) -> Option<Moment> {
    let hit = ev.iter().find(|e| e.kind == Kind::FpvHit)?;
    let from = hit.t - 10.0;
    Some(Moment { from, secs: 11.5, score: 100.0, line: format!("goggles {from:.2} 11.5") })
}

/// Share the moments among runs so that none of a run's shots overlap (with a second between them, for the
/// filming step to settle): first fit, in time order. Returns each run's moments, as indices into `moments`.
pub fn runs(moments: &[Moment]) -> Vec<Vec<usize>> {
    let mut by_time: Vec<usize> = (0..moments.len()).collect();
    by_time.sort_by(|&a, &b| moments[a].from.total_cmp(&moments[b].from));
    let mut runs: Vec<Vec<usize>> = Vec::new();
    for i in by_time {
        let m = &moments[i];
        match runs.iter_mut().find(|r| r.last().is_none_or(|&l| moments[l].from + moments[l].secs + 1.0 <= m.from)) {
            Some(r) => r.push(i),
            None => runs.push(vec![i]),
        }
    }
    runs
}

/// The order the reel shows a day's moments in, as indices: the second best to open, then from the least up, the
/// best last.
pub fn order(moments: &[Moment]) -> Vec<usize> {
    let mut o: Vec<usize> = (0..moments.len()).collect();
    o.sort_by(|&a, &b| moments[a].score.total_cmp(&moments[b].score));
    if o.len() > 2 {
        let second = o.remove(o.len() - 2);
        o.insert(0, second);
    }
    o
}
