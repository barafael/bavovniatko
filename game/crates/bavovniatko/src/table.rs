//! The map table (design/command.md phase 5, design/next.md "the night before"): in Command, after the briefing, the
//! player's Allotment is made here. A paper map of the field, drawn from the layout (the ground's contours, the roads,
//! the tree lines, the minefields we know of, the grey zone, the road heads), with our positions marked on it; beside
//! it the day's catalog of Units and Support with their costs and the points left. A unit picked on the right goes
//! to a position clicked on the map (one where it can stand: men in tree lines, trenches and ruins, a tank in a hide
//! or a tree line); a click on a held position takes it off again; Support is so many of each. It starts from the
//! day's own picks (or `BAV_ALLOT`'s), and every change is checked as the battle will check it (`allot::build`). "To
//! the field" settles the Allotment and starts the battle.
//!
//! During the battle T opens the same map over the picture, the battle running on under it: our units marked where
//! they are, and orders given from it as from the drone's view (`bav_sim::orders`).
//!
//! Headless (`BAV_SKIP_BRIEFING`) the table settles its first picks at once; `BAV_TABLE=open` keeps it open for a shot,
//! `BAV_TABLE=battle` opens it once the battle is on, and `BAV_COMPOSE` makes up a Column on it.
//!
//! On the battle's table a Column is made up and sent on Supply (`bav_sim::supply`): so many of each vehicle there is
//! left, maybe a team to bring into the position, the road head (a click steps to the next), a position clicked on the
//! map with none of ours picked; the panel says how long its way is, how much of it runs through the grey zone, and
//! what it costs.

use bav_core::{
    field::{Ground, PositionKind, field},
    geo,
};
use bav_sim::allot::{self, Pick, SupportKind, UnitKind};
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    text::FontSource,
};

pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        if !allot::at_table() {
            return;
        }
        app.insert_resource(Table { open: false, picks: allot::first_picks(), armed: None, status: String::new(), dirty: true })
            .add_systems(Startup, spawn_table)
            .add_systems(Update, (table_flow, fit_map, table_clicks, table_hover, show_table).chain().run_if(in_state(crate::Phase::Briefing)).in_set(crate::Step::Screen))
            .add_systems(OnEnter(crate::Phase::Playing), to_battle)
            .insert_resource(Composer::from_dev())
            .add_systems(Update, (battle_table, show_spotted, fit_map, battle_clicks, compose_clicks, show_composer).chain().run_if(in_state(crate::Phase::Playing)).in_set(crate::Step::Screen));
    }
}

/// The table's state: open or not, the picks so far, the unit to place next, and the line under the panel.
#[derive(Resource)]
struct Table {
    open: bool,
    picks: Vec<Pick>,
    armed: Option<String>,
    status: String,
    dirty: bool,
}

#[derive(Component)]
struct TableRoot;
/// The space the map has, and the map in it.
#[derive(Component)]
struct MapArea;
#[derive(Component)]
struct MapNode;
/// A position on the map, by its number in the field.
#[derive(Component)]
struct Marker(usize);
#[derive(Component)]
struct MarkerLetter(usize);
/// A unit that goes to a position: picked here, placed on the map.
#[derive(Component)]
struct UnitRow(String);
/// So many of an item: one fewer, one more.
#[derive(Component)]
struct Count(String, i32);
#[derive(Component)]
struct CountText(String);
#[derive(Component)]
struct PointsText;
#[derive(Component)]
struct StatusText;
#[derive(Component)]
struct ToField;
/// The panel before the battle (the Allotment), and the one during it (our units and their orders).
#[derive(Component)]
struct AllotPanel;
#[derive(Component)]
struct BattlePanel;
#[derive(Component)]
struct BattleText;
/// The two panels, to show one or the other.
type Panels<'w, 's> = Query<'w, 's, (&'static mut Node, Has<AllotPanel>), (Or<(With<AllotPanel>, With<BattlePanel>)>, Without<TableRoot>)>;
/// One of our units on the map during the battle, by its wave.
#[derive(Component)]
struct UnitMark(u32);
/// Where the enemy thinks something of ours is (Command's Enemy staff: `bav_sim::staff::Spotted`).
#[derive(Component)]
struct SpottedMark;
/// The Column composer's parts: so many of a vehicle, a team, the road head, and the send button; its texts.
#[derive(Component)]
struct SendCount(String, i32);
#[derive(Component)]
struct TeamPick(String);
#[derive(Component)]
struct HeadPick;
#[derive(Component)]
struct SendButton;
#[derive(Component)]
struct SendText(String);
#[derive(Component)]
struct SupplyText;
#[derive(Component)]
struct HeadText;
#[derive(Component)]
struct ComposeText;

/// The Column being made up on the table.
#[derive(Resource, Default)]
struct Composer {
    vehicles: std::collections::BTreeMap<String, u32>,
    team: Option<String>,
    head: usize,
    to: Option<usize>,
    status: String,
}

impl Composer {
    /// Empty, or as `BAV_COMPOSE` makes it up.
    fn from_dev() -> Self {
        let Some(r) = crate::dev::dev().compose.as_deref() else { return Self::default() };
        let r = bav_sim::supply::parse_request(r).unwrap_or_else(|| panic!("BAV_COMPOSE: cannot read {r:?} (<road head>:<id>+<id>@<position>[:<team id>])"));
        Self { vehicles: r.vehicles.into_iter().collect(), team: r.team, head: r.head, to: Some(r.to), status: String::new() }
    }
}

const PAPER: [u8; 3] = [233, 226, 207];
const INK: Color = Color::srgb(0.16, 0.13, 0.10);
const DIM_INK: Color = Color::srgb(0.42, 0.37, 0.30);
const PANEL: Color = Color::srgb(0.90, 0.87, 0.78);
const ARMED: Color = Color::srgb(0.98, 0.84, 0.45);
const OURS: Color = Color::srgb(0.12, 0.42, 0.20);

/// Map pixels per metre, and the map's size.
const MAP_W: u32 = 1152;
const MAP_H: u32 = 896;

/// Where on the map (0..1 from the north-west corner) a place on the field is: north is up, so our side is at the top.
fn on_map(p: Vec2) -> Vec2 {
    Vec2::new((p.x + geo::HALF_X) / (2.0 * geo::HALF_X), (p.y + geo::HALF_Z) / (2.0 * geo::HALF_Z))
}

fn font_of(assets: &AssetServer, size: f32) -> TextFont {
    TextFont { font: FontSource::Handle(assets.load(crate::text::FONT)), font_size: FontSize::Px(size), ..default() }
}

/// What a catalog id is called: (English, Ukrainian).
fn name_of(id: &str) -> (String, String) {
    let d = bav_sim::data::sim_data();
    d.catalog.iter().find(|u| u.id == id).map(|u| u.name.clone()).or_else(|| d.support.iter().find(|s| s.id == id).map(|s| s.name.clone())).unwrap_or((id.to_string(), String::new()))
}

/// The letter a unit is marked with on the map.
fn letter(id: &str) -> &'static str {
    let d = bav_sim::data::sim_data();
    match d.catalog.iter().find(|u| u.id == id).map(|u| u.what) {
        Some(UnitKind::Post(Some(bav_core::vocab::Atgm::Stugna))) => "S",
        Some(UnitKind::Post(Some(bav_core::vocab::Atgm::Javelin))) => "J",
        Some(UnitKind::Post(Some(_))) => "A",
        Some(UnitKind::Post(None)) => "R",
        Some(UnitKind::Hide) => "T",
        _ => "?",
    }
}

