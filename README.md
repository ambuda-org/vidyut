<div align="center">
<h1>विद्युत्</h1>
</div>

Vidyut provides reliable infrastructure for Sanskrit software.

Vidyut compiles to fast and efficient native code, and it can be bound to other
programming languages with minimal work. We provide first-class support for
Python and are eager to support other bindings as well.

Vidyut is under active development as part of the [Ambuda][ambuda] project.

License: MIT

[![Build status](https://github.com/ambuda-org/vidyut/workflows/ci/badge.svg)](https://github.com/ambuda-org/vidyut/actions)

[ambuda]: https://ambuda.org


Contents
--------

- [Installation](#installation)
- [Building from source](#building-from-source)
- [Components](#components)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Community](#community)


Installation
------------

Vidyut is implemented in [Rust][rust], which provides low-level control with
high-level ergonomics. We also provide first-class support for Python bindings
through the [vidyut][vidyut-py] Python package. This section describes how to
use Vidyut either through Rust or through Python.

[pypi]: https://vidyut.readthedocs.io/en/latest/
[rust]: https://www.rust-lang.org/

### Through Rust

First, install Rust on your computer by following the instructions [here][install-rust].

[install-rust]: https://www.rust-lang.org/tools/install

Once you've done so, create a new project with `cargo new` and install Vidyut's packages:

```shell
cargo add vidyut-prakriya
cargo add vidyut-kosha
cargo add vidyut-lipi
# ... and so on
```

You can also install directly from this repository:

```shell
cargo add vidyut-prakriya --git https://github.com/ambuda-org/vidyut.git
cargo add vidyut-kosha --git https://github.com/ambuda-org/vidyut.git
cargo add vidyut-lipi --git https://github.com/ambuda-org/vidyut.git
# ... and so on
```

We recommend using our pre-built linguistic data, which is available as a ZIP file
[here][zip].

[zip]: https://github.com/ambuda-org/vidyut-py/releases/download/0.3.0/data-0.3.0.zip

For more information, see our [Rust documentation][docs-rs].

[docs-rs]: https://docs.rs/releases/search?query=vidyut


### Through Python

First, install Python on your computer. There are many ways to do so, but we
recommend installing [uv][uv] then running `uv init my-project` to create a
Python project.

[uv]: https://docs.astral.sh/uv/getting-started/installation/

Once your setup is ready, you can install the `vidyut` package:

```shell
# With Pip
$ pip install vidyut

# With uv
$ uv add vidyut
````

You can also install directly from this repository. Doing so compiles the repository
from scratch and might take several minutes, so we strongly suggest using our latest
[PyPI release][pypi] instead.

```
# The command is very slow, so pass `--verbose` to monitor its status.
pip install -e "git+https://github.com/ambuda-org/vidyut.git#egg=vidyut&subdirectory=bindings-python" --verbose
```

We recommend using our pre-built linguistic data, which is available as a ZIP file
[here][zip].

[pypi]: https://pypi.org/project/vidyut/
[zip]: https://github.com/ambuda-org/vidyut-py/releases/download/0.3.0/data-0.3.0.zip

For more information, see our [Python documentation][rtd].


Building from source
--------------------

Building from source lets you work with Vidyut as a developer and contributor.


### Through Rust

(This setup requires `cargo`. Confirm that you have `cargo` installed by running
`cargo --version`.)

Once you download the repo, you can run `cargo test --all` to run unit tests.

```shell
$ git clone https://github.com/ambuda-org/vidyut.git
$ cd vidyut
$ cargo test --all
```

(If you [install `cargo-nextest`][nextest], you can also run `make test` for a
nicer testing experience.)

Your first build will likely take a few minutes, but future builds will
be much faster.

We recommend using our pre-built linguistic data, which is available as a ZIP file
[here][zip]. Or if you prefer, you can build this data for yourself:

[zip]: https://github.com/ambuda-org/vidyut-py/releases/download/0.3.0/data-0.3.0.zip

```shell
$ cd vidyut-data
$ make create_all_data
```

Output will be written to `data/build/vidyut-latest`.

NOTE: this command is resource-intensive and might stall on slower machines.


### Through Python

(This setup requires `uv`. Confirm that you have `uv` installed by running
`uv --version`.)

Once you download the repo, you can run `make test` in the `bindings-python`
directory to run Python-specific unit tests:

```shell
$ git clone https://github.com/ambuda-org/vidyut.git
$ cd vidyut/bindings-python
$ make test
```

`make test` uses a development build, which compiles more quickly but has worse
runtime performance. To create a release build instead, run `make release`.


Components
----------

Vidyut contains several standard components for common Sanskrit processing
tasks. These components work together well, but you can also use them
independently depending on your use case.

In Rust, components of this kind are called *crates*.


### [`vidyut-chandas`][vidyut-chandas]

`vidyut-chandas` identifies the meter in some piece of Sanskrit text. This
crate is experimental, and while it is useful for common and basic use cases,
it is not a state-of-the-art solution.

For details, see the [vidyut-chandas README][vidyut-chandas].


### [`vidyut-cheda`][vidyut-cheda]

`vidyut-cheda` segments Sanskrit expressions into words then annotates those
words with their morphological data. Our segmenter is optimized for real-time
and interactive usage: it is fast, low-memory, and capably handles pathological
input.

For details, see the [vidyut-cheda README][vidyut-cheda].


### [`vidyut-kosha`][vidyut-kosha]

`vidyut-kosha` defines a key-value store that can compactly map tens of
millions of Sanskrit words to their inflectional data. Depending on the
application, storage costs can be as low as 1 byte per word. This storage
efficiency comes at the cost of increased lookup time, but in practice, we have
found that this increase is negligible and well worth the efficiency gains
elsewhere.

For details, see the [vidyut-kosha README][vidyut-kosha].


### [`vidyut-lipi`][vidyut-lipi]

`vidyut-lipi` is a transliteration library for Sanskrit and Pali that also
supports many of the scripts used within the Indosphere. Our goal is to provide
a standard transliterator that is easy to bind to other programming languages.

For details, see the [vidyut-lipi README][vidyut-lipi].


### [`vidyut-prakriya`][vidyut-prakriya]

`vidyut-prakriya` generates Sanskrit words with their prakriyās (derivations)
according to the rules of Paninian grammar. Our long-term goal is to provide a
complete implementation of the Ashtadhyayi.

For details, see the [vidyut-prakriya README][vidyut-prakriya].


### [`vidyut-sandhi`][vidyut-sandhi]

`vidyut-sandhi` contains various utilities for working with sandhi changes
between words. It is fast, simple, and appropriate for most use cases.

For details, see the [vidyut-sandhi README][vidyut-sandhi].


[vidyut-chandas]: vidyut-chandas/README.md
[vidyut-cheda]: vidyut-cheda/README.md
[vidyut-kosha]: vidyut-kosha/README.md
[vidyut-lipi]: vidyut-lipi/README.md
[vidyut-prakriya]: vidyut-prakriya/README.md
[vidyut-sandhi]: vidyut-sandhi/README.md


Documentation
-------------

To view documentation for all crates (including private modules and structs),
run `make docs`. This command will generate Rust's standard documentation and
open it in your default web browser.


Contributing
------------

Thank you for considering a contribution to Vidyut! Vidyut is an ambitious
and transformative project, and it can grow only with your help.

For all of the details, see our [CONTRIBUTING.md][contrib] file.

[contrib]: https://github.com/ambuda-org/vidyut/blob/main/CONTRIBUTING.md


Community
---------

If you're excited about our work on Vidyut, we would love to have you join our
community.

- Most of our conversation occurs on [Ambuda's Discord server][discord] on the
  `#vidyut` channel, where you can chat directly with our team and get fast
  answers to your questions. We also schedule time to spend together virtually,
  usually on a weekly frequency.

- Occasional discussion related to Vidyut might also appear on
  [ambuda-discuss][ambuda-discuss] or on standard mailing lists like
  [sanskrit-programmers][sanskrit-programmers].

- You can also follow along with project announcements on
  [ambuda-announce][ambuda-announce].

- More technical discussions will appear on our [issues][issues] page.

[discord]: https://discord.gg/7rGdTyWY7Z
[ambuda-discuss]: https://groups.google.com/g/ambuda-discuss
[sanskrit-programmers]: https://groups.google.com/g/sanskrit-programmers
[ambuda-announce]: https://groups.google.com/g/ambuda-announce
[issues]: https://github.com/ambuda-org/vidyut/issues

<div align="center">
<small>बलमिति विद्युति</small>
</div>

