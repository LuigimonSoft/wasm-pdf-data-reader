use leptos::prelude::*;

#[component]
pub fn WordSidebarEmpty(empty_message: &'static str) -> impl IntoView {
    view! {
        <div class="word-sidebar__empty">
            <p>{empty_message}</p>
        </div>
    }
}
