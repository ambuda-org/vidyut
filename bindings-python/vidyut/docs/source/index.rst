Vidyut
======

Welcome! This documentation describes the Python bindings for `Vidyut`_, a Rust toolkit that
provides reliable infrastructure for Sanskrit software. (For a quick demo of what Vidyut
can do, see our WebAssembly bindings `here`_.)

Vidyut aims to provide performant and high-quality solutions for the common problems that
Sanskrit programmers face. These problems include:

- *Transliteration*, or conversion of Sanskrit text from one script to another. (``भू`` → ``bhū``)

- *Word generation*, or converting bases and suffixes into complete words. (``भू`` → ``भवति``)

- *Word lookup*, or mapping a complete word back to its bases and suffixes. (``भवति`` → ``भू``)

- *Metrical analysis*, or understanding the meter used by a piece of Sanskrit text.

- *Sandhi changes*, or applying and undoing the sound changes that occur between pieces of
  Sanskrit text.

- *Segmentation*, or splitting a piece of Sanskrit text into distinct words.

Vidyut is under active development as part of the `Ambuda`_ project. If you want to learn more
about Vidyut or get involved, we encourage you to `join our community`_ of Sanskrit enthusiasts.
All are welcome regardless of background.

.. _here: https://ambuda-org.github.io/
.. _Vidyut: https://github.com/ambuda-org/vidyut
.. _Ambuda: https://ambuda.org
.. _join our community: https://github.com/ambuda-org/vidyut?tab=readme-ov-file#community

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   introduction
   tutorial
   chandas
   cheda
   kosha
   lipi
   prakriya
   sandhi
   api
   changes
