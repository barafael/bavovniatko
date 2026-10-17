//! The game's tasks, as `cargo xtask <task>` (the alias is in `.cargo/config.toml`).
//!
//! The tasks are `USAGE`'s. The game films headless on a fixed step (`BAV_SHOWCASE`, `crates/bavovniatko/src/showcase.rs`) and logs every
//! sound it plays; this builds it, films the parts, renders the cards, rebuilds each part's soundtrack from the log
//! (`mix.rs`), levels the parts, and encodes and joins them. `--scout` films at 640x360 with contact sheets, for
//! retiming shots. Needs ffmpeg, ffprobe and ImageMagick's `magick` on the path. The work dir holds thousands of
//! 1080p frames (tens of GB): it goes under `devlog/`, on the disk, never in `target/` or on a tmpfs.

mod mix;
mod reel;

use std::{
    ffi::OsString,
    fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
    time::{Duration, Instant},
};

/// A command's arguments, strings and paths alike.
macro_rules! args {
    ($($a:expr),* $(,)?) => { vec![$(OsString::from($a)),*] };
}

type Result<T = ()> = std::result::Result<T, String>;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let run = match args.first().map(String::as_str) {
        Some("trailer") => trailer(&args[1..]),
        Some("film") => film(&args[1..]),
        Some("tour") => tour(&args[1..]),
        Some("mix") => remix(&args[1..]),
        Some("smoke") => smoke(&args[1..]),
        _ => Err(USAGE.into()),
    };
    match run {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("xtask: {e}");
            ExitCode::FAILURE
        }
    }
}

const USAGE: &str = "usage:
  cargo xtask trailer [--scout] [out.mp4]      the trailer: the action of both days (design/trailer.md)
  cargo xtask film [--scout] [out.mp4]         the showcase film of both (design/showcase.md)
  cargo xtask tour [robotyne] [out.mp4]        the one-minute tour of one day
  cargo xtask mix <dir> <out.wav> [lead-s] [gain-db]   rebuild a filmed run's soundtrack
  cargo xtask smoke                            start the game headless once per switch and diorama (devlog/smoke/)";

/// The game's directory: the workspace root, one up from this crate.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("xtask sits in the workspace").to_path_buf()
}

/// What the film and the trailer share: where they go, at what quality, the work dir, and the cards.
struct Job {
    root: PathBuf,
    name: &'static str,
    scout: bool,
    res: (u32, u32),
    crf: u32,
    preset: &'static str,
    out: PathBuf,
    logs: PathBuf,
    work: WorkDir,
    card: Cards,
}

impl Job {
    /// `[--scout] [out.mp4]`: the scout at 640x360 into devlog/film/, else 1920x1080 at `crf` into devlog/videos/;
    /// the game built, a work dir made.
    fn new(name: &'static str, args: &[String], crf: u32) -> Result<Job> {
        let scout = args.iter().any(|a| a == "--scout");
        needs(&["ffmpeg", "ffprobe", "magick"])?;
        let root = root();
        let stamp = now("+%Y%m%d-%H%M")?;
        let (res, crf, preset) = if scout { ((640, 360), 30, "veryfast") } else { ((1920, 1080), crf, "slow") };
        let out = match args.iter().find(|a| !a.starts_with("--")) {
            Some(o) => absolute(o)?,
            None if scout => root.join(format!("devlog/film/{stamp}-{name}-scout.mp4")),
            None => root.join(format!("devlog/videos/{stamp}-{name}.mp4")),
        };
        let logs = root.join("devlog/film");
        mkdir(&logs)?;
        mkdir(out.parent().unwrap_or(&root))?;
        build(&root)?;
        let work = WorkDir::new(&root)?;
        let card = Cards::new(&root, res);
        Ok(Job { root, name, scout, res, crf, preset, out, logs, work, card })
    }

    fn w(&self) -> &Path {
        &self.work.0
    }

    fn size(&self) -> String {
        format!("{}x{}", self.res.0, self.res.1)
    }

    /// The one encoding every piece gets, so the join needs no re-encode.
    fn enc(&self) -> Vec<OsString> {
        args!["-c:v", "libx264", "-crf", self.crf.to_string(), "-preset", self.preset, "-pix_fmt", "yuv420p", "-c:a", "aac", "-b:a", "160k"]
    }

