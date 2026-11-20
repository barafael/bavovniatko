//! The Bradleys of ours dug in on the shoulder (design/avdiivka.md, phases 3 and 6): a `Dug(Bradley)` unit in the
//! catalog stands hull-down at a hide. By itself it works what it sees within a mile and a half: 25 mm AP into
//! BMPs and MT-LBs, bursts at a tank's sights and tracks, a TOW at a tank it can hold in the sight. The player can
//! take a gunner's seat (`Mode::Bradley`, key 6): mouse look turns the turret, the right button is the sight (day,
//! or the thermal channel on I), the left button a burst of what is selected (X: AP or HE), T a TOW that follows
//! the crosshair out to the hull.
//!
//! A Bradley also goes out on a **run** (`Run`): out of its hide a hundred-odd metres towards what it goes for, its
//! gun working, and back into the hide under its smoke. The script orders one (`Act::Run`, part III's pair: the first
//! pass that fails, then the second, v02 §6, §7), and so does the player from the seat: G / K / B order the driver on,
//! to hold, or back, as Command's orders do for a unit. A tank that sees a Bradley out of its hide turns its gun on it
//! (`tanks_answer`); a TOW's launch or a long burst gives the hide away to the enemy staff (`Revealed`), and in Command
//! the Lancets come for it.

use crate::fire::{GuidedMissile, Missile};
use crate::fx::{self, Effects, FxQueue};
use crate::geo;
use crate::hits::{HitEntry, HitLog, Victim};
use crate::player::{Controls, MainCamera, View};
use crate::sound::{Sfx, Sound};
use crate::units::{self, Blast, Dress, Blinded, Cause, Coasting, CrewPlan, DirectHit, Driver, Gear, Gun, Radio, Turret, VState, Vehicle};
use bav_core::diorama::{Play, Side, play};
use bav_core::vocab::Mode;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct BradleyPlugin;

/// Whether the day has Bradleys dug in (a `Dug` unit in its catalog): only then is any of this in its world.
pub fn present() -> bool {
    crate::data::sim_data().catalog.iter().any(|u| matches!(u.what, crate::allot::UnitKind::Dug(_)))
}

/// Whether the Bradleys keep quiet until their run (`Opens::Run`): the part is a duel (part III), and keeps its ledger.
pub fn opens_on_run() -> bool {
    crate::data::sim_data().bradley.is_some_and(|b| b.opens == Opens::Run)
}

impl Plugin for BradleyPlugin {
    fn build(&self, app: &mut App) {
        if !present() {
            return;
        }
        app.insert_resource(BradleyRng(SmallRng::seed_from_u64(crate::dev::seeded(2525))))
            // After the guns, the shells and the aircraft (Avdiivka has Su-25s and Iglas too): they share the effects, the
            // hulls' places and the net, and the day must replay the same.
            .add_systems(Update, (seat_orders, start_runs, player_gun, auto_guns, drive_runs, spin_blind, turrets).chain().run_if(crate::running).in_set(crate::Step::Fire).after(crate::fire::FireSystems).after(crate::air::AirSystems))
            .add_systems(Update, tanks_answer.run_if(crate::running).in_set(crate::Step::Fire).after(crate::manpads::ManpadsSystems).before(crate::fire::FireSystems).before(crate::air::AirSystems));
        if opens_on_run() {
            app.init_resource::<Duel>().add_systems(Update, keep_duel.after(turrets).run_if(crate::running).in_set(crate::Step::Fire));
        }
    }
}

#[derive(Resource)]
struct BradleyRng(SmallRng);

/// Our Bradleys under way, not coasting to a stop.
type Bradleys = (With<BradleyArms>, Without<Coasting>);

/// Where a run is told: the radio, and the duel's record of runs when the day keeps one.
#[derive(bevy::ecs::system::SystemParam)]
struct RunLog<'w> {
    radio: ResMut<'w, Radio>,
    duel: Option<ResMut<'w, Duel>>,
}

/// What a gun firing at a hull needs: the clock, the Bradleys' random stream, the effects and the sound, the hits it
/// makes, and the turrets its muzzle is on.
#[derive(bevy::ecs::system::SystemParam)]
struct Gunnery<'w, 's> {
    time: Res<'w, Time>,
    rng: ResMut<'w, BradleyRng>,
    effects: Effects<'w, 's>,
    hits: MessageWriter<'w, DirectHit>,
    turrets: Query<'w, 's, &'static GlobalTransform, With<Turret>>,
}

/// What a crew left to themselves can see: the hulls on the field, and the smoke that may hide them.
#[derive(bevy::ecs::system::SystemParam)]
struct Sight<'w, 's> {
    smoke: Res<'w, fx::Smoke>,
    vehicles: Query<'w, 's, (Entity, &'static Vehicle, &'static Transform)>,
}

/// The player in a Bradley's seat: the keys and buttons, the view and the sight's camera, the TOW they are guiding,
/// if one is in the air, and the crew's voice on the radio.
#[derive(bevy::ecs::system::SystemParam)]
struct Seat<'w, 's> {
    controls: Controls<'w>,
    view: ResMut<'w, View>,
    cam: Single<'w, 's, &'static GlobalTransform, With<MainCamera>>,
    guided: Query<'w, 's, (), With<GuidedMissile>>,
    radio: ResMut<'w, Radio>,
}

