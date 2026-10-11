//! The screen: drone telemetry, the radio net, fire control, the tally, the briefing and the debrief.
//! The debrief is the day's own account. The record shaped the day; it does not grade it (design/next.md).

use crate::camera::{Mode, View};
use crate::director::{self, Script};
use crate::fire::{Atgm, AtgmMode, FireControl, JAVELIN_LOCK, Post};
use crate::geo;
use crate::diorama::{Diorama, diorama};
use crate::units::{ColumnOrder, Radio, Side, Stats};
use crate::Phase;
use bevy::{prelude::*, text::FontSource};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, (update_hud, briefing_input).chain().in_set(crate::Step::Screen))
            .add_systems(OnEnter(Phase::Playing), hide_briefing)
            .add_systems(OnEnter(Phase::Debrief), show_debrief)
            .add_systems(Update, debrief_input.run_if(in_state(Phase::Debrief)).in_set(crate::Step::Screen).after(briefing_input));
    }
}

#[derive(Component)]
struct Telemetry;
/// The drone's own display, as the controller app draws it over the video (design/next-items.md §2.6): the
/// status bar with its flashing warnings, the link and the fix, the battery, the zoom and the recording, and the
/// flight readouts round the compass. It never breaks up: the video does, the display does not.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum Osd {
    /// Top left: flight mode and status, or the warning.
    Status,
    /// Top right: satellites, the signal dots, the battery.
    Links,
    /// Right edge: zoom and recording.
    Camera,
    /// Bottom centre: heading, altitude, distance, speeds, gimbal.
    Flight,
}
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
struct Reticle;
#[derive(Component)]
struct Briefing;
#[derive(Component)]
struct Debrief;
#[derive(Component)]
struct DebriefText;

const AMBER: Color = Color::srgb(0.95, 0.85, 0.55);
const WHITE: Color = Color::srgba(0.95, 0.96, 0.97, 0.92);
/// The gloss under a Ukrainian line.
const DIM: Color = Color::srgba(0.80, 0.80, 0.78, 0.70);

fn text(font: &Handle<Font>, s: &str, size: f32, color: Color) -> (Text, TextFont, TextColor) {
    (Text::new(s), TextFont { font: FontSource::Handle(font.clone()), font_size: FontSize::Px(size), ..default() }, TextColor(color))
}

/// The interface's font, for text made after start-up.
#[derive(Resource)]
struct Fonts(Handle<Font>);

