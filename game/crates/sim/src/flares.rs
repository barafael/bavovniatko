//! Light over the field at night (design/hostomel.md, phase 6): the hand flares the attackers send up as they go in
//! and the defenders' posts fire when they are rushed in the dark, and the illumination rounds the player calls from
//! the guns (L). A flare is a light in the air for as long as it burns, falling as it goes; under it men see as far as
//! by day, whichever side they are on (`fight::Sight::at`). The picture lights it (`bav_look::fx`).
//!
//! Only a fight whose file gives its posts flares, or a day with illumination rounds in its support, has any of this:
//! a system more would change another day's Record.

use crate::fight::{FightRng, PulledOut, Sight};
use crate::fire::{Post, Strike, StrikeMission};
use crate::geo;
use crate::units::{Radio, SState, Soldier};
use bevy::prelude::*;
use rand::Rng;

/// What is burning up there.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FlareKind {
    /// A hand-held illumination rocket or a flare pistol's star: up 150 m, fifteen seconds or so, a hundred and fifty
    /// metres of ground lit round the point under it.
    Hand,
    /// A 152 mm illumination round's star under its parachute: opened at 450 m, it comes down at 5 m/s and burns for
    /// about a minute over a kilometre of ground.
    Shell,
}

impl FlareKind {
    /// Where it opens, above the ground.
    pub fn height(self) -> f32 {
        match self {
            FlareKind::Hand => 150.0,
            FlareKind::Shell => 450.0,
        }
    }
    /// How long it burns (s).
    pub fn secs(self) -> f32 {
        match self {
            FlareKind::Hand => 15.0,
            FlareKind::Shell => 60.0,
        }
    }
    /// How fast it comes down (m/s).
    pub fn fall(self) -> f32 {
        match self {
            FlareKind::Hand => 4.0,
            FlareKind::Shell => 5.0,
        }
    }
    /// The ground it lights enough to see by: so far round the point under it (m).
    pub fn reach(self) -> f32 {
        match self {
            FlareKind::Hand => 150.0,
            FlareKind::Shell => 500.0,
        }
    }
}

/// A flare burning in the air: what it is, and when it goes out (the battle's clock, s). Its `Transform` is where
/// it is.
#[derive(Component, Debug)]
pub struct Flare {
    pub kind: FlareKind,
    pub out: f64,
}

/// A defenders' post's hand flares left, and when it may fire the next.
#[derive(Component)]
pub struct HandFlares {
    pub left: u32,
    pub next: f64,
}

/// The flares' systems, where the day has hand flares (`fight.flares`) or illumination rounds (`Illum` in its support).
pub(crate) fn add(app: &mut App) {
    let illum = crate::data::sim_data().support.iter().any(|s| matches!(s.what, crate::allot::SupportKind::Illum(_)));
    let hand = crate::fight::spec().is_some_and(|f| f.flares > 0);
    if illum {
        app.add_systems(Update, call_illum.run_if(crate::running).in_set(crate::Step::Fire).before(crate::fire::FireSystems));
    }
    if hand || illum {
        app.add_systems(Update, (burn, see_lit).chain().run_if(crate::running).in_set(crate::Step::Units).after(crate::fight::see).before(crate::fight::supply_posts));
    }
    if hand {
        app.add_systems(Update, post_flares.run_if(crate::running).in_set(crate::Step::Units).after(crate::fight::return_fire).before(crate::fight::pull_out));
    }
}

/// The ground the flares light, for the fight's sight lines.
fn see_lit(mut sight: ResMut<Sight>, flares: Query<(&Flare, &Transform)>) {
    sight.lit = flares.iter().map(|(f, t)| (t.translation.xz(), f.kind.reach())).collect();
}

/// A flare of this kind lights at `at` (in the air) now.
pub fn light(commands: &mut Commands, kind: FlareKind, at: Vec3, now: f64) {
    commands.spawn((Flare { kind, out: now + kind.secs() as f64 }, Transform::from_translation(at)));
}

/// The flares come down as they burn, and go out.
pub(crate) fn burn(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &Flare, &mut Transform)>) {
    let (now, dt) = (time.elapsed_secs_f64(), crate::sim_dt(&time));
    for (e, f, mut tf) in &mut q {
        if now >= f.out {
            commands.entity(e).despawn();
            continue;
        }
        tf.translation.y -= f.kind.fall() * dt;
    }
}

/// L: an illumination round over the ground under the cursor, from the guns' stock of them.
pub(crate) fn call_illum(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<bevy::window::PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform), With<crate::player::MainCamera>>,
    mut fc: ResMut<crate::fire::FireControl>,
    mut radio: ResMut<Radio>,
) {
    if !keys.just_pressed(KeyCode::KeyL) || fc.start.illum == 0 {
        return;
    }
    let now = time.elapsed_secs_f64();
    if fc.illum == 0 {
        radio.say2(now, "Освітлювальних більше нема.", "No more illumination.");
        return;
    }
    let Some(hit) = crate::player::cursor_ground(&window, cam.0, cam.1) else { return };
    let target = hit.xz();
    fc.illum -= 1;
    Strike::Illum.call(&mut commands, target, 20.0, now);
    let (lat, lon) = geo::latlon(target);
    radio.say2(now, format!("{{guns}}, я {{player}}. Освітлювальний, {lat:.4} {lon:.4}. Підсвітіть нам."), format!("{{guns}}, this is {{player}}. Illumination, {lat:.4} {lon:.4}. Light it up for us."));
}

/// The defenders' posts, rushed in the dark, send up a hand flare over the men coming at them: one at a time, a
/// minute or two apart, while they have any.
#[allow(clippy::type_complexity)]
pub(crate) fn post_flares(
    mut commands: Commands,
    time: Res<Time>,
    sight: Res<Sight>,
    mut rng: ResMut<FightRng>,
    mut radio: ResMut<Radio>,
    soldiers: Query<(&Soldier, &Transform)>,
    mut posts: Query<(&Post, &mut HandFlares), Without<PulledOut>>,
) {
    // Only when the dark hides them.
    if sight.dark > 0.8 {
        return;
    }
    let now = time.elapsed_secs_f64();
    let attacker = crate::diorama::diorama().attacker();
    for (post, mut fl) in &mut posts {
        if post.strength == 0 || fl.left == 0 || now < fl.next {
            continue;
        }
        let near = soldiers
            .iter()
            .filter(|(s, _)| s.side == attacker && !matches!(s.state, SState::Wounded | SState::DugIn))
            .map(|(_, t)| t.translation.xz())
            .filter(|p| p.distance(post.pos) < 200.0)
            .min_by(|a, b| a.distance(post.pos).total_cmp(&b.distance(post.pos)));
        let Some(p) = near else { continue };
        fl.left -= 1;
        fl.next = now + rng.0.random_range(60.0..150.0);
        let over = post.pos.lerp(p, rng.0.random_range(0.6..1.1));
        light(&mut commands, FlareKind::Hand, geo::ground(over) + Vec3::Y * FlareKind::Hand.height(), now);
        // (Heard on our net when it is theirs, over our men.)
        if post.side == crate::units::Side::Ru {
            radio.say2(now, "Ракета! Нас підсвічують.", "Flare! They're lighting us up.");
        }
    }
}
