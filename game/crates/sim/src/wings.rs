//! Drones against drones in the air (design/myrnohrad.md, phase 6): their reconnaissance wings as aircraft on a path,
//! and our interceptors against them.
//!
//! - **The wings.** An Orlan-10 or a ZALA flies a path over our side at height, and its camera spots what passes under
//!   it as the staff's strip did (a post one time in three, a vehicle four times in five, less in fog): into the kill
//!   zone's picture, which their FPV crews hunt from, and in Command into the staff's `Spotted` too. In the Record the
//!   day's file flies its passes; in Command the staff sends its own (`staff::orlan`). Where a day has no `wings` the
//!   staff's Orlan is still the strip it was.
//! - **Our interceptors.** Interceptor drones wore the Orlans down from 2024 and by March 2026 claimed tens of thousands
//!   of Russian drones (01 e7, e9). 9 puts one up from the drone team and the player in its goggles, a faster airframe
//!   than the strike FPV that climbs to the wing; it brings the wing down if it passes within a few metres of it. Where
//!   the day has a radar, one goes up by itself at a wing inside its range and flies a lead pursuit onto it; a near
//!   pass brings the wing down most times (`P_KILL`). Both are blinded inside our own EW domes (`ew::Ew::ours_at`), so
//!   a radar-cued launch pauses them for the climb out (02 §6: EW pauses scheduled for a strike).
//!
//! Nothing of it exists where the day's file has no `wings`: an entity, a resource or a draw more would change an older
//! day's Record. `BAV_INTERCEPT` puts the goggles' interceptor up at game times, flown by itself at the nearest wing.

use crate::fx::{Emitter, EmitterKind, FxQueue};
use crate::geo;
use crate::killzone::{Class, Mark, Prey};
use crate::player::{Fpv, Operator, View};
use crate::sound::{Sfx, Sound};
use crate::units::{Coasting, Part, Radio, Side, Vehicle};
use bav_core::diorama::{Play, play};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use serde::Deserialize;

/// The day's wings and interceptors (its file's `wings`).
#[derive(Deserialize, Debug, Clone)]
pub struct WingsSpec {
    /// The Record's passes, each at its time; in Command the staff sends its own.
    #[serde(default)]
    pub passes: Vec<PassSpec>,
    /// Our interceptors for the day, the goggles' and the radar's alike.
    #[serde(default)]
    pub interceptors: u32,
    /// A radar that cues them (none: only the goggles).
    #[serde(default)]
    pub radar: Option<RadarSpec>,
}

/// One pass of a wing of theirs.
#[derive(Deserialize, Debug, Clone)]
pub struct PassSpec {
    /// When it sets out from the path's first point (game s).
    pub at: f32,
    pub kind: WingKind,
    /// Its way, from beyond the edge over our side and back.
    pub path: Vec<(f32, f32)>,
    /// Its height over the ground (m).
    #[serde(default = "pass_agl")]
    pub agl: f32,
}

fn pass_agl() -> f32 {
    700.0
}

/// A radar that cues our interceptors: one goes up by itself at a wing inside its range.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct RadarSpec {
    pub at: (f32, f32),
    pub range: f32,
}

/// Which wing.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WingKind {
    /// The Orlan-10: the artillery's spotter, 3.1 m across, a petrol engine, about 110 km/h.
    Orlan,
    /// A ZALA 421-16E: smaller, electric, about 85 km/h.
    Zala,
}

impl WingKind {
    /// Cruising speed (m/s).
    pub fn speed(self) -> f32 {
        match self {
            WingKind::Orlan => 30.0,
            WingKind::Zala => 24.0,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            WingKind::Orlan => "Orlan-10",
            WingKind::Zala => "ZALA",
        }
    }

    pub fn uk(self) -> &'static str {
        match self {
            WingKind::Orlan => "Орлан",
            WingKind::Zala => "Зала",
        }
    }
}

