How to contribute to vidyut-py
==============================

Thank you for your interest in contributing to vidyut-py!

- [Scope](#scope)
- [Reporting issues](#reporting-issues)
- [Understanding the code](#understanding-the-code)
- [Submitting changes](#reporting-issues)


Scope
-----

Vidyut-py provides Python bindings for our [Vidyut][vidyut-rs] Rust library.
Our work here focuses specifically on the Rust-Python conversion and *not* on
the underlying logic. This means we focus on:

- improving comments, documentation, and type hints
- adding better error messages
- making our API more exhaustive
- making our API more Pythonic
- adding extensive tests for our Rust-Python conversions

For all other work, please see the [main Vidyut repo][vidyut-rs] instead.

[vidyut-rs]: https://github.com/ambuda-org/vidyut.git


Building in development
-----------------------

You can build this package directly like so:

    $ git clone https://github.com/ambuda-org/vidyut-py.git
    $ cd vidyut-py
    $ make install

You can also build Vidyut's linguistic data yourself from our Rust repo:

    $ git clone https://github.com/ambuda-org/vidyut.git
    $ cd vidyut/vidyut-cheda
    $ make install


Reporting issues
----------------

If you encounter any issues with this library, please file a [GitHub
issue][issues]. If the issue seems like a logical error, please file the issue
in the main Vidyut repo instead.

We are happy to answer more general support questions through GitHub issues as
well. For the fastest response, we recomend using the `#vidyut` channel [on our
Discord server][discord].

[discord]: https://discord.gg/7rGdTyWY7Z
[issues]: https://github.com/ambuda-org/vidyut-py/issues


Understanding the code
----------------------

We define our Python bindings with [PyO3](https://pyo3.rs) and publish
the package with [Maturin](https://www.maturin.rs). Both of these libraries
have great user guides that are suitable for beginners.

Here are the most important directories in this repo:

- `env` defines our virtualenv for Maturin and will contain our final package.
  `env` is created when you run `make install`.

- `src` contains our Rust code with PyO3 annotations. This code tells Maturin
  how to define a native Python module. To better understand this code, see the
  header comments in `src/lib.rs`.

- `vidyut` contains simple Python wrapper code and docstrings. We use the code
  here to make our native module more user-friendly. To better understand this
  code, see the comments in `__init__.py`.


Submitting changes
------------------

First, ensure that you can set up your development environment correctly:

```shell
$ git clone https://github.com/ambuda-org/vidyut-py.git
$ cd vidyut-py
$ make install
$ make test
```

Next, feel free to improve whatever you think needs improving. Our [issues
tracker][issues] contains major issues, but anything is fair game.

Once your change is ready, run `make lint` and `make test` then submit a PR.

[issues]: https://github.com/ambuda-org/vidyut-py/issues
