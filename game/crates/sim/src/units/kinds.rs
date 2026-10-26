//! The kinds: vehicles and their states, drivers, soldiers, the tally, the net, the models and their paint.

use super::*;

#[derive(Resource)]
pub struct UnitRng(pub SmallRng);

/// Where the last direct hit landed, for test shots (`BAV_LOOK=hit`).
#[derive(Resource, Default)]
pub struct LastHit(pub Option<Vec3>);

pub use bav_core::diorama::Side;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, strum::VariantArray, strum::EnumString, serde::Deserialize)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum VKind {
    /// T-80BV
    T80,
    /// T-72B3
    T72,
    /// BMP-1AM
    Bmp1,
    /// BMP-2
    Bmp2,
    /// MT-LB
    Mtlb,
    /// Leopard 2A6 (the 33rd Brigade's, on 8 June)
    Leo2a6,
    /// Leopard 2R mine-clearing tank (Finnish, a full-width plow)
    Leo2r,
    /// Wisent 1 engineer vehicle (Leopard 1 hull, dozer-plow, crane)
    Wisent,
    /// BMR-2 mine-clearing vehicle (T-55 hull, KMT-7 rollers)
    Bmr2,
    /// M2A2 ODS Bradley
    Bradley,
    /// T-90M Proryv: Relikt reactive armour, a carousel, the best tank Russia had at Avdiivka
    T90m,
    /// A ground robot (НРК): a tracked cart of about 500 kg at 10 km/h with 20 km of range, driven over a radio
    /// link, no crew (the Zmiy, c15 §5); it carries cargo in and the wounded out (`robots`).
    Ugv,
}

