//! Vehicles, soldiers, mines, and what happens when steel and explosive meet.
//!
//! The Russian column behaviour follows the dossier (§5): single file down known dirt roads, a plow tank in front,
//! exhaust smoke, poor reaction to mines. Drivers who panic leave the road onto the mined shoulders, and the
//! wrecks block the road for the next column, which comes down the same road anyway.

use crate::audio::{self, Pending, Sfx};
use crate::camera::MainCamera;
use crate::fx::{self, Emitter, EmitterKind, FxAssets, FxRng};
use crate::geo::{self, Polyline};
use crate::meshes::{self, MUD, RU_GREEN, UA_GREEN};
use crate::terrain::{Paint, PaintQueue};
use crate::world::Palette;
use bevy::{light::NotShadowCaster, platform::collections::HashMap, prelude::*};
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Blast>()
            .add_message::<DirectHit>()
            .insert_resource(UnitRng(SmallRng::seed_from_u64(155)))
            .init_resource::<Stats>()
            .init_resource::<Radio>()
            .add_systems(Startup, (setup_models, lay_minefields, spawn_old_wrecks).chain().after(crate::world::spawn_belts))
            .add_systems(
                Update,
                (drive_vehicles, check_mines, resolve_blasts, resolve_direct_hits, crew_bailout, move_soldiers, animate_legs, fly_debris, drop_riders)
                    .chain()
                    .run_if(crate::running),
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VKind {
    T80,
    T72,
    Bmp1,
    Bmp2,
    Mtlb,
}

impl VKind {
    pub fn is_tank(self) -> bool {
        matches!(self, VKind::T80 | VKind::T72)
    }
    pub fn name(self) -> &'static str {
        match self {
            VKind::T80 => "T-80BV",
            VKind::T72 => "T-72B3",
            VKind::Bmp1 => "BMP-1AM",
            VKind::Bmp2 => "BMP-2",
            VKind::Mtlb => "MT-LB",
        }
    }
    fn crew(self) -> u32 {
        if self == VKind::Mtlb { 2 } else { 3 }
    }
    fn troops(self) -> u32 {
        match self {
            VKind::T80 | VKind::T72 => 0,
            VKind::Bmp1 | VKind::Bmp2 => 6,
            VKind::Mtlb => 8,
        }
    }
    fn speed(self) -> f32 {
        if self.is_tank() { 5.2 } else { 5.8 }
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
    pub legs: [Entity; 2],
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
    pub sent: u32,
    pub returned: u32,
    pub ru_dead: u32,
    pub vehicles_committed: u32,
    pub destroyed: u32,
    pub abandoned: u32,
    pub withdrawn: u32,
    pub ua_losses: u32,
    pub turret_tosses: u32,
    pub mine_hits: u32,
    pub atgm_kills: u32,
    pub artillery_kills: u32,
    /// Russians alive at or past the first belt (a foothold).
    pub foothold: u32,
    pub max_advance: f32,
    pub per_wave: Vec<(u32, u32, u32)>,
}

impl Stats {
    pub fn wave(&mut self, w: u32) -> &mut (u32, u32, u32) {
        while self.per_wave.len() <= w as usize {
            self.per_wave.push((0, 0, 0));
        }
        &mut self.per_wave[w as usize]
    }
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

#[derive(Resource)]
pub struct Models {
    pub hulls: HashMap<(u8, bool), Handle<Mesh>>,
    pub turrets: HashMap<u8, Handle<Mesh>>,
    pub turret_at: HashMap<u8, Vec3>,
    pub ru_body: Handle<Mesh>,
    pub ru_leg: Handle<Mesh>,
    pub ru_fallen: Handle<Mesh>,
    pub ua_body: Handle<Mesh>,
    pub ua_leg: Handle<Mesh>,
    pub ua_fallen: Handle<Mesh>,
    pub hip: Vec3,
    pub raam: Handle<Mesh>,
    pub missile: Handle<Mesh>,
    pub ua_tank: (Handle<Mesh>, Handle<Mesh>, Vec3),
}

fn kid(k: VKind) -> u8 {
    k as u8
}

pub fn setup_models(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut hulls = HashMap::new();
    let mut turrets = HashMap::new();
    let mut turret_at = HashMap::new();
    let ru = |k: u32| {
        // Slight per-type colour variation and mud on the lower hull.
        let base = RU_GREEN;
        let v = 1.0 + (k as f32 - 2.0) * 0.03;
        [base[0] * v, base[1] * v, base[2] * v * 0.98]
    };
    for (k, plow) in [(VKind::T80, false), (VKind::T80, true), (VKind::T72, false), (VKind::T72, true), (VKind::Bmp1, false), (VKind::Bmp2, false), (VKind::Mtlb, false)] {
        let m = match k {
            VKind::T80 | VKind::T72 => meshes::tank(ru(kid(k) as u32), plow, false),
            VKind::Bmp1 => meshes::bmp(ru(2), false),
            VKind::Bmp2 => meshes::bmp(ru(3), true),
            VKind::Mtlb => meshes::mtlb(ru(4)),
        };
        let mut hull = m.hull;
        hull.tint([MUD[0] / 0.33, MUD[1] / 0.29, MUD[2] / 0.24].map(|x: f32| 0.9 + 0.1 * x));
        hulls.insert((kid(k), plow), meshes.add(hull.build()));
        turrets.entry(kid(k)).or_insert_with(|| meshes.add(m.turret.build()));
        turret_at.insert(kid(k), m.turret_at);
    }
    let ua = meshes::tank(UA_GREEN, false, true);
    let ua_tank = (meshes.add(ua.hull.build()), meshes.add(ua.turret.build()), ua.turret_at);
    let ru_fig = meshes::figure([0.36, 0.37, 0.31], [0.33, 0.34, 0.29], [0.3, 0.32, 0.26], Some([0.8, 0.3, 0.25]));
    let ua_fig = meshes::figure([0.34, 0.36, 0.27], [0.30, 0.32, 0.24], [0.28, 0.31, 0.22], Some([0.85, 0.8, 0.3]));
    let mut raam = meshes::MeshBuilder::new();
    raam.cylinder(Vec3::ZERO, Vec3::Y * 0.12, 0.08, 6, [0.22, 0.24, 0.17]);
    let mut missile = meshes::MeshBuilder::new();
    missile.cylinder(Vec3::new(0.0, 0.0, 0.6), Vec3::new(0.0, 0.0, -1.2), 0.07, 6, [0.3, 0.32, 0.25]);
    commands.insert_resource(Models {
        hulls,
        turrets,
        turret_at,
        ru_body: meshes.add(ru_fig.body.build()),
        ru_leg: meshes.add(ru_fig.leg.build()),
        ru_fallen: meshes.add(meshes::fallen([0.36, 0.37, 0.31], [0.33, 0.34, 0.29]).build()),
        ua_body: meshes.add(ua_fig.body.build()),
        ua_leg: meshes.add(ua_fig.leg.build()),
        ua_fallen: meshes.add(meshes::fallen([0.34, 0.36, 0.27], [0.30, 0.32, 0.24]).build()),
        hip: ru_fig.hip,
        raam: meshes.add(raam.build()),
        missile: meshes.add(missile.build()),
        ua_tank,
    });
}

// ---------------------------------------------------------------------------------------------------------
// Mines

#[derive(Clone, Copy)]
pub struct Mine {
    pub pos: Vec2,
    pub live: bool,
    pub raam: bool,
    pub entity: Option<Entity>,
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
            let side = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
            let lat = side * rng.random_range(f.inner..f.outer);
            let p = road.point(s) + road.normal(s) * lat;
            mf.add(Mine { pos: p, live: true, raam: false, entity: None });
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
            mf.add(Mine { pos: p, live: true, raam: false, entity: None });
        }
    }
    info!("minefields: {} TM-62", mf.mines.len());
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
    pub kind: VKind,
    pub route: usize,
    pub s: f32,
    pub lat: f32,
    pub wave: u32,
    pub idx: u32,
    pub plow: bool,
    pub smoke: bool,
}

