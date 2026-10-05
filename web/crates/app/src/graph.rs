//! The claim graph: claims and the relations between them, as a force-directed picture. It always starts from a
//! seed and walks outward, because most claims are unconnected and the whole graph would be a wall of dots.
//! `#/graph/claim:c07_0029_12?hops=2` is shareable, and the step count rides in the URL.

use std::collections::HashMap;

use leptos::prelude::*;
use serde_json::Value;

use crate::claim::{RELATIONS, relation_color};
use crate::kb::Kb;
use crate::router::{Route, go, href};
use crate::ui::{claim_key, error_box, loading, rows_resource, waiting_for_db};

/// How many claims to draw at most. Past this the picture stops helping.
const MAX_NODES: usize = 90;
/// The viewBox the layout fits into.
const W: f64 = 900.0;
const H: f64 = 620.0;
/// Layout iterations. Enough for this size and deterministic, unlike a live simulation.
const STEPS: usize = 400;

/// A claim placed in the picture.
struct Node {
    id: String,
    text: String,
    x: f64,
    y: f64,
    depth: i32,
}

/// A relation between two claims, kept for drawing and for the list under the picture.
struct Link {
    a: usize,
    b: usize,
    rel: String,
    strength: f64,
    rationale: String,
}

/// Force-directed layout: repulsion between every pair, springs along the relations, and a pull to the centre.
/// Deterministic — the same seed always draws the same graph.
fn layout(nodes: &mut [Node], links: &[Link]) {
    let n = nodes.len();
    if n < 2 {
        return;
    }
    let (cx, cy) = (W / 2.0, H / 2.0);
    let k = (W * H / n as f64).sqrt();
    // Put the seed and its first neighbours on a ring, so the walk's shape is visible before the springs settle.
    let ring: Vec<usize> = (0..n).filter(|i| nodes[*i].depth <= 1).collect();
    for (i, node) in ring.iter().enumerate() {
        let a = (i as f64 / ring.len().max(1) as f64) * std::f64::consts::TAU;
        let r = if nodes[*node].depth == 0 { 0.0 } else { 70.0 };
        nodes[*node].x = cx + r * a.cos();
        nodes[*node].y = cy + r * a.sin();
    }
    let (mut vx, mut vy) = (vec![0.0; n], vec![0.0; n]);
    for step in 0..STEPS {
        let cooling = 1.0 - step as f64 / STEPS as f64;
        vx.iter_mut().for_each(|v| *v = 0.0);
        vy.iter_mut().for_each(|v| *v = 0.0);
        for i in 0..n {
            for j in (i + 1)..n {
                let (mut dx, mut dy) = (nodes[i].x - nodes[j].x, nodes[i].y - nodes[j].y);
                let d2 = dx * dx + dy * dy;
                if d2 < 1.0 {
                    // Coincident nodes get a nudge along their index difference, so they can separate.
                    dx = ((i + 1) as f64).sin() * 0.7;
                    dy = ((j + 1) as f64).cos() * 0.7;
                }
                let d = (dx * dx + dy * dy).sqrt().max(0.5);
                let f = k * k / d * 0.02;
                let (ux, uy) = (dx / d, dy / d);
                vx[i] += ux * f;
                vy[i] += uy * f;
                vx[j] -= ux * f;
                vy[j] -= uy * f;
            }
        }
        for l in links {
            let (dx, dy) = (nodes[l.b].x - nodes[l.a].x, nodes[l.b].y - nodes[l.a].y);
            let d = (dx * dx + dy * dy).sqrt().max(0.1);
            let f = (d - k) / d * 0.6;
            vx[l.a] += dx * f;
            vy[l.a] += dy * f;
            vx[l.b] -= dx * f;
            vy[l.b] -= dy * f;
        }
        for i in 0..n {
            vx[i] += (cx - nodes[i].x) * 0.012;
            vy[i] += (cy - nodes[i].y) * 0.012;
            let speed = (vx[i] * vx[i] + vy[i] * vy[i]).sqrt().max(0.01);
            let capped = speed.min(k * 2.0);
            nodes[i].x += vx[i] / speed * capped * cooling;
            nodes[i].y += vy[i] / speed * capped * cooling;
        }
    }
    // Fit into the viewBox with a margin for the node radii.
    let pad = 24.0;
    let (mut x0, mut y0) = (f64::MAX, f64::MAX);
    let (mut x1, mut y1) = (f64::MIN, f64::MAX);
    for node in nodes.iter() {
        x0 = x0.min(node.x);
        y0 = y0.min(node.y);
        x1 = x1.max(node.x);
        y1 = y1.max(node.y);
    }
    let scale = ((W - 2.0 * pad) / (x1 - x0).max(1.0)).min((H - 2.0 * pad) / (y1 - y0).max(1.0));
    let (ox, oy) = ((W - (x1 - x0) * scale) / 2.0, (H - (y1 - y0) * scale) / 2.0);
    for node in nodes.iter_mut() {
        node.x = ox + (node.x - x0) * scale;
        node.y = oy + (node.y - y0) * scale;
    }
}