fn spawn_hud(mut commands: Commands, assets: Res<AssetServer>) {
    let font: Handle<Font> = assets.load(crate::text::FONT);
    commands.insert_resource(Fonts(font.clone()));
    let text = |s: &str, size: f32, color: Color| text(&font, s, size, color);
    // Larger offscreen renders (the showcase films at 1080p) scale the interface with them.
    if let Some((_, h)) = crate::dev::dev().res {
        commands.insert_resource(UiScale(1.35 * h as f32 / 900.0));
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
    // The drone's display: white with a dark shadow, monospaced, in the corners and along the bottom.
    let shadow = TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.8) };
    commands.spawn((text("", 15.0, WHITE), abs(Some(14.0), None, Some(18.0), None), shadow, Osd::Status));
    commands.spawn((text("", 15.0, WHITE), abs(Some(14.0), None, None, Some(18.0)), shadow, Osd::Links, TextLayout::justify(Justify::Right)));
    commands.spawn((text("", 16.0, WHITE), Node { top: percent(44), ..abs(None, None, None, Some(18.0)) }, shadow, Osd::Camera, TextLayout::justify(Justify::Right)));
    commands.spawn((
        text("", 14.0, WHITE),
        Node { position_type: PositionType::Absolute, bottom: px(16), width: percent(100), justify_content: JustifyContent::Center, ..default() },
        shadow,
        Osd::Flight,
        TextLayout::justify(Justify::Center),
    ));
    commands.spawn((text("", 15.0, WHITE), abs(Some(40.0), None, None, Some(18.0)), Tally, TextLayout::justify(Justify::Right)));
    commands.spawn((text("", 14.0, AMBER), Node { max_width: px(470), ..abs(None, Some(16.0), Some(18.0), None) }, RadioLog));
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

    // The ATGM sight's reticle: a long thin cross with stadia ticks.
    let line = |w: f32, h: f32, dx: f32, dy: f32| {
        (
            Node { position_type: PositionType::Absolute, left: percent(50), top: percent(50), width: px(w), height: px(h), margin: UiRect { left: px(dx), top: px(dy), ..default() }, ..default() },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
            Visibility::Hidden,
            Reticle,
        )
    };
    for (w, h, dx, dy) in [(260.0, 1.5, -290.0, -0.75), (260.0, 1.5, 30.0, -0.75), (1.5, 180.0, -0.75, 30.0), (1.5, 120.0, -0.75, -150.0)] {
        commands.spawn(line(w, h, dx, dy));
    }
    for k in 1..=4 {
        let y = 30.0 + k as f32 * 34.0;
        commands.spawn(line(18.0 - k as f32 * 2.0, 1.5, -(9.0 - k as f32), y));
    }

    // The briefing.
    let briefing = if diorama() == Diorama::Robotyne { ROBOTYNE_BRIEFING } else { "VUHLEDAR (ВУГЛЕДАР)\n6 FEBRUARY 2023\n\n\
Vuhledar is a coal-mining town of Soviet apartment blocks on a ridge above open, flat fields.\n\
Since late October the Pacific Fleet's 155th Naval Infantry Brigade (155-та ОБрМП) has attacked it, and been cut to pieces.\n\
On 24 January Russia attacked again. Today is the largest assault yet.\n\n\
You are with the 72nd Mechanised Brigade (72-га ОМБр, the Black Zaporozhians): a drone operator and a fire controller.\n\
The roads are left clear. The fields beside them are mined. ATGM teams wait in the tree lines.\n\
The day will run whether or not you do anything. What you do matters at the margin.\n\n\
Mouse: right-drag to look, wheel to zoom.  WASD: fly the drone.  Q/E: turn.  R/F: climb/descend.\n\
Left click: 155 mm fire mission (4 rounds).  Shift+click: RAAM mines.  Ctrl+click: HIMARS.\n\
Space: ATGM teams AUTO / HOLD / FREE.  1/2 or Tab: drone / rooftop observer.  P: pause.  [ ]: time.\n\
3: into a tree line with an ATGM team (again for the next).  Right button: the sight.  Left click: launch.\n\n\
Press Enter." };
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

const ROBOTYNE_BRIEFING: &str = "SOUTH OF MALA TOKMACHKA (МАЛА ТОКМАЧКА)\n8 JUNE 2023\n\n\
South of Orikhiv (Оріхів) the steppe climbs towards Tokmak (Токмак) through fields cut by tree lines.\n\
Since the autumn Russia has dug in across it: minefields hundreds of metres deep, a trench on every crest,\n\
guns registered on every track.\n\n\
Before dawn the 47th Mechanised Brigade (47-ма ОМБр «Маґура») goes forward with its new armour: Leopards and Bradleys,\n\
mine plows in front. The guns have finished their preparation. The column is late.\n\n\
You are with the brigade: a drone operator and a fire controller. These are our vehicles, and our men.\n\
The morning will run whether or not you do anything. What you do decides what it costs.\n\n\
Mouse: right-drag to look, wheel to zoom.  WASD: fly the drone.  Q/E: turn.  R/F: climb/descend.\n\
Left click: 155 mm fire mission (4 rounds).  Shift+click: smoke.  Ctrl+click: HIMARS.\n\
Space: the column PUSH ON / HOLD / PULL BACK.  1/2 or Tab: drone / the water tower.  P: pause.  [ ]: time.\n\
3: into the forward tree line with an ATGM team.  Right button: the sight.  Left click: launch.\n\n\
Press Enter.";

