#![allow(non_snake_case)]

use wasm_pdf_data_reader::{
    APP_HEADING, App, BROWSER_ONLY_MESSAGE, PDF_VIEWER_EMPTY_STATE, WORD_LIST_TITLE,
};

#[test]
fn givenPublicApplicationContract_whenLoadingTheCrate_shouldDescribeThePdfReaderWorkspace_thenTheCoreContractRemainsConsistent()
 {
    // Given

    // When
    let _app = App();

    // Then
    assert_eq!(APP_HEADING, "WASM PDF Data Reader");
    assert_eq!(
        BROWSER_ONLY_MESSAGE,
        "This application only renders PDFs when compiled to WebAssembly."
    );
    assert!(PDF_VIEWER_EMPTY_STATE.contains("render"));
    assert_eq!(WORD_LIST_TITLE, "Detected Words");
    assert!(BROWSER_ONLY_MESSAGE.ends_with("WebAssembly."));
}

#[test]
fn givenNativeBinaryBuild_whenExecutingTheApplication_shouldPrintTheBrowserOnlyMessage_thenTheCliContractRemainsConsistent()
 {
    // Given
    let binary_path = env!("CARGO_BIN_EXE_wasm-pdf-data-reader");

    // When
    let output = std::process::Command::new(binary_path)
        .output()
        .expect("native binary should execute successfully");

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");

    // Then
    assert!(output.status.success());
    assert_eq!(stdout.trim(), BROWSER_ONLY_MESSAGE);
}
