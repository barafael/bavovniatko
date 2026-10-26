//! The ground robot: sent, driven, its link, the ground under it, the FPVs that find it, its post and its way home.

use super::*;
use crate::columns::{self, Columns};
use crate::evac::Casualties;
use crate::fx::FxQueue;
use crate::geo;
use crate::sound::{Sfx, Sound};
use crate::units::{Coasting, DirectHit, Cause, Driver, Models, Palette, RouteBook, Side, VKind, VState, Vehicle, VehicleSpec};
use bav_core::field::{Ground, field};
use rand::Rng;

/// The wounded a robot carries out: two on a stretcher frame (c15 §5, the UGV that brought out two after a direct FPV
/// hit).
pub const CAPACITY: u32 = 2;
/// Seconds at the post to unload and load.
const AT_POST: f32 = 25.0;

/// A ground robot on a run: its number, what it carries in, where to, how far along it is, the wounded aboard, and its
/// link and the ground under it.
#[derive(Component)]
pub struct Robot {
    pub n: u32,
    pub cargo: Option<Cargo>,
    pub to: Vec2,
    pub leg: Leg,
    pub wounded: u32,
    /// The link's quality, 1 clean, 0 down.
    pub link: f32,
    /// The link is down until then (game s).
    pub down_until: f64,
    /// Stuck in a crater or in the mud until then (game s).
    pub stuck_until: f64,
    /// The odometer when the ground was last looked at, and the metres driven since the last time their FPVs had
    /// a chance at it.
    looked: f32,
    exposed: f32,
    last_odo: f32,
}

/// Where a run is.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Leg {
    In,
    /// At the post, unloading and loading, for so many seconds.
    AtPost(f32),
    Out,
    /// Knocked out, stuck for good or drowned: left where it is.
    Lost,
}

/// A robot that does not move: its link is down or it is stuck. (The driving stops it, `units::drive`.)
#[derive(Component)]
pub struct Stalled;

/// The player's hand on a robot from its camera (key 7): his throttle and his lane across the route. Not held, the
/// robot drives its route by itself.
#[derive(Component, Default)]
pub struct Hand {
    pub held: bool,
    /// 0..1 of its speed.
    pub throttle: f32,
    /// Metres either side of the route.
    pub lat: f32,
}

impl Robot {
    fn new(n: u32, cargo: Option<Cargo>, to: Vec2) -> Self {
        Self { n, cargo, to, leg: Leg::In, wounded: 0, link: 1.0, down_until: 0.0, stuck_until: 0.0, looked: 0.0, exposed: 0.0, last_odo: 0.0 }
    }

    /// Whether it is under way, in or out.
    pub fn running(&self) -> bool {
        self.leg != Leg::Lost
    }
}

/// The position of ours nearest a point, that a robot can get to.
fn nearest_position(p: Vec2) -> Option<usize> {
    field().positions.iter().enumerate().filter(|(_, pos)| pos.ground == Ground::Ours).min_by(|a, b| a.1.at.distance(p).total_cmp(&b.1.at.distance(p))).map(|(i, _)| i)
}

