# Pine Script V6 Upgrade Design

**Date:** 2026-04-02
**Status:** Planning Ready
**Scope:** Upgrade this Zed extension from Pine Script v5-oriented support to Pine Script v6-oriented support while preserving practical compatibility with Pine Script v5.

## Goal

Make this repository the source of truth for Pine Script grammar support used by the Zed extension, with Pine Script v6 as the default target and Pine Script v5 retained as a compatibility target.

Success means:

- The extension no longer depends on the external `kvarenzn/tree-sitter-pine` v5-only grammar repository at runtime configuration level.
- The repository contains a maintained Pine grammar baseline that can parse representative Pine Script v6 files.
- Existing Pine Script v5 fixtures remain parseable and highlighted without regression.
- Highlight queries and documentation are updated to reflect Pine Script v6 as the default supported version.

## Current State

The repository currently consists of a small Zed extension shell:

- [`extension.toml`](/Users/tinzleung/github/pine-script-zed/extension.toml) registers the grammar named `pine` and points to `https://github.com/kvarenzn/tree-sitter-pine` at revision `b9e8bd4e69d6f98186604cca949cb807f8121b4e`.
- [`languages/pinescript/config.toml`](/Users/tinzleung/github/pine-script-zed/languages/pinescript/config.toml) defines basic Zed language behavior.
- [`languages/pinescript/highlights.scm`](/Users/tinzleung/github/pine-script-zed/languages/pinescript/highlights.scm) contains a hand-maintained list of built-in identifiers and query captures.
- [`tests/fixtures/basic.pine`](/Users/tinzleung/github/pine-script-zed/tests/fixtures/basic.pine) and [`tests/fixtures/complex.pine`](/Users/tinzleung/github/pine-script-zed/tests/fixtures/complex.pine) are both Pine Script v5 examples.

The external grammar repository currently identifies itself as a Pine Script v5 grammar. That makes the current extension structurally dependent on a parser baseline that predates Pine Script v6.

The repository's actual Git remote is `https://github.com/Pvxlabs/pine-script-zed.git`. That URL is the canonical repository URL for this design.

## Constraints

- This project is a Zed language extension, not a TradingView compiler or linter.
- The upgrade target is editor-grade support: parsing, syntax-aware highlighting, and language registration.
- The project should not attempt to implement Pine Script semantic validation beyond what the grammar and static highlighting queries can reasonably express.
- Compatibility with Pine Script v5 must remain practical, but Pine Script v6 becomes the default documented and tested version.

## Non-Goals

- Implementing TradingView compiler diagnostics.
- Implementing Pine Script language-server features such as completion, type inference, or symbol resolution.
- Perfectly modeling every runtime or semantic change introduced by Pine Script v6.
- Maintaining dependency on an external grammar repository that remains v5-only.

## Design Summary

The repository will vendor the upstream `tree-sitter-pine` grammar source at `vendor/tree-sitter-pine/` and treat it as the local grammar baseline. We will then evolve that local grammar for Pine Script v6 support, update the extension to reference that vendored grammar through Zed's grammar `path` support, and add a compatibility test matrix covering Pine Script v5, Pine Script v6, and migration-oriented samples.

This keeps all Pine support concerns in one repository:

- parser baseline
- Zed extension registration
- highlight queries
- fixtures and regression tests
- user-facing documentation

## Architecture

### 1. Local Grammar Ownership

Introduce a vendored grammar directory at `vendor/tree-sitter-pine/`.

The vendored grammar should initially be a faithful copy of the upstream baseline currently pinned by this repository: `https://github.com/kvarenzn/tree-sitter-pine` at revision `b9e8bd4e69d6f98186604cca949cb807f8121b4e`. From that seed revision onward, the grammar diverges in this repository as needed to support Pine Script v6 syntax. This avoids blocking on upstream v5-only maintenance and makes grammar changes reviewable alongside extension changes.

The repository will vendor the full upstream seed revision contents required for:

- Rust crate compilation
- grammar regeneration
- provenance retention
- Zed grammar loading

For the pinned upstream seed revision used by this design, the expected vendored baseline is:

