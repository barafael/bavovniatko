//! The map: every place the knowledge base talks about, sized by the number of claims about it in a chosen month
//! range. Click a place for its claims. `#/map/place:…` focuses a place; `#/map/topic:c14` a dossier's places.

use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;

use crate::bridge::GeoMap;
use crate::kb::Kb;
use crate::router::{Route, href};
use crate::ui::{MONTHS, claim_link, fmt_when, key_of, list, ok_rows, rows_resource, text, ymd};

/// Month index 0 = Jan 2022.
const MONTH_COUNT: i32 = 12 * 5;

fn month_label(i: i32) -> String {
    format!("{} {}", MONTHS[(i % 12) as usize], 2022 + i / 12)
}

/// "2024-08-06T…" → month index.
fn month_of(v: &Value) -> Option<i32> {
    let (y, m, _) = ymd(v).filter(|(_, m, _)| *m > 0)?;
    Some((y - 2022) * 12 + m as i32 - 1)
}

const PLACES: &str = "SELECT id, labels.en AS label, kind, centroid, (<-about<-claim.time.from) AS times FROM place
    WHERE centroid != NONE AND kind NOT IN [kind:['place', 'country'], kind:['place', 'oblast'], kind:['place', 'region'],
    kind:['place', 'sea'], kind:['place', 'axis'], kind:['place', 'front_sector']]";
const BOXES: &str = "SELECT id, labels.en AS label, geometry FROM place WHERE kind = kind:['place', 'sample_box'] AND geometry != NONE";

/// The box around the central 80% of points, so a few far-off places (Moscow, Pyongyang) don't stretch it.
fn bbox_of(points: &[(f64, f64)]) -> Option<(f64, f64, f64, f64)> {
    if points.is_empty() {
        return None;
    }
    let mut xs: Vec<f64> = points.iter().map(|p| p.0).collect();
    let mut ys: Vec<f64> = points.iter().map(|p| p.1).collect();
    xs.sort_by(|a, b| a.total_cmp(b));
    ys.sort_by(|a, b| a.total_cmp(b));
    let q = |v: &[f64], f: f64| v[((v.len() - 1) as f64 * f).round() as usize];
    let (lo, hi) = if points.len() >= 10 { (0.1, 0.9) } else { (0.0, 1.0) };
    Some((q(&xs, lo), q(&ys, lo), q(&xs, hi), q(&ys, hi)))
}

