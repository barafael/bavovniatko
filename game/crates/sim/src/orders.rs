//! Orders (накази, design/command.md phase 4): in Command the player gives each of our units an Order over the net
//! — Advance (Вперед), Hold (Тримати), Fall back (Відхід), Take position (Зайняти позицію) — and it takes effect
//! when the unit has it: a few seconds for the operator to pass it on and the commander to answer, longer where the
//! enemy's jamming is thick, and never if it is lost in the noise (the player hears nothing back, and says it again).
//!
//! A unit is a group of our vehicles sent in together: a Column (`columns`), or one of the script's waves where we
//! attack. Each answers to a callsign. The order a unit holds is on its vehicles: `Holding` stops them where they
//! are; Take position gives them a way of their own from where they stand to the position (`field::Routes`).
//!
//! The field view of it (`OrdersView`): O opens it in the drone's view; a click on one of ours picks it, G / K / B
//! give Advance / Hold / Fall back, and a click on a position sends it there. `BAV_ORDER` gives orders at game
//! times instead, for the headless runs: `"200:100:hold,260:100:advance,300:100:position@40,500:100:back"`.

use crate::geo;
use crate::player::{MainCamera, cursor_ground};
use crate::units::{self, Coasting, Driver, Gear, Radio, Rescue, RouteBook, Side, VKind, VState, Vehicle};
use bav_core::{
    diorama::{Play, play},
    field::{Mover, field, routes},
};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use std::collections::BTreeMap;

pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
    fn build(&self, app: &mut App) {
        // Nothing of the orders in the Record's world, not even a resource: one more resource there was enough to change
        // the day (same-day, 2026-10-17), so the Record keeps the world it had.
        if play() != Play::Command {
            return;
        }
        app.insert_resource(OrdersView { open: crate::dev::dev().orders_open, selected: None });
        app.add_message::<OrderRequest>()
            .init_resource::<Net>()
            .insert_resource(OrderRng(SmallRng::seed_from_u64(crate::dev::seeded(20230608))))
            .add_systems(Update, orders_input.in_set(crate::Step::Control).after(crate::dev::DevSystems))
            .add_systems(Update, (scheduled_orders, put_on_net, deliver).chain().run_if(crate::running).in_set(crate::Step::Script));
    }
}

/// An Order, as the player gives it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Order {
    Advance,
    Hold,
    FallBack,
    /// To a position of the field's, by number.
    TakePosition(usize),
}

impl Order {
    /// (English, Ukrainian).
    pub fn name(self) -> (&'static str, &'static str) {
        match self {
            Order::Advance => ("Advance", "Вперед"),
            Order::Hold => ("Hold", "Тримати"),
            Order::FallBack => ("Fall back", "Відхід"),
            Order::TakePosition(_) => ("Take position", "Зайняти позицію"),
        }
    }

    /// The unit's answer once it has the order: (Ukrainian, English).
    fn answer(self) -> (&'static str, &'static str) {
        match self {
            Order::Advance => ("Прийняв. Рушаємо", "Copy. Moving on"),
            Order::Hold => ("Прийняв. Стоїмо", "Copy. Holding"),
            Order::FallBack => ("Прийняв. Відходимо", "Copy. Falling back"),
            Order::TakePosition(_) => ("Прийняв. Висуваємось на позицію", "Copy. Moving to the position"),
        }
    }
}

/// The player gives a unit an order.
#[derive(Message, Clone, Copy, Debug)]
pub struct OrderRequest {
    pub wave: u32,
    pub order: Order,
}

/// A unit holding where it was told to: its vehicles stop where they are (and a vehicle holding a firing line
/// stays on it), until the next order.
#[derive(Component)]
pub struct Holding;

/// The field view of the orders: open or not, and which unit is picked.
#[derive(Resource, Default)]
pub struct OrdersView {
    pub open: bool,
    pub selected: Option<u32>,
}

/// Orders on their way, and the last one each unit had (for the screen).
#[derive(Resource, Default)]
pub struct Net {
    pending: Vec<Pending>,
    pub standing: BTreeMap<u32, Order>,
}

impl Net {
    /// The order on its way to a unit, if there is one.
    pub fn on_the_way(&self, wave: u32) -> Option<Order> {
        self.pending.iter().find(|p| p.wave == wave).map(|p| p.order)
    }
}

