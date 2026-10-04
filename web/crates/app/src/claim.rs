//! A claim's page: the statement, its claimants and sources, what it is about, its numbers, and its place in the
//! claim graph (an ego graph of supports, weakens, contradicts, refines, supersedes, duplicates).

use leptos::prelude::*;
use serde_json::Value;

use crate::router::{Route, go, href};
use crate::ui::{error_box, fmt_time, loading, rid_link, rows_resource, strs, waiting_for_db};

pub const RELATIONS: &[(&str, &str, &str)] = &[
    // (relation, colour, phrase from this claim's point of view: "to" / "from")
    ("supports", "#2e8540", "supports"),
    ("weakens", "#d97706", "weakens"),
    ("contradicts", "#c1121f", "contradicts"),
    ("refines", "#0057b7", "refines"),
    ("supersedes", "#7b2cbf", "supersedes"),
    ("duplicates", "#6b7280", "duplicates"),
];

pub fn relation_color(rel: &str) -> &'static str {
    RELATIONS.iter().find(|r| r.0 == rel).map_or("#6b7280", |r| r.1)
}

fn phrase(rel: &str, dir: &str) -> String {
    match (rel, dir) {
        ("contradicts", _) => "Contradicted by / contradicts".into(),
        (r, "to") => format!("This claim {r}"),
        ("supports", _) => "Supported by".into(),
        ("weakens", _) => "Weakened by".into(),
        ("refines", _) => "Refined by".into(),
        ("supersedes", _) => "Superseded by".into(),
        ("duplicates", _) => "Duplicated by".into(),
        (r, _) => r.to_string(),
    }
}

#[component]
pub fn ClaimPage(id: String) -> impl IntoView {
    let q = format!("fn::claim({id})");
    let data = rows_resource(move || Some(q.clone()));
    move || match data.get() {
        None => loading(),
        Some(None) => waiting_for_db(),
        Some(Some(Err(e))) => error_box(e),
        Some(Some(Ok(rows))) => match rows.into_iter().next() {
            Some(v) if !v["claim"].is_null() => view! { <ClaimView data=v /> }.into_any(),
            _ => error_box("No such claim.".into()),
        },
    }
}

