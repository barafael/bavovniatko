//! Driving: the traffic on the routes, the drivers' decisions, steering, the tracks they leave, the turrets, and
//! coasting to a stop.

use super::*;

#[derive(Clone, Copy)]
pub(super) struct Snap {
    e: Entity,
    route: usize,
    s: f32,
    lat: f32,
    state: VState,
    speed: f32,
    gear: Gear,
    pos: Vec2,
}

/// How fast the ground lets a tracked vehicle go, as a fraction of its road speed: the road itself, the plowed
/// lane, a field under thin snow or in June's dry stubble, a turn off the road, a shell crater under a track.
pub(super) fn going(p: Vec2, lat: f32, lay: &geo::Layout, craters: &crate::paint::Craters, turning: bool) -> f32 {
    if craters.near(p, 4.0) {
        return 0.3;
    }
    if lay.roads.iter().any(|r| r.line.project(p).0 < r.width * 0.5 + 1.0) {
        return 1.0;
    }
    if lat.abs() < 3.0 && lay.barriers.iter().any(|b| (b.z0..b.z1).contains(&p.y)) {
        return 0.85;
    }
    let field = if diorama().summer() { 0.8 } else { 0.6 };
    if turning { field * 0.8 } else { field }
}

/// Does it drive? The attacker's columns do, and on either side a Column sent to a position; the rest stand.
/// (A hull knocked out wears `Coasting`, and the driving systems leave it out by that.)
pub(super) fn drives(v: &Vehicle, d: &Driver, book: &RouteBook, attacker: Side) -> bool {
    v.side == attacker || book.0[d.route].hold
}

/// What the driver sees of the other hulls this frame (`sense_traffic`), for his decisions and his steering.
#[derive(Component, Default, Clone, Copy)]
pub struct Traffic {
    /// The nearest vehicle ahead in the lane he is steering into, within 55 m.
    blocker: Option<Snap>,
    /// A hull within a length ahead: nothing drives through it.
    blocked_ahead: bool,
    /// A hull beside him, on the side he is moving to.
    blocked_beside: bool,
    /// Which side of the route the hull ahead sits on (the way round it is the other), and where across.
    away: f32,
    ahead_across: f32,
    /// He is more than a couple of metres off the line he wants.
    turning: bool,
}

/// What the driver means to do this frame (`drive_decisions`), for the hull to answer (`steer_and_move`).
#[derive(Component, Default, Clone, Copy)]
pub struct Intent {
    /// The speed he wants, before the ground, the bends and the steering take their share.
    want_speed: f32,
}

/// Who is ahead of me in my lane, and which hulls are right in front of me or beside me. Two hulls that have slid
/// into each other sideways (where neither test looks) work apart.
#[allow(clippy::type_complexity)]
pub(super) fn sense_traffic(
    time: Res<Time>,
    book: Res<RouteBook>,
    mut q: ParamSet<(Query<(Entity, &Vehicle, &Driver, &Transform)>, Sensing)>,
    mut snaps: Local<Vec<Snap>>,
) {
    let dt = crate::sim_dt(&time);
    let attacker = diorama().attacker();
    snaps.clear();
    snaps.extend(q.p0().iter().map(|(e, v, d, t)| Snap { e, route: d.route, s: d.s, lat: d.lat, state: v.state, speed: d.speed, gear: d.gear, pos: t.translation.xz() }));
    for (e, v, mut d, mut tf, mut traffic) in &mut q.p1() {
        if !drives(v, &d, &book, attacker) {
            continue;
        }
        let route: &Polyline = &book.0[d.route].line;
        // Who is ahead of me in my lane?
        let dir = d.dir();
        let mut blocker: Option<Snap> = None;
        for o in snaps.iter() {
            if o.e == e || o.route != d.route {
                continue;
            }
            let ds = (o.s - d.s) * dir;
            // Judge the lane I am steering into, so a vehicle pulling out round a wreck is not blocked by it.
            if ds > 0.0 && ds < 55.0 && (o.lat - d.target_lat).abs() < 6.0 && blocker.is_none_or(|b| (b.s - d.s).abs() > ds) {
                blocker = Some(*o);
            }
        }

        // Steel does not pass through steel: a hull within a length ahead stops this one, and a hull beside it
        // stops the sideways move towards it, whatever lane either is in.
        let mut me = tf.translation.xz();
        let turning = (d.target_lat - d.lat).abs() > 2.0;
        let fwd2 = d.travel();
        let right2 = Vec2::new(-fwd2.y, fwd2.x);
        let side_move = route.normal(d.s) * (d.target_lat - d.lat).signum();
        let (mut blocked_ahead, mut blocked_beside) = (false, false);
        // Which side of the route the hull in the way sits on: the way round it is the other.
        let (mut away, mut ahead_across) = (0.0f32, 0.0f32);
        let mut push = Vec2::ZERO;
        for o in snaps.iter() {
            if o.e == e {
                continue;
            }
            let off = o.pos - me;
            let (along, across) = (off.dot(fwd2), off.dot(right2));
            if (0.0..7.5).contains(&along) && across.abs() < 3.6 {
                blocked_ahead = true;
                away = -(across * dir).signum();
                ahead_across = across;
            }
            // (A hull in front is the test above's: a driver can still turn away from it.)
            blocked_beside |= turning && off.length() < 5.5 && off.dot(side_move) > 0.0 && along < 2.0;
            if off.length() < 5.0 {
                push -= off.normalize_or(route.normal(d.s)) * (5.0 - off.length());
            }
        }
        if push != Vec2::ZERO {
            me += push.clamp_length_max(1.5 * dt);
            (d.lat, d.s) = route.lateral_near(me, d.s, 20.0);
            tf.translation = geo::ground(me) + Vec3::Y * 0.05;
        }
        *traffic = Traffic { blocker, blocked_ahead, blocked_beside, away, ahead_across, turning };
    }
}

