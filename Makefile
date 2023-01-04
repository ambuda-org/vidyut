# The commands in this file apply generally for all crates in this repo. For
# more specific commands, see the `Makefile`s contained within each crate.

# Runs tests for all crates in the repo.
#
# Currently, this command excludes our more expensive end-to-end tests, such as
# the `make test_all` test suite in `vidyut-prakriya`.
test:
	cargo test --all

# Creates a coverage report for all crates in the repo. Results will be opened
# automatically in your default browser.
#
# This command uses `cargo-llvm-cov`, which you can read about here:
# https://github.com/taiki-e/cargo-llvm-cov
coverage:
	cargo llvm-cov --all --open

# Generates documentation pages for all crates in the repo. Results will be
# opened automatically in your default browser.
docs:
	cargo doc --all --no-deps --open --document-private-items
