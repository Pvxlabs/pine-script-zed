; Pine entry points such as indicator/strategy/library are the most useful top-level symbols.
(
  (call
    function: (identifier) @context
    arguments: (argument_list
      (string
        (double_quotted_string
          (string_content) @name))))
  (#any-of? @context "indicator" "strategy" "library")
) @item

((
  (call
    function: (identifier) @context
    arguments: (argument_list
      (string
        (single_quotted_string
          (string_content) @name))))
  (#any-of? @context "indicator" "strategy" "library")
) @item)

; User-defined functions.
(function_declaration_statement
  function: (identifier) @name) @item

; Methods use the `method` keyword and a distinct field in the grammar.
(function_declaration_statement
  "method" @annotation
  method: (identifier) @name) @item

; User-defined types.
(type_definition_statement
  "type" @annotation
  name: (identifier) @name) @item

; Top-level variable definitions assigned to expressions.
(source_file
  (simple_statement
    (variable_definition
      variable: (identifier) @name) @item))

; Top-level variable definitions assigned to structures such as if/switch.
(source_file
  (variable_definition_statement
    variable: (identifier) @name) @item)
