//! Blasts and hits: the messages, what a blast does to whatever is near it, direct hits on hulls, reactive armour,
//! and the crews bailing out.

use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cause {
    Artillery,
    Mine,
    Atgm,
    TankGun,
    Himars,
    Thermobaric,
    /// A wreck's ammunition going: it hurts whoever is near it.
    CookOff,
    /// The player's FPV going in.
    Fpv,
    /// A Bradley's 25 mm: AP that goes through a BMP and bounces off a tank's front, where it takes the optics, a
    /// track or the engine instead (design/avdiivka.md, phase 2).
    Autocannon,
    /// 25 mm HE laid on a tank's sights and periscopes: it gets through nothing, and it is how the Stepove gunner
    /// stopped the T-90M once his AP gave trouble ("I started blinding him so he couldn't leave", v02 §6).
    Blinding,
    /// A grenade dropped from the reconnaissance drone (a VOG-17): a small burst on men, nothing to a hull
    /// (design/avdiivka.md, phase 4).
    Drop,
}

/// An explosion that hurts whatever is near it.
#[derive(Message, Clone, Copy)]
pub struct Blast {
    pub at: Vec3,
    /// Lethal radius for people in the open; vehicles need much closer hits.
    pub radius: f32,
    pub cause: Cause,
}

/// A guided weapon striking a specific vehicle.
#[derive(Message, Clone, Copy)]
pub struct DirectHit {
    pub target: Entity,
    pub cause: Cause,
    /// For the hit feed: what hit, and from where.
    pub weapon: &'static str,
    pub from: &'static str,
    /// Onto the roof, where the armour is thinnest (a Javelin's dive).
    pub top: bool,
}

