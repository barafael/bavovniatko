//! The game's tasks, as `cargo xtask <task>` (the alias is in `.cargo/config.toml`).
//!
//! ```text
//! cargo xtask trailer [--scout] [out.mp4]      the trailer: the action of both days (design/trailer.md)
//! cargo xtask film [--scout] [out.mp4]         the showcase film of both (design/showcase.md)
//! cargo xtask tour [robotyne] [out.mp4]        the one-minute tour of one day
//! cargo xtask mix <dir> <out.wav> [lead-s] [gain-db]   rebuild a filmed run's soundtrack
//! ```
//!
//! The game films headless on a fixed step (`BAV_SHOWCASE`, `crates/bavovniatko/src/showcase.rs`) and logs every
//! sound it plays; this builds it, films the parts, renders the cards, rebuilds each part's soundtrack from the log
//! (`mix.rs`), levels the parts, and encodes and joins them. `--scout` films at 640x360 with contact sheets, for
//! retiming shots. Needs ffmpeg, ffprobe and ImageMagick's `magick` on the path. The work dir holds thousands of
//! 1080p frames (tens of GB): it goes under `devlog/`, on the disk, never in `target/` or on a tmpfs.

mod mix;
mod reel;

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
    time::{Duration, Instant},
};

type Result<T = ()> = std::result::Result<T, String>;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let run = match args.first().map(String::as_str) {
        Some("trailer") => trailer(&args[1..]),
        Some("film") => film(&args[1..]),
        Some("tour") => tour(&args[1..]),
        Some("mix") => remix(&args[1..]),
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
  cargo xtask mix <dir> <out.wav> [lead-s] [gain-db]   rebuild a filmed run's soundtrack";

/// The game's directory: the workspace root, one up from this crate.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("xtask sits in the workspace").to_path_buf()
}

