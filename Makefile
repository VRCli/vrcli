# Makefile for vrcli

.PHONY: format check build test clean install pre-commit setup-hooks

# Format code
format:
	cargo fmt --all

# Check formatting without making changes
check-format:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run all checks (format + clippy + test)
check: format clippy test

# Build the project
build:
	cargo build

# Build release
build-release:
	cargo build --release

# Run tests
test:
	cargo test --verbose

# Clean build artifacts
clean:
	cargo clean

# Install the binary
install:
	cargo install --path .

# Pre-commit hook (format + check)
pre-commit: format clippy
	@echo "Pre-commit checks completed successfully!"

# Setup git hooks
setup-hooks:
	@echo "Setting up git hooks..."
	@mkdir -p .git/hooks
	@echo '#!/bin/sh' > .git/hooks/pre-commit
	@echo 'make pre-commit' >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "Git pre-commit hook installed!"

# Development setup
dev-setup: setup-hooks
	@echo "Development environment setup complete!"
	@echo "Run 'make check' to verify everything works"
