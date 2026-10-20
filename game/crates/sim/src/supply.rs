//! Supply (підвіз, design/command.md phase 6): in Command, what comes up the road during the battle, spent on
//! Columns sent in. There is some at the start and more each minute of the day's clock (the diorama's
//! `allotment.supply`); each vehicle and each team costs its catalog `supply`. A Column is vehicles of the day's
//! kinds, as many as there were less what is already out (in the Allotment, or sent before), from a road head to a
//! position (`columns::send`); it may bring a team with it (Stugna, Javelin or riflemen), which takes the position as
//! a post of ours when the Column gets there, and is lost with it if none of it does.
//!
//! The map table composes and sends them (`bavovniatko::table`); `BAV_SEND="200:0:bmp2+mtlb@20:stugna"` sends one at a
//! game time the same way (when, road head, vehicles, position, and the team), for the headless runs.

use crate::allot::UnitKind;
use crate::columns::{self, ColumnSpec, Columns};
use crate::units::{Coasting, Models, Palette, Radio, RouteBook, Side, VState, Vehicle};
use bav_core::{
    diorama::{Play, play},
    field::field,
    vocab::Atgm,
};
use bevy::prelude::*;
use std::collections::BTreeMap;

pub struct SupplyPlugin;

impl Plugin for SupplyPlugin {
    fn build(&self, app: &mut App) {
        // Nothing of it in the Record's world (see `orders`).
        if play() != Play::Command {
            return;
        }
        let (start, per_min) = crate::data::sim_data().allotment.supply;
        app.insert_resource(Supply { have: start as f32, per_min, last_t: 0.0, sent: BTreeMap::new() })
            .init_resource::<Teams>()
            .add_message::<ColumnRequest>()
            .add_systems(Update, (accrue, scheduled_sends, send_requested, teams_arrive).chain().run_if(crate::running).in_set(crate::Step::Script));
    }
}

/// The Supply there is, what comes each minute, and what has been sent of each catalog item.
#[derive(Resource)]
pub struct Supply {
    pub have: f32,
    pub per_min: f32,
    last_t: f32,
    sent: BTreeMap<String, u32>,
}

impl Supply {
    /// How many more of a catalog item there are to send: as many as there were, less the Allotment's and what has
    /// been sent.
    pub fn left(&self, id: &str) -> u32 {
        let d = crate::data::sim_data();
        let Some(u) = d.catalog.iter().find(|u| u.id == id) else { return 0 };
        let allotted = placed(u.what);
        u.available.saturating_sub(allotted + self.sent.get(id).copied().unwrap_or(0))
    }

    /// What a Column costs in Supply, or why it cannot go.
    pub fn cost(&self, r: &ColumnRequest) -> Result<u32, String> {
        let d = crate::data::sim_data();
        let mut cost = 0;
        let mut n = 0;
        for (id, count) in &r.vehicles {
            let u = d.catalog.iter().find(|u| &u.id == id).ok_or_else(|| format!("no {id} in the catalog"))?;
            if !matches!(u.what, UnitKind::Vehicle(_)) {
                return Err(format!("{} is not a vehicle", u.name.0));
            }
            if *count > self.left(id) {
                return Err(format!("{} {}, but {} left", count, u.name.0, self.left(id)));
            }
            cost += u.supply * count;
            n += count;
        }
        if n == 0 {
            return Err("a Column of nothing".into());
        }
        if let Some(id) = &r.team {
            let u = d.catalog.iter().find(|u| &u.id == id).ok_or_else(|| format!("no {id} in the catalog"))?;
            if !matches!(u.what, UnitKind::Post(_)) {
                return Err(format!("{} does not go to a position with a Column", u.name.0));
            }
            if self.left(id) == 0 {
                return Err(format!("no {} left", u.name.0));
            }
            cost += u.supply;
        }
        Ok(cost)
    }
}

