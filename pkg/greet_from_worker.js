self.importScripts("window_or_worker.js");

self.wasm_bindgen("window_or_worker_bg.wasm").then(function () {
  self.postMessage(wasm_bindgen.greet());
});
