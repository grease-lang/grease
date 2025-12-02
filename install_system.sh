#!/bin/bash

# Create a proper installation with package manager support

# 1. Build the release binary
cargo build --release

# 2. Create installation directories
sudo mkdir -p /usr/local/bin
sudo mkdir -p /usr/local/share/man/man1
sudo mkdir -p /usr/local/share/doc/grease

# 3. Install binary
sudo cp target/release/grease /usr/local/bin/
sudo chmod +x /usr/local/bin/grease

# 4. Create man page
cat > grease.1 << 'EOF'
.TH GREASE 1 "December 2024" "Grease v0.1.0" "User Commands"

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
cp examples/*.grease /usr/local/share/doc/grease/ 2>/dev/null || true

echo "✅ Grease installed successfully!"
echo "📖 Run 'man grease' for documentation"
echo "🧪 Run 'grease' to start the REPL"