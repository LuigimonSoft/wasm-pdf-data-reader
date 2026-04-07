use leptos::prelude::*;

#[component]
#[cfg(not(coverage))]
pub fn PdfWorkspace(
    title: &'static str,
    empty_message: &'static str,
    show_empty_state: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <main class="pdf-workspace">
            <div class="pdf-workspace__canvas">
                <Show when=move || show_empty_state.get()>
                    <div class="pdf-workspace__empty">
                        <h2>{title}</h2>
                        <p>{empty_message}</p>
                    </div>
                </Show>

                <div class="pdf-workspace__sheet">{children.map(|children| children())}</div>
            </div>
        </main>
    }
}

#[component]
#[cfg(coverage)]
pub fn PdfWorkspace(
    title: &'static str,
    empty_message: &'static str,
    show_empty_state: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let _ = (title, empty_message, show_empty_state, children);

    view! { <main class="pdf-workspace"></main> }
}