    /// Frames (from `first`, so many, or all) and their sound into a piece; `audio` filters the sound.
    fn encode_frames(&self, dir: &Path, first: u32, frames: Option<u32>, wav: &Path, audio: Option<(f64, f64, String)>, out: &Path) -> Result {
        let mut a = args!["-framerate", "30", "-start_number", first.to_string(), "-i", dir.join("f%05d.png")];
        if let Some((from, secs, filter)) = &audio {
            a.extend(args!["-ss", format!("{from:.3}"), "-t", format!("{secs:.3}"), "-i", wav, "-af", filter]);
        } else {
            a.extend(args!["-i", wav]);
        }
        if let Some(n) = frames {
            a.extend(args!["-frames:v", n.to_string()]);
        }
        a.extend(self.enc());
        a.extend(args!["-shortest", out]);
        ffmpeg(a)
    }

    /// A card held over a bed of wind and far-off guns, fading in and out.
    fn still(&self, name: &str, secs: f64, fades: (f64, f64), text: (&str, &str, &str), bed_dir: &Path) -> Result<PathBuf> {
        let w = self.w();
        let (png, wav, mp4) = (w.join(format!("{name}.png")), w.join(format!("{name}.wav")), w.join(format!("{name}.mp4")));
        self.card.render(&png, text.0, text.1, text.2)?;
        mix::write_wav(&wav, &mix::bed(bed_dir, (secs * mix::RATE as f64) as usize)?)?;
        self.still_from(&png, &wav, secs, fades, &mp4)?;
        Ok(mp4)
    }

    fn still_from(&self, png: &Path, wav: &Path, secs: f64, (fade_in, fade_out): (f64, f64), mp4: &Path) -> Result {
        let fades = format!("fade=t=in:st=0:d={fade_in},fade=t=out:st={:.2}:d={fade_out}", secs - fade_out);
        let mut a = args!["-loop", "1", "-t", format!("{secs}"), "-framerate", "30", "-i", png, "-i", wav, "-vf", fades];
        a.extend(self.enc());
        a.extend(args!["-shortest", mp4]);
        ffmpeg(a)
    }

    /// Join the pieces, then the scout's contact sheet (a tile a second, for retiming) or the devlog's row; the work
    /// dir goes.
    fn finish(self, pieces: &[PathBuf], what: &str) -> Result {
        let list: String = pieces.iter().map(|p| format!("file '{}'\n", p.display())).collect();
        let txt = self.w().join("list.txt");
        fs::write(&txt, list).map_err(|e| e.to_string())?;
        ffmpeg(args!["-f", "concat", "-safe", "0", "-i", &txt, "-c", "copy", &self.out])?;
        if self.scout {
            let sheet = self.logs.join(format!("{}-sheet-%02d.png", self.out.file_stem().unwrap_or_default().to_string_lossy()));
            ffmpeg(args!["-i", &self.out, "-vf", "fps=1,scale=240:-1,tile=8x15", sheet])?;
        } else {
            let file = self.out.file_name().unwrap_or_default().to_string_lossy();
            append(&self.root.join("devlog/videos.md"), &format!("| {} | [{file}](videos/{file}) | {what} |\n", now("+%Y-%m-%d %H:%M")?))?;
        }
        let (out, name) = (self.out.clone(), self.name);
        drop(self);
        println!("{name}: {} ({:.0} MB, {:.0} s)", out.display(), fs::metadata(&out).map(|m| m.len() as f64 / 1e6).unwrap_or(0.0), duration(&out)?);
        Ok(())
    }
}

/// A part's or a run's sound, levelled: mixed, measured (EBU R128, integrated), and brought to -18 LUFS, so the
/// pieces sit within a loudness point whatever the day's bursts did. The gain is a plain scale: the mix has already
/// been soft-clipped.
fn level(dir: &Path, wav: &Path) -> Result<mix::Mixed> {
    let mut m = mix::mix(dir, 0.0)?;
    mix::write_wav(wav, &m.samples)?;
    let lufs = loudness(wav)?;
    let g = mix::gain((-18.0 - lufs) as f32);
    m.samples.iter_mut().for_each(|s| *s *= g);
    mix::write_wav(wav, &m.samples)?;
    println!("level {}: {lufs:.1} LUFS, {:+.1} dB; {:.1} s, {} sounds, {} cards", dir.file_name().unwrap_or_default().to_string_lossy(), -18.0 - lufs, m.secs, m.sounds, m.cards);
    Ok(m)
}

