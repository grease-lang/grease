# Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
# SPDX-License-Identifier: Apache-2.0

#!/bin/bash

# Grease Installation Script
set -e

# Parse command line arguments
ARCH=""
while [[ $# -gt 0 ]]; do
    case $1 in
        --arch)
            ARCH="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [--arch <architecture>]"
            echo ""
            echo "Options:"
            echo "  --arch <arch>    Target architecture (armeabi-v7a, arm64-v8a, x86_64, riscv64, i686, i386)"
            echo "                   Uses Fedora naming scheme. Auto-detects if not specified."
            echo "  --help           Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo "🦀 Installing Grease Scripting Language..."
echo "The high-performance oil for your Rust engine."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Determine target architecture
if [ -z "$ARCH" ]; then
    echo "🔍 Auto-detecting architecture..."
    MACHINE=$(uname -m)
    case $MACHINE in
        x86_64)
            TARGET="x86_64-unknown-linux-gnu"
            ;;
        aarch64)
            TARGET="aarch64-unknown-linux-gnu"
            ;;
        armv7l)
            TARGET="armv7-unknown-linux-gnueabihf"
            ;;
        riscv64)
            TARGET="riscv64gc-unknown-linux-gnu"
            ;;
        i686|i386)
            TARGET="i686-unknown-linux-gnu"
            ;;
        *)
            echo "⚠️  Unknown architecture: $MACHINE. Attempting native build."
            TARGET=""
            ;;
    esac
else
    echo "🎯 Building for architecture: $ARCH"
    case $ARCH in
        x86_64)
            TARGET="x86_64-unknown-linux-gnu"
            ;;
        arm64-v8a)
            TARGET="aarch64-unknown-linux-gnu"
            ;;
        armeabi-v7a)
            TARGET="armv7-unknown-linux-gnueabihf"
            ;;
        riscv64)
            TARGET="riscv64gc-unknown-linux-gnu"
            ;;
        i686|i386)
            TARGET="i686-unknown-linux-gnu"
            ;;
        *)
            echo "❌ Unsupported architecture: $ARCH"
            echo "Supported: x86_64, arm64-v8a, armeabi-v7a, riscv64, i686, i386"
            exit 1
            ;;
    esac
fi

# Build Grease
echo "🔨 Building Grease..."
if [ -z "$TARGET" ]; then
    # Native build
    cargo build --release
    BINARY_PATH="target/release/grease"
else
    # Cross-compilation
    if ! command -v cross &> /dev/null; then
        echo "📦 Installing cross for cross-compilation..."
        cargo install cross --locked
    fi
    cross build --release --target "$TARGET"
    BINARY_PATH="target/$TARGET/release/grease"
fi

# Install to system directory
INSTALL_DIR="/usr/local/bin"
if [ -w "$INSTALL_DIR" ]; then
    echo "📦 Installing to $INSTALL_DIR..."
    sudo cp "$BINARY_PATH" "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/grease"
else
    echo "📦 Installing to ~/.local/bin..."
    mkdir -p ~/.local/bin
    cp "$BINARY_PATH" ~/.local/bin/
    chmod +x ~/.local/bin/grease
    
    # Add to PATH if not already there
    if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
        echo "✅ Added ~/.local/bin to PATH. Please restart your shell or run:"
        echo "   source ~/.bashrc"
    fi
fi

# Verify installation
echo "✅ Installation complete!"
echo "🧪 Testing installation..."

if command -v grease &> /dev/null; then
    echo "🎉 Grease is now available system-wide!"
    echo ""
    echo "Try it out:"
    echo "  grease                    # Start REPL"
    echo "  grease script.grease       # Run script"
    echo "  echo 'print(42)' | grease  # Pipe input"
    echo ""
    echo "📖 For more info: https://github.com/your-repo/grease"
else
    echo "❌ Installation failed. Please check your PATH."
    exit 1
fi