/// Send the runs asked for while there are robots: each from the road head to its position, a Column of one.
#[allow(clippy::too_many_arguments)]
pub(super) fn send_runs(
    mut commands: Commands,
    time: Res<Time>,
    mut robots: ResMut<Robots>,
    casualties: Option<Res<Casualties>>,
    mut radio: ResMut<Radio>,
    models: Res<Models>,
    palette: Res<Palette>,
    mut book: ResMut<RouteBook>,
    mut columns: ResMut<Columns>,
) {
    let now = time.elapsed_secs_f64();
    let (runs, rest): (Vec<Order>, Vec<Order>) = std::mem::take(&mut robots.orders).into_iter().partition(|o| matches!(o, Order::Run { .. }));
    robots.orders = rest;
    for o in runs {
        let Order::Run { to, cargo } = o else { continue };
        let to = match to {
            Dest::Position(i) => Some(i),
            Dest::Near(p) => nearest_position(p),
            Dest::Wounded => match casualties.as_deref().and_then(|c| c.oldest()) {
                Some(p) => nearest_position(p),
                None => {
                    radio.say2(now, "Викликів на евакуацію нема.", "No calls for evacuation.");
                    continue;
                }
            },
        };
        let Some(to) = to else { continue };
        if robots.left == 0 {
            radio.say2(now, "{player}, я {command}. Роботів більше нема.", "{player}, this is {command}. No robots left to send.");
            continue;
        }
        let way = match columns::way(0, to) {
            Ok(w) => w,
            Err(e) => {
                warn!("robot not sent: {e}");
                continue;
            }
        };
        let line = way.line;
        let len = line.length();
        if len < 80.0 {
            warn!("robot not sent: position {to} is {len:.0} m from the road head");
            continue;
        }
        let pos = field().positions[to].at;
        let r = book.0.len();
        book.0.push(geo::Route { line, deploy_s: len - 30.0, objective: pos, hold: true });
        let wave = 100 + columns.sent;
        columns.sent += 1;
        robots.left -= 1;
        robots.sent += 1;
        let n = robots.sent;
        let e = crate::units::spawn_vehicle(&mut commands, &models, &palette, &book, VehicleSpec { side: Side::Ua, kind: VKind::Ugv, route: r, s: 8.0, lat: 0.0, wave, idx: 0, plow: false, smoke: false, rescue: false, column_pace: f32::INFINITY });
        commands.entity(e).insert((Robot::new(n, Some(cargo), pos), Hand::default()));
        info!("robot {n}: sent with {} to position {to} at {:.0},{:.0}, {len:.0} m ({:.0} m exposed), {} left, at {now:.0} s", cargo.name().1, pos.x, pos.y, way.exposed + 0.0, robots.left);
        let (uk, en) = cargo.name();
        radio.say2(now, format!("Робот {n} пішов: {uk}. {:.1} км.", len / 1000.0), format!("Robot {n} on its way with {en}: {:.1} km.", len / 1000.0));
    }
}

/// A UGV sent some other way (a Column on Supply in Command, `BAV_COLUMN`) is a robot too, with ammunition.
pub(super) fn adopt(mut commands: Commands, mut robots: ResMut<Robots>, book: Res<RouteBook>, q: Query<(Entity, &Vehicle, &Driver), Without<Robot>>) {
    for (e, v, d) in &q {
        if v.kind == VKind::Ugv && v.state.mobile() {
            robots.sent += 1;
            commands.entity(e).insert((Robot::new(robots.sent, Some(Cargo::Ammo), book.0[d.route].objective), Hand::default()));
        }
    }
}

/// The radio link: it frays in their EW, and drops out now and then, mostly for a minute or less, now and then for
/// the best part of an hour of the day's clock (the Zmiy's 1.5 hours, c15 §5). Without it the robot stands.
#[allow(clippy::too_many_arguments)]
pub(super) fn link(mut commands: Commands, time: Res<Time>, ew: Res<crate::ew::Ew>, mut rng: ResMut<RobotRng>, mut radio: ResMut<Radio>, mut q: Query<(Entity, &mut Robot, &Transform, Has<Stalled>)>) {
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    let Some(spec) = spec() else { return };
    let per_s = spec.drops_per_hour * crate::diorama::diorama().compression() / 60.0;
    for (e, mut r, tf, stalled) in &mut q {
        if !r.running() {
            continue;
        }
        let jam = ew.jam_at(tf.translation + Vec3::Y * 1.5);
        let was_down = now < r.down_until;
        if !was_down && rng.0.random_bool(((per_s * (1.0 + 4.0 * jam)) * dt).min(1.0) as f64) {
            // About one in seven is a long one: an hour or an hour and a half of the clock.
            let secs = if rng.0.random_bool(0.15) { rng.0.random_range(60.0..90.0) / crate::diorama::diorama().compression() } else { rng.0.random_range(8.0..45.0) };
            r.down_until = now + secs as f64;
            info!("robot {}: link down for {secs:.0} s at {:.0},{:.0} (jam {jam:.2}), at {now:.0} s", r.n, tf.translation.x, tf.translation.z);
            radio.say2(now, format!("Втратили зв'язок з роботом {}.", r.n), format!("Lost the link to robot {}.", r.n));
        } else if was_down && now + dt as f64 >= r.down_until {
            radio.say2(now, format!("Робот {} знову на зв'язку.", r.n), format!("Robot {} is back on the link.", r.n));
        }
        let down = now < r.down_until;
        r.link = if down { 0.0 } else { (1.0 - jam * 0.8).clamp(0.05, 1.0) };
        let stall = down || now < r.stuck_until;
        if stall && !stalled {
            commands.entity(e).insert(Stalled);
        } else if !stall && stalled {
            commands.entity(e).remove::<Stalled>();
        }
    }
}

