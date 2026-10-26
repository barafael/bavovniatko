//! The screen: drone telemetry, the radio net, fire control, the tally, the briefing and the debrief.
//! The debrief is the day's own account. The record shaped the day; it does not grade it (design/next.md), except where
//! the day's file states the player's task (`objective`, Myrnohrad): then it opens with the verdict (`task`).

use crate::camera::{Mode, View};
use crate::director::{self, Script};
use crate::fire::{Atgm, AtgmMode, AtgmTeam, FireControl, JAVELIN_LOCK, Post};
use crate::geo;
use crate::diorama::diorama;
use crate::screen;
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
/// What the tally reads of the day's seep, where it has one (`bav_sim::seep`): theirs on the way in and in burrows.
#[derive(bevy::ecs::system::SystemParam)]
struct SeepView<'w, 's> {
    seep: Option<Res<'w, bav_sim::seep::Seep>>,
    walking: Query<'w, 's, (), With<bav_sim::seep::Seeping>>,
    burrowed: Query<'w, 's, (), With<bav_sim::seep::InBurrow>>,
}

impl SeepView<'_, '_> {
    fn line(&self) -> String {
        self.seep.as_deref().map_or(String::new(), |s| bav_sim::seep::tally_line(s, self.walking.iter().count(), self.burrowed.iter().count()))
    }
}

/// Their drones' line in the tally, where the day has a kill zone: the FPVs they have flown, and our cars on the road.
fn killzone_line(t: &bav_sim::killzone::KzTally) -> String {
    let cars = if t.cars_out > 0 { format!("   our cars out {}   through {}   burnt {}", t.cars_out, t.cars_through, t.cars_lost) } else { String::new() };
    format!("\nTheir FPVs flown {}{cars}", t.sorties.iter().sum::<u32>())
}

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

/// Who holds an area, in the debrief.
fn held_by(held: Option<Side>) -> &'static str {
    match held {
        Some(Side::Ua) => "ours",
        Some(Side::Ru) => "theirs",
        None => "contested",
    }
}

fn capital(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or(String::new(), |f| f.to_uppercase().chain(c).collect())
}

/// The runway in the tally: its longest whole stretch, and whether an Il-76 could land on it.
fn runway_line(r: &bav_sim::air::Airfield) -> String {
    format!("   runway: {:.1} km whole{}", r.longest() / 1000.0, if r.usable() { "" } else { ", NO LANDING" })
}

