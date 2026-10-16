//! Fire: the positions in the tree lines and their ATGM teams, artillery (the player's battery, the defenders'
//! pre-registered fires, RAAM, smoke, a HIMARS launcher), the vehicles' guns, and the fire coming back.
//!
//! The kill zone is the same on both sides of the war (dossier c08 §5, Lt. Bayak; c09 §5): a drone spots the
//! column, the fields are mined, ATGM teams in the tree lines open fire, drivers swerve off the cleared line,
//! and the guns registered on the lanes work over the stalled column and the crews climbing out.

use crate::sound::{Sfx, Sound};
use crate::diorama::{Diorama, diorama};
use crate::player::{self, MainCamera, Shake, View};
use bav_core::vocab::Mode;
use crate::dev::dev;
use crate::fx::{self, FxQueue, FxRng};
use crate::geo;
use crate::paint::{Paint, PaintQueue};
use crate::units::{self, Blast, Cause, ColumnOrder, DirectHit, Line, Mine, Minefield, Models, Radio, SState, Side, Soldier, Stats, Vehicle, VState};
use crate::player::ObservationPost;
use crate::units::Palette;
use bevy::{light::NotShadowCaster, prelude::*, window::PrimaryWindow};
use rand::{Rng, SeedableRng, rngs::SmallRng};
pub use bav_core::vocab::{Atgm, Strike};

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FireRng(SmallRng::seed_from_u64(crate::dev::seeded(72))))
            .init_resource::<FireControl>()
            .add_systems(Startup, spawn_positions.in_set(crate::schedule::Setup::Field))
            .add_systems(
                Update,
                (player_fire, player_atgm, fly_guided, run_missions, land_shells, atgm_teams, fly_missiles, small_arms, vehicle_fire, pre_registered_fires, ua_tank, mortars)
                    .chain()
                    .run_if(crate::running)
                    .in_set(crate::Step::Fire),
            )
            .add_systems(Update, (atgm_mode_key.in_set(crate::Step::Control).after(crate::dev::DevSystems), aim_gizmo.in_set(crate::Step::Screen)));
    }
}

#[derive(Resource)]
pub struct FireRng(pub SmallRng);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AtgmMode {
    /// Teams wait until the column is committed (the lead vehicle is deep in the kill zone).
    Auto,
    /// Hold fire unless a vehicle is within 400 m.
    Hold,
    /// Fire at anything in range.
    Free,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShellKind {
    He155,
    Mortar120,
    Raam,
    Himars,
    Ru152,
    Tos,
    TankHe,
    Cannon30,
    /// White phosphorus and smoke: a screen on the ground for a minute or two.
    Smoke,
}

/// A [`Strike`] as the guns fire it.
pub trait StrikeMission: Sized {
    fn mission(self, target: Vec2, sigma: f32, now: f64, player: bool) -> Mission;
    fn on_nearest_stalled(self, fc: &mut FireControl, vehicles: &Query<(&Vehicle, &Transform)>, from: Vec2, now: f64) -> Option<Vec2>;
}

impl StrikeMission for Strike {
    /// The one place a fire mission is built: rounds, spacing, flight time and dispersion for each kind.
    fn mission(self, target: Vec2, sigma: f32, now: f64, player: bool) -> Mission {
        let (kind, rounds, interval, flight, sigma, delay) = match self {
            Strike::He => (ShellKind::He155, 4, 3.5, 24.0, sigma, 2.0),
            Strike::Raam => (ShellKind::Raam, 3, 5.0, 28.0, sigma * 1.2, 3.0),
            Strike::Himars => (ShellKind::Himars, 2, 4.0, 55.0, 3.0, 2.0),
            Strike::Smoke => (ShellKind::Smoke, 4, 2.5, 24.0, sigma * 1.5 + 15.0, 2.0),
        };
        Mission { target, kind, rounds, sigma, interval, next: now + delay, flight, player, by: Side::Ua }
    }