impl VKind {
    pub fn is_tank(self) -> bool {
        matches!(self, VKind::T80 | VKind::T72 | VKind::Leo2a6 | VKind::T90m)
    }
    /// Explosive reactive armour on the hull and turret front: Kontakt-1 on the T-80BV, Kontakt-5 on the
    /// T-72B3, the BRAT tiles on the 47th's Bradleys (M2A2 ODS-SA).
    pub fn era(self) -> bool {
        matches!(self, VKind::T80 | VKind::T72 | VKind::Bradley | VKind::T90m)
    }
    /// T-72/T-80 carousel autoloaders: when the charges go, the turret goes up.
    pub fn carousel(self) -> bool {
        matches!(self, VKind::T80 | VKind::T72 | VKind::T90m)
    }
    /// Built for crew survival (blow-out panels, spall liners, escape hatches and ramps): most crews get out.
    pub fn western(self) -> bool {
        matches!(self, VKind::Leo2a6 | VKind::Leo2r | VKind::Wisent | VKind::Bradley)
    }
    /// The marks its tracks leave: the distance between their centres and each track's width, in metres.
    pub fn tracks(self) -> (f32, f32) {
        match self {
            VKind::T80 | VKind::T72 | VKind::T90m => (2.79, 0.58),
            VKind::Leo2a6 | VKind::Leo2r => (2.79, 0.64),
            VKind::Wisent => (2.7, 0.55),
            VKind::Bmr2 => (2.64, 0.58),
            VKind::Bmp1 | VKind::Bmp2 => (2.65, 0.3),
            VKind::Mtlb => (2.5, 0.35),
            VKind::Bradley => (2.75, 0.53),
            VKind::Ugv => (0.95, 0.2),
        }
    }
    /// Mine-clearing vehicles: no gun worth firing, a plow or rollers in front.
    pub fn breacher(self) -> bool {
        matches!(self, VKind::Leo2r | VKind::Wisent | VKind::Bmr2)
    }
    /// A gun its crew fires: not a breacher's, not a robot's.
    pub fn armed(self) -> bool {
        !self.breacher() && self != VKind::Ugv
    }
    pub fn crew(self) -> u32 {
        match self {
            VKind::Ugv => 0,
            VKind::Mtlb | VKind::Bmr2 => 2,
            VKind::Leo2a6 => 4,
            _ => 3,
        }
    }
    pub fn troops(self) -> u32 {
        match self {
            VKind::T80 | VKind::T72 | VKind::T90m | VKind::Leo2a6 | VKind::Leo2r | VKind::Wisent | VKind::Bmr2 | VKind::Ugv => 0,
            VKind::Bmp1 | VKind::Bmp2 | VKind::Bradley => 6,
            VKind::Mtlb => 8,
        }
    }
    /// Chance a mine under a track destroys the vehicle outright, rather than only tearing off the track.
    pub(super) fn mine_kill(self, stacked: bool) -> f64 {
        let base = match self {
            VKind::T80 | VKind::T72 | VKind::T90m => 0.3,
            VKind::Leo2a6 => 0.1,
            VKind::Bradley => 0.25,
            VKind::Leo2r | VKind::Wisent | VKind::Bmr2 => 0.12,
            VKind::Ugv => 0.9,
            _ => 0.5,
        };
        if stacked { (base + 0.3f64).min(0.8) } else { base }
    }
    /// Chance a guided missile destroys rather than disables.
    pub(super) fn hit_kill(self) -> f64 {
        match self {
            // Kontakt-1 on the glacis and turret front saves some; side and top hits mostly don't.
            VKind::T80 | VKind::T72 => 0.72,
            // Relikt on the front and sides, and the glacis sloped: more of what hits it is thrown.
            VKind::T90m => 0.55,
            VKind::Leo2a6 => 0.3,
            VKind::Bradley => 0.55,
            VKind::Leo2r | VKind::Wisent | VKind::Bmr2 => 0.5,
            VKind::Ugv => 0.85,
            _ => 0.92,
        }
    }
    /// Everyone aboard: crew and dismounts.
    pub fn men(self) -> u32 {
        self.crew() + self.troops()
    }
    pub fn speed(self) -> f32 {
        match self {
            // A plow digging in moves at a walking pace.
            VKind::Leo2r | VKind::Wisent | VKind::Bmr2 => 2.4,
            // 10 km/h loaded (c15 §5).
            VKind::Ugv => 2.8,
            VKind::Leo2a6 => 5.4,
            VKind::Bradley => 6.0,
            _ if self.is_tank() => 5.2,
            _ => 5.8,
        }
    }
    /// Pulling away (m/s², from a standstill on firm ground): 46 tonnes on 840 hp take their time; a Leopard's
    /// 1500 hp and a Bradley's power-to-weight less so.
    pub(super) fn accel(self) -> f32 {
        match self {
            VKind::Leo2a6 | VKind::Bradley => 1.3,
            VKind::Leo2r | VKind::Wisent | VKind::Bmr2 => 0.6,
            VKind::T80 => 1.0,
            VKind::Ugv => 0.7,
            _ if self.is_tank() => 0.8,
            _ => 1.0,
        }
    }
    /// Backing up: a T-72's one reverse gear is a walking pace (4 km/h), a T-80's a little better; a Leopard
    /// reverses nearly as fast as it goes forward.
    pub(super) fn reverse_speed(self) -> f32 {
        match self {
            VKind::T72 | VKind::T90m | VKind::Bmr2 => 1.4,
            VKind::T80 => 3.0,
            VKind::Leo2a6 | VKind::Leo2r => 6.0,
            VKind::Bradley => 4.5,
            VKind::Ugv => 2.5,
            _ => 2.8,
        }
    }
    /// The hull's turn rate on the spot (rad/s): a Leopard or a Bradley turns on its own axis (neutral steer); a
    /// Soviet hull pivots on one braked track, more slowly.
    pub(super) fn pivot_rate(self) -> f32 {
        match self {
            VKind::Leo2a6 | VKind::Leo2r | VKind::Wisent | VKind::Bradley => 0.55,
            VKind::Ugv => 0.8,
            VKind::T80 | VKind::T72 | VKind::T90m | VKind::Bmr2 => 0.38,
            _ => 0.45,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VState {
    Advancing,
    Halted,
    Deploying,
    Dismounting,
    Overwatch,
    Withdrawing,
    Immobilized,
    Destroyed,
    Abandoned,
}

/// Knocked out while moving: it grinds to a halt at `decel`, slewing round the dead track at `slew`, and then
/// stays where it stopped. A vehicle wears it from its knock-out on, and `coast` tends it instead of the driving.
#[derive(Component, Clone, Copy, Debug)]
pub struct Coasting {
    pub decel: f32,
    pub slew: f32,
}

/// The main armament as the gunner works it: when it fires next, what it is laid on, and where the turret has
/// got to in traversing onto it. Breachers carry none.
#[derive(Component)]
pub struct Gun {
    pub fire_timer: f32,
    pub aim: Option<Vec2>,
    pub turret_yaw: f32,
}

impl Vehicle {
    /// Knocked out of action but not destroyed: it coasts to a stop at `decel`, slewing at `slew`. The caller puts
    /// the `Coasting` on the entity.
    #[must_use]
    pub(crate) fn immobilise(&mut self, decel: f32, slew: f32) -> Coasting {
        self.state = VState::Immobilized;
        self.timer = 0.0;
        Coasting { decel, slew }
    }

    /// What the crew of a knocked-out vehicle does now: a fit crew is out in 5–15 s, a concussed one (a mine)
    /// in 20–60 s; a tank crew may stay a few minutes and fight from the dead track, turning the hull to face
    /// the enemy; and once in a long while nobody is at the tiller any more and the hull drives a slow circle
    /// on its one track until it stalls (the T-80BV of October 2022).
    pub(super) fn crew_plan(&mut self, rng: &mut SmallRng, cause: Cause) {
        self.timer = 0.0;
        let at = if cause == Cause::Mine { rng.random_range(20.0..60.0) } else { rng.random_range(5.0..15.0) };
        self.plan = CrewPlan::Bail { at };
        if self.kind.is_tank() && self.crew > 0 {
            if cause == Cause::Mine && rng.random_bool(1.0 / 15.0) {
                let secs = rng.random_range(20.0..40.0);
                self.plan = CrewPlan::Ghost { secs, at: secs + 10.0 };
            } else if rng.random_bool(0.3) {
                self.plan = CrewPlan::FightFromTrack { at: rng.random_range(120.0..300.0) };
            }
        }
    }
}

/// Which way the hull goes along its route: forward, or back down it, either hull first (it reversed out of the
/// worst) or turned round. A hull hit while backing keeps backing as it coasts, whatever state it is in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gear {
    Forward,
    Backing,
    TurnedRound,
}

impl Gear {
    /// Going back down the route, whichever way the hull points.
    pub fn reversing(self) -> bool {
        self != Gear::Forward
    }
}

/// What the crew of a knocked-out vehicle does (`Vehicle::crew_plan`), with the vehicle's `timer` counting from
/// the hit.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CrewPlan {
    /// Out of the hatches once the timer passes `at`.
    Bail { at: f32 },
    /// Nobody at the tiller and the engine alive: the hull circles on its one track for `secs`, then whoever is
    /// left gets out at `at`.
    Ghost { secs: f32, at: f32 },
    /// A live crew on a dead track: it turns the hull to face the enemy and fights from it until `at`.
    FightFromTrack { at: f32 },
}

impl CrewPlan {
    pub fn bail_at(self) -> f32 {
        match self {
            CrewPlan::Bail { at } | CrewPlan::Ghost { at, .. } | CrewPlan::FightFromTrack { at } => at,
        }
    }
}

/// A halt on the approach: planned for an odometer reading (a driver who has lost the one ahead in the exhaust
/// smoke, or waits for an order), then held for its seconds.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Halt {
    Planned { at: f32, secs: f32 },
    Holding { secs: f32 },
}

