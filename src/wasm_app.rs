use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::components::app_header::AppHeader;
use crate::components::pdf_workspace::PdfWorkspace;
use crate::components::word_sidebar::WordSidebar;
use crate::services::pdf_service::process_pdf;
use crate::{
    APP_HEADING, APP_SUBHEADING, PDF_VIEWER_EMPTY_STATE, TOOLBAR_OPEN_FILE_LABEL,
    WORD_LIST_EMPTY_STATE, WORD_LIST_TITLE, build_document_status, build_word_list_entries,
};

fn get_html_input_by_id(id: &str) -> Option<web_sys::HtmlInputElement> {
    let document = web_sys::window()?.document()?;
    let element = document.get_element_by_id(id)?;

    element.dyn_into::<web_sys::HtmlInputElement>().ok()
}

fn get_html_host_by_id(id: &str) -> Option<web_sys::HtmlElement> {
    let document = web_sys::window()?.document()?;
    let element = document.get_element_by_id(id)?;

    element.dyn_into::<web_sys::HtmlElement>().ok()
}

fn is_dark_theme_preferred() -> bool {
    web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
        })
        .map(|media| media.matches())
        .unwrap_or(false)
}

#[component]
pub fn AppShell() -> impl IntoView {
    let (file_name, set_file_name) = signal::<Option<String>>(None);
    let (total_pages, set_total_pages) = signal(0_u32);
    let (pdf_items, set_pdf_items) = signal(Vec::new());
    let word_entries = Signal::derive(move || build_word_list_entries(&pdf_items.get()));
    let (is_loading, set_is_loading) = signal(false);
    let (error_message, set_error_message) = signal::<Option<String>>(None);
    let (is_dark_theme, set_is_dark_theme) = signal(is_dark_theme_preferred());

    let toggle_theme = move |_| {
        set_is_dark_theme.update(|is_dark| *is_dark = !*is_dark);
    };

    let status_text = Signal::derive(move || {
        build_document_status(
            file_name.get().as_deref(),
            total_pages.get(),
            word_entries.get().len(),
        )
    });

    let open_button_text = Signal::derive(move || {
        if is_loading.get() {
            "Loading PDF...".to_string()
        } else {
            TOOLBAR_OPEN_FILE_LABEL.to_string()
        }
    });

    let theme_button_text = Signal::derive(move || {
        if is_dark_theme.get() {
            "Light theme".to_string()
        } else {
            "Dark theme".to_string()
        }
    });

    let open_disabled = Signal::derive(move || is_loading.get());
    let word_items_text = Signal::derive(move || format!("{} items", word_entries.get().len()));

    let open_file_picker = move |_| {
        if let Some(input) = get_html_input_by_id("pdf-file-input") {
            input.click();
        }
    };

    let load_pdf_file = move |event: ev::Event| {
        let input = event_target::<web_sys::HtmlInputElement>(&event);
        let files = input.files();
        let Some(files) = files else {
            return;
        };

        let Some(file) = files.get(0) else {
            return;
        };

        let Some(viewer_host) = get_html_host_by_id("pdf-viewer-host") else {
            set_error_message.set(Some(
                "The PDF viewer container is not available.".to_string(),
            ));
            return;
        };

        let selected_file_name = file.name();

        set_is_loading.set(true);
        set_error_message.set(None);
        set_file_name.set(Some(selected_file_name.clone()));
        set_total_pages.set(0);
        set_pdf_items.set(Vec::new());

        spawn_local(async move {
            match process_pdf(file, viewer_host).await {
                Ok(result) => {
                    set_total_pages.set(result.total_pages);
                    set_pdf_items.set(result.items);
                }
                Err(error) => {
                    let message = error
                        .as_string()
                        .unwrap_or_else(|| "Unable to render the selected PDF.".to_string());

                    set_error_message.set(Some(message));
                    set_total_pages.set(0);
                    set_pdf_items.set(Vec::new());
                }
            }

            set_is_loading.set(false);
        });
    };

    view! {
        <div class="app-shell" class=("dark", move || is_dark_theme.get())>
            <div class="app-layout">
                <input
                    id="pdf-file-input"
                    class="sr-only"
                    type="file"
                    accept="application/pdf"
                    on:change=load_pdf_file
                />

                <AppHeader
                    title=APP_HEADING
                    subtitle=APP_SUBHEADING
                    status_text
                    open_button_text
                    theme_button_text
                    open_disabled
                    on_open_click=Callback::new(open_file_picker)
                    on_theme_click=Callback::new(toggle_theme)
                />

                <div class="workspace-shell">
                    <PdfWorkspace
                        title="PDF Viewer"
                        empty_message=PDF_VIEWER_EMPTY_STATE
                        show_empty_state=Signal::derive(move || file_name.get().is_none())
                    >
                        <div id="pdf-viewer-host" class="pdf-viewer-host"></div>

                        <Show when=move || error_message.get().is_some()>
                            <div class="error-banner">
                                {move || error_message.get().unwrap_or_default()}
                            </div>
                        </Show>
                    </PdfWorkspace>

                    <WordSidebar
                        title=WORD_LIST_TITLE
                        empty_message=WORD_LIST_EMPTY_STATE
                        total_items_text=word_items_text
                        entries=word_entries
                    />
                </div>
            </div>
        </div>
    }
}
