//! Infiltration as a flow (design/myrnohrad.md, phase 4; c15 §5 "infiltration"). Their infantry does not come in
//! columns here but seeps: groups of two or three out of a stretch of the box's edge, at a rate an hour that the
//! weather multiplies (about ten a day in sun, thirty to forty in rain or fog), along the belts, the balkas and the
//! ruins (the route's line, c15 §3) to a building, where they go to ground: a **burrow**, a hidden post with a few
//! rounds and a radio. Most go for our drone crews, close enough that the crews cannot fly; the second radio corrects
//! their guns onto what they see (c15 §5 "two radios"), and in Command it feeds the staff's `Spotted`.
//!
//! A burrow is hidden until it is found: by a post close by, by the observers on their roof, by the drone (its day
//! camera barely; its thermal better, though their cloaks keep a still man's heat in at range, and moving gives most
//! of it away, 02 §6), and by its own fire and its radio. Our posts do not fire on what nobody has found. The drop, an
//! FPV, the guns and, once it is found, a post's rifles kill the men in it; in Command a search-and-strike team (c15
//! §5: the SSO's, HUR's, SBU's and police's joint teams) goes in and clears it.
//!
//! Only where the day's script has a `Seep` (`director::Act::Seep`): nothing of it is spawned or scheduled anywhere
//! else, so the other days' Records do not change.

use crate::diorama::diorama;
use crate::fight::{Eye, Seeing};
use crate::fire::{Mission, Post, ShellKind};
use crate::geo;
use crate::player::{ObservationPost, Operator, View};
use crate::sound::Sfx;
use crate::units::{Dress, Radio, Report, SState, Side, Soldier, Stats};
use bav_core::diorama::{Play, play};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct SeepPlugin;

/// The flow's systems, for what has to run after them in `Step::Units` (the areas, the staff).
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SeepSystems;

impl Plugin for SeepPlugin {
    fn build(&self, app: &mut App) {
        if !present() {
            return;
        }
        let shelters = geo::layout().buildings.iter().map(|b| b.foot.iter().copied().sum::<Vec2>() / b.foot.len().max(1) as f32).filter(|c| geo::in_bounds(*c)).collect();
        app.insert_resource(SeepRng(SmallRng::seed_from_u64(crate::dev::seeded(1511))))
            .insert_resource(Seep { shelters, ..default() })
            // After the units have moved and died this frame, before the areas are counted and the staff looks.
            .add_systems(Update, (flow, walk, find, close_in, calls, empty, log_seep).chain().in_set(SeepSystems).after(crate::units::drop_riders).before(crate::staff::StaffSystems).run_if(crate::running).in_set(crate::Step::Units));
        // The search-and-strike teams are Command's (`SupportKind::Search`): the Record has none.
        if play() == Play::Command {
            app.init_resource::<Searches>().add_systems(Update, search.in_set(SeepSystems).after(calls).before(empty).run_if(crate::running).in_set(crate::Step::Units));
        }
    }
}

/// Whether the day's script sends a seep.
pub fn present() -> bool {
    crate::data::sim_data().script.iter().any(|(_, a)| matches!(a, crate::director::Act::Seep { .. }))
}

/// The flow's own stream: its groups, their ways and what finds them never shift the day's other draws.
#[derive(Resource)]
struct SeepRng(SmallRng);

/// The day's seep: its flows, and its count.
#[derive(Resource, Default)]
pub struct Seep {
    flows: Vec<Flow>,
    /// Where a building stands (the middle of its footprint), each one in the box: where they may go to ground.
    shelters: Vec<Vec2>,
    /// The groups that have gone to ground, whose burrow we have found, and whose burrow is empty, each once: a man of
    /// a group who comes in after the rest are dead goes to ground again, and is no new burrow in the count.
    formed: Vec<u32>,
    found: Vec<u32>,
    emptied: Vec<u32>,
    /// Groups and men sent in.
    pub groups: u32,
    pub men: u32,
    /// Times a burrow's fire hit one of our drone crews, and its radio called their guns.
    pub crews_hit: u32,
    pub calls: u32,
}

impl Seep {
    /// Burrows made, found by us, and emptied (their men dead or gone), a group's counted once.
    pub fn burrows(&self) -> usize {
        self.formed.len()
    }

    pub fn found(&self) -> usize {
        self.found.len()
    }

    pub fn emptied(&self) -> usize {
        self.emptied.len()
    }
}