fn spawn_table(mut commands: Commands, assets: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    let map = images.add(paint_map());
    let d = bav_sim::data::sim_data();
    let f = field();
    let text = |s: &str, size: f32, color: Color| (Text::new(s), font_of(&assets, size), TextColor(color));
    commands
        .spawn((
            Node { position_type: PositionType::Absolute, width: percent(100), height: percent(100), padding: UiRect::all(px(16)), column_gap: px(16), display: Display::None, ..default() },
            BackgroundColor(Color::srgb(0.10, 0.09, 0.08)),
            GlobalZIndex(12),
            TableRoot,
        ))
        .with_children(|root| {
            // The map, its positions marked on it, as large as the space beside the panel lets it be (`fit_map`).
            root.spawn((Node { flex_grow: 1.0, flex_basis: px(0), height: percent(100), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() }, MapArea))
                .with_children(|area| {
                area.spawn((Node { width: px(MAP_W as f32 * 0.7), height: px(MAP_H as f32 * 0.7), ..default() }, ImageNode::new(map), MapNode))
                .with_children(|m| {
                    for (i, p) in f.positions.iter().enumerate().filter(|(_, p)| p.ground == Ground::Ours) {
                        let at = on_map(p.at);
                        let size = if p.kind == PositionKind::Treeline { 11.0 } else { 15.0 };
                        m.spawn((
                            Button,
                            Node { position_type: PositionType::Absolute, left: percent(at.x * 100.0), top: percent(at.y * 100.0), width: px(size), height: px(size), margin: UiRect { left: px(-size * 0.5), top: px(-size * 0.5), ..default() }, border: UiRect::all(px(2)), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                            BorderColor::all(marker_ink(p.kind)),
                            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
                            Marker(i),
                        ))
                        .with_child((Text::new(""), font_of(&assets, 10.0), TextColor(Color::WHITE), MarkerLetter(i)));
                    }
                    for (i, h) in f.road_heads.iter().enumerate() {
                        let at = on_map(h.at);
                        // Its number beside it, towards the middle of the map (below it on the top edge).
                        let (left, right) = if at.x < 0.5 { (percent(at.x * 100.0 + 0.8), Val::Auto) } else { (Val::Auto, percent((1.0 - at.x) * 100.0 + 0.8)) };
                        let top = if at.y < 0.05 { px(4) } else { percent(at.y * 100.0) };
                        m.spawn((
                            Node { position_type: PositionType::Absolute, left, right, top, margin: UiRect { top: px(if at.y < 0.05 { 0.0 } else { -18.0 }), ..default() }, ..default() },
                            Text::new(format!("road head {i}")),
                            font_of(&assets, 11.0),
                            TextColor(Color::srgb(0.12, 0.25, 0.55)),
                        ));
                    }
                    let grey = on_map(Vec2::new(-geo::HALF_X, (f.grey.0 + f.grey.1) * 0.5));
                    m.spawn((
                        Node { position_type: PositionType::Absolute, left: percent(1), top: percent(grey.y * 100.0), margin: UiRect { top: px(-8), ..default() }, ..default() },
                        Text::new("GREY ZONE (СІРА ЗОНА)"),
                        font_of(&assets, 12.0),
                        TextColor(DIM_INK),
                    ));
                    m.spawn((Node { position_type: PositionType::Absolute, right: percent(1), top: percent(1), ..default() }, Text::new("OURS (НАШІ)"), font_of(&assets, 13.0), TextColor(OURS)));
                    m.spawn((Node { position_type: PositionType::Absolute, right: percent(1), bottom: percent(1), ..default() }, Text::new("THEIRS (ВОРОГ)"), font_of(&assets, 13.0), TextColor(Color::srgb(0.55, 0.12, 0.10))));
                });
                });
            // The catalog, the support, the points.
            root.spawn((
                Node { flex_direction: FlexDirection::Column, width: px(560), flex_shrink: 0.0, padding: UiRect::all(px(14)), display: Display::None, ..default() },
                BackgroundColor(PANEL),
                BattlePanel,
            ))
            .with_children(|p| {
                p.spawn((text("", 14.0, INK), BattleText));
                // The Column composer: vehicles, a team, the road head, then a position on the map, and send.
                p.spawn(text("\nSEND A COLUMN (ВІДПРАВИТИ КОЛОНУ)", 13.0, DIM_INK));
                p.spawn((text("", 14.0, INK), SupplyText));
                for u in d.catalog.iter().filter(|u| matches!(u.what, UnitKind::Vehicle(_))) {
                    p.spawn(Node { column_gap: px(4), align_items: AlignItems::Start, ..default() }).with_children(|r| {
                        for (sign, label) in [(-1, " − "), (1, " + ")] {
                            r.spawn((Button, Node { padding: UiRect::axes(px(4), px(0)), flex_shrink: 0.0, ..default() }, BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.12)), SendCount(u.id.clone(), sign)))
                                .with_child((Text::new(label), font_of(&assets, 14.0), TextColor(INK)));
                        }
                        r.spawn((Text::new(""), font_of(&assets, 14.0), TextColor(INK), Node { margin: UiRect::left(px(4)), ..default() }, SendText(u.id.clone())));
                    });
                }
                p.spawn(text("a team to bring into the position (розрахунок):", 12.0, DIM_INK));
                for u in d.catalog.iter().filter(|u| matches!(u.what, UnitKind::Post(_))) {
                    p.spawn((Button, Node { padding: UiRect::axes(px(6), px(2)), ..default() }, BackgroundColor(Color::NONE), TeamPick(u.id.clone())))
                        .with_child((text("", 14.0, INK), SendText(u.id.clone())));
                }
                p.spawn((Button, Node { margin: UiRect::top(px(4)), padding: UiRect::axes(px(6), px(2)), align_self: AlignSelf::Start, ..default() }, BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.12)), HeadPick))
                    .with_child((text("", 14.0, INK), HeadText));
                p.spawn((text("", 13.0, INK), ComposeText));
                p.spawn((Button, Node { margin: UiRect::top(px(8)), padding: UiRect::axes(px(14), px(6)), align_self: AlignSelf::Start, ..default() }, BackgroundColor(INK), SendButton))
                    .with_child(text("SEND (ВІДПРАВИТИ)", 15.0, Color::srgb(0.95, 0.92, 0.84)));
            });
            root.spawn((Node { flex_direction: FlexDirection::Column, width: px(560), flex_shrink: 0.0, padding: UiRect::all(px(14)), row_gap: px(4), overflow: Overflow::clip(), ..default() }, BackgroundColor(PANEL), AllotPanel))
                .with_children(|p| {
                    p.spawn(text(&format!("ALLOTMENT (НАРЯД СИЛ)\n{}", bav_core::diorama::diorama().tally_date()), 18.0, INK));
                    p.spawn((text("", 15.0, INK), PointsText));
                    p.spawn(text("\nUNITS (ПІДРОЗДІЛИ): pick one, then a position on the map", 13.0, DIM_INK));
                    for u in &d.catalog {
                        match u.what {
                            UnitKind::Post(_) | UnitKind::Hide => {
                                p.spawn((Button, Node { padding: UiRect::axes(px(6), px(3)), ..default() }, BackgroundColor(Color::NONE), UnitRow(u.id.clone())))
                                    .with_child((text("", 14.0, INK), CountText(u.id.clone())));
                            }
                            UnitKind::Evacuation => count_row(p, &assets, &u.id),
                            UnitKind::Vehicle(_) => {}
                        }
                    }
                    let vehicles: Vec<String> = d.catalog.iter().filter(|u| matches!(u.what, UnitKind::Vehicle(_))).filter_map(|u| {
                        let n = d.allotment.record.iter().filter(|p| p.id == u.id).map(|p| p.count).sum::<u32>();
                        (n > 0).then(|| format!("{} ×{n}", u.name.0))
                    }).collect();
                    if !vehicles.is_empty() {
                        p.spawn(text(&format!("The day's columns, as the script sends them: {}", vehicles.join(", ")), 12.0, DIM_INK));
                    }
                    let heads: Vec<String> = f.road_heads.iter().enumerate().map(|(i, h)| format!("{i}: {} ({})", h.name.0, h.name.1)).collect();
                    p.spawn(text(&format!("Road heads (в'їзди), where our columns come in: {}", heads.join("; ")), 12.0, DIM_INK));
                    p.spawn(text("\nSUPPORT (ПІДТРИМКА)", 13.0, DIM_INK));
                    for s in &d.support {
                        count_row(p, &assets, &s.id);
                    }
                    p.spawn((text("", 13.0, Color::srgb(0.5, 0.12, 0.08)), Node { margin: UiRect::top(px(10)), ..default() }, StatusText));
                    p.spawn((Button, Node { margin: UiRect::top(px(12)), padding: UiRect::axes(px(14), px(8)), align_self: AlignSelf::Start, ..default() }, BackgroundColor(INK), ToField))
                        .with_child(text("TO THE FIELD (НА ПОЗИЦІЇ)   [Enter]", 15.0, Color::srgb(0.95, 0.92, 0.84)));
                });
        });
}

