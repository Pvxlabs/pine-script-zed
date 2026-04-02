use std::path::Path;

#[test]
fn vendored_grammar_baseline_exists() {
    for path in [
        "vendor/tree-sitter-pine/Cargo.toml",
        "vendor/tree-sitter-pine/grammar.js",
        "vendor/tree-sitter-pine/package.json",
        "vendor/tree-sitter-pine/README.md",
        "vendor/tree-sitter-pine/UPSTREAM.md",
        "vendor/tree-sitter-pine/src/parser.c",
        "vendor/tree-sitter-pine/src/scanner.c",
        "vendor/tree-sitter-pine/src/grammar.json",
        "vendor/tree-sitter-pine/src/node-types.json",
        "vendor/tree-sitter-pine/src/tree_sitter/alloc.h",
        "vendor/tree-sitter-pine/src/tree_sitter/array.h",
        "vendor/tree-sitter-pine/src/tree_sitter/parser.h",
        "vendor/tree-sitter-pine/bindings/rust/lib.rs",
        "vendor/tree-sitter-pine/bindings/rust/build.rs",
    ] {
        assert!(Path::new(path).exists(), "{path} should exist");
    }
}

#[test]
fn vendored_highlight_query_is_not_shipped() {
    assert!(
        !Path::new("vendor/tree-sitter-pine/queries/highlights.scm").exists(),
        "vendored runtime highlights query should not exist"
    );
    assert!(!Path::new("vendor/tree-sitter-pine/.git").exists());
    assert!(!Path::new("vendor/tree-sitter-pine/node_modules").exists());
    assert!(!Path::new("vendor/tree-sitter-pine/bindings/node").exists());
    assert!(
        Path::new("languages/pinescript/highlights.scm").exists(),
        "extension highlight query should exist"
    );
}