/// The day's wings, if it has them.
pub fn spec() -> Option<&'static WingsSpec> {
    crate::data::sim_data().wings.as_ref()
}

/// Whether the day has their wings as aircraft (and so anything of this module).
pub fn present() -> bool {
    spec().is_some()
}

/// A wing of theirs on its path.
#[derive(Component)]
pub struct Wing {
    pub kind: WingKind,
    path: Vec<Vec2>,
    /// Metres along the path.
    s: f32,
    agl: f32,
    /// What its camera has already had its chance at on this pass.
    looked: Vec<Mark>,
    /// Over the box, and the net has said so.
    over: bool,
}

impl Wing {
    /// Which way and how fast it flies now.
    pub fn velocity(&self) -> Vec3 {
        let (_, dir) = crate::killzone::along(&self.path, self.s);
        Vec3::new(dir.x, 0.0, dir.y) * self.kind.speed()
    }
}

/// A wing brought down, falling.
#[derive(Component)]
pub struct Downed {
    pub vel: Vec3,
}

/// One of our interceptors flying itself onto a wing, cued by the radar.
#[derive(Component)]
pub struct Interceptor {
    pub target: Entity,
    vel: Vec3,
    battery: f32,
    /// Seconds without its control link (inside one of our own domes).
    lost_for: f32,
}

/// Our interceptors left for the day.
#[derive(Resource)]
pub struct Interceptors {
    pub left: u32,
}

/// What the day's wings and interceptors did, for the log and the bands.
#[derive(Resource, Default, Debug)]
pub struct WingsTally {
    pub passes: u32,
    /// Things of ours their cameras spotted.
    pub spotted: u32,
    /// Interceptors up: the radar's, the goggles'.
    pub up: [u32; 2],
    /// Wings brought down: by the radar's, by the goggles'.
    pub downed: [u32; 2],
    /// The radar's that passed and missed.
    pub missed: u32,
    /// Interceptors lost to our own domes.
    pub blinded: u32,
}

/// The models the wings and the interceptors wear, as the look loads them.
#[derive(Resource)]
pub struct WingModels {
    pub orlan: Vec<Part>,
    pub zala: Vec<Part>,
    pub fpv: Vec<Part>,
}

/// The wings' own stream: their draws never shift the rest of the day's.
#[derive(Resource)]
struct WingRng(SmallRng);

/// Half the width of the ground a wing's camera sweeps (m): the staff's strip was 600 m wide.
const SWATH: f32 = 300.0;
/// How often the camera looks (s).
const LOOK_EVERY: f64 = 1.0;
/// An interceptor's speed (m/s), its battery (s), and the miss distance inside which its charge goes off (m).
const INTERCEPTOR_SPEED: f32 = 50.0;
const INTERCEPTOR_BATTERY: f32 = 240.0;
const PROXIMITY: f32 = 6.0;
/// The chance a radar-cued interceptor's pass brings the wing down.
const P_KILL: f64 = 0.6;
/// How near the goggles' interceptor must pass (m): the player has flown it there.
pub const GOGGLES_PASS: f32 = 5.0;
/// How long a radar-cued launch pauses our domes (s).
const PAUSE: f64 = 45.0;

/// The wings' systems, for what must run after them.
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct WingsSystems;

pub struct WingsPlugin;

impl Plugin for WingsPlugin {
    fn build(&self, app: &mut App) {
        let Some(spec) = spec() else { return };
        app.insert_resource(WingRng(SmallRng::seed_from_u64(crate::dev::seeded(1007))))
            .insert_resource(Interceptors { left: spec.interceptors })
            .init_resource::<WingsTally>()
            // After the team's FPV (which may have hit one), the domes and the robots; before their FPVs, which hunt
            // from the picture the wings fill.
            .add_systems(
                Update,
                (goggles, passes, fly_wings, eyes, cue, intercept, fall)
                    .chain()
                    .run_if(crate::running)
                    .in_set(crate::Step::Script)
                    .in_set(WingsSystems)
                    .after(crate::ew::ew_update)
                    .after(crate::ew::DomeSystems)
                    .after(crate::robots::RobotSystems)
                    .after(crate::director::DirectorSystems)
                    .before(crate::killzone::KillzoneSystems),
            )
            .add_systems(Update, log_wings.run_if(crate::running).in_set(crate::Step::Effects));
    }
}

