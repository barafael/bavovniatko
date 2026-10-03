// Starts the knowledge-base worker (crates/worker), which Trunk builds as a no-modules script. The query string
// (?v=<build>) is passed on so a new deploy never mixes with cached files of an older one.
importScripts("./kbworker.js" + self.location.search);
wasm_bindgen("./kbworker_bg.wasm" + self.location.search);