/// The ground under it, every five metres: a crater may hold it a while or for good (more in the autumn's mud), the
/// mud off the road now and then, a marsh more often; water is the end of it.
#[allow(clippy::too_many_arguments)]
pub(super) fn going(mut commands: Commands, time: Res<Time>, paint: Res<crate::paint::PaintQueue>, mut rng: ResMut<RobotRng>, mut radio: ResMut<Radio>, mut q: Query<(Entity, &mut Robot, &mut Vehicle, &Driver, &Transform), Without<Coasting>>) {
    let now = time.elapsed_secs_f64();
    let lay = geo::layout();
    let wet = match crate::diorama::diorama().season() {
        bav_core::data::Season::Autumn => 1.0,
        bav_core::data::Season::Winter => 0.5,
        bav_core::data::Season::Summer => 0.3,
    };
    for (e, mut r, mut v, d, tf) in &mut q {
        if !r.running() || !v.state.mobile() || d.odometer < r.looked + 5.0 {
            continue;
        }
        r.looked = d.odometer;
        let p = tf.translation.xz();
        let area = |k: geo::AreaKind| lay.areas.iter().any(|a| a.kind == k && a.contains(p));
        if area(geo::AreaKind::Water) {
            info!("robot {}: into the water at {:.0},{:.0}, at {now:.0} s", r.n, p.x, p.y);
            radio.say2(now, format!("Робот {} у воді. Все.", r.n), format!("Robot {} has gone into the water. That's it.", r.n));
            commands.entity(e).insert(v.immobilise(3.0, 0.0));
            continue;
        }
        let on_road = lay.roads.iter().any(|road| road.line.project(p).0 < road.width * 0.5 + 1.0);
        let crater = paint.craters.near(p, 3.0);
        let p_stuck = if crater {
            0.08 + 0.12 * wet
        } else if area(geo::AreaKind::Wetland) {
            0.03 * wet
        } else if on_road {
            0.0
        } else {
            0.004 * wet
        };
        if p_stuck <= 0.0 || !rng.0.random_bool(p_stuck as f64) {
            continue;
        }
        let what = if crater { ("у вирві", "in a crater") } else { ("у багні", "in the mud") };
        if crater && rng.0.random_bool(0.25) {
            info!("robot {}: stuck for good {} at {:.0},{:.0}, at {now:.0} s", r.n, what.1, p.x, p.y);
            radio.say2(now, format!("Робот {} застряг {} намертво. Не витягнемо.", r.n, what.0), format!("Robot {} is stuck {} for good. We can't get it out.", r.n, what.1));
            commands.entity(e).insert(v.immobilise(3.0, 0.0));
        } else {
            let secs = rng.0.random_range(20.0..90.0);
            r.stuck_until = now + secs;
            info!("robot {}: stuck {} at {:.0},{:.0} for {secs:.0} s, at {now:.0} s", r.n, what.1, p.x, p.y);
            radio.say2(now, format!("Робот {} застряг {}. Розгойдуємо.", r.n, what.0), format!("Robot {} is stuck {}. Rocking it free.", r.n, what.1));
        }
    }
}

