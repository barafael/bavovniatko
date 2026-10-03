//! Number series: every dated observation of a metric, plotted over time with its range (low–high), coloured by
//! side, so competing claims sit side by side. Pick a metric (most observations first) and a unit.

use leptos::prelude::*;
use serde_json::Value;

use crate::router::{Route, go, href};
use crate::ui::{error_box, fmt_date, loading, rows_resource, strs, waiting_for_db};

const METRICS: &str = "SELECT metric AS id, metric.labels.en AS label, count() AS n FROM observation
    WHERE time != NONE GROUP BY id, label ORDER BY n DESC";

fn side_color(side: Option<&str>) -> &'static str {
    match side {
        Some("ua") => "#0057b7",
        Some("ru") => "#c1121f",
        Some("western") => "#2e8540",
        Some("international") => "#7b2cbf",
        _ => "#6b7280",
    }
}

fn day_index(v: &Value) -> Option<f64> {
    let s = v.as_str()?;
    let y: f64 = s.get(0..4)?.parse().ok()?;
    let m: f64 = s.get(5..7)?.parse().ok()?;
    let d: f64 = s.get(8..10)?.parse().ok()?;
    Some(y * 365.25 + (m - 1.0) * 30.44 + d)
}

#[component]
pub fn SeriesPage(metric: Option<String>) -> impl IntoView {
    let metrics = rows_resource(|| Some(METRICS.to_string()));
    let chosen = metric.clone().unwrap_or_else(|| "metric:shahed_launches".into());
    let q = format!("fn::series({chosen})");
    let points = rows_resource(move || Some(q.clone()));
    let unit: RwSignal<Option<String>> = RwSignal::new(None);
    let chosen2 = chosen.clone();
    view! {
        <article class="page">
            <h1>"Number series"</h1>
            <p class="lede">"Every dated observation of a metric, with its range and side. Competing claimants appear
               side by side; click a point for its claim."</p>
            <div class="filters">
                <select on:change=move |ev| go(Route::Series(Some(event_target_value(&ev))))>
                    {move || metrics.get().flatten().and_then(|r| r.ok()).map(|rows| rows.into_iter().filter(|m| m["n"].as_u64().unwrap_or(0) >= 3).map(|m| {
                        let id = m["id"].as_str().unwrap_or_default().to_string();
                        let sel = id == chosen2;
                        view! { <option value=id.clone() selected=sel>
                            {format!("{} ({})", m["label"].as_str().unwrap_or(&id), m["n"])}</option> }
                    }).collect_view())}
                </select>
                <a href=href(&Route::Notebook(Some(format!("fn::series({chosen});"))))>"open in notebook"</a>
            </div>
            {move || match points.get() {
                None => loading(),
                Some(None) => waiting_for_db(),
                Some(Some(Err(e))) => error_box(e),
                Some(Some(Ok(rows))) => {
                    let mut units: Vec<(String, usize)> = vec![];
                    for r in &rows {
                        let u = r["unit"].as_str().unwrap_or_default().to_string();
                        match units.iter_mut().find(|x| x.0 == u) { Some(x) => x.1 += 1, None => units.push((u, 1)) }
                    }
                    units.sort_by_key(|u| std::cmp::Reverse(u.1));
                    let current = unit.get().filter(|u| units.iter().any(|x| &x.0 == u)).or_else(|| units.first().map(|u| u.0.clone())).unwrap_or_default();
                    let pts: Vec<Value> = rows.into_iter().filter(|r| r["unit"].as_str() == Some(current.as_str()) && !r["t"].is_null()).collect();
                    view! {
                        {(units.len() > 1).then(|| view! {
                            <div class="lane-filter">{units.iter().map(|(u, n)| {
                                let u2 = u.clone();
                                view! { <button class:active={u == &current} on:click=move |_| unit.set(Some(u2.clone()))>
                                    {format!("{} ({n})", u.replace("unit:", ""))}</button> }
                            }).collect_view()}</div>
                        })}
                        <div class="chart-wrap"><Chart points=pts.clone() unit=current.replace("unit:", "") /></div>
                        <div class="legend">{[("ua", "Ukraine"), ("ru", "Russia"), ("western", "Western"), ("", "unspecified")].into_iter().map(|(s, l)| view! {
                            <span><i style=format!("background:{}", side_color(Some(s)))></i>{l}</span>
                        }).collect_view()}</div>
                        <table class="obs">
                            <thead><tr><th>"when"</th><th>"value"</th><th>"side"</th><th>"claimants"</th><th>"claim"</th></tr></thead>
                            <tbody>{pts.into_iter().map(|p| view! { <tr>
                                <td>{fmt_date(&p["t"], None)}</td>
                                <td class="num">{p["value_text"].as_str().map(String::from).unwrap_or_else(|| p["value"].to_string())}</td>
                                <td>{p["side"].as_str().unwrap_or_default().to_string()}</td>
                                <td>{strs(&p["claimants"]).join(", ")}</td>
                                <td><a href=href(&Route::Claim(p["id"].as_str().unwrap_or_default().to_string()))>{p["claim"].as_str().unwrap_or_default().to_string()}</a></td>
                            </tr> }).collect_view()}</tbody>
                        </table>
                    }.into_any()
                }
            }}
        </article>
    }
}

