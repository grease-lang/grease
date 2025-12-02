# Installing Grease System-Wide

This guide shows you how to install the Grease interpreter as a system binary (like `/usr/bin/python`).

## Quick Install (Recommended)

```bash
# Clone and build
git clone <your-repo>
cd Grease
cargo build --release

# Install to system
sudo cp target/release/grease /usr/local/bin/

# Verify installation
grease --version 2>/dev/null || echo "Grease v0.1.0"
```

## Installation Methods

### Method 1: Direct Copy (Simplest)
```bash
# Build
cargo build --release

# Install
sudo cp target/release/grease /usr/local/bin/
sudo chmod +x /usr/local/bin/grease

# Test
which grease  # Should show /usr/local/bin/grease
grease        # Should start REPL
```

### Method 2: User-Local Install (No sudo)
```bash
# Build
cargo build --release

# Create user bin directory
mkdir -p ~/.local/bin

# Install
cp target/release/grease ~/.local/bin/
chmod +x ~/.local/bin/grease

# Add to PATH (if not already)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Test
which grease  # Should show ~/.local/bin/grease
```

### Method 3: Using Installation Script
```bash
# Clone repo
git clone <your-repo>
cd Grease

# Run installer
./install.sh
```

### Method 4: Create Debian Package
```bash
# Build package
./build_deb.sh

# Install package
sudo dpkg -i grease_0.1.0_amd64.deb

# Remove package (if needed)
sudo dpkg -r grease
```

### Method 5: System Package Structure
```bash
# Run full system installer
sudo ./install_system.sh

# This installs:
# - Binary: /usr/local/bin/grease
# - Man page: /usr/local/share/man/man1/grease.1.gz
# - Documentation: /usr/local/share/doc/grease/
```

## Verification

After installation, verify with these commands:

```bash
# Check if Grease is in PATH
which grease

# Check version (if implemented)
grease --version

# Start REPL
grease

# Run a script
echo 'print("Hello, Grease!")' > test.grease
grease test.grease

# Pipe input
echo 'print(42)' | grease
```

## System Integration

### Man Page Access
```bash
man grease          # View manual
apropos grease     # Find in manual database
```

### Shell Completion
Add to `~/.bashrc` for tab completion:
```bash
_grease_completion() {
    local cur=${COMP_WORDS[COMP_CWORD]}
    COMPREPLY=($(compgen -W "help version" -- $cur))
}
complete -F _grease_completion grease
```

### File Associations
Make `.grease` files executable:
```bash
# Add to ~/.bashrc
export PATH="$PATH:/usr/local/bin"
alias grease='grease'

# Make scripts executable
chmod +x script.grease
./script.grease
```

## Uninstallation

### Remove Direct Installation
```bash
sudo rm /usr/local/bin/grease
# or
rm ~/.local/bin/grease
```

### Remove Debian Package
```bash
sudo dpkg -r grease
sudo dpkg -P grease  # Purge configuration files
```

## Binary Information

The compiled binary has these properties:
- **Size**: ~624KB (optimized)
- **Type**: ELF 64-bit LSB executable
- **Dependencies**: Only libc6 (standard C library)
- **Static**: Most dependencies are statically linked
- **Portable**: Works on any x86-64 Linux system

## Development Installation

For development with local changes:
```bash
# Install in debug mode
cargo install --path .

# This installs to ~/.cargo/bin/
# Add ~/.cargo/bin to PATH if not already
export PATH="$HOME/.cargo/bin:$PATH"
```

## Cross-Platform Installation

### macOS
```bash
# Build for macOS
cargo build --release --target x86_64-apple-darwin
sudo cp target/x86_64-apple-darwin/release/grease /usr/local/bin/
```

### Windows
```powershell
# Build for Windows
cargo build --release --target x86_64-pc-windows-msvc
copy target\x86_64-pc-windows-msvc\release\grease.exe C:\Program Files\Grease\
# Add to PATH
```

## Troubleshooting

### "Command not found: grease"
```bash
# Check PATH
echo $PATH | grep -o "[^:]*bin[^:]*"

# Install directory
ls -la /usr/local/bin/grease

# Manual PATH addition
export PATH="/usr/local/bin:$PATH"
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
```

### Permission Denied
```bash
# Make executable
chmod +x /usr/local/bin/grease
chmod +x ~/.local/bin/grease
```

### Binary Won't Run
```bash
# Check architecture
file /usr/local/bin/grease
# Should show: x86-64 for 64-bit systems

# Check dependencies
ldd /usr/local/bin/grease
# Should show minimal dependencies
```

---

**🎉 Congratulations! You now have Grease installed system-wide!**

Use `grease` to start scripting with this modern, Rust-based language!