//! The infantry fight for a place (design/hostomel.md, phase 3): what the column game did not need. Our posts
//! carry ammunition and run dry; the enemy's men on foot fire back and wear the posts down; a post that is empty, or
//! told to (Space: "Відхід!"), pulls out and its men walk off the field; one overrun where it stands is taken, men
//! and all. Over it, the field's areas (the hangar, the apron, the tower, the base) are whoever's men are in them.
//!
//! Only a diorama whose battle file has a `fight` has any of this: the Record of the others must not change. A day
//! with `areas` and no fight (Avdiivka) has the areas alone: who holds each, on the HUD, the table and in the debrief.

use crate::fire::Post;
use crate::geo;
use crate::units::{Models, Palette, Radio, SState, Side, Soldier, Stats};
use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use serde::Deserialize;

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        let Some(spec) = spec() else {
            // A fog or a night without the fight (Myrnohrad): the one visibility all the same, for the posts' rifles,
            // the observers, the staff, the drone and the FPVs; before the script, so the frame's fire sees this
            // frame's light.
            if seeing_here() {
                app.insert_resource(Seeing::new()).add_systems(Update, (see, lift).chain().run_if(crate::running).in_set(crate::Step::Script).before(crate::director::DirectorSystems));
                app.add_systems(Update, log_seen.run_if(crate::running).in_set(crate::Step::Effects));
            }
            // The areas without the fight (Avdiivka): who holds each, and nothing else of it. The posts there are the
            // column game's, worn down by its own fire.
            let areas = &crate::data::sim_data().areas;
            if !areas.is_empty() {
                app.insert_resource(Field { zones: zones(areas), ..default() }).add_systems(Update, (hold_zones, log_areas).chain().run_if(crate::running).in_set(crate::Step::Units).after(crate::units::drop_riders).after(crate::seep::SeepSystems));
            }
            return;
        };
        app.insert_resource(FightRng(SmallRng::seed_from_u64(crate::dev::seeded(2202))))
            .insert_resource(Seeing::new())
            .insert_resource(Field { zones: zones(&spec.zones), ..default() })
            // After the units have moved, fought and died this frame: the fight reads and writes the same men, posts and
            // places, and in no fixed order against them a day would not replay itself.
            .add_systems(Update, (see, supply_posts, order_out, assault, return_fire, pull_out, overrun, leave, hold_zones, log_field).chain().after(crate::units::drop_riders).run_if(crate::running).in_set(crate::Step::Units));
        crate::flares::add(app);
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
    /// Hand flares at each of the defenders' posts, sent up over the men rushing it in the dark (`flares`).
    #[serde(default)]
    pub flares: u32,
    /// How much of a post's fire still goes out while it is down under shells and cannon fire (`fire::Suppressed`):
    /// 1 (all of it) unless the file says less.
    #[serde(default = "whole")]
    pub pinned: f32,
    /// Men the attackers leave in an area they hold when the rest go on to the next post, so that it stays theirs.
    #[serde(default)]
    pub keep: usize,
    /// So many of the attackers round a post (within 60 m) and it draws no more of them: the rest go on to the next.
    #[serde(default)]
    pub ring: usize,
    /// The field's areas.
    pub zones: Vec<ZoneSpec>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ZoneSpec {
    /// What the rules know it by (`apron`, `tower`: the airlift needs both).
    pub key: String,
    pub name: (String, String),
    pub poly: Vec<(f32, f32)>,
}

fn whole() -> f32 {
    1.0
}

pub fn spec() -> Option<&'static FightSpec> {
    crate::data::sim_data().fight.as_ref()
}

/// The field's areas as the file draws them: the fight's, or the day's own where it has no fight (none: no areas).
pub fn zone_specs() -> &'static [ZoneSpec] {
    let d = crate::data::sim_data();
    d.fight.as_ref().map_or(&d.areas[..], |f| &f.zones[..])
}

