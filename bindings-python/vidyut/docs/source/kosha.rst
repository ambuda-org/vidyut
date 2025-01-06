.. py:currentmodule:: vidyut.kosha

Storing words with `vidyut.kosha`
=================================

`vidyut.kosha` defines a key-value store that can compactly map hundreds of
millions of Sanskrit words to their inflectional data. Depending on the
application, storage costs can be as low as 1 byte per word. This storage
efficiency comes at the cost of increased lookup time, but in practice, we have
found that this increase is negligible and well worth the efficiency gains
elsewhere.

`vidyut.kosha` is tightly integrated with `vidyut.prakriya` and makes it easy
to look up a word then derive it using the Ashtadhyayi.

.. note::
    All inputs to `vidyut.kosha` should use the `SLP1`_ encoding scheme, and
    output is likewise encoded in SLP1. You can convert to and from SLP1 by using
    `vidyut.lipi` or your favorite transliterator.

.. note::
    If you are using our official data release, note that all word-final visargas
    are stored as *s* and *r* as appropriate. If you wish to look up रामः, search
    for ``"rAmas"`` instead.

.. _SLP1: https://en.wikipedia.org/wiki/SLP1


Quickstart
----------

The main class here is :class:`Kosha`, which defines an interface to the underlying
dictionary data. The main return type is :class:`~vidyut.kosha.PadaEntry`, which
defines rich morphological data about the given word.


Example usage::

    from vidyut.kosha import Kosha

    kosha = Kosha("/path/to/vidyut-data/kosha")

    for entry in kosha.get("gacCati"):
        print(entry)

    # `Kosha` also provides fast existence checks.
    assert "gacCati" in kosha

    # Simple lookups with `[]` work as well. These will raise `KeyError` if
    # the key does not exist.
    assert kosha["gacCati"]


Return types
------------

The main return types are :class:`PadaEntry`, :class:`PratipadikaEntry`, and
:class:`DhatuEntry`. Together, these types provide detailed information about
the items in the Kosha.

:class:`Kosha` will create all of these types on your behalf. On this page,
we will create these types manually so that you can better understand their
structure and usage.


:class:`PadaEntry`
~~~~~~~~~~~~~~~~~~

The core return type is :class:`PadaEntry`, which contains morphological data
for a single Sanskrit *pada*. :class:`PadaEntry` has four basic varieties:

.. testcode::

    from vidyut.kosha import PadaEntry

    def check_type(entry: PadaEntry):
        # `match` is supported as of Python 3.10.
        match entry:
            case PadaEntry.Subanta():
                return "subanta"
            case PadaEntry.Tinanta():
                return "tinanta"
            case PadaEntry.Avyaya():
                return "avyaya"
            case PadaEntry.Unknown():
                return "unknown"

    unk = PadaEntry.Unknown()
    assert check_type(unk) == "unknown"
        
The first variety is `PadaEntry.Subanta`, which models a *subanta* (nominal):

.. testcode::

    from vidyut.kosha import PratipadikaEntry, PadaEntry
    from vidyut.prakriya import Pratipadika, Linga, Vibhakti, Vacana

    rama = Pratipadika.basic("rAma")
    rama_entry = PratipadikaEntry.Basic(pratipadika=rama, lingas=[Linga.Pum])
    pada = PadaEntry.Subanta(
        pratipadika_entry=rama_entry,
        linga=Linga.Pum,
        vibhakti=Vibhakti.Prathama,
        vacana=Vacana.Eka)
    assert pada.lemma == "rAma"

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT

The second variety is `PadaEntry.Tinanta`, which models a *tinanta* (verb):

.. testcode::

    from vidyut.kosha import DhatuEntry, PadaEntry
    from vidyut.prakriya import Dhatu, Gana, Prayoga, Lakara, Purusha, Vacana

    gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
    gam_entry = DhatuEntry(dhatu=gam, clean_text="gam")
    pada = PadaEntry.Tinanta(
        dhatu_entry=gam_entry,
        prayoga=Prayoga.Kartari,
        lakara=Lakara.Lat,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka)
    assert pada.lemma == "gam"

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT

The third variety is `PadaEntry.Avyaya`, which models an *avyaya* (indeclinable):

.. testcode::

    from vidyut.kosha import PratipadikaEntry, PadaEntry
    from vidyut.prakriya import Pratipadika

    ca = Pratipadika.basic("ca")
    ca_entry = PratipadikaEntry.Basic(pratipadika=ca, lingas=[])
    pada = PadaEntry.Avyaya(pratipadika_entry=ca_entry)
    assert pada.lemma == "ca"

The fourth and final variety is `PadaEntry.Unknown`, which models that data
is missing or unknown:

.. testcode::

    from vidyut.kosha import PadaEntry

    unk = PadaEntry.Unknown()
    assert unk.lemma is None

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT


:class:`PratipadikaEntry`
~~~~~~~~~~~~~~~~~~~~~~~~~

:class:`PratipadikaEntry` is a helper class within :class:`PadaEntry`. It models
a *prātipadika* (nominal stem) along with helper information.

