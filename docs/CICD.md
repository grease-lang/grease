# CI/CD Documentation

This document explains the Continuous Integration and Continuous Deployment (CI/CD) setup for the Grease programming language project.

## Overview

Grease uses GitLab CI/CD to automate building, testing, and packaging of nightly releases across multiple platforms and architectures. The CI pipeline ensures code quality, cross-platform compatibility, and automated distribution of binaries.

## GitLab CI Configuration

The CI/CD pipeline is defined in `.gitlab-ci.yml` at the root of the repository.

### Global Configuration

```yaml
stages:
  - build

image: rust:1.91.1

variables:
  CARGO_HOME: $CI_PROJECT_DIR/.cargo

cache:
  paths:
    - .cargo/
    - target/
```

- **Stages**: Single `build` stage containing all compilation and packaging jobs
- **Image**: Uses Rust 1.91.1 Docker image as the base environment
- **Variables**: Sets `CARGO_HOME` to cache Rust toolchain and dependencies within the project directory
- **Cache**: Caches Cargo registry and build artifacts to speed up subsequent builds

## Jobs

### nightly

Builds the standard Linux x64 nightly release.

**Purpose**: Primary nightly build for the most common platform.

**Commands**:
- `rustc --version && cargo --version`: Verify Rust toolchain installation
- `cargo test`: Run the full test suite to ensure code quality
- Version management: Extract commit hash, update Cargo.toml and man page with nightly version
- `cargo clean`: Clear build cache to ensure version changes are picked up
- `cargo build --release`: Build optimized release binary

**Artifacts**: `target/release/grease` binary

### nightly-deb

Builds Debian (.deb) packages.

**Purpose**: Distribution for Debian-based Linux distributions.

**Commands**:
- `apt-get update -y && apt-get install -y dpkg-dev`: Install Debian packaging tools
- Toolchain verification and testing (same as nightly)
- `./build_tools/debian/build_deb.sh --nightly`: Execute Debian package build script

**Artifacts**: `.deb` package files

### nightly-arch

Builds Arch Linux packages.

**Purpose**: Distribution for Arch Linux and derivatives.

**Environment**: Uses `archlinux:base-20251019.0.436919` image with Arch Linux package manager.

**Commands**:
- `pacman -Syu --noconfirm && pacman -S --noconfirm base-devel rust sudo`: Update system and install build dependencies
- Toolchain verification and testing
- User setup for package building (Arch Linux requires non-root user for makepkg)
- `makepkg -s --noconfirm`: Build package without confirmation prompts

**Artifacts**: `.pkg.tar.zst` package files

### nightly-rpm

Builds RPM packages.

**Purpose**: Distribution for RPM-based Linux distributions (Fedora, RHEL, etc.).

**Environment**: Uses `fedora:43` image.

**Commands**:
- `dnf update -y && dnf install -y rpm-build rust cargo gcc`: Update system and install RPM build tools
- Toolchain verification and testing
- `./build_tools/rpm/build_rpm.sh --nightly`: Execute RPM package build script

**Artifacts**: `.rpm` files in `rpmbuild/RPMS/` directory

## Cross-Compilation Jobs

The following jobs use cross-compilation to build for different architectures and platforms:

### nightly-arm64, nightly-arm32, nightly-i686, nightly-riscv64

**Purpose**: Build Linux binaries for ARM64, ARM32, x86 (32-bit), and RISC-V 64-bit architectures.

**Configuration**:
```yaml
services:
  - docker:28.5.1-dind
variables:
  DOCKER_TLS_CERTDIR: "/certs"
  DOCKER_DRIVER: overlay2
  DOCKER_HOST: tcp://docker:2376
  DOCKER_TLS_VERIFY: 1
  DOCKER_CERT_PATH: "$DOCKER_TLS_CERTDIR/client"
  CROSS_DOCKER_IMAGE: ghcr.io/cross-rs/{target}:v0.2.5
  CROSS_CONTAINER_IN_CONTAINER: "true"
```

- **Services**: Docker-in-Docker v28.5.1 to enable cross-compilation with the `cross` tool
- **Variables**: Configure Docker TLS certificates, storage driver, daemon connection, cross-compilation images, and container-in-container mode

**Commands**:
- Install Docker client
- Verify Docker daemon connectivity with `docker info`
- Install `cross` tool v0.2.5 for cross-compilation
- Run tests and build for target architecture using `cross test` and `cross build`

