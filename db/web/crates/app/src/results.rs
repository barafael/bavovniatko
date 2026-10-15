//! Result views for a notebook query: one panel per statement, as a table (arrays of records) or as JSON.
//! Record ids in results are links that open the record. Whenever rows are claims (they have a claim `id` or a
//! claim `key`), the notebook fetches their sources too, so every claim on the page shows where it comes from.
//!
//! A big result is rendered a batch of rows at a time, each batch its own `<tbody>`: the table starts with two,
//! and "Show all" appends one per frame without re-rendering what is shown, so the page keeps responding. The
//! browser still lays the whole table out after each batch, which is what bounds a batch's cost.

use std::collections::HashMap;
use std::sync::Arc;

use leptos::prelude::*;
use serde_json::{Map, Value};

use crate::kb::{Kb, QueryResult, StatementResult};
use crate::router::{Route, href};
use crate::ui::{LABEL_FIELDS, claim_key, claim_of_key, label_of, rid_link_with, text};

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
    row.get("key")?.as_str().and_then(claim_of_key)
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

/// Rows per batch, and how many rows a table shows before "Show all".
const BATCH: usize = 100;
const ROWS_SHOWN: usize = 2 * BATCH;
const CELL_CHARS: usize = 400;
const JSON_CHARS: usize = 300_000;

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

#[component]
fn Statement(index: usize, count: usize, statement: StatementResult) -> impl IntoView {
    let StatementResult { result, error } = statement;
    let summary = match &result {
        Value::Array(a) => format!("{} rows", a.len()),
        Value::Null => String::new(),
        _ => "1 value".into(),
    };
    let label = if count > 1 { format!("Statement {}", index + 1) } else { "Result".into() };
    // An array is a table of its items; any other value is a table of one row, and shows as JSON first.
    let array = result.is_array();
    let rows: Arc<[Value]> = match result {
        Value::Array(a) => a.into(),
        v => vec![v].into(),
    };
    let table = array && !rows.is_empty();
    let mode = RwSignal::new(if table { Mode::Table } else { Mode::Json });
    // The JSON text is made once, the first time it is shown.
    let json: StoredValue<Option<Arc<str>>> = StoredValue::new(None);
    let body = {
        let rows = rows.clone();
        move || match mode.get() {
            Mode::Table => view! { <Table rows=rows.clone() /> }.into_any(),
            Mode::Json => {
                let s = json.get_value().unwrap_or_else(|| {
                    let s: Arc<str> = pretty(&rows, array).into();
                    json.set_value(Some(s.clone()));
                    s
                });
                view! { <pre class="json">{s.to_string()}</pre> }.into_any()
            }
        }
    };
    view! {
        <div class="statement">
            <div class="statement-head">
                <span class="label">{label}</span>
                <span class="rows">{summary}</span>
                <span class="modes">
                    <button class:active=move || mode.get() == Mode::Table on:click=move |_| mode.set(Mode::Table)
                        disabled=!table>"Table"</button>
                    <button class:active=move || mode.get() == Mode::Json on:click=move |_| mode.set(Mode::Json)>"JSON"</button>
                </span>
            </div>
            {match error {
                Some(e) => view! { <div class="error">{e}</div> }.into_any(),
                None => body.into_any(),
            }}
        </div>
    }
}

/// The statement's value as indented JSON, cut at JSON_CHARS.
fn pretty(rows: &[Value], array: bool) -> String {
    let json = if array { serde_json::to_string_pretty(rows) } else { serde_json::to_string_pretty(&rows[0]) };
    let mut s = json.unwrap_or_default();
    if s.len() > JSON_CHARS {
        s.truncate(s.char_indices().take_while(|(i, _)| *i < JSON_CHARS).last().map_or(0, |(i, _)| i));
        s.push_str("\n… (truncated)");
    }
    s
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
    rank_columns(&mut cols, rows);
    cols
}

/// Identity, time and text come first, then links to other records (`{id, name}` objects, such as a counter and
/// its measure), then a claim's sources, so they stay in view when a wide table scrolls.
fn rank_columns(cols: &mut [String], rows: &[Value]) {
    let links = |c: &str| rows.iter().take(50).filter_map(|r| r.get(c)).find(|v| !v.is_null())
        .is_some_and(|v| label_of(v).is_some() && v["id"].as_str().is_some_and(|id| id.contains(':')));
    cols.sort_by_cached_key(|c| match c.as_str() {
        "id" => 0,
        "key" => 1,
        "t" | "time" | "month" => 2,
        "text" => 3,
        "sources" => 5,
        c if links(c) => 4,
        _ => 6,
    });
}

