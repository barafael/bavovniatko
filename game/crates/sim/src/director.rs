//! The day as it happened: the script that runs whether or not the player does anything.
//!
//! Vuhledar, 6 February 2023, the largest mechanised assault (dossier c08 §2, §6 set piece 3): about two
//! battalions in T-80s and BMPs behind exhaust smoke and a mine plow, then more columns down the same roads, then
//! the volunteer battalions, then men on foot. Real minutes are compressed: the day runs from 06:40 (Kyiv time) to dusk in
//! about 25.
//!
//! Mala Tokmachka, 8 June 2023 (c09 §2, §6 set piece 1, §11 on the date): the guns finish their preparation in
//! the dark; the column is late and moves off at first light; nine vehicles behind a Leopard 2R and a BMR-2 on
//! the western track, eleven on the eastern; Ka-52s from beyond reach; four Bradleys back in for the men; and in
//! the end, everyone back to the start line. 03:50 to late morning in about 25 minutes.

use crate::diorama::{Diorama, diorama};
use crate::fire::{FireControl, Mission, ShellKind};
use crate::geo;
use crate::units::{self, ColumnOrder, Models, Radio, SState, Side, Stats, VKind, VState, VehicleSpec};
use crate::units::Palette;
use crate::{Phase, sound as audio};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct DirectorPlugin;

impl Plugin for DirectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Script::new())
            .insert_resource(DirRng(SmallRng::seed_from_u64(crate::dev::seeded(6))))
            .add_systems(Update, (run_script, russian_artillery, check_end).chain().run_if(crate::running).in_set(crate::Step::Script))
            .add_systems(Update, time_controls.in_set(crate::Step::Control).after(crate::dev::DevSystems));
    }
}

#[derive(Resource)]
struct DirRng(SmallRng);

pub const DAY_LENGTH: f32 = 1500.0;

/// Clock time in minutes after midnight at game time `t`.
pub fn clock_minutes(t: f32) -> f32 {
    diorama().day_start_min() + t * diorama().compression()
}

pub fn clock_text(t: f32) -> String {
    let m = clock_minutes(t);
    format!("{:02}:{:02}", (m / 60.0) as u32, (m % 60.0) as u32)
}

