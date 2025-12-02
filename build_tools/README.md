# Build Tools

This directory contains packaging and installation scripts for various Linux distributions and systems. Each sub-folder provides distribution-specific installation methods and packaging tools.

## Available Distributions

- **`any-linux/`** - Universal Linux installation scripts for any distribution
- **`arch/`** - Arch Linux package building scripts (moved from `archlinux/`)
- **`debian/`** - Debian/Ubuntu package building scripts
- **`rpm/`** - RPM package building scripts for Fedora, RHEL, CentOS, and other RPM-based systems

Each directory contains its own README.md with detailed instructions for that specific distribution or installation method.

## Package Types

- **Binary packages**: Pre-compiled binaries for easy installation
- **Source packages**: Source code with build instructions
- **Nightly builds**: Development versions with latest features
- **Stable releases**: Official release versions

## CI/CD Integration

All build systems are integrated with GitLab CI/CD for automated package building:
- **Nightly builds**: Automatically generated from the main branch
- **Release builds**: Triggered by Git tags
- **Multi-platform**: Support for various Linux distributions

For more information about each distribution's packaging system, see the respective README files in each subdirectory.