fn briefing_input(keys: Res<ButtonInput<KeyCode>>, phase: Res<State<Phase>>, mut next: ResMut<NextState<Phase>>) {
    if *phase.get() == Phase::Briefing && (keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) || crate::dev::dev().skip_briefing) {
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
    order: Res<ColumnOrder>,
    positions: Query<&Post>,
    mut texts: ParamSet<(
        Single<&mut Text, With<Telemetry>>,
        Single<&mut Text, With<Tally>>,
        Single<&mut Text, With<FirePanel>>,
        Single<&mut Text, With<NoSignal>>,
        Query<(&Osd, &mut Text, &mut Visibility)>,
    )>,
    mut crosshair: Query<&mut Visibility, (With<Crosshair>, Without<Reticle>, Without<Osd>)>,
    mut reticle: Query<&mut Visibility, (With<Reticle>, Without<Crosshair>, Without<Osd>)>,
    phase: Res<State<Phase>>,
    mut commands: Commands,
    net: (Res<Fonts>, Single<Entity, With<RadioLog>>, Single<Entity, With<Banner>>, Local<(String, String)>),
) {
    let (fonts, log_root, banner_root, mut shown) = net;
    osd_update(&time, &view, &script, &mut texts.p4());
    for mut v in &mut reticle {
        v.set_if_neq(if view.mode == Mode::Ground && view.sight { Visibility::Inherited } else { Visibility::Hidden });
    }
    let now = time.elapsed_secs_f64();
    // Each text is written only when it reads differently, so Bevy lays out again only what changed.
    let telemetry = match view.mode {
        Mode::Drone => Text::new(String::new()),
        Mode::Op => Text::new(format!("{}   x{:.1}", if diorama() == Diorama::Robotyne { "OBSERVER, WATER TOWER (СПОСТЕРІГАЧ, ВОДОНАПІРНА ВЕЖА)" } else { "OBSERVATION POST, 9TH FLOOR (СП, 9-Й ПОВЕРХ)" }, view.zoom())),
        Mode::Ground => {
            let team = view.ground_team.and_then(|e| positions.get(e).ok());
            let (weapon, left) = team.map(|p| (p.weapon, p.atgm)).unwrap_or((None, 0));
            let name = weapon.map_or("ATGM".to_string(), |w| w.to_string().to_uppercase());
            if view.sight {
                let how = if weapon == Some(Atgm::Javelin) {
                    match view.lock {
                        Some((_, t)) if t >= JAVELIN_LOCK => "LOCKED   [LMB] fire and forget".to_string(),
                        Some((_, t)) => format!("LOCKING {:.0}%   hold the seeker on him", t / JAVELIN_LOCK * 100.0),
                        None => "no target: put the crosshair on a vehicle".to_string(),
                    }
                } else {
                    "[LMB] launch, then hold the crosshair on him until impact".to_string()
                };
                Text::new(format!(
                    "{} SIGHT x10   RANGE {}   MISSILES {}\n{}",
                    name,
                    view.ground_range.map(|r| format!("{:.0} m", r)).unwrap_or("----".into()),
                    left,
                    how
                ))
            } else {
                Text::new(format!("TREE LINE, {} TEAM   MISSILES {}\n[RMB] sight   [3] next position   [1] drone", name, left))
            }
        }
    };
    texts.p0().set_if_neq(telemetry);
    let committed = stats.vehicles_committed;
    let out = stats.destroyed + stats.abandoned;
    texts.p1().set_if_neq(Text::new(match diorama() {
        Diorama::Vuhledar => format!(
            "{}   {}\nRussian vehicles committed {}   knocked out {}   withdrew {}\nmen sent {}   came back {}",
            diorama().date(), director::clock_text(script.t), committed, out, stats.withdrawn, stats.sent, stats.returned
        ),
        Diorama::Robotyne => format!(
            "{}   {}\nour vehicles committed {}   stopped in the field {}   came back {}\nmen sent {}   brought out {}",
            diorama().date(), director::clock_text(script.t), committed, out, stats.withdrawn, stats.sent, stats.returned
        ),
    }));
    // The net, in two registers: what was said, full weight; what it means, dimmer beneath it.
    let mut spans: Vec<(String, bool)> = Vec::new();
    for (t, line) in radio.lines.iter().rev().take(6).collect::<Vec<_>>().into_iter().rev() {
        let age = now - t;
        if age < 50.0 {
            let clock = director::clock_text((script.t as f64 - age).max(0.0) as f32);
            if line.uk.is_empty() {
                spans.push((format!("[{clock}] {}\n", line.en), true));
            } else {
                spans.push((format!("[{clock}] {}\n", line.uk), true));
                spans.push((format!("        {}\n", line.en), false));
            }
        }
    }
    let key = spans.iter().map(|(s, _)| s.as_str()).collect::<String>();
    if shown.0 != key {
        shown.0 = key;
        commands.entity(*log_root).despawn_children().with_children(|p| {
            for (s, uk) in &spans {
                p.spawn((TextSpan::new(s.clone()), TextFont { font: FontSource::Handle(fonts.0.clone()), font_size: FontSize::Px(if *uk { 14.0 } else { 12.0 }), ..default() }, TextColor(if *uk { AMBER } else { DIM })));
            }
        });
    }
    let teams = positions.iter().filter(|p| p.side == Side::Ua && p.atgm > 0 && p.strength >= 2).count();
    let missiles: u32 = positions.iter().filter(|p| p.side == Side::Ua && p.strength >= 2).map(|p| p.atgm).sum();
    let mode = match fc.atgm_mode {
        AtgmMode::Auto => "AUTO",
        AtgmMode::Hold => "HOLD",
        AtgmMode::Free => "FREE",
    };
    let speed = vtime.relative_speed();
    let speed_line = if (speed - 1.0).abs() > 0.01 { format!("\ntime x{speed}") } else { String::new() };
    texts.p2().set_if_neq(Text::new(match diorama() {
        Diorama::Vuhledar => format!("155 mm  {} rds   RAAM  {}   HIMARS  {}\nATGM teams {} ({} missiles)   [{}]{}", fc.shells, fc.raam, fc.himars, teams, missiles, mode, speed_line),
        Diorama::Robotyne => {
            let order = match *order {
                ColumnOrder::Push => "PUSH ON",
                ColumnOrder::Hold => "HOLD",
                ColumnOrder::PullBack => "PULL BACK",
            };
            format!("155 mm  {} rds   SMOKE  {}   HIMARS  {}\ncolumn [{}]   ATGM teams {} ({} missiles){}", fc.shells, fc.smoke, fc.himars, order, teams, missiles, speed_line)
        }
    }));
    // The day's caption, the same way: the Ukrainian line over its gloss.
    let banner = script.banner.as_ref().map(|b| b.0.clone()).unwrap_or_default();
    if shown.1 != banner {
        shown.1 = banner.clone();
        let uk = crate::text::uk(&banner);
        commands.entity(*banner_root).despawn_children().with_children(|p| {
            if uk.is_empty() {
                p.spawn((TextSpan::new(banner.clone()), TextFont { font: FontSource::Handle(fonts.0.clone()), font_size: FontSize::Px(26.0), ..default() }, TextColor(WHITE)));
            } else {
                p.spawn((TextSpan::new(format!("{uk}\n")), TextFont { font: FontSource::Handle(fonts.0.clone()), font_size: FontSize::Px(26.0), ..default() }, TextColor(WHITE)));
                p.spawn((TextSpan::new(banner.clone()), TextFont { font: FontSource::Handle(fonts.0.clone()), font_size: FontSize::Px(18.0), ..default() }, TextColor(DIM)));
            }
        });
    }
    texts.p3().set_if_neq(Text::new(if view.mode == Mode::Drone && view.feed_down > 0.0 {
        format!("NO SIGNAL\nbattery swap, relaunching ({:.0} s)", view.feed_down)
    } else {
        String::new()
    }));
    for mut v in &mut crosshair {
        v.set_if_neq(if view.mode == Mode::Drone && view.feed_down <= 0.0 && *phase.get() == Phase::Playing { Visibility::Inherited } else { Visibility::Hidden });
    }
}

