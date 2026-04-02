use std::{path::Path, process::Command};

fn git_ls_files(args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("ls-files")
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("git ls-files should run");
    assert!(
        output.status.success(),
        "git ls-files should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("git ls-files output should be utf8")
}

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
        let tracked = git_ls_files(&["--error-unmatch", path]);
        assert_eq!(tracked.trim(), path, "{path} should be tracked by git");
    }
}

#[test]
fn vendored_highlight_query_is_not_shipped() {
    let tracked_vendor_files = git_ls_files(&["vendor/tree-sitter-pine"]);

    assert!(
        !tracked_vendor_files.contains("vendor/tree-sitter-pine/queries/highlights.scm"),
        "vendored runtime highlights query should not exist"
    );
    assert!(
        !tracked_vendor_files.contains("vendor/tree-sitter-pine/bindings/node"),
        "vendored node bindings should not be tracked"
    );
    assert!(
        Path::new("languages/pinescript/highlights.scm").exists(),
        "extension highlight query should exist"
    );
}

fn read(path: &str) -> String {
    std::fs::read_to_string(path).expect("file should be readable")
}

#[test]
fn manifest_points_to_vendored_grammar() {
    let manifest = read("extension.toml");
    assert_eq!(
        manifest
            .matches("repository = \"https://github.com/Pvxlabs/pine-script-zed.git\"")
            .count(),
        2
    );
    assert!(manifest.contains("[grammars.pine]"));
    let grammar_section = manifest
        .split("[grammars.pine]")
        .nth(1)
        .expect("grammar section should exist");
    assert!(
        grammar_section.contains("repository = \"https://github.com/Pvxlabs/pine-script-zed.git\"")
    );
    assert!(grammar_section.contains("path = \"vendor/tree-sitter-pine\""));
}

#[test]
fn cargo_has_tree_sitter_dev_dependencies() {
    let cargo = read("Cargo.toml");
    assert!(cargo.contains("[dev-dependencies]"));
    assert!(cargo.contains("tree-sitter = "));
    assert!(cargo.contains("tree-sitter-pine = { path = \"vendor/tree-sitter-pine\" }"));
}

#[test]
fn manifest_rev_is_a_40_character_sha() {
    let manifest = read("extension.toml");
    let rev_line = manifest
        .lines()
        .find(|line| line.trim_start().starts_with("rev = "))
        .expect("rev line should exist");
    let rev = rev_line.split('"').nth(1).expect("rev should be quoted");
    assert_eq!(rev.len(), 40);
    assert!(rev.chars().all(|ch| ch.is_ascii_hexdigit()));
}
