install:
	./scripts/install.sh

check-memory:
	cargo build --release
	/usr/bin/time -l cargo run --release -- --input-file bg.txt --cache-file data/snapshot.bin

# Using `sudo` because otherwise OSX's `dtrace` is blocked by SIP
flamegraph-osx:
	cargo build --release
	CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -- --input-file bg.txt --cache-file data/snapshot.bin

train:
	./scripts/train_model.py

eval:
	cargo run --release --bin eval -- --cache-file data/snapshot.bin --paths \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		"dcs-data/dcs/data/conllu/files/Mahābhārata/Mahābhārata-0900-MBh, 6, BhaGī 18-7707.conllu"
