//! The trailer as an action reel (design/trailer.md): the battle picks the shots. An events pass films nothing and
//! logs every blast and every hit on a vehicle (`BAV_TOUR=reel`, `events.txt`); the action is found in it, each
//! moment framed by the drone a couple of seconds before its first blast and held until the smoke rises, and the
//! moments filmed in as many runs as it takes for none of a run's shots to overlap. The day does not depend on
//! the shots, so every run films the day the events pass saw. The reel is cut from the runs: Vuhledar, then Mala
//! Tokmachka, each building to its biggest moment, the FPV's dive last.

use std::{fs, path::Path};

/// A day of the reel: the diorama, what the player does in it (in its events pass and its filming runs alike, as
/// reel lines), when its events pass ends, and its own switches.
pub struct Day {
    pub part: &'static str,
    pub plan: &'static str,
    pub end: f32,
    pub extra: &'static [(&'static str, &'static str)],
}

pub const DAYS: [Day; 2] = [
    Day {
        part: "vuhledar",
        // 155 mm on the first column in the kill zone; RAAM on the second column's lane, HIMARS on its tail; 155 mm on
        // the third at the middle belt and on Alga's at the Pavlivka road.
        plan: "fire 86 he 1200 90\nfire 480 raam 1233 150\nfire 505 himars 1335 350\nfire 700 he 1150 30\nfire 860 he -870 -10\n",
        end: 1440.0,
        extra: &[],
    },
    Day {
        part: "robotyne",
        // 155 mm on the jammer behind the crest; the FPV at the western dug-in T-72; HIMARS on the eastern one. The
        // Bradleys go for the wounded half a minute after each call, as a player would send them.
        plan: "fire 334 he -350 950\nfpv 590 -150 724\nfire 640 himars 1050 720\n",
        end: 1300.0,
        extra: &[("BAV_EVAC", "30")],
    },
];

/// How many moments each day gets, and how far apart in time and place two must be to be different scenes.
const MOMENTS: usize = 12;
const SAME_SCENE: (f32, f32) = (14.0, 280.0);
/// A shot: from this long before the moment's first blast to this long after its last, within these bounds.
const LEAD: f32 = 1.8;
const TAIL: f32 = 2.4;
const SHOT: (f32, f32) = (4.0, 5.5);

/// One logged event.
#[derive(Clone, Copy, Debug)]
struct Event {
    t: f32,
    x: f32,
    z: f32,
    weight: f32,
}

/// How much a thing on the log is worth watching: a vehicle hit most, a heavy blast a lot, a shell some, the
/// tanks' own gunfire (thousands of rounds, small splashes) hardly at all.
fn weight(kind: &str, what: &str) -> f32 {
    match (kind, what) {
        ("hit", "FPV") => 9.0,
        ("hit", "a_mine") => 6.0,
        ("hit", "a_tank_gun") => 3.0,
        ("hit", _) => 6.0,
        ("blast", "Himars") => 5.0,
        ("blast", "CookOff") => 4.0,
        ("blast", "Fpv") => 6.0,
        ("blast", "Mine") => 2.0,
        ("blast", "Thermobaric") => 1.6,
        ("blast", "Artillery") => 0.7,
        ("blast", "Atgm") => 1.0,
        ("blast", "TankGun") => 0.05,
        _ => 0.5,
    }
}

fn events(path: &Path) -> Result<Vec<Event>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(text
        .lines()
        .filter_map(|l| {
            let w: Vec<&str> = l.split_whitespace().collect();
            Some(Event { t: w.first()?.parse().ok()?, x: w.get(3)?.parse().ok()?, z: w.get(4)?.parse().ok()?, weight: weight(w.get(1)?, w.get(2)?) })
        })
        .collect())
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