#[component]
pub fn MapPage(focus: Option<String>) -> impl IntoView {
    let kb = expect_context::<Kb>();
    let from = RwSignal::new(0);
    let to = RwSignal::new(MONTH_COUNT - 1);
    let undated = RwSignal::new(true);
    let selected: RwSignal<Option<String>> = RwSignal::new(focus.clone().filter(|f| f.starts_with("place:")));
    let chapter = focus.clone().filter(|f| f.starts_with("topic:"));

    let places = rows_resource(|| Some(PLACES.to_string()));
    let boxes = rows_resource(|| Some(BOXES.to_string()));
    let chapter_q = chapter.clone().map(|t| format!(
        "array::distinct(array::flatten((SELECT VALUE ->about->place FROM claim WHERE topics CONTAINS {t})))"));
    let chapter_places = rows_resource(move || chapter_q.clone());

    let host = NodeRef::<leptos::html::Div>::new();
    let map = StoredValue::new_local(None::<GeoMap>);
    let fitted = StoredValue::new(false);
    Effect::new(move |_| {
        let Some(el) = host.get() else { return };
        if map.with_value(|m| m.is_some()) {
            return;
        }
        let on_click = Closure::<dyn FnMut(String)>::new(move |props: String| {
            if let Ok(p) = serde_json::from_str::<Value>(&props)
                && let Some(id) = p["id"].as_str()
            {
                selected.set(Some(id.to_string()));
            }
        });
        map.set_value(Some(GeoMap::new(&el, on_click.as_ref())));
        on_click.forget();
    });
    on_cleanup(move || map.with_value(|m| if let Some(m) = m { m.destroy() }));

    // Points and areas, recomputed when the data or the filters change.
    Effect::new(move |_| {
        let (lo, hi, und) = (from.get(), to.get(), undated.get());
        let Some(rows) = ok_rows(places.get()) else { return };
        let highlight: Vec<String> = ok_rows(chapter_places.get()).unwrap_or_default()
            .iter().filter_map(|v| v.as_str().map(String::from)).collect();
        let sel = selected.get();
        let mut feats = vec![];
        let mut hl_points = vec![];
        for r in &rows {
            let times = list(&r["times"]);
            let n = times.iter().filter(|t| match month_of(t) { Some(m) => m >= lo && m <= hi, None => und }).count();
            let id = text(&r["id"]);
            let is_hl = highlight.contains(&id);
            if n == 0 && !is_hl && sel.as_deref() != Some(id.as_str()) {
                continue;
            }
            let c = &r["centroid"]["coordinates"];
            let (x, y) = (c[0].as_f64().unwrap_or(0.0), c[1].as_f64().unwrap_or(0.0));
            if is_hl {
                hl_points.push((x, y));
            }
            let color = if sel.as_deref() == Some(id.as_str()) { "#c1121f" } else if is_hl { "#e0a800" } else { "#0057b7" };
            feats.push(json!({"type": "Feature", "geometry": r["centroid"],
                "properties": {"id": id, "label": r["label"], "n": n, "color": color}}));
        }
        let areas: Vec<Value> = ok_rows(boxes.get()).unwrap_or_default().iter().map(|b| json!({"type": "Feature",
            "geometry": b["geometry"], "properties": {"id": b["id"], "label": b["label"], "color": "#7b2cbf"}})).collect();
        map.with_value(|m| if let Some(m) = m {
            m.set_data(&json!({"type": "FeatureCollection", "features": feats}).to_string(),
                       &json!({"type": "FeatureCollection", "features": areas}).to_string());
            if !fitted.get_value() {
                if let Some((w, s, e, n)) = bbox_of(&hl_points) {
                    m.fit(w - 0.1, s - 0.1, e + 0.1, n + 0.1);
                    fitted.set_value(true);
                } else if let Some(p) = sel.as_ref().and_then(|s| rows.iter().find(|r| r["id"].as_str() == Some(s.as_str()))) {
                    let c = &p["centroid"]["coordinates"];
                    let (x, y) = (c[0].as_f64().unwrap_or(0.0), c[1].as_f64().unwrap_or(0.0));
                    m.fit(x - 0.25, y - 0.15, x + 0.25, y + 0.15);
                    fitted.set_value(true);
                }
            }
        });
    });

    // The selected place's claims.
    let history: RwSignal<Option<(String, Vec<Value>)>> = RwSignal::new(None);
    Effect::new(move |_| {
        let Some(id) = selected.get() else { return };
        if !kb.is_ready() {
            return;
        }
        spawn_local(async move {
            let rows = kb.rows(format!("fn::place_history({id})")).await.unwrap_or_default();
            history.set(Some((id, rows)));
        });
    });

    view! {
        <div class="map-page">
            <div class="map-controls">
                <label>"From " <strong>{move || month_label(from.get())}</strong>
                    <input type="range" min="0" max=MONTH_COUNT - 1 prop:value=move || from.get()
                        on:input=move |ev| { let v: i32 = event_target_value(&ev).parse().unwrap_or(0); from.set(v.min(to.get_untracked())); } /></label>
                <label>"to " <strong>{move || month_label(to.get())}</strong>
                    <input type="range" min="0" max=MONTH_COUNT - 1 prop:value=move || to.get()
                        on:input=move |ev| { let v: i32 = event_target_value(&ev).parse().unwrap_or(0); to.set(v.max(from.get_untracked())); } /></label>
                <label><input type="checkbox" prop:checked=move || undated.get() on:change=move |ev| undated.set(event_target_checked(&ev)) />
                    " include undated claims"</label>
                {chapter.clone().map(|t| view! { <span class="chip">{format!("highlighting dossier {}", key_of(&t))}</span> })}
                <span class="muted small">"Circle size: claims about the place in the range. Purple boxes: terrain sample areas."</span>
            </div>
            <div class="map-body">
                <div class="map" node_ref=host></div>
                <aside class="map-side">
                    {move || match history.get() {
                        None => view! { <p class="muted">"Click a place to see what the knowledge base says about it."</p> }.into_any(),
                        Some((id, rows)) => {
                            let (lo, hi, und) = (from.get(), to.get(), undated.get());
                            let shown: Vec<Value> = rows.into_iter().filter(|r| match month_of(&r["t"]) {
                                Some(m) => m >= lo && m <= hi, None => und }).collect();
                            view! {
                                <h2>{key_of(&id).replace('_', " ")}</h2>
                                <p class="muted small">{format!("{} claims in range", shown.len())} " · "
                                    <a href=href(&Route::Notebook(Some(format!("fn::place_history({id});"))))>"open in notebook"</a></p>
                                <ul class="history">{shown.into_iter().map(|r| view! {
                                    <li>
                                        <span class="when">{fmt_when(&r["t"], None, &r["precision"])}</span>
                                        {claim_link(&r["id"], &r["text"])}
                                    </li>
                                }).collect_view()}</ul>
                            }.into_any()
                        }
                    }}
                </aside>
            </div>
        </div>
    }
}
