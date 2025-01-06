.. py:currentmodule:: vidyut.chandas

Analyzing meter with `vidyut.chandas`
=====================================

.. warning::
   This module is experimental.

`vidyut.chandas` detects the meter used by a piece of Sanskrit text encoded in `SLP1`_.

.. _SLP1: https://en.wikipedia.org/wiki/SLP1

Usage::

    from vidyut.chandas import Chandas

    c = Chandas("/path/to/meters.tsv")
    match = c.classify("mAtaH samastajagatAM maDukEwaBAreH")

    print(match.padya)
    print('-' * 20)
    for akshara in match.aksharas:
        print(akshara.text, akshara.weight)

Output:

.. code-block:: text

    vasantatilakA
    --------------------
    mA G
    taH G
    sa L
    ma G
    sta L
    ja L
    ga L
    tAM G
    ma L
    Du L
    kE G
    wa L
    BA G
    reH G
