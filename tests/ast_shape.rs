use std::fs;

use tree_sitter::{Node, Parser, Tree};

fn parse_file(source: &str) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_pine::language())
        .expect("pine grammar should load");
    parser.parse(source, None).expect("tree should parse")
}

fn subtree_has_call_text(node: Node, source: &str, needle: &str) -> bool {
    if node.kind() == "call" && source[node.byte_range()].contains(needle) {
        return true;
    }

    let mut cursor = node.walk();
    let has_match = node
        .children(&mut cursor)
        .any(|child| subtree_has_call_text(child, source, needle));
    has_match
}

fn if_statement_body_contains_call(path: &str, needle: &str) {
    let source = fs::read_to_string(path).expect("fixture should exist");
    let tree = parse_file(&source);
    let root = tree.root_node();
    let mut cursor = root.walk();
    let if_nodes: Vec<_> = root
        .children(&mut cursor)
        .filter(|node| node.kind() == "if_statement")
        .collect();
    assert!(
        !if_nodes.is_empty(),
        "{path} should contain an if_statement"
    );
    assert!(
        if_nodes
            .into_iter()
            .filter_map(|node| node.child_by_field_name("consequence"))
            .any(|suite| subtree_has_call_text(suite, &source, needle)),
        "{path} should contain a call node for {needle} inside the if_statement body"
    );
}

#[test]
fn dynamic_request_is_nested_in_if_statement() {
    if_statement_body_contains_call(
        "tests/fixtures/v6_dynamic_requests.pine",
        "request.security(",
    );
}

#[test]
fn migration_strategy_entry_is_nested_in_if_statement() {
    if_statement_body_contains_call(
        "tests/fixtures/migration_v6_strategy.pine",
        "strategy.entry(\"L\", strategy.long)",
    );
}
