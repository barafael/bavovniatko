//! What every part of the game shares: the dioramas' files (`data`, from `game/dioramas/`), the ground's geometry
//! and layout built from them (`geo`), which diorama is on the table and who attacks (`diorama`), the words in two registers (`text`), and the order every frame's
//! systems run in, which keeps a day deterministic (`schedule`).

pub mod data;
pub mod dev;
pub mod diorama;
pub mod geo;
pub mod schedule;
pub mod text;
pub mod vocab;
