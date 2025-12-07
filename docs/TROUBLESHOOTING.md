# Troubleshooting Guide

This guide helps resolve common issues with Grease modules and platform setup.

## Module Issues

### Module Not Found

**Error Message**:
```
❌ Module 'grease-ui' not found

🔍 Searched in:
  - ./grease-ui
  - ../grease-ui
  - ./modules/grease-ui
  - ./lib/grease-ui
  - /custom/path/from/env

💡 To fix:
1. Copy module folder to one of the locations above
2. Set environment variable:
   export GREASE_UI_PATH=/path/to/grease-ui
3. Ensure module folder contains Cargo.toml
```

**Common Causes**:
- Module folder not copied to project
- Incorrect environment variable path
- Missing `Cargo.toml` in module folder
- Typos in folder names

**Solutions**:
```bash
# Copy modules to project
cp -r /path/to/grease/grease-ui ./
cp -r /path/to/grease/grease-webassembly ./

# Set environment variables
export GREASE_UI_PATH=/absolute/path/to/grease-ui
export GREASE_WASM_PATH=/absolute/path/to/grease-webassembly

# Verify module structure
ls grease-ui/
# Should show: Cargo.toml, src/, etc.

# Check search paths
grease --verbose script.grease
# Shows searched locations
```

### Version Mismatch

**Error Message**:
```
⚠️ Version mismatch: Module 'grease-ui' (0.2.0) vs Core (0.1.0)

🎯 Compatibility: Incompatible

💡 Recommended action: Will not work
```

**Common Causes**:
- Module version newer than core
- Core version outdated
- Development version mismatch

**Solutions**:
```bash
# Update core to match module
cargo update
cargo install --force grease

# Update module to match core
cd grease-ui
cargo update
# Edit Cargo.toml to match core version

# Check current versions
grease --version
cd grease-ui && grep version Cargo.toml
```

### Build Failures

**Linux GTK Issues**:
```
❌ Module 'grease-ui' build failed

🔧 Error: Could not find library 'gtk-3'

💡 Suggestions:
  - Install GTK development libraries
  - Check pkg-config paths
  - Verify library versions
```

**Solutions**:
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y libgtk-3-dev libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install -y gtk3-devel webkit2gtk4.0-devel

# Arch Linux
sudo pacman -S gtk3 webkit2gtk

# Check GTK installation
pkg-config --modversion gtk+-3.0
pkg-config --cflags gtk+-3.0
```

**Windows Build Issues**:
```
❌ Module 'grease-ui' build failed

🔧 Error: 'gtk.h' file not found

💡 Suggestions:
  - Install Visual Studio Build Tools
  - Install vcpkg
  - Use MSYS2 environment
```

**Solutions**:
```cmd
REM Install vcpkg
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg integrate install

REM Install GTK
vcpkg install gtk3:x64-windows

REM Set environment
set VCPKG_ROOT=C:\vcpkg
set INCLUDE=%VCPKG_ROOT%\installed\x64-windows\include;%INCLUDE%
set LIB=%VCPKG_ROOT%\installed\x64-windows\lib;%LIB%

REM Install Visual Studio Build Tools
# Download from Visual Studio Installer
```

**macOS Build Issues**:
```
❌ Module 'grease-ui' build failed

🔧 Error: 'Cocoa/Cocoa.h' file not found

💡 Suggestions:
  - Install Xcode command line tools
  - Install Homebrew
  - Set up proper SDK paths
```

**Solutions**:
```bash
# Install Xcode tools
xcode-select --install

# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install GTK via Homebrew
brew install gtk+3

# Set SDK paths
sudo xcode-select -switch /Applications/Xcode.app/Contents/Developer
```

## Runtime Issues

### UI Not Displaying

**Linux**:
```bash
# Check GTK theme
echo $GTK_THEME

# Check display server
echo $XDG_SESSION_TYPE
echo $WAYLAND_DISPLAY

# Test GTK
gtk3-demo

# Check for Wayland
echo $WAYLAND_DISPLAY

