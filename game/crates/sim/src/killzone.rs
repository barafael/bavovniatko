//! Their drones over the ground (design/myrnohrad.md, phase 3): the kill zone over the road into the pocket. By
//! November 2025 nothing on wheels got through to Myrnohrad in clear weather; Rubikon's FPV teams held the one road
//! "almost constantly under fire control", with Sudny Den, the "Irish" (Grom-Kaskad) and the Archangels on the same
//! axis (c15 §4, §5), and "waiting" FPVs sat on the shoulders of every road (c15 §5; Ukrainska Pravda's five
//! observations).
//!
//! - **The crews** are things on the ground at their launch points beyond the +z edge and the jaw, each of a kind:
//!   radio FPVs, fibre-optic FPVs, or a waiter, an FPV landed on a road's shoulder until something passes. Their tempo
//!   is the day's light and weather (`fight::Seeing`): the light radio ones do not fly in fog (c15 §3), the fibre
//!   ones fly worse, and the thermal cameras on most of them leave the night only a little safer than the day
//!   (c15 §5). Their reach is the whole box: fibre spools reached 15 km and more by 2025 (01 e8, e9).
//! - **What they hunt** is their picture: what their cameras find near the roads they watch, every emitter they
//!   locate, and in Command the staff's `Spotted` too. The preference is the record's: drone teams first, then EW,
//!   then vehicles and robots, then men (01 e8: over a quarter of Rubikon's strikes on drone teams, 15% on EW; c15 §5:
//!   "in a day they torched eight of our cars"). Each FPV flies from its launch point to its target, so the player
//!   sees it coming.
//! - **What stops them**: our EW domes (`ew::Dome`) break a radio FPV's link, never a fibre one's, whose cable can
//!   snag instead (02 §6); nets over a stretch of road make a vehicle under them hard to hit until the FPVs have torn
//!   through (c15 §5: "helpful, but enemy drones simply need to find the entrance or create one"; by October they
//!   cut the netting); fog grounds the light ones.
//! - **Rubikon** is a scripted arrival of tempo, not a unit to beat (c15 §10): a crew whose file gives it an arrival
//!   joins then. In the Record its tempo is the day's; in Command the staff spends it, a budget of sorties on what it
//!   has spotted.
//! - **The clock and the road.** A crew's tempo is by the hour of the day's clock, which suits what sits through it
//!   (the posts, the drone team, the domes). What is on the road is there for real minutes, so the crews' sorties at
//!   vehicles and robots go by the real hour (`launch`).
//! - **Our runs**: the day's cars into the town and back at their times; N sends the next run early, Shift+N turns
//!   back what is on the way in or in the town (`RunOrder`; part I's "when the run goes and when it turns back").
//!
//! Only a day whose file has a `killzone` has any of this: the Record of the others must not change.

use crate::fire::{Post, Suppressed};
use crate::fx::{Emitter, EmitterKind, FxQueue};
use crate::geo;
use crate::player::{Fpv, Operator, View};
use crate::sound::{Sfx, Sound};
use crate::units::{Cause, Coasting, DirectHit, Part, Radio, Side, Skin, Stats, VKind, Vehicle, VehicleMaterial};
use bav_core::diorama::{Play, play};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use serde::Deserialize;

/// The day's kill zone (its file's `killzone`).
#[derive(Deserialize, Debug, Clone)]
pub struct KillzoneSpec {
    pub crews: Vec<CrewSpec>,
    /// The roads their cameras watch: what is within `WATCHED` metres of one they may find.
    pub watch: Vec<Vec<(f32, f32)>>,
    /// Our EW domes over posts (`ew::Dome`).
    #[serde(default)]
    pub domes: Vec<crate::ew::DomeSpec>,
    /// Stretches of road under nets, each as its line.
    #[serde(default)]
    pub nets: Vec<Vec<(f32, f32)>>,
    /// Our vehicles on the road: the day's runs into the town and back.
    #[serde(default)]
    pub runs: Vec<RunSpec>,
    /// How often an hour of the day's clock their direction-finders locate our drone team's ground station while a
    /// drone of ours is up (02 §6, "emit and die").
    #[serde(default)]
    pub team: f32,
    /// The share of fibre sorties whose cable snags on the way (02 §6); more in the frost.
    #[serde(default = "snag")]
    pub snag: f32,
    /// Our strikes on their launch points and relays in the Record, each at its time (`StrikeSpec`).
    #[serde(default)]
    pub strikes: Vec<StrikeSpec>,
    /// How long a launch point or a relay our fire has hit stays out (hours of the day's clock): a lever, not a win
    /// (c15 §5, §10).
    #[serde(default = "down")]
    pub down: f32,
}

fn snag() -> f32 {
    0.04
}

fn down() -> f32 {
    1.0
}

/// A strike of ours on a crew's launch point, or on its relay.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct StrikeSpec {
    pub at: f32,
    /// The crew's row in the file.
    pub crew: usize,
    pub by: StrikeBy,
    /// On its relay rather than its launch point.
    #[serde(default)]
    pub relay: bool,
}

/// What a strike of ours on a launch point is.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrikeBy {
    /// A 155 mm mission of four rounds.
    Guns,
    /// An FPV of ours from the drone team, flown at it; our domes pause while it goes out.
    Fpv,
}

/// One of their drone crews.
#[derive(Deserialize, Debug, Clone)]
pub struct CrewSpec {
    pub name: (String, String),
    pub kind: Kind,
    /// Its launch point; a waiter's spot on the shoulder.
    pub at: (f32, f32),
    /// Sorties an hour of the day's clock at full tempo; a waiter's, how often it is laid again once it has gone.
    pub per_hour: f32,
    /// A scripted arrival (game s): the crew flies from then on, and the net says so (Rubikon, c15 §10).
    #[serde(default)]
    pub arrives: Option<f32>,
    /// In Command, the sorties the staff has of an arriving crew to spend on what it has spotted.
    #[serde(default)]
    pub budget: u32,
    /// A radio crew's relay: the repeater on a mast on their ground that carries its links over the road (none: it
    /// needs none). While it is down the crew flies nothing.
    #[serde(default)]
    pub relay: Option<(f32, f32)>,
}

/// What a crew flies.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Radio-controlled FPVs: light, fast, and blind in fog; an EW dome breaks their link.
    Rf,
    /// Fibre-optic FPVs: the cable carries the picture and the control, so no EW touches them (c15 §2, 01 e8).
    Fibre,
    /// An FPV landed on a road's shoulder, waiting for something to pass (c15 §5), on fibre.
    Waiter,
}

impl Kind {
    /// Cruising speed (m/s).
    fn speed(self) -> f32 {
        match self {
            Kind::Rf => 28.0,
            Kind::Fibre => 22.0,
            Kind::Waiter => 18.0,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Kind::Rf => "radio",
            Kind::Fibre => "fibre",
            Kind::Waiter => "waiter",
        }
    }
}

/// A run of ours on the road: a car or a pickup from the road head into the town and back.
#[derive(Deserialize, Debug, Clone)]
pub struct RunSpec {
    /// When it leaves the road head (game s).
    pub at: f32,
    pub kind: CarKind,
    /// Its way in, from the road head.
    pub way: Vec<(f32, f32)>,
    /// How long it stays in the town before it comes back (s); none: it stays.
    #[serde(default)]
    pub back: Option<f32>,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarKind {
    /// A Mitsubishi L200: the brigades' pickup.
    L200,
    /// A Lada, a 2107 or a Niva: the volunteers' cars.
    Lada,
}

impl CarKind {
    pub fn name(self) -> &'static str {
        match self {
            CarKind::L200 => "pickup",
            CarKind::Lada => "Lada",
        }
    }

    fn uk(self) -> &'static str {
        match self {
            CarKind::L200 => "Пікап",
            CarKind::Lada => "Жигуль",
        }
    }

    /// m/s on a shelled road, flat out.
    fn speed(self) -> f32 {
        match self {
            CarKind::L200 => 17.0,
            CarKind::Lada => 15.0,
        }
    }
}

/// The day's kill zone, if it has one.
pub fn spec() -> Option<&'static KillzoneSpec> {
    crate::data::sim_data().killzone.as_ref()
}

