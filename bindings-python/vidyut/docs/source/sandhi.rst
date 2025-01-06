.. py:currentmodule:: vidyut.sandhi

Handling sandhi with `vidyut.sandhi`
====================================

.. warning::
   This module is experimental.

`vidyut.sandhi` contains various utilities for working with sandhi changes
between words. It is fast, simple, and appropriate for common use cases.

The main class here is :class:`~vidyut.sandhi.Splitter`, which manages a list
of sandhi rules. The main return type is :class:`Split`, which contains
information about the split.

`vidyut.sandhi` tends to overgenerate, so we suggest using `vidyut.sandhi` only
as part of a larger system. This is the exact approach we take with
`vidyut.cheda`, which combines these splits with a dictionary and a scoring
model.

Example usage::

    from vidyut.sandhi import Splitter

    splitter = Splitter.from_csv("/path/to/vidyut-data/sandhi/rules.csv")

    for split in splitter.split_at("ityapi", 2):
        print(split.first, split.second, split.is_valid)

