import re
import tempfile

import pytest


from vidyut.kosha import (
    Builder,
    Kosha,
    DhatuEntry,
    PratipadikaEntry,
    PadaEntry,
)

from vidyut.prakriya import (
    Gana,
    Purusha,
    Vacana,
    Krt,
    Prayoga,
    Pratipadika,
    Lakara,
    Linga,
    Vibhakti,
    Dhatu,
)


@pytest.fixture(scope="session")
def kosha():
    gam_entry = DhatuEntry(dhatu=Dhatu.mula("ga\\mx", Gana.Bhvadi), clean_text="gam")
    gacchati_tin = PadaEntry.Tinanta(
        dhatu_entry=gam_entry,
        prayoga=Prayoga.Kartari,
        lakara=Lakara.Lat,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
    )

    gacchati_sup = PadaEntry.Subanta(
        pratipadika_entry=PratipadikaEntry.Krdanta(dhatu_entry=gam_entry, krt=Krt.Satf),
        linga=Linga.Pum,
        vibhakti=Vibhakti.Saptami,
        vacana=Vacana.Eka,
    )

    ish_entry = DhatuEntry(dhatu=Dhatu.mula("izu~", Gana.Tudadi), clean_text="iz")
    icchati = PadaEntry.Tinanta(
        dhatu_entry=ish_entry,
        prayoga=Prayoga.Kartari,
        lakara=Lakara.Lat,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
    )

    with tempfile.TemporaryDirectory() as tempdir:
        b = Builder(tempdir)
        b.insert("gacCati", gacchati_tin)
        b.insert("gacCati", gacchati_sup)
        b.insert("icCati", icchati)
        b.finish()

        return Kosha(tempdir)


def test_init(kosha):
    assert kosha


def test_init_fails_if_file_does_not_exist():
    with pytest.raises(OSError):
        with tempfile.TemporaryDirectory() as tempdir:
            _kosha = Kosha(tempdir)


def test_contains(kosha):
    assert "Bavati" not in kosha
    assert "gacCati" in kosha


def test_getitem(kosha):
    assert kosha["gacCati"] is not None

    with pytest.raises(KeyError):
        _ = kosha["missing"]


def test_len(kosha):
    assert len(kosha) == 3


def test_repr(kosha):
    assert repr(kosha) == "Kosha()"


@pytest.mark.skip("Not implemented yet")
def test_contains_prefix(kosha):
    for prefix in ["", "g", "ga", "gac", "gacC", "gacCa", "gacCat", "gacCati"]:
        assert kosha.contains_prefix(prefix)

    assert not kosha.contains_prefix("gacCati2")


def test_get(kosha):
    gam_entry = DhatuEntry(dhatu=Dhatu.mula("ga\\mx", Gana.Bhvadi), clean_text="gam")
    gacchati_tin = PadaEntry.Tinanta(
        dhatu_entry=gam_entry,
        prayoga=Prayoga.Kartari,
        lakara=Lakara.Lat,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
    )

    gacchati_sup = PadaEntry.Subanta(
        pratipadika_entry=PratipadikaEntry.Krdanta(dhatu_entry=gam_entry, krt=Krt.Satf),
        linga=Linga.Pum,
        vibhakti=Vibhakti.Saptami,
        vacana=Vacana.Eka,
    )

    assert len(kosha.get("Bavati")) == 0
    assert len(kosha.get("gacCati")) == 2

    [tin, sup] = kosha.get("gacCati")
    assert tin == gacchati_tin
    assert tin.lemma == "gam"
    assert tin.dhatu_entry == gam_entry

    assert sup == gacchati_sup
    assert tin.lemma == "gam"
    assert tin.dhatu_entry == gam_entry


def test_dhatus(kosha):
    gam_entry = DhatuEntry(dhatu=Dhatu.mula("ga\\mx", Gana.Bhvadi), clean_text="gam")
    ish_entry = DhatuEntry(dhatu=Dhatu.mula("izu~", Gana.Tudadi), clean_text="iz")

    items = [d for d in kosha.dhatus()]
    assert len(items) == 2
    assert items[0] == gam_entry
    assert items[1] == ish_entry


def test_pratipadikas(kosha):
    gam_entry = DhatuEntry(dhatu=Dhatu.mula("ga\\mx", Gana.Bhvadi), clean_text="gam")
    pratipadika_entry = PratipadikaEntry.Krdanta(dhatu_entry=gam_entry, krt=Krt.Satf)

    items = [d for d in kosha.pratipadikas()]
    assert len(items) == 1
    assert items[0] == pratipadika_entry
