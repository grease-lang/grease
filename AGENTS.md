# AGENTS.md - Grease Programming Language

This file provides comprehensive instructions for AI agents working with the Grease programming language project. It is designed specifically for OpenCode agents using "Big Pickle" and "Grok Code Fast 1" models to work autonomously and effectively.

## Project Overview

Grease is a modern scripting language written in pure Rust that compiles to platform-agnostic bytecode and runs on a custom virtual machine. It's designed as "the high-performance oil for your Rust engine."

### Pure Rust Philosophy
Grease embraces Rust's ecosystem philosophy by maintaining a 100% Rust codebase from the ground up. While we leverage carefully selected Rust dependencies, every library we use—and every dependency those libraries use—is pure Rust. **We will NEVER use a dependency that is not pure Rust**, ensuring a completely Rust-native stack where:

- **Pure Rust Dependencies**: All dependencies are Rust-only, no C libraries or foreign code
- **Unified Toolchain**: Everything compiles with Cargo, no complex build systems or external tools
- **Consistent Safety**: Rust's memory safety guarantees extend through the entire dependency chain
- **Effortless Deployment**: Single binary distribution with consistent behavior across platforms

This Rust-native approach ensures Grease inherits the reliability, performance, and security of the entire Rust ecosystem while maintaining the simplicity of pure Rust compilation.

## Architecture Overview

The compilation pipeline follows this structure:
```
Source Code → Lexer → Tokens → Parser → AST → Compiler → Bytecode → VM → Execution
```

### Core Components (Detailed Breakdown)

#### Main Entry Points
- **`src/main.rs`** (156 lines): CLI interface using clap, supports file execution, REPL, eval mode, linting, and LSP server
- **`src/lib.rs`** (158 lines): Module exports and comprehensive test suite (57 tests covering all components)

#### Language Processing Pipeline
- **`src/token.rs`** (82 lines): Token definitions with 65+ token types including literals, keywords, operators, delimiters
- **`src/lexer.rs`** (396 lines): Tokenizes source code, handles indentation-sensitive parsing, comprehensive test coverage
- **`src/ast.rs`** (78 lines): Abstract Syntax Tree definitions with Expression and Statement enums
- **`src/parser.rs`** (719 lines): Recursive descent parser, handles all language constructs, extensive test coverage

#### Compilation & Execution Engine
- **`src/bytecode.rs`** (268 lines): OpCode definitions (32 opcodes), Value types, Chunk structure, disassembler
- **`src/compiler.rs`** (498 lines): AST to bytecode compiler, local variable management, control flow compilation
- **`src/vm.rs`** (662 lines): Stack-based virtual machine, function calls, native function support, error handling

#### Tooling & Language Services
- **`src/repl.rs`** (89 lines): Interactive Read-Eval-Print Loop with value display
- **`src/linter.rs`** (222 lines): Static analysis, unused variable detection, scope tracking
- **`src/lsp_server.rs`** (498 lines): Full Language Server Protocol implementation
- **`src/lsp_workspace.rs`** (342 lines): Document management, symbol extraction, LSP workspace handling
- **`src/grease.rs`** (169 lines): Main interpreter interface, module loading, verbose execution

## Language Features (Current Implementation Status)

### ✅ Fully Implemented and Working
1. **Variables**: `name = "Grease"` with optional type annotations `name: String = "Grease"`
2. **Data Types**: Numbers (f64), Strings, Booleans, Null
3. **Arithmetic**: `+`, `-`, `*`, `/`, `%` with proper precedence
4. **Comparisons**: `==`, `!=`, `<`, `<=`, `>`, `>=`
5. **Boolean Logic**: `and`, `or`, `not` operators
6. **String Operations**: Concatenation with automatic type coercion
7. **Control Flow**: `if`/`elif`/`else`, `while` loops, basic `for` loops
8. **Functions**: Definitions with parameters, return values, recursion support
9. **Built-in Functions**: `print()` function
10. **Module System**: `use module` and `use module as alias` syntax
11. **Standard Library**: Math and string modules in `std/` (with syntax issues)
12. **Native Functions**: Rust function integration via `native_add` example
13. **REPL**: Interactive mode with value display
14. **Linter**: Static analysis for unused variables
15. **LSP Server**: Complete IDE support with diagnostics, completion, go-to-definition