struct Pending {
    at: f64,
    wave: u32,
    order: Order,
    lost: bool,
}

/// The orders' own stream: what the net does to them never shifts the day's other draws.
#[derive(Resource)]
pub struct OrderRng(SmallRng);

/// Callsigns for our units: (Latin, Cyrillic).
const CALLSIGNS: [(&str, &str); 12] = [
    ("Bereza", "Береза"),
    ("Sokil", "Сокіл"),
    ("Topolia", "Тополя"),
    ("Vovk", "Вовк"),
    ("Kobzar", "Кобзар"),
    ("Leleka", "Лелека"),
    ("Hrim", "Грім"),
    ("Yasen", "Ясен"),
    ("Chaika", "Чайка"),
    ("Dub", "Дуб"),
    ("Kalyna", "Калина"),
    ("Orel", "Орел"),
];

/// A unit's callsign: the script's waves first, then the Columns (from wave 100).
pub fn callsign(wave: u32) -> (&'static str, &'static str) {
    let i = if wave >= 100 { wave - 100 + 6 } else { wave };
    CALLSIGNS[i as usize % CALLSIGNS.len()]
}

/// One of our units on the field: its vehicles, where they are, what they are.
pub struct Unit {
    pub wave: u32,
    pub vehicles: Vec<Entity>,
    pub kinds: Vec<VKind>,
    pub centre: Vec2,
    /// What it is doing, as the screen says it.
    pub doing: Doing,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Doing {
    Moving,
    Holding,
    OnPosition,
    FallingBack,
}

impl Doing {
    pub fn name(self) -> (&'static str, &'static str) {
        match self {
            Doing::Moving => ("moving", "рух"),
            Doing::Holding => ("holding", "стоїть"),
            Doing::OnPosition => ("on its position", "на позиції"),
            Doing::FallingBack => ("falling back", "відходить"),
        }
    }
}

/// Our vehicles that take orders: under way, and not a vehicle sent for the wounded (it answers to the call).
pub type Ours<'w, 's> = Query<'w, 's, (Entity, &'static Vehicle, &'static Driver, &'static Transform, Has<Holding>), (Without<Coasting>, Without<Rescue>)>;

/// The same, to work them over.
type OursMut<'w, 's> = Query<'w, 's, (Entity, &'static mut Vehicle, &'static mut Driver, &'static Transform, Has<Holding>), (Without<Coasting>, Without<Rescue>)>;

/// Whether a vehicle (of `Ours`) takes orders: ours, under way, and driving (where we defend, only a Column does).
pub fn takes_orders(v: &Vehicle, d: &Driver, book: &RouteBook) -> bool {
    v.side == Side::Ua && v.state.mobile() && (v.side == crate::diorama::diorama().attacker() || book.0[d.route].hold)
}

/// Our units now, by wave, from our vehicles (`Ours`).
pub fn units<'a>(vehicles: impl Iterator<Item = (Entity, &'a Vehicle, &'a Driver, &'a Transform, bool)>, book: &RouteBook) -> Vec<Unit> {
    let mut by_wave: BTreeMap<u32, Unit> = BTreeMap::new();
    for (e, v, d, tf, holding) in vehicles {
        if !takes_orders(v, d, book) {
            continue;
        }
        let u = by_wave.entry(v.wave).or_insert(Unit { wave: v.wave, vehicles: Vec::new(), kinds: Vec::new(), centre: Vec2::ZERO, doing: Doing::OnPosition });
        u.vehicles.push(e);
        u.kinds.push(v.kind);
        u.centre += tf.translation.xz();
        // (A vehicle told to hold stays where it is, even one whose crew has started back.)
        let doing = if holding {
            Doing::Holding
        } else if v.state == VState::Withdrawing {
            Doing::FallingBack
        } else if v.state == VState::Overwatch {
            Doing::OnPosition
        } else {
            Doing::Moving
        };
        // The unit is doing what the most urgent of its vehicles is.
        u.doing = match (u.doing, doing) {
            (Doing::FallingBack, _) | (_, Doing::FallingBack) => Doing::FallingBack,
            (Doing::Moving, _) | (_, Doing::Moving) => Doing::Moving,
            (Doing::Holding, _) | (_, Doing::Holding) => Doing::Holding,
            _ => Doing::OnPosition,
        };
    }
    by_wave
        .into_values()
        .map(|mut u| {
            u.centre /= u.vehicles.len() as f32;
            u
        })
        .collect()
}

/// O opens and closes the field view of the orders; in it a click picks one of ours or sends the picked one to a
/// position, and G / K / B order it on, to hold, or back.
#[allow(clippy::too_many_arguments)]
fn orders_input(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    cam: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    view: Res<crate::player::View>,
    mut ui: ResMut<OrdersView>,
    ours: Ours,
    book: Res<RouteBook>,
    mut requests: MessageWriter<OrderRequest>,
) {
    if keys.just_pressed(KeyCode::KeyO) {
        ui.open = !ui.open;
    }
    if !ui.open || view.mode != bav_core::vocab::Mode::Drone {
        return;
    }
    let units = units(ours.iter(), &book);
    if ui.selected.is_some_and(|w| !units.iter().any(|u| u.wave == w)) {
        ui.selected = None;
    }
    if let Some(wave) = ui.selected {
        for (key, order) in [(KeyCode::KeyG, Order::Advance), (KeyCode::KeyK, Order::Hold), (KeyCode::KeyB, Order::FallBack)] {
            if keys.just_pressed(key) {
                requests.write(OrderRequest { wave, order });
            }
        }
    }
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(hit) = window.and_then(|w| cursor_ground(&w, cam.0, cam.1)) else { return };
    let at = hit.xz();
    // One of ours under the cursor: pick it.
    let near = ours.iter().filter(|(e, ..)| units.iter().any(|u| u.vehicles.contains(e))).map(|(_, v, _, tf, _)| (v.wave, tf.translation.xz().distance(at))).filter(|(_, d)| *d < 40.0).min_by(|a, b| a.1.total_cmp(&b.1));
    if let Some((wave, _)) = near {
        ui.selected = Some(wave);
        return;
    }
    // Else a position for the picked one.
    if let Some(wave) = ui.selected
        && let Some(to) = position_near(at)
    {
        requests.write(OrderRequest { wave, order: Order::TakePosition(to) });
    }
}

/// The position for vehicles nearest a place, within 40 m.
pub fn position_near(at: Vec2) -> Option<usize> {
    field().positions.iter().enumerate().filter(|(_, p)| p.kind.holds_vehicles() && p.at.distance(at) < 40.0).min_by(|a, b| a.1.at.distance(at).total_cmp(&b.1.at.distance(at))).map(|(i, _)| i)
}

/// `BAV_ORDER`: the orders, by game time.
fn plan() -> Vec<(f32, OrderRequest)> {
    let Some(spec) = crate::dev::dev().order.as_deref() else { return Vec::new() };
    spec.split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let parse = || -> Option<(f32, OrderRequest)> {
                let mut w = s.trim().split(':');
                let (t, wave, what) = (w.next()?.parse().ok()?, w.next()?.parse().ok()?, w.next()?);
                let order = match what.split_once('@') {
                    Some(("position", n)) => Order::TakePosition(n.parse().ok()?),
                    None => match what {
                        "advance" => Order::Advance,
                        "hold" => Order::Hold,
                        "back" => Order::FallBack,
                        _ => return None,
                    },
                    _ => return None,
                };
                Some((t, OrderRequest { wave, order }))
            };
            parse().unwrap_or_else(|| panic!("BAV_ORDER: cannot read {s:?} (<game s>:<wave>:advance|hold|back|position@<n>)"))
        })
        .collect()
}

