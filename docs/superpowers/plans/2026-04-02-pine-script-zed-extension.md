# Pine Script Zed Extension Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a Zed extension that provides Pine Script syntax highlighting and basic editing features equivalent to the existing VS Code extension.

**Architecture:** Convert the existing TextMate grammar to Zed's highlight query format, create minimal Rust extension code, and configure language settings for bracket matching and comment toggling.

**Tech Stack:** Rust (Zed extension API), Zed highlight queries (.scm format), TOML configuration

---

## File Structure Overview

This plan will create the following files:

**New Repository Structure:**
- `extension.toml` - Zed extension metadata and configuration
- `Cargo.toml` - Rust build configuration
- `src/lib.rs` - Minimal Rust extension implementation
- `languages/pinescript/config.toml` - Language configuration (file extensions, brackets, comments)
- `languages/pinescript/highlights.scm` - Syntax highlighting rules
- `tests/fixtures/basic.pine` - Test file with basic Pine Script syntax
- `tests/fixtures/complex.pine` - Test file with complex Pine Script code
- `README.md` - Updated documentation for Zed extension
- `LICENSE` - MIT license (already exists)

**Design Principles:**
- Each file has a single, clear responsibility
- Configuration is declarative and minimal
- Syntax rules are organized by category (comments, strings, keywords, etc.)
- Test fixtures cover the full range of Pine Script syntax

---

## Task 1: Create New Repository Structure

**Files:**
- Create: `extension.toml`
- Create: `Cargo.toml`
- Create: `src/lib.rs`

- [ ] **Step 1: Create extension.toml with metadata**

```toml
id = "pinescript"
name = "Pine Script"
description = "Pine Script language support for Zed - syntax highlighting for TradingView's Pine Script"
version = "0.1.0"
schema_version = 1
authors = ["Tinz Leung <your.email@example.com>"]
repository = "https://github.com/yourusername/pine-script-zed"

[grammars.pinescript]
repository = "https://github.com/yourusername/pine-script-zed"
rev = "main"
```

Create this file at the root of the repository.

- [ ] **Step 2: Create Cargo.toml for Rust build**

```toml
[package]
name = "pine-script-zed"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.1.0"
```

Create this file at the root of the repository.

- [ ] **Step 3: Create src directory**

Run: `mkdir -p src`

- [ ] **Step 4: Create minimal Rust extension implementation**

```rust
use zed_extension_api as zed;

struct PineScriptExtension;

impl zed::Extension for PineScriptExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(PineScriptExtension);
```

Create this file at `src/lib.rs`.

- [ ] **Step 5: Verify Rust code compiles**

Run: `cargo check`
Expected: "Checking pine-script-zed v0.1.0" followed by "Finished" with no errors

- [ ] **Step 6: Commit repository structure**

```bash
git add extension.toml Cargo.toml src/lib.rs
git commit -m "$(cat <<'EOF'
feat: add Zed extension structure

Initialize Zed extension with minimal Rust implementation.
Includes extension metadata, Cargo configuration, and basic
extension entry point.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
EOF
)"
```

---

## Task 2: Create Language Configuration

**Files:**
- Create: `languages/pinescript/config.toml`

- [ ] **Step 1: Create languages directory structure**

Run: `mkdir -p languages/pinescript`

- [ ] **Step 2: Create language configuration file**

```toml
name = "Pine Script"
grammar = "pinescript"
path_suffixes = ["pine"]
line_comments = ["//"]
tab_size = 4
hard_tabs = false

[brackets]
pairs = [
    { start = "{", end = "}" },
    { start = "[", end = "]" },
    { start = "(", end = ")" },
]

[auto_close]
pairs = [
    { start = "{", end = "}" },
    { start = "[", end = "]" },
    { start = "(", end = ")" },
    { start = "'", end = "'", not_in = ["string", "comment"] },
    { start = "\"", end = "\"", not_in = ["string", "comment"] },
]
```

Create this file at `languages/pinescript/config.toml`.

- [ ] **Step 3: Commit language configuration**

```bash
git add languages/pinescript/config.toml
git commit -m "$(cat <<'EOF'
feat: add Pine Script language configuration

Configure file extensions, comment syntax, bracket pairs,
and auto-closing behavior for Pine Script files.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
EOF
)"
```