/// A row for so many of an item: [-] the item [+].
fn count_row(p: &mut ChildSpawnerCommands, assets: &AssetServer, id: &str) {
    p.spawn(Node { column_gap: px(4), align_items: AlignItems::Start, ..default() }).with_children(|r| {
        for (sign, label) in [(-1, " − "), (1, " + ")] {
            r.spawn((Button, Node { padding: UiRect::axes(px(4), px(0)), flex_shrink: 0.0, ..default() }, BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.12)), Count(id.to_string(), sign)))
                .with_child((Text::new(label), font_of(assets, 14.0), TextColor(INK)));
        }
        r.spawn((Text::new(""), font_of(assets, 14.0), TextColor(INK), Node { margin: UiRect::left(px(4)), ..default() }, CountText(id.to_string())));
    });
}

fn marker_ink(kind: PositionKind) -> Color {
    match kind {
        PositionKind::Trench => Color::srgb(0.45, 0.28, 0.12),
        PositionKind::Treeline => Color::srgb(0.12, 0.42, 0.20),
        PositionKind::Hide => Color::srgb(0.20, 0.20, 0.22),
        PositionKind::Ruin => Color::srgb(0.40, 0.40, 0.42),
    }
}

/// Enter opens the table from the briefing, and again goes to the field; headless the first picks go at once.
fn table_flow(keys: Res<ButtonInput<KeyCode>>, mut table: ResMut<Table>, mut next: ResMut<NextState<crate::Phase>>, mut root: Single<&mut Node, With<TableRoot>>) {
    let dev = crate::dev::dev();
    if dev.table_open && !table.open {
        table.open = true;
        table.dirty = true;
    } else if dev.skip_briefing && !dev.table_open {
        to_field(&mut table, &mut next);
        return;
    }
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        if table.open {
            to_field(&mut table, &mut next);
        } else {
            table.open = true;
            table.dirty = true;
        }
    }
    root.display = if table.open { Display::Flex } else { Display::None };
}