/// `BAV_ORDER`'s orders when their time comes, each unit picked as the player would pick it to give it.
fn scheduled_orders(script: Res<crate::director::Script>, mut requests: MessageWriter<OrderRequest>, mut ui: ResMut<OrdersView>, mut planned: Local<Option<Vec<(f32, OrderRequest)>>>) {
    let planned = planned.get_or_insert_with(plan);
    while let Some((t, _)) = planned.first()
        && script.t >= *t
    {
        let r = planned.remove(0).1;
        ui.selected = Some(r.wave);
        requests.write(r);
    }
}

/// The player says the order on the net; it reaches the unit after the operator has passed it on and the commander
/// has answered: four to eight seconds, and up to half a minute more of repeats through jamming. Where the jamming is
/// thick it may not get through at all.
#[allow(clippy::too_many_arguments)]
fn put_on_net(
    time: Res<Time>,
    mut requests: MessageReader<OrderRequest>,
    mut net: ResMut<Net>,
    mut rng: ResMut<OrderRng>,
    mut radio: ResMut<Radio>,
    ew: Res<crate::ew::Ew>,
    ours: Ours,
    book: Res<RouteBook>,
) {
    let now = time.elapsed_secs_f64();
    let units = units(ours.iter(), &book);
    for r in requests.read() {
        let Some(unit) = units.iter().find(|u| u.wave == r.wave) else {
            warn!("order {:?} for wave {}: no such unit of ours", r.order, r.wave);
            continue;
        };
        let cs = callsign(r.wave);
        let what = match r.order {
            Order::TakePosition(n) => {
                let Some(p) = field().positions.get(n) else {
                    warn!("order for wave {}: no position {n}", r.wave);
                    continue;
                };
                let (lat, lon) = geo::latlon(p.at);
                (format!("Зайняти позицію, {}, {lat:.4} {lon:.4}", p.kind.name().1), format!("Take the position, the {}, at {lat:.4} {lon:.4}", p.kind.name().0))
            }
            o => (o.name().1.to_string(), o.name().0.to_string()),
        };
        radio.say2(now, format!("{}, я {{player}}. {}.", cs.1, what.0), format!("{}, this is {{player}}. {}.", cs.0, what.1));
        let jam = ew.radio_jam_at(geo::ground(unit.centre));
        let rng = &mut rng.0;
        let delay = rng.random_range(4.0..8.0) + jam as f64 * rng.random_range(10.0..25.0);
        let lost = rng.random_bool((((jam - 0.45) / 0.55).clamp(0.0, 1.0) * 0.85) as f64);
        info!("order {:?} to {} (wave {}, {} vehicles at {:.0},{:.0}): jamming {jam:.2}, {} in {delay:.1} s, at {now:.0} s", r.order, cs.0, r.wave, unit.vehicles.len(), unit.centre.x, unit.centre.y, if lost { "lost" } else { "through" });
        // A newer order to a unit replaces one still on its way.
        net.pending.retain(|p| p.wave != r.wave);
        net.pending.push(Pending { at: now + delay, wave: r.wave, order: r.order, lost });
    }
}