/// A vehicle under way: not knocked out (and not a turret).
type Underway = (Without<Turret>, Without<Coasting>);

/// The vehicles under way as `sense_traffic` works them over.
type Sensing<'w, 's> = Query<'w, 's, (Entity, &'static Vehicle, &'static mut Driver, &'static mut Transform, &'static mut Traffic), Without<Coasting>>;

/// A vehicle as its driver works it.
#[derive(bevy::ecs::query::QueryData)]
#[query_data(mutable)]
pub(super) struct Driving {
    e: Entity,
    v: &'static mut Vehicle,
    d: &'static mut Driver,
    tf: &'static Transform,
    traffic: &'static Traffic,
    intent: &'static mut Intent,
    evac_at: Option<&'static crate::evac::EvacTarget>,
    rescue: Has<Rescue>,
    plow: Has<Plow>,
    holding: Has<crate::orders::Holding>,
    stay: Option<&'static Stay>,
}

/// The driver's head: what state the vehicle is in, where he means to be, how fast he wants to go.
#[allow(clippy::too_many_arguments)]
pub(super) fn drive_decisions(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<UnitRng>,
    mut drive: ResMut<DriveRng>,
    mut fxr: ResMut<FxRng>,
    mut fx: ResMut<FxQueue>,
    mut stats: ResMut<Stats>,
    models: Res<Models>,
    palette: Res<Palette>,
    order: Res<ColumnOrder>,
    mut radio: ResMut<Radio>,
    mut smoke: ResMut<fx::Smoke>,
    book: Res<RouteBook>,
    mut q: Query<Driving, Without<Coasting>>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let lay = geo::layout();
    let attacker = diorama().attacker();
    let rng = &mut rng.0;

    for DrivingItem { e, mut v, mut d, tf, traffic, mut intent, evac_at, rescue, plow, holding, stay } in &mut q {
        if !drives(&v, &d, &book, attacker) {
            continue;
        }
        let rt = &book.0[d.route];
        let route: &Polyline = &rt.line;
        let deploy_s = stay.map_or(rt.deploy_s, |s| s.deploy_s);
        // (On a holding route the end of it comes first: the position.)
        let dismount_s = (deploy_s + 200.0 + v.idx as f32 % 3.0 * 25.0).min(route.length() - 5.0);
        v.panic = (v.panic - dt * 0.05).max(0.0);
        v.timer += dt;
        let Traffic { blocker, blocked_ahead, away, .. } = *traffic;

        // A driver's pace drifts over the minutes.
        let mut want_speed = (v.kind.speed() * d.pace * (1.0 + 0.07 * geo::value_noise(now as f32 * 0.02, d.quirk, 79))).min(d.column_pace);
        // A breacher drives with its plow up until the minefield, then digs in at a walking pace.
        if v.kind.breacher() && !lay.barriers.iter().any(|b| (b.z0 - 60.0..b.z1).contains(&tf.translation.z)) {
            want_speed = 4.6 * d.pace;
        }
        if blocker.is_none() {
            d.lead_speed = want_speed;
        }
        // The player's order to the column (Robotyne): hold everyone where they are, or bring them all back. (A
        // Column at its position answers to its own orders.)
        if v.state != VState::Withdrawing && !rescue && !rt.hold {
            match *order {
                ColumnOrder::PullBack => start_withdrawal(&mut v, &mut d, rng),
                ColumnOrder::Hold if matches!(v.state, VState::Advancing | VState::Halted | VState::Deploying) => {
                    want_speed = 0.0;
                }
                _ => {}
            }
        }
        // A rescue vehicle stops short of the first wreck on its track, or where it was sent, puts down smoke and
        // waits for the men.
        if rescue && v.state == VState::Advancing && (blocker.is_some_and(|b| !b.state.mobile() && (b.s - d.s).abs() < 70.0) || evac_at.is_some_and(|t| d.s >= t.0)) {
            v.state = VState::Overwatch;
            v.timer = 0.0;
            pop_smoke(&mut commands, &mut fx, &mut fxr.0, &mut smoke, v.side, tf, now);
            radio.say(now, "Bradley at the wrecks. Ramp down. Get on, get on!");
            info!("rescue: Bradley {}/{} stopped at {:.0},{:.0}, at {now:.0} s", v.wave, v.idx, tf.translation.x, tf.translation.z);
        }
        match v.state {
            VState::Advancing | VState::Halted => {
                if let Some(b) = blocker {
                    let gap = (b.s - d.s).abs();
                    let b_moving = b.state.mobile() && b.speed > 0.5 && b.gear.reversing() == d.gear.reversing() && b.state != VState::Halted;
                    if b_moving {
                        // The driver answers the vehicle ahead a second or three late: the column breathes.
                        d.lead_speed += (b.speed - d.lead_speed) * (dt / d.lag).min(1.0);
                        // Each keeps his own gap, and lets it open and close: the column bunches and strings out.
                        let keep = d.gap * (1.0 + 0.3 * geo::value_noise(now as f32 * 0.015, d.quirk, 80));
                        want_speed = want_speed.min(d.lead_speed * (gap / keep).clamp(0.0, 1.25));
                        v.state = VState::Advancing;
                    } else if gap < 45.0 {
                        // The road is blocked by a wreck or a stalled vehicle. Wait, or go round it through the field;
                        // one already pulling out into a lane with another wreck up it edges on across while he
                        // looks for a way.
                        want_speed = if (b.lat - d.lat).abs() < 6.0 { 0.0 } else { want_speed.min(1.5) };
                        if v.state != VState::Halted {
                            v.state = VState::Halted;
                            v.timer = 0.0;
                        }
                        if v.timer > rng.random_range(4.0..14.0) * (1.0 - v.panic * 0.7) {
                            if rng.random_bool(0.75) {
                                d.target_lat = b.lat + sign(rng) * rng.random_range(8.0..18.0);
                                v.state = VState::Advancing;
                            } else if rng.random_bool(0.3) {
                                start_withdrawal(&mut v, &mut d, rng);
                            }
                            v.timer = 0.0;
                        }
                    }
                } else if v.state == VState::Halted {
                    // A planned halt holds for its time; a halt behind a hull outside the lane ends with a pull
                    // round it; any other halt ends when the road ahead is clear.
                    if d.halt.is_some_and(|h| v.timer < h.secs()) {
                        want_speed = 0.0;
                    } else if blocked_ahead {
                        if v.timer > rng.random_range(4.0..14.0) {
                            let side = sign(rng);
                            d.target_lat = d.lat + if away != 0.0 { away } else { side } * rng.random_range(8.0..18.0);
                            v.timer = 0.0;
                        }
                    } else {
                        d.halt = None;
                        v.state = VState::Advancing;
                    }
                } else if blocked_ahead {
                    v.state = VState::Halted;
                    v.timer = 0.0;
                }
                if let Some(Halt::Planned { at, secs }) = d.halt
                    && v.state == VState::Advancing
                    && d.odometer > at
                {
                    v.state = VState::Halted;
                    v.timer = 0.0;
                    d.halt = Some(Halt::Holding { secs });
                }
                // Stuck in the minefield under fire, a Bradley drops its ramp and the squad goes on foot.
                if v.side == Side::Ua && v.state == VState::Halted && v.troops > 0 && v.timer > 3.5 && v.panic > 0.3 {
                    v.state = VState::Dismounting;
                    v.timer = 0.0;
                    want_speed = 0.0;
                }
                if v.state == VState::Advancing && d.s >= deploy_s {
                    v.state = VState::Deploying;
                    // Fan out into the field to dismount: straight into the shoulder minefields.
                    let spread = [0.0f32, -22.0, 24.0, -45.0, 40.0, -64.0, 60.0, -15.0, 15.0, -34.0, 34.0, -55.0];
                    d.target_lat = if plow { 0.0 } else { spread[(v.idx as usize) % spread.len()] + rng.random_range(-6.0..6.0) };
                }
                // Panic: a hit nearby, and a mobilised driver yanks the tiller.
                if v.panic > 0.55 && rng.random_bool((dt * 0.12) as f64) {
                    if rng.random_bool(0.55) {
                        d.target_lat = d.lat + sign(rng) * rng.random_range(8.0..25.0);
                    } else {
                        start_withdrawal(&mut v, &mut d, rng);
                    }
                }
            }
            VState::Deploying => {
                want_speed = 3.8;
                if blocker.is_some_and(|b| !b.state.mobile() && (b.s - d.s).abs() < 20.0) {
                    d.target_lat += if rng.random_bool(0.5) { 6.0 } else { -6.0 };
                }
                if d.s >= dismount_s {
                    v.state = if v.troops > 0 { VState::Dismounting } else { VState::Overwatch };
                    v.timer = 0.0;
                    if rt.hold {
                        info!("column {}/{}: {:?} at its position, {:.0},{:.0}, at {now:.0} s", v.wave, v.idx, v.kind, tf.translation.x, tf.translation.z);
                    }
                }
            }
            VState::Dismounting => {
                want_speed = 0.0;
                if v.timer > 3.0 && v.troops > 0 {
                    // The ramp drops; the section runs for the tree line.
                    let obj = rt.objective;
                    let back = route.tangent(d.s) * -6.0;
                    let pos = tf.translation.xz();
                    for k in 0..v.troops {
                        let at = pos + back + Vec2::new(rng.random_range(-2.0..2.0), rng.random_range(-2.0..2.0));
                        let goal = if rt.hold {
                            // Into the position, spread along it.
                            obj + Vec2::new((k as f32 - 3.0) * 6.0 + rng.random_range(-2.0..2.0), rng.random_range(-4.0..4.0))
                        } else if v.side == Side::Ua {
                            // The 47th's squads go forward a bound into cover, not across a kilometre of open ground:
                            // a hundred-odd metres, then down, then back to the vehicles.
                            pos + route.tangent(d.s) * rng.random_range(80.0..160.0) + route.normal(d.s) * (k as f32 - 3.0) * 7.0
                        } else {
                            obj + Vec2::new(rng.random_range(-120.0..120.0), rng.random_range(-20.0..20.0)) + Vec2::X * (k as f32 - 3.0) * 6.0
                        };
                        spawn_soldier(&mut commands, &models, &palette, v.side, at, goal, route.point(0.0), rng.random_range(6.0..12.0), None, rng);
                    }
                    v.troops = 0;
                }
                if v.timer > 8.0 {
                    v.state = VState::Overwatch;
                    v.timer = 0.0;
                }
            }
            VState::Overwatch if rescue => {
                want_speed = 0.0;
                // Wait for the men to climb on: until full, or until staying any longer is death too. One the player
                // sent waits longer: the wounded it came for are crawling to it.
                let wait = if evac_at.is_some() { 240.0 } else { 110.0 };
                if v.troops >= 7 || v.timer > wait || (v.panic > 0.95 && v.timer > 45.0) {
                    start_withdrawal(&mut v, &mut d, rng);
                    radio.say2(now, format!("Бредлі виходить, на борту {}.", v.troops), format!("Bradley coming out with {} aboard.", v.troops));
                    info!("rescue: Bradley {}/{} coming out with {} aboard after {:.0} s, at {now:.0} s", v.wave, v.idx, v.troops, v.timer);
                }
            }
            // A Column at its position holds it until it is ordered off, and so does a unit told to hold.
            VState::Overwatch if rt.hold || holding => want_speed = 0.0,
            // One sent to stay holds its place through its time, whatever comes at it.
            VState::Overwatch if stay.is_some_and(|s| v.timer < s.secs) => want_speed = 0.0,
            VState::Overwatch => {
                want_speed = 0.0;
                if v.timer > rng.random_range(60.0..140.0) || v.panic > 0.8 {
                    start_withdrawal(&mut v, &mut d, rng);
                }
            }
            VState::Withdrawing => {
                if d.s < 30.0 {
                    // (The tally is the attacker's: our Columns coming back at Vuhledar are not in it.)
                    if v.side == attacker {
                        stats.withdrawn += 1;
                        stats.returned += v.crew + v.troops;
                    }
                    commands.entity(e).despawn();
                    intent.want_speed = 0.0;
                    continue;
                }
                want_speed = v.kind.speed() * 1.1 * d.pace;
                // Back out of the worst with the glacis to the enemy, then stop and turn round.
                if d.backing() {
                    want_speed = want_speed.min(v.kind.reverse_speed());
                    if v.timer > 6.0 + 10.0 * geo::hash01(v.wave * 100 + v.idx, 28) {
                        want_speed = 0.0;
                        if d.speed < 0.1 {
                            d.gear = Gear::TurnedRound;
                        }
                    }
                }
                if let Some(b) = blocker
                    && !b.state.mobile()
                    && (b.s - d.s).abs() < 25.0
                {
                    d.target_lat = b.lat + if d.lat > b.lat { 10.0 } else { -10.0 };
                }
                // Nose to nose with one coming the other way: pull out to the side away from it.
                if blocked_ahead && d.speed < 0.2 && (d.target_lat - d.lat).abs() < 3.0 && away != 0.0 {
                    d.target_lat = d.lat + away * 8.0;
                }
            }
            _ => {}
        }

        // Hesitations: a driver slows to a crawl for a few seconds (a look round, a word on the intercom, a missed
        // gear), and the ones behind close up on him.
        if v.state == VState::Advancing && !rescue && d.s < deploy_s - 50.0 && d.hesitate <= 0.0 && d.speed > 1.5 && drive.0.random_bool((dt / 150.0) as f64) {
            d.hesitate = drive.0.random_range(1.5..5.0);
        }
        if d.hesitate > 0.0 {
            d.hesitate -= dt;
            want_speed = want_speed.min(0.7);
        }
        // Told to hold (Command's orders): stop where it is, going either way.
        if holding {
            want_speed = 0.0;
        }
        intent.want_speed = want_speed;
    }
}

/// The hull answers the driver: skid steering, the weight in the speed, the ground under the tracks, and the
/// suspension over it all.
pub(super) fn steer_and_move(
    time: Res<Time>,
    paint: Res<PaintQueue>,
    book: Res<RouteBook>,
    mut stats: ResMut<Stats>,
    mut q: Query<(&Vehicle, &mut Driver, &mut Suspension, &mut Transform, &Traffic, &Intent), Underway>,
) {
    let dt = crate::sim_dt(&time);
    let lay = geo::layout();
    let attacker = diorama().attacker();
    for (v, mut d, mut sus, mut tf, traffic, intent) in &mut q {
        if !drives(v, &d, &book, attacker) {
            continue;
        }
        let route: &Polyline = &book.0[d.route].line;
        let Traffic { blocked_ahead, blocked_beside, blocker, ahead_across, turning, .. } = *traffic;
        let mut want_speed = intent.want_speed;
        let dir = d.dir();
        let me = tf.translation.xz();

        // Where the driver means to be: his lane, and on a road or in the field a line of his own that wanders a
        // metre or so either side of it. In the barrier he keeps to the plowed strip, in the plow's own tracks.
        let in_lane = d.target_lat.abs() < 1.0 && lay.barriers.iter().any(|b| (b.z0 - 30.0..b.z1 + 30.0).contains(&me.y));
        let wander = if in_lane { 0.0 } else { 1.2 * geo::value_noise(d.odometer * 0.012, d.quirk, 81) };
        let aim_lat = if blocked_beside { d.lat } else { d.target_lat + wander };
        // He steers for a point a little way up the route, closing on his line at a slant, not sideways.
        let look = if in_lane { 7.0 } else { 8.0 + 2.0 * d.speed };
        let s_aim = d.s + dir * look;
        let lat_aim = d.lat + (aim_lat - d.lat).clamp(-0.6 * look, 0.6 * look);
        let aim = route.point_beyond(s_aim) + route.normal(s_aim.clamp(0.0, route.length())) * lat_aim;
        let travel = d.travel();
        let err = wrap(vehicle_yaw(aim - me) - vehicle_yaw(travel));

        // The ground: a road, a plowed lane, a field under snow, a turn off the road, a crater under a track, and
        // nobody holds a speed over ploughed ground.
        let going = going(me, d.lat, lay, &paint.craters, turning);
        let rough = geo::value_noise(d.odometer * 0.04, d.quirk, 78);
        want_speed *= going * (1.0 + (0.05 + 0.3 * (1.0 - going)) * rough);
        // Slow for a bend coming up, slower for a hard one.
        let bend = route.tangent(d.s).angle_to(route.tangent(d.s + dir * 30.0)).abs();
        want_speed *= 1.0 - 0.35 * (bend / 0.6).min(1.0);
        // Nothing left of the route to drive.
        if (dir > 0.0 && d.s > route.length() - 1.0) || (dir < 0.0 && d.s < 1.0) {
            want_speed = 0.0;
        }
        let eager = want_speed > 0.3;
        // A hull far off the way it wants to go stops and turns on the spot before it drives on.
        if d.detour <= 0.0 {
            want_speed *= 1.0 - ((err.abs() - 0.25) / 0.35).clamp(0.0, 1.0);
        }
        if blocked_ahead {
            want_speed = 0.0;
        }

        // Skid steering. The driver lets the hull run straight until it is off by more than he will stand, then
        // pulls a lever until it is back and lets go; the tracks slip, the hull answers late and swings a little
        // past. So a bend is driven as a few straight runs and turns. In the lane he steers all the time, gently.
        // Stopped, a hull turns on the spot (a pivot, or a Leopard's neutral turn), only if it means to go on.
        let max_rate = if d.speed < 0.8 { v.kind.pivot_rate() } else { 0.5 / (1.0 + d.speed / 6.0) };
        if blocked_ahead && d.speed < 0.3 && eager && (d.target_lat - d.lat).abs() > 3.0 {
            // A hull in the way and a mind to go round it: swing the nose away from it until the way is clear.
            d.steer = v.kind.pivot_rate() * if ahead_across > 0.0 { 1.0 } else { -1.0 };
            d.detour = 4.0;
        } else if d.detour > 0.0 {
            d.steer = 0.0;
            d.detour -= dt;
        } else if in_lane {
            d.steer = (err * 2.0).clamp(-max_rate, max_rate);
        } else if d.speed < 0.3 && !eager {
            d.steer = 0.0;
        } else if d.steer == 0.0 {
            let slack = d.slack * (2.0 - going);
            if err.abs() > slack {
                d.steer = err.signum() * (err.abs() * 2.5).clamp(0.12, max_rate);
            }
        } else if err.abs() < 0.012 || err.signum() != d.steer.signum() {
            d.steer = 0.0;
        } else {
            d.steer = err.signum() * (err.abs() * 2.5).clamp(0.12, max_rate);
        }
        d.yaw_rate += (d.steer - d.yaw_rate) * (dt / 0.35).min(1.0);
        d.yaw = wrap(d.yaw + d.yaw_rate * dt);
        // A turn scrubs speed off: one track brakes, both slide.
        want_speed *= 1.0 - 0.5 * (d.yaw_rate.abs() / v.kind.pivot_rate()).min(1.0);
        // A spoil heap's face is no ground for a hull: it stops at the foot and fires from there (the fields
        // and the belts' swells are far below the grade that slows one).
        let grade = geo::slope(me + d.travel() * 8.0);
        if grade > 0.12 {
            want_speed *= (1.0 - (grade - 0.12) / 0.12).clamp(0.0, 1.0);
        }

        // Speed, with the weight in it: it pulls away slowly, slower in the field and near its top speed, and
        // brakes at about 1.6 m/s² (hard only when something is right in front of it).
        let hard = blocked_ahead || blocker.is_some_and(|b| (b.s - d.s).abs() < 22.0) || v.panic > 0.7;
        let prev_speed = d.speed;
        let accel = v.kind.accel() * (0.45 + 0.55 * going) * (1.0 - 0.55 * (d.speed / v.kind.speed()).min(1.0));
        d.speed += (want_speed - d.speed).clamp(-if hard { 3.5 } else { 1.6 } * dt, accel * dt);
        let heading = d.travel();
        let mut p = me + heading * d.speed * dt;
        (d.lat, d.s) = route.lateral_near(p, d.s, 20.0);
        if in_lane && (d.lat - d.target_lat).abs() > 0.05 {
            // Off the strip by a hand's breadth is off it: he puts the tracks back in the plow's.
            d.lat = d.target_lat + (d.lat - d.target_lat).clamp(-0.05, 0.05);
            p = route.point(d.s) + route.normal(d.s) * d.lat;
        }
        tf.translation = geo::ground(p) + Vec3::Y * 0.05;
        // Suspension: nose down under braking, squat under power, roll in turns, pitching and rocking over the
        // ruts, and worse the worse the going.
        d.accel = (d.speed - prev_speed) / dt.max(1e-4) * if d.backing() { -1.0 } else { 1.0 };
        d.going = going;
        let shake = (d.speed / 5.0).min(1.2) * (1.0 + 2.0 * (1.0 - going));
        let bump = geo::value_noise(d.odometer * 0.45, d.quirk, 77) * 0.025 * shake;
        let rock = geo::value_noise(d.odometer * 0.35, d.quirk, 82) * 0.018 * shake;
        let roll_target = -d.yaw_rate * d.speed * 0.012 + rock;
        let pitch_target = d.accel * 0.022 + bump;
        suspension(&mut sus, pitch_target, roll_target, dt);
        tf.rotation = ground_rotation(p, d.yaw) * Quat::from_rotation_x(sus.pitch) * Quat::from_rotation_z(sus.roll);
        let moved = me.distance(p);
        d.travelled += moved;
        d.odometer += moved;
        stats.vehicle_advance = stats.vehicle_advance.max(p.y * lay.advance);
    }
}