/// How many of a catalog unit the Allotment has: the vehicles of a kind, the posts with a weapon, the tanks in hides,
/// the vehicles for the wounded.
fn placed(what: UnitKind) -> u32 {
    let Some(a) = crate::allot::settled() else { return 0 };
    match what {
        UnitKind::Vehicle(k) => a.vehicles.get(&k).copied().unwrap_or(0),
        UnitKind::Post(w) => a.posts.iter().filter(|(_, pw)| *pw == w).count() as u32,
        UnitKind::Hide => a.hides.len() as u32,
        UnitKind::Evacuation => a.evacuation,
        UnitKind::Air(w) => a.air.iter().filter(|(_, aw)| *aw == w).count() as u32,
    }
}

/// A Column to send: so many of each vehicle, maybe a team for the position, from a road head to a position.
#[derive(Message, Clone, Debug)]
pub struct ColumnRequest {
    pub vehicles: Vec<(String, u32)>,
    pub team: Option<String>,
    pub head: usize,
    pub to: usize,
}

/// Teams riding with Columns: the Column's wave, the team's weapon (none: riflemen), its catalog id, and the position.
#[derive(Resource, Default)]
pub struct Teams(Vec<(u32, Option<Atgm>, String, usize)>);

fn accrue(script: Res<crate::director::Script>, mut supply: ResMut<Supply>) {
    let dt = script.t - supply.last_t;
    if dt > 0.0 {
        supply.have += dt / 60.0 * supply.per_min;
        supply.last_t = script.t;
    }
}

/// `BAV_SEND`: the Columns, by game time.
fn plan() -> Vec<(f32, ColumnRequest)> {
    let Some(spec) = crate::dev::dev().send.as_deref() else { return Vec::new() };
    spec.split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let (t, rest) = s.trim().split_once(':').unwrap_or_else(|| panic!("BAV_SEND: cannot read {s:?}"));
            let t: f32 = t.parse().unwrap_or_else(|_| panic!("BAV_SEND: cannot read {s:?}"));
            (t, parse_request(rest).unwrap_or_else(|| panic!("BAV_SEND: cannot read {s:?} (<game s>:<road head>:<id>+<id>@<position>[:<team id>])")))
        })
        .collect()
}

/// A Column written out: `0:bmp2+mtlb@10:stugna` (road head, vehicles, position, and the team).
pub fn parse_request(s: &str) -> Option<ColumnRequest> {
    let mut w = s.trim().split(':');
    let (head, rest) = (w.next()?.parse().ok()?, w.next()?);
    let team = w.next().map(str::to_string);
    let (kinds, to) = rest.split_once('@')?;
    let mut vehicles: Vec<(String, u32)> = Vec::new();
    for k in kinds.split('+') {
        match vehicles.iter_mut().find(|(id, _)| id == k) {
            Some((_, n)) => *n += 1,
            None => vehicles.push((k.to_string(), 1)),
        }
    }
    Some(ColumnRequest { vehicles, team, head, to: to.parse().ok()? })
}

fn scheduled_sends(script: Res<crate::director::Script>, mut requests: MessageWriter<ColumnRequest>, mut planned: Local<Option<Vec<(f32, ColumnRequest)>>>) {
    let planned = planned.get_or_insert_with(plan);
    while let Some((t, _)) = planned.first()
        && script.t >= *t
    {
        requests.write(planned.remove(0).1);
    }
}

