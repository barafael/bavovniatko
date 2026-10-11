//! The front page: what the knowledge base holds, and the shortest routes into it. Counts come from `fn::stats()`.

use leptos::prelude::*;

use crate::router::{Route, go, href};
use crate::ui::{error_box, loading, rid_link, rows_resource, waiting_for_db};

const ERAS: &str = "SELECT id, code, labels.en AS title, from, to, summary,
    (SELECT VALUE count() FROM claim WHERE eras CONTAINS $parent.id) AS claims
    FROM era ORDER BY from";

const TOPICS: &str = "SELECT id, code, labels.en AS title, file,
    (SELECT VALUE count() FROM claim WHERE topics CONTAINS $parent.id) AS claims
    FROM topic WHERE file != NONE ORDER BY code LIMIT 9";

const DISPUTED: &str = "SELECT strength, in.id AS a, in.text AS a_text, out.id AS b, out.text AS b_text
    FROM contradicts ORDER BY strength DESC LIMIT 6";

/// One number on the front page, with what it counts. Clicking it opens a notebook query that shows the breakdown.
#[component]
fn Figure(label: &'static str, value: String, note: String, route: Route) -> impl IntoView {
    view! {
        <a class="figure" href=href(&route)>
            <span class="n">{value.clone()}</span>
            <span class="l">{label.to_string()}</span>
            <span class="note">{note.clone()}</span>
        </a>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let stats = rows_resource(|| Some("fn::stats();".to_string()));
    let eras = rows_resource(|| Some(ERAS.to_string()));
    let topics = rows_resource(|| Some(TOPICS.to_string()));
    let disputes = rows_resource(|| Some(DISPUTED.to_string()));
    view! {
        <article class="page home">
            <header class="home-head">
                <h1>"bavovniatko"</h1>
                <p class="lede">"A cited research knowledge base on the Russo-Ukrainian war, from the full-scale invasion of
                   February 2022 to the present. Every statement is an atomic claim with its sources, its claimants and
                   its numbers. The whole thing runs in your browser and never phones home."</p>
                <div class="home-search">
                    <input type="search" placeholder="Search the claims …"
                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                            let t = event_target_value(&ev);
                            if ev.key() == "Enter" && t.trim().len() > 1 {
                                go(Route::Search(t));
                            }
                        } />
                    <a class="go" href=href(&Route::Search(String::new()))>"search"</a>
                </div>
            </header>

            {move || match stats.get() {
                None => loading(),
                Some(None) => waiting_for_db(),
                Some(Some(Err(e))) => error_box(e),
                Some(Some(Ok(rows))) => match rows.into_iter().next() {
                    None => view! { <p class="placeholder">"The knowledge base is still loading."</p> }.into_any(),
                    Some(s) => {
                        let n = |k: &str| s[k].as_u64().unwrap_or(0).to_string();
                        // Most claims were never judged related to another, which is what the graph view is for.
                        let (claims, connected) = (s["claims"].as_u64().unwrap_or(0), s["connected"].as_u64().unwrap_or(0));
                        let pct = if claims > 0 { connected * 100 / claims } else { 0 };
                        view! {
                            <div class="figures">
                                <Figure label="claims" value=claims.to_string() note="atomic, cited statements".to_string()
                                    route=Route::Notebook(Some(
                                        "SELECT kind, count() AS n FROM claim GROUP BY kind ORDER BY n DESC;".into())) />
                                <Figure label="cited sources" value=n("sources") note=format!("{} of them cited", n("cited"))
                                    route=Route::Notebook(Some("SELECT origin, count() AS n FROM source GROUP BY origin ORDER BY n DESC;".into())) />
                                <Figure label="numbers" value=n("observations") note="values, ranges and units".to_string()
                                    route=Route::Series(None) />
                                <Figure label="relations" value=n("relations") note=format!("{} contradictions", n("contradictions"))
                                    route=Route::Disputes />
                                <Figure label="connected claims" value=format!("{pct}%") note="judged related to another claim".to_string()
                                    route=Route::Graph(None, 2) />
                                <Figure label="open questions" value=n("questions") note="what is not known yet".to_string()
                                    route=Route::Questions />
                            </div>
                            <div class="home-cols">
                                <section class="panel">
                                    <h2>"Start here"</h2>
                                    <ul class="plain routes">
                                        <li><a href=href(&Route::Chapters(None))>"Dossiers"</a>
                                            <span class="muted">{format!("— {} episodes of the war, each with its outcome and a dated timeline", n("dossiers"))}</span></li>
                                        <li><a href=href(&Route::Map(None))>"Map"</a>
                                            <span class="muted">{format!("— {} places, sized by how much the base says about them", n("places"))}</span></li>
                                        <li><a href=href(&Route::Arms)>"Arms race"</a>
                                            <span class="muted">"— every measure and what countered it, on a timeline"</span></li>
                                        <li><a href=href(&Route::Disputes)>"Disputes"</a>
                                            <span class="muted">"— where claims cannot both be true, and why"</span></li>
                                        <li><a href=href(&Route::Graph(None, 2))>"Claim graph"</a>
                                            <span class="muted">"— how claims support, contradict and replace each other"</span></li>
                                        <li><a href=href(&Route::Notebook(None))>"Notebook"</a>
                                            <span class="muted">"— query the whole base yourself, in SurrealQL"</span></li>
                                    </ul>
                                    <p class="muted small">{format!("{} systems, {} actors and {} places, all linked from {} measure and countermeasure edges.",
                                        n("systems"), n("actors"), n("places"), n("counters"))}</p>
                                </section>

                                <section class="panel">
                                    <h2>"The strongest disagreements"</h2>
                                    {move || match disputes.get() {
                                        None => loading(),
                                        Some(None) => waiting_for_db(),
                                        Some(Some(Err(e))) => error_box(e),
                                        Some(Some(Ok(rows))) => view! {
                                            <ul class="plain">{rows.into_iter().map(|r| view! {
                                                <li class="mini-dispute">
                                                    <a href=href(&Route::Claim(r["a"].as_str().unwrap_or_default().to_string()))>
                                                        {r["a_text"].as_str().unwrap_or_default().to_string()}</a>
                                                    <span class="vs">"vs"</span>
                                                    <a href=href(&Route::Claim(r["b"].as_str().unwrap_or_default().to_string()))>
                                                        {r["b_text"].as_str().unwrap_or_default().to_string()}</a>
                                                </li>
                                            }).collect_view()}</ul>
                                            <p class="muted small"><a href=href(&Route::Disputes)>"all disputes"</a></p>
                                        }.into_any(),
                                    }}
                                </section>
                            </div>

                            <section class="panel">
                                <h2>"Eras"</h2>
                                {move || match eras.get() {
                                    None => loading(),
                                    Some(None) => waiting_for_db(),
                                    Some(Some(Err(e))) => error_box(e),
                                    Some(Some(Ok(rows))) => view! {
                                        <ol class="eras">{rows.into_iter().map(|r| {
                                            let year = r["from"].as_str().and_then(|s| s.get(0..4)).unwrap_or("").to_string();
                                            let summary = r["summary"].as_str().unwrap_or_default().to_string();
                                            let n = r["claims"].as_u64().unwrap_or(0);
                                            view! { <li>
                                                <span class="code">{rid_link(r["id"].as_str().unwrap_or_default(), Some(year.clone()))}</span>
                                                <strong>{r["title"].as_str().unwrap_or_default().to_string()}</strong>
                                                <span class="muted">{format!(" {n} claims")}</span>
                                                {(!summary.is_empty()).then(|| view! { <div class="muted small">{summary}</div> })}
                                            </li> }
                                        }).collect_view()}</ol>
                                    }.into_any(),
                                }}
                            </section>

                            <section class="panel">
                                <h2>"The research base"</h2>
                                <p class="muted small">"The nine topic surveys, each a cited account of one part of the war, and the 21 dossiers built from them:"</p>
                                {move || match topics.get() {
                                    None => loading(),
                                    Some(None) => waiting_for_db(),
                                    Some(Some(Err(e))) => error_box(e),
                                    Some(Some(Ok(rows))) => view! {
                                        <ul class="plain topics">{rows.into_iter().map(|r| {
                                            let file = r["file"].as_str().unwrap_or_default().to_string();
                                            let title = r["title"].as_str().unwrap_or_default().trim_start_matches("Topic: ").to_string();
                                            let n = r["claims"].as_u64().unwrap_or(0);
                                            view! { <li>
                                                <a href=format!("https://github.com/barafael/bavovniatko/blob/main/{file}")>
                                                    {title.clone()}</a>
                                                <span class="muted small">{format!(" {n} claims")}</span>
                                            </li> }
                                        }).collect_view()}</ul>
                                        <p class="muted small"><a href=href(&Route::Questions)>"what is still open"</a>
                                            {" · "}<a href=href(&Route::About)>"how it was built and how to read it"</a></p>
                                    }.into_any(),
                                }}
                            </section>
                        }.into_any()
                    }
                },
            }}
        </article>
    }
}