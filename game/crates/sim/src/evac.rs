//! Bringing the wounded out, when the player asks. Wounded men in the field call for evacuation over the net; the
//! player answers a call (V) or marks a place (Alt+click), and a vehicle from the diorama's pool (its file's
//! `evacuation`) drives up the nearest track to there, stops, puts down smoke, takes on whoever reaches it, and
//! comes back. Nothing goes in unless the player sends it: the day does not replay the battle's own rescue.
//! `BAV_EVAC=<seconds>` answers every call after that delay, for the balance runs.

use crate::diorama::{Side, diorama};
use crate::geo;
use crate::player::{MainCamera, cursor_ground};
use crate::units::{Models, Palette, Radio, SState, Soldier, Stats, VKind, Vehicle, VehicleSpec};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct EvacPlugin;

impl Plugin for EvacPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EvacRequest>()
            .insert_resource(Evacuation::new())
            .add_systems(Update, evac_input.in_set(crate::Step::Control).after(crate::dev::DevSystems))
            .add_systems(Update, (answer_requests, call_for_evacuation).chain().run_if(crate::running).in_set(crate::Step::Script).after(crate::director::DirectorSystems));
    }
}

/// The diorama's evacuation vehicles (`evacuation` in its file).
#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub struct EvacSpec {
    pub kind: VKind,
    /// How many there are for the day.
    pub vehicles: u32,
    /// Where on a track they set off from (arc length along it).
    pub start: f32,
}

/// The player asks for an evacuation here.
#[derive(Message, Clone, Copy)]
pub struct EvacRequest {
    pub at: Vec2,
}

/// Where an evacuation vehicle was sent: the arc length along its track where it stops for the men.
#[derive(Component)]
pub struct EvacTarget(pub f32);

#[derive(Resource)]
pub struct Evacuation {
    /// Vehicles not yet sent.
    pub left: u32,
    /// The last call for evacuation from the field: where, and when.
    pub last_call: Option<(Vec2, f64)>,
    /// Calls the balance runs answer for the player: when, and where.
    auto: Vec<(f64, Vec2)>,
    /// The column number the evacuation vehicles count under (after the script's columns).
    pub wave: u32,
    /// Vehicles sent so far.
    pub sent: u32,
    next_check: f64,
}

impl Evacuation {
    fn new() -> Self {
        let d = crate::data::sim_data();
        let columns = d.script.iter().filter(|(_, a)| matches!(a, crate::director::Act::Column { .. })).count() as u32;
        Self { left: d.evacuation.map_or(0, |e| e.vehicles), last_call: None, auto: Vec::new(), wave: columns + 1, sent: 0, next_check: 20.0 }
    }
}

/// V answers the last call; Alt+click asks for an evacuation at the cursor (and fires nothing).
fn evac_input(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    cam: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    time: Res<Time>,
    evac: Res<Evacuation>,
    mut radio: ResMut<Radio>,
    mut requests: MessageWriter<EvacRequest>,
) {
    if keys.just_pressed(KeyCode::KeyV) {
        match evac.last_call {
            Some((at, _)) => {
                requests.write(EvacRequest { at });
            }
            None => radio.say2(time.elapsed_secs_f64(), "Викликів на евакуацію нема.", "No calls for evacuation."),
        }
    }
    let alt = keys.pressed(KeyCode::AltLeft) || keys.pressed(KeyCode::AltRight);
    if alt
        && buttons.just_pressed(MouseButton::Left)
        && let Some(w) = window
        && let Some(hit) = cursor_ground(&w, cam.0, cam.1)
    {
        requests.write(EvacRequest { at: hit.xz() });
    }
}