/// The showcase film: both dioramas, cards over black between the shots, each part's sound rebuilt and levelled,
/// then title, part one, part two and end card joined.
fn film(args: &[String]) -> Result {
    let job = Job::new("film", args, 27)?;
    let w = job.w().to_path_buf();
    let parts = reel::DAYS.map(|d| d.part);

    // 1. The two parts, one after the other: they share the GPU.
    for part in parts {
        let dir = w.join(part);
        mkdir(&dir)?;
        let start = Instant::now();
        film_part(&job.root, &env_for(part, "film", None, &dir, job.res, &[]), &job.logs.join(format!("film-{part}.log")))?;
        println!("film: {part} filmed in {} min at {}", start.elapsed().as_secs() / 60, job.size());
    }

    // 2. The cards: black frames with a title, a smaller line and a dim note, linked into the frame ranges the
    //    tour reserved for them (before each shot's first frame).
    for part in parts {
        let dir = w.join(part);
        for c in mix::cards(&dir) {
            let (first, last) = ((c.vt * 30.0).round() as u32, ((c.vt + c.secs) * 30.0).round() as u32);
            let png = dir.join(format!("card-{first}.png"));
            job.card.render(&png, &c.title, &c.line, &c.note)?;
            for i in first..last {
                let frame = dir.join(format!("f{i:05}.png"));
                let _ = fs::remove_file(&frame);
                fs::hard_link(&png, &frame).map_err(|e| format!("{}: {e}", frame.display()))?;
            }
        }
    }

    // 3. Each part's sound, levelled, and the part encoded.
    for part in parts {
        let (dir, wav) = (w.join(part), w.join(format!("{part}.wav")));
        level(&dir, &wav)?;
        job.encode_frames(&dir, 0, None, &wav, None, &w.join(format!("{part}.mp4")))?;
    }

    // 4. Title and end card; the end card reads both parts' ledgers: four rows each, then the closing lines verbatim.
    let title = job.still("title", 5.0, (0.8, 0.6), ("BAVOVNIATKO", "Two dioramas of the Russo-Ukrainian war", "Everything here is simulated, and seen through a Ukrainian drone."), &w.join(parts[0]))?;
    fs::write(w.join("end.txt"), end_card(&w)?).map_err(|e| e.to_string())?;
    job.card.text_file(&w.join("end.png"), &w.join("end.txt"))?;
    mix::write_wav(&w.join("end.wav"), &mix::bed(&w.join(parts[1]), 10 * mix::RATE)?)?;
    job.still_from(&w.join("end.png"), &w.join("end.wav"), 10.0, (0.5, 1.0), &w.join("end.mp4"))?;

    // 5. The scout's contact sheets, a part at a time.
    if job.scout {
        let stem = job.out.file_stem().unwrap_or_default().to_string_lossy().to_string();
        for part in parts {
            ffmpeg(args!["-i", w.join(format!("{part}.mp4")), "-vf", "fps=1,scale=240:-1,tile=8x15", job.logs.join(format!("{stem}-{part}-sheet-%02d.png"))])?;
        }
    }
    let pieces = [title, w.join(format!("{}.mp4", parts[0])), w.join(format!("{}.mp4", parts[1])), w.join("end.mp4")];
    job.finish(&pieces, "The showcase film (design/showcase.md): title, Vuhledar with its cards, the intertitle, Mala Tokmachka, the end card from both ledgers.")
}