/// The showcase film: both dioramas, cards over black between the shots, each part's sound rebuilt and levelled,
/// then title, part one, part two and end card joined.
fn film(args: &[String]) -> Result {
    let scout = args.iter().any(|a| a == "--scout");
    let out_arg = args.iter().find(|a| !a.starts_with("--"));
    needs(&["ffmpeg", "ffprobe", "magick"])?;
    let root = root();
    let name = "film";
    let stamp = now("+%Y%m%d-%H%M")?;
    let (res, crf, preset) = if scout { ((640, 360), 30, "veryfast") } else { ((1920, 1080), 27, "slow") };
    let out = match out_arg {
        Some(o) => absolute(o)?,
        None if scout => root.join(format!("devlog/film/{stamp}-{name}-scout.mp4")),
        None => root.join(format!("devlog/videos/{stamp}-{name}.mp4")),
    };
    let logs = root.join("devlog/film");
    mkdir(&logs)?;
    mkdir(out.parent().unwrap_or(&root))?;
    build(&root)?;
    let work = WorkDir::new(&root)?;
    let w = &work.0;
    let card = Cards::new(&root, res);

    // 1. The two parts, one after the other: they share the GPU.
    for part in ["vuhledar", "robotyne"] {
        let dir = w.join(part);
        mkdir(&dir)?;
        let env = vec![("BAV_DIORAMA", part.to_string()), ("BAV_TOUR", name.into()), ("BAV_RES", format!("{}x{}", res.0, res.1)), ("BAV_SHOWCASE", dir.display().to_string())];
        let start = Instant::now();
        film_part(&root, &env, &logs.join(format!("{name}-{part}.log")))?;
        println!("film: {part} filmed in {} min at {}x{}", start.elapsed().as_secs() / 60, res.0, res.1);
    }

    // 2. The cards: black frames with a title, a smaller line and a dim note, linked into the frame ranges the
    //    tour reserved for them (before each shot's first frame).
    for part in ["vuhledar", "robotyne"] {
        let dir = w.join(part);
        for line in fs::read_to_string(dir.join("cards.txt")).unwrap_or_default().lines().filter(|l| !l.is_empty()) {
            let f: Vec<&str> = line.split('\t').collect();
            let mut head = f[0].split_whitespace();
            let (vt, secs): (f64, f64) = (num(head.next())?, num(head.next())?);
            let (first, last) = ((vt * 30.0).round() as u32, ((vt + secs) * 30.0).round() as u32);
            let png = dir.join(format!("card-{first}.png"));
            card.render(&png, f.get(1).unwrap_or(&""), f.get(2).unwrap_or(&""), f.get(3).unwrap_or(&""))?;
            for i in first..last {
                let frame = dir.join(format!("f{i:05}.png"));
                let _ = fs::remove_file(&frame);
                fs::hard_link(&png, &frame).map_err(|e| format!("{}: {e}", frame.display()))?;
            }
        }
    }

    // 3. Each part's sound, levelled: mixed once and measured (EBU R128, integrated), then mixed again with the gain
    //    that brings it to -18 LUFS, so the parts sit within a loudness point whatever the day's bursts did. Then
    //    identical encodes, so the join needs no re-encode.
    let enc = |crf: u32| ["-c:v", "libx264", "-crf", &crf.to_string(), "-preset", preset, "-pix_fmt", "yuv420p", "-c:a", "aac", "-b:a", "160k"].map(String::from);
    for part in ["vuhledar", "robotyne"] {
        let (dir, wav) = (w.join(part), w.join(format!("{part}.wav")));
        mix::mix(&dir, &wav, 0.0, 0.0)?;
        let level = loudness(&wav)?;
        let gain = -18.0 - level;
        println!("level {part}: {level:.1} LUFS, gain {gain:+.1} dB");
        let m = mix::mix(&dir, &wav, 0.0, gain as f32)?;
        println!("mixed {part}: {:.1} s, {} sounds logged, {} cards", m.secs, m.sounds, m.cards);
        let mut a: Vec<String> = ["-framerate", "30", "-i"].map(String::from).to_vec();
        a.push(dir.join("f%05d.png").display().to_string());
        a.extend(["-i".into(), wav.display().to_string()]);
        a.extend(enc(crf));
        a.extend(["-shortest".into(), w.join(format!("{part}.mp4")).display().to_string()]);
        ffmpeg(&a)?;
    }
    for part in ["vuhledar", "robotyne"] {
        println!("loudness {part}: {:.1} LUFS", loudness(&w.join(format!("{part}.mp4")))?);
    }

    // 4. Title and end card over a bed of wind and far-off guns; a fade in on the title, out on the end. The end
    //    card reads both parts' ledgers: four rows each, then the closing lines verbatim.
    mix::write_wav(&w.join("bed-title.wav"), &mix::bed(&w.join("vuhledar"), 5 * mix::RATE, 0.0)?)?;
    mix::write_wav(&w.join("bed-end.wav"), &mix::bed(&w.join("robotyne"), 10 * mix::RATE, 0.0)?)?;
    card.render(&w.join("title.png"), "BAVOVNIATKO", "Two dioramas of the Russo-Ukrainian war", "Everything here is simulated, and seen through a Ukrainian drone.")?;
    fs::write(w.join("end.txt"), end_card(w)?).map_err(|e| e.to_string())?;
    card.text_file(&w.join("end.png"), &w.join("end.txt"))?;
    for (still, secs, fades, bed, mp4) in [("title.png", 5, "fade=t=in:st=0:d=0.8,fade=t=out:st=4.4:d=0.6", "bed-title.wav", "title.mp4"), ("end.png", 10, "fade=t=in:st=0:d=0.5,fade=t=out:st=9:d=1", "bed-end.wav", "end.mp4")] {
        let mut a: Vec<String> = ["-loop", "1", "-t", &secs.to_string(), "-framerate", "30", "-i"].map(String::from).to_vec();
        a.extend([w.join(still).display().to_string(), "-i".into(), w.join(bed).display().to_string(), "-vf".into(), fades.into()]);
        a.extend(enc(crf));
        a.extend(["-shortest".into(), w.join(mp4).display().to_string()]);
        ffmpeg(&a)?;
    }
    let list: String = ["title", "vuhledar", "robotyne", "end"].iter().map(|p| format!("file '{}'\n", w.join(format!("{p}.mp4")).display())).collect();
    fs::write(w.join("list.txt"), list).map_err(|e| e.to_string())?;
    ffmpeg(&["-f", "concat", "-safe", "0", "-i", &w.join("list.txt").display().to_string(), "-c", "copy", &out.display().to_string()].map(String::from))?;

    // 5. The scout's contact sheets, one tile per second, for retiming the shots.
    if scout {
        let stem = out.file_stem().unwrap_or_default().to_string_lossy().to_string();
        for part in ["vuhledar", "robotyne"] {
            let sheet = logs.join(format!("{stem}-{part}-sheet-%02d.png"));
            ffmpeg(&["-i", &w.join(format!("{part}.mp4")).display().to_string(), "-vf", "fps=1,scale=240:-1,tile=8x15", &sheet.display().to_string()].map(String::from))?;
        }
    } else {
        let what = "The showcase film (design/showcase.md): title, Vuhledar with its cards, the intertitle, Mala Tokmachka, the end card from both ledgers.";
        let file = out.file_name().unwrap_or_default().to_string_lossy();
        append(&root.join("devlog/videos.md"), &format!("| {} | [{file}](videos/{file}) | {what} |\n", now("+%Y-%m-%d %H:%M")?))?;
    }
    drop(work);
    println!("{name}: {} ({:.0} MB, {:.0} s)", out.display(), fs::metadata(&out).map(|m| m.len() as f64 / 1e6).unwrap_or(0.0), duration(&out)?);
    Ok(())
}

