//! The infantry fight for a place (design/hostomel.md, phase 3): what the column game did not need. Our posts
//! carry ammunition and run dry; the enemy's men on foot fire back and wear the posts down; a post that is empty, or
//! told to (Space: "Відхід!"), pulls out and its men walk off the field; one overrun where it stands is taken, men
//! and all. Over it, the field's areas (the hangar, the apron, the tower, the base) are whoever's men are in them.
//!
//! Only a diorama whose battle file has a `fight` has any of this: the Record of the others must not change.

use crate::fire::Post;
use crate::geo;
use crate::units::{Models, Palette, Radio, SState, Side, Soldier, Stats};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use serde::Deserialize;

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        let Some(spec) = spec() else { return };
        app.insert_resource(FightRng(SmallRng::seed_from_u64(crate::dev::seeded(2202))))
            .insert_resource(Field { zones: spec.zones.iter().map(|z| Zone { name: z.name.clone(), poly: z.poly.iter().map(|&(x, y)| Vec2::new(x, y)).collect(), held: None }).collect(), ..default() })
            .add_systems(Update, (supply_posts, order_out, assault, return_fire, pull_out, overrun, leave, hold_zones, log_field).chain().run_if(crate::running).in_set(crate::Step::Units));
    }
}

/// The fight's rules for the day, from the battle's file.
#[derive(Deserialize, Debug, Clone)]
pub struct FightSpec {
    /// The volleys each post has before it is dry.
    pub ammo: u32,
    /// Men at a post, for every point of its strength (who are captured, or walk out).
    pub men_per: u32,
    /// Where our men go when they pull out.
    pub out_to: (f32, f32),
    /// How much of the open field's rifle fire tells here, among buildings, aircraft and the apron's cover.
    pub rifles: f32,
    /// How readily the attackers' men who have got where they were going go on to the next post (each, every five
    /// seconds).
    pub assault: f64,
    /// The field's areas.
    pub zones: Vec<ZoneSpec>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ZoneSpec {
    pub name: (String, String),
    pub poly: Vec<(f32, f32)>,
}

pub fn spec() -> Option<&'static FightSpec> {
    crate::data::sim_data().fight.as_ref()
}

#[derive(Resource)]
struct FightRng(SmallRng);

/// A post's ammunition: volleys left. (The posts' own fire, `fire::small_arms`, spends it.)
#[derive(Component)]
pub struct Ammo(pub u32);

/// A post whose men are leaving: it fires no more.
#[derive(Component)]
pub struct PulledOut;

/// One of our men walking off the field.
#[derive(Component)]
pub struct Leaving(pub Vec2);

/// An area of the field, and who holds it.
pub struct Zone {
    pub name: (String, String),
    pub poly: Vec<Vec2>,
    pub held: Option<Side>,
}

/// The field: its areas, and the day's count of our posts and men.
#[derive(Resource, Default)]
pub struct Field {
    pub zones: Vec<Zone>,
    /// Men who walked out, were taken, and posts lost where they stood.
    pub out: u32,
    pub captured: u32,
    pub lost: u32,
    /// When the last post of ours stopped holding (the day's clock, s).
    pub held_until: Option<f32>,
    /// The order to pull out has been given.
    pub ordered_out: bool,
}

/// The defenders' posts get their ammunition once they are on the ground (ours in the morning, the VDV's at dusk).
#[allow(clippy::type_complexity)]
fn supply_posts(mut commands: Commands, posts: Query<(Entity, &Post), (Without<Ammo>, Without<PulledOut>)>) {
    let Some(s) = spec() else { return };
    for (e, p) in &posts {
        if p.side == crate::diorama::diorama().defender() {
            commands.entity(e).insert(Ammo(s.ammo));
        }
    }
}

/// Space: everyone out.
fn order_out(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut field: ResMut<Field>, mut radio: ResMut<Radio>) {
    if field.ordered_out || !keys.just_pressed(KeyCode::Space) || crate::data::sim_data().space_key != crate::fire::SpaceKey::Withdraw {
        return;
    }
    field.ordered_out = true;
    radio.say2(time.elapsed_secs_f64(), "Всім, я {player}. Відхід! Відходимо на Горенку. Поранених забрати.", "All, this is {player}. Pull out! Fall back to Horenka. Bring the wounded.");
}

