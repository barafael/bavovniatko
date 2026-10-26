//! Robots (design/myrnohrad.md, phase 5): what took the trucks' place on the road into the pocket, where nothing on
//! wheels got through in clear weather (c15 §5).
//!
//! - **The ground robot** (`VKind::Ugv`, a tracked cart like the Zmiy: 500 kg, 10 km/h, 20 km of range) drives as a
//!   Column of one from the road head to a position of ours (`columns::way`), unloads its cargo there (ammunition for
//!   the post, food and water, fuel and batteries for the drone team), takes on the wounded waiting there, and
//!   comes back. Its radio link frays in their EW and drops out, now and then for the best part of an hour of the
//!   day's clock (the Zmiy's lasted 1.5 hours, c15 §5), and without it the robot stands where it is; it sticks in
//!   craters and mud, and water is the end of it. In the Record the script sends the day's runs (`Act::Ugv`); where
//!   the wounded wait for a robot (`casevac`), V sends one for the longest-waiting call and Alt+click one with
//!   ammunition to the position nearest the cursor; in Command the robots come on Supply, and a UGV sent from the map
//!   table as a Column carries ammunition.
//!   Key 7 is the robot's own camera (`Mode::Robot`), from which the player can drive it (`Hand`) or leave it to its
//!   route.
//! - **The heavy bomber** (a Vampire, 01 §3: about 15 kg over 20 km, a dozen sorties in its life) flies a sortie from
//!   the drone team's position with four 60 mm bombs (never 120 mm, c15 §5) or a bundle for a post. Key 8 launches
//!   one and flies it (`Mode::Bomber`: the drone's sticks, C lets go, H brings it home); the script's sorties
//!   (`Act::Sortie`) fly themselves to their point, drop and come home.
//!
//! None of it exists where the day's file has no `robots`: an entity or a resource more renumbers an older day's and
//! changes it. `BAV_UGV` and `BAV_BOMBER` send runs and sorties at game times, for the bands.

mod bomber;
mod ugv;

pub use bomber::*;
pub use ugv::*;

use crate::units::Radio;
use bav_core::vocab::Mode;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::{SeedableRng, rngs::SmallRng};
use serde::Deserialize;

pub struct RobotsPlugin;

impl Plugin for RobotsPlugin {
    fn build(&self, app: &mut App) {
        let Some(spec) = spec() else { return };
        // In Command the ground robots come up on Supply, sent from the map table as Columns.
        let left = if crate::allot::at_table() { 0 } else { spec.ugvs };
        app.insert_resource(Robots { left, sorties_left: spec.bomber.map_or(0, |b| b.sorties), ..default() })
            .insert_resource(RobotRng(SmallRng::seed_from_u64(crate::dev::seeded(5150))))
            .add_systems(Update, robot_input.run_if(crate::running).in_set(crate::Step::Control).after(crate::dev::DevSystems))
            // After the script, the evacuation's calls and the drone team's turn: they all speak on the same net.
            .add_systems(
                Update,
                (scheduled, send_runs, adopt, link, going, peril, at_the_post, home_or_lost, launch_sorties, fly, bombs_fall, log_robots)
                    .chain()
                    .run_if(crate::running)
                    .in_set(crate::Step::Script)
                    .in_set(RobotSystems)
                    .after(crate::evac::EvacSystems)
                    .after(crate::ew::ew_update),
            );
    }
}

/// The robots' systems, for what must run after them (their drones over the ground, `killzone`).
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RobotSystems;

/// The day's robots (`robots` in its file).
#[derive(Deserialize, Debug, Clone)]
pub struct RobotsSpec {
    /// Ground robots for the day: the runs the script and the player may send between them.
    pub ugvs: u32,
    /// The wounded wait for a robot: the calls for evacuation become a queue only a robot empties
    /// (`evac::Casualties`; c15 §5, days to two weeks).
    #[serde(default)]
    pub casevac: bool,
    /// The wounded already waiting when the day begins, from the days before it (an earlier part's aftermath).
    #[serde(default)]
    pub waiting: Vec<WaitingSpec>,
    /// How often an hour of the day's clock a robot's link drops out with no jamming at all (the ground, the
    /// relays, the range); their EW makes it up to five times as often.
    #[serde(default = "drops")]
    pub drops_per_hour: f32,
    /// The chance per kilometre driven by day that their FPVs find a robot, twice that off our ground and a third of
    /// it in the dark: the road's danger until their drone crews are things of their own (phase 3).
    #[serde(default)]
    pub found_per_km: f32,
    /// The heavy bomber (none: the day has none).
    #[serde(default)]
    pub bomber: Option<BomberSpec>,
}

