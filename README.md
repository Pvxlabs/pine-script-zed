# Pine Script for Zed

Pine Script v6 language support for Zed, focused on syntax highlighting and core editing behavior for TradingView `.pine` files while retaining practical compatibility with Pine Script v5.

## Features

- Tree-sitter-based syntax highlighting for Pine Script
- `.pine` file recognition
- `//` line comments
- Bracket and quote auto-closing
- Pine Script v6 fixtures and parser coverage with Pine Script v5 compatibility fixtures

## Development Install

1. Clone this repository.
2. Open Zed.
3. Run `Install Dev Extension` from the command palette.
4. Select the repository directory.
5. Restart Zed if the language support does not load immediately.

## Usage

Open any `.pine` file in Zed. The extension will apply Pine Script language configuration and syntax highlighting automatically.

## Development

### Prerequisites

- Rust toolchain
- Node.js and npm
- Zed editor

### Build

```bash
cargo build --release
```

### Verify

```bash
cargo test
cargo check
```

For manual verification, reinstall the repository as a dev extension in Zed and open files from `tests/fixtures/`.

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
├── Cargo.toml
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
