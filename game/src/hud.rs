//! The screen: drone telemetry, the radio net, fire control, the tally, the briefing and the debrief.
//! The debrief puts the day next to the record, with every claimant named (dossier §1, §10).

use crate::camera::{Mode, View};
use crate::director::{self, Script};
use crate::fire::{AtgmMode, FireControl, UaPosition};
use crate::geo;
use crate::units::{Radio, Stats};
use crate::Phase;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, (update_hud, briefing_input))
            .add_systems(OnEnter(Phase::Playing), hide_briefing)
            .add_systems(OnEnter(Phase::Debrief), show_debrief)
            .add_systems(Update, debrief_input.run_if(in_state(Phase::Debrief)));
    }
}

#[derive(Component)]
struct Telemetry;
#[derive(Component)]
struct Tally;
#[derive(Component)]
struct RadioLog;
#[derive(Component)]
struct FirePanel;
#[derive(Component)]
struct Banner;
#[derive(Component)]
struct NoSignal;
#[derive(Component)]
struct Crosshair;
#[derive(Component)]
struct Briefing;
#[derive(Component)]
struct Debrief;
#[derive(Component)]
struct DebriefText;

const AMBER: Color = Color::srgb(0.95, 0.85, 0.55);
const WHITE: Color = Color::srgba(0.95, 0.96, 0.97, 0.92);
const DIM: Color = Color::srgba(0.85, 0.87, 0.9, 0.7);

fn text(s: &str, size: f32, color: Color) -> (Text, TextFont, TextColor) {
    (Text::new(s), TextFont { font_size: FontSize::Px(size), ..default() }, TextColor(color))
}

fn spawn_hud(mut commands: Commands) {
    // Larger offscreen renders (the showcase films at 1080p) scale the interface with them.
    if let Some(h) = std::env::var("VUHLEDAR_RES").ok().and_then(|r| r.split_once('x').and_then(|(_, h)| h.parse::<f32>().ok())) {
        commands.insert_resource(UiScale(1.35 * h / 900.0));
    }
    let abs = |top: Option<f32>, bottom: Option<f32>, left: Option<f32>, right: Option<f32>| Node {
        position_type: PositionType::Absolute,
        top: top.map(px).unwrap_or(Val::Auto),
        bottom: bottom.map(px).unwrap_or(Val::Auto),
        left: left.map(px).unwrap_or(Val::Auto),
        right: right.map(px).unwrap_or(Val::Auto),
        ..default()
    };
    commands.spawn((text("", 15.0, WHITE), abs(Some(14.0), None, Some(18.0), None), Telemetry));
    commands.spawn((text("", 15.0, WHITE), abs(Some(14.0), None, None, Some(18.0)), Tally, TextLayout::justify(Justify::Right)));
    commands.spawn((text("", 14.0, AMBER), Node { max_width: px(620), ..abs(None, Some(16.0), Some(18.0), None) }, RadioLog));
    commands.spawn((text("", 14.0, WHITE), abs(None, Some(16.0), None, Some(18.0)), FirePanel, TextLayout::justify(Justify::Right)));
    commands.spawn((
        text("", 26.0, WHITE),
        Node { position_type: PositionType::Absolute, top: percent(16), width: percent(100), justify_content: JustifyContent::Center, ..default() },
        TextLayout::justify(Justify::Center),
        Banner,
    ));
    commands.spawn((
        text("", 30.0, WHITE),
        Node { position_type: PositionType::Absolute, top: percent(46), width: percent(100), ..default() },
        TextLayout::justify(Justify::Center),
        NoSignal,
    ));
    // Drone crosshair.
    for (w, h, dx, dy) in [(26.0, 2.0, -40.0, -1.0), (26.0, 2.0, 14.0, -1.0), (2.0, 26.0, -1.0, -40.0), (2.0, 26.0, -1.0, 14.0)] {
        commands.spawn((
            Node { position_type: PositionType::Absolute, left: percent(50), top: percent(50), width: px(w), height: px(h), margin: UiRect { left: px(dx), top: px(dy), ..default() }, ..default() },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.75)),
            Crosshair,
        ));
    }

    // The briefing.
    let briefing = "VUHLEDAR\n6 FEBRUARY 2023\n\n\
Vuhledar is a coal-mining town of Soviet apartment blocks on a ridge above open, flat fields.\n\
Since late October the Pacific Fleet's 155th Naval Infantry Brigade has attacked it, and been cut to pieces.\n\
On 24 January Russia attacked again. Today is the largest assault yet.\n\n\
You are with the 72nd Mechanised Brigade: a drone operator and a fire controller.\n\
The roads are left clear. The fields beside them are mined. ATGM teams wait in the tree lines.\n\
The day will run whether or not you do anything. What you do matters at the margin.\n\n\
Mouse: right-drag to look, wheel to zoom.  WASD: fly the drone.  Q/E: turn.  R/F: climb/descend.\n\
Left click: 155 mm fire mission (4 rounds).  Shift+click: RAAM mines.  Ctrl+click: HIMARS.\n\
Space: ATGM teams AUTO / HOLD / FREE.  1/2 or Tab: drone / rooftop observer.  P: pause.  [ ]: time.\n\n\
Press Enter.";
    commands
        .spawn((
            Node { position_type: PositionType::Absolute, width: percent(100), height: percent(100), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.025, 0.86)),
            GlobalZIndex(10),
            Briefing,
        ))
        .with_child((text(briefing, 17.0, WHITE), TextLayout::justify(Justify::Center), Node { max_width: px(980), ..default() }));

    commands
        .spawn((
            Node { position_type: PositionType::Absolute, width: percent(100), height: percent(100), justify_content: JustifyContent::Center, align_items: AlignItems::Center, display: Display::None, ..default() },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.025, 0.94)),
            GlobalZIndex(10),
            Debrief,
        ))
        .with_child((text("", 15.0, WHITE), Node { max_width: px(1100), ..default() }, DebriefText));
}

