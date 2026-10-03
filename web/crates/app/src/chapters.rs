//! Dossiers: each covers one bounded episode of the war. A dossier's page shows its historical outcome (the claims
//! flagged as such) and its dated claims as a timeline.

use leptos::prelude::*;
use serde_json::{Value, json};

use crate::router::{Route, href};
use crate::ui::{error_box, fmt_time, loading, rows_resource, strs, waiting_for_db};

const LIST: &str = "LET $out = (SELECT topics[0] AS t, count() AS n FROM claim WHERE ext.outcome = true GROUP BY t);
    LET $all = (SELECT topics[0] AS t, count() AS n FROM claim GROUP BY t);
    SELECT id, code, labels.en AS title, file,
        ($out[WHERE t = $parent.id][0].n ?? 0) AS outcome, ($all[WHERE t = $parent.id][0].n ?? 0) AS claims
    FROM topic WHERE string::starts_with(code, 'c') OR string::starts_with(code, 'v') ORDER BY code";

fn title(v: &Value) -> String {
    v.as_str().unwrap_or_default().trim_start_matches("Dossier: ").trim_start_matches("Chapter: ").trim_start_matches("Vignette: ").to_string()
}

#[component]
pub fn Chapters(topic: Option<String>) -> impl IntoView {
    let list = rows_resource(|| Some(LIST.to_string()));
    let current = topic.clone();
    view! {
        <div class="chapters">
            <details class="chapter-list side-list" open=crate::ui::wide_screen()>
                <summary>"Dossiers"</summary>
                {move || match list.get() {
                    None => loading(),
                    Some(None) => waiting_for_db(),
                    Some(Some(Err(e))) => error_box(e),
                    Some(Some(Ok(rows))) => view! { <ol>{rows.into_iter().map(|r| {
                        let id = r["id"].as_str().unwrap_or_default().to_string();
                        let active = current.as_deref() == Some(id.as_str());
                        view! { <li class:active=active>
                            <a href=href(&Route::Chapters(Some(id.clone())))>
                                <span class="code">{r["code"].as_str().unwrap_or_default().to_string()}</span>
                                {title(&r["title"])}</a>
                            <span class="muted small">{format!("{} claims · {} outcome", r["claims"], r["outcome"])}</span>
                        </li> }
                    }).collect_view()}</ol> }.into_any(),
                }}
            </details>
            <section class="chapter-body">
                {match topic {
                    None => view! { <div class="panel"><h1>"Dossiers"</h1>
                        <p>"Each dossier covers one bounded episode of the war, from the march on Kyiv to Operation Vivaldi:
                           what happened and how it ended, a dated timeline, the geography, the forces and the disputed figures.
                           Claims flagged "<span class="badge outcome">"outcome"</span>" describe the historical outcome."</p>
                        <p class="muted">"Pick a dossier from the list."</p></div> }.into_any(),
                    Some(t) => view! { <Chapter topic=t /> }.into_any(),
                }}
            </section>
        </div>
    }
}

#[component]
fn Chapter(topic: String) -> impl IntoView {
    let t2 = topic.clone();
    let head = rows_resource(move || Some(format!("SELECT code, labels.en AS title, file FROM ONLY {t2}")));
    let t3 = topic.clone();
    let outcome = rows_resource(move || Some(format!(
        "SELECT id, text, time FROM claim WHERE topics CONTAINS {t3} AND ext.outcome = true ORDER BY time.from, key")));
    let t4 = topic.clone();
    let timeline = rows_resource(move || Some(format!("fn::timeline({t4})")));
    let topic_map = topic.clone();
    let topic_nb = topic.clone();
    view! {
        {move || head.get().flatten().and_then(|r| r.ok()).and_then(|r| r.into_iter().next()).map(|h| view! {
            <h1>{title(&h["title"])}</h1>
            <p class="muted small">
                <a href=format!("https://github.com/barafael/bavovniatko/blob/main/{}", h["file"].as_str().unwrap_or_default())>
                    "Read the full dossier text"</a>
            </p>
        })}
        <p>
            <a href=href(&Route::Map(Some(topic_map.clone())))>"Show its places on the map"</a>" · "
            <a href=href(&Route::Notebook(Some(format!("fn::timeline({topic_nb});"))))>"open the timeline in the notebook"</a>
        </p>
        <div class="chapter-cols">
        <section class="panel">
            <h2>"Historical outcome"</h2>
            {move || match outcome.get() {
                None => loading(),
                Some(None) => waiting_for_db(),
                Some(Some(Err(e))) => error_box(e),
                Some(Some(Ok(rows))) if rows.is_empty() => view! { <p class="muted">"No outcome claims flagged."</p> }.into_any(),
                Some(Some(Ok(rows))) => view! { <ul class="history">{rows.into_iter().map(|r| view! {
                    <li><span class="when">{fmt_time(&r["time"])}</span>
                        <a href=href(&Route::Claim(r["id"].as_str().unwrap_or_default().to_string()))>{r["text"].as_str().unwrap_or_default().to_string()}</a></li>
                }).collect_view()}</ul> }.into_any(),
            }}
        </section>
        <section class="panel">
            <h2>"Timeline: dated claims in order"</h2>
            {move || match timeline.get() {
                None => loading(),
                Some(None) => waiting_for_db(),
                Some(Some(Err(e))) => error_box(e),
                Some(Some(Ok(rows))) => view! { <ul class="history script">{rows.into_iter().map(|r| {
                    let when = fmt_time(&json!({"from": r["t"], "to": r["t_to"], "precision": r["precision"]}));
                    let outcome = r["outcome"].as_bool().unwrap_or(false);
                    view! { <li class:outcome=outcome>
                        <span class="when">{when}</span>
                        <a href=href(&Route::Claim(r["id"].as_str().unwrap_or_default().to_string()))>{r["text"].as_str().unwrap_or_default().to_string()}</a>
                        {(!strs(&r["claimants"]).is_empty()).then(|| view! { <span class="muted small">{format!(" — {}", strs(&r["claimants"]).join(", "))}</span> })}
                    </li> }
                }).collect_view()}</ul> }.into_any(),
            }}
        </section>
        </div>
    }
}
