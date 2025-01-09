Tutorial
========

Welcome! This tutorial will teach you what Vidyut is and how to use it. It does
so by showing you how to use Vidyut to build a simple Sanskrit dictionary.


Setup
-----

First, install Vidyut and its side data:

.. code-block:: text

    $ pip install vidyut
    $ curl -LO https://github.com/ambuda-org/vidyut-py/releases/download/0.3.0/data-0.3.0.zip
    $ unzip data-0.3.0.zip

You can confirm that your setup works by trying to load :class:`~vidyut.kosha.Kosha`:

.. code-block::

    # dict.py
    import sys
    from vidyut.kosha import Kosha

    kosha = Kosha("vidyut-latest/kosha")

    query = sys.argv[1]
    for entry in kosha.get(query):
        print(entry)

Try running `python dict.py Bavati` to see all results for the word *bhavati*.


Handling kosha data
-------------------

In our kosha data, words that end with the *visarga* are instead made to end
with either ``'s'`` or ``'r'``. While these letters usefully tell us about the
sandhi rules that a word follows, they are confusing when looking up words. In
addition, our kosha prints out long ``__repr__`` results that are hard to read.
So, let's fix both of these problems and make our interface a little nicer::

    import sys
    from vidyut.kosha import Kosha, PadaEntry


    def get_all(kosha: Kosha, query: str) -> list:
        if query.endswith("H"):
            queries = [query[:-1] + 's', query[:-1] + 'r']
        else:
            queries = [query]

        results = []
        for query in queries:
            results += kosha.get(query)
            return results


    def display_entry(e: PadaEntry):
        match e:
            case PadaEntry.Subanta():
                print(f"subanta: {e.lemma:10} {e.linga}, {e.vibhakti}, {e.vacana}")
            case PadaEntry.Avyaya():
                print(f"avyaya: {e.lemma:10}")
            case PadaEntry.Tinanta():
                print(f"tinanta: {e.lemma:10} {e.prayoga}, {e.lakara}, {e.purusha}, {e.vacana}")
            case PadaEntry.Unknown():
                print("unknown type")


    def run(query: str):
        kosha = Kosha("vidyut-latest/kosha")
        entries = get_all(kosha, query)
        for entry in entries:
            display_entry(entry)


    run(sys.argv[1])

Here, :class:`~vidyut.kosha.PadaEntry` uses Sanskrit names for grammatical concepts:

.. csv-table:: Sanskrit terms with English equivalents
    :header: "Sanskrit", "English"

    *pada*, word
    *subanta*, nominal
    *tiṅanta*, verb
    *avyaya*, uninflected word
    *liṅga*, gender
    *vibhakti*, case
    *vacana*, number
    *prayoga*, voice
    *lakāra*, tense-mood
    *puruṣa*, person


Transliteration
---------------

Vidyut's kosha, like most of Vidyut's libraries, expects text to be encoded in
SLP1.  SLP1 is convenient for programs, but it is less so for human beings. So
as a first step, let's make this program more human-friendly::

    import sys
    from vidyut.kosha import Kosha, PadaEntry
    from vidyut.lipi import Scheme, transliterate, detect

    ...

    def display_entry(e: PadaEntry, output_scheme: Scheme):
        lemma = transliterate(e.lemma, Scheme.Slp1, output_scheme)
        match e:
            case PadaEntry.Subanta():
                print(f"subanta: {e.lemma:10} {e.linga}, {e.vibhakti}, {e.vacana}")
            case PadaEntry.Avyaya():
                print(f"avyaya: {e.lemma:10}")
            case PadaEntry.Tinanta():
                print(f"tinanta: {e.lemma:10} {e.prayoga}, {e.lakara}, {e.purusha}, {e.vacana}")
            case PadaEntry.Unknown():
                print("unknown type")


    def run(query: str, output_scheme: Scheme):
        # Convert to SLP1.
        # `detect` can return `None`, so use `HarvardKyoto` as a backup.
        encoding = detect(query) or Scheme.HarvardKyoto
        slp_query = transliterate(query, encoding, Scheme.Slp1)

        kosha = Kosha("vidyut-latest/kosha")
        entries = get_all(kosha, slp_query)
        for entry in entries:
            display_entry(entry, output_scheme)


    run(sys.argv[1], Scheme.Iast)

Now, try running `python dict.py bhavāmi` to see all results for the word *bhavāmi*.


Generating words
----------------

Now that we have a basic dictionary in place, let's extend it by showing related
words. Here, we'll use :mod:`vidyut.prakriya` to show the various forms that a
verb might take::

    from vidyut.prakriya import Vyakarana, Pada, Dhatu, Prayoga, Lakara, Purusha, Vacana


    def show_tinantas(dhatu: Dhatu, output_scheme: Scheme):
        v = Vyakarana()
        for lakara in Lakara.choices():
            pada = Pada.Tinanta(dhatu, Prayoga.Kartari, lakara, Purusha.Prathama, Vacana.Eka)
            prakriyas = v.derive(pada)
            for p in prakriyas:
                text = transliterate(p.text, Scheme.Slp1, output_scheme)
                print(f"- {text}")


    def display_entry(e: PadaEntry, output_scheme: Scheme):
        lemma = transliterate(e.lemma, Scheme.Slp1, output_scheme)
        match e:
            case PadaEntry.Subanta():
                print(f"subanta: {e.lemma:10} {e.linga}, {e.vibhakti}, {e.vacana}")
            case PadaEntry.Avyaya():
                print(f"avyaya: {e.lemma:10}")
            case PadaEntry.Tinanta():
                print(f"tinanta: {e.lemma:10} {e.prayoga}, {e.lakara}, {e.purusha}, {e.vacana}")
                show_tinantas(e.dhatu_entry.dhatu, output_scheme)
            case PadaEntry.Unknown():
                print("unknown type")


Our word generator uses traditional Sanskrit terms like *prayoga* and *vacana*
because it closely follows the rules of the *Aṣṭādhyāyī*, the core text of
the Sanskrit grammatical tradition. For details of what these terms mean and
their English equivalents, see our :doc:`documentation <prakriya>` for
:mod:`vidyut.prakriya`.