/// The trailer: the action of both days, found by an events pass and filmed in as many runs as it takes
/// (`reel.rs`, design/trailer.md), cut together: a title, Vuhledar, Mala Tokmachka, an end card.
fn trailer(args: &[String]) -> Result {
    // For a big screen: less grain lost to the encoder than the film's 27.
    let job = Job::new("trailer", args, 24)?;
    let w = job.w().to_path_buf();

    // 1. The events passes, both days at once at a postage-stamp size: nothing is filmed, every blast and hit is
    //    logged.
    let start = Instant::now();
    std::thread::scope(|s| -> Result {
        let runs: Vec<_> = reel::DAYS
            .iter()
            .map(|day| {
                let part = day.part;
                let (dir, file, log) = (w.join(format!("events-{part}")), w.join(format!("events-{part}.reel")), job.logs.join(format!("reel-events-{part}.log")));
                let root = &job.root;
                s.spawn(move || -> Result {
                    mkdir(&dir)?;
                    fs::write(&file, format!("end {}\n", day.end)).map_err(|e| e.to_string())?;
                    film_part(root, &env_for(part, "reel", Some(&file), &dir, (320, 180), day.env), &log)
                })
            })
            .collect();
        for r in runs {
            r.join().map_err(|_| "an events pass panicked".to_string())??;
        }
        Ok(())
    })?;
    println!("trailer: events passes in {} min", start.elapsed().as_secs() / 60);

    // 2. Each day's moments, filmed in runs; each run's sound rebuilt and levelled, and each moment cut from the run's
    //    frames and sound.
    let mut pieces = vec![job.still("title", 3.5, (0.4, 0.4), ("BAVOVNIATKO", "One minefield, seen from both sides", "Two days of the Russo-Ukrainian war, simulated, and seen through a Ukrainian drone."), &w.join("events-vuhledar"))?];
    for day in &reel::DAYS {
        let part = day.part;
        let events = w.join(format!("events-{part}/events.txt"));
        let _ = fs::copy(&events, job.logs.join(format!("reel-events-{part}.txt")));
        let moments = reel::moments(&events)?;
        let runs = reel::runs(&moments);
        println!("trailer: {part}: {} moments in {} runs", moments.len(), runs.len());
        let mut clips: Vec<Option<PathBuf>> = vec![None; moments.len()];
        for (k, run) in runs.iter().enumerate() {
            let dir = w.join(format!("{part}-{k}"));
            mkdir(&dir)?;
            let file = w.join(format!("{part}-{k}.reel"));
            let last = run.iter().map(|&i| moments[i].from + moments[i].secs).fold(0.0, f32::max);
            let shots: String = run.iter().map(|&i| format!("{}\n", moments[i].line)).collect();
            fs::write(&file, format!("{shots}end {:.2}\n", last + 1.0)).map_err(|e| e.to_string())?;
            let start = Instant::now();
            film_part(&job.root, &env_for(part, "reel", Some(&file), &dir, job.res, day.env), &job.logs.join(format!("reel-{part}-{k}.log")))?;
            println!("film: {part} run {k}, {} shots, in {} min", run.len(), start.elapsed().as_secs() / 60);
            let wav = w.join(format!("{part}-{k}.wav"));
            level(&dir, &wav)?;
            // The run's shots are its segments, in time order: where each starts in the run's video.
            for (&i, (vs, _, _)) in run.iter().zip(mix::segments(&dir)?) {
                let secs = moments[i].secs as f64;
                let clip = w.join(format!("{part}-{k}-{i}.mp4"));
                let fade = format!("afade=t=in:d=0.04,afade=t=out:st={:.2}:d=0.12", secs - 0.12);
                job.encode_frames(&dir, (vs * 30.0).round() as u32, Some((secs * 30.0).round() as u32), &wav, Some((vs, secs, fade)), &clip)?;
                clips[i] = Some(clip);
            }
            for f in fs::read_dir(&dir).map_err(|e| e.to_string())?.flatten() {
                if f.file_name().to_string_lossy().ends_with(".png") {
                    let _ = fs::remove_file(f.path());
                }
            }
        }
        pieces.push(job.still(&format!("card-{part}"), 2.5, (0.4, 0.4), day.card, &w.join(format!("events-{part}")))?);
        pieces.extend(reel::order(&moments).into_iter().filter_map(|i| clips[i].clone()));
    }
    pieces.push(job.still("end", 4.0, (0.4, 0.4), ("BAVOVNIATKO", "One minefield, seen from both sides", "Everything here is simulated."), &w.join("events-robotyne"))?);
    job.finish(&pieces, "The trailer (design/trailer.md): the action of both days, found by an events pass.")
}

/// A filming run's switches: the day, the tour (and the reel's file), the size, where it films to, and the day's
/// own.
fn env_for(part: &str, tour: &str, reel: Option<&Path>, dir: &Path, size: (u32, u32), extra: &[(&'static str, &'static str)]) -> Vec<(&'static str, String)> {
    let mut env = vec![("BAV_DIORAMA", part.to_string()), ("BAV_TOUR", tour.to_string()), ("BAV_RES", format!("{}x{}", size.0, size.1)), ("BAV_SHOWCASE", dir.display().to_string())];
    env.extend(reel.map(|r| ("BAV_REEL", r.display().to_string())));
    env.extend(extra.iter().map(|(k, v)| (*k, v.to_string())));
    env
}

