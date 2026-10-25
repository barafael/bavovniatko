//! The dioramas built into the binary: `game/dioramas/index.ron` read at build time, and every file it names
//! included as text (`$OUT_DIR/dioramas.rs`, included by `diorama.rs`). A new diorama is a new entry there.

use serde::Deserialize;
use std::{fmt::Write, path::Path};

/// An entry of the index: the id, the files merged in order, and the tours' shots.
#[derive(Deserialize)]
struct Entry {
    id: String,
    files: Vec<String>,
    #[serde(default)]
    shots: Option<String>,
}

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dioramas");
    let index = dir.join("index.ron");
    // (The files themselves are `include_str!`'s: rustc tracks them.)
    println!("cargo::rerun-if-changed={}", index.display());
    let text = std::fs::read_to_string(&index).unwrap_or_else(|e| panic!("{}: {e}", index.display()));
    // (Written as the dioramas are, an optional field as its value.)
    let entries: Vec<Entry> = ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME).from_str(&text).unwrap_or_else(|e| panic!("dioramas/index.ron: {e}"));
    assert!(!entries.is_empty(), "dioramas/index.ron lists no diorama");
    let file = |name: &str| {
        let path = dir.join(name);
        assert!(path.is_file(), "dioramas/index.ron names {name}, which is not there");
        format!("({name:?}, include_str!({:?}))", path.display().to_string())
    };
    let mut out = String::from("/// The dioramas of `dioramas/index.ron`, in its order (written by build.rs).\nconst BUILTIN: &[Builtin] = &[\n");
    for e in &entries {
        let files = e.files.iter().map(|f| file(f)).collect::<Vec<_>>().join(", ");
        let shots = e.shots.as_deref().map_or("None".to_string(), |s| format!("Some({})", file(s)));
        writeln!(out, "    Builtin {{ id: {:?}, files: &[{files}], shots: {shots} }},", e.id).unwrap();
    }
    out.push_str("];\n");
    writeln!(out, "/// How many there are.\npub const COUNT: usize = {};", entries.len()).unwrap();
    let dest = Path::new(&std::env::var("OUT_DIR").expect("cargo sets OUT_DIR")).join("dioramas.rs");
    std::fs::write(&dest, out).unwrap_or_else(|e| panic!("{}: {e}", dest.display()));
}
