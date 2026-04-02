use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(module = "/src/bindings/pdf_bridge.js")]
extern "C" {
  #[wasm_bindgen(catch)]
  pub async fn load_pdf_and_extract(bytes: js_sys::Uint8Array, canvas: HtmlCanvasElement) -> Result<JsValue, JsValue>;
}