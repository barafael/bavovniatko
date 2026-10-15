//! A claim's page: the statement, its claimants and sources, what it is about, its numbers, and its place in the
//! claim graph (an ego graph of supports, weakens, contradicts, refines, supersedes, duplicates).

use leptos::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use strum::{IntoStaticStr, VariantArray};

use crate::router::{Route, go, href};
use crate::ui::{MUTED, error_box, fmt_time, group_by, key_of, list, on_rows, rid_link, rows_resource, strs, text, value_label};

/// The claim-to-claim edges (one table each in the schema).
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, IntoStaticStr, VariantArray)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Relation {
    Supports,
    Weakens,
    Contradicts,
    Refines,
    Supersedes,
    Duplicates,
}

impl Relation {
    pub fn color(self) -> &'static str {
        match self {
            Relation::Supports => "var(--ok)",
            Relation::Weakens => "var(--warn)",
            Relation::Contradicts => "var(--ru)",
            Relation::Refines => "var(--ua)",
            Relation::Supersedes => "var(--intl)",
            Relation::Duplicates => MUTED,
        }
    }

    /// The relation from this claim's point of view, as a heading.
    fn phrase(self, dir: Dir) -> String {
        let name: &str = self.into();
        match (self, dir) {
            (Relation::Contradicts, _) => "Contradicted by / contradicts".into(),
            (_, Dir::To) => format!("This claim {name}"),
            (Relation::Supports, Dir::From) => "Supported by".into(),
            (Relation::Weakens, Dir::From) => "Weakened by".into(),
            (Relation::Refines, Dir::From) => "Refined by".into(),
            (Relation::Supersedes, Dir::From) => "Superseded by".into(),
            (Relation::Duplicates, Dir::From) => "Duplicated by".into(),
        }
    }
}

/// A relation's colour by name; grey for anything unknown.
pub fn relation_color(rel: &str) -> &'static str {
    Relation::VARIANTS.iter().find(|r| <&str>::from(**r) == rel).map_or(MUTED, |r| r.color())
}

/// Whether the edge leaves this claim (`to` the other) or arrives at it (`from` the other).
#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Dir {
    To,
    From,
}

/// One of `fn::claim(…).relations`.
#[derive(Clone, Deserialize)]
struct Link {
    other: String,
    rel: Relation,
    dir: Dir,
    text: Option<String>,
    strength: Option<f64>,
    independent: Option<bool>,
    rationale: Option<String>,
    resolution: Option<String>,
}

#[component]
pub fn ClaimPage(id: String) -> impl IntoView {
    let q = format!("fn::claim({id})");
    let data = rows_resource(move || Some(q.clone()));
    move || on_rows(data.get(), |rows| match rows.into_iter().next() {
        Some(v) if !v["claim"].is_null() => view! { <ClaimView data=v /> }.into_any(),
        _ => error_box("No such claim.".into()),
    })
}

