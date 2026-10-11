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
use crate::meshes::{self, MUD, RU_GREEN, UA_GREEN, WEST_GREEN};
use crate::terrain::{Paint, PaintQueue};
use crate::world::Palette;
use bevy::{light::NotShadowCaster, platform::collections::HashMap, prelude::*};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use strum::VariantArray;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Blast>()
            .add_message::<DirectHit>()
            .insert_resource(UnitRng(SmallRng::seed_from_u64(155)))
            .insert_resource(Stats { vehicle_advance: f32::MIN, max_advance: if diorama() == Diorama::Robotyne { f32::MIN } else { 0.0 }, ..default() })
            .init_resource::<Radio>()
            .init_resource::<ColumnOrder>()
            .add_systems(Startup, (setup_models, lay_minefields, spawn_old_wrecks).chain().after(crate::world::spawn_belts))
            .add_systems(
                Update,
                (drive_vehicles, coast, check_mines, resolve_blasts, resolve_direct_hits, crew_bailout, move_soldiers, board_vehicles, animate_legs, fly_debris, drop_riders)
                    .chain()
                    .run_if(crate::running)
                    .in_set(crate::Step::Units),
            );
    }
}

#[derive(Resource)]
pub struct UnitRng(pub SmallRng);

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

#[derive(Resource, Default)]
pub struct Radio {
    pub lines: Vec<(f64, String)>,
    pub new: Vec<String>,
}

impl Radio {
    pub fn say(&mut self, now: f64, s: impl Into<String>) {
        let s = s.into();
        self.lines.push((now, s.clone()));
        self.new.push(s);
    }
}