### 🚧 Partially Implemented/Limited Features
1. **Arrays**: Basic syntax exists but limited functionality
2. **For Loops**: Basic implementation but not fully featured
3. **String Functions**: Standard library functions are placeholders with syntax errors
4. **Module System**: Works but has path resolution limitations

### ❌ Known Issues Requiring Fixes
1. **Standard Library Syntax**: `std/math.grease` and `std/string.grease` have incorrect indentation in if statements
2. **Boolean Operation Error**: Runtime error when concatenating boolean results with strings
3. **Type Coercion**: Limited automatic type conversion in some operations

## Dependencies & Technical Stack

### Core Dependencies (All Pure Rust)
- `clap` 4.0: CLI argument parsing
- `tokio` 1.0: Async runtime for LSP
- `tower-lsp` 0.20: Language Server Protocol implementation
- `serde` 1.0: JSON serialization
- `dashmap` 5.5: Concurrent HashMap
- `ropey` 1.6: Efficient text manipulation

### Build System
- **Primary**: `cargo build --release`
- **Makefile**: Provides installation, testing, and packaging targets
- **Binary Size**: ~624KB optimized
- **Dependencies**: Minimal (libc6 only)

## Development Commands & Workflow

### Essential Commands for Code Agents
```bash
# Build and test (ALWAYS run after changes)
cargo build --release
cargo test

# Run specific functionality
cargo run                           # Start REPL
cargo run examples/hello.grease     # Test example file
cargo run --eval 'print("test")'    # Quick inline test
cargo run -- lint file.grease       # Lint code
cargo run -- lsp                    # Start LSP server

# Makefile targets
make test              # Run tests with output
make test-integration  # End-to-end testing
make build             # Release build
```

## Project Structure

- `src/` - Core language implementation with all compilation pipeline components
  - `main.rs` - CLI entry point with clap interface for REPL, file execution, linting, and LSP
  - `lib.rs` - Module exports and comprehensive test suite (57 tests)
  - `grease.rs` - Main interpreter interface with module loading and verbose execution
  - `token.rs` - Token definitions with 65+ token types (literals, keywords, operators)
  - `lexer.rs` - Lexical analysis with indentation-sensitive parsing (396 lines)
  - `ast.rs` - Abstract Syntax Tree definitions with Expression and Statement enums
  - `parser.rs` - Recursive descent parser handling all language constructs (719 lines)
  - `bytecode.rs` - OpCode definitions (32 opcodes), Value types, Chunk structure, disassembler
  - `compiler.rs` - AST to bytecode compiler with local variable management (498 lines)
  - `vm.rs` - Stack-based virtual machine with function calls and native functions (662 lines)
  - `repl.rs` - Interactive Read-Eval-Print Loop with value display
  - `linter.rs` - Static analysis for unused variables and scope tracking
  - `lsp_server.rs` - Full Language Server Protocol implementation (498 lines)
  - `lsp_workspace.rs` - Document management and symbol extraction for LSP
- `examples/` - Example Grease scripts demonstrating language features (must remain functional)
- `std/` - Standard library modules with math and string functions (needs syntax fixes)
- `editors/` - Editor integrations for VSCode extension and Neovim LSP configuration
- `Cargo.toml` - Project configuration with pure Rust dependencies
- `Makefile` - Build system with installation, testing, and packaging targets

## Project Hierarchy

