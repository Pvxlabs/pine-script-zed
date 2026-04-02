use std::{collections::BTreeSet, fs};

use tree_sitter::{Parser, Query, QueryCursor};

fn query_capture_set(query_path: &str, fixture_path: &str, needle: &str) -> BTreeSet<String> {
    let source = fs::read_to_string(fixture_path).expect("fixture should exist");
    let start = source.find(needle).expect("needle should exist");
    let end = start + needle.len();

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_pine::language())
        .expect("pine grammar should load");
    let tree = parser.parse(&source, None).expect("tree should parse");

    let query_source = fs::read_to_string(query_path).expect("query should exist");
    let query =
        Query::new(&tree_sitter_pine::language(), &query_source).expect("query should compile");
    let capture_names = query.capture_names();
    let mut cursor = QueryCursor::new();

    cursor
        .matches(&query, tree.root_node(), source.as_bytes())
        .flat_map(|matches| matches.captures)
        .filter(|capture| capture.node.start_byte() <= end && capture.node.end_byte() >= start)
        .map(|capture| capture_names[capture.index as usize].to_string())
        .collect()
}

fn assert_capture(query_path: &str, fixture_path: &str, needle: &str, expected: &str) {
    let captures = query_capture_set(query_path, fixture_path, needle);
    assert!(
        captures.contains(expected),
        "{query_path} should capture {needle:?} as {expected}, got {captures:?}"
    );
}

#[test]
fn outline_query_compiles_and_captures_primary_symbols() {
    let query_path = "languages/pinescript/outline.scm";

    assert_capture(
        query_path,
        "tests/fixtures/v5_strategy_basic.pine",
        "strategy",
        "context",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v5_strategy_basic.pine",
        "V5 Strategy",
        "name",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v5_strategy_basic.pine",
        "strategy(\"V5 Strategy\")",
        "item",
    );

    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "Editor Queries",
        "name",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "sum_to",
        "name",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "type Signal",
        "annotation",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "Signal",
        "name",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "method scaled(float value, float factor) =>",
        "annotation",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "scaled",
        "name",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "var float global_seed = na",
        "item",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "global_seed",
        "name",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "trend_value",
        "name",
    );
}

#[test]
fn indent_query_compiles_and_marks_primary_block_structures() {
    let query_path = "languages/pinescript/indents.scm";

    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "sum_to(float value, int steps) =>",
        "indent",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "for i = 1 to steps",
        "indent",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "trend_value = if close > open",
        "indent",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "switch",
        "indent",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "total += i",
        "end",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "global_seed := close",
        "end",
    );
}

#[test]
fn brackets_query_compiles_and_marks_matching_pairs() {
    let query_path = "languages/pinescript/brackets.scm";

    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "library(\"Editor Queries\")",
        "open",
    );
    assert_capture(
        query_path,
        "tests/fixtures/v6_editor_queries.pine",
        "library(\"Editor Queries\")",
        "close",
    );
}
