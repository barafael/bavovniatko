//! Result views for a notebook query: one panel per statement, as a table (arrays of records) or as JSON.
//! Record ids in results are links that open the record.

use leptos::prelude::*;
use serde_json::Value;

use crate::kb::{Kb, QueryResult, StatementResult};
use crate::ui::rid_link;

const ROWS_SHOWN: usize = 200;
const CELL_CHARS: usize = 400;

#[component]
pub fn Results(result: QueryResult) -> impl IntoView {
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
    let rank = |c: &str| match c { "id" => 0, "key" => 1, "t" | "time" | "month" => 2, "text" => 3, _ => 4 };
    cols.sort_by_key(|c| rank(c));
    cols
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
    let cols = if objects { columns(&rows) } else { vec!["value".into()] };
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
                                view! { <td>{cell(&v)}</td> }
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
