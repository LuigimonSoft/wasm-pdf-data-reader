use leptos::prelude::*;

pub const APP_HEADING: &str = "Hello, Leptos!";
pub const BROWSER_ONLY_MESSAGE: &str = "This example only works when compiled to WebAssembly.";

#[component]
pub fn App() -> impl IntoView {
    view! {
      <div>
        <h1>{APP_HEADING}</h1>
      </div>
    }
}
