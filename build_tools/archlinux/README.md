# Arch Linux Package Build Scripts

This directory contains PKGBUILD scripts for building Arch Linux packages for the Grease programming language.

## Files

- `nightly/PKGBUILD` - PKGBUILD for nightly builds with Git integration

## Usage

### Nightly Builds

Navigate to the nightly directory and build:

```bash
cd nightly
makepkg -s --noconfirm
```

This will:
1. Clone the latest source from GitLab
2. Extract current commit information
3. Update version with nightly tag
4. Build the optimized binary
5. Create a `.pkg.tar.zst` package

### Installation

Install the built package:

```bash
sudo pacman -U grease-git-*.pkg.tar.zst
```

## Package Features

The generated Arch Linux package includes:
- **Binary**: `/usr/bin/grease`
- **Man page**: `/usr/share/man/man1/grease.1`
- **Documentation**: `/usr/share/doc/grease/`
  - README.md
  - LSP_README.md
- **Shell completions**:
  - Bash: `/usr/share/bash-completion/completions/grease`
  - Zsh: `/usr/share/zsh/site-functions/_grease`

## Version Information

- **Package name**: `grease-git`
- **Version format**: `r{commits}.{commit_short}`
  - Example: `r15.3f520af` (15 commits, short hash 3f520af)
- **Nightly version**: `{base_version}-nightly-{commit_short}`
  - Example: `0.1.1-nightly-3f520af`
- **Architecture**: `x86_64`

## Dependencies

### Build Dependencies
- `rust` - Rust compiler toolchain
- `cargo` - Rust package manager
- `git` - Version control for source checkout

### Runtime Dependencies
- None (statically linked binary)

## PKGBUILD Details

The PKGBUILD script performs these steps:

1. **Version extraction**: Gets commit count and short hash
2. **Source checkout**: Clones from GitLab repository
3. **Version updating**: Modifies Cargo.toml and man page with nightly version
4. **Clean build**: Clears cargo cache to ensure version consistency
5. **Release build**: Compiles optimized binary
6. **Package installation**: Installs all components to package directory

## CI/CD Integration

This PKGBUILD is used in GitLab CI/CD for automated nightly builds. The `nightly-arch` job:
1. Runs `makepkg` in this directory
2. Uploads resulting `.pkg.tar.zst` package as CI artifact
3. Makes it available for download from GitLab artifacts

## Manual Version Override

If you need to build with a specific version, edit the PKGBUILD:

```bash
# In pkgver() function
BASE_VERSION="0.1.1"  # Change this
COMMIT_SHORT="custom"  # Optional custom identifier
```

## Troubleshooting

### Build Failures
- Ensure Rust toolchain is installed: `sudo pacman -S rust cargo git`
- Check internet connectivity for Git repository access
- Verify sufficient disk space for build artifacts

### Permission Issues
- Make sure you have write permissions in the build directory
- Use `makepkg -s` to automatically install build dependencies

### Version Conflicts
- Remove existing Grease installation:
  ```bash
  sudo pacman -R grease-git
  # Or for conflicts with other packages:
  sudo pacman -Rdd grease-git
  ```

### Clean Build
To start fresh:
```bash
makepkg -C  # Clean build directory
makepkg -s --noconfirm
```

## Customization

### Different Repository
To build from a different Git repository, modify the source line:
```bash
source=("grease-git::git+https://gitlab.com/grease-lang/grease.git")
```

### Additional Files
To include additional documentation or files, add to the `package()` function:
```bash
install -Dm644 additional_file "$pkgdir/usr/share/doc/grease/additional_file"
```