- `vendor/tree-sitter-pine/Cargo.toml`
- `vendor/tree-sitter-pine/grammar.js`
- `vendor/tree-sitter-pine/package.json`
- `vendor/tree-sitter-pine/README.md`
- `vendor/tree-sitter-pine/UPSTREAM.md`
- `vendor/tree-sitter-pine/src/parser.c`
- `vendor/tree-sitter-pine/src/scanner.c`
- `vendor/tree-sitter-pine/src/grammar.json`
- `vendor/tree-sitter-pine/src/node-types.json`
- `vendor/tree-sitter-pine/bindings/rust/*`

The repository will not vendor `.git`, `node_modules`, build artifacts, or non-Rust language bindings from the upstream grammar repository. The pinned seed revision does not currently provide a separate `LICENSE` or `NOTICE` file, so provenance retention in this design is satisfied by preserving upstream `README.md`, `package.json`, and a repository-owned provenance file at `vendor/tree-sitter-pine/UPSTREAM.md`.

Vendored provenance is part of the design:

- `vendor/tree-sitter-pine/README.md` is retained from upstream
- `vendor/tree-sitter-pine/UPSTREAM.md` records the seed upstream repository URL and seed revision
- upstream package metadata, including license metadata in `package.json`, remains intact unless there is a deliberate reason to change it

Grammar regeneration ownership is explicit:

- Source of truth for grammar edits: `vendor/tree-sitter-pine/grammar.js` and any hand-maintained scanner code
- Generated artifacts that must be committed after grammar changes: `vendor/tree-sitter-pine/src/parser.c`, `vendor/tree-sitter-pine/src/grammar.json`, `vendor/tree-sitter-pine/src/node-types.json`
- Regeneration command owner: `npm --prefix vendor/tree-sitter-pine install` followed by `npm --prefix vendor/tree-sitter-pine run generate`

### 2. Extension Configuration

Update [`extension.toml`](/Users/tinzleung/github/pine-script-zed/extension.toml) so the extension no longer points to the external `kvarenzn/tree-sitter-pine` repository revision as its active grammar source.

The required wiring is:

- `repository` in [`extension.toml`](/Users/tinzleung/github/pine-script-zed/extension.toml) must be `https://github.com/Pvxlabs/pine-script-zed.git`
- `[grammars.pine].repository` must point to `https://github.com/Pvxlabs/pine-script-zed.git`
- `[grammars.pine].path` must be `vendor/tree-sitter-pine`
- `[grammars.pine].rev` must remain pinned to a specific commit for release builds

This makes grammar evolution part of this repository's change history rather than an opaque external pin while still using a Zed-supported grammar repository layout.

Zed's grammar subdirectory support is treated as a fixed architectural capability for this design, not an open feasibility question. The first implementation checkpoint only validates that this repository's manifest syntax and vendored path are wired correctly in practice. Failure at that checkpoint is treated as an implementation/configuration defect to fix within this architecture, not a reason to switch to a second repository.

Manifest acceptance rules are explicit:

- `repository` must equal `https://github.com/Pvxlabs/pine-script-zed.git`
- `[grammars.pine].repository` must equal `https://github.com/Pvxlabs/pine-script-zed.git`
- `[grammars.pine].path` must equal `vendor/tree-sitter-pine`
- `[grammars.pine].rev` must be a 40-character Git SHA in committed code, not a branch name or placeholder
- local dev installs use the checked-out `vendor/tree-sitter-pine` directory and do not depend on the committed `[grammars.pine].rev` value
- release workflow uses two commits:
  - commit A contains the vendored grammar, tests, and extension changes
  - commit B updates `[grammars.pine].rev` to point to commit A and is the release-ready manifest commit
- final release verification for commit B must compare `[grammars.pine].rev` with `git rev-parse HEAD~1`

### 3. Highlight Query Compatibility Layer

Extend [`languages/pinescript/highlights.scm`](/Users/tinzleung/github/pine-script-zed/languages/pinescript/highlights.scm) so it remains compatible with both v5 and v6 scripts while treating v6 as the primary vocabulary target.

[`languages/pinescript/highlights.scm`](/Users/tinzleung/github/pine-script-zed/languages/pinescript/highlights.scm) is the sole highlight-query source for this extension. The vendored grammar directory will not include `queries/highlights.scm`, specifically to avoid runtime ambiguity with Zed highlight loading.

