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

enum Act {
    /// A column of the attacking side down a route, single file; `start` is where its tail spawns.
    Column { route: usize, kinds: &'static [VKind], plow: bool, smoke: usize, start: f32, label: &'static str, rescue: bool },
    /// The attacker's artillery preparation on the defenders' line: so many missions over so many seconds.
    Prep { missions: u32, over: f32 },
    /// A pair of Ka-52s arrives on station at these points, beyond reach.
    Helicopters(&'static [(f32, f32)]),
    /// An order to the column, if the player has not already given it.
    Order(ColumnOrder),
    Infantry { route: usize, groups: u32, size: u32, every: f32, label: &'static str },
    Tos { label: &'static str },
    Say(&'static str),
    Jam(f32),
}

struct Beat {
    at: f32,
    act: Act,
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

use VKind::*;

impl Script {
    fn new() -> Self {
        let beats = match diorama() {
            Diorama::Vuhledar => Self::vuhledar(),
            Diorama::Robotyne => Self::robotyne(),
        };
        Self { beats, t: 0.0, wave: 0, banner: None, infantry: Vec::new(), next_barrage: 40.0 }
    }

    fn robotyne() -> Vec<Beat> {
        let b = |at: f32, act: Act| Beat { at, act, done: false };
        const GROUP_1: &[VKind] = &[Leo2r, Bmr2, Leo2a6, Bradley, Bradley, Bradley, Bradley, Bradley, Bradley];
        const GROUP_2: &[VKind] = &[Leo2r, Wisent, Leo2a6, Bradley, Bradley, Bradley, Bradley, Leo2a6, Bradley, Bradley, Bradley];
        vec![
            b(1.0, Act::Say("Bird up. Still dark. Our guns are working the crest.")),
            b(3.0, Act::Prep { missions: 14, over: 70.0 }),
            b(8.0, Act::Say("All, this is Hranit. The column is late. The guns will be finished before it gets there.")),
            b(85.0, Act::Say("This is Sokil. Guns are done. Nothing on the track yet.")),
            b(110.0, Act::Column { route: 0, kinds: GROUP_1, plow: true, smoke: 0, start: 350.0, label: "04:23  The first group moves off behind the plows. It is getting light.", rescue: false }),
            b(125.0, Act::Say("This is Sokil. First group on the western track. Nine vehicles. The Leopard with the plow in front, the rollers behind it.")),
            b(150.0, Act::Jam(0.4)),
            b(330.0, Act::Column { route: 1, kinds: GROUP_2, plow: true, smoke: 0, start: 280.0, label: "05:29  The second group, eleven vehicles, on the eastern track", rescue: false }),
            b(345.0, Act::Say("This is Sokil. Second group moving. Same plan, the other track.")),
            b(430.0, Act::Helicopters(&[(2900.0, 3000.0), (3500.0, 2650.0)])),
            b(436.0, Act::Say("Ka-52s! Two of them, south-east, low over the ridge. Too far for anything we have.")),
            b(640.0, Act::Column { route: 0, kinds: &[Bradley, Bradley, Bradley, Bradley], plow: false, smoke: 0, start: 900.0, label: "07:02  Four Bradleys go back in for the men", rescue: true }),
            b(652.0, Act::Say("All, this is Hranit. Four Bradleys going back in. Get the infantry onto them.")),
            b(770.0, Act::Helicopters(&[(-3100.0, 2900.0), (-2500.0, 3300.0)])),
            b(776.0, Act::Say("This is Sokil. Helicopters again. South-west this time.")),
            b(1000.0, Act::Order(ColumnOrder::PullBack)),
            b(1300.0, Act::Say("This is Sokil. Quiet on the lanes. Only the smoke.")),
        ]
    }

    fn vuhledar() -> Vec<Beat> {
        let b = |at: f32, act: Act| Beat { at, act, done: false };
        {
            vec![
                b(1.0, Act::Say("Bird up. Visibility good, frost. Watch the dacha road and the Pavlivka road.")),
                b(6.0, Act::Say("All, this is Hranit. They're coming again today. Leave the roads clear. Let them in.")),
                b(14.0, Act::Column { route: 0, kinds: &[T80, T80, Bmp2, Bmp2, T80, Bmp2, Bmp1, Mtlb, Bmp2, T80, Bmp1, Bmp2], plow: true, start: 1150.0, smoke: 3, label: "06:45  The 155th Naval Infantry attacks down the dacha road", rescue: false }),
                b(20.0, Act::Jam(0.5)),
                b(22.0, Act::Say("Bird: column on the dacha road. Twelve units, tanks leading, a plow in front. Smoke.")),
                b(150.0, Act::Tos { label: "TOS-1A on the town" }),
                b(330.0, Act::Column { route: 0, kinds: &[T80, Bmp2, Bmp2, T72, Bmp1, Bmp2, Mtlb], plow: false, start: 500.0, smoke: 1, label: "08:40  A second column. The same road.", rescue: false }),
                b(345.0, Act::Say("This is Sokil. Second column. Same route. Exactly the same route.")),
                b(600.0, Act::Column { route: 1, kinds: &[T72, Bmp1, Bmp1, Bmp2, Mtlb, Bmp1, T72, Bmp1], plow: true, start: 650.0, smoke: 2, label: "10:20  The Alga battalion, from Pavlivka", rescue: false }),
                b(610.0, Act::Jam(0.6)),
                b(615.0, Act::Say("Bird: Pavlivka road now. Volunteers by the look of it. BMP-1s.")),
                b(700.0, Act::Tos { label: "TOS-1A again" }),
                b(900.0, Act::Column { route: 0, kinds: &[T80, Bmp2, Bmp2, Bmp2, T80, Bmp1], plow: false, start: 550.0, smoke: 0, label: "12:00  Again, the dacha road", rescue: false }),
                b(915.0, Act::Say("This is Sokil. They're driving past their own wrecks from this morning. Same line.")),
                b(1110.0, Act::Infantry { route: 1, groups: 6, size: 5, every: 14.0, label: "13:20  No more armour. Infantry, on foot." }),
                b(1125.0, Act::Infantry { route: 0, groups: 6, size: 5, every: 14.0, label: "" }),
                b(1140.0, Act::Say("This is Sokil. Groups of five. Walking across the open field. Towards the tree line.")),
                b(1230.0, Act::Tos { label: "TOS-1A" }),
                b(1300.0, Act::Infantry { route: 1, groups: 4, size: 5, every: 12.0, label: "14:30  More men on foot" }),
            ]
        }
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
        match &beat.act {
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
                    for p in at.iter() {
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
            Act::Say(s) => radio.say(now, *s),
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