/// Wounded waiting at a post at first light: `n` of them at the layout's `ua_positions[post]`, waiting `hours` of the
/// clock already.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct WaitingSpec {
    pub post: usize,
    pub n: u32,
    pub hours: f32,
}

fn drops() -> f32 {
    0.5
}

/// The heavy bomber's day.
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct BomberSpec {
    /// Sorties left in it.
    pub sorties: u32,
    /// How long one flies (game s) before it has to be home.
    pub endurance: f32,
}

pub fn spec() -> Option<&'static RobotsSpec> {
    crate::data::sim_data().robots.as_ref()
}

/// Whether the day has robots (and so anything of this module).
pub fn present() -> bool {
    spec().is_some()
}

/// Whether the day's wounded wait for a robot.
pub fn casevac() -> bool {
    spec().is_some_and(|s| s.casevac)
}

/// What a robot or a bundle carries in.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cargo {
    /// Ammunition for the post (`fight::Ammo` where the posts carry it).
    Ammo,
    /// Food and water.
    Food,
    /// Fuel and batteries for the drone team: c15 §5, keeping the drone teams in fuel took many sorties.
    Power,
}

impl Cargo {
    /// Its name on the net, in Ukrainian and English.
    pub fn name(self) -> (&'static str, &'static str) {
        match self {
            Cargo::Ammo => ("БК", "ammunition"),
            Cargo::Food => ("їжа і вода", "food and water"),
            Cargo::Power => ("пальне і батареї", "fuel and batteries"),
        }
    }

    fn index(self) -> usize {
        self as usize
    }

    fn parse(s: &str) -> Option<Self> {
        match s {
            "ammo" => Some(Cargo::Ammo),
            "food" => Some(Cargo::Food),
            "power" => Some(Cargo::Power),
            _ => None,
        }
    }
}

/// What a bomber takes up.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Load {
    /// Four 60 mm mortar bombs on its releases.
    Bombs,
    /// A bundle of this for a post.
    Bundle(Cargo),
}

/// Where a run goes.
#[derive(Clone, Copy, Debug)]
pub enum Dest {
    /// A position of the field, by number.
    Position(usize),
    /// The position nearest the longest-waiting call for evacuation.
    Wounded,
    /// The position of ours nearest a point.
    Near(Vec2),
}

/// A run or a sortie asked for, by the script, the player or the switches; sent in `send_runs` and
/// `launch_sorties`.
#[derive(Clone, Copy, Debug)]
pub enum Order {
    Run { to: Dest, cargo: Cargo },
    /// A sortie at a point (none: the player flies it), and what it carries.
    Sortie { to: Option<Vec2>, load: Load, flown: bool },
}

/// The robots' day: what is left, what went, what came back.
#[derive(Resource, Default)]
pub struct Robots {
    /// Ground robots not yet sent.
    pub left: u32,
    pub sent: u32,
    pub back: u32,
    pub lost: u32,
    /// Loads delivered, by cargo: ammunition, food and water, fuel and batteries.
    pub delivered: [u32; 3],
    /// The bomber's sorties left, flown, bombs dropped and bundles let go.
    pub sorties_left: u32,
    pub sorties: u32,
    pub bombs: u32,
    pub bundles: u32,
    orders: Vec<Order>,
}

/// Ask for a run or a sortie (the script's acts, `director::Act::Ugv` and `Act::Sortie`).
pub fn order(w: &mut World, o: Order) {
    if let Some(mut r) = w.get_resource_mut::<Robots>() {
        r.orders.push(o);
    }
}

/// The robots' own stream: their links, the ground and the FPVs never shift the rest of the day's draws.
#[derive(Resource)]
pub struct RobotRng(pub SmallRng);

/// The switches' runs and sorties (`BAV_UGV`, `BAV_BOMBER`), by game time.
fn scheduled(script: Res<crate::director::Script>, mut robots: ResMut<Robots>, mut done: Local<(usize, usize)>) {
    let dev = crate::dev::dev();
    while let Some((t, what)) = dev.ugv.get(done.0)
        && script.t >= *t
    {
        done.0 += 1;
        let mut w = what.split(':');
        let to = w.next().unwrap_or_default();
        let cargo = w.next().map(|c| Cargo::parse(c).unwrap_or_else(|| panic!("BAV_UGV: no cargo {c:?} (ammo, food, power)"))).unwrap_or(Cargo::Ammo);
        let to = if to == "wounded" { Dest::Wounded } else { Dest::Position(to.parse().unwrap_or_else(|_| panic!("BAV_UGV: cannot read {what:?} (<position>[:<cargo>] or wounded)"))) };
        robots.orders.push(Order::Run { to, cargo });
    }
    while let Some((t, what)) = dev.bomber.get(done.1)
        && script.t >= *t
    {
        done.1 += 1;
        let mut w = what.split(':');
        let at = w.next().and_then(|p| p.split_once(';')).and_then(|(x, z)| Some(Vec2::new(x.trim().parse().ok()?, z.trim().parse().ok()?))).unwrap_or_else(|| panic!("BAV_BOMBER: cannot read {what:?} (<x>;<z>[:bundle])"));
        let load = if w.next() == Some("bundle") { Load::Bundle(Cargo::Ammo) } else { Load::Bombs };
        robots.orders.push(Order::Sortie { to: Some(at), load, flown: false });
    }
}

