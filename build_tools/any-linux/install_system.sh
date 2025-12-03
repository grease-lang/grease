# Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
# SPDX-License-Identifier: Apache-2.0

#!/bin/bash

# Create a proper installation with package manager support

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

# 1. Build the release binary
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

# 2. Create installation directories
sudo mkdir -p /usr/local/bin
sudo mkdir -p /usr/local/share/man/man1
sudo mkdir -p /usr/local/share/doc/grease

# 3. Install binary
sudo cp "$BINARY_PATH" /usr/local/bin/
sudo chmod +x /usr/local/bin/grease

# 4. Create man page
VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
cat > grease.1 << EOF
.TH GREASE 1 "December 2024" "Grease v$VERSION" "User Commands"

.SH NAME
Grease \- A modern scripting language written in Rust

.SH SYNOPSIS
.B grease
[script_file]

.SH DESCRIPTION
Grease is a scripting language written in pure Rust.
It compiles to platform-agnostic bytecode and runs on a custom virtual machine.

.SH OPTIONS
.TP
.B script_file
Path to a Grease script file to execute. If not provided, starts interactive REPL mode.

.SH EXAMPLES
.TP
.B grease
Start interactive REPL mode.
.TP
.B grease script.grease
Execute a Grease script.
.TP
.B echo 'print("Hello")' | grease
Execute Grease code via pipe.

.SH SEE ALSO
The full documentation is available at: https://github.com/your-repo/grease

.SH AUTHOR
Grease Development Team
EOF

sudo gzip -c grease.1 > /usr/local/share/man/man1/grease.1.gz

# 5. Install documentation
cp README.md /usr/local/share/doc/grease/
cp docs/*.md /usr/local/share/doc/grease/ 2>/dev/null || true
cp examples/*.grease /usr/local/share/doc/grease/ 2>/dev/null || true

echo "✅ Grease installed successfully!"
echo "📖 Run 'man grease' for documentation"
echo "🧪 Run 'grease' to start the REPL"