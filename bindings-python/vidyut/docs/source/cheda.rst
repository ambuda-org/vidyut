.. py:currentmodule:: vidyut.cheda

Segmenting and tagging with `vidyut.cheda`
==========================================

.. warning::
   This module is incomplete and may be deleted in a future release. We recommend
   using the `Dharmamitra`_ analyzer instead if possible.

.. _Dharmamitra: https://github.com/sebastian-nehrdich/byt5-sanskrit-analyzers

`vidyut.cheda` segments Sanskrit expressions into words then annotates those
words with their morphological data. Our segmenter is optimized for real-time
and interactive usage: it is fast, low-memory, and capably handles pathological
input.

The main class here is :class:`~vidyut.cheda.Chedaka`, which defines a
segmenter. The main return type is :class:`~vidyut.cheda.Token`, which contains
the segmented text with its associated :class:`~vidyut.kosha.Pada` data.

Example usage::

    from vidyut.cheda import Chedaka

    chedaka = Chedaka("/path/to/vidyut-data")

    for token in chedaka.run('gacCati'):
        print(token.text, token.data)

