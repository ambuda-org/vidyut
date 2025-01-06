Introduction
============

This package defines Python bindings for `Vidyut`_, a high-performance Sanskrit
toolkit written in Rust. It contains three main modules:

- `vidyut.kosha` compactly stores millions of Sanskrit words.

- `vidyut.lipi` transliterates Sanskrit texts between different encodings.

- `vidyut.prakriya` derives Sanskrit expressions according to Paninian grammar.

And three experimental modules:

- `vidyut.chandas` (experimental) identifies the meter of some piece of Sanskrit text.

- `vidyut.cheda` (experimental) segments and annotates Sanskrit expressions.

- `vidyut.sandhi` (experimental) has utilities for working with sandhi rules.

Our Python API is lightweight and mirrors the underlying Rust API, with minor
changes for ergonomics.

.. _Vidyut: https://github.com/ambuda-org/vidyut


System requirements
-------------------

This package requires Python 3.7 or newer. You can check your Python
installation by running the following command:

.. code-block:: text

    $ python3 --version


Installation
------------

Our Python bindings are published under the `vidyut` package on PyPI and do not
require a Rust installation. You can install the `vidyut` package like so:

.. code-block:: text

    $ pip install vidyut


Linguistic data
---------------

Vidyut is more interesting when used with our rich linguistic data, which you
can download here:

.. code-block:: text

    $ wget https://github.com/ambuda-org/vidyut-py/releases/download/0.3.0/data-0.3.0.zip
    $ unzip data-0.3.0.zip

You can use this data like so::

    from vidyut.kosha import Kosha

    kosha = kosha("data-0.3.0/kosha")
    for entry in kosha.get("gacCati"):
        print(entry)

For details on which data files to pass where, see the documentation pages for
specific modules.


Getting help
------------

To ask for help and file bugs, we encourage you to `create an issue`_ on our
repo on GitHub. For more casual questions, you can also join the `#vidyut` channel
on our `Discord`_ server.

.. _`create an issue`: https://github.com/ambuda-org/vidyut-py/issues
.. _Discord: https://discord.gg/7rGdTyWY7Z