pub(super) fn resolve_blasts(
    mut commands: Commands,
    mut blasts: MessageReader<Blast>,
    mut rng: ResMut<UnitRng>,
    mut stats: ResMut<Stats>,
    dress: Dress,
    mut vehicles: Query<(Entity, &mut Vehicle, &mut Suspension, &Transform)>,
    mut soldiers: Query<(Entity, &mut Soldier, &Transform)>,
) {
    let rng = &mut rng.0;
    for b in blasts.read() {
        let bp = b.at.xz();
        for (e, mut v, mut sus, tf) in &mut vehicles {
            // A hull abandoned intact is still there to be hit: that is how most of them end.
            if !v.state.alive() && v.state != VState::Abandoned {
                continue;
            }
            let d = bp.distance(tf.translation.xz());
            match b.cause {
                Cause::Mine => {
                    // The mine's own vehicle is handled where it went off.
                    continue;
                }
                Cause::Artillery | Cause::TankGun | Cause::Himars | Cause::Thermobaric | Cause::CookOff => {
                    let (kill_r, dmg_r) = if b.cause == Cause::Himars { (9.0, 20.0) } else { (3.0, 10.0) };
                    if d < kill_r && rng.random_bool(0.7) {
                        commands.trigger(Wrecked { vehicle: e, cause: b.cause, how: kill_how(v.kind, b.cause, rng) });
                    } else if d < dmg_r && v.state != VState::Abandoned {
                        v.panic = (v.panic + 0.5).min(1.0);
                        if v.state.mobile() && rng.random_bool(0.25) {
                            let coasting = v.immobilise(4.0, rng.random_range(-0.4..0.4));
                            commands.entity(e).insert(coasting);
                            v.crew_plan(rng, b.cause);
                            sus.roll_v += rng.random_range(-1.5..1.5);
                        }
                    }
                }
                _ => {}
            }
            if d < 160.0 {
                v.panic = (v.panic + 0.25 * (1.0 - d / 160.0)).min(1.0);
            }
        }
        for (e, mut s, tf) in &mut soldiers {
            let sp = tf.translation.xz();
            let d = bp.distance(sp);
            if d > b.radius * 3.0 {
                if d < 120.0 {
                    s.fear += 0.3;
                }
                // A burst within thirty metres puts a man on his face for a moment.
                if d < 30.0 && matches!(s.state, SState::Advance | SState::Flee) {
                    s.state = SState::Prone;
                    s.timer = 4.0;
                }
                continue;
            }
            // Flat in the snow a man is a little safer; flat in June grass and old craters, much safer.
            let flat = if s.side == Side::Ua { 0.2 } else { 0.4 };
            let prone = if matches!(s.state, SState::Prone | SState::DugIn | SState::Wounded) { flat } else { 1.0 };
            let p_kill = (1.0 - d / (b.radius * 3.0)).powf(1.5) * prone;
            if rng.random_bool(p_kill.clamp(0.0, 1.0) as f64) {
                // In Robotyne most men hit are wounded, not killed: they lie in the grass until someone brings
                // them out (dossier c09 §6: the 47th's losses that day were counted in single figures dead).
                if s.side == Side::Ua && rng.random_bool(if s.state == SState::Wounded { 0.5 } else { 0.75 }) {
                    s.state = SState::Wounded;
                    stats.wounded += 1;
                    continue;
                }
                stats.died(match b.cause {
                    Cause::Mine => "mines",
                    Cause::Atgm | Cause::TankGun => "direct fire",
                    Cause::CookOff => "cook-offs",
                    Cause::Drop => "drops",
                    _ => "shells",
                }, (s.side == diorama().attacker()) as u32);
                // Inside four metres (five of a cook-off) the blast throws the body a few metres; a man killed
                // further out by a fragment drops where he is.
                let throw_r = if b.cause == Cause::CookOff { 5.0 } else { 4.0 };
                let throw = (d < throw_r && b.radius > 3.0).then(|| {
                    let away = (sp - bp).normalize_or(Vec2::from_angle(rng.random_range(0.0..std::f32::consts::TAU)));
                    let speed = rng.random_range(3.0..10.0) * (1.0 - d / throw_r).max(0.3);
                    Vec3::new(away.x * speed, rng.random_range(2.0..5.0), away.y * speed)
                });
                kill_soldier(&mut commands, &dress.models, &dress.palette, &mut stats, (e, &s, tf), throw, rng);
            } else {
                s.fear += 1.0;
                // Knocked flat and stunned, five to twenty seconds, when it lands within ten metres.
                if d < 10.0 && b.radius > 3.0 && matches!(s.state, SState::Advance | SState::Flee) {
                    s.state = SState::Prone;
                    s.timer = -rng.random_range(5.0..20.0);
                }
            }
        }
    }
}

/// A body thrown by a blast: it tumbles through the air and lies where it lands.
pub fn throw_body(commands: &mut Commands, models: &Models, palette: &Palette, side: Side, from: Vec3, vel: Vec3, rng: &mut SmallRng) {
    commands.spawn((
        Mesh3d(models.figure(side).fallen.clone()),
        MeshMaterial3d(palette.vertex.clone()),
        Transform::from_translation(from).with_rotation(Quat::from_rotation_y(rng.random_range(0.0..std::f32::consts::TAU))),
        Debris { vel, spin: Vec3::new(0.0, rng.random_range(-4.0..4.0), 0.0) },
        NotShadowCaster,
    ));
}

/// A man (his entity, himself, where he is) killed where he stands, or thrown (`throw`: the blast's push on him) a
/// few metres and left there.
pub fn kill_soldier(commands: &mut Commands, models: &Models, palette: &Palette, stats: &mut Stats, man: (Entity, &Soldier, &Transform), throw: Option<Vec3>, rng: &mut SmallRng) {
    let (e, s, tf) = man;
    commands.entity(e).despawn();
    let p = tf.translation.xz();
    if let Some(vel) = throw {
        throw_body(commands, models, palette, s.side, geo::ground(p) + Vec3::Y * 0.8, vel, rng);
    } else {
        let mesh = models.figure(s.side).fallen.clone();
        commands.spawn((Mesh3d(mesh), MeshMaterial3d(palette.vertex.clone()), Transform::from_translation(geo::ground(p)).with_rotation(Quat::from_rotation_y(rng.random_range(0.0..std::f32::consts::TAU))), NotShadowCaster));
    }
    if s.side == diorama().attacker() {
        stats.attacker_dead += 1;
    } else {
        stats.defender_losses += 1;
    }
}

