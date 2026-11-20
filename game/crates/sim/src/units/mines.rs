//! Mines: the minefields laid at the start, and what goes off under a track.

use super::*;

#[derive(Clone, Copy)]
pub struct Mine {
    pub pos: Vec2,
    pub live: bool,
    pub entity: Option<Entity>,
    /// Two anti-tank mines one on the other: no plow or roller survives it.
    pub stacked: bool,
    /// Anti-personnel: goes off under a foot, not under a track (a track sets it off harmlessly).
    pub ap: bool,
}

impl Mine {
    pub fn at(pos: Vec2) -> Self {
        Self { pos, live: true, entity: None, stacked: false, ap: false }
    }
}

#[derive(Resource, Default)]
pub struct Minefield {
    pub mines: Vec<Mine>,
    grid: HashMap<(i32, i32), Vec<usize>>,
}

const MINE_CELL: f32 = 8.0;

impl Minefield {
    pub fn add(&mut self, m: Mine) {
        let i = self.mines.len();
        self.mines.push(m);
        let c = ((m.pos.x / MINE_CELL).floor() as i32, (m.pos.y / MINE_CELL).floor() as i32);
        self.grid.entry(c).or_default().push(i);
    }

    /// The live mines in the grid cells within `r` of `p`, into `out`, which is cleared first so a caller can keep
    /// one buffer across calls.
    pub fn near_into(&self, p: Vec2, r: f32, out: &mut Vec<usize>) {
        out.clear();
        let (x0, x1) = (((p.x - r) / MINE_CELL).floor() as i32, ((p.x + r) / MINE_CELL).floor() as i32);
        let (z0, z1) = (((p.y - r) / MINE_CELL).floor() as i32, ((p.y + r) / MINE_CELL).floor() as i32);
        for cz in z0..=z1 {
            for cx in x0..=x1 {
                if let Some(v) = self.grid.get(&(cx, cz)) {
                    out.extend(v.iter().copied().filter(|&i| self.mines[i].live));
                }
            }
        }
    }
}

pub(super) fn lay_minefields(mut commands: Commands) {
    let lay = geo::layout();
    let mut mf = Minefield::default();
    let mut rng = SmallRng::seed_from_u64(crate::dev::seeded(72));
    for f in &lay.shoulder_fields {
        let road = &lay.roads[f.road].line;
        let area = (f.s1 - f.s0) * (f.outer - f.inner) * 2.0;
        let n = (area * f.density) as usize;
        for _ in 0..n {
            let s = rng.random_range(f.s0..f.s1);
            let lat = sign(&mut rng) * rng.random_range(f.inner..f.outer);
            let p = road.point(s) + road.normal(s) * lat;
            mf.add(Mine::at(p));
        }
    }
    for (line, width) in &lay.field_mines {
        let n = (line.length() * width / 60.0) as usize;
        for _ in 0..n {
            let s = rng.random_range(0.0..line.length());
            let p = line.point(s) + line.normal(s) * rng.random_range(-width * 0.5..width * 0.5);
            // Lanes left open where the roads cross.
            if lay.roads.iter().any(|r| r.line.project(p).0 < 7.0) {
                continue;
            }
            mf.add(Mine::at(p));
        }
    }
    for b in &lay.barriers {
        let area = 2.0 * geo::HALF_X * (b.z1 - b.z0);
        for _ in 0..(area * b.at) as usize {
            let p = Vec2::new(rng.random_range(-geo::HALF_X..geo::HALF_X), rng.random_range(b.z0..b.z1));
            mf.add(Mine { stacked: rng.random_bool(b.stacked as f64), ..Mine::at(p) });
        }
        for _ in 0..(area * b.ap) as usize {
            let p = Vec2::new(rng.random_range(-geo::HALF_X..geo::HALF_X), rng.random_range(b.z0..b.z1));
            mf.add(Mine { ap: true, ..Mine::at(p) });
        }
    }
    info!("minefields: {} mines", mf.mines.len());
    commands.insert_resource(mf);
}

/// A vehicle as the mines and the hits find it.
#[derive(bevy::ecs::query::QueryData)]
#[query_data(mutable)]
pub(super) struct Hulls {
    e: Entity,
    v: &'static mut Vehicle,
    d: &'static mut Driver,
    sus: &'static mut Suspension,
    tf: &'static Transform,
    plow: Has<Plow>,
    rescue: Has<Rescue>,
}

/// The anti-tank mines a hull drives onto: the minefield, a scratch list of the mines near, and what one going off
/// makes.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct TrackMines<'w, 's> {
    field: ResMut<'w, Minefield>,
    near: Local<'s, Vec<usize>>,
    blast: MineBlast<'w, 's>,
}