/// An order reaches its unit, and the unit answers and does it; one lost in the noise is met with silence.
#[allow(clippy::too_many_arguments)]
fn deliver(
    mut commands: Commands,
    time: Res<Time>,
    mut net: ResMut<Net>,
    mut radio: ResMut<Radio>,
    mut book: ResMut<RouteBook>,
    mut rng: ResMut<OrderRng>,
    mut vehicles: OursMut,
) {
    let now = time.elapsed_secs_f64();
    let (due, waiting): (Vec<_>, Vec<_>) = std::mem::take(&mut net.pending).into_iter().partition(|p| p.at <= now);
    net.pending = waiting;
    if due.is_empty() {
        return;
    }
    let units = units(vehicles.iter(), &book);
    for p in due {
        let cs = callsign(p.wave);
        let Some(unit) = units.iter().find(|u| u.wave == p.wave) else { continue };
        if p.lost {
            radio.say2(now, format!("{} не відповідає.", cs.1), format!("{} is not answering.", cs.0));
            info!("order {:?} to {} lost, at {now:.0} s", p.order, cs.0);
            continue;
        }
        let done = match p.order {
            Order::Hold => {
                for &e in &unit.vehicles {
                    commands.entity(e).insert(Holding);
                }
                Ok(())
            }
            Order::Advance => {
                for &e in &unit.vehicles {
                    commands.entity(e).remove::<Holding>();
                    if let Ok((_, mut v, mut d, ..)) = vehicles.get_mut(e)
                        && v.state == VState::Withdrawing
                    {
                        v.state = VState::Advancing;
                        d.gear = Gear::Forward;
                        v.timer = 0.0;
                    }
                }
                Ok(())
            }
            Order::FallBack => {
                for &e in &unit.vehicles {
                    commands.entity(e).remove::<Holding>();
                    if let Ok((_, mut v, mut d, ..)) = vehicles.get_mut(e)
                        && v.state != VState::Withdrawing
                    {
                        units::start_withdrawal(&mut v, &mut d, &mut rng.0);
                    }
                }
                Ok(())
            }
            Order::TakePosition(to) => take_position(&mut commands, &mut book, &mut vehicles, unit, to),
        };
        match done {
            Ok(()) => {
                let (uk, en) = p.order.answer();
                radio.say2(now, format!("{{player}}, я {}. {uk}.", cs.1), format!("{{player}}, this is {}. {en}.", cs.0));
                net.standing.insert(p.wave, p.order);
                info!("order {:?} to {} carried out, at {now:.0} s", p.order, cs.0);
            }
            Err(e) => {
                radio.say2(now, format!("{{player}}, я {}. Туди не пройдемо.", cs.1), format!("{{player}}, this is {}. No way through to there.", cs.0));
                info!("order {:?} to {} cannot be done: {e}, at {now:.0} s", p.order, cs.0);
            }
        }
    }
}

