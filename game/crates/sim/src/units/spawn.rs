//! Spawning: vehicles on their routes or at a place, the old wrecks, and soldiers.

use super::*;

pub fn vehicle_yaw(dir: Vec2) -> f32 {
    f32::atan2(-dir.x, -dir.y)
}

pub(crate) fn ground_rotation(p: Vec2, yaw: f32) -> Quat {
    let n = geo::normal(p.x, p.y);
    Quat::from_rotation_arc(Vec3::Y, n) * Quat::from_rotation_y(yaw)
}

/// Every route a vehicle drives: the layout's, at their own indices, and after them each Column's way to its
/// position as it is sent (`crate::columns`). A vehicle's `route` indexes this.
#[derive(Resource)]
pub struct RouteBook(pub Vec<geo::Route>);

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
    /// A Column's pace: its slowest vehicle's speed, which none of it outruns (`f32::INFINITY` outside one).
    pub column_pace: f32,
}

pub fn spawn_vehicle(commands: &mut Commands, models: &Models, palette: &Palette, book: &RouteBook, v: VehicleSpec) -> Entity {
    let route = &book.0[v.route].line;
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
    let seed = v.wave * 100 + v.idx;
    let mut e = commands.spawn((
        skin(hull),
        Transform::from_translation(geo::ground(p)).with_rotation(ground_rotation(p, yaw)),
        Vehicle { kind: v.kind, side: v.side, state: VState::Advancing, wave: v.wave, idx: v.idx, crew: v.kind.crew(), troops: v.kind.troops(), timer: 0.0, panic: 0.0, turret, muzzle: m.muzzle, plan: CrewPlan::Bail { at: 20.0 }, exit_timer: 0.0 },
        Driver {
            route: v.route,
            s: v.s,
            lat: v.lat,
            target_lat: v.lat,
            speed: 0.0,
            gear: Gear::Forward,
            yaw,
            yaw_rate: 0.0,
            steer: 0.0,
            accel: 0.0,
            going: 1.0,
            pace: 0.9 + 0.2 * geo::hash01(seed, 23),
            lag: 1.0 + 2.0 * geo::hash01(seed, 24),
            lead_speed: 0.0,
            odometer: 0.0,
            travelled: 0.0,
            halt: (geo::hash01(seed, 25) < 0.1).then(|| Halt::Planned { at: 100.0 + 500.0 * geo::hash01(seed, 26), secs: 5.0 + 15.0 * geo::hash01(seed, 27) }),
            hesitate: 0.0,
            detour: 0.0,
            slack: 0.04 + 0.07 * geo::hash01(seed, 29),
            gap: 28.0 + 27.0 * geo::hash01(seed, 30),
            quirk: 1000.0 * geo::hash01(seed, 31),
            column_pace: v.column_pace,
        },
        Suspension::default(),
    ));
    e.insert((Traffic::default(), Intent::default()));
    if v.kind.armed() {
        e.insert(Gun { fire_timer: 8.0 + v.idx as f32 * 2.3, aim: None, turret_yaw: 0.0 });
    }
    if plow {
        e.insert(Plow);
    }
    if v.rescue {
        e.insert(Rescue);
    }
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
    if v.kind == VKind::Ugv {
        // A robot's motors put out nothing to see.
    } else if v.smoke {
        // TDA: diesel injected into the exhaust, a white wall that drifts with the wind.
        e.insert(Emitter::new(EmitterKind::SmokeScreen, 2.5, 240.0, Vec3::new(0.0, 1.2, 3.6)));
    } else {
        e.insert(Emitter::new(EmitterKind::Exhaust, 3.0, 1.0e6, Vec3::new(0.6, 1.3, 3.4)));
    }
    e.id()
}

pub(super) fn spawn_old_wrecks(mut commands: Commands, models: Res<Models>) {
    // The fields already hold the earlier assaults' wrecks (24 Jan – 4 Feb): BMPs and tanks, burned out, with
    // their turrets beside them. Nobody has come for them.
    let kinds = &crate::data::sim_data().old_wreck_kinds;
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

/// A man to put on the field: his side, where he stands, where he is going, where he falls back to, how hard he
/// runs, and the cover he makes for, if any.
pub struct NewSoldier {
    pub side: Side,
    pub at: Vec2,
    pub goal: Vec2,
    pub home: Vec2,
    pub sprint: f32,
    pub cover: Option<Vec2>,
}

pub fn spawn_soldier(commands: &mut Commands, models: &Models, palette: &Palette, man: NewSoldier, rng: &mut SmallRng) -> Entity {
    let NewSoldier { side, at, goal, home, sprint, cover } = man;
    let (body, leg) = (models.figure(side).body.clone(), models.figure(side).leg.clone());
    let hip = models.hip;
    let l0 = commands.spawn((Mesh3d(leg.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(hip), Leg(0.0), NotShadowCaster)).id();
    let l1 = commands.spawn((Mesh3d(leg), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(hip * Vec3::new(-1.0, 1.0, 1.0)), Leg(std::f32::consts::PI), NotShadowCaster)).id();
    let mut e = commands.spawn((
        Mesh3d(body),
        MeshMaterial3d(palette.vertex.clone()),
        Transform::from_translation(geo::ground(at)),
        Soldier { side, state: SState::Advance, goal, home, speed: rng.random_range(1.5..2.3), fear: 0.0, timer: 0.0, phase: rng.random_range(0.0..6.0), sprint, leg: cover, carrying: None },
        NotShadowCaster,
    ));
    e.add_children(&[l0, l1]);
    e.id()
}
