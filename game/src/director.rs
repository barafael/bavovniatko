//! The day as it happened: the script that runs whether or not the player does anything.
//!
//! 6 February 2023, the largest mechanised assault (dossier §2, §6 set piece 3): about two battalions in T-80s and
//! BMPs behind exhaust smoke and a mine plow, then more columns down the same roads, then the volunteer
//! battalions, then men on foot. Real minutes are compressed: the day runs from 07:40 to dusk in about 25.

use crate::fire::{FireControl, Mission, ShellKind};
use crate::geo;
use crate::units::{self, Models, Radio, Side, Stats, VKind, VehicleSpec};
use crate::world::Palette;
use crate::{Phase, audio};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct DirectorPlugin;

impl Plugin for DirectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Script::new())
            .insert_resource(DirRng(SmallRng::seed_from_u64(6)))
            .add_systems(Update, (run_script, russian_artillery, check_end).chain().run_if(crate::running).in_set(crate::Step::Script))
            .add_systems(Update, time_controls.in_set(crate::Step::Control).after(crate::dev::DevSystems));
    }
}

#[derive(Resource)]
struct DirRng(SmallRng);

pub const DAY_START_MIN: f32 = 7.0 * 60.0 + 40.0;
/// Game minutes per real second.
pub const COMPRESSION: f32 = 0.36;
pub const DAY_LENGTH: f32 = 1500.0;

pub fn clock_text(t: f32) -> String {
    let m = DAY_START_MIN + t * COMPRESSION;
    format!("{:02}:{:02}", (m / 60.0) as u32, (m % 60.0) as u32)
}

