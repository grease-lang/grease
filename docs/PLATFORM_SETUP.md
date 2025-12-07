# Platform Setup Guide

This guide provides platform-specific setup instructions for Grease modules.

## Linux

### System Requirements

- **Distribution**: Ubuntu 18.04+, Debian 10+, Fedora 32+, Arch Linux
- **GTK**: Version 3.20 or higher
- **Rust**: 1.70.0 or higher
- **Memory**: 2GB RAM minimum
- **Storage**: 500MB free space

### Installing Dependencies

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libsoup-3.0-dev \
    libxdo-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libssl-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev
```

#### Fedora/CentOS/RHEL
```bash
sudo dnf update
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y \
    pkgconfig \
    gtk3-devel \
    webkit2gtk4.0-devel \
    libsoup-devel \
    libXdo-devel \
    libxcb-devel \
    libxkbcommon-devel \
    openssl-devel \
    mesa-libGL-devel \
    mesa-libGLU-devel
```

#### Arch Linux
```bash
sudo pacman -Syu
sudo pacman -S --needed \
    base-devel \
    pkgconf \
    gtk3 \
    webkit2gtk \
    libsoup \
    libxdo \
    xcb-util \
    libxkbcommon \
    openssl \
    mesa \
    glu
```

### Verification

```bash
# Check GTK installation
pkg-config --modversion gtk+-3.0

# Check WebKit installation
pkg-config --modversion webkit2gtk-4.1

# Test GTK program
gtk3-demo
```

### Troubleshooting

#### GTK Not Found
```bash
# Check if GTK is installed
ldconfig -p | grep gtk

# Install GTK if missing
sudo apt install libgtk-3-dev  # Ubuntu/Debian
sudo dnf install gtk3-devel      # Fedora
sudo pacman -S gtk3               # Arch
```

#### WebKit Issues
```bash
# WebKit2GTK for Ubuntu 20.04+
sudo apt install libwebkit2gtk-4.1-dev

# For older Ubuntu, use webkit2gtk-3.0
sudo apt install libwebkit2gtk-3.0-dev
```

#### Header Files Missing
```bash
# Find GTK headers
find /usr -name "gtk.h" 2>/dev/null

# Set PKG_CONFIG_PATH if needed
export PKG_CONFIG_PATH=/usr/lib/pkgconfig:$PKG_CONFIG_PATH
```

## macOS

### System Requirements

- **macOS**: 10.15 (Catalina) or higher
- **Xcode**: 12.0 or higher
- **Rust**: 1.70.0 or higher
- **Memory**: 4GB RAM minimum
- **Storage**: 1GB free space

### Installing Dependencies

#### Using Homebrew (Recommended)
```bash
# Install Homebrew if not present
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install gtk+3
brew install pkg-config
brew install cmake
```

#### Using MacPorts
```bash
# Install MacPorts if not present
sudo port -v selfupdate

# Install dependencies
sudo port install gtk3 +quartz
sudo port install pkgconfig
sudo port install cmake
```

#### Manual Installation
```bash
# Download and install GTK
curl -O https://download.gnome.org/sources/gtk+/3.24/gtk+-3.24.0.tar.xz
tar xf gtk+-3.24.0.tar.xz
cd gtk+-3.24.0
./configure --prefix=/usr/local
make
sudo make install
```

### Xcode Setup

```bash
# Install Xcode command line tools
xcode-select --install

# Set active developer directory
sudo xcode-select -switch /Applications/Xcode.app/Contents/Developer

# Verify installation
xcode-select -p
```

### Verification

```bash
# Check GTK installation
pkg-config --modversion gtk+-3.0

# Check Cocoa frameworks
ls /System/Library/Frameworks/

# Test compilation
echo 'fn main() {}' > test.c
gcc -o test test.c -framework Cocoa
./test
```

### Troubleshooting

#### GTK Not Found
```bash
# Check Homebrew installation
brew --prefix gtk+3

# Add to PATH if needed
export PATH=$(brew --prefix gtk+3)/bin:$PATH

# Rebuild GTK
brew reinstall gtk+3
```

#### Code Signing Issues
```bash
# Allow unsigned apps (development only)
sudo spctl --master-disable

# Or sign your app
codesign --force --deep --sign "Developer ID" YourApp.app
```

## Windows

### System Requirements

- **Windows**: 10 (1903) or higher
- **Visual Studio**: 2019 or higher (Build Tools)
- **Rust**: 1.70.0 or higher
- **Memory**: 4GB RAM minimum
- **Storage**: 1GB free space

### Installing Dependencies

#### Using vcpkg (Recommended)
```cmd
# Install vcpkg if not present
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat

# Add to PATH
.\vcpkg integrate install

# Install GTK
vcpkg install gtk3:x64-windows

# Install additional dependencies
vcpkg install glib:x64-windows
vcpkg install gdk-pixbuf:x64-windows
vcpkg install pango:x64-windows
vcpkg install cairo:x64-windows
```

#### Using MSYS2 (Alternative)
```cmd
# Install MSYS2
# Download from: https://www.msys2.org/

