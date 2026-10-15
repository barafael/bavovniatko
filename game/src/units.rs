//! Vehicles, soldiers, mines, and what happens when steel and explosive meet.
//!
//! The attacking columns behave the same in both dioramas, because the minefield does the same thing to them:
//! single file down a known track behind a plow, the lead stops, the ones behind bunch up, drivers who go round
//! leave the cleared lane, and the wrecks block the track for whoever comes next. At Vuhledar the Russian columns
//! do it (dossier c08 §5); south of Mala Tokmachka the 47th Brigade's do (c09 §5), in better vehicles, and most of
//! their crews get out.

use crate::audio::{self, Sfx, Sound};
use crate::diorama::{Diorama, diorama};
use crate::camera::MainCamera;
use crate::fx::{self, Emitter, EmitterKind, FxAssets, FxRng};
use crate::geo::{self, Polyline};
use crate::meshes::{self, WEST_GREEN};
use crate::terrain::{Paint, PaintQueue};
use crate::baked::{self, Part};
use crate::world::Palette;
use bevy::{light::NotShadowCaster, platform::collections::HashMap, prelude::*};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use strum::VariantArray;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Blast>()
            .add_message::<DirectHit>()
            .insert_resource(UnitRng(SmallRng::seed_from_u64(crate::dev::seeded(155))))
            .insert_resource(Stats { vehicle_advance: f32::MIN, max_advance: if diorama() == Diorama::Robotyne { f32::MIN } else { 0.0 }, ..default() })
            .init_resource::<Radio>()
            .init_resource::<ColumnOrder>()
            .init_resource::<LastHit>()
            .add_systems(Startup, (setup_models, lay_minefields, spawn_old_wrecks).chain().after(crate::world::spawn_belts))
            .add_systems(
                Update,
                (drive_vehicles, coast, check_mines, resolve_blasts, resolve_direct_hits, era_reacts, crew_bailout, burn, move_soldiers, board_vehicles, animate_legs, fly_debris, drop_riders)
                    .chain()
                    .run_if(crate::running)
                    .in_set(crate::Step::Units),
            );
    }
}

#[derive(Resource)]
pub struct UnitRng(pub SmallRng);

/// Where the last direct hit landed, for test shots (`BAV_LOOK=hit`).
#[derive(Resource, Default)]
pub struct LastHit(pub Option<Vec3>);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    Ru,
    Ua,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, strum::VariantArray)]
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
}