pub(super) fn check_mines(
    mut commands: Commands,
    time: Res<Time>,
    dress: Dress,
    mut mines: TrackMines,
    mut record: Record,
    mut rng: ResMut<UnitRng>,
    mut q: Query<Hulls, Without<Coasting>>,
) {
    let now = time.elapsed_secs_f64();
    for HullsItem { e: _e, mut v, d, mut sus, tf, plow: has_plow, rescue } in &mut q {
        if d.speed < 0.2 {
            continue;
        }
        let mut plow = has_plow;
        let p = tf.translation.xz();
        let fwd3 = tf.rotation * Vec3::NEG_Z;
        let fwd = fwd3.xz().normalize_or_zero();
        let right = Vec2::new(-fwd.y, fwd.x);
        mines.field.near_into(p, 6.0, &mut mines.near);
        for &i in mines.near.iter() {
            let m = mines.field.mines[i];
            if m.ap {
                continue;
            }
            let off = m.pos - p;
            let (f, x) = (off.dot(fwd), off.dot(right));
            // The plow's teeth turn mines out of the ground ahead of the tracks; now and then one goes off.
            if plow && (-4.7..-3.6).contains(&-f) && x.abs() < 1.9 {
                mines.field.mines[i].live = false;
                if let Some(ent) = m.entity {
                    commands.entity(ent).despawn();
                }
                // A stacked pair wrecks the plow or rollers outright (a 47th T-64 commander: rollers survive "up to
                // four explosions", but stacked mines destroyed one after a single blast).
                if rng.0.random_bool(if m.stacked { 0.45 } else { 0.12 }) {
                    mines.blast.detonate(m.pos);
                    plow = false;
                    commands.entity(_e).remove::<Plow>();
                    if m.stacked && rng.0.random_bool(0.5) {
                        let coasting = v.immobilise(rng.0.random_range(3.0..5.0), rng.0.random_range(-0.3..0.3));
                        commands.entity(_e).insert(coasting);
                        v.crew_plan(&mut rng.0, Cause::Mine);
                        sus.pitch_v += rng.0.random_range(2.0..3.0);
                        record.stats.mine_hits += 1;
                        record.log.open(now, "a double mine", "", Victim { target: _e, vehicle: &v, rescue }, "under the plow: plow gone, stopped");
                        record.radio.say(now, "Double mine under the plow. Plow's gone, he's not moving. The lane stops here.");
                    } else if let Some(line) = &bav_core::data::data().words.plow_mine {
                        record.radio.say(now, line.as_str());
                    }
                }
                continue;
            }
            if f.abs() < 3.3 && (x.abs() - 1.4).abs() < 0.5 {
                mines.field.mines[i].live = false;
                if let Some(ent) = m.entity {
                    commands.entity(ent).despawn();
                }
                mines.blast.detonate(m.pos);
                record.stats.mine_hits += 1;
                // TM-62 under a track: the track and road wheels are gone. Often more.
                let kill = rng.0.random_bool(v.kind.mine_kill(m.stacked));
                record.log.open(now, if m.stacked { "a double mine" } else { "a mine" }, "", Victim { target: _e, vehicle: &v, rescue }, if kill { "under the track: destroyed" } else { "under the track: track off, stopped" });
                // The track is off on that side: it lurches, yaws round the dead side and grinds to a stop within
                // a length or two; the cut track lies out behind it, a road wheel beside it.
                let coasting = v.immobilise(rng.0.random_range(2.0..3.5), x.signum() * -rng.0.random_range(0.35..0.8));
                commands.entity(_e).insert(coasting);
                v.crew_plan(&mut rng.0, Cause::Mine);
                sus.pitch_v += rng.0.random_range(1.8..3.0);
                sus.roll_v += x.signum() * rng.0.random_range(1.5..2.5);
                let side = right * x.signum();
                let yaw = d.yaw;
                for (mesh, at, spin) in [(&dress.models.track_strip, p - fwd * 4.5 + side * 1.4, 0.0), (&dress.models.road_wheel, p + side * 2.4 - fwd * rng.0.random_range(0.5..2.5), rng.0.random_range(-0.4..0.4))] {
                    commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(dress.palette.vertex.clone()), Transform::from_translation(geo::ground(at)).with_rotation(ground_rotation(at, yaw + spin)), NotShadowCaster));
                }
                if kill {
                    if rng.0.random_bool(0.15) {
                        // The blast starts a fire in the engine bay instead: the crew gets out, and whether it
                        // burns down to a wreck is decided over the next minutes.
                        commands.entity(_e).insert(Fire::deck(&mut rng.0, Cause::Mine, v.kind));
                        commands.entity(_e).insert(Emitter::new(EmitterKind::EngineSmoke, 4.0, 400.0, Vec3::new(0.0, 1.5, 2.2)));
                    } else {
                        commands.trigger(Wrecked { vehicle: _e, cause: Cause::Mine, how: kill_how(v.kind, Cause::Mine, &mut rng.0) });
                    }
                }
                let lines = &bav_core::data::data().words.mine_lines;
                if rng.0.random_bool(0.6) && !lines.is_empty() {
                    record.radio.say(now, lines[rng.0.random_range(0..lines.len())].as_str());
                }
                break;
            }
        }
    }
}

/// What an anti-tank mine going off makes: the flash, the crater (seeded from the effects' stream), the sound and
/// the blast.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct MineBlast<'w, 's> {
    fx: ResMut<'w, FxQueue>,
    fxr: ResMut<'w, FxRng>,
    paint: ResMut<'w, PaintQueue>,
    sound: Sound<'w, 's>,
    blasts: MessageWriter<'w, Blast>,
}

impl MineBlast<'_, '_> {
    fn detonate(&mut self, at: Vec2) {
        let p = geo::ground(at);
        self.fx.explosion(p, 1.1, true);
        self.paint.push(Paint::Crater { at, radius: 2.0, seed: self.fxr.0.random() });
        self.sound.at(p, Sfx::MineBlast, 1.0);
        self.blasts.write(Blast { at: p, radius: 6.0, cause: Cause::Mine });
    }
}
