use wasm_pdf_data_reader::{APP_HEADING, App, BROWSER_ONLY_MESSAGE};

#[test]
fn given_public_application_contract_when_loading_the_crate_then_integration_should_validate_the_basic_experience()
{
    let _app = App();

    assert_eq!(APP_HEADING, "Hello, Leptos!");
    assert_eq!(
        BROWSER_ONLY_MESSAGE,
        "This example only works when compiled to WebAssembly."
    );
    assert!(APP_HEADING.starts_with("Hello"));
    assert!(BROWSER_ONLY_MESSAGE.ends_with("WebAssembly."));
}