/// Whether the day has their drones over the ground.
pub fn present() -> bool {
    spec().is_some()
}

/// What they hunt, in the record's order of preference (01 e8).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Class {
    DroneTeam,
    Ew,
    Vehicle,
    /// Our ground robots (`VKind::Ugv`, `robots`): as much a vehicle to them as a car.
    Robot,
    Men,
}

impl Class {
    /// Lower goes first; vehicles and robots alike.
    fn rank(self) -> u8 {
        match self {
            Class::DroneTeam => 0,
            Class::Ew => 1,
            Class::Vehicle | Class::Robot => 2,
            Class::Men => 3,
        }
    }

    /// What moves on the road in real time rather than sitting through the clock's hours (`launch`).
    fn on_road(self) -> bool {
        matches!(self, Class::Vehicle | Class::Robot)
    }

    pub const ALL: [Class; 5] = [Class::DroneTeam, Class::Ew, Class::Vehicle, Class::Robot, Class::Men];

    pub fn name(self) -> &'static str {
        match self {
            Class::DroneTeam => "drone team",
            Class::Ew => "EW",
            Class::Vehicle => "vehicles",
            Class::Robot => "robots",
            Class::Men => "men",
        }
    }

    /// The chance an FPV that reaches it in clear air does what it came for.
    fn p_hit(self) -> f64 {
        match self {
            Class::DroneTeam => 0.6,
            // A mast and a box among sandbags: harder to find the vitals of than a man.
            Class::Ew => 0.4,
            Class::Vehicle | Class::Robot => 0.75,
            Class::Men => 0.45,
        }
    }
}

/// Something of ours they hunt that is neither a post nor a vehicle of the column game: our cars on the road and our
/// EW domes.
#[derive(Component, Clone, Copy, Debug)]
pub struct Prey(pub Class);

/// One of their drone crews at its launch point (or a waiter on its shoulder).
#[derive(Component)]
pub struct Crew {
    /// Its row in the file.
    pub idx: usize,
    pub kind: Kind,
    /// Sorties it has ready: the tempo fills it, a launch spends one.
    charge: f32,
    /// Sorties it has ready for what is on the road (`Class::Vehicle`, `Class::Robot`): the same tempo, by the hour
    /// of real time rather than of the day's clock, and by the light alone: a crew the fog grounds readies its drones
    /// all the same and flies them when it lifts (`launch`).
    road: f32,
    /// A waiter: when it is on its shoulder again.
    rearm: f64,
    /// An arriving crew in Command: the staff's sorties left.
    budget: u32,
    /// Its relay, if it flies through one.
    pub relay: Option<Entity>,
}

/// A crew that has not arrived yet (`CrewSpec::arrives`).
#[derive(Component)]
pub struct Arriving(pub f32);

/// A radio crew's relay on its mast, on their ground (`CrewSpec::relay`).
#[derive(Component)]
pub struct Relay {
    pub crew: usize,
}

/// A launch point or a relay our fire has put out of action, until then: its crew flies nothing (c15 §5: the HUR's
/// strike on a Rubikon base had a limited effect).
#[derive(Component)]
pub struct Struck(pub f64);

/// A launch point or a relay our direction-finders have located (02 §6, "emit and die": theirs emit too): it shows
/// on the picture. A radio crew gives itself away as it flies; a fibre one does not.
#[derive(Component)]
pub struct Located;

/// An FPV of ours flying itself at a launch point or a relay (`StrikeBy::Fpv`).
#[derive(Component)]
pub struct OurStrike {
    aim: Vec3,
    vel: Vec3,
    battery: f32,
    /// Seconds without its control link (inside one of our own domes).
    lost_for: f32,
}

/// The stream for our strikes on their points and their finding: its draws never shift the kill zone's own.
#[derive(Resource)]
struct PointRng(SmallRng);

/// The chance a radio sortie gives its launch point and relay away to our direction-finders.
const LOCATES: f64 = 0.3;
/// Within so far of a 155 mm burst (times its lethal radius), and of our FPV's, a launch point or a relay may be put
/// out, by these chances.
const SHELL_REACH: f32 = 1.5;
const SHELL_TAKES: f64 = 0.6;
const FPV_REACH: f32 = 10.0;
const FPV_TAKES: f64 = 0.85;

/// Whose: our drone team (the operator, a resource and no thing) or a thing on the field.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mark {
    Team,
    Thing(Entity),
}

/// One thing in their picture, where they last saw it.
#[derive(Clone, Copy, Debug)]
pub struct Sighting {
    pub mark: Mark,
    pub class: Class,
    pub at: Vec2,
    pub seen: f64,
}

/// Their picture of the road: what their cameras have found and when (a minute old and it is gone).
#[derive(Resource, Default)]
pub struct Picture(pub Vec<Sighting>);

impl Picture {
    pub(crate) fn see(&mut self, mark: Mark, class: Class, at: Vec2, now: f64) {
        match self.0.iter_mut().find(|s| s.mark == mark) {
            Some(s) => {
                s.at = at;
                s.seen = now;
            }
            None => self.0.push(Sighting { mark, class, at, seen: now }),
        }
    }
}

/// How the link holds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Link {
    Held,
    /// An EW dome took it: it flies on blind and comes down.
    Broken,
    /// The cable snagged: the same.
    Snagged,
}

/// One of their FPVs in the air, from its launch point to what it is after.
#[derive(Component)]
pub struct TheirFpv {
    pub crew: usize,
    pub kind: Kind,
    pub mark: Mark,
    pub class: Class,
    /// Where it is going in: its target's place as last seen.
    pub aim: Vec3,
    pub vel: Vec3,
    pub link: Link,
    /// The domes whose edge it has crossed (by `ew::Dome::idx`): each tries its link once.
    domes: u32,
    /// A fibre one: the metres it flies before its cable snags, if it does.
    snag_at: Option<f32>,
    flown: f32,
    /// Seconds of battery left.
    battery: f32,
}

/// Where an FPV of theirs came down: on what it was after, or blind.
#[derive(Message, Clone, Copy)]
pub struct Impact {
    pub at: Vec3,
    pub crew: usize,
    pub mark: Option<(Mark, Class)>,
}

/// A stretch of road under nets: an FPV that goes in under it bursts on the mesh, mostly, and tears it; enough
/// holes and it is no cover (c15 §5).
#[derive(Component)]
pub struct Net {
    pub line: Vec<Vec2>,
    pub holes: u32,
}

/// Holes in a net before it is no cover.
const NET_HOLES: u32 = 6;
/// How near the line a vehicle is under it (m).
const NET_HALF: f32 = 7.0;
/// What is left of an FPV's chance on a vehicle under an intact net.
const NET_STOPS: f64 = 0.15;

impl Net {
    fn covers(&self, p: Vec2) -> bool {
        self.holes < NET_HOLES && dist_to_line(&self.line, p) < NET_HALF
    }
}

/// A car of ours on the road.
#[derive(Component)]
pub struct Car {
    pub kind: CarKind,
    way: Vec<Vec2>,
    /// Metres along the way.
    s: f32,
    pub leg: Leg,
    /// How long it stays in the town before it comes back (none: it stays).
    back: Option<f32>,
}

/// Where a car is in its run.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Leg {
    In,
    /// In the town until then.
    Stays(f64),
    Out,
    /// Through: in the town for good, or back at the road head.
    Done,
    Burnt,
}

/// The models their FPVs and our cars wear, as the look loads them.
#[derive(Resource)]
pub struct KillzoneModels {
    pub fpv: Vec<Part>,
    pub l200: Vec<Part>,
    pub lada: Vec<Part>,
}

/// What the day's kill zone did, for the log and the bands.
#[derive(Resource, Default, Debug)]
pub struct KzTally {
    /// Sorties by kind: radio, fibre, waiter.
    pub sorties: [u32; 3],
    /// Strikes that did what they came for, by class (`Class::ALL`'s order).
    pub hits: [u32; 5],
    pub misses: u32,
    pub links_broken: u32,
    pub snagged: u32,
    /// FPVs that went in under a net, and of those how many the net stopped.
    pub netted: u32,
    pub net_saved: u32,
    pub cars_out: u32,
    pub cars_through: u32,
    pub cars_lost: u32,
    /// Cars turned back short of the town (Shift+N) that made it back to the road head.
    pub cars_turned: u32,
    /// Bursts of ours within reach of a launch point or a relay, and of those the ones that put it out.
    pub strikes: u32,
    pub struck: u32,
    /// The sorties their crews did not fly while their points were out: what the strikes cost them.
    pub lost: f32,
}