impl VKind {
    pub fn is_tank(self) -> bool {
        matches!(self, VKind::T80 | VKind::T72 | VKind::Leo2a6)
    }
    /// Explosive reactive armour on the hull and turret front: Kontakt-1 on the T-80BV, Kontakt-5 on the
    /// T-72B3, the BRAT tiles on the 47th's Bradleys (M2A2 ODS-SA).
    pub fn era(self) -> bool {
        matches!(self, VKind::T80 | VKind::T72 | VKind::Bradley)
    }
    /// T-72/T-80 carousel autoloaders: when the charges go, the turret goes up.
    pub fn carousel(self) -> bool {
        matches!(self, VKind::T80 | VKind::T72)
    }
    /// Built for crew survival (blow-out panels, spall liners, escape hatches and ramps): most crews get out.
    pub fn western(self) -> bool {
        matches!(self, VKind::Leo2a6 | VKind::Leo2r | VKind::Wisent | VKind::Bradley)
    }
    /// Mine-clearing vehicles: no gun worth firing, a plow or rollers in front.
    pub fn breacher(self) -> bool {
        matches!(self, VKind::Leo2r | VKind::Wisent | VKind::Bmr2)
    }
    fn crew(self) -> u32 {
        match self {
            VKind::Mtlb | VKind::Bmr2 => 2,
            VKind::Leo2a6 => 4,
            _ => 3,
        }
    }
    pub fn troops(self) -> u32 {
        match self {
            VKind::T80 | VKind::T72 | VKind::Leo2a6 | VKind::Leo2r | VKind::Wisent | VKind::Bmr2 => 0,
            VKind::Bmp1 | VKind::Bmp2 | VKind::Bradley => 6,
            VKind::Mtlb => 8,
        }
    }
    /// Chance a mine under a track destroys the vehicle outright, rather than only tearing off the track.
    fn mine_kill(self, stacked: bool) -> f64 {
        let base = match self {
            VKind::T80 | VKind::T72 => 0.3,
            VKind::Leo2a6 => 0.1,
            VKind::Bradley => 0.25,
            VKind::Leo2r | VKind::Wisent | VKind::Bmr2 => 0.12,
            _ => 0.5,
        };
        if stacked { (base + 0.3f64).min(0.8) } else { base }
    }
    /// Chance a guided missile destroys rather than disables.
    fn hit_kill(self) -> f64 {
        match self {
            // Kontakt-1 on the glacis and turret front saves some; side and top hits mostly don't.
            VKind::T80 | VKind::T72 => 0.72,
            VKind::Leo2a6 => 0.3,
            VKind::Bradley => 0.55,
            VKind::Leo2r | VKind::Wisent | VKind::Bmr2 => 0.5,
            _ => 0.92,
        }
    }
    /// Everyone aboard: crew and dismounts.
    pub fn men(self) -> u32 {
        self.crew() + self.troops()
    }
    fn speed(self) -> f32 {
        match self {
            // A plow digging in moves at a walking pace.
            VKind::Leo2r | VKind::Wisent | VKind::Bmr2 => 2.4,
            VKind::Leo2a6 => 5.4,
            VKind::Bradley => 6.0,
            _ if self.is_tank() => 5.2,
            _ => 5.8,
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

impl Vehicle {
    /// Knocked out of action but not destroyed: it coasts to a stop at `decel`, slewing at `slew`.
    fn immobilise(&mut self, decel: f32, slew: f32) {
        self.state = VState::Immobilized;
        self.coast_decel = decel;
        self.slew = slew;
        self.timer = 0.0;
    }

    /// What the crew of a knocked-out vehicle does now: a fit crew is out in 5–15 s, a concussed one (a mine)
    /// in 20–60 s; a tank crew may stay a few minutes and fight from the dead track, turning the hull to face
    /// the enemy; and once in a long while nobody is at the tiller any more and the hull drives a slow circle
    /// on its one track until it stalls (the T-80BV of October 2022).
    fn crew_plan(&mut self, rng: &mut SmallRng, cause: Cause) {
        self.timer = 0.0;
        self.bail_at = if cause == Cause::Mine { rng.random_range(20.0..60.0) } else { rng.random_range(5.0..15.0) };
        self.pivot = false;
        self.ghost = 0.0;
        if self.kind.is_tank() && self.crew > 0 {
            if cause == Cause::Mine && rng.random_bool(1.0 / 15.0) {
                self.ghost = rng.random_range(20.0..40.0);
                self.bail_at = self.ghost + 10.0;
                self.slew = if self.slew >= 0.0 { 0.16 } else { -0.16 };
            } else if rng.random_bool(0.3) {
                self.pivot = true;
                self.bail_at = rng.random_range(120.0..300.0);
            }
        }
    }
}

impl VState {
    pub fn alive(self) -> bool {
        !matches!(self, VState::Destroyed | VState::Abandoned)
    }
    pub fn mobile(self) -> bool {
        !matches!(self, VState::Destroyed | VState::Abandoned | VState::Immobilized)
    }
}

#[derive(Component)]
pub struct Vehicle {
    pub kind: VKind,
    pub side: Side,
    pub state: VState,
    pub route: usize,
    pub s: f32,
    pub lat: f32,
    pub target_lat: f32,
    pub speed: f32,
    pub wave: u32,
    pub idx: u32,
    pub plow: bool,
    pub crew: u32,
    pub troops: u32,
    pub timer: f32,
    pub fire_timer: f32,
    pub panic: f32,
    pub turret: Entity,
    /// The gun's muzzle in the turret's frame.
    pub muzzle: Vec3,
    pub turret_yaw: f32,
    pub aim: Option<Vec2>,
    pub travelled: f32,
    pub reversing: bool,
    pub counted_returned: bool,
    /// Set once the wreck has been counted and set burning (several blasts can claim it in one frame).
    pub wrecked: bool,
    /// Hull heading (rate-limited: a tank turns, it does not snap).
    pub yaw: f32,
    /// Suspension: pitch and roll and their rates, a damped spring.
    pub pitch: f32,
    pub pitch_v: f32,
    pub roll: f32,
    pub roll_v: f32,
    /// After a hit or a mine: how fast it grinds to a halt, and how it slews round the dead track.
    pub coast_decel: f32,
    pub slew: f32,
    /// Sent back in for the men: it stops at the wrecks, waits for them to climb on, and comes out again.
    pub rescue: bool,
    /// Smoke grenades fired (once).
    pub smoked: bool,
    /// This driver's own pace, as a fraction of the kind's speed: a column is never one speed.
    pub pace: f32,
    /// How many seconds behind the truth this driver's idea of the vehicle ahead runs.
    pub lag: f32,
    /// The speed of the vehicle ahead, as the driver has caught up with it.
    pub lead_speed: f32,
    /// Metres driven in all (`travelled` is the paint odometer, reset every 2 m).
    pub odometer: f32,
    /// A halt planned on the approach: at this odometer reading, for this many seconds (a driver who has lost
    /// the one ahead in the exhaust smoke, or waits for an order).
    pub halt_at: Option<(f32, f32)>,
    /// Knocked out: seconds until the crew gets out.
    pub bail_at: f32,
    /// A dead crew and a live engine: seconds left of the hull circling on its one track.
    pub ghost: f32,
    /// A live crew on a dead track, turning the hull to face the enemy with the glacis (or to lay a jammed gun).
    pub pivot: bool,
    /// Seconds until the next man climbs out of a knocked-out vehicle.
    pub exit_timer: f32,
}

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
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
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
        let en = en.into();
        Self { uk: crate::text::uk(&en).to_string(), en }
    }

    pub fn both(uk: impl Into<String>, en: impl Into<String>) -> Self {
        Self { uk: uk.into(), en: en.into() }
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
pub struct Skin(pub Handle<StandardMaterial>);

/// The first of a model's meshes, worn by the entity itself (as every model was one mesh before the baked ones,
/// and the day's order of play hangs on which entities carry a mesh: see `Step` in main.rs).
pub fn skin(parts: &[Part]) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
    (Mesh3d(parts[0].mesh.clone()), MeshMaterial3d(parts[0].material.clone()))
}

/// ...burned out.
pub fn burnt(parts: &[Part]) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
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
pub fn char_skins(e: Entity, own: &mut MeshMaterial3d<StandardMaterial>, parts: &[Part], children: &Query<&Children>, skins: &mut Query<(&Skin, &mut MeshMaterial3d<StandardMaterial>), Without<Vehicle>>) {
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
    ru: FigureMeshes,
    ua: FigureMeshes,
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

pub fn setup_models(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<StandardMaterial>>, palette: Res<Palette>) {
    // The real models (tools/bake). Their paint is in the textures; this only weathers it.
    let weathered = Color::srgb(0.93, 0.9, 0.84);
    let baked: HashMap<&str, baked::Baked> = baked::IDS.iter().map(|&id| (id, baked::load(id, weathered, &mut meshes, &mut images, &mut materials))).collect();
    let hand = |meshes: &mut Assets<Mesh>, m: meshes::MeshBuilder| vec![Part { mesh: meshes.add(m.build()), material: palette.vertex.clone(), charred: palette.charred.clone() }];
    let mut vehicles = HashMap::new();
    for &k in VKind::VARIANTS {
        // The model whose hull it takes, and the hand-built top for those without a model of their own: the
        // Leopard 2R and the Wisent (a Leopard 1 hull, near enough a 2A4's), and the BMR-2 (a T-55's, here a T-64's).
        let (id, top) = match k {
            VKind::T80 => ("t80", None),
            VKind::T72 => ("t72", None),
            VKind::Bmp1 => ("bmp1", None),
            VKind::Bmp2 => ("bmp2", None),
            VKind::Mtlb => ("mtlb", None),
            VKind::Leo2a6 => ("leo2a6", None),
            VKind::Leo2r => ("leo2r", Some(meshes::leopard2r_top(WEST_GREEN))),
            VKind::Wisent => ("leo2r", Some(meshes::wisent_top(WEST_GREEN))),
            VKind::Bmr2 => ("t64", Some(meshes::bmr2_top(WEST_GREEN))),
            VKind::Bradley => ("bradley", None),
        };
        let b = &baked[id];
        let (turret, turret_at, muzzle) = match top {
            Some((t, at)) => (hand(&mut meshes, t), at, Vec3::new(0.0, 0.6, -1.6)),
            None => (b.turret.clone(), b.turret_at, b.muzzle),
        };
        let extra = match k {
            VKind::Leo2r => hand(&mut meshes, meshes::full_plow(-4.4, 4.2)),
            VKind::Wisent => hand(&mut meshes, meshes::full_plow(-4.1, 3.8)),
            VKind::Bmr2 => hand(&mut meshes, meshes::kmt_rollers()),
            _ => vec![],
        };
        // The carousel tanks may carry a KMT-7 at the head of a column.
        let plow_hull = k.carousel().then(|| b.hull.iter().cloned().chain(hand(&mut meshes, meshes::kmt7())).collect());
        let hull = b.hull.iter().cloned().chain(extra).collect();
        vehicles.insert(k, VehicleMeshes { hull, plow_hull, turret, turret_at, muzzle });
    }
    let t64 = &baked["t64"];
    let ua_tank = VehicleMeshes { hull: t64.hull.clone(), plow_hull: None, turret: t64.turret.clone(), turret_at: t64.turret_at, muzzle: t64.muzzle };
    let mut figure = |coat, trousers, helmet, tape| {
        let f = meshes::figure(coat, trousers, helmet, Some(tape));
        (FigureMeshes { body: meshes.add(f.body.build()), leg: meshes.add(f.leg.build()), fallen: meshes.add(meshes::fallen(coat, trousers).build()) }, f.hip)
    };
    let ((ru, hip), (ua, _)) = if diorama().summer() {
        // Summer: no coats. Russian flora pattern, greyish; Ukrainian multicam-like, browner; tape on the arms.
        (figure([0.40, 0.42, 0.33], [0.36, 0.38, 0.30], [0.33, 0.35, 0.28], [0.8, 0.3, 0.25]), figure([0.45, 0.43, 0.32], [0.40, 0.38, 0.29], [0.36, 0.36, 0.27], [0.85, 0.8, 0.3]))
    } else {
        (figure([0.36, 0.37, 0.31], [0.33, 0.34, 0.29], [0.3, 0.32, 0.26], [0.8, 0.3, 0.25]), figure([0.34, 0.36, 0.27], [0.30, 0.32, 0.24], [0.28, 0.31, 0.22], [0.85, 0.8, 0.3]))
    };
    let mut raam = meshes::MeshBuilder::new();
    raam.cylinder(Vec3::ZERO, Vec3::Y * 0.12, 0.08, 6, [0.22, 0.24, 0.17]);
    let mut missile = meshes::MeshBuilder::new();
    missile.cylinder(Vec3::new(0.0, 0.0, 0.6), Vec3::new(0.0, 0.0, -1.2), 0.07, 6, [0.3, 0.32, 0.25]);
    let mut marker = meshes::MeshBuilder::new();
    marker.cylinder(Vec3::ZERO, Vec3::Y * 1.3, 0.025, 4, [0.3, 0.3, 0.28]);
    marker.boxc(Vec3::new(0.12, 1.2, 0.0), Vec3::new(0.24, 0.16, 0.01), [0.95, 0.45, 0.1]);
    let mut strip = meshes::MeshBuilder::new();
    strip.boxc(Vec3::new(0.0, 0.05, 0.0), Vec3::new(0.55, 0.1, 5.5), [0.12, 0.12, 0.11]);
    let mut wheel = meshes::MeshBuilder::new();
    wheel.cylinder(Vec3::new(-0.1, 0.37, 0.0), Vec3::new(0.2, 0.0, 0.0), 0.37, 10, [0.16, 0.16, 0.15]);
    commands.insert_resource(Models { vehicles, ru, ua, hip, raam: meshes.add(raam.build()), missile: meshes.add(missile.build()), ua_tank, marker: meshes.add(marker.build()), track_strip: meshes.add(strip.build()), road_wheel: meshes.add(wheel.build()) });
}

// ---------------------------------------------------------------------------------------------------------
// Mines

#[derive(Clone, Copy)]
pub struct Mine {
    pub pos: Vec2,
    pub live: bool,
    pub entity: Option<Entity>,
    /// Two anti-tank mines one on the other: no plow or roller survives it.
    pub stacked: bool,
    /// Anti-personnel: goes off under a foot, not under a track (a track sets it off harmlessly).
    pub ap: bool,
}

impl Mine {
    pub fn at(pos: Vec2) -> Self {
        Self { pos, live: true, entity: None, stacked: false, ap: false }
    }
}

#[derive(Resource, Default)]
pub struct Minefield {
    pub mines: Vec<Mine>,
    grid: HashMap<(i32, i32), Vec<usize>>,
}

const MINE_CELL: f32 = 8.0;

impl Minefield {
    pub fn add(&mut self, m: Mine) {
        let i = self.mines.len();
        self.mines.push(m);
        let c = ((m.pos.x / MINE_CELL).floor() as i32, (m.pos.y / MINE_CELL).floor() as i32);
        self.grid.entry(c).or_default().push(i);
    }

    /// The live mines in the grid cells within `r` of `p`, into `out`, which is cleared first so a caller can keep
    /// one buffer across calls.
    pub fn near_into(&self, p: Vec2, r: f32, out: &mut Vec<usize>) {
        out.clear();
        let (x0, x1) = (((p.x - r) / MINE_CELL).floor() as i32, ((p.x + r) / MINE_CELL).floor() as i32);
        let (z0, z1) = (((p.y - r) / MINE_CELL).floor() as i32, ((p.y + r) / MINE_CELL).floor() as i32);
        for cz in z0..=z1 {
            for cx in x0..=x1 {
                if let Some(v) = self.grid.get(&(cx, cz)) {
                    out.extend(v.iter().copied().filter(|&i| self.mines[i].live));
                }
            }
        }
    }
}

fn lay_minefields(mut commands: Commands) {
    let lay = geo::layout();
    let mut mf = Minefield::default();
    let mut rng = SmallRng::seed_from_u64(crate::dev::seeded(72));
    for f in &lay.shoulder_fields {
        let road = &lay.roads[f.road].line;
        let area = (f.s1 - f.s0) * (f.outer - f.inner) * 2.0;
        let n = (area * f.density) as usize;
        for _ in 0..n {
            let s = rng.random_range(f.s0..f.s1);
            let lat = sign(&mut rng) * rng.random_range(f.inner..f.outer);
            let p = road.point(s) + road.normal(s) * lat;
            mf.add(Mine::at(p));
        }
    }
    for (line, width) in &lay.field_mines {
        let n = (line.length() * width / 60.0) as usize;
        for _ in 0..n {
            let s = rng.random_range(0.0..line.length());
            let p = line.point(s) + line.normal(s) * rng.random_range(-width * 0.5..width * 0.5);
            // Lanes left open where the roads cross.
            if lay.roads.iter().any(|r| r.line.project(p).0 < 7.0) {
                continue;
            }
            mf.add(Mine::at(p));
        }
    }
    for b in &lay.barriers {
        let area = 2.0 * geo::HALF_X * (b.z1 - b.z0);
        for _ in 0..(area * b.at) as usize {
            let p = Vec2::new(rng.random_range(-geo::HALF_X..geo::HALF_X), rng.random_range(b.z0..b.z1));
            mf.add(Mine { stacked: rng.random_bool(b.stacked as f64), ..Mine::at(p) });
        }
        for _ in 0..(area * b.ap) as usize {
            let p = Vec2::new(rng.random_range(-geo::HALF_X..geo::HALF_X), rng.random_range(b.z0..b.z1));
            mf.add(Mine { ap: true, ..Mine::at(p) });
        }
    }
    info!("minefields: {} mines", mf.mines.len());
    commands.insert_resource(mf);
}

// ---------------------------------------------------------------------------------------------------------
// Spawning

pub fn vehicle_yaw(dir: Vec2) -> f32 {
    f32::atan2(-dir.x, -dir.y)
}

fn ground_rotation(p: Vec2, yaw: f32) -> Quat {
    let n = geo::normal(p.x, p.y);
    Quat::from_rotation_arc(Vec3::Y, n) * Quat::from_rotation_y(yaw)
}

pub struct VehicleSpec {
    pub side: Side,
    pub kind: VKind,
    pub route: usize,
    pub s: f32,
    pub lat: f32,
    pub wave: u32,
    pub idx: u32,
    pub plow: bool,
    pub smoke: bool,
    pub rescue: bool,
}

pub fn spawn_vehicle(commands: &mut Commands, models: &Models, palette: &Palette, v: VehicleSpec) -> Entity {
    let route = &geo::layout().routes[v.route].line;
    let p = route.point(v.s) + route.normal(v.s) * v.lat;
    let yaw = vehicle_yaw(route.tangent(v.s));
    spawn_vehicle_at(commands, models, palette, v, p, yaw)
}

/// A vehicle placed by hand rather than on a route (a dug-in tank).
pub fn spawn_vehicle_at(commands: &mut Commands, models: &Models, palette: &Palette, v: VehicleSpec, p: Vec2, yaw: f32) -> Entity {
    let m = models.vehicle(v.kind);
    let plow_hull = m.plow_hull.as_ref().filter(|_| v.plow);
    let plow = plow_hull.is_some();
    let hull = plow_hull.unwrap_or(&m.hull);
    let mut t = commands.spawn((skin(&m.turret), Transform::from_translation(m.turret_at), Turret));
    dress(&mut t, &m.turret);
    let turret = t.id();
    let mut e = commands.spawn((
        skin(hull),
        Transform::from_translation(geo::ground(p)).with_rotation(ground_rotation(p, yaw)),
        Vehicle {
            kind: v.kind,
            side: v.side,
            state: VState::Advancing,
            route: v.route,
            s: v.s,
            lat: v.lat,
            target_lat: v.lat,
            speed: 0.0,
            wave: v.wave,
            idx: v.idx,
            plow,
            crew: v.kind.crew(),
            troops: v.kind.troops(),
            timer: 0.0,
            fire_timer: 8.0 + v.idx as f32 * 2.3,
            panic: 0.0,
            turret,
            muzzle: m.muzzle,
            turret_yaw: 0.0,
            aim: None,
            travelled: 0.0,
            reversing: false,
            counted_returned: false,
            wrecked: false,
            yaw,
            pitch: 0.0,
            pitch_v: 0.0,
            roll: 0.0,
            roll_v: 0.0,
            coast_decel: 3.0,
            slew: 0.0,
            rescue: v.rescue,
            smoked: false,
            pace: 0.9 + 0.2 * geo::hash01(v.wave * 100 + v.idx, 23),
            lag: 1.0 + 2.0 * geo::hash01(v.wave * 100 + v.idx, 24),
            lead_speed: 0.0,
            odometer: 0.0,
            halt_at: (geo::hash01(v.wave * 100 + v.idx, 25) < 0.1).then(|| (100.0 + 500.0 * geo::hash01(v.wave * 100 + v.idx, 26), 5.0 + 15.0 * geo::hash01(v.wave * 100 + v.idx, 27))),
            bail_at: 20.0,
            ghost: 0.0,
            pivot: false,
            exit_timer: 0.0,
        },
    ));
    e.add_child(turret);
    dress(&mut e, hull);
    // Desant: the section rides on the roof, not inside (dossier footage, and every Vuhledar video). Mines and
    // RPGs taught both armies that the hull is the worst place to be. A Bradley's squad rides inside, behind
    // the ramp; the rescue Bradleys go in empty.
    let riders = if v.side == Side::Ru { v.kind.troops().min(6) } else { 0 };
    let roof = m.turret_at.y;
    for k in 0..riders {
        let side = if k % 2 == 0 { -0.55 } else { 0.55 };
        let z = 0.4 + (k / 2) as f32 * 0.85;
        e.with_child((
            Mesh3d(models.figure(v.side).body.clone()),
            MeshMaterial3d(palette.vertex.clone()),
            Transform::from_xyz(side, roof - 0.85, z).with_rotation(Quat::from_rotation_y(side.signum() * 1.2 + k as f32 * 0.3)),
            NotShadowCaster,
            Rider,
        ));
    }
    if v.smoke {
        // TDA: diesel injected into the exhaust, a white wall that drifts with the wind.
        e.insert(Emitter::new(EmitterKind::SmokeScreen, 2.5, 240.0, Vec3::new(0.0, 1.2, 3.6)));
    } else {
        e.insert(Emitter::new(EmitterKind::Exhaust, 3.0, 1.0e6, Vec3::new(0.6, 1.3, 3.4)));
    }
    e.id()
}

fn spawn_old_wrecks(mut commands: Commands, models: Res<Models>) {
    // The fields already hold the earlier assaults' wrecks (24 Jan – 4 Feb): BMPs and tanks, burned out, with
    // their turrets beside them. Nobody has come for them.
    let kinds = match diorama() {
        Diorama::Vuhledar => [VKind::Bmp2, VKind::T80, VKind::Bmp1, VKind::Bmp2, VKind::T72, VKind::Mtlb, VKind::Bmp1, VKind::T80],
        Diorama::Robotyne => [VKind::T72, VKind::Bmp2, VKind::Mtlb, VKind::T72, VKind::Bmp2, VKind::Mtlb, VKind::T72, VKind::Bmp2],
    };
    for (i, (p, yaw)) in geo::layout().old_wrecks.iter().enumerate() {
        let k = kinds[i % kinds.len()];
        let toss = k.is_tank() && i % 2 == 1;
        let m = models.vehicle(k);
        let mut e = commands.spawn((burnt(&m.hull), Transform::from_translation(geo::ground(*p) - Vec3::Y * 0.15).with_rotation(ground_rotation(*p, *yaw))));
        if !toss {
            e.with_children(|c| dress_charred(&mut c.spawn((burnt(&m.turret), Transform::from_translation(m.turret_at).with_rotation(Quat::from_rotation_y(i as f32)))), &m.turret));
        }
        dress_charred(&mut e, &m.hull);
        if i % 3 == 0 {
            e.insert(Emitter::new(EmitterKind::Smoulder, 0.5, 1.0e5, Vec3::Y));
        }
        if toss {
            let tp = *p + Vec2::from_angle(i as f32 * 1.7) * 14.0;
            let mut t = commands.spawn((burnt(&m.turret), Transform::from_translation(geo::ground(tp) + Vec3::Y * 0.3).with_rotation(Quat::from_euler(EulerRot::XYZ, 2.6, i as f32, 0.3))));
            dress_charred(&mut t, &m.turret);
        }
    }
}

pub fn spawn_soldier(commands: &mut Commands, models: &Models, palette: &Palette, side: Side, at: Vec2, goal: Vec2, home: Vec2, sprint: f32, cover: Option<Vec2>, rng: &mut SmallRng) -> Entity {
    let (body, leg) = (models.figure(side).body.clone(), models.figure(side).leg.clone());
    let hip = models.hip;
    let l0 = commands.spawn((Mesh3d(leg.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(hip), Leg(0.0), NotShadowCaster)).id();
    let l1 = commands.spawn((Mesh3d(leg), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(hip * Vec3::new(-1.0, 1.0, 1.0)), Leg(std::f32::consts::PI), NotShadowCaster)).id();
    let mut e = commands.spawn((
        Mesh3d(body),
        MeshMaterial3d(palette.vertex.clone()),
        Transform::from_translation(geo::ground(at)),
        Soldier { side, state: SState::Advance, goal, home, speed: rng.random_range(1.5..2.3), fear: 0.0, timer: 0.0, phase: rng.random_range(0.0..6.0), sprint, leg: cover },
        NotShadowCaster,
    ));
    e.add_children(&[l0, l1]);
    e.id()
}

// ---------------------------------------------------------------------------------------------------------
// Messages

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cause {
    Artillery,
    Mine,
    Atgm,
    TankGun,
    Himars,
    Thermobaric,
    /// A wreck's ammunition going: it hurts whoever is near it.
    CookOff,
}

/// An explosion that hurts whatever is near it.
#[derive(Message, Clone, Copy)]
pub struct Blast {
    pub at: Vec3,
    /// Lethal radius for people in the open; vehicles need much closer hits.
    pub radius: f32,
    pub cause: Cause,
}

/// A guided weapon striking a specific vehicle.
#[derive(Message, Clone, Copy)]
pub struct DirectHit {
    pub target: Entity,
    pub cause: Cause,
    /// Onto the roof, where the armour is thinnest (a Javelin's dive).
    pub top: bool,
}

// ---------------------------------------------------------------------------------------------------------
// Driving

#[derive(Clone, Copy)]
struct Snap {
    e: Entity,
    route: usize,
    s: f32,
    lat: f32,
    state: VState,
    speed: f32,
    reversing: bool,
    pos: Vec2,
}

/// How fast the ground lets a tracked vehicle go, as a fraction of its road speed: the road itself, the plowed
/// lane, a field under thin snow or in June's dry stubble, a turn off the road, a shell crater under a track.
fn going(p: Vec2, lat: f32, lay: &geo::Layout, craters: &crate::terrain::Craters, turning: bool) -> f32 {
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

fn drive_vehicles(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<UnitRng>,
    mut fxr: ResMut<FxRng>,
    mut fx: ResMut<FxAssets>,
    mut paint: ResMut<PaintQueue>,
    mut stats: ResMut<Stats>,
    models: Res<Models>,
    palette: Res<Palette>,
    cam: Single<&GlobalTransform, With<MainCamera>>,
    mut levels: ResMut<audio::AmbienceLevels>,
    (order, mut radio, mut smoke): (Res<ColumnOrder>, ResMut<Radio>, ResMut<fx::Smoke>),
    mut q: Query<(Entity, &mut Vehicle, &mut Transform), Without<Turret>>,
    mut turrets: Query<&mut Transform, With<Turret>>,
    mut snaps: Local<Vec<Snap>>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let lay = geo::layout();
    let attacker = diorama().attacker();
    snaps.clear();
    snaps.extend(q.iter().map(|(e, v, t)| Snap { e, route: v.route, s: v.s, lat: v.lat, state: v.state, speed: v.speed, reversing: v.reversing, pos: t.translation.xz() }));
    let cam_pos = cam.translation();
    let mut nearest_engine = f32::MAX;
    let rng = &mut rng.0;

    for (e, mut v, mut tf) in &mut q {
        if v.side != attacker || !v.state.mobile() {
            continue;
        }
        let route: &Polyline = &lay.routes[v.route].line;
        let deploy_s = lay.routes[v.route].deploy_s;
        let dismount_s = deploy_s + 200.0 + v.idx as f32 % 3.0 * 25.0;
        v.panic = (v.panic - dt * 0.05).max(0.0);
        v.timer += dt;

        // Who is ahead of me in my lane?
        let dir = if v.reversing { -1.0 } else { 1.0 };
        let mut blocker: Option<Snap> = None;
        for o in snaps.iter() {
            if o.e == e || o.route != v.route {
                continue;
            }
            let ds = (o.s - v.s) * dir;
            // Judge the lane I am steering into, so a vehicle pulling out round a wreck is not blocked by it.
            if ds > 0.0 && ds < 55.0 && (o.lat - v.target_lat).abs() < 6.0 && blocker.is_none_or(|b| (b.s - v.s).abs() > ds) {
                blocker = Some(*o);
            }
        }

        // Steel does not pass through steel: a hull within a length ahead stops this one, and a hull beside it
        // stops the sideways move towards it, whatever lane either is in.
        let me = tf.translation.xz();
        let turning = (v.target_lat - v.lat).abs() > 1.0;
        let fwd2 = Vec2::new(-v.yaw.sin(), -v.yaw.cos()) * dir;
        let right2 = Vec2::new(-fwd2.y, fwd2.x);
        let side_move = route.normal(v.s) * (v.target_lat - v.lat).signum();
        let (mut blocked_ahead, mut blocked_beside) = (false, false);
        let mut push = Vec2::ZERO;
        for o in snaps.iter() {
            if o.e == e {
                continue;
            }
            let d = o.pos - me;
            let (along, across) = (d.dot(fwd2), d.dot(right2));
            blocked_ahead |= (0.0..7.5).contains(&along) && across.abs() < 3.6;
            blocked_beside |= turning && d.length() < 5.5 && d.dot(side_move) > 0.0;
            // Two hulls that have slid into each other sideways (where neither test looks) work apart.
            if d.length() < 5.0 {
                push -= d.normalize_or(route.normal(v.s)) * (5.0 - d.length());
            }
        }
        if push != Vec2::ZERO {
            let step = push.clamp_length_max(1.5 * dt);
            v.s = (v.s + step.dot(route.tangent(v.s))).clamp(0.0, route.length());
            v.lat += step.dot(route.normal(v.s));
        }
        let mut want_speed = v.kind.speed() * v.pace;
        // A breacher drives with its plow up until the minefield, then digs in at a walking pace.
        if v.kind.breacher() && !lay.barriers.iter().any(|b| (b.z0 - 60.0..b.z1).contains(&tf.translation.z)) {
            want_speed = 4.6 * v.pace;
        }
        if blocker.is_none() {
            v.lead_speed = want_speed;
        }
        // The player's order to the column (Robotyne): hold everyone where they are, or bring them all back.
        if v.state != VState::Withdrawing && !v.rescue {
            match *order {
                ColumnOrder::PullBack => start_withdrawal(&mut v, rng),
                ColumnOrder::Hold if matches!(v.state, VState::Advancing | VState::Halted | VState::Deploying) => {
                    want_speed = 0.0;
                }
                _ => {}
            }
        }
        // A rescue vehicle stops short of the first wreck on its track, puts down smoke and waits for the men.
        if v.rescue && v.state == VState::Advancing && blocker.is_some_and(|b| !b.state.mobile() && (b.s - v.s).abs() < 70.0) {
            v.state = VState::Overwatch;
            v.timer = 0.0;
            pop_smoke(&mut commands, &mut fx, &mut fxr.0, &mut smoke, &mut v, &tf, now);
            radio.say(now, "Bradley at the wrecks. Ramp down. Get on, get on!");
        }
        match v.state {
            VState::Advancing | VState::Halted => {
                if let Some(b) = blocker {
                    let gap = (b.s - v.s).abs();
                    let b_moving = b.state.mobile() && b.speed > 0.5 && b.reversing == v.reversing && b.state != VState::Halted;
                    if b_moving {
                        // The driver answers the vehicle ahead a second or three late: the column breathes.
                        v.lead_speed += (b.speed - v.lead_speed) * (dt / v.lag).min(1.0);
                        want_speed = want_speed.min(v.lead_speed * (gap / 40.0).clamp(0.0, 1.2));
                        v.state = VState::Advancing;
                    } else if gap < 45.0 {
                        // The road is blocked by a wreck or a stalled vehicle. Wait, or go round it through the field.
                        want_speed = 0.0;
                        if v.state != VState::Halted {
                            v.state = VState::Halted;
                            v.timer = 0.0;
                        }
                        if v.timer > rng.random_range(4.0..14.0) * (1.0 - v.panic * 0.7) {
                            if rng.random_bool(0.75) {
                                v.target_lat = b.lat + sign(rng) * rng.random_range(8.0..18.0);
                                v.state = VState::Advancing;
                            } else if rng.random_bool(0.3) {
                                start_withdrawal(&mut v, rng);
                            }
                            v.timer = 0.0;
                        }
                    }
                } else if v.state == VState::Halted {
                    // A planned halt holds for its time; a halt behind a hull outside the lane ends with a pull
                    // round it; any other halt ends when the road ahead is clear.
                    if v.halt_at.is_some_and(|(_, secs)| v.timer < secs) {
                        want_speed = 0.0;
                    } else if blocked_ahead {
                        if v.timer > rng.random_range(4.0..14.0) {
                            v.target_lat += sign(rng) * rng.random_range(8.0..18.0);
                            v.timer = 0.0;
                        }
                    } else {
                        v.halt_at = None;
                        v.state = VState::Advancing;
                    }
                } else if blocked_ahead {
                    v.state = VState::Halted;
                    v.timer = 0.0;
                }
                if let Some((at, secs)) = v.halt_at
                    && v.state == VState::Advancing
                    && v.odometer > at
                {
                    v.state = VState::Halted;
                    v.timer = 0.0;
                    v.halt_at = Some((f32::MAX, secs));
                }
                // Stuck in the minefield under fire, a Bradley drops its ramp and the squad goes on foot.
                if v.side == Side::Ua && v.state == VState::Halted && v.troops > 0 && v.timer > 3.5 && v.panic > 0.3 {
                    v.state = VState::Dismounting;
                    v.timer = 0.0;
                    want_speed = 0.0;
                }
                if v.state == VState::Advancing && v.s >= deploy_s {
                    v.state = VState::Deploying;
                    // Fan out into the field to dismount: straight into the shoulder minefields.
                    let spread = [0.0f32, -22.0, 24.0, -45.0, 40.0, -64.0, 60.0, -15.0, 15.0, -34.0, 34.0, -55.0];
                    v.target_lat = if v.plow { 0.0 } else { spread[(v.idx as usize) % spread.len()] + rng.random_range(-6.0..6.0) };
                }
                // Panic: a hit nearby, and a mobilised driver yanks the tiller.
                if v.panic > 0.55 && rng.random_bool((dt * 0.12) as f64) {
                    if rng.random_bool(0.55) {
                        v.target_lat = v.lat + sign(rng) * rng.random_range(8.0..25.0);
                    } else {
                        start_withdrawal(&mut v, rng);
                    }
                }
            }
            VState::Deploying => {
                want_speed = 3.8;
                if blocker.is_some_and(|b| !b.state.mobile() && (b.s - v.s).abs() < 20.0) {
                    v.target_lat += if rng.random_bool(0.5) { 6.0 } else { -6.0 };
                }
                if v.s >= dismount_s {
                    v.state = if v.troops > 0 { VState::Dismounting } else { VState::Overwatch };
                    v.timer = 0.0;
                }
            }
            VState::Dismounting => {
                want_speed = 0.0;
                if v.timer > 3.0 && v.troops > 0 {
                    // The ramp drops; the section runs for the tree line.
                    let obj = lay.routes[v.route].objective;
                    let back = route.tangent(v.s) * -6.0;
                    let pos = tf.translation.xz();
                    for k in 0..v.troops {
                        let at = pos + back + Vec2::new(rng.random_range(-2.0..2.0), rng.random_range(-2.0..2.0));
                        let goal = if v.side == Side::Ua {
                            // The 47th's squads go forward a bound into cover, not across a kilometre of open ground:
                            // a hundred-odd metres, then down, then back to the vehicles.
                            pos + route.tangent(v.s) * rng.random_range(80.0..160.0) + route.normal(v.s) * (k as f32 - 3.0) * 7.0
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
            VState::Overwatch if v.rescue => {
                want_speed = 0.0;
                // Wait for the men to climb on: until full, or until staying any longer is death too.
                if v.troops >= 7 || v.timer > 110.0 || (v.panic > 0.95 && v.timer > 45.0) {
                    start_withdrawal(&mut v, rng);
                    radio.say2(now, format!("Бредлі виходить, на борту {}.", v.troops), format!("Bradley coming out with {} aboard.", v.troops));
                }
            }
            VState::Overwatch => {
                want_speed = 0.0;
                if v.timer > rng.random_range(60.0..140.0) || v.panic > 0.8 {
                    start_withdrawal(&mut v, rng);
                }
            }
            VState::Withdrawing => {
                if v.s < 30.0 {
                    if !v.counted_returned {
                        v.counted_returned = true;
                        stats.withdrawn += 1;
                        stats.returned += v.crew + v.troops;
                    }
                    commands.entity(e).despawn();
                    continue;
                }
                want_speed = v.kind.speed() * 1.1 * v.pace;
                if let Some(b) = blocker
                    && !b.state.mobile()
                    && (b.s - v.s).abs() < 25.0
                {
                    v.target_lat = b.lat + if v.lat > b.lat { 10.0 } else { -10.0 };
                }
            }
            _ => {}
        }

        // The ground: a road, a plowed lane, a field under snow, a turn off the road, a crater under a track.
        let going = going(me, v.lat, lay, &paint.craters, turning);
        want_speed *= going;
        if blocked_ahead {
            want_speed = 0.0;
        }
        // Move in route coordinates. A tank accelerates at about 1 m/s² and brakes at about 2; it brakes hard only
        // when something is right in front of it.
        let hard = blocked_ahead || blocker.is_some_and(|b| (b.s - v.s).abs() < 22.0) || v.panic > 0.7;
        let prev_speed = v.speed;
        v.speed += (want_speed - v.speed).clamp(-if hard { 3.5 } else { 1.8 } * dt, 1.1 * dt);
        let lat_rate = if blocked_beside { 0.0 } else { (v.target_lat - v.lat).clamp(-2.5, 2.5) };
        let ds = v.speed * dt * if v.reversing { -1.0 } else { 1.0 };
        v.s = (v.s + ds).clamp(0.0, route.length());
        v.lat += lat_rate * dt * (v.speed / 4.0).clamp(0.2, 1.0);
        let p = route.point(v.s) + route.normal(v.s) * v.lat;
        let fwd = route.tangent(v.s) * if v.reversing { -1.0 } else { 1.0 };
        let heading = (fwd * v.speed.max(0.5) + route.normal(v.s) * lat_rate * (v.speed / 4.0).clamp(0.2, 1.0) * if v.reversing { -1.0 } else { 1.0 }).normalize_or_zero();
        // The hull turns at most ~26°/s (reversing, the hull keeps facing forward).
        let want_yaw = if heading.length_squared() > 0.0 { vehicle_yaw(heading * if v.reversing { -1.0 } else { 1.0 }) } else { v.yaw };
        let dyaw = wrap(want_yaw - v.yaw).clamp(-0.45 * dt, 0.45 * dt);
        v.yaw = wrap(v.yaw + dyaw);
        let yaw = v.yaw;
        let old = tf.translation.xz();
        tf.translation = geo::ground(p) + Vec3::Y * 0.05;
        // Suspension: nose down under braking, squat under power, roll in turns, pitching over the ruts, and
        // worse the worse the going.
        let accel = (v.speed - prev_speed) / dt.max(1e-4) * if v.reversing { -1.0 } else { 1.0 };
        let rough = geo::value_noise(v.s * 0.35, v.idx as f32 * 3.1, 77) * 0.03 * (v.speed / 5.0) * (1.0 + 2.0 * (1.0 - going));
        let roll_target = -dyaw / dt.max(1e-4) * v.speed * 0.012;
        suspension(&mut v, accel * 0.022 + rough, roll_target, dt);
        tf.rotation = ground_rotation(p, yaw) * Quat::from_rotation_x(v.pitch) * Quat::from_rotation_z(v.roll);
        let moved = old.distance(p);
        v.travelled += moved;
        v.odometer += moved;
        if v.travelled > 2.0 {
            v.travelled = 0.0;
            paint.push(Paint::Track { at: p, dir: heading, gauge: 2.8 });
            // The plow turns the ground over: the lane shows from the air as a raw brown strip.
            if v.plow {
                paint.push(Paint::Plow { at: p, dir: heading });
                // The 2R marks the lane's edges as it goes, so the vehicles behind can keep to it.
                if v.kind == VKind::Leo2r && ((v.s / 2.0) as u32).is_multiple_of(20) && lay.barriers.iter().any(|b| (b.z0..b.z1).contains(&p.y)) {
                    for side in [-1.0f32, 1.0] {
                        let at = p + route.normal(v.s) * side * 2.6;
                        commands.spawn((Mesh3d(models.marker.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(at)), NotShadowCaster));
                    }
                }
            }
            // Hard going throws more: the tracks dig.
            if v.speed > 2.0 && rng.random_bool((0.5 / going.max(0.4)).min(1.0) as f64) {
                fx::spray(&mut commands, &mut fx, geo::ground(p) + Vec3::new(0.0, 0.0, 0.0) - (heading * 3.0).extend(0.0).xzy(), (heading * v.speed).extend(0.0).xzy());
            }
        }
        if v.speed > 0.5 {
            nearest_engine = nearest_engine.min(cam_pos.distance(tf.translation));
        }
        stats.vehicle_advance = stats.vehicle_advance.max(p.y * lay.advance);

        // Turret: track the aim point.
        if let Some(aim) = v.aim {
            let to = aim - p;
            let world_yaw = vehicle_yaw(to);
            let rel = world_yaw - yaw;
            v.turret_yaw += (wrap(rel - v.turret_yaw)).clamp(-dt * 0.6, dt * 0.6);
            if let Ok(mut ttf) = turrets.get_mut(v.turret) {
                ttf.rotation = Quat::from_rotation_y(v.turret_yaw);
            }
        }
    }
    levels.engines = if nearest_engine < f32::MAX { (0.6 * 120.0 / (120.0 + nearest_engine)).min(0.5) } else { 0.0 };
}

/// A damped spring on the hull's pitch and roll.
fn suspension(v: &mut Vehicle, pitch_target: f32, roll_target: f32, dt: f32) {
    let (w, z) = (5.5f32, 0.28f32);
    v.pitch_v += (w * w * (pitch_target.clamp(-0.12, 0.12) - v.pitch) - 2.0 * z * w * v.pitch_v) * dt;
    v.pitch += v.pitch_v * dt;
    v.roll_v += (w * w * (roll_target.clamp(-0.1, 0.1) - v.roll) - 2.0 * z * w * v.roll_v) * dt;
    v.roll += v.roll_v * dt;
}

/// Vehicles knocked out or immobilised while moving coast to a stop, slewing; then the hull settles.
fn coast(time: Res<Time>, mut paint: ResMut<PaintQueue>, posts: Query<&crate::fire::Post>, mut q: Query<(Entity, &mut Vehicle, &mut Transform)>, mut hulls: Local<Vec<(Entity, Vec2)>>) {
    let dt = crate::sim_dt(&time);
    let lay = geo::layout();
    hulls.clear();
    hulls.extend(q.iter().map(|(e, _, t)| (e, t.translation.xz())));
    for (e, mut v, mut tf) in &mut q {
        if v.state.mobile() {
            continue;
        }
        // Nobody at the tiller and the engine alive: the hull circles on its one track until it stalls.
        if v.ghost > 0.0 {
            v.ghost -= dt;
            v.speed = v.speed.max(1.6);
            v.slew = if v.slew >= 0.0 { 0.16 } else { -0.16 };
        }
        // A live crew on a dead track turns the hull to put the glacis towards the nearest enemy position.
        let mut turned = false;
        if v.pivot && v.state == VState::Immobilized && v.crew > 0 && v.speed <= 0.02 {
            let me = tf.translation.xz();
            if let Some(target) = posts.iter().filter(|p| p.side != v.side && p.strength > 0).map(|p| p.pos).min_by(|a, b| a.distance(me).total_cmp(&b.distance(me))) {
                let dyaw = wrap(vehicle_yaw(target - me) - v.yaw);
                if dyaw.abs() < 0.03 {
                    v.pivot = false;
                } else {
                    v.yaw = wrap(v.yaw + dyaw.clamp(-0.2 * dt, 0.2 * dt));
                    turned = true;
                }
            } else {
                v.pivot = false;
            }
        }
        // A rolling wreck stops against the next hull instead of sliding into it.
        if v.speed > 0.02 {
            let dir = Vec2::new(-v.yaw.sin(), -v.yaw.cos()) * if v.reversing { -1.0 } else { 1.0 };
            let me = tf.translation.xz();
            if hulls.iter().any(|(o, p)| *o != e && { let d = *p - me; d.dot(dir) > 0.0 && d.length() < 6.0 }) {
                v.speed = 0.0;
            }
        }
        let moving = v.speed > 0.02;
        if !moving && !turned && v.pitch.abs() < 1e-4 && v.pitch_v.abs() < 1e-4 && v.roll.abs() < 1e-4 && v.roll_v.abs() < 1e-4 {
            continue;
        }
        let prev = v.speed;
        if moving {
            v.speed = (v.speed - v.coast_decel * dt).max(0.0);
            v.slew *= (1.0 - dt * 0.8).max(0.0);
            let yaw = v.yaw + v.slew * dt * (v.speed / 3.0).min(1.0);
            v.yaw = wrap(yaw);
            let dir = Vec2::new(-v.yaw.sin(), -v.yaw.cos()) * if v.reversing { -1.0 } else { 1.0 };
            let p = tf.translation.xz() + dir * v.speed * dt;
            tf.translation = geo::ground(p) + Vec3::Y * 0.05;
            // Keep route coordinates current so the vehicles behind see where the wreck actually is.
            let route = &lay.routes[v.route].line;
            (v.lat, v.s) = route.lateral(p);
            v.target_lat = v.lat;
            v.travelled += v.speed * dt;
            if v.travelled > 2.0 {
                v.travelled = 0.0;
                paint.push(Paint::Track { at: p, dir, gauge: 2.8 });
            }
        }
        let accel = (v.speed - prev) / dt.max(1e-4);
        suspension(&mut v, accel * 0.022, 0.0, dt);
        let p = tf.translation.xz();
        tf.rotation = ground_rotation(p, v.yaw) * Quat::from_rotation_x(v.pitch) * Quat::from_rotation_z(v.roll);
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
/// a near-instant dense white burst that is gone in half a minute.
pub fn pop_smoke(commands: &mut Commands, fx: &mut FxAssets, rng: &mut SmallRng, smoke: &mut fx::Smoke, v: &mut Vehicle, tf: &Transform, now: f64) {
    if v.smoked {
        return;
    }
    v.smoked = true;
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
        fx::spawn(commands, fx, fx::Spawn { kind: fx::Kind::GreySmoke, pos: from, vel, life: secs, size0: 0.6, size1: 0.8, alpha: 0.9, drag: 0.0, gravity: 9.8, streak: 5.0, ..default() });
        let (kind, rate, burn) = if v.side == Side::Ua { (EmitterKind::Wp, 1.5, rng.random_range(18.0..30.0)) } else { (EmitterKind::Hc, 0.8, rng.random_range(90.0..150.0)) };
        commands.spawn((Transform::from_translation(geo::ground(land)), Emitter::after(secs, kind, rate, burn, Vec3::Y * 0.3)));
    }
    smoke.add(tf.translation.xz() + fwd * 45.0, 32.0, now + 120.0);
}

fn start_withdrawal(v: &mut Vehicle, rng: &mut SmallRng) {
    v.state = VState::Withdrawing;
    v.reversing = true;
    // Turning a tank round on a narrow road means a pivot onto the shoulder.
    v.target_lat = v.lat + sign(rng) * rng.random_range(3.0..12.0);
    v.timer = 0.0;
}

// ---------------------------------------------------------------------------------------------------------
// Mines

fn check_mines(
    mut commands: Commands,
    time: Res<Time>,
    models: Res<Models>,
    palette: Res<Palette>,
    mut mf: ResMut<Minefield>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut paint: ResMut<PaintQueue>,
    mut stats: ResMut<Stats>,
    mut radio: ResMut<Radio>,
    mut sound: Sound,
    mut blasts: MessageWriter<Blast>,
    mut rng: ResMut<UnitRng>,
    mut q: Query<(Entity, &mut Vehicle, &Transform)>,
    mut near: Local<Vec<usize>>,
) {
    let now = time.elapsed_secs_f64();
    for (_e, mut v, tf) in &mut q {
        if !v.state.mobile() || v.speed < 0.2 {
            continue;
        }
        let p = tf.translation.xz();
        let fwd3 = tf.rotation * Vec3::NEG_Z;
        let fwd = fwd3.xz().normalize_or_zero();
        let right = Vec2::new(-fwd.y, fwd.x);
        mf.near_into(p, 6.0, &mut near);
        for &i in near.iter() {
            let m = mf.mines[i];
            if m.ap {
                continue;
            }
            let d = m.pos - p;
            let (f, x) = (d.dot(fwd), d.dot(right));
            // The plow's teeth turn mines out of the ground ahead of the tracks; now and then one goes off.
            if v.plow && (-4.7..-3.6).contains(&-f) && x.abs() < 1.9 {
                mf.mines[i].live = false;
                if let Some(ent) = m.entity {
                    commands.entity(ent).despawn();
                }
                // A stacked pair wrecks the plow or rollers outright (a 47th T-64 commander: rollers survive "up to
                // four explosions", but stacked mines destroyed one after a single blast).
                if rng.0.random_bool(if m.stacked { 0.45 } else { 0.12 }) {
                    detonate(&mut commands, &mut fx, &mut fxr.0, &mut paint, &mut sound, &mut blasts, m.pos);
                    v.plow = false;
                    if m.stacked && rng.0.random_bool(0.5) {
                        v.immobilise(rng.0.random_range(3.0..5.0), rng.0.random_range(-0.3..0.3));
                        v.crew_plan(&mut rng.0, Cause::Mine);
                        v.pitch_v += rng.0.random_range(2.0..3.0);
                        stats.mine_hits += 1;
                        radio.say(now, "Double mine under the plow. Plow's gone, he's not moving. The lane stops here.");
                    } else {
                        radio.say(now, match diorama() {
                            Diorama::Vuhledar => "Mine under the plow. The lead tank's plow is gone, it keeps going.",
                            Diorama::Robotyne => "Mine under the plow. Plow's gone. The rest of the lane is not cleared.",
                        });
                    }
                }
                continue;
            }
            if f.abs() < 3.3 && (x.abs() - 1.4).abs() < 0.5 {
                mf.mines[i].live = false;
                if let Some(ent) = m.entity {
                    commands.entity(ent).despawn();
                }
                detonate(&mut commands, &mut fx, &mut fxr.0, &mut paint, &mut sound, &mut blasts, m.pos);
                stats.mine_hits += 1;
                // TM-62 under a track: the track and road wheels are gone. Often more.
                let kill = rng.0.random_bool(v.kind.mine_kill(m.stacked));
                // The track is off on that side: it lurches, yaws round the dead side and grinds to a stop within
                // a length or two; the cut track lies out behind it, a road wheel beside it.
                v.immobilise(rng.0.random_range(2.0..3.5), x.signum() * -rng.0.random_range(0.35..0.8));
                v.crew_plan(&mut rng.0, Cause::Mine);
                v.pitch_v += rng.0.random_range(1.8..3.0);
                v.roll_v += x.signum() * rng.0.random_range(1.5..2.5);
                let side = right * x.signum();
                let yaw = v.yaw;
                for (mesh, at, spin) in [(&models.track_strip, p - fwd * 4.5 + side * 1.4, 0.0), (&models.road_wheel, p + side * 2.4 - fwd * rng.0.random_range(0.5..2.5), rng.0.random_range(-0.4..0.4))] {
                    commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(at)).with_rotation(ground_rotation(at, yaw + spin)), NotShadowCaster));
                }
                if kill {
                    if rng.0.random_bool(0.15) {
                        // The blast starts a fire in the engine bay instead: the crew gets out, and whether it
                        // burns down to a wreck is decided over the next minutes.
                        commands.entity(_e).insert(Fire::deck(&mut rng.0, Cause::Mine, v.kind));
                        commands.entity(_e).insert(Emitter::new(EmitterKind::EngineSmoke, 4.0, 400.0, Vec3::new(0.0, 1.5, 2.2)));
                    } else {
                        v.state = VState::Destroyed;
                        commands.trigger(Wrecked { vehicle: _e, cause: Cause::Mine, how: kill_how(v.kind, Cause::Mine, &mut rng.0) });
                    }
                }
                let lines = match diorama() {
                    Diorama::Vuhledar => [
                        "Mine! He went off the road. That one's done.",
                        "Another one on the shoulder. Tracks off.",
                        "He drove right past the last wreck, same line. Mine.",
                        "Boom. Mine. He's not moving.",
                    ],
                    Diorama::Robotyne => [
                        "Mine! Left track gone. He's stuck in the lane.",
                        "He went round the Leopard. Off the lane. Mine.",
                        "Another one. They're everywhere off the plowed strip.",
                        "Mine. Crew's alive, they're getting out.",
                    ],
                };
                if rng.0.random_bool(0.6) {
                    radio.say(now, lines[rng.0.random_range(0..lines.len())]);
                }
                break;
            }
        }
    }
}

fn detonate(commands: &mut Commands, fx: &mut FxAssets, rng: &mut SmallRng, paint: &mut PaintQueue, sound: &mut Sound, blasts: &mut MessageWriter<Blast>, at: Vec2) {
    let p = geo::ground(at);
    fx::explosion(commands, fx, p, 1.1, true);
    paint.push(Paint::Crater { at, radius: 2.0, seed: rng.random() });
    sound.at(p, Sfx::MineBlast, 1.0);
    blasts.write(Blast { at: p, radius: 6.0, cause: Cause::Mine });
}

/// A vehicle has just been destroyed: char it, set it burning, maybe throw its turret.
#[derive(Event, Clone, Copy)]
pub struct Wrecked {
    pub vehicle: Entity,
    pub cause: Cause,
    pub how: Kill,
}

/// How a vehicle dies (§2.5 of design/next-items.md).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kill {
    /// The hit reached the charges: the fireball and the turret now.
    BrewUp,
    /// It burns; the charges go over the next minutes, and a carousel tank may throw its turret on one of them.
    Torch,
    /// A fire that grew into the fighting compartment: no blast, just the torch from here.
    Quiet,
}

/// Which death a kill is: Soviet hulls brew up at once when the hit reaches the carousel (an ATGM or a tank
/// round more often than a mine); Western hulls and the MT-LB burn instead.
fn kill_how(kind: VKind, cause: Cause, rng: &mut SmallRng) -> Kill {
    if kind.western() || kind == VKind::Mtlb {
        return Kill::Torch;
    }
    let p_brew = match cause {
        Cause::Atgm | Cause::TankGun => 0.4,
        Cause::Mine => 0.25,
        _ => 0.3,
    };
    if rng.random_bool(p_brew) { Kill::BrewUp } else { Kill::Torch }
}

/// How a knocked-out vehicle burns, from the first smoke to the next day's wisps.
#[derive(Component)]
pub struct Fire {
    pub phase: Blaze,
    pub cause: Cause,
    pub t: f32,
    /// Deck: when the fire reaches the fighting compartment (None: it dies down instead).
    pub escalate_at: Option<f32>,
    /// Torch: when the next charges go, how many bursts are left, and when the turret goes, if it does.
    pub next_burst: f32,
    pub bursts: u32,
    pub toss_at: Option<f32>,
    /// Torch: how long until it burns down to a smoulder.
    pub torch_secs: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Blaze {
    /// A fire in the engine bay: white smoke, then grey; it may grow or die.
    Deck,
    /// The whole hull.
    Torch,
    /// The next day's wisps.
    Smoulder,
}

impl Fire {
    /// An engine-bay fire: six in ten grow into the hull within a minute to three.
    fn deck(rng: &mut SmallRng, cause: Cause, _kind: VKind) -> Self {
        Self { phase: Blaze::Deck, cause, t: 0.0, escalate_at: rng.random_bool(0.6).then(|| rng.random_range(60.0..180.0)), next_burst: 0.0, bursts: 0, toss_at: None, torch_secs: 0.0 }
    }

    /// The hull burning: a carousel tank's charges go 4–10 times over the next minutes and may take the turret
    /// with them; a BMP's a few times; a Leopard vents once through its blow-out panel; a Bradley's rounds
    /// crackle; an MT-LB has nothing to burst.
    fn torch(rng: &mut SmallRng, cause: Cause, kind: VKind, toss: bool, torch_secs: f32) -> Self {
        let bursts = match kind {
            VKind::T80 | VKind::T72 => rng.random_range(4..=10),
            VKind::Bmp1 | VKind::Bmp2 => rng.random_range(3..=6),
            VKind::Leo2a6 => 1,
            VKind::Bradley => rng.random_range(2..=4),
            _ => 0,
        };
        let toss_at = (toss && kind.carousel()).then(|| rng.random_range(20.0..300.0));
        Self { phase: Blaze::Torch, cause, t: 0.0, escalate_at: None, next_burst: rng.random_range(5.0..40.0), bursts: bursts.max(toss_at.is_some() as u32), toss_at, torch_secs }
    }
}

/// How hard a kind's charges go off when they cook.
fn burst_strength(kind: VKind) -> f32 {
    match kind {
        VKind::T80 | VKind::T72 => 1.0,
        VKind::Bmp1 | VKind::Bmp2 => 0.8,
        VKind::Leo2a6 => 0.6,
        _ => 0.3,
    }
}

/// The charges go all at once: the fireball, its sound, and the blast on whoever stands near.
fn cook_off_blast(commands: &mut Commands, fx: &mut FxAssets, sound: &mut Sound, blasts: &mut MessageWriter<Blast>, p: Vec3) {
    fx::fireball(commands, fx, p);
    sound.at(p, Sfx::CookOff, 1.4);
    blasts.write(Blast { at: p, radius: 8.0, cause: Cause::CookOff });
}

/// The turret goes up: typically a few metres, down beside the hull, inverted; one in ten to fifteen or thirty.
fn toss_turret(commands: &mut Commands, rng: &mut SmallRng, stats: &mut Stats, radio: &mut Radio, v: &Vehicle, turret: &GlobalTransform, now: f64) {
    stats.turret_tosses += 1;
    let h = if rng.random_bool(0.1) { rng.random_range(15.0..30.0) } else { rng.random_range(2.0..8.0) };
    let vy = (2.0 * 9.81 * h as f32).sqrt();
    let a = rng.random_range(0.0..std::f32::consts::TAU);
    let side = rng.random_range(1.0..4.0);
    let world = turret.compute_transform();
    commands.entity(v.turret).remove::<ChildOf>().insert((
        Transform::from_translation(world.translation).with_rotation(world.rotation),
        Debris { vel: Vec3::new(a.cos() * side, vy, a.sin() * side), spin: Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(-2.0..2.0), rng.random_range(-3.0..3.0)) },
    ));
    if rng.random_bool(0.6) {
        radio.say(now, "Turret's off. Jack in the box.");
    }
}

/// A vehicle's turret: the skin that chars as the wreck burns, and where it sits, for when the charges throw it.
type TurretMats<'w, 's> = Query<
    'w,
    's,
    (&'static mut MeshMaterial3d<StandardMaterial>, &'static GlobalTransform),
    (With<Turret>, Without<Skin>),
>;

/// The turret of every vehicle, for charring its skin and throwing it when the charges go.
#[derive(bevy::ecs::system::SystemParam)]
pub struct Turrets<'w, 's> {
    mats: TurretMats<'w, 's>,
}

pub fn on_wrecked(
    ev: On<Wrecked>,
    mut commands: Commands,
    time: Res<Time>,
    palette: Res<Palette>,
    models: Res<Models>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut stats: ResMut<Stats>,
    mut radio: ResMut<Radio>,
    mut sound: Sound,
    mut paint: ResMut<PaintQueue>,
    mut blasts: MessageWriter<Blast>,
    mut q: Query<(&mut Vehicle, &Transform, &mut MeshMaterial3d<StandardMaterial>), Without<Turret>>,
    mut turrets: Turrets,
    children: Query<&Children>,
    mut skins: Query<(&Skin, &mut MeshMaterial3d<StandardMaterial>), Without<Vehicle>>,
) {
    let Ok((mut v, tf, mut mat)) = q.get_mut(ev.vehicle) else { return };
    if v.wrecked {
        return;
    }
    v.wrecked = true;
    // An abandoned hull that burns now is no longer "abandoned" in the ledger.
    if v.state == VState::Abandoned {
        if v.side == diorama().attacker() {
            stats.abandoned -= 1;
            stats.wave(v.wave).abandoned -= 1;
        } else {
            stats.defender_vehicles -= 1;
        }
    }
    let now = time.elapsed_secs_f64();
    let rng = &mut fxr.0;
    // It does not stop dead: the driver is gone, the engine dies, the hull rolls on and slews. (A vehicle already
    // immobilised, by a mine say, keeps the slew round its dead track that it already has.)
    if v.state.mobile() {
        v.coast_decel = rng.random_range(2.5..4.0);
        v.slew = rng.random_range(-0.3..0.3);
    }
    v.pitch_v -= rng.random_range(0.8..1.6);
    v.state = VState::Destroyed;
    let m = models.vehicle(v.kind);
    char_skins(ev.vehicle, &mut mat, if v.plow { m.plow_hull.as_ref().unwrap_or(&m.hull) } else { &m.hull }, &children, &mut skins);
    let attacker = v.side == diorama().attacker();
    if attacker {
        stats.destroyed += 1;
        stats.wave(v.wave).destroyed += 1;
    } else {
        stats.defender_vehicles += 1;
    }
    match ev.cause {
        Cause::Atgm => stats.atgm_kills += 1,
        Cause::Artillery | Cause::Himars | Cause::TankGun => stats.artillery_kills += 1,
        _ => {}
    }
    // Who gets out depends on how it dies. A brew-up kills everyone aboard; a hull that catches fire lets most
    // of the crew out, fewer of the desant; in a Leopard or a Bradley most get out anyway: the hatches, the
    // ramp, the ammunition behind blow-out panels (dossier c09 §6: "most crews escaped through open hatches and
    // ramps"). A fire that grew took its time: whoever was aboard is long gone.
    let dead = match ev.how {
        Kill::Quiet => 0,
        _ if v.kind.western() => rng.random_range(0..=1).min(v.crew) + (v.troops as f32 * rng.random_range(0.0..0.3)) as u32,
        Kill::BrewUp => v.crew + (v.troops as f32 * rng.random_range(0.5..0.9)) as u32,
        Kill::Torch => v.crew.min(rng.random_range(0..=(v.crew / 2).max(1))) + (v.troops as f32 * rng.random_range(0.2..0.5)) as u32,
    };
    if attacker {
        stats.attacker_dead += dead;
        stats.died("in vehicles", dead);
    } else {
        stats.defender_losses += dead;
    }
    v.crew -= v.crew.min(dead);
    v.troops = v.troops.saturating_sub(dead);
    let p = tf.translation;
    paint.push(Paint::Scorch { at: p.xz(), radius: 9.0 });
    // Men riding on top are thrown off: a brew-up flings them, a fire tips them into the snow.
    for _ in 0..dead.min(5) {
        let a = rng.random_range(0.0..std::f32::consts::TAU);
        let speed = if ev.how == Kill::BrewUp { rng.random_range(5.0..9.0) } else { rng.random_range(1.5..3.0) };
        throw_body(&mut commands, &models, &palette, v.side, p + Vec3::Y * 2.2, Vec3::new(a.cos() * speed, rng.random_range(2.0..5.0), a.sin() * speed), rng);
    }
    let turret = turrets.mats.get_mut(v.turret).ok().map(|(mut tm, tgt)| {
        char_skins(v.turret, &mut tm, &m.turret, &children, &mut skins);
        *tgt
    });
    let torch_secs = rng.random_range(150.0..320.0);
    let toss_later = ev.how != Kill::BrewUp && v.kind.carousel() && rng.random_bool(0.5);
    match ev.how {
        Kill::BrewUp => {
            cook_off_blast(&mut commands, &mut fx, &mut sound, &mut blasts, p);
            if v.kind.carousel()
                && let Some(t) = turret.as_ref()
            {
                toss_turret(&mut commands, rng, &mut stats, &mut radio, &v, t, now);
            }
        }
        Kill::Torch => {
            fx::explosion(&mut commands, &mut fx, p + Vec3::Y, 0.9, false);
            sound.at(p, Sfx::BlastNear, 1.0);
        }
        Kill::Quiet => {}
    }
    commands.entity(ev.vehicle).insert((Fire::torch(rng, ev.cause, v.kind, toss_later, torch_secs), Emitter::new(EmitterKind::WreckFire, 5.0, torch_secs, Vec3::Y * 1.5)));
    // Fuel running out of a holed tank burns in a pool beside the hull; in June it can set the field alight.
    fx::pool_fire(&mut commands, &mut fx, &mut paint, tf, ev.how != Kill::Quiet);
}

/// The fire on a knocked-out vehicle, minute by minute: an engine-bay fire that grows into the hull or dies
/// down; a burning hull whose charges go off in bursts, one of which may throw the turret; then the smoulder.
fn burn(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut sound: Sound,
    mut stats: ResMut<Stats>,
    mut radio: ResMut<Radio>,
    mut blasts: MessageWriter<Blast>,
    turrets: Query<&GlobalTransform, With<Turret>>,
    mut q: Query<(Entity, &Vehicle, &Transform, &mut Fire)>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let rng = &mut fxr.0;
    for (e, v, tf, mut f) in &mut q {
        f.t += dt;
        let p = tf.translation;
        match f.phase {
            Blaze::Deck => {
                if let Some(at) = f.escalate_at
                    && f.t > at
                {
                    // The fire reaches the fighting compartment: from here it is a wreck.
                    f.escalate_at = None;
                    commands.trigger(Wrecked { vehicle: e, cause: f.cause, how: Kill::Quiet });
                } else if f.escalate_at.is_none() && f.t > 240.0 {
                    f.phase = Blaze::Smoulder;
                }
            }
            Blaze::Torch => {
                let toss = f.toss_at.is_some_and(|at| f.t >= at);
                if toss || (f.bursts > 0 && f.t > f.next_burst) {
                    f.bursts = f.bursts.saturating_sub(1);
                    f.next_burst = f.t + rng.random_range(8.0..90.0);
                    if toss {
                        f.toss_at = None;
                        cook_off_blast(&mut commands, &mut fx, &mut sound, &mut blasts, p);
                        if let Ok(t) = turrets.get(v.turret) {
                            toss_turret(&mut commands, rng, &mut stats, &mut radio, v, t, now);
                        }
                    } else {
                        let s = burst_strength(v.kind);
                        fx::cookoff(&mut commands, &mut fx, p, s);
                        sound.at(p, Sfx::CookOff, 0.4 + 0.5 * s);
                    }
                }
                if f.t > f.torch_secs {
                    f.phase = Blaze::Smoulder;
                }
            }
            Blaze::Smoulder => {}
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// Damage

fn resolve_blasts(
    mut commands: Commands,
    mut blasts: MessageReader<Blast>,
    mut rng: ResMut<UnitRng>,
    mut stats: ResMut<Stats>,
    models: Res<Models>,
    palette: Res<Palette>,
    mut vehicles: Query<(Entity, &mut Vehicle, &Transform)>,
    mut soldiers: Query<(Entity, &mut Soldier, &Transform)>,
) {
    let rng = &mut rng.0;
    for b in blasts.read() {
        let bp = b.at.xz();
        for (e, mut v, tf) in &mut vehicles {
            // A hull abandoned intact is still there to be hit: that is how most of them end.
            if !v.state.alive() && v.state != VState::Abandoned {
                continue;
            }
            let d = bp.distance(tf.translation.xz());
            match b.cause {
                Cause::Mine => {
                    // The mine's own vehicle is handled where it went off.
                    continue;
                }
                Cause::Artillery | Cause::TankGun | Cause::Himars | Cause::Thermobaric | Cause::CookOff => {
                    let (kill_r, dmg_r) = if b.cause == Cause::Himars { (9.0, 20.0) } else { (3.0, 10.0) };
                    if d < kill_r && rng.random_bool(0.7) {
                        commands.trigger(Wrecked { vehicle: e, cause: b.cause, how: kill_how(v.kind, b.cause, rng) });
                    } else if d < dmg_r && v.state != VState::Abandoned {
                        v.panic = (v.panic + 0.5).min(1.0);
                        if v.state.mobile() && rng.random_bool(0.25) {
                            v.immobilise(4.0, rng.random_range(-0.4..0.4));
                            v.crew_plan(rng, b.cause);
                            v.roll_v += rng.random_range(-1.5..1.5);
                        }
                    }
                }
                _ => {}
            }
            if d < 160.0 {
                v.panic = (v.panic + 0.25 * (1.0 - d / 160.0)).min(1.0);
            }
        }
        for (e, mut s, tf) in &mut soldiers {
            let sp = tf.translation.xz();
            let d = bp.distance(sp);
            if d > b.radius * 3.0 {
                if d < 120.0 {
                    s.fear += 0.3;
                }
                // A burst within thirty metres puts a man on his face for a moment.
                if d < 30.0 && matches!(s.state, SState::Advance | SState::Flee) {
                    s.state = SState::Prone;
                    s.timer = 4.0;
                }
                continue;
            }
            // Flat in the snow a man is a little safer; flat in June grass and old craters, much safer.
            let flat = if s.side == Side::Ua { 0.2 } else { 0.4 };
            let prone = if matches!(s.state, SState::Prone | SState::DugIn | SState::Wounded) { flat } else { 1.0 };
            let p_kill = (1.0 - d / (b.radius * 3.0)).powf(1.5) * prone;
            if rng.random_bool(p_kill.clamp(0.0, 1.0) as f64) {
                // In Robotyne most men hit are wounded, not killed: they lie in the grass until someone brings
                // them out (dossier c09 §6: the 47th's losses that day were counted in single figures dead).
                if s.side == Side::Ua && rng.random_bool(if s.state == SState::Wounded { 0.5 } else { 0.75 }) {
                    s.state = SState::Wounded;
                    stats.wounded += 1;
                    continue;
                }
                stats.died(match b.cause {
                    Cause::Mine => "mines",
                    Cause::Atgm | Cause::TankGun => "direct fire",
                    Cause::CookOff => "cook-offs",
                    _ => "shells",
                }, (s.side == diorama().attacker()) as u32);
                // Inside four metres (five of a cook-off) the blast throws the body a few metres; a man killed
                // further out by a fragment drops where he is.
                let throw_r = if b.cause == Cause::CookOff { 5.0 } else { 4.0 };
                let throw = (d < throw_r && b.radius > 3.0).then(|| {
                    let away = (sp - bp).normalize_or(Vec2::from_angle(rng.random_range(0.0..std::f32::consts::TAU)));
                    let speed = rng.random_range(3.0..10.0) * (1.0 - d / throw_r).max(0.3);
                    Vec3::new(away.x * speed, rng.random_range(2.0..5.0), away.y * speed)
                });
                kill_soldier(&mut commands, &models, &palette, &mut stats, e, &s, tf, throw, rng);
            } else {
                s.fear += 1.0;
                // Knocked flat and stunned, five to twenty seconds, when it lands within ten metres.
                if d < 10.0 && b.radius > 3.0 && matches!(s.state, SState::Advance | SState::Flee) {
                    s.state = SState::Prone;
                    s.timer = -rng.random_range(5.0..20.0);
                }
            }
        }
    }
}

/// A body thrown by a blast: it tumbles through the air and lies where it lands.
pub fn throw_body(commands: &mut Commands, models: &Models, palette: &Palette, side: Side, from: Vec3, vel: Vec3, rng: &mut SmallRng) {
    commands.spawn((
        Mesh3d(models.figure(side).fallen.clone()),
        MeshMaterial3d(palette.vertex.clone()),
        Transform::from_translation(from).with_rotation(Quat::from_rotation_y(rng.random_range(0.0..std::f32::consts::TAU))),
        Debris { vel, spin: Vec3::new(0.0, rng.random_range(-4.0..4.0), 0.0) },
        NotShadowCaster,
    ));
}

/// A man killed where he stands, or thrown (`throw`: the blast's push on him) a few metres and left there.
pub fn kill_soldier(commands: &mut Commands, models: &Models, palette: &Palette, stats: &mut Stats, e: Entity, s: &Soldier, tf: &Transform, throw: Option<Vec3>, rng: &mut SmallRng) {
    commands.entity(e).despawn();
    let p = tf.translation.xz();
    if let Some(vel) = throw {
        throw_body(commands, models, palette, s.side, geo::ground(p) + Vec3::Y * 0.8, vel, rng);
    } else {
        let mesh = models.figure(s.side).fallen.clone();
        commands.spawn((Mesh3d(mesh), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(p)).with_rotation(Quat::from_rotation_y(rng.random_range(0.0..std::f32::consts::TAU))), NotShadowCaster));
    }
    if s.side == diorama().attacker() {
        stats.attacker_dead += 1;
    } else {
        stats.defender_losses += 1;
    }
}

/// Reactive armour going off under a hit: a chain of flashes and grey puffs where the blocks were, and the
/// plates thrown off. The look of the hit only: the simulation has already decided what the hit did.
fn era_reacts(mut commands: Commands, mut hits: MessageReader<DirectHit>, mut fx: ResMut<FxAssets>, mut last: ResMut<LastHit>, q: Query<(&Vehicle, &Transform)>) {
    for h in hits.read() {
        if let Ok((_, tf)) = q.get(h.target) {
            last.0 = Some(tf.translation);
        }
        if let Ok((v, tf)) = q.get(h.target)
            && v.kind.era()
            && h.cause != Cause::TankGun
        {
            fx::era_pops(&mut commands, &mut fx, tf, h.top);
            info!("era pops on {:?} {}/{} at {:.0},{:.0}", v.kind, v.wave, v.idx, tf.translation.x, tf.translation.z);
        }
    }
}

fn resolve_direct_hits(mut commands: Commands, mut hits: MessageReader<DirectHit>, mut rng: ResMut<UnitRng>, mut q: Query<&mut Vehicle>) {
    for h in hits.read() {
        let Ok(mut v) = q.get_mut(h.target) else { continue };
        if !v.state.alive() {
            continue;
        }
        let p_kill = if h.top { (v.kind.hit_kill() + 0.2).min(0.95) } else { v.kind.hit_kill() };
        if rng.0.random_bool(p_kill) {
            if rng.0.random_bool(0.2) {
                // A hit on the engine: it crawls on trailing smoke, the crew gets out, and the fire decides.
                v.immobilise(rng.0.random_range(0.6..1.2), rng.0.random_range(-0.2..0.2));
                v.crew_plan(&mut rng.0, h.cause);
                commands.entity(h.target).insert((Fire::deck(&mut rng.0, h.cause, v.kind), Emitter::new(EmitterKind::EngineSmoke, 4.0, 400.0, Vec3::new(0.0, 1.5, 2.2))));
            } else {
                commands.trigger(Wrecked { vehicle: h.target, cause: h.cause, how: kill_how(v.kind, h.cause, &mut rng.0) });
            }
        } else {
            v.immobilise(3.5, rng.0.random_range(-0.3..0.3));
            v.crew_plan(&mut rng.0, h.cause);
            v.pitch_v -= 1.2;
            v.panic = 1.0;
        }
    }
}

/// Where a man who has just got out runs first: the nearest crater, wreck or tree line within reach, chosen
/// among them so that no two men run the same way; or nowhere in particular, but off to one side.
fn cover_near(p: Vec2, home: Vec2, craters: &crate::terrain::Craters, hulls: &[Vec2], rng: &mut SmallRng) -> Vec2 {
    let mut spots: Vec<Vec2> = Vec::new();
    if let Some(c) = craters.nearest(p, 15.0) {
        spots.push(c);
    }
    if let Some(h) = hulls.iter().filter(|h| h.distance(p) > 6.0 && h.distance(p) < 60.0).min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))) {
        // Behind the hull, away from the enemy's line.
        spots.push(*h + (home - *h).normalize_or_zero() * 5.0);
    }
    if let Some((d, s, belt)) = geo::layout().belts.iter().map(|b| { let (d, s) = b.line.project(p); (d, s, b) }).min_by(|a, b| a.0.total_cmp(&b.0))
        && d < 150.0
    {
        spots.push(belt.line.point(s));
    }
    if spots.is_empty() {
        // Away from here, spread out, then home.
        let dir = (home - p).normalize_or_zero();
        let side = Vec2::new(-dir.y, dir.x);
        p + dir * rng.random_range(20.0..40.0) + side * rng.random_range(-40.0..40.0)
    } else {
        spots[rng.random_range(0..spots.len())] + Vec2::new(rng.random_range(-2.0..2.0), rng.random_range(-2.0..2.0))
    }
}

/// Crews of knocked-out vehicles get out one at a time, out of the hatches on either side, once the crew's plan
/// says so (`bail_at`); dismounts from wrecks, if any live, too. A fit crew is out in seconds; a concussed one
/// takes a minute. Each man sprints for the nearest cover before he thinks of the way home.
fn crew_bailout(mut commands: Commands, time: Res<Time>, mut rng: ResMut<UnitRng>, mut stats: ResMut<Stats>, models: Res<Models>, palette: Res<Palette>, paint: Res<PaintQueue>, mut q: Query<(&mut Vehicle, &Transform)>, mut hulls: Local<Vec<Vec2>>) {
    let dt = time.delta_secs();
    let lay = geo::layout();
    hulls.clear();
    hulls.extend(q.iter().filter(|(v, _)| !v.state.mobile()).map(|(_, t)| t.translation.xz()));
    for (mut v, tf) in &mut q {
        if v.crew + v.troops == 0 {
            continue;
        }
        match v.state {
            VState::Immobilized => {
                v.timer += dt;
                if v.timer <= v.bail_at {
                    continue;
                }
            }
            VState::Destroyed | VState::Abandoned => {}
            _ => continue,
        }
        v.exit_timer -= dt;
        if v.exit_timer > 0.0 {
            continue;
        }
        v.exit_timer = rng.0.random_range(0.5..3.0);
        let route = &lay.routes[v.route];
        let p = tf.translation.xz();
        let fwd = (tf.rotation * Vec3::NEG_Z).xz().normalize_or_zero();
        let right = Vec2::new(-fwd.y, fwd.x);
        // Attackers run back down their track; defenders back behind their own line.
        let attacker = v.side == diorama().attacker();
        let home = if attacker { route.line.point(0.0) } else { p + Vec2::new(0.0, 600.0 * lay.advance) };
        // The crew leaves first. Some dismounts go forward anyway (fewer of the 47th's: they had been told the
        // infantry comes back); the rest go for cover, then back.
        let is_crew = v.crew > 0;
        let p_forward = if v.side == Side::Ru { 0.4 } else { 0.15 };
        let forward = attacker && !is_crew && rng.0.random_bool(p_forward);
        let at = p + right * sign(&mut rng.0) * 2.2 + fwd * rng.0.random_range(-1.5..1.5);
        let goal = if forward { route.objective + Vec2::new(rng.0.random_range(-100.0..100.0), 0.0) } else { home };
        let leg = (!forward).then(|| cover_near(at, home, &paint.craters, &hulls, &mut rng.0));
        let e = spawn_soldier(&mut commands, &models, &palette, v.side, at, goal, home, rng.0.random_range(10.0..25.0), leg, &mut rng.0);
        if !forward {
            commands.entity(e).insert(Fleeing);
        }
        if is_crew {
            v.crew -= 1;
        } else {
            v.troops -= 1;
        }
        if v.state == VState::Immobilized {
            v.state = VState::Abandoned;
            if attacker {
                stats.abandoned += 1;
                stats.wave(v.wave).abandoned += 1;
            } else {
                stats.defender_vehicles += 1;
            }
        }
    }
}

#[derive(Component)]
pub struct Fleeing;

// ---------------------------------------------------------------------------------------------------------
// Soldiers

fn move_soldiers(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<UnitRng>,
    mut stats: ResMut<Stats>,
    mut paint: ResMut<PaintQueue>,
    mut mines: FootMines,
    order: Res<ColumnOrder>,
    wrecks: Query<(&Vehicle, &Transform), Without<Soldier>>,
    mut q: Query<(Entity, &mut Soldier, &mut Transform, Option<&Fleeing>)>,
) {
    let dt = crate::sim_dt(&time);
    let rng = &mut rng.0;
    let lay = geo::layout();
    let attacker = diorama().attacker();
    let wreck_pos: Vec<Vec2> = wrecks.iter().filter(|(v, _)| !v.state.mobile()).map(|(_, t)| t.translation.xz()).collect();
    // Bradleys waiting for men: whoever is falling back runs for the nearest one.
    let pickups: Vec<Vec2> = wrecks
        .iter()
        .filter(|(v, _)| v.kind == VKind::Bradley && v.side == attacker && v.state.mobile() && v.troops < 7 && (v.rescue || v.state == VState::Overwatch))
        .map(|(_, t)| t.translation.xz())
        .collect();
    let mut foothold = 0;
    for (e, mut s, mut tf, fleeing) in &mut q {
        if s.side != attacker {
            continue;
        }
        // Ordered back, everyone on foot falls back too, to the vehicles or the start line.
        if (fleeing.is_some() && s.state == SState::Advance) || (*order == ColumnOrder::PullBack && !matches!(s.state, SState::Flee | SState::Wounded)) {
            s.state = SState::Flee;
        }
        s.fear = (s.fear - dt * 0.15).max(0.0);
        s.timer += dt;
        s.sprint = (s.sprint - dt).max(0.0);
        let p = tf.translation.xz();
        let mut speed = 0.0;
        let mut target = p;
        match s.state {
            SState::Advance => {
                target = s.goal;
                speed = s.speed;
                if s.fear > 1.0 {
                    s.state = SState::Prone;
                    s.timer = 0.0;
                }
                if p.distance(s.goal) < 12.0 {
                    s.state = SState::DugIn;
                }
            }
            SState::Prone => {
                if s.timer > rng.random_range(5.0..12.0) {
                    s.timer = 0.0;
                    // "They'd come in groups of five. We'd kill four, one would stay and dig in."
                    let near_wreck = wreck_pos.iter().any(|w| w.distance(p) < 35.0);
                    s.state = if fleeing.is_some() || (s.side == Side::Ua && *order == ColumnOrder::PullBack) {
                        SState::Flee
                    } else if s.fear > 2.2 && near_wreck && rng.random_bool(0.5) {
                        SState::DugIn
                    } else if s.fear > 2.6 && rng.random_bool(0.5) {
                        SState::Flee
                    } else {
                        SState::Advance
                    };
                    s.fear *= 0.6;
                }
            }
            SState::Flee => {
                // First to cover, and down in it for a moment; then the nearest Bradley or home.
                if let Some(leg) = s.leg
                    && p.distance(leg) < 3.0
                {
                    s.leg = None;
                    s.state = SState::Prone;
                    s.timer = rng.random_range(2.0..6.0);
                }
                target = s.leg.unwrap_or_else(|| pickups.iter().copied().filter(|b| b.distance(p) < 160.0).min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))).unwrap_or(s.home));
                // Flat out while the sprint lasts, a jog after.
                speed = s.speed * if s.sprint > 0.0 { 2.2 } else { 1.3 };
                // Falling back under shellfire, the 47th's men drop flat when it lands close, then go on.
                if s.side == Side::Ua && s.fear > 1.2 {
                    s.state = SState::Prone;
                    s.timer = 0.0;
                    s.fear *= 0.5;
                    speed = 0.0;
                }
                if p.distance(s.home) < 25.0 || !geo::in_bounds(p) {
                    stats.returned += 1;
                    commands.entity(e).despawn();
                    continue;
                }
            }
            SState::Wounded => {
                target = pickups.iter().copied().filter(|b| b.distance(p) < 220.0).min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))).unwrap_or(s.home);
                speed = 0.6;
                if p.distance(s.home) < 25.0 {
                    stats.returned += 1;
                    stats.wounded_out += 1;
                    commands.entity(e).despawn();
                    continue;
                }
            }
            SState::DugIn => {}
        }
        if speed > 0.0 {
            let d = (target - p).normalize_or_zero();
            let wob = Vec2::new((s.timer * 0.7 + s.phase).sin(), (s.timer * 0.5 + s.phase).cos()) * 0.25;
            let step = (d + wob).normalize_or_zero() * speed * dt;
            let np = p + step;
            tf.translation = geo::ground(np);
            tf.rotation = Quat::from_rotation_y(vehicle_yaw(step));
            if s.state == SState::Wounded {
                // Crawling: flat on the ground, head first.
                tf.translation.y += 0.25;
                tf.rotation *= Quat::from_rotation_x(-1.45);
            }
            s.phase += speed * dt * 3.2;
            if rng.random_bool((dt * 0.6) as f64) {
                paint.push(Paint::Footprints { at: np });
            }
            mines.step(&mut commands, np);
        } else {
            // Prone or dug in: lie flat.
            tf.translation = geo::ground(p) + Vec3::Y * if s.state == SState::DugIn { -0.6 } else { -0.75 };
        }
        // A foothold: attackers alive at the defenders' forward line or past it.
        if matches!(s.state, SState::DugIn | SState::Prone | SState::Advance) && (p.y - lay.foothold_z) * lay.advance > 0.0 {
            foothold += 1;
        }
        stats.max_advance = stats.max_advance.max(p.y * lay.advance);
    }
    stats.foothold = foothold;
}