/// What putting up a smoke screen takes: the Bradleys' random stream, the effects, and the smoke on the field.
#[derive(bevy::ecs::system::SystemParam)]
struct Screen<'w> {
    rng: ResMut<'w, BradleyRng>,
    fx: ResMut<'w, FxQueue>,
    smoke: ResMut<'w, fx::Smoke>,
}

/// A Bradley's gun as its crew work it on their own: the hull, its arms and gun, the run it is on, and whether it is
/// to keep quiet or is on the pass that fails.
type Gunner = (Entity, &'static Vehicle, &'static Transform, &'static mut BradleyArms, &'static mut Gun, Option<&'static mut Run>, Has<Quiet>, Has<Failing>);

/// A Bradley on a run as its driver takes it: the hull, the driver, where it is, the run, and whether it is the pass
/// that fails.
type OnRun = (Entity, &'static Vehicle, &'static mut Driver, &'static mut Transform, &'static mut Run, Has<Failing>);

/// Hulls that can answer a Bradley: not Bradleys themselves, and not blinded.
type Answering = (Without<BradleyArms>, Without<Blinded>);

/// The sight's magnification (the ISU's low setting).
pub const SIGHT_ZOOM: f32 = 4.0;
/// How far a burst is worth firing, and a TOW.
pub const GUN_RANGE: f32 = 1800.0;
pub const TOW_RANGE: f32 = 2800.0;
/// Rounds a burst.
pub const BURST: u32 = 6;
/// Rounds in the last few seconds that make a long burst, which the enemy sees from far off.
const LONG_BURST: f32 = 18.0;
/// How far a tank sees a Bradley out of its hide well enough to lay its gun on it.
const TANK_SIGHT: f32 = 1500.0;
/// How near a stopped hull of theirs must be for a run ordered by the script to go at it.
const RUN_REACH: f32 = 1000.0;

/// When the Bradleys' gunners left to themselves first open fire.
#[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Opens {
    /// At whatever they see within their reach (parts I and II).
    #[default]
    Seen,
    /// Not before their run: until it, a Bradley in its hide keeps quiet (`Quiet`) and the tank fires on the trench
    /// first (part III, v02 §1, §6).
    Run,
}

/// A Bradley's arms: the ready boxes of 25 mm, the TOWs in the launcher and the reloads, and the gunner's selection.
#[derive(Component)]
pub struct BradleyArms {
    pub ap: u32,
    pub he: u32,
    pub tows: u32,
    pub tow_reload: f32,
    pub burst_timer: f32,
    pub he_selected: bool,
    /// Rounds fired lately, falling away a few a second: a long burst is seen from far off.
    pub heat: f32,
}

impl Default for BradleyArms {
    fn default() -> Self {
        let tows = crate::data::sim_data().bradley.map_or(7, |b| b.tows);
        Self { ap: 300, he: 230, tows, tow_reload: 10.0, burst_timer: 6.0, he_selected: false, heat: 0.0 }
    }
}

/// A Bradley in its hide that has not opened fire yet (`Opens::Run`): it waits for its run.
#[derive(Component)]
pub struct Quiet;

/// A run the script has ordered, waiting for a hull of theirs to stop within reach and for the other Bradley to be
/// back in its hide: the pair go one after the other ("it didn't work out, so we had to muster up the courage and
/// go", v02 §7).
#[derive(Component)]
pub struct RunOrdered {
    fails: bool,
    say: &'static str,
}

/// A pass that fails, as the first crew's did at Stepove: its AP does nothing to the tank's front, and after a few
/// bursts it backs off under its smoke.
#[derive(Component)]
pub struct Failing;

/// A Bradley out of its hide on a run: the hide and the far end of the run, how far out it is, which way its driver is
/// going, and what it went for.
#[derive(Component)]
pub struct Run {
    pub hide: Vec2,
    pub end: Vec2,
    /// Metres out from the hide.
    pub along: f32,
    pub leg: Leg,
    pub target: Option<Entity>,
    /// Seconds since it left the hide, and the bursts it has fired since.
    pub out: f32,
    pub bursts: u32,
    /// Shots a tank has fired at it on this run (the first goes wide: v02 §1, "the T-90M fired first and missed").
    pub shot_at: u32,
    /// The hull's heading out along the run.
    yaw: f32,
}

/// Which way the driver is going on a run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Leg {
    /// Out towards the far end of the run.
    Out,
    /// Stopped where it is, the gun working.
    Held,
    /// Back into the hide, hull first, the glacis to the enemy.
    Back,
}

impl Leg {
    pub fn name(self) -> &'static str {
        match self {
            Leg::Out => "ON",
            Leg::Held => "HOLDING",
            Leg::Back => "BACK",
        }
    }
}

/// A Bradley that has given its hide away to the enemy: a TOW's launch, or a long burst (`staff::spot_fire` reads it).
#[derive(Component)]
pub struct Revealed {
    pub at: f64,
}

/// Part III's ledger: every hit the feed told of, and the pair's runs, for the debrief.
#[derive(Resource, Default)]
pub struct Duel {
    pub hits: Vec<HitEntry>,
    /// What the pair did, by game time: "out of its hide", "back in its hide".
    pub runs: Vec<(f64, String)>,
    read: u64,
}

