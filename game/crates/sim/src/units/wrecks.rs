//! Wrecks and fire: a vehicle destroyed, charred and set burning, its turret thrown, its fire through its phases.

use super::*;

/// A vehicle has just been destroyed: char it, set it burning, maybe throw its turret.
#[derive(Event, Clone, Copy)]
pub struct Wrecked {
    pub vehicle: Entity,
    pub cause: Cause,
    pub how: Kill,
}

/// How a vehicle dies (§2.5 of design/next-items.md).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kill {
    /// The hit reached the charges: the fireball and the turret now.
    BrewUp,
    /// It burns; the charges go over the next minutes, and a carousel tank may throw its turret on one of them.
    Torch,
    /// A fire that grew into the fighting compartment: no blast, just the torch from here.
    Quiet,
}

/// Which death a kill is: Soviet hulls brew up at once when the hit reaches the carousel (an ATGM or a tank
/// round more often than a mine); Western hulls and the MT-LB burn instead.
pub(super) fn kill_how(kind: VKind, cause: Cause, rng: &mut SmallRng) -> Kill {
    if kind.western() || kind == VKind::Mtlb {
        return Kill::Torch;
    }
    let p_brew = match cause {
        Cause::Atgm | Cause::TankGun => 0.4,
        Cause::Mine => 0.25,
        _ => 0.3,
    };
    if rng.random_bool(p_brew) { Kill::BrewUp } else { Kill::Torch }
}

/// How a knocked-out vehicle burns, from the first smoke to the next day's wisps.
#[derive(Component)]
pub struct Fire {
    pub phase: Blaze,
    pub cause: Cause,
    pub t: f32,
    /// Deck: when the fire reaches the fighting compartment (None: it dies down instead).
    pub escalate_at: Option<f32>,
    /// Torch: when the next charges go, how many bursts are left, and when the turret goes, if it does.
    pub next_burst: f32,
    pub bursts: u32,
    pub toss_at: Option<f32>,
    /// Torch: how long until it burns down to a smoulder.
    pub torch_secs: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Blaze {
    /// A fire in the engine bay: white smoke, then grey; it may grow or die.
    Deck,
    /// The whole hull.
    Torch,
    /// The next day's wisps.
    Smoulder,
}

impl Fire {
    /// An engine-bay fire: six in ten grow into the hull within a minute to three.
    pub(super) fn deck(rng: &mut SmallRng, cause: Cause, _kind: VKind) -> Self {
        Self { phase: Blaze::Deck, cause, t: 0.0, escalate_at: rng.random_bool(0.6).then(|| rng.random_range(60.0..180.0)), next_burst: 0.0, bursts: 0, toss_at: None, torch_secs: 0.0 }
    }

