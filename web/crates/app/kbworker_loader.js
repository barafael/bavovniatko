// Starts the knowledge-base worker (crates/worker), which Trunk builds as a no-modules script.
importScripts("./kbworker.js");
wasm_bindgen("./kbworker_bg.wasm");