/// The script's run for the `k`th Bradley of the day's (`Act::Run`): it waits in its hide until it has something to
/// go at.
pub fn order_run(w: &mut World, k: usize, fails: bool, say: &'static str) {
    let mut q = w.query_filtered::<(Entity, &Vehicle), (With<BradleyArms>, Without<Run>)>();
    let found = q.iter(w).find(|(_, v)| v.idx == 200 + k as u32 && v.crew > 0 && v.state.alive()).map(|(e, _)| e);
    match found {
        Some(e) => {
            w.entity_mut(e).insert(RunOrdered { fails, say });
        }
        None => info!("bradley: no Bradley {k} in its hide for the run"),
    }
}

/// A sight line from `eye` along `fwd`: the enemy vehicle nearest along it whose hull it passes within three metres,
/// out to `range`.
fn along_line<'a>(eye: Vec3, fwd: Vec3, range: f32, vehicles: impl Iterator<Item = (Entity, &'a Vehicle, &'a Transform)>) -> Option<(Entity, f32)> {
    vehicles
        .filter(|(_, v, _)| v.side == Side::Ru && v.state.alive())
        .filter_map(|(e, _, t)| {
            let c = t.translation + Vec3::Y * 1.2;
            let along = (c - eye).dot(fwd);
            if !(5.0..range).contains(&along) {
                return None;
            }
            let off = (c - eye - fwd * along).length();
            (off < 3.0).then_some((e, along))
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
}

/// The gun's muzzle, from the turret's frame.
fn muzzle_of(v: &Vehicle, tf: &Transform, turrets: &Query<&GlobalTransform, With<Turret>>) -> Vec3 {
    turrets.get(v.turret).map(|t| t.transform_point(v.muzzle)).unwrap_or(tf.translation + Vec3::Y * 2.2)
}

/// A burst's picture and sound: the muzzle, the tracers out along the line, the report.
fn burst_fx(fx: &mut FxQueue, sound: &mut Sound, rng: &mut SmallRng, muzzle: Vec3, dir: Vec3, range: f32, side: Side) {
    fx.muzzle(muzzle, dir, false);
    sound.at(muzzle, Sfx::Cannon25, 0.9);
    for k in 0..BURST {
        if k % 2 == 0 {
            let miss = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(-0.5..0.5), rng.random_range(-1.0..1.0)) * 0.004;
            fx.tracer(muzzle + dir * (k as f32 * 2.0), muzzle + (dir + miss).normalize() * range, 1100.0, side);
        }
    }
}

/// A burst fired: the barrel's heat, and in Command a long one gives the hide away.
fn fired(commands: &mut Commands, e: Entity, a: &mut BradleyArms, now: f64) {
    a.heat += BURST as f32;
    if a.heat >= LONG_BURST && play() == Play::Command {
        commands.entity(e).insert(Revealed { at: now });
    }
}

/// Whether a hull of theirs is still a thing to fight: alive, crewed, and with its sights.
fn in_action(v: &Vehicle, blinded: bool) -> bool {
    v.state.alive() && v.crew > 0 && !blinded && v.state != VState::Withdrawing
}

/// The way out of a hide towards `to`: the far end of a run of `run` metres, stopping 250 m short of it.
fn run_end(hide: Vec2, to: Vec2, run: f32) -> Vec2 {
    let d = to - hide;
    hide + d.normalize_or(Vec2::NEG_Y) * run.min(d.length() - 250.0).max(30.0)
}

fn run_length() -> f32 {
    crate::data::sim_data().bradley.map_or(150.0, |b| b.run)
}

/// From the seat, G / K / B order the driver on, to hold, or back: out of the hide along the gunner's line, stop
/// where it is, back into the hide. The driver answers on the intercom.
fn seat_orders(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    view: Res<View>,
    log: RunLog,
    cam: Single<&GlobalTransform, With<MainCamera>>,
    mut q: Query<(&Vehicle, &Transform, Option<&mut Run>), Bradleys>,
) {
    let RunLog { mut radio, mut duel } = log;
    if view.mode != Mode::Bradley {
        return;
    }
    let Some(e) = view.ground_team else { return };
    let Ok((v, tf, run)) = q.get_mut(e) else { return };
    if v.crew == 0 || !v.state.alive() {
        return;
    }
    let now = time.elapsed_secs_f64();
    let order = [(KeyCode::KeyG, Leg::Out), (KeyCode::KeyK, Leg::Held), (KeyCode::KeyB, Leg::Back)].into_iter().find(|(k, _)| keys.just_pressed(*k)).map(|(_, l)| l);
    let Some(leg) = order else { return };
    match (run, leg) {
        (Some(mut r), leg) => {
            if r.leg != leg {
                r.leg = leg;
                say_leg(&mut radio, now, leg);
            }
        }
        (None, Leg::Out) => {
            // Out of the hide along the gunner's line.
            let hide = tf.translation.xz();
            let fwd = cam.forward().xz().normalize_or(Vec2::NEG_Y);
            let end = run_end(hide, hide + fwd * 2000.0, run_length());
            commands.entity(e).remove::<(Quiet, RunOrdered)>().insert(new_run(hide, end, None));
            say_leg(&mut radio, now, Leg::Out);
            if let Some(d) = duel.as_mut() {
                d.runs.push((now, "your Bradley out of its hide".into()));
            }
        }
        (None, _) => radio.say2(now, "Ми в капонірі.", "We're in the hide."),
    }
}