/// Put a wing up at the start of its path.
pub fn launch(commands: &mut Commands, models: Option<&WingModels>, kind: WingKind, path: Vec<Vec2>, agl: f32, now: f64) {
    if path.len() < 2 {
        return;
    }
    let at = crate::killzone::ground(path[0]) + Vec3::Y * agl;
    let dir = path[1] - path[0];
    let km = path.windows(2).map(|w| w[0].distance(w[1])).sum::<f32>() / 1000.0;
    let mut e = commands.spawn((
        Wing { kind, path, s: 0.0, agl, looked: Vec::new(), over: false },
        // Drawn larger than life, as their FPVs are: at its height it is a speck otherwise.
        Transform::from_translation(at).looking_to(Vec3::new(dir.x, 0.0, dir.y).normalize_or(Vec3::NEG_Z), Vec3::Y).with_scale(Vec3::splat(3.0)),
        bevy::light::NotShadowCaster,
    ));
    if let Some(m) = models {
        let parts = match kind {
            WingKind::Orlan => &m.orlan,
            WingKind::Zala => &m.zala,
        };
        e.insert(crate::units::skin(parts));
        crate::units::dress(&mut e, parts);
    }
    info!("wings: a {} up, {km:.1} km of path at {agl:.0} m, at {now:.0} s", kind.name());
}

/// The goggles' FPV, an interceptor or a strike FPV, that passes close enough to a wing brings it down: an
/// interceptor's few metres, a strike FPV's less.
#[allow(clippy::too_many_arguments)]
fn goggles(
    mut commands: Commands,
    time: Res<Time>,
    mut fpv: ResMut<Fpv>,
    mut tally: ResMut<WingsTally>,
    (mut fx, mut sound, mut log, mut radio): (ResMut<FxQueue>, Sound, ResMut<crate::hits::HitLog>, ResMut<Radio>),
    wings: Query<(Entity, &Wing, &Transform), Without<Downed>>,
) {
    if !fpv.flying {
        return;
    }
    let now = time.elapsed_secs_f64();
    let p = fpv.pos;
    let pass = if fpv.airframe == crate::player::Airframe::Interceptor { GOGGLES_PASS } else { 3.0 };
    let Some((e, wing, t)) = wings.iter().filter(|(_, _, t)| t.translation.distance(p) < pass).min_by(|a, b| a.2.translation.distance(p).total_cmp(&b.2.translation.distance(p))) else { return };
    fpv.flying = false;
    fpv.gone = 0.0;
    fx.explosion(p, 0.4, false);
    fx.flash(p, 3.0e6, 40.0, 0.1);
    sound.at(p, Sfx::BlastNear, 1.0);
    tally.downed[1] += 1;
    bring_down(&mut commands, e, wing, t.translation, &mut log, &mut radio, now, "your drone team");
}

/// The Record's passes, each at its time.
fn passes(mut commands: Commands, time: Res<Time>, script: Res<crate::director::Script>, models: Option<Res<WingModels>>, mut done: Local<usize>) {
    let Some(spec) = spec() else { return };
    if play() != Play::Record {
        return;
    }
    while *done < spec.passes.len() && script.t >= spec.passes[*done].at {
        let p = &spec.passes[*done];
        *done += 1;
        launch(&mut commands, models.as_deref(), p.kind, p.path.iter().map(|&(x, z)| Vec2::new(x, z)).collect(), p.agl, time.elapsed_secs_f64());
    }
}

