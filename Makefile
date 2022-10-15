install:
	./scripts/install.sh

check-memory:
	cargo build --release
	/usr/bin/time -l cargo run --release -- --input-file bg.txt --cache-file data/snapshot.bin

# Using `sudo` because otherwise OSX's `dtrace` is blocked by SIP
flamegraph-osx:
	cargo build --release
	CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -- --input-file bg.txt --cache-file data/snapshot.bin