fn say_leg(radio: &mut Radio, now: f64, leg: Leg) {
    match leg {
        Leg::Out => radio.say2(now, "Пішли.", "Driver: moving out."),
        Leg::Held => radio.say2(now, "Стоїмо.", "Driver: holding."),
        Leg::Back => radio.say2(now, "Назад, у капонір.", "Driver: backing into the hide."),
    }
}

fn new_run(hide: Vec2, end: Vec2, target: Option<Entity>) -> Run {
    Run { hide, end, along: 0.0, leg: Leg::Out, target, out: 0.0, bursts: 0, shot_at: 0, yaw: units::vehicle_yaw(end - hide) }
}

/// A run the script ordered goes once a hull of theirs has stopped within reach of the hide, in its sight and out of
/// the smoke (a tank before anything else), and once the other Bradley is back in its hide.
fn start_runs(
    mut commands: Commands,
    time: Res<Time>,
    log: RunLog,
    smoke: Res<fx::Smoke>,
    out: Query<&Vehicle, (With<Run>, Without<Coasting>)>,
    vehicles: Query<(Entity, &Vehicle, &Transform, Has<Blinded>)>,
    ordered: Query<(Entity, &Vehicle, &Transform, &RunOrdered), Without<Run>>,
) {
    let RunLog { mut radio, mut duel } = log;
    if out.iter().any(|v| v.state.alive() && v.crew > 0) {
        return;
    }
    let now = time.elapsed_secs_f64();
    for (e, v, tf, o) in &ordered {
        if v.crew == 0 || !v.state.alive() {
            commands.entity(e).remove::<RunOrdered>();
            continue;
        }
        let hide = tf.translation.xz();
        let mut seen: Vec<(Entity, bool, Vec2)> = vehicles
            .iter()
            .filter(|(_, o, t, blind)| o.side == Side::Ru && in_action(o, *blind) && matches!(o.state, VState::Overwatch | VState::Dismounting | VState::Halted | VState::Immobilized) && t.translation.xz().distance(hide) < RUN_REACH)
            .map(|(e, o, t, _)| (e, o.kind.is_tank(), t.translation.xz()))
            .filter(|(_, _, p)| geo::line_of_sight(hide, *p) && !smoke.blocks(hide, *p, now))
            .collect();
        seen.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.distance(hide).total_cmp(&b.2.distance(hide))).then(a.2.x.total_cmp(&b.2.x)));
        let Some(&(target, _, at)) = seen.first() else { continue };
        let end = run_end(hide, at, run_length());
        let mut c = commands.entity(e);
        c.remove::<(RunOrdered, Quiet)>().insert(new_run(hide, end, Some(target)));
        if o.fails {
            c.insert(Failing);
        }
        if !o.say.is_empty() {
            radio.say(now, o.say);
        }
        info!("bradley: {} out of its hide at {:.0},{:.0} for the hull at {:.0},{:.0}{}, at {now:.0} s", v.idx - 200, hide.x, hide.y, at.x, at.y, if o.fails { " (the pass that fails)" } else { "" });
        if let Some(d) = duel.as_mut() {
            d.runs.push((now, format!("{} out of its hide", if o.fails { "the first Bradley" } else { "the second Bradley" })));
        }
        // One at a time.
        return;
    }
}

