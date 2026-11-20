//! Soldiers: men on foot, the mines under them, boarding, their legs, and the debris in the air.

use super::*;

/// Wrecks men take cover behind, and vehicles men can run to: neither a soldier.
pub(super) type Wrecks = (With<Vehicle>, With<Coasting>, Without<Soldier>);
type Rides = (Without<Coasting>, Without<Soldier>);

/// What a man on foot goes by: the player's order to the column, the wrecks he can take cover behind, and the
/// vehicles that can pick him up.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct Bearings<'w, 's> {
    order: Res<'w, ColumnOrder>,
    wrecks: Query<'w, 's, &'static Transform, Wrecks>,
    bradleys: Query<'w, 's, (&'static Vehicle, &'static Transform, Has<Rescue>), Rides>,
}

pub(super) fn move_soldiers(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<UnitRng>,
    mut stats: ResMut<Stats>,
    mut footfall: Footfall,
    bearings: Bearings,
    mut q: Query<(Entity, &mut Soldier, &mut Transform, Option<&Fleeing>)>,
) {
    let dt = crate::sim_dt(&time);
    let rng = &mut rng.0;
    let lay = geo::layout();
    let attacker = diorama().attacker();
    let wreck_pos: Vec<Vec2> = bearings.wrecks.iter().map(|t| t.translation.xz()).collect();
    // Bradleys waiting for men: whoever is falling back runs for the nearest one.
    let pickups: Vec<Vec2> = bearings.bradleys
        .iter()
        .filter(|(v, _, rescue)| v.kind == VKind::Bradley && v.side == attacker && v.troops < 7 && (*rescue || v.state == VState::Overwatch))
        .map(|(_, t, _)| t.translation.xz())
        .collect();
    let mut foothold = 0;
    // Back is home, or behind the attackers' own start line where there is one: the tree line with the medics.
    let safe = |p: Vec2, home: Vec2| p.distance(home) < 25.0 || lay.start_z.is_some_and(|z| (p.y - z) * lay.advance < 0.0);
    // Men falling back drag a wounded man with them: who is lying where, and who is being dragged already.
    let lying: Vec<(Entity, Vec2)> = q.iter().filter(|(_, s, _, _)| s.state == SState::Wounded && s.side == attacker).map(|(e, _, t, _)| (e, t.translation.xz())).collect();
    let mut carried: Vec<Entity> = q.iter().filter_map(|(_, s, _, _)| s.carrying).filter(|c| lying.iter().any(|(w, _)| w == c)).collect();
    let mut dragged_to: Vec<(Entity, Vec3, Quat)> = Vec::new();
    for (e, mut s, mut tf, fleeing) in &mut q {
        if s.side != attacker {
            continue;
        }
        // Ordered back, everyone on foot falls back too, to the vehicles or the start line.
        if (fleeing.is_some() && s.state == SState::Advance) || (*bearings.order == ColumnOrder::PullBack && !matches!(s.state, SState::Flee | SState::Wounded)) {
            s.state = SState::Flee;
        }
        s.fear = (s.fear - dt * 0.15).max(0.0);
        s.timer += dt;
        s.sprint = (s.sprint - dt).max(0.0);
        let p = tf.translation.xz();
        let mut speed = 0.0;
        let mut target = p;
        match s.state {
            SState::Advance => {
                target = s.goal;
                speed = s.speed;
                if s.fear > 1.0 {
                    s.state = SState::Prone;
                    s.timer = 0.0;
                }
                if p.distance(s.goal) < 12.0 {
                    s.state = SState::DugIn;
                }
            }
            SState::Prone => {
                if s.timer > rng.random_range(5.0..12.0) {
                    s.timer = 0.0;
                    // "They'd come in groups of five. We'd kill four, one would stay and dig in."
                    let near_wreck = wreck_pos.iter().any(|w| w.distance(p) < 35.0);
                    s.state = if fleeing.is_some() || (s.side == Side::Ua && *bearings.order == ColumnOrder::PullBack) {
                        SState::Flee
                    } else if s.fear > 2.2 && near_wreck && rng.random_bool(0.5) {
                        SState::DugIn
                    } else if s.fear > 1.6 && geo::slope(p) > 0.1 && rng.random_bool(0.5) {
                        // Pinned on a spoil heap's face, a man scrapes a hole in the slag rather than get up into the
                        // fire from the brow again: the face is reached and held against (c11 §6 set piece 2).
                        SState::DugIn
                    } else if s.fear > 2.6 && rng.random_bool(0.5) {
                        SState::Flee
                    } else {
                        SState::Advance
                    };
                    s.fear *= 0.6;
                }
            }
            SState::Flee => {
                // First to cover, and down in it for a moment; then the nearest Bradley or home.
                if let Some(leg) = s.leg
                    && p.distance(leg) < 3.0
                {
                    s.leg = None;
                    s.state = SState::Prone;
                    s.timer = rng.random_range(2.0..6.0);
                }
                target = s.leg.unwrap_or_else(|| pickups.iter().copied().filter(|b| b.distance(p) < 160.0).min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))).unwrap_or(s.home));
                // Flat out while the sprint lasts, a jog after.
                speed = s.speed * if s.sprint > 0.0 { 2.2 } else { 1.3 };
                // A wounded man nearby is not left: he goes to him, then drags him back, both slow, both in the open.
                if s.carrying.is_some_and(|c| !lying.iter().any(|(w, _)| *w == c)) {
                    s.carrying = None;
                }
                if s.carrying.is_none()
                    && s.leg.is_none()
                    && let Some(&(w, _)) = lying.iter().filter(|(w, wp)| !carried.contains(w) && wp.distance(p) < 30.0).min_by(|a, b| a.1.distance(p).total_cmp(&b.1.distance(p)))
                {
                    s.carrying = Some(w);
                    carried.push(w);
                    debug!("drag: a man falling back goes for a wounded man at {:.0},{:.0}", p.x, p.y);
                }
                if let Some(c) = s.carrying
                    && let Some(&(_, wp)) = lying.iter().find(|(w, _)| *w == c)
                {
                    if wp.distance(p) > 2.5 {
                        target = wp;
                    } else {
                        speed = 0.9;
                    }
                }
                // Falling back under shellfire, the 47th's men drop flat when it lands close, then go on.
                if s.side == Side::Ua && s.fear > 1.2 {
                    s.state = SState::Prone;
                    s.timer = 0.0;
                    s.fear *= 0.5;
                    speed = 0.0;
                }
                if safe(p, s.home) || !geo::in_bounds(p) {
                    stats.returned += 1;
                    commands.entity(e).despawn();
                    continue;
                }
            }
            SState::Wounded => {
                target = pickups.iter().copied().filter(|b| b.distance(p) < 220.0).min_by(|a, b| a.distance(p).total_cmp(&b.distance(p))).unwrap_or(s.home);
                // Being dragged, he goes where the man dragging him goes (placed after this loop).
                speed = if carried.contains(&e) { 0.0 } else { 0.6 };
                if safe(p, s.home) {
                    stats.returned += 1;
                    stats.wounded_out += 1;
                    commands.entity(e).despawn();
                    continue;
                }
            }
            SState::DugIn => {}
        }
        if speed > 0.0 {
            let d = (target - p).normalize_or_zero();
            let wob = Vec2::new((s.timer * 0.7 + s.phase).sin(), (s.timer * 0.5 + s.phase).cos()) * 0.25;
            // Climbing a spoil heap's face is a scramble on loose black slag: a third of a walking pace at best.
            let grade = geo::slope(p);
            let climb = if grade > 0.1 { (1.0 - (grade - 0.1) * 2.5).clamp(0.3, 1.0) } else { 1.0 };
            let step = (d + wob).normalize_or_zero() * speed * climb * dt;
            let np = p + step;
            tf.translation = geo::ground(np);
            tf.rotation = Quat::from_rotation_y(vehicle_yaw(step));
            if s.state == SState::Wounded {
                // Crawling: flat on the ground, head first.
                tf.translation.y += 0.25;
                tf.rotation *= Quat::from_rotation_x(-1.45);
            }
            s.phase += speed * dt * 3.2;
            if let Some(w) = s.carrying
                && lying.iter().any(|(lw, wp)| *lw == w && wp.distance(p) <= 2.5)
            {
                // A metre behind, on his back.
                let back = geo::ground(np - step.normalize_or_zero() * 1.0) + Vec3::Y * 0.25;
                dragged_to.push((w, back, Quat::from_rotation_y(vehicle_yaw(step)) * Quat::from_rotation_x(-1.45)));
            }
            if rng.random_bool((dt * 0.6) as f64) {
                footfall.paint.push(Paint::Footprints { at: np });
            }
            footfall.step(np);
        } else if !(s.state == SState::Wounded && carried.contains(&e)) {
            // Prone or dug in: lie flat.
            tf.translation = geo::ground(p) + Vec3::Y * if s.state == SState::DugIn { -0.6 } else { -0.75 };
        }
        // A foothold: attackers alive at the defenders' forward line or past it.
        if matches!(s.state, SState::DugIn | SState::Prone | SState::Advance) && lay.foothold_z.is_some_and(|z| (p.y - z) * lay.advance > 0.0) {
            foothold += 1;
        }
        stats.max_advance = stats.max_advance.max(p.y * lay.advance);
    }
    stats.foothold = foothold;
    for (w, at, rot) in dragged_to {
        if let Ok((_, _, mut tf, _)) = q.get_mut(w) {
            tf.translation = at;
            tf.rotation = rot;
        }
    }
}

