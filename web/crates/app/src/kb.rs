//! Client for the knowledge-base worker (crates/worker): starts it, tracks loading, and turns its replies into
//! futures. The worker and the pending replies live in a thread-local; the reactive state is a `Copy` handle.

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

use futures::channel::oneshot;
use leptos::prelude::*;
use serde::Deserialize;
use serde_json::{Value, json};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{MessageEvent, Worker};

#[derive(Debug, Clone, Deserialize)]
pub struct StatementResult {
    pub result: Value,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryResult {
    pub ms: f64,
    pub statements: Vec<StatementResult>,
    pub error: Option<String>,
}

impl QueryResult {
    fn failed(e: impl Into<String>) -> Self {
        QueryResult { ms: 0.0, statements: vec![], error: Some(e.into()) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Loading { file: String, done: u64, total: u64, ms: f64 },
    Ready { version: String, ms: f64, rows: u64, attribution: String },
    Failed(String),
}

/// The reactive side of the client, shared through context.
#[derive(Clone, Copy)]
pub struct Kb {
    pub status: RwSignal<Status>,
    /// Table names in the dataset, for recognising record ids in results.
    pub tables: RwSignal<BTreeSet<String>>,
}

struct Inner {
    worker: Worker,
    next_id: u64,
    pending: HashMap<u64, oneshot::Sender<QueryResult>>,
}

thread_local! {
    static INNER: RefCell<Option<Inner>> = const { RefCell::new(None) };
}

impl Kb {
    pub fn start() -> Kb {
        let kb = Kb {
            status: RwSignal::new(Status::Loading { file: "manifest.json".into(), done: 0, total: 1, ms: 0.0 }),
            tables: RwSignal::new(BTreeSet::new()),
        };
        // Trunk builds the worker crate as `kbworker` and generates this loader next to index.html.
        let worker = Worker::new("./kbworker_loader.js").expect("start the database worker");
        let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |ev: MessageEvent| {
            let Some(text) = ev.data().as_string() else { return };
            let Ok(msg) = serde_json::from_str::<Value>(&text) else { return };
            match msg["type"].as_str() {
                Some("progress") => {
                    let p = &msg["progress"];
                    kb.status.set(Status::Loading {
                        file: p["file"].as_str().unwrap_or_default().into(),
                        done: p["done_bytes"].as_u64().unwrap_or(0),
                        total: p["total_bytes"].as_u64().unwrap_or(1).max(1),
                        ms: p["ms"].as_f64().unwrap_or(0.0),
                    });
                }
                Some("ready") => {
                    let m = &msg["manifest"];
                    let counts = m["counts"].as_object().cloned().unwrap_or_default();
                    kb.tables.set(counts.keys().cloned().collect());
                    kb.status.set(Status::Ready {
                        version: m["version"].as_str().unwrap_or_default().into(),
                        ms: msg["ms"].as_f64().unwrap_or(0.0),
                        rows: counts.values().filter_map(|v| v.as_u64()).sum(),
                        attribution: m["attribution"].as_str().unwrap_or_default().into(),
                    });
                }
                Some("failed") => kb.status.set(Status::Failed(msg["error"].as_str().unwrap_or("unknown error").into())),
                Some("result") => {
                    let id = msg["id"].as_u64().unwrap_or(0);
                    let result = serde_json::from_value(msg["result"].clone())
                        .unwrap_or_else(|e| QueryResult::failed(format!("bad reply: {e}")));
                    if let Some(tx) = INNER.with(|i| i.borrow_mut().as_mut().and_then(|i| i.pending.remove(&id))) {
                        let _ = tx.send(result);
                    }
                }
                _ => {}
            }
        });
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();
        INNER.with(|i| *i.borrow_mut() = Some(Inner { worker, next_id: 1, pending: HashMap::new() }));
        kb
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.status.get(), Status::Ready { .. })
    }

    pub async fn query(&self, q: String) -> QueryResult {
        let rx = INNER.with(|i| {
            let mut i = i.borrow_mut();
            let inner = i.as_mut()?;
            let id = inner.next_id;
            inner.next_id += 1;
            let (tx, rx) = oneshot::channel();
            inner.pending.insert(id, tx);
            let msg = json!({"id": id, "op": "query", "q": q}).to_string();
            inner.worker.post_message(&JsValue::from_str(&msg)).ok()?;
            Some(rx)
        });
        match rx {
            Some(rx) => rx.await.unwrap_or_else(|_| QueryResult::failed("the worker stopped")),
            None => QueryResult::failed("the worker is not running"),
        }
    }

    /// Run a query and return its last statement's rows (an error if any statement failed).
    pub async fn rows(&self, q: impl Into<String>) -> Result<Vec<Value>, String> {
        let r = self.query(q.into()).await;
        if let Some(e) = r.error.clone().or_else(|| r.statements.iter().find_map(|s| s.error.clone())) {
            return Err(e);
        }
        Ok(match r.statements.into_iter().last().map(|s| s.result) {
            Some(Value::Array(a)) => a,
            Some(Value::Null) | None => vec![],
            Some(v) => vec![v],
        })
    }

    /// A record id in a result ("claim:c00_0063_04", "source:⟨…⟩") if the string is one.
    pub fn record_table(&self, s: &str) -> Option<String> {
        let (table, key) = s.split_once(':')?;
        (!key.is_empty() && self.tables.with_untracked(|t| t.contains(table))).then(|| table.to_string())
    }
}

/// Fetch a text file relative to the page.
pub async fn fetch_text(url: &str) -> Result<String, String> {
    let window = web_sys::window().ok_or("no window")?;
    let resp: web_sys::Response = JsFuture::from(window.fetch_with_str(url)).await
        .map_err(|e| format!("{e:?}"))?.unchecked_into();
    if !resp.ok() {
        return Err(format!("{url}: HTTP {}", resp.status()));
    }
    let text = JsFuture::from(resp.text().map_err(|e| format!("{e:?}"))?).await.map_err(|e| format!("{e:?}"))?;
    text.as_string().ok_or_else(|| "not text".into())
}