/// One thing the day's script does (written in the diorama's file).
#[derive(serde::Deserialize, Debug)]
pub enum Act {
    /// A column of the attacking side down a route, single file; `start` is where its tail spawns.
    Column { route: usize, kinds: Vec<VKind>, plow: bool, smoke: usize, start: f32, label: String, #[serde(default)] rescue: bool },
    /// The attacker's artillery preparation on the defenders' line: so many missions over so many seconds.
    Prep { missions: u32, over: f32 },
    /// A pair of Ka-52s arrives on station at these points, beyond reach.
    Helicopters(Vec<(f32, f32)>),
    /// An order to the column, if the player has not already given it.
    Order(ColumnOrder),
    Infantry { route: usize, groups: u32, size: u32, every: f32, #[serde(default)] label: String },
    Tos { label: String },
    Say(String),
    Jam(f32),
}

struct Beat {
    at: f32,
    act: &'static Act,
    done: bool,
}

#[derive(Resource)]
pub struct Script {
    beats: Vec<Beat>,
    pub t: f32,
    pub wave: u32,
    pub banner: Option<(String, f32)>,
    /// Infantry groups waiting for their time: (game time, route, men).
    infantry: Vec<(f32, usize, u32)>,
    next_barrage: f32,
}

impl Script {
    fn new() -> Self {
        let beats = crate::data::sim_data().script.iter().map(|(at, act)| Beat { at: *at, act, done: false }).collect();
        Self { beats, t: 0.0, wave: 0, banner: None, infantry: Vec::new(), next_barrage: 40.0 }
    }
}

fn run_script(
    mut commands: Commands,
    time: Res<Time>,
    mut script: ResMut<Script>,
    mut rng: ResMut<DirRng>,
    mut radio: ResMut<Radio>,
    mut stats: ResMut<Stats>,
    mut fc: ResMut<FireControl>,
    mut view: ResMut<crate::player::View>,
    mut sound: audio::Sound,
    op: Res<crate::player::ObservationPost>,
    models: Res<Models>,
    palette: Res<Palette>,
    (mut order, helis): (ResMut<ColumnOrder>, Option<Res<crate::robotyne::HeliModels>>),
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs_f64();
    script.t += dt;
    let t = script.t;
    if let Some((_, life)) = script.banner.as_mut() {
        *life -= dt;
        if *life <= 0.0 {
            script.banner = None;
        }
    }
    let rng = &mut rng.0;
    let lay = geo::layout();
    let Script { beats, wave, banner, infantry, .. } = &mut *script;
    for beat in beats.iter_mut() {
        if beat.done || t < beat.at {
            continue;
        }
        beat.done = true;
        match beat.act {
            Act::Column { route, kinds, plow, smoke, start, label, rescue } => {
                *wave += 1;
                let n = kinds.len();
                // Single file, the lead furthest up the road, 30–60 m between vehicles: no two drivers keep the
                // same distance.
                let mut at = *start + 10.0;
                let mut places = vec![0.0f32; n];
                for i in (0..n).rev() {
                    places[i] = at;
                    at += 30.0 + geo::hash01(*wave * 100 + i as u32, 21) * 30.0;
                }
                for (i, k) in kinds.iter().enumerate() {
                    let s = places[i];
                    // The plow leads; a roller vehicle or a second plow behind it keeps its own.
                    let plow = *plow && (i == 0 || k.breacher());
                    let side = diorama().attacker();
                    units::spawn_vehicle(&mut commands, &models, &palette, VehicleSpec { side, kind: *k, route: *route, s, lat: 0.0, wave: *wave, idx: i as u32, plow, smoke: i < *smoke, rescue: *rescue });
                    stats.vehicles_committed += 1;
                    stats.wave(*wave).vehicles += 1;
                    stats.sent += k.men();
                }
                *banner = Some((label.to_string(), 7.0));
                sound.now(audio::Sfx::Radio, 0.3);
            }
            Act::Infantry { route, groups, size, every, label } => {
                infantry.extend((0..*groups).map(|g| (t + g as f32 * every, *route, *size)));
                if !label.is_empty() {
                    *banner = Some((label.to_string(), 7.0));
                }
            }
            Act::Tos { label } => {
                // Twenty-four rockets rippling onto the southern blocks: the observers' building is the target.
                let aim = op.building + Vec2::new(rng.random_range(-250.0..250.0), rng.random_range(30.0..110.0));
                // Twenty-four rockets in pairs over about ten seconds, scattered over some 200 × 400 m with the long
                // axis along the line of fire (from the south).
                fc.missions.push(Mission { target: aim, kind: ShellKind::Tos, rounds: 24, sigma: 100.0, interval: 0.4, next: now + 3.0, flight: 12.0, player: false, by: Side::Ru });
                radio.say(now, "TOS! Solntsepyok on the town! Get down!");
                *banner = Some((label.to_string(), 4.0));
            }
            Act::Prep { missions, over } => {
                // The guns walk up and down the crest trench line and the positions behind it.
                let lay_posts = &lay.ru_positions;
                for k in 0..*missions {
                    let base = if k % 2 == 0 && !lay_posts.is_empty() { lay_posts[k as usize % lay_posts.len()] } else { Vec2::new(rng.random_range(-2000.0..2000.0), lay.trench_z.unwrap_or(lay.defence_z) + rng.random_range(-10.0..60.0)) };
                    let at = now + (k as f32 / *missions as f32 * over) as f64;
                    fc.missions.push(Mission { target: base, kind: ShellKind::He155, rounds: 6, sigma: 45.0, interval: 1.6, next: at, flight: 18.0, player: false, by: Side::Ua });
                }
            }
            Act::Helicopters(at) => {
                if let Some(h) = helis.as_ref() {
                    for p in at {
                        crate::robotyne::spawn_helicopter(&mut commands, h, Vec2::new(p.0, p.1));
                    }
                }
            }
            Act::Order(o) => {
                if *order == ColumnOrder::Push {
                    *order = *o;
                    radio.say(now, "All, this is Hranit. That's enough. All callsigns back to the start line. Bring the infantry.");
                    *banner = Some(("08:50  Everyone back to the start line".to_string(), 7.0));
                }
            }
            Act::Say(s) => radio.say(now, s.as_str()),
            Act::Jam(j) => {
                view.jamming = (view.jamming + j).min(1.0);
                radio.say(now, "Bird: interference. Their EW is up.");
            }
        }
    }
    // Release infantry groups as their time comes: they walk from the start line straight across the field.
    let (due, waiting) = std::mem::take(infantry).into_iter().partition::<Vec<_>, _>(|g| t >= g.0);
    *infantry = waiting;
    for (_, route, size) in due {
        let r = &lay.routes[route];
        // They come out of the middle tree line, about a kilometre from the Ukrainian positions.
        let s0 = (r.deploy_s - rng.random_range(450.0..650.0)).max(0.0);
        let start = r.line.point(s0) + r.line.normal(s0) * rng.random_range(-250.0..250.0);
        for k in 0..size {
            let at = start + Vec2::new(k as f32 * 4.0, rng.random_range(-3.0..3.0));
            let goal = r.objective + Vec2::new(rng.random_range(-150.0..150.0), rng.random_range(-10.0..10.0));
            units::spawn_soldier(&mut commands, &models, &palette, Side::Ru, at, goal, r.line.point(0.0), 0.0, None, rng);
        }
        stats.sent += size;
    }
}

/// Russian artillery works the tree lines and the town all day: 152 mm barrages every minute or so. South of Mala
/// Tokmachka it hunts the column instead, cued by its drones: whatever has stopped and is not under smoke, and
/// now and then a single Krasnopol, guided, onto a vehicle that cannot move ("effect over volume", RUSI).
fn russian_artillery(
    time: Res<Time>,
    mut script: ResMut<Script>,
    mut fc: ResMut<FireControl>,
    mut rng: ResMut<DirRng>,
    smoke: Res<crate::fx::Smoke>,
    positions: Query<&crate::fire::Post>,
    vehicles: Query<(&units::Vehicle, &Transform)>,
    soldiers: Query<(&units::Soldier, &Transform)>,
) {
    let now = time.elapsed_secs_f64();
    if script.t < script.next_barrage {
        return;
    }
    let rng = &mut rng.0;
    if diorama() == Diorama::Robotyne {
        script.next_barrage = script.t + rng.random_range(35.0..70.0);
        // Their drones watch the slope; their guns wait until the column is in the minefields' reach.
        let seen = |p: Vec2| !smoke.covers(p, now) && p.y > -650.0;
        let stopped: Vec<Vec2> = vehicles
            .iter()
            .filter(|(v, t)| v.side == Side::Ua && v.state.alive() && (!v.state.mobile() || v.state == VState::Halted) && seen(t.translation.xz()))
            .map(|(_, t)| t.translation.xz())
            .collect();
        let men: Vec<Vec2> = soldiers.iter().filter(|(s, t)| s.side == Side::Ua && s.state != SState::DugIn && seen(t.translation.xz())).map(|(_, t)| t.translation.xz()).collect();
        if !stopped.is_empty() && rng.random_bool(0.12) {
            let target = stopped[rng.random_range(0..stopped.len())];
            fc.missions.push(Mission { target, kind: ShellKind::Ru152, rounds: 1, sigma: 3.0, interval: 1.0, next: now, flight: 20.0, player: false, by: Side::Ru });
            return;
        }
        let target = if !stopped.is_empty() && rng.random_bool(0.45) {
            stopped[rng.random_range(0..stopped.len())] + Vec2::new(rng.random_range(-45.0..45.0), rng.random_range(-45.0..45.0))
        } else if men.len() >= 4 && rng.random_bool(0.2) {
            men[rng.random_range(0..men.len())]
        } else {
            // The start line and the forward tree line.
            let posts: Vec<Vec2> = positions.iter().filter(|p| p.side == Side::Ua && p.strength > 0).map(|p| p.pos).collect();
            if !posts.is_empty() && rng.random_bool(0.3) { posts[rng.random_range(0..posts.len())] + Vec2::new(rng.random_range(-60.0..60.0), rng.random_range(-30.0..30.0)) } else { Vec2::new(rng.random_range(-1800.0..1800.0), rng.random_range(-1500.0..-850.0)) }
        };
        fc.missions.push(Mission { target, kind: ShellKind::Ru152, rounds: rng.random_range(3..8), sigma: 40.0, interval: 2.0, next: now, flight: 20.0, player: false, by: Side::Ru });
        return;
    }
    script.next_barrage = script.t + rng.random_range(35.0..75.0);
    let live: Vec<Vec2> = positions.iter().filter(|p| p.strength > 0).map(|p| p.pos).collect();
    let target = if !live.is_empty() && rng.random_bool(0.6) {
        live[rng.random_range(0..live.len())] + Vec2::new(rng.random_range(-60.0..60.0), rng.random_range(-30.0..30.0))
    } else {
        Vec2::new(rng.random_range(-1800.0..1500.0), if rng.random_bool(0.5) { geo::layout().defence_z } else { rng.random_range(-1400.0..-1150.0) })
    };
    fc.missions.push(Mission { target, kind: ShellKind::Ru152, rounds: rng.random_range(3..8), sigma: 45.0, interval: 2.0, next: now, flight: 20.0, player: false, by: Side::Ru });
}

fn check_end(script: Res<Script>, stats: Res<Stats>, vehicles: Query<&units::Vehicle>, soldiers: Query<&units::Soldier>, mut next: ResMut<NextState<Phase>>) {
    let attacker = diorama().attacker();
    // While the showcase films, the quiet exit waits: the tour's own debrief shot ends the day.
    let quiet = crate::dev::dev().showcase.is_none()
        && script.t > 1350.0
        && vehicles.iter().filter(|v| v.side == attacker).all(|v| !v.state.mobile())
        && soldiers.iter().filter(|s| s.side == attacker && s.state != units::SState::DugIn).count() == 0;
    if script.t > DAY_LENGTH || quiet || stats.foothold > 25 {
        next.set(Phase::Debrief);
    }
}

/// P pauses; [ and ] slow down and speed up time.
fn time_controls(keys: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if keys.just_pressed(KeyCode::KeyP) {
        if time.is_paused() { time.unpause() } else { time.pause() }
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        let s = (time.relative_speed() * 2.0).min(8.0);
        time.set_relative_speed(s);
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        let s = (time.relative_speed() * 0.5).max(0.25);
        time.set_relative_speed(s);
    }
}