/// What a man's step does to the ground: now and then a footprint, and an anti-personnel mine (among the anti-tank
/// ones) set off when it finds one.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct Footfall<'w, 's> {
    paint: ResMut<'w, PaintQueue>,
    mf: ResMut<'w, Minefield>,
    fx: ResMut<'w, FxQueue>,
    sound: Sound<'w, 's>,
    blasts: MessageWriter<'w, Blast>,
    near: Local<'s, Vec<usize>>,
}

impl Footfall<'_, '_> {
    /// A foot comes down at `p`: if it finds an anti-personnel mine, the mine goes off.
    fn step(&mut self, p: Vec2) {
        let mf = &mut self.mf;
        mf.near_into(p, 0.6, &mut self.near);
        let Some(i) = self.near.iter().copied().find(|&i| mf.mines[i].ap && mf.mines[i].pos.distance(p) < 0.45) else { return };
        mf.mines[i].live = false;
        let at = geo::ground(p);
        self.fx.explosion(at, 0.35, true);
        self.sound.at(at, Sfx::MineBlast, 0.5);
        self.blasts.write(Blast { at, radius: 2.5, cause: Cause::Mine });
    }
}

/// Men in the field climb onto a Bradley that has stopped near them, or is backing out slowly: the squad it
/// dropped, a crew whose vehicle burned, anyone. "A Bradley crew pops smoke and covers the dismounts as they
/// climb onto it" (dossier c09 §6).
pub(super) fn board_vehicles(mut commands: Commands, mut stats: ResMut<Stats>, mut vehicles: Query<(&mut Vehicle, &Driver, &Transform, Has<Rescue>), Without<Coasting>>, soldiers: Query<(Entity, &Soldier, &Transform)>) {
    let mut taken: Vec<Entity> = Vec::new();
    for (mut v, d, tf, rescue) in &mut vehicles {
        if v.kind != VKind::Bradley || d.speed > 2.0 || v.state == VState::Dismounting || v.troops >= 7 {
            continue;
        }
        // A squad that has just got out does not get straight back in, unless the vehicle is leaving.
        if !(rescue || v.state == VState::Withdrawing || v.state == VState::Overwatch) {
            continue;
        }
        let p = tf.translation.xz();
        // The wounded first, then whoever else is falling back.
        let order = soldiers.iter().filter(|(_, s, _)| s.state == SState::Wounded).chain(soldiers.iter().filter(|(_, s, _)| s.state != SState::Wounded));
        for (e, s, st) in order {
            if s.side != v.side || taken.contains(&e) || v.troops >= 7 {
                continue;
            }
            let fleeing = matches!(s.state, SState::Flee | SState::Prone | SState::Wounded);
            if fleeing && st.translation.xz().distance(p) < 14.0 {
                taken.push(e);
                commands.entity(e).despawn();
                v.troops += 1;
                stats.picked_up += 1;
                if s.state == SState::Wounded {
                    stats.wounded_out += 1;
                }
            }
        }
    }
}