This work includes:

- preserving existing captures that still apply to v5 and v6
- adding missing built-in namespaces, functions, constants, or common identifiers that appear in v6 code
- removing assumptions that are clearly outdated or too narrowly tied to v5 examples

Highlight queries should remain conservative: they only need to improve editor readability and avoid obvious mis-highlighting.

The minimum highlight acceptance matrix defined in this spec is also the full required highlight surface for this iteration. Additional built-in or namespace highlighting is out of scope unless required to satisfy those listed fixture assertions or to fix a contradiction introduced by the vendored grammar changes.

### 4. Compatibility Test Matrix

Replace the current implicit single-version fixture set with an explicit compatibility matrix:

- `tests/fixtures/v5_*.pine`
- `tests/fixtures/v6_*.pine`
- `tests/fixtures/migration_*.pine`
- `tests/fixtures/compat_*.pine`

Each category has a different purpose:

- `v5_*`: regression coverage for older scripts that users still open in Zed
- `v6_*`: default-version coverage for current Pine Script code
- `migration_*`: representative constructs from Pine's v5-to-v6 migration guidance that are structurally important for parsing and highlighting
- `compat_*`: header and forward-compatibility behavior that should not break parsing or highlighting

Rust tests should verify at minimum:

- grammar registration still matches the language config
- required vendored grammar assets exist in the repository
- fixture inventory is versioned and intentional
- documentation and examples are aligned with the supported-version statement
- Pine fixtures in the supported matrix parse without syntax errors
- the Zed highlight query compiles against the vendored grammar
- curated v5 and v6 token captures match expected highlight classes

Automated verification oracles are mandatory:

- Parse oracle: a Rust integration test uses the vendored Rust grammar crate from `vendor/tree-sitter-pine` to parse every supported fixture and fails if the syntax tree contains `ERROR` or missing nodes.
- Highlight oracle: a Rust integration test loads [`languages/pinescript/highlights.scm`](/Users/tinzleung/github/pine-script-zed/languages/pinescript/highlights.scm) as a Tree-sitter query against the vendored language and asserts that the capture set for each targeted token includes the expected capture listed in the acceptance matrix.
- Compatibility highlight oracle: the same Rust integration test must also assert at least one stable highlight capture from each `compat_*` fixture's script body, proving that headerless and future-version files remain highlightable as generic Pine source.
- AST-shape oracle: fixture-specific parser tests must assert key structural nodes for the v6-only cases. At minimum:
  - `v6_dynamic_requests.pine` must contain a `request.security(...)` call nested inside an `if_statement` body
  - `migration_v6_strategy.pine` must contain a `strategy.entry(...)` call nested inside an `if_statement` body rather than malformed argument or recovery nodes
- Highlight runtime-source oracle: a repository test must assert that [`languages/pinescript/highlights.scm`](/Users/tinzleung/github/pine-script-zed/languages/pinescript/highlights.scm) exists and that `vendor/tree-sitter-pine/queries/highlights.scm` does not exist in the committed extension assets.
- Manual Zed verification remains a smoke test only and does not replace the automated gates.

Cargo integration is explicit:

- [`Cargo.toml`](/Users/tinzleung/github/pine-script-zed/Cargo.toml) gains a `dev-dependencies` section
- `tree-sitter` is added under `dev-dependencies`
- `tree-sitter-pine = { path = "vendor/tree-sitter-pine" }` is added under `dev-dependencies`

The main crate keeps its current production dependency surface. The vendored grammar crate is used only for tests and verification.

The first Zed smoke-test checkpoint is concrete:

1. Install the repository as a dev extension in Zed.
2. Open `tests/fixtures/v6_smoke.pine`, a minimal fixture containing only `//@version=6`, `indicator("Smoke")`, and `plot(close)`.
3. Confirm the extension loads without grammar-load errors or query-compilation errors.
4. Confirm the file is recognized as Pine Script and that `indicator`, `plot`, and `close` receive syntax highlighting.

This checkpoint must pass before broader grammar edits continue.

Release-mode manifest verification is also required:

