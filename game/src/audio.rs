//! Sound, synthesised at startup (no sample files). Every sound travels at 343 m/s: you see the flash on the
//! drone feed, and the thud arrives seconds later, or never, if it is far enough away.

use bevy::{
    audio::{AddAudioSource, ChannelCount, Decodable, SampleRate, Source, Volume},
    prelude::*,
    reflect::TypePath,
};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::{sync::Arc, time::Duration};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<Synth>()
            .init_resource::<Pending>()
            .add_systems(Startup, (build_bank, start_ambience).chain())
            .add_systems(Update, (play_pending, ambience_mix));
    }
}

const RATE: u32 = 44_100;

#[derive(Asset, TypePath, Clone)]
pub struct Synth {
    samples: Arc<Vec<f32>>,
}

pub struct SynthDecoder {
    samples: Arc<Vec<f32>>,
    i: usize,
}

impl Iterator for SynthDecoder {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let s = self.samples.get(self.i).copied();
        self.i += 1;
        s
    }
}

impl Source for SynthDecoder {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> ChannelCount {
        ChannelCount::new(1).unwrap()
    }
    fn sample_rate(&self) -> SampleRate {
        SampleRate::new(RATE).unwrap()
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(self.samples.len() as f32 / RATE as f32))
    }
}

impl Decodable for Synth {
    type Decoder = SynthDecoder;
    fn decoder(&self) -> SynthDecoder {
        SynthDecoder { samples: self.samples.clone(), i: 0 }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sfx {
    BlastNear,
    BlastFar,
    Thermobaric,
    MineBlast,
    TankGun,
    Cannon30,
    MachineGun,
    AtgmLaunch,
    Whistle,
    CookOff,
    Radio,
}

#[derive(Resource)]
pub struct Bank {
    sounds: Vec<(Sfx, Handle<Synth>)>,
    wind: Handle<Synth>,
    rumble: Handle<Synth>,
    engines: Handle<Synth>,
    rotor: Handle<Synth>,
}

impl Bank {
    fn get(&self, s: Sfx) -> Handle<Synth> {
        self.sounds.iter().find(|(k, _)| *k == s).unwrap().1.clone()
    }
}

// ---------------------------------------------------------------------------------------------------------
// Synthesis helpers

struct Lp {
    y: f32,
    a: f32,
}

impl Lp {
    fn new(cutoff: f32) -> Self {
        let a = 1.0 - (-2.0 * std::f32::consts::PI * cutoff / RATE as f32).exp();
        Self { y: 0.0, a }
    }
    fn set(&mut self, cutoff: f32) {
        self.a = 1.0 - (-2.0 * std::f32::consts::PI * cutoff / RATE as f32).exp();
    }
    fn run(&mut self, x: f32) -> f32 {
        self.y += self.a * (x - self.y);
        self.y
    }
}

fn len(secs: f32) -> usize {
    (secs * RATE as f32) as usize
}

fn normalize(mut v: Vec<f32>, peak: f32) -> Vec<f32> {
    let m = v.iter().fold(0.0f32, |m, x| m.max(x.abs())).max(1e-6);
    for x in &mut v {
        *x *= peak / m;
    }
    v
}

/// A blast: a sharp transient, then low-passed noise with a long sub-bass tail.
fn blast(rng: &mut SmallRng, secs: f32, bright: f32, sub: f32) -> Vec<f32> {
    let n = len(secs);
    let mut lp = Lp::new(bright);
    let mut lp2 = Lp::new(60.0);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / RATE as f32;
        let env = (-t * 3.5 / secs.max(0.1) * 2.0).exp();
        lp.set(bright * (0.25 + 0.75 * (-t * 6.0).exp()));
        let noise = rng.random_range(-1.0f32..1.0);
        let crack = if t < 0.012 { rng.random_range(-1.0f32..1.0) * (1.0 - t / 0.012) * 1.5 } else { 0.0 };
        let body = lp.run(noise) * env * 2.5;
        let low = lp2.run(noise) * (-t * 2.0).exp() * sub * 6.0;
        let thump = (t * 2.0 * std::f32::consts::PI * (48.0 - t * 30.0)).sin() * (-t * 9.0).exp() * sub;
        out.push(crack + body + low + thump);
    }
    normalize(out, 0.95)
}

fn burst(rng: &mut SmallRng, shots: u32, interval: f32, bright: f32) -> Vec<f32> {
    let n = len(shots as f32 * interval + 0.6);
    let mut out = vec![0.0; n];
    for s in 0..shots {
        let start = len(s as f32 * interval + rng.random_range(0.0..interval * 0.15));
        let shot = blast(rng, 0.35, bright, 0.2);
        for (i, x) in shot.iter().enumerate() {
            if start + i < n {
                out[start + i] += x * 0.7;
            }
        }
    }
    normalize(out, 0.9)
}

fn whoosh(rng: &mut SmallRng) -> Vec<f32> {
    let n = len(2.2);
    let mut lp = Lp::new(800.0);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / RATE as f32;
        let env = if t < 0.05 { t / 0.05 } else { (-(t - 0.05) * 1.6).exp() };
        lp.set(600.0 + 2400.0 * (-t * 2.0).exp());
        let pop = if t < 0.02 { rng.random_range(-1.0f32..1.0) * 2.0 } else { 0.0 };
        out.push(lp.run(rng.random_range(-1.0f32..1.0)) * env * 3.0 + pop);
    }
    normalize(out, 0.8)
}

