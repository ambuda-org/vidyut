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


# Data commands
# =============
# These commands generally apply across multiple crates in the repo.

# Creates and collects all data files required to use Vidyut.
create_all_data:
	@./scripts/create_all_data.sh

create_sandhi_rules:
	RUST_LOG=info cargo run --release --bin create_sandhi_rules -- \
			 --data-dir data/build/vidyut-0.2.0

# Creates a koshas and write it to disk.
create_kosha:
	RUST_LOG=info cargo run --release --bin create_kosha -- \
			 --input-dir data/raw/lex --output-dir data/build/vidyut-0.2.0

# Trains a padaccheda model and saves important features to disk.
# NOTE: when training, exclude the file paths used in `make eval`.
train_cheda:
	cargo run --release --bin train_cheda -- \
		--vidyut-dir "data/build/vidyut-0.2.0" \
		--include "data/raw/dcs/conllu/files/**/*.conllu" \
		--exclude "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		--exclude "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		--exclude "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-0900-MBh, 6, BhaGī 18-7707.conllu"


# Integration tests
# =================

# Runs basic end-to-end tests against the given kosha.
test_kosha:
	RUST_LOG=info cargo run --release --bin test_kosha -- --data-dir data/build/vidyut-0.2.0/kosha


# Evaluate our parsing quality on a large sample of text.
eval_cheda:
	cargo run --release --bin eval_cheda -- \
		--vidyut-dir "data/build/vidyut-0.2.0" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-0900-MBh, 6, BhaGī 18-7707.conllu"


# Performance
# ===========

# Run the system timer while parsing a large sample of text.
profile-cheda-general:
	cargo build --release
	/usr/bin/time -l make eval

# Profile parsing a large sample of text (OSX only). Usage:
#
#     $ make target=time profile-target-osx
#     $ make target=alloc profile-target-osx
#
profile-cheda-target-osx:
	cargo instruments -t $(target) --release --bin eval -- \
		--vidyut-dir "data/vidyut-0.1.0" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-090*.conllu" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-091*.conllu" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-092*.conllu"