/// The map as large as its space lets it be, its own shape kept (so the marks stay on what they mark).
fn fit_map(area: Single<&ComputedNode, With<MapArea>>, mut map: Single<&mut Node, With<MapNode>>) {
    let space = area.size() * area.inverse_scale_factor();
    if space.x < 1.0 || space.y < 1.0 {
        return;
    }
    let k = (space.x / MAP_W as f32).min(space.y / MAP_H as f32);
    let (w, h) = (px(MAP_W as f32 * k), px(MAP_H as f32 * k));
    if map.width != w || map.height != h {
        map.width = w;
        map.height = h;
    }
}

/// Settle the Allotment and go to the field; or say why not.
fn to_field(table: &mut Table, next: &mut NextState<crate::Phase>) {
    match allot::settle(&table.picks) {
        Ok(_) => next.set(crate::Phase::Playing),
        Err(e) => {
            table.status = e;
            table.dirty = true;
        }
    }
}

/// The picks after a change, if the battle would take them; else why not.
fn try_picks(table: &mut Table, picks: Vec<Pick>, done: String) {
    match allot::build(&picks) {
        Ok(_) => {
            table.picks = picks;
            table.status = done;
        }
        Err(e) => table.status = e,
    }
    table.dirty = true;
}

#[allow(clippy::type_complexity)]
fn table_clicks(
    mut table: ResMut<Table>,
    mut next: ResMut<NextState<crate::Phase>>,
    markers: Query<(&Interaction, &Marker), Changed<Interaction>>,
    rows: Query<(&Interaction, &UnitRow), Changed<Interaction>>,
    counts: Query<(&Interaction, &Count), Changed<Interaction>>,
    to: Query<&Interaction, (Changed<Interaction>, With<ToField>)>,
) {
    if !table.open {
        return;
    }
    for (i, row) in &rows {
        if *i == Interaction::Pressed {
            table.armed = if table.armed.as_deref() == Some(&row.0) { None } else { Some(row.0.clone()) };
            table.status = match &table.armed {
                Some(id) => format!("{}: now a position on the map", name_of(id).0),
                None => String::new(),
            };
            table.dirty = true;
        }
    }
    for (i, m) in &markers {
        if *i != Interaction::Pressed {
            continue;
        }
        let pos = &field().positions[m.0];
        if let Some(k) = table.picks.iter().position(|p| p.at == Some(m.0)) {
            let mut picks = table.picks.clone();
            let gone = picks.remove(k);
            try_picks(&mut table, picks, format!("{} taken off the {} at position {}", name_of(&gone.id).0, pos.kind.name().0, m.0));
        } else if let Some(id) = table.armed.clone() {
            let mut picks = table.picks.clone();
            picks.push(Pick { id: id.clone(), count: 1, at: Some(m.0) });
            try_picks(&mut table, picks, format!("{} to the {} at position {}", name_of(&id).0, pos.kind.name().0, m.0));
        } else {
            table.status = "Pick a unit on the right first.".into();
            table.dirty = true;
        }
    }
    for (i, c) in &counts {
        if *i != Interaction::Pressed {
            continue;
        }
        let mut picks = table.picks.clone();
        match picks.iter().position(|p| p.id == c.0 && p.at.is_none()) {
            Some(k) => {
                let n = picks[k].count as i32 + c.1;
                if n <= 0 {
                    picks.remove(k);
                } else {
                    picks[k].count = n as u32;
                }
            }
            None if c.1 > 0 => picks.push(Pick { id: c.0.clone(), count: 1, at: None }),
            None => continue,
        }
        try_picks(&mut table, picks, String::new());
    }
    if to.iter().any(|i| *i == Interaction::Pressed) {
        to_field(&mut table, &mut next);
    }
}

/// Over a position: what it is, in the line under the panel.
fn table_hover(mut table: ResMut<Table>, markers: Query<(&Interaction, &Marker), Changed<Interaction>>, mut shown: Local<Option<usize>>) {
    for (i, m) in &markers {
        if *i == Interaction::Hovered && *shown != Some(m.0) {
            *shown = Some(m.0);
            let p = &field().positions[m.0];
            let held = table.picks.iter().find(|k| k.at == Some(m.0)).map(|k| format!(", held by the {}", name_of(&k.id).0)).unwrap_or_default();
            let holds = match (p.kind.holds_men(), p.kind.holds_vehicles()) {
                (true, true) => "men or a tank",
                (true, false) => "men",
                (false, true) => "a tank",
                _ => "nothing",
            };
            table.status = format!("Position {}: {} ({}), for {holds}{held}", m.0, p.kind.name().0, p.kind.name().1);
            table.dirty = true;
        }
    }
}

