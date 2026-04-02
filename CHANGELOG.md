# Changelog

All notable changes to the Pine Script Zed extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added
- Initial release of Pine Script support for Zed
- Syntax highlighting for Pine Script v5
- Support for keywords, functions, constants, operators, and variables
- Comment toggling (// line comments)
- Bracket auto-matching and auto-closing
- File type recognition for .pine files
- Test fixtures for basic and complex Pine Script code

### Features
- Highlights control flow keywords (if, else, for, while, etc.)
- Highlights built-in functions (plot, sma, ema, rsi, etc.)
- Highlights constants (true, false, na, open, close, etc.)
- Highlights operators (arithmetic, comparison, logical, assignment)
- Highlights strings (single and double quoted)
- Highlights comments

### Known Limitations
- No Tree-sitter grammar (planned for future release)
- No language server support (planned for future release)
- No code completion (planned for future release)

## Future Plans

### [0.2.0] - Tree-sitter Grammar
- Implement full Tree-sitter grammar for more precise parsing
- Improve error recovery
- Add code folding support

### [0.3.0] - Basic Language Server
- Code completion for keywords and built-in functions
- Function signature hints
- Basic error diagnostics

### [0.4.0] - Advanced Language Server
- Type checking
- Go to definition
- Find references
- Rename symbol
- Semantic error diagnostics
