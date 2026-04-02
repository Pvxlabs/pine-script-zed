; Indent Pine's block-producing structures. `@end` marks the contained block range when present.

((function_declaration_statement
   body: (block) @end) @indent)

((if_statement
   consequence: (block) @end) @indent)

((else_if_clause
   consequence: (block) @end) @indent)

((else_clause
   consequence: (block) @end) @indent)

((for_statement
   body: (block) @end) @indent)

((for_in_statement
   body: (block) @end) @indent)

((while_statement
   body: (block) @end) @indent)

((case_clause
   body: (block) @end) @indent)

(switch_statement) @indent

(type_definition_statement) @indent
