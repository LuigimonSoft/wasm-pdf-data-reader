use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, HtmlCanvasElement};

use crate::models::pdf_text_item::PdfTextItem;
use crate::services::js_bridge::load_pdf_and_extract;

pub async fn file_to_uint8array(file: File) -> Result<js_sys::Uint8Array, JsValue> {
    let buffer = JsFuture::from(file.array_buffer()).await?;
    Ok(js_sys::Uint8Array::new(&buffer))
}

pub async fn process_pdf(
    file: File,
    canvas: HtmlCanvasElement,
) -> Result<Vec<PdfTextItem>, JsValue> {
    let bytes = file_to_uint8array(file).await?;
    let value = load_pdf_and_extract(bytes, canvas).await?;

    serde_wasm_bindgen::from_value(value)
        .map_err(|e| JsValue::from_str(&format!("serde error: {e}")))
}