/// The player's seat: the right button is the sight, the left a burst of what X has selected, T a TOW.
fn player_gun(
    mut commands: Commands,
    seat: Seat,
    gunnery: Gunnery,
    mut blasts: MessageWriter<Blast>,
    dress: Dress,
    vehicles: Query<(Entity, &Vehicle, &Transform)>,
    mut arms: Query<(&Vehicle, &Transform, &mut BradleyArms, &mut Gun, Option<&mut Run>)>,
) {
    let Seat { controls: Controls { keys, buttons }, mut view, cam, guided, mut radio } = seat;
    let Gunnery { time, mut rng, effects: Effects { mut fx, mut sound }, mut hits, turrets } = gunnery;
    let Dress { models, palette } = dress;
    if view.mode != Mode::Bradley {
        return;
    }
    let now = time.elapsed_secs_f64();
    let Some(e) = view.ground_team else { return };
    let Ok((v, tf, mut a, mut gun, run)) = arms.get_mut(e) else {
        view.mode = Mode::Drone;
        return;
    };
    if v.crew == 0 || !v.state.alive() {
        radio.say(now, "This one is done. Nobody left at the gun.");
        view.mode = Mode::Drone;
        return;
    }
    let gt = cam.compute_transform();
    let fwd = *gt.forward();
    // The gunner's hand on the turret: it follows the sight.
    gun.aim = Some((gt.translation + fwd * 600.0).xz());
    // The range readout: the hull on the line, else the ground the line meets.
    view.ground_range = along_line(gt.translation, fwd, TOW_RANGE, vehicles.iter()).map(|(_, d)| d).or_else(|| geo::raycast(gt.translation, fwd, 4000.0).map(|p| p.distance(gt.translation)));
    if keys.just_pressed(KeyCode::KeyX) {
        a.he_selected = !a.he_selected;
        radio.say(now, if a.he_selected { "HE up." } else { "AP up." });
    }
    if !view.sight {
        return;
    }
    let rng = &mut rng.0;
    let muzzle = muzzle_of(v, tf, &turrets);
    if keys.just_pressed(KeyCode::KeyT) && guided.is_empty() {
        if a.tows == 0 {
            radio.say(now, "No TOWs left in the launcher.");
        } else if a.tow_reload > 0.0 {
            radio.say2(now, format!("Перезарядка. {:.0} секунд.", a.tow_reload.ceil()), format!("Reloading the launcher. {:.0} seconds.", a.tow_reload.ceil()));
        } else {
            a.tows -= 1;
            a.tow_reload = 30.0;
            fx.flash(muzzle, 1.5e6, 30.0, 0.08);
            for _ in 0..10 {
                fx.spawn(fx::Spawn { kind: fx::Kind::WhiteSmoke, pos: muzzle + Vec3::new(rng.random_range(-1.0..1.0), 0.0, rng.random_range(-1.0..1.0)), vel: -fwd * rng.random_range(2.0..6.0) + Vec3::Y, life: rng.random_range(3.0..5.0), size0: 1.0, size1: 5.0, alpha: 0.5, drag: 1.2, ..default() });
            }
            sound.now(Sfx::AtgmLaunch, 1.2);
            commands.spawn((Mesh3d(models.missile.clone()), MeshMaterial3d(palette.vertex.clone()), NotShadowCaster, Transform::from_translation(muzzle + Vec3::Y * 0.3).looking_to(fwd, Vec3::Y), GuidedMissile { travelled: 1.2, speed: 40.0, trail: 0.0, team: e, eye: gt.translation, dir: fwd, weapon: "TOW", from: "your Bradley" }));
            radio.say2(now, "ТОУ: пуск. Тримай його.", "TOW: launch. Hold him.");
            if play() == Play::Command {
                commands.entity(e).insert(Revealed { at: now });
            }
        }
        return;
    }
    if !buttons.just_pressed(MouseButton::Left) || a.burst_timer > 0.0 {
        return;
    }
    let (stock, he) = if a.he_selected { (&mut a.he, true) } else { (&mut a.ap, false) };
    if *stock < BURST {
        radio.say(now, if he { "HE box is empty." } else { "AP box is empty. Switch to HE, or reload." });
        return;
    }
    *stock -= BURST;
    a.burst_timer = 1.2;
    fired(&mut commands, e, &mut a, now);
    commands.entity(e).remove::<Quiet>();
    if let Some(mut r) = run {
        r.bursts += 1;
    }
    let target = along_line(gt.translation, fwd, GUN_RANGE, vehicles.iter());
    let range = target.map_or(GUN_RANGE, |(_, d)| d);
    burst_fx(&mut fx, &mut sound, rng, muzzle, fwd, range, Side::Ua);
    let tank = target.is_some_and(|(t, _)| vehicles.get(t).is_ok_and(|(_, o, _)| o.kind.is_tank()));
    match target {
        Some((t, d)) if !he || tank => {
            // A stabilised sight, a short burst: it goes where the crosshair is, nearly always. HE on a tank goes for
            // the sights (v02 §6).
            let p = (0.9 - d / 6000.0).clamp(0.3, 0.9);
            if rng.random_bool(p as f64) {
                let (cause, weapon) = if he { (Cause::Blinding, "25 mm HE") } else { (Cause::Autocannon, "25 mm") };
                hits.write(DirectHit { target: t, cause, top: false, weapon, from: "your Bradley" });
                fx.explosion(gt.translation + fwd * d, 0.3, false);
            } else {
                fx.explosion(geo::ground((gt.translation + fwd * d).xz()) + Vec3::Y * 0.3, 0.25, true);
            }
        }
        _ => {
            // HE, or AP into the ground: six small bursts where the line meets the field, enough for men.
            let Some(at) = geo::raycast(gt.translation, fwd, 3000.0) else { return };
            for _ in 0..BURST {
                let bp = geo::ground(at.xz() + Vec2::new(rng.random_range(-3.0..3.0), rng.random_range(-3.0..3.0)));
                fx.explosion(bp, if he { 0.3 } else { 0.18 }, true);
                blasts.write(Blast { at: bp, radius: if he { 4.0 } else { 1.5 }, cause: Cause::Artillery });
            }
            if let Some((t, _)) = target
                && he
                && rng.random_bool(0.35)
            {
                // HE on a hull does little, but a lucky one finds the optics or the engine deck.
                hits.write(DirectHit { target: t, cause: Cause::Autocannon, top: false, weapon: "25 mm HE", from: "your Bradley" });
            }
        }
    }
}

