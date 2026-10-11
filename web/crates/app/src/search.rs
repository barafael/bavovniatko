//! Search: BM25 full-text search over the claim texts, best matches first, with the matching words highlighted and
//! each hit's sources. `#/search?q=…` is a shareable link; the query lives in the URL.

use std::rc::Rc;

use leptos::prelude::*;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::kb::{Kb, Status};
use crate::results::{ClaimSources, fetch_sources};
use crate::router::{Route, href, replace};
use crate::ui::{error_box, fmt_date, loading, waiting_for_db};

/// A signal that follows `source`, but only settles `DELAY` ms after it last changed, so typing doesn't run a query
/// per keystroke.
fn debounced(source: impl Fn() -> String + 'static, delay: i32) -> RwSignal<String> {
    // Shared by the effect (which watches it) and the timer callback (which reads it).
    let source = Rc::new(source);
    let out = RwSignal::new(source());
    let handle = StoredValue::new(None::<i32>);
    let read = source.clone();
    let watch = source.clone();
    let settle = source.clone();
    // Kept for the page's life, because a pending timer may fire after the component is gone.
    // `into_js_value` leaks the closure on purpose: a pending timer may fire after the component is gone.
    let cb: js_sys::Function = Closure::<dyn FnMut()>::new(move || out.set(read())).into_js_value().unchecked_into();
    Effect::new(move |_| {
        let _ = watch();
        let Some(w) = web_sys::window() else { return };
        handle.update_value(|h| {
            if let Some(h) = h.take() {
                w.clear_timeout_with_handle(h);
            }
        });
        if w.set_timeout_with_callback_and_timeout_and_arguments_0(&cb, delay).is_err() {
            out.set(settle());
        }
    });
    out
}

#[component]
pub fn Search(initial: String) -> impl IntoView {
    let kb = expect_context::<Kb>();
    let q = RwSignal::new(initial.clone());
    let terms = debounced(move || q.get(), 220);

    // The URL keeps the query, so every search is shareable.
    Effect::new(move |_| {
        let v = q.get();
        if v != initial {
            replace(&Route::Search(v));
        }
    });

    let results: LocalResource<Option<Result<(Vec<Value>, ClaimSources), String>>> = LocalResource::new(move || {
        let terms = terms.get();
        let ready = matches!(kb.status.get(), Status::Ready { .. });
        async move {
            if terms.trim().len() < 2 || !ready {
                return None;
            }
            let rows = kb.rows(format!("fn::search({})", surql_string(terms.trim()))).await;
            let sources = match &rows {
                Ok(rows) => {
                    let ids: Vec<String> = rows.iter().filter_map(|r| r["id"].as_str().map(String::from)).collect();
                    fetch_sources(kb, ids).await
                }
                Err(_) => ClaimSources::default(),
            };
            Some(rows.map(|rows| (rows, sources)))
        }
    });

    view! {
        <article class="page search-page">
            <h1>"Search"</h1>
            <p class="lede">"Full-text search over every claim, ranked by BM25. Matching words are highlighted, each hit
               links to its claim, and the sources behind it are named."</p>
            <div class="search-bar">
                <input type="search" autofocus placeholder="Search the claims …  glide bomb, kill zone, Avdiivka"
                    prop:value=move || q.get() on:input=move |ev| q.set(event_target_value(&ev)) />
            </div>
            <p class="muted">{move || match results.get() {
                Some(Some(Ok((rows, _)))) => format!("{} matches for “{}”", rows.len(), terms.get().trim()),
                Some(Some(Err(_))) => String::new(),
                _ if terms.get().trim().len() >= 2 => format!("Searching for “{}” …", terms.get().trim()),
                _ => String::new(),
            }}</p>
            {move || match results.get() {
                Some(Some(Err(e))) => error_box(e),
                Some(Some(Ok((rows, sources)))) => {
                    let hits = rows.into_iter().map(|r| view! { <Hit r=r sources=sources.clone() /> }).collect_view();
                    view! { <ul class="hits">{hits}</ul> }.into_any()
                }
                Some(None) if terms.get().trim().len() >= 2 => {
                    if kb.is_ready() { loading() } else { waiting_for_db() }
                }
                _ => view! {
                    <p>"Try one of these:"</p>
                    <ul class="plain examples">
                        {["glide bomb", "kill zone", "Avdiivka", "fibre optic", "Shahed", "countermeasure",
                          "artillery shell", "EW", "Krynky"].into_iter().map(|s| {
                            let set = s.to_string();
                            view! { <li><button class="example" on:click=move |_| q.set(set.clone())>{s.to_string()}</button></li> }
                        }).collect_view()}
                    </ul>
                }.into_any(),
            }}
        </article>
    }
}

#[component]
fn Hit(r: Value, sources: ClaimSources) -> impl IntoView {
    let id = r["id"].as_str().unwrap_or_default().to_string();
    let key = r["key"].as_str().unwrap_or_default().to_string();
    let t = r["t"].as_str().map(String::from);
    let srcs = sources.0.get(&id).cloned().unwrap_or_default();
    view! {
        <li class="hit">
            <div class="hit-head">
                <a class="rid" href=href(&Route::Claim(id.clone()))>{key.clone()}</a>
                {t.map(|d| view! { <span class="muted small">{fmt_date(&Value::String(d), None)}</span> }).into_any()}
                <a class="muted small" href=href(&Route::Graph(Some(id.clone()), 2))>"its graph"</a>
                <a class="muted small" href=href(&Route::Notebook(Some(format!("fn::claim({id});"))))>"notebook"</a>
            </div>
            <p class="hit-text">{marked(r["text"].as_str().unwrap_or_default())}</p>
            {(!srcs.is_empty()).then(|| view! {
                <div class="hit-sources">{srcs.iter().map(|(title, url)| view! {
                    <a href=url.clone() target="_blank" rel="noopener noreferrer" title=url.clone()>{title.clone()}</a>
                }).collect_view()}</div>
            })}
        </li>
    }
}

/// Text with `**highlighted**` spans, as `search::highlight` returns it.
fn marked(s: &str) -> AnyView {
    if !s.contains("**") {
        return s.to_string().into_any();
    }
    s.split("**").enumerate()
        .map(|(i, part)| if i % 2 == 1 { view! { <mark>{part.to_string()}</mark> }.into_any() } else { part.to_string().into_any() })
        .collect_view().into_any()
}

/// A search term as a SurrealQL string literal.
fn surql_string(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}