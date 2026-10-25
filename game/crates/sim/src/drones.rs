//! The drone team, beyond the reconnaissance drone itself: where it flies from and the enemy finding it, and the
//! strike drone (an FPV) where the diorama gives one (game/design/roadmap.md step 7; db/research/10 and 11).
//!
//! - **The team.** While a drone is up, its controller can be found (in early 2023 DJI's AeroScope gave the pilot's
//!   position away, and fire followed within minutes). A ranging round comes in first; M moves the team, which
//!   takes the feed down while it moves. A team that stays is hit by the mission that follows, and loses a drone.
//!   South of Mala Tokmachka the team is hard to find.
//! - **The FPV.** 4 launches it from the team's position and puts the player in its goggles; the camera flies it.
//!   It goes in where it meets a hull or the ground, and the goggles go to static.
//!
//! - **The drop.** Where the day gives the reconnaissance drone grenades (`SupportKind::Drops`: VOG-17s on a release
//!   under the Mavic, db/research/10-drones-ua-2023.md §5), C lets one go from where the drone is. It falls in the
//!   day's wind, which blows it off by metres from the drone's height and less from low down, and bursts where it
//!   meets the ground: a small blast on men, nothing to a hull.
//!
//! `BAV_FPV="<s>:<x>;<z>"` launches one by itself and flies it at the vehicle nearest that point, and
//! `BAV_DROP="<s>:<x>;<z>,…"` drops on the enemy's man nearest each point (tests and bands).

use crate::diorama::{Side, diorama};
use crate::fire::{Mission, ShellKind};
use crate::fx::FxQueue;
use crate::geo;
use crate::player::{Fpv, Operator, View};
use crate::sound::{Sfx, Sound};
use crate::units::{Blast, Cause, DirectHit, Models, Palette, Radio, SState, Soldier, Stats, Vehicle};
use bav_core::vocab::Mode;
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct DronesPlugin;

impl Plugin for DronesPlugin {
    fn build(&self, app: &mut App) {
        let d = crate::data::sim_data();
        app.insert_resource(Fpv { left: crate::allot::settled().map_or(0, |a| a.fpvs), ..default() })
            .insert_resource(Operator { pos: Vec2::new(d.operator.pos.0, d.operator.pos.1), found: None, moving_until: 0.0, moves: 0 })
            .insert_resource(OperatorRng(SmallRng::seed_from_u64(crate::dev::seeded(1306))))
            .add_systems(Update, drone_input.in_set(crate::Step::Control).after(crate::dev::DevSystems))
            .add_systems(Update, (team_found, fpv_strikes, crate::ew::ew_update).chain().run_if(crate::running).in_set(crate::Step::Script).after(crate::evac::EvacSystems));
        // The drop, only where the day has grenades for it: the Record of the others must not change.
        if d.support.iter().any(|s| matches!(s.what, crate::allot::SupportKind::Drops(_))) {
            app.insert_resource(Drops { left: crate::allot::settled().map_or(0, |a| a.drops), dropped: 0 })
                .add_systems(Update, drop_input.run_if(crate::running).in_set(crate::Step::Control).after(crate::dev::DevSystems))
                .add_systems(Update, fall.run_if(crate::running).in_set(crate::Step::Fire));
        }
    }
}

/// The team's own stream: whether they are found never shifts the rest of the day's draws.
#[derive(Resource)]
struct OperatorRng(SmallRng);

/// The FPV's warhead: a PG-7 class shaped charge, and fragments.
const FPV_BLAST: f32 = 6.0;

/// Put the player in an FPV's goggles over the team's position, facing the enemy.
pub fn launch_fpv(fpv: &mut Fpv, view: &mut View, op: &Operator, radio: &mut Radio, now: f64) -> bool {
    let Some(spec) = crate::data::sim_data().fpv else {
        return false;
    };
    if fpv.left == 0 || fpv.flying || now < op.moving_until {
        return false;
    }
    fpv.left -= 1;
    fpv.flying = true;
    fpv.gone = 0.0;
    fpv.pos = geo::ground(op.pos) + Vec3::Y * 1.5;
    fpv.vel = Vec3::ZERO;
    // Our side faces the other: south where the attack goes south, north where it comes north.
    let lay = geo::layout();
    let south = if diorama().attacker() == Side::Ua { lay.advance > 0.0 } else { lay.advance < 0.0 };
    fpv.yaw = if south { std::f32::consts::PI } else { 0.0 };
    fpv.aim_yaw = fpv.yaw;
    fpv.pitch = 0.0;
    fpv.aim_pitch = -0.15;
    fpv.roll = 0.0;
    fpv.throttle = 0.6;
    fpv.battery = spec.endurance;
    fpv.signal = 1.0;
    fpv.control = true;
    view.mode = Mode::Fpv;
    radio.say2(now, "FPV у повітрі. Відео є.", "FPV up. Video's good.");
    true
}