fn once(groups: &mut Vec<u32>, g: u32) {
    if !groups.contains(&g) {
        groups.push(g);
    }
}

/// One stretch of an edge that sends groups in (`Act::Seep`).
struct Flow {
    route: usize,
    from: f32,
    width: f32,
    per_hour: f32,
    fog: f32,
    crews: f64,
    until: Option<f32>,
    men: (u32, u32),
    /// Men owed by the rate so far, and how many the next group is.
    owed: f32,
    next: u32,
}

/// The script starts a flow (`Act::Seep`).
pub fn start(w: &mut World, act: &'static crate::director::Act) {
    let crate::director::Act::Seep { route, from, width, per_hour, fog, crews, until, men, .. } = act else { return };
    // Owed something already, so the first group is not always a whole group's wait away.
    let (owed, next) = {
        let mut rng = w.resource_mut::<SeepRng>();
        let next = rng.0.random_range(men.0..=men.1);
        (rng.0.random_range(0.0..next as f32), next)
    };
    w.resource_mut::<Seep>().flows.push(Flow { route: *route, from: *from, width: *width, per_hour: *per_hour, fog: *fog, crews: *crews as f64, until: *until, men: *men, owed, next });
}

/// One of a group on its way in: the way's points, the one he is making for, and his group.
#[derive(Component)]
pub struct Seeping {
    way: Vec<Vec2>,
    next: usize,
    group: u32,
}

/// One of theirs in a cloak against thermal cameras (02 §6, partial masking): the picture shows less of his heat the
/// further off it is, unless he moves (`heat_shown`).
#[derive(Component)]
pub struct Cloaked;

/// What keeps one of theirs from our posts' rifles (`fire::small_arms`).
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cover {
    /// On his way in, keeping to the belts, the balkas and the ruins (c15 §3): seen no further than `SNEAK_SEEN` in
    /// clear daylight, and less in fog, and hit as seldom as a man lying down.
    Sneaking,
    /// In a burrow nobody of ours has found: not fired on.
    Hidden,
    /// In a burrow we have found: fired on, behind walls.
    Walls,
}

/// How far a post sees a man keeping to cover in clear daylight (m), against 750 m for one in the open.
pub const SNEAK_SEEN: f32 = 150.0;

impl Cover {
    /// How far a post sees him, for what it would see of a man in the open at `base` (none: not at all).
    pub fn seen(self, base: f32) -> Option<f32> {
        match self {
            Cover::Sneaking => Some(SNEAK_SEEN.min(base)),
            Cover::Hidden => None,
            Cover::Walls => Some(base),
        }
    }

    /// How much of a post's fire that would kill a man standing kills him: a man in cover as seldom as one lying down,
    /// one behind a house's walls a third of that.
    pub fn hit(self) -> f32 {
        match self {
            Cover::Sneaking => 0.35,
            Cover::Hidden | Cover::Walls => 0.12,
        }
    }
}

/// One of the men of this burrow.
#[derive(Component)]
pub struct InBurrow(pub Entity);

/// A group gone to ground in a building: where, its group, the bursts it has left, and when it next fires on a drone
/// crew in reach and next calls its guns (game time, s).
#[derive(Component)]
pub struct Burrow {
    pub at: Vec2,
    group: u32,
    rounds: u32,
    next_shot: f64,
    next_call: f64,
}

/// A burrow we have found, and how: the posts may fire on it, the guns know where it is.
#[derive(Component)]
pub struct Found(pub Finder);

/// What found a burrow.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Finder {
    Post,
    Observers,
    Drone,
    Thermal,
    /// The drone crew it fired on.
    Crew,
    /// Its radio, taken by our direction finders.
    Intercept,
}

/// A cloaked man's heat shows in full this close to the thermal camera (m, slant).
const CLOAK_NEAR: f32 = 250.0;
/// The share of it the cloak lets through at any range.
const CLOAK_LEAK: f32 = 0.12;
/// The share a man on the move shows, cloak or none: the cloak slips, the body works.
const MOVING_SHOWS: f32 = 0.8;

/// How much of a cloaked man's heat the thermal picture shows at `d` metres (slant): all of it close in, falling with
/// the square of the range beyond, to the cloak's leak; on the move, most of it whatever the range.
pub fn heat_shown(d: f32, moving: bool) -> f32 {
    let still = if d <= CLOAK_NEAR { 1.0 } else { (CLOAK_NEAR / d).powi(2).max(CLOAK_LEAK) };
    if moving { still.max(MOVING_SHOWS) } else { still }
}

