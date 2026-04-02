use std::{collections::BTreeSet, fs};

use tree_sitter::{Parser, Query, QueryCursor};

fn capture_set_for_text(path: &str, needle: &str) -> BTreeSet<String> {
    let source = fs::read_to_string(path).expect("fixture should exist");
    let start = source.find(needle).expect("needle should exist");
    let end = start + needle.len();

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_pine::language())
        .expect("pine grammar should load");
    let tree = parser.parse(&source, None).expect("tree should parse");

    let query_source =
        fs::read_to_string("languages/pinescript/highlights.scm").expect("query should exist");
    let query = Query::new(&tree_sitter_pine::language(), &query_source)
        .expect("highlight query should compile");
    let capture_names = query.capture_names();
    let mut cursor = QueryCursor::new();

    cursor
        .matches(&query, tree.root_node(), source.as_bytes())
        .flat_map(|matches| matches.captures)
        .filter(|capture| capture.node.start_byte() <= end && capture.node.end_byte() >= start)
        .map(|capture| capture_names[capture.index as usize].to_string())
        .collect()
}

fn assert_capture(path: &str, needle: &str, expected: &str) {
    let captures = capture_set_for_text(path, needle);
    assert!(
        captures.contains(expected),
        "{path} should capture {needle:?} as {expected}, got {captures:?}"
    );
}

#[test]
fn highlight_matrix_matches_spec() {
    assert_capture(
        "tests/fixtures/v6_smoke.pine",
        "indicator",
        "function.builtin",
    );
    assert_capture("tests/fixtures/v6_smoke.pine", "plot", "function.builtin");
    assert_capture("tests/fixtures/v6_smoke.pine", "close", "variable.builtin");
    assert_capture(
        "tests/fixtures/v5_indicator_basic.pine",
        "plot",
        "function.builtin",
    );
    assert_capture(
        "tests/fixtures/v5_indicator_basic.pine",
        "close",
        "variable.builtin",
    );
    assert_capture(
        "tests/fixtures/v5_strategy_basic.pine",
        "strategy",
        "function.builtin",
    );
    assert_capture(
        "tests/fixtures/v6_indicator_basic.pine",
        "indicator",
        "function.builtin",
    );
    assert_capture(
        "tests/fixtures/v6_indicator_basic.pine",
        "array",
        "constant.builtin",
    );
    assert_capture(
        "tests/fixtures/v6_dynamic_requests.pine",
        "request",
        "constant.builtin",
    );
    assert_capture(
        "tests/fixtures/migration_v6_strategy.pine",
        "strategy(\"V6 Migration\")",
        "function.builtin",
    );
    assert_capture(
        "tests/fixtures/migration_v6_strategy.pine",
        "strategy.entry",
        "constant.builtin",
    );
    assert_capture(
        "tests/fixtures/compat_headerless.pine",
        "indicator",
        "function.builtin",
    );
    assert_capture(
        "tests/fixtures/compat_future_version.pine",
        "indicator",
        "function.builtin",
    );
}
