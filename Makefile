# The commands in this file apply generally for all crates in this repo. For
# more specific commands, see the `Makefile`s contained within each crate.

lint:
	cargo fmt --all

# Runs unit tests for all crates in the repository.
#
# This command excludes our more expensive end-to-end tests, such as the
# `make test_all` test suite in `vidyut-prakriya`. It also excludes tests
# for `bindings-python`.
test:
	cargo nextest run --no-fail-fast --status-level=fail --workspace
	# Also test bindings.
	cd bindings-python && make test

# Creates a coverage report for all crates in the repository. Results will be
# opened automatically in your default browser.
#
# This command uses `cargo-llvm-cov`, which you can read about here:
# https://github.com/taiki-e/cargo-llvm-cov
coverage:
	cargo llvm-cov --all --open

# Generates documentation pages for all crates in the repo. Results will be
# opened automatically in your default browser.
docs:
	cargo doc --all --no-deps --open --document-private-items

# Runs documentation tests for all crates in the repository.
test_docs:
	cargo test --all --doc
