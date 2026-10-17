//! Columns (колони, design/command.md): vehicles sent in together from a Road head along a Route to a Position.
//! The slowest sets the pace; at the end they fan out abreast, put their men into the position and hold it, on
//! either side of the battle. A Column's way is the field's tracked route (`bav_core::field::routes`), written into
//! the route book as a holding route (`geo::Route::hold`), so it is driven by the driving systems in `units` like any.
//!
//! Until the map table and Supply exist, `BAV_COLUMN` sends Columns at game times:
//! `"120:0:leo2a6+bradley+bradley@37,300:1:bradley@40"`: when, from which road head, what, to which position.

use crate::units::{Models, Palette, Radio, RouteBook, Side, VKind, VehicleSpec, spawn_vehicle};
use bav_core::{
    field::{Ground, Mover, field, routes},
    geo,
};
use bevy::prelude::*;

pub struct ColumnsPlugin;

impl Plugin for ColumnsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Columns>().add_systems(Update, scheduled_columns.run_if(crate::running).in_set(crate::Step::Script));
        if crate::dev::dev().column.is_some() {
            app.add_systems(Update, column_progress.run_if(crate::running).in_set(crate::Step::Script));
        }
    }
}

/// How many Columns have been sent (each counts its vehicles under a wave of its own, from 100).
#[derive(Resource, Default)]
pub struct Columns {
    pub sent: u32,
}

/// A Column: what, from which road head, to which position.
#[derive(Clone, Debug)]
pub struct ColumnSpec {
    pub kinds: Vec<VKind>,
    pub head: usize,
    pub to: usize,
}

/// Between two vehicles of a Column on the road (m).
const SPACING: f32 = 35.0;

/// Send a Column for our side: its way from the road head to the position, the vehicles in a line at the road
/// head, the lead first, all at the slowest one's pace (a little under it, so every driver can keep it). Its wave.
#[allow(clippy::too_many_arguments)]
pub fn send(commands: &mut Commands, models: &Models, palette: &Palette, book: &mut RouteBook, columns: &mut Columns, radio: &mut Radio, now: f64, order: &ColumnSpec) -> Result<u32, String> {
    if order.kinds.is_empty() {
        return Err("a column of nothing".into());
    }
    let line = way(order.head, order.to)?.line;
    let f = field();
    let pos = &f.positions[order.to];
    let len = line.length();
    let n = order.kinds.len();
    if len < SPACING * n as f32 + 60.0 {
        return Err(format!("position {} is too near road head {} for {n} vehicles", order.to, order.head));
    }
    let r = book.0.len();
    // They fan out over the last 60 m.
    book.0.push(geo::Route { line, deploy_s: len - 60.0, objective: pos.at, hold: true });
    let pace = order.kinds.iter().map(|k| k.speed()).fold(f32::INFINITY, f32::min) * 0.9;
    let wave = 100 + columns.sent;
    columns.sent += 1;
    for (i, &kind) in order.kinds.iter().enumerate() {
        let s = 8.0 + SPACING * (n - 1 - i) as f32;
        spawn_vehicle(commands, models, palette, book, VehicleSpec { side: Side::Ua, kind, route: r, s, lat: 0.0, wave, idx: i as u32, plow: false, smoke: false, rescue: false, column_pace: pace });
    }
    let head = &f.road_heads[order.head].name;
    radio.say2(now, format!("Колона пішла: {n} од., {}.", head.1), format!("Column on its way: {n} vehicles, from {}.", head.0));
    info!("column {wave}: {n} vehicles ({:?}) from road head {} to position {} ({:?} at {:.0},{:.0}), {len:.0} m at {pace:.1} m/s, at {now:.0} s", order.kinds, order.head, order.to, pos.kind, pos.at.x, pos.at.y);
    Ok(wave)
}

/// A Column's way, and how much of it runs forward of our side.
pub struct Way {
    pub line: geo::Polyline,
    /// Metres of it in the grey zone or beyond.
    pub exposed: f32,
}

/// The way a Column takes from a road head to a position of ours: the field's tracked route to a point 45 m behind
/// the position, and the last stretch up to it the way it faces, so the Column fans out abreast along its edge,
/// facing out (into a tree line from the back of it, as vehicles take one).
pub fn way(head: usize, to: usize) -> Result<Way, String> {
    let f = field();
    let pos = f.positions.get(to).ok_or_else(|| format!("no position {to}"))?;
    if pos.ground != Ground::Ours {
        return Err(format!("position {to} is not on our side of the grey zone"));
    }
    let behind = pos.at - pos.facing * 45.0;
    let way = routes().path(head, Mover::Tracked, behind).ok_or_else(|| format!("no way from road head {head} to position {to}"))?;
    let mut pts = way.pts;
    pts.push(pos.at);
    let line = geo::Polyline::new(pts);
    let exposed = line.pts.windows(2).filter(|w| (w[0] + w[1]).y * 0.5 >= f.grey.0).map(|w| w[0].distance(w[1])).sum();
    Ok(Way { line, exposed })
}

/// `BAV_COLUMN`: the Columns, by game time.
fn plan() -> Vec<(f32, ColumnSpec)> {
    let Some(spec) = crate::dev::dev().column.as_deref() else { return Vec::new() };
    spec.split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let parse = || -> Option<(f32, ColumnSpec)> {
                let (head, to) = s.trim().split_once('@')?;
                let mut w = head.split(':');
                let (t, head, kinds) = (w.next()?.parse().ok()?, w.next()?.parse().ok()?, w.next()?);
                let kinds = kinds.split('+').map(|k| k.parse().ok()).collect::<Option<Vec<VKind>>>()?;
                Some((t, ColumnSpec { kinds, head, to: to.parse().ok()? }))
            };
            parse().unwrap_or_else(|| panic!("BAV_COLUMN: cannot read {s:?} (<game s>:<road head>:<kind>+<kind>@<position>)"))
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn scheduled_columns(
    mut commands: Commands,
    time: Res<Time>,
    script: Res<crate::director::Script>,
    models: Res<Models>,
    palette: Res<Palette>,
    mut book: ResMut<RouteBook>,
    mut columns: ResMut<Columns>,
    mut radio: ResMut<Radio>,
    mut planned: Local<Option<Vec<(f32, ColumnSpec)>>>,
) {
    let planned = planned.get_or_insert_with(plan);
    while let Some((t, _)) = planned.first()
        && script.t >= *t
    {
        let (_, order) = planned.remove(0);
        if let Err(e) = send(&mut commands, &models, &palette, &mut book, &mut columns, &mut radio, time.elapsed_secs_f64(), &order) {
            warn!("column not sent: {e}");
        }
    }
}

/// `BAV_COLUMN`: every 30 s of the day, where each Column's vehicles are and what they are doing.
fn column_progress(script: Res<crate::director::Script>, book: Res<RouteBook>, q: Query<(&crate::units::Vehicle, &crate::units::Driver, &Transform)>, mut next: Local<f32>) {
    if script.t < *next {
        return;
    }
    *next = script.t + 30.0;
    for (v, d, tf) in q.iter().filter(|(v, ..)| v.wave >= 100) {
        let len = book.0[d.route].line.length();
        info!("column {}/{}: {:?} {:?} at {:.0},{:.0}, {:.0} of {len:.0} m, {:.1} m/s, at game {:.0} s", v.wave, v.idx, v.kind, v.state, tf.translation.x, tf.translation.z, d.s, d.speed, script.t);
    }
}