pub fn spawn_vehicle(commands: &mut Commands, models: &Models, palette: &Palette, v: VehicleSpec) -> Entity {
    let route = &geo::layout().routes[v.route].line;
    let p = route.point(v.s) + route.normal(v.s) * v.lat;
    let yaw = vehicle_yaw(route.tangent(v.s));
    let plow = v.plow && v.kind.is_tank();
    let turret = commands
        .spawn((
            Mesh3d(models.turrets[&kid(v.kind)].clone()),
            MeshMaterial3d(palette.vertex.clone()),
            Transform::from_translation(models.turret_at[&kid(v.kind)]),
            Turret,
        ))
        .id();
    let mut e = commands.spawn((
        Mesh3d(models.hulls[&(kid(v.kind), plow)].clone()),
        MeshMaterial3d(palette.vertex.clone()),
        Transform::from_translation(geo::ground(p)).with_rotation(ground_rotation(p, yaw)),
        Vehicle {
            kind: v.kind,
            side: Side::Ru,
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
        },
    ));
    e.add_child(turret);
    // Desant: the section rides on the roof, not inside (dossier footage, and every Vuhledar video). Mines and
    // RPGs taught both armies that the hull is the worst place to be.
    let riders = v.kind.troops().min(6);
    let roof = models.turret_at[&kid(v.kind)].y;
    for k in 0..riders {
        let side = if k % 2 == 0 { -0.55 } else { 0.55 };
        let z = 0.4 + (k / 2) as f32 * 0.85;
        e.with_child((
            Mesh3d(models.ru_body.clone()),
            MeshMaterial3d(palette.vertex.clone()),
            Transform::from_xyz(side, roof - 0.85, z).with_rotation(Quat::from_rotation_y(side.signum() * 1.2 + k as f32 * 0.3)),
            NotShadowCaster,
            Rider,
        ));
    }
    if v.smoke {
        // TDA: diesel injected into the exhaust, a white wall that drifts with the wind.
        e.insert(Emitter { kind: EmitterKind::SmokeScreen, rate: 2.5, timer: 0.0, remaining: 240.0, offset: Vec3::new(0.0, 1.2, 3.6) });
    } else {
        e.insert(Emitter { kind: EmitterKind::Exhaust, rate: 3.0, timer: 0.0, remaining: 1.0e6, offset: Vec3::new(0.6, 1.3, 3.4) });
    }
    e.id()
}