```
grease/
├── src/                          # Core language implementation
│   ├── main.rs                   # CLI entry point
│   ├── lib.rs                    # Module exports & tests
│   ├── grease.rs                 # Main interpreter interface
│   ├── token.rs                  # Token definitions
│   ├── lexer.rs                  # Lexical analysis
│   ├── ast.rs                    # AST definitions
│   ├── parser.rs                 # Parser implementation
│   ├── bytecode.rs               # Bytecode & opcodes
│   ├── compiler.rs               # AST to bytecode compiler
│   ├── vm.rs                     # Virtual machine
│   ├── repl.rs                   # Interactive REPL
│   ├── linter.rs                 # Static analysis
│   ├── lsp_server.rs             # Language Server Protocol
│   └── lsp_workspace.rs          # LSP workspace management
├── examples/                     # Example Grease scripts
│   ├── hello.grease              # Basic hello world example
│   ├── basics.grease             # Language basics demonstration
│   ├── control_flow.grease       # Control flow examples
│   ├── functions.grease          # Function definitions and calls
│   ├── modules.grease            # Module system usage
│   ├── native.grease             # Native function integration
│   └── prime_calculator.grease    # Complex program example
├── std/                          # Standard library modules
│   ├── math.grease               # Math functions (add, multiply, sqrt, etc.)
│   └── string.grease             # String functions (length, uppercase, etc.)
├── editors/                      # Editor integrations
│   ├── vscode/                   # VSCode extension
│   │   ├── src/extension.ts      # TypeScript extension client
│   │   ├── syntaxes/grease.tmLanguage.json
│   │   ├── language-configuration.json
│   │   ├── package.json
│   │   └── tsconfig.json
│   └── neovim/
│       └── grease-lsp.lua        # Neovim LSP configuration
├── Cargo.toml                    # Rust project configuration
├── Cargo.lock                    # Dependency lock file
├── Makefile                      # Build system targets
├── README.md                     # Project documentation
├── AGENTS.md                     # This file - AI agent instructions
├── LICENSE                       # Apache 2.0 license
├── TODO.md                       # Development roadmap
├── grease.1                      # Man page documentation
├── grease.bash                   # Bash shell completions
├── grease.zsh                    # Zsh shell completions
├── install.sh                    # Installation script
├── install_system.sh             # System-wide installation
├── build_deb.sh                  # Debian package builder
└── test_lsp_server.py            # LSP server test script
```

## Testing Philosophy & Requirements

### Comprehensive Test Coverage
- **57 unit tests** covering all major components
- **Lexer tests**: Tokenization of numbers, strings, keywords, operators
- **Parser tests**: Expression parsing, statement parsing, error handling
- **Compiler tests**: Bytecode generation for all constructs
- **VM tests**: Execution of literals, arithmetic, control flow, functions
- **Integration tests**: End-to-end language execution
- **Linter tests**: Static analysis validation

### Testing Workflow for Code Agents
1. **Write Code**: Implement the feature or fix using Rust development tools
2. **Write Tests**: Create comprehensive tests that actually verify the intended functionality
3. **Run Tests**: Execute `cargo test` to verify all tests pass
4. **Evaluate Results**: Analyze test output and ensure functionality works correctly
5. **Test Examples**: Verify `examples/` files still work with `cargo run examples/hello.grease`
6. **Iterate**: Continue programming and testing until achieving the task with functional tests

### Critical Testing Rules
- **NEVER circumvent tests** - All tests must remain functional and meaningful
- **NEVER disable or remove tests** - Tests are permanent safeguards
- **Tests must be functional** - Each test should actually verify what it claims to test
- **ALWAYS run `cargo test`** after any changes to ensure no regressions
- **Verify examples work** - Use `cargo run examples/hello.grease` to test end-to-end functionality

## Autonomous Development Loop

This writing, testing, and evaluation cycle is designed to enable code agents to program features to completion **without user input or intervention**. The loop must continue until:

- The requested feature is **production ready and complete**
- All tests pass and demonstrate the feature works correctly
- The implementation meets Grease's quality and performance standards
- Edge cases are handled appropriately
- Examples in `examples/` continue to work
- LSP server functionality is preserved (if applicable)
- Documentation is updated if necessary

### Autonomous Development Mandate
A code agent must continue iterating through this development loop independently, making programming decisions, writing tests, and evaluating results until the feature is fully implemented and production-ready. This self-sufficient approach ensures robust, reliable features that maintain Grease's quality standards without requiring human guidance during the development process.

## Code Standards

