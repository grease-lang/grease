
# Grease - A Rust-based Scripting Language

Grease is a modern scripting language written in pure Rust. It compiles to platform-agnostic bytecode and runs on a custom virtual machine.

*The high-performance oil for your Rust engine.*

## Features

### ✅ Currently Implemented
- **Variables**: `name = "Grease"` (with optional type annotations: `name: String = "Grease"`)
- **Data Types**: Numbers, Strings, Booleans, Null, Arrays
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparisons**: `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Boolean Logic**: `and`, `or`, `not`
- **String Concatenation**: Automatic type coercion between strings and numbers
- **Control Flow**: `if`/`else`, `while` loops, `for` loops
- **Functions**: Function definitions with parameters and return values
- **Built-in Functions**: `print()` function
- **Module System**: Import standard library modules with `use`
- **Standard Library**: `math` (add, multiply, sqrt, abs, pow, pi) and `string` (length, uppercase, lowercase, contains) modules
- **Native Functions**: Call Rust functions from Grease scripts
- **REPL**: Interactive mode for testing
- **File Execution**: Run scripts from files
- **Linter**: Static analysis for unused variables and code quality
- **Language Server Protocol (LSP)**: Full IDE support with auto-completion, diagnostics, go-to-definition, and more

### 🚧 Syntax Examples

```grease
# Variable declarations
name = "Grease"
version: Number = 0.1
is_awesome = true

# Module imports
use math
use string as str

# Basic arithmetic
x = 10
y = 20
print(x + y)  # 30

# Boolean operations
print(true and false)  # false
print(true or false)   # true
print(not true)        # false

# String operations
print("Hello" + " " + "World")  # Hello World
print("Value: " + 42)           # Value: 42

# Comparisons
print(10 > 5)   # true
print(10 == 10) # true
print(10 != 5)  # true

# Using standard library
use math
print(math.add(5, 3))      # 8
print(math.sqrt(16))       # 4.0
print(math.pi)             # 3.141592653589793

use string as str
print(str.length("hello")) # 5
print(str.contains("hello", "ell")) # true
```

## Installation & Usage

### Building from Source
```bash
git clone <repository>
cd Grease
cargo build --release
```

### Running Grease

#### Interactive REPL
```bash
cargo run
```

#### Execute Script Files
```bash
cargo run script.grease
```

#### Pipe Input
```bash
echo 'print("Hello, World!")' | cargo run
```

### Command Line Options
Grease supports standard command line options:
- `--version`: Display version information
- `--help`: Display help information
- `--eval <CODE>`: Execute inline code
- `--verbose`: Enable verbose output during execution
- `--lint <FILE>`: Lint Grease source code for issues
- `--lsp`: Start Language Server Protocol server
- `FILE`: Execute a script file

### Language Server Protocol (LSP)

Grease includes a complete LSP implementation for professional IDE support:

#### Features
- **Auto-completion**: Intelligent code completion for keywords, functions, and variables
- **Diagnostics**: Real-time syntax and semantic error checking
- **Go to Definition**: Navigate to symbol definitions across files
- **Hover Information**: Documentation and type information on hover
- **Document Symbols**: Outline view of file structure
- **Semantic Tokens**: Enhanced syntax highlighting

#### Quick Start
```bash
# Start LSP server
grease lsp

# VSCode: Install extension from editors/vscode/
# Neovim: Use configuration from editors/neovim/grease-lsp.lua
```

#### Editor Setup
- **VSCode**: Extension available in `editors/vscode/` directory
- **Neovim**: Configuration provided in `editors/neovim/grease-lsp.lua`
- **Other LSP-compatible editors**: Use command `grease lsp` with language `grease`

See [LSP_README.md](LSP_README.md) for detailed setup instructions.

### Manpage
A manpage is available for detailed documentation:
```bash
man ./grease.1
```

### Shell Completions
Shell completions are provided for bash and zsh:
- Source `grease.bash` for bash
- Source `grease.zsh` for zsh

To generate completions for other shells:
```bash
./target/release/grease completions <shell> > grease.<shell>
```

## Language Design

Grease features:
- Simple and readable syntax
- Clear variable declarations
- Optional type safety

### Type System
- **Dynamic typing** by default with automatic type coercion
- **Optional type annotations** for clarity and documentation
- **Simple and intuitive** operations between types

### Memory Safety
Written entirely in safe Rust with:
- No undefined behavior
- Memory safety guarantees
- Platform-agnostic bytecode

## Architecture

```
Source Code → Lexer → Tokens → Parser → AST → Compiler → Bytecode → VM → Execution
```

- **Lexer**: Tokenizes source code
- **Parser**: Builds Abstract Syntax Tree
- **Compiler**: Generates bytecode instructions
- **VM**: Executes bytecode on stack machine

## Future Roadmap

### 🎯 Next Features
- [ ] Dictionaries/objects
- [ ] Error handling with try/catch
- [ ] Classes and object-oriented features
- [ ] Enhanced array operations
- [ ] Improved for loop functionality

### 🚀 Long-term Goals
- [ ] Package manager
- [ ] JIT compilation
- [ ] WebAssembly target
- [ ] Enhanced standard library
- [ ] Performance optimizations

### ✅ Recently Completed
- [x] Language Server Protocol (LSP) implementation
- [x] Static analysis and linting
- [x] Cross-language function interop
- [x] Module system with imports
- [x] Comprehensive standard library
- [x] IDE support for major editors

## Testing

Run the test suite:
```bash
cargo test
```

Run examples:
```bash
cargo run examples/hello.grease
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests
5. Submit a pull request

## License

Apache 2.0 License - see LICENSE file for details.

---

**Grease**: The high-performance oil for your Rust engine! 🦀