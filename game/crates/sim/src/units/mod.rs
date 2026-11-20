//! Vehicles, soldiers, mines, and what happens when steel and explosive meet.
//!
//! The attacking columns behave the same in both dioramas, because the minefield does the same thing to them:
//! single file down a known track behind a plow, the lead stops, the ones behind bunch up, drivers who go round
//! leave the cleared lane, and the wrecks block the track for whoever comes next. At Vuhledar the Russian columns
//! do it (dossier c08 §5); south of Mala Tokmachka the 47th Brigade's do (c09 §5), in better vehicles, and most of
//! their crews get out.

use crate::sound::{Sfx, Sound};
use crate::diorama::diorama;
use crate::fx::{self, Emitter, EmitterKind, FxQueue, FxRng};
use crate::geo::{self, Polyline};
use crate::hits::Victim;
use crate::paint::{Paint, PaintQueue};
use bevy::{light::NotShadowCaster, platform::collections::HashMap, prelude::*};
use rand::{Rng, SeedableRng, rngs::SmallRng};

mod kinds;
mod mines;
mod spawn;
mod drive;
mod wrecks;
mod blasts;
mod soldiers;

pub use kinds::*;
pub use mines::*;
pub use spawn::*;
pub use drive::*;
pub use wrecks::*;
pub use blasts::*;
pub(crate) use soldiers::*;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Blast>()
            .add_message::<DirectHit>()
            .insert_resource(UnitRng(SmallRng::seed_from_u64(crate::dev::seeded(155))))
            .insert_resource(DriveRng(SmallRng::seed_from_u64(crate::dev::seeded(1557))))
            .insert_resource(Stats { vehicle_advance: f32::MIN, max_advance: if geo::layout().start_z.is_some() { f32::MIN } else { 0.0 }, ..default() })
            .init_resource::<Radio>()
            .init_resource::<ColumnOrder>()
            .insert_resource(RouteBook(geo::layout().routes.clone()))
            .init_resource::<LastHit>()
            .add_systems(Startup, (lay_minefields, spawn_old_wrecks).chain().in_set(crate::schedule::Setup::Field))
            .add_systems(
                Update,
                (
                    (sense_traffic, drive_decisions, steer_and_move, track_marks, turret_tracking).chain(),
                    (coast, check_mines, resolve_blasts, resolve_direct_hits, era_reacts, crew_bailout, burn, move_soldiers, board_vehicles, animate_legs, fly_debris, drop_riders).chain(),
                )
                    .chain()
                    .run_if(crate::running)
                    .in_set(crate::Step::Units),
            );
    }
}
