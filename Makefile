.PHONY: build test test-integration test-all clean doc bigtable-up bigtable-down help

# Default target
help:
	@echo "rust-bigtable Makefile"
	@echo ""
	@echo "Build targets:"
	@echo "  make build            - Build the library"
	@echo "  make check            - Check compilation without building"
	@echo "  make doc              - Generate documentation"
	@echo "  make clean            - Clean build artifacts"
	@echo ""
	@echo "Test targets:"
	@echo "  make test             - Run doc tests"
	@echo "  make test-integration - Run integration tests (requires Bigtable)"
	@echo "  make test-all         - Run all tests"
	@echo ""
	@echo "Bigtable infrastructure:"
	@echo "  make bigtable-up      - Create Bigtable instance and table"
	@echo "  make bigtable-down    - Delete Bigtable instance"
	@echo ""
	@echo "Environment variables:"
	@echo "  PROJECT_ID     - GCP project ID (default: gen-lang-client-0421059902)"
	@echo "  INSTANCE_ID    - Bigtable instance ID (default: test-inst)"
	@echo "  TABLE_NAME     - Table name (default: my-table)"
	@echo "  COLUMN_FAMILY  - Column family name (default: cf1)"

# Build targets
build:
	cargo build

check:
	cargo check

doc:
	cargo doc --no-deps --open

clean:
	cargo clean

# Test targets
test:
	cargo test

test-integration:
	cargo test --test integration_tests -- --ignored --test-threads=1

test-all: test test-integration

# Bigtable infrastructure
bigtable-up:
	./scripts/bigtable-up.sh

bigtable-down:
	./scripts/bigtable-down.sh
