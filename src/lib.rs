use leptos::prelude::*;

pub mod components;
pub mod models;

#[cfg(target_arch = "wasm32")]
pub mod services;
#[cfg(target_arch = "wasm32")]
mod wasm_app;

#[cfg(not(target_arch = "wasm32"))]
mod native_app;

use crate::models::pdf_text_item::PdfTextItem;

pub const APP_HEADING: &str = "WASM PDF Data Reader";
pub const APP_SUBHEADING: &str = "Render PDFs with pdf.js and inspect every detected word.";
pub const BROWSER_ONLY_MESSAGE: &str =
    "This application only renders PDFs when compiled to WebAssembly.";
pub const TOOLBAR_OPEN_FILE_LABEL: &str = "Open PDF";
pub const PDF_VIEWER_EMPTY_STATE: &str = "Open a PDF to render it in the main viewer.";
pub const WORD_LIST_EMPTY_STATE: &str = "Detected PDF words will appear in this panel.";
pub const WORD_LIST_TITLE: &str = "Detected Words";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordListEntry {
    pub id: String,
    pub page: u32,
    pub word: String,
}

pub fn build_word_list_entries(items: &[PdfTextItem]) -> Vec<WordListEntry> {
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let word = item.text.trim();

            if word.is_empty() {
                return None;
            }

            Some(WordListEntry {
                id: format!("page-{}-word-{index}", item.page),
                page: item.page,
                word: word.to_string(),
            })
        })
        .collect()
}

pub fn build_document_status(
    file_name: Option<&str>,
    total_pages: u32,
    total_words: usize,
) -> String {
    match file_name {
        Some(name) if !name.trim().is_empty() && total_pages > 0 => {
            format!("{name} · {total_pages} pages · {total_words} words")
        }
        Some(name) if !name.trim().is_empty() => format!("{name} · loading document"),
        _ => "No PDF loaded".to_string(),
    }
}

#[component]
pub fn App() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        return wasm_app::AppShell();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        native_app::AppShell()
    }
}
