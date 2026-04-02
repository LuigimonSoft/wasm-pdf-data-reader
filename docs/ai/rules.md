# Engineering Rules

These rules are written for Codex and must be treated as repository-specific operating instructions.

## General Rules

- Codex must follow this document before making or proposing code changes.
- Keep the codebase focused on the PDF rendering and word-overlay use case.
- Prefer simple, explicit boundaries between Rust UI/state code and JavaScript interop code.
- Avoid introducing hidden behavior, implicit global state, or tightly coupled modules.
- Keep naming aligned with the PDF domain: document, page, word, overlay, viewport, and `PdfTextItem`.
- Document non-obvious interop or coordinate-conversion logic close to the code.

## Implementation Rules

- Use `pdf.js` for PDF loading, page rendering, and text extraction.
- Keep raw `pdf.js` objects out of most Rust components; map them into typed Rust models quickly.
- Treat each word as an individual overlay target.
- Ensure the overlay rectangle is transparent with a red border.
- Preserve visual alignment between rendered pages and overlay boxes under resizing or viewport changes.
- Prefer incremental changes that keep the app buildable and testable.

## Testing Rules

- At the end of each modification, verify the unit tests and integration tests.
- If tests fail, correct them or create new tests as needed.
- Maintain test coverage at a minimum of 80%.
- Coverage must be checked with `cargo llvm-cov --all-features --workspace --cobertura --output-path coverage/cobertura.xml`.
- The total coverage must be read from `coverage/cobertura.xml`.
- If total coverage is below 80%, more tests must be created to cover the missing behavior until the minimum threshold is restored.
- Test method names must follow the format `given{scenario}_when{condition}_should{expectedbehavior}_then{expectedresult}`.
- Tests must include separator comments labeled `Given`, `When`, and `Then` to make the intent and flow explicit.
- Tests should use mocks to avoid relying on real positions or real coordinate sources.
- Do not leave feature work without automated validation for the affected behavior.
- Add tests for parsing, mapping, and rendering logic when the feature surface expands.

## Quality Rules

- Keep `PdfTextItem` as the authoritative model for extracted word data.
- Validate boundary cases such as empty PDFs, pages without text, and coordinate conversion issues.
- Prefer deterministic test inputs and stable rendering assertions where feasible.
- Do not remove existing coverage without replacing it with equivalent or stronger validation.

## Change Management Rules

- Update Codex-facing documentation when project architecture, workflow, or rules change materially.
- Keep changes small enough that failures can be isolated quickly.
- Before Codex closes a task, confirm that implementation, tests, and documentation remain consistent.
