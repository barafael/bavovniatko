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
//! `BAV_FPV="<s>:<x>;<z>"` launches one by itself and flies it at the vehicle nearest that point (tests).

use crate::diorama::{Side, diorama};
use crate::fire::{Mission, ShellKind};
use crate::fx::FxQueue;
use crate::geo;
use crate::player::{Fpv, Operator, View};
use crate::sound::{Sfx, Sound};
use crate::units::{Blast, Cause, DirectHit, Radio, Stats, Vehicle};
use bav_core::vocab::Mode;
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct DronesPlugin;

impl Plugin for DronesPlugin {
    fn build(&self, app: &mut App) {
        let d = crate::data::sim_data();
        app.insert_resource(Fpv { left: crate::allot::allotment().fpvs, ..default() })
            .insert_resource(Operator { pos: Vec2::new(d.operator.pos.0, d.operator.pos.1), found: None, moving_until: 0.0, moves: 0 })
            .insert_resource(OperatorRng(SmallRng::seed_from_u64(crate::dev::seeded(1306))))
            .add_systems(Update, drone_input.in_set(crate::Step::Control).after(crate::dev::DevSystems))
            .add_systems(Update, (team_found, fpv_strikes, crate::ew::ew_update).chain().run_if(crate::running).in_set(crate::Step::Script).after(crate::evac::EvacSystems));
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
