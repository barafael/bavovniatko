//! The bavovniatko knowledge-base explorer: a static, client-side site. The knowledge base runs as SurrealDB
//! (WebAssembly) in a Web Worker (crates/worker); this crate is the UI. Views are hash routes (see router.rs).

mod arms;
mod bridge;
mod chapters;
mod claim;
mod disputes;
mod graph;
mod home;
mod kb;
mod map;
mod notebook;
mod questions;
mod record;
mod results;
mod router;
mod schema;
mod search;
mod series;
mod ui;

use leptos::prelude::*;

use kb::{Kb, Status};
use router::{Route, Router, href};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let kb = Kb::start();
    provide_context(kb);
    leptos::task::spawn_local(async { schema::load().await; });
    let Router(route) = Router::start();
    let tab = move |target: Route, label: &'static str| {
        let t = target.clone();
        let active = move || std::mem::discriminant(&route.get()) == std::mem::discriminant(&t);
        view! { <a class="tab" class:active=active href=href(&target)>{label}</a> }
    };
    view! {
        <header>
            <a class="brand" href="#/">
                <span class="title">"bavovniatko"</span>
                <span class="subtitle">"knowledge base of the Russo-Ukrainian war"</span>
            </a>
            <nav>
                {tab(Route::Home, "Home")}
                {tab(Route::Search(String::new()), "Search")}
                {tab(Route::Chapters(None), "Dossiers")}
                {tab(Route::Map(None), "Map")}
                {tab(Route::Arms, "Arms race")}
                {tab(Route::Series(None), "Numbers")}
                {tab(Route::Disputes, "Disputes")}
                {tab(Route::Graph(None, 2), "Graph")}
                {tab(Route::Questions, "Questions")}
                {tab(Route::Notebook(None), "Notebook")}
                {tab(Route::About, "About")}
            </nav>
            <LoadStatus />
        </header>
        <main>
            {move || match route.get() {
                Route::Home => view! { <home::Home /> }.into_any(),
                Route::Notebook(q) => view! { <notebook::Notebook initial=q /> }.into_any(),
                Route::Search(q) => view! { <search::Search initial=q /> }.into_any(),
                Route::Questions => view! { <questions::Questions /> }.into_any(),
                Route::Graph(seed, hops) => view! { <graph::Graph seed=seed hops=hops /> }.into_any(),
                Route::Claim(id) => view! { <claim::ClaimPage id=id /> }.into_any(),
                Route::Record(id) => view! { <record::RecordPage id=id /> }.into_any(),
                Route::Disputes => view! { <disputes::Disputes /> }.into_any(),
                Route::Map(focus) => view! { <map::MapPage focus=focus /> }.into_any(),
                Route::Arms => view! { <arms::Arms /> }.into_any(),
                Route::Series(m) => view! { <series::SeriesPage metric=m /> }.into_any(),
                Route::Chapters(t) => view! { <chapters::Chapters topic=t /> }.into_any(),
                Route::About => view! { <About /> }.into_any(),
            }}
        </main>
        <footer>
            "Content CC BY 4.0 · place geometry ODbL, © OpenStreetMap contributors · code MIT or Apache-2.0 · runs "
            <a href="licenses/SurrealDB-BSL-1.1.txt">"SurrealDB (Business Source License 1.1)"</a>" · "
            <a href="#/about">"licences and attribution"</a>" · "
            <a href="https://github.com/barafael/bavovniatko">"source"</a>
        </footer>
    }
}

#[component]
fn LoadStatus() -> impl IntoView {
    let kb = expect_context::<Kb>();
    move || match kb.status.get() {
        Status::Loading { file, done, total, ms } => {
            let pct = (done as f64 / total as f64 * 100.0).round();
            view! {
                <div class="status loading" title=format!("{file} · {:.1} s", ms / 1000.0)>
                    <span>{format!("Loading the knowledge base … {pct}%")}</span>
                    <progress max="100" value=pct></progress>
                </div>
            }.into_any()
        }
        Status::Ready { version, ms, rows, .. } => view! {
            <div class="status ready" title=format!("dataset {version}")>
                {format!("{rows} records · ready in {:.1} s", ms / 1000.0)}
            </div>
        }.into_any(),
        Status::Failed(e) => view! { <div class="status failed">{format!("Loading failed: {e}")}</div> }.into_any(),
    }
}

#[component]
fn About() -> impl IntoView {
    let kb = expect_context::<Kb>();
    let attribution = move || match kb.status.get() {
        Status::Ready { attribution, .. } => attribution,
        _ => String::new(),
    };
    view! {
        <article class="about">
            <h1>"About this explorer"</h1>
            <p>"A cited research knowledge base on the Russo-Ukrainian war, from the full-scale invasion of February 2022
               to the present. Every statement is an atomic, cited "<strong>"claim"</strong>", with its claimants, eras, places,
               observations (numbers with units and ranges) and its relations to other claims: supports, weakens,
               contradicts, refines, supersedes, duplicates."</p>
            <p>"The whole knowledge base runs in your browser: SurrealDB compiled to WebAssembly, loaded fresh on
               each visit. Your queries never leave your machine, and the database is read-only. The map loads its
               base tiles from OpenFreeMap."</p>
            <h2>"Reading the data"</h2>
            <ul>
                <li>"Sources are English-language, preferring Ukrainian outlets, paired with Western analysis and OSINT.
                     Russian claims appear only as Ukrainian or Western sources report them."</li>
                <li>"Contested figures are kept as ranges with their claimants; disagreements are explicit "
                     <code>"contradicts"</code>" relations, often with an explanation."</li>
                <li>"Claims were extracted from the research texts by AI agents and checked by tools and spot reviews.
                     Treat every claim as a pointer to its sources, not as established fact."</li>
            </ul>
            <h2>"Licences"</h2>
            <ul>
                <li>"Content and data: "<a href="https://creativecommons.org/licenses/by/4.0/">"CC BY 4.0"</a>
                    ", except the OpenStreetMap-derived place geometry, which is "
                    <a href="https://opendatacommons.org/licenses/odbl/1-0/">"ODbL 1.0"</a>" (© OpenStreetMap contributors),
                    and quotations, which remain their authors'. See "
                    <a href="https://github.com/barafael/bavovniatko/blob/main/LICENSE-CONTENT.md">"LICENSE-CONTENT.md"</a>"."</li>
                <li>"Code: MIT or Apache-2.0, at your option ("
                    <a href="https://github.com/barafael/bavovniatko">"source on GitHub"</a>")."</li>
                <li><strong>"This site runs SurrealDB, which is licensed under the "
                    <a href="licenses/SurrealDB-BSL-1.1.txt">"Business Source License 1.1"</a>"."</strong>
                    " It is used under the licence's Additional Use Grant: visitors query a read-only copy and cannot
                     create, manage or control schemas or tables."</li>
                <li>"All other bundled software and its licences: "
                    <a href="licenses/THIRD-PARTY-NOTICES.txt">"third-party notices"</a>"."</li>
            </ul>
            <h2>"Attribution"</h2>
            <p>{attribution}</p>
            <p>"Base map: © OpenFreeMap, © OpenMapTiles, data © OpenStreetMap contributors. Weather figures: Open-Meteo
               (CC BY 4.0), contains modified Copernicus Climate Change Service information. Frontline areas computed
               from DeepStateMap's published maps. Elevation: SRTM (NASA/USGS, public domain)."</p>
            <p>"Query engine: SurrealDB. Editor: CodeMirror with @surrealdb/codemirror. Map renderer: MapLibre GL.
               UI: Leptos."</p>
        </article>
    }
}