enum Act {
    Column { route: usize, kinds: &'static [VKind], plow: bool, smoke: usize, start: f32, label: &'static str },
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
        let b = |at: f32, act: Act| Beat { at, act, done: false };
        Self {
            beats: vec![
                b(1.0, Act::Say("Bird up. Visibility good, frost. Watch the dacha road and the Pavlivka road.")),
                b(6.0, Act::Say("72nd: they're coming again today. Leave the roads clear. Let them in.")),
                b(14.0, Act::Column { route: 0, kinds: &[T80, T80, Bmp2, Bmp2, T80, Bmp2, Bmp1, Mtlb, Bmp2, T80, Bmp1, Bmp2], plow: true, start: 1150.0, smoke: 3, label: "07:45  The 155th Naval Infantry attacks down the dacha road" }),
                b(20.0, Act::Jam(0.5)),
                b(22.0, Act::Say("Bird: column on the dacha road. Twelve units, tanks leading, a plow in front. Smoke.")),
                b(150.0, Act::Tos { label: "TOS-1A on the town" }),
                b(330.0, Act::Column { route: 0, kinds: &[T80, Bmp2, Bmp2, T72, Bmp1, Bmp2, Mtlb], plow: false, start: 500.0, smoke: 1, label: "09:40  A second column. The same road." }),
                b(345.0, Act::Say("Second column. Same route. Exactly the same route.")),
                b(600.0, Act::Column { route: 1, kinds: &[T72, Bmp1, Bmp1, Bmp2, Mtlb, Bmp1, T72, Bmp1], plow: true, start: 650.0, smoke: 2, label: "11:20  The Alga battalion, from Pavlivka" }),
                b(610.0, Act::Jam(0.6)),
                b(615.0, Act::Say("Bird: Pavlivka road now. Volunteers by the look of it. BMP-1s.")),
                b(700.0, Act::Tos { label: "TOS-1A again" }),
                b(900.0, Act::Column { route: 0, kinds: &[T80, Bmp2, Bmp2, Bmp2, T80, Bmp1], plow: false, start: 550.0, smoke: 0, label: "13:00  Again, the dacha road" }),
                b(915.0, Act::Say("They're driving past their own wrecks from this morning. Same line.")),
                b(1110.0, Act::Infantry { route: 1, groups: 6, size: 5, every: 14.0, label: "14:20  No more armour. Infantry, on foot." }),
                b(1125.0, Act::Infantry { route: 0, groups: 6, size: 5, every: 14.0, label: "" }),
                b(1140.0, Act::Say("Groups of five. Walking across the open field. Towards the tree line.")),
                b(1230.0, Act::Tos { label: "TOS-1A" }),
                b(1300.0, Act::Infantry { route: 1, groups: 4, size: 5, every: 12.0, label: "15:30  More men on foot" }),
            ],
            t: 0.0,
            wave: 0,
            banner: None,
            infantry: Vec::new(),
            next_barrage: 40.0,
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
    mut view: ResMut<crate::camera::View>,
    mut sound: audio::Sound,
    op: Option<Res<crate::world::ObservationPost>>,
    models: Res<Models>,
    palette: Res<Palette>,
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
            Act::Column { route, kinds, plow, smoke, start, label } => {
                *wave += 1;
                let n = kinds.len();
                for (i, k) in kinds.iter().enumerate() {
                    // Single file, about 42 m apart, the lead furthest up the road.
                    let s = start + (n - 1 - i) as f32 * 42.0 + 10.0;
                    units::spawn_vehicle(&mut commands, &models, &palette, VehicleSpec { kind: *k, route: *route, s, lat: 0.0, wave: *wave, idx: i as u32, plow: *plow && i == 0, smoke: i < *smoke });
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
                let aim = op.as_ref().map(|o| o.building).unwrap_or(Vec2::new(-300.0, -1225.0)) + Vec2::new(rng.random_range(-250.0..250.0), rng.random_range(30.0..110.0));
                fc.missions.push(Mission { target: aim, kind: ShellKind::Tos, rounds: 24, sigma: 75.0, interval: 0.3, next: now + 3.0, flight: 12.0, player: false, by: Side::Ru });
                radio.say(now, "TOS! Solntsepyok on the town! Get down!");
                *banner = Some((label.to_string(), 4.0));
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
            units::spawn_soldier(&mut commands, &models, &palette, Side::Ru, at, goal, r.line.point(0.0), rng);
        }
        stats.sent += size;
    }
}

/// Russian artillery works the tree lines and the town all day: 152 mm barrages every minute or so.
fn russian_artillery(time: Res<Time>, mut script: ResMut<Script>, mut fc: ResMut<FireControl>, mut rng: ResMut<DirRng>, positions: Query<&crate::fire::UaPosition>) {
    let now = time.elapsed_secs_f64();
    if script.t < script.next_barrage {
        return;
    }
    let rng = &mut rng.0;
    script.next_barrage = script.t + rng.random_range(35.0..75.0);
    let live: Vec<Vec2> = positions.iter().filter(|p| p.strength > 0).map(|p| p.pos).collect();
    let target = if !live.is_empty() && rng.random_bool(0.6) {
        live[rng.random_range(0..live.len())] + Vec2::new(rng.random_range(-60.0..60.0), rng.random_range(-30.0..30.0))
    } else {
        Vec2::new(rng.random_range(-1800.0..1500.0), if rng.random_bool(0.5) { -660.0 } else { rng.random_range(-1400.0..-1150.0) })
    };
    fc.missions.push(Mission { target, kind: ShellKind::Ru152, rounds: rng.random_range(3..8), sigma: 45.0, interval: 2.0, next: now, flight: 20.0, player: false, by: Side::Ru });
}

fn check_end(script: Res<Script>, stats: Res<Stats>, vehicles: Query<&units::Vehicle>, soldiers: Query<&units::Soldier>, mut next: ResMut<NextState<Phase>>) {
    let quiet = script.t > 1350.0 && vehicles.iter().all(|v| !v.state.mobile()) && soldiers.iter().filter(|s| s.side == Side::Ru && s.state != units::SState::DugIn).count() == 0;
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