/// Reactive armour going off under a hit: a chain of flashes and grey puffs where the blocks were, and the
/// plates thrown off. The look of the hit only: the simulation has already decided what the hit did.
pub(super) fn era_reacts(mut hits: MessageReader<DirectHit>, mut sound: Sound, mut fx: ResMut<FxQueue>, mut last: ResMut<LastHit>, q: Query<(&Vehicle, &Transform)>) {
    for h in hits.read() {
        if let Ok((_, tf)) = q.get(h.target) {
            last.0 = Some(tf.translation);
        }
        if let Ok((v, tf)) = q.get(h.target)
            && v.kind.era()
            && h.cause != Cause::TankGun
        {
            fx.era_pops(tf, h.top);
            sound.at(tf.translation, Sfx::EraCrack, 1.2);
            info!("era pops on {:?} {}/{} at {:.0},{:.0}", v.kind, v.wave, v.idx, tf.translation.x, tf.translation.z);
        }
    }
}

pub(super) fn resolve_direct_hits(mut commands: Commands, time: Res<Time>, mut log: ResMut<crate::hits::HitLog>, mut hits: MessageReader<DirectHit>, mut rng: ResMut<UnitRng>, mut q: Query<(&mut Vehicle, &mut Suspension, Has<Rescue>, Has<Blinded>)>) {
    let now = time.elapsed_secs_f64();
    for h in hits.read() {
        let Ok((mut v, mut sus, rescue, blinded)) = q.get_mut(h.target) else { continue };
        if !v.state.alive() {
            continue;
        }
        if blinded && matches!(h.cause, Cause::Autocannon | Cause::Blinding) {
            log.open(now, h.weapon, h.from, Victim { target: h.target, vehicle: &v, rescue }, "on a hull already blind: nothing more");
            continue;
        }
        // The sights shot out: the hull drives blind on its last order, slewing, and the crew gets out.
        let mut blind = |v: &mut Vehicle, rng: &mut SmallRng| {
            log.open(now, h.weapon, h.from, Victim { target: h.target, vehicle: v, rescue }, "blinded: the sights gone, driving blind, the crew getting out");
            let coasting = v.immobilise(0.0, 0.0);
            let secs = rng.random_range(20.0..40.0);
            v.plan = CrewPlan::Ghost { secs, at: secs + rng.random_range(5.0..15.0) };
            v.timer = 0.0;
            v.panic = 1.0;
            commands.entity(h.target).insert((coasting, Blinded));
        };
        if h.cause == Cause::Blinding && v.kind.is_tank() {
            // HE on the sights: the periscopes and the sight heads take most of a burst that reaches the turret.
            if rng.0.random_bool(0.6) {
                blind(&mut v, &mut rng.0);
            } else {
                log.open(now, h.weapon, h.from, Victim { target: h.target, vehicle: &v, rescue }, "HE on the turret: the sights smeared, still seeing");
                v.panic = (v.panic + 0.3).min(1.0);
            }
            continue;
        }
        if h.cause == Cause::Autocannon && v.kind.is_tank() {
            // A burst of 25 mm on a tank: it does not get through the armour, but a tank is more than its armour.
            // The Stepove gunner went for the sights, then the engine deck (v02 §5, §7).
            let roll: f64 = rng.0.random();
            if roll < 0.35 {
                blind(&mut v, &mut rng.0);
            } else if roll < 0.60 {
                log.open(now, h.weapon, h.from, Victim { target: h.target, vehicle: &v, rescue }, "a track off: stopped");
                let coasting = v.immobilise(2.5, rng.0.random_range(-0.4..0.4));
                commands.entity(h.target).insert(coasting);
                v.crew_plan(&mut rng.0, h.cause);
            } else if roll < 0.72 {
                log.open(now, h.weapon, h.from, Victim { target: h.target, vehicle: &v, rescue }, "into the engine deck: on fire, crawling, crew getting out");
                let coasting = v.immobilise(rng.0.random_range(0.6..1.2), rng.0.random_range(-0.2..0.2));
                v.crew_plan(&mut rng.0, h.cause);
                commands.entity(h.target).insert((coasting, Fire::deck(&mut rng.0, h.cause, v.kind), Emitter::new(EmitterKind::EngineSmoke, 4.0, 400.0, Vec3::new(0.0, 1.5, 2.2))));
            } else {
                log.open(now, h.weapon, h.from, Victim { target: h.target, vehicle: &v, rescue }, "sparks off the turret: nothing");
                v.panic = (v.panic + 0.4).min(1.0);
            }
            continue;
        }
        let p_kill = if h.top { (v.kind.hit_kill() + 0.2).min(0.95) } else { v.kind.hit_kill() };
        let how = format!("{}{}", if h.top { "from above: " } else { "" }, if v.kind.era() && !matches!(h.cause, Cause::TankGun | Cause::Autocannon | Cause::Blinding) { "reactive armour went; " } else { "" });
        if rng.0.random_bool(p_kill) {
            if rng.0.random_bool(0.2) {
                log.open(now, h.weapon, h.from, Victim { target: h.target, vehicle: &v, rescue }, format!("{how}into the engine: on fire, crawling, crew getting out"));
                // A hit on the engine: it crawls on trailing smoke, the crew gets out, and the fire decides.
                let coasting = v.immobilise(rng.0.random_range(0.6..1.2), rng.0.random_range(-0.2..0.2));
                v.crew_plan(&mut rng.0, h.cause);
                commands.entity(h.target).insert((coasting, Fire::deck(&mut rng.0, h.cause, v.kind), Emitter::new(EmitterKind::EngineSmoke, 4.0, 400.0, Vec3::new(0.0, 1.5, 2.2))));
            } else {
                log.open(now, h.weapon, h.from, Victim { target: h.target, vehicle: &v, rescue }, format!("{how}penetrated"));
                commands.trigger(Wrecked { vehicle: h.target, cause: h.cause, how: kill_how(v.kind, h.cause, &mut rng.0) });
            }
        } else {
            log.open(now, h.weapon, h.from, Victim { target: h.target, vehicle: &v, rescue }, format!("{how}did not get through: stopped, crew shaken"));
            let coasting = v.immobilise(3.5, rng.0.random_range(-0.3..0.3));
            commands.entity(h.target).insert(coasting);
            v.crew_plan(&mut rng.0, h.cause);
            sus.pitch_v -= 1.2;
            v.panic = 1.0;
        }
    }
}

