//! The heavy bomber: a Vampire's sortie from the drone team's position, flown by the player from its camera or by
//! itself to the script's point, its 60 mm bombs or its bundle let go, and home.

use super::*;
use crate::diorama::diorama;
use crate::geo;
use crate::sound::Sfx;
use crate::units::{Blast, Cause, Dress, Part, SState, Side, Soldier};

/// The bomber's model, built by the look where the day has one.
#[derive(Resource)]
pub struct BomberModel(pub Vec<Part>);

/// The bomber in the air: where, how fast, which way, what it carries and where to, and the hand on it.
#[derive(Component)]
pub struct Vampire {
    pub pos: Vec3,
    pub vel: Vec3,
    /// Heading (0 north, left positive) and the camera's tilt under the belly.
    pub yaw: f32,
    pub gimbal: f32,
    pub load: Load,
    /// Bombs (or the bundle) still on it.
    pub left: u32,
    /// The script's point: it flies there by itself and drops (none: the player's sortie).
    pub target: Option<Vec2>,
    /// On its way home, by the player's word (H), an empty rack, a low battery or a lost link.
    pub home: bool,
    /// Seconds of battery left.
    pub battery: f32,
    /// The control link, and how long it has been out; their EW at it, 0..1.
    pub control: bool,
    pub lost_for: f32,
    pub jam: f32,
    /// The player flies it (from its camera): the sticks this frame, forward, right, climb and turn (left
    /// positive), each -1..1.
    pub held: bool,
    pub stick: [f32; 4],
    /// C this frame.
    pub release: bool,
    next_release: f64,
}

/// A bomb, or a bundle, on its way down from the bomber.
#[derive(Component)]
pub struct Bomb {
    pub vel: Vec3,
    pub load: Load,
}

/// The height it cruises at over the ground, and its speed (m/s): a Vampire loaded is no Mavic.
const CRUISE_AGL: f32 = 100.0;
const SPEED: f32 = 14.0;
/// A 60 mm mortar bomb's burst: a good deal more than a VOG's (`drones`), lethal to about 15 m surely only at the
/// point.
const BOMB_BLAST: f32 = 5.0;

/// Launch the sorties asked for, one at a time, while there are any in it.
pub(super) fn launch_sorties(mut commands: Commands, time: Res<Time>, mut robots: ResMut<Robots>, mut radio: ResMut<Radio>, op: Res<crate::player::Operator>, model: Option<Res<BomberModel>>, flying: Query<(), With<Vampire>>) {
    let now = time.elapsed_secs_f64();
    let Some(spec) = spec().and_then(|s| s.bomber) else {
        robots.orders.retain(|o| !matches!(o, Order::Sortie { .. }));
        return;
    };
    let (sorties, rest): (Vec<Order>, Vec<Order>) = std::mem::take(&mut robots.orders).into_iter().partition(|o| matches!(o, Order::Sortie { .. }));
    robots.orders = rest;
    let mut up = !flying.is_empty();
    // The script's sorties kept back while the bomber was up or the team moving go first, in their order.
    let kept = std::mem::take(&mut robots.queued);
    let n_kept = kept.len();
    for (i, o) in kept.into_iter().chain(sorties).enumerate() {
        let Order::Sortie { to, load, flown } = o else { continue };
        if robots.sorties_left == 0 {
            radio.say2(now, "Вампіра більше не піднімемо сьогодні.", "The Vampire has no more sorties in it today.");
            continue;
        }
        // The script's sortie waits for the bomber to be down and the team set up again; the player's is told.
        let busy = up || now < op.moving_until;
        if busy && to.is_some() && !flown {
            if i >= n_kept {
                info!("bomber: a sortie to {to:?} kept until the {}, at {now:.0} s", if up { "Vampire is down" } else { "team is set up" });
                if up {
                    radio.say2(now, "Наступний виліт Вампіра, коли сяде.", "The Vampire's next sortie goes when it's down.");
                } else {
                    radio.say2(now, "Наступний виліт Вампіра, коли група стане.", "The Vampire's next sortie goes when the team is set up.");
                }
            }
            robots.queued.push(o);
            continue;
        }
        if up {
            radio.say2(now, "Вампір уже в повітрі.", "The Vampire is already up.");
            continue;
        }
        if now < op.moving_until {
            radio.say2(now, "Не зараз: група на переміщенні.", "Not now: the team is moving.");
            continue;
        }
        robots.sorties_left -= 1;
        up = true;
        let pos = geo::ground(op.pos) + Vec3::Y * 1.0;
        let left = if load == Load::Bombs { 4 } else { 1 };
        let v = Vampire { pos, vel: Vec3::ZERO, yaw: std::f32::consts::PI, gimbal: -0.9, load, left, target: to, home: false, battery: spec.endurance, control: true, lost_for: 0.0, jam: 0.0, held: flown, stick: [0.0; 4], release: false, next_release: 0.0 };
        let mut e = commands.spawn((Transform::from_translation(pos), v));
        if let Some(m) = model.as_deref() {
            e.insert(crate::units::skin(&m.0));
            crate::units::dress(&mut e, &m.0);
        }
        let what = match load {
            Load::Bombs => ("чотири 60-ки", "four 60 mm bombs"),
            Load::Bundle(c) => (c.name().0, c.name().1),
        };
        info!("bomber: up from {:.0},{:.0} with {}, {} to {:?}, {} sorties left, at {now:.0} s", op.pos.x, op.pos.y, what.1, if flown { "flown" } else { "by itself" }, to, robots.sorties_left);
        radio.say2(now, format!("Вампір у повітрі: {}.", what.0), format!("Vampire up with {}.", what.1));
    }
}