    /// The hull burning: a carousel tank's charges go 4–10 times over the next minutes and may take the turret
    /// with them; a BMP's a few times; a Leopard vents once through its blow-out panel; a Bradley's rounds
    /// crackle; an MT-LB has nothing to burst.
    fn torch(rng: &mut SmallRng, cause: Cause, kind: VKind, toss: bool, torch_secs: f32) -> Self {
        let bursts = match kind {
            VKind::T80 | VKind::T72 => rng.random_range(4..=10),
            VKind::Bmp1 | VKind::Bmp2 => rng.random_range(3..=6),
            VKind::Leo2a6 => 1,
            VKind::Bradley => rng.random_range(2..=4),
            _ => 0,
        };
        let toss_at = (toss && kind.carousel()).then(|| rng.random_range(20.0..300.0));
        Self { phase: Blaze::Torch, cause, t: 0.0, escalate_at: None, next_burst: rng.random_range(5.0..40.0), bursts: bursts.max(toss_at.is_some() as u32), toss_at, torch_secs }
    }
}

/// How hard a kind's charges go off when they cook.
fn burst_strength(kind: VKind) -> f32 {
    match kind {
        VKind::T80 | VKind::T72 => 1.0,
        VKind::Bmp1 | VKind::Bmp2 => 0.8,
        VKind::Leo2a6 => 0.6,
        _ => 0.3,
    }
}

/// The charges go all at once: the fireball, its sound, and the blast on whoever stands near.
fn cook_off_blast(fx: &mut FxQueue, sound: &mut Sound, blasts: &mut MessageWriter<Blast>, p: Vec3) {
    fx.fireball(p);
    sound.at(p, Sfx::CookOff, 1.4);
    blasts.write(Blast { at: p, radius: 8.0, cause: Cause::CookOff });
}

/// The turret goes up: typically a few metres, down beside the hull, inverted; one in ten to fifteen or thirty.
fn toss_turret(commands: &mut Commands, rng: &mut SmallRng, stats: &mut Stats, radio: &mut Radio, v: &Vehicle, turret: &GlobalTransform, now: f64) {
    stats.turret_tosses += 1;
    let h = if rng.random_bool(0.1) { rng.random_range(15.0..30.0) } else { rng.random_range(2.0..8.0) };
    let vy = (2.0 * 9.81 * h as f32).sqrt();
    let a = rng.random_range(0.0..std::f32::consts::TAU);
    let side = rng.random_range(1.0..4.0);
    let world = turret.compute_transform();
    commands.entity(v.turret).remove::<ChildOf>().insert((
        Transform::from_translation(world.translation).with_rotation(world.rotation),
        Debris { vel: Vec3::new(a.cos() * side, vy, a.sin() * side), spin: Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(-2.0..2.0), rng.random_range(-3.0..3.0)) },
    ));
    if rng.random_bool(0.6) {
        radio.say(now, "Turret's off. Jack in the box.");
    }
}

/// A wrecked vehicle: its skin to char, and whether it was the plow or a rescue vehicle.
type WreckQuery<'w, 's> = Query<'w, 's, (&'static mut Vehicle, &'static mut Suspension, &'static Transform, &'static mut MeshMaterial3d<VehicleMaterial>, Has<Rescue>, Has<Plow>), Without<Turret>>;

/// A vehicle's turret: the skin that chars as the wreck burns, and where it sits, for when the charges throw it.
type TurretMats<'w, 's> = Query<
    'w,
    's,
    (&'static mut MeshMaterial3d<VehicleMaterial>, &'static GlobalTransform),
    (With<Turret>, Without<Skin>),
>;

/// A wreck as it is made one: its hull, the turret of every vehicle (for charring its skin and throwing it when the
/// charges go), each part's skin and the children that hold them, and the ground it scorches.
#[derive(bevy::ecs::system::SystemParam)]
pub struct Wreck<'w, 's> {
    hulls: WreckQuery<'w, 's>,
    turrets: TurretMats<'w, 's>,
    children: Query<'w, 's, &'static Children>,
    skins: Query<'w, 's, (&'static Skin, &'static mut MeshMaterial3d<VehicleMaterial>), Without<Vehicle>>,
    paint: ResMut<'w, PaintQueue>,
}

/// What a burning wreck makes: the fire and smoke, their random stream, the sound, and the blasts of its
/// ammunition.
#[derive(bevy::ecs::system::SystemParam)]
pub struct WreckFire<'w, 's> {
    fx: ResMut<'w, FxQueue>,
    fxr: ResMut<'w, FxRng>,
    sound: Sound<'w, 's>,
    blasts: MessageWriter<'w, Blast>,
}

