//! The query notebook: a SurrealQL editor, the example gallery and the result views. The query lives in the URL
//! hash (`#q=…`), so every query can be shared as a link.

use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::bridge::Editor;
use crate::kb::{Kb, QueryResult, fetch_text};
use crate::results::Results;
use crate::router::{self, Route};

#[derive(Debug, Clone, Deserialize)]
struct GalleryItem {
    section: String,
    title: String,
    query: String,
}

const DEFAULT_QUERY: &str = "-- Every claim about Robotyne, in time order. Ctrl+Enter runs the query.\nfn::place_history(place:robotyne);";

fn set_hash_query(q: &str) {
    router::replace(&Route::Notebook(Some(q.to_string())));
}

#[component]
pub fn Notebook(initial: Option<String>) -> impl IntoView {
    let kb = expect_context::<Kb>();
    let from_hash = initial;
    let autorun = RwSignal::new(from_hash.is_some());
    let code = RwSignal::new(from_hash.unwrap_or_else(|| DEFAULT_QUERY.to_string()));
    let result: RwSignal<Option<QueryResult>> = RwSignal::new(None);
    let running = RwSignal::new(false);

    let run = move || {
        let q = code.get_untracked();
        if q.trim().is_empty() || running.get_untracked() || !kb.is_ready() {
            return;
        }
        set_hash_query(&q);
        running.set(true);
        spawn_local(async move {
            let r = kb.query(q).await;
            result.set(Some(r));
            running.set(false);
        });
    };
    // A query from a shared link runs as soon as the database is ready.
    Effect::new(move |_| {
        if kb.is_ready() && autorun.get_untracked() {
            autorun.set(false);
            run();
        }
    });

    // The CodeMirror editor, created once its host element exists; `code` stays the source of truth.
    let host = NodeRef::<leptos::html::Div>::new();
    let editor = StoredValue::new_local(None::<Editor>);
    Effect::new(move |_| {
        let Some(el) = host.get() else { return };
        if editor.with_value(|e| e.is_some()) {
            return;
        }
        let on_change = Closure::<dyn FnMut(String)>::new(move |s: String| code.set(s));
        let on_run = Closure::<dyn FnMut()>::new(move || run());
        let ed = Editor::new(&el, &code.get_untracked(), on_change.as_ref(), on_run.as_ref());
        on_change.forget();
        on_run.forget();
        editor.set_value(Some(ed));
    });
    Effect::new(move |_| {
        let c = code.get();
        editor.with_value(|e| if let Some(e) = e { e.set_value(&c) });
    });
    on_cleanup(move || editor.with_value(|e| if let Some(e) = e { e.destroy() }));

    let gallery = LocalResource::new(|| async {
        fetch_text("data/gallery.json").await.ok()
            .and_then(|t| serde_json::from_str::<Vec<GalleryItem>>(&t).ok())
            .unwrap_or_default()
    });

    view! {
        <div class="notebook">
            <details class="gallery side-list" open=crate::ui::wide_screen()>
                <summary>"Examples"</summary>
                {move || gallery.get().map(|items| {
                    let mut sections: Vec<(String, Vec<GalleryItem>)> = vec![];
                    for it in items {
                        match sections.last_mut() {
                            Some((s, v)) if *s == it.section => v.push(it),
                            _ => sections.push((it.section.clone(), vec![it])),
                        }
                    }
                    sections.into_iter().map(|(s, items)| view! {
                        <section>
                            <h3>{s}</h3>
                            <ul>
                                {items.into_iter().map(|it| {
                                    let q = it.query.clone();
                                    view! { <li><button class="example" title=it.query.clone()
                                        on:click=move |_| { code.set(q.clone()); run(); }>{it.title}</button></li> }
                                }).collect_view()}
                            </ul>
                        </section>
                    }).collect_view()
                })}
            </details>
            <section class="work">
                <div class="editor">
                    <div class="cm-host" node_ref=host></div>
                    <div class="toolbar">
                        <button class="run" on:click=move |_| run()
                            disabled=move || running.get() || !kb.is_ready()>
                            {move || if running.get() { "Running …" } else if kb.is_ready() { "Run  (Ctrl+Enter)" } else { "Loading …" }}
                        </button>
                        <button on:click=move |_| {
                            set_hash_query(&code.get_untracked());
                            if let Some(w) = web_sys::window() {
                                if let Ok(href) = w.location().href() {
                                    let _ = w.navigator().clipboard().write_text(&href);
                                }
                            }
                        }>"Copy link"</button>
                        <span class="hint">"SurrealQL · read-only · "<code>"fn::"</code>" helpers in the examples"</span>
                    </div>
                </div>
                {move || result.get().map(|r| view! { <Results result=r /> })}
            </section>
        </div>
    }
}
