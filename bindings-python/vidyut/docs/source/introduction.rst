Introduction
============

This page will show you what Vidyut is for and how to install it on your machine.

Summary
-------

This package defines Python bindings for `Vidyut`_, a Rust toolkit that provides reliable
infrastructure for Sanskrit software. We also provide rich and interesting linguistic data
so that you don't have to generate everything from scratch. We hope that these bindings provide
a simple and ergonomic way to use Vidyut in your Python projects.

Vidut contains six modules. The first three are ready for production, and the last three are
experimental or need more work.

.. _Vidyut: https://github.com/ambuda-org/vidyut


:mod:`vidyut.kosha`
~~~~~~~~~~~~~~~~~~~

`vidyut.kosha` compactly stores millions of Sanskrit words. When combined with our official
data download, `vidyut.kosha` is a database of almost 100 million Sanskrit words, each of
which can be mapped back to its basic parts.

For example, you might use `vidyut.kosha` to learn that *bhavati* is a verb in the present
3rd-person singular form and that it is derived from *bhū*, which is part of the *bhvādi*
or "first" class of verb roots.

Increasing the quantity and quality of the data in `vidyut.kosha` is a major focus for us,
and we are eager for help in pushing it further.


:mod:`vidyut.prakriya`
~~~~~~~~~~~~~~~~~~~~~~

`vidyut.prakriya` derives Sanskrit expressions according to Paninian grammar. When combined
with our Dhatupatha and other linguistic data, `vidyut.prakriya` provides a powerful programmatic
interface to the Ashtadhyayi and other traditional grammar resources.

For example, you might use `vidyut.prakriya` to derive all verb forms of the root *abhi-bhū*
across all persons, numbers, tense-moods, voices, and secondary derivations (causative,
intensive, ...). You might then use the results to see which rules from the Ashtadhyayi were
triggered and cross-reference these rules with the rules used by some other root.

`vidyut.prakriya` generates most of the prakriyas on `ashtadhyayi.com`_, and we are slowly
integrating its results into Ambuda.

.. _`ashtadhyayi.com`: https://ashtadhyayi.com


:mod:`vidyut.lipi`
~~~~~~~~~~~~~~~~~~

`vidyut.lipi` transliterates Sanskrit texts between different schemes.

For example, you might use `vidyut.lipi` to recognize that the text ``Bavati`` is written
in the SLP1 scheme, transliterate the text to Devanagari to check against some data you own,
and display your output results in Kannada or some other scheme that your users prefer.

Although we haven't run a formal analysis, `vidyut.lipi` compares favorably with standard
Python transliterators like Aksharamukha and indic_transliteration and even handles a few
cases that these transliterators miss. But its main feature is that with a small bit of
work, you can use the same high-quality transliteration engine almost anywhere in your
software stack. For example, `here`_ you can see `vidyut-lipi` running entirely in the
browser via a WebAssembly build.

.. _here: https://ambuda-org.github.io/vidyut-lipi/


:mod:`vidyut.chandas` (experimental)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

`vidyut.chandas` identifies the meter of some piece of Sanskrit text. `vidyut.chandas`
has support for partial matching, but otherwise it is a work in progress.

For example, you might use `vidyut.chandas` to identify the meter of the first verse of
the Raghuvamsha.


:mod:`vidyut.cheda` (experimental)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

`vidyut.cheda` (experimental) segments and annotates Sanskrit expressions.

While best-in-class results come from the Dharmamitra project (https://github.com/dharmamitra),
`vidyut.cheda` aims to provide an alternative solution where latency and resource consumption
are major concerns.


:mod:`vidyut.sandhi` (experimental)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

`vidyut.sandhi` (experimental) has utilities for working with sandhi rules.

`vidyut.sandhi` is primarily a helper library used by the modules above.


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
can download like so:

.. code-block::

    import vidyut

    # `path` is wherever you want to store your data.
    path = "vidyut-0.4.0"
    vidyut.download_data(path)

You can use this data like so::

    from vidyut.kosha import Kosha

    kosha = kosha("vidyut-0.4.0/kosha")
    for entry in kosha.get("gacCati"):
        print(entry)

For details on which data files to pass where, see the documentation pages for
specific modules.


Getting help
------------

To ask for help and file bugs, we encourage you to `create an issue`_ on our
repo on GitHub. For more casual questions, you can also join the `#vidyut` channel
on our `Discord`_ server.

.. _`create an issue`: https://github.com/ambuda-org/vidyut/issues
.. _Discord: https://discord.gg/7rGdTyWY7Z
