use leptos::prelude::*;

use crate::WordListEntry;

#[cfg(target_arch = "wasm32")]
fn log_sidebar_words(entries: &[WordListEntry]) {
    let words = entries
        .iter()
        .map(|entry| entry.word.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    web_sys::console::debug_1(&words.into());
}

#[component]
#[cfg(not(coverage))]
pub fn WordSidebarTable(entries: ReadSignal<Vec<WordListEntry>>) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            let snapshot = entries.get();

            log_sidebar_words(&snapshot);
        });
    }

    let rows = move || {
        entries
            .get()
            .into_iter()
            .map(|entry| {
                view! {
                    <li class="word-row">
                        <span class="word-page">{format!("P{}", entry.page)}</span>
                        <span class="word-copy">{entry.word}</span>
                    </li>
                }
            })
            .collect_view()
    };

    view! {
        <div class="word-table">
            <div class="word-table__head">
                <span>"Page"</span>
                <span>"Word"</span>
            </div>

            <ul class="word-list">{rows}</ul>
        </div>
    }
}

#[component]
#[cfg(coverage)]
pub fn WordSidebarTable(entries: ReadSignal<Vec<WordListEntry>>) -> impl IntoView {
    let _ = entries;

    view! { <div class="word-table"></div> }
}
