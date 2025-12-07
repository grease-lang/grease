#!/bin/bash

# Grease RPM Build Script
# This script builds RPM packages for Fedora/RHEL-based systems

set -e

# Default values
NIGHTLY=false
VERSION=""
RELEASE="1"
FEATURES=""
USE_BINARY=""
BINARY_PATH=""
SPEC_FILE="build_tools/rpm/grease.spec"
BUILD_DIR="rpmbuild"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Show help
show_help() {
    cat << EOF
Grease RPM Build Script

Usage: $0 [OPTIONS]

 OPTIONS:
      --nightly          Build a nightly package with Git commit hash
      --version VERSION  Override version (default: from Cargo.toml)
      --release RELEASE  Override release number (default: 1)
      --features FEATURES  Build with specified Cargo features (e.g., "ui")
      --use-binary PATH  Use pre-built binary at specified path instead of building
      --help             Show this help message

 EXAMPLES:
     $0                              # Build stable release package
     $0 --nightly                    # Build nightly package
     $0 --version 0.2.0              # Build specific version
     $0 --nightly --release 2       # Build nightly with custom release
     $0 --use-binary target/release/grease  # Package pre-built binary

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --nightly)
            NIGHTLY=true
            shift
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        --release)
            RELEASE="$2"
            shift 2
            ;;
         --features)
             FEATURES="$2"
             shift 2
             ;;
         --use-binary)
             USE_BINARY=true
             BINARY_PATH="$2"
             shift 2
             ;;
         --help)
             show_help
             exit 0
             ;;
         *)
             log_error "Unknown option: $1"
             show_help
             exit 1
             ;;
    esac
done

# Check if required tools are available
check_dependencies() {
    log_info "Checking dependencies..."
    
    local missing_deps=()
    
    if ! command -v rpmbuild &> /dev/null; then
        missing_deps+=("rpm-build")
    fi
    
    # Only check for cargo if we're not using a pre-built binary
    if [ -z "$USE_BINARY" ]; then
        if ! command -v cargo &> /dev/null; then
            missing_deps+=("rust cargo")
        fi
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        log_info "Install with: sudo dnf install ${missing_deps[*]}"
        exit 1
    fi
    
    log_success "All dependencies found"
}

# Get version from Cargo.toml if not specified
get_version() {
    if [ -z "$VERSION" ]; then
        VERSION=$(grep "^version = " Cargo.toml | sed 's/version = "\(.*\)"/\1/')
        if [ -z "$VERSION" ]; then
            log_error "Could not determine version from Cargo.toml"
            exit 1
        fi
    fi
    
    if [ "$NIGHTLY" = true ]; then
        COMMIT_SHORT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
        VERSION="${VERSION}.nightly.${COMMIT_SHORT}"
    fi
    
    log_info "Building version: $VERSION"
}

# Prepare source tarball
prepare_source() {
    log_info "Preparing source tarball..."
    
    # Create temporary directory for source
    temp_dir="grease-${VERSION}"
    
    # Clean up any existing temp directory
    rm -rf "$temp_dir"
    
    # Create source directory
    mkdir "$temp_dir"
    
    # Copy all necessary files
    cp -r src "$temp_dir/"
    cp -r std "$temp_dir/"
    cp -r examples "$temp_dir/"
    cp -r docs "$temp_dir/"
    cp -r completions "$temp_dir/"
    cp -r editors "$temp_dir/"
    cp -r build_tools "$temp_dir/"
    cp Cargo.toml "$temp_dir/"
    cp Cargo.lock "$temp_dir/"
    cp README.md "$temp_dir/"
    cp LICENSE "$temp_dir/"
    cp AGENTS.md "$temp_dir/"
    cp Makefile "$temp_dir/"
    cp .gitignore "$temp_dir/"
    
    # Create tarball
    tar -czf "${temp_dir}.tar.gz" "$temp_dir"
    
    # Clean up temp directory
    rm -rf "$temp_dir"
    
    log_success "Source tarball created: ${temp_dir}.tar.gz"
}

# Update spec file for nightly builds
update_spec() {
    if [ "$NIGHTLY" = true ]; then
        log_info "Updating spec file for nightly build..."
        
        # Create a copy of the spec file
        local nightly_spec="grease-nightly.spec"
        cp "$SPEC_FILE" "$nightly_spec"
        
        # Update version and release
        sed -i "s/^Version:.*/Version: $VERSION/" "$nightly_spec"
        sed -i "s/^Release:.*/Release: $RELEASE%{?dist}/" "$nightly_spec"
        sed -i "s/^Name:.*/Name: grease-nightly/" "$nightly_spec"
        
        # Update source line to use local tarball
        sed -i "s|^Source0:.*|Source0: ${temp_dir}.tar.gz|" "$nightly_spec"
        
        # Fix autosetup line for nightly builds (use original directory name)
        sed -i "s|^%autosetup -n %{name}-%{version}|%autosetup -n ${temp_dir}|" "$nightly_spec"
        
        SPEC_FILE="$nightly_spec"
        log_success "Spec file updated for nightly build"
    else
        # Update version in spec file
        sed -i "s/^Version:.*/Version: $VERSION/" "$SPEC_FILE"
        sed -i "s/^Release:.*/Release: $RELEASE%{?dist}/" "$SPEC_FILE"
    fi
}

