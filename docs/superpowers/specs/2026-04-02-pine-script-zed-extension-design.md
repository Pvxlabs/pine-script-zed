# Pine Script Zed Extension - Design Document

**Date:** 2026-04-02
**Author:** Tinz Leung
**Status:** Draft
**Version:** 1.0

## Executive Summary

This document describes the design for converting the existing VS Code Pine Script syntax highlighter extension to a Zed editor extension. After evaluating the scope, we decided to create a **simplified first version** that matches the functionality of the existing VS Code extension, rather than building a full-featured extension with Tree-sitter and language server support.

**Project Goal:** Create a Zed extension that provides Pine Script syntax highlighting and basic editing features, equivalent to the existing VS Code extension.

**Timeline:** 1-2 weeks for initial release

**Future Enhancements:** Tree-sitter grammar and language server support can be added in subsequent versions based on user feedback.

## Background

### Current State

The existing `pine-script-syntax-highlighter` VS Code extension provides:
- ✅ Basic syntax highlighting (using TextMate grammar)
- ✅ Comment toggling (// line comments)
- ✅ Bracket auto-matching and auto-closing
- ❌ No language server
- ❌ No code completion
- ❌ No semantic analysis

### Target State (Version 1.0)

The Zed extension will provide equivalent functionality:
- ✅ Pine Script syntax highlighting
- ✅ Comment toggling
- ✅ Bracket auto-matching and auto-closing
- ✅ File type recognition (.pine extension)

### Future Enhancements (Optional)

If the first version is successful, we can consider:
- **Phase 2:** Tree-sitter grammar (more precise parsing)
- **Phase 3:** Basic language server (code completion)
- **Phase 4:** Advanced language server (type checking, go to definition)

## Architecture

### Project Structure

```
pine-script-zed/
├── extension.toml           # Zed extension configuration
├── Cargo.toml              # Rust build configuration
├── src/
│   └── lib.rs              # Extension entry point (minimal implementation)
├── languages/
│   └── pinescript/
│       ├── config.toml      # Language configuration
│       └── highlights.scm   # Syntax highlighting rules
├── grammars/
│   └── pinescript.json      # Converted grammar from VS Code
├── tests/
│   └── fixtures/           # Test Pine Script files
├── README.md
└── LICENSE
```

### Key Design Decisions

1. **Single Repository Architecture**
   - All code in one repository for simplicity
   - Easy to manage and version control
   - Can be split later if needed

2. **No Tree-sitter (Version 1.0)**
   - Use simplified syntax rules supported by Zed
   - Faster development, quicker to market
   - Tree-sitter can be added in future versions

3. **No Language Server (Version 1.0)**
   - Focus on syntax highlighting only
   - Reduces complexity significantly
   - Language server can be added later based on demand

4. **Manual Grammar Conversion**
   - Convert TextMate grammar to Zed's highlight queries
   - Prioritize core syntax elements
   - Complex nested rules can be simplified

## Technical Implementation

### Syntax Rule Conversion

**TextMate to Zed Mapping:**

| TextMate Scope | Zed Capture Type |
|----------------|------------------|
| `comment.line.double-slash.pine` | `@comment` |
| `string.quoted.double.pine` | `@string` |
| `string.quoted.single.pine` | `@string` |
| `keyword.control.pine` | `@keyword` |
| `constant.language.pine` | `@constant` |
| `constant.numeric.pine` | `@number` |
| `entity.name.function.pine` | `@function` |
| `variable.other.pine` | `@variable` |
| `keyword.operator.*` | `@operator` |

**Conversion Strategy:**

1. Analyze all patterns in `pinescript.tmLanguage.json`
2. Create corresponding regex or simple match rules
3. Write Zed-compatible syntax in `highlights.scm`
4. Test with real Pine Script code samples

**Example highlights.scm:**

```scheme
; Comments
(comment) @comment

; Strings
(string) @string

; Keywords
["if" "else" "for" "while" "return" "var" "continue" "break"] @keyword

; Built-in functions
["plot" "study" "strategy" "input" "hline" "fill" "bgcolor" "barcolor"] @function.builtin

; Constants
["true" "false" "na" "open" "high" "low" "close" "volume"] @constant

; Numbers
(number) @number

; Operators
["+" "-" "*" "/" "%" "==" "!=" "<" ">" "<=" ">=" "and" "or" "not" "?" ":"] @operator

; Assignment operators
["=" ":="] @operator
```

### Configuration Files

**extension.toml:**

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

**languages/pinescript/config.toml:**

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

**src/lib.rs (Minimal Implementation):**

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

**Cargo.toml:**

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

## Implementation Plan

### Phase 1: Project Setup (Day 1-2)

**Tasks:**
1. Create new Git repository
2. Set up basic Zed extension structure
   - Create `extension.toml`
   - Create `Cargo.toml`
   - Create `src/lib.rs` with minimal implementation
3. Create directory structure
4. Initialize README and LICENSE

**Deliverables:**
- Working Zed extension skeleton
- Can be loaded in Zed (even if it does nothing yet)

### Phase 2: Grammar Conversion (Day 3-5)

**Tasks:**
1. Analyze existing `pinescript.tmLanguage.json`
2. Extract all syntax patterns
3. Create `languages/pinescript/config.toml`
4. Write `languages/pinescript/highlights.scm`
5. Convert TextMate patterns to Zed syntax

**Priority Order:**
1. Comments (highest priority)
2. Strings
3. Keywords
4. Built-in functions
5. Constants
6. Numbers
7. Operators
8. Variables

**Deliverables:**
- Complete `highlights.scm` file
- Complete `config.toml` file
- Syntax highlighting works for core elements

### Phase 3: Testing and Refinement (Day 6-8)

**Tasks:**
1. Create test fixtures with real Pine Script code
2. Test in Zed using "Install Dev Extension"
3. Compare with VS Code extension side-by-side
4. Fix any highlighting issues
5. Test bracket matching and comment toggling
6. Refine syntax rules based on test results

**Test Cases:**
- Variable declarations and assignments
- Function calls (built-in and user-defined)
- Control flow (if/else/for/while)
- Strings (single and double quotes)
- Comments (line comments)
- Numbers and color constants
- Operators (arithmetic, comparison, logical)
- Complex nested structures

**Deliverables:**
- All test cases pass
- Syntax highlighting quality matches or exceeds VS Code version
- All basic editing features work correctly

### Phase 4: Documentation and Release (Day 9-10)

**Tasks:**
1. Write comprehensive README
   - Installation instructions
   - Features list
   - Screenshots
   - Comparison with VS Code version
2. Add LICENSE file (MIT)
3. Create CHANGELOG.md
4. Prepare for Zed extension marketplace submission
5. Test installation process
6. Create release notes

**Deliverables:**
- Complete documentation
- Ready for public release
- Submission to Zed extension marketplace

## Error Handling

### Development Phase

**Grammar Conversion Errors:**
- If TextMate rules cannot be directly converted, skip and mark as TODO
- Prioritize core syntax elements
- Simplify complex nested rules

**Extension Loading Errors:**
- Use `zed --foreground` to view detailed error logs
- Validate `extension.toml` and `config.toml` format
- Verify all referenced file paths exist

**Build Errors:**
- Ensure Rust toolchain is installed
- Verify `zed_extension_api` version compatibility
- Check Cargo.toml syntax

### User Experience

**File Recognition Issues:**
- Ensure `.pine` extension is correctly configured
- Document how to manually select language if needed

**Syntax Highlighting Issues:**
- Collect problem cases from users
- Iteratively improve `highlights.scm`
- Document known limitations in README

**Performance Issues:**
- Monitor extension load time
- Optimize regex patterns if needed
- Profile with large Pine Script files

## Testing Strategy

### Development Testing

**Local Testing Workflow:**
1. Build extension with `cargo build --release`
2. Use Zed's "Install Dev Extension" feature
3. Open test `.pine` files
4. Verify syntax highlighting
5. Test comment toggling (Cmd+/ or Ctrl+/)
6. Test bracket matching and auto-closing

**Test Fixtures:**

Create `tests/fixtures/` directory with:
- `basic.pine` - Simple variable declarations and functions
- `control-flow.pine` - If/else/for/while statements
- `functions.pine` - Built-in and user-defined functions
- `strings.pine` - Various string formats
- `operators.pine` - All operator types
- `complex.pine` - Real-world complex example from TradingView

**Manual Test Checklist:**

```
□ .pine files are recognized as Pine Script
□ Keywords (if, else, for, var, etc.) highlight correctly
□ Strings (single and double quotes) highlight correctly
□ Comments highlight correctly
□ Built-in functions (plot, study, strategy, etc.) highlight correctly
□ Numbers and color constants highlight correctly
□ Operators highlight correctly
□ Variables highlight correctly
□ Comment toggle (Cmd+/ or Ctrl+/) works
□ Bracket auto-matching works
□ Bracket auto-closing works
□ Bracket highlighting works
```

### Pre-Release Testing

**Cross-Platform Testing:**
- Test on macOS (primary platform)
- Test on Linux (if available)
- Test on Windows (if available)

**Comparison Testing:**
- Open same Pine Script files in VS Code and Zed
- Compare syntax highlighting side-by-side
- Ensure Zed version quality matches or exceeds VS Code

**Performance Testing:**
- Test with large Pine Script files (1000+ lines)
- Measure extension load time
- Verify no lag or stuttering

**Installation Testing:**
- Test "Install Dev Extension" workflow
- Test marketplace installation (after submission)
- Verify extension appears in Zed's extension list

## Success Criteria

### Version 1.0 Release Criteria

**Functional Requirements:**
- ✅ Pine Script files (.pine) are correctly recognized
- ✅ Syntax highlighting works for all major syntax elements
- ✅ Comment toggling works (Cmd+/ or Ctrl+/)
- ✅ Bracket matching and auto-closing work
- ✅ No crashes or errors during normal use

**Quality Requirements:**
- ✅ Syntax highlighting quality matches VS Code version
- ✅ Extension loads quickly (< 1 second)
- ✅ Works on macOS (minimum requirement)
- ✅ Documentation is clear and complete

**Release Requirements:**
- ✅ README with installation instructions
- ✅ LICENSE file (MIT)
- ✅ CHANGELOG.md
- ✅ At least 3 test fixtures
- ✅ All manual tests pass

### Future Enhancement Criteria

**Phase 2 (Tree-sitter):**
- More precise syntax parsing
- Better error recovery
- Support for code folding
- Improved performance

**Phase 3 (Basic Language Server):**
- Code completion for keywords and built-in functions
- Function signature hints
- Basic error diagnostics

**Phase 4 (Advanced Language Server):**
- Type checking
- Go to definition
- Find references
- Rename symbol
- Semantic error diagnostics

## Risks and Mitigation

### Technical Risks

**Risk:** Grammar conversion may be incomplete or incorrect
- **Mitigation:** Start with core syntax elements, iterate based on testing
- **Mitigation:** Compare with VS Code version side-by-side
- **Mitigation:** Collect user feedback and fix issues incrementally

**Risk:** Zed's syntax highlighting system may have limitations
- **Mitigation:** Research Zed's documentation thoroughly
- **Mitigation:** Study existing Zed extensions for best practices
- **Mitigation:** Simplify complex rules if necessary

**Risk:** Extension may not work on all platforms
- **Mitigation:** Test on multiple platforms if possible
- **Mitigation:** Document platform-specific issues
- **Mitigation:** Prioritize macOS support initially

### Project Risks

**Risk:** Scope creep (trying to add too many features)
- **Mitigation:** Stick to the defined scope for version 1.0
- **Mitigation:** Document future enhancements separately
- **Mitigation:** Get user feedback before adding new features

**Risk:** Time overrun
- **Mitigation:** Focus on core functionality first
- **Mitigation:** Use existing VS Code extension as reference
- **Mitigation:** Skip complex edge cases initially

## Dependencies

### Development Dependencies

- Rust toolchain (rustup)
- Zed editor (latest version)
- Git
- Text editor for editing configuration files

### Runtime Dependencies

- Zed editor (user's machine)
- No external dependencies required

### External Resources

- Zed extension documentation: https://zed.dev/docs/extensions
- Zed language extension guide: https://zed.dev/docs/extensions/languages
- Tree-sitter documentation (for future reference)
- Pine Script documentation (TradingView)

## Appendix

### Pine Script Language Overview

Pine Script is TradingView's proprietary scripting language for creating custom indicators and strategies. Key features:

- **Version:** Currently at v5
- **Paradigm:** Declarative, functional
- **Type System:** Dynamic with type annotations (series, simple, const)
- **Key Constructs:** Variables, functions, control flow, built-in functions
- **Use Cases:** Technical indicators, trading strategies, alerts

### Key Syntax Elements

**Keywords:**
- Control flow: `if`, `else`, `for`, `while`, `break`, `continue`, `return`
- Declarations: `var`, `varip`
- Types: `int`, `float`, `bool`, `string`, `color`

**Built-in Functions:**
- Plotting: `plot()`, `plotshape()`, `plotchar()`, `plotarrow()`
- Studies: `study()`, `strategy()`
- Indicators: `sma()`, `ema()`, `rsi()`, `macd()`, `atr()`
- Input: `input()`, `input.int()`, `input.float()`

**Built-in Constants:**
- Price data: `open`, `high`, `low`, `close`, `volume`
- Special values: `na`, `true`, `false`
- Time: `time`, `timenow`, `year`, `month`, `dayofmonth`
- Colors: `color.red`, `color.green`, `color.blue`, etc.

### Reference Materials

- Original VS Code extension: https://github.com/kendinikertenkelebek/pine-script-syntax-highlighter
- Pine Script documentation: https://www.tradingview.com/pine-script-docs/
- Zed extension examples: https://github.com/zed-industries/extensions

---

**Document Version History:**

- v1.0 (2026-04-02): Initial design document
