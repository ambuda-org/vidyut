from vidyut.kosha import DhatuEntry, PratipadikaEntry, PadaEntry
from vidyut.prakriya import (
    Vyakarana,
    Dhatu,
    Gana,
    Pratipadika,
    Linga,
    Krt,
    Vibhakti,
    Vacana,
    Prayoga,
    Purusha,
    Lakara,
)


def test_dhatu_entry():
    gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
    entry = DhatuEntry(dhatu=gam, clean_text="gam")

    assert entry.dhatu == gam
    assert entry.clean_text == "gam"

    # Nested attributes
    assert entry.dhatu.aupadeshika == "ga\\mx~"
    assert entry.dhatu.gana == Gana.Bhvadi

    v = Vyakarana()
    results = {p.text for p in v.derive(entry)}
    assert results == {"gam"}


def test_dhatu_entry__dunders():
    gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
    entry_gam = DhatuEntry(dhatu=gam, clean_text="gam")

    bhu = Dhatu.mula("BU", Gana.Bhvadi)
    entry_bhu = DhatuEntry(dhatu=bhu, clean_text="BU")

    # __eq__, __ne__
    assert entry_gam == entry_gam
    assert entry_gam != entry_bhu

    # __lt__, __gt__
    _ = sorted([entry_gam, entry_bhu])

    # __repr__
    assert repr(entry_gam) == (
        "DhatuEntry(dhatu=Dhatu(aupadeshika='ga\\mx~', gana=Gana.Bhvadi), "
        "clean_text='gam')"
    )


def test_pratipadika_entry__basic():
    rama = Pratipadika.basic("rAma")
    rama_entry = PratipadikaEntry.Basic(pratipadika=rama, lingas=[Linga.Pum])
    assert rama_entry.pratipadika == rama
    assert rama_entry.lingas == [Linga.Pum]

    v = Vyakarana()
    results = {p.text for p in v.derive(rama_entry)}
    assert results == {"rAma"}


def test_pratipadika_entry__krdanta():
    gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
    gam_entry = DhatuEntry(dhatu=gam, clean_text="gam")
    gata = PratipadikaEntry.Krdanta(dhatu_entry=gam_entry, krt=Krt.kta)
    assert gata.dhatu_entry == gam_entry
    assert gata.krt == Krt.kta
    assert gata.prayoga == None
    assert gata.lakara == None

    v = Vyakarana()
    results = {p.text for p in v.derive(gata)}
    assert results == {"gata"}


def test_pratipadika_entry__dunders():
    rama = Pratipadika.basic("rAma")
    rama_entry = PratipadikaEntry.Basic(pratipadika=rama, lingas=[Linga.Pum])

    gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
    gam_entry = DhatuEntry(dhatu=gam, clean_text="gam")
    gata_entry = PratipadikaEntry.Krdanta(dhatu_entry=gam_entry, krt=Krt.kta)

    # __eq__, __ne__
    assert rama_entry == rama_entry
    assert rama_entry != gata_entry

    # __lt__, __gt__
    _ = sorted([rama_entry, gata_entry])

    # __repr__
    assert repr(rama_entry) == (
        "PratipadikaEntry.Basic(pratipadika=Pratipadika(text='rAma'), lingas=[Linga.Pum])"
    )

    assert repr(gata_entry) == (
        "PratipadikaEntry.Krdanta(dhatu_entry=DhatuEntry(dhatu="
        "Dhatu(aupadeshika='ga\\mx~', gana=Gana.Bhvadi), clean_text='gam'), "
        "krt=Krt.kta, prayoga=None, lakara=None)"
    )


def test_pada_entry__subanta():
    rama = Pratipadika.basic("rAma")
    rama_entry = PratipadikaEntry.Basic(pratipadika=rama, lingas=[Linga.Pum])

    pada = PadaEntry.Subanta(
        pratipadika_entry=rama_entry,
        linga=Linga.Pum,
        vibhakti=Vibhakti.Prathama,
        vacana=Vacana.Eka,
    )
    assert pada.pratipadika_entry == rama_entry
    assert pada.linga == Linga.Pum
    assert pada.vibhakti == Vibhakti.Prathama
    assert pada.vacana == Vacana.Eka
    assert pada.lemma == "rAma"

    v = Vyakarana()
    results = {p.text for p in v.derive(pada)}
    assert results == {"rAmaH"}