/// The kill zone's own stream: its draws never shift the rest of the day's.
#[derive(Resource)]
struct KzRng(SmallRng);

/// What is within this of a watched road their cameras may find (m).
const WATCHED: f32 = 300.0;
/// How often they look over the road (s).
const LOOK_EVERY: f64 = 5.0;
/// A sighting this old is gone from the picture (s).
const FADES: f64 = 60.0;
/// At most this many FPVs at one thing at once.
const PER_MARK: usize = 2;
/// Height over the ground they cruise at (m), and the distance out they dive from.
const CRUISE: f32 = 50.0;
const DIVE: f32 = 220.0;
/// An FPV's battery (s): a radio one's 8 minutes or so, a fibre one's spool lasts as long.
const BATTERY: f32 = 480.0;
/// A waiter wakes when one of ours passes within this (m), and as far as it sees in fog.
const WAKE: f32 = 150.0;

/// Their FPVs' systems, for what must run after them.
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct KillzoneSystems;

pub struct KillzonePlugin;

impl Plugin for KillzonePlugin {
    fn build(&self, app: &mut App) {
        if !present() {
            return;
        }
        app.insert_resource(KzRng(SmallRng::seed_from_u64(crate::dev::seeded(2511))))
            .insert_resource(PointRng(SmallRng::seed_from_u64(crate::dev::seeded(2512))))
            .init_resource::<Picture>()
            .init_resource::<KzTally>()
            .add_message::<Impact>()
            .add_message::<RunOrder>()
            .add_systems(Startup, (spawn_crews, spawn_nets).in_set(crate::schedule::Setup::Field))
            // After the script, the team's systems, the jammers and the robots: the frame's operator, drone, domes and robots
            // are settled.
            .add_systems(
                Update,
                (drive, look_over, arrive, points, launch, wait, fly, strike, our_strikes)
                    .chain()
                    .run_if(crate::running)
                    .in_set(crate::Step::Script)
                    .in_set(KillzoneSystems)
                    .after(crate::ew::ew_update)
                    .after(crate::ew::DomeSystems)
                    .after(crate::director::DirectorSystems)
                    .after(crate::robots::RobotSystems),
            )
            .add_systems(Update, log_tally.run_if(crate::running).in_set(crate::Step::Effects));
        if spec().is_some_and(|s| !s.runs.is_empty()) {
            app.add_systems(Update, run_input.run_if(crate::running).in_set(crate::Step::Control).after(crate::dev::DevSystems));
        }
    }
}

fn spawn_crews(mut commands: Commands) {
    let Some(spec) = spec() else { return };
    let command = play() == Play::Command;
    for (idx, c) in spec.crews.iter().enumerate() {
        let at = Vec2::new(c.at.0, c.at.1);
        let relay = c.relay.map(|(x, z)| commands.spawn((Relay { crew: idx }, Transform::from_translation(ground(Vec2::new(x, z))))).id());
        let mut e = commands.spawn((Crew { idx, kind: c.kind, charge: 0.0, road: 0.0, rearm: 0.0, budget: if command { c.budget } else { 0 }, relay }, Transform::from_translation(ground(at))));
        if let Some(t) = c.arrives {
            e.insert(Arriving(t));
        }
    }
}

fn spawn_nets(mut commands: Commands) {
    let Some(spec) = spec() else { return };
    for line in &spec.nets {
        commands.spawn(Net { line: line.iter().map(|&(x, z)| Vec2::new(x, z)).collect(), holes: 0 });
    }
}

/// The ground under a point, in the box or beyond its edge (the launch points are off the map).
pub(crate) fn ground(p: Vec2) -> Vec3 {
    let c = Vec2::new(p.x.clamp(-geo::HALF_X, geo::HALF_X), p.y.clamp(-geo::HALF_Z, geo::HALF_Z));
    Vec3::new(p.x, geo::height(c.x, c.y), p.y)
}

/// How far a point is from a line of points.
fn dist_to_line(line: &[Vec2], p: Vec2) -> f32 {
    line.windows(2)
        .map(|w| {
            let (a, b) = (w[0], w[1]);
            let ab = b - a;
            let t = ((p - a).dot(ab) / ab.length_squared().max(1e-6)).clamp(0.0, 1.0);
            p.distance(a + ab * t)
        })
        .fold(f32::MAX, f32::min)
}

/// The share of a crew's full tempo it flies at in this light and weather (`fight::Seeing`; none: clear daylight).
pub fn tempo(kind: Kind, seeing: Option<&crate::fight::Seeing>) -> f32 {
    let Some(s) = seeing else { return 1.0 };
    let light = light(Some(s));
    let wet = match kind {
        Kind::Rf if s.grounds_light_fpvs() => 0.0,
        Kind::Rf => (s.murk / 2000.0).clamp(0.3, 1.0),
        Kind::Fibre | Kind::Waiter => (s.murk / 2000.0).clamp(0.2, 1.0),
    };
    light * wet
}

/// The share of a crew's full tempo the light alone allows. Thermal cameras on most of them by 2025: the night is only
/// a little safer than the day (c15 §5).
fn light(seeing: Option<&crate::fight::Seeing>) -> f32 {
    seeing.map_or(1.0, |s| 0.4 + 0.6 * s.dark)
}

/// How readily their cameras find what is near the road in this light and weather, 0..1.
fn finding(seeing: Option<&crate::fight::Seeing>) -> f32 {
    seeing.map_or(1.0, |s| (s.through(crate::fight::Eye::Man) / 1500.0).clamp(0.15, 1.0) * (0.5 + 0.5 * s.dark))
}


/// Vehicles on the ground, not coasting to a stop, and not one of their FPVs.
type Ground = (Without<Coasting>, Without<TheirFpv>);
/// Our domes that stand and jam: neither knocked out nor switched off.
type Standing = (Without<crate::ew::DomeDown>, Without<crate::ew::DomeOff>, Without<TheirFpv>);

/// What of ours they can hunt, and where it is now (none: gone, burnt, emptied).
#[derive(bevy::ecs::system::SystemParam)]
pub struct Quarry<'w, 's> {
    prey: Query<'w, 's, (Entity, &'static Prey, &'static Transform, Has<crate::ew::DomeOff>), Without<TheirFpv>>,
    vehicles: Query<'w, 's, (Entity, &'static Vehicle, &'static Transform), Ground>,
    posts: Query<'w, 's, (Entity, &'static Post)>,
}

impl Quarry<'_, '_> {
    /// Where a thing they are after is now. Our drone team is where they located it: a team that has moved since is
    /// not there.
    fn find(&self, mark: Mark) -> Option<Vec3> {
        let Mark::Thing(e) = mark else { return None };
        if let Ok((_, _, t, _)) = self.prey.get(e) {
            return Some(t.translation + Vec3::Y);
        }
        if let Ok((_, v, t)) = self.vehicles.get(e)
            && v.side == Side::Ua
            && v.state.alive()
        {
            return Some(t.translation + Vec3::Y * 1.5);
        }
        self.posts.get(e).ok().filter(|(_, p)| p.side == Side::Ua && p.strength > 0).map(|(_, p)| geo::ground(p.pos) + Vec3::Y)
    }

    /// Whether a thing in their picture is still there to go for.
    fn there(&self, mark: Mark) -> bool {
        mark == Mark::Team || self.find(mark).is_some()
    }

    /// What of ours is on wheels or tracks: the cars, the robots, the column game's vehicles.
    fn wheels(&self) -> Vec<(Mark, Class, Vec2)> {
        let prey = self.prey.iter().filter(|(_, p, _, _)| matches!(p.0, Class::Vehicle | Class::Robot)).map(|(e, p, t, _)| (Mark::Thing(e), p.0, t.translation.xz()));
        let vehicles = self.vehicles.iter().filter(|(_, v, _)| v.side == Side::Ua && v.state.alive()).map(|(e, v, t)| (Mark::Thing(e), if v.kind == VKind::Ugv { Class::Robot } else { Class::Vehicle }, t.translation.xz()));
        prey.chain(vehicles).collect()
    }