/// The trailer: the action of both days, found by an events pass and filmed in as many runs as it takes
/// (`reel.rs`, design/trailer.md), cut together: a title, Vuhledar, Mala Tokmachka, an end card.
fn trailer(args: &[String]) -> Result {
    let scout = args.iter().any(|a| a == "--scout");
    let out_arg = args.iter().find(|a| !a.starts_with("--"));
    needs(&["ffmpeg", "ffprobe", "magick"])?;
    let root = root();
    let stamp = now("+%Y%m%d-%H%M")?;
    // For a big screen: less grain lost to the encoder than the film's 27.
    let (res, crf, preset) = if scout { ((640, 360), 30, "veryfast") } else { ((1920, 1080), 24, "slow") };
    let out = match out_arg {
        Some(o) => absolute(o)?,
        None if scout => root.join(format!("devlog/film/{stamp}-trailer-scout.mp4")),
        None => root.join(format!("devlog/videos/{stamp}-trailer.mp4")),
    };
    let logs = root.join("devlog/film");
    mkdir(&logs)?;
    mkdir(out.parent().unwrap_or(&root))?;
    build(&root)?;
    let work = WorkDir::new(&root)?;
    let w = &work.0;
    let card = Cards::new(&root, res);
    let enc = |crf: u32| ["-c:v", "libx264", "-crf", &crf.to_string(), "-preset", preset, "-pix_fmt", "yuv420p", "-c:a", "aac", "-b:a", "160k"].map(String::from);

    // 1. The events passes, both days at once at a postage-stamp size: nothing is filmed, every blast and hit is
    //    logged.
    let start = Instant::now();
    std::thread::scope(|s| -> Result {
        let runs: Vec<_> = reel::DAYS
            .iter()
            .map(|&reel::Day { part, plan, end, extra }| {
                let (dir, file, log) = (w.join(format!("events-{part}")), w.join(format!("events-{part}.reel")), logs.join(format!("reel-events-{part}.log")));
                let root = &root;
                s.spawn(move || -> Result {
                    mkdir(&dir)?;
                    fs::write(&file, format!("{plan}end {end}\n")).map_err(|e| e.to_string())?;
                    film_part(root, &env_for(part, extra, &file, &dir, (320, 180)), &log)
                })
            })
            .collect();
        for r in runs {
            r.join().map_err(|_| "an events pass panicked".to_string())??;
        }
        Ok(())
    })?;
    println!("trailer: events passes in {} min", start.elapsed().as_secs() / 60);

    // 2. Each day's moments, filmed in runs; each run's sound rebuilt and levelled to -18 LUFS, and the run encoded;
    //    then each moment cut out of its run.
    let mut clips: Vec<(String, Vec<PathBuf>)> = Vec::new();
    for &reel::Day { part, plan, extra, .. } in &reel::DAYS {
        let events = w.join(format!("events-{part}/events.txt"));
        let _ = fs::copy(&events, logs.join(format!("reel-events-{part}.txt")));
        let moments = reel::moments(&events)?;
        let order: Vec<String> = reel::order(moments.clone()).into_iter().map(|m| m.line).collect();
        let runs = reel::runs(moments);
        println!("trailer: {part}: {} moments in {} runs", order.len(), runs.len());
        let mut cut: Vec<(String, PathBuf)> = Vec::new();
        for (k, run) in runs.iter().enumerate() {
            let dir = w.join(format!("{part}-{k}"));
            mkdir(&dir)?;
            let file = w.join(format!("{part}-{k}.reel"));
            let last = run.iter().map(|m| m.from + m.secs).fold(0.0, f32::max);
            let shots: String = run.iter().map(|m| format!("{}\n", m.line)).collect();
            fs::write(&file, format!("{plan}{shots}end {:.2}\n", last + 1.0)).map_err(|e| e.to_string())?;
            let start = Instant::now();
            film_part(&root, &env_for(part, extra, &file, &dir, res), &logs.join(format!("reel-{part}-{k}.log")))?;
            println!("film: {part} run {k}, {} shots, in {} min", run.len(), start.elapsed().as_secs() / 60);
            let wav = w.join(format!("{part}-{k}.wav"));
            mix::mix(&dir, &wav, 0.0, 0.0)?;
            let gain = -18.0 - loudness(&wav)?;
            mix::mix(&dir, &wav, 0.0, gain as f32)?;
            let mp4 = w.join(format!("{part}-{k}.mp4"));
            let mut a: Vec<String> = ["-framerate", "30", "-i"].map(String::from).to_vec();
            a.extend([dir.join("f%05d.png").display().to_string(), "-i".into(), wav.display().to_string()]);
            a.extend(enc(crf));
            a.extend(["-shortest".into(), mp4.display().to_string()]);
            ffmpeg(&a)?;
            // The run's shots are its segments, in time order: where each starts in the run's video.
            let segs: Vec<f64> = fs::read_to_string(dir.join("segments.txt")).unwrap_or_default().lines().filter_map(|l| l.split_whitespace().next()?.parse().ok()).collect();
            for (i, m) in run.iter().enumerate() {
                let Some(&vs) = segs.get(i) else { continue };
                let clip = w.join(format!("{part}-{k}-{i}.mp4"));
                let fade = format!("afade=t=in:d=0.04,afade=t=out:st={:.2}:d=0.12", m.secs - 0.12);
                let mut a: Vec<String> = vec!["-ss".into(), format!("{vs:.3}"), "-t".into(), format!("{:.3}", m.secs), "-i".into(), mp4.display().to_string(), "-af".into(), fade];
                a.extend(enc(crf));
                a.push(clip.display().to_string());
                ffmpeg(&a)?;
                cut.push((m.line.clone(), clip));
            }
            for f in fs::read_dir(&dir).map_err(|e| e.to_string())?.flatten() {
                if f.file_name().to_string_lossy().ends_with(".png") {
                    let _ = fs::remove_file(f.path());
                }
            }
        }
        let ordered = order.iter().filter_map(|line| cut.iter().find(|(l, _)| l == line).map(|(_, c)| c.clone())).collect();
        clips.push((part.to_string(), ordered));
    }

    // 3. The cards, over a bed of wind and far-off guns, and the cut: title, Vuhledar's card and moments, Mala
    //    Tokmachka's, the end.
    let bed_dir = w.join("events-vuhledar");
    let still = |name: &str, secs: f64, title: &str, line: &str, note: &str| -> Result<PathBuf> {
        let (png, wav, mp4) = (w.join(format!("{name}.png")), w.join(format!("{name}.wav")), w.join(format!("{name}.mp4")));
        card.render(&png, title, line, note)?;
        mix::write_wav(&wav, &mix::bed(&bed_dir, (secs * mix::RATE as f64) as usize, 0.0)?)?;
        let fades = format!("fade=t=in:st=0:d=0.4,fade=t=out:st={:.2}:d=0.4", secs - 0.4);
        let mut a: Vec<String> = vec!["-loop".into(), "1".into(), "-t".into(), format!("{secs}"), "-framerate".into(), "30".into(), "-i".into(), png.display().to_string(), "-i".into(), wav.display().to_string(), "-vf".into(), fades];
        a.extend(enc(crf));
        a.extend(["-shortest".into(), mp4.display().to_string()]);
        ffmpeg(&a)?;
        Ok(mp4)
    };
    let mut list = vec![still("title", 3.5, "BAVOVNIATKO", "One minefield, seen from both sides", "Two days of the Russo-Ukrainian war, simulated, and seen through a Ukrainian drone.")?];
    for (part, cs) in &clips {
        let (title, line, note) = match part.as_str() {
            "vuhledar" => ("ВУГЛЕДАР (VUHLEDAR)", "6 February 2023", "Russian columns into the 72nd Brigade's minefields."),
            _ => ("МАЛА ТОКМАЧКА (MALA TOKMACHKA)", "8 June 2023", "The 47th Brigade into the Russian minefields."),
        };
        list.push(still(&format!("card-{part}"), 2.5, title, line, note)?);
        list.extend(cs.iter().cloned());
    }
    list.push(still("end", 4.0, "BAVOVNIATKO", "One minefield, seen from both sides", "Everything here is simulated.")?);
    let txt: String = list.iter().map(|p| format!("file '{}'\n", p.display())).collect();
    fs::write(w.join("list.txt"), txt).map_err(|e| e.to_string())?;
    ffmpeg(&["-f", "concat", "-safe", "0", "-i", &w.join("list.txt").display().to_string(), "-c", "copy", &out.display().to_string()].map(String::from))?;
    println!("loudness: {:.1} LUFS", loudness(&out)?);
    if scout {
        let sheet = logs.join(format!("{}-sheet-%02d.png", out.file_stem().unwrap_or_default().to_string_lossy()));
        ffmpeg(&["-i", &out.display().to_string(), "-vf", "fps=1,scale=240:-1,tile=8x15", &sheet.display().to_string()].map(String::from))?;
    } else {
        let file = out.file_name().unwrap_or_default().to_string_lossy();
        append(&root.join("devlog/videos.md"), &format!("| {} | [{file}](videos/{file}) | The trailer (design/trailer.md): the action of both days, found by an events pass. |\n", now("+%Y-%m-%d %H:%M")?))?;
    }
    drop(work);
    println!("trailer: {} ({:.0} MB, {:.0} s)", out.display(), fs::metadata(&out).map(|m| m.len() as f64 / 1e6).unwrap_or(0.0), duration(&out)?);
    Ok(())
}

