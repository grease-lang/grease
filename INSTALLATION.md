
# 🦀 Grease Installation Guide

## Quick Start (3 commands)

```bash
git clone <your-repo>
cd Grease
make install  # or: sudo make install
```

## Installation Options

### 🚀 Method 1: Makefile (Recommended)
```bash
make install        # Install system-wide
make install-user   # Install for current user
make uninstall      # Remove system-wide
make test          # Run tests
```

### 📦 Method 2: Manual Install
```bash
cargo build --release
sudo cp target/release/grease /usr/local/bin/
```

### 🏠 Method 3: User Install (No sudo)
```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/grease ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### 📦 Method 4: Debian Package
```bash
make deb
sudo dpkg -i grease_*.deb
```

### 🎯 Method 5: Installation Script
```bash
./install.sh          # Interactive installer
./install_system.sh   # Full system install
```

## Verification

```bash
# Check installation
which grease          # Should show path
grease               # Should start REPL

# Test functionality
echo 'print("🦀 Grease works!")' | grease
```

## Binary Details

- **Location**: `target/release/grease`
- **Size**: 624KB (optimized)
- **Type**: ELF 64-bit executable
- **Dependencies**: libc6 only
- **Installation**: `/usr/local/bin/grease` (system) or `~/.local/bin/grease` (user)
- **Features**: REPL, LSP server, linter, module system, standard library

## Usage Examples

```bash
# Interactive REPL
grease

# Run script
grease script.grease

# Pipe input
echo 'print(42 + 8)' | grease

# Make executable script
echo 'print("Hello")' > hello.grease
chmod +x hello.grease
./hello.grease

# Lint code
grease --lint script.grease

# Start LSP server for IDE support
grease lsp

# Execute inline code
grease --eval 'print("Quick test")'
```

## Uninstallation

```bash
make uninstall      # If installed with make
sudo rm /usr/local/bin/grease    # Manual system
rm ~/.local/bin/grease              # Manual user
```

---

**🎉 You're ready to use Grease!** 

A modern, Rust-based scripting language.