/// Fog this thin (m) is thick enough to bring them in at the weather's full rate; at `fight::LIFTED` it is gone.
const THICK: f32 = 500.0;

/// How thick the weather is for the flow, 0 in clear air to 1 at its thickest.
fn thickness(seeing: Option<&Seeing>) -> f32 {
    seeing.map_or(0.0, |s| ((crate::fight::LIFTED - s.through(Eye::Man)) / (crate::fight::LIFTED - THICK)).clamp(0.0, 1.0))
}

/// A group goes to ground this close to a post of ours at the nearest: closer, and the post would hear them.
const CLEAR_OF_POSTS: f32 = 150.0;

/// A building for a group to go to ground in, `ring` metres from `target`, clear of our posts and of the other
/// groups' buildings: one of the nearest few. None standing there: the ground itself.
fn shelter(shelters: &[Vec2], target: Vec2, ring: (f32, f32), taken: &[Vec2], posts: &[Vec2], rng: &mut SmallRng) -> Vec2 {
    let mut near: Vec<(f32, Vec2)> = shelters
        .iter()
        .map(|&c| (c.distance(target), c))
        .filter(|(d, c)| *d >= ring.0 && *d <= ring.1 && posts.iter().all(|p| p.distance(*c) > CLEAR_OF_POSTS) && taken.iter().all(|t| t.distance(*c) > 40.0))
        .collect();
    near.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.x.total_cmp(&b.1.x)));
    match near.len() {
        0 => target + Vec2::from_angle(rng.random_range(0.0..std::f32::consts::TAU)) * ring.0.max(30.0),
        n => near[rng.random_range(0..n.min(8))].1,
    }
}

/// How wide a berth a group gives our posts on its way in (m): they know roughly where they are.
const BERTH: f32 = 300.0;

/// A point of the way moved out from any post of ours it is within `BERTH` of.
fn round_posts(mut p: Vec2, posts: &[Vec2]) -> Vec2 {
    for q in posts {
        let d = p - *q;
        if d.length() < BERTH {
            p = *q + d.normalize_or(Vec2::X) * BERTH;
        }
    }
    p
}

/// The seep, and its own random stream.
#[derive(bevy::ecs::system::SystemParam)]
struct Seepage<'w> {
    seep: ResMut<'w, Seep>,
    rng: ResMut<'w, SeepRng>,
}

/// What decides where a group goes: the light, our drone crew's post, our posts, the burrows already taken, and where
/// the groups on the way are going.
#[derive(bevy::ecs::system::SystemParam)]
struct Lay<'w, 's> {
    seeing: Option<Res<'w, Seeing>>,
    op: Res<'w, Operator>,
    posts: Query<'w, 's, &'static Post>,
    burrows: Query<'w, 's, &'static Burrow>,
    walking: Query<'w, 's, &'static Seeping>,
}

