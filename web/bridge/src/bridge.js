// JavaScript side of the explorer: the libraries the Leptos app (crates/app) binds to through wasm-bindgen.
// Bundled by esbuild into crates/app/bridge/bridge.js (see package.json).

import { EditorView, keymap, placeholder } from "@codemirror/view";
import { EditorState, Prec } from "@codemirror/state";
import { basicSetup } from "codemirror";
import { autocompletion } from "@codemirror/autocomplete";
import { surrealql } from "@surrealdb/codemirror";
import * as maplibregl from "maplibre-gl";

// --- SurrealQL editor ---------------------------------------------------------------------------------------------

let completions = [];

/** Autocomplete words: tables, fields and fn:: helpers, given as [{label, type, detail, info}]. */
export function setCompletions(json) {
  completions = JSON.parse(json);
}

function complete(ctx) {
  const word = ctx.matchBefore(/[\w:.]+/);
  if (!word || (word.from === word.to && !ctx.explicit)) return null;
  return { from: word.from, options: completions, validFor: /^[\w:.]*$/ };
}

export class Editor {
  constructor(parent, doc, onChange, onRun) {
    const run = () => { onRun(); return true; };
    this.view = new EditorView({
      parent,
      state: EditorState.create({
        doc,
        extensions: [
          Prec.highest(keymap.of([{ key: "Mod-Enter", run }, { key: "Shift-Enter", run }])),
          basicSetup,
          surrealql(),
          autocompletion({ override: [complete] }),
          placeholder("SurrealQL … (Ctrl+Enter runs)"),
          EditorView.lineWrapping,
          EditorView.updateListener.of((u) => { if (u.docChanged) onChange(u.state.doc.toString()); }),
        ],
      }),
    });
  }
  value() { return this.view.state.doc.toString(); }
  setValue(text) {
    if (text === this.value()) return;
    this.view.dispatch({ changes: { from: 0, to: this.view.state.doc.length, insert: text } });
  }
  focus() { this.view.focus(); }
  destroy() { this.view.destroy(); }
}

// --- Map ----------------------------------------------------------------------------------------------------------

// MapLibre's worker is an ES module next to the bundle (copied by the build script), not inside it.
maplibregl.setWorkerUrl(new URL("maplibre-gl-worker.mjs", document.baseURI).href);

const STYLE_LIGHT = "https://tiles.openfreemap.org/styles/positron";
const STYLE_DARK = "https://tiles.openfreemap.org/styles/dark";
const EMPTY = { type: "FeatureCollection", features: [] };

export class GeoMap {
  /** onClick receives the clicked feature's properties as a JSON string. */
  constructor(parent, onClick) {
    const dark = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
    this.map = new maplibregl.Map({
      container: parent,
      style: dark ? STYLE_DARK : STYLE_LIGHT,
      center: [35.5, 48.5],
      zoom: 5.2,
      attributionControl: { compact: true },
    });
    this.map.addControl(new maplibregl.NavigationControl({ showCompass: false }), "top-right");
    this.ready = new Promise((res) => this.map.on("load", res));
    this.ready.then(() => {
      const m = this.map;
      m.addSource("areas", { type: "geojson", data: EMPTY });
      m.addSource("points", { type: "geojson", data: EMPTY });
      m.addLayer({ id: "areas-fill", type: "fill", source: "areas",
        paint: { "fill-color": ["coalesce", ["get", "color"], "#0057b7"], "fill-opacity": 0.12 } });
      m.addLayer({ id: "areas-line", type: "line", source: "areas",
        paint: { "line-color": ["coalesce", ["get", "color"], "#0057b7"], "line-width": 1.5 } });
      m.addLayer({ id: "points", type: "circle", source: "points",
        paint: {
          "circle-radius": ["interpolate", ["linear"], ["coalesce", ["get", "n"], 1], 1, 4, 50, 10, 500, 18],
          "circle-color": ["coalesce", ["get", "color"], "#0057b7"],
          "circle-opacity": 0.75, "circle-stroke-color": "#fff", "circle-stroke-width": 1 } });
      m.addLayer({ id: "labels", type: "symbol", source: "points", minzoom: 7,
        layout: { "text-field": ["get", "label"], "text-font": ["Noto Sans Regular"], "text-size": 11, "text-offset": [0, 1.1], "text-anchor": "top" },
        paint: { "text-color": dark ? "#e3e7ea" : "#1d232b", "text-halo-color": dark ? "#12161b" : "#fff", "text-halo-width": 1.2 } });
      for (const layer of ["points", "areas-fill"]) {
        m.on("click", layer, (e) => { if (e.features.length) onClick(JSON.stringify(e.features[0].properties)); });
        m.on("mouseenter", layer, () => { m.getCanvas().style.cursor = "pointer"; });
        m.on("mouseleave", layer, () => { m.getCanvas().style.cursor = ""; });
      }
    });
  }
  /** Replace the layers' data: GeoJSON FeatureCollections as JSON strings. */
  setData(points, areas) {
    this.ready.then(() => {
      this.map.getSource("points").setData(JSON.parse(points));
      this.map.getSource("areas").setData(JSON.parse(areas));
    });
  }
  /** Fit to [west, south, east, north]. */
  fit(w, s, e, n) {
    this.ready.then(() => this.map.fitBounds([[w, s], [e, n]], { padding: 40, maxZoom: 11, duration: 600 }));
  }
  resize() { this.map.resize(); }
  destroy() { this.map.remove(); }
}
