//! The soundtrack, rebuilt offline from what a `BAV_SHOWCASE` run wrote: `sfx-*.wav` (the synthesised sounds),
//! `audio.log` (every sound the game played, and the ambience levels each frame) and `segments.txt` (video time,
//! game time and length per shot). Each shot's sounds are placed at their video time and cut at the shot's end, as
//! an editor would; the ambience runs under every shot as the game had it, and a bed of wind and far-off guns lies
//! under every card (`cards.txt`), so a cut never lands on silence.

use std::{fs, path::Path};

pub const RATE: usize = 44_100;

/// A mono 16-bit WAV, as the game writes them (`bav_look::audio::write_wav`).
pub fn read_wav(path: &Path) -> Result<Vec<f32>, String> {
    let b = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if b.len() < 12 || &b[0..4] != b"RIFF" || &b[8..12] != b"WAVE" {
        return Err(format!("{}: not a WAV file", path.display()));
    }
    // Walk the chunks to `data`; the game's files have only `fmt ` before it, but don't count on it.
    let mut at = 12;
    while at + 8 <= b.len() {
        let id = &b[at..at + 4];
        let len = u32::from_le_bytes(b[at + 4..at + 8].try_into().unwrap()) as usize;
        let body = &b[at + 8..(at + 8 + len).min(b.len())];
        if id == b"data" {
            return Ok(body.as_chunks::<2>().0.iter().map(|s| i16::from_le_bytes(*s) as f32 / 32767.0).collect());
        }
        at += 8 + len + (len & 1);
    }
    Err(format!("{}: no data chunk", path.display()))
}