    /// Our posts with men in them.
    fn posts(&self) -> Vec<(Mark, Vec2)> {
        self.posts.iter().filter(|(_, p)| p.side == Side::Ua && p.strength > 0).map(|(e, p)| (Mark::Thing(e), p.pos)).collect()
    }
}

/// By place, west to east: an order that does not hang on entity ids.
fn by_place<T>(v: &mut [(Mark, Class, Vec2, T)]) {
    v.sort_by(|a, b| a.2.x.total_cmp(&b.2.x).then(a.2.y.total_cmp(&b.2.y)));
}

/// An order for the day's car run (N, Shift+N, `BAV_RUN`): the plan's "what decides it" in part I, when the run goes
/// and when it turns back.
#[derive(Message, Clone, Copy, PartialEq, Eq, Debug)]
pub enum RunOrder {
    /// The next of the day's runs goes now, with the cars that were to follow it.
    Go,
    /// The cars on their way in or in the town turn back for the road head.
    Back,
}

/// A car turned back short of the town: back at the road head it has not been through.
#[derive(Component)]
pub struct TurnedBack;

/// N sends the day's next car run now, Shift+N turns back the cars on the road in and in the town; `BAV_RUN` gives
/// the same orders at game times. Only where the day has runs.
fn run_input(keys: Res<ButtonInput<KeyCode>>, script: Res<crate::director::Script>, view: Res<View>, mut orders: MessageWriter<RunOrder>, mut done: Local<usize>) {
    let plan = &crate::dev::dev().run;
    while *done < plan.len() && script.t >= plan[*done].0 {
        orders.write(if plan[*done].1 == "back" { RunOrder::Back } else { RunOrder::Go });
        *done += 1;
    }
    if keys.just_pressed(KeyCode::KeyN) && view.mode != bav_core::vocab::Mode::Fpv {
        let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
        orders.write(if shift { RunOrder::Back } else { RunOrder::Go });
    }
}

/// Our cars drive their runs: in along the way, a while in the town, and out the way they came. N sends the next run
/// before its time (the cars that were to follow it as far behind it as they were), Shift+N turns back what is on the
/// way in or in the town.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn drive(
    mut commands: Commands,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    mut tally: ResMut<KzTally>,
    mut radio: ResMut<Radio>,
    models: Option<Res<KillzoneModels>>,
    mut orders: MessageReader<RunOrder>,
    mut cars: Query<(Entity, &mut Car, &mut Transform, Has<TurnedBack>)>,
    mut started: Local<usize>,
    mut starts: Local<Vec<f32>>,
) {
    let Some(spec) = spec() else { return };
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    if starts.len() != spec.runs.len() {
        *starts = spec.runs.iter().map(|r| r.at).collect();
    }
    for o in orders.read() {
        match o {
            RunOrder::Go => match starts.get(*started).copied() {
                Some(first) if first > script.t => {
                    // The run is the cars that were to leave within a minute of the first; they go as far apart.
                    let k = *started;
                    for t in starts.iter_mut().skip(k).take_while(|t| **t < first + 60.0) {
                        *t = script.t + (*t - first);
                    }
                    info!("killzone: the run ordered out now, {:.0} s early, at {now:.0} s", first - script.t);
                    radio.say2(now, "Колона, рушайте зараз.", "Run, go now.");
                }
                _ => radio.say2(now, "Машин на сьогодні більше нема.", "There are no more cars to send today."),
            },
            RunOrder::Back => {
                let mut n = 0;
                for (e, mut car, _, _) in &mut cars {
                    match car.leg {
                        Leg::In => {
                            car.leg = Leg::Out;
                            commands.entity(e).insert(TurnedBack);
                        }
                        Leg::Stays(_) => car.leg = Leg::Out,
                        _ => continue,
                    }
                    n += 1;
                }
                info!("killzone: {n} cars turned back, at {now:.0} s");
                if n > 0 {
                    radio.say2(now, "Машинам: назад, на виїзд.", "Cars: turn back, out to the road head.");
                }
            }
        }
    }
    // The day's runs, each at its time.
    while *started < spec.runs.len() && script.t >= starts[*started] {
        let r = &spec.runs[*started];
        *started += 1;
        let way: Vec<Vec2> = r.way.iter().map(|&(x, z)| Vec2::new(x, z)).collect();
        if way.len() < 2 {
            continue;
        }
        let tf = Transform::from_translation(geo::ground(way[0])).with_rotation(Quat::from_rotation_y(crate::units::vehicle_yaw(way[1] - way[0])));
        let mut e = commands.spawn((Car { kind: r.kind, way, s: 0.0, leg: Leg::In, back: r.back }, Prey(Class::Vehicle), tf));
        if let Some(m) = &models {
            let parts = m.car(r.kind);
            e.insert(crate::units::skin(parts));
            crate::units::dress(&mut e, parts);
        }
        tally.cars_out += 1;
        info!("killzone: a {} out on the road, at {now:.0} s", r.kind.name());
        radio.say2(now, format!("{} пішов дорогою на місто.", r.kind.uk()), format!("A {} is on the road into the town.", r.kind.name()));
    }
    for (e, mut car, mut tf, turned) in &mut cars {
        let len: f32 = car.way.windows(2).map(|w| w[0].distance(w[1])).sum();
        let v = car.kind.speed();
        match car.leg {
            Leg::In => {
                car.s = (car.s + v * dt).min(len);
                if car.s >= len {
                    car.leg = match car.back {
                        Some(stay) => Leg::Stays(now + stay as f64),
                        None => Leg::Done,
                    };
                    through(&mut tally, &mut radio, &car, false, now);
                    if car.leg == Leg::Done {
                        commands.entity(e).remove::<Prey>();
                    }
                }
            }
            Leg::Stays(until) => {
                if now >= until {
                    car.leg = Leg::Out;
                    info!("killzone: the {} back out onto the road, at {now:.0} s", car.kind.name());
                }
            }
            Leg::Out => {
                car.s -= v * dt;
                if car.s <= 0.0 {
                    if turned {
                        tally.cars_turned += 1;
                        info!("killzone: the {} back at the road head, turned back short of the town, at {now:.0} s", car.kind.name());
                        radio.say2(now, "Машина повернулась, до міста не дійшла.", "The car's back. It turned back short of the town.");
                    } else {
                        through(&mut tally, &mut radio, &car, true, now);
                    }
                    commands.entity(e).despawn();
                    continue;
                }
            }
            Leg::Done | Leg::Burnt => continue,
        }
        let (p, dir) = along(&car.way, car.s);
        let dir = if car.leg == Leg::Out { -dir } else { dir };
        tf.translation = geo::ground(p);
        tf.rotation = Quat::from_rotation_y(crate::units::vehicle_yaw(dir));
    }
}

/// A car into the town, or back out of it at the road head.
fn through(tally: &mut KzTally, radio: &mut Radio, car: &Car, back: bool, now: f64) {
    if back || car.back.is_none() {
        tally.cars_through += 1;
    }
    info!("killzone: the {} {}, at {now:.0} s", car.kind.name(), if back { "back at the road head" } else { "in the town" });
    radio.say2(
        now,
        if back { "Машина вийшла з міста. Ціла." } else { "Машина в місті. Доїхали." },
        if back { "The car's out of the town. In one piece." } else { "The car's in the town. They made it." },
    );
}

/// The point `s` metres along a line, and the way it runs there.
pub(crate) fn along(line: &[Vec2], mut s: f32) -> (Vec2, Vec2) {
    for w in line.windows(2) {
        let d = w[0].distance(w[1]);
        let dir = (w[1] - w[0]).normalize_or(Vec2::X);
        if s <= d {
            return (w[0] + dir * s, dir);
        }
        s -= d;
    }
    let n = line.len();
    (line[n - 1], (line[n - 1] - line[n - 2]).normalize_or(Vec2::X))
}

