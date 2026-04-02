# Zed-Only Cleanup Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove VS Code-only product artifacts and rewrite the README so the repository is clearly a Zed-only extension.

**Architecture:** The cleanup is limited to repository metadata and documentation. We will add a small regression test first, then remove the obsolete VS Code/TextMate files, then rewrite the README to describe the current Zed extension structure and workflow.

**Tech Stack:** Rust, Zed extension manifest/config files, Markdown documentation

---

## Chunk 1: Product Cleanup

### Task 1: Guard and remove legacy VS Code product artifacts

**Files:**
- Create: `tests/no_vscode_artifacts.rs`
- Delete: `package.json`
- Delete: `language-configuration.json`
- Delete: `syntaxes/pinescript.tmLanguage.json`
- Delete: `.vscodeignore`
- Delete: `.vscode/launch.json`
- Delete if empty: `.vscode/`
- Delete if empty: `syntaxes/`

- [ ] **Step 1: Write the failing test**

```rust
use std::path::Path;

#[test]
fn vscode_product_artifacts_are_absent() {
    for path in [
        "package.json",
        "language-configuration.json",
        "syntaxes/pinescript.tmLanguage.json",
        ".vscodeignore",
        ".vscode/launch.json",
    ] {
        assert!(
            !Path::new(path).exists(),
            "{path} should not exist in the Zed-only extension"
        );
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test vscode_product_artifacts_are_absent`
Expected: FAIL because the legacy files still exist

- [ ] **Step 3: Delete the legacy files**

Remove the obsolete VS Code/TextMate packaging files listed above. Do not touch `grammars/pinescript/**`.
If `.vscode/` becomes empty after deleting `.vscode/launch.json`, remove the empty directory too.
If `syntaxes/` becomes empty after deleting `syntaxes/pinescript.tmLanguage.json`, remove the empty directory too.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test vscode_product_artifacts_are_absent`
Expected: PASS

## Chunk 2: README Cleanup

### Task 2: Rewrite README as a Zed-only project document

**Files:**
- Modify: `README.md`
- Reference: `extension.toml`
- Reference: `languages/pinescript/config.toml`

- [ ] **Step 1: Write the failing test**

Extend the regression test so it checks that `README.md` does not contain:
- `VS Code`
- `VSCode`
- `vscode`
- `TextMate`
- `tmLanguage`
- `package.json`
- `language-configuration.json`

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL because the current README still references VS Code/TextMate history

- [ ] **Step 3: Rewrite the README**

Update the README to:
- describe the extension as Zed-only
- document development install and verification for Zed
- show the current project structure that actually exists
- remove VS Code comparison/migration framing

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test`
Expected: PASS

### Task 3: Final verification

**Files:**
- Verify: `README.md`
- Verify: `tests/no_vscode_artifacts.rs`
- Verify: `extension.toml`
- Verify: `languages/pinescript/config.toml`

- [ ] **Step 1: Run the full verification suite**

Run: `cargo test`
Expected: PASS

- [ ] **Step 2: Run the build verification**

Run: `cargo check`
Expected: PASS

- [ ] **Step 3: Search for residue in the approved scope**

Run: `rg -n "VS Code|VSCode|vscode|TextMate|tmLanguage|package\\.json|language-configuration\\.json" README.md extension.toml languages`
Expected: no matches
