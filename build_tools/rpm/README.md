# RPM Package Build Scripts

This directory contains scripts and spec files for building RPM packages for the Grease programming language on Fedora, RHEL, CentOS, and other RPM-based Linux distributions.

## Files

- `grease.spec` - RPM spec file for stable releases
- `build_rpm.sh` - Main script for building RPM packages with support for nightly builds

## Usage

### Prerequisites

Install the required build dependencies:

```bash
# On Fedora/RHEL/CentOS
sudo dnf install rpm-build rust cargo

# On older systems with yum
sudo yum install rpm-build rust cargo

# Ensure Rust 1.85.1 is installed
rustup install 1.85.1
rustup default 1.85.1
```

### Standard Build

Build a stable release package using the version from `Cargo.toml`:

```bash
./build_rpm.sh
```

### Nightly Build

Build a nightly package with the current Git commit hash in the version:

```bash
./build_rpm.sh --nightly
```

This creates a package with version format: `0.1.1-nightly-{commit_short}`
- Example: `0.1.1-nightly-3f520af`

### Custom Version

Build with a specific version:

```bash
./build_rpm.sh --version 0.2.0
```

### Custom Release Number

Build with a custom release number:

```bash
./build_rpm.sh --release 2
```

### Help

Show all available options:

```bash
./build_rpm.sh --help
```

## Package Features

The generated RPM packages include:

### Main Package (`grease`)
- **Binary**: `/usr/bin/grease`
- **Man page**: `/usr/share/man/man1/grease.1`
- **Shell completions**:
  - Bash: `/usr/share/bash-completion/completions/grease`
  - Zsh: `/usr/share/zsh/site-functions/_grease`
- **License**: `/usr/share/licenses/grease/LICENSE`
- **Basic documentation**: `/usr/share/doc/grease/README.md`

### Development Package (`grease-devel`)
- **Additional documentation**: `/usr/share/doc/grease/`
  - LSP_README.md
  - TODO.md
- **Examples**: `/usr/share/doc/grease/examples/`

## Installation

### Install Stable Release

```bash
sudo dnf install rpmbuild/RPMS/*/grease-*.rpm
```

### Install Nightly Release

```bash
sudo dnf install rpmbuild/RPMS/*/grease-nightly-*.rpm
```

### Install Development Package

```bash
sudo dnf install rpmbuild/RPMS/*/grease-devel-*.rpm
```

## Version Information

- **Stable packages**: Use version from `Cargo.toml` (e.g., `0.1.1`)
- **Nightly packages**: Include commit hash (e.g., `0.1.1-nightly-3f520af`)
- **Package architecture**: `x86_64` (automatically detected)
- **Release number**: `1` by default, configurable with `--release`

## Dependencies

### Build Dependencies
- `rust` - Rust compiler toolchain
- `cargo` - Rust package manager
- `rpm-build` - RPM build tools
- `gcc` - C compiler (required by some Rust crates)

### Runtime Dependencies
- None (statically linked binary, only requires glibc)

## Build Process

The `build_rpm.sh` script performs these steps:

1. **Dependency Check**: Verifies all required tools are installed
2. **Version Detection**: Extracts version from `Cargo.toml` or uses provided version
3. **Source Preparation**: Creates source tarball with all necessary files
4. **Spec File Update**: Updates version and release information
5. **RPM Build**: Uses `rpmbuild` to create binary and source packages
6. **Results Display**: Shows built packages and installation commands

## Manual RPM Build

If you prefer to use `rpmbuild` directly:

```bash
# Create rpmbuild directory structure
mkdir -p ~/rpmbuild/{SOURCES,SPECS,RPMS,SRPMS,BUILD}

# Prepare source tarball
tar -czf ~/rpmbuild/SOURCES/grease-0.1.1.tar.gz \
    --exclude-vcs \
    --exclude=target \
    --exclude=rpmbuild \
    .

# Copy spec file
cp grease.spec ~/rpmbuild/SPECS/

# Build package
rpmbuild -ba ~/rpmbuild/SPECS/grease.spec
```

## CI/CD Integration

This build system is designed to work with GitLab CI/CD for automated builds. The `nightly-rpm` job can:

1. Run this script with `--nightly` flag
2. Upload the resulting `.rpm` packages as CI artifacts
3. Make them available for download from GitLab

Example CI configuration:

```yaml
nightly-rpm:
  stage: build
  image: fedora:latest
  script:
    - dnf install -y rpm-build rust cargo
    - chmod +x build_tools/rpm/build_rpm.sh
    - ./build_tools/rpm/build_rpm.sh --nightly
  artifacts:
    paths:
      - "rpmbuild/RPMS/**/*.rpm"
    name: "grease-nightly-rpm-$CI_COMMIT_SHORT_SHA"
```

## Troubleshooting

### Build Failures

**Missing dependencies:**
```bash
sudo dnf install rpm-build rust cargo gcc
```

**Permission issues:**
- Ensure you have write permissions in the build directory
- Don't run as root unless necessary

**Rust compilation errors:**
- Check Rust toolchain version: `rustc --version` (should be 1.85.1)
- Install correct version: `rustup install 1.85.1 && rustup default 1.85.1`
- Clean build: `cargo clean` before rebuilding

### Package Conflicts

**Remove existing Grease installation:**
```bash
sudo dnf remove grease grease-nightly
```

**Force remove if necessary:**
```bash
sudo dnf remove --nodeps grease grease-nightly
```

### Clean Build

To start completely fresh:
```bash
# Remove build artifacts
rm -rf rpmbuild/
rm -f grease-*.tar.gz
rm -f grease-nightly.spec

# Rebuild
./build_rpm.sh
```

### Debug Mode

To see detailed build output:
```bash
rpmbuild -ba --verbose ~/rpmbuild/SPECS/grease.spec
```

## Customization

### Different Build Options

Modify the `grease.spec` file to customize:

- **Build flags**: Change `cargo build --release` options
- **Installation paths**: Modify `%install` section
- **Dependencies**: Add or remove `BuildRequires` and `Requires`
- **File list**: Update `%files` sections

### Additional Files

To include additional files in the package, add to the `%install` section:

```spec
# Install additional configuration file
install -Dm644 config/grease.conf %{buildroot}%{_sysconfdir}/grease.conf

# Then add to %files section
%config(noreplace) %{_sysconfdir}/grease.conf
```

### Patch Management

To apply patches during build:

```spec
# Add after Source0 line
Patch0: grease-fix-bug.patch

# Add in %prep section
%patch0 -p1
```

## Repository Setup

For system administrators who want to host a local repository:

```bash
# Create repository directory
mkdir -p /var/www/html/grease-repo

# Copy built packages
cp rpmbuild/RPMS/*/*.rpm /var/www/html/grease-repo/

# Create repository metadata
createrepo /var/www/html/grease-repo/

# Add repository on client systems
sudo dnf config-manager --add-repo http://your-server/grease-repo
```

## Support

For issues with the RPM packages:
1. Check the [Grease GitLab Issues](https://gitlab.com/grease-lang/grease/-/issues)
2. Verify you're using a supported distribution (Fedora, RHEL, CentOS, etc.)
3. Ensure all dependencies are properly installed
4. Try building from source if package issues persist