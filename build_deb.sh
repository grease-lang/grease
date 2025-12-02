# Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
# SPDX-License-Identifier: Apache-2.0

#!/bin/bash

# Create Debian package for Grease

set -e

PACKAGE_NAME="grease"
VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
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
cat > "$PKG_DIR/usr/share/man/man1/grease.1" << MANEOF
.TH GREASE 1 "December 2024" "Grease v$VERSION" "User Commands"

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

License: Apache-2.0
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 .
     http://www.apache.org/licenses/LICENSE-2.0
 .
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
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