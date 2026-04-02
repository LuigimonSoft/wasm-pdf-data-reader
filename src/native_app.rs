use leptos::prelude::*;

use crate::components::app_header::AppHeader;
use crate::components::pdf_workspace::PdfWorkspace;
use crate::components::word_sidebar::WordSidebar;
use crate::{
    APP_HEADING, APP_SUBHEADING, PDF_VIEWER_EMPTY_STATE, TOOLBAR_OPEN_FILE_LABEL,
    WORD_LIST_EMPTY_STATE, WORD_LIST_TITLE, WordListEntry,
};

#[component]
pub fn AppShell() -> impl IntoView {
    let (is_dark_theme, set_is_dark_theme) = signal(false);
    let (word_entries, _) = signal(Vec::<WordListEntry>::new());

    let toggle_theme = move |_| {
        set_is_dark_theme.update(|is_dark| *is_dark = !*is_dark);
    };

    let status_text = Signal::derive(|| {
        "Native preview mode. PDF rendering is enabled in WebAssembly builds.".to_string()
    });
    let open_button_text = Signal::derive(|| TOOLBAR_OPEN_FILE_LABEL.to_string());
    let theme_button_text = Signal::derive(move || {
        if is_dark_theme.get() {
            "Light theme".to_string()
        } else {
            "Dark theme".to_string()
        }
    });
    let open_disabled = Signal::derive(|| false);
    let word_items_text = Signal::derive(move || format!("{} items", word_entries.get().len()));

    view! {
        <div class="app-shell" class=("dark", move || is_dark_theme.get())>
            <div class="app-layout">
                <AppHeader
                    title=APP_HEADING
                    subtitle=APP_SUBHEADING
                    status_text
                    open_button_text
                    theme_button_text
                    open_disabled
                    on_open_click=Callback::new(|_| {})
                    on_theme_click=Callback::new(toggle_theme)
                />

                <div class="workspace-shell">
                    <PdfWorkspace
                        title="PDF Viewer"
                        empty_message=PDF_VIEWER_EMPTY_STATE
                        show_empty_state=Signal::derive(|| true)
                    />

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