/// The wings fly their paths at their height; the net says so when one comes over the box, and one at the end of its
/// path has gone home.
fn fly_wings(mut commands: Commands, time: Res<Time>, mut radio: ResMut<Radio>, mut tally: ResMut<WingsTally>, mut wings: Query<(Entity, &mut Wing, &mut Transform), Without<Downed>>) {
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    for (e, mut w, mut tf) in &mut wings {
        let len: f32 = w.path.windows(2).map(|p| p[0].distance(p[1])).sum();
        w.s += w.kind.speed() * dt;
        if w.s >= len {
            info!("wings: the {} gone home, at {now:.0} s", w.kind.name());
            commands.entity(e).despawn();
            continue;
        }
        let (p, dir) = crate::killzone::along(&w.path, w.s);
        tf.translation = crate::killzone::ground(p) + Vec3::Y * w.agl;
        // Banked into its turns a little, by how its heading changes over the next 60 m.
        let ahead = crate::killzone::along(&w.path, w.s + 60.0).1;
        let bank = dir.perp_dot(ahead).clamp(-0.5, 0.5);
        tf.rotation = Transform::IDENTITY.looking_to(Vec3::new(dir.x, 0.0, dir.y), Vec3::Y).rotation * Quat::from_rotation_z(-bank);
        if !w.over && geo::in_bounds(p) {
            w.over = true;
            tally.passes += 1;
            let (lat, lon) = geo::latlon(p);
            info!("wings: the {} over the box at {:.0},{:.0}, at {now:.0} s", w.kind.name(), p.x, p.y);
            radio.say2(now, format!("{} над нами, {lat:.3} {lon:.3}. Ховайтесь.", w.kind.uk()), format!("{} over us, near {lat:.3} {lon:.3}. Keep under cover.", w.kind.name()));
        }
    }
}

