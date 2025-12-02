# Debian Package Build Scripts

This directory contains scripts for building Debian/Ubuntu packages for the Grease programming language.

## Files

- `build_deb.sh` - Main script for building Debian packages

## Usage

### Standard Build
```bash
./build_deb.sh
```
Creates a stable release package using the version from `Cargo.toml`.

### Nightly Build
```bash
./build_deb.sh --nightly
```
Creates a nightly package with the current Git commit hash in the version:
- Format: `0.1.1-nightly-{commit_short}`
- Example: `0.1.1-nightly-3f520af`

### Help
```bash
./build_deb.sh --help
```

## Package Features

The generated Debian package includes:
- **Binary**: `/usr/bin/grease`
- **Man page**: `/usr/share/man/man1/grease.1.gz`
- **Documentation**: `/usr/share/doc/grease/`
  - README.md
  - LSP_README.md
  - TODO.md
- **Shell completions**: 
  - Bash: `/usr/share/bash-completion/completions/grease`
  - Zsh: `/usr/share/zsh/site-functions/_grease`

## Dependencies

The package only depends on `libc6` since Grease is a statically linked Rust binary.

## Installation

After building, install with:
```bash
sudo dpkg -i grease_*.deb
```

For nightly builds:
```bash
sudo dpkg -i grease_*-nightly-*.deb
```

## CI/CD Integration

This script is used in GitLab CI/CD for automated nightly builds. The `nightly-deb` job automatically:
1. Runs this script with `--nightly` flag
2. Uploads the resulting `.deb` package as a CI artifact
3. Makes it available for download from GitLab

## Version Information

- **Stable packages**: Use version from `Cargo.toml` (e.g., `0.1.1`)
- **Nightly packages**: Include commit hash (e.g., `0.1.1-nightly-3f520af`)
- **Package architecture**: `amd64`
- **Maintainer**: Nick Girga <nickgirga@gmail.com>

## Troubleshooting

### Build Failures
- Ensure Rust/Cargo is installed and in PATH
- Check that all source files are present
- Verify Git repository is initialized for nightly builds

### Dependency Issues
- The package should only require `libc6`
- If other dependencies appear, check the build configuration

### Version Conflicts
- Remove existing Grease installation before installing new package:
  ```bash
  sudo dpkg -r grease
  sudo dpkg -P grease  # Purge configuration files
  ```