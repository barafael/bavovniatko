//! The arms race: every measure/countermeasure link on a timeline. A bar runs from when the measure was being
//! countered (first seen minus the reported lag) to when the counter was first observed; lanes group the measures
//! by kind. Click a bar for its effect, rationale and evidence.

use leptos::prelude::*;
use serde_json::Value;

use crate::router::{Route, href};
use crate::ui::{error_box, fmt_time, kind_label, loading, rid_link, rows_resource, strs, waiting_for_db};

const QUERY: &str = "SELECT id, in AS counter, in.labels.en AS counter_name, out AS measure, out.labels.en AS measure_name,
    out.kind AS kind, first_observed, lag_days, effect, rationale, evidence
    FROM counters WHERE first_observed != NONE ORDER BY first_observed.from";

const START_YEAR: i32 = 2022;
const MONTHS: f64 = 60.0;
const MONTH_PX: f64 = 19.0;
const ROW_PX: f64 = 11.0;
const LANE_GAP: f64 = 26.0;
const LEFT: f64 = 150.0;

fn months(v: &Value) -> Option<f64> {
    let s = v.as_str()?;
    let y: i32 = s.get(0..4)?.parse().ok()?;
    let m: f64 = s.get(5..7)?.parse().ok()?;
    let d: f64 = s.get(8..10).and_then(|d| d.parse().ok()).unwrap_or(1.0);
    Some(((y - START_YEAR) * 12) as f64 + m - 1.0 + (d - 1.0) / 30.0)
}

struct Bar {
    lane: String,
    row: usize,
    x0: f64,
    x1: f64,
    item: Value,
}

fn layout(rows: &[Value], lanes_on: &[String]) -> (Vec<Bar>, Vec<(String, f64, usize)>) {
    let mut by_lane: Vec<(String, Vec<&Value>)> = vec![];
    for r in rows {
        let lane = kind_label(&r["kind"]);
        if !lanes_on.is_empty() && !lanes_on.contains(&lane) {
            continue;
        }
        match by_lane.iter_mut().find(|l| l.0 == lane) {
            Some(l) => l.1.push(r),
            None => by_lane.push((lane, vec![r])),
        }
    }
    by_lane.sort_by_key(|l| std::cmp::Reverse(l.1.len()));
    let mut bars = vec![];
    let mut lanes = vec![];
    let mut y = 24.0;
    for (lane, items) in by_lane {
        let mut row_ends: Vec<f64> = vec![];
        let top = y;
        for r in items {
            let Some(end) = months(&r["first_observed"]["from"]) else { continue };
            let lag = r["lag_days"].as_f64().unwrap_or(0.0) / 30.4;
            let (x0, x1) = (LEFT + (end - lag).max(0.0) * MONTH_PX, LEFT + end * MONTH_PX + 5.0);
            let row = match row_ends.iter().position(|e| *e + 3.0 < x0) {
                Some(i) => { row_ends[i] = x1; i }
                None => { row_ends.push(x1); row_ends.len() - 1 }
            };
            bars.push(Bar { lane: lane.clone(), row, x0, x1, item: r.clone() });
        }
        let n_rows = row_ends.len().max(1);
        lanes.push((lane, top, n_rows));
        y += n_rows as f64 * ROW_PX + LANE_GAP;
    }
    for b in &mut bars {
        b.x0 = b.x0.min(b.x1 - 5.0);
    }
    (bars, lanes)
}

