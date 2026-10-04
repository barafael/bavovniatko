//! A record's page, for every table without a view of its own (systems, actors, sources, events, works, kinds,
//! relation edges …): its fields, then what points at it (`fn::record`): what counters it and what it counters,
//! its parts, a kind's members, and the claims about it, asserted by it or citing it.

use std::collections::HashMap;

use leptos::prelude::*;
use serde_json::{Value, json};

use crate::results::cell;
use crate::router::{Route, href};
use crate::ui::{claim_key, error_box, fmt_date, fmt_time, kind_label, loading, rid_link, rows_resource, waiting_for_db};

/// Fields the page shows elsewhere (the title) or that mean nothing to a reader.
const HIDDEN: &[&str] = &["id", "labels", "ext", "geometry", "legacy_ids", "created_at", "updated_at", "published_raw"];
/// The fields a record's title can come from, in order of preference (then its id).
const TITLE_FIELDS: &[&str] = &["labels", "title", "display", "term", "key"];
const CLAIMS_SHOWN: usize = 100;

#[component]
pub fn RecordPage(id: String) -> impl IntoView {
    let q = format!("fn::record({id})");
    let data = rows_resource(move || Some(q.clone()));
    move || match data.get() {
        None => loading(),
        Some(None) => waiting_for_db(),
        Some(Some(Err(e))) => error_box(e),
        Some(Some(Ok(rows))) => match rows.into_iter().next() {
            Some(v) if v["record"].is_object() => view! { <RecordView data=v /> }.into_any(),
            _ => error_box("No such record.".into()),
        },
    }
}

/// What a record is called (its English label, title or term, else its key or id), and the field that said so.
fn title_of(r: &Value) -> (String, Option<&'static str>) {
    let field = |f: &str| if f == "labels" { r["labels"]["en"].as_str() } else { r[f].as_str() };
    match TITLE_FIELDS.iter().find_map(|f| field(f).map(|t| (t.to_string(), Some(*f)))) {
        Some(t) => t,
        None => (r["id"].as_str().unwrap_or_default().to_string(), None),
    }
}

fn is_empty(v: &Value) -> bool {
    match v {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(a) => a.is_empty(),
        Value::Object(o) => o.is_empty(),
        _ => false,
    }
}

/// A field's value: record ids by name, prose in full, anything else (keys, links, objects) as in notebook results.
fn value(v: &Value, names: &HashMap<String, String>) -> AnyView {
    match v {
        Value::String(s) if names.contains_key(s) => rid_link(s, names.get(s).cloned()),
        Value::String(s) if s.contains(' ') => s.clone().into_any(),
        Value::Array(a) if a.iter().all(|x| x.is_string()) => view! {
            <span class="list">{a.iter().map(|x| view! { <span class="item">{value(x, names)}</span> }).collect_view()}</span>
        }.into_any(),
        _ => cell(v),
    }
}

#[component]
fn RecordView(data: Value) -> impl IntoView {
    let r = &data["record"];
    let id = r["id"].as_str().unwrap_or_default().to_string();
    let table = id.split(':').next().unwrap_or_default().to_string();
    let (title, title_field) = title_of(r);
    let kind = r["kind"].as_str().filter(|k| k.starts_with("kind:")).map(String::from);
    let fields: Vec<(String, Value)> = r.as_object().into_iter().flatten()
        .filter(|(k, v)| !HIDDEN.contains(&k.as_str()) && k.as_str() != "kind" && Some(k.as_str()) != title_field && !is_empty(v))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let list = |k: &str| match &data[k] { Value::Array(a) => a.clone(), _ => vec![] };
    let names: HashMap<String, String> = list("names").iter()
        .filter_map(|n| Some((n["id"].as_str()?.to_string(), n["name"].as_str()?.to_string())))
        .collect();
    let notebook = href(&Route::Notebook(Some(format!("fn::record({id});"))));

    view! {
        <article class="claim-page record-page">
            <div class="claim-head">
                <div class="badges">
                    <span class="badge">{table}</span>
                    {kind.map(|k| view! { <span class="badge">{rid_link(&k, Some(kind_label(&json!(k))))}</span> })}
                </div>
                <h1 class="record-title">{title}</h1>
                <div class="meta">
                    <span class="rid-label">{id}</span>
                    <a href=notebook>"open in notebook"</a>
                </div>
            </div>
            <div class="claim-grid">
                {(!fields.is_empty()).then(|| view! {
                    <section class="panel">
                        <h2>"Details"</h2>
                        <dl class="fields">{fields.into_iter().map(|(k, v)| view! {
                            <dt>{k.replace('_', " ")}</dt><dd>{value(&v, &names)}</dd>
                        }).collect_view()}</dl>
                    </section>
                })}
                {counter_panel("What counters it", list("countered_by"))}
                {counter_panel("What it counters", list("counters"))}
                {link_panel("Parts", list("parts"))}
                {link_panel("Members", list("members"))}
                {claims_panel("Claims about it", list("about"))}
                {claims_panel("Claims it asserts", list("asserted"))}
                {claims_panel("Claims citing it", list("cited_by"))}
            </div>
        </article>
    }
}

