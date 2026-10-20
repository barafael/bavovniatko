//! The Enemy staff (ворожий штаб, design/command.md phase 8): in Command the script is still the enemy's plan, the
//! day's columns, guns, TOS and Ka-52s as they were; over it a staff answers what happens. Nothing of it in the Record.
//!
//! **Spotted** (виявлено) is the enemy's picture of our forces, not the truth: a post or a tank in its hide is spotted
//! when it fires; a vehicle of ours when one of their posts or vehicles has it in sight (`geo::line_of_sight`, not
//! through smoke); and anything under an Orlan when one passes over our side. A sighting is where it was then, give or
//! take, and fades: a post's after five minutes, a vehicle's after two, unless it is seen again.
//!
//! What the staff does with it, each a rule with its own budget from the day (`staff` in the diorama's file):
//! - their guns fire on what is spotted instead of on where our posts truly are (`director::russian_artillery`);
//! - Lancets, a few a day, go at our spotted vehicles;
//! - their next column learns from the last ones (`adapt`): heavy losses on one road send it down the other, mines
//!   bring a plow, our missiles and shells more smoke and a BMP in a tank's place.
//!
//! The net tells the player when the intercepts catch it.
//!
//! `BAV_STAFF=off` turns it off in Command; `BAV_STAFF=spot,guns` (of `spot`, `guns`, `lancet`, `columns`) keeps only
//! those rules, for the balance runs to measure what each changes.

use crate::fire::{Missile, Post, UaTank};
use crate::geo;
use crate::hits::HitLog;
use crate::units::{Coasting, Radio, Side, VKind, Vehicle};
use bav_core::diorama::{Play, diorama, play};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::collections::BTreeMap;

pub struct StaffPlugin;

impl Plugin for StaffPlugin {
    fn build(&self, app: &mut App) {
        if play() != Play::Command || crate::dev::dev().staff.as_deref() == Some("off") {
            return;
        }
        let spec = crate::data::sim_data().staff;
        app.insert_resource(Spotted::default())
            .insert_resource(StaffRng(SmallRng::seed_from_u64(crate::dev::seeded(1945))))
            .insert_resource(Staff { lancets: spec.lancets, next_lancet: 240.0, next_orlan: 90.0, losses: BTreeMap::new(), read: 0 })
            // After the units' systems, so the frame's hits are all in the log and the vehicles where they ended up.
            .add_systems(Update, ((spot_fire, spot_by_eye, orlan, forget).run_if(|| rule("spot")), lancets, count_losses).chain().after(crate::units::drop_riders).run_if(crate::running).in_set(crate::Step::Units));
        // Where aircraft fly: a launch's smoke and a ZU's tracers give the post away to the gunships (`air::fight`).
        if crate::air::flies() {
            app.add_systems(Update, spot_air.run_if(|| rule("spot")).after(spot_fire).run_if(crate::running).in_set(crate::Step::Units));
        }
    }
}

/// Whether a rule of the staff's is on: in Command, unless `BAV_STAFF` turns it off or names others.
pub fn rule(name: &str) -> bool {
    if play() != Play::Command {
        return false;
    }
    match crate::dev::dev().staff.as_deref() {
        None | Some("all") => true,
        Some("off") => false,
        Some(list) => list.split(',').any(|r| r.trim() == name),
    }
}

/// The staff's rules from the diorama's file.
#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub struct StaffSpec {
    /// Lancets for the day.
    pub lancets: u32,
    /// Seconds between Orlan passes over our side (at least, at most).
    pub orlan: (f32, f32),
}

impl Default for StaffSpec {
    fn default() -> Self {
        Self { lancets: 0, orlan: (180.0, 360.0) }
    }
}

/// How something of ours was spotted.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum How {
    Fired,
    Seen,
    Orlan,
}

/// What was spotted: a post, a tank in its hide, a vehicle.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum What {
    Post,
    Hide,
    Vehicle,
}

#[derive(Clone, Copy, Debug)]
pub struct Sighting {
    /// Where they think it is.
    pub at: Vec2,
    pub seen: f64,
    pub how: How,
    pub what: What,
}

/// The enemy's picture of our forces.
#[derive(Resource, Default)]
pub struct Spotted(pub BTreeMap<Entity, Sighting>);

impl Spotted {
    fn see(&mut self, e: Entity, at: Vec2, now: f64, how: How, what: What, rng: &mut SmallRng) {
        // Give or take: a firing post is pinned closer than a glimpse from a drone.
        let err = match how {
            How::Fired => 15.0,
            How::Seen => 25.0,
            How::Orlan => 40.0,
        };
        let at = at + Vec2::new(rng.random_range(-err..err), rng.random_range(-err..err));
        self.0.insert(e, Sighting { at, seen: now, how, what });
    }