/// Every five seconds their cameras look over the roads: what of ours moves within reach of one they may find (more
/// readily in clear air and by day), our domes they locate as emitters, a post near the road now and then, and our
/// drone team's ground station while a drone of ours is up.
#[allow(clippy::too_many_arguments)]
fn look_over(
    time: Res<Time>,
    mut rng: ResMut<KzRng>,
    mut picture: ResMut<Picture>,
    seeing: Option<Res<crate::fight::Seeing>>,
    view: Res<View>,
    fpv: Res<Fpv>,
    op: Res<Operator>,
    quarry: Quarry,
    mut next: Local<f64>,
) {
    let Some(spec) = spec() else { return };
    let now = time.elapsed_secs_f64();
    if now < *next {
        return;
    }
    *next = now + LOOK_EVERY;
    picture.0.retain(|s| now - s.seen < FADES && quarry.there(s.mark));
    let find = finding(seeing.as_deref()) as f64;
    let roads: Vec<Vec<Vec2>> = spec.watch.iter().map(|w| w.iter().map(|&(x, z)| Vec2::new(x, z)).collect()).collect();
    let watched = |p: Vec2| roads.iter().any(|w| dist_to_line(w, p) < WATCHED);
    let mut seen: Vec<(Mark, Class, Vec2, f64)> = Vec::new();
    seen.extend(quarry.wheels().into_iter().filter(|w| watched(w.2)).map(|(m, c, p)| (m, c, p, 0.6 * find)));
    // Every emitter shows (02 §6): a dome is located whatever the weather, given time, while it jams.
    seen.extend(quarry.prey.iter().filter(|(_, p, _, off)| p.0 == Class::Ew && !off).map(|(e, _, t, _)| (Mark::Thing(e), Class::Ew, t.translation.xz(), 0.25)));
    seen.extend(quarry.posts().into_iter().filter(|p| watched(p.1)).map(|(m, p)| (m, Class::Men, p, 0.1 * find)));
    let up = view.feed_down <= 0.0 || fpv.flying;
    if up && spec.team > 0.0 && now >= op.moving_until {
        // The chance in one look: the hours of the day's clock in five game seconds.
        let p = spec.team * crate::diorama::diorama().compression() / 60.0 * LOOK_EVERY as f32;
        seen.push((Mark::Team, Class::DroneTeam, op.pos, p as f64));
    }
    by_place(&mut seen);
    let rng = &mut rng.0;
    for (mark, class, at, p) in seen {
        if rng.random_bool(p.clamp(0.0, 1.0)) {
            let at = at + Vec2::new(rng.random_range(-8.0..8.0), rng.random_range(-8.0..8.0));
            picture.see(mark, class, at, now);
        }
    }
}

/// A crew whose time has come arrives, and the intercepts say so (Rubikon on the road).
fn arrive(mut commands: Commands, time: Res<Time>, script: Res<crate::director::Script>, mut radio: ResMut<Radio>, crews: Query<(Entity, &Crew, &Arriving)>) {
    let Some(spec) = spec() else { return };
    let now = time.elapsed_secs_f64();
    let mut due: Vec<(usize, Entity)> = crews.iter().filter(|(_, _, a)| script.t >= a.0).map(|(e, c, _)| (c.idx, e)).collect();
    due.sort_by_key(|d| d.0);
    for (idx, e) in due {
        commands.entity(e).remove::<Arriving>();
        let name = &spec.crews[idx].name;
        info!("killzone: {} on the road, at {now:.0} s", name.0);
        radio.say2(now, format!("Перехоплення: на дорогу заходить «{}». Їхніх пташок стане більше.", name.1), format!("Intercept: {} is coming onto the road. There'll be more of their birds.", name.0));
    }
}

