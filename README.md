<div align="center">
<h1>विद्युत्</h1>
</div>

Vidyut provides reliable infrastructure for Sanskrit software. Our main focus
is on building libraries for natural language processing.

Vidyut compiles to fast, safe, and memory-efficient native code, and it can be
bound to other programming languages with minimal work. We commit to providing
first-class support for Python bindings through [vidyut-py][vidyut-py], and we
are eager to help you create bindings for your language of choice.

Vidyut is an ambitious and transformative project, and you can help us make it
a success. If you simply want to join our community of Sanskrit enthusiasts,
see the [Community](#community) section -- we are very friendly and welcome
members of all backgrounds. For specific details on how you can contribute, see
the [Contributing](#contributing) section instead.

Vidyut is under active development as part of the [Ambuda][ambuda] project and
is published under the MIT license.

[![Build status](https://github.com/ambuda-org/vidyut/workflows/ci/badge.svg)](https://github.com/ambuda-org/vidyut/actions)


[ambuda]: https://ambuda.org
[vidyut-py]: https://github.com/ambuda-org/vidyut-py
[discord]: https://discord.gg/7rGdTyWY7Z
[issues]: https://github.com/ambuda-org/vidyut/issues


Contents
--------

- [Installation](#installation)
- [Components](#components)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Community](#community)


Installation
------------

*Vidyut is meant for programmers who are building Sanskrit software. If you are
not comfortable writing software or using tools like a command line interface,
we recommend that you [use the tools on Ambuda][ambuda-tools] instead.*

[ambuda-tools]: https://ambuda.org/tools/dictionaries

We currently offer two ways to use Vidyut:


### Through Python

We provide first-class support for Python through the [vidyut][vidyut-pypi]
Python package, which we define in the [vidyut-py][vidyut-py] repo. If you have
Python installed on your machine, you can install Vidyut as follows.

```shell
$ pip install vidyut
```

[vidyut-pypi]: https://pypi.org/project/vidyut/


### Through Rust

Vidyut is implemented in [Rust][rust], which provides low-level control with
high-level ergonomics. You can install Rust on your computer by following
the instructions [here][install-rust].

[rust]: https://www.rust-lang.org/
[install-rust]: https://www.rust-lang.org/tools/install

Once you've installed Rust, you can try cloning the Vidyut repo and running our
tests:

```shell
$ git clone https://github.com/ambuda-org/vidyut.git
$ cd vidyut
$ make test
```

Your first build will likely take a few minutes, but future builds will
be much faster.

Next, we recommend creating and collecting our rich linguistic data:

```shell
$ make create_all_data
```

This command will take several minutes, but most users will not need to re-run
this command after the first run completes.

To learn how to navigate this repo, see the [Components](#components) section.
For details on how to get involved, see the [Contributing](#contributing)
section.


Components
----------

Vidyut contains several standard components for common Sanskrit processing
tasks. These components work together well, but you can also use them
independently depending on your use case.

In Rust, components of this kind are called *crates*.


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


### [`vidyut-prakriya`][vidyut-prakriya]

`vidyut-prakriya` generates Sanskrit words with their prakriyās (derivations)
according to the rules of Paninian grammar. Our long-term goal is to provide a
complete implementation of the Ashtadhyayi.

For details, see the [vidyut-prakriya README][vidyut-prakriya].


### [`vidyut-sandhi`][vidyut-sandhi]

`vidyut-sandhi` contains various utilities for working with sandhi changes
between words. It is fast, simple, and appropriate for most use cases.

For details, see the [vidyut-sandhi README][vidyut-sandhi].


[vidyut-cheda]: vidyut-cheda/README.md
[vidyut-kosha]: vidyut-kosha/README.md
[vidyut-prakriya]: vidyut-prakriya/README.md
[vidyut-sandhi]: vidyut-sandhi/README.md


Documentation
-------------

To view documentation for all crates (including private modules and structs),
run `make docs`. This command will generate Rust's standard documentation and
open it in your default web browser.


Contributing
------------

Vidyut is an ambitious and tranformative project, and *you* can help us build
it. Depending on your background and skills, there are different ways you can
contribute.

First, we recommend [joining our community](#community) so that you can follow
along with progress on Ambuda and Vidyut and participate in discussions around
them.

If you use a tool that depends on Vidyut, please [file GitHub issues][issues]
when you see errors or surprising behavior. Please also feel free to file
issues for feature requests. We'll do our best to accommodate them.

If you know Sanskrit, please give us detailed feedback on any mistakes you see
and what you think the correction should be. This kind of work is especially
valuable for `vidyut-prakriya`.

If you can program, we encourage you to [learn some Rust][learn-rust] and get
involved with Vidyut directly. We encourage you to **be bold** and make pull
requests for work that you think will improve the project. Or if you would like
some pointers on where to get started, you can explore the issues in our [issue
tracker][issue-tracker]. All of our open work items are listed there, and we
encourage you to create a PR for any open issue. Issues tagged with `sanskrit`
require some basic familiarity with Sanskrit, and issues tagged with
`vyakarana` require a much deeper level of Sanskrit grammatical knowledge.

If you are familiar with machine learning as well, we are always eager for
improvements to `vidyut-cheda`. Our current model use simple bigram statistics;
there is plenty of room to improve!

If you want to pursue an open-ended research project, here are the components
we are most excited about:

- dependency parsing and *anvaya* generation
- search indexing that accounts for sandhi and Sanskrit's complex morphology.
- transliteration, perhaps through a port of [Aksharamukha][aksharamukha]
- meter recognition
- support for Vedic Sanskrit
- implementations of non-Paninian grammars

And if there's something else you're excited about, please [let us know about
it](#community) -- we'll probably be excited about it too!

[learn-rust]: https://doc.rust-lang.org/book/
[issue-tracker]: https://github.com/ambuda-org/vidyut/issues
[aksharamukha]: https://github.com/virtualvinodh/aksharamukha


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

[discord]: https://discord.gg/7rGdTyWY7Z
[ambuda-discuss]: https://groups.google.com/g/ambuda-discuss
[ambuda-announce]: https://groups.google.com/g/ambuda-announce 
[sanskrit-programmers]: https://groups.google.com/g/sanskrit-programmers

<div align="center">
<small>बलमिति विद्युति</small>
</div>