/// A reel run's switches: the day, the reel's file, the size, where it films to, and the day's own.
fn env_for(part: &str, extra: &'static [(&'static str, &'static str)], reel: &Path, dir: &Path, size: (u32, u32)) -> Vec<(&'static str, String)> {
    let mut env = vec![("BAV_DIORAMA", part.to_string()), ("BAV_TOUR", "reel".into()), ("BAV_REEL", reel.display().to_string()), ("BAV_RES", format!("{}x{}", size.0, size.1)), ("BAV_SHOWCASE", dir.display().to_string())];
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
    let m = mix::mix(w, &w.join("sound.wav"), lead, 0.0)?;
    println!("mixed {:.1} s, {} sounds logged", m.secs, m.sounds);
    ffmpeg(&[
        "-loop", "1", "-t", "3", "-framerate", "30", "-i", &w.join("title.png").display().to_string(),
        "-framerate", "30", "-i", &w.join("f%05d.png").display().to_string(),
        "-i", &w.join("sound.wav").display().to_string(),
        "-filter_complex", "[0:v]fade=t=in:st=0:d=0.8,fade=t=out:st=2.4:d=0.6,format=yuv420p[t];[1:v]format=yuv420p[g];[t][g]concat=n=2:v=1:a=0[v]",
        "-map", "[v]", "-map", "2:a", "-c:v", "libx264", "-crf", "27", "-preset", "slow", "-tune", "grain", "-c:a", "aac", "-b:a", "160k", "-shortest",
        &out.display().to_string(),
    ].map(String::from))?;
    drop(work);
    println!("tour: {} ({:.0} MB)", out.display(), fs::metadata(&out).map(|m| m.len() as f64 / 1e6).unwrap_or(0.0));
    Ok(())
}

