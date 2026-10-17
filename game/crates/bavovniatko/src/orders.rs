//! The field view of the orders (`bav_sim::orders`), in Command: over the drone's picture, our units ringed and
//! named by callsign, the positions vehicles can take, and the picked unit's way ahead; on the left, the units with
//! what each is doing and the keys for the orders.

use crate::camera::{Mode, View};
use bav_core::{
    diorama::{Play, play},
    field::{Ground, field},
};
use bav_sim::orders::{Doing, Net, Ours, OrdersView, callsign, position_near, takes_orders, units};
use bav_sim::player::{MainCamera, cursor_ground};
use bav_sim::units::RouteBook;
use bevy::{prelude::*, text::FontSource, window::PrimaryWindow};

pub struct OrdersScreenPlugin;

impl Plugin for OrdersScreenPlugin {
    fn build(&self, app: &mut App) {
        if play() != Play::Command {
            return;
        }
        app.add_systems(Startup, spawn_panel).add_systems(Update, (panel, marks).in_set(crate::Step::Screen));
    }
}

/// The units and the keys, on the left.
#[derive(Component)]
struct Panel;

/// A callsign over one of our units on the picture.
#[derive(Component)]
struct Label;

type Labels<'w, 's> = Query<'w, 's, (&'static mut Text, &'static mut Node, &'static mut Visibility), (With<Label>, Without<Panel>)>;

const GREEN: Color = Color::srgb(0.45, 1.0, 0.55);
const PICKED: Color = Color::srgb(1.0, 1.0, 1.0);
const OURS: Color = Color::srgb(0.35, 0.85, 1.0);

fn spawn_panel(mut commands: Commands, assets: Res<AssetServer>) {
    let font: Handle<Font> = assets.load(crate::text::FONT);
    commands.spawn((
        Text::new(""),
        TextFont { font: FontSource::Handle(font), font_size: FontSize::Px(14.0), ..default() },
        TextColor(Color::srgba(0.95, 0.96, 0.97, 0.92)),
        TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.8) },
        Node { position_type: PositionType::Absolute, top: px(110), left: px(18), max_width: px(560), ..default() },
        Panel,
    ));
}

/// How the panel names a unit's vehicles: "Leopard 2A6, Bradley ×2".
fn vehicles(kinds: &[bav_sim::units::VKind]) -> String {
    let mut counted: Vec<(bav_sim::units::VKind, u32)> = Vec::new();
    for &k in kinds {
        match counted.iter_mut().find(|(c, _)| *c == k) {
            Some((_, n)) => *n += 1,
            None => counted.push((k, 1)),
        }
    }
    counted.iter().map(|(k, n)| if *n > 1 { format!("{} ×{n}", k.name()) } else { k.name().to_string() }).collect::<Vec<_>>().join(", ")
}