/// The day's best moments, from its events pass: each heavy event in turn is the heart of a candidate, the events
/// within a few seconds and 150 m of it its body; the best candidates that are not the same scene as a better one
/// win. Only something worth seeing is a heart (a vehicle hit, HIMARS, a cook-off, the FPV, a TOS salvo): shells
/// on open ground add to a moment but never make one, the first scout's lesson.
pub fn moments(events_txt: &Path) -> Result<Vec<Moment>, String> {
    let ev = events(events_txt)?;
    let mut candidates: Vec<(f32, f32, f32, [f32; 2], f32)> = Vec::new(); // (score, first, last, centre, spread)
    for a in &ev {
        let body: Vec<&Event> = ev.iter().filter(|e| (e.t - a.t).abs() <= 4.0 && ((e.x - a.x).powi(2) + (e.z - a.z).powi(2)).sqrt() < 150.0).collect();
        let salvo = a.weight == 1.6 && body.iter().filter(|e| e.weight == 1.6).count() >= 4;
        // The FPV's hit is the goggles' shot; from the drone its dug-in tank is under the trees.
        if (a.weight < 4.0 && !salvo) || a.weight >= 9.0 {
            continue;
        }
        let score: f32 = body.iter().map(|e| e.weight).sum();
        let w: f32 = body.iter().map(|e| e.weight).sum::<f32>().max(1e-3);
        let c = [body.iter().map(|e| e.x * e.weight).sum::<f32>() / w, body.iter().map(|e| e.z * e.weight).sum::<f32>() / w];
        let spread = body.iter().map(|e| ((e.x - c[0]).powi(2) + (e.z - c[1]).powi(2)).sqrt()).fold(0.0, f32::max).min(120.0);
        let (first, last) = body.iter().filter(|e| e.weight >= 0.5).fold((a.t, a.t), |(f, l), e| (f.min(e.t), l.max(e.t)));
        candidates.push((score, first, last, c, spread));
    }
    candidates.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut chosen: Vec<(f32, f32, f32, [f32; 2], f32)> = Vec::new();
    for c in candidates {
        let same = chosen.iter().any(|o| (o.1 - c.1).abs() < SAME_SCENE.0 && ((o.3[0] - c.3[0]).powi(2) + (o.3[1] - c.3[1]).powi(2)).sqrt() < SAME_SCENE.1);
        if !same {
            chosen.push(c);
        }
        if chosen.len() == MOMENTS {
            break;
        }
    }
    let mut out: Vec<Moment> = chosen.iter().enumerate().map(|(i, &(score, first, last, c, spread))| frame(i, score, first, last, c, spread)).collect();
    out.extend(goggles(&ev));
    Ok(out)
}

/// The drone's shot of a moment: about 130 m off and 240 m up, the gimbal some 60 degrees down, from one of eight
/// sides in turn so the reel does not look from one side only, pushing in a little; the field of view holds the
/// moment's spread and a margin. Steep, because the tracks south of Mala Tokmachka run along the tree lines, and
/// at 46 degrees the second scout saw the crowns where the vehicles were.
fn frame(i: usize, score: f32, first: f32, last: f32, c: [f32; 2], spread: f32) -> Moment {
    let from = first - LEAD;
    let secs = (last + TAIL - from).clamp(SHOT.0, SHOT.1);
    let az = (i as f32 * 2.399 + 0.6) % std::f32::consts::TAU;
    let (dx, dz) = (az.sin() * 130.0, az.cos() * 130.0);
    let slant = (130.0f32.powi(2) + 240.0f32.powi(2)).sqrt();
    let fov = (2.0 * ((spread + 35.0) / slant).atan()).to_degrees().clamp(7.0, 18.0);
    let line = format!(
        "shot {from:.2} {secs:.2} {:.0} {:.0} 250 {:.0} {:.0} 240 {:.0} {:.0} {:.1} {:.1}",
        c[0] + dx * 1.08,
        c[1] + dz * 1.08,
        c[0] + dx,
        c[1] + dz,
        c[0],
        c[1],
        fov * 1.1,
        fov
    );
    Moment { from, secs, score, line }
}

/// The FPV's last seconds through its goggles, to the hit and the static after it.
fn goggles(ev: &[Event]) -> Option<Moment> {
    let hit = ev.iter().find(|e| e.weight >= 9.0)?;
    let from = hit.t - 10.0;
    Some(Moment { from, secs: 11.5, score: 100.0, line: format!("goggles {from:.2} 11.5") })
}

/// Share the moments among runs so that none of a run's shots overlap (with a second between them, for the
/// filming step to settle): first fit, in time order. Returns each run's moments.
pub fn runs(mut moments: Vec<Moment>) -> Vec<Vec<Moment>> {
    moments.sort_by(|a, b| a.from.total_cmp(&b.from));
    let mut runs: Vec<Vec<Moment>> = Vec::new();
    for m in moments {
        match runs.iter_mut().find(|r| r.last().is_none_or(|l| l.from + l.secs + 1.0 <= m.from)) {
            Some(r) => r.push(m),
            None => runs.push(vec![m]),
        }
    }
    runs
}

/// The order the reel shows a day's moments in: the second best to open, then from the least up, the best last.
pub fn order(mut moments: Vec<Moment>) -> Vec<Moment> {
    moments.sort_by(|a, b| a.score.total_cmp(&b.score));
    if moments.len() > 2 {
        let second = moments.remove(moments.len() - 2);
        moments.insert(0, second);
    }
    moments
}
