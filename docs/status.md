# Pine Script for Zed Status

## Current MVP Scope

This repository currently ships a Pine Script language extension MVP for Zed.

Implemented today:

- `.pine` and `.ps` file recognition
- Vendored Tree-sitter grammar integration via `vendor/tree-sitter-pine`
- Syntax highlighting through `languages/pinescript/highlights.scm`
- Basic editor behavior from `languages/pinescript/config.toml`
- Outline / symbols for entrypoints, functions, methods, types, and top-level variables
- Indentation and bracket-matching queries for common Pine block structures
- Parser, highlight, and manifest wiring tests

Not implemented yet:

- Snippets
- LSP, diagnostics, completion, or hover
- AI-specific integration
- Full query coverage such as folds, locals, or tags

## Runtime Source of Truth

- `extension.toml` pins the grammar source and revision
- `languages/pinescript/` contains the shipped language config and editor queries
- `vendor/tree-sitter-pine/` contains the vendored grammar baseline
- `extension.wasm` is a generated artifact, not a source-of-truth file

## Near-Term Roadmap

Phase 2 should stay incremental:

1. Keep grammar and manifest tests green while expanding fixture coverage.
2. Refine query coverage incrementally for folds, locals, or tags only when the grammar support is reliable.
3. Revisit diagnostics or LSP only after the current query-level editor support is stable and documented.