impl Halt {
    pub fn secs(self) -> f32 {
        match self {
            Halt::Planned { secs, .. } | Halt::Holding { secs } => secs,
        }
    }
}

/// Sent back in for the men: it stops at the wrecks, waits for them to climb on, and comes out again.
#[derive(Component)]
pub struct Rescue;

/// A plow (or rollers) on the front: it turns the mines out of the lane, until one takes it off.
#[derive(Component)]
pub struct Plow;

/// Its sights shot out (25 mm, `Cause::Autocannon` or `Cause::Blinding`): the crew can see nothing to fire at, and
/// the turret swings loose while the hull drives blind (v02 §1: "backed away with its turret spinning").
#[derive(Component)]
pub struct Blinded;

/// A column of the script's that stops at a place of its own on its route and stays there a while before its crews
/// think of going back (part III's T-90M in Stepove's east end, firing on the trench): where it deploys, and for how
/// many seconds it holds its overwatch at least.
#[derive(Component, Clone, Copy)]
pub struct Stay {
    pub deploy_s: f32,
    pub secs: f32,
}

impl VState {
    pub fn alive(self) -> bool {
        !matches!(self, VState::Destroyed | VState::Abandoned)
    }
    pub fn mobile(self) -> bool {
        !matches!(self, VState::Destroyed | VState::Abandoned | VState::Immobilized)
    }
}