### nightly-windows-x64, nightly-windows-x86

**Purpose**: Build Windows binaries for 64-bit and 32-bit architectures.

**Configuration**: Same Docker-in-Docker setup as Linux cross-compilation jobs.

**Commands**:
- Install Docker client and `cross` tool
- Execute `build_tools/windows/build_windows.sh` script with appropriate architecture flag

## Why These Commands?

### Testing
- `cargo test` is run in every job to ensure code changes don't break existing functionality
- Cross-compilation jobs run `cross test --target <target>` to verify the code compiles and tests pass on target architectures

### Version Management
- Nightly builds update version strings to include commit hash for traceability
- Version updates ensure nightly releases are distinguishable from stable releases
- Man page version is also updated for consistency

### Build Optimization
- `cargo clean` is used when version changes to ensure the new version is embedded in binaries
- `--release` flag builds optimized binaries suitable for distribution

### Platform-Specific Packaging
- Each Linux distribution has its own package format and build tools
- Debian uses `dpkg-dev`, Arch uses `makepkg`, RPM uses `rpm-build`
- Package builds ensure Grease can be easily installed on different Linux distributions

### Cross-Compilation
- `cross` tool provides Docker-based cross-compilation without complex toolchain setup
- Docker-in-Docker allows running Docker containers within GitLab CI jobs
- Supports building for architectures not natively available in CI runners

## Artifacts

All jobs produce artifacts that are stored and can be downloaded:

- **Binaries**: Executable files for each platform/architecture
- **Packages**: Distribution-specific package files (.deb, .rpm, .pkg.tar.zst)
- **Naming**: Artifacts include commit hash for traceability (`$CI_COMMIT_SHORT_SHA`)

## Cache Strategy

- **Cargo Registry**: Cached to avoid re-downloading dependencies
- **Build Artifacts**: Cached to speed up incremental builds
- **Location**: Cache stored in project directory to persist across jobs

## Triggers

All jobs are triggered only on the `main` branch (`only: - main`), ensuring nightly builds are created for stable commits.

## Maintenance Notes

### Docker-in-Docker Setup
The cross-compilation jobs require Docker-in-Docker configuration:
- `services: - docker:28.5.1-dind` enables Docker daemon v28.5.1 in the CI job
- `DOCKER_TLS_CERTDIR: "/certs"` configures TLS certificates directory
- `DOCKER_DRIVER: overlay2` sets the storage driver for better performance
- `DOCKER_HOST: tcp://docker:2376` connects to the Docker daemon via TCP
- `DOCKER_TLS_VERIFY: 1` enables TLS verification
- `DOCKER_CERT_PATH: "$DOCKER_TLS_CERTDIR/client"` points to client certificates
- `CROSS_DOCKER_IMAGE` specifies the exact cross-compilation Docker image version
- `CROSS_CONTAINER_IN_CONTAINER: "true"` tells cross it's running inside a container
- `docker info` verifies Docker daemon connectivity after installation
- Cross tool version 0.2.5 and Docker images v0.2.5 are pinned for stability
- This setup allows `cross` tool to function properly

### Version Updates
Nightly version format: `{base_version}-nightly-{commit_hash}`
- Base version extracted from `Cargo.toml`
- Commit hash ensures uniqueness and traceability

### Platform Coverage
Current platforms supported:
- Linux: x64, ARM64, ARM32, x86, RISC-V64
- Windows: x64, x86
- Packaging: Debian, Arch Linux, RPM

### Version Pinning
For stability and reproducibility:
- Docker-in-Docker: `docker:28.5.1-dind`
- Cross tool: `cross 0.2.5`
- Cross-compilation images: `ghcr.io/cross-rs/{target}:v0.2.5`
- Cross environment: `CROSS_CONTAINER_IN_CONTAINER=true`
- Base images:
  - Rust: `rust:1.91.1`
  - Arch Linux: `archlinux:base-20251019.0.436919`
  - Fedora: `fedora:43`

## Troubleshooting

### Common Issues
- **Docker daemon not running**: Ensure `docker:dind` service is configured for cross-compilation jobs
- **Cross-compilation failures**: Verify target architecture support in `cross` tool
- **Package build failures**: Check platform-specific build scripts and dependencies

### Debugging
- CI logs provide detailed output for each command
- Failed jobs can be retried or investigated using GitLab CI interface
- Local testing of build scripts can help identify issues before CI runs