/// The flows send their groups as the rate owes them: out of the stretch of the edge, along the route's line, to a
/// building near our drone crew (most) or near the route's objective.
fn flow(
    mut commands: Commands,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    seepage: Seepage,
    mut stats: ResMut<Stats>,
    dress: Dress,
    lay: Lay,
) {
    let Seepage { mut seep, mut rng } = seepage;
    let Dress { models, palette } = dress;
    let Lay { seeing, op, posts, burrows, walking } = lay;

    let dt = crate::sim_dt(&time);
    // Hours of the day's clock in this step.
    let hours = dt * diorama().compression() / 60.0;
    let thick = thickness(seeing.as_deref());
    let rng = &mut rng.0;
    let lay = geo::layout();
    let attacker = diorama().attacker();
    let ours: Vec<Vec2> = posts.iter().filter(|p| p.side != attacker && p.strength > 0).map(|p| p.pos).collect();
    let mut taken: Vec<Vec2> = burrows.iter().map(|b| b.at).chain(walking.iter().filter_map(|w| w.way.last().copied())).collect();
    let Seep { flows, shelters, groups, men, .. } = &mut *seep;
    for f in flows.iter_mut() {
        if f.until.is_some_and(|u| script.t > u) {
            continue;
        }
        f.owed += f.per_hour * (1.0 + (f.fog - 1.0) * thick) * hours;
        if f.owed < f.next as f32 {
            continue;
        }
        let size = f.next;
        // Never quite on the beat: the next group is owed a little sooner or later.
        f.owed -= size as f32 * rng.random_range(0.6..1.4);
        f.next = rng.random_range(f.men.0..=f.men.1);
        let r = &lay.routes[f.route];
        let off = rng.random_range(-f.width..f.width);
        let start = r.line.point(f.from) + r.line.normal(f.from) * off;
        let start = start.clamp(Vec2::new(-geo::HALF_X + 2.0, -geo::HALF_Z + 2.0), Vec2::new(geo::HALF_X - 2.0, geo::HALF_Z - 2.0));
        // Most go for the drone crews, close enough that they cannot launch (c15 §5); the rest for the route's end.
        let crews = rng.random_bool(f.crews);
        let goal = if crews { shelter(shelters, op.pos, (120.0, 280.0), &taken, &ours, rng) } else { shelter(shelters, r.objective, (0.0, 220.0), &taken, &ours, rng) };
        taken.push(goal);
        // Along the line to where it passes nearest the building, drawing in from the stretch's spread as it goes,
        // then across to the building; every point of it round our posts rather than past them.
        let end = r.line.project(goal).1;
        let mut way: Vec<Vec2> = Vec::new();
        let mut s = f.from + 150.0;
        while s < end - 60.0 {
            way.push(r.line.point(s) + r.line.normal(s) * off * (1.0 - (s - f.from) / (end - f.from)));
            s += 150.0;
        }
        let last = way.last().copied().unwrap_or(start);
        let legs = (last.distance(goal) / 150.0) as usize;
        way.extend((1..legs).map(|k| last.lerp(goal, k as f32 / legs as f32)));
        for p in &mut way {
            *p = round_posts(*p, &ours);
        }
        way.push(goal);
        *groups += 1;
        *men += size;
        stats.sent += size;
        info!("seep: a group of {size} in on route {} at {:.0},{:.0} for {} at {:.0},{:.0}, {}", f.route, start.x, start.y, if crews { "the drone crew" } else { "the objective" }, goal.x, goal.y, crate::director::clock_text(script.t));
        for k in 0..size {
            let at = start + Vec2::new(k as f32 * 3.0, rng.random_range(-2.0..2.0));
            let e = crate::units::spawn_soldier(&mut commands, &models, &palette, crate::units::NewSoldier { side: attacker, at, goal: way[0], home: start, sprint: 0.0, cover: None }, rng);
            commands.entity(e).insert((Seeping { way: way.clone(), next: 0, group: *groups }, Cloaked, Cover::Sneaking));
        }
    }
}

/// On their way the men make for one point of it after another; at the building they go to ground in it, a burrow,
/// hidden.
fn walk(mut commands: Commands, time: Res<Time>, mut rng: ResMut<SeepRng>, mut seep: ResMut<Seep>, mut men: Query<(Entity, &mut Soldier, &Transform, &mut Seeping)>, burrows: Query<(Entity, &Burrow, Has<Found>)>) {
    let now = time.elapsed_secs_f64();
    let mut made: Vec<(u32, Entity)> = Vec::new();
    for (e, mut s, tf, mut w) in &mut men {
        if s.state != SState::DugIn {
            continue;
        }
        let last = w.way.len() - 1;
        let at = w.way[last];
        // Down in a hole on the way (on a heap's face, under fire), or arrived at a point: up and on to the next.
        if w.next < last || tf.translation.xz().distance(at) > 15.0 {
            w.next = (w.next + 1).min(last);
            s.goal = w.way[w.next];
            s.state = SState::Advance;
            continue;
        }
        let found = burrows.iter().find(|(_, b, _)| b.group == w.group).map(|(b, _, f)| (b, f));
        let (b, found) = match found.or_else(|| made.iter().find(|(g, _)| *g == w.group).map(|(_, b)| (*b, false))) {
            Some(x) => x,
            None => {
                let b = commands.spawn(Burrow { at, group: w.group, rounds: ROUNDS, next_shot: now + 30.0, next_call: now + rng.0.random_range(40.0..90.0) }).id();
                made.push((w.group, b));
                once(&mut seep.formed, w.group);
                info!("seep: group {} goes to ground at {:.0},{:.0}, at {now:.0} s", w.group, at.x, at.y);
                (b, false)
            }
        };
        commands.entity(e).remove::<Seeping>().insert((InBurrow(b), if found { Cover::Walls } else { Cover::Hidden }));
    }
}

