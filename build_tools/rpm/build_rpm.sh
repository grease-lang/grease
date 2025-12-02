#!/bin/bash

# Grease RPM Build Script
# This script builds RPM packages for Fedora/RHEL-based systems

set -e

# Default values
NIGHTLY=false
VERSION=""
RELEASE="1"
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
    --help             Show this help message

EXAMPLES:
    $0                              # Build stable release package
    $0 --nightly                    # Build nightly package
    $0 --version 0.2.0              # Build specific version
    $0 --nightly --release 2       # Build nightly with custom release

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
    
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("rust cargo")
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
    
    rpmbuild -ba \
        --define "_topdir $PWD/$BUILD_DIR" \
        --define "_version $VERSION" \
        --define "_release $RELEASE" \
        "$BUILD_DIR/SPECS/$spec_name"
    
    log_success "RPM package built successfully"
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

# Main execution
main() {
    log_info "Starting Grease RPM build process..."
    
    check_dependencies
    get_version
    prepare_source
    update_spec
    build_rpm
    show_results
    
    log_success "RPM build completed successfully!"
}

# Run main function
main "$@"