/// The unit's way to a position from where it stands, as a holding route of its own: back along the way it came to
/// a little behind its rearmost vehicle (out of the tree line it holds, the way it went in), then across the field to
/// the position, coming up to it from behind as a Column does. The way it came stays the start of the new one, so it
/// can fall back along it later. Each vehicle joins the way where it is, at the slowest one's pace.
fn take_position(commands: &mut Commands, book: &mut RouteBook, vehicles: &mut OursMut, unit: &Unit, to: usize) -> Result<(), String> {
    let pos = field().positions.get(to).ok_or_else(|| format!("no position {to}"))?;
    // The way it came: the route most of it is on, to 50 m behind the rearmost of it there.
    let mut on: BTreeMap<usize, (u32, f32)> = BTreeMap::new();
    for &e in &unit.vehicles {
        if let Ok((_, _, d, ..)) = vehicles.get(e) {
            let r = on.entry(d.route).or_insert((0, f32::INFINITY));
            *r = (r.0 + 1, r.1.min(d.s));
        }
    }
    let (&came_by, &(_, rearmost)) = on.iter().max_by_key(|(_, (n, _))| *n).ok_or("no vehicles")?;
    let came = &book.0[came_by].line;
    let cut = (rearmost - 50.0).max(0.0);
    let mut pts: Vec<Vec2> = came.pts.iter().zip(&came.cum).take_while(|(_, c)| **c < cut).map(|(p, _)| *p).collect();
    let from = came.point(cut);
    pts.push(from);
    let behind = pos.at - pos.facing * 45.0;
    let ahead = routes().path_from(from, Mover::Tracked, behind).ok_or("no way to the position")?;
    pts.extend(ahead.pts);
    pts.push(pos.at);
    pts.dedup_by(|a, b| a.distance(*b) < 1.0);
    let line = geo::Polyline::new(pts);
    let len = line.length();
    if len - cut < 70.0 {
        return Err("already there".into());
    }
    let route = book.0.len();
    book.0.push(geo::Route { line, deploy_s: len - 60.0, objective: pos.at, hold: true });
    let line = &book.0[route].line;
    let pace = unit.kinds.iter().map(|k| k.speed()).fold(f32::INFINITY, f32::min) * 0.9;
    for &e in &unit.vehicles {
        commands.entity(e).remove::<Holding>();
        let Ok((_, mut v, mut d, tf, _)) = vehicles.get_mut(e) else { continue };
        let (lat, s) = line.lateral_near(tf.translation.xz(), cut, 80.0);
        d.route = route;
        d.s = s;
        d.lat = lat;
        d.target_lat = 0.0;
        v.state = VState::Advancing;
        d.gear = Gear::Forward;
        d.halt = None;
        v.timer = 0.0;
        d.column_pace = pace;
    }
    let every: Vec<String> = (0..=((len - cut) / 150.0) as usize).map(|k| line.point(cut + k as f32 * 150.0)).map(|p| format!("{:.0},{:.0}", p.x, p.y)).collect();
    debug!("orders: the way on: {}", every.join(" "));
    info!("orders: wave {} to position {to} ({:?} at {:.0},{:.0}), {:.0} m on from {cut:.0} m of route {came_by}", unit.wave, pos.kind, pos.at.x, pos.at.y, len - cut);
    Ok(())
}
