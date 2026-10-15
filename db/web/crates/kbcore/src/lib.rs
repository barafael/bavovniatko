//! The explorer's database core, shared by the browser worker, the native checker (`kbcheck`) and the app.
//!
//! Without features this crate is only the protocol between the app and the worker: the dataset manifest,
//! loading progress, query results, and the messages themselves ([`Request`], [`Reply`]). The `engine` feature
//! adds [`Kb`], the knowledge base itself (see `engine.rs`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(feature = "engine")]
mod engine;
#[cfg(feature = "engine")]
pub use engine::{Error, Kb, Source};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    /// Records in the dataset, over all tables.
    pub fn rows(&self) -> u64 {
        self.counts.values().sum()
    }
}

/// Loading progress, reported after each file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    pub file: String,
    pub done_bytes: u64,
    pub total_bytes: u64,
    pub ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementResult {
    pub result: Value,
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
    pub fn failed(error: impl Into<String>) -> Self {
        QueryResult { ms: 0.0, statements: vec![], error: Some(error.into()) }
    }

    /// The answer to a query sent before the database is ready.
    pub fn loading() -> Self {
        Self::failed("the database is still loading")
    }

    /// The last statement's result, which is what a notebook cell shows.
    pub fn last(&self) -> Option<&StatementResult> {
        self.statements.last()
    }

    pub fn first_error(&self) -> Option<&str> {
        self.error.as_deref().or_else(|| self.statements.iter().find_map(|s| s.error.as_deref()))
    }

    /// The last statement's rows, or the first error.
    pub fn into_rows(self) -> Result<Vec<Value>, String> {
        if let Some(e) = self.first_error() {
            return Err(e.to_string());
        }
        Ok(match self.statements.into_iter().last().map(|s| s.result) {
            Some(Value::Array(a)) => a,
            Some(Value::Null) | None => vec![],
            Some(v) => vec![v],
        })
    }
}

/// App → worker: run a query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: u64,
    pub query: String,
}

/// Worker → app.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Reply {
    Progress(Progress),
    Ready { manifest: Manifest, ms: f64 },
    Failed { error: String },
    Result { id: u64, result: QueryResult },
}
