//! The screen: drone telemetry, the radio net, fire control, the tally, the briefing and the debrief.
//! The debrief is the day's own account. The record shaped the day; it does not grade it (design/next.md).

use crate::camera::{Mode, View};
use crate::director::{self, Script};
use crate::fire::{Atgm, AtgmMode, AtgmTeam, FireControl, JAVELIN_LOCK, Post};
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
            .add_systems(Update, (update_hud, update_hit_feed, briefing_input, hud_switch, igla_ring).chain().in_set(crate::Step::Screen))
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
/// The hit feed: the last hits, on the right under the tally.
#[derive(Component)]
struct HitFeed;
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

/// Who holds each of the field's areas.
fn zones_line(f: &bav_sim::fight::Field) -> String {
    f.zones
        .iter()
        .map(|z| {
            let who = match z.held {
                Some(Side::Ua) => "ours",
                Some(Side::Ru) => "THEIRS",
                None => "contested",
            };
            format!("{} {who}", z.name.0)
        })
        .collect::<Vec<_>>()
        .join("   ")
}

/// The runway in the tally: its longest whole stretch, and whether an Il-76 could land on it.
fn runway_line(r: &bav_sim::air::Runway) -> String {
    format!("   runway: {:.1} km whole{}", r.longest() / 1000.0, if r.usable() { "" } else { ", NO LANDING" })
}

/// The airlift in the debrief.
fn airlift_row(a: Option<&bav_sim::air::AirTally>) -> &'static str {
    a.map_or("—", |a| if a.airlift_landed { "landed: the field lost for good" } else if a.airlift_turned_back { "turned back in the air" } else { "not yet decided" })
}

/// The field's tally: what has come down, what has landed, who holds what, and our men.
fn field_line(f: &bav_sim::fight::Field, air: Option<&bav_sim::air::AirTally>, runway: Option<&bav_sim::air::Runway>) -> String {
    let zones = zones_line(f);
    let (down, landed) = air.map_or((0, 0), |a| (a.down.len(), a.men_landed));
    let strip = runway.map_or(String::new(), runway_line);
    format!("\nhelicopters down {down}   VDV landed {landed}   our men out {}   taken {}\n{zones}{strip}", f.out, f.captured)
}

/// The Igla's ring, and its size on a 900 px high screen.
#[derive(Component)]
struct IglaRing;
const IGLA_RING: f32 = 116.0;