/// The panel's counts, the points, the line under it, and the markers, after a change.
#[allow(clippy::type_complexity)]
fn show_table(
    mut table: ResMut<Table>,
    mut counts: Query<(&CountText, &mut Text), (Without<PointsText>, Without<StatusText>, Without<MarkerLetter>)>,
    mut points: Single<&mut Text, (With<PointsText>, Without<StatusText>, Without<MarkerLetter>)>,
    mut status: Single<&mut Text, (With<StatusText>, Without<PointsText>, Without<MarkerLetter>)>,
    mut rows: Query<(&UnitRow, &mut BackgroundColor), Without<Marker>>,
    mut markers: Query<(&Marker, &mut BackgroundColor), Without<UnitRow>>,
    mut letters: Query<(&MarkerLetter, &mut Text), (Without<PointsText>, Without<StatusText>, Without<CountText>)>,
) {
    if !table.dirty {
        return;
    }
    table.dirty = false;
    let d = bav_sim::data::sim_data();
    let built = allot::build(&table.picks);
    let spent = built.as_ref().map_or(0, |a| a.spent);
    points.0 = format!("{spent} of {} points, {} left", d.allotment.points, d.allotment.points.saturating_sub(spent));
    status.0 = table.status.clone();
    let taken = |id: &str| table.picks.iter().filter(|p| p.id == id).map(|p| p.count).sum::<u32>();
    for (c, mut t) in &mut counts {
        let (name, cost, available, what) = match (d.catalog.iter().find(|u| u.id == c.0), d.support.iter().find(|s| s.id == c.0)) {
            (Some(u), _) => (&u.name, u.cost, u.available, None),
            (_, Some(s)) => (&s.name, s.cost, s.available, Some(s.what)),
            _ => continue,
        };
        let n = taken(&c.0);
        let each = match what {
            Some(SupportKind::Shells(r) | SupportKind::Brigade(r) | SupportKind::Mortar(r)) => format!(" ({} in all)", r * n),
            _ => String::new(),
        };
        t.0 = format!("{} ({})   {cost} pts   {n} of {available}{each}", name.0, name.1);
    }
    for (row, mut bg) in &mut rows {
        bg.0 = if table.armed.as_deref() == Some(&row.0) { ARMED } else { Color::NONE };
    }
    for (m, mut bg) in &mut markers {
        let held = table.picks.iter().any(|p| p.at == Some(m.0));
        bg.0 = if held { OURS } else { Color::srgba(1.0, 1.0, 1.0, 0.35) };
    }
    for (l, mut t) in &mut letters {
        t.0 = table.picks.iter().find(|p| p.at == Some(l.0)).map(|p| letter(&p.id).to_string()).unwrap_or_default();
    }
}

