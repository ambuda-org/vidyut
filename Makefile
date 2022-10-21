# Project setup
# =============
# Most of the interesting day-to-day commands are in `cargo`.


# Install our main data sources from scratch.
install:
	./scripts/install.sh

test:
	cargo test --lib


# Modeling
# ========

# Train a model and save important features to disk.
train:
	./scripts/train_model.py

# Evaluate our parsing quality on a large sample of text.
eval:
	cargo run --release --bin eval -- --cache-file data/snapshot.bin --paths \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-0900-MBh, 6, BhaGī 18-7707.conllu"


# Performance
# ===========

# Run the system timer while parsing a large sample of text.
profile-general:
	cargo build --release
	/usr/bin/time -l make eval

# Profile parsing a large sample of text (OSX only)
#
#     $ make target=time profile-target-osx
#     $ make target=alloc profile-target-osx
#
profile-target-osx:
	cargo instruments -t $(target) --release --bin eval -- --cache-file data/snapshot.bin --paths \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-090*.conllu" \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-091*.conllu" \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-092*.conllu"
