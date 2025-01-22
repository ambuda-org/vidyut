Deployment checklist
====================


Step 1: Prepare data (if major release)
---------------------------------------

Check:

- `make create_all_data` passes.
- Data directory exists with release version
  (`cd .../vidyut-latest && zip -r data-VERSION.zip data-VERSION/`)

NOTE: important to cd *into* vidyut-latest so that the zip files aren't nested.


Step 2: Update version number
-----------------------------

- Increase version number in various files:
    - `pyproject.toml`.
    - `vidyut/docs/source/conf.py`
    - If updating data:
        - `introduction.rst`
- Create changelog entry in CHANGES.rst. Use a temporary release date if
  necessary.
- Grep for previous version and confirm it exists only in comments and
  CHANGES.rst.


Step 3: Quality checks
----------------------

- `make test` passes
- `make integration_tests` passes
- `make lint-check` passes
- `make docs` passes
- `make test_docs` passes

Finalize:

- Update release date in CHANGES.rst
- Tag commit with new VERSION.
- Check that GitHub CI succeeds with new version.