1. In the release-ready commit, inspect [`extension.toml`](/Users/tinzleung/github/pine-script-zed/extension.toml).
2. Confirm `[grammars.pine].repository`, `[grammars.pine].path`, and `[grammars.pine].rev` match the manifest acceptance rules.
3. Confirm `[grammars.pine].rev` points to an existing commit in the repository history chosen by the release workflow.

This is a manifest-level release gate rather than a local dev-install smoke test.

## Syntax Scope

The parser and editor support target is structural compatibility, not semantic equivalence with TradingView.

This iteration must cover the following required fixture surface:

| Fixture | Required constructs | Purpose |
|---------|---------------------|---------|
| `v6_smoke.pine` | `//@version=6`, `indicator("Smoke")`, `plot(close)` | earliest dev-install smoke test |
| `v5_indicator_basic.pine` | `//@version=5`, `indicator()`, namespaced `ta.*` calls, `plot()`, ternary expression, reassignment | v5 regression baseline |
| `v5_strategy_basic.pine` | `//@version=5`, `strategy()`, `strategy.entry()`, `input.*`, conditional blocks | v5 strategy regression baseline |
| `v6_indicator_basic.pine` | `//@version=6`, `indicator()`, `array.new_float(...)`, `color.new(...)`, `plot(close)` | v6 default indicator baseline |
| `v6_dynamic_requests.pine` | `//@version=6`, `if` block, `request.security(...)` call inside the block | v6 parsing coverage for dynamic request placement |
| `migration_v6_strategy.pine` | `//@version=6`, top-level `strategy()`, `if` block, `strategy.entry("L", strategy.long)` inside the block, no `when` argument, no `transp` argument | v6 migration-oriented strategy coverage |
| `compat_headerless.pine` | no `//@version` directive, otherwise valid Pine syntax | compatibility behavior for unspecified version |
| `compat_future_version.pine` | exact header `//@version=7`, otherwise valid Pine syntax | compatibility behavior for unknown future version markers |

Within those fixtures, the upgrade should prioritize syntax that affects parsing shape or common editor usage, including:

- Pine Script v6 file headers and top-level declarations
- control-flow constructs already supported by the grammar but exercised in v6 samples
- `request.*()` usage patterns in local scopes
- strategy call forms used after v6 migration away from `when`
- collection and namespaced built-in usage commonly seen in v6 scripts

Semantic migration changes that do not materially change parse structure should not block the upgrade. For example, the grammar does not need to reject semantically invalid v6 code if the code remains syntactically well-formed.

Explicitly out of scope for this iteration:

- semantic rejection of `transp`, `when`, implicit bool casts, or other v6 migration errors
- exact runtime behavior changes such as lazy boolean evaluation
- compiler-level validation of unsupported future Pine versions

Minimum highlight acceptance matrix:

| Fixture | Token or call | Expected capture |
|---------|---------------|------------------|
| `v6_smoke.pine` | `indicator` | `@function.builtin` |
| `v6_smoke.pine` | `plot` | `@function.builtin` |
| `v6_smoke.pine` | `close` | `@variable.builtin` |
| `v5_indicator_basic.pine` | `plot` | `@function.builtin` |
| `v5_indicator_basic.pine` | `close` | `@variable.builtin` |
| `v5_strategy_basic.pine` | `strategy` | `@function.builtin` |
| `v6_indicator_basic.pine` | `indicator` | `@function.builtin` |
| `v6_indicator_basic.pine` | `array` in `array.new_float(...)` | `@constant.builtin` |
| `v6_dynamic_requests.pine` | `request` | `@constant.builtin` |
| `migration_v6_strategy.pine` | `strategy` in `strategy("V6 Migration")` | `@function.builtin` |
| `migration_v6_strategy.pine` | `strategy` in `strategy.entry("L", strategy.long)` | `@constant.builtin` |
| `compat_headerless.pine` | `indicator` | `@function.builtin` |
| `compat_future_version.pine` | `indicator` | `@function.builtin` |

## Documentation Strategy

Update these documentation and example surfaces:

- [`README.md`](/Users/tinzleung/github/pine-script-zed/README.md)
- remove [`test.pine`](/Users/tinzleung/github/pine-script-zed/test.pine)

