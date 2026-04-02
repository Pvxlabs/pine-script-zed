use std::fs;

use tree_sitter::{Node, Parser};

fn assert_clean_tree(node: Node) {
    assert!(!node.is_error(), "unexpected ERROR node: {}", node.kind());
    assert!(
        !node.is_missing(),
        "unexpected missing node: {}",
        node.kind()
    );

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        assert_clean_tree(child);
    }
}

const FIXTURES: &[&str] = &[
    "tests/fixtures/v6_smoke.pine",
    "tests/fixtures/v5_indicator_basic.pine",
    "tests/fixtures/v5_strategy_basic.pine",
    "tests/fixtures/v6_indicator_basic.pine",
    "tests/fixtures/v6_dynamic_requests.pine",
    "tests/fixtures/migration_v6_strategy.pine",
    "tests/fixtures/compat_headerless.pine",
    "tests/fixtures/compat_future_version.pine",
];

#[test]
fn parse_v6_smoke_without_errors() {
    let source = fs::read_to_string("tests/fixtures/v6_smoke.pine").expect("fixture should exist");
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_pine::language())
        .expect("pine grammar should load");
    let tree = parser.parse(&source, None).expect("tree should parse");
    assert_clean_tree(tree.root_node());
}

#[test]
fn parse_all_supported_fixtures_without_errors() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_pine::language())
        .expect("pine grammar should load");

    for path in FIXTURES {
        let source = fs::read_to_string(path).expect("fixture should exist");
        let tree = parser.parse(&source, None).expect("tree should parse");
        assert_clean_tree(tree.root_node());
    }
}

#[test]
fn fixture_inventory_is_versioned() {
    assert!(
        !std::path::Path::new("tests/fixtures/basic.pine").exists(),
        "legacy basic fixture should be removed"
    );
    assert!(
        !std::path::Path::new("tests/fixtures/complex.pine").exists(),
        "legacy complex fixture should be removed"
    );
}
