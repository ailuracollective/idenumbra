# Makefile for idenumbra
# Usage: make <target>

.PHONY: all build test test-all test-features fmt clippy doc clean example tree hooks

all: build

build:
	cargo build --workspace

build-all:
	cargo build --workspace --all-features

test:
	cargo test --workspace

test-all:
	cargo test --workspace --all-features

# Test with a subset of country features, e.g. make test-features FEATURES="ecu arg"
test-features:
	cargo test -p idenumbra --features "$(FEATURES)"

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-features --examples

doc:
	cargo doc --workspace --all-features --no-deps

example:
	cargo run --example basic --all-features

tree:
	cargo tree -p idenumbra --all-features --depth 1

tree-features:
	cargo tree -p idenumbra --features "$(FEATURES)" --depth 1

clean:
	cargo clean

hooks:
	lefthook install