/// A cell's value when its row lacks the column.
static NULL: Value = Value::Null;

#[component]
fn Table(rows: Arc<[Value]>) -> impl IntoView {
    let kb = expect_context::<Kb>();
    let total = rows.len();
    let objects = rows.iter().all(Value::is_object);
    let mut cols = if objects { columns(&rows) } else { vec!["value".into()] };
    // The claim each row stands for, found once per row. Claims always show their sources, whatever the query
    // selected.
    let claims: Arc<[Option<String>]> = rows.iter().map(claim_ref).collect();
    let sources = use_context::<ClaimSources>().unwrap_or_default();
    let with_sources = claims.iter().flatten().any(|c| sources.0.contains_key(c));
    if with_sources && !cols.iter().any(|c| c == "sources") {
        cols.push("sources".into());
        if objects {
            rank_columns(&mut cols, &rows);
        }
    }
    let cols: Arc<[String]> = cols.into();
    let shown = RwSignal::new(total.min(ROWS_SHOWN));
    let growing = RwSignal::new(false);
    // Outside the view: a `>` inside a `view!` attribute would end the tag.
    let all_shown = move || shown.get() >= total;
    let head = cols.iter().map(|c| view! { <th>{c.clone()}</th> }).collect_view();
    let row = move |i: usize| {
        let r = &rows[i];
        let claim = claims[i].as_deref();
        let cells = cols.iter().map(|c| {
            let v = if objects { r.get(c).unwrap_or(&NULL) } else { r };
            let content = match (c.as_str(), claim) {
                ("sources", Some(id)) if with_sources => source_links(sources.0.get(id)),
                ("key", Some(id)) => view! { <a class="rid" href=href(&Route::Claim(id.to_string()))>{text(v)}</a> }.into_any(),
                _ => cell_at(&kb, v, 0),
            };
            view! { <td data-label=c.clone()>{content}</td> }
        }).collect_view();
        view! { <tr>{cells}</tr> }
    };
    let batch = move |b: usize| {
        let rows = (b * BATCH..((b + 1) * BATCH).min(total)).map(row.clone()).collect_view();
        view! { <tbody>{rows}</tbody> }
    };
    view! {
        <div class="table-wrap">
            <table>
                <thead><tr>{head}</tr></thead>
                <For each=move || 0..shown.get().div_ceil(BATCH) key=|b| *b children=batch />
            </table>
            {(total > ROWS_SHOWN).then(|| view! {
                <button class="more" hidden=all_shown disabled=move || growing.get()
                    on:click=move |_| { growing.set(true); grow(shown, total); }>
                    {format!("Show all {total} rows")}
                </button>
            })}
        </div>
    }
}