/// Each crew's tempo fills its sorties; with one ready it launches at the best thing in its picture that has not
/// already two of their FPVs on the way.
///
/// The tempo is by the hour of the day's clock, which runs fourteen to eighteen times faster than the game's
/// (`compression`): over a post or the drone team, which sit through the clock's hours, that is the rate. What is on
/// the road does not: a robot at 10 km/h, or a car, is out for real minutes, and a crew's sorties at it can only be
/// those of the minutes it is there. So vehicles and robots draw on a charge of their own, filled by the hour of
/// real time; on the clock's, a robot half an hour on the road met the sorties of seven hours. It fills by the light
/// alone: in fog the crews ready their drones and wait, so what is on the road when it lifts meets them at once.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn launch(
    mut commands: Commands,
    time: Res<Time>,
    mut tally: ResMut<KzTally>,
    picture: Res<Picture>,
    seeing: Option<Res<crate::fight::Seeing>>,
    spotted: Option<Res<crate::staff::Spotted>>,
    models: Option<Res<KillzoneModels>>,
    mut rng: ResMut<KzRng>,
    (mut found, mut radio): (ResMut<PointRng>, ResMut<Radio>),
    quarry: Quarry,
    mut crews: Query<(Entity, &mut Crew, &Transform, Has<Located>), (Without<Arriving>, Without<Struck>)>,
    relays: Query<Has<Struck>, With<Relay>>,
    flying: Query<&TheirFpv>,
) {
    let Some(spec) = spec() else { return };
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    // Sorties an hour of the day's clock, as a rate a game second; and an hour of real time, for the road's.
    let per_s = crate::diorama::diorama().compression() / 60.0;
    let real_s = 1.0 / 3600.0;
    let mut out: Vec<Mark> = flying.iter().map(|f| f.mark).collect();
    // A crew whose launch point or relay our fire has put out flies nothing until it is back (`points`).
    let mut crews: Vec<_> = crews.iter_mut().filter(|(_, c, _, _)| c.kind != Kind::Waiter && !c.relay.is_some_and(|r| relays.get(r).unwrap_or(false))).collect();
    crews.sort_by_key(|(_, c, _, _)| c.idx);
    for (e, mut crew, tf, located) in crews {
        let cs = &spec.crews[crew.idx];
        // In Command the staff spends an arriving crew's sorties, on what it has spotted.
        let staffed = cs.arrives.is_some() && spotted.is_some();
        let tempo = tempo(crew.kind, seeing.as_deref());
        crew.charge = (crew.charge + cs.per_hour * tempo * per_s * dt).min(2.0);
        crew.road = (crew.road + cs.per_hour * light(seeing.as_deref()) * real_s * dt).min(1.0);
        let (ready, on_road) = (crew.charge >= 1.0, crew.road >= 1.0);
        // Grounded (radio crews in thick fog), it flies nothing, whatever it has ready.
        if !(ready || on_road) || tempo <= 0.0 || (staffed && crew.budget == 0) {
            continue;
        }
        let from = tf.translation.xz();
        let mut options: Vec<(Mark, Class, Vec2)> = if staffed { Vec::new() } else { picture.0.iter().map(|s| (s.mark, s.class, s.at)).collect() };
        if let Some(sp) = &spotted {
            options.extend(sp.prey());
        }
        let best = options
            .into_iter()
            .filter(|(m, c, _)| if c.on_road() { on_road } else { ready } && out.iter().filter(|o| *o == m).count() < PER_MARK && quarry.there(*m))
            .min_by(|a, b| a.1.rank().cmp(&b.1.rank()).then(a.2.distance(from).total_cmp(&b.2.distance(from))).then(a.2.x.total_cmp(&b.2.x)));
        let Some((mark, class, at)) = best else { continue };
        if class.on_road() {
            crew.road -= 1.0;
        } else {
            crew.charge -= 1.0;
        }
        if staffed {
            crew.budget -= 1;
        }
        out.push(mark);
        let k = crew.kind;
        let snag_at = (k == Kind::Fibre && rng.0.random_bool(spec.snag as f64)).then(|| rng.0.random_range(300.0..from.distance(at).max(400.0)));
        put_up(&mut commands, models.as_deref(), crew.idx, k, tf.translation + Vec3::Y * 2.0, mark, class, ground(at) + Vec3::Y, snag_at);
        tally.sorties[k as usize] += 1;
        info!("killzone: {} sends a {} FPV at the {} at {:.0},{:.0}, {:.1} km, at {now:.0} s", cs.name.0, k.name(), class.name(), at.x, at.y, from.distance(at) / 1000.0);
        // A radio crew's control link is an emitter: our direction-finders may take a bearing on it, and on its relay.
        if k == Kind::Rf && !located && found.0.random_bool(LOCATES) {
            commands.entity(e).insert(Located);
            if let Some(r) = crew.relay {
                commands.entity(r).insert(Located);
            }
            let (lat, lon) = geo::latlon(from);
            info!("killzone: {} located at {:.0},{:.0}, at {now:.0} s", cs.name.0, from.x, from.y);
            radio.say2(
                now,
                format!("Пеленг: точка запуску «{}», {lat:.3} {lon:.3}{}.", cs.name.1, if crew.relay.is_some() { ", і її ретранслятор" } else { "" }),
                format!("A bearing: {}'s launch point, {lat:.3} {lon:.3}{}.", cs.name.0, if crew.relay.is_some() { ", and its relay" } else { "" }),
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn put_up(commands: &mut Commands, models: Option<&KillzoneModels>, crew: usize, kind: Kind, from: Vec3, mark: Mark, class: Class, aim: Vec3, snag_at: Option<f32>) {
    let mut e = commands.spawn((
        TheirFpv { crew, kind, mark, class, aim, vel: Vec3::Y * 4.0, link: Link::Held, domes: 0, snag_at, flown: 0.0, battery: BATTERY },
        // Drawn larger than life: a seven-inch quad is a speck from the drone's height, and the player must see it
        // coming.
        Transform::from_translation(from).with_scale(Vec3::splat(3.0)),
        bevy::light::NotShadowCaster,
    ));
    if let Some(m) = models {
        e.insert(crate::units::skin(&m.fpv));
        crate::units::dress(&mut e, &m.fpv);
    }
}

/// The waiters on the shoulders: one wakes when something of ours on wheels passes within what it sees, and goes for
/// it; its crew lays another there after a while.
#[allow(clippy::too_many_arguments)]
fn wait(
    mut commands: Commands,
    time: Res<Time>,
    mut tally: ResMut<KzTally>,
    seeing: Option<Res<crate::fight::Seeing>>,
    models: Option<Res<KillzoneModels>>,
    quarry: Quarry,
    mut crews: Query<(&mut Crew, &Transform), Without<Arriving>>,
) {
    let Some(spec) = spec() else { return };
    let now = time.elapsed_secs_f64();
    let reach = seeing.as_ref().map_or(WAKE, |s| WAKE.min(s.through(crate::fight::Eye::Man)));
    let mut wheels: Vec<(Mark, Class, Vec2, ())> = quarry.wheels().into_iter().map(|(m, c, p)| (m, c, p, ())).collect();
    by_place(&mut wheels);
    let mut crews: Vec<_> = crews.iter_mut().filter(|(c, _)| c.kind == Kind::Waiter).collect();
    crews.sort_by_key(|(c, _)| c.idx);
    for (mut crew, tf) in crews {
        if now < crew.rearm {
            continue;
        }
        let at = tf.translation.xz();
        let Some(&(mark, class, p, ())) = wheels.iter().filter(|w| w.2.distance(at) < reach).min_by(|a, b| a.2.distance(at).total_cmp(&b.2.distance(at))) else { continue };
        let cs = &spec.crews[crew.idx];
        // Laid again an hour of the day's clock over its rate later.
        crew.rearm = now + (60.0 / (cs.per_hour.max(0.01) * crate::diorama::diorama().compression())) as f64;
        put_up(&mut commands, models.as_deref(), crew.idx, Kind::Waiter, tf.translation + Vec3::Y * 0.3, mark, class, geo::ground(p) + Vec3::Y, None);
        tally.sorties[Kind::Waiter as usize] += 1;
        info!("killzone: a waiter of {} wakes at {:.0},{:.0} for the {} {:.0} m off, at {now:.0} s", cs.name.0, at.x, at.y, class.name(), p.distance(at));
    }
}

/// Their FPVs fly: out at fifty metres, then the dive onto what they are after, steering onto it as it moves. A radio
/// one crossing into a dome of ours loses its link, most times; a fibre one's cable may snag; and one whose link is
/// gone, or whose battery is spent, comes down where it is going.
#[allow(clippy::too_many_arguments)]
fn fly(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<KzRng>,
    mut tally: ResMut<KzTally>,
    mut radio: ResMut<Radio>,
    domes: Query<(&crate::ew::Dome, &Transform), Standing>,
    quarry: Quarry,
    mut fpvs: Query<(Entity, &mut TheirFpv, &mut Transform)>,
    mut impacts: MessageWriter<Impact>,
) {
    let Some(spec) = spec() else { return };
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    let rng = &mut rng.0;
    for (e, mut f, mut tf) in &mut fpvs {
        let mut p = tf.translation;
        if f.link == Link::Held {
            if let Some(at) = quarry.find(f.mark) {
                f.aim = at;
            } else if f.mark != Mark::Team {
                // Gone before it got there: the pilot takes the nearest thing of ours on wheels by it, or goes in on
                // the spot.
                let by = f.aim.xz();
                if let Some((m, c, q)) = quarry.wheels().into_iter().filter(|w| w.2.distance(by) < 150.0).min_by(|a, b| a.2.distance(by).total_cmp(&b.2.distance(by)).then(a.2.x.total_cmp(&b.2.x))) {
                    f.mark = m;
                    f.class = c;
                    f.aim = geo::ground(q) + Vec3::Y;
                }
            }
            if f.kind == Kind::Rf {
                for (d, at) in &domes {
                    let bit = 1u32 << (d.idx % 32);
                    if f.domes & bit == 0 && p.xz().distance(at.translation.xz()) < d.radius {
                        f.domes |= bit;
                        if rng.random_bool(crate::ew::DOME_BREAKS) {
                            f.link = Link::Broken;
                            tally.links_broken += 1;
                            info!("killzone: a dome breaks the link of an FPV of {}, at {now:.0} s", spec.crews[f.crew].name.0);
                            if tally.links_broken <= 2 || tally.links_broken.is_multiple_of(5) {
                                radio.say2(now, "РЕБ зрізав їхню пташку.", "The EW's cut one of their birds.");
                            }
                            break;
                        }
                    }
                }
            }
            if f.link == Link::Held && f.snag_at.is_some_and(|s| f.flown >= s) {
                f.link = Link::Snagged;
                tally.snagged += 1;
                info!("killzone: the cable of an FPV of {} snags, at {now:.0} s", spec.crews[f.crew].name.0);
            }
            f.battery -= dt;
            if f.battery <= 0.0 {
                f.link = Link::Broken;
            }
        }
        if f.link == Link::Held {
            let speed = f.kind.speed();
            let to = f.aim - p;
            let want = if to.xz().length() > DIVE { Vec3::new(f.aim.x, ground(p.xz()).y.max(f.aim.y) + CRUISE, f.aim.z) - p } else { to };
            f.vel = f.vel.lerp(want.normalize_or(Vec3::NEG_Y) * speed, (4.0 * dt).min(1.0));
            if to.length() < speed * dt + 2.0 {
                impacts.write(Impact { at: f.aim, crew: f.crew, mark: Some((f.mark, f.class)) });
                commands.entity(e).despawn();
                continue;
            }
        } else {
            // Blind: it glides on and falls.
            f.vel.y -= 9.81 * dt;
            let decay = 1.0 - 0.3 * dt;
            f.vel.x *= decay;
            f.vel.z *= decay;
        }
        p += f.vel * dt;
        let step = f.vel.length() * dt;
        f.flown += step;
        let floor = ground(p.xz()).y;
        if p.y <= floor + 0.2 {
            impacts.write(Impact { at: Vec3::new(p.x, floor, p.z), crew: f.crew, mark: None });
            commands.entity(e).despawn();
            continue;
        }
        tf.translation = p;
        if f.vel.length_squared() > 0.01 {
            tf.look_to(f.vel.normalize(), Vec3::Y);
        }
    }
}

/// Where their FPVs come down: on what they came for, under a net, or blind into the ground. A strike that goes in
/// burns a car, puts a post's man down, knocks out a dome, or hits our drone team where it was located; on a vehicle
/// of the column game it is a shaped charge from above.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn strike(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<KzRng>,
    mut impacts: MessageReader<Impact>,
    (mut tally, mut radio, mut stats, mut log): (ResMut<KzTally>, ResMut<Radio>, ResMut<Stats>, ResMut<crate::hits::HitLog>),
    (mut fx, mut sound, mut hits): (ResMut<FxQueue>, Sound, MessageWriter<DirectHit>),
    seeing: Option<Res<crate::fight::Seeing>>,
    (mut op, mut view): (ResMut<Operator>, ResMut<View>),
    models: Option<Res<KillzoneModels>>,
    mut nets: Query<&mut Net>,
    mut posts: Query<&mut Post>,
    mut cars: Query<(&mut Car, Option<&mut MeshMaterial3d<VehicleMaterial>>), Without<Skin>>,
    children: Query<&Children>,
    mut skins: Query<(&Skin, &mut MeshMaterial3d<VehicleMaterial>), Without<Car>>,
) {
    let Some(spec) = spec() else { return };
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    // The pilot finds his mark worse in fog.
    let fog = seeing.as_ref().map_or(1.0, |s| (s.through(crate::fight::Eye::Man) / 400.0).clamp(0.5, 1.0)) as f64;
    for imp in impacts.read() {
        let from: &'static str = &spec.crews[imp.crew].name.0;
        fx.explosion(imp.at, 0.6, true);
        fx.flash(imp.at + Vec3::Y, 4.0e6, 40.0, 0.1);
        sound.at(imp.at, Sfx::BlastNear, 1.3);
        let Some((mark, class)) = imp.mark else {
            info!("killzone: an FPV of {from} down blind at {:.0},{:.0}, at {now:.0} s", imp.at.x, imp.at.z);
            continue;
        };
        let at = imp.at.xz();
        let mut p = class.p_hit() * fog;
        let netted = matches!(class, Class::Vehicle | Class::Robot) && {
            match nets.iter_mut().find(|n| n.covers(at)) {
                Some(mut n) => {
                    n.holes += 1;
                    true
                }
                None => false,
            }
        };
        if netted {
            tally.netted += 1;
            p *= NET_STOPS;
        }
        // A car two of them went in on in one frame: the first burnt it.
        if let Mark::Thing(e) = mark
            && cars.get(e).is_ok_and(|(c, _)| c.leg == Leg::Burnt)
        {
            continue;
        }
        // Our drone team is hit only if it is still where they located it.
        let team_moved = mark == Mark::Team && (now < op.moving_until || op.pos.distance(at) > 40.0);
        if team_moved || !rng.random_bool(p) {
            tally.misses += 1;
            if netted {
                tally.net_saved += 1;
                radio.say2(now, "Пташка в сітці! Пронесло.", "A bird in the net! That was close.");
            }
            info!("killzone: an FPV of {from} misses the {} at {:.0},{:.0}{}, at {now:.0} s", class.name(), at.x, at.y, if netted { " (the net)" } else { "" });
            continue;
        }
        tally.hits[Class::ALL.iter().position(|c| *c == class).unwrap_or(0)] += 1;
        info!("killzone: an FPV of {from} hits the {} at {:.0},{:.0}, at {now:.0} s", class.name(), at.x, at.y);
        let Mark::Thing(e) = mark else {
            crate::drones::team_hit(&mut op, &mut view, &mut radio, now);
            continue;
        };
        match class {
            Class::Ew => {
                commands.entity(e).remove::<Prey>().insert(crate::ew::DomeDown);
                log.note(now, "FPV", from, "EW dome", Side::Ua, "knocked out");
                radio.say2(now, "Нашу РЕБку розбили. Купол упав.", "They've hit our EW. The dome's down.");
            }
            Class::Men => {
                if let Ok(mut post) = posts.get_mut(e)
                    && post.strength > 0
                {
                    post.strength -= 1;
                    stats.defender_losses += 1;
                    if let Some(f) = post.figures.pop() {
                        commands.entity(f).despawn();
                    }
                    commands.entity(e).insert(Suppressed { until: now + 30.0 });
                    log.note(now, "FPV", from, "a post", Side::Ua, "one down");
                    radio.say2(now, "FPV по позиції. Є трьохсотий.", "An FPV on the post. One wounded.");
                }
            }
            Class::Vehicle | Class::Robot => {
                if let Ok((mut car, mat)) = cars.get_mut(e) {
                    // One already in the town for good burns there: it got through, and is lost all the same.
                    if car.leg == Leg::Done {
                        tally.cars_through -= 1;
                    }
                    car.leg = Leg::Burnt;
                    tally.cars_lost += 1;
                    stats.defender_losses += rng.random_range(1..=2);
                    commands.entity(e).remove::<Prey>().insert(Emitter::new(EmitterKind::WreckFire, 4.0, 300.0, Vec3::Y));
                    if let (Some(m), Some(mut own)) = (&models, mat) {
                        own.0 = m.car(car.kind)[0].charred.clone();
                        for c in children.get(e).into_iter().flatten() {
                            if let Ok((skin, mut mat)) = skins.get_mut(*c) {
                                mat.0 = skin.0.clone();
                            }
                        }
                    }
                    log.note(now, "FPV", from, &format!("{} (on the road)", car.kind.name()), Side::Ua, "burning");
                    radio.say2(now, format!("{} горить! FPV на дорозі!", car.kind.uk()), format!("The {} is burning! FPVs on the road!", car.kind.name()));
                } else {
                    hits.write(DirectHit { target: e, cause: Cause::Atgm, top: true, weapon: "FPV", from });
                }
            }
            Class::DroneTeam => {}
        }
    }
}

/// A launch point or a relay of theirs: which crew's, where, and whether it is out now.
struct Point {
    e: Entity,
    crew: usize,
    relay: bool,
    at: Vec2,
    struck: Option<f64>,
}

/// Our fire on their launch points and relays (c15 §5, §10; 02 §6): a 155 mm burst or an FPV of ours near one may put
/// it out for a while (`KillzoneSpec::down`), and its crew flies nothing until it is back; what it did not fly in that
/// time is counted. In the Record the day's own strikes go in at their times.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn points(
    mut commands: Commands,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    mut rng: ResMut<PointRng>,
    (mut tally, mut radio, mut log): (ResMut<KzTally>, ResMut<Radio>, ResMut<crate::hits::HitLog>),
    seeing: Option<Res<crate::fight::Seeing>>,
    op: Res<Operator>,
    models: Option<Res<KillzoneModels>>,
    (mut landed, mut blasts): (MessageReader<crate::fire::ShellLanded>, MessageReader<crate::units::Blast>),
    mut crews: Query<(Entity, &mut Crew, &Transform, Option<&Struck>), (Without<Relay>, Without<Arriving>)>,
    relays: Query<(Entity, &Relay, &Transform, Option<&Struck>), Without<Crew>>,
    domes: Query<(Entity, &crate::ew::Dome), (Without<crate::ew::DomeDown>, Without<crate::ew::DomeOff>)>,
    mut done: Local<usize>,
) {
    let Some(spec) = spec() else { return };
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    let per_s = crate::diorama::diorama().compression() / 60.0;
    let mut pts: Vec<Point> = crews
        .iter()
        .filter(|(_, c, _, _)| c.kind != Kind::Waiter)
        .map(|(e, c, t, s)| Point { e, crew: c.idx, relay: false, at: t.translation.xz(), struck: s.map(|s| s.0) })
        .chain(relays.iter().map(|(e, r, t, s)| Point { e, crew: r.crew, relay: true, at: t.translation.xz(), struck: s.map(|s| s.0) }))
        .collect();
    pts.sort_by_key(|p| (p.crew, p.relay));
    // Back up when its time is out.
    for p in pts.iter_mut() {
        if p.struck.is_some_and(|until| now >= until) {
            p.struck = None;
            commands.entity(p.e).remove::<Struck>();
            let name = &spec.crews[p.crew].name;
            info!("killzone: {}'s {} back up, at {now:.0} s", name.0, if p.relay { "relay" } else { "launch point" });
            radio.say2(now, format!("Перехоплення: «{}» знову в ефірі.", name.1), format!("Intercept: {} is back on the air.", name.0));
        }
    }
    // What the crews that are out do not fly.
    for (_, c, _, _) in &crews {
        if c.kind != Kind::Waiter && pts.iter().any(|p| p.crew == c.idx && p.struck.is_some()) {
            tally.lost += spec.crews[c.idx].per_hour * per_s * tempo(c.kind, seeing.as_deref()) * dt;
        }
    }
    // Our bursts this frame: the guns' (theirs land on our side), and our FPVs going in.
    let mut bursts: Vec<(Vec2, f32, f64, &'static str)> = landed.read().filter(|l| l.by == Side::Ua && l.radius > 0.0).map(|l| (l.at, l.radius * SHELL_REACH, SHELL_TAKES, "155 mm")).collect();
    bursts.extend(blasts.read().filter(|b| b.cause == Cause::Fpv).map(|b| (b.at.xz(), FPV_REACH, FPV_TAKES, "FPV")));
    let down = (spec.down * 60.0 / crate::diorama::diorama().compression()) as f64;
    for (at, reach, takes, weapon) in bursts {
        for p in pts.iter_mut().filter(|p| p.struck.is_none() && p.at.distance(at) < reach) {
            tally.strikes += 1;
            let name = &spec.crews[p.crew].name;
            let what = if p.relay { "relay" } else { "launch point" };
            if !rng.0.random_bool(takes) {
                info!("killzone: a {weapon} burst {:.0} m from {}'s {what}, no effect, at {now:.0} s", p.at.distance(at), name.0);
                continue;
            }
            tally.struck += 1;
            p.struck = Some(now + down);
            commands.entity(p.e).insert((Struck(now + down), Located));
            if !p.relay
                && let Ok((_, mut c, _, _)) = crews.get_mut(p.e)
            {
                // The drones it had ready are gone with it.
                c.charge = 0.0;
                c.road = 0.0;
            }
            info!("killzone: {}'s {what} out by a {weapon} burst {:.0} m off, for {down:.0} s, at {now:.0} s", name.0, p.at.distance(at));
            log.note(now, weapon, "our fire", &format!("{}'s {what}", name.0), Side::Ru, "out of action");
            radio.say2(
                now,
                format!("Є влучання по {} «{}». Якийсь час буде тихо.", if p.relay { "ретранслятору" } else { "точці запуску" }, name.1),
                format!("A hit on {}'s {what}. That'll keep them quiet for a while.", name.0),
            );
        }
    }
    // The Record's strikes, each at its time.
    while *done < spec.strikes.len() && script.t >= spec.strikes[*done].at {
        let s = spec.strikes[*done];
        *done += 1;
        if play() != Play::Record {
            continue;
        }
        let Some(cs) = spec.crews.get(s.crew) else { continue };
        let Some(target) = (if s.relay { cs.relay } else { Some(cs.at) }).map(|(x, z)| Vec2::new(x, z)) else { continue };
        let what = if s.relay { "relay" } else { "launch point" };
        match s.by {
            StrikeBy::Guns => {
                commands.spawn(crate::fire::Mission { target, kind: crate::fire::ShellKind::He155, rounds: 4, sigma: 20.0, interval: 3.0, next: now, flight: 20.0, by: Side::Ua });
                radio.say2(now, format!("Вогонь по точці «{}», чотири снаряди.", cs.name.1), format!("Fire mission on {}'s {what}, four rounds.", cs.name.0));
            }
            StrikeBy::Fpv => {
                let from = geo::ground(op.pos) + Vec3::Y * 1.5;
                let flight = from.xz().distance(target) / Kind::Rf.speed();
                let mut e = commands.spawn((OurStrike { aim: ground(target) + Vec3::Y, vel: Vec3::Y * 4.0, battery: BATTERY, lost_for: 0.0 }, Transform::from_translation(from).with_scale(Vec3::splat(3.0)), bevy::light::NotShadowCaster));
                if let Some(m) = &models {
                    e.insert(crate::units::skin(&m.fpv));
                    crate::units::dress(&mut e, &m.fpv);
                }
                // Our domes pause while it climbs out of them (02 §6).
                let mut paused: Vec<(usize, Entity)> = domes.iter().map(|(e, d)| (d.idx, e)).collect();
                paused.sort_by_key(|d| d.0);
                for (_, d) in &paused {
                    crate::ew::pause(&mut commands, *d, now + 40.0);
                }
                radio.say2(
                    now,
                    format!("FPV пішов на {} «{}»{}.", if s.relay { "ретранслятор" } else { "точку" }, cs.name.1, if paused.is_empty() { "" } else { ", РЕБ на паузі" }),
                    format!("An FPV away at {}'s {what}{}.", cs.name.0, if paused.is_empty() { "" } else { ", the EW paused" }),
                );
                info!("killzone: an FPV of ours at {}'s {what}, {:.0} s out, at {now:.0} s", cs.name.0, flight);
            }
        }
        info!("killzone: our strike on {}'s {what} at {:.0},{:.0}, at {now:.0} s", cs.name.0, target.x, target.y);
    }
}

/// Our FPVs at their points fly as theirs do: out at fifty metres, the dive, and a burst where they go in (`points`
/// reads it). Inside one of our own domes the link goes and it falls.
fn our_strikes(mut commands: Commands, time: Res<Time>, ew: Res<crate::ew::Ew>, mut fx: ResMut<FxQueue>, mut sound: Sound, mut blasts: MessageWriter<crate::units::Blast>, mut q: Query<(Entity, &mut OurStrike, &mut Transform)>) {
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    for (e, mut f, mut tf) in &mut q {
        let mut p = tf.translation;
        f.battery -= dt;
        f.lost_for = if ew.ours_at(p) > 0.5 { f.lost_for + dt } else { 0.0 };
        if f.battery > 0.0 && f.lost_for < 0.8 {
            let speed = Kind::Rf.speed();
            let to = f.aim - p;
            let want = if to.xz().length() > DIVE { Vec3::new(f.aim.x, ground(p.xz()).y.max(f.aim.y) + CRUISE, f.aim.z) - p } else { to };
            f.vel = f.vel.lerp(want.normalize_or(Vec3::NEG_Y) * speed, (4.0 * dt).min(1.0));
            if to.length() < speed * dt + 2.0 {
                p = f.aim;
                f.vel = Vec3::NEG_Y;
            }
        } else {
            f.vel.y -= 9.81 * dt;
            f.vel *= 1.0 - 0.3 * dt;
        }
        p += f.vel * dt;
        let floor = ground(p.xz()).y;
        if p.y <= floor + 0.3 || p.distance(f.aim) < 1.0 {
            let at = Vec3::new(p.x, floor, p.z);
            commands.entity(e).despawn();
            fx.explosion(at, 0.7, true);
            sound.at(at, Sfx::BlastNear, 1.3);
            blasts.write(crate::units::Blast { at, radius: 6.0, cause: Cause::Fpv });
            info!("killzone: our FPV goes in at {:.0},{:.0}, {:.0} m from its point, at {now:.0} s", at.x, at.z, at.xz().distance(f.aim.xz()));
            continue;
        }
        tf.translation = p;
        if f.vel.length_squared() > 0.01 {
            tf.look_to(f.vel.normalize(), Vec3::Y);
        }
    }
}

/// For the bands (`BAV_LOG_STATS`): the kill zone's count once a game minute.
fn log_tally(script: Res<crate::director::Script>, tally: Res<KzTally>, mut last: Local<Option<u32>>) {
    if !crate::dev::dev().log_stats {
        return;
    }
    let minute = script.t as u32 / 60;
    if *last == Some(minute) {
        return;
    }
    *last = Some(minute);
    let t = &*tally;
    info!(
        "killzone: {} sorties={}/{}/{} hits team={} ew={} vehicles={} robots={} men={} misses={} broken={} snagged={} netted={}/{} cars={}/{}/{} points={}/{} lost={:.1}",
        crate::director::clock_text(script.t),
        t.sorties[0],
        t.sorties[1],
        t.sorties[2],
        t.hits[0],
        t.hits[1],
        t.hits[2],
        t.hits[3],
        t.hits[4],
        t.misses,
        t.links_broken,
        t.snagged,
        t.netted,
        t.net_saved,
        t.cars_out,
        t.cars_through,
        t.cars_lost,
        t.strikes,
        t.struck,
        t.lost
    );
}

impl KillzoneModels {
    pub fn car(&self, k: CarKind) -> &[Part] {
        match k {
            CarKind::L200 => &self.l200,
            CarKind::Lada => &self.lada,
        }
    }
}
