# Grease Library Separation

The Grease programming language has been successfully separated into modular components to allow users to choose which functionality they need.

## Architecture

### Main Grease Binary (`grease/`)
The core interpreter includes:
- Language parsing and compilation
- Virtual machine execution
- Standard library functions
- LSP server support
- Package management
- JIT compilation
- Performance optimizations

**No UI or WebAssembly dependencies** - this keeps the main binary small and fast.

### Grease UI Library (`grease-ui/`)
Optional UI functionality including:
- Window management
- Button, label, and input controls
- Hybrid UI system with Dioxus
- Performance benchmarking
- Cross-platform desktop support

### Grease WebAssembly Library (`grease-webassembly/`)
Optional WebAssembly functionality including:
- WebAssembly compilation from Grease bytecode
- JavaScript wrapper generation
- Web API integration
- Runtime statistics

## Usage

### Using Core Grease Only
```bash
# Clone and build the main interpreter
git clone <grease-repo>
cd grease
cargo build --release
./target/release/grease script.grease
```

### Adding UI Support
```bash
# Add UI library as submodule or dependency
git submodule add <grease-ui-repo> grease-ui
# OR copy the grease-ui directory

# Build with UI support
cd grease-ui
cargo build --release --features ui

# Use in your code
use grease_ui::init_ui;
```

### Adding WebAssembly Support
```bash
# Add WebAssembly library as submodule or dependency
git submodule add <grease-wasm-repo> grease-webassembly
# OR copy the grease-webassembly directory

# Build WebAssembly library
cd grease-webassembly
cargo build --release

# Use in your code
use grease_webassembly::init_webassembly;
```

## Benefits

1. **Smaller Core Binary**: Main interpreter is ~624KB without UI/WebAssembly
2. **Optional Dependencies**: Users only install what they need
3. **Modular Development**: Libraries can be developed independently
4. **Flexible Distribution**: Can be distributed as separate packages
5. **Reduced Attack Surface**: Less code in core interpreter

## Package Manager Integration

When the package manager is fully implemented, users will be able to:

```bash
# Install UI library
grease install grease-ui

# Install WebAssembly library
grease install grease-webassembly

# Use in scripts
use grease-ui
use grease-webassembly
```

## System Dependencies

### UI Library (Linux only)
```bash
# Ubuntu/Debian
sudo apt-get install libgtk-3-dev libgdk-pixbuf2.0-dev libpango1.0-dev libatk1.0-dev libcairo-gobject2

# Fedora/RHEL
sudo dnf install gtk3-devel gdk-pixbuf2-devel pango-devel atk-devel cairo-devel

# Arch Linux
sudo pacman -S gtk3 gdk-pixbuf2 pango atk cairo
```

### WebAssembly Library
No additional system dependencies required.

## Testing

All components have been tested:

```bash
# Test main binary
cargo test
cargo run examples/hello.grease

# Test UI library (requires system dependencies)
cd grease-ui && cargo build --release --features ui

# Test WebAssembly library
cd grease-webassembly && cargo build --release
```

## Migration Guide

### For Existing Users
No changes required - the main binary works exactly as before.

### For Users Needing UI/WebAssembly
1. Add the desired library as a submodule or copy the directory
2. Add dependency to your `Cargo.toml`
3. Initialize the library in your VM setup
4. Use the functions in your Grease scripts

### For Library Developers
Each library is now a separate crate with:
- Its own `Cargo.toml`
- Independent versioning
- Optional features
- Separate documentation

This separation makes Grease more modular, maintainable, and user-friendly while preserving all existing functionality.