fn zones(specs: &[ZoneSpec]) -> Vec<Zone> {
    specs.iter().map(|z| Zone { key: z.key.clone(), name: z.name.clone(), poly: z.poly.iter().map(|&(x, y)| Vec2::new(x, y)).collect(), held: None }).collect()
}

#[derive(Resource)]
pub(crate) struct FightRng(pub(crate) SmallRng);

/// Whether the day has a visibility of its own without the fight: a fog in its file, or a night (a part whose file
/// dates its moon, `bav_core::sun::moon`). Elsewhere there is no `Seeing` and the battle sees as on a clear day.
fn seeing_here() -> bool {
    let d = bav_core::data::data();
    !d.fog.is_empty() || d.moon.is_some()
}

/// How far one sees now: the battle's one visibility (design/myrnohrad.md, phase 2), asked by the posts' rifles, the
/// attackers' return fire, the observers, the staff's eyes, the drone's picture and the FPVs. Two terms, light and
/// weather. The light is a share of the day's: the dark (`bav_core::sun::sight`), and under the flares burning over
/// the field as by day. The weather is the fog's metres (`bav_core::data::fog_at`): men see a few hundred metres in
/// it, the drone's day camera as little, its thermal some (c15 §5 "fog"). Only where the day has a fight, a fog or a
/// night: a resource more would be in another day's Record.
#[derive(Resource)]
pub struct Seeing {
    pub dark: f32,
    /// The ground the flares light: under each, so far round (`flares::FlareKind::reach`).
    pub lit: Vec<(Vec2, f32)>,
    /// How far anything is seen through the weather now (m): the fog's; infinite in clear air.
    pub murk: f32,
}

/// What is looking through the weather.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Eye {
    /// A man's, through binoculars or a sight; and a day camera's, which sees no further in fog.
    Man,
    /// A thermal camera's: long-wave infrared carries through fog's fine droplets about three times as far as light
    /// (the drone's channel I; c15 §5: in fog the thermal sees "some").
    Thermal,
}

/// How much further a thermal camera sees through fog than an eye.
const THERMAL_IN_FOG: f32 = 3.0;

impl Seeing {
    fn new() -> Self {
        Seeing { dark: 1.0, lit: Vec::new(), murk: bav_core::data::fog_at(crate::director::clock_minutes(0.0)).unwrap_or(f32::INFINITY) }
    }

    /// The light at `p`, as a share of the day's.
    pub fn light(&self, p: Vec2) -> f32 {
        if self.lit.iter().any(|(c, r)| c.distance(p) < *r) { 1.0 } else { self.dark }
    }

    /// How far this eye sees through the weather now (m).
    pub fn through(&self, eye: Eye) -> f32 {
        match eye {
            Eye::Man => self.murk,
            Eye::Thermal => self.murk * THERMAL_IN_FOG,
        }
    }

    /// How far men see a man or a post at `p`, for what they would see at `base` metres in clear daylight: the
    /// light's share of it, times the weather's (the fog's metres as a share of `base`, at most all of it).
    pub fn reach(&self, base: f32, p: Vec2) -> f32 {
        base * self.light(p) * (self.murk / base).min(1.0)
    }

    /// Whether the fog is thick enough to keep the light FPVs on the ground (c15 §3: useless in the wet).
    pub fn grounds_light_fpvs(&self) -> bool {
        self.murk < FPV_FOG
    }
}

/// Below this much sight through the fog (m) a light FPV's pilot cannot find his way in the goggles.
const FPV_FOG: f32 = 500.0;

pub(crate) fn see(script: Res<crate::director::Script>, mut seeing: ResMut<Seeing>) {
    let minutes = crate::director::clock_minutes(script.t);
    seeing.dark = bav_core::sun::sight(minutes);
    seeing.murk = bav_core::data::fog_at(minutes).unwrap_or(f32::INFINITY);
}