/// Where a man who has just got out runs first: the nearest crater, wreck or tree line within reach, chosen
/// among them so that no two men run the same way; or nowhere in particular, but off to one side.
fn cover_near(p: Vec2, home: Vec2, craters: &crate::paint::Craters, hulls: &[Vec2], rng: &mut SmallRng) -> Vec2 {
    let mut spots: Vec<Vec2> = Vec::new();
    if let Some(c) = craters.nearest(p, 15.0) {
        spots.push(c);
    }
    if let Some(h) = hulls.iter().filter(|h| h.distance(p) > 6.0 && h.distance(p) < 60.0).min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))) {
        // Behind the hull, away from the enemy's line.
        spots.push(*h + (home - *h).normalize_or_zero() * 5.0);
    }
    if let Some((d, s, belt)) = geo::layout().belts.iter().map(|b| { let (d, s) = b.line.project(p); (d, s, b) }).min_by(|a, b| a.0.total_cmp(&b.0))
        && d < 150.0
    {
        spots.push(belt.line.point(s));
    }
    if spots.is_empty() {
        // Away from here, spread out, then home.
        let dir = (home - p).normalize_or_zero();
        let side = Vec2::new(-dir.y, dir.x);
        p + dir * rng.random_range(20.0..40.0) + side * rng.random_range(-40.0..40.0)
    } else {
        spots[rng.random_range(0..spots.len())] + Vec2::new(rng.random_range(-2.0..2.0), rng.random_range(-2.0..2.0))
    }
}

