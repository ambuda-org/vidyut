.. py:currentmodule:: vidyut.prakriya

Generating words with `vidyut.prakriya`
=======================================

`vidyut.prakriya` generates Sanskrit words with their prakriyās (derivations)
according to the rules of Paninian grammar. This powerful word generator is
useful both for creating Sanskrit words and for understanding how words are
derived.

.. note::
    All inputs to `vidyut.prakriya` should use the `SLP1`_ encoding scheme, and
    output is likewise encoded in SLP1. You can convert to and from SLP1 by using
    `vidyut.lipi` or your favorite transliterator.

.. _SLP1: https://en.wikipedia.org/wiki/SLP1


Quickstart
----------

All usage of `vidyut.prakriya` follows a basic flow:

1. We create a new :class:`Vyakarana` object, which is our main interface to
   the grammar. When we create :class:`Vyakarana`, we can also fine-tune the
   rules and settings the system will use. Creating this object is fast and
   lightweight.

2. We pass various arguments to :class:`Vyakarana` to define our derivation.

3. We receive a list of :class:`Prakriya` objects, which contains our final
   results and the rules that produced them.

To illustrate, let's derive the simple verb ``Bavati``:

.. testcode::

    from vidyut.prakriya import (
        Vyakarana,
        Dhatu,
        Gana,
        Prayoga,
        Purusha,
        Vacana,
        Lakara
    )

    # 1. Create a new `Vyakarana` object.
    v = Vyakarana()

    # 2. Pass arguments to define our derivation.
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    prakriyas = v.derive_tinantas(
        dhatu=bhu,
        prayoga=Prayoga.Kartari,
        lakara=Lakara.Lat,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
    )

    # 3. Receive a list of prakriyas.
    for prakriya in prakriyas:
        print(prakriya.text)
        print('===================')
        for step in prakriya.history:
            result = ' + '.join(step.result)
            print("{:<10}: {}".format(step.code, result))


This produces the following output:

.. testoutput::

    Bavati
    ===================
    1.3.1     : BU
    3.2.123   : BU + la~w
    1.3.2     : BU + la~w
    1.3.3     : BU + la~w
    1.3.9     : BU + l
    1.3.78    : BU + l
    3.4.78    : BU + tip
    1.3.3     : BU + tip
    1.3.9     : BU + ti
    3.4.113   : BU + ti
    3.1.68    : BU + Sap + ti
    1.3.3     : BU + Sap + ti
    1.3.8     : BU + Sap + ti
    1.3.9     : BU + a + ti
    3.4.113   : BU + a + ti
    1.4.13    : BU + a + ti
    7.3.84    : Bo + a + ti
    1.4.14    : Bo + a + ti
    6.1.78    : Bav + a + ti
    8.4.68    : Bav + a + ti


Basic methods
-------------

.. py:currentmodule:: vidyut.prakriya.Vyakarana

`vidyut.prakriya` currently exposes four basic methods.

The first is :meth:`derive_tinantas`, which creates verbs:

.. testcode::

    from vidyut.prakriya import *

    v = Vyakarana()
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    prakriyas = v.derive_tinantas(
        dhatu=bhu,
        prayoga=Prayoga.Kartari,
        lakara=Lakara.Lat,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
    )

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "Bavati"

The second is :meth:`derive_subantas`, which creates nominals:

.. testcode::

    from vidyut.prakriya import (
        Pratipadika,
        Linga,
        Vibhakti,
    )

    deva = Pratipadika.basic("deva")
    prakriyas = v.derive_subantas(
        pratipadika=deva,
        linga=Linga.Pum,
        vibhakti=Vibhakti.Prathama,
        vacana=Vacana.Eka,
    )

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "devaH"

The third is :meth:`derive_pratipadikas`, which combines various terms to
create a pratipadika:

.. testcode::

    from vidyut.prakriya import *

    v = Vyakarana()

    # Krdanta
    abhibhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi).with_prefixes(["aBi"])
    abhibhavaka = Pratipadika.krdanta(abhibhu, Krt.Rvul)
    prakriyas = v.derive_pratipadikas(abhibhavaka)
    assert prakriyas[0].text == "aBiBAvaka"

    # Taddhitanta
    guru = Pratipadika.basic("guru")
    gaurava = Pratipadika.taddhitanta(guru, Taddhita.aR)
    prakriyas = v.derive_pratipadikas(gaurava)
    assert prakriyas[0].text == "gOrava"

