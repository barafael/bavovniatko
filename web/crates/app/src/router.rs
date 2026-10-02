//! Hash routes, so every view is a shareable link and the site works as plain static files:
//! `#/notebook?q=…`, `#/claim/claim:…`, `#/disputes`, `#/map[/place:…|/topic:…]`, `#/arms`,
//! `#/series[/metric:…]`, `#/chapters[/topic:…]`, `#/about`. The older `#q=…` opens the notebook.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Route {
    Notebook(Option<String>),
    Claim(String),
    Disputes,
    Map(Option<String>),
    Arms,
    Series(Option<String>),
    Chapters(Option<String>),
    About,
}

fn decode(s: &str) -> String {
    js_sys::decode_uri_component(s).ok().and_then(|v| v.as_string()).unwrap_or_else(|| s.to_string())
}

fn encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}

pub fn parse(hash: &str) -> Route {
    let h = hash.trim_start_matches('#');
    if let Some(q) = h.strip_prefix("q=") {
        return Route::Notebook(Some(decode(q)));
    }
    let h = h.trim_start_matches('/');
    let (path, query) = h.split_once('?').unwrap_or((h, ""));
    let (page, arg) = path.split_once('/').map(|(p, a)| (p, Some(decode(a)))).unwrap_or((path, None));
    let arg = arg.filter(|a| !a.is_empty());
    match page {
        "claim" => arg.map(Route::Claim).unwrap_or(Route::Notebook(None)),
        "disputes" => Route::Disputes,
        "map" => Route::Map(arg),
        "arms" => Route::Arms,
        "series" => Route::Series(arg),
        "chapters" => Route::Chapters(arg),
        "about" => Route::About,
        _ => Route::Notebook(query.strip_prefix("q=").map(decode)),
    }
}

pub fn href(r: &Route) -> String {
    let with = |p: &str, a: &Option<String>| match a {
        Some(a) => format!("#/{p}/{}", encode(a)),
        None => format!("#/{p}"),
    };
    match r {
        Route::Notebook(None) => "#/notebook".into(),
        Route::Notebook(Some(q)) => format!("#/notebook?q={}", encode(q)),
        Route::Claim(id) => format!("#/claim/{}", encode(id)),
        Route::Disputes => "#/disputes".into(),
        Route::Map(a) => with("map", a),
        Route::Arms => "#/arms".into(),
        Route::Series(a) => with("series", a),
        Route::Chapters(a) => with("chapters", a),
        Route::About => "#/about".into(),
    }
}

fn current_hash() -> String {
    web_sys::window().and_then(|w| w.location().hash().ok()).unwrap_or_default()
}

/// The current route, kept in sync with `hashchange`.
#[derive(Clone, Copy)]
pub struct Router(pub RwSignal<Route>);

impl Router {
    pub fn start() -> Router {
        let route = RwSignal::new(parse(&current_hash()));
        let on_change = Closure::<dyn FnMut()>::new(move || route.set(parse(&current_hash())));
        if let Some(w) = web_sys::window() {
            let _ = w.add_event_listener_with_callback("hashchange", on_change.as_ref().unchecked_ref());
        }
        on_change.forget();
        Router(route)
    }
}

/// Navigate (adds a history entry).
pub fn go(r: Route) {
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_hash(&href(&r));
    }
}

/// Update the URL without a history entry or a route change (the notebook's current query).
pub fn replace(r: &Route) {
    if let Some(w) = web_sys::window() {
        let _ = w.history().and_then(|h| h.replace_state_with_url(&JsValue::NULL, "", Some(&href(r))));
    }
}

/// Where a record id from a result leads.
pub fn record_route(table: &str, rid: &str) -> Route {
    match table {
        "claim" => Route::Claim(rid.into()),
        "place" | "topic" => if table == "topic" && is_chapter(rid) { Route::Chapters(Some(rid.into())) } else { Route::Map(Some(rid.into())) },
        "metric" => Route::Series(Some(rid.into())),
        "system" => Route::Notebook(Some(format!("SELECT * FROM ONLY {rid};\nfn::counters_of({rid});"))),
        _ => Route::Notebook(Some(format!("SELECT * FROM ONLY {rid};"))),
    }
}

pub fn is_chapter(rid: &str) -> bool {
    let k = rid.trim_start_matches("topic:");
    k.len() == 3 && (k.starts_with('c') || k.starts_with('v')) && k[1..].chars().all(|c| c.is_ascii_digit())
}