/// The attackers do not stop at the end of their road: every few seconds a few of the men who have got where they
/// were going (dug in at their goal) get up and go for the nearest of the defenders' posts still holding, to within
/// a stone's throw of it, where they go to ground again round it.
fn assault(time: Res<Time>, mut rng: ResMut<FightRng>, mut tick: Local<f32>, posts: Query<&Post, Without<PulledOut>>, mut soldiers: Query<(&mut Soldier, &Transform), Without<Leaving>>) {
    *tick -= crate::sim_dt(&time);
    if *tick > 0.0 {
        return;
    }
    *tick = 5.0;
    let attacker = crate::diorama::diorama().attacker();
    let holding: Vec<Vec2> = posts.iter().filter(|p| p.side != attacker && p.strength > 0).map(|p| p.pos).collect();
    if holding.is_empty() {
        return;
    }
    for (mut s, tf) in &mut soldiers {
        let p = tf.translation.xz();
        if s.side != attacker || s.state != SState::DugIn || !rng.0.random_bool(spec().map_or(0.0, |f| f.assault)) {
            continue;
        }
        let Some(&post) = holding.iter().min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))) else { continue };
        let d = post.distance(p);
        if d < 40.0 || d > 900.0 {
            continue;
        }
        // Not straight at it: round to a side, about twenty-five metres out.
        let side = Vec2::from_angle(rng.0.random_range(-1.2..1.2)).rotate((p - post).normalize_or(Vec2::Y));
        s.goal = post + side * 25.0;
        s.state = SState::Advance;
    }
}

/// Their men fire back: a man lying or dug in within rifle range of a post he can see, now and then, and the post
/// loses a man to it now and then.
#[allow(clippy::type_complexity)]
fn return_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<FightRng>,
    mut fx: ResMut<crate::fx::FxQueue>,
    mut stats: ResMut<Stats>,
    mut radio: ResMut<Radio>,
    soldiers: Query<(&Soldier, &Transform)>,
    mut posts: Query<(Entity, &mut Post), Without<PulledOut>>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let attacker = crate::diorama::diorama().attacker();
    let rng = &mut rng.0;
    for (e, mut post) in &mut posts {
        if post.side == attacker || post.strength == 0 {
            continue;
        }
        let shooters: Vec<(Vec3, f32)> = soldiers
            .iter()
            .filter(|(s, _)| s.side == attacker && !matches!(s.state, SState::Flee | SState::Wounded))
            .map(|(_, t)| (t.translation, t.translation.xz().distance(post.pos)))
            .filter(|(p, d)| *d < 400.0 && geo::line_of_sight(p.xz(), post.pos))
            .collect();
        if shooters.is_empty() {
            continue;
        }
        // Each man a few rounds a minute that tell; the closer the more.
        let p: f32 = shooters.iter().map(|(_, d)| 0.003 * (1.0 - d / 450.0)).sum::<f32>() * dt;
        if ((now * 10.0) as u64).is_multiple_of(7)
            && let Some((from, _)) = shooters.first()
        {
            fx.tracer(*from + Vec3::Y * 0.6, geo::ground(post.pos) + Vec3::Y, 700.0, attacker);
        }
        if rng.random_bool(p.min(0.9) as f64) {
            post.strength -= 1;
            stats.defender_losses += 1;
            if let Some(f) = post.figures.pop() {
                commands.entity(f).despawn();
            }
            if post.strength == 0 {
                radio.say2(now, format!("Втратили позицію на {:.0}, {:.0}.", post.pos.x, post.pos.y), format!("We've lost the position at {:.0}, {:.0}.", post.pos.x, post.pos.y));
                let _ = e;
            }
        }
    }
}

/// A post out of ammunition, or told to, pulls out: its men get up and go, and it fires no more.
#[allow(clippy::too_many_arguments)]
fn pull_out(
    mut commands: Commands,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    mut rng: ResMut<FightRng>,
    mut field: ResMut<Field>,
    mut radio: ResMut<Radio>,
    models: Res<Models>,
    palette: Res<Palette>,
    mut posts: Query<(Entity, &mut Post, &Ammo), Without<PulledOut>>,
) {
    let Some(s) = spec() else { return };
    let now = time.elapsed_secs_f64();
    let out_to = Vec2::new(s.out_to.0, s.out_to.1);
    for (e, mut post, ammo) in &mut posts {
        if post.side != Side::Ua || post.strength == 0 || !(ammo.0 == 0 || field.ordered_out) {
            continue;
        }
        if ammo.0 == 0 {
            radio.say2(now, "Набої закінчились! Відходимо.", "Out of ammunition! We're pulling out.");
        }
        let men = (post.strength * s.men_per).min(12);
        for k in 0..men {
            let at = post.pos + Vec2::new(k as f32 * 1.5 - 8.0, rng.0.random_range(-3.0..3.0));
            let man = crate::units::spawn_soldier(&mut commands, &models, &palette, Side::Ua, at, out_to, out_to, 1.2, None, &mut rng.0);
            commands.entity(man).insert(Leaving(out_to + Vec2::new(rng.0.random_range(-40.0..40.0), rng.0.random_range(-40.0..40.0))));
        }
        info!("fight: the post at {:.0},{:.0} pulls out ({}), {} men, at {:.0} s", post.pos.x, post.pos.y, if ammo.0 == 0 { "dry" } else { "ordered" }, post.strength * s.men_per, script.t);
        field.out += post.strength * s.men_per;
        post.strength = 0;
        for f in post.figures.drain(..) {
            commands.entity(f).despawn();
        }
        commands.entity(e).insert(PulledOut);
        field.held_until = Some(script.t);
    }
}