/// A vehicle: what it is, whose, what state it is in, and who is aboard. What its driver does with it is the
/// `Driver` beside it, and how it rides the ground the `Suspension`.
#[derive(Component)]
pub struct Vehicle {
    pub kind: VKind,
    pub side: Side,
    pub state: VState,
    pub wave: u32,
    pub idx: u32,
    pub crew: u32,
    pub troops: u32,
    /// Seconds in the state it is in.
    pub timer: f32,
    pub panic: f32,
    pub turret: Entity,
    /// The gun's muzzle in the turret's frame.
    pub muzzle: Vec3,
    /// Knocked out: what the crew does, and when it gets out.
    pub plan: CrewPlan,
    /// Seconds until the next man climbs out of a knocked-out vehicle.
    pub exit_timer: f32,
}

/// The driver's hand on a vehicle: where it is on its route, where he means to be, how fast it goes and which
/// way the hull points, and his own habits. The driving systems work it; a knocked-out hull keeps it for `coast`.
#[derive(Component)]
pub struct Driver {
    pub route: usize,
    pub s: f32,
    pub lat: f32,
    pub target_lat: f32,
    pub speed: f32,
    pub gear: Gear,
    /// Hull heading (rate-limited: a tank turns, it does not snap).
    pub yaw: f32,
    /// How fast the hull is turning now (rad/s): the tracks slip, so it follows the tiller a moment late.
    pub yaw_rate: f32,
    /// The driver's hand on the tiller: the turn rate he is asking for, nothing while he lets it run straight.
    pub steer: f32,
    /// This frame's acceleration along the way it is going (m/s²), and how fast the ground let it go (`going`,
    /// 0..1): the suspension and the track marks answer to both.
    pub accel: f32,
    pub going: f32,
    /// This driver's own pace, as a fraction of the kind's speed: a column is never one speed.
    pub pace: f32,
    /// How many seconds behind the truth this driver's idea of the vehicle ahead runs.
    pub lag: f32,
    /// The speed of the vehicle ahead, as the driver has caught up with it.
    pub lead_speed: f32,
    /// Metres driven in all, and the paint odometer, reset every 2 m.
    pub odometer: f32,
    pub travelled: f32,
    /// A halt planned on the approach, and then held.
    pub halt: Option<Halt>,
    /// Seconds left of a driver's hesitation (a look round, a word on the intercom, a gear missed).
    pub hesitate: f32,
    /// Seconds left of driving on straight past a hull he has swung away from, before he steers for his line.
    pub detour: f32,
    /// This driver's own habits: how far off his line he lets the hull wander before he corrects (rad), the gap
    /// he likes to the vehicle ahead (m), and the phase of his drift in pace and line.
    pub slack: f32,
    pub gap: f32,
    pub quirk: f32,
    /// A Column's pace: its slowest vehicle's speed, which none of it outruns (unbounded outside a Column).
    pub column_pace: f32,
}