/// The paper map: the ground's relief and contours (every 5 m, every 25 m darker), the grey zone hatched, the town's
/// blocks, the minefields we know of in red, the river, the roads, the tree lines.
fn paint_map() -> Image {
    let lay = geo::layout();
    let f = field();
    let (w, h) = (MAP_W as usize, MAP_H as usize);
    let m = 2.0 * geo::HALF_X / w as f32;
    let world = |px: f32, py: f32| Vec2::new(-geo::HALF_X + (px + 0.5) * m, -geo::HALF_Z + (py + 0.5) * m);
    let heights: Vec<f32> = (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).map(|(x, y)| {
        let p = world(x as f32, y as f32);
        geo::height(p.x, p.y)
    }).collect();
    let mut px = vec![[0f32; 3]; w * h];
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let hz = heights[i];
            let dx = heights[y * w + (x + 1).min(w - 1)] - heights[y * w + x.saturating_sub(1)];
            let dz = heights[(y + 1).min(h - 1) * w + x] - heights[y.saturating_sub(1) * w + x];
            // Lit from the north-west, gently.
            let shade = 1.0 + (-dx - dz) / (4.0 * m) * 0.9;
            let mut c = PAPER.map(|v| v as f32 / 255.0 * shade.clamp(0.85, 1.08));
            // A contour where the 5 m band changes towards the next pixel.
            let band = |v: f32, step: f32| (v / step).floor();
            let right = heights[y * w + (x + 1).min(w - 1)];
            let below = heights[(y + 1).min(h - 1) * w + x];
            if band(hz, 25.0) != band(right, 25.0) || band(hz, 25.0) != band(below, 25.0) {
                c = mix(c, [0.55, 0.38, 0.22], 0.55);
            } else if band(hz, 5.0) != band(right, 5.0) || band(hz, 5.0) != band(below, 5.0) {
                c = mix(c, [0.60, 0.45, 0.30], 0.3);
            }
            // The grey zone: diagonal hatching.
            let p = world(x as f32, y as f32);
            if (f.grey.0..f.grey.1).contains(&p.y) && (x + y) % 14 < 2 {
                c = mix(c, [0.45, 0.43, 0.40], 0.45);
            }
            // Their barrier minefields, cross-hatched in red.
            if lay.barriers.iter().any(|b| (b.z0..b.z1).contains(&p.y)) && ((x + y) % 10 == 0 || (x + 100 * h - y) % 10 == 0) {
                c = mix(c, [0.75, 0.15, 0.10], 0.55);
            }
            px[i] = c;
        }
    }
    let to_px = |p: Vec2| Vec2::new((p.x + geo::HALF_X) / m - 0.5, (p.y + geo::HALF_Z) / m - 0.5);
    // A stroke lays its coverage first and inks once, so where its dabs overlap it is no darker than anywhere else.
    let stroke = |px: &mut Vec<[f32; 3]>, pts: &[Vec2], r: f32, c: [f32; 3], a: f32, skip: &dyn Fn(f32) -> bool| {
        let mut cover = vec![0f32; w * h];
        let mut s = 0.0;
        for seg in pts.windows(2) {
            let len = seg[0].distance(seg[1]);
            let n = (len / (m * 0.5)).ceil().max(1.0) as usize;
            for k in 0..n {
                let t = k as f32 / n as f32;
                if !skip(s + len * t) {
                    dab(&mut cover, w, h, to_px(seg[0].lerp(seg[1], t)), r);
                }
            }
            s += len;
        }
        for (p, k) in px.iter_mut().zip(&cover) {
            if *k > 0.0 {
                *p = mix(*p, c, k * a);
            }
        }
    };
    let none = |_: f32| false;
    for r in [lay.town, lay.dachas, lay.pavlivka, lay.mykilske] {
        if r.width() < 1.0 {
            continue;
        }
        let (a, b) = (to_px(r.min), to_px(r.max));
        for y in a.y.max(0.0) as usize..(b.y as usize).min(h) {
            for x in a.x.max(0.0) as usize..(b.x as usize).min(w) {
                let block = (x / 6 + y / 6) % 2 == 0 && x % 6 != 0 && y % 6 != 0;
                px[y * w + x] = mix(px[y * w + x], [0.45, 0.42, 0.40], if block { 0.5 } else { 0.15 });
            }
        }
    }
    for (line, width) in &lay.field_mines {
        stroke(&mut px, &line.pts, (width * 0.5 / m).max(1.5), [0.85, 0.30, 0.22], 0.28, &none);
    }
    for sf in &lay.shoulder_fields {
        let road = &lay.roads[sf.road].line;
        for side in [-1.0f32, 1.0] {
            let pts: Vec<Vec2> = (0..=((sf.s1 - sf.s0) / 20.0) as usize).map(|k| sf.s0 + k as f32 * 20.0).map(|s| road.point(s) + road.normal(s) * side * (sf.inner + sf.outer) * 0.5).collect();
            stroke(&mut px, &pts, ((sf.outer - sf.inner) * 0.5 / m).max(1.0), [0.80, 0.25, 0.18], 0.35, &none);
        }
    }
    if let Some(river) = &lay.river {
        stroke(&mut px, &river.pts, 2.0, [0.25, 0.42, 0.68], 0.9, &none);
    }
    for road in &lay.roads {
        let (r, c, dashed) = match road.kind {
            geo::RoadKind::Asphalt => (1.8, [0.18, 0.15, 0.12], false),
            geo::RoadKind::Dirt => (1.3, [0.35, 0.27, 0.18], false),
            geo::RoadKind::Track => (1.0, [0.35, 0.27, 0.18], true),
        };
        let dash = move |s: f32| dashed && (s / 30.0) as i32 % 2 == 1;
        stroke(&mut px, &road.line.pts, r, c, 0.9, &dash);
    }
    for belt in &lay.belts {
        let gap = |s: f32| belt.in_gap(s, 0.0);
        stroke(&mut px, &belt.line.pts, (belt.width * 0.5 / m).max(1.6), [0.22, 0.40, 0.20], 0.85, &gap);
    }
    for h0 in &f.road_heads {
        stroke(&mut px, &[h0.at, h0.at + Vec2::X * 0.01], 6.0, [0.15, 0.30, 0.62], 0.9, &none);
    }
    let data: Vec<u8> = px.iter().flat_map(|c| [(c[0].clamp(0.0, 1.0) * 255.0) as u8, (c[1].clamp(0.0, 1.0) * 255.0) as u8, (c[2].clamp(0.0, 1.0) * 255.0) as u8, 255]).collect();
    Image::new(Extent3d { width: MAP_W, height: MAP_H, depth_or_array_layers: 1 }, TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default())
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// A round dab of a stroke's coverage, soft at its edge.
fn dab(cover: &mut [f32], w: usize, h: usize, at: Vec2, r: f32) {
    let (x0, x1) = ((at.x - r - 1.0).max(0.0) as usize, ((at.x + r + 1.0).max(0.0) as usize).min(w - 1));
    let (y0, y1) = ((at.y - r - 1.0).max(0.0) as usize, ((at.y + r + 1.0).max(0.0) as usize).min(h - 1));
    for y in y0..=y1 {
        for x in x0..=x1 {
            let d = Vec2::new(x as f32, y as f32).distance(at);
            let k = (r + 0.5 - d).clamp(0.0, 1.0);
            let i = y * w + x;
            cover[i] = cover[i].max(k);
        }
    }
}

/// The battle starts: the table closes, and from now on shows our units and takes their orders.
fn to_battle(mut root: Single<&mut Node, With<TableRoot>>, mut panels: Panels) {
    root.display = Display::None;
    for (mut n, allot) in &mut panels {
        n.display = if allot { Display::None } else { Display::Flex };
    }
}