/// Appends a batch of rows per frame until all are shown, so the page keeps responding while thousands render.
fn grow(shown: RwSignal<usize>, total: usize) {
    // The table may be gone by the next frame.
    let Some(n) = shown.try_get_untracked().map(|n| (n + BATCH).min(total)) else { return };
    shown.set(n);
    if n < total {
        request_animation_frame(move || grow(shown, total));
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

/// Nested objects and lists render as links and short field lists down to this depth, then as compact JSON.
const NEST: usize = 2;
const NESTED_CHARS: usize = 2000;
const LIST_ITEMS: usize = 30;

/// Whether a value's JSON stays under `chars`, estimated while walking it and stopping as soon as it does not.
fn fits(v: &Value, chars: usize) -> bool {
    fn walk(v: &Value, left: &mut isize) -> bool {
        *left -= match v {
            Value::Null => 4,
            Value::Bool(_) => 5,
            Value::Number(_) => 8,
            Value::String(s) => s.len() as isize + 2,
            Value::Array(a) => 2 + a.len() as isize,
            Value::Object(o) => 2 + o.keys().map(|k| k.len() as isize + 4).sum::<isize>(),
        };
        match v {
            Value::Array(a) => a.iter().all(|x| walk(x, left)),
            Value::Object(o) => o.values().all(|x| walk(x, left)),
            _ => *left >= 0,
        }
    }
    let mut left = chars as isize;
    walk(v, &mut left) && left >= 0
}

/// A result value as a cell: record ids and claim keys as links, dates and URLs shortened, nested values unfolded.
pub fn cell(v: &Value) -> AnyView {
    cell_at(&expect_context::<Kb>(), v, 0)
}

fn cell_at(kb: &Kb, v: &Value, depth: usize) -> AnyView {
    let nests = |v: &Value| depth <= NEST && fits(v, NESTED_CHARS);
    match v {
        Value::Null => view! { <span class="null">"—"</span> }.into_any(),
        Value::Bool(b) => b.to_string().into_any(),
        Value::Number(n) => view! { <span class="num">{n.to_string()}</span> }.into_any(),
        Value::String(s) if s.starts_with("claim:") && kb.record_table(s).is_some() => view! {
            <a class="rid" href=href(&Route::Claim(s.clone())) title=s.clone()>{claim_key(s)}</a>
        }.into_any(),
        Value::String(s) if kb.record_table(s).is_some() => rid_link_with(kb, s, None),
        // A date without a time of day ("2025-09-01T00:00:00Z") as just the date.
        Value::String(s) if s.len() == 20 && s.ends_with("T00:00:00Z") && s.as_bytes()[..4].iter().all(u8::is_ascii_digit) => {
            view! { <span class="num">{s[..10].to_string()}</span> }.into_any()
        }
        Value::String(s) if s.starts_with("https://") || s.starts_with("http://") => view! {
            <a href=s.clone() target="_blank" rel="noopener noreferrer">{short(s, 80)}</a>
        }.into_any(),
        Value::String(s) => match claim_of_key(s) {
            Some(id) => view! { <a class="rid" href=href(&Route::Claim(id))>{s.clone()}</a> }.into_any(),
            None => marked(&short(s, CELL_CHARS)),
        },
        Value::Array(a) if a.iter().all(|x| !x.is_array() && !x.is_object()) => {
            let more = a.len().saturating_sub(LIST_ITEMS);
            view! {
                <span class="list">
                    {a.iter().take(LIST_ITEMS).map(|x| view! { <span class="item">{cell_at(kb, x, depth + 1)}</span> }).collect_view()}
                    {(more > 0).then(|| view! { <span class="null">{format!("+{more} more")}</span> })}
                </span>
            }.into_any()
        }
        Value::Array(a) if a.len() <= LIST_ITEMS && a.iter().all(|x| !x.is_array()) && nests(v) => view! {
            <span class="list block">{a.iter().map(|x| view! { <span class="item">{cell_at(kb, x, depth + 1)}</span> }).collect_view()}</span>
        }.into_any(),
        Value::Object(o) if o.contains_key("type") && o.contains_key("coordinates") => {
            view! { <span class="geo">{format!("geometry: {}", o["type"].as_str().unwrap_or("?"))}</span> }.into_any()
        }
        Value::Object(o) if o.len() <= 12 && nests(v) => fields(kb, o, depth),
        other => view! { <code class="compact">{short(&other.to_string(), CELL_CHARS)}</code> }.into_any(),
    }
}

/// An object: a link when it has a record `id` (labelled by its name, title or text), then its other fields.
fn fields(kb: &Kb, o: &Map<String, Value>, depth: usize) -> AnyView {
    let id = o.get("id").and_then(|v| v.as_str()).filter(|id| kb.record_table(id).is_some());
    let label = id.and(LABEL_FIELDS.iter().copied().find(|k| o.get(*k).is_some_and(|v| v.is_string())));
    let head = id.map(|id| rid_link_with(kb, id, label.and_then(|k| o[k].as_str()).map(|s| short(s, CELL_CHARS))));
    let rest = o.iter()
        .filter(|(k, v)| !v.is_null() && !(id.is_some() && (*k == "id" || Some(k.as_str()) == label)))
        .map(|(k, v)| view! { <span class="field"><span class="k">{k.clone()}</span>" "{cell_at(kb, v, depth + 1)}</span> })
        .collect_view();
    view! { <span class="obj">{head}{rest}</span> }.into_any()
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
