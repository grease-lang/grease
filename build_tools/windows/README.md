# Windows Build Scripts

This directory contains scripts to build Windows binaries for Grease via cross-compilation from Linux.

## Prerequisites

- **Rust toolchain**: Rust 1.91.1 with Cargo
- **MinGW toolchain**: For Windows cross-compilation (installed via package manager)
- **Cross-compilation**: Direct cargo cross-compilation to Windows targets
- **Linux host**: Cross-compilation is performed on Linux systems

## Usage

```bash
./build_windows.sh --arch <arch> [--nightly] [--features FEATURES]
```

### Options

- `--arch x64|x86`: Required. Target architecture
  - `x64`: 64-bit Windows (x86_64-pc-windows-gnu)
  - `x86`: 32-bit Windows (i686-pc-windows-gnu)
- `--nightly`: Optional. Build a nightly version with commit hash in version string
- `--features FEATURES`: Optional. Build with specified Cargo features (e.g., "ui")

## Examples

### Build stable 64-bit Windows binary
```bash
./build_windows.sh --arch x64
```

### Build nightly 32-bit Windows binary
```bash
./build_windows.sh --arch x86 --nightly
```

### Build with UI features
```bash
./build_windows.sh --arch x64 --features ui
```

### Build nightly with UI features
```bash
./build_windows.sh --arch x86 --nightly --features ui
```

## Output

- **Binary location**: `target/<target>/release/grease.exe`
  - For x64: `target/x86_64-pc-windows-gnu/release/grease.exe`
  - For x86: `target/i686-pc-windows-gnu/release/grease.exe`
- **Testing**: The script runs `cross test` before building to ensure compatibility

## Nightly Builds

When using `--nightly`, the script:
1. Appends `-nightly-<commit-short>` to the version in `Cargo.toml`
2. Builds with the updated version
3. Restores the original version after building

This matches the versioning scheme used in other build scripts (Debian, Arch, etc.).

## Integration with CI/CD

This script can be used in GitLab CI/CD pipelines for automated Windows builds:

```yaml
nightly-windows-x64:
  script:
    - ./build_tools/windows/build_windows.sh --arch x64 --nightly
  artifacts:
    paths:
      - target/x86_64-pc-windows-gnu/release/grease.exe
```

## UI Features on Windows

When building with `--features ui`, additional GTK3 libraries are required:

### Option 1: Using vcpkg
```bash
# For 64-bit builds
vcpkg install gtk3:x64-windows

# For 32-bit builds  
vcpkg install gtk3:x86-windows
```

### Option 2: Using MSYS2
```bash
# For 64-bit builds
pacman -S mingw-w64-x86_64-gtk3

# For 32-bit builds
pacman -S mingw-w64-i686-gtk3
```

### Option 3: Using Chocolatey
```bash
choco install gtksharp
```

After installing GTK3, you may need to set environment variables:
```bash
export GTK_LIB_DIR=/path/to/gtk/lib
export GTK_INCLUDE_DIR=/path/to/gtk/include
```

## Troubleshooting

### Cross tool not found
```bash
cargo install cross --locked
```

### Build fails
- Ensure Rust 1.91.1 is installed: `rustup update 1.91.1` or use `rust-toolchain.toml`
- Clean and retry: `cargo clean`
- For UI features: Ensure GTK3 libraries are properly installed

### Permission issues
- Ensure the script is executable: `chmod +x build_windows.sh`

### UI feature build issues
- Verify GTK3 installation path is correct
- Check that architecture matches (x64 vs x86)
- Ensure environment variables are set correctly

## Notes

- Binaries are built using MinGW (GNU toolchain) for broad Windows compatibility
- No Windows-specific packaging is performed (unlike Debian/RPM scripts)
- For MSVC-compatible binaries, additional setup would be required (not supported here)