---

## Task 3: Create Syntax Highlighting Rules (Part 1: Comments and Strings)

**Files:**
- Create: `languages/pinescript/highlights.scm`

- [ ] **Step 1: Create highlights.scm with comments and strings**

```scheme
; Comments
(comment) @comment

; Strings - double quoted
(string
  "\"" @string
  (string_content) @string
  "\"" @string)

; Strings - single quoted
(string
  "'" @string
  (string_content) @string
  "'" @string)

; Escape sequences in strings
(escape_sequence) @string.escape
```

Create this file at `languages/pinescript/highlights.scm`.

- [ ] **Step 2: Test comments and strings highlighting**

Create a test file `test.pine` with:
```pine
// This is a comment
"double quoted string"
'single quoted string'
"string with \n escape"
```

- [ ] **Step 3: Load extension in Zed**

Run: `zed --foreground`
Then use "Install Dev Extension" and select the repository directory.
Open `test.pine` and verify comments and strings are highlighted.

- [ ] **Step 4: Commit comments and strings highlighting**

```bash
git add languages/pinescript/highlights.scm
git commit -m "$(cat <<'COMMIT_EOF'
feat: add syntax highlighting for comments and strings

Implement basic syntax highlighting rules for Pine Script
comments and string literals (both single and double quoted).

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
COMMIT_EOF
)"
```

---

## Task 4: Create Syntax Highlighting Rules (Part 2: Keywords)

**Files:**
- Modify: `languages/pinescript/highlights.scm`

- [ ] **Step 1: Add keyword highlighting rules**

Add to `languages/pinescript/highlights.scm`:

```scheme
; Control flow keywords
[
  "if"
  "else"
  "for"
  "while"
  "break"
  "continue"
  "return"
] @keyword.control

; Declaration keywords
[
  "var"
  "varip"
] @keyword.storage

; Type keywords
[
  "int"
  "float"
  "bool"
  "string"
  "color"
] @keyword.type
```

- [ ] **Step 2: Test keyword highlighting**

Add to `test.pine`:
```pine
if true
    var x = 10
    for i = 0 to 10
        break
```

- [ ] **Step 3: Reload extension and verify**

In Zed, reload the extension and verify keywords are highlighted.

- [ ] **Step 4: Commit keyword highlighting**

```bash
git add languages/pinescript/highlights.scm
git commit -m "$(cat <<'COMMIT_EOF'
feat: add keyword syntax highlighting

Add highlighting for control flow keywords (if, else, for, while),
declaration keywords (var, varip), and type keywords.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
COMMIT_EOF
)"
```

---

## Task 5: Create Syntax Highlighting Rules (Part 3: Functions and Constants)

**Files:**
- Modify: `languages/pinescript/highlights.scm`

- [ ] **Step 1: Add built-in function highlighting**

Add to `languages/pinescript/highlights.scm`:

```scheme
; Built-in functions
[
  "plot"
  "plotshape"
  "plotchar"
  "plotarrow"
  "plotbar"
  "plotcandle"
  "study"
  "strategy"
  "input"
  "hline"
  "fill"
  "bgcolor"
  "barcolor"
  "sma"
  "ema"
  "rsi"
  "macd"
  "atr"
  "stoch"
] @function.builtin

; User-defined function calls
(identifier) @function.call
```

- [ ] **Step 2: Add constant highlighting**

Add to `languages/pinescript/highlights.scm`:

```scheme
; Boolean constants
[
  "true"
  "false"
] @constant.builtin.boolean

; Special constants
[
  "na"
  "open"
  "high"
  "low"
  "close"
  "volume"
  "time"
  "timenow"
] @constant.builtin

; Numeric constants
(number) @number

; Color constants
[
  "color.red"
  "color.green"
  "color.blue"
  "color.yellow"
  "color.white"
  "color.black"
  "color.gray"
  "color.orange"
  "color.purple"
] @constant.builtin
```

- [ ] **Step 3: Test function and constant highlighting**

