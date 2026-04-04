use leptos::prelude::*;

use crate::WordListEntry;

#[component]
pub fn WordSidebar(
    title: &'static str,
    empty_message: &'static str,
    total_items_text: Signal<String>,
    entries: Signal<Vec<WordListEntry>>,
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
                        <div class="word-sidebar__empty">
                            <p>{empty_message}</p>
                        </div>
                    }
                }
            >
                <div class="word-table">
                    <div class="word-table__head">
                        <span>"Page"</span>
                        <span>"Word"</span>
                    </div>

                    <ul class="word-list">
                        <For
                            each=move || entries.get()
                            key=|entry| entry.id.clone()
                            children=move |entry| {
                                view! {
                                    <li class="word-row">
                                        <span class="word-page">{format!("P{}", entry.page)}</span>
                                        <span class="word-copy">{entry.word}</span>
                                    </li>
                                }
                            }
                        />
                    </ul>
                </div>
            </Show>
        </aside>
    }
}