/// Send a vehicle up the nearest track to where it was asked for, while there are any.
fn answer_requests(
    mut commands: Commands,
    time: Res<Time>,
    mut requests: MessageReader<EvacRequest>,
    mut evac: ResMut<Evacuation>,
    mut radio: ResMut<Radio>,
    mut stats: ResMut<Stats>,
    models: Res<Models>,
    palette: Res<Palette>,
) {
    let now = time.elapsed_secs_f64();
    for r in requests.read() {
        let (lat, lon) = geo::latlon(r.at);
        radio.say2(now, format!("{{command}}, я {{player}}. Евакуація, {lat:.4} {lon:.4}."), format!("{{command}}, this is {{player}}. Evacuation at {lat:.4} {lon:.4}."));
        let Some(spec) = crate::data::sim_data().evacuation else {
            radio.say2(now, "{player}, я {command}. Вдень туди нічого не дійде. Чекатимуть темряви.", "{player}, this is {command}. Nothing gets down there in daylight. They wait for dark.");
            continue;
        };
        if evac.left == 0 {
            radio.say2(now, "{player}, я {command}. Більше нікого відправити.", "{player}, this is {command}. Nothing left to send.");
            continue;
        }
        let lay = geo::layout();
        let Some((route, stop)) = lay.routes.iter().enumerate().map(|(i, rt)| (i, rt.line.project(r.at))).min_by(|a, b| a.1.0.total_cmp(&b.1.0)).map(|(i, (_, s))| (i, s)) else { continue };
        evac.left -= 1;
        let side = Side::Ua;
        let s = spec.start.min(stop - 80.0).max(0.0);
        let e = crate::units::spawn_vehicle(
            &mut commands,
            &models,
            &palette,
            VehicleSpec { side, kind: spec.kind, route, s, lat: 0.0, wave: evac.wave, idx: evac.sent, plow: false, smoke: false, rescue: true },
        );
        commands.entity(e).insert(EvacTarget(stop - 25.0));
        evac.sent += 1;
        info!("evacuation sent: {:?} up route {route} to {:.0},{:.0}, {} left, at {now:.0} s", spec.kind, r.at.x, r.at.y, evac.left);
        stats.vehicles_committed += 1;
        stats.wave(evac.wave).vehicles += 1;
        stats.sent += spec.kind.crew();
        let (uk, en) = if spec.kind == VKind::Bradley { ("Бредлі пішов", "Bradley on its way") } else { ("Машина пішла", "A vehicle on its way") };
        radio.say2(now + 2.0, format!("{{player}}, я {{command}}. {uk}. Лишилось {}.", evac.left), format!("{{player}}, this is {{command}}. {en}. {} left.", evac.left));
    }
}

/// Wounded men lying in a group call for help, every so often, until something comes for them.
fn call_for_evacuation(
    time: Res<Time>,
    mut evac: ResMut<Evacuation>,
    mut radio: ResMut<Radio>,
    mut requests: MessageWriter<EvacRequest>,
    soldiers: Query<(&Soldier, &Transform)>,
    rescues: Query<(&Vehicle, &Transform), With<EvacTarget>>,
) {
    let now = time.elapsed_secs_f64();
    // The balance runs' stand-in for the player.
    let (due, waiting): (Vec<_>, Vec<_>) = std::mem::take(&mut evac.auto).into_iter().partition(|(t, _)| *t <= now);
    evac.auto = waiting;
    for (_, at) in due {
        requests.write(EvacRequest { at });
    }
    if now < evac.next_check {
        return;
    }
    evac.next_check = now + 15.0;
    let wounded: Vec<Vec2> = soldiers.iter().filter(|(s, _)| s.side == Side::Ua && s.state == SState::Wounded).map(|(_, t)| t.translation.xz()).collect();
    let Some(best) = wounded.iter().max_by_key(|p| wounded.iter().filter(|q| q.distance(**p) < 80.0).count()) else { return };
    let group: Vec<Vec2> = wounded.iter().copied().filter(|q| q.distance(*best) < 80.0).collect();
    if group.len() < 2 {
        return;
    }
    let at = group.iter().copied().sum::<Vec2>() / group.len() as f32;
    let served = rescues.iter().any(|(v, t)| v.state.mobile() && t.translation.xz().distance(at) < 200.0);
    let recent = evac.last_call.is_some_and(|(p, t)| p.distance(at) < 150.0 && now - t < 120.0);
    if served || recent || diorama().attacker() != Side::Ua {
        return;
    }
    evac.last_call = Some((at, now));
    info!("evacuation call: {} wounded at {:.0},{:.0}, at {now:.0} s", group.len(), at.x, at.y);
    let (lat, lon) = geo::latlon(at);
    let n = group.len();
    radio.say2(now, format!("Триста, {n}, {lat:.4} {lon:.4}. Потрібна евакуація."), format!("Three hundred, {n} of them, at {lat:.4} {lon:.4}. Need evacuation."));
    if let Some(delay) = crate::dev::dev().evac {
        evac.auto.push((now + delay as f64, at));
    }
}