/// The end card: both days' ledgers side by side, four rows each, then each day's closing lines.
fn end_card(work: &Path) -> Result<String> {
    let ledger = |part: &str| fs::read_to_string(work.join(part).join("ledger.txt")).map_err(|e| format!("{part}'s ledger: {e}"));
    let rows = |text: &str| -> Vec<String> {
        text.lines()
            .map(|l| l.split('\t').collect::<Vec<_>>())
            .filter(|f| f[0] != "closing")
            .take(4)
            .map(|f| match f.get(1) {
                // A row's note in parentheses would not fit the card's column: the label only.
                Some(v) => format!("{:<46} {v:>8}", f[0].split(" (").next().unwrap_or("").trim().chars().take(46).collect::<String>()),
                None => f[0].to_string(),
            })
            .collect()
    };
    let closings = |text: &str| -> Vec<String> { text.lines().filter_map(|l| l.strip_prefix("closing\t")).map(String::from).collect() };
    let (v, r) = (ledger("vuhledar")?, ledger("robotyne")?);
    let mut lines = vec![format!("{:<57} МАЛА ТОКМАЧКА (MALA TOKMACHKA)", "ВУГЛЕДАР (VUHLEDAR)"), String::new()];
    lines.extend(rows(&v).into_iter().zip(rows(&r)).map(|(a, b)| format!("{a}   {b}")));
    lines.push(String::new());
    lines.extend(closings(&v));
    lines.push(String::new());
    lines.extend(closings(&r));
    Ok(lines.join("\n"))
}

/// The one-minute tour of one day: a title over a lead-in bed, then the tour, in one encode.
fn tour(args: &[String]) -> Result {
    let robotyne = args.iter().any(|a| a == "robotyne");
    let out_arg = args.iter().find(|a| a.as_str() != "robotyne" && a.as_str() != "vuhledar");
    needs(&["ffmpeg", "ffprobe", "magick"])?;
    let root = root();
    let out = match out_arg {
        Some(o) => absolute(o)?,
        None => root.join(format!("devlog/videos/{}-tour-{}.mp4", now("+%Y%m%d-%H%M")?, if robotyne { "robotyne" } else { "vuhledar" })),
    };
    mkdir(out.parent().unwrap_or(&root))?;
    mkdir(&root.join("devlog/film"))?;
    build(&root)?;
    let work = WorkDir::new(&root)?;
    let w = &work.0;
    let mut env = vec![("BAV_RES", "1920x1080".to_string()), ("BAV_SHOWCASE", w.display().to_string())];
    if robotyne {
        env.push(("BAV_DIORAMA", "robotyne".into()));
    }
    film_part(&root, &env, &root.join("devlog/film/tour.log"))?;
    let lead = 3.0;
    let (place, date) = if robotyne { ("SOUTH OF MALA TOKMACHKA", "8 JUNE 2023") } else { ("VUHLEDAR", "6 FEBRUARY 2023") };
    magick(&[
        "-size", "1920x1080", "xc:black", "-font", "DejaVu-Sans", "-fill", "#e8e8ea", "-gravity", "center",
        "-pointsize", "64", "-annotate", "+0-60", place, "-pointsize", "34", "-annotate", "+0+20", date,
        "-fill", "#9a9ca0", "-pointsize", "22", "-annotate", "+0+90", "bavovniatko: a diorama of the Russo-Ukrainian war",
        &w.join("title.png").display().to_string(),
    ])?;
    let m = mix::mix(w, lead)?;
    mix::write_wav(&w.join("sound.wav"), &m.samples)?;
    println!("mixed {:.1} s, {} sounds logged", m.secs, m.sounds);
    ffmpeg(args![
        "-loop", "1", "-t", "3", "-framerate", "30", "-i", w.join("title.png"),
        "-framerate", "30", "-i", w.join("f%05d.png"),
        "-i", w.join("sound.wav"),
        "-filter_complex", "[0:v]fade=t=in:st=0:d=0.8,fade=t=out:st=2.4:d=0.6,format=yuv420p[t];[1:v]format=yuv420p[g];[t][g]concat=n=2:v=1:a=0[v]",
        "-map", "[v]", "-map", "2:a", "-c:v", "libx264", "-crf", "27", "-preset", "slow", "-tune", "grain", "-c:a", "aac", "-b:a", "160k", "-shortest",
        &out,
    ])?;
    drop(work);
    println!("tour: {} ({:.0} MB)", out.display(), fs::metadata(&out).map(|m| m.len() as f64 / 1e6).unwrap_or(0.0));
    Ok(())
}