#[component]
fn ClaimView(data: Value) -> impl IntoView {
    let c = &data["claim"];
    let id = text(&c["id"]);
    let canonical = text(&data["canonical"]);
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
    let relations: Vec<Link> = list(&data["relations"]).into_iter().filter_map(|r| serde_json::from_value(r).ok()).collect();
    let sources = list(&data["sources"]);
    let claimants = list(&data["claimants"]);
    let about = list(&data["about"]);
    let observations = list(&data["observations"]);

    view! {
        <article class="claim-page">
            <div class="claim-head">
                <div class="badges">
                    {badges.into_iter().map(|(k, v)| view! { <span class=format!("badge {k} {v}")>{v.clone()}</span> }).collect_view()}
                    {outcome.then(|| view! { <span class="badge outcome" title="This claim describes the historical outcome of a dossier's episode">"outcome"</span> })}
                    {(!time.is_empty()).then(|| view! { <span class="when">{time.clone()}</span> })}
                </div>
                <p class="claim-text">{text(&c["text"])}</p>
                {c["confidence_note"].as_str().map(|n| view! { <p class="note">{n.to_string()}</p> })}
                <div class="meta">
                    <span class="rid-label">{id.clone()}</span>
                    {eras.into_iter().map(|e| view! { <span class="chip">{key_of(&e).to_string()}</span> }).collect_view()}
                    {topics.into_iter().map(|t| view! { <span class="chip">{rid_link(&t, Some(key_of(&t).to_string()))}</span> }).collect_view()}
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
                        let name = a["name"].as_str().map(String::from);
                        view! { <li>{rid_link(&text(&a["target"]), name)}{a["role"].as_str().map(|r| view! { <span class="muted">{format!(" · {r}")}</span> })}</li> }
                    }).collect_view()}</ul>
                    {(!observations.is_empty()).then(|| view! {
                        <h2>"Numbers"</h2>
                        <table class="obs">
                            <thead><tr><th>"metric"</th><th>"value"</th><th>"unit"</th><th>"side"</th><th>"when"</th></tr></thead>
                            <tbody>{observations.into_iter().map(|o| {
                                let value = value_label(&o);
                                let m = text(&o["metric"]);
                                view! { <tr>
                                    <td>{rid_link(&m, Some(o["metric_label"].as_str().map(String::from)
                                        .unwrap_or_else(|| key_of(&m).replace('_', " "))))}</td>
                                    <td class="num">{value}</td>
                                    <td>{key_of(o["unit"].as_str().unwrap_or_default()).to_string()}</td>
                                    <td>{text(&o["side"])}</td>
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
                                {text(&s["title"])}</a>
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
fn EgoGraph(relations: Vec<Link>) -> impl IntoView {
    let n = relations.len().min(40);
    let (w, h, r) = (560.0_f64, 340.0_f64, 140.0_f64);
    let (cx, cy) = (w / 2.0, h / 2.0);
    let nodes: Vec<_> = relations.into_iter().take(n).enumerate().map(|(i, l)| {
        let a = (i as f64 / n as f64) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
        let title = format!("{} — {}", l.rel.phrase(l.dir), l.text.as_deref().unwrap_or_default());
        (cx + r * 1.7 * a.cos(), cy + r * a.sin(), l, title)
    }).collect();
    view! {
        <svg class="ego" viewBox=format!("0 0 {w} {h}") role="img" aria-label="Claim relation graph">
            {nodes.iter().map(|(x, y, l, _)| view! {
                <line x1=cx y1=cy x2=*x y2=*y stroke=l.rel.color() stroke-width=1.0 + 3.0 * l.strength.unwrap_or(0.5) stroke-opacity="0.7" />
            }).collect_view()}
            <circle cx=cx cy=cy r="16" class="ego-center" />
            {nodes.into_iter().map(|(x, y, l, title)| view! {
                <circle class="node" cx=x cy=y r="9" fill=l.rel.color()
                    on:click=move |_| go(Route::Claim(l.other.clone()))><title>{title}</title></circle>
            }).collect_view()}
        </svg>
        <div class="legend">{Relation::VARIANTS.iter().map(|r| view! {
            <span><i style=format!("background:{}", r.color())></i>{<&str>::from(r)}</span>
        }).collect_view()}</div>
    }
}

#[component]
fn RelationList(relations: Vec<Link>) -> impl IntoView {
    let groups = group_by(relations, |l| l.rel.phrase(l.dir));
    view! {
        <div class="relations">{groups.into_iter().map(|(title, items)| view! {
            <h3>{title}</h3>
            <ul class="rel-list">{items.into_iter().map(|l| view! {
                <li style=format!("border-left-color:{}", l.rel.color())>
                    <a href=href(&Route::Claim(l.other))>{l.text}</a>
                    <div class="muted small">
                        {format!("strength {:.1}", l.strength.unwrap_or(0.0))}
                        {(l.independent == Some(true)).then_some(" · independent")}
                        {l.rationale.map(|t| format!(" · {t}"))}
                    </div>
                    {l.resolution.map(|t| view! { <div class="resolution">{format!("Explanation: {t}")}</div> })}
                </li>
            }).collect_view()}</ul>
        }).collect_view()}</div>
    }
}