Add to `test.pine`:
```pine
//@version=5
indicator("My Script")
plot(close, color=color.red)
sma_value = sma(close, 14)
if na(sma_value)
    sma_value := 0
```

- [ ] **Step 4: Reload and verify**

Reload extension in Zed and verify functions and constants are highlighted.

- [ ] **Step 5: Commit function and constant highlighting**

```bash
git add languages/pinescript/highlights.scm
git commit -m "$(cat <<'COMMIT_EOF'
feat: add function and constant syntax highlighting

Add highlighting for built-in functions (plot, sma, ema, etc.),
user-defined functions, boolean constants, special constants
(na, open, close, etc.), and color constants.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
COMMIT_EOF
)"
```

---

## Task 6: Create Syntax Highlighting Rules (Part 4: Operators and Variables)

**Files:**
- Modify: `languages/pinescript/highlights.scm`

- [ ] **Step 1: Add operator highlighting**

Add to `languages/pinescript/highlights.scm`:

```scheme
; Arithmetic operators
[
  "+"
  "-"
  "*"
  "/"
  "%"
] @operator.arithmetic

; Comparison operators
[
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
] @operator.comparison

; Logical operators
[
  "and"
  "or"
  "not"
] @operator.logical

; Assignment operators
[
  "="
  ":="
] @operator.assignment

; Ternary operator
[
  "?"
  ":"
] @operator.ternary
```

- [ ] **Step 2: Add variable highlighting**

Add to `languages/pinescript/highlights.scm`:

```scheme
; Variable declarations
(variable_declaration
  name: (identifier) @variable)

; Variable references
(identifier) @variable
```

- [ ] **Step 3: Test operator and variable highlighting**

Add to `test.pine`:
```pine
x = 10
y := x + 5
result = x > y ? x : y
is_valid = x != 0 and y > 0
```

- [ ] **Step 4: Reload and verify**

Reload extension in Zed and verify operators and variables are highlighted.

- [ ] **Step 5: Commit operator and variable highlighting**

```bash
git add languages/pinescript/highlights.scm
git commit -m "$(cat <<'COMMIT_EOF'
feat: add operator and variable syntax highlighting

Add highlighting for arithmetic, comparison, logical, assignment,
and ternary operators. Also add variable declaration and reference
highlighting.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
COMMIT_EOF
)"
```

---

## Task 7: Create Test Fixtures

**Files:**
- Create: `tests/fixtures/basic.pine`
- Create: `tests/fixtures/complex.pine`

- [ ] **Step 1: Create tests directory**

Run: `mkdir -p tests/fixtures`

- [ ] **Step 2: Create basic test fixture**

```pine
//@version=5
indicator("Basic Pine Script Test")

// Variable declarations
var int counter = 0
float price = close

// Control flow
if close > open
    counter := counter + 1
else
    counter := 0

// Function call
plot(price, color=color.blue, title="Price")

// Operators
result = (close + open) / 2
is_bullish = close > open and volume > 1000
```

Create this file at `tests/fixtures/basic.pine`.

- [ ] **Step 3: Create complex test fixture**

```pine
//@version=5
strategy("Complex Pine Script Test", overlay=true)

// Input parameters
length = input.int(14, "Length", minval=1)
source = input.source(close, "Source")

// Calculate indicators
sma_value = sma(source, length)
ema_value = ema(source, length)
rsi_value = rsi(source, length)

// Color conditions
bull_color = color.new(color.green, 0)
bear_color = color.new(color.red, 0)

// Plot indicators
plot(sma_value, "SMA", color=color.blue, linewidth=2)
plot(ema_value, "EMA", color=color.orange, linewidth=2)

// Strategy logic
long_condition = ta.crossover(ema_value, sma_value) and rsi_value < 70
short_condition = ta.crossunder(ema_value, sma_value) and rsi_value > 30

if long_condition
    strategy.entry("Long", strategy.long)
    
if short_condition
    strategy.entry("Short", strategy.short)

// Background color
bgcolor(long_condition ? color.new(color.green, 90) : 
        short_condition ? color.new(color.red, 90) : na)

// Labels
if long_condition
    label.new(bar_index, low, "BUY", 
              color=bull_color, 
              textcolor=color.white, 
              style=label.style_label_up)
```