/// Every second each wing's camera looks at the ground it passes over: each thing of ours under it gets one chance a
/// pass, as under the staff's strip (a post one time in three, a vehicle four times in five, our domes' masts and
/// our drone team's antennas between), less in fog as its camera sees through it from a kilometre up. What it finds
/// goes into the kill zone's picture and, in Command, the staff's `Spotted`.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn eyes(
    time: Res<Time>,
    mut rng: ResMut<WingRng>,
    mut tally: ResMut<WingsTally>,
    seeing: Option<Res<crate::fight::Seeing>>,
    smoke: Res<crate::fx::Smoke>,
    (view, fpv, op): (Res<View>, Res<Fpv>, Res<Operator>),
    mut picture: Option<ResMut<crate::killzone::Picture>>,
    mut spotted: Option<ResMut<crate::staff::Spotted>>,
    mut wings: Query<(&mut Wing, &Transform), Without<Downed>>,
    posts: Query<(Entity, &crate::fire::Post)>,
    vehicles: Query<(Entity, &Vehicle, &Transform), (Without<Coasting>, Without<Wing>)>,
    prey: Query<(Entity, &Prey, &Transform, Has<crate::ew::DomeOff>), Without<Wing>>,
    mut next: Local<f64>,
) {
    let now = time.elapsed_secs_f64();
    if now < *next {
        return;
    }
    *next = now + LOOK_EVERY;
    let fog = seeing.as_ref().map_or(1.0, |s| (s.through(crate::fight::Eye::Man) / 1000.0).min(1.0) as f64);
    // What of ours there is to see, and how readily: west to east, an order that does not hang on entity ids.
    let mut things: Vec<(Mark, Class, Vec2, f64, Option<(Entity, crate::staff::What)>)> = Vec::new();
    for (e, p) in &posts {
        if p.side == Side::Ua && p.strength > 0 {
            things.push((Mark::Thing(e), Class::Men, p.pos, 0.35, Some((e, crate::staff::What::Post))));
        }
    }
    for (e, v, tf) in &vehicles {
        if v.side == Side::Ua && v.state.alive() {
            let class = if v.kind == crate::units::VKind::Ugv { Class::Robot } else { Class::Vehicle };
            things.push((Mark::Thing(e), class, tf.translation.xz(), 0.8, Some((e, crate::staff::What::Vehicle))));
        }
    }
    for (e, p, tf, off) in &prey {
        match p.0 {
            // A dome switched off is a mast like any other, its emitter quiet.
            Class::Ew if !off => things.push((Mark::Thing(e), Class::Ew, tf.translation.xz(), 0.5, None)),
            Class::Vehicle | Class::Robot => things.push((Mark::Thing(e), p.0, tf.translation.xz(), 0.8, None)),
            _ => {}
        }
    }
    if (view.feed_down <= 0.0 || fpv.flying) && now >= op.moving_until {
        things.push((Mark::Team, Class::DroneTeam, op.pos, 0.25, None));
    }
    things.sort_by(|a, b| a.2.x.total_cmp(&b.2.x).then(a.2.y.total_cmp(&b.2.y)));
    let mut wings: Vec<_> = wings.iter_mut().collect();
    wings.sort_by(|a, b| a.1.translation.x.total_cmp(&b.1.translation.x).then(a.1.translation.z.total_cmp(&b.1.translation.z)));
    let rng = &mut rng.0;
    for (mut w, tf) in wings {
        let under = tf.translation.xz();
        if !geo::in_bounds(under) {
            continue;
        }
        let kind = w.kind;
        for &(mark, class, at, p, staff) in &things {
            if at.distance(under) > SWATH || w.looked.contains(&mark) || smoke.covers(at, now) {
                continue;
            }
            w.looked.push(mark);
            if !rng.random_bool((p * fog).clamp(0.0, 1.0)) {
                continue;
            }
            tally.spotted += 1;
            info!("wings: the {} spots the {} at {:.0},{:.0}, at {now:.0} s", kind.name(), class.name(), at.x, at.y);
            if let Some(pic) = picture.as_deref_mut() {
                let jitter = Vec2::new(rng.random_range(-25.0..25.0), rng.random_range(-25.0..25.0));
                pic.see(mark, class, at + jitter, now);
            }
            if let (Some(sp), Some((e, what))) = (spotted.as_deref_mut(), staff) {
                sp.see(e, at, now, crate::staff::How::Orlan, what, rng);
            }
        }
    }
}

/// The radar cues an interceptor at the nearest wing inside its range that none is already after, from the drone
/// team, while there are any; our domes pause for its climb out.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn cue(
    mut commands: Commands,
    time: Res<Time>,
    mut radio: ResMut<Radio>,
    mut tally: ResMut<WingsTally>,
    mut stock: ResMut<Interceptors>,
    op: Res<Operator>,
    models: Option<Res<WingModels>>,
    wings: Query<(Entity, &Wing, &Transform), Without<Downed>>,
    up: Query<&Interceptor>,
    domes: Query<(Entity, &crate::ew::Dome), (Without<crate::ew::DomeDown>, Without<crate::ew::DomeOff>)>,
) {
    let Some(radar) = spec().and_then(|s| s.radar) else { return };
    let now = time.elapsed_secs_f64();
    if stock.left == 0 || now < op.moving_until {
        return;
    }
    let at = Vec2::new(radar.at.0, radar.at.1);
    let Some((target, w, tf)) = wings
        .iter()
        .filter(|(e, _, t)| t.translation.xz().distance(at) < radar.range && up.iter().all(|i| i.target != *e))
        .min_by(|a, b| a.2.translation.xz().distance(at).total_cmp(&b.2.translation.xz().distance(at)).then(a.2.translation.x.total_cmp(&b.2.translation.x)))
    else {
        return;
    };
    stock.left -= 1;
    tally.up[0] += 1;
    let from = geo::ground(op.pos) + Vec3::Y * 2.0;
    let mut e = commands.spawn((Interceptor { target, vel: Vec3::Y * 10.0, battery: INTERCEPTOR_BATTERY, lost_for: 0.0 }, Transform::from_translation(from).with_scale(Vec3::splat(3.0)), bevy::light::NotShadowCaster));
    if let Some(m) = models.as_deref() {
        e.insert(crate::units::skin(&m.fpv));
        crate::units::dress(&mut e, &m.fpv);
    }
    let mut paused: Vec<(usize, Entity)> = domes.iter().map(|(e, d)| (d.idx, e)).collect();
    paused.sort_by_key(|d| d.0);
    for (_, d) in &paused {
        crate::ew::pause(&mut commands, *d, now + PAUSE);
    }
    info!("wings: the radar cues an interceptor at the {} {:.1} km off, {} left, {} domes paused, at {now:.0} s", w.kind.name(), tf.translation.xz().distance(op.pos) / 1000.0, stock.left, paused.len());
    radio.say2(
        now,
        format!("Радар веде {}а. Перехоплювач пішов{}", w.kind.uk(), if paused.is_empty() { "." } else { ", РЕБ на паузі." }),
        format!("The radar has the {}. Interceptor away{}", w.kind.name(), if paused.is_empty() { "." } else { ", the EW paused." }),
    );
}

