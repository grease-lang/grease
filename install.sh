# Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
# SPDX-License-Identifier: Apache-2.0

#!/bin/bash

# Grease Installation Script
set -e

echo "🦀 Installing Grease Scripting Language..."
echo "The high-performance oil for your Rust engine."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build Grease
echo "🔨 Building Grease..."
cargo build --release

# Install to system directory
INSTALL_DIR="/usr/local/bin"
if [ -w "$INSTALL_DIR" ]; then
    echo "📦 Installing to $INSTALL_DIR..."
    sudo cp target/release/grease "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/grease"
else
    echo "📦 Installing to ~/.local/bin..."
    mkdir -p ~/.local/bin
    cp target/release/grease ~/.local/bin/
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