/// Where a crew getting out runs: the routes back, and the craters and stopped hulls to take cover by.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct Escape<'w, 's> {
    book: Res<'w, RouteBook>,
    paint: Res<'w, PaintQueue>,
    hulls: Local<'s, Vec<Vec2>>,
}

/// Crews of knocked-out vehicles get out one at a time, out of the hatches on either side, once the crew's plan
/// says so (`bail_at`); dismounts from wrecks, if any live, too. A fit crew is out in seconds; a concussed one
/// takes a minute. Each man sprints for the nearest cover before he thinks of the way home.
pub(super) fn crew_bailout(mut commands: Commands, time: Res<Time>, mut rng: ResMut<UnitRng>, mut stats: ResMut<Stats>, dress: Dress, mut escape: Escape, mut q: Query<(&mut Vehicle, &Driver, &Transform)>) {
    let dt = time.delta_secs();
    let lay = geo::layout();
    escape.hulls.clear();
    escape.hulls.extend(q.iter().filter(|(v, ..)| !v.state.mobile()).map(|(_, _, t)| t.translation.xz()));
    for (mut v, d, tf) in &mut q {
        if v.crew + v.troops == 0 {
            continue;
        }
        match v.state {
            VState::Immobilized => {
                v.timer += dt;
                if v.timer <= v.plan.bail_at() {
                    continue;
                }
            }
            VState::Destroyed | VState::Abandoned => {}
            _ => continue,
        }
        v.exit_timer -= dt;
        if v.exit_timer > 0.0 {
            continue;
        }
        v.exit_timer = rng.0.random_range(0.5..3.0);
        let route = &escape.book.0[d.route];
        let p = tf.translation.xz();
        let fwd = (tf.rotation * Vec3::NEG_Z).xz().normalize_or_zero();
        let right = Vec2::new(-fwd.y, fwd.x);
        // Attackers run back down their track; defenders back behind their own line.
        let attacker = v.side == diorama().attacker();
        let home = if attacker { route.line.point(0.0) } else { p + Vec2::new(0.0, 600.0 * lay.advance) };
        // The crew leaves first. Some dismounts go forward anyway (fewer of the 47th's: they had been told the
        // infantry comes back); the rest go for cover, then back.
        let is_crew = v.crew > 0;
        let p_forward = if v.side == Side::Ru { 0.4 } else { 0.15 };
        let forward = attacker && !is_crew && rng.0.random_bool(p_forward);
        let at = p + right * sign(&mut rng.0) * 2.2 + fwd * rng.0.random_range(-1.5..1.5);
        let goal = if forward { route.objective + Vec2::new(rng.0.random_range(-100.0..100.0), 0.0) } else { home };
        let leg = (!forward).then(|| cover_near(at, home, &escape.paint.craters, &escape.hulls, &mut rng.0));
        let e = spawn_soldier(&mut commands, &dress.models, &dress.palette, NewSoldier { side: v.side, at, goal, home, sprint: rng.0.random_range(10.0..25.0), cover: leg }, &mut rng.0);
        if !forward {
            commands.entity(e).insert(Fleeing);
        }
        if is_crew {
            v.crew -= 1;
        } else {
            v.troops -= 1;
        }
        if v.state == VState::Immobilized {
            v.state = VState::Abandoned;
            if attacker {
                stats.abandoned += 1;
                stats.wave(v.wave).abandoned += 1;
            } else {
                stats.defender_vehicles += 1;
            }
        }
    }
}

#[derive(Component)]
pub struct Fleeing;
