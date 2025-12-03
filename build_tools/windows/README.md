# Windows Build Scripts

This directory contains scripts to build Windows binaries for Grease via cross-compilation from Linux.

## Prerequisites

- **Rust toolchain**: Latest stable Rust with Cargo
- **Cross tool**: For cross-compilation to Windows
  ```bash
  cargo install cross --locked
  ```
- **Linux host**: Cross-compilation is performed on Linux systems

## Usage

```bash
./build_windows.sh --arch <arch> [--nightly]
```

### Options

- `--arch x64|x86`: Required. Target architecture
  - `x64`: 64-bit Windows (x86_64-pc-windows-gnu)
  - `x86`: 32-bit Windows (i686-pc-windows-gnu)
- `--nightly`: Optional. Build a nightly version with commit hash in version string

## Examples

### Build stable 64-bit Windows binary
```bash
./build_windows.sh --arch x64
```

### Build nightly 32-bit Windows binary
```bash
./build_windows.sh --arch x86 --nightly
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

## Troubleshooting

### Cross tool not found
```bash
cargo install cross --locked
```

### Build fails
- Ensure Rust is up to date: `rustup update`
- Clean and retry: `cargo clean`

### Permission issues
- Ensure the script is executable: `chmod +x build_windows.sh`

## Notes

- Binaries are built using MinGW (GNU toolchain) for broad Windows compatibility
- No Windows-specific packaging is performed (unlike Debian/RPM scripts)
- For MSVC-compatible binaries, additional setup would be required (not supported here)