pub fn write_wav(path: &Path, samples: &[f32]) -> Result<(), String> {
    let data = (samples.len() * 2) as u32;
    let mut b = Vec::with_capacity(44 + samples.len() * 2);
    b.extend_from_slice(b"RIFF");
    b.extend_from_slice(&(36 + data).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&(RATE as u32).to_le_bytes());
    b.extend_from_slice(&(RATE as u32 * 2).to_le_bytes());
    b.extend_from_slice(&2u16.to_le_bytes());
    b.extend_from_slice(&16u16.to_le_bytes());
    b.extend_from_slice(b"data");
    b.extend_from_slice(&data.to_le_bytes());
    for s in samples {
        b.extend_from_slice(&((s.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes());
    }
    fs::write(path, b).map_err(|e| format!("{}: {e}", path.display()))
}

fn load(dir: &Path, name: &str) -> Result<Vec<f32>, String> {
    read_wav(&dir.join(format!("sfx-{name}.wav")))
}

/// Ramp the first and last `f` samples in and out (linearly, both ends reached).
fn fade_edges(s: &mut [f32], f: usize) {
    let n = s.len();
    let f = f.min(n / 2);
    if f < 2 {
        return;
    }
    for i in 0..f {
        let k = i as f32 / (f - 1) as f32;
        s[i] *= k;
        s[n - 1 - i] *= k;
    }
}

fn gain(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

/// Wind and far-off guns, wrapped to any length, with short edge fades: under the cards, the title and the end.
pub fn bed(dir: &Path, n: usize, gain_db: f32) -> Result<Vec<f32>, String> {
    let mut mix = vec![0.0f32; n];
    for (name, level) in [("Wind", 0.25), ("Rumble", 0.3)] {
        let lp = load(dir, name)?;
        for (i, m) in mix.iter_mut().enumerate() {
            *m += lp[i % lp.len()] * level;
        }
    }
    fade_edges(&mut mix, (0.3 * RATE as f32) as usize);
    let g = gain(gain_db);
    mix.iter_mut().for_each(|m| *m *= g);
    Ok(mix)
}

/// Linear interpolation over increasing `xs`, held flat past either end.
fn interp(x: f64, xs: &[f64], ys: &[f32]) -> f32 {
    if xs.is_empty() {
        return 0.0;
    }
    let i = xs.partition_point(|&v| v <= x);
    if i == 0 {
        return ys[0];
    }
    if i == xs.len() {
        return ys[xs.len() - 1];
    }
    let (x0, x1) = (xs[i - 1], xs[i]);
    let k = if x1 > x0 { ((x - x0) / (x1 - x0)) as f32 } else { 0.0 };
    ys[i - 1] + (ys[i] - ys[i - 1]) * k
}

/// What a mix reports: its length, the sounds logged, and the cards it laid beds under.
pub struct Mixed {
    pub secs: f64,
    pub sounds: usize,
    pub cards: usize,
}

/// Mix `dir`'s soundtrack into `out`: `lead` seconds of bed before the first shot (the one-minute tour's title),
/// `gain_db` on the finished mix.
pub fn mix(dir: &Path, out: &Path, lead: f64, gain_db: f32) -> Result<Mixed, String> {
    let read = |name: &str| fs::read_to_string(dir.join(name)).map_err(|e| format!("{}: {e}", dir.join(name).display()));
    let segs: Vec<(f64, f64, f64)> = read("segments.txt")?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let v: Vec<f64> = l.split_whitespace().filter_map(|x| x.parse().ok()).collect();
            (v[0], v[1], v[2])
        })
        .collect();
    let Some(&(lv, _, ll)) = segs.last() else { return Err(format!("{}: no segments", dir.display())) };
    let total = lv + ll + lead;
    let samples = |t: f64| (t * RATE as f64) as usize;
    let mut mix = vec![0.0f32; samples(total) + RATE];
    let seg_of = |t: f64| segs.iter().copied().find(|&(_, gs, ln)| gs - 1e-3 <= t && t < gs + ln);

    let log = read("audio.log")?;
    let mut bank: std::collections::HashMap<String, Vec<f32>> = Default::default();
    let (mut amb_t, mut amb_eng, mut amb_drone) = (Vec::new(), Vec::new(), Vec::new());
    let mut sounds = 0;
    for line in log.lines() {
        let p: Vec<&str> = line.split_whitespace().collect();
        match p.first() {
            Some(&"sfx") if p.len() >= 5 => {
                sounds += 1;
                let (t, name, vol, speed): (f64, &str, f32, f64) = (p[1].parse().unwrap_or(0.0), p[2], p[3].parse().unwrap_or(0.0), p[4].parse().unwrap_or(1.0));
                let Some((vs, gs, ln)) = seg_of(t) else { continue };
                if !bank.contains_key(name) {
                    bank.insert(name.to_string(), load(dir, name)?);
                }
                let raw = &bank[name];
                // A sound played faster or slower is resampled, linearly, as it was heard.
                let resampled;
                let src: &[f32] = if (speed - 1.0).abs() > 1e-3 {
                    let n = ((raw.len() as f64 - 1.0) / speed).ceil().max(0.0) as usize;
                    resampled = (0..n)
                        .map(|i| {
                            let x = i as f64 * speed;
                            let j = x as usize;
                            let k = (x - j as f64) as f32;
                            raw[j] + (raw[(j + 1).min(raw.len() - 1)] - raw[j]) * k
                        })
                        .collect::<Vec<f32>>();
                    &resampled
                } else {
                    raw
                };
                let start = samples(lead + vs + (t - gs));
                // A hard cut at the shot's end.
                let end = samples(lead + vs + ln).min(start + src.len()).min(mix.len());
                if end <= start {
                    continue;
                }
                let mut chunk: Vec<f32> = src[..end - start].iter().map(|s| s * vol).collect();
                let fade = chunk.len().min((0.03 * RATE as f64) as usize);
                let n = chunk.len();
                for i in 0..fade {
                    chunk[n - fade + i] *= if fade > 1 { 1.0 - i as f32 / (fade - 1) as f32 } else { 1.0 };
                }
                for (m, c) in mix[start..end].iter_mut().zip(chunk) {
                    *m += c;
                }
            }
            Some(&"amb") if p.len() >= 4 => {
                amb_t.push(p[1].parse::<f64>().unwrap_or(0.0));
                amb_eng.push(p[2].parse::<f32>().unwrap_or(0.0));
                amb_drone.push(p[3].parse::<f32>().unwrap_or(1.0));
            }
            _ => {}
        }
    }

    // Ambience: wind and the distant war all along; engines and the drone's whine as the game had them.
    let loops: Vec<(&str, Vec<f32>)> = ["Wind", "Rumble", "Engines", "Rotor"].iter().map(|k| load(dir, k).map(|v| (*k, v))).collect::<Result<_, _>>()?;
    for &(vs, gs, ln) in &segs {
        let (a, b) = (samples(lead + vs), samples(lead + vs + ln).min(mix.len()));
        if b <= a {
            continue;
        }
        let n = b - a;
        let off = samples(gs);
        let level_at = |i: usize| {
            let t = gs + i as f64 / RATE as f64;
            let (eng, drone) = if amb_t.is_empty() { (0.0, 1.0) } else { (interp(t, &amb_t, &amb_eng), interp(t, &amb_t, &amb_drone)) };
            [0.18 + 0.12 * (1.0 - drone), 0.35, eng, 0.05 * drone]
        };
        let levels: Vec<[f32; 4]> = (0..n).map(level_at).collect();
        for (k, (_, lp)) in loops.iter().enumerate() {
            let mut seg: Vec<f32> = (0..n).map(|i| lp[(off + i) % lp.len()] * levels[i][k]).collect();
            fade_edges(&mut seg, (0.05 * RATE as f64) as usize);
            for (m, s) in mix[a..b].iter_mut().zip(seg) {
                *m += s;
            }
        }
    }

    // The lead-in under the title card: wind and far-off guns, fading in.
    let n = samples(lead);
    let (wind, rumble) = (&loops[0].1, &loops[1].1);
    for i in 0..n {
        let k = if n > 1 { i as f32 / (n - 1) as f32 } else { 1.0 };
        mix[i] += (wind[i % wind.len()] * 0.25 + rumble[i % rumble.len()] * 0.3) * k;
    }

    // A bed under each card, so the cuts between shots never land on silence.
    let cards: Vec<(f64, f64)> = fs::read_to_string(dir.join("cards.txt"))
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| {
            let mut h = l.split('\t').next()?.split_whitespace();
            Some((h.next()?.parse().ok()?, h.next()?.parse().ok()?))
        })
        .collect();
    for &(vt, secs) in &cards {
        let (a, b) = (samples(lead + vt), samples(lead + vt + secs).min(mix.len()));
        if b > a {
            for (m, s) in mix[a..b].iter_mut().zip(bed(dir, b - a, gain_db)?) {
                *m += s;
            }
        }
    }

    // Loudness: the drone is far from most of it, so bring it up and soft-clip the peaks. With cards (the film,
    // the trailer) the filmed segments are brought to about -20 dBFS RMS first; the measured -18 LUFS of the
    // finished parts is the caller's (`gain_db`).
    if !cards.is_empty() {
        let (mut sum, mut count) = (0.0f64, 0usize);
        for &(vs, _, ln) in &segs {
            for m in &mix[samples(lead + vs).min(mix.len())..samples(lead + vs + ln).min(mix.len())] {
                sum += (*m as f64).powi(2);
                count += 1;
            }
        }
        let rms = (sum / count.max(1) as f64).sqrt() + 1e-9;
        let g = gain((-20.0 - 20.0 * rms.log10()) as f32);
        mix.iter_mut().for_each(|m| *m *= g);
    }
    let peak = percentile_abs(&mix, 99.9) + 1e-6;
    let out_gain = gain(gain_db);
    for m in &mut mix {
        // A pure output gain: before the clip it would be squeezed away by the tanh.
        *m = (*m / peak * 0.9).tanh() * 0.95 * out_gain;
    }
    write_wav(out, &mix)?;
    Ok(Mixed { secs: total, sounds, cards: cards.len() })
}

/// The `p`th percentile of the absolute values, interpolated between ranks as numpy's is.
fn percentile_abs(v: &[f32], p: f64) -> f32 {
    if v.is_empty() {
        return 0.0;
    }
    let mut a: Vec<f32> = v.iter().map(|x| x.abs()).collect();
    let rank = p / 100.0 * (a.len() - 1) as f64;
    let lo = rank.floor() as usize;
    let (_, &mut x0, rest) = a.select_nth_unstable_by(lo, f32::total_cmp);
    let x1 = rest.iter().copied().fold(f32::INFINITY, f32::min);
    if lo + 1 >= v.len() {
        return x0;
    }
    x0 + (x1 - x0) * (rank - lo as f64) as f32
}
