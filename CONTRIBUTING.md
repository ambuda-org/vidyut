How to contribute to Vidyut
===========================

Thank you for considering a contribution to Vidyut! Vidyut is an ambitious and
transformative project, and it can grow only with your help. There are many
ways to contribute, and everyone is welcome regardless of background.

Here are some of the many ways you can help:

- [Report bugs and request features](#report-bugs-and-request-features)
- [Write documentation](#write-documentation)
- [Write code](#write-code)
- [Spread the word](#spread-the-word)

And whether you contribute or not, we would love to have you join our community:

- Most of our conversation occurs on [Ambuda's Discord server][discord] on the
  `#vidyut` channel, where you can chat directly with our team and get fast
  answers to your questions. We also schedule time to spend together virtually,
  usually on a weekly frequency.

- Occasional discussion related to Vidyut might also appear on
  [ambuda-discuss][ambuda-discuss] or on standard mailing lists like
  [sanskrit-programmers][sanskrit-programmers].

- You can also follow along with project announcements on
  [ambuda-announce][ambuda-announce].

[discord]: https://discord.gg/7rGdTyWY7Z
[ambuda-discuss]: https://groups.google.com/g/ambuda-discuss
[sanskrit-programmers]: https://groups.google.com/g/sanskrit-programmers
[ambuda-announce]: https://groups.google.com/g/ambuda-announce


Report bugs and request features
--------------------------------

One of the best ways to contribute is by telling us about your experience using
Vidyut. Bug reports help us make our code even better, and feature requests
help us plan our project roadmap.

Please use GitHub's [issues][issues] tracker for bug reports and feature
requests.

[issues]: https://github.com/ambuda-org/vidyut/issues


Write documentation
-------------------

Vidyut aims to be reliable infrastructure for all Sanskrit software, so having
good documentation is a high priority. Writing documentation, and improving our
existing docs, is a great way to improve Vidyut without having to dive into the
code.

### Rust

You can build our Rust documentation by running `make docs` from the repository
root. All of our documentation follows [the usual rustdoc conventions][rustdoc].

Once your changes are ready for review, run `make test_doc` to verify that the
code samples in the docs build correctly.

[rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html

### Python

You can build our Python documentation by running `make docs` from the
`bindings-python` directory. Documentation for specific functions is written in
[rustdoc][rustdoc] on our Rust code, and documentation for the package as a
whole is managed through [Sphinx][sphinx].

Once your changes are ready for review, run `make test_doc` to verify that the
code samples in the docs build and run correctly.

[sphinx]: https://www.sphinx-doc.org/en/master/index.html


Write code
----------

Vidyut is written by a small team, and we are eager for more contributors!


### Building

First, make sure you have `uv` and `cargo` installed:

```
$ uv --version
$ cargo --version
```

We also recommend that you [install `cargo-nextest`][nextest] for better Rust
testing. You can confirm that it is installed like so:

```
$ cargo nextest --version
```

[nextest]: https://nexte.st/docs/installation/pre-built-binaries/

Once you have `uv` and `cargo` installed, you can test your dev environment by
running `make test` at the repository root:

```shell
$ git clone https://github.com/ambuda-org/vidyut.git
$ cd vidyut
$ make test
```

`make test` will test all Rust code, excluding bindings. To test bindings
as well, run `make test` within the `bindings-python` directory.


### Navigating the codebase

Each crate (Rust module) is in its own directory: `vidyut-kosha`, `vidyut-lipi`,
and so on. Each crate has its own README, and many crates have their own `Makefile`.

Each crate is independent and interacts through the others only through public
APIs. This means that Vidyut is highly modular, which we hope makes it easier
to explore specific projects.

All logic related to Python bindings is in `bindings-python`. We define our
Python bindings with [PyO3](https://pyo3.rs) and publish the package with
[Maturin](https://www.maturin.rs). Both of these libraries have great user
guides that are suitable for beginners. Within `bindings-python`, two directories
are especially important:

- `src` contains our Rust code with PyO3 annotations. This code tells Maturin
  how to define a native Python module. To better understand this code, see the
  header comments in `src/lib.rs`.

- `vidyut` contains simple Python wrapper code and docstrings. We use the code
  here to make our native module more user-friendly. To better understand this
  code, see the comments in `__init__.py`.


### Finding something to work on

Vidyut is a new project, and almost everything needs improvement. We encourage
you to work on whatever intererests you, but if you would like more direction,
here are the areas we care about most:

- *Fixing bugs and writing features* -- This is self-explanatory. Check our
  [issues tracker][issues] for bugs and feature requests.

- *Improving performance* -- Making Vidyut more performant is always a high
  priority, especially for resource-intensive crates like `vidyut-prakriya`.

- *Cleaning up technical debt* -- Vidyut has grown rapidly and has accrued
  a lot of technical debt. This is most obvious in `vidyut-prakriya`, where
  the same operations are often written in three or four different ways.

- *Writing tests* -- Ideally, Vidyut should have 100% test coverage in its core
  functions. You can use `make coverage` to check our current coverage and find
  parts of the code that have weak coverage.

- *Clarifying code* -- Vidyut aims to support the next century of Sanskrit
  software, and it can do so only if the code is clear and simple. Due to our
  high development velocity, many core functions are not as clear as they could
  be and would benefit from comments and light refactoring.

- *Improve compile times* -- This is especially an issue for `vidyut-prakriya`,
  which is our largest crate by far. We think there's plenty of low-hanging
  fruit here to make the compilation much faster, but we haven't invested the
  time to look into it further.


### Preparing your change

Once your change is ready, run `make lint` and `make test` then submit a PR.

[issues]: https://github.com/ambuda-org/vidyut/issues


Spread the word
---------------

Vidyut is still a new project, and much of the Sanskrit ecosystem does not know
about it. If you know someone working on Sanskrit software, please let them
know that Vidyut exists and might help with some of their problems. Ultimately,
our hope is that Vidyut will give programmers the freedom and tools to work on
more interesting and ambitious projects.
