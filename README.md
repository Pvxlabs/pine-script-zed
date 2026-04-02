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

## Current Status

This extension is currently an early-stage, stable MVP.

### Supported

- Language registration for `.pine` and `.ps`
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
- Parser and highlight tests against Pine Script v5 and v6 fixtures

### Not Yet Supported

- Snippets
- LSP features (diagnostics, completion, hover, goto-definition)
- Advanced query coverage (folds, locals, tags)
- AI-specific integration or tooling

---

## Runtime Sources

The runtime-relevant components are:

- `extension.toml` — extension manifest and grammar pin
- `languages/pinescript/config.toml` — file associations and editor behavior
- `languages/pinescript/highlights.scm` — syntax highlighting
- `languages/pinescript/outline.scm` — outline / symbols
- `languages/pinescript/indents.scm` — indentation rules
- `languages/pinescript/brackets.scm` — bracket matching
- `vendor/tree-sitter-pine` — vendored grammar baseline

---

## Build / Development

Zed runs the compiled `extension.wasm` artifact.

This file is a **build artifact**, not source code.

### Build

```bash
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/pine_script_zed.wasm extension.wasm

Notes
	•	Build target: wasm32-wasip2
	•	Output: target/wasm32-wasip2/release/pine_script_zed.wasm
	•	Local runtime artifact: extension.wasm
	•	extension.wasm is ignored by git and should not be committed

⸻

Development Install
	1.	Clone this repository
	2.	Build the extension:

cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/pine_script_zed.wasm extension.wasm


	3.	Open Zed
	4.	Run Install Dev Extension
	5.	Select this repository
	6.	Restart Zed if needed

⸻

Usage

Open any .pine or .ps file in Zed.

The extension will automatically apply:
	•	Pine Script syntax highlighting
	•	Language configuration
	•	Structural outline (symbols)

⸻

Verification

cargo check
cargo test

For manual verification:
	•	Rebuild extension.wasm
	•	Install as a dev extension in Zed
	•	Open files from tests/fixtures/

⸻

Grammar Regeneration

The vendored grammar lives in:

vendor/tree-sitter-pine

To regenerate:

npm --prefix vendor/tree-sitter-pine install
npm --prefix vendor/tree-sitter-pine run generate


⸻

Project Structure

.
├── extension.toml
├── Cargo.toml
├── docs/
│   └── status.md
├── src/
│   └── lib.rs
├── vendor/
│   └── tree-sitter-pine/
├── languages/
│   └── pinescript/
│       ├── config.toml
│       ├── highlights.scm
│       ├── outline.scm
│       ├── indents.scm
│       └── brackets.scm
├── tests/
│   ├── fixtures/
│   ├── editor_queries.rs
│   ├── fixture_parsing.rs
│   ├── grammar_manifest_invariants.rs
│   └── grammar_registration.rs


⸻

Planned / Next

Future improvements will focus on strengthening the current language support:
	•	Expand grammar coverage for Pine Script v6 edge cases
	•	Improve query accuracy and stability
	•	Add additional editor queries where grammar support is reliable
	•	Evaluate diagnostics / LSP integration once the grammar layer is stable

See docs/status.md for a short roadmap.

⸻

Reporting Issues

When reporting a problem, please include:
	1.	A minimal Pine Script snippet
	2.	A screenshot from Zed
	3.	Expected vs actual behavior

⸻

License

MIT. See LICENSE￼.

⸻

References
	•	Pine Script Docs: https://www.tradingview.com/pine-script-docs/
	•	Zed Extensions: https://zed.dev/docs/extensions
	•	TradingView: https://www.tradingview.com/

---
