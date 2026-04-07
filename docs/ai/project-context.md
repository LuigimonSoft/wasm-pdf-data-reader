# Project Context

## Product Goal

Build a browser-based PDF inspection application using Rust, Leptos, WebAssembly, and `pdf.js`.

The application must:

1. open a PDF file in the browser
2. render the PDF visibly to the user
3. create a visual overlay layer above the rendered PDF
4. draw one transparent rectangle with a red border for each detected word
5. store every detected word in an array of `PdfTextItem`

## Core Functional Flow

1. The user selects a PDF file from the UI.
2. The application loads the document through `pdf.js`.
3. Each page is rendered inside the browser.
4. Text extraction is performed per page using `pdf.js` text content APIs.
5. Every detected word is converted into a `PdfTextItem`.
6. A positioned overlay rectangle is drawn for each word using the word coordinates returned by the extraction pipeline.

## Main Technical Constraints

- Rust is the source of truth for application state and domain models.
- JavaScript is used only where browser APIs or `pdf.js` interop require it.
- `PdfTextItem` must remain the canonical typed representation of extracted word data.
- Overlay coordinates must stay aligned with the rendered PDF viewport.
- The overlay layer must remain visually transparent except for the red border around each word box.

## Suggested Architecture Direction

### UI

- a file upload control
- a PDF viewer container
- an overlay layer positioned on top of each rendered page

### Services

- a PDF loading service
- a bridge to `pdf.js` for rendering and text extraction
- a mapping layer that converts raw JS data into Rust `PdfTextItem` values

### Models

- `PdfTextItem` should represent at least the extracted text and the geometry needed to draw the rectangle

## Current Delivery Status

- Phase 1 is implemented: the placeholder UI has been replaced with a layout that includes a header, a main PDF viewing area, and a right-side word list panel.
- Phase 2 is implemented: `pdf.js` is integrated through the JS bridge and renders PDF pages inside the application without the native `pdf.js` viewer controls.
- Extracted text is already collected into `PdfTextItem` values and exposed in the sidebar as a word list.

## Near-Term Implementation Plan

1. Refine `PdfTextItem` for precise per-word geometry validation.
2. Render overlay rectangles above the PDF pages.
3. Synchronize overlay positioning with the rendered viewport on every page.
4. Expand unit and integration tests for the word overlay behavior.

## Definition Of Done

Work on this project is considered complete only when:

- a PDF can be selected and rendered
- each word is highlighted by a transparent red-bordered rectangle
- extracted words are available as `PdfTextItem` entries
- tests pass
- coverage stays at or above the required threshold described in the rules