/// A post still holding with their men all round it and no answer left is taken.
#[allow(clippy::type_complexity)]
fn overrun(
    mut commands: Commands,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    mut field: ResMut<Field>,
    mut radio: ResMut<Radio>,
    soldiers: Query<(&Soldier, &Transform)>,
    mut posts: Query<(Entity, &mut Post, Option<&crate::fire::Suppressed>, Option<&Ammo>), Without<PulledOut>>,
) {
    let Some(s) = spec() else { return };
    let now = time.elapsed_secs_f64();
    let attacker = crate::diorama::diorama().attacker();
    for (e, mut post, down, ammo) in &mut posts {
        if post.side == attacker {
            continue;
        }
        let close = soldiers.iter().filter(|(x, t)| x.side == attacker && x.state != SState::Wounded && t.translation.xz().distance(post.pos) < 35.0).count();
        // Down to its last men, or out of rounds, with them all round it: a post with fight left in it is not taken.
        let spent = post.strength <= 2 || ammo.is_some_and(|a| a.0 == 0);
        let _ = down;
        if close >= 6 && spent {
            let men = post.strength.max(1) * s.men_per;
            field.captured += men;
            field.lost += 1;
            post.strength = 0;
            for f in post.figures.drain(..) {
                commands.entity(f).despawn();
            }
            commands.entity(e).insert(PulledOut);
            field.held_until = Some(script.t);
            radio.say2(now, "Позиція оточена. Зв'язку нема.", "The position is surrounded. No contact.");
            info!("fight: a post at {:.0},{:.0} taken, {men} men, at {now:.0} s", post.pos.x, post.pos.y);
        }
    }
}

/// Our men walking out: straight for where they are going, and off the field when they get there.
fn leave(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &Leaving, &mut Transform, &Soldier)>) {
    let dt = crate::sim_dt(&time);
    for (e, l, mut tf, s) in &mut q {
        if s.state == SState::Wounded {
            continue;
        }
        let p = tf.translation.xz();
        let d = l.0 - p;
        if d.length() < 5.0 {
            commands.entity(e).despawn();
            continue;
        }
        let step = d.normalize() * s.speed * 1.3 * dt;
        tf.translation = geo::ground(p + step);
        tf.rotation = Quat::from_rotation_y(crate::units::vehicle_yaw(d));
    }
}

/// For the bands (`BAV_LOG_STATS`): the field each minute of the day's clock.
fn log_field(script: Res<crate::director::Script>, field: Res<Field>, air: Option<Res<crate::air::AirTally>>, posts: Query<(&Post, Option<&Ammo>), Without<PulledOut>>, mut last: Local<u32>) {
    if !crate::dev::dev().log_stats || script.t as u32 / 60 == *last {
        return;
    }
    *last = script.t as u32 / 60;
    let holding: Vec<String> = posts.iter().filter(|(p, _)| p.side == Side::Ua && p.strength > 0).map(|(p, a)| format!("{}/{}", p.strength, a.map_or(0, |a| a.0))).collect();
    let zones: Vec<String> = field.zones.iter().map(|z| format!("{}={}", z.name.0.trim_start_matches("the "), z.held.map_or("-", |s| if s == Side::Ua { "ua" } else { "ru" }))).collect();
    let (flown, down, off, landed) = air.as_ref().map_or((0, 0, 0, 0), |a| (a.flown, a.down.len(), a.driven_off, a.men_landed));
    info!(
        "field: t={:>4.0} {} flown={flown} down={down} driven_off={off} vdv_landed={landed} posts={:?} out={} taken={} lost={} held_until={:?} zones={}",
        script.t,
        crate::director::clock_text(script.t),
        holding,
        field.out,
        field.captured,
        field.lost,
        field.held_until.map(crate::director::clock_text),
        zones.join(",")
    );
}

/// Who holds each area: whoever has men in it, the other side none.
fn hold_zones(mut field: ResMut<Field>, soldiers: Query<(&Soldier, &Transform)>, posts: Query<&Post, Without<PulledOut>>) {
    let men: Vec<(Side, Vec2)> = soldiers.iter().filter(|(s, _)| s.state != SState::Wounded).map(|(s, t)| (s.side, t.translation.xz())).chain(posts.iter().filter(|p| p.strength > 0).map(|p| (p.side, p.pos))).collect();
    for z in &mut field.zones {
        let ua = men.iter().any(|(s, p)| *s == Side::Ua && geo::inside(&z.poly, *p));
        let ru = men.iter().any(|(s, p)| *s == Side::Ru && geo::inside(&z.poly, *p));
        z.held = match (ua, ru) {
            (true, false) => Some(Side::Ua),
            (false, true) => Some(Side::Ru),
            _ => None,
        };
    }
}