/// Measure/countermeasure links: the other system, since when, the lag, the effect, why, and the evidence.
fn counter_panel(title: &'static str, rows: Vec<Value>) -> Option<impl IntoView> {
    (!rows.is_empty()).then(|| view! {
        <section class="panel">
            <h2>{title}</h2>
            <ul class="plain counters">{rows.into_iter().map(|c| {
                let sys = &c["system"];
                let meta: Vec<String> = [
                    c["since"].as_str().map(|_| format!("since {}", fmt_date(&c["since"], Some("month")))),
                    c["lag_days"].as_i64().map(|d| format!("lag {d} days")),
                ].into_iter().flatten().collect();
                let evidence: Vec<String> = c["evidence"].as_array().into_iter().flatten()
                    .filter_map(|e| e.as_str().map(String::from)).collect();
                view! {
                    <li>
                        <strong>{rid_link(sys["id"].as_str().unwrap_or_default(), sys["name"].as_str().map(String::from))}</strong>
                        {(!meta.is_empty()).then(|| view! { <span class="muted small">{format!(" · {}", meta.join(" · "))}</span> })}
                        {c["effect"].as_str().map(|e| view! { <p class="small">{e.to_string()}</p> })}
                        {c["rationale"].as_str().map(|r| view! { <p class="muted small">{r.to_string()}</p> })}
                        {(!evidence.is_empty()).then(|| view! {
                            <p class="small">"evidence: "{evidence.into_iter().map(|e| view! {
                                <a class="rid" href=href(&Route::Claim(e.clone()))>{claim_key(&e)}</a>" "
                            }).collect_view()}</p>
                        })}
                    </li>
                }
            }).collect_view()}</ul>
        </section>
    })
}

fn link_panel(title: &'static str, rows: Vec<Value>) -> Option<impl IntoView> {
    (!rows.is_empty()).then(|| view! {
        <section class="panel">
            <h2>{title}<span class="muted small">{format!(" {}", rows.len())}</span></h2>
            <ul class="plain">{rows.into_iter().map(|p| view! {
                <li>{rid_link(p["id"].as_str().unwrap_or_default(), p["name"].as_str().map(String::from))}</li>
            }).collect_view()}</ul>
        </section>
    })
}

/// Claims in time order, each with its role (about, asserted) or the cited source's quote.
fn claims_panel(title: &'static str, rows: Vec<Value>) -> Option<impl IntoView> {
    let total = rows.len();
    let all = RwSignal::new(false);
    let rows = StoredValue::new(rows);
    (total > 0).then(|| view! {
        <section class="panel">
            <h2>{title}<span class="muted small">{format!(" {total}")}</span></h2>
            <ul class="history compact">{move || rows.with_value(|rows| rows.iter().take(if all.get() { total } else { CLAIMS_SHOWN }).map(|c| view! {
                <li>
                    <span class="when">{fmt_time(&json!({"from": c["t"], "precision": c["precision"]}))}</span>
                    <a href=href(&Route::Claim(c["id"].as_str().unwrap_or_default().to_string()))>
                        {c["text"].as_str().unwrap_or_default().to_string()}</a>
                    {c["role"].as_str().map(|r| view! { <span class="muted small">{r.to_string()}</span> })}
                    {c["quote"].as_str().map(|q| view! { <blockquote>{q.to_string()}</blockquote> })}
                </li>
            }).collect_view())}</ul>
            {(total > CLAIMS_SHOWN).then(|| view! {
                <button class="more" on:click=move |_| all.set(true) hidden=move || all.get()>{format!("Show all {total}")}</button>
            })}
        </section>
    })
}
