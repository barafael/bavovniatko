//! Small shared pieces of UI: record links, dates, kinds, loading states.

use leptos::prelude::*;
use serde_json::Value;
use strum::{EnumString, IntoStaticStr};

use crate::kb::Kb;
use crate::router::{Route, href, record_route};

/// Colours are the stylesheet's variables, so charts, legends and the map follow the light or dark theme like
/// everything else. This one is for whatever has no side or relation of its own.
pub const MUTED: &str = "var(--muted)";

/// Whose side a claimant or a number is on (`side` in the schema).
#[derive(Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum Side {
    Ua,
    Ru,
    Western,
    International,
    Other,
}

impl Side {
    pub fn parse(v: &Value) -> Option<Side> {
        v.as_str()?.parse().ok()
    }

    pub fn color(self) -> &'static str {
        match self {
            Side::Ua => "var(--ua)",
            Side::Ru => "var(--ru)",
            Side::Western => "var(--ok)",
            Side::International => "var(--intl)",
            Side::Other => MUTED,
        }
    }
}

/// How precisely a time is known (`time.precision` in the schema).
#[derive(Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Precision {
    Day,
    Month,
    Quarter,
    Year,
}

pub const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

/// "2024-08-06T…" → (2024, 8, 6); the month and day are 0 when absent.
pub fn ymd(v: &Value) -> Option<(i32, usize, usize)> {
    let s = v.as_str()?;
    let part = |from: usize| s.get(from..from + 2).and_then(|p| p.parse().ok()).unwrap_or(0);
    Some((s.get(0..4)?.parse().ok()?, part(5), part(8)))
}

/// A record id's key: "topic:c08" → "c08" (the whole string if it has no table).
pub fn key_of(rid: &str) -> &str {
    rid.split_once(':').map_or(rid, |(_, k)| k)
}

/// A string field as an owned string ("" when absent).
pub fn text(v: &Value) -> String {
    v.as_str().unwrap_or_default().to_string()
}

/// An array field's items (none when absent).
pub fn list(v: &Value) -> Vec<Value> {
    v.as_array().cloned().unwrap_or_default()
}

/// Each distinct item and how often it occurs, in order of first appearance.
pub fn tally<K: PartialEq>(items: impl IntoIterator<Item = K>) -> Vec<(K, usize)> {
    let mut out: Vec<(K, usize)> = vec![];
    for k in items {
        match out.iter_mut().find(|x| x.0 == k) {
            Some(x) => x.1 += 1,
            None => out.push((k, 1)),
        }
    }
    out
}

/// The items grouped by `key`: the groups in order of first appearance, the items in their original order.
pub fn group_by<K: PartialEq, V>(items: impl IntoIterator<Item = V>, key: impl Fn(&V) -> K) -> Vec<(K, Vec<V>)> {
    let mut out: Vec<(K, Vec<V>)> = vec![];
    for v in items {
        let k = key(&v);
        match out.iter_mut().find(|g| g.0 == k) {
            Some(g) => g.1.push(v),
            None => out.push((k, vec![v])),
        }
    }
    out
}

/// A link to a claim's page.
pub fn claim_link(id: &Value, label: &Value) -> impl IntoView + use<> {
    view! { <a href=href(&Route::Claim(text(id)))>{text(label)}</a> }
}

/// A link to a record's view ("claim:…" → claim page, "place:…" → map, …); plain text if not a record id.
pub fn rid_link(rid: &str, label: Option<String>) -> AnyView {
    rid_link_with(&expect_context::<Kb>(), rid, label)
}

/// `rid_link` with the database handle in hand, for the thousands of cells of a result table.
pub fn rid_link_with(kb: &Kb, rid: &str, label: Option<String>) -> AnyView {
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

/// The fields that name a record in a query result (`{id, name}`, `{id, title}`, …), in order of preference.
pub const LABEL_FIELDS: [&str; 4] = ["name", "label", "title", "text"];

/// The label of a `{id, name}`-style object.
pub fn label_of(v: &Value) -> Option<&str> {
    let o = v.as_object()?;
    LABEL_FIELDS.iter().find_map(|k| o.get(*k)?.as_str())
}

/// The claim a well-formed claim key stands for: "07-0030-17" is claim:c07_0030_17, "c09-0014-02" is
/// claim:cc09_0014_02, "v01-0003-01" is claim:cv01_0003_01.
pub fn claim_of_key(s: &str) -> Option<String> {
    let parts: Vec<&str> = s.strip_prefix(['c', 'v']).unwrap_or(s).split('-').collect();
    let digits = parts.iter().all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()));
    let shape = matches!(parts.as_slice(), [a, b, c] if a.len() == 2 && b.len() == 4 && (2..=3).contains(&c.len()));
    (digits && shape).then(|| format!("claim:c{}", s.replace('-', "_")))
}