#[component]
fn ClaimView(data: Value) -> impl IntoView {
    let c = &data["claim"];
    let id = c["id"].as_str().unwrap_or_default().to_string();
    let canonical = data["canonical"].as_str().unwrap_or_default().to_string();
    let outcome = c["ext"]["outcome"].as_bool().unwrap_or(false);
    let badges: Vec<(String, String)> = [
        ("kind", c["kind"].as_str().map(String::from)),
        ("epistemic", c["epistemic"].as_str().map(String::from)),
        ("status", c["status"].as_str().map(String::from)),
    ].into_iter().filter_map(|(k, v)| v.map(|v| (k.to_string(), v))).collect();
    let time = fmt_time(&c["time"]);
    let eras = strs(&c["eras"]);
    let topics = strs(&c["topics"]);
    let anchor = c["anchor"].clone();
    let relations = match &data["relations"] { Value::Array(a) => a.clone(), _ => vec![] };
    let sources = match &data["sources"] { Value::Array(a) => a.clone(), _ => vec![] };
    let claimants = match &data["claimants"] { Value::Array(a) => a.clone(), _ => vec![] };
    let about = match &data["about"] { Value::Array(a) => a.clone(), _ => vec![] };
    let observations = match &data["observations"] { Value::Array(a) => a.clone(), _ => vec![] };

    view! {
        <article class="claim-page">
            <div class="claim-head">
                <div class="badges">
                    {badges.into_iter().map(|(k, v)| view! { <span class=format!("badge {k} {v}")>{v.clone()}</span> }).collect_view()}
                    {outcome.then(|| view! { <span class="badge outcome" title="This claim describes the historical outcome of a dossier's episode">"outcome"</span> })}
                    {(!time.is_empty()).then(|| view! { <span class="when">{time.clone()}</span> })}
                </div>
                <p class="claim-text">{c["text"].as_str().unwrap_or_default().to_string()}</p>
                {c["confidence_note"].as_str().map(|n| view! { <p class="note">{n.to_string()}</p> })}
                <div class="meta">
                    <span class="rid-label">{id.clone()}</span>
                    {eras.into_iter().map(|e| view! { <span class="chip">{e.replace("era:", "")}</span> }).collect_view()}
                    {topics.into_iter().map(|t| view! { <span class="chip">{rid_link(&t, Some(t.replace("topic:", "")))}</span> }).collect_view()}
                    {anchor["file"].as_str().map(|f| view! { <span class="from">"from "
                        <a href=format!("https://github.com/barafael/bavovniatko/blob/main/{f}")>"the research text"</a>
                        {format!(", {}", anchor["section"].as_str().unwrap_or_default())}</span> })}
                </div>
                {(canonical != id && !canonical.is_empty()).then(|| view! {
                    <p class="note">"This is an extracted copy. Canonical claim: "{rid_link(&canonical, None)}</p>
                })}
            </div>

            <div class="claim-grid">
                <section class="panel">
                    <h2>"Who claims it"</h2>
                    {if claimants.is_empty() {
                        view! { <p class="muted">"No explicit claimant: the cited sources state it."</p> }.into_any()
                    } else {
                        view! { <ul class="plain">{claimants.into_iter().map(|a| view! {
                            <li>{rid_link(a["actor"].as_str().unwrap_or_default(), a["name"].as_str().map(String::from))}
                                {a["side"].as_str().map(|s| view! { <span class=format!("side {s}")>{s.to_string()}</span> })}
                                {a["role"].as_str().map(|r| view! { <span class="muted">{format!(" · {r}")}</span> })}</li>
                        }).collect_view()}</ul> }.into_any()
                    }}
                    <h2>"About"</h2>
                    <ul class="plain">{about.into_iter().map(|a| {
                        let t = a["target"].as_str().unwrap_or_default().to_string();
                        let name = a["name"].as_str().map(String::from);
                        view! { <li>{rid_link(&t, name)}{a["role"].as_str().map(|r| view! { <span class="muted">{format!(" · {r}")}</span> })}</li> }
                    }).collect_view()}</ul>
                    {(!observations.is_empty()).then(|| view! {
                        <h2>"Numbers"</h2>
                        <table class="obs">
                            <thead><tr><th>"metric"</th><th>"value"</th><th>"unit"</th><th>"side"</th><th>"when"</th></tr></thead>
                            <tbody>{observations.into_iter().map(|o| {
                                let value = o["value_text"].as_str().map(String::from).unwrap_or_else(|| match (&o["value"], &o["low"], &o["high"]) {
                                    (v, _, _) if !v.is_null() => v.to_string(),
                                    (_, l, h) => format!("{l}–{h}"),
                                });
                                let m = o["metric"].as_str().unwrap_or_default().to_string();
                                view! { <tr>
                                    <td>{rid_link(&m, Some(o["metric_label"].as_str().map(String::from)
                                        .unwrap_or_else(|| m.replace("metric:", "").replace('_', " "))))}</td>
                                    <td class="num">{value}</td>
                                    <td>{o["unit"].as_str().unwrap_or_default().replace("unit:", "")}</td>
                                    <td>{o["side"].as_str().unwrap_or_default().to_string()}</td>
                                    <td>{fmt_time(&o["time"])}</td>
                                </tr> }
                            }).collect_view()}</tbody>
                        </table>
                    })}
                </section>
                <section class="panel">
                    <h2>"Sources"</h2>
                    <ol class="sources">{sources.into_iter().map(|s| view! {
                        <li>
                            <a href=s["url"].as_str().unwrap_or("#").to_string() target="_blank" rel="noopener noreferrer">
                                {s["title"].as_str().unwrap_or_default().to_string()}</a>
                            <div class="muted small">
                                {[s["publisher"].as_str(), s["published"].as_str(), s["origin"].as_str()].into_iter().flatten()
                                    .map(String::from).collect::<Vec<_>>().join(" · ")}
                                {s["reliability"].as_str().map(|r| view! { <span class=format!("rel {r}")>{format!(" · reliability {r}")}</span> })}
                            </div>
                            {s["quote"].as_str().map(|q| view! { <blockquote>{format!("“{q}”")}</blockquote> })}
                        </li>
                    }).collect_view()}</ol>
                </section>
            </div>

            <section class="panel claim-relations">
                <h2>{format!("Relations to other claims ({})", relations.len())}</h2>
                {if relations.is_empty() {
                    view! { <p class="muted">"No related claims were found for this one."</p> }.into_any()
                } else {
                    view! { <div class="relations-layout">
                        <div><EgoGraph relations=relations.clone() /></div>
                        <RelationList relations=relations />
                    </div> }.into_any()
                }}
            </section>
        </article>
    }
}