The fourth is :meth:`derive_dhatus`, which combines various terms to create a
dhatu:

.. testcode::

    from vidyut.prakriya import *

    v = Vyakarana()
    upa_r = Dhatu.mula(aupadeshika="f\\", gana=Gana.Bhvadi).with_prefixes(["upa"])
    prakriyas = v.derive_dhatus(upa_r)

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "upAr"


Specifying arguments
--------------------

.. py:currentmodule:: vidyut.prakriya

The real expressive power in this library comes from expressing different kinds
of arguments.

For example, we can modify a :class:`Dhatu` to use one or more prefixes:

.. testcode::

    bhu = Dhatu.mula("BU", Gana.Bhvadi)
    abhibhu = Dhatu.mula("BU", Gana.Bhvadi, prefixes=["abhi"])

    # Or, use the shorthand method `with_prefixes`:
    paribhu = bhu.with_prefixes(["abhi"])

Or one or more :class:`Sanadi` suffixes:

.. testcode::

    bhu = Dhatu.mula("BU", Gana.Bhvadi)
    bhavaya = Dhatu.mula("BU", Gana.Bhvadi, sanadi=[Sanadi.Ric])

    # Or, use the shorthand method `with_sanadi`.
    bubhusha = bhu.with_sanadi([Sanadi.san])

    # These can be combined with prefixes.
    abhibubhusha = bhu.with_prefixes(["aBi"]).with_sanadi([Sanadi.san])

These dhatus can be passed to any function that expects a :class:`Dhatu`:

.. testcode::

    prakriyas = v.derive_tinantas(
        dhatu=abhibubhusha,
        prayoga=Prayoga.Kartari,
        lakara=Lakara.Lat,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
    )
    assert prakriyas[0].text == 'aBibuBUzati'

Likewise, we can declare that a :class:`Pratipadika` is a *krdanta*:

.. testcode::

    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    bhavat = Pratipadika.krdanta(bhu, Krt.Satf)

Or a *taddhitanta*:

.. testcode::

    guru = Pratipadika.basic("guru")
    gaurava = Pratipadika.taddhitanta(guru, Taddhita.aR)

These pratipadikas can be passed to any function that expects a :class:`Pratipadika`:

.. testcode::

    prakriyas = v.derive_subantas(
        pratipadika=bhavat,
        linga=Linga.Pum,
        vibhakti=Vibhakti.Prathama,
        vacana=Vacana.Dvi,
    )
    assert prakriyas[0].text == 'BavantO'


Working with data files
-----------------------

`vidyut.prakriya` is more interesting when used with the side data provided in
Vidyut's standard data download. We expose this data through the :class:`Data`
object, whose main methods are :meth:`~Data.load_dhatu_entries` and
:meth:`~Data.load_sutras`.

:meth:`~Data.load_dhatu_entries` loads all dhatus from the Dhatupatha along
with their meanings::

    from vidyut.prakriya import *

    data = Data("/path/to/dhatupatha.tsv")
    dhatus = [e.dhatu for e in data.load_dhatu_entries()]

    v = Vyakarana(log_steps=False)
    for dhatu in dhatus:
        prakriyas = v.derive_tinantas(
            dhatu=dhatu,
            prayoga=Prayoga.Kartari,
            lakara=Lakara.Lat,
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
        )
        for prakriya in prakriyas:
            print(prakriya.text)

The Dhatupatha we provide is essentially identical to the one used on `ashtadhyayi.com`_.

.. _ashtadhyayi.com: https://ashtadhyayi.com

:meth:`~Data.load_sutras`, meanwhile, loads all sutras used as part of a derivation:

.. testsetup::

    class MockData:
        def load_sutras(self):
            assert hasattr(Data, "load_sutras")
            return [Sutra(source=Source.Ashtadhyayi, code="1.1.1", text="vfdDirAdEc")]
    data = MockData()