/// What the tracks leave every two metres: the marks, the plow's raw strip and the 2R's lane markers, and the
/// mud or snow the tracks throw where the going is hard.
pub(super) fn track_marks(
    mut commands: Commands,
    mut fxr: ResMut<FxRng>,
    mut fx: ResMut<FxQueue>,
    mut paint: ResMut<PaintQueue>,
    models: Res<Models>,
    palette: Res<Palette>,
    book: Res<RouteBook>,
    mut q: Query<(&Vehicle, &mut Driver, &Transform, Has<Plow>), Without<Coasting>>,
) {
    let lay = geo::layout();
    let attacker = diorama().attacker();
    for (v, mut d, tf, plow) in &mut q {
        if !drives(v, &d, &book, attacker) || d.travelled <= 2.0 {
            continue;
        }
        d.travelled = 0.0;
        let p = tf.translation.xz();
        let heading = d.travel();
        // Turning or braking hard, the tracks skid and dig in: the look paints that churn deeper.
        let churn = (d.yaw_rate.abs() / 0.45).max(d.accel.abs() / 3.5).clamp(0.0, 1.0);
        let (gauge, tread) = v.kind.tracks();
        paint.push(Paint::Track { at: p, dir: heading, gauge, tread, churn });
        // The plow turns the ground over: the lane shows from the air as a raw brown strip.
        if plow {
            paint.push(Paint::Plow { at: p, dir: heading });
            // The 2R marks the lane's edges as it goes, so the vehicles behind can keep to it.
            if v.kind == VKind::Leo2r && ((d.s / 2.0) as u32).is_multiple_of(20) && lay.barriers.iter().any(|b| (b.z0..b.z1).contains(&p.y)) {
                let route = &book.0[d.route].line;
                for side in [-1.0f32, 1.0] {
                    let at = p + route.normal(d.s) * side * 2.6;
                    commands.spawn((Mesh3d(models.marker.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(at)), NotShadowCaster));
                }
            }
        }
        // Hard going throws more: the tracks dig.
        if d.speed > 2.0 && fxr.0.random_bool((0.5 / d.going.max(0.4)).min(1.0) as f64) {
            fx.spray(geo::ground(p) - (heading * 3.0).extend(0.0).xzy(), (heading * d.speed).extend(0.0).xzy());
        }
    }
}

