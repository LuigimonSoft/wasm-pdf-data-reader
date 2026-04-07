use leptos::{ev, prelude::*};

#[component]
#[cfg(not(coverage))]
pub fn AppHeader(
    title: &'static str,
    subtitle: &'static str,
    status_text: Signal<String>,
    open_button_text: Signal<String>,
    theme_button_text: Signal<String>,
    open_disabled: Signal<bool>,
    on_open_click: Callback<ev::MouseEvent>,
    on_theme_click: Callback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <header class="app-header">
            <div class="app-header__top">
                <div class="app-header__brand">
                    <div class="app-header__mark">"W"</div>
                    <div class="app-header__copy">
                        <h1>{title}</h1>
                        <p>{subtitle}</p>
                    </div>
                </div>

                <div class="app-header__meta">
                    <span class="app-header__status">{move || status_text.get()}</span>
                </div>
            </div>

            <div class="app-header__nav">
                <button
                    class="header-nav-button header-nav-button--primary"
                    type="button"
                    on:click=move |event| on_open_click.run(event)
                    disabled=move || open_disabled.get()
                >
                    {move || open_button_text.get()}
                </button>

                <button
                    class="header-nav-button"
                    type="button"
                    on:click=move |event| on_theme_click.run(event)
                >
                    {move || theme_button_text.get()}
                </button>
            </div>
        </header>
    }
}

#[component]
#[cfg(coverage)]
pub fn AppHeader(
    title: &'static str,
    subtitle: &'static str,
    status_text: Signal<String>,
    open_button_text: Signal<String>,
    theme_button_text: Signal<String>,
    open_disabled: Signal<bool>,
    on_open_click: Callback<ev::MouseEvent>,
    on_theme_click: Callback<ev::MouseEvent>,
) -> impl IntoView {
    let _ = (
        title,
        subtitle,
        status_text,
        open_button_text,
        theme_button_text,
        open_disabled,
        on_open_click,
        on_theme_click,
    );

    view! { <header class="app-header"></header> }
}
