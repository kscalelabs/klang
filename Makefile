.PHONY: all check test lint fmt clippy

# Default target that runs all checks
all: check test lint

# Run cargo check
check:
	cargo check

# Run tests
test:
	cargo test

# Run all lints (fmt + clippy)
lint: fmt clippy

# Run rustfmt
fmt:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean
