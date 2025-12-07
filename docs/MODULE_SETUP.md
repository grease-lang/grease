# Module Setup Guide

## Overview

Grease supports a modular architecture where optional UI and WebAssembly modules can be loaded dynamically. This guide explains how to set up and use these modules.

## Quick Start

### Option 1: Copy Module Folders (Recommended)

The simplest way to use modules is to copy them to your project:

```bash
# Copy from Grease repository
cp -r path/to/grease/grease-ui ./
cp -r path/to/grease/grease-webassembly ./

# Your project structure:
project/
├── main.grease
├── grease-ui/          # UI module
└── grease-webassembly/ # WebAssembly module
```

### Option 2: Use Environment Variables

Set custom paths for modules:

```bash
export GREASE_UI_PATH=/path/to/grease-ui
export GREASE_WASM_PATH=/path/to/grease-webassembly
```

### Option 3: Standard Directory Structure

Organize modules in a standard location:

```bash
mkdir -p modules
cp -r path/to/grease-ui modules/
cp -r path/to/grease/grease-webassembly modules/
```

## Module Detection Priority

Grease searches for modules in this order:

1. **Same Directory**: `./grease-ui`, `./grease-webassembly`
2. **Parent Directory**: `../grease-ui`, `../grease-webassembly`
3. **Subdirectory**: `./modules/grease-ui`, `./modules/grease-webassembly`
4. **Lib Directory**: `./lib/grease-ui`, `./lib/grease-webassembly`
5. **Environment Variables**: `$GREASE_UI_PATH`, `$GREASE_WASM_PATH`

## Building Modules

### UI Module

The UI module requires platform-specific dependencies:

#### Linux
```bash
# Ubuntu/Debian
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install gtk3-devel webkit2gtk4.0-devel

# Arch Linux
sudo pacman -S gtk3 webkit2gtk
```

#### macOS
```bash
# Install via Homebrew
brew install gtk+3

# Or use Xcode command line tools
xcode-select --install
```

#### Windows
```bash
# Using vcpkg (recommended)
vcpkg install gtk3:x64-windows

# Or using MSYS2
pacman -S mingw-w64-x86_64-gtk3
```

#### Android
```bash
# Set up Android NDK
export ANDROID_NDK_ROOT=/path/to/android-ndk
export ANDROID_HOME=/path/to/android-sdk
```

#### WebAssembly
```bash
# No additional dependencies needed
# Works in any modern web browser
```

### WebAssembly Module

The WebAssembly module builds for the target platform:

```bash
# Build for current platform
cd grease-webassembly
cargo build --release

# Build for WebAssembly target
cargo build --release --target wasm32-unknown-unknown

# Build for specific targets
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target aarch64-apple-darwin
```

## Version Compatibility

Grease checks module versions and provides warnings:

- ✅ **Compatible**: Same major.minor version
- ⚠️ **WarningMinor**: Different patch version
- ⚠️ **WarningMajor**: Different minor version
- ❌ **Incompatible**: Different major version

### Checking Version Compatibility

```bash
# Run with verbose output to see version info
grease --verbose script.grease

# Example output:
🔍 Initializing Grease modules...
✅ Found UI module at: ./grease-ui (version 0.1.0)
⚠️ UI module version 0.1.0 vs Core 0.1.1: minor version difference
💡 Recommended action: Should work fine
```

## Using Modules in Grease Code

### Basic Usage

```grease
# Import UI module
use grease-ui

# Create a window
window = ui.create_window("My App", 800, 600, "main")

# Add widgets
button = ui.create_button(window, "btn1", "Click Me", 10, 10, 100)
label = ui.create_label(window, "label1", "Hello, World!", 10, 50)

# Show window
ui.show_window(window)

# Run event loop
ui.run_event_loop()
```

### Advanced Usage

```grease
# Use WebAssembly for computations
use grease-webassembly

# Call WebAssembly function
result = wasm.compute_heavy_operation(data)

# Update UI with results
ui.set_label_value(window, "result_label", result)
```

## Troubleshooting

### Module Not Found

**Error**: `❌ Module 'grease-ui' not found`

**Solutions**:
1. Copy module folder to one of the search locations
2. Set environment variable: `export GREASE_UI_PATH=/path/to/grease-ui`
3. Ensure module folder contains `Cargo.toml`

### Version Mismatch

**Error**: `⚠️ Version mismatch: Module 'grease-ui' (0.2.0) vs Core (0.1.0)`

**Solutions**:
1. Update module to match core version
2. Update core to match module version
3. Accept warning if functionality works

### Build Failures

**Error**: `❌ Module 'grease-ui' build failed`

**Common Solutions**:
- **Linux**: Install GTK development libraries
- **Windows**: Install Visual Studio Build Tools or vcpkg
- **macOS**: Install Xcode command line tools
- **Android**: Set up Android NDK

### Performance Issues

**Symptoms**: Slow UI response, high memory usage

**Solutions**:
1. Use WebAssembly for heavy computations
2. Enable hardware acceleration
3. Optimize widget creation
4. Use platform-specific optimizations

## Project Structure Examples

### Single Project with Modules

```
my-project/
├── src/
│   └── main.grease
├── grease-ui/              # UI module
├── grease-webassembly/     # WebAssembly module
└── Cargo.toml
```

### Workspace with Multiple Projects

```
grease-workspace/
├── modules/
│   ├── grease-ui/
│   └── grease-webassembly/
├── project1/
│   ├── src/
│   └── Cargo.toml
└── project2/
    ├── src/
    └── Cargo.toml
```

## Next Steps

1. **Try Examples**: Run `examples/ui_example.grease` to see modules in action
2. **Read Platform Docs**: See `PLATFORM_SETUP.md` for platform-specific details
3. **Check Troubleshooting**: See `TROUBLESHOOTING.md` for common issues
4. **Join Community**: Get help at [Grease Discord/GitHub](https://github.com/grease-lang/grease)

## Migration to Git Submodules

When ready to convert to git submodules:

```bash
# Remove copied folders
rm -rf grease-ui grease-webassembly

# Add as submodules
git submodule add https://gitlab.com/grease-lang/grease-ui.git
git submodule add https://gitlab.com/grease-lang/grease-webassembly.git

# Update submodules
git submodule update --init --recursive
```

This provides the same functionality with better version management and update capabilities.