/// The ring shows while the tube is shouldered: dark while it searches, amber while the seeker growls, green on the
/// steady tone.
fn igla_ring(view: Res<View>, mut q: Query<(&mut Visibility, &mut BorderColor), With<IglaRing>>) {
    for (mut v, mut border) in &mut q {
        v.set_if_neq(if view.mode == Mode::Manpads && view.sight { Visibility::Inherited } else { Visibility::Hidden });
        let c = match view.lock {
            Some((_, t)) if t >= bav_sim::manpads::LOCK => Color::srgba(0.3, 0.9, 0.35, 0.9),
            Some(_) => AMBER,
            None => Color::srgba(0.05, 0.05, 0.05, 0.8),
        };
        *border = BorderColor::all(c);
    }
}
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
        // Above the corner panels' three lines: the fire panel's column order is long enough to reach the middle.
        Node { position_type: PositionType::Absolute, bottom: px(74), width: percent(100), justify_content: JustifyContent::Center, ..default() },
        shadow,
        Osd::Flight,
        TextLayout::justify(Justify::Center),
    ));
    commands.spawn((text("", 15.0, WHITE), abs(Some(40.0), None, None, Some(18.0)), Tally, TextLayout::justify(Justify::Right), HudKeep));
    commands.spawn((text("", 14.0, AMBER), Node { max_width: px(470), ..abs(None, Some(16.0), Some(18.0), None) }, RadioLog, HudKeep));
    commands.spawn((text("", 13.0, WHITE), Node { max_width: px(470), ..abs(Some(150.0), None, None, Some(18.0)) }, TextLayout::justify(Justify::Right), HitFeed, HudKeep));
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

    // The Igla's ring: the seeker's field, about 2.2° across the shouldered eye's narrower view. (Only where there are
    // Iglas: an entity more would renumber another day's.)
    if bav_sim::manpads::present() {
        commands.spawn((
        Node { position_type: PositionType::Absolute, left: percent(50), top: percent(50), width: px(IGLA_RING), height: px(IGLA_RING), margin: UiRect { left: px(-IGLA_RING * 0.5), top: px(-IGLA_RING * 0.5), ..default() }, border: UiRect::all(px(2)), border_radius: BorderRadius::MAX, ..default() },
        BorderColor::all(Color::srgba(0.05, 0.05, 0.05, 0.8)),
        BackgroundColor(Color::NONE),
            Visibility::Hidden,
            IglaRing,
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
    let briefing = bav_core::data::data().words.briefing.as_str();
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

fn briefing_input(keys: Res<ButtonInput<KeyCode>>, phase: Res<State<Phase>>, mut next: ResMut<NextState<Phase>>) {
    // In Command the briefing leads to the map table, which starts the battle (`table`).
    if *phase.get() == Phase::Briefing && !bav_sim::allot::at_table() && (keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) || crate::dev::dev().skip_briefing) {
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
    (order, fpv_res, op_res, iglas, field, air, runway): (Res<ColumnOrder>, Res<bav_sim::player::Fpv>, Res<bav_sim::player::Operator>, Query<&bav_sim::manpads::Igla>, Option<Res<bav_sim::fight::Field>>, Option<Res<bav_sim::air::AirTally>>, Option<Res<bav_sim::air::Runway>>),
    positions: Query<(&Post, Option<&AtgmTeam>)>,
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
    let team_weapon = view.ground_team.and_then(|e| positions.get(e).ok()).and_then(|(_, t)| t.map(|t| t.weapon));
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
        Mode::Op => Text::new(format!("{}   x{:.1}", bav_core::data::data().words.op_label, view.zoom())),
        Mode::Manpads => {
            let igla = view.ground_team.and_then(|e| iglas.get(e).ok());
            let (left, ready) = igla.map_or((0, false), |i| (i.missiles, i.reload <= 0.0));
            let range = view.ground_range.map_or("----".to_string(), |d| format!("{d:>4.0}"));
            let seeker = match view.lock {
                _ if !view.sight => "[RMB] shoulder it".to_string(),
                _ if !ready => "the next tube is coming up".to_string(),
                Some((_, t)) if t >= bav_sim::manpads::LOCK => format!("TONE, STEADY   {range} m   [LMB] launch"),
                Some((_, t)) => format!("growl {}   {range} m   hold him in the ring", "|".repeat(1 + (t / bav_sim::manpads::LOCK * 8.0) as usize)),
                None => "put a helicopter in the ring".to_string(),
            };
            let near = view.ground_range.is_some_and(|d| d < bav_sim::manpads::MIN_RANGE);
            Text::new(format!(
                "IGLA (ІГЛА)   MISSILES {left}   {seeker}{}\n[5] next team   [1] drone",
                if near { "   TOO CLOSE: let him pass" } else { "" }
            ))
        }
        Mode::Ground => {
            let team = view.ground_team.and_then(|e| positions.get(e).ok()).and_then(|(_, t)| t);
            let (weapon, left) = team.map(|t| (Some(t.weapon), t.atgm)).unwrap_or((None, 0));
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
        // The field's own tally (helicopters down, minutes held, the runway) comes with its fight (design/hostomel.md,
        // phases 2 to 5); until then, the clock.
        Diorama::HostomelLanding => format!("{}   {}{}", diorama().tally_date(), director::clock_text(script.t), field.as_ref().map_or(String::new(), |f| field_line(f, air.as_deref(), runway.as_deref()))),
        Diorama::HostomelCounter => {
            let zones = field.as_ref().map_or(String::new(), |f| zones_line(f));
            let strip = runway.as_deref().map_or(String::new(), runway_line);
            let taken = field.as_ref().map_or(0, |f| f.lost);
            format!(
                "{}   {}\nour vehicles committed {}   lost {}   men sent {}   VDV posts taken {taken}\n{zones}{strip}",
                diorama().tally_date(), director::clock_text(script.t), committed, out, stats.sent
            )
        }
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
    let teams = positions.iter().filter(|(p, t)| p.side == Side::Ua && t.is_some_and(|t| t.atgm > 0) && p.strength >= 2).count();
    let missiles: u32 = positions.iter().filter(|(p, _)| p.side == Side::Ua && p.strength >= 2).map(|(_, t)| t.map_or(0, |t| t.atgm)).sum();
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
            // (Command's air defence on call, while it has missiles.)
            let ad = if fc.air_defence > 0 { format!("   AD  {}", fc.air_defence) } else { String::new() };
            format!("155 mm  {} rds   SMOKE  {}   HIMARS  {}{ad}\ncolumn [{}]   ATGM teams {} ({} missiles){}", fc.shells, fc.smoke, fc.himars, order, teams, missiles, speed_line)
        }
        Diorama::HostomelCounter => {
            let order = match *order {
                ColumnOrder::Push => "PUSH ON",
                ColumnOrder::Hold => "HOLD",
                ColumnOrder::PullBack => "PULL BACK",
            };
            format!("152 mm  {} rds   SMOKE  {}\nthe attack [{order}]{}", fc.shells, fc.smoke, speed_line)
        }
        Diorama::HostomelLanding => {
            let out = match field.as_ref() {
                Some(f) if f.ordered_out => "   PULLING OUT".to_string(),
                Some(_) if bav_sim::data::sim_data().space_key == bav_sim::fire::SpaceKey::Withdraw => "   [Space] pull out".to_string(),
                _ => String::new(),
            };
            format!("D-30  {} rds   [5] Igla{out}{}", fc.shells, speed_line)
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

/// A row of the sources table: clicking it opens the page.
#[derive(Component)]
struct SourceLink(String);

/// Every top-level UI node: no parent, and not the debrief's own.
type TopLevel<'w, 's> = Query<'w, 's, &'static mut Visibility, (With<Node>, Without<ChildOf>, Without<Debrief>)>;

/// The drone's screen and its overlays: they go under the debrief screen when the day ends.
#[derive(bevy::ecs::system::SystemParam)]
struct HudNodes<'w, 's> {
    nodes: TopLevel<'w, 's>,
}

/// The debrief's copy that is counted: the heading, the clock span, the (label, value) rows, the per-column table,
/// and its three heads. The lines that come after, the sources and the keys are the diorama's words.
type DebriefCopy = (
    String,
    String,
    Vec<(String, String)>,
    Vec<(String, [u32; 3])>,
    [&'static str; 3],
);

/// The end of the day: its date and clock span as the heading; the day's own account in aligned rows; the
/// lines about what came after, with no heading; and the sources the day rests on, each a row that opens in the
/// browser. No figures from the record are set against the day's own: the sources say where the day came from,
/// the ledger what it cost (design/next-items.md, package K).
#[allow(clippy::type_complexity)]
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
    (field, air, iglas, runway): (Option<Res<bav_sim::fight::Field>>, Option<Res<bav_sim::air::AirTally>>, Query<&bav_sim::manpads::Igla>, Option<Res<bav_sim::air::Runway>>),
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
    let (heading, span, rows, columns, col_heads): DebriefCopy = match diorama() {
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
                    ("Shells left".into(), format!("{} of {}", fc.shells, fc.start.shells)),
                ],
                stats.per_wave.iter().enumerate().skip(1).map(|(i, w)| (format!("column {i}"), [w.vehicles, w.destroyed, w.abandoned])).collect(),
                ["vehicles", "destroyed", "abandoned"],
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
                    ("Shells left".into(), format!("{} of {}", fc.shells, fc.start.shells)),
                    ("Smoke left".into(), format!("{} of {}", fc.smoke, fc.start.smoke)),
                ],
                stats.per_wave.iter().enumerate().skip(1).map(|(i, w)| (names.get(i).copied().unwrap_or("another group").to_string(), [w.vehicles, w.destroyed, w.abandoned])).collect(),
                ["vehicles", "burned", "left"],
            )
        }
        Diorama::HostomelCounter => {
            let a = air.as_deref();
            let ru = positions.iter().filter(|p| p.side == Side::Ru).count();
            let quiet = positions.iter().filter(|p| p.side == Side::Ru && p.strength == 0).count();
            let mut rows: Vec<(String, String)> = vec![
                ("Our vehicles committed".into(), stats.vehicles_committed.to_string()),
                ("    lost".into(), out.to_string()),
                ("Men sent forward".into(), stats.sent.to_string()),
                ("    killed".into(), stats.attacker_dead.to_string()),
                ("    wounded".into(), stats.wounded.to_string()),
                ("VDV positions silenced".into(), format!("{quiet} of {ru}")),
                ("Ka-52s shot down".into(), a.map_or(0, |a| a.down.len()).to_string()),
            ];
            if let Some(f) = field.as_deref() {
                let ours: Vec<&str> = f.zones.iter().filter(|z| z.held == Some(Side::Ua)).map(|z| z.name.0.as_str()).collect();
                rows.push(("Ours at the end".into(), if ours.is_empty() { "nothing".into() } else { ours.join(", ") }));
            }
            if let Some(r) = runway.as_deref() {
                rows.push(("The runway: the longest stretch left whole".into(), format!("{:.1} km{}", r.longest() / 1000.0, if r.usable() { ", enough for an Il-76" } else { ", too short to land on" })));
            }
            rows.push(("The airlift".into(), airlift_row(a).into()));
            rows.push(("Shells left".into(), format!("{} of {}", fc.shells, fc.start.shells)));
            (format!("24 February 2022   {}", crate::text::HOSTOMEL.hud()), format!("{} – {}", director::clock_text(0.0), director::clock_text(script.t)), rows, Vec::new(), ["", "", ""])
        }
        Diorama::HostomelLanding => {
            let a = air.as_deref();
            let down = |k: bav_sim::air::AirKind| a.map_or(0, |a| a.down.iter().filter(|(d, _)| *d == k).count());
            let by = |w: &str| a.map_or(0, |a| a.down.iter().filter(|(_, x)| *x == w).count());
            let mut rows: Vec<(String, String)> = vec![
                ("Helicopters in the stream".into(), a.map_or(0, |a| a.flown).to_string()),
                ("Shot down".into(), a.map_or(0, |a| a.down.len()).to_string()),
                ("    Ka-52".into(), down(bav_sim::air::AirKind::Ka52).to_string()),
                ("    Mi-24".into(), down(bav_sim::air::AirKind::Mi24).to_string()),
                ("    Mi-8, with the men aboard".into(), down(bav_sim::air::AirKind::Mi8).to_string()),
                ("    by Igla".into(), by("Igla").to_string()),
                ("    by the ZU-23-2".into(), by("ZU-23-2").to_string()),
                ("Hit and turned away".into(), a.map_or(0, |a| a.driven_off).to_string()),
                ("Paratroopers landed".into(), a.map_or(0, |a| a.men_landed).to_string()),
                ("    killed".into(), stats.attacker_dead.to_string()),
            ];
            if let Some(f) = field.as_deref() {
                rows.push(("Our men lost at their posts".into(), stats.defender_losses.to_string()));
                rows.push(("Walked out to Horenka".into(), f.out.to_string()));
                rows.push(("Taken prisoner".into(), f.captured.to_string()));
                rows.push(("The field held until".into(), f.held_until.map_or("the end".into(), director::clock_text)));
                let held: Vec<&str> = f.zones.iter().filter(|z| z.held == Some(Side::Ru)).map(|z| z.name.0.as_str()).collect();
                rows.push(("Theirs at the end".into(), if held.is_empty() { "nothing".into() } else { held.join(", ") }));
            }
            rows.push(("The airlift".into(), airlift_row(a).into()));
            rows.push(("Iglas left".into(), iglas.iter().map(|i| i.missiles).sum::<u32>().to_string()));
            rows.push(("D-30 rounds left".into(), format!("{} of {}", fc.shells, fc.start.shells)));
            (format!("24 February 2022   {}", crate::text::HOSTOMEL.hud()), format!("{} – {}", director::clock_text(0.0), director::clock_text(script.t)), rows, Vec::new(), ["", "", ""])
        }
    };
    let words = &bav_core::data::data().words;
    let (closing, sources, keys) = (&words.closing, &words.sources, words.keys.as_str());
    let dim = Color::srgba(0.72, 0.72, 0.70, 0.85);
    // The film's end card reads this: one label and value per row, and the closing lines prefixed `closing\t`.
    // Written fresh, once, when the screen comes up.
    if let Some(dir) = crate::dev::dev().showcase.as_deref() {
        let mut text = rows.iter().map(|(l, v)| format!("{l}\t{v}\n")).collect::<String>();
        for line in closing {
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
        for line in closing {
            p.spawn((Text::new(line.as_str()), tf(15.0), TextColor(WHITE)));
        }
        p.spawn(Node { height: px(26), ..default() });
        p.spawn((Text::new("Sources"), tf(13.0), TextColor(dim), Node { margin: UiRect::bottom(px(6)), ..default() }));
        for s in sources {
            p.spawn((Interaction::None, Node { flex_direction: FlexDirection::Row, column_gap: px(16), padding: UiRect::vertical(px(2)), ..default() }, SourceLink(s.url.clone()))).with_children(|r| {
                r.spawn((Text::new(s.publisher.as_str()), tf(13.0), TextColor(dim), Node { width: px(250), ..default() }, nowrap));
                r.spawn((Text::new(s.date.as_str()), tf(13.0), TextColor(dim), Node { width: px(110), ..default() }, nowrap));
                r.spawn((Text::new(s.title.as_str()), tf(13.0), TextColor(AMBER), Node { flex_grow: 1.0, ..default() }));
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
            let _ = std::process::Command::new("xdg-open").arg(&link.0).spawn();
            #[cfg(target_os = "macos")]
            let _ = std::process::Command::new("open").arg(&link.0).spawn();
            #[cfg(target_os = "windows")]
            let _ = std::process::Command::new("cmd").args(["/C", "start", "", link.0.as_str()]).spawn();
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

/// The hit feed: the last five hits within 45 s, newest first. What hit, from where, at what, and what it did;
/// our hits in white, theirs on our vehicles in amber.
fn update_hit_feed(mut commands: Commands, time: Res<Time>, script: Res<Script>, log: Res<bav_sim::hits::HitLog>, fonts: Res<Fonts>, root: Single<Entity, With<HitFeed>>, mut shown: Local<String>) {
    let now = time.elapsed_secs_f64();
    let mut spans: Vec<(String, Color, f32)> = Vec::new();
    for h in log.entries.iter().rev().filter(|h| now - h.at < 45.0).take(5) {
        let clock = director::clock_text((script.t - (now - h.at) as f32).max(0.0));
        let from = if h.from.is_empty() { String::new() } else { format!(" from {}", h.from) };
        let colour = if h.side == Side::Ua { AMBER } else { WHITE };
        spans.push((format!("{clock}  {}{} → {}\n", h.weapon, from, h.target_name), colour, 13.0));
        spans.push((format!("{}\n\n", h.result), DIM, 12.0));
    }
    let key = spans.iter().map(|(s, _, _)| s.as_str()).collect::<String>();
    if *shown != key {
        *shown = key;
        commands.entity(*root).despawn_children().with_children(|p| {
            for (s, colour, size) in &spans {
                p.spawn((TextSpan::new(s.clone()), TextFont { font: FontSource::Handle(fonts.0.clone()), font_size: FontSize::Px(*size), ..default() }, TextColor(*colour)));
            }
        });
    }
}
