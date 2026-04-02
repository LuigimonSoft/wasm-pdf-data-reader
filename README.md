# wasm-pdf-data-reader

A web application built with Rust, Leptos, and WebAssembly to open PDF files in the browser using `pdf.js`, render them visually, and overlay a transparent layer with a red border on top of each detected word.

In addition to the visual rendering, every extracted word must be stored as a typed `PdfTextItem`, so the system can work with both the visual representation and the structured text data.

## Main Idea

The goal of this project is to combine two views of the same PDF document:

- a visual view rendered in the browser
- a structured view of the detected text word by word

Each word in the document must have:

- its extracted text
- its position and dimensions inside the page
- a transparent rectangle with a red border drawn exactly on top of that word

This will make it possible to inspect PDFs precisely and establish the foundation for future features such as selection, analysis, visual validation, contextual search, or metadata export.

## Current Status

At this point, the repository includes:

- the Rust + Leptos + WASM project foundation
- an initial structure for services, models, bindings, and styles
- Codex documentation in [AGENTS.md](AGENTS.md) and `docs/ai/`
- working rules that require unit and integration tests to be validated after every modification
- a basic test suite adjusted to the current crate

The main PDF loading and rendering workflow is not implemented yet.

## Expected Functional Scope

The application is expected to support the following flow:

1. The user selects a PDF file from the interface.
2. `pdf.js` loads the document and renders its pages.
3. The text content is extracted page by page.
4. Each word is transformed into a `PdfTextItem`.
5. An overlay layer is drawn on top of the rendered page.
6. Each word receives a transparent box with a red border at its exact position.

## Proposed Architecture

### Frontend

- a Leptos interface for PDF selection and viewer rendering
- a visual container per page
- an overlay layer positioned on top of each rendered page

### PDF Integration

- a JavaScript bridge to `pdf.js`
- document loading
- per-page rendering
- text and coordinate extraction

### Rust Domain

- the `PdfTextItem` model as the main typed contract
- application state for the document, pages, and detected words
- services to transform JS data into Rust structures

## Implementation Plan

### Phase 1. Viewer Functional Base

- replace the current placeholder UI with an interface that includes a PDF selector
- connect the browser-side file loading flow
- validate that the document can be opened from the frontend

### Phase 2. `pdf.js` Integration

- complete the JS bridge for page loading and rendering
- define the data exchange between `pdf.js` and Rust
- ensure the rendering remains synchronized with the viewport

### Phase 3. Text Extraction and Model

- define or extend `PdfTextItem` with text, position, and dimensions
- extract words from `pdf.js`
- map each word into typed Rust structures

### Phase 4. Per-Word Visual Overlay

- build the overlay layer per page
- draw transparent rectangles with red borders
- validate precise alignment between detected text and the rendered PDF

### Phase 5. Quality and Testing

- expand unit tests for models, mappings, and transformation logic
- expand integration tests for the main application flow
- keep coverage at a minimum of 80% according to the project rules

## Quality Rules

- `PdfTextItem` must be the source of truth for detected words
- the `pdf.js` integration should remain encapsulated
- changes should stay small, verifiable, and tested
- unit and integration tests must be run after every modification

## Immediate Next Steps

1. Implement the UI to load a PDF.
2. Complete the bridge to `pdf.js`.
3. Model `PdfTextItem` correctly.
4. Render a first page with a temporary overlay.
5. Replace the temporary overlay with real per-word boxes.