/// The fog has lifted when it lets one see this far (m): the road is open to their drones.
pub(crate) const LIFTED: f32 = 2000.0;

/// When the fog lifts (minutes after midnight): where its metres first pass `LIFTED` in the day's table, if they do.
fn lift_time() -> Option<f32> {
    if crate::dev::dev().fog_off {
        return None;
    }
    let fog = &bav_core::data::data().fog;
    if fog.first()?.1 >= LIFTED {
        return None;
    }
    let w = fog.windows(2).find(|w| w[1].1 >= LIFTED)?;
    Some(w[0].0 + (w[1].0 - w[0].0) * (LIFTED - w[0].1) / (w[1].1 - w[0].1))
}

/// The net says the fog is going twenty minutes of the day's clock before it lifts, as the forecast gave it: what is
/// on the road then has that long to get off it (c15 §5: what is still on the road when it lifts is seen).
fn lift(script: Res<crate::director::Script>, time: Res<Time>, mut radio: ResMut<Radio>, mut said: Local<bool>) {
    if *said {
        return;
    }
    let Some(at) = lift_time() else {
        *said = true;
        return;
    };
    if crate::director::clock_minutes(script.t) < at - 20.0 {
        return;
    }
    *said = true;
    radio.say2(
        time.elapsed_secs_f64(),
        "Всім, я {command}. Туман сходить, за двадцять хвилин буде видно. Усе з дороги. Пташки готуйте.",
        "All, this is {command}. The fog's lifting, clear in twenty minutes. Everything off the road. Get the birds ready.",
    );
    info!("seeing: the net warns of the lift at {}", crate::director::clock_text(script.t));
}

/// For the bands (`BAV_LOG_STATS`), where the day has a fog or a night: how far one sees once a minute of the day's
/// clock, and each of the attackers' men once, when a post of ours first has him in its rifles' reach, and how far
/// from it he is then.
#[allow(clippy::type_complexity)]
fn log_seen(script: Res<crate::director::Script>, seeing: Res<Seeing>, soldiers: Query<(Entity, &Soldier, &Transform)>, posts: Query<&Post>, mut seen: Local<std::collections::BTreeSet<Entity>>, mut last: Local<Option<u32>>) {
    if !crate::dev::dev().log_stats {
        return;
    }
    let minute = script.t as u32 / 60;
    if *last != Some(minute) {
        *last = Some(minute);
        let m = |v: f32| if v.is_finite() { format!("{v:.0} m") } else { "clear".to_string() };
        info!("seeing: {} light {:.2}, eye {}, thermal {}, the posts' rifles {:.0} m", crate::director::clock_text(script.t), seeing.dark, m(seeing.through(Eye::Man)), m(seeing.through(Eye::Thermal)), 750.0 * seeing.dark * (seeing.murk / 750.0).min(1.0));
    }
    let attacker = crate::diorama::diorama().attacker();
    for (e, s, tf) in &soldiers {
        if s.side != attacker || seen.contains(&e) {
            continue;
        }
        let p = tf.translation.xz();
        let near = posts.iter().filter(|q| q.side != attacker && q.strength > 0).map(|q| q.pos.distance(p)).min_by(f32::total_cmp);
        if let Some(d) = near
            && d < seeing.reach(750.0, p)
        {
            seen.insert(e);
            info!("seen: one of theirs {d:.0} m from a post, {}", crate::director::clock_text(script.t));
        }
    }
}

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
    pub key: String,
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
    /// The defenders' posts still holding with none of the attackers' men within 100 m: where the attackers' guns may
    /// still fire (`Act::Prep` where there is no line across the field), west to east.
    pub open: Vec<Vec2>,
}

