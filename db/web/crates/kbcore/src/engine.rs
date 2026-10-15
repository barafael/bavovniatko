//! The knowledge base runs as SurrealDB 3.3 in memory. [`Kb::open`] starts it with root authentication, then
//! [`Kb::load`] runs the dataset from `db/tools/webexport.py` as root. [`Kb::seal`] then drops to an anonymous
//! guest session for good. Every table only grants `select`, so visitors' writes are no-ops and schema changes
//! are refused (see web/README.md).

use std::time::Duration;

use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::opt::Config;
use surrealdb::opt::auth::Root;
use surrealdb::opt::capabilities::Capabilities;
use surrealdb::types::Value;
use web_time::Instant;

use crate::{Manifest, Progress, QueryResult, StatementResult};

const NS: &str = "bavovniatko";
const DB: &str = "kb";
/// Statements per request when loading table files (one `INSERT` of up to 500 rows per line).
const LINES_PER_BATCH: usize = 10;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] surrealdb::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// A dataset file could not be read (the message names it).
    #[error("{0}")]
    Source(String),
    #[error("statement {number}: {source}")]
    Statement { number: usize, source: surrealdb::Error },
    #[error("{path}: {source}")]
    File { path: String, source: Box<Error> },
    #[error("the database is sealed")]
    Sealed,
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

fn ms_since(t0: Instant) -> f64 {
    t0.elapsed().as_secs_f64() * 1e3
}

impl Kb {
    pub async fn open() -> Result<Self, Error> {
        let root = Root { username: "root".into(), password: random_password() };
        let caps = Capabilities::default().with_guest_access(true).with_scripting(false);
        let cfg = Config::new().user(root.clone()).capabilities(caps).query_timeout(Duration::from_secs(60));
        let db = Surreal::new::<Mem>(cfg).await?;
        db.signin(root).await?;
        let kb = Kb { db, sealed: false };
        kb.exec(&format!("DEFINE NAMESPACE {NS}; USE NS {NS}; DEFINE DATABASE {DB} STRICT;")).await?;
        kb.db.use_ns(NS).use_db(DB).await?;
        Ok(kb)
    }

    /// Run the manifest's files in order, as root. Calls `progress` after each file.
    pub async fn load<S: Source>(&self, src: &S, mut progress: impl FnMut(Progress)) -> Result<Manifest, Error> {
        if self.sealed {
            return Err(Error::Sealed);
        }
        let t0 = Instant::now();
        let manifest: Manifest = serde_json::from_str(&src.text("manifest.json").await.map_err(Error::Source)?)?;
        let total: u64 = manifest.load.iter().map(|p| manifest.bytes(p)).sum();
        let mut done = 0;
        for path in &manifest.load {
            let text = src.text(path).await.map_err(Error::Source)?;
            let in_file = |e| Error::File { path: path.clone(), source: Box::new(e) };
            if path.starts_with("tables/") {
                let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
                for chunk in lines.chunks(LINES_PER_BATCH) {
                    self.exec(&chunk.join("\n")).await.map_err(in_file)?;
                }
            } else if !text.trim().is_empty() {
                self.exec(&text).await.map_err(in_file)?;
            }
            done += manifest.bytes(path);
            progress(Progress { file: path.clone(), done_bytes: done, total_bytes: total, ms: ms_since(t0) });
        }
        Ok(manifest)
    }

    /// Drop to an anonymous guest session for good: from here on the database is read-only.
    pub async fn seal(&mut self) -> Result<(), Error> {
        self.db.invalidate().await?;
        self.sealed = true;
        Ok(())
    }

    async fn exec(&self, q: &str) -> Result<(), Error> {
        let mut r = self.db.query(q).await?;
        match r.take_errors().into_iter().min_by_key(|(i, _)| *i) {
            Some((i, source)) => Err(Error::Statement { number: i + 1, source }),
            None => Ok(()),
        }
    }

    /// Run a visitor's query (any number of statements). Refuses to run before [`Kb::seal`].
    pub async fn query(&self, q: &str) -> QueryResult {
        if !self.sealed {
            return QueryResult::loading();
        }
        let t0 = Instant::now();
        let mut r = match self.db.query(q).await {
            Ok(r) => r,
            Err(e) => return QueryResult { ms: ms_since(t0), ..QueryResult::failed(e.to_string()) },
        };
        let n = r.num_statements(); // before take_errors(), which removes the failed statements
        let mut errors = r.take_errors();
        let statements = (0..n)
            .map(|i| {
                let result = match errors.remove(&i) {
                    Some(e) => Err(e),
                    None => r.take::<Value>(i),
                };
                match result {
                    Ok(v) => StatementResult { result: v.into_json_value(), error: None },
                    Err(e) => StatementResult { result: serde_json::Value::Null, error: Some(e.to_string()) },
                }
            })
            .collect();
        QueryResult { ms: ms_since(t0), statements, error: None }
    }
}
