# Pine Script for Zed

Pine Script v6 language support for Zed, packaged as a language extension MVP for TradingView `.pine` and `.ps` files while retaining practical compatibility with Pine Script v5.

## Current Status

This repository currently focuses on a small, stable MVP:

- Language registration for `.pine` and `.ps`
- A vendored Tree-sitter grammar wired through [extension.toml](extension.toml)
- Syntax highlighting via `languages/pinescript/highlights.scm`
- Basic editor behavior: `//` line comments, soft tabs, bracket and quote auto-closing
- Parser and highlight tests against Pine Script v5 and v6 fixtures

The current MVP does not yet provide:

- Outline or document symbols
- Snippets
- LSP, diagnostics, completion, hover, or goto-definition
- AI-specific integration, prompts, or context tooling

## Runtime Sources

The runtime-relevant files in this repository are:

- `extension.toml`: extension manifest and pinned grammar reference
- `languages/pinescript/config.toml`: file associations and basic editor behavior
- `languages/pinescript/highlights.scm`: shipped highlight query
- `vendor/tree-sitter-pine`: vendored grammar baseline referenced by the manifest
- `extension.wasm`: generated artifact for local Zed loads, rebuilt from `target/wasm32-wasip2/release/pine_script_zed.wasm`

## Development Install

1. Clone this repository.
2. Build the extension wasm artifact:

   ```bash
   cargo build --release --target wasm32-wasip2
   cp target/wasm32-wasip2/release/pine_script_zed.wasm extension.wasm
   ```

3. Open Zed.
4. Run `Install Dev Extension` from the command palette.
5. Select the repository directory.
6. Restart Zed if the language support does not load immediately.

## Usage

Open any `.pine` or `.ps` file in Zed. The extension will apply Pine Script language configuration and syntax highlighting automatically.

## Development

### Prerequisites

- Rust toolchain
- Node.js and npm
- Zed editor

### Build

```bash
cargo build --release --target wasm32-wasip2
```

The Zed-loadable artifact is written to `target/wasm32-wasip2/release/pine_script_zed.wasm`. Copy it to `extension.wasm` at the repository root before reinstalling the dev extension in Zed.

### Verify

```bash
cargo check
cargo test
```

For manual verification, rebuild `extension.wasm`, reinstall the repository as a dev extension in Zed, and open files from `tests/fixtures/`.

### Grammar Regeneration

The vendored parser baseline lives in `vendor/tree-sitter-pine`.

```bash
npm --prefix vendor/tree-sitter-pine install
npm --prefix vendor/tree-sitter-pine run generate
```

## Project Structure

```text
.
├── extension.toml
├── extension.wasm
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
│       └── highlights.scm
├── tests/
│   ├── fixtures/
│   ├── ast_shape.rs
│   ├── fixture_parsing.rs
│   ├── grammar_manifest_invariants.rs
│   └── grammar_registration.rs
```

## Planned / Next

The next phase should deepen the current language-extension MVP rather than expand scope abruptly:

- add clearer query coverage for outline and indentation metadata
- expand fixture coverage for more Pine Script syntax edges
- evaluate LSP or diagnostics only after the current MVP is stable

See `docs/status.md` for a short status and roadmap note.

## Reporting Issues

When reporting a syntax-highlighting problem, include:

1. A minimal Pine Script snippet that reproduces the issue.
2. A screenshot from Zed.
3. The expected highlighting behavior.

## License

MIT. See [LICENSE](LICENSE).

## References

- [Pine Script Documentation](https://www.tradingview.com/pine-script-docs/)
- [Zed Extension Documentation](https://zed.dev/docs/extensions)
- [TradingView](https://www.tradingview.com/)