/// Fly it: the player's sticks while he holds it, else by itself to its point, over it to drop, and home; the link
/// frays in their EW as the drone's does, and a lost link, an empty rack or a low battery bring it home.
pub(super) fn fly(
    mut commands: Commands,
    time: Res<Time>,
    ew: Res<crate::ew::Ew>,
    dress: Dress,
    mut station: Station,
    soldiers: Query<(&Soldier, &Transform)>,
    mut q: Query<(Entity, &mut Vampire, &mut Transform), Without<Soldier>>,
) {
    let now = time.elapsed_secs_f64();
    let dt = crate::sim_dt(&time);
    let t = time.elapsed_secs();
    let endurance = spec().and_then(|s| s.bomber).map_or(600.0, |b| b.endurance);
    for (e, mut v, mut tf) in &mut q {
        v.battery -= dt;
        v.jam = ew.jam_at(v.pos);
        let jam = v.jam;
        v.control = jam < 0.55 || geo::value_noise(t * 0.7, 13.0, 65) > (jam - 0.35) * 1.4;
        v.lost_for = if v.control { 0.0 } else { v.lost_for + dt };
        if !v.home && (v.battery < endurance * 0.2 || v.lost_for > 6.0) {
            v.home = true;
            let (uk, en) = if v.lost_for > 6.0 { ("Вампір без зв'язку, повертається сам.", "No link to the Vampire. It's coming home by itself.") } else { ("Батарея на Вампірі сідає, повертаю.", "The Vampire's battery is low. Bringing it home.") };
            station.radio.say2(now, uk, en);
        }
        let ground = geo::height(v.pos.x, v.pos.z);
        let agl = v.pos.y - ground;
        let fwd = Vec3::new(-v.yaw.sin(), 0.0, -v.yaw.cos());
        let right = Vec3::new(v.yaw.cos(), 0.0, -v.yaw.sin());
        let horizontal = Vec3::new(v.vel.x, 0.0, v.vel.z);
        let manual = v.held && v.control && !v.home;
        // Where it wants to go, and how fast it climbs.
        let (desired, climb, turn_to) = if manual {
            let [f, r, c, y] = v.stick;
            v.yaw += y * dt * 0.8;
            // Off the ground it goes up to 20 m by itself before the sticks have it.
            let climb = if agl < 20.0 { (c * 4.0).max(2.0) } else { c * 4.0 };
            ((fwd * f + right * r).normalize_or_zero() * SPEED, climb, None)
        } else {
            let aim = if v.home {
                Some(station.op.pos)
            } else {
                v.target.map(|t| {
                    // The men nearest its point, then upwind of them by the drift of the mean wind, as the drop does.
                    let at = if v.load == Load::Bombs {
                        let attacker = diorama().attacker();
                        soldiers.iter().filter(|(s, st)| s.side == attacker && s.state != SState::Wounded && st.translation.xz().distance(t) < 150.0).map(|(_, st)| st.translation.xz()).min_by(|a, b| a.distance(t).total_cmp(&b.distance(t))).unwrap_or(t)
                    } else {
                        t
                    };
                    let over = geo::ground(at) + Vec3::Y * CRUISE_AGL;
                    at + (at - crate::drones::landing(over))
                })
            };
            match aim {
                Some(a) => {
                    let to = a - v.pos.xz();
                    let d = to.length();
                    let landing = v.home && d < 12.0;
                    let climb = if landing { -2.0 } else { (CRUISE_AGL - agl).clamp(-3.0, 3.0) };
                    // Over its point, it holds and lets go one at a time.
                    if !v.home && d < 6.0 && horizontal.length() < 2.5 && v.left > 0 && now >= v.next_release {
                        v.release = true;
                    }
                    (Vec3::new(to.x, 0.0, to.y).normalize_or_zero() * (d * 0.5).min(SPEED), climb, (d > 20.0).then_some(to))
                }
                // Nowhere to go (the player's sortie, left to itself): it climbs to its height and holds there.
                None => (Vec3::ZERO, (CRUISE_AGL - agl).clamp(-3.0, 3.0), None),
            }
        };
        if let Some(to) = turn_to {
            let want = crate::units::vehicle_yaw(to);
            v.yaw += crate::units::wrap(want - v.yaw).clamp(-0.8 * dt, 0.8 * dt);
        }
        // Out of battery, it falls.
        let dead = v.battery <= 0.0;
        let accel = if dead { Vec3::ZERO } else { ((desired - horizontal) * 1.3).clamp_length_max(4.0) };
        let mut vel = horizontal + accel * dt;
        vel.y = if dead { v.vel.y - 9.81 * dt } else { v.vel.y + (climb - v.vel.y).clamp(-2.0 * dt, 2.0 * dt) };
        v.vel = vel;
        v.pos += vel * dt;
        v.pos.x = v.pos.x.clamp(-geo::HALF_X + 30.0, geo::HALF_X - 30.0);
        v.pos.z = v.pos.z.clamp(-geo::HALF_Z + 30.0, geo::HALF_Z - 30.0);
        let ground = geo::height(v.pos.x, v.pos.z);
        // Home and down: the sortie is over.
        if v.home && v.pos.y - ground < 1.5 && v.pos.xz().distance(station.op.pos) < 15.0 {
            station.robots.sorties += 1;
            info!("bomber: home with {} aboard, at {now:.0} s", v.left);
            station.radio.say2(now, "Вампір сів.", "The Vampire is down, home.");
            commands.entity(e).despawn();
            if station.view.mode == Mode::Bomber {
                station.view.mode = Mode::Drone;
            }
            continue;
        }
        if !dead {
            v.pos.y = v.pos.y.max(ground + 0.5);
        }
        if v.pos.y <= ground {
            info!("bomber: down at {:.0},{:.0} (battery {:.0} s), at {now:.0} s", v.pos.x, v.pos.z, v.battery);
            station.radio.say2(now, "Вампіра втратили.", "We've lost the Vampire.");
            commands.entity(e).despawn();
            if station.view.mode == Mode::Bomber {
                station.view.mode = Mode::Drone;
            }
            continue;
        }
        if std::mem::take(&mut v.release) && v.left > 0 && now >= v.next_release {
            v.left -= 1;
            v.next_release = now + 1.2;
            let (from, vel, load) = (v.pos - Vec3::Y * 0.4, v.vel, v.load);
            let scale = if load == Load::Bombs { Vec3::new(0.9, 0.9, 0.35) } else { Vec3::new(3.0, 3.0, 0.4) };
            commands.spawn((
                Mesh3d(dress.models.missile.clone()),
                MeshMaterial3d(dress.palette.vertex.clone()),
                Transform::from_translation(from).looking_to(Vec3::NEG_Y, Vec3::Z).with_scale(scale),
                Bomb { vel, load },
                bevy::light::NotShadowCaster,
            ));
            match load {
                Load::Bombs => station.robots.bombs += 1,
                Load::Bundle(_) => station.robots.bundles += 1,
            }
            info!("bomber: released over {:.0},{:.0} from {:.0} m, {} left, at {now:.0} s", from.x, from.z, from.y - ground, v.left);
            station.radio.say2(now, "Скид з Вампіра.", "Drop from the Vampire.");
            if v.left == 0 && !v.home {
                v.home = true;
                station.radio.say2(now, "Пусто. Вампір додому.", "Rack's empty. The Vampire is coming home.");
            }
        }
        // Its attitude: nose into the way it is going, a little tilt for its acceleration.
        tf.translation = v.pos;
        tf.rotation = Quat::from_rotation_y(v.yaw) * Quat::from_rotation_x(-accel.dot(fwd) * 0.03) * Quat::from_rotation_z(accel.dot(right) * 0.03);
    }
}

