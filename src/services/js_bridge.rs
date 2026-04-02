use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/src/bindings/pdf_bridge.js")]
extern "C" {
  #[wasm_bindgen(catch)]
  pub async fn load_pdf_and_extract(bytes: js_sys::Uint8Array, host: HtmlElement) -> Result<JsValue, JsValue>;
}
