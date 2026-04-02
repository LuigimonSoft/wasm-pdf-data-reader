use leptos::prelude::*;

use crate::components::word_sidebar_empty::WordSidebarEmpty;
use crate::components::word_sidebar_table::WordSidebarTable;
use crate::WordListEntry;

#[component]
pub fn WordSidebar(
    title: &'static str,
    empty_message: &'static str,
    total_items_text: Signal<String>,
    entries: ReadSignal<Vec<WordListEntry>>,
) -> impl IntoView {
    view! {
        <aside class="word-sidebar">
            <div class="word-sidebar__header">
                <h2>{title}</h2>
                <span>{move || total_items_text.get()}</span>
            </div>

            <Show
                when=move || !entries.get().is_empty()
                fallback=move || {
                    view! {
                        <WordSidebarEmpty empty_message />
                    }
                }
            >
                <WordSidebarTable entries />
            </Show>
        </aside>
    }
}
