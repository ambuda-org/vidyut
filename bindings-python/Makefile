# Initializes the repository for local development.
install:
	uv sync
	uv run make develop
	uv run make test
	@echo "vidyut-py built successfully."

# Builds the package.
#
# Use this command directly instead of `cargo build`, which will likely fail
# due to missing linker arguments.
build:
	uv run maturin build

# Builds the package and installs it in the local virtualenv.
develop:
	uv run maturin develop

# Runs all Python-specific unit tests.
test: develop
	uv run pytest test/unit/

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
	VIDYUT_DATA_DIR="data-0.3.0" uv run pytest test/integration/

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

doctest:
	# Run on release build to avoid debug printouts.
	uv run maturin develop --release
	cd vidyut/docs && uv run sphinx-build -b doctest source build/doctest
