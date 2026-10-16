//! What the battle asks of the effects, and the effects' state the battle reads. The battle never draws: it puts
//! requests on [`FxQueue`] in the order it makes them, and `bav_look::fx` plays them at the start of
//! `Step::Effects`, in that order, from its own random stream. Smoke on the ground ([`Smoke`]) and the battle's
//! own stream for what it decides about an effect ([`FxRng`]) live here; so do the long-lived sources the battle
//! sets burning ([`Emitter`]), which the picture keeps puffing.

use crate::diorama::Side;
use bevy::prelude::*;
use rand::rngs::SmallRng;

#[derive(Resource)]
pub struct FxRng(pub SmallRng);

/// Wind drifts every plume the same way, as the vector it blows towards (north is -z). ERA5 at the dioramas
/// (design/next-items.md §2.1): on 6 February from the north-east, 4 m/s at dawn and 5.5 by midday, so towards
/// the south-west; on 8 June from the east at night turning south-east by day, about 2 m/s at dawn, so towards
/// the west-north-west. It is the look's alone: the gameplay smoke (`Smoke`) does not drift.
#[derive(Resource)]
pub struct Wind(pub Vec3);

/// The day's wind (see `Wind`); the ground painter reads it too, for the drifts in the belts' lee.
pub fn day_wind() -> Vec3 {
    if crate::diorama::diorama().summer() { Vec3::new(-2.0, 0.0, -0.7) } else { Vec3::new(-3.2, 0.0, 3.2) }
}

/// Smoke on the ground thick enough to hide a vehicle from a sight: grenades and smoke shells. (The particles are
/// what you see; this is what a gunner cannot see through.)
#[derive(Resource, Default)]
pub struct Smoke {
    clouds: Vec<(Vec2, f32, f64)>,
}

impl Smoke {
    pub fn add(&mut self, at: Vec2, radius: f32, until: f64) {
        self.clouds.push((at, radius, until));
    }

    /// Does smoke lie across the line from `a` to `b` at game time `now`?
    pub fn blocks(&self, a: Vec2, b: Vec2, now: f64) -> bool {
        self.clouds.iter().any(|&(c, r, until)| {
            if now > until {
                return false;
            }
            let ab = b - a;
            let t = if ab.length_squared() > 0.0 { ((c - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
            (a + ab * t).distance(c) < r
        })
    }

    /// Is `p` inside smoke?
    pub fn covers(&self, p: Vec2, now: f64) -> bool {
        self.blocks(p, p, now)
    }
}

/// What a particle is made of: each kind has its colour, and in [`update_particles`] its own way of moving with
/// the wind (§2.2 of design/next-items.md: hot smoke rises before it bends; dust stays low; fog goes as a sheet).
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::VariantArray)]
pub enum Kind {
    /// Diesel, rubber and oil burning: jet-black, oily, strongly buoyant.
    BlackSmoke,
    GreySmoke,
    WhiteSmoke,
    /// A diesel-into-the-exhaust screen: a milky fog that neither rises nor sinks, moving with the wind as a sheet.
    Fog,
    /// Smoke-grenade smoke: grey-white, hugging the ground, fattening in humid air.
    Hc,
    /// Pure white: coolant steam, white phosphorus.
    Steam,
    /// Frozen earth: dark clods and the column of a winter burst.
    Dirt,
    /// Dry summer earth: the khaki column and the hanging haze of a June burst.
    Dust,
    /// Lifted snow (in summer, the pale dust a track throws).
    Snow,
    Fire,
    /// Burning propellant: the white-yellow jets and sparks of a cook-off.
    Propellant,
    Flash,
    /// A Ukrainian tracer (strontium red).
    Tracer,
    /// A Russian tracer (barium green).
    TracerGreen,
    Spark,
}

/// One particle, as the battle describes it.
#[derive(Clone)]
pub struct Spawn {
    pub kind: Kind,
    pub pos: Vec3,
    pub vel: Vec3,
    pub life: f32,
    pub size0: f32,
    pub size1: f32,
    pub alpha: f32,
    pub drag: f32,
    pub gravity: f32,
    pub buoyancy: f32,
    pub streak: f32,
}

impl Default for Spawn {
    fn default() -> Self {
        Self { kind: Kind::GreySmoke, pos: Vec3::ZERO, vel: Vec3::ZERO, life: 3.0, size0: 1.0, size1: 3.0, alpha: 0.8, drag: 0.5, gravity: 0.0, buoyancy: 0.0, streak: 0.0 }
    }
}

/// A long-lived source: a burning wreck's fire and smoke column, a vehicle's exhaust smoke screen.
#[derive(Component)]
pub struct Emitter {
    pub kind: EmitterKind,
    pub rate: f32,
    /// Seconds to the next puff; set it ahead to delay the first.
    pub timer: f32,
    pub remaining: f32,
    pub secs: f32,
    pub offset: Vec3,
}

impl Emitter {
    /// `rate` puffs a second for `secs` seconds, from `offset` in the owner's frame.
    pub fn new(kind: EmitterKind, rate: f32, secs: f32, offset: Vec3) -> Self {
        Self { kind, rate, timer: 0.0, remaining: secs, secs, offset }
    }