pub fn on_wrecked(
    ev: On<Wrecked>,
    mut commands: Commands,
    time: Res<Time>,
    dress: Dress,
    mut fire: WreckFire,
    mut record: Record,
    mut wreck: Wreck,
) {
    let Ok((mut v, mut sus, tf, mut mat, rescue, plow)) = wreck.hulls.get_mut(ev.vehicle) else { return };
    // Several blasts can claim a wreck in one frame; the first one counts.
    if v.state == VState::Destroyed {
        return;
    }
    let now = time.elapsed_secs_f64();
    if !record.log.add(now, ev.vehicle, crate::hits::fate(ev.how)) {
        let weapon = match ev.cause {
            Cause::Mine => "a mine",
            Cause::Artillery if v.side == Side::Ru => "our guns",
            Cause::Artillery => "their guns",
            Cause::Himars => "HIMARS",
            Cause::Thermobaric => "TOS-1A",
            Cause::CookOff => "a burning wreck's ammunition",
            Cause::TankGun => "a tank gun",
            Cause::Atgm => "an ATGM",
            Cause::Fpv => "an FPV",
            Cause::Autocannon | Cause::Blinding => "25 mm fire",
            Cause::Drop => "a dropped grenade",
        };
        record.log.open(now, weapon, "", Victim { target: ev.vehicle, vehicle: &v, rescue }, format!("destroyed{}", crate::hits::fate(ev.how)));
    }
    // An abandoned hull that burns now is no longer "abandoned" in the ledger.
    if v.state == VState::Abandoned {
        if v.side == diorama().attacker() {
            record.stats.abandoned -= 1;
            record.stats.wave(v.wave).abandoned -= 1;
        } else {
            record.stats.defender_vehicles -= 1;
        }
    }
    let now = time.elapsed_secs_f64();
    let rng = &mut fire.fxr.0;
    // It does not stop dead: the driver is gone, the engine dies, the hull rolls on and slews. (A vehicle already
    // immobilised, by a mine say, keeps the slew round its dead track that it already has.)
    if v.state.mobile() {
        commands.entity(ev.vehicle).insert(Coasting { decel: rng.random_range(2.5..4.0), slew: rng.random_range(-0.3..0.3) });
    }
    sus.pitch_v -= rng.random_range(0.8..1.6);
    v.state = VState::Destroyed;
    let m = dress.models.vehicle(v.kind);
    char_skins(ev.vehicle, &mut mat, if plow { m.plow_hull.as_ref().unwrap_or(&m.hull) } else { &m.hull }, &wreck.children, &mut wreck.skins);
    let attacker = v.side == diorama().attacker();
    if attacker {
        record.stats.destroyed += 1;
        record.stats.wave(v.wave).destroyed += 1;
    } else {
        record.stats.defender_vehicles += 1;
    }
    match ev.cause {
        Cause::Atgm => record.stats.atgm_kills += 1,
        Cause::Fpv => record.stats.fpv_kills += 1,
        Cause::Artillery | Cause::Himars | Cause::TankGun => record.stats.artillery_kills += 1,
        _ => {}
    }
    // Who gets out depends on how it dies. A brew-up kills everyone aboard; a hull that catches fire lets most
    // of the crew out, fewer of the desant; in a Leopard or a Bradley most get out anyway: the hatches, the
    // ramp, the ammunition behind blow-out panels (dossier c09 §6: "most crews escaped through open hatches and
    // ramps"). A fire that grew took its time: whoever was aboard is long gone.
    let dead = match ev.how {
        Kill::Quiet => 0,
        _ if v.kind.western() => rng.random_range(0..=1).min(v.crew) + (v.troops as f32 * rng.random_range(0.0..0.3)) as u32,
        Kill::BrewUp => v.crew + (v.troops as f32 * rng.random_range(0.5..0.9)) as u32,
        Kill::Torch => v.crew.min(rng.random_range(0..=(v.crew / 2).max(1))) + (v.troops as f32 * rng.random_range(0.2..0.5)) as u32,
    };
    if attacker {
        record.stats.attacker_dead += dead;
        record.stats.died("in vehicles", dead);
    } else {
        record.stats.defender_losses += dead;
    }
    v.crew -= v.crew.min(dead);
    v.troops = v.troops.saturating_sub(dead);
    let p = tf.translation;
    wreck.paint.push(Paint::Scorch { at: p.xz(), radius: 9.0 });
    // Men riding on top are thrown off: a brew-up flings them, a fire tips them into the snow.
    for _ in 0..dead.min(5) {
        let a = rng.random_range(0.0..std::f32::consts::TAU);
        let speed = if ev.how == Kill::BrewUp { rng.random_range(5.0..9.0) } else { rng.random_range(1.5..3.0) };
        throw_body(&mut commands, &dress.models, &dress.palette, v.side, p + Vec3::Y * 2.2, Vec3::new(a.cos() * speed, rng.random_range(2.0..5.0), a.sin() * speed), rng);
    }
    let turret = wreck.turrets.get_mut(v.turret).ok().map(|(mut tm, tgt)| {
        char_skins(v.turret, &mut tm, &m.turret, &wreck.children, &mut wreck.skins);
        *tgt
    });
    let torch_secs = rng.random_range(150.0..320.0);
    let toss_later = ev.how != Kill::BrewUp && v.kind.carousel() && rng.random_bool(0.5);
    match ev.how {
        Kill::BrewUp => {
            cook_off_blast(&mut fire.fx, &mut fire.sound, &mut fire.blasts, p);
            if v.kind.carousel()
                && let Some(t) = turret.as_ref()
            {
                toss_turret(&mut commands, rng, &mut record.stats, &mut record.radio, &v, t, now);
            }
        }
        Kill::Torch => {
            fire.fx.explosion(p + Vec3::Y, 0.9, false);
            fire.sound.at(p, Sfx::BlastNear, 1.0);
        }
        Kill::Quiet => {}
    }
    commands.entity(ev.vehicle).insert((Fire::torch(rng, ev.cause, v.kind, toss_later, torch_secs), Emitter::new(EmitterKind::WreckFire, 5.0, torch_secs, Vec3::Y * 1.5)));
    // Fuel running out of a holed tank burns in a pool beside the hull; in June it can set the field alight.
    fire.fx.pool_fire(tf, ev.how != Kill::Quiet);
}