# Force X11 if needed
export GDK_BACKEND=x11
```

**Windows**:
```cmd
REM Check display settings
dxdiag

REM Check graphics drivers
dxdiag /t64

REM Test GTK
gtk3-demo.exe

REM Check DLL dependencies
dumpbin /DEPENDENTS grease-ui.dll
```

**macOS**:
```bash
# Check window system
echo $MACOSX_DEPLOYMENT_TARGET

# Check graphics
system_profiler SPDisplaysDataType

# Test Cocoa
# Create simple test app with Xcode
```

### Performance Issues

**High Memory Usage**:
```bash
# Monitor memory usage
htop  # Linux
Task Manager  # Windows
Activity Monitor  # macOS

# Check for memory leaks
valgrind --leak-check=full ./grease

# Profile memory usage
cargo build --release
perf record ./grease
```

**Slow UI Response**:
```bash
# Check CPU usage
top  # Linux
Task Manager  # Windows
Activity Monitor  # macOS

# Profile application
perf record ./grease
perf report

# Check for blocking operations
strace -p $(pidof grease) 2>&1 | grep -E "(futex|sleep|wait)"
```

### WebAssembly Issues

**Module Not Loading in Browser**:
```javascript
// Check browser console
console.log('Checking WebAssembly support');

// Test WebAssembly
if (typeof WebAssembly === 'undefined') {
    console.error('WebAssembly not supported');
}

// Check for errors
WebAssembly.compileStreaming(wasmCode)
    .catch(err => console.error('WebAssembly compilation failed:', err));
```

**Build Target Issues**:
```bash
# Check installed targets
rustup target list --installed

# Install WebAssembly target
rustup target add wasm32-unknown-unknown

# Build for WebAssembly
cargo build --target wasm32-unknown-unknown

# Verify WebAssembly output
file target/wasm32-unknown-unknown/release/grease_webassembly.wasm
```

## Platform-Specific Issues

### Linux

#### GTK Theme Issues
```bash
# List available themes
ls /usr/share/themes/

# Set GTK theme
export GTK_THEME=Adwaita:dark

# Check current theme
gsettings get org.gnome.desktop.interface gtk-theme
```

#### Display Server Issues
```bash
# Check if running Wayland
echo $WAYLAND_DISPLAY

# Check X11 server
echo $DISPLAY

# Force X11 on Wayland
export GDK_BACKEND=x11

# Test GTK backend
GDK_BACKEND=wayland gtk3-demo
GDK_BACKEND=x11 gtk3-demo
```

#### Font Issues
```bash
# List available fonts
fc-list | grep -i arial

# Update font cache
fc-cache -fv

# Check GTK font configuration
cat ~/.config/gtk-3.0/settings.ini
```

### Windows

#### DLL Missing Errors
```cmd
REM Check missing DLLs
dumpbin /DEPENDENTS grease-ui.dll

REM Use Dependency Walker
# Download from Microsoft

REM Install Visual C++ Redistributable
# Download from Microsoft website
```

#### Registry Issues
```cmd
REM Check Grease registry entries
reg query HKEY_LOCAL_MACHINE\SOFTWARE\Grease

REM Add to PATH if needed
echo %PATH%

REM Set environment variables
setx GREASE_UI_PATH "C:\path\to\grease-ui"
setx PATH "%PATH%;C:\path\to\grease-ui\bin"
```

### macOS

#### Code Signing Issues
```bash
# Check app signature
spctl -a -v /Applications/Grease.app

# Allow unsigned apps (development only)
sudo spctl --master-disable

# Ad-hoc signing
codesign --force --deep --sign "Developer ID Application" Grease.app
```

#### Permission Issues
```bash
# Check app permissions
ls -la@ /Applications/Grease.app

# Reset permissions
sudo chmod -R 755 /Applications/Grease.app

# Check quarantine
xattr -l /Applications/Grease.app

# Remove quarantine
sudo xattr -d com.apple.quarantine /Applications/Grease.app
```

### Android

#### NDK Issues
```bash
# Check NDK installation
echo $ANDROID_NDK_ROOT
echo $ANDROID_HOME

