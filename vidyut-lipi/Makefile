debugger:
	./scripts/run-debugger.sh

test:
	cargo nextest run --no-fail-fast --status-level=fail && cargo test --doc

# Profiles the program's execution time on OSX. This command will probably not
# work on other operating systems.
profile-time-osx:
	cargo instruments -t time --release --example sample > /dev/null
