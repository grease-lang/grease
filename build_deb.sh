#!/bin/bash

# Create Debian package for Grease

set -e

PACKAGE_NAME="grease"
VERSION="0.1.1"
ARCHITECTURE="amd64"
MAINTAINER="Grease Developers <dev@grease-lang.org>"

# Build the binary
echo "🔨 Building Grease..."
cargo build --release

# Create package structure
PKG_DIR="$PACKAGE_NAME-$VERSION"
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR/DEBIAN"
mkdir -p "$PKG_DIR/usr/local/bin"
mkdir -p "$PKG_DIR/usr/share/doc/$PACKAGE_NAME"
mkdir -p "$PKG_DIR/usr/share/man/man1"

# Copy files
cp target/release/grease "$PKG_DIR/usr/local/bin/"
cp README.md "$PKG_DIR/usr/share/doc/$PACKAGE_NAME/"
cp examples/*.grease "$PKG_DIR/usr/share/doc/$PACKAGE_NAME/" 2>/dev/null || true

# Create control file
cat > "$PKG_DIR/DEBIAN/control" << EOF
Package: $PACKAGE_NAME
Version: $VERSION
Section: interpreters
Priority: optional
Architecture: $ARCHITECTURE
Depends: libc6
Maintainer: $MAINTAINER
 Description: Grease Scripting Language
  A modern scripting language written in pure Rust. Compiles to platform-agnostic bytecode.
  The high-performance oil for your Rust engine.
EOF

# Create man page
cat > "$PKG_DIR/usr/share/man/man1/grease.1" << 'MANEOF'
.TH GREASE 1 "December 2024" "Grease v0.1.1" "User Commands"

.SH NAME
Grease \- A modern scripting language written in Rust

.SH SYNOPSIS
.B grease
[script_file]

.SH DESCRIPTION
Grease is a scripting language written in pure Rust.
It compiles to platform-agnostic bytecode and runs on a custom virtual machine.
.PP
The high-performance oil for your Rust engine.

.SH EXAMPLES
.TP
.B grease
Start interactive REPL mode.
.TP
.B grease script.grease
Execute a Grease script.
MANEOF

gzip -9 "$PKG_DIR/usr/share/man/man1/grease.1"

# Create copyright file
cat > "$PKG_DIR/DEBIAN/copyright" << EOF
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: Grease
Upstream-Contact: dev@grease-lang.org
Source: https://github.com/your-repo/grease
Disclaimer: This software comes with absolutely no warranty.

License: MIT
 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:
 .
 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 DEALINGS IN THE SOFTWARE.
EOF

# Calculate installed size
INSTALLED_SIZE=$(du -s "$PKG_DIR" | cut -f1)
echo "Installed-Size: $INSTALLED_SIZE" >> "$PKG_DIR/DEBIAN/control"

# Build the package
echo "📦 Building Debian package..."
dpkg-deb --build "$PKG_DIR"

# Clean up
rm -rf "$PKG_DIR"

echo "✅ Debian package created: ${PACKAGE_NAME}_${VERSION}_${ARCHITECTURE}.deb"
echo "📦 Install with: sudo dpkg -i ${PACKAGE_NAME}_${VERSION}_${ARCHITECTURE}.deb"