:class:`PratipadikaEntry` has two varieties. The first variety is
`PratipadikaEntry.Basic`, which models a basic *prātipadika* (nominal stem):

.. testcode::

    from vidyut.kosha import PratipadikaEntry
    from vidyut.prakriya import Linga

    rama = PratipadikaEntry.Basic(pratipadika=Pratipadika.basic("rAma"), lingas=[Linga.Pum])

    assert rama.lemma == "rAma"
    assert rama.lingas == [Linga.Pum]

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT


The second variety is `PratipadikaEntry.Krdanta`, which models a *kṛdanta* (verbal derivative):

.. testcode::

    from vidyut.kosha import DhatuEntry, PratipadikaEntry
    from vidyut.prakriya import Dhatu, Gana, Krt

    gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
    gam_entry = DhatuEntry(dhatu=gam, clean_text="gam")
    gata = PratipadikaEntry.Krdanta(dhatu_entry=gam_entry, krt=Krt.kta)

    assert gata.lemma == "gam"

    assert gata.dhatu_entry == gam_entry
    assert gata.krt == Krt.kta
    assert gata.prayoga is None
    assert gata.lakara is None

:class:`PratipadikaEntry.Krdanta` may also set the *prayoga* and *lakāra*, which is
useful for some *kṛdanta* derivations:

.. testcode::

    gacchat = PratipadikaEntry.Krdanta(
        dhatu_entry=gam_entry,
        krt=Krt.Satf,
        lakara=Lakara.Lat,
        prayoga=Prayoga.Kartari)

    assert gacchat.lakara == Lakara.Lat
    assert gacchat.prayoga == Prayoga.Kartari

    gamisyat = PratipadikaEntry.Krdanta(
        dhatu_entry=gam_entry,
        krt=Krt.Satf,
        lakara=Lakara.Lrt,
        prayoga=Prayoga.Kartari)

    assert gamisyat.lakara == Lakara.Lrt
    assert gamisyat.prayoga == Prayoga.Kartari

    gamyamana = PratipadikaEntry.Krdanta(
        dhatu_entry=gam_entry,
        krt=Krt.Satf,
        lakara=Lakara.Lat,
        prayoga=Prayoga.Karmani)

    assert gamyamana.lakara == Lakara.Lat
    assert gamyamana.prayoga == Prayoga.Karmani


:class:`DhatuEntry`
~~~~~~~~~~~~~~~~~~~

:class:`DhatuEntry` is a helper class within :class:`PadaEntry`. It models a
Sanskrit *dhātu* (verb root) along with useful metadata.

.. testcode::

    from vidyut.kosha import DhatuEntry
    from vidyut.prakriya import Dhatu, Gana, Krt

    gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
    gam_entry = DhatuEntry(dhatu=gam, clean_text="gam")

    assert gam_entry.dhatu.aupadeshika == "ga\\mx~"
    assert gam_entry.dhatu.gana == Gana.Bhvadi
    assert gam_entry.clean_text == "gam"


Creating prakriyas
------------------

:class:`PadaEntry`, :class:`PratipadikaEntry`, and :class:`DhatuEntry` can all be
passed to :meth:`vidyut.prakriya.Vyakarana.derive`:

.. testcode::

    from vidyut.prakriya import Vyakarana, Sanadi

    dhatu_entry = DhatuEntry(
        dhatu=Dhatu.mula("ga\\mx~", Gana.Bhvadi, prefixes=["anu"], sanadi=[Sanadi.Ric]),
        clean_text="gam")

    pratipadika_entry = PratipadikaEntry.Krdanta(
        dhatu_entry=dhatu_entry,
        krt=Krt.Satf,
        lakara=Lakara.Lat,
        prayoga=Prayoga.Kartari)

    pada_entry = PadaEntry.Subanta(
        pratipadika_entry=pratipadika_entry,
        linga=Linga.Pum,
        vibhakti=Vibhakti.Dvitiya,
        vacana=Vacana.Eka)

    v = Vyakarana()
    # assert [p.text for p in v.derive(dhatu_entry)] == ["anugami"]
    assert [p.text for p in v.derive(pratipadika_entry)] == ["anugamayat"]
    assert [p.text for p in v.derive(pada_entry)] == ["anugamayantam"]

.. testoutput::
   :hide:
   :options: +IGNORE_RESULT

.. note::
    What is the difference between :class:`~vidyut.prakriya.Pada` and :class:`PadaEntry`?
    Why do we have both types?

    Think of the `vidyut.prakriya` types as input types and the `vidyut.kosha` types as
    output types. Where :class:`~vidyut.prakriya.Pada` tells us *how* to create a *pada*,
    :class:`PadaEntry` shows us the *results* of creating a *pada*. This is why the
    `vidyut.kosha` types contain useful metadata:

    - :class:`DhatuEntry` contains `clean_text`, which is the dictionary version
      of the dhatu with sandhi applied and accent marks removed.

    - :class:`PratipadikaEntry` contains `lingas`, which includes the lingas
      typcially used with this pratipadika.

    We will add more metadata like this in future releases.