/// The turret follows the aim point the gunner has (`fire::attacker_guns`, `fire::defender_tanks`), at a turret's
/// traverse rate.
pub(super) fn turret_tracking(time: Res<Time>, book: Res<RouteBook>, mut q: Query<(&Vehicle, &Driver, &mut Gun, &Transform), Underway>, mut turrets: Query<&mut Transform, With<Turret>>) {
    let dt = crate::sim_dt(&time);
    let attacker = diorama().attacker();
    for (v, d, mut gun, tf) in &mut q {
        if !drives(v, d, &book, attacker) {
            continue;
        }
        let Some(aim) = gun.aim else { continue };
        let to = aim - tf.translation.xz();
        let rel = vehicle_yaw(to) - d.yaw;
        gun.turret_yaw += (wrap(rel - gun.turret_yaw)).clamp(-dt * 0.6, dt * 0.6);
        if let Ok(mut ttf) = turrets.get_mut(v.turret) {
            ttf.rotation = Quat::from_rotation_y(gun.turret_yaw);
        }
    }
}

/// A damped spring on the hull's pitch and roll.
pub(super) fn suspension(sus: &mut Suspension, pitch_target: f32, roll_target: f32, dt: f32) {
    let (w, z) = (5.5f32, 0.28f32);
    sus.pitch_v += (w * w * (pitch_target.clamp(-0.12, 0.12) - sus.pitch) - 2.0 * z * w * sus.pitch_v) * dt;
    sus.pitch += sus.pitch_v * dt;
    sus.roll_v += (w * w * (roll_target.clamp(-0.1, 0.1) - sus.roll) - 2.0 * z * w * sus.roll_v) * dt;
    sus.roll += sus.roll_v * dt;
}