    /// Where they think our posts and tanks in hides are, west to east (in an order that does not hang on entity
    /// ids, which parallel systems may hand out in either order).
    pub fn positions(&self) -> Vec<Vec2> {
        by_place(self.0.values().filter(|s| s.what != What::Vehicle).map(|s| s.at).collect())
    }

    /// Where they think our vehicles are, west to east.
    pub fn vehicles(&self) -> Vec<Vec2> {
        by_place(self.0.values().filter(|s| s.what == What::Vehicle).map(|s| s.at).collect())
    }
}

fn by_place(mut v: Vec<Vec2>) -> Vec<Vec2> {
    v.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y)));
    v
}

/// The staff's own stream: its choices never shift the day's other draws.
#[derive(Resource)]
struct StaffRng(SmallRng);

/// The staff's budgets and what it has learned.
#[derive(Resource)]
pub struct Staff {
    lancets: u32,
    next_lancet: f32,
    next_orlan: f32,
    /// Their vehicles lost, by the route they were on and what did it.
    losses: BTreeMap<(usize, Loss), u32>,
    /// How many of the hit log's entries have been read.
    read: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Loss {
    Mine,
    Missile,
    Shell,
    Other,
}

/// A post of ours that launches, and a tank in its hide that fires, give themselves away.
#[allow(clippy::type_complexity)]
fn spot_fire(
    time: Res<Time>,
    mut spotted: ResMut<Spotted>,
    mut rng: ResMut<StaffRng>,
    missiles: Query<&Missile, Added<Missile>>,
    vehicles: Query<&Vehicle>,
    posts: Query<(Entity, &Post)>,
    tanks: Query<(Entity, &UaTank, &Transform)>,
    mut timers: Local<BTreeMap<Entity, f32>>,
) {
    let now = time.elapsed_secs_f64();
    for m in &missiles {
        if vehicles.get(m.target).is_ok_and(|v| v.side == Side::Ru)
            && let Some((e, p)) = posts.iter().filter(|(_, p)| p.side == Side::Ua).min_by(|a, b| a.1.pos.distance(m.launched_at.xz()).total_cmp(&b.1.pos.distance(m.launched_at.xz())))
            && p.pos.distance(m.launched_at.xz()) < 40.0
        {
            spotted.see(e, p.pos, now, How::Fired, What::Post, &mut rng.0);
        }
    }
    // A hide's tank fires when its timer starts again from the top.
    for (e, t, tf) in &tanks {
        let last = timers.insert(e, t.timer).unwrap_or(t.timer);
        if t.timer > last + 5.0 {
            spotted.see(e, tf.translation.xz(), now, How::Fired, What::Hide, &mut rng.0);
        }
    }
}

/// Our air defence firing: an Igla's launch from a post, a ZU-23-2's burst (its bursts going down).
#[allow(clippy::type_complexity)]
fn spot_air(
    time: Res<Time>,
    mut spotted: ResMut<Spotted>,
    mut rng: ResMut<StaffRng>,
    launches: Query<&Transform, Added<crate::manpads::IglaMissile>>,
    posts: Query<(Entity, &Post)>,
    guns: Query<(Entity, &Post, &crate::manpads::Zu23)>,
    mut bursts: Local<BTreeMap<Entity, u32>>,
) {
    let now = time.elapsed_secs_f64();
    for t in &launches {
        let at = t.translation.xz();
        if let Some((e, p)) = posts.iter().filter(|(_, p)| p.side == Side::Ua).min_by(|a, b| a.1.pos.distance(at).total_cmp(&b.1.pos.distance(at)))
            && p.pos.distance(at) < 40.0
        {
            spotted.see(e, p.pos, now, How::Fired, What::Post, &mut rng.0);
        }
    }
    for (e, p, gun) in &guns {
        let last = bursts.insert(e, gun.bursts).unwrap_or(gun.bursts);
        if gun.bursts < last {
            spotted.see(e, p.pos, now, How::Fired, What::Post, &mut rng.0);
        }
    }
}

/// Every two seconds: our vehicles that one of their posts or vehicles has in sight, within 1.5 km, not through smoke.
#[allow(clippy::too_many_arguments)]
fn spot_by_eye(
    time: Res<Time>,
    mut spotted: ResMut<Spotted>,
    mut rng: ResMut<StaffRng>,
    smoke: Res<crate::fx::Smoke>,
    posts: Query<&Post>,
    vehicles: Query<(Entity, &Vehicle, &Transform), Without<Coasting>>,
    mut next: Local<f64>,
) {
    let now = time.elapsed_secs_f64();
    if now < *next {
        return;
    }
    *next = now + 2.0;
    let eyes: Vec<Vec2> = posts
        .iter()
        .filter(|p| p.side == Side::Ru && p.strength > 0)
        .map(|p| p.pos)
        .chain(vehicles.iter().filter(|(_, v, _)| v.side == Side::Ru && v.state.alive()).map(|(_, _, t)| t.translation.xz()))
        .collect();
    for (e, v, tf) in &vehicles {
        if v.side != Side::Ua || !v.state.alive() {
            continue;
        }
        let p = tf.translation.xz();
        if eyes.iter().any(|&eye| eye.distance(p) < 1500.0 && geo::line_of_sight(eye, p) && !smoke.blocks(eye, p, now)) {
            spotted.see(e, p, now, How::Seen, What::Vehicle, &mut rng.0);
        }
    }
}

/// Now and then an Orlan crosses our side along a strip 600 m wide: what is in the strip and not under smoke it may
/// see (a post under the trees one time in three, a vehicle four times in five).
#[allow(clippy::too_many_arguments)]
fn orlan(
    time: Res<Time>,
    script: Res<crate::director::Script>,
    mut staff: ResMut<Staff>,
    mut spotted: ResMut<Spotted>,
    mut rng: ResMut<StaffRng>,
    mut radio: ResMut<Radio>,
    smoke: Res<crate::fx::Smoke>,
    posts: Query<(Entity, &Post)>,
    vehicles: Query<(Entity, &Vehicle, &Transform), Without<Coasting>>,
) {
    if script.t < staff.next_orlan {
        return;
    }
    let now = time.elapsed_secs_f64();
    let (lo, hi) = crate::data::sim_data().staff.orlan;
    let rng = &mut rng.0;
    staff.next_orlan = script.t + rng.random_range(lo..hi);
    let x = rng.random_range(-1800.0..1800.0f32);
    let under = |p: Vec2| (p.x - x).abs() < 300.0 && !smoke.covers(p, now);
    let mut found = 0;
    for (e, p) in &posts {
        if p.side == Side::Ua && p.strength > 0 && under(p.pos) && rng.random_bool(0.35) {
            spotted.see(e, p.pos, now, How::Orlan, What::Post, rng);
            found += 1;
        }
    }
    for (e, v, tf) in &vehicles {
        let p = tf.translation.xz();
        if v.side == Side::Ua && v.state.alive() && under(p) && rng.random_bool(0.8) {
            spotted.see(e, p, now, How::Orlan, What::Vehicle, rng);
            found += 1;
        }
    }
    let (lat, lon) = geo::latlon(Vec2::new(x, -900.0));
    radio.say2(now, format!("Орлан над нами, {lat:.3} {lon:.3}. Ховайтесь."), format!("An Orlan over us, near {lat:.3} {lon:.3}. Keep under cover."));
    info!("staff: an Orlan over x {x:.0}, {found} spotted, at {now:.0} s");
}

/// Sightings fade: a post's after five minutes, a vehicle's after two; and what is gone is forgotten.
#[allow(clippy::type_complexity)]
fn forget(time: Res<Time>, mut spotted: ResMut<Spotted>, alive: Query<(), Or<(With<Post>, With<UaTank>, With<Vehicle>)>>) {
    let now = time.elapsed_secs_f64();
    spotted.0.retain(|e, s| alive.contains(*e) && now - s.seen < if s.what == What::Vehicle { 120.0 } else { 300.0 });
}

/// A Lancet at a vehicle of ours they have spotted, while they have Lancets: it comes down out of the sky from their
/// side, every four to eight minutes at most.
#[allow(clippy::too_many_arguments)]
fn lancets(
    mut commands: Commands,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    mut staff: ResMut<Staff>,
    spotted: Res<Spotted>,
    mut rng: ResMut<StaffRng>,
    mut radio: ResMut<Radio>,
    models: Res<crate::units::Models>,
    palette: Res<crate::units::Palette>,
    vehicles: Query<(&Vehicle, &Transform), Without<Coasting>>,
) {
    if !rule("lancet") || staff.lancets == 0 || script.t < staff.next_lancet {
        return;
    }
    let now = time.elapsed_secs_f64();
    // The freshest sighting of a vehicle still there to hit (of those seen at once, the westernmost: not by entity).
    let Some((&target, _)) = spotted.0.iter().filter(|(e, s)| s.what == What::Vehicle && now - s.seen < 30.0 && vehicles.get(**e).is_ok_and(|(v, _)| v.state.alive())).max_by(|a, b| a.1.seen.total_cmp(&b.1.seen).then(b.1.at.x.total_cmp(&a.1.at.x))) else {
        return;
    };
    let Ok((_, tf)) = vehicles.get(target) else { return };
    let rng = &mut rng.0;
    staff.lancets -= 1;
    staff.next_lancet = script.t + rng.random_range(240.0..480.0);
    let p = tf.translation.xz();
    // From their side (+z), high, a kilometre off.
    let from = geo::ground(p + Vec2::new(rng.random_range(-500.0..500.0), 900.0)) + Vec3::Y * 320.0;
    commands.spawn((
        Mesh3d(models.missile.clone()),
        MeshMaterial3d(palette.vertex.clone()),
        Transform::from_translation(from).looking_at(tf.translation, Vec3::Y),
        Missile { target, launched_at: from, speed: 62.0, trail: 0.0, p_hit: Some(0.55), top_attack: false, weapon: "Lancet", from: "a loitering drone" },
        bevy::light::NotShadowCaster,
    ));
    radio.say2(now, "Ланцет у повітрі! Над технікою!", "A Lancet up! Over our vehicles!");
    info!("staff: a Lancet at a vehicle at {:.0},{:.0}, {} left, at {now:.0} s", p.x, p.y, staff.lancets);
}

/// What killed their vehicles, by the road they were on (from the hit log, as it fills).
fn count_losses(mut staff: ResMut<Staff>, log: Res<HitLog>, vehicles: Query<&crate::units::Driver>) {
    // The entries since the last read: the newest `total - read` of the list (no more than it keeps).
    let new = ((log.total - staff.read) as usize).min(log.entries.len());
    staff.read = log.total;
    for h in log.entries[log.entries.len() - new..].iter().filter(|h| h.side == Side::Ru && !h.result.starts_with("missed") && !h.result.contains("did not get through")) {
        let Ok(d) = vehicles.get(h.target) else { continue };
        let loss = match h.weapon {
            w if w.contains("mine") => Loss::Mine,
            "Stugna-P" | "Javelin" | "FPV" => Loss::Missile,
            w if w.contains("mm") || w.contains("HIMARS") || w.contains("RAAM") => Loss::Shell,
            _ => Loss::Other,
        };
        *staff.losses.entry((d.route, loss)).or_default() += 1;
    }
}

/// A column of theirs as the script has it, and as the staff sends it.
pub struct ColumnPlan {
    pub route: usize,
    pub kinds: Vec<VKind>,
    pub plow: bool,
    pub smoke: usize,
    pub start: f32,
}

impl Staff {
    /// The next column after what the last ones met (only where the columns are theirs, attacking):
    /// six or more lost on its road and fewer than half that on the other, the other road; three to mines, a plow in
    /// front; four to our missiles and shells, two more smoke vehicles and a BMP-2 in its first tank's place. What it
    /// changes, the intercepts catch.
    pub fn adapt(&self, mut c: ColumnPlan, radio: &mut Radio, now: f64) -> ColumnPlan {
        if !rule("columns") || diorama().attacker() != Side::Ru {
            return c;
        }
        let lost = |route: usize, of: &[Loss]| -> u32 { self.losses.iter().filter(|((r, l), _)| *r == route && of.contains(l)).map(|(_, n)| n).sum() };
        let all = [Loss::Mine, Loss::Missile, Loss::Shell, Loss::Other];
        let routes = geo::layout().routes.len();
        let mut heard = Vec::new();
        if routes > 1 {
            let other = (c.route + 1) % routes;
            if lost(c.route, &all) >= 6 && lost(other, &all) * 2 < lost(c.route, &all) {
                c.route = other;
                c.start = c.start.min(geo::layout().routes[other].line.length() - 150.0).max(0.0);
                heard.push(("іншою дорогою", "by the other road"));
            }
        }
        if !c.plow && lost(c.route, &[Loss::Mine]) >= 3 {
            c.plow = true;
            heard.push(("з тралом попереду", "with a plow in front"));
        }
        if lost(c.route, &[Loss::Missile, Loss::Shell]) >= 4 {
            c.smoke = (c.smoke + 2).min(c.kinds.len());
            if let Some(k) = c.kinds.iter_mut().find(|k| k.is_tank()) {
                *k = VKind::Bmp2;
            }
            heard.push(("під димом, піхота наперед", "under smoke, the infantry first"));
        }
        if !heard.is_empty() {
            let uk: Vec<&str> = heard.iter().map(|h| h.0).collect();
            let en: Vec<&str> = heard.iter().map(|h| h.1).collect();
            radio.say2(now, format!("Перехоплення: наступна колона йде {}.", uk.join(", ")), format!("Intercept: their next column comes {}.", en.join(", ")));
            info!("staff: the next column {} (route {}, {} vehicles, plow {}, smoke {}), at {now:.0} s", en.join(", "), c.route, c.kinds.len(), c.plow, c.smoke);
        }
        c
    }
}
