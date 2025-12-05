
# Installing Grease

This guide shows you how to install the Grease interpreter using various methods.

## 🚀 Quick Install (Recommended)

### Option 1: Install from CI/CD Nightly Artifacts

**For Debian/Ubuntu (.deb packages):**
```bash
# Download latest nightly from GitLab CI/CD
curl -LO https://gitlab.com/grease-lang/grease/-/jobs/artifacts/main/raw/grease_*.deb?job=nightly-deb
sudo dpkg -i grease_*.deb
```

**For Arch Linux (.pkg.tar.zst packages):**
```bash
# Download latest nightly from GitLab CI/CD
curl -LO https://gitlab.com/grease-lang/grease/-/jobs/artifacts/main/raw/*.pkg.tar.zst?job=nightly-arch
sudo pacman -U *.pkg.tar.zst
```

**Browse all artifacts:** https://gitlab.com/grease-lang/grease/-/artifacts

### Option 2: Build Packages Locally

**Debian Package:**
```bash
git clone https://gitlab.com/grease-lang/grease.git
cd grease
./build_tools/debian/build_deb.sh --nightly
sudo dpkg -i grease_*.deb
```

**Arch Linux Package:**
```bash
git clone https://gitlab.com/grease-lang/grease.git
cd grease/build_tools/archlinux/nightly
makepkg -s --noconfirm
sudo pacman -U *.pkg.tar.zst
```

### Option 3: Stable Releases
For stable releases, visit: https://gitlab.com/grease-lang/grease/-/releases

## Installation Methods

### Method 1: Package Manager (Recommended)
Use the pre-built packages from CI/CD or build locally using the scripts above.

### Method 2: Direct Binary Install
```bash
# Build from source
git clone https://gitlab.com/grease-lang/grease.git
cd grease
cargo build --release

# Install to system
sudo cp target/release/grease /usr/local/bin/
sudo chmod +x /usr/local/bin/grease

# Test
which grease  # Should show /usr/local/bin/grease
grease        # Should start REPL
```

### Method 3: User-Local Install (No sudo)
```bash
# Build from source
git clone https://gitlab.com/grease-lang/grease.git
cd grease
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

### Method 4: Using Installation Script
```bash
git clone https://gitlab.com/grease-lang/grease.git
cd grease
./build_tools/any-linux/install.sh
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

# Test LSP server
grease lsp --help

# Test linter
echo 'x = 5' > test.grease
grease --lint test.grease

# Test UI functionality
echo 'ui_window_create("Test", 400, 300, "win"); ui_run()' > test_ui.grease
grease test_ui.grease
```

## System Integration

### Man Page Access
```bash
man grease          # View manual
apropos grease     # Find in manual database
```

### Shell Completion
Grease provides automatic shell completions:
```bash
# Generate completions for your shell
grease completions bash > grease.bash
grease completions zsh > grease.zsh
grease completions fish > grease.fish

# Or use the provided completions
source completions/grease.bash  # Add to ~/.bashrc
source completions/grease.zsh   # Add to ~/.zshrc
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

### IDE Integration
Set up Language Server Protocol support:

#### VSCode
```bash
# Install extension
cd /path/to/grease/editors/vscode
npm install
npm run compile
code --install-extension .
```

#### Neovim
```bash
# Add to Neovim config
require('lspconfig').grease.setup {
  cmd = { 'grease', 'lsp' },
  filetypes = { 'grease' }
}
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

## System Requirements

### Minimum Requirements
- **Rust**: 1.91.1 or later (for building from source)
- **OS**: Linux (x86_64, ARM64, ARM32, i686, RISC-V 64)
- **Memory**: 64MB RAM minimum
- **Storage**: 10MB disk space

### Recommended Requirements
- **Rust**: 1.91.1 or later
- **OS**: Linux (x86_64) with glibc 2.17+
- **Memory**: 256MB RAM or more
- **Storage**: 50MB disk space for development

### UI System Requirements
For using Grease's hybrid UI system:
- **Graphics**: OpenGL 3.3+ compatible graphics driver
- **Display**: X11 or Wayland display server
- **Libraries**: System graphics libraries (automatically included in binary)
- **Memory**: Additional 50MB for UI components

## Building Packages

### Debian Package Build Script
The `build_tools/debian/build_deb.sh` script creates Debian packages:

```bash
# Nightly build (with commit hash)
./build_tools/debian/build_deb.sh --nightly

# Stable build
./build_tools/debian/build_deb.sh
```

**Features:**
- Automatic versioning with commit hash for nightly builds
- Includes man page and documentation
- Proper Debian package metadata
- Dependency management (libc6 only)

### Arch Linux PKGBUILD
The `build_tools/archlinux/nightly/PKGBUILD` creates Arch Linux packages:

```bash
cd build_tools/archlinux/nightly
makepkg -s --noconfirm
```

**Features:**
- Nightly versioning with commit hash
- Installs to `/usr/bin/grease`
- Includes man page, shell completions, and documentation
- Follows Arch Linux packaging standards

## CI/CD Integration

### Automated Nightly Builds
GitLab CI/CD automatically builds packages on every commit to `main`:

- **`nightly-deb` job**: Creates `.deb` packages for Debian/Ubuntu
- **`nightly-arch` job**: Creates `.pkg.tar.zst` packages for Arch Linux
- **Artifacts**: Available for download from https://gitlab.com/grease-lang/grease/-/artifacts
- **Stable releases**: Available at https://gitlab.com/grease-lang/grease/-/releases

### Version Information
Nightly packages include commit hash in version:
- Format: `0.1.1-nightly-{commit_short}`
- Example: `0.1.1-nightly-3f520af`
- Displayed in REPL: `Grease Scripting Language v0.1.1-nightly-3f520af`

## Binary Information

The compiled binary has these properties:
- **Size**: ~624KB (optimized)
- **Type**: ELF 64-bit LSB executable
- **Dependencies**: Only libc6 (standard C library)
- **Static**: Most dependencies are statically linked
- **Portable**: Works on any x86-64 Linux system
- **Version**: Includes commit hash for nightly builds

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