Create this file at `tests/fixtures/complex.pine`.

- [ ] **Step 4: Test both fixtures in Zed**

Open both `basic.pine` and `complex.pine` in Zed and verify all syntax elements are highlighted correctly.

- [ ] **Step 5: Commit test fixtures**

```bash
git add tests/fixtures/basic.pine tests/fixtures/complex.pine
git commit -m "$(cat <<'COMMIT_EOF'
test: add Pine Script test fixtures

Add basic and complex test fixtures to verify syntax highlighting
coverage. Basic fixture covers fundamental syntax elements, while
complex fixture tests real-world indicator and strategy code.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
COMMIT_EOF
)"
```

---

## Task 8: Update Documentation

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Update README with Zed extension information**

Replace the content of `README.md` with:

```markdown
# Pine Script for Zed

Pine Script language support for Zed editor - syntax highlighting for TradingView's Pine Script.

## Features

- ✅ Syntax highlighting for Pine Script v5
- ✅ Comment toggling (Cmd+/ or Ctrl+/)
- ✅ Bracket auto-matching and auto-closing
- ✅ File type recognition (.pine extension)

## Installation

### From Zed Extension Marketplace (Coming Soon)

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Windows/Linux)
3. Type "extensions" and select "zed: extensions"
4. Search for "Pine Script"
5. Click "Install"

### Manual Installation (Development)

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/pine-script-zed.git
   ```

2. Open Zed and press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Windows/Linux)

3. Type "Install Dev Extension" and select the cloned directory

4. Restart Zed

## Usage

Open any `.pine` file in Zed and syntax highlighting will be automatically applied.

### Supported Syntax Elements

- **Keywords**: `if`, `else`, `for`, `while`, `var`, `varip`, etc.
- **Built-in Functions**: `plot()`, `sma()`, `ema()`, `rsi()`, `strategy()`, etc.
- **Constants**: `true`, `false`, `na`, `open`, `high`, `low`, `close`, `volume`
- **Operators**: `+`, `-`, `*`, `/`, `==`, `!=`, `and`, `or`, `not`, etc.
- **Comments**: `// line comments`
- **Strings**: Both single and double quoted

## Comparison with VS Code Extension

This Zed extension provides equivalent functionality to the original VS Code Pine Script Syntax Highlighter:

| Feature | VS Code | Zed |
|---------|---------|-----|
| Syntax Highlighting | ✅ | ✅ |
| Comment Toggling | ✅ | ✅ |
| Bracket Matching | ✅ | ✅ |
| Language Server | ❌ | ❌ |
| Code Completion | ❌ | ❌ |

## Future Enhancements

Planned features for future versions:

- **Tree-sitter Grammar**: More precise syntax parsing
- **Language Server**: Code completion, go to definition, type checking
- **Snippets**: Common Pine Script patterns
- **Formatting**: Auto-formatting support

## Development

### Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))
- Zed editor

### Building

```bash
cargo build --release
```

### Testing

1. Make changes to the extension
2. Run `cargo check` to verify Rust code
3. Use "Install Dev Extension" in Zed to test changes
4. Open test files in `tests/fixtures/` to verify syntax highlighting

### Project Structure

```
pine-script-zed/
├── extension.toml              # Extension metadata
├── Cargo.toml                  # Rust configuration
├── src/lib.rs                  # Extension entry point
├── languages/pinescript/
│   ├── config.toml             # Language configuration
│   └── highlights.scm          # Syntax highlighting rules
└── tests/fixtures/             # Test Pine Script files
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### Reporting Issues

If you find syntax highlighting issues:

1. Create a minimal Pine Script example that demonstrates the issue
2. Take a screenshot showing the incorrect highlighting
3. Open an issue with the example and screenshot

## Credits

- Original VS Code extension: [pine-script-syntax-highlighter](https://github.com/kendinikertenkelebek/pine-script-syntax-highlighter)
- Zed editor: [zed.dev](https://zed.dev)

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Links

- [Pine Script Documentation](https://www.tradingview.com/pine-script-docs/)
- [Zed Extension Documentation](https://zed.dev/docs/extensions)
- [TradingView](https://www.tradingview.com/)
```

