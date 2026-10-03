//! The dispute board: every contradiction between claims, strongest first, with who claims what and how the
//! disagreement is explained. Filter by chapter or topic, or by words.

use leptos::prelude::*;
use serde_json::Value;

use crate::router::{Route, href};
use crate::ui::{error_box, loading, rid_link, rows_resource, strs, waiting_for_db};

/// Each contradiction with both ends mapped to their canonical claim (extracted copies are collapsed in Rust).
const QUERY: &str = "SELECT strength, state, rationale, resolution,
    fn::canonical(in) AS a, fn::canonical(in).text AS a_text, fn::canonical(in)->asserted_by->actor.labels.en AS a_by,
    array::union(in.topics, fn::canonical(in).topics) AS a_topics,
    fn::canonical(out) AS b, fn::canonical(out).text AS b_text, fn::canonical(out)->asserted_by->actor.labels.en AS b_by,
    array::union(out.topics, fn::canonical(out).topics) AS b_topics
    FROM contradicts ORDER BY strength DESC";

/// One entry per canonical pair (either order), keeping the strongest and counting the copies.
fn collapse(rows: Vec<Value>) -> Vec<(Value, usize)> {
    let mut out: Vec<(Value, usize)> = vec![];
    for r in rows {
        let (a, b) = (r["a"].as_str().unwrap_or_default().to_string(), r["b"].as_str().unwrap_or_default().to_string());
        if a == b {
            continue;
        }
        let key = if a < b { (a, b) } else { (b, a) };
        match out.iter_mut().find(|(x, _)| {
            let (xa, xb) = (x["a"].as_str().unwrap_or_default(), x["b"].as_str().unwrap_or_default());
            (xa, xb) == (key.0.as_str(), key.1.as_str()) || (xb, xa) == (key.0.as_str(), key.1.as_str())
        }) {
            Some((_, n)) => *n += 1,
            None => out.push((r, 1)),
        }
    }
    out
}

#[component]
pub fn Disputes() -> impl IntoView {
    let data = rows_resource(|| Some(QUERY.to_string()));
    let topic = RwSignal::new(String::new());
    let words = RwSignal::new(String::new());
    view! {
        <article class="page">
            <h1>"Disputes"</h1>
            <p class="lede">"Where claims cannot both be true as stated: the same quantity, scope and time with incompatible
               values. Many come with an explanation (different counting methods, periods or claimants)."</p>
            <div class="filters">
                <select on:change=move |ev| topic.set(event_target_value(&ev))>
                    <option value="">"All topics and dossiers"</option>
                    {move || data.get().flatten().and_then(|r| r.ok()).map(|rows| {
                        let mut ts: Vec<String> = rows.iter().flat_map(|r| [strs(&r["a_topics"]), strs(&r["b_topics"])].concat()).collect();
                        ts.sort();
                        ts.dedup();
                        ts.into_iter().map(|t| view! { <option value=t.clone()>{t.replace("topic:", "")}</option> }).collect_view()
                    })}
                </select>
                <input type="search" placeholder="Filter by words …" on:input=move |ev| words.set(event_target_value(&ev)) />
            </div>
            {move || match data.get() {
                None => loading(),
                Some(None) => waiting_for_db(),
                Some(Some(Err(e))) => error_box(e),
                Some(Some(Ok(rows))) => {
                    let t = topic.get();
                    let w = words.get().to_lowercase();
                    let shown: Vec<(Value, usize)> = collapse(rows).into_iter().filter(|(r, _)| {
                        (t.is_empty() || strs(&r["a_topics"]).contains(&t) || strs(&r["b_topics"]).contains(&t))
                            && (w.is_empty() || r.to_string().to_lowercase().contains(&w))
                    }).collect();
                    let n = shown.len();
                    view! {
                        <p class="muted">{format!("{n} distinct contradictions (copies of the same claims collapsed)")}</p>
                        <div class="disputes">{shown.into_iter().take(300).map(|(r, copies)| view! { <Dispute r=r copies=copies /> }).collect_view()}</div>
                    }.into_any()
                }
            }}
        </article>
    }
}

#[component]
fn Dispute(r: Value, copies: usize) -> impl IntoView {
    let side = |text: &str, id: &str, by: &Value| {
        let by = strs(by);
        view! {
            <div class="side">
                <a href=href(&Route::Claim(id.to_string()))>{text.to_string()}</a>
                <div class="muted small">{if by.is_empty() { "stated by its sources".to_string() } else { by.join(", ") }}</div>
            </div>
        }
    };
    view! {
        <div class="dispute">
            <div class="dispute-head">
                <span class="strength" title="strength of the contradiction">{format!("{:.1}", r["strength"].as_f64().unwrap_or(0.0))}</span>
                {r["state"].as_str().map(|s| view! { <span class=format!("badge {s}")>{s.to_string()}</span> })}
                {strs(&r["a_topics"]).into_iter().map(|t| view! { <span class="chip">{rid_link(&t, Some(t.replace("topic:", "")))}</span> }).collect_view()}
                {(copies > 1).then(|| view! { <span class="muted small">{format!("recorded {copies} times")}</span> })}
            </div>
            <div class="sides">
                {side(r["a_text"].as_str().unwrap_or_default(), r["a"].as_str().unwrap_or_default(), &r["a_by"])}
                <div class="versus">"vs"</div>
                {side(r["b_text"].as_str().unwrap_or_default(), r["b"].as_str().unwrap_or_default(), &r["b_by"])}
            </div>
            {r["resolution"].as_str().map(|t| view! { <div class="resolution">{format!("Explanation: {t}")}</div> })}
            {r["rationale"].as_str().map(|t| view! { <div class="muted small">{t.to_string()}</div> })}
        </div>
    }
}