/// Anti-personnel mines among the anti-tank ones, and what a soldier's step needs to set one off.
#[derive(bevy::ecs::system::SystemParam)]
struct FootMines<'w, 's> {
    mf: ResMut<'w, Minefield>,
    fx: ResMut<'w, FxAssets>,
    sound: Sound<'w, 's>,
    blasts: MessageWriter<'w, Blast>,
    near: Local<'s, Vec<usize>>,
}

impl FootMines<'_, '_> {
    /// A foot comes down at `p`: if it finds an anti-personnel mine, the mine goes off.
    fn step(&mut self, commands: &mut Commands, p: Vec2) {
        let mf = &mut self.mf;
        mf.near_into(p, 0.6, &mut self.near);
        let Some(i) = self.near.iter().copied().find(|&i| mf.mines[i].ap && mf.mines[i].pos.distance(p) < 0.45) else { return };
        mf.mines[i].live = false;
        let at = geo::ground(p);
        fx::explosion(commands, &mut self.fx, at, 0.35, true);
        self.sound.at(at, Sfx::MineBlast, 0.5);
        self.blasts.write(Blast { at, radius: 2.5, cause: Cause::Mine });
    }
}

/// Men in the field climb onto a Bradley that has stopped near them, or is backing out slowly: the squad it
/// dropped, a crew whose vehicle burned, anyone. "A Bradley crew pops smoke and covers the dismounts as they
/// climb onto it" (dossier c09 §6).
fn board_vehicles(mut commands: Commands, mut stats: ResMut<Stats>, mut vehicles: Query<(&mut Vehicle, &Transform)>, soldiers: Query<(Entity, &Soldier, &Transform)>) {
    let mut taken: Vec<Entity> = Vec::new();
    for (mut v, tf) in &mut vehicles {
        if v.kind != VKind::Bradley || !v.state.mobile() || v.speed > 2.0 || v.state == VState::Dismounting || v.troops >= 7 {
            continue;
        }
        // A squad that has just got out does not get straight back in, unless the vehicle is leaving.
        if !(v.rescue || v.state == VState::Withdrawing || v.state == VState::Overwatch) {
            continue;
        }
        let p = tf.translation.xz();
        for (e, s, st) in &soldiers {
            if s.side != v.side || taken.contains(&e) || v.troops >= 7 {
                continue;
            }
            let fleeing = matches!(s.state, SState::Flee | SState::Prone | SState::Wounded);
            if fleeing && st.translation.xz().distance(p) < 14.0 {
                taken.push(e);
                commands.entity(e).despawn();
                v.troops += 1;
                stats.picked_up += 1;
                if s.state == SState::Wounded {
                    stats.wounded_out += 1;
                }
            }
        }
    }
}

