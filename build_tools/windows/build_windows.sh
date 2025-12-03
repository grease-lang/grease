# Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
# SPDX-License-Identifier: Apache-2.0

#!/bin/bash

# Build Windows binaries for Grease
# Usage: ./build_windows.sh --arch x64 [--nightly]
#   --arch x64|x86  Target architecture (x64 for 64-bit, x86 for 32-bit)
#   --nightly       Build a nightly version with commit hash in version

set -e

# Show usage if requested
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: $0 --arch <arch> [--nightly]"
    echo ""
    echo "Options:"
    echo "  --arch x64|x86    Target architecture (x64 for 64-bit, x86 for 32-bit)"
    echo "  --nightly         Build a nightly version with commit hash in version"
    echo "  --help            Show this help message"
    exit 0
fi

# Parse arguments
ARCH=""
NIGHTLY=false

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
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

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

# Check if cross is available
if ! command -v cross &> /dev/null; then
    echo "❌ cross tool not found. Please install it:"
    echo "   cargo install cross --locked"
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
cross test --target "$TARGET"
cross build --release --target "$TARGET"

# Restore original version if nightly
if [ "$NIGHTLY" = true ]; then
    sed -i "s/version = \"$VERSION\"/version = \"$BASE_VERSION\"/" Cargo.toml
    echo "🔄 Restored original version"
fi

BINARY_PATH="target/$TARGET/release/grease.exe"
echo "✅ Build complete!"
echo "📦 Binary available at: $BINARY_PATH"