//! Their electronic warfare as a field (db/research/11 §5, §9): platoon jammers at their line, each switched on
//! and off on its own schedule, and the heavier emitters further back felt as a ramp past the diorama's jam line.
//! In 2023 it took the control link, not the video. The jammers are things on the ground: a 155 mm mission within
//! 25 m of one knocks it out. And it has a tell: south of Mala Tokmachka the GNSS jamming lifts about a minute
//! before each Ka-52 run, so their own helicopters can navigate.

use crate::geo;
use crate::units::Radio;
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

/// The diorama's EW (its file's `ew`).
#[derive(serde::Deserialize, Debug, Clone)]
pub struct EwSpec {
    /// The platoon jammers, (x, z).
    pub jammers: Vec<(f32, f32)>,
    /// How far each reaches (m).
    pub radius: f32,
    /// Over how many metres past the jam line the rear emitters' jamming builds to its full strength.
    pub depth: f32,
    /// Whether GNSS jamming lifts before the Ka-52s come.
    #[serde(default)]
    pub ka52_tell: bool,
}

/// One jammer on the ground, while it stands: a shell within 25 m takes it off the field (`knock_out`).
#[derive(Component)]
pub struct Jammer {
    /// When it next switches on or off.
    next: f64,
}

/// A jammer that is jamming now.
#[derive(Component)]
pub struct Jamming;

/// The field, as the drones feel it this frame.
#[derive(Resource, Default)]
pub struct Ew {
    /// Each jammer's position and whether it is jamming now.
    pub active: Vec<(Vec2, bool)>,
    pub radius: f32,
    pub depth: f32,
    /// Windows (game s) when GNSS jamming is lifted.
    pub lifts: Vec<(f64, f64)>,
    told: usize,
    /// Our domes that stand and jam now: where (on the ground) and how far (`ours_at`).
    pub ours: Vec<(Vec3, f32)>,
}

impl Ew {
    /// Jamming at a point, 0..1: the rear ramp past the jam line, and each active jammer's falling off with distance.
    pub fn jam_at(&self, p: Vec3) -> f32 {
        self.jam_within(p, self.radius)
    }

    /// Jamming on the net at a point, 0..1 (Command's orders): the same jammers, reaching twice as far on the radios'
    /// band as on the drones' links.
    pub fn radio_jam_at(&self, p: Vec3) -> f32 {
        self.jam_within(p, self.radius * 2.0)
    }

    fn jam_within(&self, p: Vec3, radius: f32) -> f32 {
        let lay = geo::layout();
        // Past the jam line, or, in a pocket drawn as outlines, into their ground.
        let pocket = bav_core::field::pocket();
        let rear = match lay.jam_z {
            Some(z) if self.depth > 0.0 => ((p.z - z) / self.depth).clamp(0.0, 1.0) * 0.9,
            None if self.depth > 0.0 && pocket.drawn => (pocket.depth_in_theirs(p.xz()) / self.depth).clamp(0.0, 1.0) * 0.9,
            _ => 0.0,
        };
        let local: f32 = self.active.iter().filter(|(_, on)| *on).map(|(j, _)| (1.0 - j.distance(p.xz()) / radius).max(0.0) * 0.8).sum();
        (rear + local).min(1.0)
    }

    /// Our own domes' jamming on our FPVs' control band at a point, 0..1 (02 §6: friendly jamming blinds our own
    /// drones): all of it over the inner three quarters of a dome, none at its edge. The reconnaissance drone's link
    /// is on another band and the domes are set clear of it.
    pub fn ours_at(&self, p: Vec3) -> f32 {
        self.ours.iter().map(|(c, r)| ((1.0 - c.distance(p) / r) * 2.0).clamp(0.0, 1.0)).fold(0.0, f32::max)
    }

    /// Whether GNSS jamming is lifted now (the Ka-52 tell).
    pub fn gnss_lifted(&self, now: f64) -> bool {
        self.lifts.iter().any(|(a, b)| (*a..*b).contains(&now))
    }
}

/// The jammers' own stream: their schedules never shift the rest of the day's draws.
#[derive(Resource)]
pub struct EwRng(SmallRng);

pub struct EwPlugin;