fn briefing_input(keys: Res<ButtonInput<KeyCode>>, phase: Res<State<Phase>>, mut next: ResMut<NextState<Phase>>) {
    if *phase.get() == Phase::Briefing && (keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) || std::env::var("VUHLEDAR_SKIP_BRIEFING").is_ok()) {
        next.set(Phase::Playing);
    }
}

fn hide_briefing(mut q: Query<&mut Node, With<Briefing>>) {
    for mut n in &mut q {
        n.display = Display::None;
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn update_hud(
    time: Res<Time>,
    vtime: Res<Time<Virtual>>,
    view: Res<View>,
    script: Res<Script>,
    stats: Res<Stats>,
    fc: Res<FireControl>,
    radio: Res<Radio>,
    positions: Query<&UaPosition>,
    mut texts: ParamSet<(
        Single<&mut Text, With<Telemetry>>,
        Single<&mut Text, With<Tally>>,
        Single<&mut Text, With<RadioLog>>,
        Single<&mut Text, With<FirePanel>>,
        Single<&mut Text, With<Banner>>,
        Single<&mut Text, With<NoSignal>>,
    )>,
    mut crosshair: Query<&mut Visibility, With<Crosshair>>,
    phase: Res<State<Phase>>,
) {
    let now = time.elapsed_secs_f64();
    let rec = script.t as u32;
    let p2 = Vec2::new(view.drone_pos.x, view.drone_pos.z);
    let (lat, lon) = geo::latlon(p2);
    let agl = view.drone_pos.y - geo::height(p2.x, p2.y);
    let home = Vec2::new(-400.0, -1300.0);
    **texts.p0() = match view.mode {
        Mode::Drone => Text::new(format!(
            "REC {:02}:{:02}:{:02}   BAT {:>3.0}%   H {:>3.0} m   D {:.2} km   {:.5} N {:.5} E   x{:.1}{}\nDRONES LEFT {}",
            rec / 3600, rec / 60 % 60, rec % 60, view.battery * 100.0, agl, p2.distance(home) / 1000.0, lat, lon, view.zoom(),
            if view.jamming > 0.3 { "   SIGNAL WEAK" } else { "" },
            view.drones_left
        )),
        Mode::Op => Text::new(format!("OBSERVATION POST, 9TH FLOOR   x{:.1}", view.zoom())),
    };
    let committed = stats.vehicles_committed;
    let out = stats.destroyed + stats.abandoned;
    **texts.p1() = Text::new(format!(
        "06 FEB 2023   {}\nRussian vehicles committed {}   knocked out {}   withdrew {}\nmen sent {}   came back {}",
        director::clock_text(script.t), committed, out, stats.withdrawn, stats.sent, stats.returned
    ));
    let mut log = String::new();
    for (t, line) in radio.lines.iter().rev().take(7).collect::<Vec<_>>().into_iter().rev() {
        let age = now - t;
        if age < 50.0 {
            log.push_str(&format!("[{}] {}\n", director::clock_text((script.t as f64 - age).max(0.0) as f32), line));
        }
    }
    **texts.p2() = Text::new(log);
    let teams = positions.iter().filter(|p| p.atgm > 0 && p.strength >= 2).count();
    let missiles: u32 = positions.iter().filter(|p| p.strength >= 2).map(|p| p.atgm).sum();
    let mode = match fc.atgm_mode {
        AtgmMode::Auto => "AUTO",
        AtgmMode::Hold => "HOLD",
        AtgmMode::Free => "FREE",
    };
    let speed = vtime.relative_speed();
    **texts.p3() = Text::new(format!(
        "155 mm  {} rds   RAAM  {}   HIMARS  {}\nATGM teams {} ({} missiles)   [{}]{}",
        fc.shells, fc.raam, fc.himars, teams, missiles, mode,
        if (speed - 1.0).abs() > 0.01 { format!("\ntime x{speed}") } else { String::new() }
    ));
    **texts.p4() = Text::new(script.banner.as_ref().map(|b| b.0.clone()).unwrap_or_default());
    **texts.p5() = Text::new(if view.mode == Mode::Drone && view.feed_down > 0.0 {
        format!("NO SIGNAL\nbattery swap, relaunching ({:.0} s)", view.feed_down)
    } else {
        String::new()
    });
    for mut v in &mut crosshair {
        *v = if view.mode == Mode::Drone && view.feed_down <= 0.0 && *phase.get() == Phase::Playing { Visibility::Inherited } else { Visibility::Hidden };
    }
}

fn show_debrief(stats: Res<Stats>, fc: Res<FireControl>, mut root: Query<&mut Node, With<Debrief>>, mut t: Single<&mut Text, With<DebriefText>>) {
    for mut n in &mut root {
        n.display = Display::Flex;
    }
    let out = stats.destroyed + stats.abandoned;
    let closest = (1130.0 - stats.max_advance).max(0.0);
    let town_held = stats.foothold < 6;
    let in_band = (22..=40).contains(&out);
    let verdict = if town_held && in_band {
        "HISTORICAL. Vuhledar held, and the columns died in the fields as they did."
    } else if !town_held {
        "NOT WHAT HAPPENED. Russian infantry hold the first tree line. In February 2023 they never did."
    } else if out < 22 {
        "NOT QUITE WHAT HAPPENED. Too many vehicles got away. The record is worse for Russia than this."
    } else {
        "HELD, AND MORE. Losses above even the Ukrainian claim for the week."
    };
    let mut waves = String::new();
    for (i, w) in stats.per_wave.iter().enumerate().skip(1) {
        waves.push_str(&format!("   column {}: {} vehicles, {} destroyed, {} abandoned\n", i, w.0, w.1, w.2));
    }
    **t = Text::new(format!(
        "THE DAY\n\n\
Russian vehicles committed  {}\n{}\
Knocked out  {}  ({} destroyed, {} abandoned; {} on mines, {} by ATGM, {} by artillery; {} turrets thrown)\n\
Withdrew  {}\n\
Men sent forward  {}.  Came back  {}.  Russian dead  {}.\n\
Closest any Russian got to the town  {:.0} m.  Ukrainian casualties  {}.\n\
Shells left  {} of 96.\n\n\
THE RECORD\n\n\
ISW: one filmed engagement cost a 155th formation 13 tanks and 12 BMPs.\n\
UK MoD: at least 30 \"mostly intact\" armoured vehicles abandoned in a single incident.\n\
Oryx: 103 Russian vehicles, 36 of them tanks, at Avdiivka and Vuhledar on 8-10 February.\n\
Ukrainian military: 130 tanks and APCs over three weeks.\n\
A Russian officer's notebook, 1-5 March: 100, 116, 103, 115 men sent; 16, 23, 15, 3 came back.\n\
ISW, 26 March: \"effectively no gains at staggering costs.\"\n\
Girkin: \"Only morons attack head-on in the same place.\"\n\n\
{}\n\n\
The 155th was rebuilt, and attacked again. Russia kept coming at the town's front for another year and a half.\n\
Vuhledar fell on 1-2 October 2024, to an attack on its flanks, not its front. The 72nd Brigade was never rotated.\n\n\
R: the same day again, down the same roads.     Esc: quit.",
        stats.vehicles_committed, waves, out, stats.destroyed, stats.abandoned, stats.mine_hits, stats.atgm_kills, stats.artillery_kills, stats.turret_tosses,
        stats.withdrawn, stats.sent, stats.returned, stats.ru_dead, closest, stats.ua_losses, fc.shells, verdict
    ));
}

fn debrief_input(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyR) {
        if let Ok(exe) = std::env::current_exe() {
            let _ = std::process::Command::new(exe).env("VUHLEDAR_SKIP_BRIEFING", "1").spawn();
        }
        exit.write(AppExit::Success);
    }
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
