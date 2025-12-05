# Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
# SPDX-License-Identifier: Apache-2.0

#!/bin/bash

# Create Debian package for Grease
# Usage: ./build_deb.sh [--nightly] [--features FEATURES]
#   --nightly  Build a nightly package with commit hash in version
#   --features  Build with specified Cargo features (e.g., "ui")

set -e

# Show usage if requested
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: $0 [--nightly]"
    echo "  --nightly  Build a nightly package with commit hash in version"
    echo "  --help     Show this help message"
    exit 0
fi

PACKAGE_NAME="grease"
ARCHITECTURE="amd64"
MAINTAINER="Nick Girga <nickgirga@gmail.com>"

# Parse arguments
NIGHTLY_BUILD=false
FEATURES=""

while [ $# -gt 0 ]; do
    case $1 in
        --nightly)
            NIGHTLY_BUILD=true
            shift
            ;;
        --features)
            FEATURES="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Check if this is a nightly build
if [ "$NIGHTLY_BUILD" = true ]; then
    COMMIT_SHORT=$(git rev-parse --short HEAD)
    BASE_VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    VERSION="${BASE_VERSION}-nightly-${COMMIT_SHORT}"
    echo "🌙 Building nightly package: $VERSION"
elif [ -n "$FEATURES" ]; then
    VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    echo "🔧 Building package with features: $FEATURES"
else
    VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    echo "📦 Building stable package: $VERSION"
fi
    
    # Update Cargo.toml with nightly version
    sed -i "s/version = \"$BASE_VERSION\"/version = \"$VERSION\"/" Cargo.toml
    echo "📝 Updated Cargo.toml version to: $(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')"
    
    # Update man page version
    sed -i "s/\"grease $BASE_VERSION\"/\"grease $VERSION\"/" docs/grease.1
    sed -i "s/v$BASE_VERSION/v$VERSION/" docs/grease.1
    echo "📝 Updated docs/grease.1 version to: $VERSION"
    

    
    # Clean build cache to ensure version is picked up
    echo "🧹 Cleaning build cache..."
    cargo clean
    
    # Build the binary
    echo "🔨 Building Grease..."
    cargo build --release
    
    # Restore original versions
    sed -i "s/version = \"$VERSION\"/version = \"$BASE_VERSION\"/" Cargo.toml
    sed -i "s/\"grease $VERSION\"/\"grease $BASE_VERSION\"/" docs/grease.1
    sed -i "s/v$VERSION/v$BASE_VERSION/" docs/grease.1
    echo "🔄 Restored original versions"
else
    VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    echo "📦 Building stable package: $VERSION"
    
    # Build the binary
    echo "🔨 Building Grease..."
    cargo build --release
fi

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