/// The claim in the middle, its related claims around it, coloured by relation.
#[component]
fn EgoGraph(relations: Vec<Value>) -> impl IntoView {
    let n = relations.len().min(40);
    let (w, h, r) = (560.0_f64, 340.0_f64, 140.0_f64);
    let (cx, cy) = (w / 2.0, h / 2.0);
    let nodes: Vec<_> = relations.iter().take(n).enumerate().map(|(i, rel)| {
        let a = (i as f64 / n as f64) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
        let (x, y) = (cx + r * 1.7 * a.cos(), cy + r * a.sin());
        let other = rel["other"].as_str().unwrap_or_default().to_string();
        let kind = rel["rel"].as_str().unwrap_or_default().to_string();
        let title = format!("{} — {}", phrase(&kind, rel["dir"].as_str().unwrap_or_default()), rel["text"].as_str().unwrap_or_default());
        (x, y, other, kind, title, rel["strength"].as_f64().unwrap_or(0.5))
    }).collect();
    view! {
        <svg class="ego" viewBox=format!("0 0 {w} {h}") role="img" aria-label="Claim relation graph">
            {nodes.iter().map(|(x, y, _, kind, _, s)| view! {
                <line x1=cx y1=cy x2=*x y2=*y stroke=relation_color(kind) stroke-width=1.0 + 3.0 * s stroke-opacity="0.7" />
            }).collect_view()}
            <circle cx=cx cy=cy r="16" class="ego-center" />
            {nodes.into_iter().map(|(x, y, other, kind, title, _)| view! {
                <circle class="node" cx=x cy=y r="9" fill=relation_color(&kind)
                    on:click=move |_| go(Route::Claim(other.clone()))><title>{title}</title></circle>
            }).collect_view()}
        </svg>
        <div class="legend">{RELATIONS.iter().map(|(r, c, _)| view! {
            <span><i style=format!("background:{c}")></i>{r.to_string()}</span>
        }).collect_view()}</div>
    }
}

#[component]
fn RelationList(relations: Vec<Value>) -> impl IntoView {
    let mut groups: Vec<(String, Vec<Value>)> = vec![];
    for r in relations {
        let key = phrase(r["rel"].as_str().unwrap_or_default(), r["dir"].as_str().unwrap_or_default());
        match groups.iter_mut().find(|g| g.0 == key) {
            Some(g) => g.1.push(r),
            None => groups.push((key, vec![r])),
        }
    }
    view! {
        <div class="relations">{groups.into_iter().map(|(title, items)| view! {
            <h3>{title}</h3>
            <ul class="rel-list">{items.into_iter().map(|r| {
                let other = r["other"].as_str().unwrap_or_default().to_string();
                let kind = r["rel"].as_str().unwrap_or_default().to_string();
                view! { <li style=format!("border-left-color:{}", relation_color(&kind))>
                    <a href=href(&Route::Claim(other.clone()))>{r["text"].as_str().unwrap_or_default().to_string()}</a>
                    <div class="muted small">
                        {format!("strength {:.1}", r["strength"].as_f64().unwrap_or(0.0))}
                        {(r["independent"].as_bool() == Some(true)).then_some(" · independent")}
                        {r["rationale"].as_str().map(|t| format!(" · {t}"))}
                    </div>
                    {r["resolution"].as_str().map(|t| view! { <div class="resolution">{format!("Explanation: {t}")}</div> })}
                </li> }
            }).collect_view()}</ul>
        }).collect_view()}</div>
    }
}

