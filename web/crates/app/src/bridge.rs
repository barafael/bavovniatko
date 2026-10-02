//! Bindings to the JavaScript bridge (web/bridge, bundled into crates/app/bridge/bridge.js and loaded by index.html
//! before the app as the global `bavBridge`): the SurrealQL editor (CodeMirror 6 + @surrealdb/codemirror) and the
//! map (MapLibre GL on OpenFreeMap tiles).

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(js_namespace = bavBridge)]
extern "C" {
    /// Autocomplete words for the editor: a JSON array of `{label, type, detail, info}`.
    #[wasm_bindgen(js_name = setCompletions)]
    pub fn set_completions(json: &str);

    pub type Editor;
    #[wasm_bindgen(constructor)]
    pub fn new(parent: &HtmlElement, doc: &str, on_change: &JsValue, on_run: &JsValue) -> Editor;
    #[wasm_bindgen(method)]
    pub fn value(this: &Editor) -> String;
    #[wasm_bindgen(method, js_name = setValue)]
    pub fn set_value(this: &Editor, text: &str);
    #[wasm_bindgen(method)]
    pub fn focus(this: &Editor);
    #[wasm_bindgen(method)]
    pub fn destroy(this: &Editor);

    pub type GeoMap;
    #[wasm_bindgen(constructor)]
    pub fn new(parent: &HtmlElement, on_click: &JsValue) -> GeoMap;
    #[wasm_bindgen(method, js_name = setData)]
    pub fn set_data(this: &GeoMap, points: &str, areas: &str);
    #[wasm_bindgen(method)]
    pub fn fit(this: &GeoMap, west: f64, south: f64, east: f64, north: f64);
    #[wasm_bindgen(method)]
    pub fn resize(this: &GeoMap);
    #[wasm_bindgen(method)]
    pub fn destroy(this: &GeoMap);
}