fn whistle() -> Vec<f32> {
    // An incoming shell: a falling tone with a doppler sweep, cut off at impact.
    let n = len(1.4);
    let mut out = Vec::with_capacity(n);
    let mut phase = 0.0f32;
    for i in 0..n {
        let t = i as f32 / RATE as f32;
        let f = 1400.0 - 700.0 * (t / 1.4);
        phase += f / RATE as f32;
        let env = (t / 1.4).powf(1.5);
        out.push((phase * std::f32::consts::TAU).sin() * env * 0.3);
    }
    out
}

fn radio(rng: &mut SmallRng) -> Vec<f32> {
    // Squelch: a click, a burst of hiss, a click.
    let n = len(0.35);
    let mut lp = Lp::new(3000.0);
    (0..n).map(|i| {
        let t = i as f32 / RATE as f32;
        let click = if t < 0.004 || (t > 0.33 && t < 0.334) { 0.8 } else { 0.0 };
        click + lp.run(rng.random_range(-1.0f32..1.0)) * 0.25 * (1.0 - t / 0.35)
    }).collect()
}

fn cookoff(rng: &mut SmallRng) -> Vec<f32> {
    // Ammunition cooking off: a roar with irregular cracks inside it.
    let n = len(6.0);
    let mut out = blast(rng, 6.0, 900.0, 0.8);
    out.resize(n, 0.0);
    for _ in 0..30 {
        let at = len(rng.random_range(0.2..5.5));
        let c = blast(rng, 0.4, 3000.0, 0.3);
        for (i, x) in c.iter().enumerate() {
            if at + i < n {
                out[at + i] += x * rng.random_range(0.2..0.6);
            }
        }
    }
    normalize(out, 0.95)
}

/// A seamless loop of noise shaped by a low-pass that wanders: wind, distant war, engines, a drone's whine.
fn bed(rng: &mut SmallRng, secs: f32, f0: f32, f1: f32, booms: u32) -> Vec<f32> {
    let n = len(secs);
    let mut lp = Lp::new(f0);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / RATE as f32;
        let wander = 0.5 + 0.5 * (t * 0.37).sin() * (t * 0.13 + 1.0).sin();
        lp.set(f0 + (f1 - f0) * wander);
        out.push(lp.run(rng.random_range(-1.0f32..1.0)) * (0.6 + 0.4 * wander));
    }
    for _ in 0..booms {
        let at = len(rng.random_range(0.0..secs - 4.0));
        let b = blast(rng, 3.5, 160.0, 1.0);
        let g = rng.random_range(0.15..0.6);
        for (i, x) in b.iter().enumerate() {
            if at + i < n {
                out[at + i] += x * g;
            }
        }
    }
    // Crossfade the ends so the loop has no seam.
    let fade = len(0.5);
    for i in 0..fade {
        let k = i as f32 / fade as f32;
        let tail = out[n - fade + i];
        out[i] = out[i] * k + tail * (1.0 - k);
    }
    out.truncate(n - fade);
    normalize(out, 0.7)
}

