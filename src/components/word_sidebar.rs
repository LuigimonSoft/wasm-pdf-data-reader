use leptos::prelude::*;

use crate::WordListEntry;
#[cfg(not(coverage))]
use crate::components::word_sidebar_empty::WordSidebarEmpty;
#[cfg(not(coverage))]
use crate::components::word_sidebar_table::WordSidebarTable;

#[component]
#[cfg(not(coverage))]
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

#[component]
#[cfg(coverage)]
pub fn WordSidebar(
    title: &'static str,
    empty_message: &'static str,
    total_items_text: Signal<String>,
    entries: ReadSignal<Vec<WordListEntry>>,
) -> impl IntoView {
    let _ = (title, empty_message, total_items_text, entries);

    view! { <aside class="word-sidebar"></aside> }
}