/// Bursts a burrow has for the drone crews (c15 §5: they carry little; a group of three is sent expecting two to die).
const ROUNDS: u32 = 6;

/// How often a burrow is found, a minute of game time at no range, by each that looks: a post (within 300 m in clear
/// daylight), the observers on their roof (800 m), the drone's day camera and its thermal (as far as each sees through
/// the weather, at most 900 m slant). The posts and the observers see a group in a house no further than the light and
/// the fog let them; the thermal sees through the fog three times as far, but the cloaks keep a still man's heat in
/// beyond a couple of hundred metres.
const BY_POST: (f32, f32) = (300.0, 0.6);
const BY_OBSERVERS: (f32, f32) = (800.0, 0.2);
const BY_DRONE: (f32, f32) = (900.0, 0.12);
const BY_THERMAL: (f32, f32) = (900.0, 0.8);

/// Who of ours may find a burrow: our posts, the observers, the drone, and the light they see by.
#[derive(bevy::ecs::system::SystemParam)]
struct Lookouts<'w, 's> {
    seeing: Option<Res<'w, Seeing>>,
    view: Res<'w, View>,
    observers: Res<'w, ObservationPost>,
    posts: Query<'w, 's, &'static Post>,
}

/// Once a second, each burrow nobody has found may be: by whichever of ours is looking, the nearer the likelier.
fn find(
    time: Res<Time>,
    mut tick: Local<f32>,
    mut seepage: Seepage,
    mut finding: Finding,
    lookouts: Lookouts,
    burrows: Query<(Entity, &Burrow), Without<Found>>,
) {
    let Lookouts { seeing, view, observers, posts } = lookouts;
    *tick -= crate::sim_dt(&time);
    if *tick > 0.0 {
        return;
    }
    *tick = 1.0;
    let now = time.elapsed_secs_f64();
    let seeing = seeing.as_deref();
    let attacker = diorama().attacker();
    // How far men see a house at `p` for what they would at `base` in clear daylight.
    let reach = |base: f32, p: Vec2| seeing.map_or(base, |s| s.reach(base, p));
    let through = |eye: Eye, base: f32| seeing.map_or(base, |s| s.through(eye).min(base));
    let chance = |(_, per_min): (f32, f32), d: f32, r: f32| if d < r { per_min * (1.0 - d / r) } else { 0.0 };
    let drone_up = view.feed_down <= 0.0 && view.control;
    for (b, burrow) in &burrows {
        let p = burrow.at;
        let mut by: Vec<(Finder, f32)> = posts.iter().filter(|q| q.side != attacker && q.strength > 0).map(|q| (Finder::Post, chance(BY_POST, q.pos.distance(p), reach(BY_POST.0, p)))).collect();
        if view.op_alive {
            by.push((Finder::Observers, chance(BY_OBSERVERS, observers.eye.xz().distance(p), reach(BY_OBSERVERS.0, p))));
        }
        if drone_up {
            let d = view.drone_pos.distance(geo::ground(p));
            if view.thermal {
                by.push((Finder::Thermal, chance(BY_THERMAL, d, through(Eye::Thermal, BY_THERMAL.0)) * heat_shown(d, false)));
            } else {
                by.push((Finder::Drone, chance(BY_DRONE, d, through(Eye::Man, BY_DRONE.0))));
            }
        }
        for (how, per_min) in by {
            if per_min > 0.0 && seepage.rng.0.random_bool((per_min / 60.0).min(1.0) as f64) {
                finding.reveal(&mut seepage.seep, b, p, burrow.group, how, now);
                break;
            }
        }
    }
}