/// The fire on a knocked-out vehicle, minute by minute: an engine-bay fire that grows into the hull or dies
/// down; a burning hull whose charges go off in bursts, one of which may throw the turret; then the smoulder.
pub(super) fn burn(
    mut commands: Commands,
    time: Res<Time>,
    mut fire: WreckFire,
    mut stats: ResMut<Stats>,
    mut radio: ResMut<Radio>,
    turrets: Query<&GlobalTransform, With<Turret>>,
    mut q: Query<(Entity, &Vehicle, &Transform, &mut Fire)>,
) {
    let dt = crate::sim_dt(&time);
    let now = time.elapsed_secs_f64();
    let rng = &mut fire.fxr.0;
    for (e, v, tf, mut f) in &mut q {
        f.t += dt;
        let p = tf.translation;
        match f.phase {
            Blaze::Deck => {
                if let Some(at) = f.escalate_at
                    && f.t > at
                {
                    // The fire reaches the fighting compartment: from here it is a wreck.
                    f.escalate_at = None;
                    commands.trigger(Wrecked { vehicle: e, cause: f.cause, how: Kill::Quiet });
                } else if f.escalate_at.is_none() && f.t > 240.0 {
                    f.phase = Blaze::Smoulder;
                }
            }
            Blaze::Torch => {
                let toss = f.toss_at.is_some_and(|at| f.t >= at);
                if toss || (f.bursts > 0 && f.t > f.next_burst) {
                    f.bursts = f.bursts.saturating_sub(1);
                    f.next_burst = f.t + rng.random_range(8.0..90.0);
                    if toss {
                        f.toss_at = None;
                        cook_off_blast(&mut fire.fx, &mut fire.sound, &mut fire.blasts, p);
                        if let Ok(t) = turrets.get(v.turret) {
                            toss_turret(&mut commands, rng, &mut stats, &mut radio, v, t, now);
                        }
                    } else {
                        let s = burst_strength(v.kind);
                        fire.fx.cookoff(p, s);
                        fire.sound.at(p, Sfx::CookOff, 0.4 + 0.5 * s);
                    }
                }
                if f.t > f.torch_secs {
                    f.phase = Blaze::Smoulder;
                }
            }
            Blaze::Smoulder => {}
        }
    }
}