/// During the battle T opens and closes the table; the battle runs on under it. Our units are marked where they are,
/// and the panel lists them, what each is doing, and the keys.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn battle_table(
    mut commands: Commands,
    assets: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ui: ResMut<bav_sim::orders::OrdersView>,
    net: Res<bav_sim::orders::Net>,
    ours: bav_sim::orders::Ours,
    book: Res<bav_sim::units::RouteBook>,
    mut root: Single<&mut Node, With<TableRoot>>,
    map: Single<Entity, With<MapNode>>,
    mut marks: Query<(&mut UnitMark, &mut Node, &mut Visibility, &Children, &mut BorderColor), Without<TableRoot>>,
    mut labels: Query<&mut Text, Without<BattleText>>,
    mut panel: Single<&mut Text, With<BattleText>>,
    mut first: Local<bool>,
) {
    if !*first {
        *first = true;
        ui.on_table = crate::dev::dev().table_battle;
    }
    if keys.just_pressed(KeyCode::KeyT) {
        ui.on_table = !ui.on_table;
    }
    let display = if ui.on_table { Display::Flex } else { Display::None };
    if root.display != display {
        root.display = display;
    }
    if !ui.on_table {
        return;
    }
    let units = bav_sim::orders::units(ours.iter(), &book);
    if ui.selected.is_some_and(|w| !units.iter().any(|u| u.wave == w)) {
        ui.selected = None;
    }
    let mut free = marks.iter_mut();
    for u in &units {
        let at = on_map(u.centre);
        let cs = bav_sim::orders::callsign(u.wave);
        let picked = ui.selected == Some(u.wave);
        let ink = if picked { Color::srgb(0.85, 0.10, 0.05) } else { Color::srgb(0.10, 0.20, 0.55) };
        let line = format!("{} {}", cs.1, u.vehicles.len());
        if let Some((mut mark, mut node, mut vis, children, mut border)) = free.next() {
            mark.0 = u.wave;
            node.left = percent(at.x * 100.0);
            node.top = percent(at.y * 100.0);
            *vis = Visibility::Inherited;
            *border = BorderColor::all(ink);
            if let Some(mut t) = children.first().and_then(|c| labels.get_mut(*c).ok()) {
                t.0 = line;
            }
        } else {
            let mark = commands
                .spawn((
                    Button,
                    Node { position_type: PositionType::Absolute, left: percent(at.x * 100.0), top: percent(at.y * 100.0), margin: UiRect { left: px(-10), top: px(-10), ..default() }, padding: UiRect::axes(px(3), px(1)), border: UiRect::all(px(2)), ..default() },
                    BorderColor::all(ink),
                    BackgroundColor(Color::srgba(0.95, 0.93, 0.86, 0.9)),
                    GlobalZIndex(13),
                    UnitMark(u.wave),
                ))
                .with_child((Text::new(line), font_of(&assets, 11.0), TextColor(INK)))
                .id();
            commands.entity(*map).add_child(mark);
        }
    }
    for (_, _, mut vis, _, _) in free {
        *vis = Visibility::Hidden;
    }
    let mut s = String::from("THE MAP TABLE (КАРТА)   [T] close; the battle goes on\nred rings: where the enemy has spotted us (виявлено)\n\nOUR UNITS (НАШІ ПІДРОЗДІЛИ)\n");
    if units.is_empty() {
        s.push_str("None of ours on the move.\n");
    }
    for u in &units {
        let cs = bav_sim::orders::callsign(u.wave);
        let doing = match net.on_the_way(u.wave) {
            Some(o) => format!("on the net: {} ({})", o.name().0, o.name().1),
            None => format!("{} ({})", u.doing.name().0, u.doing.name().1),
        };
        let mark = if ui.selected == Some(u.wave) { ">" } else { " " };
        s.push_str(&format!("{mark} {} ({}), {} vehicles: {doing}\n", cs.0, cs.1, u.vehicles.len()));
    }
    s.push_str(match ui.selected {
        Some(_) => "\n[G] Advance (Вперед)   [K] Hold (Тримати)   [B] Fall back (Відхід)\nclick a position: Take position (Зайняти позицію)",
        None => "\nclick one of ours on the map to pick it",
    });
    if panel.0 != s {
        panel.0 = s;
    }
}

/// On the table during the battle: a click on one of ours picks it, a click on a position sends the picked one
/// there, and G / K / B order it.
fn battle_clicks(
    keys: Res<ButtonInput<KeyCode>>,
    mut ui: ResMut<bav_sim::orders::OrdersView>,
    mut composer: ResMut<Composer>,
    marks: Query<(&Interaction, &UnitMark), Changed<Interaction>>,
    markers: Query<(&Interaction, &Marker), Changed<Interaction>>,
    mut requests: MessageWriter<bav_sim::orders::OrderRequest>,
) {
    use bav_sim::orders::{Order, OrderRequest};
    if !ui.on_table {
        return;
    }
    for (i, m) in &marks {
        if *i == Interaction::Pressed {
            ui.selected = if ui.selected == Some(m.0) { None } else { Some(m.0) };
        }
    }
    for (i, m) in &markers {
        if *i != Interaction::Pressed {
            continue;
        }
        match ui.selected {
            // A unit picked: it goes there.
            Some(wave) if field().positions[m.0].kind.holds_vehicles() => requests.write(OrderRequest { wave, order: Order::TakePosition(m.0) }),
            Some(_) => continue,
            // Nothing picked: the Column being made up goes there.
            None => {
                composer.to = Some(m.0);
                composer.status.clear();
                continue;
            }
        };
    }
    let Some(wave) = ui.selected else { return };
    for (key, order) in [(KeyCode::KeyG, Order::Advance), (KeyCode::KeyK, Order::Hold), (KeyCode::KeyB, Order::FallBack)] {
        if keys.just_pressed(key) {
            requests.write(OrderRequest { wave, order });
        }
    }
}

/// The Column composer's buttons: one more or fewer of a vehicle, a team or none, the next road head, and send.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn compose_clicks(
    ui: Res<bav_sim::orders::OrdersView>,
    mut composer: ResMut<Composer>,
    counts: Query<(&Interaction, &SendCount), Changed<Interaction>>,
    teams: Query<(&Interaction, &TeamPick), Changed<Interaction>>,
    head: Query<&Interaction, (Changed<Interaction>, With<HeadPick>)>,
    send: Query<&Interaction, (Changed<Interaction>, With<SendButton>)>,
    supply: Res<bav_sim::supply::Supply>,
    mut requests: MessageWriter<bav_sim::supply::ColumnRequest>,
) {
    if !ui.on_table {
        return;
    }
    for (i, c) in &counts {
        if *i == Interaction::Pressed {
            let n = composer.vehicles.entry(c.0.clone()).or_default();
            *n = (*n as i32 + c.1).clamp(0, supply.left(&c.0) as i32) as u32;
            composer.status.clear();
        }
    }
    for (i, t) in &teams {
        if *i == Interaction::Pressed {
            composer.team = if composer.team.as_deref() == Some(&t.0) { None } else { Some(t.0.clone()) };
            composer.status.clear();
        }
    }
    if head.iter().any(|i| *i == Interaction::Pressed) {
        composer.head = (composer.head + 1) % field().road_heads.len().max(1);
    }
    if send.iter().any(|i| *i == Interaction::Pressed) {
        let Some(to) = composer.to else {
            composer.status = "Click a position on the map for it first (with none of ours picked).".into();
            return;
        };
        let r = request(&composer, to);
        match supply.cost(&r) {
            Ok(c) if c as f32 > supply.have => composer.status = format!("Not enough Supply: it needs {c}, there is {:.0}.", supply.have),
            Ok(_) => {
                requests.write(r);
                composer.vehicles.clear();
                composer.team = None;
                composer.status = "Sent.".into();
            }
            Err(e) => composer.status = e,
        }
    }
}

