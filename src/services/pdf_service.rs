use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, HtmlElement};

use crate::models::pdf_text_item::PdfTextItem;
use crate::services::js_bridge::load_pdf_and_extract;

#[derive(Debug, Clone, PartialEq)]
pub struct PdfLoadResult {
    pub total_pages: u32,
    pub items: Vec<PdfTextItem>,
}

fn get_object_property(value: &JsValue, key: &str) -> Result<JsValue, JsValue> {
    js_sys::Reflect::get(value, &JsValue::from_str(key))
        .map_err(|_| JsValue::from_str(&format!("Unable to read '{key}' from the PDF payload.")))
}

fn read_required_f64(value: &JsValue, key: &str) -> Result<f64, JsValue> {
    let property = get_object_property(value, key)?;

    property
        .as_f64()
        .ok_or_else(|| JsValue::from_str(&format!("Expected '{key}' to be a number.")))
}

fn read_required_u32(value: &JsValue, key: &str) -> Result<u32, JsValue> {
    let number = read_required_f64(value, key)?;

    if !number.is_finite() || number < 0.0 {
        return Err(JsValue::from_str(&format!(
            "Expected '{key}' to be a non-negative number."
        )));
    }

    Ok(number as u32)
}

fn read_required_string(value: &JsValue, key: &str) -> Result<String, JsValue> {
    let property = get_object_property(value, key)?;

    property
        .as_string()
        .ok_or_else(|| JsValue::from_str(&format!("Expected '{key}' to be a string.")))
}

fn read_transform(value: &JsValue) -> Result<Vec<f64>, JsValue> {
    let raw_transform = get_object_property(value, "transform")?;
    let transform = js_sys::Array::from(&raw_transform);
    let mut values = Vec::with_capacity(transform.length() as usize);

    for entry in transform.iter() {
        let number = entry
            .as_f64()
            .ok_or_else(|| JsValue::from_str("Expected every transform entry to be numeric."))?;

        values.push(number);
    }

    Ok(values)
}

fn parse_pdf_text_item(value: &JsValue) -> Result<PdfTextItem, JsValue> {
    Ok(PdfTextItem {
        page: read_required_u32(value, "page")?,
        text: read_required_string(value, "text")?,
        left: read_required_f64(value, "left")?,
        top: read_required_f64(value, "top")?,
        width: read_required_f64(value, "width")?,
        height: read_required_f64(value, "height")?,
        transform: read_transform(value)?,
    })
}

fn parse_pdf_load_result(value: JsValue) -> Result<PdfLoadResult, JsValue> {
    let total_pages = read_required_u32(&value, "total_pages")?;
    let raw_items = get_object_property(&value, "items")?;
    let items_array = js_sys::Array::from(&raw_items);
    let mut items = Vec::with_capacity(items_array.length() as usize);

    for raw_item in items_array.iter() {
        items.push(parse_pdf_text_item(&raw_item)?);
    }

    Ok(PdfLoadResult { total_pages, items })
}

pub async fn file_to_uint8array(file: File) -> Result<js_sys::Uint8Array, JsValue> {
    let buffer = JsFuture::from(file.array_buffer()).await?;
    Ok(js_sys::Uint8Array::new(&buffer))
}

pub async fn process_pdf(file: File, host: HtmlElement) -> Result<PdfLoadResult, JsValue> {
    let bytes = file_to_uint8array(file).await?;
    let value = load_pdf_and_extract(bytes, host).await?;

    parse_pdf_load_result(value)
}