/// The Bradleys nobody is sitting in work the field by themselves: a TOW at a tank they can see, AP into anything
/// else, a burst at a tank's front when the launcher is reloading. One on a run goes for what it went out for, as
/// fast as the gun will take it ("we fired with all we could", v02 §6): AP first, and when that does nothing to a
/// tank, HE at its sights.
fn auto_guns(mut commands: Commands, view: Res<View>, gunnery: Gunnery, mut log: ResMut<HitLog>, sight: Sight, dress: Dress, mut arms: Query<Gunner>) {
    let Gunnery { time, mut rng, effects: Effects { mut fx, mut sound }, mut hits, turrets } = gunnery;
    let Sight { smoke, vehicles } = sight;
    let Dress { models, palette } = dress;
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    for (e, v, tf, mut a, mut gun, mut run, quiet, failing) in &mut arms {
        a.tow_reload = (a.tow_reload - dt).max(0.0);
        a.burst_timer = (a.burst_timer - dt).max(0.0);
        a.heat = (a.heat - dt * 2.5).max(0.0);
        let held = view.mode == Mode::Bradley && view.ground_team == Some(e);
        if held || quiet || v.crew == 0 || !v.state.alive() || a.burst_timer > 0.0 {
            continue;
        }
        let from = tf.translation.xz();
        let eye = tf.translation + Vec3::Y * 2.4;
        // What it can see: theirs, alive, out of the smoke, in a fixed order (by place, not by entity id).
        let mut seen: Vec<(Entity, &Vehicle, Vec3)> = vehicles
            .iter()
            .filter(|(_, o, t)| o.side == Side::Ru && o.state.alive() && t.translation.xz().distance(from) < TOW_RANGE && geo::line_of_sight(from, t.translation.xz()) && !smoke.blocks(from, t.translation.xz(), now))
            .map(|(e, o, t)| (e, o, t.translation))
            .collect();
        seen.sort_by(|x, y| x.2.distance(eye).total_cmp(&y.2.distance(eye)).then(x.2.x.total_cmp(&y.2.x)));
        if seen.is_empty() {
            a.burst_timer = 1.5;
            continue;
        }
        let muzzle = muzzle_of(v, tf, &turrets);
        // A tank it can hold in the sight gets the TOW; else the nearest thing gets the gun.
        let tow_reach = crate::data::sim_data().bradley.map_or(TOW_RANGE, |b| b.range.min(TOW_RANGE));
        if a.tows > 0 && a.tow_reload <= 0.0 && let Some((t, _, p)) = seen.iter().find(|(_, o, p)| o.kind.is_tank() && p.distance(eye) < tow_reach) {
            a.tows -= 1;
            a.tow_reload = rng.random_range(25.0..40.0);
            a.burst_timer = 3.0;
            gun.aim = Some(p.xz());
            fx.flash(muzzle, 1.5e6, 30.0, 0.08);
            sound.at(muzzle, Sfx::AtgmLaunch, 1.0);
            let d = p.distance(eye);
            commands.spawn((
                Mesh3d(models.missile.clone()),
                MeshMaterial3d(palette.vertex.clone()),
                NotShadowCaster,
                Transform::from_translation(muzzle + Vec3::Y * 0.3).looking_at(*p, Vec3::Y),
                Missile { target: *t, launched_at: muzzle, speed: 190.0, trail: 0.0, p_hit: Some((0.65 - d / 8000.0).clamp(0.25, 0.6)), top_attack: false, weapon: "TOW", from: "a Bradley" },
            ));
            if play() == Play::Command {
                commands.entity(e).insert(Revealed { at: now });
            }
            continue;
        }
        // On a run, what it went out for; else the nearest.
        let went_for = run.as_ref().and_then(|r| r.target).and_then(|t| seen.iter().find(|s| s.0 == t)).copied();
        let (t, o, p) = went_for.unwrap_or(seen[0]);
        let d = p.distance(eye);
        let reach = if run.is_some() { GUN_RANGE } else { reach() };
        if d > reach || a.ap < BURST {
            a.burst_timer = 2.0;
            continue;
        }
        // On a run, HE at a tank's sights once its AP has done nothing (the failing pass keeps to AP).
        let blinding = o.kind.is_tank() && !failing && run.as_ref().is_some_and(|r| r.bursts >= 1) && a.he >= BURST;
        if blinding {
            a.he -= BURST;
        } else {
            a.ap -= BURST;
        }
        a.burst_timer = if run.is_some() { rng.random_range(2.0..3.5) } else { rng.random_range(4.0..8.0) };
        fired(&mut commands, e, &mut a, now);
        if let Some(r) = run.as_mut() {
            r.bursts += 1;
        }
        gun.aim = Some(p.xz());
        let dir = (p + Vec3::Y * 1.0 - muzzle).normalize();
        burst_fx(&mut fx, &mut sound, rng, muzzle, dir, d, Side::Ua);
        if failing && o.kind.is_tank() {
            // The first pass: the AP off the front of the tank, and nothing to show for it.
            log.open(now, "25 mm", "the first Bradley", Victim { target: t, vehicle: o, rescue: false }, "AP off the front armour: nothing");
            fx.explosion(p + Vec3::Y * 1.0, 0.3, false);
            continue;
        }
        // A burst at a mile: a hit on a hull more often than not; a tank's front shrugs most of it off, and the
        // hit itself decides what it took (`units::resolve_direct_hits`). HE at the sights wants the turret's whole
        // face, not a weak spot in it.
        let p_hit = (0.7 - d / 3500.0).clamp(0.15, 0.6) * if o.kind.is_tank() && !blinding { 0.6 } else { 1.0 };
        if rng.random_bool(p_hit as f64) {
            let (cause, weapon) = if blinding { (Cause::Blinding, "25 mm HE") } else { (Cause::Autocannon, "25 mm") };
            hits.write(DirectHit { target: t, cause, top: false, weapon, from: if run.is_some() { "the second Bradley" } else { "a Bradley" } });
            fx.explosion(p + Vec3::Y * 1.0, 0.3, false);
        } else {
            fx.explosion(geo::ground(p.xz() + Vec2::new(rng.random_range(-6.0..6.0), rng.random_range(-6.0..6.0))) + Vec3::Y * 0.3, 0.25, true);
        }
    }
}