    /// The same, starting after `delay` seconds.
    pub fn after(delay: f32, kind: EmitterKind, rate: f32, secs: f32, offset: Vec3) -> Self {
        Self { timer: delay, remaining: secs + delay, ..Self::new(kind, rate, secs, offset) }
    }

    /// Seconds since it started.
    pub fn age(&self) -> f32 {
        self.secs - self.remaining
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EmitterKind {
    /// The whole vehicle burning: flames twice the hull's height and a sooty column.
    WreckFire,
    Smoulder,
    /// Diesel injected into the exhaust: a milky fog pouring out low behind a moving vehicle.
    SmokeScreen,
    Exhaust,
    /// A fire in the engine bay: white oil and coolant steam, going grey, from the rear deck.
    EngineSmoke,
    /// A smoke grenade burning on the ground: grey-white puffs that merge into a low wall.
    Hc,
    /// A white-phosphorus wedge or a smoke canister on the ground: a pure white bank that rises a little.
    Wp,
    /// Fuel burning in a pool on the ground beside a hull: low flames, a thinner black plume, steam off the
    /// snow it melts.
    PoolFire,
}

/// One request of the effects, as the battle made it.
#[derive(Clone)]
pub enum FxCall {
    Spawn(Spawn),
    Flash { pos: Vec3, intensity: f32, range: f32, life: f32 },
    Explosion { at: Vec3, size: f32, ground: bool },
    Cookoff { at: Vec3, strength: f32 },
    EraPops { hull: Transform, top: bool },
    PoolFire { hull: Transform, violent: bool },
    Fireball { at: Vec3 },
    Thermobaric { at: Vec3 },
    Muzzle { at: Vec3, dir: Vec3, big: bool },
    Backblast { from: Vec3, dir: Vec3 },
    Phosphorus { at: Vec3 },
    Tracer { from: Vec3, to: Vec3, speed: f32, side: Side },
    Spray { at: Vec3, vel: Vec3 },
    /// A sound that belongs with an effect, heard as `sound::Sound::at` would make it.
    Sound { at: Vec3, sfx: crate::sound::Sfx, loudness: f32 },
}

/// The frame's requests of the effects, in the order the battle made them.
#[derive(Resource, Default)]
pub struct FxQueue(pub Vec<FxCall>);

impl FxQueue {
    pub fn spawn(&mut self, s: Spawn) {
        self.0.push(FxCall::Spawn(s));
    }
    /// A short point light: a blast, a muzzle.
    pub fn flash(&mut self, pos: Vec3, intensity: f32, range: f32, life: f32) {
        self.0.push(FxCall::Flash { pos, intensity, range, life });
    }
    pub fn explosion(&mut self, at: Vec3, size: f32, ground: bool) {
        self.0.push(FxCall::Explosion { at, size, ground });
    }
    pub fn cookoff(&mut self, at: Vec3, strength: f32) {
        self.0.push(FxCall::Cookoff { at, strength });
    }
    /// Reactive armour blocks going off on this hull.
    pub fn era_pops(&mut self, hull: &Transform, top: bool) {
        self.0.push(FxCall::EraPops { hull: *hull, top });
    }
    /// Maybe a fuel fire beside this wreck (the picture decides, from its own stream).
    pub fn pool_fire(&mut self, hull: &Transform, violent: bool) {
        self.0.push(FxCall::PoolFire { hull: *hull, violent });
    }
    pub fn fireball(&mut self, at: Vec3) {
        self.0.push(FxCall::Fireball { at });
    }
    pub fn thermobaric(&mut self, at: Vec3) {
        self.0.push(FxCall::Thermobaric { at });
    }
    pub fn muzzle(&mut self, at: Vec3, dir: Vec3, big: bool) {
        self.0.push(FxCall::Muzzle { at, dir, big });
    }
    pub fn backblast(&mut self, from: Vec3, dir: Vec3) {
        self.0.push(FxCall::Backblast { from, dir });
    }
    pub fn phosphorus(&mut self, at: Vec3) {
        self.0.push(FxCall::Phosphorus { at });
    }
    pub fn tracer(&mut self, from: Vec3, to: Vec3, speed: f32, side: Side) {
        self.0.push(FxCall::Tracer { from, to, speed, side });
    }
    pub fn spray(&mut self, at: Vec3, vel: Vec3) {
        self.0.push(FxCall::Spray { at, vel });
    }
    pub fn sound(&mut self, at: Vec3, sfx: crate::sound::Sfx, loudness: f32) {
        self.0.push(FxCall::Sound { at, sfx, loudness });
    }
}