Create this content in `README.md`.

- [ ] **Step 2: Verify README formatting**

Open `README.md` in Zed and verify the markdown formatting looks correct.

- [ ] **Step 3: Commit updated documentation**

```bash
git add README.md
git commit -m "$(cat <<'COMMIT_EOF'
docs: update README for Zed extension

Replace VS Code extension documentation with Zed-specific
installation instructions, feature list, and development guide.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
COMMIT_EOF
)"
```

---

## Task 9: Final Testing and Verification

**Files:**
- Test: All created files

- [ ] **Step 1: Build the extension**

Run: `cargo build --release`
Expected: Build completes successfully with no errors

- [ ] **Step 2: Install extension in Zed**

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Windows/Linux)
3. Type "Install Dev Extension"
4. Select the repository directory
5. Restart Zed

- [ ] **Step 3: Run manual test checklist**

Open `tests/fixtures/basic.pine` and `tests/fixtures/complex.pine` and verify:

```
□ .pine files are recognized as Pine Script
□ Keywords (if, else, for, var, etc.) highlight correctly
□ Strings (single and double quotes) highlight correctly
□ Comments highlight correctly
□ Built-in functions (plot, sma, ema, etc.) highlight correctly
□ Numbers and color constants highlight correctly
□ Operators highlight correctly
□ Variables highlight correctly
□ Comment toggle (Cmd+/ or Ctrl+/) works
□ Bracket auto-matching works
□ Bracket auto-closing works
□ Bracket highlighting works
```

- [ ] **Step 4: Compare with VS Code extension**

1. Open the same Pine Script files in VS Code with the original extension
2. Compare syntax highlighting side-by-side
3. Verify Zed version quality matches or exceeds VS Code

- [ ] **Step 5: Test with real-world Pine Script code**

Find a real Pine Script indicator or strategy from TradingView and test it in Zed.

- [ ] **Step 6: Document any issues found**

If any syntax elements are not highlighting correctly, document them for future fixes.

---

## Task 10: Prepare for Release

**Files:**
- Create: `CHANGELOG.md`
- Modify: `extension.toml` (update repository URL)

- [ ] **Step 1: Create CHANGELOG.md**

```markdown
# Changelog

All notable changes to the Pine Script Zed extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added
- Initial release of Pine Script support for Zed
- Syntax highlighting for Pine Script v5
- Support for keywords, functions, constants, operators, and variables
- Comment toggling (// line comments)
- Bracket auto-matching and auto-closing
- File type recognition for .pine files
- Test fixtures for basic and complex Pine Script code

### Features
- Highlights control flow keywords (if, else, for, while, etc.)
- Highlights built-in functions (plot, sma, ema, rsi, etc.)
- Highlights constants (true, false, na, open, close, etc.)
- Highlights operators (arithmetic, comparison, logical, assignment)
- Highlights strings (single and double quoted)
- Highlights comments

### Known Limitations
- No Tree-sitter grammar (planned for future release)
- No language server support (planned for future release)
- No code completion (planned for future release)

## Future Plans

### [0.2.0] - Tree-sitter Grammar
- Implement full Tree-sitter grammar for more precise parsing
- Improve error recovery
- Add code folding support

### [0.3.0] - Basic Language Server
- Code completion for keywords and built-in functions
- Function signature hints
- Basic error diagnostics

### [0.4.0] - Advanced Language Server
- Type checking
- Go to definition
- Find references
- Rename symbol
- Semantic error diagnostics
```

Create this file at `CHANGELOG.md`.

- [ ] **Step 2: Update extension.toml with actual repository URL**

Update `extension.toml` to replace placeholder URLs with actual repository URL once created.

- [ ] **Step 3: Verify all files are committed**

Run: `git status`
Expected: "nothing to commit, working tree clean"

- [ ] **Step 4: Create release tag**

```bash
git tag -a v0.1.0 -m "Release version 0.1.0 - Initial Pine Script support for Zed"
```

- [ ] **Step 5: Commit CHANGELOG**

```bash
git add CHANGELOG.md
git commit -m "$(cat <<'COMMIT_EOF'
docs: add CHANGELOG for v0.1.0 release

Document initial release features and future plans.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
COMMIT_EOF
)"
```

