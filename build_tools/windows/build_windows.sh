# Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
# SPDX-License-Identifier: Apache-2.0

#!/bin/bash

# Build Windows binaries for Grease
# Usage: ./build_windows.sh --arch x64|x86 [--nightly] [--features FEATURES]
#   --arch x64|x86  Target architecture (x64 for 64-bit, x86 for 32-bit)
#   --nightly       Build a nightly version with commit hash in version
#   --features FEATURES  Build with specified Cargo features (e.g., "ui")

set -e

# Show usage if requested
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: $0 --arch <arch> [--nightly] [--features FEATURES]"
    echo ""
    echo "Options:"
    echo "  --arch x64|x86    Target architecture (x64 for 64-bit, x86 for 32-bit)"
    echo "  --nightly         Build a nightly version with commit hash in version"
    echo "  --features FEATURES  Build with specified Cargo features (e.g., \"ui\")"
    echo "  --help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --arch x64                    # Build stable 64-bit"
    echo "  $0 --arch x86 --nightly          # Build nightly 32-bit"
    echo "  $0 --arch x64 --features ui      # Build with UI features"
    echo "  $0 --arch x86 --nightly --features ui  # Build nightly with UI features"
    echo ""
    echo "Use --help for usage information"
    exit 0
fi

# Initialize variables
ARCH=""
NIGHTLY=false
FEATURES=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --arch)
            ARCH="$2"
            shift 2
            ;;
        --nightly)
            NIGHTLY=true
            shift
            ;;
        --features)
            FEATURES="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 --arch <arch> [--nightly] [--features FEATURES]"
            echo ""
            echo "Options:"
            echo "  --arch x64|x86    Target architecture (x64 for 64-bit, x86 for 32-bit)"
            echo "  --nightly         Build a nightly version with commit hash in version"
            echo "  --features FEATURES  Build with specified Cargo features (e.g., \"ui\")"
            echo "  --help            Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --arch x64                    # Build stable 64-bit"
            echo "  $0 --arch x86 --nightly          # Build nightly 32-bit"
            echo "  $0 --arch x64 --features ui      # Build with UI features"
            echo "  $0 --arch x86 --nightly --features ui  # Build nightly with UI features"
            echo ""
            echo "Use --help for usage information"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Validate required arguments
if [ -z "$ARCH" ]; then
    echo "❌ Missing required argument: --arch"
    echo "Supported: x64, x86"
    echo "Use --help for usage information"
    exit 1
fi

# Validate arch
if [ "$ARCH" != "x64" ] && [ "$ARCH" != "x86" ]; then
    echo "❌ Invalid architecture: $ARCH"
    echo "Supported: x64, x86"
    echo "Use --help for usage information"
    exit 1
fi

# Set target based on arch
if [ "$ARCH" = "x64" ]; then
    TARGET="x86_64-pc-windows-gnu"
else
    TARGET="i686-pc-windows-gnu"
fi

echo "🦀 Building Grease for Windows $ARCH..."
echo "Target: $TARGET"

# Check if Rust target is installed
if ! rustup target list --installed | grep -q "$TARGET"; then
    echo "📦 Installing Windows target: $TARGET"
    rustup target add "$TARGET"
fi

# Check for MinGW toolchain
if [ "$ARCH" = "x64" ]; then
    MINGW_PREFIX="x86_64-w64-mingw32"
else
    MINGW_PREFIX="i686-w64-mingw32"
fi

# Add UI dependencies for Windows builds
if [ -n "$FEATURES" ] && [[ "$FEATURES" == *"ui"* ]]; then
    echo "📦 Installing additional Windows dependencies for UI features..."
    echo "⚠️  Warning: UI features on Windows require GTK3 libraries"
    echo ""
    echo "   Option 1 - Using vcpkg:"
    echo "     vcpkg install gtk3:x64-windows    # For 64-bit builds"
    echo "     vcpkg install gtk3:x86-windows    # For 32-bit builds"
    echo ""
    echo "   Option 2 - Using MSYS2:"
    if [ "$ARCH" = "x64" ]; then
        echo "     pacman -S mingw-w64-x86_64-gtk3"
    else
        echo "     pacman -S mingw-w64-i686-gtk3"
    fi
    echo ""
    echo "   Option 3 - Using Chocolatey:"
    echo "     choco install gtksharp"
    echo ""
    echo "   After installing, set environment variables:"
    echo "     export GTK_LIB_DIR=/path/to/gtk/lib"
    echo "     export GTK_INCLUDE_DIR=/path/to/gtk/include"
fi

if ! command -v "${MINGW_PREFIX}-gcc" &> /dev/null; then
    echo "❌ MinGW toolchain not found. Please install it:"
    echo "   On Ubuntu/Debian: sudo apt-get install gcc-mingw-w64-$ARCH"
    echo "   On Arch: sudo pacman -S mingw-w64-gcc"
    exit 1
fi

# Handle nightly version
if [ "$NIGHTLY" = true ]; then
    COMMIT_SHORT=$(git rev-parse --short HEAD)
    BASE_VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    VERSION="${BASE_VERSION}-nightly-${COMMIT_SHORT}"
    echo "🌙 Building nightly version: $VERSION"
    
    # Update Cargo.toml with nightly version
    sed -i "s/version = \"$BASE_VERSION\"/version = \"$VERSION\"/" Cargo.toml
    echo "📝 Updated Cargo.toml version to: $(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')"

    # Clean build cache to ensure version is picked up
    echo "🧹 Cleaning build cache..."
    cargo clean
fi

# Build the binary
echo "🔨 Building Grease..."
export RUSTFLAGS="-C target-feature=+crt-static"
export CC="${MINGW_PREFIX}-gcc"
export CXX="${MINGW_PREFIX}-g++"

# Prepare build commands with features if specified
TEST_CMD="cargo test --target $TARGET"
BUILD_CMD="cargo build --release --target $TARGET"

if [ -n "$FEATURES" ]; then
    TEST_CMD="$TEST_CMD --features $FEATURES"
    BUILD_CMD="$BUILD_CMD --features $FEATURES"
    echo "🎯 Building with features: $FEATURES"
fi

# Execute build
$TEST_CMD
$BUILD_CMD

# Restore original version if nightly
if [ "$NIGHTLY" = true ]; then
    sed -i "s/version = \"$VERSION\"/version = \"$BASE_VERSION\"/" Cargo.toml
    echo "🔄 Restored original version"
fi

BINARY_PATH="target/$TARGET/release/grease.exe"
echo "✅ Build complete!"
echo "📦 Binary available at: $BINARY_PATH"

# Show build summary
echo ""
echo "📋 Build Summary:"
echo "   Architecture: $ARCH ($TARGET)"
if [ -n "$FEATURES" ]; then
    echo "   Features: $FEATURES"
fi
if [ "$NIGHTLY" = true ]; then
    echo "   Version: $VERSION (nightly)"
else
    BASE_VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    echo "   Version: $BASE_VERSION"
fi
echo "   Binary: $BINARY_PATH"
echo ""