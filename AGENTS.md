# AGENTS.md

This file is intended for Codex agents working in this repository.

## Purpose

This repository will host a WebAssembly application built with Rust and Leptos that opens PDF files in the browser through `pdf.js`, renders the document to the user, and overlays a transparent red-bordered rectangle on top of every detected word.

Each detected word must also be persisted as a Rust `PdfTextItem` entry so the application can expose structured text metadata in parallel with the visual overlay.

## Codex Operating Context

Before making changes, review these project documents:

- [Project Context](docs/ai/project-context.md)
- [Engineering Rules](docs/ai/rules.md)

## Expected Delivery Shape

Codex should prioritize:

- keeping the PDF rendering pipeline and word overlay pipeline clearly separated
- preserving a typed Rust model for every extracted word
- keeping JavaScript interop with `pdf.js` minimal and well-bounded
- expanding tests together with feature work instead of postponing validation

## Output Standard

When Codex implements work in this repository, prefer small, verifiable changes that keep the application in a runnable state after each modification.