/// The radar's interceptors fly a lead pursuit onto their wing, faster than it, and go off when they pass within a few
/// metres: most times it comes down. Inside one of our own domes an interceptor loses its link and falls; one whose
/// wing has gone takes another near it, or comes down.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn intercept(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<WingRng>,
    mut tally: ResMut<WingsTally>,
    (mut radio, mut log, mut fx, mut sound): (ResMut<Radio>, ResMut<crate::hits::HitLog>, ResMut<FxQueue>, Sound),
    ew: Res<crate::ew::Ew>,
    mut ours: Query<(Entity, &mut Interceptor, &mut Transform), Without<Wing>>,
    wings: Query<(Entity, &Wing, &Transform), (Without<Downed>, Without<Interceptor>)>,
) {
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    let mut gone: Vec<Entity> = Vec::new();
    let mut ours: Vec<_> = ours.iter_mut().collect();
    ours.sort_by(|a, b| a.2.translation.x.total_cmp(&b.2.translation.x).then(a.2.translation.z.total_cmp(&b.2.translation.z)));
    for (e, mut i, mut tf) in ours {
        let mut p = tf.translation;
        i.battery -= dt;
        let was = i.lost_for;
        i.lost_for = if ew.ours_at(p) > 0.5 { i.lost_for + dt } else { 0.0 };
        if was < 0.8 && i.lost_for >= 0.8 {
            tally.blinded += 1;
            info!("wings: our own dome takes an interceptor's link, at {now:.0} s");
            radio.say2(now, "Наш РЕБ осліпив перехоплювач!", "Our own EW's blinded the interceptor!");
        }
        if wings.get(i.target).is_err() || gone.contains(&i.target) {
            // Its wing is down or gone home: the nearest other within a kilometre and a half, if any.
            let by = p.xz();
            match wings.iter().filter(|(w, _, t)| !gone.contains(w) && t.translation.xz().distance(by) < 1500.0).min_by(|a, b| a.2.translation.distance(p).total_cmp(&b.2.translation.distance(p))) {
                Some((w, _, _)) => i.target = w,
                None => i.battery = i.battery.min(0.0),
            }
        }
        let flying = i.battery > 0.0 && i.lost_for < 0.8;
        match wings.get(i.target) {
            Ok((w, wing, wt)) if flying => {
                let to = wt.translation - p;
                // Lead: where the wing will be when it gets there.
                let t_go = to.length() / INTERCEPTOR_SPEED;
                let aim = wt.translation + wing.velocity() * t_go.min(20.0) - p;
                i.vel = i.vel.lerp(aim.normalize_or(Vec3::Y) * INTERCEPTOR_SPEED, (3.0 * dt).min(1.0));
                if to.length() < PROXIMITY.max(INTERCEPTOR_SPEED * dt) {
                    commands.entity(e).despawn();
                    fx.explosion(p, 0.35, false);
                    sound.at(p, Sfx::BlastNear, 0.8);
                    if rng.0.random_bool(P_KILL) {
                        gone.push(w);
                        tally.downed[0] += 1;
                        bring_down(&mut commands, w, wing, wt.translation, &mut log, &mut radio, now, "the radar's interceptor");
                    } else {
                        tally.missed += 1;
                        info!("wings: an interceptor misses the {}, at {now:.0} s", wing.kind.name());
                        radio.say2(now, format!("Перехоплювач не дістав {}а.", wing.kind.uk()), format!("The interceptor missed the {}.", wing.kind.name()));
                    }
                    continue;
                }
            }
            _ => {
                // No link, no battery or nothing to go for: it falls.
                i.vel.y -= 9.81 * dt;
                i.vel *= 1.0 - 0.3 * dt;
            }
        }
        p += i.vel * dt;
        if p.y <= crate::killzone::ground(p.xz()).y + 0.3 {
            commands.entity(e).despawn();
            fx.explosion(p, 0.3, true);
            info!("wings: an interceptor down at {:.0},{:.0}, at {now:.0} s", p.x, p.z);
            continue;
        }
        tf.translation = p;
        if i.vel.length_squared() > 0.01 {
            tf.look_to(i.vel.normalize(), Vec3::Y);
        }
    }
}