/// The player's hand on the robots: where the wounded wait for a robot, V sends one for the longest-waiting call and
/// Alt+click one with ammunition to the position nearest the cursor; 8 launches the bomber (Shift+8 with a bundle of
/// ammunition) and puts the player in its camera, or back in it while it flies; in its camera C lets a bomb go and H
/// brings it home.
#[allow(clippy::too_many_arguments)]
fn robot_input(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    cam: Single<(&Camera, &GlobalTransform), With<crate::player::MainCamera>>,
    time: Res<Time>,
    mut view: ResMut<crate::player::View>,
    mut robots: ResMut<Robots>,
    mut radio: ResMut<Radio>,
    mut vampires: Query<&mut Vampire>,
    orders: Option<Res<crate::orders::OrdersView>>,
) {
    let now = time.elapsed_secs_f64();
    if casevac() && view.mode != Mode::Fpv {
        if keys.just_pressed(KeyCode::KeyV) {
            robots.orders.push(Order::Run { to: Dest::Wounded, cargo: Cargo::Food });
        }
        let alt = keys.pressed(KeyCode::AltLeft) || keys.pressed(KeyCode::AltRight);
        if alt
            && !orders.is_some_and(|o| o.takes_clicks())
            && buttons.just_pressed(MouseButton::Left)
            && let Some(w) = window
            && let Some(hit) = crate::player::cursor_ground(&w, cam.0, cam.1)
        {
            robots.orders.push(Order::Run { to: Dest::Near(hit.xz()), cargo: Cargo::Ammo });
        }
    }
    if spec().is_some_and(|s| s.bomber.is_some()) && keys.just_pressed(KeyCode::Digit8) && view.mode != Mode::Fpv {
        if !vampires.is_empty() {
            view.mode = Mode::Bomber;
        } else if robots.sorties_left == 0 {
            radio.say2(now, "Вампіра більше не піднімемо сьогодні.", "The Vampire has no more sorties in it today.");
        } else {
            let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
            robots.orders.push(Order::Sortie { to: None, load: if shift { Load::Bundle(Cargo::Ammo) } else { Load::Bombs }, flown: true });
            view.mode = Mode::Bomber;
        }
    }
    if view.mode == Mode::Bomber {
        for mut v in &mut vampires {
            v.release |= keys.just_pressed(KeyCode::KeyC);
            if keys.just_pressed(KeyCode::KeyH) && !v.home {
                v.home = true;
                radio.say2(now, "Вампір додому.", "Bringing the Vampire home.");
            }
        }
    }
}

/// For the bands (`BAV_LOG_STATS`): the robots' day once a minute of game time: sent, back, lost and left, the loads
/// delivered (ammunition, food, fuel and batteries), the wounded waiting, brought out and dead aboard, and the
/// bomber's sorties left and flown, bombs and bundles.
fn log_robots(script: Res<crate::director::Script>, robots: Res<Robots>, casualties: Option<Res<crate::evac::Casualties>>, mut last: Local<u32>) {
    if !crate::dev::dev().log_stats || script.t as u32 / 60 == *last {
        return;
    }
    *last = script.t as u32 / 60;
    let (waiting, out, died) = casualties.as_deref().map_or((0, 0, 0), |c| (c.count(), c.out, c.died));
    let r = &*robots;
    info!(
        "robots: t={:>4.0} {} robots={}/{}/{}/{} delivered={}/{}/{} casevac={waiting}/{out}/{died} sorties={}/{} bombs={} bundles={}",
        script.t,
        crate::director::clock_text(script.t),
        r.sent,
        r.back,
        r.lost,
        r.left,
        r.delivered[0],
        r.delivered[1],
        r.delivered[2],
        r.sorties_left,
        r.sorties,
        r.bombs,
        r.bundles
    );
}
