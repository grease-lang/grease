
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
- **Hybrid UI System**: High-performance GUI with both VM-based and pure Rust components
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

# Hybrid UI System - High Performance GUI
ui_window_create("My App", 800, 600, "main_window")

# Traditional VM-based UI (flexible but slower)
ui_button_add("main_window", "btn1", "VM Button", 10, 10, 120)
ui_label_add("main_window", "lbl1", "VM Label", 10, 50)

# High-performance hybrid UI (native Rust speed)
fast_button = ui_create_button_pure("Fast Button", "handle_click")
fast_label = ui_create_label_pure("Fast Label")

# Add hybrid components to window
ui_add_hybrid_component("main_window", fast_button)
ui_add_hybrid_component("main_window", fast_label)

# Show window and start event loop
ui_window_show("main_window")
ui_run()
```

## Installation

### 🚀 Quick Install (Recommended)

#### Option 1: Install from CI/CD Artifacts (Nightly Builds)
Download the latest nightly packages from GitLab CI/CD:

**Debian/Ubuntu (.deb packages):**
```bash
# Download from GitLab CI/CD artifacts
curl -LO https://gitlab.com/grease-lang/grease/-/jobs/artifacts/main/raw/grease_*.deb?job=nightly-deb
sudo dpkg -i grease_*.deb
```

**Arch Linux (.pkg.tar.zst packages):**
```bash
# Download from GitLab CI/CD artifacts
curl -LO https://gitlab.com/grease-lang/grease/-/jobs/artifacts/main/raw/*.pkg.tar.zst?job=nightly-arch
sudo pacman -U *.pkg.tar.zst
```

**Fedora/RHEL/CentOS (.rpm packages):**
```bash
# Download from GitLab CI/CD artifacts
curl -LO https://gitlab.com/grease-lang/grease/-/jobs/artifacts/main/raw/*.rpm?job=nightly-rpm
sudo dnf install *.rpm
# Or for older systems:
sudo yum install *.rpm
```

**Or browse all artifacts:** https://gitlab.com/grease-lang/grease/-/artifacts

**Cross-compiled Binaries:**
```bash
# ARM64 (64-bit ARM)
curl -LO https://gitlab.com/grease-lang/grease/-/jobs/artifacts/main/raw/target/aarch64-unknown-linux-gnu/release/grease?job=nightly-arm64

# ARM32 (32-bit ARM)
curl -LO https://gitlab.com/grease-lang/grease/-/jobs/artifacts/main/raw/target/armv7-unknown-linux-gnueabihf/release/grease?job=nightly-arm32

# i686 (32-bit x86)
curl -LO https://gitlab.com/grease-lang/grease/-/jobs/artifacts/main/raw/target/i686-unknown-linux-gnu/release/grease?job=nightly-i686

# RISC-V 64-bit
curl -LO https://gitlab.com/grease-lang/grease/-/jobs/artifacts/main/raw/target/riscv64gc-unknown-linux-gnu/release/grease?job=nightly-riscv64
```

#### Option 2: Build Packages Locally
```bash
# Debian package
./build_tools/debian/build_deb.sh --nightly
sudo dpkg -i grease_*.deb

# Arch Linux package
cd build_tools/arch/nightly
makepkg -s --noconfirm
sudo pacman -U *.pkg.tar.zst

# RPM package (Fedora/RHEL/CentOS)
./build_tools/rpm/build_rpm.sh --nightly
sudo dnf install rpmbuild/RPMS/*/grease-nightly-*.rpm
```

#### Option 3: Stable Releases
For stable releases, visit: https://gitlab.com/grease-lang/grease/-/releases

#### Option 3: Build from Source
```bash
git clone https://gitlab.com/grease-lang/grease.git
cd grease

# Ensure correct Rust version
rustup install 1.91.1
rustup default 1.91.1

# Native build (auto-detects architecture)
cargo build --release
sudo cp target/release/grease /usr/local/bin/

# Or use the installation scripts for cross-compilation support
./build_tools/any-linux/install.sh --arch arm64-v8a
./build_tools/any-linux/install.sh --arch i686
```

### 🔧 Rust Version Requirements

**Required Rust version: 1.91.1**

Grease requires Rust 1.91.1. The project includes a `rust-toolchain.toml` file that automatically selects the correct version when using Rustup.

To verify your Rust version:
```bash
rustc --version  # Should show rustc 1.91.1
```

To install the correct version:
```bash
rustup install 1.91.1
rustup default 1.91.1
```

### 🛠️ Building Packages

#### Debian Package
```bash
# Nightly build (with commit hash in version)
./build_tools/debian/build_deb.sh --nightly

# Stable build
./build_tools/debian/build_deb.sh
```

#### Arch Linux Package
```bash
cd build_tools/arch/nightly
makepkg -s --noconfirm
```

#### RPM Package (Fedora/RHEL/CentOS)
```bash
# Nightly build
./build_tools/rpm/build_rpm.sh --nightly

# Stable build
./build_tools/rpm/build_rpm.sh
```

## Usage

### Running Grease

#### Interactive REPL
```bash
grease
# Or if built from source:
cargo run
```

#### Execute Script Files
```bash
grease script.grease
# Or if built from source:
cargo run script.grease
```

#### Pipe Input
```bash
echo 'print("Hello, World!")' | grease
# Or if built from source:
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
- **Auto-completion**: Intelligent code completion for keywords, functions, variables, and UI functions
- **Diagnostics**: Real-time syntax and semantic error checking
- **Go to Definition**: Navigate to symbol definitions across files
- **Hover Information**: Documentation and type information on hover
- **Document Symbols**: Outline view of file structure
- **Semantic Tokens**: Enhanced syntax highlighting
- **UI Function Support**: Complete auto-completion for all hybrid UI functions

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

See [docs/LSP_README.md](docs/LSP_README.md) for detailed setup instructions.

### Manpage
A manpage is available for detailed documentation:
```bash
man ./docs/grease.1
```

### Shell Completions
Shell completions are provided for bash and zsh:
- Source `completions/grease.bash` for bash
- Source `completions/grease.zsh` for zsh

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
                                                    ↓
                                              Hybrid UI System
                                                    ↓
                                            Dioxus + eframe Rendering
```

- **Lexer**: Tokenizes source code
- **Parser**: Builds Abstract Syntax Tree
- **Compiler**: Generates bytecode instructions
- **VM**: Executes bytecode on stack machine
- **Hybrid UI System**: Provides both VM-based and pure Rust UI components
- **Dioxus Integration**: High-performance template caching and lazy loading

## Future Roadmap

### 🎯 Next Features
- [ ] Dictionaries/objects
- [ ] Error handling with try/catch
- [ ] Classes and object-oriented features
- [ ] Enhanced array operations
- [ ] Improved for loop functionality
- [ ] Advanced UI components (tables, trees, charts)
- [ ] UI layout managers
- [ ] Event-driven programming patterns

### 🚀 Long-term Goals
- [ ] Package manager
- [ ] JIT compilation

- [ ] Enhanced standard library
- [ ] Performance optimizations

### ✅ Recently Completed
- [x] Language Server Protocol (LSP) implementation
- [x] Static analysis and linting
- [x] Cross-language function interop
- [x] Module system with imports
- [x] Comprehensive standard library
- [x] IDE support for major editors
- [x] Hybrid UI system with Dioxus integration
- [x] High-performance GUI components
- [x] Template caching and lazy loading
- [x] Performance benchmarking suite

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

Apache 2.0 License - see [LICENSE](LICENSE) file for details.

---

**Grease**: The high-performance oil for your Rust engine! 🦀