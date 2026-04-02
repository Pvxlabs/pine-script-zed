# Pine Script for Zed

Pine Script language support for Zed, targeting Pine Script v6 (with v5 compatibility).

Provides syntax highlighting, structural outline, and basic editing support for Pine Script in Zed.

---

## Language Support

This extension targets Pine Script v6 syntax, with backward compatibility for v5.

Note:
- The underlying grammar is still evolving and may not cover all edge cases.
- Some advanced or less common constructs may not yet be fully recognized.

---

## Features

- Syntax highlighting via Tree-sitter
- Structural outline / symbols:
  - `indicator(...)`
  - `strategy(...)`
  - `library(...)`
  - User-defined functions
  - `method`
  - `type`
  - Top-level variables
- Basic editor intelligence:
  - Indentation for common control structures (`if`, `for`, `while`, `switch`, etc.)
  - Bracket and quote matching
- Language support for `.pine` and `.ps` files

---

## Status

This extension is currently an early-stage, stable MVP.

### Not Yet Supported

- Snippets
- LSP features (diagnostics, completion, hover, goto-definition)
- Advanced query coverage (folds, locals, tags)
- AI-specific integration or tooling

---

## Usage

Open any `.pine` or `.ps` file in Zed.

The extension will automatically apply:
- Pine Script syntax highlighting
- Language configuration
- Structural outline (symbols)

---

## Development

Build the extension with:

```bash
cargo build --release --target wasm32-wasip2

Install in Zed via:
	•	Command palette → Install Dev Extension
	•	Select this repository
```
⸻

License

MIT. See LICENSE￼.

⸻

References
•Pine Script Docs: https://www.tradingview.com/pine-script-docs/
•Zed Extensions: https://zed.dev/docs/extensions

---