#[component]
pub fn Graph(seed: Option<String>, hops: i32) -> impl IntoView {
    let seed = seed.filter(|s| !s.is_empty());
    let (q_head, q_claims) = (seed.clone(), seed.clone());
    let head = rows_resource(move || q_head.clone().map(|s| format!(
        "SELECT id, labels.en AS name, labels.uk AS name_uk, labels.ru_translit AS name_ru, title, term, key
         FROM ONLY {s}")));
    let claims = rows_resource(move || q_claims.clone().map(|s| format!("fn::seed_claims({s});")));

    view! {
        <article class="page graph-page">
            <div class="graph-head">
                <h1>"The claim graph"</h1>
                <div class="filters">
                    <span class="muted">"Steps out from the seed"</span>
                    {(1..=3).into_iter().map(|h| {
                        let r = Route::Graph(seed.clone(), h);
                        view! { <a class=format!("steps {}", if h == hops { "active" } else { "" })
                            href=href(&r)>{h.to_string()}</a> }
                    }).collect_view()}
                    {seed.clone().map(|_| view! {
                        <a class="muted small" href=href(&Route::Graph(None, hops))>"clear the seed"</a>
                    }).into_any()}
                </div>
            </div>
            {move || match seed.clone() {
                None => view! { <GraphIntro /> }.into_any(),
                Some(s) => match (head.get(), claims.get()) {
                    (None, _) | (_, None) => loading(),
                    (Some(Some(Err(e))), _) | (_, Some(Some(Err(e)))) => error_box(e),
                    (Some(Some(Ok(h))), Some(Some(Ok(c)))) => {
                        match h.into_iter().next() {
                            Some(row) if row["id"].is_string() => view! {
                                <GraphBody head=row hops=hops seed=s claims=c />
                            }.into_any(),
                            Some(_) => view! { <p class="error">{format!("No record “{s}”. Try a claim, place, system, metric, topic or actor id.")}</p> }.into_any(),
                            None => view! { <p class="placeholder">"No such record."</p> }.into_any(),
                        }
                    }
                    _ => waiting_for_db(),
                },
            }}
        </article>
    }
}

/// What a record is called, best label first, for a page title.
fn title_of(r: &Value) -> String {
    ["name", "name_uk", "name_ru", "title", "term", "key"].iter()
        .find_map(|k| r[*k].as_str())
        .unwrap_or_default()
        .to_string()
}

#[component]
fn GraphIntro() -> impl IntoView {
    // Somewhere to type any record id: a claim key is enough, and the claim's id is derived from it.
    let text = RwSignal::new(String::new());
    let go_to = move || {
        let s = text.get_untracked().trim().to_string();
        if let Some(id) = crate::ui::claim_of_key(&s).or((!s.is_empty()).then(|| s)) {
            go(Route::Graph(Some(id), 2));
        }
    };
    view! {
        <div class="panel">
            <p>"Claims about the same thing argue with each other: one supports another, contradicts it, refines it or
               replaces it. Those relations are the graph."</p>
            <p>"Most claims are unconnected — about 70% were never judged related to another — so drawing all ten thousand
               at once would be a wall of dots. The graph starts from one record and walks outward."</p>
            <h2>"Start from a record"</h2>
            <div class="search-bar">
                <input type="search" placeholder="A record id or claim key, e.g. place:avdiivka or c07-0029-12"
                    prop:value=move || text.get() on:input=move |ev| text.set(event_target_value(&ev))
                    on:keydown=move |ev: web_sys::KeyboardEvent| { if ev.key() == "Enter" { go_to(); } } />
                <button on:click=move |_| go_to()>"draw"</button>
            </div>
            <h2>"Or try one of these"</h2>
            <ul class="plain seed-list">
                {[
                    ("claim:cc16_0047_11", "the densest contradiction cluster in the base"),
                    ("claim:c07_0029_12", "Russian ground-gain figures the sources disagree about"),
                    ("place:avdiivka", "every claim about Avdiivka"),
                    ("system:fibre_optic_fpv", "the fibre-optic FPV claims and their counters"),
                    ("metric:kab_launches", "every glide-bomb figure"),
                    ("topic:c16", "Operation Spiderweb"),
                    ("topic:c14", "the Kursk incursion"),
                ].into_iter().map(|(id, what)| {
                    let r = Route::Graph(Some(id.to_string()), 2);
                    view! { <li><a href=href(&r)><code>{id.to_string()}</code></a>
                        <span class="muted">" — "{what.to_string()}</span></li> }
                }).collect_view()}
            </ul>
            <p class="muted small">"Any record id works as a seed, and the notebook queries anything these views don't cover."</p>
        </div>
    }
}

#[component]
fn GraphBody(head: Value, hops: i32, seed: String, claims: Vec<Value>) -> impl IntoView {
    let title = title_of(&head);
    let q = format!("fn::graph([{}], {hops});", claims.iter().filter_map(|c| c.as_str()).collect::<Vec<_>>().join(", "));
    let edges = rows_resource(move || Some(q.clone()));
    view! {
        <>
            <p class="lede">{format!("Claims within {hops} relation{} of {title}, and the relations between them.",
                if hops == 1 { " step" } else { " steps" })}
                <span class="muted">" Extracted duplicate copies are left out; only claims that argue with, refine or replace each other appear."</span></p>
            <div class="graph-links">
                <a href=href(&Route::Record(seed.clone()))>"the seed's own page"</a>
                {" · "}<a href=href(&Route::Notebook(Some(format!("fn::graph(fn::seed_claims({seed}), {hops});"))))>
                    "open in notebook"</a>
                {" · "}<a href=href(&Route::Graph(Some(seed.clone()), (hops % 3) + 1))>
                    {if hops == 3 { "back to 1 step" } else { "one step further" }}</a>
            </div>
            {move || match edges.get() {
                None => loading(),
                Some(None) if !kb_ready() => waiting_for_db(),
                Some(Some(Err(e))) => error_box(e),
                Some(Some(Ok(rows))) => {
                    // fn::graph returns one object: {edges, claims}.
                    let row = rows.first().cloned().unwrap_or_default();
                    let (nodes, links, extra) = build(&rows_of(&row, "edges"), &rows_of(&row, "claims"), &claims, hops);
                    picture(nodes, links, extra)
                }
                _ => waiting_for_db(),
            }}
        </>
    }
}

/// Whether the browser database has finished loading, for the placeholder text.
fn kb_ready() -> bool {
    expect_context::<Kb>().is_ready()
}

/// One of fn::graph's two result arrays. A function returning an object comes back as a single row, so its arrays are
/// read by name.
fn rows_of(row: &Value, field: &str) -> Vec<Value> {
    row[field].as_array().cloned().unwrap_or_default()
}

/// Turn the edge rows into placed nodes and the links between them.
///
/// The walk is a breadth-first search from the seeds, taking the strongest relations first, so a crowded neighbourhood
/// keeps its most meaningful relations rather than whichever rows happened to arrive first.
fn build(rows: &[Value], details: &[Value], seeds: &[Value], hops: i32) -> (Vec<Node>, Vec<Link>, usize) {
    let seed = seeds.iter().filter_map(|c| c.as_str()).next().unwrap_or_default().to_string();
    // All the claims named by an edge, in either direction.
    let mut edges: Vec<(String, String, String, f64, String)> = vec![];
    for e in rows {
        let (Some(from), Some(to)) = (e["from"].as_str(), e["to"].as_str()) else { continue };
        if from == to || from.is_empty() || to.is_empty() {
            continue;
        }
        edges.push((from.to_string(), to.to_string(), e["rel"].as_str().unwrap_or_default().to_string(),
            e["strength"].as_f64().unwrap_or(0.5), e["rationale"].as_str().unwrap_or_default().to_string()));
    }
    edges.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

    // Breadth-first, strongest relations first, so `hops` steps means exactly that.
    let mut seen: std::collections::HashSet<String> = Default::default();
    let mut depth: HashMap<String, i32> = HashMap::new();
    // Every seed starts at depth 0: for a place or topic seed, the claims around it are the centre of the picture.
    let mut chosen: Vec<String> = vec![];
    for s in seeds.iter().filter_map(|c| c.as_str()) {
        if seen.insert(s.to_string()) {
            depth.insert(s.to_string(), 0);
            chosen.push(s.to_string());
        }
    }
    if chosen.is_empty() {
        chosen.push(seed.clone());
        seen.insert(seed.clone());
        depth.insert(seed.clone(), 0);
    }
    let mut extra = 0;
    for step in 1..=hops {
        let before = chosen.len();
        // The edges of everything found last step, strongest first.
        for (from, to, _, _, _) in &edges {
            for (near, far) in [(from, to), (to, from)] {
                if depth.get(near) == Some(&(step - 1)) && seen.insert(far.to_string()) {
                    depth.insert(far.to_string(), step);
                    chosen.push(far.to_string());
                }
            }
        }
        if chosen.len() == before {
            break;
        }
        if chosen.len() > MAX_NODES {
            // Count what the cap is hiding, so the caption can say the picture is a subset.
            extra = edges
                .iter()
                .flat_map(|(f, t, _, _, _)| [f.as_str(), t.as_str()])
                .filter(|id| depth.contains_key(*id) && !seen.insert((*id).to_string()))
                .count();
            chosen.truncate(MAX_NODES);
            break;
        }
    }
    let index: HashMap<&str, usize> = chosen.iter().enumerate().map(|(i, id)| (id.as_str(), i)).collect();

    // The claims' own text, from the same query.
    let mut nodes: Vec<Node> = chosen.iter().map(|id| Node {
        id: id.clone(),
        text: String::new(),
        x: W / 2.0,
        y: H / 2.0,
        depth: depth.get(id).copied().unwrap_or(hops),
    }).collect();
    for d in details {
        let Some(id) = d["id"].as_str() else { continue };
        let Some(i) = index.get(id) else { continue };
        nodes[*i].text = d["text"].as_str().unwrap_or_default().to_string();
    }

    // Links, each pair kept once.
    let mut links: Vec<Link> = vec![];
    let mut seen_links: std::collections::HashSet<(usize, usize, String)> = Default::default();
    for (from, to, rel, strength, rationale) in &edges {
        let (Some(a), Some(b)) = (index.get(from.as_str()), index.get(to.as_str())) else { continue };
        let key = if a <= b { (*a, *b, rel.clone()) } else { (*b, *a, rel.clone()) };
        if !seen_links.insert(key) {
            continue;
        }
        links.push(Link { a: *a, b: *b, rel: rel.clone(), strength: *strength, rationale: rationale.clone() });
    }
    (nodes, links, extra)
}

fn picture(nodes: Vec<Node>, links: Vec<Link>, extra: usize) -> AnyView {
    if nodes.len() < 2 {
        return view! {
            <p class="placeholder">{if nodes.is_empty() { "Nothing to draw.".to_string() } else {
                "That claim has no relations to other claims, so there is no graph to draw. Its page lists what it does link to.".to_string() }}
            </p>
        }.into_any();
    }
    let mut nodes = nodes;
    layout(&mut nodes, &links);
    // Relation counts for the legend, in the claim page's order.
    let mut legend: Vec<(String, usize)> = RELATIONS.iter().map(|(rel, _, _)| (rel.to_string(), 0)).collect();
    for l in &links {
        if let Some((_, n)) = legend.iter_mut().find(|(rel, _)| rel == &l.rel) {
            *n += 1;
        }
    }
    legend.retain(|(_, n)| *n > 0);
    legend.sort_by_key(|(rel, _)| RELATIONS.iter().position(|(r, _, _)| r == rel).unwrap_or(99));

    view! {
        <div class="graph-body">
            <div class="graph-wrap">
                <svg class="graph" viewBox=format!("0 0 {W} {H}") role="img"
                    aria-label="Claims and the relations between them">
                    {links.iter().map(|l| {
                        let (a, b) = (&nodes[l.a], &nodes[l.b]);
                        let color = relation_color(&l.rel);
                        let why = if l.rationale.is_empty() { String::new() } else { format!("\n{}", l.rationale) };
                        view! {
                            <line class="gedge" x1=a.x y1=a.y x2=b.x y2=b.y stroke=color
                                stroke-width=(0.8 + 1.8 * l.strength).to_string() stroke-opacity=".55">
                                <title>{format!("{} {} {} (strength {:.1}){why}", claim_key(&a.id), l.rel, claim_key(&b.id), l.strength)}</title>
                            </line>
                        }
                    }).collect_view()}
                    {nodes.iter().map(|node| {
                        // The seeds are largest and yellow; each step out is smaller and paler.
                        let r = 4.0 + 9.0 / (1.0 + node.depth as f64);
                        let fill = match node.depth {
                            0 => "var(--accent-2)",
                            1 => "var(--accent)",
                            _ => "var(--muted)",
                        };
                        let link = href(&Route::Claim(node.id.clone()));
                        view! {
                            <a href=link><circle class="gnode" cx=node.x cy=node.y r=r.to_string()
                                fill=fill stroke="var(--panel)" stroke-width="1.5" />
                                <title>{format!("{}{}", claim_key(&node.id),
                                    if node.text.is_empty() { String::new() } else { format!("\n{}", node.text) })}</title></a>
                        }
                    }).collect_view()}
                </svg>
                <div class="legend">{legend.iter().map(|(rel, n)| view! {
                    <span><i style=format!("background:{}", relation_color(rel))></i>{rel.clone()}{format!(" ({n})")}</span>
                }).collect_view()}</div>
            </div>
            <aside class="panel graph-side">
                <h2>{format!("{} claims, {} relations{}", nodes.len(), links.len(),
                    if extra > 0 { format!(" · {extra} more not drawn") } else { String::new() })}</h2>
                <p class="muted small">"The seeds are yellow and largest; each step out is smaller and paler. Line width
                   is the relation's strength, and hovering a line says why the two claims are linked. Clicking a claim
                   opens it."</p>
                <h3>"Relations"</h3>
                <ul class="plain rel-list">{links.iter().map(|l| {
                    let rel = l.rel.clone();
                    view! { <li style=format!("border-left-color:{}", relation_color(&rel))>
                        <a href=href(&Route::Claim(nodes[l.a].id.clone()))>{claim_key(&nodes[l.a].id)}</a>
                        <span class="muted small">{format!(" {rel} ")}</span>
                        <a href=href(&Route::Claim(nodes[l.b].id.clone()))>{claim_key(&nodes[l.b].id)}</a>
                        {(!l.rationale.is_empty()).then(|| view! { <div class="muted small">{l.rationale.clone()}</div> })}
                    </li> }
                }).collect_view()}</ul>
            </aside>
        </div>
    }
    .into_any()
}