#[allow(clippy::too_many_arguments)]
fn drone_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    mut fpv: ResMut<Fpv>,
    mut view: ResMut<View>,
    mut op: ResMut<Operator>,
    mut radio: ResMut<Radio>,
    mut auto_launched: Local<bool>,
) {
    let now = time.elapsed_secs_f64();
    let auto = crate::dev::dev().fpv.is_some_and(|(t, _)| script.t >= t) && !*auto_launched;
    if keys.just_pressed(KeyCode::Digit4) || auto {
        *auto_launched |= auto;
        if !launch_fpv(&mut fpv, &mut view, &op, &mut radio, now) {
            let why = if crate::data::sim_data().fpv.is_none() {
                ("Ударних дронів сьогодні нема.", "No strike drones today.")
            } else if fpv.left == 0 {
                ("FPV більше нема.", "No more FPVs.")
            } else {
                ("Не зараз: група на переміщенні.", "Not now: the team is moving.")
            };
            radio.say2(now, why.0, why.1);
        }
    }
    if keys.just_pressed(KeyCode::KeyM) && now >= op.moving_until {
        move_team(&mut op, &mut view, &mut radio, now, false);
    }
}

/// Pack up and move: the feed is down until the team is set up again somewhere else.
fn move_team(op: &mut Operator, view: &mut View, radio: &mut Radio, now: f64, hit: bool) {
    // About fifteen minutes of the day's clock: the antennas down, a few hundred metres along the tree line, up again.
    let secs = 15.0 / crate::diorama::diorama().compression();
    op.moving_until = now + secs as f64;
    op.found = None;
    op.moves += 1;
    let side = if op.moves % 2 == 1 { 1.0 } else { -1.0 };
    op.pos.x = (op.pos.x + side * 260.0).clamp(-geo::HALF_X + 200.0, geo::HALF_X - 200.0);
    view.feed_down = view.feed_down.max(secs);
    info!("operator: moving to {:.0},{:.0} for {secs:.0} s, at {now:.0} s", op.pos.x, op.pos.y);
    if !hit {
        radio.say2(now, "{player} переміщується. Пташка сідає, підніму з нової точки.", "{player} moving. Bringing the bird down; up again from a new spot.");
    }
}

/// While a drone is up the enemy may find its controller: a ranging round first, then the mission.
#[allow(clippy::too_many_arguments)]
fn team_found(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<OperatorRng>,
    mut op: ResMut<Operator>,
    mut view: ResMut<View>,
    mut radio: ResMut<Radio>,
    fpv: Res<Fpv>,
) {
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    if crate::dev::dev().operator_off || now < op.moving_until {
        return;
    }
    match op.found {
        None => {
            let up = view.feed_down <= 0.0 || fpv.flying;
            let per_hour = crate::data::sim_data().operator.found_per_hour;
            // Hours of the day's clock per game second.
            let p = per_hour * crate::diorama::diorama().compression() / 60.0 * dt;
            if up && p > 0.0 && rng.0.random_bool(p.min(1.0) as f64) {
                op.found = Some(now);
                info!("operator: found at {:.0},{:.0}, at {now:.0} s", op.pos.x, op.pos.y);
                let near = op.pos + Vec2::new(rng.0.random_range(-60.0..60.0), rng.0.random_range(-60.0..60.0));
                commands.spawn(Mission { target: near, kind: ShellKind::Ru152, rounds: 1, sigma: 10.0, interval: 1.0, next: now, flight: 15.0, by: Side::Ru });
                radio.say2(now, "Приліт поруч з нами! Нас вирахували. {player}, треба міняти точку (M).", "Incoming next to us! They've found us. {player}, we have to move (M).");
            }
        }
        Some(t) => {
            // Twenty seconds after the ranging round, the mission; it lands on whoever is still there.
            if now >= t + 20.0 && now - (dt as f64) < t + 20.0 {
                commands.spawn(Mission { target: op.pos, kind: ShellKind::Ru152, rounds: 6, sigma: 25.0, interval: 2.0, next: now, flight: 15.0, by: Side::Ru });
            }
            if now >= t + 38.0 {
                info!("operator: hit at {:.0},{:.0}, at {now:.0} s", op.pos.x, op.pos.y);
                view.drones_left = view.drones_left.saturating_sub(1);
                let wait = if view.drones_left == 0 { 240.0 } else { 75.0 };
                view.feed_down = view.feed_down.max(wait);
                radio.say2(now, "Нас накрили. Двоє трьохсотих, пульт розбитий. Шукаємо новий.", "They've hit us. Two wounded, the controller's smashed. Looking for another.");
                let o = &mut *op;
                move_team(o, &mut view, &mut radio, now, true);
            }
        }
    }
}