/// What finding a burrow takes: the commands to mark it and its men, the radio, and the men in the burrows.
#[derive(bevy::ecs::system::SystemParam)]
struct Finding<'w, 's> {
    commands: Commands<'w, 's>,
    radio: ResMut<'w, Radio>,
    men: Query<'w, 's, (Entity, &'static InBurrow)>,
}

impl Finding<'_, '_> {
    /// We have found a burrow: the net says where, and its men are no longer hidden from our posts.
    fn reveal(&mut self, seep: &mut Seep, b: Entity, p: Vec2, group: u32, how: Finder, now: f64) {
        let Self { commands, radio, men } = self;
        commands.entity(b).insert(Found(how));
        for (e, i) in men.iter() {
            if i.0 == b {
                commands.entity(e).insert(Cover::Walls);
            }
        }
        once(&mut seep.found, group);
        let (x, z) = (p.x, p.y);
        let (uk, en) = match how {
            Finder::Post => (format!("Рух у будинку на {x:.0}, {z:.0}. Двоє-троє, засіли."), format!("Movement in a house at {x:.0}, {z:.0}. Two or three, dug in.")),
            Finder::Observers => (format!("Я {{op}}. Бачу їх у будинку на {x:.0}, {z:.0}."), format!("This is {{op}}. I see them in a house at {x:.0}, {z:.0}.")),
            Finder::Drone => (format!("Є рух у вікні на {x:.0}, {z:.0}. Там хтось є."), format!("Movement at a window at {x:.0}, {z:.0}. Someone's in there.")),
            Finder::Thermal => (format!("Тепло в руїні на {x:.0}, {z:.0}. Сидять, під накидками."), format!("Heat in a ruin at {x:.0}, {z:.0}. Sitting under cloaks.")),
            Finder::Crew => (format!("Б'ють з будинку на {x:.0}, {z:.0}!"), format!("Fire from a house at {x:.0}, {z:.0}!")),
            Finder::Intercept => (format!("Перехоплення: коригувальник біля {x:.0}, {z:.0}."), format!("Intercept: their spotter is near {x:.0}, {z:.0}.")),
        };
        radio.say2(now, uk, en);
        info!("seep: a burrow at {x:.0},{z:.0} found by {how:?}, at {now:.0} s");
    }
}

/// A drone crew this close to a burrow is under its fire (m): the crew cannot put a drone up without being seen.
const CLOSE: f32 = 280.0;

/// Our drone crew: where it is set up, and the player's view through its drone.
#[derive(bevy::ecs::system::SystemParam)]
struct DroneCrew<'w> {
    op: ResMut<'w, Operator>,
    view: ResMut<'w, View>,
}

/// A burrow with rounds left fires on our drone crew when the crew is within reach and set up, a burst every minute or
/// so: it may hit a man, and then the crew has to move (`drones::move_team`); the crew sees where it came from.
fn close_in(
    time: Res<Time>,
    mut seepage: Seepage,
    mut finding: Finding,
    mut stats: ResMut<Stats>,
    mut effects: crate::fx::Effects,
    mut crew: DroneCrew,
    mut burrows: Query<(Entity, &mut Burrow, Has<Found>)>,
) {
    let now = time.elapsed_secs_f64();
    let rng = &mut seepage.rng.0;
    let attacker = diorama().attacker();
    for (b, mut burrow, found) in &mut burrows {
        let d = burrow.at.distance(crew.op.pos);
        if burrow.rounds == 0 || now < burrow.next_shot || d > CLOSE || now < crew.op.moving_until || !finding.men.iter().any(|(_, i)| i.0 == b) {
            continue;
        }
        burrow.rounds -= 1;
        burrow.next_shot = now + rng.random_range(40.0..80.0);
        let from = geo::ground(burrow.at) + Vec3::Y * 1.2;
        for _ in 0..2 {
            effects.fx.tracer(from, geo::ground(crew.op.pos + Vec2::new(rng.random_range(-6.0..6.0), rng.random_range(-6.0..6.0))) + Vec3::Y, 700.0, attacker);
        }
        effects.sound.at(from, Sfx::MachineGun, 0.6);
        if rng.random_bool((0.15 + 0.35 * (1.0 - d / CLOSE)) as f64) {
            stats.defender_losses += 1;
            seepage.seep.crews_hit += 1;
            info!("seep: the burrow at {:.0},{:.0} hits the drone crew at {:.0},{:.0}, at {now:.0} s", burrow.at.x, burrow.at.y, crew.op.pos.x, crew.op.pos.y);
            finding.radio.say2(now, "По нас стріляють впритул! Один трьохсотий. Згортаємось, міняємо точку.", "They're shooting at us from close in! One wounded. Packing up, moving.");
            crate::drones::move_team(&mut crew.op, &mut crew.view, &mut finding.radio, now, true);
        } else {
            finding.radio.say2(now, "Обстріл по нашій точці зі стрілецької!", "Small arms on our position!");
        }
        if !found && rng.random_bool(0.5) {
            finding.reveal(&mut seepage.seep, b, burrow.at, burrow.group, Finder::Crew, now);
        }
    }
}