# Install packages
pacman -S mingw-w64-x86_64-gtk3
pacman -S mingw-w64-x86_64-pkg-config
pacman -S mingw-w64-x86_64-cmake
```

#### Using Chocolatey
```cmd
# Install Chocolatey if not present
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install dependencies
choco install gtk-runtime
choco install pkgconfiglite
choco install cmake
```

### Visual Studio Setup

```cmd
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/

# Or install via Visual Studio Installer
# Select "Desktop development with C++" workload
# Ensure "Windows 10 SDK" and "CMake tools" are included
```

### Environment Variables

```cmd
# Set vcpkg toolchain
set VCPKG_ROOT=C:\vcpkg
set VCPKG_TARGET_TRIPLET=x64-windows

# Add to PATH
set PATH=%VCPKG_ROOT%\installed\x64-windows\bin;%PATH%

# Set Rust target
set RUSTFLAGS=-C target-feature=+crt-static
```

### Verification

```cmd
# Check GTK installation
pkg-config --modversion gtk+-3.0

# Check vcpkg libraries
vcpkg list gtk3

# Test compilation
cargo test --manifest-path grease-ui/Cargo.toml
```

### Troubleshooting

#### Linker Errors
```cmd
# Ensure vcpkg is integrated
vcpkg integrate install

# Check library paths
echo %LIB%
echo %LIBPATH%

# Rebuild with clean environment
cargo clean
cargo build --release
```

#### GTK Not Found
```cmd
# Check vcpkg installation
vcpkg list gtk3

# Reinstall GTK
vcpkg remove gtk3:x64-windows
vcpkg install gtk3:x64-windows
```

#### Runtime Errors
```cmd
# Install GTK runtime
# Download from: https://gtk.org/download/windows.php

# Ensure DLLs are in PATH
copy vcpkg\installed\x64-windows\bin\*.dll C:\Windows\System32\
```

## Android

### System Requirements

- **Android**: API Level 21 (Android 5.0) or higher
- **NDK**: r21 or higher
- **Rust**: 1.70.0 or higher
- **Memory**: 2GB RAM minimum
- **Storage**: 500MB free space

### Installing Dependencies

#### Android Studio Setup
```bash
# Install Android Studio
# Download from: https://developer.android.com/studio

# Install NDK and CMake
# In Android Studio: Tools > SDK Manager > SDK Tools
# - Android NDK (Side by side) 21.0.0+
# - NDK (Side by side) 21.0.0+
# - CMake 3.10.2+
```

#### Command Line Setup
```bash
# Download command line tools
wget https://dl.google.com/android/repository/commandlinetools-linux-8512504_latest.zip
unzip commandlinetools-linux-8512504_latest.zip

# Set up environment
export ANDROID_HOME=/path/to/android/sdk
export ANDROID_NDK_ROOT=/path/to/android/ndk/21.0.0
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin
```

### Rust Target Setup

```bash
# Install Android Rust target
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Install cargo-ndk
cargo install cargo-ndk

# Set default target
rustup default stable
```

### Verification

```bash
# Check NDK installation
echo $ANDROID_NDK_ROOT

# Test compilation
cargo build --target aarch64-linux-android --manifest-path grease-ui/Cargo.toml

# Check for required tools
which ndk-build
which cmake
```

### Troubleshooting

#### NDK Not Found
```bash
# Verify NDK path
ls $ANDROID_NDK_ROOT

# Set NDK path manually
export ANDROID_NDK_ROOT=/opt/android-ndk

# Update .bashrc or .zshrc
echo 'export ANDROID_NDK_ROOT=/opt/android-ndk' >> ~/.bashrc
```

#### Compilation Errors
```bash
# Check API level
echo $ANDROID_MIN_SDK_VERSION

# Update Cargo.toml for Android
[dependencies]
jni = "0.21"
android_logger = "0.13"

[[target.'cfg(target_os = "android")'.dependencies]]
android-activity = "0.4"
android-ndk-sys = "0.6"
```

#### Device Testing
```bash
# Install APK on device
adb install app-debug.apk

# Check logs
adb logcat | grep Grease
```



## Cross-Platform Considerations

### File Paths

- **Linux/macOS**: Use forward slashes `/`
- **Windows**: Use backslashes `\`

### Environment Variables

- **Case Sensitivity**: Linux/macOS are case-sensitive, Windows is not
- **PATH**: Ensure all binary directories are in PATH
- **HOME**: User home directory varies by platform

### Testing

```bash
# Test on multiple platforms
cargo test --target x86_64-unknown-linux-gnu
cargo test --target x86_64-pc-windows-msvc
cargo test --target aarch64-apple-darwin
```

### Performance Optimization

- **Release Builds**: Always use `--release` for production
- **Target-Specific**: Optimize for each platform's characteristics
- **Profile**: Use profiling tools to identify bottlenecks
- **Memory**: Monitor memory usage on each platform

## Getting Help

- **Documentation**: [Grease Docs](https://docs.grease-lang.org)
- **Community**: [Grease Discord](https://discord.gg/grease)
- **Issues**: [GitHub Issues](https://github.com/grease-lang/grease/issues)
- **Discussions**: [GitHub Discussions](https://github.com/grease-lang/grease/discussions)