/// `cargo xtask mix <dir> <out.wav> [lead-s] [gain-db]`: a filmed run's soundtrack again, to tune the mix.
fn remix(args: &[String]) -> Result {
    let [dir, out, rest @ ..] = args else { return Err(USAGE.into()) };
    let lead = rest.first().map(|s| s.parse::<f64>().map_err(|e| format!("lead {s}: {e}"))).transpose()?.unwrap_or(0.0);
    let gain = rest.get(1).map(|s| s.parse::<f32>().map_err(|e| format!("gain {s}: {e}"))).transpose()?.unwrap_or(0.0);
    let mut m = mix::mix(Path::new(dir), lead)?;
    let g = mix::gain(gain);
    m.samples.iter_mut().for_each(|s| *s *= g);
    mix::write_wav(Path::new(out), &m.samples)?;
    println!("mixed {:.1} s, {} sounds logged, {} cards -> {out}", m.secs, m.sounds, m.cards);
    Ok(())
}

/// The game's release build: what ships, and what the films are made with.
fn build(root: &Path) -> Result {
    let ok = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into())).args(["build", "-q", "--release", "-p", "bavovniatko"]).current_dir(root).status().map_err(|e| format!("cargo: {e}"))?.success();
    if ok { Ok(()) } else { Err("the game's release build failed".into()) }
}

/// The game built with the `fast` profile (release speed at a fraction of the build time), for the smoke runs.
fn build_fast(root: &Path) -> Result {
    let ok = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into())).args(["build", "-q", "--profile", "fast", "-p", "bavovniatko"]).current_dir(root).status().map_err(|e| format!("cargo: {e}"))?.success();
    if ok { Ok(()) } else { Err("the game's fast build failed".into()) }
}

/// `cargo xtask smoke`: the game started headless once per development switch and diorama, a second of the day
/// each, its log and a frame into devlog/smoke/. Bevy checks a system's queries and resources only when the system
/// first runs, so one that exists only under a switch is checked by nothing else (the showcase's by the same-day
/// runs). A run that panics, fails or does not finish in three minutes fails the task.
fn smoke(_args: &[String]) -> Result {
    const SWITCHES: &[(&str, &[(&str, &str)])] = &[
        ("plain", &[]),
        ("column", &[("BAV_COLUMN", "5:0:bradley@0")]),
        ("field", &[("BAV_FIELD", "1")]),
        ("lineup", &[("BAV_LINEUP", "0,0,0")]),
        ("vshadows-off", &[("BAV_OFF", "vshadows")]),
        ("command", &[("BAV_PLAY", "command"), ("BAV_ORDER", "3:1:hold")]),
        ("table", &[("BAV_PLAY", "command"), ("BAV_TABLE", "battle"), ("BAV_ORDERS", "open")]),
        ("ground", &[("BAV_VIEW", "ground")]),
        ("diag", &[("BAV_DIAG", "1")]),
    ];
    let root = root();
    build_fast(&root)?;
    let bin = root.join("target/fast/bavovniatko");
    let out = root.join("devlog/smoke");
    mkdir(&out)?;
    let mut failed = Vec::new();
    for dio in ["vuhledar", "robotyne"] {
        for (name, extra) in SWITCHES {
            let tag = format!("{dio}-{name}");
            let mut env: Vec<(&str, String)> = vec![("BAV_DIORAMA", dio.into()), ("BAV_RES", "320x180".into()), ("BAV_SHOT", out.join(format!("{tag}.png")).display().to_string()), ("BAV_SHOT_AT", "1".into())];
            env.extend(extra.iter().map(|(k, v)| (*k, v.to_string())));
            let started = Instant::now();
            let (finished, lines) = run_game(&bin, &root, &env, Duration::from_secs(180))?;
            let log = out.join(format!("{tag}.log"));
            fs::write(&log, lines.join("\n") + "\n").map_err(|e| format!("{}: {e}", log.display()))?;
            let panicked = lines.iter().position(|l| l.contains("panicked"));
            let good = finished && panicked.is_none();
            println!("{tag}: {} ({:.0} s)", if good { "ok" } else { "FAILED" }, started.elapsed().as_secs_f32());
            if let Some(i) = panicked {
                lines.iter().skip(i).take(4).for_each(|l| println!("  {l}"));
            }
            if !good {
                failed.push(tag);
            }
        }
    }
    if failed.is_empty() { Ok(()) } else { Err(format!("{} smoke runs failed: {}", failed.len(), failed.join(", "))) }
}

