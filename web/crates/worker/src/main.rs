//! The knowledge-base Web Worker. It loads the dataset (`data/manifest.json` and the files it lists) into
//! in-memory SurrealDB, seals it read-only, then answers queries. All messages are JSON strings:
//!
//! a [`Request`] in, a [`Reply`] out (see kbcore).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use js_sys::Promise;
use kbcore::{Kb, Manifest, QueryResult, Reply, Request, Source};
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

fn post(reply: &Reply) {
    let json = serde_json::to_string(reply).expect("replies serialise");
    let _ = scope().post_message(&JsValue::from_str(&json));
}

/// Fetches dataset files relative to the worker script. Once the manifest is in, every file it lists is
/// requested at once, so downloads overlap with loading.
struct Fetcher {
    base: String,
    /// The worker's own query string (`?v=<build>`), appended to every data URL as a cache key.
    query: String,
    pending: RefCell<HashMap<String, Promise>>,
}

impl Fetcher {
    fn start(&self, path: &str) -> Promise {
        scope().fetch_with_str(&format!("{}{}{}", self.base, path, self.query))
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

async fn boot() {
    let t0 = js_sys::Date::now();
    let query = scope().location().search();
    let fetcher = Fetcher { base: "data/".into(), query, pending: RefCell::new(HashMap::new()) };
    let result = async {
        let mut kb = Kb::open().await?;
        let manifest = kb.load(&fetcher, |p| post(&Reply::Progress(p))).await?;
        kb.seal().await?;
        Ok::<_, kbcore::Error>((kb, manifest))
    }
    .await;
    match result {
        Ok((kb, manifest)) => {
            KB.with(|k| *k.borrow_mut() = Some(Rc::new(kb)));
            post(&Reply::Ready { manifest, ms: js_sys::Date::now() - t0 });
        }
        Err(e) => post(&Reply::Failed { error: e.to_string() }),
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(|ev: MessageEvent| {
        let Some(text) = ev.data().as_string() else { return };
        let Ok(req) = serde_json::from_str::<Request>(&text) else { return };
        spawn_local(async move {
            let result = match KB.with(|k| k.borrow().clone()) {
                Some(kb) => kb.query(&req.query).await,
                None => QueryResult::loading(),
            };
            post(&Reply::Result { id: req.id, result });
        });
    });
    scope().set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    spawn_local(boot());
}