pub(super) fn animate_legs(soldiers: Query<(&Soldier, &Children)>, mut legs: Query<(&Leg, &mut Transform)>) {
    for (s, children) in &soldiers {
        let moving = matches!(s.state, SState::Advance | SState::Flee);
        for c in children.iter() {
            if let Ok((leg, mut tf)) = legs.get_mut(c) {
                let a = if moving { (s.phase + leg.0).sin() * 0.6 } else { 0.0 };
                // Standing legs stay as they are: a write would send every man's legs through transform
                // propagation and extraction each frame.
                let r = Quat::from_rotation_x(a);
                if tf.rotation != r {
                    tf.rotation = r;
                }
            }
        }
    }
}

pub(super) fn fly_debris(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Debris, &mut Transform)>) {
    let dt = crate::sim_dt(&time);
    for (e, mut d, mut tf) in &mut q {
        d.vel.y -= 9.81 * dt;
        tf.translation += d.vel * dt;
        let spin = d.spin * dt;
        tf.rotate(Quat::from_euler(EulerRot::XYZ, spin.x, spin.y, spin.z));
        let g = geo::height(tf.translation.x, tf.translation.z);
        if tf.translation.y < g + 0.3 && d.vel.y < 0.0 {
            tf.translation.y = g + 0.3;
            commands.entity(e).remove::<Debris>();
        }
    }
}

/// Riders get off when the section dismounts or bails out, and are thrown off when the vehicle is hit.
pub(crate) fn drop_riders(mut commands: Commands, vehicles: Query<(&Vehicle, Has<Coasting>)>, riders: Query<(Entity, &ChildOf), With<Rider>>) {
    for (rider, hull) in &riders {
        if let Ok((v, knocked_out)) = vehicles.get(hull.parent())
            && (v.troops == 0 || knocked_out)
        {
            commands.entity(rider).despawn();
        }
    }
}