/// The bombs fall in the day's wind as the drop's grenades do, and burst; a bundle lands, and the post nearest it
/// within 40 m takes it.
pub(super) fn bombs_fall(
    mut commands: Commands,
    time: Res<Time>,
    mut burst: crate::drones::Burst,
    mut station: Station,
    mut posts: Query<(&crate::fire::Post, Option<&mut crate::fight::Ammo>)>,
    mut q: Query<(Entity, &mut Bomb, &mut Transform)>,
    soldiers: Query<(&Soldier, &Transform), Without<Bomb>>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    for (e, mut b, mut tf) in &mut q {
        let mut p = tf.translation;
        crate::drones::fall_step(&mut p, &mut b.vel, dt, Some(now));
        let ground = geo::height(p.x, p.z);
        if p.y > ground {
            tf.translation = p;
            tf.look_to(b.vel.normalize_or(Vec3::NEG_Y), Vec3::Y);
            continue;
        }
        commands.entity(e).despawn();
        let at = Vec3::new(p.x, ground, p.z);
        match b.load {
            Load::Bombs => {
                burst.fx.explosion(at, 0.5, true);
                burst.sound.at(at, Sfx::BlastNear, 0.9);
                burst.blasts.write(Blast { at, radius: BOMB_BLAST, cause: Cause::Drop });
                let attacker = diorama().attacker();
                let near: Vec<f32> = soldiers.iter().filter(|(s, _)| s.side == attacker && s.state != SState::Wounded).map(|(_, t)| t.translation.xz().distance(at.xz())).collect();
                let among = near.iter().filter(|&&d| d < BOMB_BLAST * 3.0).count();
                let closest = near.iter().copied().fold(f32::MAX, f32::min);
                let (what, result) = match among {
                    0 if closest < 60.0 => ("the ground", format!("missed: {closest:.0} m from the nearest man")),
                    0 => ("the ground", "missed: nobody near".to_string()),
                    1 => ("a man in the open", "burst beside him".to_string()),
                    n => ("men in the open", format!("burst among {n}")),
                };
                burst.log.note(now, "60 mm bomb", "the Vampire", what, attacker, result);
                info!("bomber: a bomb burst at {:.0},{:.0}, {among} men within {:.0} m, the nearest {closest:.0} m, at {now:.0} s", at.x, at.z, BOMB_BLAST * 3.0);
            }
            Load::Bundle(c) => {
                let post = posts.iter().filter(|(p, _)| p.side == Side::Ua && p.strength > 0).map(|(p, _)| p.pos).filter(|q| q.distance(at.xz()) < 40.0).min_by(|a, b| a.distance(at.xz()).total_cmp(&b.distance(at.xz())));
                match post {
                    Some(q) => {
                        station.deliver(c, q, &mut posts);
                        info!("bomber: the bundle of {} landed {:.0} m from the post at {:.0},{:.0}, at {now:.0} s", c.name().1, q.distance(at.xz()), q.x, q.y);
                        station.radio.say2(now, format!("Скид прийняли: {}.", c.name().0), format!("Bundle in at the post: {}.", c.name().1));
                    }
                    None => {
                        info!("bomber: the bundle of {} landed at {:.0},{:.0}, nobody's, at {now:.0} s", c.name().1, at.x, at.z);
                        station.radio.say2(now, "Скид упав мимо. Не дістати.", "The bundle came down wide. Nobody can get to it.");
                    }
                }
            }
        }
    }
}
