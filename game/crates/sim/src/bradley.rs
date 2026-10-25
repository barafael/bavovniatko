//! The Bradleys of ours dug in on the shoulder (design/avdiivka.md, phases 3 and 6): a `Dug(Bradley)` unit in the
//! catalog stands hull-down at a hide. By itself it works what it sees within a mile and a half: 25 mm AP into
//! BMPs and MT-LBs, bursts at a tank's sights and tracks, a TOW at a tank it can hold in the sight. The player can
//! take a gunner's seat (`Mode::Bradley`, key 6): mouse look turns the turret, the right button is the sight (day,
//! or the thermal channel on I), the left button a burst of what is selected (X: AP or HE), T a TOW that follows
//! the crosshair out to the hull.

use crate::fire::{GuidedMissile, Missile};
use crate::fx::{self, FxQueue};
use crate::geo;
use crate::player::{MainCamera, View};
use crate::sound::{Sfx, Sound};
use crate::units::{self, Blast, Cause, DirectHit, Gun, Radio, Turret, Vehicle};
use bav_core::diorama::Side;
use bav_core::vocab::Mode;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct BradleyPlugin;

/// Whether the day has Bradleys dug in (a `Dug` unit in its catalog): only then is any of this in its world.
pub fn present() -> bool {
    crate::data::sim_data().catalog.iter().any(|u| matches!(u.what, crate::allot::UnitKind::Dug(_)))
}

impl Plugin for BradleyPlugin {
    fn build(&self, app: &mut App) {
        if !present() {
            return;
        }
        app.insert_resource(BradleyRng(SmallRng::seed_from_u64(crate::dev::seeded(2525))))
            .add_systems(Update, (player_gun, auto_guns, turrets).chain().run_if(crate::running).in_set(crate::Step::Fire).after(crate::fire::FireSystems));
    }
}

#[derive(Resource)]
struct BradleyRng(SmallRng);

/// The sight's magnification (the ISU's low setting).
pub const SIGHT_ZOOM: f32 = 4.0;
/// How far a burst is worth firing, and a TOW.
pub const GUN_RANGE: f32 = 1800.0;
pub const TOW_RANGE: f32 = 2800.0;
/// Rounds a burst.
pub const BURST: u32 = 6;

/// A Bradley's arms: the ready boxes of 25 mm, the TOWs in the launcher and the reloads, and the gunner's selection.
#[derive(Component)]
pub struct BradleyArms {
    pub ap: u32,
    pub he: u32,
    pub tows: u32,
    pub tow_reload: f32,
    pub burst_timer: f32,
    pub he_selected: bool,
}

impl Default for BradleyArms {
    fn default() -> Self {
        let tows = crate::data::sim_data().bradley.map_or(7, |b| b.tows);
        Self { ap: 300, he: 230, tows, tow_reload: 10.0, burst_timer: 6.0, he_selected: false }
    }
}

/// How far the gunners left to themselves open fire: the file's, else the gun's reach.
fn reach() -> f32 {
    crate::data::sim_data().bradley.map_or(GUN_RANGE, |b| b.range.min(GUN_RANGE))
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
    sound.at(muzzle, Sfx::Cannon30, 0.9);
    for k in 0..BURST {
        if k % 2 == 0 {
            let miss = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(-0.5..0.5), rng.random_range(-1.0..1.0)) * 0.004;
            fx.tracer(muzzle + dir * (k as f32 * 2.0), muzzle + (dir + miss).normalize() * range, 1100.0, side);
        }
    }
}