/// How far the gunners left to themselves open fire: the file's, else the gun's reach.
fn reach() -> f32 {
    crate::data::sim_data().bradley.map_or(GUN_RANGE, |b| b.range.min(GUN_RANGE))
}

/// The driver on a run: he turns the hull onto the run's line, takes it out at a run, holds where he is told, and
/// backs it into the hide again under its smoke. Left to themselves the crew come back when what they went for is out
/// of the fight, after a minute and a half out, or, on the pass that fails, after a few bursts that did nothing.
fn drive_runs(
    mut commands: Commands,
    time: Res<Time>,
    view: Res<View>,
    screen: Screen,
    log: RunLog,
    targets: Query<(&Vehicle, Has<Blinded>), Without<BradleyArms>>,
    mut q: Query<OnRun, Bradleys>,
) {
    let Screen { mut rng, mut fx, mut smoke } = screen;
    let RunLog { mut radio, mut duel } = log;
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    for (e, v, mut d, mut tf, mut r, failing) in &mut q {
        if !v.state.alive() || v.crew == 0 {
            continue;
        }
        r.out += dt;
        let held = view.mode == Mode::Bradley && view.ground_team == Some(e);
        let len = r.end.distance(r.hide);
        if !held && r.leg != Leg::Back {
            let done = r.target.is_none_or(|t| targets.get(t).map_or(true, |(o, blind)| !in_action(o, blind)));
            let quit = if failing { r.bursts >= 4 } else { (done && r.target.is_some()) || r.out > 90.0 };
            if quit {
                r.leg = Leg::Back;
                units::pop_smoke(&mut commands, &mut fx, &mut rng.0, &mut smoke, v.side, &tf, now);
                radio.say2(now, if failing { "Не бере! Назад, назад!" } else { "Готово. Назад у капонір." }, if failing { "It's not going in! Back, back!" } else { "Done. Back into the hide." });
            }
        }
        // Onto the run's heading first, on the spot; then the throttle.
        let dyaw = units::wrap(r.yaw - d.yaw);
        let want = match r.leg {
            _ if dyaw.abs() > 0.15 => 0.0,
            Leg::Out if r.along < len => 5.5,
            Leg::Back if r.along > 0.0 => -4.0,
            _ => 0.0,
        };
        d.yaw = units::wrap(d.yaw + dyaw.clamp(-0.55 * dt, 0.55 * dt));
        let signed = if d.gear == Gear::Backing { -d.speed } else { d.speed };
        let speed = signed + (want - signed).clamp(-2.5 * dt, 1.3 * dt);
        r.along = (r.along + speed * dt).clamp(0.0, len);
        if r.leg == Leg::Out && r.along >= len {
            r.leg = Leg::Held;
        }
        d.speed = speed.abs();
        d.gear = if speed < 0.0 { Gear::Backing } else { Gear::Forward };
        let dir = (r.end - r.hide).normalize_or(Vec2::NEG_Y);
        let p = r.hide + dir * r.along;
        // Hull-down in the hide, up on the ground as it leaves it.
        let sunk = 0.9 * (1.0 - r.along / 12.0).clamp(0.0, 1.0);
        tf.translation = geo::ground(p) - Vec3::Y * sunk;
        tf.rotation = Quat::from_rotation_arc(Vec3::Y, geo::normal(p.x, p.y)) * Quat::from_rotation_y(d.yaw);
        if r.leg == Leg::Back && r.along <= 0.0 && d.speed < 0.05 {
            commands.entity(e).remove::<(Run, Failing)>();
            d.speed = 0.0;
            d.gear = Gear::Forward;
            info!("bradley: {} back in its hide after {:.0} s and {} bursts, at {now:.0} s", v.idx - 200, r.out, r.bursts);
            if let Some(du) = duel.as_mut() {
                du.runs.push((now, format!("{} back in its hide, {} bursts fired", if held { "your Bradley" } else if failing { "the first Bradley" } else { "the second Bradley" }, r.bursts)));
            }
        }
    }
}

