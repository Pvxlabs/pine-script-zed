# Changelog

All notable changes to the Pine Script Zed extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Added `.ps` alongside `.pine` as a recognized file suffix for the language config
- Clarified that the project is a Pine Script for Zed MVP centered on language registration, vendored Tree-sitter wiring, syntax highlighting, and basic editor behavior
- Documented the wasm build flow and the runtime source-of-truth files used by the extension
- Tightened project invariants so the manifest `rev` must point to a reachable committed vendored grammar baseline instead of assuming `HEAD~1`

### Still Not Implemented
- Outline or symbols
- Snippets
- LSP, diagnostics, completion, hover, or goto-definition
- AI-specific integration

## [0.1.0] - 2026-04-02

### Added
- Initial Pine Script for Zed MVP
- Vendored Tree-sitter grammar baseline at `vendor/tree-sitter-pine`
- Syntax highlighting for Pine Script tokens, builtins, operators, comments, strings, and types
- Comment toggling with `//`
- Bracket auto-matching and auto-closing
- File type recognition for `.pine`
- Parser and highlighting fixtures for Pine Script v5 and v6 compatibility checks

### Known Limitations
- No outline or symbols
- No snippets
- No language server support
- No diagnostics or completion engine
- No AI-specific integration
