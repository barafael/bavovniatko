//! The explorer's database core, shared by the browser worker and the native checker (`kbcheck`).
//!
//! The knowledge base runs as SurrealDB 3.3 in memory. [`Kb::open`] starts it with root authentication, then
//! [`Kb::load`] runs the dataset from `db/tools/webexport.py` as root. [`Kb::seal`] then drops to an anonymous
//! guest session for good. Every table only grants `select`, so visitors' writes are no-ops and schema changes
//! are refused (see web/README.md).

use std::collections::BTreeMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::opt::Config;
use surrealdb::opt::auth::Root;
use surrealdb::opt::capabilities::Capabilities;
use surrealdb::types::Value;
use web_time::Instant;

pub const NS: &str = "bavovniatko";
pub const DB: &str = "kb";
/// Statements per request when loading table files (one `INSERT` of up to 500 rows per line).
const LINES_PER_BATCH: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub format: u32,
    pub version: String,
    pub engine: String,
    pub load: Vec<String>,
    pub files: Vec<FileEntry>,
    pub counts: BTreeMap<String, u64>,
    pub attribution: String,
}

impl Manifest {
    pub fn bytes(&self, path: &str) -> u64 {
        self.files.iter().find(|f| f.path == path).map_or(0, |f| f.bytes)
    }
}

/// Loading progress, reported after each file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub file: String,
    pub done_bytes: u64,
    pub total_bytes: u64,
    pub ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementResult {
    pub result: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub ms: f64,
    pub statements: Vec<StatementResult>,
    /// Set when the query as a whole failed (e.g. a parse error).
    pub error: Option<String>,
}

impl QueryResult {
    /// The last statement's result, which is what a notebook cell shows.
    pub fn last(&self) -> Option<&StatementResult> {
        self.statements.last()
    }
    pub fn first_error(&self) -> Option<&str> {
        self.error.as_deref().or_else(|| self.statements.iter().find_map(|s| s.error.as_deref()))
    }
}

/// Where the dataset files come from: `fetch` in the browser, the file system natively.
#[allow(async_fn_in_trait)]
pub trait Source {
    async fn text(&self, path: &str) -> Result<String, String>;
}

pub struct Kb {
    db: Surreal<Db>,
    sealed: bool,
}

fn random_password() -> String {
    let mut b = [0u8; 24];
    getrandom::fill(&mut b).expect("random bytes");
    b.iter().map(|x| format!("{x:02x}")).collect()
}

impl Kb {
    pub async fn open() -> Result<Self, String> {
        let root = Root { username: "root".into(), password: random_password() };
        let caps = Capabilities::default().with_guest_access(true).with_scripting(false);
        let cfg = Config::new().user(root.clone()).capabilities(caps).query_timeout(Duration::from_secs(60));
        let db = Surreal::new::<Mem>(cfg).await.map_err(|e| e.to_string())?;
        db.signin(root).await.map_err(|e| e.to_string())?;
        let kb = Kb { db, sealed: false };
        kb.exec(&format!("DEFINE NAMESPACE {NS}; USE NS {NS}; DEFINE DATABASE {DB} STRICT;")).await?;
        kb.db.use_ns(NS).use_db(DB).await.map_err(|e| e.to_string())?;
        Ok(kb)
    }

    /// Run the manifest's files in order, as root. Calls `progress` after each file.
    pub async fn load<S: Source>(&self, src: &S, mut progress: impl FnMut(Progress)) -> Result<Manifest, String> {
        if self.sealed {
            return Err("the database is sealed".into());
        }
        let t0 = Instant::now();
        let manifest: Manifest = serde_json::from_str(&src.text("manifest.json").await?).map_err(|e| e.to_string())?;
        let total: u64 = manifest.load.iter().map(|p| manifest.bytes(p)).sum();
        let mut done = 0;
        for path in &manifest.load {
            let text = src.text(path).await?;
            if path.starts_with("tables/") {
                let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
                for chunk in lines.chunks(LINES_PER_BATCH) {
                    self.exec(&chunk.join("\n")).await.map_err(|e| format!("{path}: {e}"))?;
                }
            } else if !text.trim().is_empty() {
                self.exec(&text).await.map_err(|e| format!("{path}: {e}"))?;
            }
            done += manifest.bytes(path);
            progress(Progress { file: path.clone(), done_bytes: done, total_bytes: total, ms: t0.elapsed().as_secs_f64() * 1e3 });
        }
        Ok(manifest)
    }

    /// Drop to an anonymous guest session for good: from here on the database is read-only.
    pub async fn seal(&mut self) -> Result<(), String> {
        self.db.invalidate().await.map_err(|e| e.to_string())?;
        self.sealed = true;
        Ok(())
    }

    pub fn is_sealed(&self) -> bool {
        self.sealed
    }

    async fn exec(&self, q: &str) -> Result<(), String> {
        let mut r = self.db.query(q).await.map_err(|e| e.to_string())?;
        match r.take_errors().into_iter().min_by_key(|(i, _)| *i) {
            Some((i, e)) => Err(format!("statement {}: {e}", i + 1)),
            None => Ok(()),
        }
    }

    /// Run a visitor's query (any number of statements). Refuses to run before [`Kb::seal`].
    pub async fn query(&self, q: &str) -> QueryResult {
        let t0 = Instant::now();
        if !self.sealed {
            return QueryResult { ms: 0.0, statements: vec![], error: Some("the database is still loading".into()) };
        }
        let mut r = match self.db.query(q).await {
            Ok(r) => r,
            Err(e) => return QueryResult { ms: t0.elapsed().as_secs_f64() * 1e3, statements: vec![], error: Some(e.to_string()) },
        };
        let n = r.num_statements(); // before take_errors(), which removes the failed statements
        let mut errors = r.take_errors();
        let statements = (0..n)
            .map(|i| match errors.remove(&i) {
                Some(e) => StatementResult { result: serde_json::Value::Null, error: Some(e.to_string()) },
                None => match r.take::<Value>(i) {
                    Ok(v) => StatementResult { result: v.into_json_value(), error: None },
                    Err(e) => StatementResult { result: serde_json::Value::Null, error: Some(e.to_string()) },
                },
            })
            .collect();
        QueryResult { ms: t0.elapsed().as_secs_f64() * 1e3, statements, error: None }
    }
}
