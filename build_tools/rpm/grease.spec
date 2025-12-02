# Grease RPM Spec File
# Maintainer: Nick Girga <nickgirga@gmail.com>

Name:           grease
Version: 0.1.1
Release: 1%{?dist}
Summary:        A modern scripting language written in pure Rust

License:        Apache-2.0
URL:            https://gitlab.com/grease-lang/grease
Source0:        %{url}/-/archive/v%{version}/%{name}-%{version}.tar.gz

# Build dependencies
BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  gcc

# Runtime dependencies - none (statically linked binary)

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

%package devel
Summary:        Development files for %{name}
Requires:       %{name}%{?_isa} = %{version}-%{release}

%description devel
Development files for the Grease programming language. This package includes
documentation and examples for developing with Grease.

%prep
%autosetup -n %{name}-%{version}

%build
export RUSTUP_AUTO_SELF_UPDATE=never
cargo build --release

%install
# Create directories
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_mandir}/man1
mkdir -p %{buildroot}%{_datadir}/bash-completion/completions
mkdir -p %{buildroot}%{_datadir}/zsh/site-functions
mkdir -p %{buildroot}%{_docdir}/%{name}

# Install binary
install -Dm755 target/release/grease %{buildroot}%{_bindir}/grease

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

%files devel
# Development package currently contains no additional files beyond main package

%changelog
* Tue Dec 02 2025 Nick Girga <nickgirga@gmail.com> - 0.1.1-1
- Initial RPM package for Grease programming language
- Support for Fedora and RHEL-based systems
- Includes binary, documentation, and shell completions