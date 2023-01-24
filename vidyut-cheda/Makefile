# Project setup
# =============
# Most of the interesting day-to-day commands are in `cargo`.


# Installs our main data sources from scratch.
install:
	./scripts/install.sh


# Data
# ====

generate_sandhi_rules:
	RUST_LOG=info cargo run --release --bin generate_sandhi_rules -- \
			 --data-dir data/vidyut-0.1.0

# Creates a koshas and write it to disk.
create_kosha:
	RUST_LOG=info cargo run --release --bin create_kosha -- --input-dir data --output-dir data/vidyut-0.1.0

# Runs basic end-to-end tests against the given kosha.
test_kosha:
	RUST_LOG=info cargo run --release --bin test_kosha -- --data-dir data/vidyut-0.1.0


# Modeling
# ========

# Train a model and save important features to disk.
# NOTE: when training, exclude the file paths used in `make eval`.
train:
	python3 ./scripts/fetch_training_data.py
	cargo run --release --bin train -- \
		--vidyut-dir "data/vidyut-0.1.0" \
		--include "dcs-data/dcs/data/conllu/files/**/*.conllu" \
		--exclude "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		--exclude "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		--exclude "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-0900-MBh, 6, BhaGī 18-7707.conllu"

# Evaluate our parsing quality on a large sample of text.
eval:
	cargo run --release --bin eval -- \
		--vidyut-dir "data/vidyut-0.1.0" \
		--paths "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		--paths "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		--paths "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-0900-MBh, 6, BhaGī 18-7707.conllu"


# Performance
# ===========

# Run the system timer while parsing a large sample of text.
profile-general:
	cargo build --release
	/usr/bin/time -l make eval

# Profile parsing a large sample of text (OSX only). Usage:
#
#     $ make target=time profile-target-osx
#     $ make target=alloc profile-target-osx
#
profile-target-osx:
	cargo instruments -t $(target) --release --bin eval -- \
		--vidyut-dir "data/vidyut-0.1.0" \
		--paths "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		--paths "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		--paths "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-090*.conllu" \
		--paths "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-091*.conllu" \
		--paths "dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-092*.conllu"