impl Driver {
    /// Which way the hull is moving: its nose, or its tail while it backs up.
    pub fn travel(&self) -> Vec2 {
        let fwd = Vec2::new(-self.yaw.sin(), -self.yaw.cos());
        if self.backing() { -fwd } else { fwd }
    }

    /// Reversing hull first rather than turned round.
    pub fn backing(&self) -> bool {
        self.gear == Gear::Backing
    }

    /// Which way along the route it is going: +1 up it, -1 back down it.
    pub fn dir(&self) -> f32 {
        if self.gear.reversing() { -1.0 } else { 1.0 }
    }
}

/// The hull on its springs: pitch and roll and their rates, a damped spring (`suspension`); a hit or a mine kicks
/// the rates.
#[derive(Component, Default)]
pub struct Suspension {
    pub pitch: f32,
    pub pitch_v: f32,
    pub roll: f32,
    pub roll_v: f32,
}

/// The driving stream: drivers' habits and hesitations, apart from `UnitRng` so the column's looseness does not
/// reshuffle every other draw of the battle.
#[derive(Resource)]
pub struct DriveRng(pub SmallRng);

#[derive(Component)]
pub struct Turret;

/// A soldier riding on a hull, until the vehicle stops to dismount or is hit.
#[derive(Component)]
pub struct Rider;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SState {
    Advance,
    Prone,
    Flee,
    DugIn,
    /// Hit, alive, unable to walk: crawling towards the nearest vehicle or back, until someone carries him out.
    Wounded,
}

#[derive(Component)]
pub struct Soldier {
    pub side: Side,
    pub state: SState,
    pub goal: Vec2,
    pub home: Vec2,
    pub speed: f32,
    pub fear: f32,
    pub timer: f32,
    pub phase: f32,
    /// Seconds of sprint left: a man who has just got out of a vehicle runs flat out for a while, then jogs.
    pub sprint: f32,
    /// The first place he runs to, before anywhere else: the nearest crater, wreck or tree line.
    pub leg: Option<Vec2>,
    /// The wounded man he is dragging back with him.
    pub carrying: Option<Entity>,
}

#[derive(Component)]
pub struct Leg(pub f32);

/// A turret thrown off by an ammunition cook-off, or other debris in flight.
#[derive(Component)]
pub struct Debris {
    pub vel: Vec3,
    pub spin: Vec3,
}

#[derive(Resource, Default)]
pub struct Stats {
    /// The attackers' men sent forward, those who came back, and their dead.
    pub sent: u32,
    pub returned: u32,
    pub attacker_dead: u32,
    pub vehicles_committed: u32,
    pub destroyed: u32,
    pub abandoned: u32,
    pub withdrawn: u32,
    /// The defenders' losses: men in the positions, and their vehicles.
    pub defender_losses: u32,
    pub defender_vehicles: u32,
    /// Men picked up in the field by a vehicle and carried out.
    pub picked_up: u32,
    /// The attackers' dead by what killed them (for the development log).
    pub deaths: Vec<(&'static str, u32)>,
    /// Robotyne: losses in our own overwatch positions, not among the men sent forward.
    pub overwatch_losses: u32,
    /// The attackers' wounded, and how many of them were carried or crawled out.
    pub wounded: u32,
    pub wounded_out: u32,
    pub turret_tosses: u32,
    /// Shots the ATGM teams did not take because a belt or a crest hid the target.
    pub hidden: u32,
    pub mine_hits: u32,
    pub atgm_kills: u32,
    /// The player's FPVs that went in, and the vehicles they killed.
    pub fpv_strikes: u32,
    pub fpv_kills: u32,
    pub artillery_kills: u32,
    /// Attackers alive at or past the defenders' forward line (a foothold).
    pub foothold: u32,
    /// The furthest any attacker on foot got, and any vehicle, as z along the direction of attack.
    pub max_advance: f32,
    pub vehicle_advance: f32,
    /// By column (index 0 unused: columns count from 1).
    pub per_wave: Vec<WaveTally>,
}

#[derive(Clone, Copy, Default)]
pub struct WaveTally {
    pub vehicles: u32,
    pub destroyed: u32,
    pub abandoned: u32,
}

impl Stats {
    pub fn died(&mut self, why: &'static str, n: u32) {
        match self.deaths.iter_mut().find(|(w, _)| *w == why) {
            Some((_, k)) => *k += n,
            None => self.deaths.push((why, n)),
        }
    }