/// `cargo xtask mix <dir> <out.wav> [lead-s] [gain-db]`: a filmed run's soundtrack again, to tune the mix.
fn remix(args: &[String]) -> Result {
    let [dir, out, rest @ ..] = args else { return Err(USAGE.into()) };
    let lead = rest.first().map(|s| s.parse::<f64>().map_err(|e| format!("lead {s}: {e}"))).transpose()?.unwrap_or(0.0);
    let gain = rest.get(1).map(|s| s.parse::<f32>().map_err(|e| format!("gain {s}: {e}"))).transpose()?.unwrap_or(0.0);
    let m = mix::mix(Path::new(dir), Path::new(out), lead, gain)?;
    println!("mixed {:.1} s, {} sounds logged, {} cards -> {out}", m.secs, m.sounds, m.cards);
    Ok(())
}

/// The game's release build: what ships, and what the films are made with.
fn build(root: &Path) -> Result {
    let ok = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into())).args(["build", "-q", "--release", "-p", "bavovniatko"]).current_dir(root).status().map_err(|e| format!("cargo: {e}"))?.success();
    if ok { Ok(()) } else { Err("the game's release build failed".into()) }
}

/// Film one part headless: the game's log, without its colour codes and its thousands of "Screenshot saved", goes
/// to `log`; its panics and errors are shown. A run that has not finished in a little over an hour is stopped.
fn film_part(root: &Path, env: &[(&str, String)], log: &Path) -> Result {
    let mut cmd = Command::new(root.join("target/release/bavovniatko"));
    cmd.current_dir(root)
        .envs([("BAV_HEADLESS", "1"), ("BAV_SKIP_BRIEFING", "1"), ("BAV_NO_BATTERY", "1"), ("BAV_OPERATOR", "off")])
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
    let deadline = Instant::now() + Duration::from_secs(4000);
    loop {
        if child.try_wait().map_err(|e| e.to_string())?.is_some() {
            break;
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            eprintln!("film: stopped a run that did not finish in time");
            break;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    for r in readers {
        let _ = r.join();
    }
    let lines = lines.lock().unwrap();
    fs::write(log, lines.join("\n") + "\n").map_err(|e| format!("{}: {e}", log.display()))?;
    for (i, l) in lines.iter().enumerate().filter(|(_, l)| (l.contains("panicked") || l.contains("ERROR")) && !l.contains("rodio")).take(5) {
        eprintln!("{l}");
        if l.contains("panicked") {
            lines.iter().skip(i + 1).take(3).for_each(|l| eprintln!("{l}"));
        }
    }
    Ok(())
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

fn ffmpeg(args: &[String]) -> Result {
    let ok = Command::new("ffmpeg").args(["-loglevel", "error", "-y"]).args(args).status().map_err(|e| format!("ffmpeg: {e}"))?.success();
    if ok { Ok(()) } else { Err(format!("ffmpeg failed: {}", args.join(" "))) }
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

fn num(s: Option<&str>) -> Result<f64> {
    s.and_then(|s| s.parse().ok()).ok_or_else(|| "cards.txt: a line without its time".into())
}