/// Vehicles knocked out or immobilised while moving coast to a stop, slewing; then the hull settles. (The other
/// hulls' places are last frame's, from their `GlobalTransform`: near enough for a wreck to stop short of one.)
pub(super) fn coast(
    time: Res<Time>,
    mut paint: ResMut<PaintQueue>,
    posts: Query<&crate::fire::Post>,
    book: Res<RouteBook>,
    hulls: Query<(Entity, &GlobalTransform), With<Vehicle>>,
    mut q: Query<(Entity, &mut Vehicle, &mut Driver, &mut Suspension, &mut Transform, &mut Coasting)>,
) {
    let dt = crate::sim_dt(&time);
    for (e, mut v, mut d, mut sus, mut tf, mut c) in &mut q {
        // Nobody at the tiller and the engine alive: the hull circles on its one track until it stalls.
        if let CrewPlan::Ghost { secs, at } = v.plan
            && secs > 0.0
        {
            v.plan = CrewPlan::Ghost { secs: secs - dt, at };
            d.speed = d.speed.max(1.6);
            c.slew = if c.slew >= 0.0 { 0.16 } else { -0.16 };
        }
        // A live crew on a dead track turns the hull to put the glacis towards the nearest enemy position.
        let mut turned = false;
        if let CrewPlan::FightFromTrack { at } = v.plan
            && v.state == VState::Immobilized
            && v.crew > 0
            && d.speed <= 0.02
        {
            let me = tf.translation.xz();
            if let Some(target) = posts.iter().filter(|p| p.side != v.side && p.strength > 0).map(|p| p.pos).min_by(|a, b| a.distance(me).total_cmp(&b.distance(me))) {
                let dyaw = wrap(vehicle_yaw(target - me) - d.yaw);
                if dyaw.abs() < 0.03 {
                    v.plan = CrewPlan::Bail { at };
                } else {
                    d.yaw = wrap(d.yaw + dyaw.clamp(-0.2 * dt, 0.2 * dt));
                    turned = true;
                }
            } else {
                v.plan = CrewPlan::Bail { at };
            }
        }
        // A rolling wreck stops against the next hull instead of sliding into it.
        if d.speed > 0.02 {
            let dir = d.travel();
            let me = tf.translation.xz();
            if hulls.iter().any(|(o, g)| o != e && { let d = g.translation().xz() - me; d.dot(dir) > 0.0 && d.length() < 6.0 }) {
                d.speed = 0.0;
            }
        }
        let moving = d.speed > 0.02;
        if !moving && !turned && sus.pitch.abs() < 1e-4 && sus.pitch_v.abs() < 1e-4 && sus.roll.abs() < 1e-4 && sus.roll_v.abs() < 1e-4 {
            continue;
        }
        let prev = d.speed;
        if moving {
            d.speed = (d.speed - c.decel * dt).max(0.0);
            c.slew *= (1.0 - dt * 0.8).max(0.0);
            let yaw = d.yaw + c.slew * dt * (d.speed / 3.0).min(1.0);
            d.yaw = wrap(yaw);
            let dir = d.travel();
            let p = tf.translation.xz() + dir * d.speed * dt;
            tf.translation = geo::ground(p) + Vec3::Y * 0.05;
            // Keep route coordinates current so the vehicles behind see where the wreck actually is.
            let route = &book.0[d.route].line;
            (d.lat, d.s) = route.lateral(p);
            d.target_lat = d.lat;
            d.travelled += d.speed * dt;
            if d.travelled > 2.0 {
                d.travelled = 0.0;
                // Coasting to a stop with the driver gone or a track off: a skid, dug in.
                let (gauge, tread) = v.kind.tracks();
                paint.push(Paint::Track { at: p, dir, gauge, tread, churn: (c.decel / 3.5 + c.slew.abs() * 2.0).min(1.0) });
            }
        }
        d.accel = (d.speed - prev) / dt.max(1e-4);
        let pitch_target = d.accel * 0.022;
        suspension(&mut sus, pitch_target, 0.0, dt);
        let p = tf.translation.xz();
        tf.rotation = ground_rotation(p, d.yaw) * Quat::from_rotation_x(sus.pitch) * Quat::from_rotation_z(sus.roll);
    }
}