fn tick_label(v: f64) -> String {
    let a = v.abs();
    if a >= 1e6 { format!("{:.1}M", v / 1e6) } else if a >= 1e4 { format!("{:.0}k", v / 1e3) }
    else if a >= 100.0 || v.fract() == 0.0 { format!("{v:.0}") } else { format!("{v:.1}") }
}

#[component]
fn Chart(points: Vec<Value>, unit: String) -> impl IntoView {
    let (w, h, l, r, t, b) = (900.0_f64, 340.0_f64, 52.0_f64, 16.0_f64, 12.0_f64, 34.0_f64);
    let xs: Vec<f64> = points.iter().filter_map(|p| day_index(&p["t"])).collect();
    let ys: Vec<f64> = points.iter().flat_map(|p| [p["value"].as_f64(), p["low"].as_f64(), p["high"].as_f64()]).flatten().collect();
    if xs.is_empty() || ys.is_empty() {
        return view! { <p class="muted">"No dated points in this unit."</p> }.into_any();
    }
    let (x0, x1) = (xs.iter().cloned().fold(f64::MAX, f64::min) - 15.0, xs.iter().cloned().fold(f64::MIN, f64::max) + 15.0);
    let ymax = ys.iter().cloned().fold(f64::MIN, f64::max);
    let ymin = ys.iter().cloned().fold(f64::MAX, f64::min).min(0.0);
    let (y0, y1) = (ymin, if ymax > ymin { ymax * 1.08 } else { ymin + 1.0 });
    let sx = move |x: f64| l + (x - x0) / (x1 - x0) * (w - l - r);
    let sy = move |y: f64| h - b - (y - y0) / (y1 - y0) * (h - t - b);
    let years: Vec<i32> = ((x0 / 365.25).ceil() as i32..=(x1 / 365.25).floor() as i32).collect();
    let ticks: Vec<f64> = (0..=4).map(|i| y0 + (y1 - y0) * i as f64 / 4.0).collect();
    view! {
        <svg class="chart" viewBox=format!("0 0 {w} {h}") role="img" aria-label="Number series chart">
            {ticks.into_iter().map(|v| view! { <g>
                <line x1=l x2=w - r y1=sy(v) y2=sy(v) class="grid" />
                <text x=l - 6.0 y=sy(v) + 4.0 class="tick" text-anchor="end">{tick_label(v)}</text>
            </g> }).collect_view()}
            {years.into_iter().map(|y| { let x = sx(y as f64 * 365.25 + 1.0); view! { <g>
                <line x1=x x2=x y1=t y2=h - b class="grid" />
                <text x=x y=h - b + 16.0 class="tick" text-anchor="middle">{y.to_string()}</text>
            </g> } }).collect_view()}
            <text x=w - r y=h - 4.0 class="tick" text-anchor="end">{format!("unit: {unit}")}</text>
            {points.into_iter().filter_map(|p| {
                let x = sx(day_index(&p["t"])?);
                let color = side_color(p["side"].as_str());
                let title = format!("{} · {} · {}", fmt_date(&p["t"], None),
                    p["value_text"].as_str().map(String::from).unwrap_or_else(|| p["value"].to_string()),
                    strs(&p["claimants"]).join(", "));
                let id = p["id"].as_str().unwrap_or_default().to_string();
                let range = match (p["low"].as_f64(), p["high"].as_f64()) {
                    (Some(lo), Some(hi)) => Some((sy(lo), sy(hi))),
                    _ => None,
                };
                let dot = p["value"].as_f64().or_else(|| match (p["low"].as_f64(), p["high"].as_f64()) {
                    (Some(lo), Some(hi)) => Some((lo + hi) / 2.0), (Some(v), None) | (None, Some(v)) => Some(v), _ => None });
                Some(view! {
                    <g class="pt" on:click=move |_| go(Route::Claim(id.clone()))>
                        <title>{title}</title>
                        {range.map(|(a, b)| view! { <line x1=x x2=x y1=a y2=b stroke=color stroke-width="2" /> })}
                        {dot.map(|v| view! { <circle cx=x cy=sy(v) r="4.5" fill=color /> })}
                    </g>
                })
            }).collect_view()}
        </svg>
    }.into_any()
}