README changes must say:

- the extension defaults to Pine Script v6 support
- Pine Script v5 remains supported for compatibility
- the extension currently provides syntax highlighting and core editing behavior, not compiler-grade validation
- grammar regeneration for maintainers requires Node.js and npm because the vendored grammar uses `tree-sitter-cli` through `package.json`

Examples in the README and primary sample files should default to `//@version=6`. Compatibility examples for v5 should remain present but secondary.
The old fixture names [`tests/fixtures/basic.pine`](/Users/tinzleung/github/pine-script-zed/tests/fixtures/basic.pine) and [`tests/fixtures/complex.pine`](/Users/tinzleung/github/pine-script-zed/tests/fixtures/complex.pine) are transitional only and should be renamed or removed as part of the implementation so the supported fixture inventory is unambiguous.
[`README.md`](/Users/tinzleung/github/pine-script-zed/README.md)'s project-structure section must be updated to reflect the vendored grammar directory. [`test.pine`](/Users/tinzleung/github/pine-script-zed/test.pine) is removed so the maintained example surface lives only under the versioned fixture set.

## Compatibility Rules

Version-marker behavior is explicit:

- `//@version=5`: supported and covered by automated fixtures
- `//@version=6`: supported and covered by automated fixtures
- missing `//@version`: must remain parseable and highlightable as generic Pine source if the script body is otherwise valid
- unsupported future versions such as `//@version=7`: must remain parseable and highlightable as generic Pine source if the script body is otherwise valid, but they are not claimed as semantically supported Pine versions

Acceptance tests treat parser failure as:

- any fixture in the supported matrix producing `ERROR` nodes
- any fixture in the supported matrix producing missing nodes after parse
- any highlight-query compilation failure against the vendored grammar
- any curated highlight-capture assertion failure
- any required AST-shape assertion failure for the v6-only fixtures

## Error Handling and Risk Management

### Risk: Zed local grammar packaging details

There is some implementation risk around the exact `extension.toml` grammar wiring needed for a repository-owned grammar. This is acceptable because the design now fixes the intended mechanism and isolates the risk to one integration point.

Mitigation:

- Make grammar wiring the first implementation step.
- Verify with the smallest possible v6 fixture before broadening scope.

### Risk: Over-scoping into compiler behavior

Pine Script v6 migration docs include many semantic changes that do not belong in an editor grammar project.

Mitigation:

- Keep acceptance criteria at parser and highlighter level.
- Use migration fixtures only to confirm editor-grade structural support, not semantic enforcement.

### Risk: Highlight query drift

Built-in lists in highlight queries can become stale as Pine evolves.

Mitigation:

- Keep built-in additions targeted to identifiers actually used in fixtures and official migration examples.
- Prefer maintainable grouped query updates over one-off ad hoc additions.

## Testing Strategy

The implementation should follow a red-green workflow for each meaningful support change:

1. Add or update a fixture representing a v5, v6, or migration case.
2. Add the smallest test that fails because support is missing or configuration is incomplete.
3. Implement the smallest parser, query, or configuration change needed to make the test pass.
4. Re-run targeted tests, then the full Rust test suite.

Testing layers:

- unit-style Rust tests for manifest/config/resource invariants
- parser regression tests that load every supported fixture through `tree_sitter_pine::language()`
- AST-structure assertions for the required v6-only fixtures
- highlight query tests that compile [`languages/pinescript/highlights.scm`](/Users/tinzleung/github/pine-script-zed/languages/pinescript/highlights.scm) and assert curated captures
- manual verification in Zed using the fixture files once grammar wiring is complete

Test-unit boundaries are explicit:

- `tests/grammar_manifest_invariants.rs`: manifest wiring, grammar-path, runtime-source, and release-manifest checks
- `tests/fixture_parsing.rs`: parse oracle over all supported fixtures
- `tests/highlight_queries.rs`: highlight oracle and compatibility highlight assertions
- `tests/ast_shape.rs`: AST-shape assertions for `v6_dynamic_requests.pine` and `migration_v6_strategy.pine`

## Implementation Boundaries By File

Expected primary files and directories:

- Modify: [`extension.toml`](/Users/tinzleung/github/pine-script-zed/extension.toml)
- Modify: [`languages/pinescript/highlights.scm`](/Users/tinzleung/github/pine-script-zed/languages/pinescript/highlights.scm)
- Modify: [`README.md`](/Users/tinzleung/github/pine-script-zed/README.md)
- Modify: [`Cargo.toml`](/Users/tinzleung/github/pine-script-zed/Cargo.toml)
- Modify: [`tests/grammar_registration.rs`](/Users/tinzleung/github/pine-script-zed/tests/grammar_registration.rs)
- Modify: [`tests/no_vscode_artifacts.rs`](/Users/tinzleung/github/pine-script-zed/tests/no_vscode_artifacts.rs) to keep VS Code artifacts forbidden while allowing maintainer-facing references needed for the vendored grammar workflow
- Delete: [`test.pine`](/Users/tinzleung/github/pine-script-zed/test.pine)
- Create: `vendor/tree-sitter-pine/` with the owned grammar subset defined above
- Create: [`tests/grammar_manifest_invariants.rs`](/Users/tinzleung/github/pine-script-zed/tests/grammar_manifest_invariants.rs)
- Create: [`tests/fixture_parsing.rs`](/Users/tinzleung/github/pine-script-zed/tests/fixture_parsing.rs)
- Create: [`tests/highlight_queries.rs`](/Users/tinzleung/github/pine-script-zed/tests/highlight_queries.rs)
- Create: [`tests/ast_shape.rs`](/Users/tinzleung/github/pine-script-zed/tests/ast_shape.rs)
- Create: versioned fixture files under [`tests/fixtures`](/Users/tinzleung/github/pine-script-zed/tests/fixtures)

The Rust entry point in [`src/lib.rs`](/Users/tinzleung/github/pine-script-zed/src/lib.rs) is not expected to need functional changes for this upgrade.

## Recommended Execution Order

1. Vendor the current upstream Pine grammar into the repository.
2. Rewire the extension to use repository-owned grammar assets.
3. Add a minimal Pine Script v6 fixture and the first failing test proving the repository does not yet fully support the new structure.
4. Update the grammar incrementally until the v6 fixture parses.
5. Expand fixtures for v5 regression and migration coverage.
6. Update highlight queries to cover the final fixture set.
7. Update documentation and verification instructions.

## Acceptance Criteria

The work is complete when all of the following are true:

- The repository owns the Pine grammar baseline used by the extension at `vendor/tree-sitter-pine/`.
- [`extension.toml`](/Users/tinzleung/github/pine-script-zed/extension.toml) references that vendored grammar through Zed's grammar `path` support.
- Manifest acceptance rules pass for `repository`, `[grammars.pine].repository`, `[grammars.pine].path`, and `[grammars.pine].rev`.
- All required supported fixtures parse without `ERROR` or missing nodes.
- The required AST-shape assertions for `v6_dynamic_requests.pine` and `migration_v6_strategy.pine` pass.
- Curated highlight assertions for v5, v6, and `compat_*` fixtures pass.
- The highlight runtime-source oracle proves the extension uses [`languages/pinescript/highlights.scm`](/Users/tinzleung/github/pine-script-zed/languages/pinescript/highlights.scm) as its only committed highlight source.
- The first Zed smoke-test checkpoint passes on `tests/fixtures/v6_smoke.pine`.
- Representative Pine Script v6 fixtures are present and covered by automated tests.
- Representative Pine Script v5 fixtures still pass the same automated verification gates.
- `compat_*` fixtures prove that headerless and future-version scripts remain parseable and highlightable as generic Pine source.
- README and examples describe Pine Script v6 as the default supported version and Pine Script v5 as compatibility support.

## References

- TradingView Pine Script documentation welcome page: https://www.tradingview.com/pine-script-docs/welcome/
- TradingView migration guide to Pine Script version 6: https://www.tradingview.com/pine-script-docs/migration-guides/to-pine-version-6/
- Current external grammar baseline: https://github.com/kvarenzn/tree-sitter-pine
- Zed grammar subdirectory support discussion: https://github.com/zed-industries/zed/discussions/9901