/// One kind of Russian vehicle: its hull (and plow-tank hull), its turret and where the turret sits.
pub struct VehicleMeshes {
    pub hull: Handle<Mesh>,
    pub plow_hull: Option<Handle<Mesh>>,
    pub turret: Handle<Mesh>,
    pub turret_at: Vec3,
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
    pub ua_tank: (Handle<Mesh>, Handle<Mesh>, Vec3),
    /// A lane marker: a stake with an orange flag, which the Leopard 2R drops along the lane it clears.
    pub marker: Handle<Mesh>,
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

pub fn setup_models(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Slight per-type colour variation, and mud on the lower hull.
    let paint = |k: VKind| {
        let v = 1.0 + (k as u32 as f32 - 2.0) * 0.03;
        if k.western() || k == VKind::Bmr2 {
            let v = 1.0 + (k as u32 as f32 - 7.0) * 0.025;
            return [WEST_GREEN[0] * v, WEST_GREEN[1] * v, WEST_GREEN[2] * v];
        }
        [RU_GREEN[0] * v, RU_GREEN[1] * v, RU_GREEN[2] * v * 0.98]
    };
    let hull = |meshes: &mut Assets<Mesh>, mut m: meshes::MeshBuilder| {
        m.tint([MUD[0] / 0.33, MUD[1] / 0.29, MUD[2] / 0.24].map(|x: f32| 0.9 + 0.1 * x));
        meshes.add(m.build())
    };
    let mut vehicles = HashMap::new();
    for &k in VKind::VARIANTS {
        let m = match k {
            VKind::T80 | VKind::T72 => meshes::tank(paint(k), false, false),
            VKind::Bmp1 => meshes::bmp(paint(k), false),
            VKind::Bmp2 => meshes::bmp(paint(k), true),
            VKind::Mtlb => meshes::mtlb(paint(k)),
            VKind::Leo2a6 => meshes::leopard2a6(paint(k)),
            VKind::Leo2r => meshes::leopard2r(paint(k)),
            VKind::Wisent => meshes::wisent(paint(k)),
            VKind::Bmr2 => meshes::bmr2(paint(k)),
            VKind::Bradley => meshes::bradley(paint(k)),
        };
        let plow_hull = k.carousel().then(|| hull(&mut meshes, meshes::tank(paint(k), true, false).hull));
        vehicles.insert(k, VehicleMeshes { hull: hull(&mut meshes, m.hull), plow_hull, turret: meshes.add(m.turret.build()), turret_at: m.turret_at });
    }
    let ua = meshes::tank(UA_GREEN, false, true);
    let ua_tank = (meshes.add(ua.hull.build()), meshes.add(ua.turret.build()), ua.turret_at);
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
    commands.insert_resource(Models { vehicles, ru, ua, hip, raam: meshes.add(raam.build()), missile: meshes.add(missile.build()), ua_tank, marker: meshes.add(marker.build()) });
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

    pub fn near(&self, p: Vec2, r: f32) -> Vec<usize> {
        let mut out = Vec::new();
        let (x0, x1) = (((p.x - r) / MINE_CELL).floor() as i32, ((p.x + r) / MINE_CELL).floor() as i32);
        let (z0, z1) = (((p.y - r) / MINE_CELL).floor() as i32, ((p.y + r) / MINE_CELL).floor() as i32);
        for cz in z0..=z1 {
            for cx in x0..=x1 {
                if let Some(v) = self.grid.get(&(cx, cz)) {
                    out.extend(v.iter().copied().filter(|&i| self.mines[i].live));
                }
            }
        }
        out
    }
}

fn lay_minefields(mut commands: Commands) {
    let lay = geo::layout();
    let mut mf = Minefield::default();
    let mut rng = SmallRng::seed_from_u64(72);
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
    let turret = commands.spawn((Mesh3d(m.turret.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(m.turret_at), Turret)).id();
    let mut e = commands.spawn((
        Mesh3d(plow_hull.unwrap_or(&m.hull).clone()),
        MeshMaterial3d(palette.vertex.clone()),
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
        },
    ));
    e.add_child(turret);
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

fn spawn_old_wrecks(mut commands: Commands, models: Res<Models>, palette: Res<Palette>) {
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
        let mut e = commands.spawn((
            Mesh3d(m.hull.clone()),
            MeshMaterial3d(palette.charred.clone()),
            Transform::from_translation(geo::ground(*p) - Vec3::Y * 0.15).with_rotation(ground_rotation(*p, *yaw)),
        ));
        if !toss {
            e.with_child((Mesh3d(m.turret.clone()), MeshMaterial3d(palette.charred.clone()), Transform::from_translation(m.turret_at).with_rotation(Quat::from_rotation_y(i as f32))));
        }
        if i % 3 == 0 {
            e.insert(Emitter::new(EmitterKind::Smoulder, 0.5, 1.0e5, Vec3::Y));
        }
        if toss {
            let tp = *p + Vec2::from_angle(i as f32 * 1.7) * 14.0;
            commands.spawn((Mesh3d(m.turret.clone()), MeshMaterial3d(palette.charred.clone()), Transform::from_translation(geo::ground(tp) + Vec3::Y * 0.3).with_rotation(Quat::from_euler(EulerRot::XYZ, 2.6, i as f32, 0.3))));
        }
    }
}

pub fn spawn_soldier(commands: &mut Commands, models: &Models, palette: &Palette, side: Side, at: Vec2, goal: Vec2, home: Vec2, rng: &mut SmallRng) -> Entity {
    let (body, leg) = (models.figure(side).body.clone(), models.figure(side).leg.clone());
    let hip = models.hip;
    let l0 = commands.spawn((Mesh3d(leg.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(hip), Leg(0.0), NotShadowCaster)).id();
    let l1 = commands.spawn((Mesh3d(leg), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(hip * Vec3::new(-1.0, 1.0, 1.0)), Leg(std::f32::consts::PI), NotShadowCaster)).id();
    let mut e = commands.spawn((
        Mesh3d(body),
        MeshMaterial3d(palette.vertex.clone()),
        Transform::from_translation(geo::ground(at)),
        Soldier { side, state: SState::Advance, goal, home, speed: rng.random_range(1.5..2.3), fear: 0.0, timer: 0.0, phase: rng.random_range(0.0..6.0) },
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
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let lay = geo::layout();
    let attacker = diorama().attacker();
    let snaps: Vec<Snap> = q.iter().map(|(e, v, _)| Snap { e, route: v.route, s: v.s, lat: v.lat, state: v.state, speed: v.speed, reversing: v.reversing }).collect();
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
        for o in &snaps {
            if o.e == e || o.route != v.route {
                continue;
            }
            let ds = (o.s - v.s) * dir;
            // Judge the lane I am steering into, so a vehicle pulling out round a wreck is not blocked by it.
            if ds > 0.0 && ds < 55.0 && (o.lat - v.target_lat).abs() < 6.0 && blocker.is_none_or(|b| (b.s - v.s).abs() > ds) {
                blocker = Some(*o);
            }
        }

        let mut want_speed = v.kind.speed();
        // A breacher drives with its plow up until the minefield, then digs in at a walking pace.
        if v.kind.breacher() && !lay.barriers.iter().any(|b| (b.z0 - 60.0..b.z1).contains(&tf.translation.z)) {
            want_speed = 4.6;
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
                        want_speed = want_speed.min(b.speed * (gap / 40.0).clamp(0.0, 1.2));
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
                    v.state = VState::Advancing;
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
                        spawn_soldier(&mut commands, &models, &palette, v.side, at, goal, route.point(0.0), rng);
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
                    radio.say(now, format!("Bradley coming out with {} aboard.", v.troops));
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
                want_speed = v.kind.speed() * 1.1;
                if let Some(b) = blocker
                    && !b.state.mobile()
                    && (b.s - v.s).abs() < 25.0
                {
                    v.target_lat = b.lat + if v.lat > b.lat { 10.0 } else { -10.0 };
                }
            }
            _ => {}
        }

        // Move in route coordinates. A tank accelerates at about 1 m/s² and brakes at about 2; it brakes hard only
        // when something is right in front of it.
        let hard = blocker.is_some_and(|b| (b.s - v.s).abs() < 22.0) || v.panic > 0.7;
        let prev_speed = v.speed;
        v.speed += (want_speed - v.speed).clamp(-if hard { 3.5 } else { 1.8 } * dt, 1.1 * dt);
        let lat_rate = (v.target_lat - v.lat).clamp(-2.5, 2.5);
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
        // Suspension: nose down under braking, squat under power, roll in turns, pitching over the ruts.
        let accel = (v.speed - prev_speed) / dt.max(1e-4) * if v.reversing { -1.0 } else { 1.0 };
        let rough = geo::value_noise(v.s * 0.35, v.idx as f32 * 3.1, 77) * 0.03 * (v.speed / 5.0);
        let roll_target = -dyaw / dt.max(1e-4) * v.speed * 0.012;
        suspension(&mut v, accel * 0.022 + rough, roll_target, dt);
        tf.rotation = ground_rotation(p, yaw) * Quat::from_rotation_x(v.pitch) * Quat::from_rotation_z(v.roll);
        let moved = old.distance(p);
        v.travelled += moved;
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
            if v.speed > 2.0 && rng.random_bool(0.5) {
                fx::spray(&mut commands, &mut fx, &mut fxr.0, geo::ground(p) + Vec3::new(0.0, 0.0, 0.0) - (heading * 3.0).extend(0.0).xzy(), (heading * v.speed).extend(0.0).xzy());
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
fn coast(time: Res<Time>, mut paint: ResMut<PaintQueue>, mut q: Query<(&mut Vehicle, &mut Transform)>) {
    let dt = crate::sim_dt(&time);
    let lay = geo::layout();
    for (mut v, mut tf) in &mut q {
        if v.state.mobile() {
            continue;
        }
        let moving = v.speed > 0.02;
        if !moving && v.pitch.abs() < 1e-4 && v.pitch_v.abs() < 1e-4 && v.roll.abs() < 1e-4 && v.roll_v.abs() < 1e-4 {
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

/// Smoke grenades: a white wall in front of the vehicle within seconds, which hides it from the ATGM teams'
/// sights, and from the drone overhead.
pub fn pop_smoke(commands: &mut Commands, fx: &mut FxAssets, rng: &mut SmallRng, smoke: &mut fx::Smoke, v: &mut Vehicle, tf: &Transform, now: f64) {
    if v.smoked {
        return;
    }
    v.smoked = true;
    let fwd = (tf.rotation * Vec3::NEG_Z).xz().normalize_or_zero();
    for k in 0..8 {
        let a = (k as f32 - 3.5) * 0.28;
        let dir = Vec2::from_angle(a).rotate(fwd);
        let at = tf.translation + Vec3::new(dir.x, 0.0, dir.y) * 2.0 + Vec3::Y * 2.2;
        for _ in 0..3 {
            fx::spawn(commands, fx, fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: at, vel: Vec3::new(dir.x, 0.35, dir.y) * rng.random_range(14.0..22.0), life: rng.random_range(50.0..80.0), size0: 2.0, size1: rng.random_range(18.0..30.0), alpha: 0.75, drag: 0.9, ..default() });
        }
    }
    smoke.add(tf.translation.xz() + fwd * 22.0, 28.0, now + 60.0);
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
        for i in mf.near(p, 6.0) {
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
                // The track is off on that side: it lurches, slews round the dead side and grinds to a stop.
                v.immobilise(rng.0.random_range(4.0..6.5), x.signum() * -rng.0.random_range(0.35..0.8));
                v.pitch_v += rng.0.random_range(1.8..3.0);
                v.roll_v += x.signum() * rng.0.random_range(1.5..2.5);
                if kill {
                    v.state = VState::Destroyed;
                    commands.trigger(Wrecked { vehicle: _e, cause: Cause::Mine });
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
    fx::explosion(commands, fx, rng, p, 1.1, true);
    paint.push(Paint::Crater { at, radius: 2.0, seed: rng.random() });
    sound.at(p, Sfx::MineBlast, 1.0);
    blasts.write(Blast { at: p, radius: 6.0, cause: Cause::Mine });
}

/// A vehicle has just been destroyed: char it, set it burning, maybe throw its turret.
#[derive(Event, Clone, Copy)]
pub struct Wrecked {
    pub vehicle: Entity,
    pub cause: Cause,
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
    mut q: Query<(&mut Vehicle, &Transform, &mut MeshMaterial3d<StandardMaterial>), Without<Turret>>,
    mut turrets: Query<(&mut MeshMaterial3d<StandardMaterial>, &GlobalTransform), With<Turret>>,
) {
    let Ok((mut v, tf, mut mat)) = q.get_mut(ev.vehicle) else { return };
    if v.wrecked {
        return;
    }
    v.wrecked = true;
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
    mat.0 = palette.charred.clone();
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
    // Most of those inside do not get out. In a Leopard or a Bradley most do: the hatches, the ramp, the
    // ammunition behind blow-out panels (dossier c09 §6: "most crews escaped through open hatches and ramps").
    let dead = if v.kind.western() {
        rng.random_range(0..=1).min(v.crew) + (v.troops as f32 * rng.random_range(0.0..0.3)) as u32
    } else {
        v.crew.min(rng.random_range(1..=v.crew.max(1))) + (v.troops as f32 * rng.random_range(0.4..0.9)) as u32
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
    // Men riding on top are thrown into the snow.
    for k in 0..dead.min(5) {
        let at = p.xz() + Vec2::from_angle(rng.random_range(0.0..std::f32::consts::TAU)) * rng.random_range(3.0..9.0);
        commands.spawn((Mesh3d(models.figure(v.side).fallen.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(at)).with_rotation(Quat::from_rotation_y(k as f32 * 1.9)), NotShadowCaster));
    }
    fx::explosion(&mut commands, &mut fx, rng, p + Vec3::Y, 1.4, false);
    sound.at(p, Sfx::BlastNear, 1.3);
    commands.entity(ev.vehicle).insert(Emitter::new(EmitterKind::WreckFire, 5.0, rng.random_range(150.0..320.0), Vec3::Y * 1.5));
    if let Ok((mut tm, tgt)) = turrets.get_mut(v.turret) {
        tm.0 = palette.charred.clone();
        if v.kind.carousel() && rng.random_bool(0.4) {
            stats.turret_tosses += 1;
            let world = tgt.compute_transform();
            commands.entity(v.turret).remove::<ChildOf>().insert((
                Transform::from_translation(world.translation).with_rotation(world.rotation),
                Debris { vel: Vec3::new(rng.random_range(-6.0..6.0), rng.random_range(18.0..32.0), rng.random_range(-6.0..6.0)), spin: Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(-2.0..2.0), rng.random_range(-3.0..3.0)) },
            ));
            sound.at(p, Sfx::CookOff, 1.4);
            fx::flash(&mut commands, p + Vec3::Y * 4.0, 1.5e8, 300.0, 1.2);
            for _ in 0..14 {
                fx::spawn(&mut commands, &mut fx, fx::Spawn { kind: fx::Kind::Fire, pos: p + Vec3::Y * 2.0, vel: Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(10.0..30.0), rng.random_range(-3.0..3.0)), life: rng.random_range(0.6..1.4), size0: 3.0, size1: 7.0, alpha: 1.0, drag: 1.5, ..default() });
            }
            if rng.random_bool(0.6) {
                radio.say(now, "Turret's off. Jack in the box.");
            }
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
            if !v.state.alive() {
                continue;
            }
            let d = bp.distance(tf.translation.xz());
            match b.cause {
                Cause::Mine => {
                    // The mine's own vehicle is handled where it went off.
                    continue;
                }
                Cause::Artillery | Cause::TankGun | Cause::Himars | Cause::Thermobaric => {
                    let (kill_r, dmg_r) = if b.cause == Cause::Himars { (9.0, 20.0) } else { (3.0, 10.0) };
                    if d < kill_r && rng.random_bool(0.7) {
                        commands.trigger(Wrecked { vehicle: e, cause: b.cause });
                    } else if d < dmg_r {
                        v.panic = (v.panic + 0.5).min(1.0);
                        if v.state.mobile() && rng.random_bool(0.25) {
                            v.immobilise(4.0, rng.random_range(-0.4..0.4));
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
            let d = bp.distance(tf.translation.xz());
            if d > b.radius * 3.0 {
                if d < 120.0 {
                    s.fear += 0.3;
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
                    _ => "shells",
                }, (s.side == diorama().attacker()) as u32);
                kill_soldier(&mut commands, &models, &palette, &mut stats, e, &s, tf, rng);
            } else {
                s.fear += 1.0;
            }
        }
    }
}

pub fn kill_soldier(commands: &mut Commands, models: &Models, palette: &Palette, stats: &mut Stats, e: Entity, s: &Soldier, tf: &Transform, rng: &mut SmallRng) {
    commands.entity(e).despawn();
    let mesh = models.figure(s.side).fallen.clone();
    let p = tf.translation.xz();
    commands.spawn((Mesh3d(mesh), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(p)).with_rotation(Quat::from_rotation_y(rng.random_range(0.0..std::f32::consts::TAU))), NotShadowCaster));
    if s.side == diorama().attacker() {
        stats.attacker_dead += 1;
    } else {
        stats.defender_losses += 1;
    }
}

fn resolve_direct_hits(mut commands: Commands, mut hits: MessageReader<DirectHit>, mut rng: ResMut<UnitRng>, mut q: Query<&mut Vehicle>) {
    for h in hits.read() {
        let Ok(mut v) = q.get_mut(h.target) else { continue };
        if !v.state.alive() {
            continue;
        }
        if rng.0.random_bool(v.kind.hit_kill()) {
            commands.trigger(Wrecked { vehicle: h.target, cause: h.cause });
        } else {
            v.immobilise(3.5, rng.0.random_range(-0.3..0.3));
            v.pitch_v -= 1.2;
            v.panic = 1.0;
        }
    }
}

/// Crews of immobilised vehicles get out after a while; dismounts from wrecks, if any live, too.
fn crew_bailout(mut commands: Commands, time: Res<Time>, mut rng: ResMut<UnitRng>, mut stats: ResMut<Stats>, models: Res<Models>, palette: Res<Palette>, mut q: Query<(&mut Vehicle, &Transform)>) {
    let dt = time.delta_secs();
    let lay = geo::layout();
    for (mut v, tf) in &mut q {
        let out = match v.state {
            VState::Immobilized => {
                v.timer += dt;
                v.timer > 12.0 + (v.idx % 5) as f32 * 6.0
            }
            VState::Destroyed => v.crew + v.troops > 0,
            _ => false,
        };
        if !out {
            continue;
        }
        let route = &lay.routes[v.route];
        let p = tf.translation.xz();
        // Attackers run back down their track; defenders back behind their own line.
        let attacker = v.side == diorama().attacker();
        let home = if attacker { route.line.point(0.0) } else { p + Vec2::new(0.0, 600.0 * lay.advance) };
        // Some dismounts go forward anyway (fewer of the 47th's: they had been told the infantry comes back).
        let p_forward = if v.side == Side::Ru { 0.4 } else { 0.15 };
        for k in 0..(v.crew + v.troops) {
            let at = p + Vec2::from_angle(k as f32 * 1.3) * 3.5;
            // Crew run back. Some dismounts go forward anyway; most go to ground or back.
            let forward = attacker && k >= v.crew && rng.0.random_bool(p_forward);
            let goal = if forward { route.objective + Vec2::new(rng.0.random_range(-100.0..100.0), 0.0) } else { home };
            let e = spawn_soldier(&mut commands, &models, &palette, v.side, at, goal, home, &mut rng.0);
            if !forward {
                commands.entity(e).insert(Fleeing);
            }
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
        v.crew = 0;
        v.troops = 0;
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
                    s.state = if s.side == Side::Ua && (fleeing.is_some() || *order == ColumnOrder::PullBack) {
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
                target = pickups.iter().copied().filter(|b| b.distance(p) < 160.0).min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))).unwrap_or(s.home);
                speed = s.speed * 1.2;
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
    fxr: ResMut<'w, FxRng>,
    sound: Sound<'w, 's>,
    blasts: MessageWriter<'w, Blast>,
}

impl FootMines<'_, '_> {
    /// A foot comes down at `p`: if it finds an anti-personnel mine, the mine goes off.
    fn step(&mut self, commands: &mut Commands, p: Vec2) {
        let mf = &mut self.mf;
        let Some(i) = mf.near(p, 0.6).into_iter().find(|&i| mf.mines[i].ap && mf.mines[i].pos.distance(p) < 0.45) else { return };
        mf.mines[i].live = false;
        let at = geo::ground(p);
        fx::explosion(commands, &mut self.fx, &mut self.fxr.0, at, 0.35, true);
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