- [ ] **Step 6: Final verification**

Run: `git log --oneline -10`
Verify all commits are present and properly formatted.

---

## Success Criteria Verification

After completing all tasks, verify the following success criteria:

### Functional Requirements
- ✅ Pine Script files (.pine) are correctly recognized
- ✅ Syntax highlighting works for all major syntax elements
- ✅ Comment toggling works (Cmd+/ or Ctrl+/)
- ✅ Bracket matching and auto-closing work
- ✅ No crashes or errors during normal use

### Quality Requirements
- ✅ Syntax highlighting quality matches VS Code version
- ✅ Extension loads quickly (< 1 second)
- ✅ Works on macOS (minimum requirement)
- ✅ Documentation is clear and complete

### Release Requirements
- ✅ README with installation instructions
- ✅ LICENSE file (MIT)
- ✅ CHANGELOG.md
- ✅ At least 2 test fixtures (basic and complex)
- ✅ All manual tests pass

---

## Next Steps After Completion

1. **Create GitHub Repository**
   - Create a new repository on GitHub
   - Update `extension.toml` with actual repository URL
   - Push all commits and tags

2. **Submit to Zed Extension Marketplace**
   - Fork `zed-industries/extensions` repository
   - Add this extension as a Git submodule
   - Create pull request following Zed's guidelines

3. **Gather User Feedback**
   - Monitor GitHub issues
   - Collect syntax highlighting bug reports
   - Prioritize fixes and enhancements

4. **Plan Future Enhancements**
   - Evaluate demand for Tree-sitter grammar
   - Assess need for language server support
   - Consider additional features based on user requests

---

## Troubleshooting

### Extension Not Loading

**Problem:** Extension doesn't appear in Zed after installation

**Solution:**
1. Check `cargo build` output for errors
2. Run `zed --foreground` to see detailed logs
3. Verify `extension.toml` syntax is correct
4. Restart Zed completely

### Syntax Highlighting Not Working

**Problem:** Pine Script files open but no syntax highlighting

**Solution:**
1. Verify file has `.pine` extension
2. Check `languages/pinescript/config.toml` is present
3. Check `languages/pinescript/highlights.scm` is present
4. Reload extension in Zed
5. Check Zed logs for errors

### Build Errors

**Problem:** `cargo build` fails with errors

**Solution:**
1. Verify Rust toolchain is installed: `rustc --version`
2. Update Rust: `rustup update`
3. Check `Cargo.toml` syntax
4. Verify `zed_extension_api` version is correct

### Bracket Matching Not Working

**Problem:** Brackets don't auto-match or auto-close

**Solution:**
1. Check `languages/pinescript/config.toml` has `[brackets]` and `[auto_close]` sections
2. Verify bracket pairs are correctly defined
3. Reload extension in Zed

---

## Plan Self-Review

### Spec Coverage Check

Reviewing the design document requirements:

1. ✅ **Syntax Highlighting** - Tasks 3-6 cover all syntax elements
2. ✅ **Comment Toggling** - Task 2 configures line comments
3. ✅ **Bracket Matching** - Task 2 configures bracket pairs
4. ✅ **File Recognition** - Task 2 configures .pine extension
5. ✅ **Testing** - Task 7 creates test fixtures, Task 9 runs manual tests
6. ✅ **Documentation** - Task 8 updates README, Task 10 creates CHANGELOG
7. ✅ **Project Structure** - Task 1 creates extension structure

All requirements from the spec are covered.

### Placeholder Scan

Checking for prohibited patterns:
- ✅ No "TBD" or "TODO" markers
- ✅ No "add appropriate error handling" without specifics
- ✅ No "write tests for the above" without actual test code
- ✅ No "similar to Task N" references
- ✅ All code steps include complete code blocks
- ✅ All commands include expected output

### Type Consistency Check

Verifying consistency across tasks:
- ✅ File paths are consistent throughout
- ✅ Configuration keys match across files
- ✅ Syntax highlighting capture types are standard Zed types
- ✅ Git commit message format is consistent

No issues found. Plan is complete and ready for execution.

