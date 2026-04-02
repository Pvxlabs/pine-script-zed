# Zed-Only Cleanup Design

**Date:** 2026-04-02
**Status:** Approved

## Goal

Remove VS Code-specific product artifacts from this repository so it reads and behaves as a Zed-only Pine Script language support extension.

## Scope

### In Scope

- Delete product files that only exist for the old VS Code/TextMate packaging flow:
  - `package.json`
  - `language-configuration.json`
  - `syntaxes/pinescript.tmLanguage.json`
  - `.vscodeignore`
  - `.vscode/launch.json`
- Rewrite `README.md` so it describes the project purely as a Zed extension.
- Add a regression test that guards against reintroducing those deleted VS Code product artifacts.

### Out of Scope

- `docs/superpowers/**` historical planning/spec files
- `grammars/pinescript/**` vendored tree-sitter grammar content used by the Zed extension
- New user-facing features or grammar behavior changes

## Design

### Product Artifact Cleanup

The repository root still contains legacy VS Code packaging and TextMate grammar files. They are not part of the current Zed extension runtime path and should be removed to keep the project identity and maintenance surface aligned with the actual product.

### README Reframing

The README should present the project as a standalone Zed extension, not as a migration from VS Code. It should keep the useful Zed-specific material:

- what the extension does
- how to install it in development
- how to build and verify it
- the current file layout

It should remove VS Code comparison and TextMate conversion framing.

### Regression Coverage

A lightweight test should assert that the deleted legacy product files are absent. This keeps future cleanup regressions visible in CI without depending on Zed GUI automation.

## Verification

- `cargo test`
- `cargo check`
- `rg` over product files and `README.md` to ensure no VS Code-specific residue remains in the cleaned scope