- Use Rust with 4-space indentation and strict memory safety (no unsafe code)
- Variables and functions use snake_case, types and structs use PascalCase
- Error handling with `?` operator and `match` for exhaustive patterns
- Comprehensive error messages must include line/column information
- All dependencies must be pure Rust only - NEVER use dependencies with C/C++ code
- Shared functionality goes in appropriate `src/` modules with proper exports
- Tests must be comprehensive and actually verify functionality (never placeholders)
- Examples in `examples/` must remain functional at all times
- Language changes require updates to: lexer → parser → compiler → VM → tests
- LSP functionality must be preserved when modifying language features

### Language Feature Implementation Pattern
When adding new language features, follow this sequence:
1. **Lexer** (`src/lexer.rs`): Add new tokens if needed
2. **AST** (`src/ast.rs`): Add new node types
3. **Parser** (`src/parser.rs`): Add parsing logic
4. **Compiler** (`src/compiler.rs`): Add bytecode generation
5. **Bytecode** (`src/bytecode.rs`): Add new opcodes if needed
6. **VM** (`src/vm.rs`): Add execution logic for new opcodes
7. **Tests**: Add comprehensive tests for each component
8. **Examples**: Create example demonstrating the feature

### Error Handling Patterns
- Use `InterpretResult` enum for VM errors
- Provide line/column information in all error messages
- Use `Result<T, String>` for functions that can fail
- Handle errors gracefully in the CLI

## Common Tasks & Procedures

### Adding New Language Features
1. **Analyze existing patterns** in similar features
2. **Follow the implementation sequence** above
3. **Add tests at each step** to verify functionality
4. **Update examples** to demonstrate the feature
5. **Test thoroughly** with `cargo test` and example execution
6. **Verify LSP functionality** if the feature affects language services

### Debugging Issues
1. **Use verbose mode**: `cargo run -- --verbose script.grease`
2. **Check AST output** during parsing
3. **Verify bytecode generation** with disassembler
4. **Test in REPL** for immediate feedback
5. **Run specific tests**: `cargo test test_name`

### Working with Standard Library
- Modules are in `std/` directory
- **CRITICAL**: Fix syntax errors in existing modules first
- Use `use module_name` or `use module_name as alias` syntax
- Functions are defined as regular Grease functions
- Available globally after import

### Language Server Protocol
- Full LSP implementation in `src/lsp_server.rs`
- **Always test LSP functionality** after language changes
- LSP uses the same parser and compiler as the main interpreter
- Workspace management in `src/lsp_workspace.rs`

## Known Issues & Quick Wins

### Immediate Fixes Needed
1. **Fix standard library syntax** in `std/math.grease` and `std/string.grease`
2. **Fix boolean concatenation** runtime error
3. **Improve type coercion** in string operations

### Enhancement Opportunities
1. **Expand array functionality** beyond basic syntax
2. **Improve for loop implementation**
3. **Add more string functions** to standard library
4. **Enhance error messages** with more context

## Quality Assurance Checklist

Before considering any task complete, verify:
- [ ] `cargo test` passes with all 57 tests
- [ ] `cargo run examples/hello.grease` works correctly
- [ ] `cargo run -- lint examples/hello.grease` shows no critical issues
- [ ] `cargo run -- lsp` starts without errors
- [ ] REPL mode works: `cargo run`
- [ ] No new warnings introduced
- [ ] Code follows existing patterns and conventions
- [ ] Tests are comprehensive and actually test functionality
- [ ] Examples demonstrate the feature correctly
- [ ] Error messages are helpful and include line/column info

## Editor Integration Testing

If working on LSP functionality:
1. **Test with VSCode extension** in `editors/vscode/`
2. **Test with Neovim config** in `editors/neovim/grease-lsp.lua`
3. **Verify diagnostics** appear correctly
4. **Test completion** and **go-to-definition**
5. **Check hover** information display

## Final Instructions for Code Agents

1. **Start by reading existing code** to understand patterns
2. **Always run tests** before and after changes
3. **Follow the autonomous development loop** until completion
4. **Never work around tests** - fix them instead
5. **Maintain pure Rust dependency philosophy**
6. **Keep examples functional** at all times
7. **Test LSP functionality** when making language changes
8. **Document your changes** in code comments
9. **Ensure production readiness** before considering tasks complete

This comprehensive guide ensures code agents have all necessary information to work effectively and autonomously on the Grease programming language project while maintaining high quality standards and following established patterns.