# Build RPM package
build_rpm() {
    log_info "Building RPM package..."
    
    # Setup rpmbuild directory structure
    mkdir -p "$BUILD_DIR"/{SOURCES,SPECS,RPMS,SRPMS,BUILD}
    
    # Copy source tarball
    mv "${temp_dir}.tar.gz" "$BUILD_DIR/SOURCES/"
    
    # Copy spec file
    cp "$SPEC_FILE" "$BUILD_DIR/SPECS/"
    
    # Build the package
    local spec_name=$(basename "$SPEC_FILE")
    
    # Build with features if specified
    local rpmbuild_cmd="rpmbuild -ba"
    if [ -n "$FEATURES" ]; then
        rpmbuild_cmd="$rpmbuild_cmd --define '_features $FEATURES'"
    fi
    
    rpmbuild_cmd="$rpmbuild_cmd \
        --define '_topdir $PWD/$BUILD_DIR' \
        --define '_version $VERSION' \
        --define '_release $RELEASE' \
        '$BUILD_DIR/SPECS/$spec_name'"
    
    log_success "RPM package built successfully"
}

# Build RPM from pre-built binary
build_rpm_from_binary() {
    log_info "Building RPM from pre-built binary..."
    
    # Setup rpmbuild directory structure
    mkdir -p "$BUILD_DIR"/{SOURCES,SPECS,RPMS,SRPMS,BUILD}
    
    # Create a simple spec file for binary packaging
    local binary_spec="grease-binary.spec"
    
    cat > "$binary_spec" << EOF
# Grease RPM Spec File (Binary Package)
# Maintainer: Nick Girga <nickgirga@gmail.com>

Name:           $(if [ "$NIGHTLY" = true ]; then echo "grease-nightly"; else echo "grease"; fi)
Version:        $VERSION
Release:        $RELEASE%{?dist}
Summary:        A modern scripting language written in pure Rust

License:        Apache-2.0
URL:            https://gitlab.com/grease-lang/grease

# No source tarball needed for binary package

%description
Grease is a modern scripting language written in pure Rust that compiles to 
platform-agnostic bytecode and runs on a custom virtual machine. It's designed 
as "the high-performance oil for your Rust engine."

Features:
- Pure Rust implementation with no external dependencies
- Fast compilation to bytecode
- Stack-based virtual machine
- Modern language features with familiar syntax
- Complete Language Server Protocol support
- Interactive REPL mode
- Comprehensive standard library

%prep
# No preparation needed for binary package

%build
# No build needed for binary package

%install
# Create directories
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_mandir}/man1
mkdir -p %{buildroot}%{_datadir}/bash-completion/completions
mkdir -p %{buildroot}%{_datadir}/zsh/site-functions
mkdir -p %{buildroot}%{_docdir}/%{name}

# Install binary
install -Dm755 $BINARY_PATH %{buildroot}%{_bindir}/grease

# Install man page
install -Dm644 docs/grease.1 %{buildroot}%{_mandir}/man1/grease.1

# Install shell completions
install -Dm644 completions/grease.bash %{buildroot}%{_datadir}/bash-completion/completions/grease
install -Dm644 completions/grease.zsh %{buildroot}%{_datadir}/zsh/site-functions/_grease

# Install documentation
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -Dm644 docs/LSP_README.md %{buildroot}%{_docdir}/%{name}/LSP_README.md
install -Dm644 docs/TODO.md %{buildroot}%{_docdir}/%{name}/TODO.md

# Install examples
mkdir -p %{buildroot}%{_docdir}/%{name}/examples
cp -r examples/* %{buildroot}%{_docdir}/%{name}/examples/

%files
%license LICENSE
%doc README.md docs/LSP_README.md docs/TODO.md
%doc examples/
%{_bindir}/grease
%{_mandir}/man1/grease.1*
%{_datadir}/bash-completion/completions/grease
%{_datadir}/zsh/site-functions/_grease

%changelog
* $(date '+%a %b %d %Y') Nick Girga <nickgirga@gmail.com> - $VERSION-$RELEASE
- Binary package built from pre-compiled binary
EOF
    
    # Copy spec file
    cp "$binary_spec" "$BUILD_DIR/SPECS/"
    
    # Build the package
    local spec_name=$(basename "$binary_spec")
    
    rpmbuild_cmd="rpmbuild -ba \
        --define '_topdir $PWD/$BUILD_DIR' \
        --define '_version $VERSION' \
        --define '_release $RELEASE' \
        '$BUILD_DIR/SPECS/$spec_name'"
    
    log_success "RPM package built from binary"
}

# Show results
show_results() {
    log_info "Built packages:"
    
    find "$BUILD_DIR/RPMS" -name "*.rpm" -exec ls -lh {} \;
    find "$BUILD_DIR/SRPMS" -name "*.rpm" -exec ls -lh {} \;
    
    log_info "Packages are located in: $BUILD_DIR/"
    
    if [ "$NIGHTLY" = true ]; then
        log_info "Install with: sudo dnf install $BUILD_DIR/RPMS/*/grease-nightly-*.rpm"
    else
        log_info "Install with: sudo dnf install $BUILD_DIR/RPMS/*/grease-*.rpm"
    fi
}

# Validate binary path if provided
validate_binary() {
    if [ -n "$USE_BINARY" ] && [ ! -f "$BINARY_PATH" ]; then
        log_error "Binary file not found at $BINARY_PATH"
        exit 1
    fi
}

# Main execution
main() {
    log_info "Starting Grease RPM build process..."
    
    check_dependencies
    validate_binary
    get_version
    
    if [ -n "$USE_BINARY" ]; then
        build_rpm_from_binary
    else
        prepare_source
        update_spec
        build_rpm
    fi
    
    show_results
    
    log_success "RPM build completed successfully!"
}

# Run main function
main "$@"