/// The drone's display, from what the drone knows: the links, the fix, the battery, the flight.
fn osd_update(time: &Time, view: &View, script: &Script, osd: &mut Query<(&Osd, &mut Text, &mut Visibility)>) {
    let on = view.mode == Mode::Drone;
    let t = time.elapsed_secs();
    let blink = (t * 2.0) as i32 % 2 == 0;
    let p2 = view.drone_pos.xz();
    let home = geo::layout().home;
    let (lat, lon) = geo::latlon(p2 + view.gnss_error);
    let agl = view.drone_pos.y - geo::height(p2.x, p2.y);
    let rec = script.t as u32;
    // Heading from the yaw (0 faces north, turning left positive).
    let heading = ((-view.drone_yaw.to_degrees()) % 360.0 + 360.0) % 360.0;
    let hs = view.drone_vel.xz().length();
    let jam = view.jamming;
    for (kind, mut text, mut vis) in osd.iter_mut() {
        vis.set_if_neq(if on { Visibility::Inherited } else { Visibility::Hidden });
        if !on {
            continue;
        }
        let s = match kind {
            Osd::Status => {
                // The status bar: the mode, or the warning that flashes until it is read.
                let warn = match (view.rth.is_some(), view.control, view.feed_down > 0.0, jam) {
                    (_, _, true, _) => "NO SIGNAL. Aircraft returning for a new battery",
                    (true, _, _, _) => "Returning to Home. Press H to cancel",
                    (_, false, _, _) => "Signal Lost. Aircraft will Return to Home in 6 s",
                    (_, _, _, j) if j > 0.6 => "Strong Interference to Aircraft. Fly with caution",
                    (_, _, _, j) if j > 0.3 => "Weak signal. Adjust antenna or clear obstruction",
                    _ => "",
                };
                let mode = if view.gnss < 0.4 { "A" } else if view.drone_vel.length() > 8.0 { "S" } else { "N" };
                let state = if agl < 2.0 { "Landed" } else { "In-Flight" };
                if warn.is_empty() || !blink { format!("<  {mode}  {state}      REC {:02}:{:02}:{:02}", rec / 3600, rec / 60 % 60, rec % 60) } else { format!("<  {mode}  {warn}") }
            }
            Osd::Links => {
                let sats = (26.0 * view.gnss) as u32;
                let video = if view.feed_down > 0.0 { "x  " } else if jam > 0.85 { "|  " } else if jam > 0.55 { "|| " } else { "|||" };
                let rc = if view.control { "|||" } else { "x  " };
                let mins = view.battery * 35.0;
                format!("DRONES {}   GNSS {sats:>2}   RC {rc}  HD {video}   {:>2}'{:02}\"  {:>3.0}%", view.drones_left, mins.floor(), ((mins.fract()) * 60.0) as u32, view.battery * 100.0)
            }
            Osd::Camera => format!("Zoom\n{:.1}x\n\n{} REC", view.zoom(), if blink { "o" } else { " " }),
            Osd::Flight => format!(
                "HDG {heading:>3.0}   ALT {agl:>4.0} m   ASL {:>4.0} m   D {:>5.0} m\nHS {hs:>4.1} m/s   VS {:>+4.1} m/s   GIMBAL {:>+3.0}   {lat:.5} N {lon:.5} E",
                view.drone_pos.y, p2.distance(home), view.drone_vel.y, view.gimbal.to_degrees()
            ),
        };
        text.set_if_neq(Text::new(s));
    }
}

