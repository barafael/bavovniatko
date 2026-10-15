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
use std::io::Write;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HudHidden>()
            .add_systems(Startup, spawn_hud)
            .add_systems(Update, (update_hud, briefing_input, hud_switch).chain().in_set(crate::Step::Screen))
            .add_systems(OnEnter(Phase::Playing), hide_briefing)
            .add_systems(OnEnter(Phase::Debrief), show_debrief)
            .add_systems(Update, (debrief_input, open_sources).run_if(in_state(Phase::Debrief)).in_set(crate::Step::Screen).after(briefing_input));
    }
}

/// The showcase's switch, written each frame: true while a shot is filmed without the interface. While it is
/// set, every top-level node is hidden; the [`HudKeep`] ones come back with it.
#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub struct HudHidden(pub bool);

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
/// The sight's track gate: four corners round the vehicle the seeker holds (flashing until the Javelin locks).
#[derive(Component)]
struct Gate(usize);
#[derive(Component)]
struct Briefing;
#[derive(Component)]
struct Debrief;
#[derive(Component)]
struct DebriefText;
/// A top-level node the HUD switch brings back when the interface returns (the display re-tunes the rest —
/// the drone's display, the crosshair, the reticle — itself each frame).
#[derive(Component)]
struct HudKeep;

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
    commands.spawn((text("", 15.0, WHITE), abs(Some(14.0), None, Some(18.0), None), Telemetry, HudKeep));
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
    commands.spawn((text("", 15.0, WHITE), abs(Some(40.0), None, None, Some(18.0)), Tally, TextLayout::justify(Justify::Right), HudKeep));
    commands.spawn((text("", 14.0, AMBER), Node { max_width: px(470), ..abs(None, Some(16.0), Some(18.0), None) }, RadioLog, HudKeep));
    commands.spawn((text("", 14.0, WHITE), abs(None, Some(16.0), None, Some(18.0)), FirePanel, TextLayout::justify(Justify::Right), HudKeep));
    commands.spawn((
        text("", 26.0, WHITE),
        Node { position_type: PositionType::Absolute, top: percent(16), width: percent(100), justify_content: JustifyContent::Center, ..default() },
        TextLayout::justify(Justify::Center),
        Banner,
        HudKeep,
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
    // The track gate's corners, placed each frame round the vehicle the sight holds.
    for k in 0..4 {
        commands.spawn((Node { position_type: PositionType::Absolute, width: px(7), height: px(7), ..default() }, BackgroundColor(Color::NONE), Visibility::Hidden, Gate(k)));
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
M: move the drone team when they find you (the feed is down while it moves).\n\
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
        .with_child((Node { flex_direction: FlexDirection::Column, max_width: px(1100), width: percent(86), max_height: percent(96), overflow: Overflow::clip(), ..default() }, DebriefText));
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
Alt+click: send a Bradley for the wounded there (four for the day).  V: answer the last call for evacuation.\n\
4: fly the FPV (one today): mouse points it, W/S throttle, A/D bank.  M: move the drone team if they find you.\n\
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

/// The showcase films some shots with the interface off: while [`HudHidden`] says so, every top-level node is
/// hidden (as under the debrief); the [`HudKeep`] ones come back with it, after the display has already re-tuned
/// the rest this frame.
#[allow(clippy::type_complexity)]
fn hud_switch(
    hidden: Res<HudHidden>,
    phase: Res<State<Phase>>,
    mut nodes: ParamSet<(
        Query<&mut Visibility, (With<Node>, Without<ChildOf>, Without<Debrief>)>,
        Query<&mut Visibility, (With<HudKeep>, Without<ChildOf>, Without<Debrief>)>,
    )>,
    mut hid: Local<bool>,
) {
    if *phase.get() != Phase::Playing {
        return;
    }
    if hidden.0 {
        for mut v in &mut nodes.p0() {
            *v = Visibility::Hidden;
        }
        *hid = true;
    } else if *hid {
        *hid = false;
        for mut v in &mut nodes.p1() {
            *v = Visibility::Visible;
        }
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
    (order, fpv_res, op_res): (Res<ColumnOrder>, Res<bav_sim::player::Fpv>, Res<bav_sim::player::Operator>),
    positions: Query<&Post>,
    mut texts: ParamSet<(
        Single<&mut Text, With<Telemetry>>,
        Single<&mut Text, With<Tally>>,
        Single<&mut Text, With<FirePanel>>,
        Single<&mut Text, With<NoSignal>>,
        Query<(&Osd, &mut Text, &mut Visibility)>,
    )>,
    mut crosshair: Query<&mut Visibility, (With<Crosshair>, Without<Reticle>, Without<Osd>, Without<Gate>)>,
    mut reticle: Query<&mut Visibility, (With<Reticle>, Without<Crosshair>, Without<Osd>, Without<Gate>)>,
    sight: (Query<(&Gate, &mut Node, &mut Visibility, &mut BackgroundColor), (Without<Crosshair>, Without<Reticle>, Without<Osd>)>, Single<(&Camera, &GlobalTransform), With<crate::camera::MainCamera>>, Query<&Transform, With<crate::units::Vehicle>>),
    phase: Res<State<Phase>>,
    mut commands: Commands,
    net: (Res<Fonts>, Single<Entity, With<RadioLog>>, Single<Entity, With<Banner>>, Local<(String, String)>),
) {
    let team = (&*fpv_res, &*op_res);
    let (fonts, log_root, banner_root, mut shown) = net;
    // Under the debrief the display stays as `show_debrief` left it: hidden.
    if *phase.get() == Phase::Debrief {
        return;
    }
    osd_update(&time, &view, &script, &mut texts.p4());
    for mut v in &mut reticle {
        v.set_if_neq(if view.mode == Mode::Ground && view.sight { Visibility::Inherited } else { Visibility::Hidden });
    }
    // The gate: the hull's box on the screen, flashing until the seeker has locked (a Stugna's tracker holds at
    // once), and the corner marks drawn as L-shapes by colour alone.
    let (mut gate, cam, targets) = sight;
    let team_weapon = view.ground_team.and_then(|e| positions.get(e).ok()).and_then(|p| p.weapon);
    let held = (view.mode == Mode::Ground && view.sight).then_some(view.lock).flatten();
    let gate_box = held.and_then(|(e, t)| {
        let tf = targets.get(e).ok()?;
        let (c, gt) = *cam;
        let centre = c.world_to_viewport(gt, tf.translation + Vec3::Y * 1.2).ok()?;
        let edge = c.world_to_viewport(gt, tf.translation + Vec3::Y * 1.2 + gt.right() * 3.8).ok()?;
        let half = (edge - centre).length().max(10.0);
        Some((centre, half, t))
    });
    let blink = (time.elapsed_secs() * 4.0) as i32 % 2 == 0;
    for (g, mut node, mut vis, mut bg) in &mut gate {
        match gate_box {
            Some((centre, half, t)) => {
                let locked = team_weapon != Some(Atgm::Javelin) || t >= JAVELIN_LOCK;
                vis.set_if_neq(if locked || blink { Visibility::Inherited } else { Visibility::Hidden });
                let (sx, sy) = ([-1.0, 1.0, -1.0, 1.0][g.0], [-1.0, -1.0, 1.0, 1.0][g.0]);
                node.left = px(centre.x + sx * half - 3.5);
                node.top = px(centre.y + sy * half * 0.6 - 3.5);
                let colour = if team_weapon == Some(Atgm::Javelin) { Color::srgba(0.4, 1.0, 0.5, 0.9) } else { Color::srgba(0.95, 0.95, 0.9, 0.9) };
                bg.0 = colour;
            }
            None => {
                vis.set_if_neq(Visibility::Hidden);
            }
        }
    }
    let now = time.elapsed_secs_f64();
    // Each text is written only when it reads differently, so Bevy lays out again only what changed.
    let telemetry = match view.mode {
        Mode::Drone => Text::new(String::new()),
        Mode::Fpv => {
            let (fpv, op) = (&team.0, &team.1);
            let bars = ["     ", "|    ", "||   ", "|||  ", "|||| ", "|||||"][(fpv.signal * 5.0).round().clamp(0.0, 5.0) as usize];
            let agl = fpv.pos.y - geo::height(fpv.pos.x, fpv.pos.z);
            Text::new(format!(
                "FPV   VID {bars}  RC {}   BAT {:>3.0}%   ALT {:>4.0} m   {:>3.0} km/h   {:.2} km\nMouse: point it.  W/S: throttle.  A/D: bank.  It goes where it is pointed.",
                if fpv.control { "OK " } else { "-- " },
                (fpv.battery / bav_sim::data::sim_data().fpv.map_or(1.0, |f| f.endurance) * 100.0).max(0.0),
                agl,
                fpv.vel.length() * 3.6,
                op.pos.distance(fpv.pos.xz()) / 1000.0
            ))
        }
        Mode::Op => Text::new(format!("{}   x{:.1}", if diorama() == Diorama::Robotyne { "OBSERVER, WATER TOWER (СПОСТЕРІГАЧ, ВОДОНАПІРНА ВЕЖА)" } else { "OBSERVATION POST, 9TH FLOOR (СП, 9-Й ПОВЕРХ)" }, view.zoom())),
        Mode::Ground => {
            let team = view.ground_team.and_then(|e| positions.get(e).ok());
            let (weapon, left) = team.map(|p| (p.weapon, p.atgm)).unwrap_or((None, 0));
            let name = weapon.map_or("ATGM".to_string(), |w| w.to_string().to_uppercase());
            if view.sight {
                let how = if weapon == Some(Atgm::Javelin) {
                    match view.lock {
                        Some((_, t)) if t >= JAVELIN_LOCK => "SEEK   LOCKED   [LMB] fire and forget".to_string(),
                        Some((_, t)) => format!("SEEK   LOCKING {:.0}%   hold the gates on his hull", t / JAVELIN_LOCK * 100.0),
                        None => "SEEK   no target: put the crosshair on a vehicle".to_string(),
                    }
                } else {
                    "[LMB] launch, then hold the crosshair on him until impact".to_string()
                };
                let head = if weapon == Some(Atgm::Javelin) { "JAVELIN CLU   NFOV 9x   WHITE HOT".to_string() } else { format!("{name} PANEL   TV x12") };
                Text::new(format!(
                    "{}   RANGE {}   MISSILES {}\n{}",
                    head,
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
            diorama().tally_date(), director::clock_text(script.t), committed, out, stats.withdrawn, stats.sent, stats.returned
        ),
        Diorama::Robotyne => format!(
            "{}   {}\nour vehicles committed {}   stopped in the field {}   came back {}\nmen sent {}   brought out {}",
            diorama().tally_date(), director::clock_text(script.t), committed, out, stats.withdrawn, stats.sent, stats.returned
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
    let moving = now < team.1.moving_until;
    texts.p3().set_if_neq(Text::new(if view.mode == Mode::Fpv && !team.0.flying {
        "NO SIGNAL".to_string()
    } else if view.mode == Mode::Drone && moving {
        format!("NO SIGNAL\nthe team is moving ({:.0} s)", team.1.moving_until - now)
    } else if view.mode == Mode::Drone && view.feed_down > 0.0 {
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

/// A source the day rests on, shown as a row the player can open.
struct Source {
    publisher: &'static str,
    date: &'static str,
    title: &'static str,
    url: &'static str,
}

const BAV_SOURCES: &[Source] = &[
    Source { publisher: "Institute for the Study of War", date: "9 Feb 2023", title: "Russian Offensive Campaign Assessment", url: "https://www.understandingwar.org/backgrounder/russian-offensive-campaign-assessment-february-9-2023" },
    Source { publisher: "Reuters", date: "10 Feb 2023", title: "Russia likely lost dozens of armoured vehicles near Ukraine's Vuhledar: UK", url: "https://www.reuters.com/world/europe/russia-likely-lost-dozens-tanks-failed-attack-vuhledar-uk-2023-02-10/" },
    Source { publisher: "Politico Europe", date: "12 Feb 2023", title: "Russia may have lost an entire elite brigade near a Donetsk coal-mining town", url: "https://www.politico.eu/article/russia-may-have-lost-an-entire-elite-brigade-near-a-coal-mining-town-in-donbas-ukraine-says/" },
    Source { publisher: "CNN", date: "13 Feb 2023", title: "'Like turkeys at a shooting range': mauling of Russian forces in Donetsk hotspot", url: "https://www.cnn.com/2023/02/13/europe/russia-ukraine-vuhledar-donetsk-fiasco-intl/index.html" },
    Source { publisher: "Kyiv Post", date: "14 Feb 2023", title: "Skilled Vuhledar defenses halt major Kremlin attack", url: "https://www.kyivpost.com/post/12165" },
    Source { publisher: "RFE/RL", date: "17 Feb 2023", title: "What happened in Vuhledar? A battle points to major Russian military problems", url: "https://www.rferl.org/a/ukraine-russia-battle-vuhledar/32276547.html" },
    Source { publisher: "Novaya Gazeta Europe", date: "16 Mar 2023", title: "UK intelligence on Ukraine's use of RAAM anti-tank mines at Vuhledar", url: "https://novayagazeta.eu/articles/2023/03/16/uk-intel-russia-suffered-heavy-losses-at-vuhledar-due-to-ukraines-use-of-raam-anti-tank-mines-en-news" },
    Source { publisher: "Butusov Plus", date: "29 Dec 2023", title: "Битва за Вугледар. Розгром батальйону «Алга» (documentary)", url: "https://www.youtube.com/watch?v=xjx2pRiGidk" },
    Source { publisher: "RUSI", date: "Jul 2024", title: "Preliminary lessons from Ukraine's offensive operations, 2022–23", url: "https://static.rusi.org/lessons-learned-ukraine-offensive-2022-23.pdf" },
];

const ROBOTYNE_SOURCES: &[Source] = &[
    Source { publisher: "Popular Mechanics", date: "9 Jun 2023", title: "Ukraine's Western armor suffered enormous losses while breaching a minefield", url: "https://www.popularmechanics.com/military/weapons/a44157100/ukraine-suffers-losses-breaching-minefield/" },
    Source { publisher: "Euromaidan Press", date: "10 Jun 2023", title: "So how many Leopard tanks did Ukraine lose near Mala Tokmachka?", url: "https://euromaidanpress.com/2023/06/10/so-how-many-leopard-tanks-were-lost-in-ukraine/" },
    Source { publisher: "The Guardian", date: "14 Jun 2023", title: "Ukraine's failed Mala Tokmachka assault lays bare counteroffensive challenges", url: "https://www.theguardian.com/world/2023/jun/14/ukraine-failed-assault-near-mala-tokmachka-raises-counteroffensive-challenges" },
    Source { publisher: "The War Zone", date: "15 Jun 2023", title: "Ukraine's armor appears to have a Russian attack helicopter problem", url: "https://www.twz.com/ukraines-armor-appears-to-have-a-russian-attack-helicopter-problem" },
    Source { publisher: "BBC News", date: "27 Jul 2023", title: "Western armour struggles against Russian defences", url: "https://www.bbc.com/news/world-europe-66306150" },
    Source { publisher: "War on the Rocks", date: "4 Sep 2023", title: "Perseverance and adaptation: Ukraine's counteroffensive at three months", url: "https://warontherocks.com/2023/09/perseverance-and-adaptation-ukraines-counteroffensive-at-three-months/" },
    Source { publisher: "RUSI", date: "Sep 2023", title: "Stormbreak: fighting through Russian defences in Ukraine's 2023 offensive", url: "https://static.rusi.org/Stormbreak-Special-Report-web-final_0.pdf" },
    Source { publisher: "Kyiv Independent", date: "26 Sep 2023", title: "Overcoming setbacks, NATO-trained brigade breaches Surovikin line", url: "https://kyivindependent.com/overcoming-setbacks-nato-trained-brigade-breaches-surovikin-line-in-zaporizhzhia-oblast/" },
];

/// A row of the sources table: clicking it opens the page.
#[derive(Component)]
struct SourceLink(&'static str);

/// Every top-level UI node: no parent, and not the debrief's own.
type TopLevel<'w, 's> = Query<'w, 's, &'static mut Visibility, (With<Node>, Without<ChildOf>, Without<Debrief>)>;

/// The drone's screen and its overlays: they go under the debrief screen when the day ends.
#[derive(bevy::ecs::system::SystemParam)]
struct HudNodes<'w, 's> {
    nodes: TopLevel<'w, 's>,
}

/// All the copy the debrief screen is built from: the heading, the clock span, the (label, value) rows, the
/// per-column table, its three heads, the lines that come after, the sources, and the keys.
type DebriefCopy = (
    String,
    String,
    Vec<(String, String)>,
    Vec<(String, [u32; 3])>,
    [&'static str; 3],
    Vec<String>,
    &'static [Source],
    &'static str,
);

/// The end of the day: its date and clock span as the heading; the day's own account in aligned rows; the
/// lines about what came after, with no heading; and the sources the day rests on, each a row that opens in the
/// browser. No figures from the record are set against the day's own: the sources say where the day came from,
/// the ledger what it cost (design/next-items.md, package K).
fn show_debrief(
    mut commands: Commands,
    fonts: Res<Fonts>,
    stats: Res<Stats>,
    fc: Res<FireControl>,
    script: Res<Script>,
    positions: Query<&Post>,
    soldiers: Query<&crate::units::Soldier>,
    evac: Res<bav_sim::evac::Evacuation>,
    mut root: Query<&mut Node, With<Debrief>>,
    body: Single<Entity, With<DebriefText>>,
    mut hud: HudNodes,
) {
    for mut n in &mut root {
        n.display = Display::Flex;
    }
    // The drone's display and the sight go away under the screen.
    for mut v in &mut hud.nodes {
        *v = Visibility::Hidden;
    }
    let font = fonts.0.clone();
    let tf = |size: f32| TextFont { font: FontSource::Handle(font.clone()), font_size: FontSize::Px(size), ..default() };
    let out = stats.destroyed + stats.abandoned;
    // (label, value) rows, and the columns table: (name, vehicles, destroyed, abandoned).
    let (heading, span, rows, columns, col_heads, closing, sources, keys): DebriefCopy = match diorama() {
        Diorama::Vuhledar => {
            let closest = (-geo::layout().town.max.y - stats.max_advance).max(0.0);
            (
                format!("6 February 2023   {}", crate::text::VUHLEDAR.hud()),
                format!("06:40 – {}", director::clock_text(script.t)),
                vec![
                    ("Russian vehicles committed".into(), stats.vehicles_committed.to_string()),
                    ("Knocked out".into(), format!("{out}")),
                    ("    destroyed".into(), stats.destroyed.to_string()),
                    ("    abandoned".into(), stats.abandoned.to_string()),
                    ("    on mines".into(), stats.mine_hits.to_string()),
                    ("    by ATGM".into(), stats.atgm_kills.to_string()),
                    ("    by artillery".into(), stats.artillery_kills.to_string()),
                    ("    turrets thrown".into(), stats.turret_tosses.to_string()),
                    ("Withdrew".into(), stats.withdrawn.to_string()),
                    ("Men sent forward".into(), stats.sent.to_string()),
                    ("    came back".into(), stats.returned.to_string()),
                    ("    Russian dead".into(), stats.attacker_dead.to_string()),
                    ("Closest any Russian got to the town".into(), format!("{closest:.0} m")),
                    ("Ukrainian casualties".into(), stats.defender_losses.to_string()),
                    ("Shells left".into(), format!("{} of 96", fc.shells)),
                ],
                stats.per_wave.iter().enumerate().skip(1).map(|(i, w)| (format!("column {i}"), [w.vehicles, w.destroyed, w.abandoned])).collect(),
                ["vehicles", "destroyed", "abandoned"],
                vec![
                    "The 155th was rebuilt, and attacked again. Russia kept coming at the town's front for another year and a half.".into(),
                    format!("{} fell on 1–2 October 2024, to an attack on its flanks, not its front. The 72nd Brigade was never rotated.", crate::text::VUHLEDAR.hud()),
                ],
                BAV_SOURCES,
                "R: the same day again, down the same roads.     Esc: quit.",
            )
        }
        Diorama::Robotyne => {
            let names = ["", "the first group", "the second group", "sent in for the wounded"];
            let trench = crate::geo::layout().trench_z.expect("Mala Tokmachka has a trench");
            let short = |a: f32| {
                if a == f32::MIN {
                    "nowhere yet".to_string()
                } else if a > trench - 1.0 {
                    "in the trench".to_string()
                } else {
                    format!("{:.0} m short of the trench", trench - a)
                }
            };
            let ru = positions.iter().filter(|p| p.side == Side::Ru).count();
            let lying = soldiers.iter().filter(|s| s.side == Side::Ua && s.state == crate::units::SState::Wounded).count();
            let quiet = positions.iter().filter(|p| p.side == Side::Ru && p.strength == 0).count();
            (
                format!("8 June 2023   {}", crate::text::MALA_TOKMACHKA.hud()),
                format!("03:50 – {}", director::clock_text(script.t)),
                vec![
                    ("Our vehicles committed".into(), stats.vehicles_committed.to_string()),
                    ("Left in the field".into(), out.to_string()),
                    ("    burned out".into(), stats.destroyed.to_string()),
                    ("    immobilised and abandoned (the recovery crews will try tonight)".into(), stats.abandoned.to_string()),
                    ("Came back".into(), stats.withdrawn.to_string()),
                    ("Men sent forward".into(), stats.sent.to_string()),
                    ("    brought out".into(), stats.returned.to_string()),
                    ("    of them picked up in the field by a Bradley".into(), stats.picked_up.to_string()),
                    ("    killed".into(), stats.attacker_dead.to_string()),
                    ("Wounded".into(), stats.wounded.to_string()),
                    ("    brought out".into(), stats.wounded_out.to_string()),
                    ("    still lying out there".into(), lying.to_string()),
                    ("Bradleys you sent in for them".into(), format!("{} of {}", evac.sent, evac.sent + evac.left)),
                    ("The overwatch in the forward tree line lost".into(), stats.overwatch_losses.to_string()),
                    ("The furthest vehicle stopped".into(), short(stats.vehicle_advance)),
                    ("On foot, the furthest anyone got".into(), short(stats.max_advance)),
                    ("Russian positions on the crest silenced".into(), format!("{quiet} of {ru}")),
                    ("Their dug-in tanks knocked out".into(), format!("{} of 2", stats.defender_vehicles)),
                    ("Shells left".into(), format!("{} of 64", fc.shells)),
                    ("Smoke left".into(), format!("{} of 6", fc.smoke)),
                ],
                stats.per_wave.iter().enumerate().skip(1).map(|(i, w)| (names.get(i).copied().unwrap_or("another group").to_string(), [w.vehicles, w.destroyed, w.abandoned])).collect(),
                ["vehicles", "burned", "left"],
                vec![
                    "Within days the armour was stopped, and the brigades went forward on foot instead, a tree line at a time.".into(),
                    format!("{}, eleven kilometres to the south, was taken in August. {} was not reached.", crate::text::ROBOTYNE.hud(), crate::text::TOKMAK.hud()),
                ],
                ROBOTYNE_SOURCES,
                "R: the same morning again, down the same tracks.     Esc: quit.",
            )
        }
    };
    let dim = Color::srgba(0.72, 0.72, 0.70, 0.85);
    // The film's end card reads this: one label and value per row, and the closing lines prefixed `closing\t`.
    // Written fresh, once, when the screen comes up.
    if let Some(dir) = crate::dev::dev().showcase.as_deref() {
        let mut text = rows.iter().map(|(l, v)| format!("{l}\t{v}\n")).collect::<String>();
        for line in &closing {
            text.push_str(&format!("closing\t{line}\n"));
        }
        if let Ok(mut f) = std::fs::File::create(format!("{dir}/ledger.txt")) {
            let _ = f.write_all(text.as_bytes());
        }
    }
    // A text node given a width is measured for its height at its min-content width, a word a line, so a cell of
    // fixed width must not wrap, or every row with a two-word value stands two lines tall.
    let nowrap = TextLayout { linebreak: LineBreak::NoWrap, ..default() };
    let right = |r: &mut ChildSpawnerCommands, s: String, size: f32, colour: Color, width: f32| {
        r.spawn(Node { width: px(width), justify_content: JustifyContent::FlexEnd, ..default() }).with_child((Text::new(s), tf(size), TextColor(colour), nowrap));
    };
    let row = |p: &mut ChildSpawnerCommands, label: &str, value: &str, size: f32, colour: Color| {
        p.spawn(Node { flex_direction: FlexDirection::Row, column_gap: px(16), ..default() }).with_children(|r| {
            r.spawn((Text::new(label), tf(size), TextColor(colour), Node { flex_grow: 1.0, ..default() }));
            right(r, value.to_string(), size, WHITE, 300.0);
        });
    };
    commands.entity(*body).despawn_children().with_children(|p| {
        p.spawn((Text::new(heading), tf(26.0), TextColor(WHITE)));
        p.spawn((Text::new(span), tf(15.0), TextColor(dim), Node { margin: UiRect::bottom(px(22)), ..default() }));
        for (label, value) in &rows {
            row(p, label, value, 15.0, if label.starts_with(' ') { dim } else { WHITE });
        }
        // The columns, as a small table.
        if !columns.is_empty() {
            p.spawn(Node { flex_direction: FlexDirection::Row, column_gap: px(16), margin: UiRect::top(px(18)), ..default() }).with_children(|r| {
                r.spawn((Text::new(""), tf(13.0), Node { flex_grow: 1.0, ..default() }));
                for h in col_heads {
                    right(r, h.to_string(), 13.0, dim, 110.0);
                }
            });
            for (name, n) in &columns {
                p.spawn(Node { flex_direction: FlexDirection::Row, column_gap: px(16), ..default() }).with_children(|r| {
                    r.spawn((Text::new(name.as_str()), tf(15.0), TextColor(WHITE), Node { flex_grow: 1.0, ..default() }));
                    for v in n {
                        right(r, v.to_string(), 15.0, WHITE, 110.0);
                    }
                });
            }
        }
        p.spawn(Node { height: px(26), ..default() });
        for line in &closing {
            p.spawn((Text::new(line.as_str()), tf(15.0), TextColor(WHITE)));
        }
        p.spawn(Node { height: px(26), ..default() });
        p.spawn((Text::new("Sources"), tf(13.0), TextColor(dim), Node { margin: UiRect::bottom(px(6)), ..default() }));
        for s in sources {
            p.spawn((Interaction::None, Node { flex_direction: FlexDirection::Row, column_gap: px(16), padding: UiRect::vertical(px(2)), ..default() }, SourceLink(s.url))).with_children(|r| {
                r.spawn((Text::new(s.publisher), tf(13.0), TextColor(dim), Node { width: px(250), ..default() }, nowrap));
                r.spawn((Text::new(s.date), tf(13.0), TextColor(dim), Node { width: px(110), ..default() }, nowrap));
                r.spawn((Text::new(s.title), tf(13.0), TextColor(AMBER), Node { flex_grow: 1.0, ..default() }));
            });
        }
        p.spawn((Text::new(keys), tf(14.0), TextColor(dim), Node { margin: UiRect::top(px(26)), ..default() }));
    });
}

/// A click on a source opens it in the browser.
fn open_sources(links: Query<(&Interaction, &SourceLink), Changed<Interaction>>) {
    for (i, link) in &links {
        if *i == Interaction::Pressed {
            #[cfg(target_os = "linux")]
            let _ = std::process::Command::new("xdg-open").arg(link.0).spawn();
            #[cfg(target_os = "macos")]
            let _ = std::process::Command::new("open").arg(link.0).spawn();
            #[cfg(target_os = "windows")]
            let _ = std::process::Command::new("cmd").args(["/C", "start", "", link.0]).spawn();
        }
    }
}

fn debrief_input(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyR) {
        if let Ok(exe) = std::env::current_exe() {
            let _ = std::process::Command::new(exe).env("BAV_SKIP_BRIEFING", "1").spawn();
        }
        exit.write(AppExit::Success);
    }
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