    pub fn wave(&mut self, w: u32) -> &mut WaveTally {
        if self.per_wave.len() <= w as usize {
            self.per_wave.resize(w as usize + 1, WaveTally::default());
        }
        &mut self.per_wave[w as usize]
    }
}

/// The player's order to the attacking column (Robotyne): push on down the lanes, hold where you are, or pull
/// back to the start line, picking up the men on the way.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub enum ColumnOrder {
    #[default]
    Push,
    Hold,
    PullBack,
}

/// One thing said on the net: in Ukrainian, as it is said, and in English, as the player reads it.
#[derive(Clone, Debug)]
pub struct Line {
    pub uk: String,
    pub en: String,
}

impl Line {
    /// A line the string table knows (`text::uk`); one it does not is said in English alone.
    pub fn new(en: impl Into<String>) -> Self {
        let (uk, en) = crate::text::net(&en.into());
        Self { uk, en }
    }

    /// Both tongues given; their callsigns (`{player}`, `{guns}`, ...) are filled in.
    pub fn both(uk: impl Into<String>, en: impl Into<String>) -> Self {
        Self { uk: crate::text::fill(&uk.into(), true), en: crate::text::fill(&en.into(), false) }
    }
}

#[derive(Resource, Default)]
pub struct Radio {
    pub lines: Vec<(f64, Line)>,
}

impl Radio {
    /// Say a line the string table knows (or an English-only one, such as a dev message).
    pub fn say(&mut self, now: f64, s: impl Into<String>) {
        self.lines.push((now, Line::new(s)));
    }

    /// Say a line built on the spot, in both tongues.
    pub fn say2(&mut self, now: f64, uk: impl Into<String>, en: impl Into<String>) {
        self.lines.push((now, Line::both(uk, en)));
    }
}

/// One kind of vehicle: its hull (and plow-tank hull), its turret, where the turret sits and where its gun ends.
/// One mesh of a baked model with its material, and the material it takes when the vehicle burns.
#[derive(Clone)]
pub struct Part {
    pub mesh: Handle<Mesh>,
    pub material: Handle<VehicleMaterial>,
    pub charred: Handle<VehicleMaterial>,
}

/// How the look paints a vehicle over its model's texture: camouflage, mud, dust and wear, or (burned out) rust
/// and soot, worked out in the hull's own frame by its vehicle shader (`bav_look::baked`). It lives here because
/// the battle holds the handles; nothing in it is read by the battle. Bindless like the standard material it
/// extends, so the vehicles still draw in a few batches (as plain uniforms each material was a draw of its own,
/// and the frame paid for it).
#[derive(Asset, bevy::render::render_resource::AsBindGroup, bevy::render::render_resource::ShaderType, Reflect, Debug, Clone, Default)]
#[data(50, Livery, binding_array(101))]
#[bindless(index_table(range(50..51), binding(100)))]
pub struct Livery {
    /// x: the scheme (0 Russian green, 1 NATO three-tone, 2 Ukrainian green, 3 an aircraft's own paint),
    /// y: whitewash, z: 1 in June, w: a seed.
    pub scheme: Vec4,
    /// x: the hull's half width, y: its deck height, z: how high this mesh's frame sits in the hull's (a turret's
    /// ring), w: 1 burned out.
    pub frame: Vec4,
    /// x: what the material is (0 paint, 1 bare steel and rubber, 2 lamps, glass and markings), y: its texture's
    /// mean luminance (0 untextured), z: 1 for the hand-built, vertex-coloured parts.
    pub stuff: Vec4,
}

impl From<&Livery> for Livery {
    fn from(l: &Livery) -> Self {
        l.clone()
    }
}

impl bevy::pbr::MaterialExtension for Livery {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "embedded://bav_look/shaders/vehicle.wgsl".into()
    }
}