#[component]
pub fn Arms() -> impl IntoView {
    let data = rows_resource(|| Some(QUERY.to_string()));
    let selected: RwSignal<Option<Value>> = RwSignal::new(None);
    let lanes_on: RwSignal<Vec<String>> = RwSignal::new(vec![]);
    view! {
        <article class="page">
            <h1>"The arms race"</h1>
            <p class="lede">"Measure and countermeasure: each bar runs from a measure being countered to the counter's first
               observation (the reported lag), in lanes by the kind of measure. Hover for names; click for the evidence."</p>
            {move || match data.get() {
                None => loading(),
                Some(None) => waiting_for_db(),
                Some(Some(Err(e))) => error_box(e),
                Some(Some(Ok(rows))) => {
                    let mut kinds: Vec<(String, usize)> = vec![];
                    for r in &rows {
                        let k = kind_label(&r["kind"]);
                        match kinds.iter_mut().find(|x| x.0 == k) { Some(x) => x.1 += 1, None => kinds.push((k, 1)) }
                    }
                    kinds.sort_by_key(|k| std::cmp::Reverse(k.1));
                    let on = lanes_on.get();
                    let (bars, lanes) = layout(&rows, &on);
                    let height = lanes.last().map_or(80.0, |(_, t, n)| t + *n as f64 * ROW_PX + 30.0);
                    let width = LEFT + MONTHS * MONTH_PX + 20.0;
                    let tops: std::collections::HashMap<String, f64> = lanes.iter().map(|(l, t, _)| (l.clone(), *t)).collect();
                    view! {
                        <div class="lane-filter">
                            <button class:active=on.is_empty() on:click=move |_| lanes_on.set(vec![])>"all"</button>
                            {kinds.into_iter().map(|(k, n)| {
                                let k2 = k.clone();
                                let active = on.contains(&k);
                                view! { <button class:active=active on:click=move |_| lanes_on.update(|l| {
                                    if let Some(i) = l.iter().position(|x| *x == k2) { l.remove(i); } else { l.push(k2.clone()); }
                                })>{format!("{k} ({n})")}</button> }
                            }).collect_view()}
                        </div>
                        <div class="timeline-wrap">
                            <svg class="timeline" viewBox=format!("0 0 {width} {height}") preserveAspectRatio="xMinYMin meet">
                                {(0..=5).map(|i| {
                                    let x = LEFT + (i * 12) as f64 * MONTH_PX;
                                    view! { <g><line x1=x y1="14" x2=x y2=height class="year-line" />
                                        <text x=x + 3.0 y="12" class="year">{(START_YEAR + i).to_string()}</text></g> }
                                }).collect_view()}
                                {lanes.iter().map(|(l, t, _)| view! { <text x="4" y=*t + 9.0 class="lane">{l.clone()}</text> }).collect_view()}
                                {bars.into_iter().map(|b| {
                                    let y = tops.get(&b.lane).copied().unwrap_or(0.0) + b.row as f64 * ROW_PX;
                                    let title = format!("{} counters {} · {}{}",
                                        b.item["counter_name"].as_str().unwrap_or_default(),
                                        b.item["measure_name"].as_str().unwrap_or_default(),
                                        fmt_time(&b.item["first_observed"]),
                                        b.item["lag_days"].as_f64().map(|l| format!(" · lag {l:.0} days")).unwrap_or_default());
                                    let item = b.item.clone();
                                    let has_lag = b.item["lag_days"].as_f64().is_some();
                                    view! {
                                        <rect x=b.x0 y=y width=b.x1 - b.x0 height=ROW_PX - 3.0 rx="2"
                                            class=if has_lag { "bar lag" } else { "bar" }
                                            on:click=move |_| selected.set(Some(item.clone()))>
                                            <title>{title}</title>
                                        </rect>
                                    }
                                }).collect_view()}
                            </svg>
                        </div>
                    }.into_any()
                }
            }}
            {move || selected.get().map(|s| {
                let counter = s["counter"].as_str().unwrap_or_default().to_string();
                let measure = s["measure"].as_str().unwrap_or_default().to_string();
                view! {
                    <section class="panel selected">
                        <h2>{rid_link(&counter, s["counter_name"].as_str().map(String::from))}" counters "
                            {rid_link(&measure, s["measure_name"].as_str().map(String::from))}</h2>
                        <p class="muted">{format!("first observed {}", fmt_time(&s["first_observed"]))}
                            {s["lag_days"].as_f64().map(|l| format!(" · about {l:.0} days after the measure"))}</p>
                        {s["effect"].as_str().map(|e| view! { <p><strong>"Effect: "</strong>{e.to_string()}</p> })}
                        {s["rationale"].as_str().map(|e| view! { <p>{e.to_string()}</p> })}
                        <h3>"Evidence"</h3>
                        <ul class="plain">{strs(&s["evidence"]).into_iter().map(|c| view! {
                            <li><a href=href(&Route::Claim(c.clone()))>{c.clone()}</a></li>
                        }).collect_view()}</ul>
                        <p><a href=href(&Route::Notebook(Some(format!("fn::counter_chain({measure});"))))>
                            "What else countered this measure, and what countered those →"</a></p>
                    </section>
                }
            })}
        </article>
    }
}