def test_pada_entry__tinanta():
    gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
    gam_entry = DhatuEntry(dhatu=gam, clean_text="gam")
    pada = PadaEntry.Tinanta(
        dhatu_entry=gam_entry,
        prayoga=Prayoga.Kartari,
        lakara=Lakara.Lat,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
    )
    assert pada.dhatu_entry == gam_entry
    assert pada.prayoga == Prayoga.Kartari
    assert pada.lakara == Lakara.Lat
    assert pada.purusha == Purusha.Prathama
    assert pada.vacana == Vacana.Eka
    assert pada.lemma == "gam"

    v = Vyakarana()
    results = {p.text for p in v.derive(pada)}
    assert results == {"gacCati"}


def test_pada_entry__avyaya():
    ca = Pratipadika.basic("ca")
    ca_entry = PratipadikaEntry.Basic(pratipadika=ca, lingas=[])
    pada = PadaEntry.Avyaya(pratipadika_entry=ca_entry)

    assert pada.pratipadika_entry == ca_entry
    assert pada.lemma == "ca"

    v = Vyakarana()
    results = {p.text for p in v.derive(pada)}
    assert results == {"ca"}


def test_pada_entry__unknown():
    unk = PadaEntry.Unknown()
    assert unk.lemma == None


def test_pada_entry__dunders():
    rama = Pratipadika.basic("rAma")
    rama_entry = PratipadikaEntry.Basic(pratipadika=rama, lingas=[Linga.Pum])
    rama_pada = PadaEntry.Subanta(
        pratipadika_entry=rama_entry,
        linga=Linga.Pum,
        vibhakti=Vibhakti.Prathama,
        vacana=Vacana.Eka,
    )

    gam = Dhatu.mula("ga\\mx~", Gana.Bhvadi)
    gam_entry = DhatuEntry(dhatu=gam, clean_text="gam")
    gacchati_pada = PadaEntry.Tinanta(
        dhatu_entry=gam_entry,
        prayoga=Prayoga.Kartari,
        lakara=Lakara.Lat,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
    )

    ca = Pratipadika.basic("ca")
    ca_entry = PratipadikaEntry.Basic(pratipadika=ca, lingas=[])
    ca_pada = PadaEntry.Avyaya(pratipadika_entry=ca_entry)

    unk_pada = PadaEntry.Unknown()

    # __eq__, __ne__
    assert rama_pada == rama_pada
    assert rama_pada != gacchati_pada

    # __lt__, __gt__
    _ = sorted([rama_pada, gacchati_pada, ca_pada, unk_pada])

    # __repr__
    assert repr(rama_pada) == (
        "PadaEntry.Subanta("
        "pratipadika_entry=PratipadikaEntry.Basic(pratipadika=Pratipadika(text='rAma'), lingas=[Linga.Pum]), "
        "linga=Linga.Pum, vibhakti=Vibhakti.Prathama, vacana=Vacana.Eka)"
    )

    assert repr(gacchati_pada) == (
        "PadaEntry.Tinanta(dhatu_entry=DhatuEntry(dhatu="
        "Dhatu(aupadeshika='ga\\mx~', gana=Gana.Bhvadi), clean_text='gam'), "
        "prayoga=Prayoga.Kartari, lakara=Lakara.Lat, purusha=Purusha.Prathama, vacana=Vacana.Eka)"
    )

    assert repr(ca_pada) == (
        "PadaEntry.Avyaya(pratipadika_entry="
        "PratipadikaEntry.Basic(pratipadika=Pratipadika(text='ca'), lingas=[]))"
    )

    assert repr(unk_pada) == "PadaEntry.Unknown()"