fn spawn_old_wrecks(mut commands: Commands, models: Res<Models>, palette: Res<Palette>) {
    // The fields already hold the earlier assaults' wrecks (24 Jan – 4 Feb): BMPs and tanks, burned out, with
    // their turrets beside them. Nobody has come for them.
    let kinds = [VKind::Bmp2, VKind::T80, VKind::Bmp1, VKind::Bmp2, VKind::T72, VKind::Mtlb, VKind::Bmp1, VKind::T80];
    for (i, (p, yaw)) in geo::layout().old_wrecks.iter().enumerate() {
        let k = kinds[i % kinds.len()];
        let toss = k.is_tank() && i % 2 == 1;
        let mut e = commands.spawn((
            Mesh3d(models.hulls[&(kid(k), false)].clone()),
            MeshMaterial3d(palette.charred.clone()),
            Transform::from_translation(geo::ground(*p) - Vec3::Y * 0.15).with_rotation(ground_rotation(*p, *yaw)),
        ));
        if !toss {
            e.with_child((Mesh3d(models.turrets[&kid(k)].clone()), MeshMaterial3d(palette.charred.clone()), Transform::from_translation(models.turret_at[&kid(k)]).with_rotation(Quat::from_rotation_y(i as f32))));
        }
        if i % 3 == 0 {
            e.insert(Emitter { kind: EmitterKind::Smoulder, rate: 0.5, timer: 0.0, remaining: 1.0e5, offset: Vec3::Y });
        }
        if toss {
            let tp = *p + Vec2::from_angle(i as f32 * 1.7) * 14.0;
            commands.spawn((Mesh3d(models.turrets[&kid(k)].clone()), MeshMaterial3d(palette.charred.clone()), Transform::from_translation(geo::ground(tp) + Vec3::Y * 0.3).with_rotation(Quat::from_euler(EulerRot::XYZ, 2.6, i as f32, 0.3))));
        }
    }
}

