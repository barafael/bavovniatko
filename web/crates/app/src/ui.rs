//! Small shared pieces of UI: record links, dates, kinds, loading states.

use leptos::prelude::*;
use serde_json::Value;

use crate::kb::Kb;
use crate::router::{href, record_route};

/// A link to a record's view ("claim:…" → claim page, "place:…" → map, …); plain text if not a record id.
pub fn rid_link(rid: &str, label: Option<String>) -> AnyView {
    let kb = expect_context::<Kb>();
    match kb.record_table(rid) {
        Some(table) => {
            let h = href(&record_route(&table, rid));
            let text = label.unwrap_or_else(|| rid.to_string());
            let class = if text == rid { "rid" } else { "rid named" };
            view! { <a class=class href=h title=rid.to_string()>{text}</a> }.into_any()
        }
        None => label.unwrap_or_else(|| rid.to_string()).into_any(),
    }
}

/// "2024-08-06T00:00:00Z" with precision "month" → "Aug 2024".
pub fn fmt_date(v: &Value, precision: Option<&str>) -> String {
    let Some(s) = v.as_str() else { return String::new() };
    let (y, rest) = s.split_at(4.min(s.len()));
    let m: usize = rest.get(1..3).and_then(|m| m.parse().ok()).unwrap_or(0);
    let d: usize = rest.get(4..6).and_then(|d| d.parse().ok()).unwrap_or(0);
    const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let mon = MONTHS.get(m.wrapping_sub(1)).copied().unwrap_or("");
    match precision {
        Some("year") => y.to_string(),
        Some("quarter") => format!("Q{} {y}", (m + 2) / 3),
        Some("month") => format!("{mon} {y}"),
        _ if d > 0 => format!("{d} {mon} {y}"),
        _ => s.to_string(),
    }
}

/// A claim's time object `{from, to, precision}` as text.
pub fn fmt_time(t: &Value) -> String {
    if t.is_null() {
        return String::new();
    }
    let p = t["precision"].as_str();
    let from = fmt_date(&t["from"], p);
    match t.get("to").filter(|v| !v.is_null()) {
        Some(to) if fmt_date(to, p) != from => format!("{from} – {}", fmt_date(to, p)),
        _ => from,
    }
}

/// "kind:['system', 'fpv_drone']" → "fpv drone".
pub fn kind_label(v: &Value) -> String {
    let s = v.as_str().unwrap_or_default();
    let last = s.rsplit(", ").next().unwrap_or(s);
    last.trim_matches(|c| c == '\'' || c == ']' || c == '"').replace('_', " ")
}

pub fn strs(v: &Value) -> Vec<String> {
    match v {
        Value::Array(a) => a.iter().filter_map(|x| x.as_str().map(String::from)).collect(),
        Value::String(s) => vec![s.clone()],
        _ => vec![],
    }
}

pub fn loading() -> AnyView {
    view! { <div class="placeholder">"Loading …"</div> }.into_any()
}

pub fn waiting_for_db() -> AnyView {
    view! { <div class="placeholder">"The knowledge base is loading in your browser (about 10 seconds) …"</div> }.into_any()
}

pub fn error_box(e: String) -> AnyView {
    view! { <div class="error">{e}</div> }.into_any()
}

/// Run a query once the database is ready; re-runs when `query` changes.
pub fn rows_resource(query: impl Fn() -> Option<String> + 'static) -> LocalResource<Option<Result<Vec<Value>, String>>> {
    let kb = expect_context::<Kb>();
    LocalResource::new(move || {
        let ready = kb.is_ready();
        let q = query();
        async move {
            match (ready, q) {
                (true, Some(q)) => Some(kb.rows(q).await),
                _ => None,
            }
        }
    })
}

/// Whether side lists (examples, chapters) start open: on screens at least 64em (1024 px) wide.
pub fn wide_screen() -> bool {
    web_sys::window().and_then(|w| w.inner_width().ok()).and_then(|v| v.as_f64()).is_some_and(|w| w >= 1024.0)
}