/// The player's seat: the right button is the sight, the left a burst of what X has selected, T a TOW.
#[allow(clippy::too_many_arguments)]
fn player_gun(
    mut commands: Commands,
    time: Res<Time>,
    (keys, buttons): (Res<ButtonInput<KeyCode>>, Res<ButtonInput<MouseButton>>),
    mut view: ResMut<View>,
    mut rng: ResMut<BradleyRng>,
    mut fx: ResMut<FxQueue>,
    mut sound: Sound,
    mut radio: ResMut<Radio>,
    (mut hits, mut blasts): (MessageWriter<DirectHit>, MessageWriter<Blast>),
    (models, palette): (Res<units::Models>, Res<units::Palette>),
    cam: Single<&GlobalTransform, With<MainCamera>>,
    turrets: Query<&GlobalTransform, With<Turret>>,
    vehicles: Query<(Entity, &Vehicle, &Transform)>,
    mut arms: Query<(&Vehicle, &Transform, &mut BradleyArms, &mut Gun)>,
    guided: Query<(), With<GuidedMissile>>,
) {
    if view.mode != Mode::Bradley {
        return;
    }
    let now = time.elapsed_secs_f64();
    let Some(e) = view.ground_team else { return };
    let Ok((v, tf, mut a, mut gun)) = arms.get_mut(e) else {
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
    let target = along_line(gt.translation, fwd, GUN_RANGE, vehicles.iter());
    let range = target.map_or(GUN_RANGE, |(_, d)| d);
    burst_fx(&mut fx, &mut sound, rng, muzzle, fwd, range, Side::Ua);
    match target {
        Some((t, d)) if !he => {
            // A stabilised sight, a short burst: it goes where the crosshair is, nearly always.
            let p = (0.9 - d / 6000.0).clamp(0.3, 0.9);
            if rng.random_bool(p as f64) {
                hits.write(DirectHit { target: t, cause: Cause::Autocannon, top: false, weapon: "25 mm", from: "your Bradley" });
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
/// else, a burst at a tank's front when the launcher is reloading.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn auto_guns(
    mut commands: Commands,
    time: Res<Time>,
    view: Res<View>,
    mut rng: ResMut<BradleyRng>,
    mut fx: ResMut<FxQueue>,
    mut sound: Sound,
    mut hits: MessageWriter<DirectHit>,
    smoke: Res<fx::Smoke>,
    (models, palette): (Res<units::Models>, Res<units::Palette>),
    turrets: Query<&GlobalTransform, With<Turret>>,
    vehicles: Query<(Entity, &Vehicle, &Transform)>,
    mut arms: Query<(Entity, &Vehicle, &Transform, &mut BradleyArms, &mut Gun)>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let rng = &mut rng.0;
    for (e, v, tf, mut a, mut gun) in &mut arms {
        a.tow_reload = (a.tow_reload - dt).max(0.0);
        a.burst_timer = (a.burst_timer - dt).max(0.0);
        let held = view.mode == Mode::Bradley && view.ground_team == Some(e);
        if held || v.crew == 0 || !v.state.alive() || a.burst_timer > 0.0 {
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
            continue;
        }
        let (t, o, p) = seen[0];
        let d = p.distance(eye);
        if d > reach() || a.ap < BURST {
            a.burst_timer = 2.0;
            continue;
        }
        a.ap -= BURST;
        a.burst_timer = rng.random_range(4.0..8.0);
        gun.aim = Some(p.xz());
        let dir = (p + Vec3::Y * 1.0 - muzzle).normalize();
        burst_fx(&mut fx, &mut sound, rng, muzzle, dir, d, Side::Ua);
        // A burst at a mile: a hit on a hull more often than not; a tank's front shrugs most of it off, and the
        // hit itself decides what it took (`units::resolve_direct_hits`).
        let p_hit = (0.7 - d / 3500.0).clamp(0.15, 0.6) * if o.kind.is_tank() { 0.6 } else { 1.0 };
        if rng.random_bool(p_hit as f64) {
            hits.write(DirectHit { target: t, cause: Cause::Autocannon, top: false, weapon: "25 mm", from: "a Bradley" });
            fx.explosion(p + Vec3::Y * 1.0, 0.3, false);
        } else {
            fx.explosion(geo::ground(p.xz() + Vec2::new(rng.random_range(-6.0..6.0), rng.random_range(-6.0..6.0))) + Vec3::Y * 0.3, 0.25, true);
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