fn show_debrief(stats: Res<Stats>, fc: Res<FireControl>, positions: Query<&Post>, soldiers: Query<&crate::units::Soldier>, mut root: Query<&mut Node, With<Debrief>>, mut t: Single<&mut Text, With<DebriefText>>) {
    for mut n in &mut root {
        n.display = Display::Flex;
    }
    let out = stats.destroyed + stats.abandoned;
    let mut waves = String::new();
    **t = Text::new(match diorama() {
        Diorama::Vuhledar => {
            for (i, w) in stats.per_wave.iter().enumerate().skip(1) {
                waves.push_str(&format!("   column {}: {} vehicles, {} destroyed, {} abandoned\n", i, w.vehicles, w.destroyed, w.abandoned));
            }
            // How close they came to the town's southern edge.
            let closest = (-geo::layout().town.max.y - stats.max_advance).max(0.0);
            format!(
                "THE DAY\n\n\
Russian vehicles committed  {}\n{}\
Knocked out  {}  ({} destroyed, {} abandoned; {} on mines, {} by ATGM, {} by artillery; {} turrets thrown)\n\
Withdrew  {}\n\
Men sent forward  {}.  Came back  {}.  Russian dead  {}.\n\
Closest any Russian got to the town  {:.0} m.  Ukrainian casualties  {}.\n\
Shells left  {} of 96.\n\n\
The 155th was rebuilt, and attacked again. Russia kept coming at the town's front for another year and a half.\n\
{} fell on 1-2 October 2024, to an attack on its flanks, not its front. The 72nd Brigade was never rotated.\n\n\
R: the same day again, down the same roads.     Esc: quit.",
                stats.vehicles_committed, waves, out, stats.destroyed, stats.abandoned, stats.mine_hits, stats.atgm_kills, stats.artillery_kills, stats.turret_tosses,
                stats.withdrawn, stats.sent, stats.returned, stats.attacker_dead, closest, stats.defender_losses, fc.shells, crate::text::VUHLEDAR.hud()
            )
        }
        Diorama::Robotyne => {
            let names = ["", "the first group", "the second group", "sent back in for the men"];
            for (i, w) in stats.per_wave.iter().enumerate().skip(1) {
                let name = names.get(i).copied().unwrap_or("another group");
                waves.push_str(&format!("   {}: {} vehicles, {} burned, {} left in the field\n", name, w.vehicles, w.destroyed, w.abandoned));
            }
            let trench = crate::robotyne::TRENCH_Z;
            let short = |a: f32| if a > trench - 1.0 { "in the trench".to_string() } else { format!("{:.0} m short of the trench", trench - a) };
            let ru = positions.iter().filter(|p| p.side == Side::Ru).count();
            let lying = soldiers.iter().filter(|s| s.side == Side::Ua && s.state == crate::units::SState::Wounded).count();
            let quiet = positions.iter().filter(|p| p.side == Side::Ru && p.strength == 0).count();
            format!(
                "THE MORNING\n\n\
Our vehicles committed  {}\n{}\
Left in the field  {}:  {} burned out,  {} immobilised and abandoned. The recovery crews will try for those tonight.\n\
Came back  {}.\n\
Men sent forward  {}.  Brought out  {}, {} of them picked up in the field by a Bradley.  Killed  {}.\n\
Wounded  {}:  {} brought out,  {} still lying out there.\n\
The overwatch in the forward tree line lost  {}.\n\
The furthest vehicle stopped {}. On foot, the furthest anyone got: {}.\n\
Russian positions on the crest silenced  {} of {}.  Their dug-in tanks knocked out  {} of 2.\n\
Shells left  {} of 64.  Smoke left  {} of 6.\n\n\
Within days the armour was stopped, and the brigades went forward on foot instead, a tree line at a time.\n\
{}, eleven kilometres to the south, was taken in August. {} was not reached.\n\n\
R: the same morning again, down the same tracks.     Esc: quit.",
                stats.vehicles_committed, waves, out, stats.destroyed, stats.abandoned, stats.withdrawn,
                stats.sent, stats.returned, stats.picked_up, stats.attacker_dead, stats.wounded, stats.wounded_out, lying, stats.overwatch_losses,
                short(stats.vehicle_advance), short(stats.max_advance), quiet, ru, stats.defender_vehicles, fc.shells, fc.smoke, crate::text::ROBOTYNE.hud(), crate::text::TOKMAK.hud()
            )
        }
    });
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
