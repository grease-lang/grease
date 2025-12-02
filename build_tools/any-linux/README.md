# Linux Installation Scripts

This directory contains installation scripts for Grease on any Linux distribution. These scripts provide a convenient way to build and install Grease from source on your system.

## Scripts Overview

### `install.sh` - User-Friendly Installer
A user-friendly installation script that handles both system-wide and user-local installations automatically.

**Features:**
- ✅ Checks for Rust/Cargo installation
- 🔨 Builds Grease in release mode
- 📦 Installs to appropriate directory (system or user-local)
- 🛣️ Automatically updates PATH if needed
- 🧪 Verifies installation success
- 🎖️ No sudo required for user-local installs

**Usage:**
```bash
# Clone the repository
git clone https://gitlab.com/grease-lang/grease.git
cd grease

# Run the installer
./build_tools/any-linux/install.sh
```

**What it does:**
1. Checks if Rust/Cargo is installed (installs if needed)
2. Builds Grease using `cargo build --release`
3. Attempts system-wide installation to `/usr/local/bin` (requires sudo)
4. Falls back to user-local installation in `~/.local/bin` if no sudo access
5. Updates PATH in `~/.bashrc` if `~/.local/bin` isn't already in PATH
6. Tests the installation and provides usage examples

### `install_system.sh` - System-Wide Installation
A comprehensive installation script that creates a proper system-wide installation with man page and documentation.

**Features:**
- 🔨 Builds Grease in release mode
- 📦 Installs to `/usr/local/bin`
- 📖 Creates and installs man page
- 📚 Installs documentation and examples
- 🏗️ Creates proper directory structure
- 🔧 System-wide integration

**Usage:**
```bash
# Clone the repository
git clone https://gitlab.com/grease-lang/grease.git
cd grease

# Run the system installer (requires sudo)
./build_tools/any-linux/install_system.sh
```

**What it installs:**
- Binary: `/usr/local/bin/grease`
- Man page: `/usr/local/share/man/man1/grease.1.gz`
- Documentation: `/usr/local/share/doc/grease/`
- Examples: `/usr/local/share/doc/grease/*.grease`

## Requirements

### System Requirements
- **Linux**: Any x86-64 Linux distribution
- **Rust**: Latest stable Rust toolchain with Cargo
- **Permissions**: 
  - `install.sh`: No special permissions required
  - `install_system.sh`: sudo/root access for system-wide installation

### Installing Rust
If Rust isn't installed, the installer will guide you:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload your shell environment
source ~/.bashrc  # or ~/.zshrc, etc.
```

## Installation Methods Comparison

| Feature | `install.sh` | `install_system.sh` |
|---------|--------------|---------------------|
| **Sudo Required** | No (optional) | Yes |
| **Installation Location** | `/usr/local/bin` or `~/.local/bin` | `/usr/local/bin` |
| **Man Page** | ❌ | ✅ |
| **Documentation** | ❌ | ✅ |
| **Examples** | ❌ | ✅ |
| **PATH Management** | ✅ | ❌ (assumes /usr/local/bin in PATH) |
| **User-Friendly** | ✅ | ⚠️ (more technical) |

## Post-Installation Verification

After running either script, verify your installation:

```bash
# Check if Grease is available
which grease

# Start the REPL
grease

# Run a simple script
echo 'print("Hello, Grease!")' | grease

# View man page (if using install_system.sh)
man grease
```

## Troubleshooting

### "Command not found: grease"
```bash
# Check if the binary exists
ls -la ~/.local/bin/grease  # for install.sh user install
ls -la /usr/local/bin/grease  # for system install

# Check PATH
echo $PATH | grep -o "[^:]*bin[^:]*"

# Add to PATH if needed
export PATH="$HOME/.local/bin:$PATH"  # for user install
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### Permission Denied
```bash
# Make the binary executable
chmod +x ~/.local/bin/grease
chmod +x /usr/local/bin/grease
```

### Rust Not Found
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc
```

### Build Failed
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## Uninstallation

### User-Local Installation (install.sh)
```bash
# Remove binary
rm ~/.local/bin/grease

# Remove from PATH (edit ~/.bashrc)
# Remove the line: export PATH="$HOME/.local/bin:$PATH"
```

### System-Wide Installation (install_system.sh)
```bash
# Remove binary
sudo rm /usr/local/bin/grease

# Remove man page
sudo rm /usr/local/share/man/man1/grease.1.gz

# Remove documentation
sudo rm -rf /usr/local/share/doc/grease
```

## Advanced Usage

### Custom Installation Directory
You can modify the scripts to install to a custom directory:

```bash
# For install.sh, edit the INSTALL_DIR variable
INSTALL_DIR="/opt/grease"

# For install_system.sh, modify all paths
sudo mkdir -p /opt/grease/bin
sudo cp target/release/grease /opt/grease/bin/
```

### Development Installation
For development with local changes:

```bash
# Install in debug mode for faster compilation
cargo install --path .

# This installs to ~/.cargo/bin/
export PATH="$HOME/.cargo/bin:$PATH"
```

## Integration with Package Managers

These scripts are designed for direct installation from source. For package manager integration, consider:

- **Debian/Ubuntu**: Use `build_tools/debian/build_deb.sh`
- **Arch Linux**: Use `build_tools/archlinux/nightly/PKGBUILD`
- **Other distributions**: These scripts provide a universal installation method

## Security Considerations

- These scripts download and build code from the repository
- Always review scripts before running with sudo
- Consider using official packages when available
- Verify the binary after installation: `file /usr/local/bin/grease`

## Contributing

When contributing to these scripts:

1. Test on multiple Linux distributions
2. Ensure backward compatibility
3. Update this README for new features
4. Test both installation methods
5. Verify uninstallation procedures

## Support

For issues with these installation scripts:

1. Check the troubleshooting section above
2. Verify your system meets requirements
3. Test manual installation as a fallback
4. Report issues at: https://gitlab.com/grease-lang/grease/-/issues

---

**🎉 Enjoy using Grease!** 

Visit https://gitlab.com/grease-lang/grease for more information and documentation.