impl Plugin for EwPlugin {
    fn build(&self, app: &mut App) {
        let spec = crate::data::sim_data().ew.clone();
        let lifts = match &spec {
            Some(s) if s.ka52_tell => crate::data::sim_data()
                .script
                .iter()
                .filter(|(_, a)| matches!(a, crate::director::Act::Helicopters(_)))
                .map(|(t, _)| (*t as f64 - 60.0, *t as f64 + 20.0))
                .collect(),
            _ => Vec::new(),
        };
        app.insert_resource(Ew { radius: spec.as_ref().map_or(1.0, |s| s.radius), depth: spec.as_ref().map_or(0.0, |s| s.depth), lifts, ..default() })
            .insert_resource(EwRng(SmallRng::seed_from_u64(crate::dev::seeded(1702))))
            .add_systems(Startup, spawn_jammers.in_set(crate::schedule::Setup::Field));
        // Our domes, only where the day has them: an entity more anywhere else would be in that day's Record.
        if crate::killzone::spec().is_some_and(|k| !k.domes.is_empty()) {
            app.add_systems(Startup, spawn_domes.in_set(crate::schedule::Setup::Field))
                // After the jammers and the robots (whose links they leave alone), before the wings and their FPVs.
                .add_systems(Update, (switch_domes, our_domes).chain().run_if(crate::running).in_set(crate::Step::Script).in_set(DomeSystems).after(ew_update).after(crate::robots::RobotSystems));
        }
    }
}

/// One of our EW domes (design/myrnohrad.md, phase 3: the field turned to our side): a jammer at a post that
/// blankets the radio FPVs' control band 200–300 m round it (02 e8). It breaks most radio links that cross into it and
/// none of the fibre ones; it is an emitter, so their direction-finders locate it, and a fibre FPV can knock it out
/// (`killzone`).
#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub struct DomeSpec {
    /// The post it stands at (the layout's `ua_positions`).
    pub post: usize,
    pub radius: f32,
}

/// A dome of ours, while it stands.
#[derive(Component)]
pub struct Dome {
    pub idx: usize,
    pub radius: f32,
}

/// A dome their FPVs have knocked out: the mast is down and it jams nothing.
#[derive(Component)]
pub struct DomeDown;

/// The chance a dome breaks the link of a radio FPV that crosses into it (frequency hopping gets some through).
pub const DOME_BREAKS: f64 = 0.85;

/// A dome we have switched off (J, or a pause for a strike of ours): it jams nothing, theirs or ours, and emits
/// nothing for their direction-finders to find.
#[derive(Component)]
pub struct DomeOff;

/// A dome paused for a strike of ours: on again at this game time.
#[derive(Component)]
pub struct Pause(pub f64);

/// Our domes' switching, for what must run after it.
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DomeSystems;

/// Pause a dome for a strike of ours until a game time (02 §6: EW pauses scheduled for a strike).
pub fn pause(commands: &mut Commands, dome: Entity, until: f64) {
    commands.entity(dome).insert((DomeOff, Pause(until)));
}

/// A dome as its switch sees it: which one, whether it is off, and the pause it is off for, if any.
type Switch = (Entity, &'static Dome, Has<DomeOff>, Option<&'static Pause>);

/// Our domes that stand and are switched on.
pub(crate) type DomesOn = (Without<DomeDown>, Without<DomeOff>);

/// J switches our standing domes off for our FPVs to fly through, or on again (02 §6, the friendly-fire toggle);
/// `BAV_DOME="<game s>:off|on,…"` does it at game times. A pause runs out by itself.
fn switch_domes(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    mut radio: ResMut<Radio>,
    domes: Query<Switch, Without<DomeDown>>,
    mut done: Local<usize>,
) {
    let now = time.elapsed_secs_f64();
    let mut domes: Vec<_> = domes.iter().collect();
    domes.sort_by_key(|d| d.1.idx);
    let mut resumed = false;
    for &(e, _, _, pause) in &domes {
        if pause.is_some_and(|p| now >= p.0) {
            commands.entity(e).remove::<(DomeOff, Pause)>();
            resumed = true;
        }
    }
    if resumed {
        info!("ew: our domes on again after the pause, at {now:.0} s");
        radio.say2(now, "РЕБ знову працює.", "The EW's back on.");
    }
    let plan = &crate::dev::dev().dome;
    let mut want: Option<bool> = None;
    while *done < plan.len() && script.t >= plan[*done].0 {
        want = Some(plan[*done].1 != "off");
        *done += 1;
    }
    if keys.just_pressed(KeyCode::KeyJ) {
        // Any on: all off; else all on.
        want = Some(!domes.iter().any(|d| !d.2));
    }
    let Some(on) = want else { return };
    if domes.is_empty() {
        return;
    }
    for &(e, _, _, _) in &domes {
        if on {
            commands.entity(e).remove::<(DomeOff, Pause)>();
        } else {
            commands.entity(e).insert(DomeOff).remove::<Pause>();
        }
    }
    info!("ew: our domes switched {}, at {now:.0} s", if on { "on" } else { "off" });
    if on {
        radio.say2(now, "РЕБ увімкнули. Над нами купол.", "The EW's on. We're under the dome.");
    } else {
        radio.say2(now, "РЕБ вимкнули: наші FPV пройдуть, їхні теж.", "The EW's off: our FPVs can get through, and so can theirs.");
    }
}

