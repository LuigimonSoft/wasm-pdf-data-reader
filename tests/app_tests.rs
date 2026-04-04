#![allow(non_snake_case)]

use wasm_pdf_data_reader::{
    APP_HEADING, APP_SUBHEADING, App, BROWSER_ONLY_MESSAGE, TOOLBAR_OPEN_FILE_LABEL,
    WORD_LIST_EMPTY_STATE, build_document_status, build_word_list_entries,
    models::pdf_text_item::PdfTextItem,
};

fn mock_pdf_text_item(page: u32, text: &str, left: f64) -> PdfTextItem {
    PdfTextItem {
        page,
        text: text.to_string(),
        left,
        top: 24.0,
        width: 48.0,
        height: 12.0,
        transform: vec![1.0, 0.0, 0.0, 1.0, left, 24.0],
    }
}

#[test]
fn givenPublicApplicationComponent_whenConstructed_shouldExposeTheMainView_thenAppInstantiationSucceeds()
 {
    // Given
    let component_factory = App;

    // When
    let _app = component_factory();

    // Then
}

#[test]
fn givenPublicCopyConstants_whenRead_shouldDescribeThePdfWorkspace_thenValuesMatchTheExpectedContract()
 {
    // Given
    let expected_heading = "WASM PDF Data Reader";
    let expected_toolbar_label = "Open PDF";

    // When
    let heading = APP_HEADING;
    let subheading = APP_SUBHEADING;
    let toolbar_label = TOOLBAR_OPEN_FILE_LABEL;
    let browser_message = BROWSER_ONLY_MESSAGE;

    // Then
    assert_eq!(heading, expected_heading);
    assert_eq!(toolbar_label, expected_toolbar_label);
    assert!(subheading.contains("pdf.js"));
    assert!(browser_message.contains("WebAssembly"));
}

#[test]
fn givenMockPdfTextItems_whenBuildingWordListEntries_shouldFilterBlankWords_thenOnlyVisibleWordsRemain()
 {
    // Given
    let mock_items = vec![
        mock_pdf_text_item(1, "Invoice", 12.0),
        mock_pdf_text_item(1, " ", 32.0),
        mock_pdf_text_item(2, "Total", 48.0),
    ];

    // When
    let entries = build_word_list_entries(&mock_items);

    // Then
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].page, 1);
    assert_eq!(entries[0].word, "Invoice");
    assert_eq!(entries[1].page, 2);
    assert_eq!(entries[1].word, "Total");
}

#[test]
fn givenDocumentMetadata_whenBuildingDocumentStatus_shouldDescribeTheLoadedPdf_thenTheSummaryIncludesFilePagesAndWords()
 {
    // Given
    let file_name = Some("sample.pdf");
    let total_pages = 3;
    let total_words = 42;

    // When
    let status = build_document_status(file_name, total_pages, total_words);

    // Then
    assert_eq!(status, "sample.pdf · 3 pages · 42 words");
}

#[test]
fn givenNoLoadedPdf_whenBuildingDocumentStatus_shouldReturnTheEmptyState_thenTheUserSeesTheExpectedMessage()
 {
    // Given
    let file_name = None;

    // When
    let status = build_document_status(file_name, 0, 0);

    // Then
    assert_eq!(status, "No PDF loaded");
    assert!(WORD_LIST_EMPTY_STATE.contains("Detected PDF words"));
}