fn engines(rng: &mut SmallRng) -> Vec<f32> {
    // Diesel V12s at idle and moving: a pulsing low drone plus track clatter.
    let n = len(8.0);
    let mut lp = Lp::new(180.0);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / RATE as f32;
        let pulse = 0.6 + 0.4 * (t * 2.0 * std::f32::consts::PI * 23.0).sin().abs();
        let clatter = if (t * 11.0).fract() < 0.05 { rng.random_range(-0.4f32..0.4) } else { 0.0 };
        out.push(lp.run(rng.random_range(-1.0f32..1.0)) * 3.0 * pulse + clatter);
    }
    normalize(out, 0.7)
}

fn rotor(_rng: &mut SmallRng) -> Vec<f32> {
    // A quadcopter's whine heard from below: blade-pass tones beating against each other.
    let n = len(4.0);
    (0..n).map(|i| {
        let t = i as f32 / RATE as f32;
        let tone = |f: f32| (t * f * std::f32::consts::TAU).sin();
        (tone(182.0) + tone(185.5) * 0.8 + tone(364.0) * 0.3) * 0.2
    }).collect()
}

fn write_wav(path: &str, samples: &[f32]) {
    let mut b = Vec::with_capacity(44 + samples.len() * 2);
    let data = (samples.len() * 2) as u32;
    b.extend_from_slice(b"RIFF");
    b.extend_from_slice(&(36 + data).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&RATE.to_le_bytes());
    b.extend_from_slice(&(RATE * 2).to_le_bytes());
    b.extend_from_slice(&2u16.to_le_bytes());
    b.extend_from_slice(&16u16.to_le_bytes());
    b.extend_from_slice(b"data");
    b.extend_from_slice(&data.to_le_bytes());
    for s in samples {
        b.extend_from_slice(&((s.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes());
    }
    let _ = std::fs::write(path, b);
}

fn build_bank(mut commands: Commands, mut assets: ResMut<Assets<Synth>>) {
    let mut rng = SmallRng::seed_from_u64(0x2023_0206);
    let mut add = |v: Vec<f32>| assets.add(Synth { samples: Arc::new(v) });
    let sounds = vec![
        (Sfx::BlastNear, add(blast(&mut rng, 3.0, 2600.0, 1.0))),
        (Sfx::BlastFar, add(blast(&mut rng, 3.5, 300.0, 1.0))),
        (Sfx::Thermobaric, add(blast(&mut rng, 5.0, 700.0, 1.4))),
        (Sfx::MineBlast, add(blast(&mut rng, 2.5, 1800.0, 1.2))),
        (Sfx::TankGun, add(blast(&mut rng, 2.0, 3500.0, 0.9))),
        (Sfx::Cannon30, add(burst(&mut rng, 6, 0.18, 2800.0))),
        (Sfx::MachineGun, add(burst(&mut rng, 12, 0.085, 4000.0))),
        (Sfx::AtgmLaunch, add(whoosh(&mut rng))),
        (Sfx::Whistle, add(whistle())),
        (Sfx::CookOff, add(cookoff(&mut rng))),
        (Sfx::Radio, add(radio(&mut rng))),
    ];
    let wind = add(bed(&mut rng, 24.0, 250.0, 900.0, 0));
    let rumble = add(bed(&mut rng, 40.0, 90.0, 180.0, 9));
    let engines = add(engines(&mut rng));
    let rotor = add(rotor(&mut rng));
    if let Some(dir) = crate::dev_env::dev().showcase.as_deref() {
        // The showcase rebuilds the soundtrack offline from these.
        let dump = |name: &str, h: &Handle<Synth>| {
            if let Some(a) = assets.get(h) {
                write_wav(&format!("{dir}/sfx-{name}.wav"), &a.samples);
            }
        };
        for (k, h) in &sounds {
            dump(&format!("{k:?}"), h);
        }
        dump("Wind", &wind);
        dump("Rumble", &rumble);
        dump("Engines", &engines);
        dump("Rotor", &rotor);
    }
    commands.insert_resource(Bank { sounds, wind, rumble, engines, rotor });
}

// ---------------------------------------------------------------------------------------------------------
// Playback

#[derive(Resource, Default)]
pub struct Pending {
    queue: Vec<(f64, Sfx, f32, f32)>,
}

/// Play `sfx` from `pos`, heard at the camera after the sound's travel time, attenuated with distance.
pub fn play_at(pending: &mut Pending, now: f64, listener: Vec3, pos: Vec3, sfx: Sfx, loudness: f32) {
    let d = listener.distance(pos);
    let delay = d as f64 / 343.0;
    // Distant blasts lose their crack: switch to the far variant.
    let sfx = if sfx == Sfx::BlastNear && d > 900.0 { Sfx::BlastFar } else { sfx };
    let vol = loudness * (80.0 / (80.0 + d)).powf(1.2);
    if vol < 0.004 {
        return;
    }
    let speed = 1.0 - (d / 6000.0).min(0.15);
    pending.queue.push((now + delay, sfx, vol, speed));
}

/// Play without delay (radio, UI).
pub fn play_now(pending: &mut Pending, now: f64, sfx: Sfx, vol: f32) {
    pending.queue.push((now, sfx, vol, 1.0));
}

fn play_pending(mut commands: Commands, time: Res<Time>, bank: Option<Res<Bank>>, mut pending: ResMut<Pending>, playing: Query<(), With<AudioPlayer<Synth>>>) {
    let Some(bank) = bank else { return };
    let now = time.elapsed_secs_f64();
    let mut busy = playing.iter().count();
    let mut keep = Vec::new();
    for item in pending.queue.drain(..) {
        if item.0 > now {
            keep.push(item);
            continue;
        }
        if busy > 40 {
            continue; // the mix is saturated; drop the quiet ones
        }
        busy += 1;
        if let Some(dir) = crate::dev_env::dev().showcase.as_deref() {
            crate::showcase::append(&format!("{dir}/audio.log"), &format!("sfx {:.4} {:?} {:.5} {:.4}\n", now, item.1, item.2.min(1.5), item.3));
        }
        commands.spawn((AudioPlayer(bank.get(item.1)), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(item.2.min(1.5))).with_speed(item.3)));
    }
    pending.queue = keep;
}

#[derive(Component)]
pub struct Ambience(pub AmbienceKind);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AmbienceKind {
    Wind,
    Rumble,
    Engines,
    Rotor,
}

fn start_ambience(mut commands: Commands, bank: Res<Bank>) {
    for (h, k, v) in [(bank.wind.clone(), AmbienceKind::Wind, 0.25), (bank.rumble.clone(), AmbienceKind::Rumble, 0.35), (bank.engines.clone(), AmbienceKind::Engines, 0.0), (bank.rotor.clone(), AmbienceKind::Rotor, 0.04)] {
        commands.spawn((AudioPlayer(h), PlaybackSettings::LOOP.with_volume(Volume::Linear(v)), Ambience(k)));
    }
}

/// Engine noise follows the nearest moving Russian vehicle; the rotor whine is only there in the drone view.
#[derive(Resource, Default)]
pub struct AmbienceLevels {
    pub engines: f32,
    pub drone_view: bool,
}

fn ambience_mix(time: Res<Time>, levels: Option<Res<AmbienceLevels>>, mut q: Query<(&Ambience, &mut AudioSink)>) {
    let Some(levels) = levels else { return };
    if let Some(dir) = crate::dev_env::dev().showcase.as_deref() {
        crate::showcase::append(&format!("{dir}/audio.log"), &format!("amb {:.4} {:.4} {}\n", time.elapsed_secs_f64(), levels.engines, levels.drone_view as u8));
    }
    for (a, mut sink) in &mut q {
        let v = match a.0 {
            AmbienceKind::Wind => if levels.drone_view { 0.18 } else { 0.3 },
            AmbienceKind::Rumble => 0.35,
            AmbienceKind::Engines => levels.engines,
            AmbienceKind::Rotor => if levels.drone_view { 0.05 } else { 0.0 },
        };
        sink.set_volume(Volume::Linear(v));
    }
}