fn animate_legs(soldiers: Query<(&Soldier, &Children)>, mut legs: Query<(&Leg, &mut Transform)>) {
    for (s, children) in &soldiers {
        let moving = matches!(s.state, SState::Advance | SState::Flee);
        for c in children.iter() {
            if let Ok((leg, mut tf)) = legs.get_mut(c) {
                let a = if moving { (s.phase + leg.0).sin() * 0.6 } else { 0.0 };
                tf.rotation = Quat::from_rotation_x(a);
            }
        }
    }
}

fn fly_debris(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Debris, &mut Transform)>) {
    let dt = crate::sim_dt(&time);
    for (e, mut d, mut tf) in &mut q {
        d.vel.y -= 9.81 * dt;
        tf.translation += d.vel * dt;
        let spin = d.spin * dt;
        tf.rotate(Quat::from_euler(EulerRot::XYZ, spin.x, spin.y, spin.z));
        let g = geo::height(tf.translation.x, tf.translation.z);
        if tf.translation.y < g + 0.3 && d.vel.y < 0.0 {
            tf.translation.y = g + 0.3;
            commands.entity(e).remove::<Debris>();
        }
    }
}

/// Riders get off when the section dismounts or bails out, and are thrown off when the vehicle is hit.
fn drop_riders(mut commands: Commands, vehicles: Query<(&Vehicle, &Children)>, riders: Query<(), With<Rider>>) {
    for (v, children) in &vehicles {
        if v.troops > 0 && v.state.mobile() {
            continue;
        }
        for c in children.iter() {
            if riders.contains(c) {
                commands.entity(c).despawn();
            }
        }
    }
}