/// A wing hit: it falls trailing smoke, and the hit feed and the net say so.
#[allow(clippy::too_many_arguments)]
pub fn bring_down(commands: &mut Commands, e: Entity, wing: &Wing, at: Vec3, log: &mut crate::hits::HitLog, radio: &mut Radio, now: f64, by: &'static str) {
    commands.entity(e).insert((Downed { vel: wing.velocity() * 0.8 }, Emitter::new(EmitterKind::EngineSmoke, 8.0, 40.0, Vec3::ZERO)));
    info!("wings: the {} brought down by {by} at {:.0},{:.0}, at {now:.0} s", wing.kind.name(), at.x, at.z);
    log.note(now, "interceptor", by, wing.kind.name(), Side::Ru, "brought down");
    radio.say2(now, format!("{} збито! Падає.", wing.kind.uk()), format!("The {}'s down! It's falling.", wing.kind.name()));
}

/// A wing brought down falls, turning over, and breaks up where it meets the ground.
fn fall(mut commands: Commands, time: Res<Time>, mut fx: ResMut<FxQueue>, mut q: Query<(Entity, &mut Downed, &mut Transform)>) {
    let dt = crate::sim_dt(&time);
    for (e, mut d, mut tf) in &mut q {
        d.vel.y -= 9.81 * dt;
        let decay = 1.0 - 0.2 * dt;
        d.vel.x *= decay;
        d.vel.z *= decay;
        let p = tf.translation + d.vel * dt;
        let floor = crate::killzone::ground(p.xz()).y;
        if p.y <= floor + 0.5 {
            fx.explosion(Vec3::new(p.x, floor, p.z), 0.4, true);
            commands.entity(e).despawn();
            continue;
        }
        tf.translation = p;
        tf.rotate_local_z(2.5 * dt);
    }
}

/// For the bands (`BAV_LOG_STATS`): the wings' count once a game minute.
fn log_wings(script: Res<crate::director::Script>, tally: Res<WingsTally>, stock: Res<Interceptors>, mut last: Local<Option<u32>>) {
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
        "wings: {} passes={} spotted={} interceptors={} up={}/{} downed={}/{} missed={} blinded={}",
        crate::director::clock_text(script.t),
        t.passes,
        t.spotted,
        stock.left,
        t.up[0],
        t.up[1],
        t.downed[0],
        t.downed[1],
        t.missed,
        t.blinded
    );
}

