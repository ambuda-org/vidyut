VERBOSE ?= "-v"

# Initializes the repository for local development.
install: test
	@echo "vidyut-python built successfully."

# Builds the package.
#
# Use this command directly instead of `cargo build`, which will likely fail
# due to missing linker arguments.
build:
	rm -rf dist .venv; uv sync
	uv run maturin build --release --out dist --interpreter=$$(which python3)
	uv run pip3 install dist/vidyut-* --force-reinstall


# Runs all Python-specific unit tests.
test: build
	uv run pytest $(VERBOSE) test/unit/

coverage-clean:
	(cargo --list | grep llvm-cov) || cargo install cargo-llvm-cov
	cargo llvm-cov clean --workspace

# https://github.com/Michael-J-Ward/rust-python-coverage
coverage: coverage-clean build
	uv run pytest --cov=vidyut --cov-report lcov:python-coverage.lcov
	cargo llvm-cov report --html

# NOTE: this command is very slow.
create_data:
	rm -rf vidyut-rust
	git clone --depth 1 https://github.com/ambuda-org/vidyut.git vidyut-rust
	cd vidyut-rust && make create_all_data

# Runs integration tests against a data directory.
#
# Usage:
#
# VIDYUT_DATA_DIR="/path/to/your/dir" make integration_tests
integration_tests: develop
	VIDYUT_DATA_DIR="../vidyut-data/data/build/vidyut-latest" uv run pytest test/integration/

# Lints all Rust and Python code.
lint:
	cargo fmt
	uv run ruff format

# Lints all Rust and Python code.
lint-check:
	cargo fmt --check
	uv run ruff format --check

# Generates Sphinx docs.
#
# To view the docs, open `vidyut/docs/build/html/index.html`.
docs: develop
	# Sphinx caches old code, so clear these out first.
	rm -R -f vidyut/docs/build/doctrees/
	cd vidyut/docs && SPHINXOPTS="-a" uv run make html

test_docs:
	# Run on release build to avoid debug printouts.
	uv run maturin develop --release
	cd vidyut/docs && uv run sphinx-build -b doctest source build/doctest
