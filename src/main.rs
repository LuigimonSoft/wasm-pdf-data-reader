#[cfg(target_arch = "wasm32")]
use leptos::mount::mount_to_body;
#[cfg(target_arch = "wasm32")]
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_pdf_data_reader::App;
#[cfg(not(target_arch = "wasm32"))]
use wasm_pdf_data_reader::BROWSER_ONLY_MESSAGE;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() {
    main();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("{BROWSER_ONLY_MESSAGE}");
}