/// Film one part headless: the game's log, without its colour codes and its thousands of "Screenshot saved", goes
/// to `log`; its panics and errors are shown. A run that has not finished in a little over an hour is stopped.
fn film_part(root: &Path, env: &[(&str, String)], log: &Path) -> Result {
    let mut env: Vec<(&str, String)> = env.to_vec();
    env.push(("BAV_OPERATOR", "off".into()));
    let (_, lines) = run_game(&root.join("target/release/bavovniatko"), root, &env, Duration::from_secs(4000))?;
    fs::write(log, lines.join("\n") + "\n").map_err(|e| format!("{}: {e}", log.display()))?;
    for (i, l) in lines.iter().enumerate().filter(|(_, l)| (l.contains("panicked") || l.contains("ERROR")) && !l.contains("rodio")).take(5) {
        eprintln!("{l}");
        if l.contains("panicked") {
            lines.iter().skip(i + 1).take(3).for_each(|l| eprintln!("{l}"));
        }
    }
    Ok(())
}

/// Run the game headless (no briefing, no battery) with `env`, and keep what it logged, without its colour codes
/// and its thousands of "Screenshot saved". A run that has not finished within `limit` is stopped. Returns whether
/// it finished by itself and well, with the log.
fn run_game(bin: &Path, root: &Path, env: &[(&str, String)], limit: Duration) -> Result<(bool, Vec<String>)> {
    let mut cmd = Command::new(bin);
    cmd.current_dir(root)
        .envs([("BAV_HEADLESS", "1"), ("BAV_SKIP_BRIEFING", "1"), ("BAV_NO_BATTERY", "1")])
        .envs(env.iter().map(|(k, v)| (*k, v.as_str())))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("the game: {e}"))?;
    let lines = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    let readers: Vec<_> = [child.stdout.take().map(|o| Box::new(o) as Box<dyn std::io::Read + Send>), child.stderr.take().map(|e| Box::new(e) as Box<dyn std::io::Read + Send>)]
        .into_iter()
        .flatten()
        .map(|r| {
            let lines = lines.clone();
            std::thread::spawn(move || {
                for l in BufReader::new(r).lines().map_while(std::result::Result::ok) {
                    let l = strip_ansi(&l);
                    if !l.contains("Screenshot saved") {
                        lines.lock().unwrap().push(l);
                    }
                }
            })
        })
        .collect();
    let deadline = Instant::now() + limit;
    let finished = loop {
        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            break status.success();
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            eprintln!("xtask: stopped a run that did not finish in time");
            break false;
        }
        std::thread::sleep(Duration::from_millis(200));
    };
    for r in readers {
        let _ = r.join();
    }
    let lines = std::mem::take(&mut *lines.lock().unwrap());
    Ok((finished, lines))
}

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for c in chars.by_ref() {
                if c.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// The cards: black, a title, a smaller line and a dim note in the interface's font, centred, sized for the
/// resolution.
struct Cards {
    font: String,
    res: (u32, u32),
    s: f64,
}

impl Cards {
    fn new(root: &Path, res: (u32, u32)) -> Self {
        Self { font: root.join("crates/bavovniatko/src/fonts/LiberationMono-Regular.ttf").display().to_string(), res, s: res.1 as f64 / 1080.0 }
    }

    fn px(&self, v: f64) -> String {
        (v * self.s).round().to_string()
    }

    fn render(&self, out: &Path, title: &str, line: &str, note: &str) -> Result {
        let size = format!("{}x{}", self.res.0, self.res.1);
        let (y1, y2, y3) = (format!("+0-{}", self.px(70.0)), format!("+0+{}", self.px(10.0)), format!("+0+{}", self.px(80.0)));
        let (t1, t2, t3) = (self.px(54.0), self.px(30.0), self.px(22.0));
        let mut a = vec!["-size", &size, "xc:black", "-font", &self.font, "-fill", "#e8e8ea", "-gravity", "center", "-pointsize", &t1, "-annotate", &y1, title, "-pointsize", &t2, "-annotate", &y2, line];
        if !note.is_empty() {
            a.extend(["-fill", "#9a9ca0", "-pointsize", &t3, "-annotate", &y3, note]);
        }
        let o = out.display().to_string();
        a.push(&o);
        magick(&a)
    }

    /// A card of many lines from a file. 24 pt at 1080p: 22 read small on the first trailer scout, and at 26 the
    /// longest closing line runs off the card.
    fn text_file(&self, out: &Path, text: &Path) -> Result {
        let size = format!("{}x{}", self.res.0, self.res.1);
        let at = format!("@{}", text.display());
        magick(&["-size", &size, "xc:black", "-font", &self.font, "-pointsize", &self.px(24.0), "-fill", "#e8e8ea", "-gravity", "center", "-annotate", "+0+0", &at, &out.display().to_string()])
    }
}

/// The work dir under `devlog/`, removed when done (and when a task fails part way).
struct WorkDir(PathBuf);

impl WorkDir {
    fn new(root: &Path) -> Result<Self> {
        let dir = root.join(format!("devlog/filmwork.{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        mkdir(&dir)?;
        Ok(Self(dir))
    }
}

impl Drop for WorkDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn ffmpeg(args: Vec<OsString>) -> Result {
    let ok = Command::new("ffmpeg").args(["-loglevel", "error", "-y"]).args(&args).status().map_err(|e| format!("ffmpeg: {e}"))?.success();
    if ok { Ok(()) } else { Err(format!("ffmpeg failed: {}", args.iter().map(|a| a.to_string_lossy()).collect::<Vec<_>>().join(" "))) }
}

fn magick(args: &[&str]) -> Result {
    let ok = Command::new("magick").args(args).status().map_err(|e| format!("magick: {e}"))?.success();
    if ok { Ok(()) } else { Err(format!("magick failed making {}", args.last().unwrap_or(&""))) }
}

/// Integrated loudness (EBU R128) of a WAV or a video's sound, in LUFS.
fn loudness(file: &Path) -> Result<f64> {
    let o = Command::new("ffmpeg").args(["-nostats", "-i"]).arg(file).args(["-map", "0:a", "-af", "ebur128", "-f", "null", "-"]).output().map_err(|e| format!("ffmpeg: {e}"))?;
    String::from_utf8_lossy(&o.stderr)
        .lines()
        .filter_map(|l| l.trim().strip_prefix("I:")?.trim().strip_suffix("LUFS")?.trim().parse::<f64>().ok())
        .next_back()
        .ok_or_else(|| format!("no loudness measured for {}", file.display()))
}

fn duration(file: &Path) -> Result<f64> {
    let o = Command::new("ffprobe").args(["-v", "error", "-show_entries", "format=duration", "-of", "csv=p=0"]).arg(file).output().map_err(|e| format!("ffprobe: {e}"))?;
    String::from_utf8_lossy(&o.stdout).trim().parse().map_err(|e| format!("duration of {}: {e}", file.display()))
}

fn needs(tools: &[&str]) -> Result {
    for t in tools {
        if Command::new(t).arg("-version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_err() {
            return Err(format!("needs {t} on the path"));
        }
    }
    Ok(())
}

/// The local time, formatted by `date` (std has no time zones).
fn now(format: &str) -> Result<String> {
    let o = Command::new("date").arg(format).output().map_err(|e| format!("date: {e}"))?;
    Ok(String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn absolute(p: &str) -> Result<PathBuf> {
    std::path::absolute(p).map_err(|e| format!("{p}: {e}"))
}

fn mkdir(p: &Path) -> Result {
    fs::create_dir_all(p).map_err(|e| format!("{}: {e}", p.display()))
}

fn append(p: &Path, line: &str) -> Result {
    fs::OpenOptions::new().create(true).append(true).open(p).and_then(|mut f| f.write_all(line.as_bytes())).map_err(|e| format!("{}: {e}", p.display()))
}