pub fn spawn_soldier(commands: &mut Commands, models: &Models, palette: &Palette, side: Side, at: Vec2, goal: Vec2, home: Vec2, rng: &mut SmallRng) -> Entity {
    let (body, leg) = match side {
        Side::Ru => (models.ru_body.clone(), models.ru_leg.clone()),
        Side::Ua => (models.ua_body.clone(), models.ua_leg.clone()),
    };
    let hip = models.hip;
    let l0 = commands.spawn((Mesh3d(leg.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(hip), Leg(0.0), NotShadowCaster)).id();
    let l1 = commands.spawn((Mesh3d(leg), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(hip * Vec3::new(-1.0, 1.0, 1.0)), Leg(std::f32::consts::PI), NotShadowCaster)).id();
    let mut e = commands.spawn((
        Mesh3d(body),
        MeshMaterial3d(palette.vertex.clone()),
        Transform::from_translation(geo::ground(at)),
        Soldier { side, state: SState::Advance, goal, home, speed: rng.random_range(1.5..2.3), fear: 0.0, timer: 0.0, phase: rng.random_range(0.0..6.0), legs: [l0, l1] },
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
    SmallArms,
}

/// An explosion that hurts whatever is near it.
#[derive(Message, Clone, Copy)]
pub struct Blast {
    pub at: Vec3,
    /// Lethal radius for people in the open; vehicles need much closer hits.
    pub radius: f32,
    pub cause: Cause,
    pub by: Side,
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

#[allow(clippy::too_many_arguments)]
fn drive_vehicles(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<UnitRng>,
    mut fxr: ResMut<FxRng>,
    mut fx: ResMut<FxAssets>,
    mut paint: ResMut<PaintQueue>,
    mut stats: ResMut<Stats>,
    mut radio: ResMut<Radio>,
    mut pending: ResMut<Pending>,
    models: Res<Models>,
    palette: Res<Palette>,
    cam: Single<&GlobalTransform, With<MainCamera>>,
    mut levels: ResMut<audio::AmbienceLevels>,
    mut q: Query<(Entity, &mut Vehicle, &mut Transform), Without<Turret>>,
    mut turrets: Query<&mut Transform, With<Turret>>,
) {
    let dt = time.delta_secs().min(0.05);
    let now = time.elapsed_secs_f64();
    let lay = geo::layout();
    let snaps: Vec<Snap> = q.iter().map(|(e, v, _)| Snap { e, route: v.route, s: v.s, lat: v.lat, state: v.state, speed: v.speed, reversing: v.reversing }).collect();
    let cam_pos = cam.translation();
    let mut nearest_engine = f32::MAX;
    let rng = &mut rng.0;

    for (e, mut v, mut tf) in &mut q {
        if v.side != Side::Ru || !v.state.mobile() {
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
                                let side = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
                                v.target_lat = b.lat + side * rng.random_range(8.0..18.0);
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
                if v.state == VState::Advancing && v.s >= deploy_s {
                    v.state = VState::Deploying;
                    // Fan out into the field to dismount: straight into the shoulder minefields.
                    let spread = [0.0f32, -22.0, 24.0, -45.0, 40.0, -64.0, 60.0, -15.0, 15.0, -34.0, 34.0, -55.0];
                    v.target_lat = if v.plow { 0.0 } else { spread[(v.idx as usize) % spread.len()] + rng.random_range(-6.0..6.0) };
                }
                // Panic: a hit nearby, and a mobilised driver yanks the tiller.
                if v.panic > 0.55 && rng.random_bool((dt * 0.12) as f64) {
                    if rng.random_bool(0.55) {
                        v.target_lat = v.lat + if rng.random_bool(0.5) { 1.0 } else { -1.0 } * rng.random_range(8.0..25.0);
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
                    let pos = Vec2::new(tf.translation.x, tf.translation.z);
                    for k in 0..v.troops {
                        let at = pos + back + Vec2::new(rng.random_range(-2.0..2.0), rng.random_range(-2.0..2.0));
                        let goal = obj + Vec2::new(rng.random_range(-120.0..120.0), rng.random_range(-20.0..20.0)) + Vec2::X * (k as f32 - 3.0) * 6.0;
                        spawn_soldier(&mut commands, &models, &palette, Side::Ru, at, goal, route.point(0.0), rng);
                    }
                    v.troops = 0;
                }
                if v.timer > 8.0 {
                    v.state = VState::Overwatch;
                    v.timer = 0.0;
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

        // Move in route coordinates.
        v.speed += (want_speed - v.speed).clamp(-4.0 * dt, 2.0 * dt);
        let lat_rate = (v.target_lat - v.lat).clamp(-2.5, 2.5);
        let ds = v.speed * dt * if v.reversing { -1.0 } else { 1.0 };
        v.s = (v.s + ds).clamp(0.0, route.length());
        v.lat += lat_rate * dt * (v.speed / 4.0).clamp(0.2, 1.0);
        let p = route.point(v.s) + route.normal(v.s) * v.lat;
        let fwd = route.tangent(v.s) * if v.reversing { -1.0 } else { 1.0 };
        let heading = (fwd * v.speed.max(0.5) + route.normal(v.s) * lat_rate * (v.speed / 4.0).clamp(0.2, 1.0) * if v.reversing { -1.0 } else { 1.0 }).normalize_or_zero();
        let yaw = if heading.length_squared() > 0.0 { vehicle_yaw(heading) } else { tf.rotation.to_euler(EulerRot::YXZ).0 };
        let old = Vec2::new(tf.translation.x, tf.translation.z);
        tf.translation = geo::ground(p) + Vec3::Y * 0.05;
        let target_rot = ground_rotation(p, yaw);
        tf.rotation = tf.rotation.slerp(target_rot, (dt * 3.0).min(1.0));
        let moved = old.distance(p);
        v.travelled += moved;
        if v.travelled > 2.0 {
            v.travelled = 0.0;
            paint.push(Paint::Track { at: p, dir: heading, gauge: 2.8 });
            if v.speed > 2.0 && rng.random_bool(0.5) {
                fx::spray(&mut commands, &mut fx, &mut fxr.0, geo::ground(p) + Vec3::new(0.0, 0.0, 0.0) - (heading * 3.0).extend(0.0).xzy(), (heading * v.speed).extend(0.0).xzy());
            }
        }
        if v.speed > 0.5 {
            nearest_engine = nearest_engine.min(cam_pos.distance(tf.translation));
        }
        let _ = (&mut stats, &mut radio, &mut pending, now);

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

pub fn wrap(a: f32) -> f32 {
    let mut a = a % std::f32::consts::TAU;
    if a > std::f32::consts::PI {
        a -= std::f32::consts::TAU;
    } else if a < -std::f32::consts::PI {
        a += std::f32::consts::TAU;
    }
    a
}

fn start_withdrawal(v: &mut Vehicle, rng: &mut SmallRng) {
    v.state = VState::Withdrawing;
    v.reversing = true;
    // Turning a tank round on a narrow road means a pivot onto the shoulder.
    v.target_lat = v.lat + if rng.random_bool(0.5) { 1.0 } else { -1.0 } * rng.random_range(3.0..12.0);
    v.timer = 0.0;
}

// ---------------------------------------------------------------------------------------------------------
// Mines

#[allow(clippy::too_many_arguments)]
fn check_mines(
    mut commands: Commands,
    time: Res<Time>,
    mut mf: ResMut<Minefield>,
    mut fx: ResMut<FxAssets>,
    mut fxr: ResMut<FxRng>,
    mut paint: ResMut<PaintQueue>,
    mut stats: ResMut<Stats>,
    mut radio: ResMut<Radio>,
    mut pending: ResMut<Pending>,
    mut blasts: MessageWriter<Blast>,
    mut rng: ResMut<UnitRng>,
    cam: Single<&GlobalTransform, With<MainCamera>>,
    mut q: Query<(Entity, &mut Vehicle, &Transform)>,
) {
    let now = time.elapsed_secs_f64();
    for (_e, mut v, tf) in &mut q {
        if !v.state.mobile() || v.speed < 0.2 {
            continue;
        }
        let p = Vec2::new(tf.translation.x, tf.translation.z);
        let fwd3 = tf.rotation * Vec3::NEG_Z;
        let fwd = Vec2::new(fwd3.x, fwd3.z).normalize_or_zero();
        let right = Vec2::new(-fwd.y, fwd.x);
        for i in mf.near(p, 6.0) {
            let m = mf.mines[i];
            let d = m.pos - p;
            let (f, x) = (d.dot(fwd), d.dot(right));
            // The plow's teeth turn mines out of the ground ahead of the tracks; now and then one goes off.
            if v.plow && (-4.7..-3.6).contains(&-f) && x.abs() < 1.9 {
                mf.mines[i].live = false;
                if let Some(ent) = m.entity {
                    commands.entity(ent).despawn();
                }
                if rng.0.random_bool(0.12) {
                    detonate(&mut commands, &mut fx, &mut fxr.0, &mut paint, &mut pending, &mut blasts, now, cam.translation(), m.pos);
                    v.plow = false;
                    radio.say(now, "Mine under the plow. The lead tank's plow is gone, it keeps going.");
                }
                continue;
            }
            if f.abs() < 3.3 && (x.abs() - 1.4).abs() < 0.5 {
                mf.mines[i].live = false;
                if let Some(ent) = m.entity {
                    commands.entity(ent).despawn();
                }
                detonate(&mut commands, &mut fx, &mut fxr.0, &mut paint, &mut pending, &mut blasts, now, cam.translation(), m.pos);
                stats.mine_hits += 1;
                // TM-62 under a track: the track and road wheels are gone. Often more.
                let kill = rng.0.random_bool(if v.kind.is_tank() { 0.3 } else { 0.5 });
                v.state = VState::Immobilized;
                v.speed = 0.0;
                v.timer = 0.0;
                if kill {
                    v.state = VState::Destroyed;
                    commands.trigger(Wrecked { vehicle: _e, cause: Cause::Mine });
                }
                let lines = [
                    "Mine! He went off the road. That one's done.",
                    "Another one on the shoulder. Tracks off.",
                    "He drove right past the last wreck, same line. Mine.",
                    "Boom. Mine. He's not moving.",
                ];
                if rng.0.random_bool(0.6) {
                    radio.say(now, lines[rng.0.random_range(0..lines.len())]);
                }
                break;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn detonate(commands: &mut Commands, fx: &mut FxAssets, rng: &mut SmallRng, paint: &mut PaintQueue, pending: &mut Pending, blasts: &mut MessageWriter<Blast>, now: f64, listener: Vec3, at: Vec2) {
    let p = geo::ground(at);
    fx::explosion(commands, fx, rng, p, 1.1, true);
    paint.push(Paint::Crater { at, radius: 2.0, seed: rng.random() });
    audio::play_at(pending, now, listener, p, Sfx::MineBlast, 1.0);
    blasts.write(Blast { at: p, radius: 6.0, cause: Cause::Mine, by: Side::Ua });
}

/// A vehicle has just been destroyed: char it, set it burning, maybe throw its turret.
#[derive(Event, Clone, Copy)]
pub struct Wrecked {
    pub vehicle: Entity,
    pub cause: Cause,
}

#[allow(clippy::too_many_arguments)]
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
    mut pending: ResMut<Pending>,
    mut paint: ResMut<PaintQueue>,
    cam: Single<&GlobalTransform, With<MainCamera>>,
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
    v.state = VState::Destroyed;
    v.speed = 0.0;
    mat.0 = palette.charred.clone();
    stats.destroyed += 1;
    stats.wave(v.wave).1 += 1;
    match ev.cause {
        Cause::Atgm => stats.atgm_kills += 1,
        Cause::Artillery | Cause::Himars | Cause::TankGun => stats.artillery_kills += 1,
        _ => {}
    }
    // Most of those inside do not get out.
    let dead = v.crew.min(rng.random_range(1..=v.crew.max(1))) + (v.troops as f32 * rng.random_range(0.4..0.9)) as u32;
    stats.ru_dead += dead;
    v.crew -= v.crew.min(dead);
    v.troops = v.troops.saturating_sub(dead);
    let p = tf.translation;
    paint.push(Paint::Scorch { at: Vec2::new(p.x, p.z), radius: 9.0 });
    // Men riding on top are thrown into the snow.
    for k in 0..dead.min(5) {
        let at = Vec2::new(p.x, p.z) + Vec2::from_angle(rng.random_range(0.0..6.28)) * rng.random_range(3.0..9.0);
        commands.spawn((Mesh3d(models.ru_fallen.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(at)).with_rotation(Quat::from_rotation_y(k as f32 * 1.9)), NotShadowCaster));
    }
    fx::explosion(&mut commands, &mut fx, rng, p + Vec3::Y, 1.4, false);
    audio::play_at(&mut pending, now, cam.translation(), p, Sfx::BlastNear, 1.3);
    commands.entity(ev.vehicle).insert(Emitter { kind: EmitterKind::WreckFire, rate: 5.0, timer: 0.0, remaining: rng.random_range(150.0..320.0), offset: Vec3::Y * 1.5 });
    if let Ok((mut tm, tgt)) = turrets.get_mut(v.turret) {
        tm.0 = palette.charred.clone();
        // T-72/T-80 carousel autoloaders: when the charges go, the turret goes up.
        if v.kind.is_tank() && rng.random_bool(0.4) {
            stats.turret_tosses += 1;
            let world = tgt.compute_transform();
            commands.entity(v.turret).remove::<ChildOf>().insert((
                Transform::from_translation(world.translation).with_rotation(world.rotation),
                Debris { vel: Vec3::new(rng.random_range(-6.0..6.0), rng.random_range(18.0..32.0), rng.random_range(-6.0..6.0)), spin: Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(-2.0..2.0), rng.random_range(-3.0..3.0)) },
            ));
            audio::play_at(&mut pending, now, cam.translation(), p, Sfx::CookOff, 1.4);
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

#[allow(clippy::too_many_arguments)]
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
        let bp = Vec2::new(b.at.x, b.at.z);
        for (e, mut v, tf) in &mut vehicles {
            if !v.state.alive() {
                continue;
            }
            let d = bp.distance(Vec2::new(tf.translation.x, tf.translation.z));
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
                            v.state = VState::Immobilized;
                            v.speed = 0.0;
                            v.timer = 0.0;
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
            let d = bp.distance(Vec2::new(tf.translation.x, tf.translation.z));
            if d > b.radius * 3.0 {
                if d < 120.0 {
                    s.fear += 0.3;
                }
                continue;
            }
            let prone = if s.state == SState::Prone || s.state == SState::DugIn { 0.4 } else { 1.0 };
            let p_kill = (1.0 - d / (b.radius * 3.0)).powf(1.5) * prone;
            if rng.random_bool(p_kill.clamp(0.0, 1.0) as f64) {
                kill_soldier(&mut commands, &models, &palette, &mut stats, e, &s, tf, rng);
            } else {
                s.fear += 1.0;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn kill_soldier(commands: &mut Commands, models: &Models, palette: &Palette, stats: &mut Stats, e: Entity, s: &Soldier, tf: &Transform, rng: &mut SmallRng) {
    commands.entity(e).despawn();
    let mesh = if s.side == Side::Ru { models.ru_fallen.clone() } else { models.ua_fallen.clone() };
    let p = Vec2::new(tf.translation.x, tf.translation.z);
    commands.spawn((Mesh3d(mesh), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(p)).with_rotation(Quat::from_rotation_y(rng.random_range(0.0..6.28))), NotShadowCaster));
    if s.side == Side::Ru {
        stats.ru_dead += 1;
    } else {
        stats.ua_losses += 1;
    }
}

#[allow(clippy::too_many_arguments)]
fn resolve_direct_hits(mut commands: Commands, mut hits: MessageReader<DirectHit>, mut rng: ResMut<UnitRng>, mut q: Query<&mut Vehicle>) {
    for h in hits.read() {
        let Ok(mut v) = q.get_mut(h.target) else { continue };
        if !v.state.alive() {
            continue;
        }
        // Kontakt-1 on the glacis and turret front saves some; side and top hits mostly don't.
        let p_kill = if v.kind.is_tank() { 0.72 } else { 0.92 };
        if rng.0.random_bool(p_kill) {
            commands.trigger(Wrecked { vehicle: h.target, cause: h.cause });
        } else {
            v.state = VState::Immobilized;
            v.speed = 0.0;
            v.timer = 0.0;
            v.panic = 1.0;
        }
    }
}

/// Crews of immobilised vehicles get out after a while; dismounts from wrecks, if any live, too.
fn crew_bailout(mut commands: Commands, time: Res<Time>, mut rng: ResMut<UnitRng>, mut stats: ResMut<Stats>, models: Res<Models>, palette: Res<Palette>, mut q: Query<(&mut Vehicle, &Transform, &mut MeshMaterial3d<StandardMaterial>)>) {
    let dt = time.delta_secs();
    let lay = geo::layout();
    for (mut v, tf, _mat) in &mut q {
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
        let p = Vec2::new(tf.translation.x, tf.translation.z);
        let home = route.line.point(0.0);
        for k in 0..(v.crew + v.troops) {
            let at = p + Vec2::from_angle(k as f32 * 1.3) * 3.5;
            // Crew run back. Some dismounts go forward anyway; most go to ground or back.
            let forward = k >= v.crew && rng.0.random_bool(0.4);
            let goal = if forward { route.objective + Vec2::new(rng.0.random_range(-100.0..100.0), 0.0) } else { home };
            let e = spawn_soldier(&mut commands, &models, &palette, Side::Ru, at, goal, home, &mut rng.0);
            if !forward {
                commands.entity(e).insert(Fleeing);
            }
        }
        if v.state == VState::Immobilized {
            v.state = VState::Abandoned;
            stats.abandoned += 1;
            stats.wave(v.wave).2 += 1;
        }
        v.crew = 0;
        v.troops = 0;
    }
}

#[derive(Component)]
pub struct Fleeing;

// ---------------------------------------------------------------------------------------------------------
// Soldiers

#[allow(clippy::too_many_arguments)]
fn move_soldiers(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<UnitRng>,
    mut stats: ResMut<Stats>,
    mut paint: ResMut<PaintQueue>,
    wrecks: Query<(&Vehicle, &Transform), Without<Soldier>>,
    mut q: Query<(Entity, &mut Soldier, &mut Transform, Option<&Fleeing>)>,
) {
    let dt = time.delta_secs().min(0.05);
    let rng = &mut rng.0;
    let wreck_pos: Vec<Vec2> = wrecks.iter().filter(|(v, _)| !v.state.mobile()).map(|(_, t)| Vec2::new(t.translation.x, t.translation.z)).collect();
    let mut foothold = 0;
    for (e, mut s, mut tf, fleeing) in &mut q {
        if s.side != Side::Ru {
            continue;
        }
        if fleeing.is_some() && s.state == SState::Advance {
            s.state = SState::Flee;
        }
        s.fear = (s.fear - dt * 0.15).max(0.0);
        s.timer += dt;
        let p = Vec2::new(tf.translation.x, tf.translation.z);
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
                    s.state = if s.fear > 2.2 && near_wreck && rng.random_bool(0.5) {
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
                target = s.home;
                speed = s.speed * 1.2;
                if p.distance(s.home) < 25.0 || !geo::in_bounds(p) {
                    stats.returned += 1;
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
            s.phase += speed * dt * 3.2;
            if rng.random_bool((dt * 0.6) as f64) {
                paint.push(Paint::Footprints { at: np });
            }
        } else {
            // Prone or dug in: lie flat.
            tf.translation = geo::ground(p) + Vec3::Y * if s.state == SState::DugIn { -0.6 } else { -0.75 };
        }
        // A foothold: Russians alive at the first tree line or past it.
        if matches!(s.state, SState::DugIn | SState::Prone | SState::Advance) && p.y < -560.0 {
            foothold += 1;
        }
        stats.max_advance = stats.max_advance.max(-p.y);
    }
    stats.foothold = foothold;
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
    let dt = time.delta_secs().min(0.05);
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
