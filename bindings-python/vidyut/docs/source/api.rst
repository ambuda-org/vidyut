API
===

This page defines an API reference for the `vidyut` package.

The tools we used to generate this page are optimized for pure Python modules.
But since this package is not pure Python, these tools occasionally have issues
when processing our code. Known issues:

- Enum members don't have docstrings.
- Special methods (`__new__`, ...) don't have docstrings.
- Function docs don't have argument types or return types.
- Properties and attributes don't have types.

We will fix these issues as soon as we can. In the meantime, please refer to
the extensive examples elsewhere in the docs.


`vidyut.cheda`
--------------

.. automodule:: vidyut.cheda
    :members: Chedaka, Token


`vidyut.chandas`
----------------

.. automodule:: vidyut.chandas
    :members: Chandas, Match, Vrtta, VrttaPada, Jati, Akshara


`vidyut.kosha`
--------------

.. autoclass:: vidyut.kosha.Kosha
   :members:
   :special-members: __new__, __contains__
   :undoc-members:

Return types
~~~~~~~~~~~~

.. autoclass:: vidyut.kosha.PadaEntry
   :members:

.. autoclass:: vidyut.kosha.DhatuEntry
   :members:
   :undoc-members:

.. autoclass:: vidyut.kosha.PratipadikaEntry
   :members:
   :undoc-members:

Builder API
~~~~~~~~~~~

.. autoclass:: vidyut.kosha.Builder
   :members:
   :undoc-members:


`vidyut.lipi`
-------------

.. automodule:: vidyut.lipi
    :members: Scheme, detect, transliterate


`vidyut.prakriya`
-----------------

The members here are documented in four sections:

- *Main types* specifies our high-level API.
- *Disk types* specifies the types used to read data from external files.
- *Input types* specifies the types passed into our API.
- *Output types* specifies the types returned by our API.


Main types
~~~~~~~~~~

.. autoclass:: vidyut.prakriya.Vyakarana
   :members:
   :undoc-members:


Disk types
~~~~~~~~~~

.. autoclass:: vidyut.prakriya.Data
   :members:

.. autoclass:: vidyut.prakriya.DhatupathaEntry
   :members:
   :undoc-members:

.. autoclass:: vidyut.prakriya.Sutra
   :members:
   :undoc-members:


Input types
~~~~~~~~~~~

Names of suffixes, including the members of `Krt`, `Sanadi`, and `Taddhita`,
are written in the `SLP1`_ encoding scheme.

.. _SLP1: https://en.wikipedia.org/wiki/SLP1

.. autoclass:: vidyut.prakriya.Dhatu
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Pratipadika
   :members:
   :undoc-members:

.. autoclass:: vidyut.prakriya.Antargana
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Gana
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Krt
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Lakara
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Linga
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Pada
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Prayoga
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Purusha
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Sanadi
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Taddhita
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Vacana
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Vibhakti
   :members:
   :undoc-members:


Output types
~~~~~~~~~~~~

.. autoclass:: vidyut.prakriya.Prakriya
   :members:

.. autoclass:: vidyut.prakriya.Step
   :members:

.. autoclass:: vidyut.prakriya.Source
   :members:
   :undoc-members:


`vidyut.sandhi`
---------------

.. automodule:: vidyut.sandhi
    :members: Split, Splitter