.. testcode::

    sutras = data.load_sutras()
    ashtadhyayi = [s for s in sutras if s.source == Source.Ashtadhyayi]

    assert ashtadhyayi[0].source == Source.Ashtadhyayi
    assert ashtadhyayi[0].code == "1.1.1"
    assert ashtadhyayi[0].text == "vfdDirAdEc"

:meth:`~Data.load_sutras` includes data from the Ashtadhyayi, the Unadipatha,
ganasutras from the Dhatupatha, various Varttikas, and other smaller sources.


Recipes
-------

This section contains various recipes that show how to use `vidyut.prakriya`
for different tasks.


Generate all tinantas for some dhatu and prayoga
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. testcode::

    from vidyut.prakriya import *

    v = Vyakarana(log_steps=False)
    bhu = Dhatu.mula("BU", Gana.Bhvadi)

    for lakara in Lakara.choices():
        for purusha in Purusha.choices():
            for vacana in Vacana.choices():
                prakriyas = v.derive_tinantas(
                    dhatu=bhu,
                    prayoga=Prayoga.Kartari,
                    lakara=lakara,
                    purusha=purusha,
                    vacana=vacana,
                )
                for p in prakriyas:
                    print(p.text)

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT


Generate all tinantas for some prayoga
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from vidyut.prakriya import *

    data = Data("/path/to/prakriya")
    dhatus = [e.dhatu for e in data.load_dhatu_entries()]

    v = Vyakarana(log_steps=False)

    for dhatu in dhatus:
        for prayoga in Prayoga.choices():
            for lakara in Lakara.choices():
                for purusha in Purusha.choices():
                    for vacana in Vacana.choices():
                        prakriyas = v.derive_tinantas(
                            dhatu=dhatu,
                            prayoga=prayoga,
                            lakara=lakara,
                            purusha=purusha,
                            vacana=vacana,
                        )
                        for p in prakriyas:
                            print(p.text)


Generate all subantas for some pratipadika
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. testcode::

    from vidyut.prakriya import *

    v = Vyakarana(log_steps=False)
    nara = Pratipadika.basic("nara")

    for vibhakti in Vibhakti.choices():
        for vacana in Vacana.choices():
            prakriyas = v.derive_subantas(
                pratipadika=nara,
                linga=Linga.Pum,
                vibhakti=vibhakti,
                vacana=vacana,
            )
            for p in prakriyas:
                print(vibhakti, vacana, p.text)

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT


Generate all krdantas for some dhatu
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. testcode::

    from vidyut.prakriya import *

    v = Vyakarana()
    bhu = Dhatu.mula("BU", Gana.Bhvadi)

    for krt in Krt.choices():
        anga = Pratipadika.krdanta(bhu, krt)
        prakriyas = v.derive_pratipadikas(anga)
        for p in prakriyas:
            print(krt, p.text)
        else:
            print(f"- (no results for BU + {krt})")

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT



Generate all taddhitantas for some pratipadika
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. testcode::

    from vidyut.prakriya import *

    v = Vyakarana()
    guru = Pratipadika.basic("guru")

    for taddhita in Taddhita.choices():
        anga = Pratipadika.taddhitanta(guru, taddhita)
        prakriyas = v.derive_pratipadikas(anga)
        for p in prakriyas:
            print(taddhita, p.text)
        else:
            print(f"- (no results for guru + {taddhita})")

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT


Find all rules used by a dhatu's tinantas
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. testcode::

    from vidyut.prakriya import *

    v = Vyakarana()
    bhu = Dhatu.mula("BU", Gana.Bhvadi)

    rules = set()
    for prayoga in Prayoga.choices():
        for lakara in Lakara.choices():
            for purusha in Purusha.choices():
                for vacana in Vacana.choices():
                    prakriyas = v.derive_tinantas(
                        dhatu=bhu,
                        prayoga=prayoga,
                        lakara=lakara,
                        purusha=purusha,
                        vacana=vacana,
                    )
                    for p in prakriyas:
                        for step in p.history:
                            rules.add(step.code)

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT
