//! Result views for a notebook query: one panel per statement, as a table (arrays of records) or as JSON.
//! Record ids in results are links that open the record. Whenever rows are claims (they have a claim `id` or a
//! claim `key`), the notebook fetches their sources too, so every claim on the page shows where it comes from.

use std::collections::HashMap;
use std::sync::Arc;

use leptos::prelude::*;
use serde_json::Value;

use crate::kb::{Kb, QueryResult, StatementResult};
use crate::router::{Route, href};
use crate::ui::rid_link;

/// Sources of the claims in a result: claim id → [(title, url)].
#[derive(Clone, Default)]
pub struct ClaimSources(pub Arc<HashMap<String, Vec<(String, String)>>>);

/// The claim a result row stands for: its `id`, or its claim `key` ("07-0030-17" is claim:c07_0030_17,
/// "c09-0014-02" is claim:cc09_0014_02).
pub fn claim_ref(row: &Value) -> Option<String> {
    let id = match row {
        Value::Object(o) => o.get("id").and_then(|v| v.as_str()),
        Value::String(s) => Some(s.as_str()),
        _ => None,
    };
    if let Some(id) = id.filter(|s| s.starts_with("claim:")) {
        return Some(id.to_string());
    }
    let key = row.get("key")?.as_str()?;
    let well_formed = key.split('-').count() == 3 && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
    well_formed.then(|| format!("claim:c{}", key.replace('-', "_")))
}

/// The claims referred to by any statement's rows (at most 2,000).
pub fn claim_ids(r: &QueryResult) -> Vec<String> {
    let mut ids: Vec<String> = r.statements.iter()
        .filter_map(|s| s.result.as_array())
        .flat_map(|rows| rows.iter().filter_map(claim_ref))
        .collect();
    ids.sort();
    ids.dedup();
    ids.truncate(2000);
    ids
}

/// One query for the sources of all those claims.
pub async fn fetch_sources(kb: Kb, ids: Vec<String>) -> ClaimSources {
    if ids.is_empty() {
        return ClaimSources::default();
    }
    let q = format!("SELECT id, ->cites->source.title AS t, ->cites->source.url AS u FROM [{}]", ids.join(", "));
    let mut map = HashMap::new();
    for row in kb.rows(q).await.unwrap_or_default() {
        let (Some(id), Some(t), Some(u)) = (row["id"].as_str(), row["t"].as_array(), row["u"].as_array()) else { continue };
        let srcs = t.iter().zip(u).map(|(t, u)| (t.as_str().unwrap_or_default().to_string(), u.as_str().unwrap_or_default().to_string())).collect();
        map.insert(id.to_string(), srcs);
    }
    ClaimSources(Arc::new(map))
}

const ROWS_SHOWN: usize = 200;
const CELL_CHARS: usize = 400;