/// Where our standing domes jam this frame, for our own drones' links.
fn our_domes(mut ew: ResMut<Ew>, domes: Query<(&Dome, &Transform), DomesOn>) {
    let mut ours: Vec<(usize, Vec3, f32)> = domes.iter().map(|(d, t)| (d.idx, t.translation, d.radius)).collect();
    ours.sort_by_key(|d| d.0);
    ew.ours = ours.into_iter().map(|(_, c, r)| (c, r)).collect();
}

fn spawn_domes(mut commands: Commands) {
    let Some(k) = crate::killzone::spec() else { return };
    let lay = geo::layout();
    for (idx, d) in k.domes.iter().enumerate() {
        // The mast beside the post, not in it.
        let Some(&p) = lay.ua_positions.get(d.post) else { continue };
        let at = p + Vec2::new(9.0, 6.0);
        commands.spawn((Dome { idx, radius: d.radius }, crate::killzone::Prey(crate::killzone::Class::Ew), Transform::from_translation(geo::ground(at))));
    }
}

fn spawn_jammers(mut commands: Commands, mut rng: ResMut<EwRng>) {
    let Some(spec) = &crate::data::sim_data().ew else { return };
    for &(x, z) in &spec.jammers {
        let on = rng.0.random_bool(0.7);
        let next = rng.0.random_range(30.0..120.0);
        let mut e = commands.spawn((Jammer { next }, Transform::from_translation(geo::ground(Vec2::new(x, z)))));
        if on {
            e.insert(Jamming);
        }
    }
}

/// Switch the jammers on and off, keep the field current, and give the tell. (A switch shows in the field the
/// frame after.)
pub fn ew_update(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<EwRng>,
    mut ew: ResMut<Ew>,
    mut radio: ResMut<Radio>,
    mut on: Query<(Entity, &mut Jammer, &Transform), With<Jamming>>,
    mut off: Query<(Entity, &mut Jammer, &Transform), Without<Jamming>>,
) {
    let now = time.elapsed_secs_f64();
    ew.active.clear();
    for (e, mut j, tf) in &mut on {
        if now >= j.next {
            commands.entity(e).remove::<Jamming>();
            j.next = now + rng.0.random_range(20.0..90.0);
        }
        ew.active.push((tf.translation.xz(), true));
    }
    for (e, mut j, tf) in &mut off {
        if now >= j.next {
            commands.entity(e).insert(Jamming);
            j.next = now + rng.0.random_range(60.0..180.0);
        }
        ew.active.push((tf.translation.xz(), false));
    }
    let told = ew.told;
    if let Some(&(a, _)) = ew.lifts.get(told)
        && now >= a
    {
        ew.told += 1;
        info!("ew: GNSS jamming lifted (the Ka-52 tell), at {now:.0} s");
        radio.say2(now, "{player}, супутники раптом чисті. Щось летить.", "{player}, the satellites are suddenly clean. Something's coming.");
    }
}

/// A shell landing within 25 m of a jammer takes it off the field (`gone`: the ones this frame's earlier shells
/// took, whose going is still on its way through the commands).
pub fn knock_out(commands: &mut Commands, jammers: &Query<(Entity, &Transform), With<Jammer>>, at: Vec2, gone: &mut Vec<Entity>) -> Option<Vec2> {
    for (e, tf) in jammers.iter() {
        if !gone.contains(&e) && tf.translation.xz().distance(at) < 25.0 {
            gone.push(e);
            commands.entity(e).remove::<(Jammer, Jamming)>();
            return Some(tf.translation.xz());
        }
    }
    None
}