/// How far a burrow's spotter sees one of our posts in clear daylight (m), and our drone crew.
const SPOTS_POST: f32 = 600.0;
const SPOTS_CREW: f32 = 400.0;

/// What a burrow's spotter may see of ours: our drone crew, our posts, and the light he sees them by.
#[derive(bevy::ecs::system::SystemParam)]
struct SpotterView<'w, 's> {
    seeing: Option<Res<'w, Seeing>>,
    op: Res<'w, Operator>,
    posts: Query<'w, 's, (Entity, &'static Post)>,
}

/// Every few minutes a burrow's second radio brings their guns onto what it sees: our drone crew if it is near, else
/// the nearest post of ours in sight, corrected (a tighter sheaf than their guns fire blind). In Command what it sees
/// goes into the staff's picture. A radio heard is a radio found, now and then.
fn calls(
    time: Res<Time>,
    mut seepage: Seepage,
    mut finding: Finding,
    spotter: SpotterView,
    mut spotted: Option<ResMut<crate::staff::Spotted>>,
    mut burrows: Query<(Entity, &mut Burrow, Has<Found>)>,
) {
    let now = time.elapsed_secs_f64();
    let rng = &mut seepage.rng.0;
    let attacker = diorama().attacker();
    let reach = |base: f32, p: Vec2| spotter.seeing.as_deref().map_or(base, |s| s.reach(base, p));
    for (b, mut burrow, found) in &mut burrows {
        if now < burrow.next_call || !finding.men.iter().any(|(_, i)| i.0 == b) {
            continue;
        }
        burrow.next_call = now + rng.random_range(150.0..300.0);
        let p = burrow.at;
        let crew = (spotter.op.pos.distance(p) < reach(SPOTS_CREW, spotter.op.pos)).then_some((None, spotter.op.pos));
        let post = spotter.posts
            .iter()
            .filter(|(_, q)| q.side != attacker && q.strength > 0 && q.pos.distance(p) < reach(SPOTS_POST, q.pos) && geo::line_of_sight(p, q.pos))
            .min_by(|a, b| a.1.pos.distance(p).total_cmp(&b.1.pos.distance(p)))
            .map(|(e, q)| (Some(e), q.pos));
        let Some((post_e, target)) = crew.or(post) else { continue };
        seepage.seep.calls += 1;
        let aim = target + Vec2::new(rng.random_range(-15.0..15.0), rng.random_range(-15.0..15.0));
        finding.commands.spawn(Mission { target: aim, kind: ShellKind::Ru152, rounds: 3, sigma: 18.0, interval: 2.5, next: now + 20.0, flight: 18.0, by: Side::Ru });
        info!("seep: the burrow at {:.0},{:.0} calls their guns onto {:.0},{:.0}, at {now:.0} s", p.x, p.y, target.x, target.y);
        if let (Some(s), Some(e)) = (spotted.as_deref_mut(), post_e) {
            s.0.insert(e, crate::staff::Sighting { at: aim, seen: now, how: crate::staff::How::Seen, what: crate::staff::What::Post });
        }
        if !found && rng.random_bool(0.12) {
            finding.reveal(&mut seepage.seep, b, p, burrow.group, Finder::Intercept, now);
        }
    }
}

/// The search-and-strike teams out (Command): each at a burrow we have found, when it gets there, and when it is back.
#[derive(Resource, Default)]
struct Searches(Vec<SearchRun>);

struct SearchRun {
    burrow: Entity,
    at: Vec2,
    arrive: f64,
    back: Option<f64>,
}

/// A team's pace through the town to a burrow (m/s), and the time to get ready.
const TEAM_PACE: f32 = 2.5;
const TEAM_READY: f32 = 40.0;

/// The burrows we have found, and the men in them.
#[derive(bevy::ecs::system::SystemParam)]
struct FoundBurrows<'w, 's> {
    burrows: Query<'w, 's, (Entity, &'static Burrow), With<Found>>,
    men: Query<'w, 's, (Entity, &'static Soldier, &'static Transform, &'static InBurrow)>,
}

/// Command: a free search-and-strike team goes to each burrow found, and clears it when it gets there: most of the
/// men in it killed, now and then one of ours hit; then it walks back.
fn search(
    mut commands: Commands,
    time: Res<Time>,
    mut runs: ResMut<Searches>,
    mut rng: ResMut<SeepRng>,
    mut report: Report,
    dress: Dress,
    found: FoundBurrows,
) {
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    let teams = crate::allot::settled().map_or(0, |a| a.search) as usize;
    let home = geo::layout().home;
    for (b, burrow) in &found.burrows {
        if runs.0.len() >= teams {
            break;
        }
        if runs.0.iter().any(|r| r.burrow == b) {
            continue;
        }
        let arrive = now + (TEAM_READY + home.distance(burrow.at) / TEAM_PACE) as f64;
        runs.0.push(SearchRun { burrow: b, at: burrow.at, arrive, back: None });
        report.radio.say2(now, format!("Пошуково-ударна група виходить на {:.0}, {:.0}.", burrow.at.x, burrow.at.y), format!("The search team is going for {:.0}, {:.0}.", burrow.at.x, burrow.at.y));
    }
    for run in &mut runs.0 {
        if run.back.is_some() || now < run.arrive {
            continue;
        }
        let (mut killed, mut hit) = (0, 0);
        for (e, s, tf, i) in &found.men {
            if i.0 != run.burrow || s.state == SState::Wounded {
                continue;
            }
            if rng.random_bool(0.8) {
                report.stats.died("search teams", 1);
                crate::units::kill_soldier(&mut commands, &dress.models, &dress.palette, &mut report.stats, (e, s, tf), None, rng);
                killed += 1;
            } else if rng.random_bool(0.3) {
                report.stats.defender_losses += 1;
                hit += 1;
            }
        }
        run.back = Some(now + (home.distance(run.at) / TEAM_PACE) as f64);
        info!("seep: the search team clears the burrow at {:.0},{:.0}: {killed} killed, {hit} of ours hit, at {now:.0} s", run.at.x, run.at.y);
        let (uk, en) = match (killed, hit) {
            (0, _) => ("Будинок порожній. Повертаємось.".to_string(), "The house is empty. Coming back.".to_string()),
            (k, 0) => (format!("Зачищено. Двохсотих у них: {k}. Повертаємось."), format!("Cleared. {k} of them dead. Coming back.")),
            (k, h) => (format!("Зачищено, {k} у них. У нас {h} трьохсотих."), format!("Cleared, {k} of them. {h} of ours wounded.")),
        };
        report.radio.say2(now, uk, en);
    }
    runs.0.retain(|r| r.back.is_none_or(|t| now < t));
}

/// A burrow with no men left in it is gone.
fn empty(mut commands: Commands, time: Res<Time>, mut seep: ResMut<Seep>, burrows: Query<(Entity, &Burrow, Has<Found>)>, men: Query<&InBurrow>) {
    for (b, burrow, found) in &burrows {
        if men.iter().any(|i| i.0 == b) {
            continue;
        }
        commands.entity(b).despawn();
        once(&mut seep.emptied, burrow.group);
        info!("seep: the burrow at {:.0},{:.0} is empty ({}), at {:.0} s", burrow.at.x, burrow.at.y, if found { "found" } else { "never found" }, time.elapsed_secs_f64());
    }
}

/// For the bands (`BAV_LOG_STATS`): the flow once a minute of the day's clock.
fn log_seep(script: Res<crate::director::Script>, seep: Res<Seep>, walking: Query<&Soldier, With<Seeping>>, burrowed: Query<&Soldier, With<InBurrow>>, mut last: Local<Option<u32>>) {
    if !crate::dev::dev().log_stats || *last == Some(script.t as u32 / 60) {
        return;
    }
    *last = Some(script.t as u32 / 60);
    info!(
        "seep: t={:>4.0} {} groups={} men={} walking={} burrows={} found={} cleared={} in_burrows={} crews_hit={} calls={}",
        script.t,
        crate::director::clock_text(script.t),
        seep.groups,
        seep.men,
        walking.iter().count(),
        seep.burrows(),
        seep.found(),
        seep.emptied(),
        burrowed.iter().count(),
        seep.crews_hit,
        seep.calls
    );
}

/// For the HUD's tally: theirs on the way in, in burrows, and the burrows found.
pub fn tally_line(seep: &Seep, walking: usize, burrowed: usize) -> String {
    format!("\nTheirs on the way in {walking}   in burrows {burrowed}   burrows found {} of {}", seep.found(), seep.burrows())
}