#[component]
pub fn Results(result: QueryResult, sources: ClaimSources) -> impl IntoView {
    provide_context(sources);
    let ms = result.ms;
    let n = result.statements.len();
    view! {
        <div class="results">
            {result.error.map(|e| view! { <div class="error">{e}</div> })}
            {result.statements.into_iter().enumerate().map(|(i, s)| view! {
                <Statement index=i count=n statement=s />
            }).collect_view()}
            <div class="timing">{format!("{n} statement(s) in {ms:.0} ms")}</div>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Table,
    Json,
}

fn is_table(v: &Value) -> bool {
    matches!(v, Value::Array(a) if !a.is_empty())
}

#[component]
fn Statement(index: usize, count: usize, statement: StatementResult) -> impl IntoView {
    let StatementResult { result, error } = statement;
    let mode = RwSignal::new(if is_table(&result) { Mode::Table } else { Mode::Json });
    let rows = match &result {
        Value::Array(a) => format!("{} rows", a.len()),
        Value::Null => String::new(),
        _ => "1 value".into(),
    };
    let label = if count > 1 { format!("Statement {}", index + 1) } else { "Result".into() };
    let result = StoredValue::new(result);
    view! {
        <div class="statement">
            <div class="statement-head">
                <span class="label">{label}</span>
                <span class="rows">{rows}</span>
                <span class="modes">
                    <button class:active=move || mode.get() == Mode::Table on:click=move |_| mode.set(Mode::Table)
                        disabled=move || !result.with_value(is_table)>"Table"</button>
                    <button class:active=move || mode.get() == Mode::Json on:click=move |_| mode.set(Mode::Json)>"JSON"</button>
                </span>
            </div>
            {match error {
                Some(e) => view! { <div class="error">{e}</div> }.into_any(),
                None => (move || match mode.get() {
                    Mode::Table => result.with_value(|v| view! { <Table value=v.clone() /> }).into_any(),
                    Mode::Json => result.with_value(|v| {
                        let mut s = serde_json::to_string_pretty(v).unwrap_or_default();
                        if s.len() > 300_000 {
                            s.truncate(s.char_indices().take_while(|(i, _)| *i < 300_000).last().map_or(0, |(i, _)| i));
                            s.push_str("\n… (truncated)");
                        }
                        view! { <pre class="json">{s}</pre> }
                    }).into_any(),
                }).into_any(),
            }}
        </div>
    }
}

/// Column order: identifying fields first, then the rest as they first appear.
fn columns(rows: &[Value]) -> Vec<String> {
    let mut cols: Vec<String> = vec![];
    for r in rows.iter().take(500) {
        if let Value::Object(o) = r {
            for k in o.keys() {
                if !cols.contains(k) {
                    cols.push(k.clone());
                }
            }
        }
    }
    cols.sort_by_key(|c| column_rank(c));
    cols
}

/// Identity, time and text come first, then a claim's sources, so they stay in view when a wide table scrolls.
fn column_rank(c: &str) -> u8 {
    match c { "id" => 0, "key" => 1, "t" | "time" | "month" => 2, "text" => 3, "sources" => 4, _ => 5 }
}

#[component]
fn Table(value: Value) -> impl IntoView {
    let rows = match value {
        Value::Array(a) => a,
        v => vec![v],
    };
    let all = RwSignal::new(false);
    let total = rows.len();
    let objects = rows.iter().all(|r| r.is_object());
    let mut cols = if objects { columns(&rows) } else { vec!["value".into()] };
    // Claims always show their sources, whatever the query selected.
    let sources = use_context::<ClaimSources>().unwrap_or_default();
    let with_sources = !sources.0.is_empty() && rows.iter().any(|r| claim_ref(r).is_some_and(|c| sources.0.contains_key(&c)));
    if with_sources && !cols.iter().any(|c| c == "sources") {
        cols.push("sources".into());
        if objects {
            cols.sort_by_key(|c| column_rank(c));
        }
    }
    let sources = StoredValue::new(sources);
    let rows = StoredValue::new(rows);
    let cols = StoredValue::new(cols);
    view! {
        <div class="table-wrap">
            <table>
                <thead><tr>{cols.get_value().into_iter().map(|c| view! { <th>{c}</th> }).collect_view()}</tr></thead>
                <tbody>
                    {move || {
                        let limit = if all.get() { total } else { ROWS_SHOWN };
                        rows.with_value(|rows| rows.iter().take(limit).map(|r| view! {
                            <tr>{cols.with_value(|cols| cols.iter().map(|c| {
                                let v = if objects { r.get(c).cloned().unwrap_or(Value::Null) } else { r.clone() };
                                let claim = claim_ref(r);
                                let content = match (c.as_str(), &claim) {
                                    ("sources", Some(id)) if with_sources => sources.with_value(|s| source_links(s.0.get(id))),
                                    ("key", Some(id)) => view! { <a class="rid" href=href(&Route::Claim(id.clone()))>
                                        {v.as_str().unwrap_or_default().to_string()}</a> }.into_any(),
                                    _ => cell(&v),
                                };
                                view! { <td data-label=c.clone()>{content}</td> }
                            }).collect_view())}</tr>
                        }).collect_view())
                    }}
                </tbody>
            </table>
            {(total > ROWS_SHOWN).then(|| view! {
                <button class="more" on:click=move |_| all.set(true) hidden=move || all.get()>
                    {format!("Show all {total} rows")}
                </button>
            })}
        </div>
    }
}

fn short(s: &str, n: usize) -> String {
    if s.chars().count() <= n { s.to_string() } else { s.chars().take(n).collect::<String>() + " …" }
}

/// Text with `**highlighted**` spans (from `search::highlight`).
fn marked(s: &str) -> AnyView {
    if !s.contains("**") {
        return s.to_string().into_any();
    }
    s.split("**").enumerate()
        .map(|(i, part)| if i % 2 == 1 { view! { <mark>{part.to_string()}</mark> }.into_any() } else { part.to_string().into_any() })
        .collect_view().into_any()
}

fn cell(v: &Value) -> AnyView {
    let kb = expect_context::<Kb>();
    match v {
        Value::Null => view! { <span class="null">"—"</span> }.into_any(),
        Value::Bool(b) => b.to_string().into_any(),
        Value::Number(n) => view! { <span class="num">{n.to_string()}</span> }.into_any(),
        Value::String(s) if kb.record_table(s).is_some() => rid_link(s, None),
        Value::String(s) => marked(&short(s, CELL_CHARS)),
        Value::Array(a) if a.iter().all(|x| x.is_string() || x.is_number()) && a.len() <= 30 => view! {
            <span class="list">{a.iter().map(|x| view! { <span class="item">{cell(x)}</span> }).collect_view()}</span>
        }.into_any(),
        Value::Object(o) if o.contains_key("type") && o.contains_key("coordinates") => {
            view! { <span class="geo">{format!("geometry: {}", o["type"].as_str().unwrap_or("?"))}</span> }.into_any()
        }
        other => view! { <code class="compact">{short(&other.to_string(), CELL_CHARS)}</code> }.into_any(),
    }
}

fn source_links(srcs: Option<&Vec<(String, String)>>) -> AnyView {
    match srcs {
        Some(list) if !list.is_empty() => view! {
            <span class="sources-cell">{list.iter().map(|(t, u)| view! {
                <a href=u.clone() target="_blank" rel="noopener noreferrer" title=u.clone()>{short(t, 90)}</a>
            }).collect_view()}</span>
        }.into_any(),
        _ => view! { <span class="null">"—"</span> }.into_any(),
    }
}
