async function initialize() {
  // await wasm_bindgen('../../wasm/proc_bg.wasm');

  console.log("hello");
  self.postMessage({
    message: "wasm INITIALIZED",
  });

}

initialize().then(() => {
  console.log("web worker finished initializing");
});