/// The units, what each is doing (or the order on its way to it), and the keys; closed, only the key to open it.
#[allow(clippy::too_many_arguments)]
fn panel(
    mut commands: Commands,
    assets: Res<AssetServer>,
    ui: Res<OrdersView>,
    net: Res<Net>,
    view: Res<View>,
    ours: Ours,
    book: Res<RouteBook>,
    cam: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut text: Single<&mut Text, With<Panel>>,
    mut labels: Labels,
) {
    let open = ui.open && view.mode == Mode::Drone;
    let units = if open { units(ours.iter(), &book) } else { Vec::new() };
    let s = if !open {
        "[O] Orders (Накази)".to_string()
    } else {
        let mut s = String::from("ORDERS (НАКАЗИ)   [O] close\n");
        if units.is_empty() {
            s.push_str("\nNone of ours on the field.\n");
        }
        for u in &units {
            let cs = callsign(u.wave);
            let doing = match net.on_the_way(u.wave) {
                Some(o) => format!("on the net: {} ({})", o.name().0, o.name().1),
                None => format!("{} ({})", u.doing.name().0, u.doing.name().1),
            };
            let mark = if ui.selected == Some(u.wave) { ">" } else { " " };
            s.push_str(&format!("{mark} {} ({})   {}   {doing}\n", cs.0, cs.1, vehicles(&u.kinds)));
        }
        s.push('\n');
        s.push_str(match ui.selected {
            Some(_) => "[G] Advance (Вперед)   [K] Hold (Тримати)   [B] Fall back (Відхід)\nclick a green ring: Take position (Зайняти позицію)",
            None => "click one of ours to pick it",
        });
        s
    };
    text.set_if_neq(Text::new(s));
    // The callsigns over the units, a label each, made as more are needed.
    let (camera, cam_tf) = *cam;
    let mut free: Vec<_> = labels.iter_mut().collect();
    for (i, u) in units.iter().enumerate() {
        let at = crate::geo::ground(u.centre) + Vec3::Y * 14.0;
        let Ok(p) = camera.world_to_viewport(cam_tf, at) else { continue };
        let cs = callsign(u.wave);
        let line = format!("{} · {}{}", cs.1, cs.0, if u.doing == Doing::Holding { "  ■" } else { "" });
        if let Some((t, node, vis)) = free.get_mut(i) {
            t.set_if_neq(Text::new(line));
            node.left = px(p.x - 40.0);
            node.top = px(p.y - 22.0);
            vis.set_if_neq(Visibility::Inherited);
        } else {
            commands.spawn((
                Text::new(line),
                TextFont { font: FontSource::Handle(assets.load(crate::text::FONT)), font_size: FontSize::Px(13.0), ..default() },
                TextColor(OURS),
                TextShadow { offset: Vec2::new(1.0, 1.0), color: Color::srgba(0.0, 0.0, 0.0, 0.85) },
                Node { position_type: PositionType::Absolute, left: px(p.x - 40.0), top: px(p.y - 22.0), ..default() },
                Label,
            ));
        }
    }
    for (_, _, vis) in free.iter_mut().skip(units.len()) {
        vis.set_if_neq(Visibility::Hidden);
    }
}

/// Over the picture: the positions vehicles can take on our side and in the grey zone (the one under the cursor
/// brighter while a unit is picked), each of our vehicles ringed, and the picked unit's way ahead.
fn marks(
    mut gizmos: Gizmos,
    ui: Res<OrdersView>,
    view: Res<View>,
    ours: Ours,
    book: Res<RouteBook>,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    cam: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if !ui.open || view.mode != Mode::Drone {
        return;
    }
    let flat = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let lift = |p: Vec2| crate::geo::ground(p) + Vec3::Y * 2.0;
    let hovered = ui.selected.and(window.and_then(|w| cursor_ground(&w, cam.0, cam.1))).and_then(|h| position_near(h.xz()));
    for (i, p) in field().positions.iter().enumerate() {
        if !p.kind.holds_vehicles() || p.ground == Ground::Theirs {
            continue;
        }
        let c = if p.ground == Ground::Grey { Color::srgb(1.0, 0.9, 0.2) } else { GREEN };
        let (r, c) = if hovered == Some(i) { (16.0, PICKED) } else { (9.0, c.with_alpha(0.75)) };
        gizmos.circle(Isometry3d::new(lift(p.at), flat), r, c);
        gizmos.line(lift(p.at), lift(p.at + p.facing * 18.0), c);
    }
    let mut way_drawn = false;
    for (_, v, d, tf, _) in ours.iter() {
        if !takes_orders(v, d, &book) {
            continue;
        }
        let picked = ui.selected == Some(v.wave);
        let at = tf.translation.xz();
        gizmos.circle(Isometry3d::new(lift(at), flat), if picked { 9.0 } else { 7.0 }, if picked { PICKED } else { OURS });
        // The way ahead, from one vehicle of the picked unit (every 20 m, along the ground).
        if picked && !way_drawn {
            way_drawn = true;
            let line = &book.0[d.route].line;
            let (from, to) = if v.state == bav_sim::units::VState::Withdrawing { (0.0, d.s) } else { (d.s, line.length()) };
            let n = ((to - from) / 20.0).ceil().max(1.0) as usize;
            gizmos.linestrip((0..=n).map(|k| lift(line.point(from + (to - from) * k as f32 / n as f32))), PICKED.with_alpha(0.8));
        }
    }
}
