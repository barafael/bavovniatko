//! Client for the knowledge-base worker (crates/worker): starts it, tracks loading, and turns its replies into
//! futures. The worker and the pending replies live in a thread-local; the reactive state is a `Copy` handle.

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

use futures::channel::oneshot;
pub use kbcore::{QueryResult, StatementResult};
use kbcore::{Manifest, Progress, Reply, Request};
use leptos::prelude::*;
use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{MessageEvent, Worker};

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Loading(Progress),
    Ready { manifest: Manifest, ms: f64 },
    Failed(String),
}

/// The reactive side of the client, shared through context.
#[derive(Clone, Copy)]
pub struct Kb {
    pub status: RwSignal<Status>,
    /// Whether the dataset is loaded; unlike `status`, it only changes twice, so readers aren't re-run per file.
    pub ready: Memo<bool>,
    /// Table names in the dataset, for recognising record ids in results.
    pub tables: RwSignal<BTreeSet<String>>,
}

struct Inner {
    worker: Worker,
    next_id: u64,
    pending: HashMap<u64, oneshot::Sender<QueryResult>>,
}

/// Set by db/web/build.sh (git commit and dataset version); "dev" for local builds.
pub const BUILD: &str = match option_env!("BAV_BUILD") {
    Some(b) => b,
    None => "dev",
};

thread_local! {
    static INNER: RefCell<Option<Inner>> = const { RefCell::new(None) };
}

impl Kb {
    pub fn start() -> Kb {
        let status = RwSignal::new(Status::Loading(Progress { file: "manifest.json".into(), done_bytes: 0, total_bytes: 1, ms: 0.0 }));
        let kb = Kb {
            status,
            ready: Memo::new(move |_| status.with(|s| matches!(s, Status::Ready { .. }))),
            tables: RwSignal::new(BTreeSet::new()),
        };
        // Trunk builds the worker crate as `kbworker`; kbworker_loader.js starts it. The build id is passed on as
        // the query string to the worker's script, its wasm and every data file, because those keep their names
        // across deploys and GitHub Pages lets browsers cache them for 10 minutes.
        let worker = Worker::new(&format!("./kbworker_loader.js?v={BUILD}")).expect("start the database worker");
        let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |ev: MessageEvent| {
            let Some(text) = ev.data().as_string() else { return };
            let reply = match serde_json::from_str::<Reply>(&text) {
                Ok(reply) => reply,
                Err(e) => return kb.status.set(Status::Failed(format!("bad reply from the database worker: {e}"))),
            };
            match reply {
                Reply::Progress(p) => kb.status.set(Status::Loading(p)),
                Reply::Ready { manifest, ms } => {
                    kb.tables.set(manifest.counts.keys().cloned().collect());
                    kb.status.set(Status::Ready { manifest, ms });
                }
                Reply::Failed { error } => kb.status.set(Status::Failed(error)),
                Reply::Result { id, result } => {
                    if let Some(tx) = INNER.with(|i| i.borrow_mut().as_mut().and_then(|i| i.pending.remove(&id))) {
                        let _ = tx.send(result);
                    }
                }
            }
        });
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();
        INNER.with(|i| *i.borrow_mut() = Some(Inner { worker, next_id: 1, pending: HashMap::new() }));
        kb
    }

    pub fn is_ready(&self) -> bool {
        self.ready.get()
    }

    pub async fn query(&self, q: String) -> QueryResult {
        let rx = INNER.with(|i| {
            let mut i = i.borrow_mut();
            let inner = i.as_mut()?;
            let id = inner.next_id;
            inner.next_id += 1;
            let (tx, rx) = oneshot::channel();
            inner.pending.insert(id, tx);
            let msg = serde_json::to_string(&Request { id, query: q }).ok()?;
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
        self.query(q.into()).await.into_rows()
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