pub type VehicleMaterial = bevy::pbr::ExtendedMaterial<StandardMaterial, Livery>;

/// Shared materials for vertex-coloured models.
#[derive(Resource, Clone)]
pub struct Palette {
    pub vertex: Handle<StandardMaterial>,
    pub charred: Handle<StandardMaterial>,
    pub branches: Handle<StandardMaterial>,
}

pub struct VehicleMeshes {
    pub hull: Vec<Part>,
    pub plow_hull: Option<Vec<Part>>,
    pub turret: Vec<Part>,
    pub turret_at: Vec3,
    /// In the turret's frame.
    pub muzzle: Vec3,
}

/// A mesh worn by a vehicle, holding the material it takes when the vehicle burns.
#[derive(Component)]
pub struct Skin(pub Handle<VehicleMaterial>);

/// The first of a model's meshes, worn by the entity itself (as every model was one mesh before the baked ones,
/// and the day's order of play hangs on which entities carry a mesh: see `bav_core::schedule::Step`).
pub fn skin(parts: &[Part]) -> (Mesh3d, MeshMaterial3d<VehicleMaterial>) {
    (Mesh3d(parts[0].mesh.clone()), MeshMaterial3d(parts[0].material.clone()))
}

/// ...burned out.
pub fn burnt(parts: &[Part]) -> (Mesh3d, MeshMaterial3d<VehicleMaterial>) {
    (Mesh3d(parts[0].mesh.clone()), MeshMaterial3d(parts[0].charred.clone()))
}

/// The rest of a model's meshes, as children that remember how they burn.
pub fn dress(e: &mut EntityCommands, parts: &[Part]) {
    for p in &parts[1..] {
        e.with_child((Mesh3d(p.mesh.clone()), MeshMaterial3d(p.material.clone()), Skin(p.charred.clone())));
    }
}

/// ...burned out from the start.
pub fn dress_charred(e: &mut EntityCommands, parts: &[Part]) {
    for p in &parts[1..] {
        e.with_child((Mesh3d(p.mesh.clone()), MeshMaterial3d(p.charred.clone())));
    }
}

/// Sets a burning vehicle's (or turret's) meshes to their charred materials: its own, and its dressed children's.
pub fn char_skins(e: Entity, own: &mut MeshMaterial3d<VehicleMaterial>, parts: &[Part], children: &Query<&Children>, skins: &mut Query<(&Skin, &mut MeshMaterial3d<VehicleMaterial>), Without<Vehicle>>) {
    own.0 = parts[0].charred.clone();
    for c in children.get(e).into_iter().flatten() {
        if let Ok((skin, mut mat)) = skins.get_mut(*c) {
            mat.0 = skin.0.clone();
        }
    }
}

/// One side's soldiers: standing body, a leg (mirrored for the other), and lying dead.
pub struct FigureMeshes {
    pub body: Handle<Mesh>,
    pub leg: Handle<Mesh>,
    pub fallen: Handle<Mesh>,
}

#[derive(Resource)]
pub struct Models {
    pub vehicles: HashMap<VKind, VehicleMeshes>,
    pub ru: FigureMeshes,
    pub ua: FigureMeshes,
    pub hip: Vec3,
    pub raam: Handle<Mesh>,
    pub missile: Handle<Mesh>,
    pub ua_tank: VehicleMeshes,
    /// A lane marker: a stake with an orange flag, which the Leopard 2R drops along the lane it clears.
    pub marker: Handle<Mesh>,
    /// What a mine leaves on the ground: the cut track lying out behind, and a road wheel.
    pub track_strip: Handle<Mesh>,
    pub road_wheel: Handle<Mesh>,
}

impl Models {
    pub fn vehicle(&self, k: VKind) -> &VehicleMeshes {
        &self.vehicles[&k]
    }

    pub fn figure(&self, side: Side) -> &FigureMeshes {
        match side {
            Side::Ru => &self.ru,
            Side::Ua => &self.ua,
        }
    }
}
