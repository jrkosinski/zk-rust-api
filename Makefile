.PHONY: all build test clean fmt lint doc bench audit help

# Default target
all: fmt lint test

# Build the project
build:
	cargo build

# Build release version
release:
	cargo build --release

# Run tests
test:
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt -- --check

# Run clippy
lint:
	cargo clippy -- -D warnings

# Fix clippy issues
fix:
	cargo clippy --fix

# Generate documentation
doc:
	cargo doc --no-deps --open

# Run benchmarks
bench:
	cargo bench

# Security audit
audit:
	cargo audit

# Check for dependency issues
deny:
	cargo deny check

# Run example
example:
	cargo run --example basic_usage

# Check project
check:
	cargo check

# Watch for changes and run tests
watch:
	cargo watch -x test

# Install development tools
install-tools:
	cargo install cargo-audit
	cargo install cargo-deny
	cargo install cargo-tarpaulin
	cargo install cargo-watch

# Coverage report
coverage:
	cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Html

# Full CI check (what CI runs)
ci: fmt-check lint test audit deny

# Help target
help:
	@echo "Available targets:"
	@echo "  all          - Format, lint, and test (default)"
	@echo "  build        - Build the project"
	@echo "  release      - Build release version"
	@echo "  test         - Run tests"
	@echo "  test-verbose - Run tests with output"
	@echo "  clean        - Clean build artifacts"
	@echo "  fmt          - Format code"
	@echo "  fmt-check    - Check code formatting"
	@echo "  lint         - Run clippy"
	@echo "  fix          - Fix clippy issues"
	@echo "  doc          - Generate and open documentation"
	@echo "  bench        - Run benchmarks"
	@echo "  audit        - Run security audit"
	@echo "  deny         - Check dependencies"
	@echo "  example      - Run basic example"
	@echo "  check        - Quick check (faster than build)"
	@echo "  watch        - Watch and test on changes"
	@echo "  install-tools - Install development tools"
	@echo "  coverage     - Generate coverage report"
	@echo "  ci           - Run full CI checks"
	@echo "  help         - Show this help message"