/// The Column as made up so far, to a position.
fn request(composer: &Composer, to: usize) -> bav_sim::supply::ColumnRequest {
    bav_sim::supply::ColumnRequest {
        vehicles: composer.vehicles.iter().filter(|(_, n)| **n > 0).map(|(id, n)| (id.clone(), *n)).collect(),
        team: composer.team.clone(),
        head: composer.head,
        to,
    }
}

/// The composer's texts: the Supply, each vehicle and team with its cost and what is left, the road head, and the
/// Column's way and cost.
#[allow(clippy::type_complexity)]
fn show_composer(
    ui: Res<bav_sim::orders::OrdersView>,
    composer: Res<Composer>,
    supply: Res<bav_sim::supply::Supply>,
    mut texts: ParamSet<(Query<(&SendText, &mut Text)>, Single<&mut Text, With<SupplyText>>, Single<&mut Text, With<HeadText>>, Single<&mut Text, With<ComposeText>>)>,
    mut teams: Query<(&TeamPick, &mut BackgroundColor), Without<Marker>>,
    mut markers: Query<(&Marker, &mut BackgroundColor), Without<TeamPick>>,
) {
    if !ui.on_table {
        return;
    }
    // The position the Column is for, marked on the map.
    for (m, mut bg) in &mut markers {
        let want = if composer.to == Some(m.0) { ARMED } else if bg.0 == ARMED { Color::srgba(1.0, 1.0, 1.0, 0.35) } else { continue };
        if bg.0 != want {
            bg.0 = want;
        }
    }
    let d = bav_sim::data::sim_data();
    let set = |t: &mut Text, s: String| {
        if t.0 != s {
            t.0 = s;
        }
    };
    for (st, mut t) in &mut texts.p0() {
        let Some(u) = d.catalog.iter().find(|u| u.id == st.0) else { continue };
        let chosen = if matches!(u.what, UnitKind::Vehicle(_)) { format!("   {} chosen", composer.vehicles.get(&st.0).copied().unwrap_or(0)) } else { String::new() };
        set(&mut t, format!("{} ({})   {} supply   {} left{chosen}", u.name.0, u.name.1, u.supply, supply.left(&st.0)));
    }
    for (tp, mut bg) in &mut teams {
        bg.0 = if composer.team.as_deref() == Some(&tp.0) { ARMED } else { Color::NONE };
    }
    set(&mut texts.p1(), format!("Supply (підвіз): {:.0}, {:.0} more a minute", supply.have.floor(), supply.per_min));
    let heads = &field().road_heads;
    let h = &heads[composer.head.min(heads.len().saturating_sub(1))];
    set(&mut texts.p2(), format!("from road head {}: {} ({}) >", composer.head, h.name.0, h.name.1));
    let mut s = match composer.to {
        None => "to: click a position on the map (with none of ours picked)".to_string(),
        Some(to) => {
            let p = &field().positions[to];
            let way = match bav_sim::columns::way(composer.head, to) {
                Ok(w) if w.exposed > 50.0 => format!("{:.1} km, {:.1} km of it through the grey zone", w.line.length() / 1000.0, w.exposed / 1000.0),
                Ok(w) => format!("{:.1} km, all on our side", w.line.length() / 1000.0),
                Err(e) => e,
            };
            let cost = match supply.cost(&request(&composer, to)) {
                Ok(c) => format!("costs {c} supply"),
                Err(e) => e,
            };
            format!("to position {to}, the {} ({}): {way}\n{cost}", p.kind.name().0, p.kind.name().1)
        }
    };
    if !composer.status.is_empty() {
        s.push_str(&format!("\n{}", composer.status));
    }
    set(&mut texts.p3(), s);
}

/// On the battle's table, a red ring where the enemy thinks something of ours is: their picture, not the truth, so
/// the ring sits a little off what it marks, and goes when they lose sight of it.
fn show_spotted(
    mut commands: Commands,
    ui: Res<bav_sim::orders::OrdersView>,
    spotted: Option<Res<bav_sim::staff::Spotted>>,
    map: Single<Entity, With<MapNode>>,
    mut marks: Query<(&mut Node, &mut Visibility), With<SpottedMark>>,
) {
    let seen: Vec<Vec2> = match (&spotted, ui.on_table) {
        (Some(s), true) => s.0.values().map(|s| s.at).collect(),
        _ => Vec::new(),
    };
    let mut free = marks.iter_mut();
    for at in &seen {
        let at = on_map(*at);
        match free.next() {
            Some((mut node, mut vis)) => {
                node.left = percent(at.x * 100.0);
                node.top = percent(at.y * 100.0);
                *vis = Visibility::Inherited;
            }
            None => {
                let mark = commands
                    .spawn((
                        Node { position_type: PositionType::Absolute, left: percent(at.x * 100.0), top: percent(at.y * 100.0), width: px(22), height: px(22), margin: UiRect { left: px(-11), top: px(-11), ..default() }, border: UiRect::all(px(2)), border_radius: BorderRadius::MAX, ..default() },
                        BorderColor::all(Color::srgb(0.85, 0.10, 0.05)),
                        GlobalZIndex(13),
                        SpottedMark,
                    ))
                    .id();
                commands.entity(*map).add_child(mark);
            }
        }
    }
    for (_, mut vis) in free {
        *vis = Visibility::Hidden;
    }
}