/// The defenders' posts get their ammunition once they are on the ground (ours in the morning, the VDV's at dusk).
#[allow(clippy::type_complexity)]
pub(crate) fn supply_posts(mut commands: Commands, posts: Query<(Entity, &Post), (Without<Ammo>, Without<PulledOut>)>) {
    let Some(s) = spec() else { return };
    for (e, p) in &posts {
        if p.side == crate::diorama::diorama().defender() {
            commands.entity(e).insert(Ammo(s.ammo));
            if s.flares > 0 {
                commands.entity(e).insert(crate::flares::HandFlares { left: s.flares, next: 0.0 });
            }
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
fn assault(time: Res<Time>, mut rng: ResMut<FightRng>, mut tick: Local<f32>, field: Res<Field>, posts: Query<&Post, Without<PulledOut>>, mut soldiers: Query<(&mut Soldier, &Transform), Without<Leaving>>) {
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
    // Where the file says so, a section stays behind in each area of the field the attackers have got into: their men
    // dug in there, area by area, and how many may still go on.
    let keep = spec().map_or(0, |f| f.keep);
    let zone_of = |p: Vec2| field.zones.iter().position(|z| geo::inside(&z.poly, p));
    let mut spare: Vec<usize> = vec![0; field.zones.len()];
    if keep > 0 {
        for (s, tf) in &soldiers {
            if s.side == attacker && s.state == SState::DugIn
                && let Some(z) = zone_of(tf.translation.xz())
            {
                spare[z] += 1;
            }
        }
        for n in &mut spare {
            *n = n.saturating_sub(keep);
        }
    }
    // Where the file says so, a post already ringed by so many of ours draws no more: the rest go on to the next.
    let ring = spec().map_or(0, |f| f.ring);
    // (Round it: there, or on the way there.)
    let mut round: Vec<usize> = vec![0; holding.len()];
    if ring > 0 {
        for (s, tf) in &soldiers {
            if s.side == attacker && !matches!(s.state, SState::Flee | SState::Wounded) {
                for (i, h) in holding.iter().enumerate() {
                    if h.distance(tf.translation.xz()) < 60.0 || (s.state == SState::Advance && h.distance(s.goal) < 60.0) {
                        round[i] += 1;
                    }
                }
            }
        }
    }
    for (mut s, tf) in &mut soldiers {
        let p = tf.translation.xz();
        if s.side != attacker || s.state != SState::DugIn || !rng.0.random_bool(spec().map_or(0.0, |f| f.assault)) {
            continue;
        }
        let Some(&post) = holding.iter().min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))) else { continue };
        if post.distance(p) < 40.0 {
            continue;
        }
        // The nearest post not yet ringed by enough of ours (any post, where the file does not say how many).
        let post = holding.iter().enumerate().filter(|(i, _)| ring == 0 || round[*i] < ring).min_by(|a, b| a.1.distance(p).total_cmp(&b.1.distance(p))).map_or(post, |(i, &q)| {
            round[i] += 1;
            q
        });
        if post.distance(p) > 900.0 {
            continue;
        }
        if keep > 0
            && let Some(z) = zone_of(p)
        {
            if spare[z] == 0 {
                continue;
            }
            spare[z] -= 1;
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
pub(crate) fn return_fire(
    mut commands: Commands,
    time: Res<Time>,
    seeing: Res<Seeing>,
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
        // Only so many can fire on one post at once, from where they lie: the nearest ten that can see it (the sight
        // lines tested nearest first, and no more than needed).
        let mut near: Vec<(Vec3, f32)> = soldiers
            .iter()
            .filter(|(s, _)| s.side == attacker && !matches!(s.state, SState::Flee | SState::Wounded))
            .map(|(_, t)| (t.translation, t.translation.xz().distance(post.pos)))
            .filter(|(_, d)| *d < seeing.reach(400.0, post.pos))
            .collect();
        near.sort_by(|a, b| a.1.total_cmp(&b.1));
        let shooters: Vec<(Vec3, f32)> = near.into_iter().filter(|(p, _)| geo::line_of_sight(p.xz(), post.pos)).take(10).collect();
        if shooters.is_empty() {
            continue;
        }
        // Each a few rounds a minute that tell, the closer the more.
        let p: f32 = shooters.iter().map(|(_, d)| 0.002 * (1.0 - d / 450.0)).sum::<f32>() * dt;
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
pub(crate) fn pull_out(
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
            if post.side == Side::Ua {
                radio.say2(now, "Позиція оточена. Зв'язку нема.", "The position is surrounded. No contact.");
            } else {
                radio.say2(now, "Їхню позицію взяли. Полонені є.", "Their position is ours. We have prisoners.");
            }
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
fn log_field(script: Res<crate::director::Script>, field: Res<Field>, air: Option<Res<crate::air::AirTally>>, airfield: Option<Res<crate::air::Airfield>>, posts: Query<(&Post, Option<&Ammo>), Without<PulledOut>>, mut last: Local<u32>) {
    if !crate::dev::dev().log_stats || script.t as u32 / 60 == *last {
        return;
    }
    *last = script.t as u32 / 60;
    // The defenders' posts still holding, strength/ammunition (ours in the morning, the VDV's at dusk).
    let defender = crate::diorama::diorama().defender();
    let holding: Vec<String> = posts.iter().filter(|(p, _)| p.side == defender && p.strength > 0).map(|(p, a)| format!("{}/{}", p.strength, a.map_or(0, |a| a.0))).collect();
    let zones: Vec<String> = field.zones.iter().map(|z| format!("{}={}", z.name.0.trim_start_matches("the "), z.held.map_or("-", |s| if s == Side::Ua { "ua" } else { "ru" }))).collect();
    let (flown, down, off) = air.as_ref().map_or((0, 0, 0), |a| (a.flown, a.down.len(), a.driven_off));
    let landed = airfield.as_ref().map_or(0, |f| f.men_landed);
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

/// For the bands (`BAV_LOG_STATS`), where the areas stand without the fight: who holds each, once a minute of the day's
/// clock, with the attackers' men alive in each.
fn log_areas(script: Res<crate::director::Script>, field: Res<Field>, soldiers: Query<(&Soldier, &Transform)>, mut last: Local<u32>) {
    if !crate::dev::dev().log_stats || script.t as u32 / 60 == *last {
        return;
    }
    *last = script.t as u32 / 60;
    let attacker = crate::diorama::diorama().attacker();
    let areas: Vec<String> = field
        .zones
        .iter()
        .map(|z| {
            let men = soldiers.iter().filter(|(s, t)| s.side == attacker && s.state != SState::Wounded && geo::inside(&z.poly, t.translation.xz())).count();
            format!("{}={}/{men}", z.key, z.held.map_or("-", |s| if s == Side::Ua { "ua" } else { "ru" }))
        })
        .collect();
    info!("areas: t={:>4.0} {} areas={}", script.t, crate::director::clock_text(script.t), areas.join(","));
}

/// Who holds each area: whoever has men in it, the other side none.
fn hold_zones(mut field: ResMut<Field>, soldiers: Query<(&Soldier, &Transform)>, posts: Query<&Post, Without<PulledOut>>) {
    let men: Vec<(Side, Vec2)> = soldiers.iter().filter(|(s, _)| s.state != SState::Wounded).map(|(s, t)| (s.side, t.translation.xz())).chain(posts.iter().filter(|p| p.strength > 0).map(|p| (p.side, p.pos))).collect();
    let attacker = crate::diorama::diorama().attacker();
    let on: Vec<Vec2> = soldiers.iter().filter(|(s, _)| s.side == attacker && !matches!(s.state, SState::Flee | SState::Wounded)).map(|(_, t)| t.translation.xz()).collect();
    field.open = posts.iter().filter(|p| p.side != attacker && p.strength > 0 && !on.iter().any(|m| m.distance(p.pos) < 100.0)).map(|p| p.pos).collect();
    field.open.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y)));
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