/// "claim:cc15_0022_15" → "c15-0022-15", the claim's key.
pub fn claim_key(id: &str) -> String {
    id.strip_prefix("claim:c").unwrap_or(id).replace('_', "-")
}

/// "2024-08-06T00:00:00Z" with precision month → "Aug 2024".
pub fn fmt_date(v: &Value, precision: Option<Precision>) -> String {
    let Some((y, m, d)) = ymd(v) else { return text(v) };
    let mon = MONTHS.get(m.wrapping_sub(1)).copied().unwrap_or("");
    match precision {
        Some(Precision::Year) => y.to_string(),
        Some(Precision::Quarter) => format!("Q{} {y}", m.div_ceil(3)),
        Some(Precision::Month) => format!("{mon} {y}"),
        _ if d > 0 => format!("{d} {mon} {y}"),
        _ => text(v),
    }
}

/// A span of time as text: its start, and its end when that reads differently ("Aug 2024 – Nov 2024").
pub fn fmt_when(from: &Value, to: Option<&Value>, precision: &Value) -> String {
    let p = precision.as_str().and_then(|p| p.parse().ok());
    let from = fmt_date(from, p);
    match to.filter(|v| !v.is_null()) {
        Some(to) if fmt_date(to, p) != from => format!("{from} – {}", fmt_date(to, p)),
        _ => from,
    }
}

/// A claim's time object `{from, to, precision}` as text.
pub fn fmt_time(t: &Value) -> String {
    if t.is_null() {
        return String::new();
    }
    fmt_when(&t["from"], t.get("to"), &t["precision"])
}

/// An observation's value as text: its `value_text`, else its value, else its low–high range.
pub fn value_label(o: &Value) -> String {
    o["value_text"].as_str().map(String::from).unwrap_or_else(|| match (&o["value"], &o["low"], &o["high"]) {
        (v, _, _) if !v.is_null() => v.to_string(),
        (_, l, h) => format!("{l}–{h}"),
    })
}

/// "kind:['system', 'fpv_drone']" → "fpv drone".
pub fn kind_label(v: &Value) -> String {
    let s = v.as_str().unwrap_or_default();
    let last = s.rsplit(", ").next().unwrap_or(s);
    last.trim_matches(|c| c == '\'' || c == ']' || c == '"').replace('_', " ")
}

pub fn strs(v: &Value) -> Vec<String> {
    match v {
        Value::Array(a) => a.iter().filter_map(|x| x.as_str().or_else(|| label_of(x)).map(String::from)).collect(),
        Value::String(s) => vec![s.clone()],
        _ => vec![],
    }
}

/// The placeholder while a query runs.
pub fn loading() -> AnyView {
    view! { <div class="placeholder">"Loading …"</div> }.into_any()
}

/// The placeholder while the browser database is still loading.
pub fn waiting_for_db() -> AnyView {
    view! { <div class="placeholder">"The knowledge base is loading in your browser (about 10 seconds) …"</div> }.into_any()
}

pub fn error_box(e: String) -> AnyView {
    view! { <div class="error">{e}</div> }.into_any()
}

/// A query's rows: `None` until the query has run, `Some(None)` while the database is still loading.
pub type Rows = Option<Option<Result<Vec<Value>, String>>>;

/// The view of a query's rows once they are in; until then, what is happening.
pub fn on_rows(rows: Rows, f: impl FnOnce(Vec<Value>) -> AnyView) -> AnyView {
    match rows {
        None => loading(),
        Some(None) => waiting_for_db(),
        Some(Some(Err(e))) => error_box(e),
        Some(Some(Ok(rows))) => f(rows),
    }
}

/// The rows, if they are in.
pub fn ok_rows(rows: Rows) -> Option<Vec<Value>> {
    rows.flatten().and_then(Result::ok)
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

/// Whether side lists (examples, chapters) start open: on screens at least 64em wide, the stylesheet's own
/// breakpoint for putting them beside the page.
pub fn wide_screen() -> bool {
    web_sys::window().and_then(|w| w.match_media("(min-width: 64em)").ok().flatten()).is_some_and(|q| q.matches())
}

/// The rows of a query, borrowed, if they are in.
pub fn rows_in(rows: &Rows) -> Option<&[Value]> {
    rows.as_ref()?.as_ref()?.as_deref().ok()
}
