# Pine Script for Zed Status

## Current MVP Scope

This repository currently ships a Pine Script language extension MVP for Zed.

Implemented today:

- `.pine` and `.ps` file recognition
- Vendored Tree-sitter grammar integration via `vendor/tree-sitter-pine`
- Syntax highlighting through `languages/pinescript/highlights.scm`
- Basic editor behavior from `languages/pinescript/config.toml`
- Parser, highlight, and manifest wiring tests

Not implemented yet:

- Outline or symbols
- Snippets
- LSP, diagnostics, completion, or hover
- AI-specific integration

## Runtime Source of Truth

- `extension.toml` pins the grammar source and revision
- `languages/pinescript/` contains the shipped language config and highlight query
- `vendor/tree-sitter-pine/` contains the vendored grammar baseline
- `extension.wasm` is a generated artifact, not a source-of-truth file

## Near-Term Roadmap

Phase 2 should stay incremental:

1. Keep grammar and manifest tests green while expanding fixture coverage.
2. Add query-level editor metadata such as outline or indentation support.
3. Revisit diagnostics or LSP only after the current MVP is stable and documented.
