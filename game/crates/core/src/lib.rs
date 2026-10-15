//! What every part of the game shares: the ground's geometry and the dioramas' layouts (`geo`), which diorama is
//! on the table and who attacks (`diorama`), the words in two registers (`text`), and the order every frame's
//! systems run in, which keeps a day deterministic (`schedule`).

pub mod dev;
pub mod diorama;
pub mod geo;
pub mod schedule;
pub mod text;
pub mod vocab;
