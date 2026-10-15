//! Open questions: the gaps the research base records and has not closed. Every one is a note that something is
//! unknown, unresolved or unverifiable, with the sources already tried. Grouped by dossier or topic survey.

use leptos::prelude::*;
use serde_json::Value;

use crate::router::{Route, href};
use crate::ui::{error_box, loading, rid_link, rows_resource, strs, waiting_for_db};

const QUERY: &str = "SELECT id, key, text, status, topics, eras, anchor.file AS file, anchor.section AS section
    FROM question ORDER BY key";

#[component]
pub fn Questions() -> impl IntoView {
    let data = rows_resource(|| Some(QUERY.to_string()));
    let topic = RwSignal::new(String::new());
    let words = RwSignal::new(String::new());
    let open = RwSignal::new(true);
    view! {
        <article class="page">
            <h1>"Open questions"</h1>
            <p class="lede">"What the research base does not know, recorded as it was found. Each entry names a gap,
               a conflict between sources or a claim that rests on one statement. Closing one means adding cited
               research and re-running the extraction, not editing this list."</p>
            <div class="filters">
                <select on:change=move |ev| topic.set(event_target_value(&ev))>
                    <option value="">"All dossiers and topic surveys"</option>
                    <option value="topic:c">"Dossiers only"</option>
                    <option value="topic:t">"Topic surveys only"</option>
                    {move || data.get().flatten().and_then(|r| r.ok()).map(|rows| {
                        let mut seen: Vec<(String, usize)> = vec![];
                        for r in &rows {
                            for t in strs(&r["topics"]) {
                                match seen.iter_mut().find(|(id, _)| id == &t) {
                                    Some((_, n)) => *n += 1,
                                    None => seen.push((t, 1)),
                                }
                            }
                        }
                        seen.sort_by_key(|(id, n)| std::cmp::Reverse((*n, id.clone())));
                        seen.into_iter().map(|(id, n)| {
                            let label = format!("{} ({n})", id.replace("topic:", ""));
                            view! { <option value=id.clone()>{label}</option> }
                        }).collect_view()
                    })}
                </select>
                <input type="search" placeholder="Filter by words …"
                    prop:value=move || words.get()
                    on:input=move |ev| words.set(event_target_value(&ev)) />
                <label class="check"><input type="checkbox" prop:checked=move || open.get()
                    on:change=move |ev| open.set(event_target_checked(&ev)) />
                    " open only"</label>
            </div>
            {move || match data.get() {
                None => loading(),
                Some(None) => waiting_for_db(),
                Some(Some(Err(e))) => error_box(e),
                Some(Some(Ok(rows))) => {
                    let (t, w) = (topic.get(), words.get().to_lowercase());
                    let mut topics: Vec<String> = rows.iter().flat_map(|r| strs(&r["topics"])).collect();
                    topics.sort();
                    topics.dedup();
                    let shown: Vec<&Value> = rows.iter().filter(|r| {
                        let ts = strs(&r["topics"]);
                        let in_group = t.is_empty()
                            || ts.iter().any(|x| x == &t)
                            || (t == "topic:c" && ts.iter().any(|x| x.starts_with("topic:c")))
                            || (t == "topic:t" && ts.iter().any(|x| x.starts_with("topic:t")));
                        let is_open = !open.get() || r["status"].as_str() == Some("open");
                        in_group && is_open && (w.is_empty() || r["text"].as_str().unwrap_or_default().to_lowercase().contains(&w))
                    }).collect();
                    let total = shown.len();
                    view! {
                        <p class="muted">{format!("{total} of {} questions, over {} topics", rows.len(), topics.len())}</p>
                        <ul class="questions">{shown.into_iter().map(|r| view! { <Question r=r.clone() /> }).collect_view()}</ul>
                    }.into_any()
                }
            }}
        </article>
    }
}

#[component]
fn Question(r: Value) -> impl IntoView {
    let id = r["id"].as_str().unwrap_or_default().to_string();
    let status = r["status"].as_str().unwrap_or("open").to_string();
    let file = r["file"].as_str().map(String::from);
    let section = r["section"].as_str().unwrap_or_default().to_string();
    let key = r["key"].as_str().unwrap_or_default().to_string();
    view! {
        <li class="question">
            <div class="q-head">
                <span class=format!("badge {status}")>{status.clone()}</span>
                {strs(&r["topics"]).into_iter().map(|t| view! { <span class="chip">{rid_link(&t, Some(t.replace("topic:", "")))}</span> }).collect_view()}
                {strs(&r["eras"]).into_iter().map(|e| view! { <span class="chip era">{e.replace("era:", "")}</span> }).collect_view()}
                <span class="muted small">{key.clone()}</span>
            </div>
            <p class="q-text">{text(r["text"].as_str().unwrap_or_default())}</p>
            <div class="muted small">
                {file.map(|f| view! {
                    <a href=format!("https://github.com/barafael/bavovniatko/blob/main/{f}")>
                        "from the research text"</a>" · "{section.clone()}
                })}
                <a href=href(&Route::Notebook(Some(format!("SELECT * FROM ONLY {id};"))))>"open in notebook"</a>
            </div>
        </li>
    }
}

/// A run of question prose, with the `[src:slug]` markers pulled out.
enum Seg<'a> {
    Text(&'a str),
    Src(&'a str),
}

/// Split on `[src:slug]`, how the research texts name a source. The slug is the source record's id, so the marker
/// becomes a link to it.
fn segments(line: &str) -> Vec<Seg<'_>> {
    let mut out = vec![];
    let mut rest = line;
    while let Some(start) = rest.find("[src:") {
        if start > 0 {
            out.push(Seg::Text(&rest[..start]));
        }
        let after = &rest[start + "[src:".len()..];
        match after.find(']') {
            Some(end) => {
                out.push(Seg::Src(&after[..end]));
                rest = &after[end + 1..];
            }
            // An unclosed marker is just text.
            None => break,
        }
    }
    if !rest.is_empty() {
        out.push(Seg::Text(rest));
    }
    out
}

/// Question prose: `**bold**` leads, `- ` bullets and `[src:…]` markers, which is how the research texts write them.
fn text(s: &str) -> AnyView {
    let mut out: Vec<AnyView> = vec![];
    for (i, line) in s.lines().enumerate() {
        let line = line.trim_start();
        let (line, bullet) = match line.strip_prefix("- ") {
            Some(rest) => (rest, true),
            None => (line, false),
        };
        if i > 0 {
            out.push(view! { <br/> }.into_any());
        }
        if bullet {
            out.push(view! { <span class="bullet">"\u{2022} "</span> }.into_any());
        }
        for seg in segments(line) {
            match seg {
                Seg::Src(slug) => {
                    let rid = format!("source:{slug}");
                    out.push(view! { <span class="src">{rid_link(&rid, Some(format!("[{slug}]")))}</span> }.into_any());
                }
                Seg::Text(part) => {
                    for (j, word) in part.split("**").enumerate() {
                        if word.is_empty() {
                            continue;
                        }
                        out.push(if j % 2 == 1 {
                            view! { <strong>{word.to_string()}</strong> }.into_any()
                        } else {
                            word.to_string().into_any()
                        });
                    }
                }
            }
        }
    }
    out.into_any()
}