/// +1 or -1, evenly.
pub fn sign(rng: &mut SmallRng) -> f32 {
    if rng.random_bool(0.5) { 1.0 } else { -1.0 }
}

pub fn wrap(a: f32) -> f32 {
    let mut a = a % std::f32::consts::TAU;
    if a > std::f32::consts::PI {
        a -= std::f32::consts::TAU;
    } else if a < -std::f32::consts::PI {
        a += std::f32::consts::TAU;
    }
    a
}

/// Smoke grenades: a fan of grenades arcs 40–60 m ahead and each burns on the ground into a low wall within a
/// minute, which hides the vehicle from the ATGM teams' sights, and from the drone overhead. The Soviet 902
/// "Tucha" dischargers make grey-black HC smoke for two minutes and more; a Bradley's are white phosphorus,
/// a near-instant dense white burst that is gone in half a minute. The dischargers fire once: the caller fires
/// them on a change of state.
pub fn pop_smoke(commands: &mut Commands, fx: &mut FxQueue, rng: &mut SmallRng, smoke: &mut fx::Smoke, side: Side, tf: &Transform, now: f64) {
    let fwd = (tf.rotation * Vec3::NEG_Z).xz().normalize_or_zero();
    let from = tf.translation + Vec3::Y * 2.2;
    for k in 0..10 {
        let a = (k as f32 - 4.5) * 0.22;
        let dir = Vec2::from_angle(a).rotate(fwd);
        let range = rng.random_range(35.0..60.0);
        // Flat arc: 2–3 s in the air with the thin grey trail of the launch charge.
        let secs = rng.random_range(2.0..2.8);
        let land = tf.translation.xz() + dir * range;
        let vel = Vec3::new(dir.x * range / secs, 0.5 * 9.8 * secs, dir.y * range / secs);
        fx.spawn(fx::Spawn { kind: fx::Kind::GreySmoke, pos: from, vel, life: secs, size0: 0.6, size1: 0.8, alpha: 0.9, drag: 0.0, gravity: 9.8, streak: 5.0, ..default() });
        let (kind, rate, burn) = if side == Side::Ua { (EmitterKind::Wp, 1.5, rng.random_range(18.0..30.0)) } else { (EmitterKind::Hc, 0.8, rng.random_range(90.0..150.0)) };
        commands.spawn((Transform::from_translation(geo::ground(land)), Emitter::after(secs, kind, rate, burn, Vec3::Y * 0.3)));
    }
    smoke.add(tf.translation.xz() + fwd * 45.0, 32.0, now, 120.0);
}

pub fn start_withdrawal(v: &mut Vehicle, d: &mut Driver, rng: &mut SmallRng) {
    v.state = VState::Withdrawing;
    d.gear = Gear::Backing;
    // Turning a tank round on a narrow road means a pivot onto the shoulder.
    d.target_lat = d.lat + sign(rng) * rng.random_range(3.0..12.0);
    v.timer = 0.0;
}