/// Send what was asked for, if the Supply is there and so are the vehicles: spend it, and the Column goes.
#[allow(clippy::too_many_arguments)]
fn send_requested(
    mut commands: Commands,
    time: Res<Time>,
    mut requests: MessageReader<ColumnRequest>,
    mut supply: ResMut<Supply>,
    mut teams: ResMut<Teams>,
    models: Res<Models>,
    palette: Res<Palette>,
    mut book: ResMut<RouteBook>,
    mut columns: ResMut<Columns>,
    mut radio: ResMut<Radio>,
) {
    let now = time.elapsed_secs_f64();
    let d = crate::data::sim_data();
    for r in requests.read() {
        let cost = match supply.cost(r) {
            Ok(c) if c as f32 <= supply.have => c,
            Ok(c) => {
                radio.say2(now, format!("{{player}}, я {{command}}. Підвозу не вистачає: треба {c}, є {:.0}.", supply.have), format!("{{player}}, this is {{command}}. Not enough Supply: it needs {c}, there is {:.0}.", supply.have));
                continue;
            }
            Err(e) => {
                warn!("column not sent: {e}");
                continue;
            }
        };
        let mut kinds = Vec::new();
        for (id, n) in &r.vehicles {
            if let Some(UnitKind::Vehicle(k)) = d.catalog.iter().find(|u| &u.id == id).map(|u| u.what) {
                kinds.extend(std::iter::repeat_n(k, *n as usize));
            }
        }
        match columns::send(&mut commands, &models, &palette, &mut book, &mut columns, &mut radio, now, &ColumnSpec { kinds, head: r.head, to: r.to }) {
            Ok(wave) => {
                supply.have -= cost as f32;
                for (id, n) in &r.vehicles {
                    *supply.sent.entry(id.clone()).or_default() += n;
                }
                if let Some(id) = &r.team {
                    *supply.sent.entry(id.clone()).or_default() += 1;
                    if let Some(UnitKind::Post(w)) = d.catalog.iter().find(|u| &u.id == id).map(|u| u.what) {
                        teams.0.push((wave, w, id.clone(), r.to));
                    }
                }
                info!("supply: column {wave} for {cost}, {:.0} left, at {now:.0} s", supply.have);
            }
            Err(e) => radio.say2(now, format!("{{player}}, я {{command}}. Колона не пройде: {e}."), format!("{{player}}, this is {{command}}. The column cannot go: {e}.")),
        }
    }
}

/// A team riding with a Column takes the position when the first of the Column is there; if none of the Column is
/// left on the way, the team is lost with it.
#[allow(clippy::too_many_arguments)]
fn teams_arrive(
    mut commands: Commands,
    time: Res<Time>,
    mut teams: ResMut<Teams>,
    models: Res<Models>,
    palette: Res<Palette>,
    mut radio: ResMut<Radio>,
    vehicles: Query<&Vehicle, Without<Coasting>>,
    posts: Query<&crate::fire::Post>,
) {
    if teams.0.is_empty() {
        return;
    }
    let now = time.elapsed_secs_f64();
    let d = crate::data::sim_data();
    let ours = posts.iter().filter(|p| p.side == Side::Ua).count();
    teams.0.retain(|(wave, weapon, id, to)| {
        let column: Vec<&Vehicle> = vehicles.iter().filter(|v| v.wave == *wave && v.state.mobile()).collect();
        let name = d.catalog.iter().find(|u| &u.id == id).map_or((id.clone(), String::new()), |u| u.name.clone());
        if column.is_empty() {
            radio.say2(now, format!("{{player}}, я {{command}}. Колону втрачено. {} не дійшов.", name.1), format!("{{player}}, this is {{command}}. The column is lost, and the {} with it.", name.0));
            info!("supply: the {} with column {wave} lost, at {now:.0} s", name.0);
            return false;
        }
        if column.iter().any(|v| v.state == VState::Overwatch) {
            let pos = &field().positions[*to];
            crate::fire::spawn_post(&mut commands, &models, &palette, ours, Side::Ua, pos.at, *weapon);
            radio.say2(now, format!("{{player}}, я {}. {} на позиції.", crate::orders::callsign(*wave).1, name.1), format!("{{player}}, this is {}. The {} is in the position.", crate::orders::callsign(*wave).0, name.0));
            info!("supply: the {} with column {wave} in position {to} at {:.0},{:.0}, at {now:.0} s", name.0, pos.at.x, pos.at.y);
            return false;
        }
        true
    });
}
