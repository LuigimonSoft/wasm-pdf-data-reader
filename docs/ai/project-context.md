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

## Near-Term Implementation Plan

1. Replace the current placeholder UI with a PDF upload and viewer workflow.
2. Integrate `pdf.js` through the existing JS bindings area.
3. Define or refine the `PdfTextItem` shape to hold text and word box coordinates.
4. Render overlay rectangles above the PDF.
5. Add or update unit and integration tests for the new behavior.

## Definition Of Done

Work on this project is considered complete only when:

- a PDF can be selected and rendered
- each word is highlighted by a transparent red-bordered rectangle
- extracted words are available as `PdfTextItem` entries
- tests pass
- coverage stays at or above the required threshold described in the rules
