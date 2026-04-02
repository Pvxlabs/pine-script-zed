use std::{fs, path::Path};

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

#[test]
fn readme_is_zed_only() {
    let readme = fs::read_to_string("README.md").expect("README.md should be readable");

    for forbidden in [
        "VS Code",
        "VSCode",
        "vscode",
        "TextMate",
        "tmLanguage",
        "package.json",
        "language-configuration.json",
        ".vscodeignore",
        ".vscode/launch.json",
    ] {
        assert!(
            !readme.contains(forbidden),
            "README.md should not contain {forbidden}"
        );
    }
}
