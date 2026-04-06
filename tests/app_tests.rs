#![allow(non_snake_case)]

use leptos::prelude::*;
use wasm_pdf_data_reader::{
    APP_HEADING, APP_SUBHEADING, App, BROWSER_ONLY_MESSAGE, TOOLBAR_OPEN_FILE_LABEL,
    WORD_LIST_EMPTY_STATE, WordListEntry, build_document_status, build_word_list_entries,
    components::word_sidebar::WordSidebar, components::word_sidebar_empty::WordSidebarEmpty,
    components::word_sidebar_table::WordSidebarTable,
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
fn givenTrimmedPdfTextItems_whenBuildingWordListEntries_shouldPreserveSequentialIds_thenTheSidebarCanCompareStableWords()
 {
    // Given
    let mock_items = vec![
        mock_pdf_text_item(1, "  Alpha  ", 12.0),
        mock_pdf_text_item(1, "\n", 24.0),
        mock_pdf_text_item(3, "Beta", 36.0),
    ];

    // When
    let entries = build_word_list_entries(&mock_items);

    // Then
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].id, "page-1-word-0");
    assert_eq!(entries[0].word, "Alpha");
    assert_eq!(entries[1].id, "page-3-word-2");
    assert_eq!(entries[1].word, "Beta");
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
fn givenSelectedPdfWithoutExtractedPages_whenBuildingDocumentStatus_shouldDescribeLoadingState_thenTheUserSeesProgressFeedback()
 {
    // Given
    let file_name = Some("sample.pdf");

    // When
    let status = build_document_status(file_name, 0, 0);

    // Then
    assert_eq!(status, "sample.pdf · loading document");
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

#[test]
fn givenBlankPdfName_whenBuildingDocumentStatus_shouldReturnTheEmptyState_thenWhitespaceNamesAreIgnored()
 {
    // Given
    let file_name = Some("   ");

    // When
    let status = build_document_status(file_name, 2, 14);

    // Then
    assert_eq!(status, "No PDF loaded");
}

#[test]
fn givenWordSidebarEmptyMessage_whenConstructed_shouldSupportEmptyPlaceholder_thenComponentInstantiationSucceeds()
 {
    // Given
    let empty_message = WORD_LIST_EMPTY_STATE;

    // When
    let _component = view! { <WordSidebarEmpty empty_message /> };

    // Then
}

#[test]
fn givenEmptyWordSidebarSignals_whenConstructed_shouldSupportTheEmptyState_thenComponentInstantiationSucceeds()
 {
    // Given
    let total_items_text = Signal::derive(|| "0 items".to_string());
    let (entries, _) = signal(Vec::<WordListEntry>::new());

    // When
    let _component = view! {
        <WordSidebar
            title="Detected Words"
            empty_message=WORD_LIST_EMPTY_STATE
            total_items_text
            entries
        />
    };

    // Then
}

#[test]
fn givenPopulatedWordSidebarSignals_whenConstructed_shouldSupportWordRows_thenComponentInstantiationSucceeds()
 {
    // Given
    let total_items_text = Signal::derive(|| "2 items".to_string());
    let populated_entries = vec![
        WordListEntry {
            id: "page-1-word-0".to_string(),
            page: 1,
            word: "Finanzas".to_string(),
        },
        WordListEntry {
            id: "page-1-word-1".to_string(),
            page: 1,
            word: "Marketing".to_string(),
        },
    ];
    let (entries, _) = signal(populated_entries);

    // When
    let _component = view! {
        <WordSidebar
            title="Detected Words"
            empty_message=WORD_LIST_EMPTY_STATE
            total_items_text
            entries
        />
    };

    // Then
}

#[test]
fn givenPopulatedWordSidebarTableEntries_whenConstructed_shouldSupportRenderingTheWordRows_thenComponentInstantiationSucceeds()
 {
    // Given
    let populated_entries = vec![
        WordListEntry {
            id: "page-2-word-0".to_string(),
            page: 2,
            word: "Contrato".to_string(),
        },
        WordListEntry {
            id: "page-2-word-1".to_string(),
            page: 2,
            word: "Firmado".to_string(),
        },
    ];
    let (entries, _) = signal(populated_entries);

    // When
    let _component = view! { <WordSidebarTable entries /> };

    // Then
}
