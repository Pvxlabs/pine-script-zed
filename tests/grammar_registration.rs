use std::fs;

#[test]
fn extension_registers_the_tree_sitter_pine_grammar_name() {
    let extension_manifest =
        fs::read_to_string("extension.toml").expect("extension.toml should be readable");
    assert!(
        extension_manifest.contains("[grammars.pine]"),
        "extension.toml must register the tree-sitter grammar as `pine`"
    );
    assert!(
        extension_manifest.contains("path = \"vendor/tree-sitter-pine\""),
        "extension.toml must point the grammar at the vendored tree-sitter source"
    );

    let language_config = fs::read_to_string("languages/pinescript/config.toml")
        .expect("language config should be readable");
    assert!(
        language_config.contains("grammar = \"pine\""),
        "language config must reference the `pine` grammar name"
    );
}

#[test]
fn language_config_uses_zed_brackets_array_syntax() {
    let language_config = fs::read_to_string("languages/pinescript/config.toml")
        .expect("language config should be readable");

    assert!(
        language_config.contains("brackets = ["),
        "language config must define `brackets` as an array"
    );
    assert!(
        !language_config.contains("[brackets]"),
        "language config must not use a `[brackets]` table"
    );
    assert!(
        !language_config.contains("[auto_close]"),
        "language config must not use an `[auto_close]` table"
    );
}

#[test]
fn language_config_recognizes_ps_and_pine_suffixes() {
    let language_config = fs::read_to_string("languages/pinescript/config.toml")
        .expect("language config should be readable");

    assert!(
        language_config.contains("path_suffixes = [\"pine\", \"ps\"]"),
        "language config must recognize both `.pine` and `.ps` files"
    );
}
