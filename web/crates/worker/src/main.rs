//! The knowledge-base Web Worker. It loads the dataset (`data/manifest.json` and the files it lists) into
//! in-memory SurrealDB, seals it read-only, then answers queries. All messages are JSON strings:
//!
//! - app → worker: `{"id": 1, "op": "query", "q": "SELECT …"}`
//! - worker → app: `{"type": "progress", …}`, `{"type": "ready", "manifest": …, "ms": …}`,
//!   `{"type": "failed", "error": …}` and `{"type": "result", "id": 1, "result": QueryResult}`

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use js_sys::Promise;
use kbcore::{Kb, Manifest, Source};
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, Response};

thread_local! {
    static KB: RefCell<Option<Rc<Kb>>> = const { RefCell::new(None) };
}

fn scope() -> DedicatedWorkerGlobalScope {
    js_sys::global().unchecked_into()
}

fn post(v: serde_json::Value) {
    let _ = scope().post_message(&JsValue::from_str(&v.to_string()));
}

/// Fetches dataset files relative to the worker script. Once the manifest is in, every file it lists is
/// requested at once, so downloads overlap with loading.
struct Fetcher {
    base: String,
    pending: RefCell<HashMap<String, Promise>>,
}

impl Fetcher {
    fn start(&self, path: &str) -> Promise {
        scope().fetch_with_str(&format!("{}{}", self.base, path))
    }
}

async fn response_text(p: Promise, path: &str) -> Result<String, String> {
    let resp: Response = JsFuture::from(p).await.map_err(|e| format!("{path}: {e:?}"))?.unchecked_into();
    if !resp.ok() {
        return Err(format!("{path}: HTTP {}", resp.status()));
    }
    let text = JsFuture::from(resp.text().map_err(|e| format!("{e:?}"))?).await.map_err(|e| format!("{e:?}"))?;
    text.as_string().ok_or_else(|| format!("{path}: not text"))
}

impl Source for Fetcher {
    async fn text(&self, path: &str) -> Result<String, String> {
        let p = self.pending.borrow_mut().remove(path).unwrap_or_else(|| self.start(path));
        let text = response_text(p, path).await?;
        if path == "manifest.json" {
            let m: Manifest = serde_json::from_str(&text).map_err(|e| e.to_string())?;
            let mut pending = self.pending.borrow_mut();
            for f in &m.load {
                pending.insert(f.clone(), self.start(f));
            }
        }
        Ok(text)
    }
}

#[derive(Deserialize)]
struct Request {
    id: u64,
    op: String,
    #[serde(default)]
    q: String,
}

async fn boot() {
    let t0 = js_sys::Date::now();
    let fetcher = Fetcher { base: "data/".into(), pending: RefCell::new(HashMap::new()) };
    let result = async {
        let mut kb = Kb::open().await?;
        let manifest = kb.load(&fetcher, |p| post(json!({"type": "progress", "progress": p}))).await?;
        kb.seal().await?;
        Ok::<_, String>((kb, manifest))
    }
    .await;
    match result {
        Ok((kb, manifest)) => {
            KB.with(|k| *k.borrow_mut() = Some(Rc::new(kb)));
            post(json!({"type": "ready", "manifest": manifest, "ms": js_sys::Date::now() - t0}));
        }
        Err(e) => post(json!({"type": "failed", "error": e})),
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(|ev: MessageEvent| {
        let Some(text) = ev.data().as_string() else { return };
        let Ok(req) = serde_json::from_str::<Request>(&text) else { return };
        spawn_local(async move {
            let kb = KB.with(|k| k.borrow().clone());
            let result = match (req.op.as_str(), kb) {
                ("query", Some(kb)) => serde_json::to_value(kb.query(&req.q).await).unwrap(),
                ("query", None) => json!({"ms": 0, "statements": [], "error": "the database is still loading"}),
                (op, _) => json!({"ms": 0, "statements": [], "error": format!("unknown op {op}")}),
            };
            post(json!({"type": "result", "id": req.id, "result": result}));
        });
    });
    scope().set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    spawn_local(boot());
}
