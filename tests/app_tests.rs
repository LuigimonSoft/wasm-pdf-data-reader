use wasm_pdf_data_reader::{APP_HEADING, App, BROWSER_ONLY_MESSAGE};

#[test]
fn given_public_app_component_when_constructed_then_it_should_be_instantiable() {
    let _app = App();
}

#[test]
fn given_public_heading_constant_when_read_then_it_should_match_expected_title() {
    assert_eq!(APP_HEADING, "Hello, Leptos!");
}

#[test]
fn given_native_environment_when_requesting_browser_only_message_then_text_should_match_expected_copy()
 {
    assert_eq!(
        BROWSER_ONLY_MESSAGE,
        "This example only works when compiled to WebAssembly."
    );
}

#[test]
fn given_public_constants_when_read_together_then_they_should_define_the_basic_native_contract() {
    assert_ne!(APP_HEADING, BROWSER_ONLY_MESSAGE);
    assert!(APP_HEADING.contains("Leptos"));
    assert!(BROWSER_ONLY_MESSAGE.contains("WebAssembly"));
}
