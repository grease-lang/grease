# Makefile for Grease Installation

.PHONY: install install-user uninstall build clean test deb help

# Default target
all: build

# Build the release binary
build:
	@echo "🔨 Building Grease..."
	cargo build --release
	@echo "✅ Build complete: target/release/grease"

# Install system-wide (requires sudo)
install: build
	@echo "📦 Installing Grease to /usr/local/bin..."
	sudo cp target/release/grease /usr/local/bin/
	sudo chmod +x /usr/local/bin/grease
	@echo "✅ Grease installed system-wide!"
	@echo "🧪 Test with: greese"

# Install for current user (no sudo required)
install-user: build
	@echo "📦 Installing Grease to ~/.local/bin..."
	mkdir -p ~/.local/bin
	cp target/release/grease ~/.local/bin/
	chmod +x ~/.local/bin/grease
	@if ! echo $$PATH | grep -q "$$HOME/.local/bin"; then \
		echo "📝 Adding ~/.local/bin to PATH..."; \
		echo 'export PATH="$$HOME/.local/bin:$$PATH"' >> ~/.bashrc; \
		echo "✅ Run 'source ~/.bashrc' or restart your shell"; \
	fi
	@echo "✅ Grease installed for user!"

# Uninstall system-wide
uninstall:
	@echo "🗑️ Removing Grease..."
	sudo rm -f /usr/local/bin/grease
	@echo "✅ Grease uninstalled!"

# Uninstall user installation
uninstall-user:
	@echo "🗑️ Removing Grease..."
	rm -f ~/.local/bin/grease
	@echo "✅ Grease uninstalled for user!"

# Create Debian package
deb: build
	@echo "📦 Building Debian package..."
	./build_deb.sh

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	cargo clean
	rm -f *.deb
	rm -rf grease_*

# Run tests
test:
	@echo "🧪 Testing Grease..."
	cargo test
	@echo "✅ Tests passed!"

# Run integration tests
test-integration: build
	@echo "🧪 Running integration tests..."
	echo 'print("Hello from test!")' | ./target/release/grease
	@echo "✅ Integration tests passed!"

# Show help
help:
	@echo "Grease Installation Makefile"
	@echo "The high-performance oil for your Rust engine."
	@echo ""
	@echo "Targets:"
	@echo "  build          - Build release binary"
	@echo "  install        - Install system-wide (requires sudo)"
	@echo "  install-user   - Install for current user"
	@echo "  uninstall      - Remove system-wide installation"
	@echo "  uninstall-user - Remove user installation"
	@echo "  deb            - Create Debian package"
	@echo "  clean          - Clean build artifacts"
	@echo "  test           - Run unit tests"
	@echo "  test-integration - Run integration tests"
	@echo "  help           - Show this help"
	@echo ""
	@echo "Examples:"
	@echo "  make install      # Install system-wide"
	@echo "  make install-user # Install for current user"
	@echo "  make test         # Run tests"