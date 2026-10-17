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
        let rear = if self.depth > 0.0 { ((p.z - lay.jam_z) / self.depth).clamp(0.0, 1.0) * 0.9 } else { 0.0 };
        let local: f32 = self.active.iter().filter(|(_, on)| *on).map(|(j, _)| (1.0 - j.distance(p.xz()) / radius).max(0.0) * 0.8).sum();
        (rear + local).min(1.0)
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