/// The airlift in the debrief.
fn airlift_row(a: Option<&bav_sim::air::Airfield>) -> &'static str {
    a.map_or("—", |a| if a.airlift_landed { "landed: the field lost for good" } else if a.airlift_turned_back { "turned back in the air" } else { "not yet decided" })
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
    (order, fpv_res, op_res, iglas, field, air, runway, bradleys, drops, robots, seep, kz, task): (Res<ColumnOrder>, Res<bav_sim::player::Fpv>, Res<bav_sim::player::Operator>, Query<&bav_sim::manpads::Igla>, Option<Res<bav_sim::fight::Field>>, Option<Res<bav_sim::air::AirTally>>, Option<Res<bav_sim::air::Airfield>>, Query<(&bav_sim::bradley::BradleyArms, Option<&bav_sim::bradley::Run>)>, Option<Res<bav_sim::drones::Drops>>, RobotView, SeepView, Option<Res<bav_sim::killzone::KzTally>>, crate::task::TaskView),
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
        Mode::Robot => Text::new(robots.robot_line(&view, now)),
        Mode::Bomber => Text::new(robots.bomber_line(&view)),
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
        Mode::Bradley => {
            let a = view.ground_team.and_then(|e| bradleys.get(e).ok());
            let (ap, he, tows, reload, he_up) = a.map_or((0, 0, 0, 0.0, false), |(a, _)| (a.ap, a.he, a.tows, a.tow_reload, a.he_selected));
            // Where the driver has the hull: in the hide, or on a run and which way.
            let driver = a.and_then(|(_, r)| r).map_or("IN THE HIDE".to_string(), |r| format!("RUN {} {:.0} m", r.leg.name(), r.along));
            let range = view.ground_range.map_or("----".to_string(), |d| format!("{d:>4.0} m"));
            let sel = if he_up { "HE" } else { "AP" };
            if view.sight {
                let tow = if tows == 0 { "no TOW".to_string() } else if reload > 0.0 { format!("TOW reloading {reload:.0}") } else { "[T] TOW".to_string() };
                Text::new(format!("ISU x4   {}   RANGE {range}   {sel} AP {ap} HE {he}   TOW {tows}   {driver}\n[LMB] burst   [X] {}   {tow}   [G/K/B] on / hold / back   [I] thermal", if view.thermal { "THERMAL, WHITE HOT" } else { "DAY TV" }, if he_up { "AP" } else { "HE" }))
            } else {
                Text::new(format!("BRADLEY (БРЕДЛІ), GUNNER'S SEAT   AP {ap} HE {he}   TOW {tows}   {driver}\n[RMB] sight   [G/K/B] driver on / hold / back   [6] next Bradley   [1] drone"))
            }
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
    // The tally, in the place's words (`hud.tally`), with what the day has: its aircraft once any have come over, its
    // areas, its fight and its runway.
    let (field, air, runway) = (field.as_deref(), air.as_deref(), runway.as_deref());
    texts.p1().set_if_neq(Text::new(screen::fill(&screen::screen().hud.tally, |key| match key {
        "date" => diorama().tally_date().to_string(),
        "clock" => director::clock_text(script.t),
        "committed" => stats.vehicles_committed.to_string(),
        "out" => (stats.destroyed + stats.abandoned).to_string(),
        "withdrawn" => stats.withdrawn.to_string(),
        "sent" => stats.sent.to_string(),
        "returned" => stats.returned.to_string(),
        "air" => air.filter(|a| a.flown > 0).map_or(String::new(), |a| format!("   aircraft over {}   down {}", a.flown, a.down.len())),
        "zones" => field.map_or(String::new(), zones_line),
        "air_down" => air.map_or(0, |a| a.down.len()).to_string(),
        "landed" => runway.map_or(0, |r| r.men_landed).to_string(),
        "men_out" => field.map_or(0, |f| f.out).to_string(),
        "captured" => field.map_or(0, |f| f.captured).to_string(),
        "taken" => field.map_or(0, |f| f.lost).to_string(),
        "runway" => runway.map_or(String::new(), runway_line),
        "robots" => robots.tally(now),
        "seep" => seep.line(),
        "killzone" => kz.as_deref().map_or(String::new(), killzone_line),
        "task" => task.line(),
        _ => String::new(),
    })));
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
    // What is left to fire, in the place's words (`hud.stock`).
    texts.p2().set_if_neq(Text::new(screen::fill(&screen::screen().hud.stock, |key| match key {
        "shells" => fc.shells.to_string(),
        "cluster" => fc.cluster.to_string(),
        "raam" => fc.raam.to_string(),
        "himars" => fc.himars.to_string(),
        "smoke" => fc.smoke.to_string(),
        "illum" => fc.illum.to_string(),
        "fpv" => team.0.left.to_string(),
        // The drone's grenades where it has them: C drops one.
        "vog" => drops.as_deref().map_or(String::new(), |d| format!("   VOG  {} [C]", d.left)),
        // Command's air defence on call, while it has missiles.
        "ad" => if fc.air_defence > 0 { format!("   AD  {}", fc.air_defence) } else { String::new() },
        "teams" => teams.to_string(),
        "missiles" => missiles.to_string(),
        "atgm_mode" => mode.to_string(),
        "order" => match *order {
            ColumnOrder::Push => "PUSH ON",
            ColumnOrder::Hold => "HOLD",
            ColumnOrder::PullBack => "PULL BACK",
        }
        .to_string(),
        // Where the fight's posts can be pulled out: whether they have been told.
        "pull_out" => match field {
            Some(f) if f.ordered_out => "   PULLING OUT".to_string(),
            Some(_) if bav_sim::data::sim_data().space_key == bav_sim::fire::SpaceKey::Withdraw => "   [Space] pull out".to_string(),
            _ => String::new(),
        },
        "speed" => speed_line.clone(),
        // The robots left to send, and the bomber's sorties, where the day has them.
        "ugv" => robots.robots.as_deref().map_or(String::new(), |r| format!("   UGV  {}{}", r.left, if bav_sim::robots::casevac() { " [V]" } else { "" })),
        "bomber" => robots.robots.as_deref().filter(|_| bav_sim::robots::spec().is_some_and(|s| s.bomber.is_some())).map_or(String::new(), |r| format!("   VAMPIRE  {} [8]", r.sorties_left)),
        _ => String::new(),
    })));
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
        robots.no_signal(&view, now).unwrap_or_default()
    }));
    for mut v in &mut crosshair {
        v.set_if_neq(if (view.mode == Mode::Drone && view.feed_down <= 0.0 || view.mode == Mode::Bomber) && *phase.get() == Phase::Playing { Visibility::Inherited } else { Visibility::Hidden });
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
    // Heading from the yaw (0 faces -z, turning left positive), as a compass bearing where the map is turned.
    let heading = geo::true_heading((-view.drone_yaw.to_degrees()).rem_euclid(360.0));
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
    positions: Query<&'static Post>,
    soldiers: Query<&'static crate::units::Soldier>,
    evac: Res<bav_sim::evac::Evacuation>,
    mut root: Query<&mut Node, With<Debrief>>,
    body: Single<Entity, With<DebriefText>>,
    mut hud: HudNodes,
    (field, air, iglas, runway, drops, robots, seep, kz): (Option<Res<bav_sim::fight::Field>>, Option<Res<bav_sim::air::AirTally>>, Query<&'static bav_sim::manpads::Igla>, Option<Res<bav_sim::air::Airfield>>, Option<Res<bav_sim::drones::Drops>>, (Option<Res<bav_sim::robots::Robots>>, Option<Res<bav_sim::evac::Casualties>>), Option<Res<bav_sim::seep::Seep>>, Option<Res<bav_sim::killzone::KzTally>>),
    duel: DuelView,
    task: crate::task::TaskView,
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
    // The heading's date and place, the clock's span, and the day's account in the place's words (`debrief`).
    let spec = &screen::screen().debrief;
    let data = bav_core::data::data();
    let place = spec.place.as_ref().unwrap_or(&data.place);
    let heading = format!("{}   {} ({})", data.date, place.0, place.1);
    let span = format!("{} – {}", director::clock_text(0.0), director::clock_text(script.t));
    let ledger = Ledger { stats: &stats, fc: &fc, positions: &positions, soldiers: &soldiers, evac: &evac, field: field.as_deref(), air: air.as_deref(), iglas: &iglas, runway: runway.as_deref(), drops: drops.as_deref(), robots: robots.0.as_deref(), casualties: robots.1.as_deref(), seep: seep.as_deref(), kz: kz.as_deref(), duel: &duel, script: &script };
    let mut rows = Vec::new();
    ledger.rows(&spec.rows, &mut rows);
    // The columns, as a small table: (name, vehicles, destroyed, abandoned).
    let columns: Vec<(String, [u32; 3])> = spec.columns.as_ref().map_or(Vec::new(), |c| stats.per_wave.iter().enumerate().skip(1).map(|(i, w)| (c.name(i), [w.vehicles, w.destroyed, w.abandoned])).collect());
    let col_heads = spec.columns.as_ref().map_or([""; 3], |c| [c.heads.0.as_str(), c.heads.1.as_str(), c.heads.2.as_str()]);
    let words = &bav_core::data::data().words;
    let (closing, sources, keys) = (&words.closing, &words.sources, words.keys.as_str());
    let dim = Color::srgba(0.72, 0.72, 0.70, 0.85);
    // The verdict, where the day states a task: each goal met or missed, and what that meant.
    let verdict = task.verdict();
    // The film's end card reads this: one label and value per row, and the closing lines prefixed `closing\t`; the
    // bands read the verdict's lines, prefixed `task\t`. Written fresh, once, when the screen comes up.
    if let Some(dir) = crate::dev::dev().showcase.as_deref() {
        let mut text = rows.iter().map(|(l, v)| format!("{l}\t{v}\n")).collect::<String>();
        for (g, figure, met) in &verdict {
            text.push_str(&format!("task\t{}\t{}\t{figure}\n", g.name.0, if *met { "met" } else { "missed" }));
        }
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
        // The verdict first: each goal, its figure and whether it was met, and under it what that meant in the day's
        // terms. A goal missed is a historical outcome as much as one met.
        if !verdict.is_empty() {
            p.spawn((Text::new("The task (Завдання)"), tf(13.0), TextColor(dim), Node { margin: UiRect::bottom(px(6)), ..default() }));
            for (g, figure, met) in &verdict {
                p.spawn(Node { flex_direction: FlexDirection::Row, column_gap: px(16), ..default() }).with_children(|r| {
                    r.spawn((Text::new(format!("{} ({})", g.name.0, g.name.1)), tf(16.0), TextColor(WHITE), Node { flex_grow: 1.0, ..default() }));
                    let value = format!("{figure}: {}", if *met { "met (виконано)" } else { "missed (не виконано)" });
                    right(r, value, 16.0, if *met { WHITE } else { AMBER }, 420.0);
                });
                p.spawn((Text::new(if *met { g.met.as_str() } else { g.missed.as_str() }), tf(14.0), TextColor(dim), Node { margin: UiRect::bottom(px(8)), ..default() }));
            }
            p.spawn(Node { height: px(14), ..default() });
        }
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

/// What the debrief reads of a duel, where the day has one (Stepove's afternoon): the hits the feed saw, the hulls, the
/// pair's arms, the FPVs.
#[derive(bevy::ecs::system::SystemParam)]
struct DuelView<'w, 's> {
    time: Res<'w, Time>,
    ledger: Option<Res<'w, bav_sim::bradley::Duel>>,
    vehicles: Query<'w, 's, (&'static crate::units::Vehicle, Has<crate::units::Blinded>)>,
    arms: Query<'w, 's, &'static bav_sim::bradley::BradleyArms>,
    fpv: Res<'w, bav_sim::player::Fpv>,
}

/// Everything the debrief's rows can count.
struct Ledger<'a, 'w, 's> {
    stats: &'a Stats,
    fc: &'a FireControl,
    script: &'a Script,
    positions: &'a Query<'w, 's, &'static Post>,
    soldiers: &'a Query<'w, 's, &'static crate::units::Soldier>,
    evac: &'a bav_sim::evac::Evacuation,
    field: Option<&'a bav_sim::fight::Field>,
    air: Option<&'a bav_sim::air::AirTally>,
    iglas: &'a Query<'w, 's, &'static bav_sim::manpads::Igla>,
    runway: Option<&'a bav_sim::air::Airfield>,
    drops: Option<&'a bav_sim::drones::Drops>,
    robots: Option<&'a bav_sim::robots::Robots>,
    casualties: Option<&'a bav_sim::evac::Casualties>,
    seep: Option<&'a bav_sim::seep::Seep>,
    kz: Option<&'a bav_sim::killzone::KzTally>,
    duel: &'a DuelView<'w, 's>,
}

impl Ledger<'_, '_, '_> {
    /// The file's rows as (label, value), in order.
    fn rows(&self, spec: &[screen::Row], out: &mut Vec<(String, String)>) {
        for row in spec {
            match row {
                screen::Row::Line(label, figure) => out.push((label.clone(), self.figure(figure))),
                screen::Row::If(gate, rows) => {
                    let has = match gate {
                        screen::Gate::Flew => self.air.is_some_and(|a| a.flown > 0),
                        screen::Gate::Field => self.field.is_some(),
                        screen::Gate::Runway => self.runway.is_some(),
                        screen::Gate::Drops => self.drops.is_some(),
                        screen::Gate::Robots => self.robots.is_some(),
                        screen::Gate::Bomber => bav_sim::robots::spec().is_some_and(|s| s.bomber.is_some()),
                        screen::Gate::Seep => self.seep.is_some(),
                        screen::Gate::Killzone => self.kz.is_some(),
                    };
                    if has {
                        self.rows(rows, out);
                    }
                }
                screen::Row::Areas(label) => {
                    out.extend(self.field.into_iter().flat_map(|f| f.zones.iter().map(|z| (label.replace("{area}", &capital(&z.name.0)), held_by(z.held).to_string()))));
                }
                screen::Row::TankHits(none) => self.tank_hits(none, out),
                screen::Row::Runs => {
                    for (at, what) in self.duel.ledger.as_deref().map_or(&[][..], |d| &d.runs[..]) {
                        out.push((format!("    {what}"), self.clock(*at)));
                    }
                }
            }
        }
    }

    /// The feed's times are the clock's since the start of the run; the day's clock started with the battle.
    fn clock(&self, at: f64) -> String {
        let offset = self.duel.time.elapsed_secs_f64() - self.script.t as f64;
        director::clock_text((at - offset) as f32)
    }

    /// Each hit on the duel's tank as the hit feed saw it (what hit it, from where, what it did), the same thing over
    /// again counted once.
    fn tank_hits(&self, none: &str, out: &mut Vec<(String, String)>) {
        let hits = self.duel.ledger.as_deref().map_or(&[][..], |d| &d.hits[..]);
        let mut lines: Vec<(String, String, u32)> = Vec::new();
        for h in hits.iter().filter(|h| h.side == Side::Ru && h.target_name.starts_with("T-90M")) {
            let what = format!("    {}{}: {}", h.weapon, if h.from.is_empty() { String::new() } else { format!(" from {}", h.from) }, h.result);
            match lines.last_mut() {
                Some((w, _, n)) if *w == what => *n += 1,
                _ => lines.push((what, self.clock(h.at), 1)),
            }
        }
        if lines.is_empty() {
            out.push((none.to_string(), String::new()));
        }
        out.extend(lines.into_iter().map(|(w, at, n)| (if n > 1 { format!("{w} (x{n})") } else { w }, at)));
    }

    /// The trench's distance from the furthest our men got: in it, so far short, or nowhere yet.
    fn short(a: f32) -> String {
        let trench = crate::geo::layout().trench_z.expect("a reach short of the trench needs a trench");
        if a == f32::MIN {
            "nowhere yet".to_string()
        } else if a > trench - 1.0 {
            "in the trench".to_string()
        } else {
            format!("{:.0} m short of the trench", trench - a)
        }
    }

    fn figure(&self, figure: &screen::Figure) -> String {
        use crate::units::{VKind, VState};
        use screen::Figure as F;
        let (stats, fc, a) = (self.stats, self.fc, self.air);
        let theirs = |held: Side| {
            let named: Vec<&str> = self.field.map_or(Vec::new(), |f| f.zones.iter().filter(|z| z.held == Some(held)).map(|z| z.name.0.as_str()).collect());
            if named.is_empty() { "nothing".to_string() } else { named.join(", ") }
        };
        match figure {
            F::Committed => stats.vehicles_committed.to_string(),
            F::Out => (stats.destroyed + stats.abandoned).to_string(),
            F::Destroyed => stats.destroyed.to_string(),
            F::Abandoned => stats.abandoned.to_string(),
            F::MineHits => stats.mine_hits.to_string(),
            F::AtgmKills => stats.atgm_kills.to_string(),
            F::ArtilleryKills => stats.artillery_kills.to_string(),
            F::TurretTosses => stats.turret_tosses.to_string(),
            F::Withdrawn => stats.withdrawn.to_string(),
            F::Sent => stats.sent.to_string(),
            F::Returned => stats.returned.to_string(),
            F::PickedUp => stats.picked_up.to_string(),
            F::AttackerDead => stats.attacker_dead.to_string(),
            F::Wounded => stats.wounded.to_string(),
            F::WoundedOut => stats.wounded_out.to_string(),
            F::WoundedLying => self.soldiers.iter().filter(|s| s.side == Side::Ua && s.state == crate::units::SState::Wounded).count().to_string(),
            F::Closest => format!("{:.0} m", (-geo::layout().town.max.y - stats.max_advance).max(0.0)),
            F::DefenderLosses => stats.defender_losses.to_string(),
            F::DefenderVehicles => stats.defender_vehicles.to_string(),
            F::OverwatchLosses => stats.overwatch_losses.to_string(),
            F::Shells => format!("{} of {}", fc.shells, fc.start.shells),
            F::Smoke => format!("{} of {}", fc.smoke, fc.start.smoke),
            F::Evacuations => format!("{} of {}", self.evac.sent, self.evac.sent + self.evac.left),
            F::VehicleReach => Self::short(stats.vehicle_advance),
            F::FootReach => Self::short(stats.max_advance),
            F::PositionsSilenced => {
                let ru = self.positions.iter().filter(|p| p.side == Side::Ru).count();
                let quiet = self.positions.iter().filter(|p| p.side == Side::Ru && p.strength == 0).count();
                format!("{quiet} of {ru}")
            }
            F::DugInKnockedOut => format!("{} of {}", stats.defender_vehicles, bav_sim::data::sim_data().dug_in.len()),
            F::Flown => a.map_or(0, |a| a.flown).to_string(),
            F::Down => a.map_or(0, |a| a.down.len()).to_string(),
            F::DownKind(k) => a.map_or(0, |a| a.down.iter().filter(|(d, _)| d == k).count()).to_string(),
            F::DownBy(w) => a.map_or(0, |a| a.down.iter().filter(|(_, x)| x == w).count()).to_string(),
            F::DrivenOff => a.map_or(0, |a| a.driven_off).to_string(),
            F::MenLanded => self.runway.map_or(0, |r| r.men_landed).to_string(),
            F::MenOut => self.field.map_or(0, |f| f.out).to_string(),
            F::Captured => self.field.map_or(0, |f| f.captured).to_string(),
            F::HeldUntil => self.field.and_then(|f| f.held_until).map_or("the end".into(), director::clock_text),
            F::Held(side) => theirs(*side),
            F::Runway => self.runway.map_or(String::new(), |r| format!("{:.1} km{}", r.longest() / 1000.0, if r.usable() { ", enough for an Il-76" } else { ", too short to land on" })),
            F::Airlift => airlift_row(self.runway).to_string(),
            F::IglasLeft => self.iglas.iter().map(|i| i.missiles).sum::<u32>().to_string(),
            F::Drops => self.drops.map_or(String::new(), |d| format!("{} of {}, {} killed", d.dropped, d.dropped + d.left, stats.deaths.iter().find(|(w, _)| *w == "drops").map_or(0, |(_, n)| *n))),
            F::TankFate => match self.duel.vehicles.iter().find(|(v, _)| v.kind == VKind::T90m && v.side == Side::Ru) {
                None => "never came".to_string(),
                Some((v, blind)) => {
                    let how = match v.state {
                        VState::Destroyed => "burned out",
                        VState::Abandoned => "left in the village",
                        VState::Immobilized => "stopped",
                        VState::Withdrawing => "withdrawing",
                        _ => "still fighting",
                    };
                    let crew = if v.state.alive() && v.crew > 0 { "" } else { ", the crew out" };
                    format!("{}{how}{crew}", if blind { "blinded, " } else { "" })
                }
            },
            F::OtherHulls => {
                let bmps: Vec<_> = self.duel.vehicles.iter().filter(|(v, _)| v.side == Side::Ru && v.kind != VKind::T90m).collect();
                let stopped = bmps.iter().filter(|(v, _)| !v.state.mobile()).count();
                format!("{stopped} of {}", bmps.len())
            }
            F::Trench => if stats.foothold > 0 { "lost" } else { "held" }.to_string(),
            F::Fired25 => {
                let (ap, he) = self.duel.arms.iter().fold((0, 0), |(ap, he), a| (ap + 300 - a.ap.min(300), he + 230 - a.he.min(230)));
                format!("{ap} and {he}")
            }
            F::FpvsLeft => self.duel.fpv.left.to_string(),
            F::Robots => self.robots.map_or(String::new(), |r| format!("{} sent, {} back, {} lost", r.sent, r.back, r.lost)),
            F::Delivered => self.robots.map_or(String::new(), |r| format!("ammunition {}, food and water {}, fuel and batteries {}", r.delivered[0], r.delivered[1], r.delivered[2])),
            F::WoundedCarried => self.casualties.map_or(0, |c| c.out).to_string(),
            F::WoundedWaiting => self.casualties.map_or("none".to_string(), |c| {
                let hours = c.longest(self.duel.time.elapsed_secs_f64()) as f32 * diorama().compression() / 60.0;
                if c.count() == 0 { "none".to_string() } else { format!("{}, the longest {hours:.1} h", c.count()) }
            }),
            F::Sorties => self.robots.map_or(String::new(), |r| format!("{} flown, {} bombs, {} bundles", r.sorties, r.bombs, r.bundles)),
            F::Burrows => self.seep.map_or(0, |s| s.burrows()).to_string(),
            F::BurrowsFound => self.seep.map_or(0, |s| s.found()).to_string(),
            F::BurrowsEmptied => self.seep.map_or(0, |s| s.emptied()).to_string(),
            F::CrewsHit => self.seep.map_or(0, |s| s.crews_hit).to_string(),
            F::TheirSorties => self.kz.map_or(String::new(), |t| format!("{}: {} on radio, {} on fibre, {} from the shoulders", t.sorties.iter().sum::<u32>(), t.sorties[0], t.sorties[1], t.sorties[2])),
            F::TheirHits => self.kz.map_or(String::new(), |t| {
                let hit: Vec<String> = bav_sim::killzone::Class::ALL.iter().zip(t.hits).filter(|(_, n)| *n > 0).map(|(c, n)| format!("{} {n}", c.name())).collect();
                if hit.is_empty() { "nothing".to_string() } else { hit.join(", ") }
            }),
            F::Cars => self.kz.map_or(String::new(), |t| format!("{} out, {} through, {} burnt", t.cars_out, t.cars_through, t.cars_lost)),
            F::CarsTurned => self.kz.map_or(String::new(), |t| if t.cars_turned == 0 { "none".to_string() } else { t.cars_turned.to_string() }),
        }
    }
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

/// What the HUD reads of the robots (`bav_sim::robots`), where the day has them.
#[derive(bevy::ecs::system::SystemParam)]
struct RobotView<'w, 's> {
    robots: Option<Res<'w, bav_sim::robots::Robots>>,
    casualties: Option<Res<'w, bav_sim::evac::Casualties>>,
    ugvs: Query<'w, 's, (&'static bav_sim::robots::Robot, &'static crate::units::Driver, &'static bav_sim::robots::Hand)>,
    vampires: Query<'w, 's, &'static bav_sim::robots::Vampire>,
    book: Res<'w, crate::units::RouteBook>,
}

impl RobotView<'_, '_> {
    /// The tally's line: the robots sent, back and lost, and the wounded waiting for one.
    fn tally(&self, now: f64) -> String {
        let Some(r) = self.robots.as_deref() else { return String::new() };
        let waiting = self.casualties.as_deref().map_or(String::new(), |c| {
            let hours = c.longest(now) as f32 * diorama().compression() / 60.0;
            if c.count() == 0 { "   wounded waiting 0".to_string() } else { format!("   wounded waiting {} ({hours:.1} h)", c.count()) }
        });
        format!("\nRobots sent {}   back {}   lost {}{waiting}", r.sent, r.back, r.lost)
    }

    /// The robot's camera: its number, its link, how fast and how far, what it carries, and whose hand it is in.
    fn robot_line(&self, view: &View, now: f64) -> String {
        let Some((r, d, h)) = view.ground_team.and_then(|e| self.ugvs.get(e).ok()) else { return String::new() };
        let bars = ["DOWN ", "|    ", "||   ", "|||  ", "|||| ", "|||||"][(r.link * 5.0).ceil().clamp(0.0, 5.0) as usize];
        let left = (self.book.0[d.route].line.length() - d.s).max(0.0);
        let load = match (r.cargo, r.wounded) {
            (Some(c), _) => c.name().1.to_uppercase(),
            (None, 0) => "EMPTY".to_string(),
            (None, n) => format!("{n} WOUNDED"),
        };
        let state = if r.leg == bav_sim::robots::Leg::Lost {
            "LOST".to_string()
        } else if now < r.stuck_until {
            format!("STUCK {:.0} s", r.stuck_until - now)
        } else if now < r.down_until {
            "NO LINK: STANDING".to_string()
        } else if let bav_sim::robots::Leg::AtPost(_) = r.leg {
            "AT THE POST".to_string()
        } else if h.held {
            format!("YOUR HAND  {:.0}%  LANE {:+.0} m", h.throttle * 100.0, h.lat)
        } else {
            "ON ITS ROUTE".to_string()
        };
        let way = if r.leg == bav_sim::robots::Leg::Out { format!("{:.2} km home", d.s / 1000.0) } else { format!("{:.2} km to go", left / 1000.0) };
        format!(
            "UGV (НРК) {}   LINK {bars}   {:>2.0} km/h   {load}   {way}   {state}\n[W/S] throttle  [A/D] lane  [G] its route  [RMB] look  [7] next robot  [1] drone",
            r.n,
            d.speed * 3.6
        )
    }

    /// The bomber's camera: the link, the battery, the height and speed, what is left on the rack.
    fn bomber_line(&self, view: &View) -> String {
        let Some(v) = self.vampires.iter().next() else { return String::new() };
        let endurance = bav_sim::robots::spec().and_then(|s| s.bomber).map_or(1.0, |b| b.endurance);
        let agl = v.pos.y - geo::height(v.pos.x, v.pos.z);
        let rack = match v.load {
            bav_sim::robots::Load::Bombs => format!("60 mm x{}", v.left),
            bav_sim::robots::Load::Bundle(c) => if v.left > 0 { format!("BUNDLE: {}", c.name().1.to_uppercase()) } else { "RACK EMPTY".to_string() },
        };
        format!(
            "VAMPIRE (ВАМПІР)   RC {}   BAT {:>3.0}%   ALT {:>4.0} m   {:>3.0} km/h   {rack}{}{}\nW/S A/D fly  R/F climb  Q/E turn  [C] drop  [H] home  [I] thermal  [1] drone",
            if v.control { "OK " } else { "-- " },
            (v.battery / endurance * 100.0).max(0.0),
            agl,
            v.vel.length() * 3.6,
            if v.home { "   HOMING" } else if v.held { "" } else { "   ON ITS OWN" },
            if view.thermal { "   WHITE HOT" } else { "" }
        )
    }

    /// The screen's warning when the robot's or the bomber's link is down.
    fn no_signal(&self, view: &View, now: f64) -> Option<String> {
        match view.mode {
            Mode::Robot => {
                let (r, _, _) = view.ground_team.and_then(|e| self.ugvs.get(e).ok())?;
                if r.leg == bav_sim::robots::Leg::Lost {
                    Some("NO SIGNAL\nthe robot is lost".to_string())
                } else if now < r.down_until {
                    Some(format!("NO SIGNAL\nthe link to robot {} is down ({:.0} s)", r.n, r.down_until - now))
                } else {
                    None
                }
            }
            Mode::Bomber => self.vampires.iter().next().filter(|v| !v.control).map(|_| "NO SIGNAL\nthe Vampire is flying itself home".to_string()),
            _ => None,
        }
    }
}