# Set NDK path manually
export ANDROID_NDK_ROOT=/opt/android-ndk

# Check NDK tools
ls $ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/
```

#### Device Issues
```bash
# Check connected devices
adb devices

# Install APK
adb install app-debug.apk

# Check app logs
adb logcat | grep Grease

# Clear app data
adb shell pm clear com.grease.app
```

### WebAssembly

#### Browser Compatibility
```javascript
// Check WebAssembly support
if (typeof WebAssembly === 'undefined') {
    alert('Your browser does not support WebAssembly');
}

// Check specific features
const wasmSupported = (() => {
    try {
        if (typeof WebAssembly === 'object'
            && typeof WebAssembly.instantiate === 'function') {
            const module = new WebAssembly.Module(Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00));
            new WebAssembly.Instance(module, {}).exports;
            return true;
        }
    } catch (e) {
        return false;
    }
})();

if (!wasmSupported) {
    console.error('WebAssembly not fully supported');
}
```

#### CORS Issues
```javascript
// Check for CORS errors
fetch('/path/to/wasm/module')
    .then(response => {
        if (!response.ok) {
            console.error('CORS or network error:', response.status);
        }
    })
    .catch(error => console.error('Network error:', error));
```

## Development Issues

### Compilation Errors

**Rust Version Mismatch**:
```bash
# Check Rust version
rustc --version

# Update Rust
rustup update stable
rustup default stable

# Check project Rust version
cat rust-toolchain.toml
```

**Dependency Conflicts**:
```bash
# Clean build cache
cargo clean

# Update dependencies
cargo update

# Check dependency tree
cargo tree | grep -E "(conflict|duplicate)"

# Rebuild with clean state
cargo build --release --target x86_64-unknown-linux-gnu
```

### Test Failures

**Module Tests**:
```bash
# Run module tests
cd grease-ui
cargo test

# Run with specific features
cargo test --features ui

# Run integration tests
cargo test --test integration
```

**Cross-Platform Tests**:
```bash
# Test multiple targets
cargo test --target x86_64-unknown-linux-gnu
cargo test --target x86_64-pc-windows-msvc
cargo test --target aarch64-apple-darwin

# Use cross for easier cross-compilation
cargo install cross
cross test --target x86_64-unknown-linux-gnu
```

## Getting Help

### Community Resources
- **GitHub Issues**: [Submit Bug Report](https://github.com/grease-lang/grease/issues/new)
- **Discord**: [Grease Discord](https://discord.gg/grease)
- **Documentation**: [Grease Docs](https://docs.grease-lang.org)

### Debug Information Collection
```bash
# Generate debug report
grease --verbose --debug script.grease > debug.log 2>&1

# System information
uname -a > system_info.txt
rustc --version >> system_info.txt
cargo --version >> system_info.txt

# Environment variables
env | grep -E "(GREASE|PATH|LD_LIBRARY_PATH)" >> system_info.txt
```

### Performance Profiling
```bash
# CPU profiling
perf record -g ./grease
perf report

# Memory profiling
valgrind --tool=massif ./grease
ms_print massif.out.*

# Flame graphs
cargo install flamegraph
cargo flamegraph --bin grease
```

## Common Solutions

### Quick Fixes
1. **Restart**: Sometimes a simple restart fixes issues
2. **Clean Build**: `cargo clean && cargo build --release`
3. **Update Dependencies**: `cargo update`
4. **Check Permissions**: Ensure read/write access to module directories
5. **Environment Variables**: Verify PATH and custom module paths

### When All Else Fails
1. **Minimal Example**: Start with a simple "Hello, World!" example
2. **Isolate Modules**: Test without UI/WebAssembly modules
3. **Different Platform**: Try on a different machine/OS
4. **Clean Installation**: Remove and reinstall Grease completely
5. **Seek Help**: Ask for help on Discord or GitHub Issues

Remember: Most issues have been encountered and solved before. Check existing issues and documentation first!