/// Their FPVs on the road: every 50 m driven a chance that one finds the robot, by the day's light and whose ground it
/// is on (`found_per_km`). It goes in from above.
#[allow(clippy::too_many_arguments)]
pub(super) fn peril(
    time: Res<Time>,
    script: Res<crate::director::Script>,
    mut rng: ResMut<RobotRng>,
    mut fx: ResMut<FxQueue>,
    mut sound: Sound,
    mut hits: MessageWriter<DirectHit>,
    mut radio: ResMut<Radio>,
    mut q: Query<(Entity, &mut Robot, &Vehicle, &Driver, &Transform)>,
) {
    let now = time.elapsed_secs_f64();
    let per_km = spec().map_or(0.0, |s| s.found_per_km);
    let light = (bav_core::sun::sight(crate::director::clock_minutes(script.t)) - 0.35) / 0.65;
    let pocket = bav_core::field::pocket();
    for (e, mut r, v, d, tf) in &mut q {
        let moved = d.odometer - r.last_odo;
        r.last_odo = d.odometer;
        if !r.running() || !v.state.mobile() || per_km <= 0.0 {
            continue;
        }
        let p = tf.translation.xz();
        r.exposed += moved * if pocket.ground(p) == Ground::Ours { 1.0 } else { 2.0 };
        if r.exposed < 50.0 {
            continue;
        }
        r.exposed -= 50.0;
        let chance = per_km * 0.05 * (0.33 + 0.67 * light);
        if !rng.0.random_bool(chance.clamp(0.0, 1.0) as f64) {
            continue;
        }
        let at = tf.translation + Vec3::Y * 0.8;
        fx.explosion(at, 0.6, false);
        fx.flash(at, 5.0e6, 50.0, 0.12);
        sound.at(at, Sfx::BlastNear, 1.4);
        hits.write(DirectHit { target: e, cause: Cause::Fpv, weapon: "FPV", from: "their drone crew", top: true });
        info!("robot {}: an FPV found it at {:.0},{:.0}, at {now:.0} s", r.n, p.x, p.y);
        radio.say2(now, format!("FPV по роботу {}!", r.n), format!("FPV on robot {}!", r.n));
    }
}

/// At the post: it unloads, takes on the wounded waiting there, and turns for home.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn at_the_post(
    time: Res<Time>,
    mut rng: ResMut<RobotRng>,
    mut robots: ResMut<Robots>,
    mut casualties: Option<ResMut<Casualties>>,
    mut radio: ResMut<Radio>,
    mut view: ResMut<crate::player::View>,
    op: Res<crate::player::Operator>,
    mut posts: Query<(&crate::fire::Post, Option<&mut crate::fight::Ammo>)>,
    mut q: Query<(&mut Robot, &mut Vehicle, &mut Driver, &Transform)>,
) {
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    for (mut r, mut v, mut d, tf) in &mut q {
        let p = tf.translation.xz();
        match r.leg {
            Leg::In if v.state == VState::Overwatch => {
                r.leg = Leg::AtPost(0.0);
                if let Some(c) = r.cargo.take() {
                    deliver(c, p, &mut robots, &mut posts, &mut view, &op);
                    info!("robot {}: at its post at {:.0},{:.0} with {}, at {now:.0} s", r.n, p.x, p.y, c.name().1);
                    radio.say2(now, format!("Робот {} на позиції. Розвантажують: {}.", r.n, c.name().0), format!("Robot {} at the post. Unloading the {}.", r.n, c.name().1));
                }
            }
            Leg::AtPost(secs) => {
                let secs = secs + dt;
                r.leg = Leg::AtPost(secs);
                if secs < AT_POST {
                    continue;
                }
                r.wounded = casualties.as_deref_mut().map_or(0, |c| c.take_near(p, 200.0, CAPACITY));
                r.leg = Leg::Out;
                crate::units::start_withdrawal(&mut v, &mut d, &mut rng.0);
                info!("robot {}: coming out with {} wounded, at {now:.0} s", r.n, r.wounded);
                if r.wounded > 0 {
                    radio.say2(now, format!("Робот {} виходить, на борту {} трьохсотих.", r.n, r.wounded), format!("Robot {} coming out with {} wounded aboard.", r.n, r.wounded));
                } else {
                    radio.say2(now, format!("Робот {} розвантажився, виходить.", r.n), format!("Robot {} unloaded, coming out.", r.n));
                }
            }
            _ => {}
        }
    }
}

