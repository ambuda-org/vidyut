Deployment checklist
====================

Data (if a major release):

- `make create_all_data` passes.
- Data directory exists with release version.
  (`zip -r data-VERSION.zip data-VERSION/`)

Version number:

- Increase version number in various files:
    - `pyproject.toml`.
    - `vidyut/docs/source/conf.py`
    - If updating data:
        - `Makefile`
        - `bindings-python/README.md`
        - `introduction.rst`
- Create changelog entry in CHANGES.rst. Use a temporary release date if
  necessary.
- Grep for previous version and confirm it exists only in comments and
  CHANGES.rst.

Quality:

- `make test` passes
- `make integration_tests` passes
- `make lint-check` passes
- `make docs` passes
- `make doctest` passes

Finalize:

- Update release date in CHANGES.rst
- Tag commit with new VERSION.
- Run `maturin publish`.