/// Where the FPV meets a hull or the ground, it goes in.
#[allow(clippy::too_many_arguments)]
fn fpv_strikes(
    time: Res<Time>,
    mut fpv: ResMut<Fpv>,
    mut fx: ResMut<FxQueue>,
    mut sound: Sound,
    mut hits: MessageWriter<DirectHit>,
    mut blasts: MessageWriter<Blast>,
    mut stats: ResMut<Stats>,
    mut radio: ResMut<Radio>,
    vehicles: Query<(Entity, &Vehicle, &Transform)>,
) {
    if !fpv.flying {
        return;
    }
    let now = time.elapsed_secs_f64();
    let p = fpv.pos;
    let ground = geo::height(p.x, p.z);
    let hull = vehicles.iter().find(|(_, _, t)| {
        let c = t.translation + Vec3::Y * 1.3;
        let d = p - c;
        Vec2::new(d.x, d.z).length() < 3.2 && d.y.abs() < 1.8
    });
    if hull.is_none() && p.y > ground + 0.4 {
        return;
    }
    fpv.flying = false;
    fpv.gone = 0.0;
    fx.explosion(p, 0.7, hull.is_none());
    fx.flash(p, 6.0e6, 60.0, 0.12);
    sound.at(p, Sfx::BlastNear, 1.6);
    blasts.write(Blast { at: p, radius: FPV_BLAST, cause: Cause::Fpv });
    stats.fpv_strikes += 1;
    match hull {
        Some((e, v, t)) => {
            let top = p.y > t.translation.y + 1.6;
            hits.write(DirectHit { target: e, cause: Cause::Fpv, top, weapon: "FPV", from: "your drone team" });
            info!("fpv: in on {:?} {}/{} at {:.0},{:.0}, {}, at {now:.0} s", v.kind, v.wave, v.idx, t.translation.x, t.translation.z, if top { "from above" } else { "from the side" });
            radio.say2(now, "Є влучання. Відео пропало.", "Hit. Video's gone.");
        }
        None => {
            info!("fpv: into the ground at {:.0},{:.0}, at {now:.0} s", p.x, p.z);
            radio.say2(now, "Мимо. В землю.", "Missed. Into the ground.");
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// The drop

/// The reconnaissance drone's grenades left for the day (where it has any: `SupportKind::Drops`).
#[derive(Resource)]
pub struct Drops {
    pub left: u32,
    pub dropped: u32,
}

/// A grenade on its way down from the drone.
#[derive(Component)]
pub struct Falling {
    pub vel: Vec3,
}

/// The VOG-17's burst: a claimed six metres that often fails to put a man down (`Blast` is lethal to three times
/// its radius, surely only at the point; the CTC's "multiple drops are needed").
const VOG_BLAST: f32 = 2.0;
/// A finned VOG-17 (350 g) falls at about this at most.
const TERMINAL: f32 = 45.0;
/// The height a scripted drop goes from: the one figure the sources give for a Mavic (about 60 m).
const SCRIPTED_AGL: f32 = 60.0;

/// The wind the grenade falls through: the day's (at the drone's height), weaker near the ground, and gusting by a
/// third either way on a slow beat (`at`: the game time; none, the mean, which is all an operator can allow for).
fn wind_at(agl: f32, at: Option<f64>) -> Vec3 {
    let w = crate::fx::day_wind();
    let shear = (agl.max(2.0) / 100.0).powf(0.14);
    let gust = at.map_or(1.0, |t| 1.0 + 0.3 * ((t * 0.83).sin() * (t * 0.31 + 1.7).sin()) as f32);
    Vec3::new(w.x, 0.0, w.z) * shear * gust
}

/// One step of the fall: gravity, and the drag of the air going past it, which carries it off with the wind.
fn fall_step(pos: &mut Vec3, vel: &mut Vec3, dt: f32, at: Option<f64>) {
    let agl = pos.y - geo::height(pos.x, pos.z);
    let rel = *vel - wind_at(agl, at);
    *vel += (Vec3::NEG_Y * 9.81 - rel * rel.length() * (9.81 / (TERMINAL * TERMINAL))) * dt;
    *pos += *vel * dt;
}

/// Where a grenade let go from here, still, would land in the mean wind.
fn landing(from: Vec3) -> Vec2 {
    let (mut p, mut v) = (from, Vec3::ZERO);
    for _ in 0..2000 {
        fall_step(&mut p, &mut v, 0.02, None);
        if p.y <= geo::height(p.x, p.z) {
            break;
        }
    }
    p.xz()
}

/// Let one go from `from` with the drone's own way on it.
#[allow(clippy::too_many_arguments)]
fn release(commands: &mut Commands, models: &Models, palette: &Palette, drops: &mut Drops, radio: &mut Radio, from: Vec3, vel: Vec3, now: f64) {
    drops.left -= 1;
    drops.dropped += 1;
    commands.spawn((
        Mesh3d(models.missile.clone()),
        MeshMaterial3d(palette.vertex.clone()),
        Transform::from_translation(from).looking_to(Vec3::NEG_Y, Vec3::Z).with_scale(Vec3::new(0.6, 0.6, 0.2)),
        Falling { vel },
        bevy::light::NotShadowCaster,
    ));
    info!("drop: released over {:.0},{:.0} from {:.0} m, {} left, at {now:.0} s", from.x, from.z, from.y - geo::height(from.x, from.z), drops.left);
    radio.say2(now, "Скид.", "Drop.");
}

/// C lets a grenade go from the drone while its picture is up; `BAV_DROP` drops by itself.
#[allow(clippy::too_many_arguments)]
fn drop_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    view: Res<View>,
    mut drops: ResMut<Drops>,
    mut radio: ResMut<Radio>,
    models: Res<Models>,
    palette: Res<Palette>,
    soldiers: Query<(&Soldier, &Transform)>,
    mut done: Local<usize>,
) {
    let now = time.elapsed_secs_f64();
    if keys.just_pressed(KeyCode::KeyC) && view.mode == Mode::Drone {
        if drops.left == 0 {
            radio.say2(now, "ВОГів більше нема.", "No grenades left.");
        } else if view.feed_down > 0.0 || !view.control {
            radio.say2(now, "Не зараз: пташка не на зв'язку.", "Not now: the bird is off the link.");
        } else {
            release(&mut commands, &models, &palette, &mut drops, &mut radio, view.drone_pos - Vec3::Y * 0.4, view.drone_vel, now);
        }
    }
    // The scripted drops: on the enemy's man nearest the point (or the point), from 60 m up and upwind of him by the
    // drift of the mean wind.
    let plan = &crate::dev::dev().drop;
    while *done < plan.len() && script.t >= plan[*done].0 {
        let at = Vec2::new(plan[*done].1.0, plan[*done].1.1);
        *done += 1;
        if drops.left == 0 {
            continue;
        }
        let attacker = diorama().attacker();
        let aim = soldiers
            .iter()
            .filter(|(s, t)| s.side == attacker && s.state != SState::Wounded && t.translation.xz().distance(at) < 150.0)
            .map(|(_, t)| t.translation.xz())
            .min_by(|a, b| a.distance(at).total_cmp(&b.distance(at)))
            .unwrap_or(at);
        let over = |p: Vec2| geo::ground(p) + Vec3::Y * SCRIPTED_AGL;
        let from = aim + (aim - landing(over(aim)));
        release(&mut commands, &models, &palette, &mut drops, &mut radio, over(from), Vec3::ZERO, now);
    }
}

/// The grenades fall, and burst where they meet the ground.
#[allow(clippy::too_many_arguments)]
fn fall(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxQueue>,
    mut sound: Sound,
    mut blasts: MessageWriter<Blast>,
    mut log: ResMut<crate::hits::HitLog>,
    mut q: Query<(Entity, &mut Falling, &mut Transform)>,
    soldiers: Query<(&Soldier, &Transform), Without<Falling>>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    for (e, mut f, mut tf) in &mut q {
        let mut p = tf.translation;
        fall_step(&mut p, &mut f.vel, dt, Some(now));
        let ground = geo::height(p.x, p.z);
        if p.y > ground {
            tf.translation = p;
            tf.look_to(f.vel.normalize_or(Vec3::NEG_Y), Vec3::Y);
            continue;
        }
        commands.entity(e).despawn();
        let at = Vec3::new(p.x, ground, p.z);
        fx.explosion(at, 0.3, true);
        sound.at(at, Sfx::BlastNear, 0.6);
        blasts.write(Blast { at, radius: VOG_BLAST, cause: Cause::Drop });
        // The feed's line: how close it came to the men it was meant for.
        let attacker = diorama().attacker();
        let near: Vec<f32> = soldiers.iter().filter(|(s, _)| s.side == attacker && s.state != SState::Wounded).map(|(_, t)| t.translation.xz().distance(at.xz())).collect();
        let among = near.iter().filter(|&&d| d < VOG_BLAST * 3.0).count();
        let closest = near.iter().copied().fold(f32::MAX, f32::min);
        let (what, result) = match among {
            0 if closest < 60.0 => ("the ground", format!("missed: {closest:.0} m from the nearest man")),
            0 => ("the ground", "missed: nobody near".to_string()),
            1 => ("a man in the open", "burst beside him".to_string()),
            n => ("men in the open", format!("burst among {n}")),
        };
        log.note(now, "VOG-17", "your drone", what, attacker, result);
        info!("drop: burst at {:.0},{:.0}, {among} men within {:.0} m, the nearest {closest:.0} m, at {now:.0} s", at.x, at.z, VOG_BLAST * 3.0);
    }
}
