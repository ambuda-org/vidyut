`vidyut` uses `Semantic Versioning`_. This versioning also applies to data
releases. That is, versions 0.x.a and 0.x.b will be able to use the same data.

.. _`Semantic Versioning`: https://semver.org/


0.3.1
-----

Released 2025-01-12.

- Fix a `PanicException` when making many call to `transliterate`.
- Fix a `PanicException` when using the `YiWa` pratyaya.
- Add `Kosha.dhatus` and `Kosha.pratipadikas`.
- Update all `vidyut.prakriya` enums to use SLP1 strings for `as_str`.
- Update Python enums to use SLP1 strings for `__str__`.
- Add stronger compatibility with Python enums, including `__new__` and `value`.
- Minor bug fixes to `vidyut.prakriya` (`vAnta`, etc.)


0.3.0
-----

Released 2025-01-06.

- Add `vidyut-chandas` and `vidyut.lipi`.
- Add various improvements and API changes for `vidyut.prakriya`. For details,
  see https://crates.io/crates/vidyut-prakriya.
- Substantially improve `vidyut.kosha` on multiple fronts, including coverage,
  storage size, and ease of use.
- Enums define many more helper methods, including `__hash__` and `__str__`.
- `vidyut.__version__` is defined.
- Substantial regressions to `vidyut.cheda`. We strongly recommend the Dharmamitra
  tools instead: https://github.com/dharmamitra


0.2.0
-----

Released 2023-01-22.

- Add `vidyut.prakriya`, `vidyut.sandhi`, `vidyut.kosha`, and `vidyut.cheda`.
- Add bindings for most of Vidyut's core logic.
- Add more documentation.
- Move `Parser` to `vidyut.cheda.Chedaka` and tweak its API.
- Use `Pada` struct instead of a simple hashmap.


0.1.0
-----

Released 2022-10-25.

- Proof-of-concept release with support for basic bindings.
