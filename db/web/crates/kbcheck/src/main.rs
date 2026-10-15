//! Load the web dataset natively, exactly as the browser worker does, then run every gallery query and the
//! read-only probes. Exits non-zero on any failure.
//!
//!     cargo run --release -p kbcheck [-- <data dir>] [--query "<SurrealQL>"]

use std::path::PathBuf;

use kbcore::{Kb, Source};

struct Dir(PathBuf);

impl Source for Dir {
    async fn text(&self, path: &str) -> Result<String, String> {
        tokio::fs::read_to_string(self.0.join(path)).await.map_err(|e| format!("{path}: {e}"))
    }
}

/// Writes a visitor must not be able to make. Each must fail or leave the data unchanged.
const WRITE_PROBES: &[&str] = &[
    "CREATE claim:probe SET text = 'probe text'",
    "UPDATE claim:c00_0063_04 SET text = 'overwritten'",
    "DELETE claim:c00_0063_04",
    "RELATE claim:c00_0063_04->supports->claim:c00_0063_04",
    "DEFINE TABLE probe SCHEMALESS",
    "REMOVE TABLE claim",
    "DEFINE USER probe ON ROOT PASSWORD 'probe' ROLES OWNER",
];

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let query = args.iter().position(|a| a == "--query").map(|i| args[i + 1].clone());
    let dir = args.iter().enumerate()
        .find(|(i, a)| !a.starts_with("--") && (*i == 0 || args[i - 1] != "--query"))
        .map(|(_, a)| PathBuf::from(a))
        .unwrap_or_else(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../app/public/data")));
    let mut kb = Kb::open().await.expect("open");
    let manifest = kb.load(&Dir(dir.clone()), |p| eprintln!("  {:>7.0} ms  {}", p.ms, p.file)).await.expect("load");
    kb.seal().await.expect("seal");
    eprintln!("dataset {} loaded", manifest.version);

    if let Some(q) = query {
        let r = kb.query(&q).await;
        println!("{}", serde_json::to_string_pretty(&r).unwrap());
        return;
    }

    let mut failures = 0;
    for (table, n) in &manifest.counts {
        let r = kb.query(&format!("RETURN count(SELECT id FROM {table})")).await;
        let got = r.last().map(|s| s.result.clone()).unwrap_or_default();
        if got != serde_json::json!(n) {
            eprintln!("COUNT {table}: {got} != {n}");
            failures += 1;
        }
    }

    let gallery: Vec<serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(dir.join("gallery.json")).unwrap()).unwrap();
    for g in &gallery {
        let r = kb.query(g["query"].as_str().unwrap()).await;
        let rows = r.last().map(|s| match &s.result { serde_json::Value::Array(a) => a.len(), _ => 1 }).unwrap_or(0);
        match r.first_error() {
            Some(e) => {
                failures += 1;
                eprintln!("FAIL {:>7.1} ms  {}: {}", r.ms, g["title"].as_str().unwrap(), e);
            }
            None => eprintln!("ok   {:>7.1} ms  {:>5} rows  {}", r.ms, rows, g["title"].as_str().unwrap()),
        }
        if rows == 0 && r.first_error().is_none() {
            eprintln!("     (empty result)");
        }
    }

    let before = kb.query("RETURN [count(SELECT id FROM claim), (SELECT VALUE text FROM ONLY claim:c00_0063_04)]").await;
    for p in WRITE_PROBES {
        let r = kb.query(p).await;
        eprintln!("probe {:58} -> {}", p, r.first_error().map(|e| e.chars().take(60).collect::<String>()).unwrap_or_else(|| "no-op".into()));
    }
    let after = kb.query("RETURN [count(SELECT id FROM claim), (SELECT VALUE text FROM ONLY claim:c00_0063_04)]").await;
    if before.last().map(|s| &s.result) != after.last().map(|s| &s.result) {
        eprintln!("WRITE PROBES CHANGED THE DATA");
        failures += 1;
    }
    eprintln!("{} gallery queries, {failures} failures", gallery.len());
    std::process::exit(if failures > 0 { 1 } else { 0 });
}