/// What a load does where it arrives: ammunition into the post's `Ammo` (where the posts carry it), fuel and
/// batteries back into the drone team's hands near it (a drone and a full battery); every load counted.
pub(super) fn deliver(c: Cargo, at: Vec2, robots: &mut Robots, posts: &mut Query<(&crate::fire::Post, Option<&mut crate::fight::Ammo>)>, view: &mut crate::player::View, op: &crate::player::Operator) {
    robots.delivered[c.index()] += 1;
    match c {
        Cargo::Ammo => {
            let post = posts.iter_mut().filter(|(p, _)| p.side == Side::Ua && p.pos.distance(at) < 120.0).min_by(|a, b| a.0.pos.distance(at).total_cmp(&b.0.pos.distance(at)));
            if let Some((_, Some(mut ammo))) = post {
                ammo.0 += crate::fight::spec().map_or(0, |f| f.ammo / 2);
            }
        }
        Cargo::Power if op.pos.distance(at) < 250.0 => {
            view.drones_left = (view.drones_left + 1).min(3);
            view.battery = 1.0;
        }
        _ => {}
    }
}

/// Home, or lost: back at the road head it counts and goes; knocked out, stuck for good or drowned, it is lost, and
/// the wounded aboard with it.
#[allow(clippy::too_many_arguments)]
pub(super) fn home_or_lost(
    mut commands: Commands,
    time: Res<Time>,
    mut robots: ResMut<Robots>,
    mut casualties: Option<ResMut<Casualties>>,
    mut stats: ResMut<crate::units::Stats>,
    mut radio: ResMut<Radio>,
    mut q: Query<(Entity, &mut Robot, &Vehicle, &Driver, &Transform)>,
) {
    let now = time.elapsed_secs_f64();
    for (e, mut r, v, d, tf) in &mut q {
        if !r.running() {
            continue;
        }
        if !v.state.mobile() {
            r.leg = Leg::Lost;
            robots.lost += 1;
            if let Some(c) = casualties.as_deref_mut() {
                c.died += r.wounded;
            }
            stats.defender_losses += r.wounded;
            commands.entity(e).remove::<Stalled>();
            info!("robot {}: lost at {:.0},{:.0} with {} wounded aboard, at {now:.0} s", r.n, tf.translation.x, tf.translation.z, r.wounded);
            radio.say2(now, format!("Робота {} втратили.", r.n), format!("Robot {} is lost.", r.n));
            continue;
        }
        // Back at the road head, whether it got there or turned back short of it (a wreck across its way).
        if v.state == VState::Withdrawing && d.s < 40.0 {
            robots.back += 1;
            if let Some(c) = casualties.as_deref_mut() {
                c.out += r.wounded;
            }
            info!("robot {}: back with {} wounded{}, at {now:.0} s", r.n, r.wounded, if r.cargo.is_some() { ", its cargo still aboard" } else { "" });
            if r.cargo.is_some() {
                radio.say2(now, format!("Робот {} повернувся, не дійшов.", r.n), format!("Robot {} is back. It never got through.", r.n));
            } else {
                radio.say2(now, format!("Робот {} повернувся. Трьохсотих вивезли: {}.", r.n, r.wounded), format!("Robot {} is back. Wounded brought out: {}.", r.n, r.wounded));
            }
            commands.entity(e).despawn();
        }
    }
}
