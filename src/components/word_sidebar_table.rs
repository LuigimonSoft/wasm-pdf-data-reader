use leptos::prelude::*;

use crate::WordListEntry;

#[component]
pub fn WordSidebarTable(entries: ReadSignal<Vec<WordListEntry>>) -> impl IntoView {
    view! {
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
    }
}