/// A tank of theirs that sees one of our Bradleys out of its hide turns its gun on it and fires when it is next
/// loaded, in place of the trench (`fire::attacker_guns`). The first shot at a run goes wide, as the T-90M's did at
/// Stepove; after it, a hit is a matter of range and of whether the Bradley is moving.
fn tanks_answer(gunnery: Gunnery, smoke: Res<fx::Smoke>, mut runs: Query<(Entity, &Vehicle, &Transform, &Driver, &mut Run), Without<Coasting>>, mut tanks: Query<(&Vehicle, &mut Gun, &Transform), Answering>) {
    let Gunnery { time, mut rng, effects: Effects { mut fx, mut sound }, mut hits, turrets } = gunnery;
    if runs.is_empty() {
        return;
    }
    let dt = time.delta_secs();
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    for (v, mut gun, tf) in &mut tanks {
        if v.side != Side::Ru || !v.kind.is_tank() || !v.state.alive() || v.crew == 0 || v.state == VState::Withdrawing {
            continue;
        }
        let from = tf.translation.xz();
        let Some((b, bp, moving)) = runs
            .iter()
            .filter(|(_, o, ..)| o.state.alive() && o.crew > 0)
            .map(|(b, _, t, d, _)| (b, t.translation, d.speed > 0.5))
            .filter(|(_, p, _)| p.xz().distance(from) < TANK_SIGHT && geo::line_of_sight(from, p.xz()) && !smoke.blocks(from, p.xz(), now))
            .min_by(|a, c| a.1.xz().distance(from).total_cmp(&c.1.xz().distance(from)))
        else {
            // Lost in its smoke: the gun goes back to the trench rather than into the smoke where it was.
            if gun.aim.is_some_and(|aim| runs.iter().any(|(.., t, _, _)| t.translation.xz().distance(aim) < 40.0)) {
                gun.aim = None;
            }
            continue;
        };
        gun.aim = Some(bp.xz());
        if gun.fire_timer - dt > 0.0 {
            continue;
        }
        // Its shot this time is at the Bradley: the gun's own timer (so `attacker_guns` waits for the next load).
        gun.fire_timer = rng.random_range(14.0..26.0) + dt;
        let Ok((.., mut r)) = runs.get_mut(b) else { continue };
        let muzzle = turrets.get(v.turret).map(|t| t.transform_point(v.muzzle)).unwrap_or(tf.translation + Vec3::Y * 2.0);
        let aim = bp + Vec3::Y * 1.2;
        fx.muzzle(muzzle, (aim - muzzle).normalize(), true);
        sound.at(muzzle, Sfx::TankGun, 1.0);
        let d = from.distance(bp.xz());
        let p_hit = if r.shot_at == 0 { 0.0 } else { (0.5 - d / 3000.0).clamp(0.1, 0.45) * if moving { 0.5 } else { 1.0 } };
        r.shot_at += 1;
        info!("bradley: a {:?} fires on a Bradley out of its hide at {d:.0} m ({}), at {now:.0} s", v.kind, if p_hit > 0.0 { "aimed" } else { "wide" });
        if rng.random_bool(p_hit as f64) {
            hits.write(DirectHit { target: b, cause: Cause::TankGun, top: false, weapon: "125 mm", from: "the tank in the village" });
            fx.explosion(aim, 0.6, false);
            sound.at(aim, Sfx::BlastNear, 0.9);
        } else {
            let miss = bp.xz() + (bp.xz() - from).normalize_or_zero() * rng.random_range(15.0..40.0) + Vec2::new(rng.random_range(-8.0..8.0), rng.random_range(-8.0..8.0));
            fx.explosion(geo::ground(miss), 0.5, true);
        }
    }
}

/// A blinded hull's turret swings loose while the hull drives blind ("backed away with its turret spinning", v02 §1),
/// and stops when the engine does.
fn spin_blind(time: Res<Time>, mut q: Query<(&Vehicle, &mut Gun), With<Blinded>>, mut turrets: Query<&mut Transform, With<Turret>>) {
    let dt = crate::sim_dt(&time);
    for (v, mut gun) in &mut q {
        let CrewPlan::Ghost { secs, .. } = v.plan else { continue };
        if secs <= 0.0 {
            continue;
        }
        gun.turret_yaw = units::wrap(gun.turret_yaw + dt * 0.9 * (secs / 10.0).min(1.0));
        if let Ok(mut ttf) = turrets.get_mut(v.turret) {
            ttf.rotation = Quat::from_rotation_y(gun.turret_yaw);
        }
    }
}

/// The turrets follow the gunners' aim (the dug-in hulls are not driven, so the driving's own tracking leaves
/// them alone).
fn turrets(time: Res<Time>, mut q: Query<(&Vehicle, &Transform, &mut Gun), With<BradleyArms>>, mut turrets: Query<&mut Transform, (With<Turret>, Without<BradleyArms>)>) {
    let dt = crate::sim_dt(&time);
    for (v, tf, mut gun) in &mut q {
        let Some(aim) = gun.aim else { continue };
        let hull_yaw = tf.rotation.to_euler(EulerRot::YXZ).0;
        let rel = units::wrap(units::vehicle_yaw(aim - tf.translation.xz()) - hull_yaw);
        gun.turret_yaw += units::wrap(rel - gun.turret_yaw).clamp(-dt * 1.0, dt * 1.0);
        if let Ok(mut ttf) = turrets.get_mut(v.turret) {
            ttf.rotation = Quat::from_rotation_y(gun.turret_yaw);
        }
    }
}

/// Part III's ledger: the hit feed's entries as they come (the feed keeps only its last twenty), each logged.
fn keep_duel(log: Res<HitLog>, mut duel: ResMut<Duel>) {
    let new = ((log.total - duel.read) as usize).min(log.entries.len());
    duel.read = log.total;
    for h in &log.entries[log.entries.len() - new..] {
        info!("duel: {:.0} s, {} from {} on {} ({:?}): {}", h.at, h.weapon, h.from, h.target_name, h.side, h.result);
        duel.hits.push(h.clone());
    }
}
