import os

import pytest
from pathlib import Path

from vidyut.kosha import Kosha, PadaEntry, DhatuEntry, PratipadikaEntry
from vidyut.prakriya import Purusha, Vacana, Lakara, Linga, Vibhakti


@pytest.fixture(scope="module")
def kosha() -> Kosha:
    path = os.environ["VIDYUT_DATA_DIR"]
    kosha_path = Path(path).absolute() / "kosha"
    assert kosha_path.exists()
    return Kosha(kosha_path)


def test_basic_tinanta(kosha):
    entries = kosha.get("Bavati")
    entries = [e for e in entries if isinstance(e, PadaEntry.Tinanta)]

    bhavati = entries[0]
    assert bhavati.lemma == "BU"
    assert bhavati.purusha == Purusha.Prathama
    assert bhavati.vacana == Vacana.Eka
    assert bhavati.lakara == Lakara.Lat

    assert repr(bhavati) == (
        "PadaEntry.Tinanta(dhatu_entry=DhatuEntry("
        "dhatu=Dhatu(aupadeshika='BU', gana=Gana.Bhvadi), clean_text='BU'), "
        "prayoga=Prayoga.Kartari, lakara=Lakara.Lat, purusha=Purusha.Prathama, vacana=Vacana.Eka)"
    )


def test_basic_subanta(kosha):
    entries = kosha.get("devasya")
    entries = [
        e for e in entries if isinstance(e, PadaEntry.Subanta) and e.linga == Linga.Pum
    ]

    devasya = entries[0]
    assert devasya.lemma == "deva"
    assert devasya.linga == Linga.Pum
    assert devasya.vibhakti == Vibhakti.Sasthi
    assert devasya.vacana == Vacana.Eka

    assert repr(devasya) == (
        "PadaEntry.Subanta(pratipadika_entry="
        "PratipadikaEntry.Basic(pratipadika=Pratipadika(text='deva'), lingas=[Linga.Pum]), "
        "linga=Linga.Pum, vibhakti=Vibhakti.Sasthi, vacana=Vacana.Eka)"
    )


def test_basic_avyaya(kosha):
    entries = kosha.get("ca")
    entries = [e for e in entries if isinstance(e, PadaEntry.Avyaya)]

    ca = entries[0]
    assert ca.lemma == "ca"

    assert repr(ca) == (
        "PadaEntry.Avyaya(pratipadika_entry="
        "PratipadikaEntry.Basic(pratipadika=Pratipadika(text='ca'), lingas=[Linga.Pum]))"
    )


@pytest.mark.parametrize(
    "word",
    [
        # Basic verb
        "Bavati",
        # Vacana
        "BavataH",
        "Bavanti",
        # Purusha
        "Bavasi",
        "BavAmi",
        # Lakaras
        "Bavati",
        "baBUva",
        "BavitA",
        "Bavizyati",
        "Bavatu",
        "aBavat",
        "Bavet",
        "BUyAt",
        "aBUt",
        "aBavizyat",
        # Nijantas
        "BAvayati",
        "BAvayitA",
        "BAvayizyati",
        "BAvayatu",
        "aBAvayat",
        "BAvayet",
        # sannantas
        "buBUzati",
        "buBUzitA",
        # Upasargas
        "praBavati",
        "praBAvayati",
        "prabuBUzati",
    ],
)
def test_contains_tinanta(kosha, word):
    if word.endswith("H"):
        word = word[:-1] + 's'
    entries = kosha.get(word)
    assert any(isinstance(e, PadaEntry.Tinanta) for e in entries)


@pytest.mark.parametrize(
    "word",
    [
        # Basic subanta
        "Svetas",
        # Vacana
        "SvetO",
        "SvetAs",
        # Vibhakti
        "Svetam",
        "Svetena",
        "SvetAya",
        "SvetAt",
        "Svetasya",
        "Svete",
        "Sveta",
        # Linga
        "SvetA",
        "SvetAni",
        # Vowel stems
        "senayA",
        "agninA",
        "nadyA",
        "guruRA",
        "vaDvA",
        "pitrA",
        "kartrA",
        "rAyA",
        "gavA",
        "nAvA",
        # Consonant stems
        "yoginA",
        "AtmanA",
        "BagavatA",
        "vAcA",
    ],
)
def test_contains_subanta(kosha, word):
    if word.endswith("H"):
        word = word[:-1] + 's'
    entries = kosha.get(word)
    assert any(isinstance(e, PadaEntry.Subanta) for e in entries)


@pytest.mark.parametrize(
    "word",
    [
        # Basic avyayas
        "ca",
        "tu",
        "eva",
        "hi",
        "taTA",
        "iti",
    ],
)
def test_contains_avyaya(kosha, word):
    entries = kosha.get(word)
    assert any(isinstance(e, PadaEntry.Avyaya) for e in entries)