    /// Call this strike on the stalled Russian vehicle nearest `from`, as a player would from the drone (the dev
    /// scripts and the showcase do). Returns where it was called.
    fn on_nearest_stalled(self, fc: &mut FireControl, vehicles: &Query<(&Vehicle, &Transform)>, from: Vec2, now: f64) -> Option<Vec2> {
        let target = vehicles
            .iter()
            .filter(|(v, _)| v.state.alive() && v.state != VState::Advancing)
            .map(|(_, t)| t.translation.xz())
            .min_by(|a, b| a.distance(from).total_cmp(&b.distance(from)))?;
        fc.missions.push(self.mission(target, 12.0, now, true));
        Some(target)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mission {
    pub target: Vec2,
    pub kind: ShellKind,
    pub rounds: u32,
    pub sigma: f32,
    pub interval: f32,
    pub next: f64,
    pub flight: f32,
    pub player: bool,
    /// Whose guns: the shells hurt the other side's positions too.
    pub by: Side,
}

#[derive(Resource)]
pub struct FireControl {
    pub shells: u32,
    pub raam: u32,
    pub himars: u32,
    pub smoke: u32,
    /// The defending brigade's guns, which fire on their registered points by themselves.
    pub brigade_shells: u32,
    /// The battalion's 120 mm mortars, which work over infantry in the open on their own.
    pub mortar_bombs: u32,
    pub missions: Vec<Mission>,
    pub atgm_mode: AtgmMode,
    /// The battery can work one player mission at a time.
    pub battery_busy_until: f64,
    pub last_call: Option<(Vec2, f64)>,
    pub warned_low: bool,
}

impl Default for FireControl {
    fn default() -> Self {
        // Zaluzhnyi: artillery ammunition is the binding constraint. ~6,000 RAAM shells reached Ukraine in late
        // 2022, but a battery on one morning gets a handful of missions.
        let s = crate::data::sim_data().stock;
        Self { shells: s.shells, raam: s.raam, himars: s.himars, smoke: s.smoke, brigade_shells: s.brigade_shells, mortar_bombs: s.mortar_bombs, missions: Vec::new(), atgm_mode: AtgmMode::Auto, battery_busy_until: 0.0, last_call: None, warned_low: false }
    }
}

/// A position in a tree line or a trench: riflemen, and maybe an ATGM team.
#[derive(Component)]
pub struct Post {
    pub side: Side,
    pub pos: Vec2,
    pub strength: u32,
    pub atgm: u32,
    pub reload: f32,
    pub fire_timer: f32,
    /// None: riflemen only.
    pub weapon: Option<Atgm>,
    pub figures: Vec<Entity>,
    /// Down in the trench under fire until this game time: its ATGM team does not fire.
    pub suppressed_until: f64,
}

#[derive(Component)]
pub struct Shell {
    pub at: Vec2,
    pub kind: ShellKind,
    pub land: f64,
    pub whistled: bool,
    pub by: Side,
}

#[derive(Component)]
pub struct Missile {
    pub target: Entity,
    pub launched_at: Vec3,
    pub speed: f32,
    pub trail: f32,
    /// The chance it hits; by default, worse the longer the gunner has to hold the sight on him.
    pub p_hit: Option<f32>,
    /// A Javelin: it climbs to its apogee, cruises, and dives onto the turret roof.
    pub top_attack: bool,
    /// For the hit feed: what it is, and from where.
    pub weapon: &'static str,
    pub from: &'static str,
}

#[derive(Component)]
pub struct UaTank {
    pub timer: f32,
}

fn spawn_positions(mut commands: Commands, models: Res<Models>, palette: Res<Palette>) {
    let lay = geo::layout();
    let d = crate::data::sim_data();
    let (ua, ru) = (&d.ua_weapons, &d.ru_weapons);
    let posts = lay.ua_positions.iter().zip(ua).map(|(p, w)| (Side::Ua, p, *w)).chain(lay.ru_positions.iter().zip(ru).map(|(p, w)| (Side::Ru, p, *w)));
    for (i, (side, p, weapon)) in posts.enumerate() {
        let atgm = weapon.map_or(0, Atgm::load);
        let mut figures = Vec::new();
        // A few figures in the trench line under the trees, mostly hidden, facing the enemy.
        let face = if side == Side::Ua { std::f32::consts::PI } else { 0.0 };
        for k in 0..3 {
            let at = *p + Vec2::new(k as f32 * 3.0 - 3.0, (k % 2) as f32 * 1.5);
            let e = commands
                .spawn((
                    Mesh3d(models.figure(side).body.clone()),
                    MeshMaterial3d(palette.vertex.clone()),
                    Transform::from_translation(geo::ground(at) - Vec3::Y * 0.9).with_rotation(Quat::from_rotation_y(face)),
                    NotShadowCaster,
                ))
                .id();
            figures.push(e);
        }
        commands.spawn((
            Post { side, pos: *p, strength: 6, atgm, reload: 4.0 + i as f32 * 7.0, fire_timer: i as f32, weapon, figures, suppressed_until: 0.0 },
            Transform::from_translation(geo::ground(*p)),
        ));
    }
    crate::robotyne::spawn_dug_in(&mut commands, &models, &palette);
    // The Wanderer (NYT): a T-64 in a hide at the town edge, firing on coordinates from a drone pilot.
    let Some(((hx, hz), (fx, fz))) = d.wanderer else { return };
    let hide = Vec2::new(hx, hz);
    let yaw = units::vehicle_yaw(Vec2::new(fx, fz));
    let m = &models.ua_tank;
    let mut e = commands.spawn((units::skin(&m.hull), Transform::from_translation(geo::ground(hide)).with_rotation(Quat::from_rotation_y(yaw)), UaTank { timer: 20.0 }));
    e.with_children(|c| units::dress(&mut c.spawn((units::skin(&m.turret), Transform::from_translation(m.turret_at))), &m.turret));
    units::dress(&mut e, &m.hull);
}

fn atgm_mode_key(keys: Res<ButtonInput<KeyCode>>, mut fc: ResMut<FireControl>, mut order: ResMut<ColumnOrder>, time: Res<Time>, mut radio: ResMut<Radio>) {
    if keys.just_pressed(KeyCode::Space) && diorama() == Diorama::Robotyne {
        // Robotyne: Space is the order to the column.
        *order = match *order {
            ColumnOrder::Push => ColumnOrder::Hold,
            ColumnOrder::Hold => ColumnOrder::PullBack,
            ColumnOrder::PullBack => ColumnOrder::Push,
        };
        let line = match *order {
            ColumnOrder::Push => "All callsigns, this is {player}. Push on. Keep to the lane.",
            ColumnOrder::Hold => "All callsigns, this is {player}. Hold where you are. Do not leave the lane.",
            ColumnOrder::PullBack => "All callsigns, this is {player}. Pull back. Back to the start line. Pick up the infantry on the way.",
        };
        radio.say(time.elapsed_secs_f64(), line);
        return;
    }
    if keys.just_pressed(KeyCode::Space) {
        fc.atgm_mode = match fc.atgm_mode {
            AtgmMode::Auto => AtgmMode::Hold,
            AtgmMode::Hold => AtgmMode::Free,
            AtgmMode::Free => AtgmMode::Auto,
        };
        let line = match fc.atgm_mode {
            AtgmMode::Auto => "All teams, this is {player}. Wait until the column is committed, then fire.",
            AtgmMode::Hold => "All teams, this is {player}. Hold fire. Let them come.",
            AtgmMode::Free => "All teams, this is {player}. Fire at will.",
        };
        radio.say(time.elapsed_secs_f64(), line);
    }
}

// ---------------------------------------------------------------------------------------------------------
// Player fire control

fn observed_sigma(view: &View, target: Vec2) -> f32 {
    // Adjusted fire: a drone over the target halves the dispersion; the OP on the roof helps close in.
    match view.mode {
        Mode::Drone => {
            let d = view.drone_pos.xz().distance(target);
            if view.feed_down > 0.0 { 40.0 } else { 9.0 + d * 0.006 }
        }
        Mode::Op | Mode::Ground | Mode::Fpv => match diorama() {
            Diorama::Vuhledar => 14.0 + 0.004 * (target.y + 1200.0).abs(),
            Diorama::Robotyne => 14.0 + 0.004 * target.distance(geo::layout().home),
        },
    }
}

fn player_fire(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    view: Res<View>,
    mut fc: ResMut<FireControl>,
    mut radio: ResMut<Radio>,
    mut sound: Sound,
) {
    // Alt+click asks for an evacuation instead (`evac`).
    if !buttons.just_pressed(MouseButton::Left) || view.mode == Mode::Ground || keys.pressed(KeyCode::AltLeft) || keys.pressed(KeyCode::AltRight) {
        return;
    }
    let now = time.elapsed_secs_f64();
    let Some(hit) = player::cursor_ground(&window, cam.0, cam.1) else { return };
    let target = hit.xz();
    let sigma = observed_sigma(&view, target);
    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    let (lat, lon) = geo::latlon(target);
    if ctrl {
        if fc.himars == 0 {
            radio.say(now, "{player}, this is {command}. No more HIMARS for you today.");
            return;
        }
        fc.himars -= 1;
        fc.missions.push(Strike::Himars.mission(target, sigma, now, true));
        radio.say2(now, format!("{{command}}, я {{player}}. Хаймарс, дві ракети, {lat:.4} {lon:.4}. Підліт близько хвилини."), format!("{{command}}, this is {{player}}. HIMARS, two rockets, {lat:.4} {lon:.4}. Flight time about a minute."));
    } else if shift && diorama() == Diorama::Robotyne {
        if fc.smoke == 0 {
            radio.say(now, "No more smoke.");
            return;
        }
        fc.smoke -= 1;
        fc.missions.push(Strike::Smoke.mission(target, sigma, now, true));
        radio.say2(now, format!("{{guns}}, я {{player}}. Дим, {lat:.4} {lon:.4}. Ставимо завісу. Нам теж нічого не видно."), format!("{{guns}}, this is {{player}}. Smoke, {lat:.4} {lon:.4}. Screening. It blinds us too."));
    } else if shift {
        if fc.raam == 0 {
            radio.say(now, "No more RAAM.");
            return;
        }
        fc.raam -= 1;
        fc.missions.push(Strike::Raam.mission(target, sigma, now, true));
        radio.say2(now, format!("{{guns}}, я {{player}}. РААМ, три снаряди, {lat:.4} {lon:.4}. Мінуємо прохід."), format!("{{guns}}, this is {{player}}. RAAM, three shells, {lat:.4} {lon:.4}. Mines going down on that lane."));
    } else {
        if fc.shells < 4 {
            radio.say(now, "{player}, this is {guns}. Out of shells. Nothing to send.");
            return;
        }
        if now < fc.battery_busy_until {
            radio.say(now, "{player}, this is {guns}. Still firing the last mission. Wait.");
            return;
        }
        // Repeated missions on the same spot get adjusted onto it.
        let adjust = fc.last_call.is_some_and(|(p, t)| p.distance(target) < 60.0 && now - t < 120.0);
        let sigma = if adjust { sigma * 0.6 } else { sigma };
        fc.shells -= 4;
        fc.battery_busy_until = now + 18.0;
        fc.last_call = Some((target, now));
        fc.missions.push(Strike::He.mission(target, sigma, now, true));
        radio.say2(now, format!("{{guns}}, я {{player}}. Ціль, {lat:.4} {lon:.4}, чотири снаряди. {}", if adjust { "Коригую від попередньої." } else { "Вихід." }), format!("{{guns}}, this is {{player}}. Fire mission, {lat:.4} {lon:.4}, four rounds. {}", if adjust { "Adjusting from the last." } else { "Shot, over." }));
        if fc.shells <= 32 && !fc.warned_low {
            fc.warned_low = true;
            radio.say2(now, format!("{{player}}, я {{guns}}. Лишилось {} снарядів. На сьогодні це все.", fc.shells), format!("{{player}}, this is {{guns}}. {} rounds left. That's all we get today.", fc.shells));
        }
    }
    sound.now(Sfx::Radio, 0.25);
}

fn aim_gizmo(
    mut gizmos: Gizmos,
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    view: Res<View>,
    keys: Res<ButtonInput<KeyCode>>,
    fc: Res<FireControl>,
    time: Res<Time>,
) {
    if view.mode == Mode::Ground {
        return;
    }
    if let Some(hit) = player::cursor_ground(&window, cam.0, cam.1) {
        let t = hit.xz();
        let sigma = observed_sigma(&view, t);
        let color = if keys.pressed(KeyCode::ControlLeft) {
            Color::srgba(1.0, 0.3, 0.2, 0.8)
        } else if keys.pressed(KeyCode::ShiftLeft) {
            Color::srgba(0.4, 0.9, 1.0, 0.8)
        } else {
            Color::srgba(1.0, 0.85, 0.3, 0.7)
        };
        let iso = Isometry3d::new(hit + Vec3::Y * 0.8, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
        gizmos.circle(iso, sigma * 2.0, color);
        gizmos.circle(iso, 1.5, color);
    }
    // Pending missions: where the rounds will land.
    let now = time.elapsed_secs_f64();
    for m in &fc.missions {
        if m.player {
            let blink = ((now * 4.0) as i64 % 2) as f32;
            let iso = Isometry3d::new(geo::ground(m.target) + Vec3::Y, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
            gizmos.circle(iso, m.sigma * 2.0, Color::srgba(1.0, 0.4, 0.2, 0.3 + 0.5 * blink));
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// Shells

fn run_missions(mut commands: Commands, time: Res<Time>, mut fc: ResMut<FireControl>, mut rng: ResMut<FireRng>, mut sound: Sound) {
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    let mut done = Vec::new();
    for (i, m) in fc.missions.iter_mut().enumerate() {
        while m.rounds > 0 && now >= m.next {
            m.rounds -= 1;
            m.next += m.interval as f64 * rng.random_range(0.7..1.3);
            // A rocket salvo spreads along its line of fire (north–south here) twice as far as across it.
            let off = gauss2(rng) * if m.kind == ShellKind::Tos { Vec2::new(m.sigma * 0.5, m.sigma) } else { Vec2::splat(m.sigma) };
            commands.spawn(Shell { at: m.target + off, kind: m.kind, land: now + m.flight as f64 * rng.random_range(0.95..1.05), whistled: false, by: m.by });
            // The guns are heard from where they stand, kilometres back: ours north of the field, theirs south;
            // mortars closer. Rocket launchers ripple.
            let (back, sfx, loud) = match m.kind {
                ShellKind::Mortar120 => (2500.0, Sfx::GunReport, 12.0),
                ShellKind::Himars => (9000.0, Sfx::Ripple, 60.0),
                ShellKind::Tos => (4000.0, Sfx::Ripple, 30.0),
                _ => (7000.0, Sfx::GunReport, 40.0),
            };
            if sfx == Sfx::GunReport || m.kind == ShellKind::Himars || m.rounds % 6 == 0 {
                let z = if m.by == Side::Ua { -back } else { back };
                sound.at(Vec3::new(m.target.x * 0.3, 150.0, z), sfx, loud);
            }
        }
        if m.rounds == 0 {
            done.push(i);
        }
    }
    for i in done.into_iter().rev() {
        fc.missions.remove(i);
    }
}

/// Two independent standard normal draws, as an offset.
pub fn gauss2(rng: &mut SmallRng) -> Vec2 {
    Vec2::new(gauss(rng), gauss(rng))
}

pub fn gauss(rng: &mut SmallRng) -> f32 {
    let u: f32 = rng.random_range(1e-6..1.0);
    let v: f32 = rng.random_range(0.0..1.0);
    (-2.0 * u.ln()).sqrt() * (std::f32::consts::TAU * v).cos()
}

fn land_shells(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxQueue>,
    mut fxr: ResMut<FxRng>,
    mut paint: ResMut<PaintQueue>,
    mut sound: Sound,
    mut blasts: MessageWriter<Blast>,
    mut mf: ResMut<Minefield>,
    mut shake: ResMut<Shake>,
    mut view: ResMut<View>,
    mut radio: ResMut<Radio>,
    op: Res<ObservationPost>,
    (models, palette, mut stats, mut smoke): (Res<Models>, Res<Palette>, ResMut<Stats>, ResMut<fx::Smoke>),
    mut positions: Query<&mut Post>,
    mut q: Query<(Entity, &mut Shell)>,
    (mut jammers, mut log): (Query<(&mut crate::ew::Jammer, &Transform)>, ResMut<crate::hits::HitLog>),
) {
    let now = time.elapsed_secs_f64();
    let listener = sound.listener();
    let rng = &mut fxr.0;
    for (e, mut s) in &mut q {
        let p = geo::ground(s.at);
        if !s.whistled && now > s.land - 1.4 {
            s.whistled = true;
            if listener.distance(p) < 400.0 && s.kind != ShellKind::Cannon30 {
                sound.at_time(now - listener.distance(p) as f64 / 343.0, p, Sfx::Whistle, 0.6);
            }
        }
        if now < s.land {
            continue;
        }
        commands.entity(e).despawn();
        let (size, radius, cause, sfx, crater) = match s.kind {
            ShellKind::He155 => (1.2, 15.0, Cause::Artillery, Sfx::BlastNear, 3.2),
            ShellKind::Mortar120 => (0.8, 12.0, Cause::Artillery, Sfx::BlastNear, 2.0),
            ShellKind::Ru152 => (1.1, 14.0, Cause::Artillery, Sfx::BlastNear, 3.0),
            ShellKind::Himars => (2.2, 30.0, Cause::Himars, Sfx::BlastNear, 5.5),
            ShellKind::Tos => (1.6, 30.0, Cause::Thermobaric, Sfx::Thermobaric, 4.5),
            ShellKind::TankHe => (0.8, 9.0, Cause::TankGun, Sfx::BlastNear, 2.0),
            ShellKind::Cannon30 => (0.18, 2.0, Cause::TankGun, Sfx::BlastNear, 0.0),
            ShellKind::Raam => (0.5, 0.0, Cause::Artillery, Sfx::BlastFar, 0.0),
            ShellKind::Smoke => (0.3, 0.0, Cause::Artillery, Sfx::BlastFar, 0.0),
        };
        match s.kind {
            ShellKind::Raam => {
                // The base ejects nine mines over the lane; they lie on the snow, visible from the drone.
                fx.spawn(fx::Spawn { kind: fx::Kind::GreySmoke, pos: p + Vec3::Y * 60.0, vel: Vec3::Y * -2.0, life: 4.0, size0: 3.0, size1: 8.0, alpha: 0.5, ..default() });
                sound.at(p + Vec3::Y * 60.0, sfx, 0.4);
                for _ in 0..9 {
                    let mp = s.at + gauss2(rng) * 22.0;
                    let ent = commands
                        .spawn((Mesh3d(models.raam.clone()), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(mp)), NotShadowCaster))
                        .id();
                    mf.add(Mine { entity: Some(ent), ..Mine::at(mp) });
                }
                continue;
            }
            ShellKind::Smoke => {
                // White phosphorus, bursting overhead.
                sound.at(p + Vec3::Y * 80.0, sfx, 0.4);
                fx.phosphorus(p);
                smoke.add(s.at, 45.0, now + 100.0);
                continue;
            }
            ShellKind::Tos => fx.thermobaric(p),
            ShellKind::Cannon30 => {
                fx.spawn(fx::Spawn { kind: fx::Kind::Dirt, pos: p, vel: Vec3::Y * 6.0, life: 0.8, size0: 0.5, size1: 2.0, alpha: 0.8, gravity: 9.0, ..default() });
            }
            _ => fx.explosion(p, size, true),
        }
        if crater > 0.0 {
            paint.push(Paint::Crater { at: s.at, radius: crater * rng.random_range(0.8..1.2), seed: rng.random() });
        }
        if s.kind != ShellKind::Cannon30 {
            sound.at(p, sfx, size);
        }
        let dist = listener.distance(p);
        if dist < 250.0 {
            shake.trauma = (shake.trauma + size * (1.0 - dist / 250.0)).min(1.0);
            if dist < 60.0 {
                shake.flash = (shake.flash + 0.5).min(1.0);
            }
        }
        blasts.write(Blast { at: p, radius, cause });
        // Our shells on their jammers: one within 25 m is knocked out.
        if s.by == Side::Ua
            && let Some(at) = crate::ew::knock_out(&mut jammers, s.at)
        {
            log.note(now, "155 mm", "our guns", "EW jammer", Side::Ru, "knocked out: the link is cleaner there");
            radio.say2(now, "Їхній РЕБ на гребені мовчить. Накрили.", "Their jammer's gone quiet. We got it.");
            info!("ew: jammer at {:.0},{:.0} knocked out, at {now:.0} s", at.x, at.y);
        }
        // Fire on the tree lines and trenches hurts the men in them, and puts the rest down in the trench for a
        // while, longer the closer it lands: a team under fire does not stand up to guide a missile.
        for mut pos in &mut positions {
            if pos.side == s.by {
                continue;
            }
            let d = pos.pos.distance(s.at);
            if d < radius * 3.0 {
                pos.suppressed_until = pos.suppressed_until.max(now + (20.0 + 40.0 * (1.0 - d / (radius * 3.0))) as f64);
            }
            if pos.strength > 0 && d < radius * 1.6 && rng.random_bool((0.5 * (1.0 - d / (radius * 1.6))) as f64) {
                pos.strength -= 1;
                if pos.side == diorama().defender() {
                    stats.defender_losses += 1;
                } else {
                    stats.overwatch_losses += 1;
                }
                if let Some(f) = pos.figures.pop() {
                    commands.entity(f).despawn();
                }
                if pos.side == Side::Ua {
                    if pos.strength == 0 {
                        radio.say2(now, format!("Втратили позицію на {:.0}, {:.0}. Ніхто не відповідає.", pos.pos.x, pos.pos.y), format!("We've lost the position at {:.0}, {:.0}. Nobody answering.", pos.pos.x, pos.pos.y));
                    } else if rng.random_bool(0.4) {
                        radio.say(now, "Three hundred. Need evacuation from the tree line.");
                    }
                } else if pos.strength == 0 {
                    radio.say(now, "This is {op}. Their position on the crest has gone quiet.");
                }
            }
        }
        if s.by == Side::Ru
            && view.op_alive
            && op.building.distance(s.at) < if s.kind == ShellKind::Tos { 40.0 } else { 15.0 }
            && rng.random_bool(0.04)
        {
            view.op_alive = false;
            radio.say(now, match diorama() {
                Diorama::Vuhledar => "The OP on the roof is gone. TOS took the building. No eyes from the high-rise.",
                Diorama::Robotyne => "The water tower is down. No eyes from the farm.",
            });
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// ATGM teams

fn atgm_teams(
    mut commands: Commands,
    time: Res<Time>,
    fc: Res<FireControl>,
    mut fx: ResMut<FxQueue>,
    mut fxr: ResMut<FxRng>,
    mut sound: Sound,
    mut radio: ResMut<Radio>,
    models: Res<Models>,
    palette: Res<Palette>,
    vehicles: Query<(Entity, &Vehicle, &Transform)>,
    missiles: Query<&Missile>,
    mut teams: Query<(Entity, &mut Post)>,
    view: Res<View>,
    smoke: Res<fx::Smoke>,
    mut stats: ResMut<Stats>,
    mut seen: Local<bevy::platform::collections::HashMap<(Entity, Entity), (f64, bool)>>,
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs_f64();
    let lay = geo::layout();
    let attacker = diorama().attacker();
    // Whether a team can see a vehicle holds for half a second at a time.
    let mut visible = |team: Entity, e: Entity, from: Vec2, to: Vec2| -> bool {
        match seen.get(&(team, e)) {
            Some(&(at, v)) if now - at < 0.5 => v,
            _ => {
                let v = geo::line_of_sight(from, to);
                seen.insert((team, e), (now, v));
                v
            }
        }
    };
    // The picture a team fires on, looked at only once some team is ready to fire: which columns are committed
    // (the lead vehicle past the deploy point, or stopped in the kill zone), whether our own column is out in the
    // field (attacking, the overwatch covers it once it is, not before), and which vehicles already have a missile
    // on the way.
    let mut picture: Option<(Vec<usize>, bool, Vec<Entity>)> = None;
    for (team, mut pos) in &mut teams {
        let Some(weapon) = pos.weapon else { continue };
        // Reloading goes on whoever is at the launcher, the AI or the player.
        pos.reload = (pos.reload - dt).max(0.0);
        if pos.atgm == 0 || pos.strength < 2 || now < pos.suppressed_until || (view.mode == Mode::Ground && view.ground_team == Some(team)) {
            continue;
        }
        if pos.reload > 0.0 {
            continue;
        }
        let (committed, column_out, targeted) = &*picture.get_or_insert_with(|| {
            let committed = (0..lay.routes.len())
                .filter(|&r| vehicles.iter().any(|(_, v, _)| v.side == attacker && v.route == r && v.state.alive() && v.s > lay.routes[r].deploy_s - 350.0))
                .collect();
            let column_out = vehicles.iter().any(|(_, v, t)| v.side == attacker && v.state.alive() && t.translation.z * lay.advance > -450.0);
            (committed, column_out, missiles.iter().map(|m| m.target).collect())
        });
        let range = weapon.range();
        let mut best: Option<(Entity, f32, Vec3)> = None;
        for (e, v, tf) in &vehicles {
            if !v.state.alive() || v.side == pos.side || targeted.contains(&e) {
                continue;
            }
            let vp = tf.translation.xz();
            let d = vp.distance(pos.pos);
            let allowed = if pos.side == Side::Ua {
                match fc.atgm_mode {
                    AtgmMode::Free => d < range,
                    AtgmMode::Hold => d < 400.0,
                    // Defending, wait for the column to commit; attacking, cover it whenever there is something to hit.
                    AtgmMode::Auto => d < range.min(2500.0) && (pos.side == attacker && *column_out || pos.side != attacker && committed.contains(&v.route) || d < 700.0),
                }
            } else {
                // The Russian teams on the crest let the column come into the minefield, then fire on anything
                // in it, stopped ones first (the scoring below).
                d < 2200.0 && (vp.y - lay.foothold_z) * lay.advance > -1050.0
            };
            // They cannot shoot what the smoke hides, nor what a belt or a crest hides.
            if !allowed || smoke.blocks(pos.pos, vp, now) {
                continue;
            }
            if !visible(team, e, pos.pos, vp) {
                stats.hidden += 1;
                continue;
            }
            // Prefer tanks, then moving vehicles, then the nearest.
            let score = d - if v.kind.is_tank() { 600.0 } else { 0.0 } - if v.state.mobile() { 400.0 } else { -2000.0 };
            if best.is_none_or(|b| score < b.1) {
                best = Some((e, score, tf.translation));
            }
        }
        let Some((target, _, tp)) = best else { continue };
        // Each team takes its own time to acquire and track: a few seconds, rarely all at once.
        if !fxr.0.random_bool((dt / 6.0).min(1.0) as f64) {
            continue;
        }
        pos.atgm -= 1;
        pos.reload = weapon.reload();
        let from = geo::ground(pos.pos) + Vec3::Y * 1.2;
        let dir = (tp - from).normalize();
        fx.muzzle(from, dir, false);
        fx.backblast(from, dir);
        sound.at(from, Sfx::AtgmLaunch, 0.9);
        info!("atgm launch: {weapon} from the post at {:.0},{:.0}, {} men, at {now:.0} s", pos.pos.x, pos.pos.y, pos.strength);
        let speed = weapon.speed();
        commands.spawn((
            Mesh3d(models.missile.clone()),
            MeshMaterial3d(palette.vertex.clone()),
            Transform::from_translation(from).looking_to(dir, Vec3::Y),
            Missile {
                target,
                launched_at: from,
                speed,
                trail: 0.0,
                p_hit: None,
                top_attack: weapon == Atgm::Javelin,
                weapon: weapon.name(),
                from: match (pos.side, diorama()) {
                    (Side::Ru, _) => "the crest",
                    (Side::Ua, Diorama::Vuhledar) => "the first tree line",
                    (Side::Ua, Diorama::Robotyne) => "the forward tree line",
                },
            },
            NotShadowCaster,
        ));
        if pos.side == Side::Ru {
            if fxr.0.random_bool(0.5) {
                radio.say(now, "ATGM from the crest! Launch, launch!");
            }
        } else if fxr.0.random_bool(0.5) {
            radio.say2(now, format!("{weapon}: пуск."), format!("{weapon}: launch."));
        }
    }
}

fn fly_missiles(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxQueue>,
    mut fxr: ResMut<FxRng>,
    mut hits: MessageWriter<DirectHit>,
    mut paint: ResMut<PaintQueue>,
    mut log: ResMut<crate::hits::HitLog>,
    targets: Query<(&Transform, &Vehicle), Without<Missile>>,
    mut q: Query<(Entity, &mut Missile, &mut Transform)>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    for (e, mut m, mut tf) in &mut q {
        let Ok((t, tv)) = targets.get(m.target) else {
            commands.entity(e).despawn();
            continue;
        };
        let aim = t.translation + Vec3::Y * if m.top_attack { 2.2 } else { 1.3 };
        let to = aim - tf.translation;
        let d = to.length();
        let dir = if m.top_attack {
            // The Javelin's top attack (§2.5 of design/next-items.md): up to an apogee of 60–150 m by range,
            // along, then down at 50–70 degrees onto the roof; 10–15 s of flight at 1.5–2 km.
            let total = m.launched_at.xz().distance(aim.xz()).max(50.0);
            let f = (1.0 - to.xz().length() / total).clamp(0.0, 1.0);
            if f > 0.82 {
                to.normalize()
            } else {
                let apogee = (total / 2000.0 * 150.0).clamp(60.0, 150.0);
                let want_y = m.launched_at.y + apogee * (std::f32::consts::PI * f.min(0.9)).sin().powf(0.6);
                let flat = Vec3::new(to.x, 0.0, to.z).normalize_or_zero();
                (flat + Vec3::Y * ((want_y - tf.translation.y) * 0.08).clamp(-1.2, 1.7)).normalize()
            }
        } else {
            // A beam rider flies a little high, then down onto the target.
            let lift = Vec3::Y * (d / 300.0).min(1.0) * 4.0;
            (to + lift).normalize()
        };
        tf.translation += dir * m.speed * dt;
        tf.look_to(dir, Vec3::Y);
        m.trail += dt;
        // The motor burns for the first few hundred metres and leaves a thin grey-white line; after that the
        // missile is a dot (a beam rider has no wire and no trail).
        if m.trail > 0.03 && m.launched_at.distance(tf.translation) < m.speed * 2.5 {
            m.trail = 0.0;
            fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: tf.translation, vel: Vec3::Y * 0.3, life: 4.0, size0: 0.3, size1: 1.8, alpha: 0.5, drag: 0.5, ..default() });
            fx.spawn(fx::Spawn { kind: fx::Kind::Flash, pos: tf.translation, vel: Vec3::ZERO, life: 0.05, size0: 0.8, size1: 0.6, alpha: 1.0, ..default() });
        }
        // Within this frame's flight of the target counts as arrival: no flying through it at low frame rates.
        if d < 3.0 + m.speed * dt {
            commands.entity(e).despawn();
            let rng = &mut fxr.0;
            // Long wire-guided shots miss more often: the gunner holds the crosshair for 15-25 s under fire.
            let range = m.launched_at.distance(aim);
            if rng.random_bool(m.p_hit.unwrap_or((0.88 - range / 9000.0).clamp(0.4, 0.88)) as f64) {
                hits.write(DirectHit { target: m.target, cause: Cause::Atgm, top: m.top_attack, weapon: m.weapon, from: m.from });
                fx.explosion(aim, 0.7, false);
            } else {
                // A miss: it ploughs into the snow beyond.
                log.open(now, m.weapon, m.from, m.target, tv, "missed: into the ground beyond");
                let miss = aim.xz() + gauss2(rng) * 12.0;
                fx.explosion(geo::ground(miss), 0.6, true);
                paint.push(Paint::Crater { at: miss, radius: 1.2, seed: rng.random() });
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// Small arms and the Russian return fire

/// Rifles and machine guns from the positions at the other side's men in the open.
fn small_arms(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxQueue>,
    mut fxr: ResMut<FxRng>,
    mut stats: ResMut<Stats>,
    mut sound: Sound,
    models: Res<Models>,
    palette: Res<Palette>,
    mut positions: Query<&mut Post>,
    mut soldiers: Query<(Entity, &mut Soldier, &Transform)>,
) {
    let dt = time.delta_secs();
    let rng = &mut fxr.0;
    let mut dead: Vec<Entity> = Vec::new();
    for mut pos in &mut positions {
        if pos.strength == 0 {
            continue;
        }
        pos.fire_timer -= dt;
        if pos.fire_timer > 0.0 {
            continue;
        }
        pos.fire_timer = rng.random_range(1.5..3.5);
        let from = geo::ground(pos.pos) + Vec3::Y * 0.8;
        // The nearest man within rifle range that a belt or a crest does not hide.
        let mut near: Vec<(Entity, f32, Vec3)> = soldiers
            .iter()
            .filter(|(e, s, tf)| s.side != pos.side && !dead.contains(e) && tf.translation.xz().distance(pos.pos) < 750.0)
            .map(|(e, _, tf)| (e, tf.translation.xz().distance(pos.pos), tf.translation))
            .collect();
        near.sort_by(|a, b| a.1.total_cmp(&b.1));
        let Some(&(e, d, tp)) = near.iter().find(|(_, _, tp)| geo::line_of_sight(pos.pos, tp.xz())) else { continue };
        for k in 0..5 {
            let spread = Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(0.0..1.5), rng.random_range(-3.0..3.0)) * (d / 300.0);
            if k % 2 == 0 {
                fx.tracer(from, tp + spread, 700.0, pos.side);
            }
        }
        sound.at(from, Sfx::MachineGun, 0.5);
        let Ok((_, mut s, tf)) = soldiers.get_mut(e) else { continue };
        let prone = if matches!(s.state, SState::Prone | SState::DugIn) { 0.35 } else { 1.0 };
        let p_kill = (0.42 * (1.0 - d / 800.0) * prone * (pos.strength as f32 / 6.0).sqrt()).clamp(0.02, 0.6);
        if rng.random_bool(p_kill as f64) {
            stats.died("rifles", (s.side == diorama().attacker()) as u32);
            units::kill_soldier(&mut commands, &models, &palette, &mut stats, e, &s, tf, None, rng);
            dead.push(e);
        } else {
            s.fear += 0.7;
        }
    }
}

/// The attacking vehicles fire on the defenders' tree line as they come (a known position if one is near, else
/// into the trees ahead), and the defenders' dug-in tanks fire on the attacking vehicles.
fn vehicle_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut fx: ResMut<FxQueue>,
    mut fxr: ResMut<FxRng>,
    mut sound: Sound,
    mut hits: MessageWriter<DirectHit>,
    smoke: Res<fx::Smoke>,
    positions: Query<&Post>,
    turrets: Query<&GlobalTransform, With<units::Turret>>,
    mut vehicles: Query<(Entity, &mut Vehicle, &Transform)>,
    soldiers: Query<(&Soldier, &Transform)>,
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs_f64();
    let rng = &mut fxr.0;
    let attacker = diorama().attacker();
    let line_z = geo::layout().defence_z;
    let live: Vec<Vec2> = positions.iter().filter(|p| p.strength > 0 && p.side != attacker).map(|p| p.pos).collect();
    let tree_line = |rng: &mut SmallRng, from: Vec2| -> Vec2 {
        // Fire at a known position if one is near, else into the nearest tree line ahead.
        if !live.is_empty() && rng.random_bool(0.5) {
            let p = live[rng.random_range(0..live.len())];
            if p.distance(from) < 2500.0 {
                return p + gauss2(rng) * 30.0;
            }
        }
        Vec2::new(from.x + rng.random_range(-400.0..400.0), line_z + rng.random_range(-15.0..15.0))
    };
    // What the defenders' tanks can see: attacking vehicles out of the smoke, within three kilometres.
    let exposed: Vec<(Entity, Vec2)> = vehicles.iter().filter(|(_, v, _)| v.side == attacker && v.state.alive()).map(|(e, _, t)| (e, t.translation.xz())).collect();
    for (_, mut v, tf) in &mut vehicles {
        if v.side != attacker {
            if v.state.alive() && v.crew > 0 && v.kind.is_tank() {
                defender_tank(&mut fx, rng, &mut sound, &mut hits, &smoke, &turrets, &mut v, tf, &exposed, dt, now);
            }
            continue;
        }
        if !matches!(v.state, VState::Advancing | VState::Deploying | VState::Overwatch | VState::Halted | VState::Immobilized) || v.crew == 0 || v.kind.breacher() {
            continue;
        }
        v.fire_timer -= dt;
        if v.aim.is_none() {
            let from = tf.translation.xz();
            v.aim = Some(tree_line(rng, from));
        }
        if v.fire_timer > 0.0 {
            continue;
        }
        let from2 = tf.translation.xz();
        let aim = v.aim.unwrap();
        let muzzle = turrets.get(v.turret).map(|t| t.transform_point(v.muzzle)).unwrap_or(tf.translation + Vec3::Y * 2.0);
        let dir = (geo::ground(aim) - muzzle).normalize();
        if v.kind.is_tank() {
            v.fire_timer = rng.random_range(14.0..26.0);
            fx.muzzle(muzzle, dir, true);
            sound.at(muzzle, Sfx::TankGun, 1.0);
            let d = from2.distance(aim);
            commands.spawn(Shell { at: aim + gauss2(rng) * (8.0 + d * 0.01), kind: ShellKind::TankHe, land: now + d as f64 / 900.0, whistled: true, by: v.side });
        } else {
            v.fire_timer = rng.random_range(6.0..14.0);
            for k in 0..6 {
                let hit = aim + gauss2(rng) * 12.0;
                if k % 2 == 0 {
                    fx.tracer(muzzle, geo::ground(hit) + Vec3::Y, 900.0, v.side);
                }
                commands.spawn(Shell { at: hit, kind: ShellKind::Cannon30, land: now + 1.5 + k as f64 * 0.18, whistled: true, by: v.side });
            }
            fx.muzzle(muzzle, dir, false);
            sound.at(muzzle, Sfx::Cannon30, 0.8);
        }
        if rng.random_bool(0.3) {
            v.aim = None;
        }
    }
    // The attackers' infantry fire back from the ground: tracers into the tree line, mostly harmless.
    let mut n = 0;
    for (s, tf) in &soldiers {
        if s.side != attacker || !matches!(s.state, SState::Prone | SState::DugIn) || n > 6 {
            continue;
        }
        if rng.random_bool((dt * 0.25) as f64) {
            n += 1;
            let from = tf.translation + Vec3::Y * 0.4;
            let to = geo::ground(tree_line(rng, from.xz())) + Vec3::Y * 2.0;
            fx.tracer(from, to, 700.0, s.side);
            sound.at(from, Sfx::MachineGun, 0.3);
        }
    }
}

/// A dug-in tank on the crest: picks an attacking vehicle it can see, prefers one that has stopped, and fires
/// at it with the main gun. A hit at two kilometres is a matter of luck and of how still the target is.
#[allow(clippy::too_many_arguments)]
fn defender_tank(
    fx: &mut FxQueue,
    rng: &mut SmallRng,
    sound: &mut Sound,
    hits: &mut MessageWriter<DirectHit>,
    smoke: &fx::Smoke,
    turrets: &Query<&GlobalTransform, With<units::Turret>>,
    v: &mut Vehicle,
    tf: &Transform,
    exposed: &[(Entity, Vec2)],
    dt: f32,
    now: f64,
) {
    v.fire_timer -= dt;
    let from = tf.translation.xz();
    let candidates = || exposed.iter().filter(|(_, p)| p.distance(from) < 2000.0 && p.y > -400.0 && !smoke.blocks(from, *p, now));
    let Some(&(_, tp)) = candidates().min_by(|a, b| a.1.distance(from).total_cmp(&b.1.distance(from))) else { return };
    v.aim = Some(tp);
    if v.fire_timer > 0.0 {
        return;
    }
    // The shot goes to the nearest one the crest does not hide; if none, look again in a few seconds.
    let Some(&(target, tp)) = candidates().filter(|(_, p)| geo::line_of_sight(from, *p)).min_by(|a, b| a.1.distance(from).total_cmp(&b.1.distance(from))) else {
        v.fire_timer = 3.0;
        return;
    };
    v.fire_timer = rng.random_range(16.0..28.0);
    let muzzle = turrets.get(v.turret).map(|t| t.transform_point(v.muzzle)).unwrap_or(tf.translation + Vec3::Y * 2.0);
    let aim = geo::ground(tp) + Vec3::Y * 1.2;
    fx.muzzle(muzzle, (aim - muzzle).normalize(), true);
    sound.at(muzzle, Sfx::TankGun, 1.0);
    let d = from.distance(tp);
    if rng.random_bool((0.75 - d as f64 / 4000.0).clamp(0.15, 0.7)) {
        hits.write(DirectHit { target, cause: Cause::TankGun, top: false, weapon: "125 mm", from: "a dug-in T-72" });
        fx.explosion(aim, 0.6, false);
        sound.at(aim, Sfx::BlastNear, 0.9);
    } else {
        let miss = tp + gauss2(rng) * 14.0;
        fx.explosion(geo::ground(miss), 0.5, true);
    }
}

/// The brigade's pre-registered fires: when a column stalls on a known point, the guns fire on their own.
/// At Vuhledar the 72nd's guns; south of Mala Tokmachka the Russian guns, cued by their drones, which do not see
/// through smoke.
fn pre_registered_fires(
    time: Res<Time>,
    mut fc: ResMut<FireControl>,
    mut radio: ResMut<Radio>,
    smoke: Res<fx::Smoke>,
    mut last: Local<Vec<f64>>,
    // Where the attackers' stalled vehicles and their men in the open are, gathered once a frame when a point is ready.
    mut seen: Local<(Vec<Vec2>, Vec<Vec2>)>,
    vehicles: Query<(&Vehicle, &Transform)>,
    soldiers: Query<(&Soldier, &Transform)>,
) {
    let now = time.elapsed_secs_f64();
    let trps = &geo::layout().trps;
    let (attacker, defender) = (diorama().attacker(), diorama().defender());
    last.resize(trps.len(), -1000.0);
    let (stalled_at, men_at) = &mut *seen;
    let mut gathered = false;
    for (i, trp) in trps.iter().enumerate() {
        let (every, rounds) = crate::data::sim_data().registered_fire;
        if now - last[i] < every || fc.brigade_shells < rounds {
            continue;
        }
        if !gathered {
            gathered = true;
            // Stalled: knocked out, dismounting or firing from the spot, or halted behind a blockage for a while.
            // The guns' spotters do not see through smoke.
            stalled_at.clear();
            stalled_at.extend(
                vehicles
                    .iter()
                    .filter(|(v, t)| {
                        v.side == attacker
                            && !smoke.covers(t.translation.xz(), now)
                            && v.state.alive()
                            && (matches!(v.state, VState::Immobilized | VState::Dismounting | VState::Overwatch) || (v.state == VState::Halted && v.timer > 6.0))
                    })
                    .map(|(_, t)| t.translation.xz()),
            );
            men_at.clear();
            men_at.extend(soldiers.iter().filter(|(s, t)| s.side == attacker && !smoke.covers(t.translation.xz(), now)).map(|(_, t)| t.translation.xz()));
        }
        let stalled = stalled_at.iter().filter(|p| p.distance(*trp) < 140.0).count();
        let men = men_at.iter().filter(|p| p.distance(*trp) < 140.0).count();
        if stalled >= 1 || men >= 6 {
            last[i] = now;
            fc.brigade_shells -= rounds;
            let kind = if defender == Side::Ua { ShellKind::He155 } else { ShellKind::Ru152 };
            fc.missions.push(Mission { target: *trp, kind, rounds, sigma: 35.0, interval: 2.5, next: now + 1.0, flight: 22.0, player: false, by: defender });
            radio.lines.push((now, match diorama() {
                Diorama::Vuhledar => Line::both(format!("Я {{guns}}. Вогонь по пристріляній точці {}.", i + 1), format!("This is {{guns}}. Firing on registered point {}.", i + 1)),
                Diorama::Robotyne => Line::new("Incoming on the lane! They have it registered. Get out of there!"),
            }));
        }
    }
}

/// The Wanderer: the T-64 crew in the hide fires on stalled columns at long range, on a drone pilot's word.
fn ua_tank(mut commands: Commands, time: Res<Time>, mut fx: ResMut<FxQueue>, mut fxr: ResMut<FxRng>, mut sound: Sound, vehicles: Query<(&Vehicle, &Transform)>, mut q: Query<(&mut UaTank, &Transform)>) {
    let now = time.elapsed_secs_f64();
    let rng = &mut fxr.0;
    for (mut t, tf) in &mut q {
        t.timer -= time.delta_secs();
        if t.timer > 0.0 {
            continue;
        }
        t.timer = rng.random_range(18.0..30.0);
        let from = tf.translation.xz();
        let targets: Vec<Vec2> = vehicles.iter().filter(|(v, _)| v.state.alive() && !matches!(v.state, VState::Advancing)).map(|(_, t)| t.translation.xz()).filter(|p| p.distance(from) < 4500.0).collect();
        if targets.is_empty() {
            continue;
        }
        let aim = targets[rng.random_range(0..targets.len())];
        let muzzle = tf.translation + Vec3::Y * 1.9 + (tf.rotation * Vec3::NEG_Z) * 5.0;
        fx.muzzle(muzzle, (geo::ground(aim) - muzzle).normalize(), true);
        sound.at(muzzle, Sfx::TankGun, 1.2);
        let d = from.distance(aim);
        commands.spawn(Shell { at: aim + gauss2(rng) * (6.0 + d * 0.006), kind: ShellKind::TankHe, land: now + d as f64 / 850.0, whistled: true, by: Side::Ua });
    }
}

/// The defenders' battalion mortars: when the attackers' infantry bunch up within range in the open, a few
/// bombs on them.
fn mortars(time: Res<Time>, mut fc: ResMut<FireControl>, mut rng: ResMut<FireRng>, mut next: Local<f64>, smoke: Res<fx::Smoke>, soldiers: Query<(&Soldier, &Transform)>) {
    let now = time.elapsed_secs_f64();
    if now < *next || fc.mortar_bombs < 3 {
        return;
    }
    let rng = &mut rng.0;
    let (z0, z1) = geo::layout().mortar_band;
    let men: Vec<Vec2> = soldiers
        .iter()
        .filter(|(s, t)| s.side == diorama().attacker() && s.state != SState::Flee && t.translation.z < z1 && t.translation.z > z0 && !smoke.covers(t.translation.xz(), now))
        .map(|(_, t)| t.translation.xz())
        .collect();
    if men.len() < 3 {
        *next = now + 5.0;
        return;
    }
    // Aim at the densest group: the man with the most others within 40 m.
    let target = *men.iter().max_by_key(|p| men.iter().filter(|q| q.distance(**p) < 40.0).count()).unwrap();
    let rounds = rng.random_range(2..5);
    fc.mortar_bombs -= rounds;
    fc.missions.push(Mission { target, kind: ShellKind::Mortar120, rounds, sigma: 22.0, interval: 2.5, next: now, flight: 16.0, player: false, by: diorama().defender() });
    *next = now + rng.random_range(14.0..26.0);
}

// ---------------------------------------------------------------------------------------------------------
// The player's own ATGM shot, from the ground view

/// A missile flying on the player's wire: it follows the sight's line of sight, so the gunner must hold the
/// crosshair on the tank for the whole 10-20 s of flight (Stugna-P: semi-automatic command to line of sight).
#[derive(Component)]
pub struct GuidedMissile {
    pub travelled: f32,
    pub speed: f32,
    pub trail: f32,
    /// The launching team and the sight's position: the wire runs back to that trench, whatever the player does.
    pub team: Entity,
    pub eye: Vec3,
    /// The line the sight last held. Updated only while the player is at that sight.
    pub dir: Vec3,
}

/// Javelin seekers lock after this long on the target.
pub const JAVELIN_LOCK: f32 = 2.5;

fn player_atgm(
    mut commands: Commands,
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut view: ResMut<View>,
    mut fx: ResMut<FxQueue>,
    mut fxr: ResMut<FxRng>,
    mut sound: Sound,
    mut radio: ResMut<Radio>,
    mut shake: ResMut<Shake>,
    (models, palette): (Res<Models>, Res<Palette>),
    cam: Single<&GlobalTransform, With<MainCamera>>,
    guided: Query<(), With<GuidedMissile>>,
    mut teams: Query<&mut Post>,
) {
    let now = time.elapsed_secs_f64();
    let scripted = view.launch || dev().auto_atgm.is_some_and(|t| now >= t && now - time.delta_secs_f64() < t);
    let fire = buttons.just_pressed(MouseButton::Left) || scripted;
    if view.mode != Mode::Ground || !fire || !view.sight || !guided.is_empty() {
        return;
    }
    let Some(team) = view.ground_team else { return };
    let Ok(mut pos) = teams.get_mut(team) else { return };
    let Some(weapon) = pos.weapon else { return };
    if pos.strength < 2 {
        radio.say(now, "Nobody left here to work the launcher.");
        return;
    }
    if pos.atgm == 0 {
        radio.say(now, "Out of missiles at this position.");
        return;
    }
    if pos.reload > 0.0 {
        radio.say2(now, format!("Перезарядка. {:.0} секунд.", pos.reload.ceil()), format!("Reloading. {:.0} seconds.", pos.reload.ceil()));
        return;
    }
    let locked = view.lock.filter(|(_, t)| *t >= JAVELIN_LOCK).map(|(e, _)| e);
    if weapon == Atgm::Javelin && locked.is_none() {
        radio.say(now, "No lock. Hold the seeker on him.");
        return;
    }
    pos.atgm -= 1;
    pos.reload = weapon.reload();
    // A lock is spent by the shot.
    view.lock = None;
    let gt = cam.compute_transform();
    let fwd = *gt.forward();
    let from = gt.translation + fwd * 1.6 + Vec3::Y * 0.1;
    let rng = &mut fxr.0;
    fx.flash(from, 1.5e6, 30.0, 0.08);
    // The launch blows snow and smoke back over the position: for a moment the gunner is half blind.
    for _ in 0..14 {
        fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: from + Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(-0.5..0.5), rng.random_range(-1.0..1.0)), vel: -fwd * rng.random_range(2.0..8.0) + Vec3::Y * rng.random_range(0.2..1.5), life: rng.random_range(3.0..6.0), size0: 1.2, size1: 6.0, alpha: 0.5, drag: 1.2, ..default() });
    }
    sound.now(Sfx::AtgmLaunch, 1.3);
    shake.trauma = (shake.trauma + 0.35).min(1.0);
    let missile = (Mesh3d(models.missile.clone()), MeshMaterial3d(palette.vertex.clone()), NotShadowCaster);
    match (weapon, locked) {
        (Atgm::Javelin, Some(target)) => {
            // Fire and forget: it climbs, then dives onto the roof by itself.
            commands.spawn((missile, Transform::from_translation(from).looking_to((fwd + Vec3::Y * 0.6).normalize(), Vec3::Y), Missile { target, launched_at: from, speed: weapon.speed(), trail: 0.0, p_hit: None, top_attack: true, weapon: weapon.name(), from: "your team" }));
            radio.say(now, "Javelin away. Get down, move.");
        }
        _ => {
            commands.spawn((missile, Transform::from_translation(from).looking_to(fwd, Vec3::Y), GuidedMissile { travelled: 1.2, speed: 40.0, trail: 0.0, team, eye: gt.translation, dir: fwd }));
            radio.say2(now, format!("{weapon}: пуск. Тримай його."), format!("{weapon}: launch. Hold it on him."));
        }
    }
}

/// Distance from `c` to the segment `a`–`b`.
fn segment_distance(a: Vec3, b: Vec3, c: Vec3) -> f32 {
    let ab = b - a;
    let t = if ab.length_squared() > 0.0 { ((c - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
    (a + ab * t).distance(c)
}

fn fly_guided(
    mut commands: Commands,
    time: Res<Time>,
    view: Res<View>,
    mut fx: ResMut<FxQueue>,
    mut fxr: ResMut<FxRng>,
    mut hits: MessageWriter<DirectHit>,
    mut paint: ResMut<PaintQueue>,
    mut sound: Sound,
    mut radio: ResMut<Radio>,
    cam: Single<&GlobalTransform, With<MainCamera>>,
    vehicles: Query<(Entity, &Vehicle, &Transform), Without<GuidedMissile>>,
    mut q: Query<(Entity, &mut GuidedMissile, &mut Transform)>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let gt = cam.compute_transform();
    for (e, mut m, mut tf) in &mut q {
        // Boost, then sustain at about 190 m/s.
        m.speed = (m.speed + 240.0 * dt).min(Atgm::Stugna.speed());
        m.travelled += m.speed * dt;
        // The guidance steers onto the sight's line at the missile's range. Only the launching sight steers it;
        // if the player leaves it, the wire carries the last line it held.
        if view.mode == Mode::Ground && view.ground_team == Some(m.team) {
            m.dir = *gt.forward();
        }
        // Real missiles fly a little above the sight line, clear of the ground in front.
        let lift = Vec3::Y * (1.2 + 1.6 * (1.0 - (m.travelled / 400.0).min(1.0)));
        let los = m.eye + m.dir * m.travelled + lift;
        let prev = tf.translation;
        let step = (los - prev).clamp_length_max(m.speed * dt * 1.25);
        let dir = step.normalize_or(*tf.forward());
        tf.translation += step;
        tf.look_to(dir, Vec3::Y);
        m.trail += dt;
        let rng = &mut fxr.0;
        if m.trail > 0.025 && m.travelled < 450.0 {
            m.trail = 0.0;
            fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: tf.translation, vel: Vec3::Y * 0.3, life: 4.0, size0: 0.3, size1: 2.0, alpha: 0.5, drag: 0.5, ..default() });
            fx.spawn(fx::Spawn { kind: fx::Kind::Flash, pos: tf.translation - dir * 0.6, vel: Vec3::ZERO, life: 0.04, size0: 0.9, size1: 0.6, alpha: 1.0, ..default() });
        }
        let p = tf.translation;
        // Test the whole path flown this frame, so a fast missile cannot pass through a hull between samples.
        // The player is Ukrainian: the missile is flown at Russian vehicles, and passes over our own.
        let hit = vehicles.iter().find(|(_, v, vt)| v.side == Side::Ru && segment_distance(prev, p, vt.translation + Vec3::Y * 1.2) < 3.8);
        let ground = m.travelled > 40.0 && p.y < geo::height(p.x, p.z) + 0.15;
        if let Some((target, v, _)) = hit {
            commands.entity(e).despawn();
            fx.explosion(p, 0.7, false);
            sound.at(p, Sfx::BlastNear, 1.0);
            if v.state.alive() {
                hits.write(DirectHit { target, cause: Cause::Atgm, top: false, weapon: "Stugna-P", from: "your team" });
                radio.say(now, "Hit! He's burning.");
            }
        } else if ground || m.travelled > Atgm::Stugna.range() + 700.0 {
            commands.entity(e).despawn();
            if ground {
                fx.explosion(p, 0.6, true);
                paint.push(Paint::Crater { at: p.xz(), radius: 1.2, seed: rng.random() });
                sound.at(p, Sfx::BlastNear, 0.8);
                radio.say(now, "Miss. Into the ground.");
            }
        }
    }
}
