import os
import pytest
from pathlib import Path

from vidyut.kosha import Kosha, PartOfSpeech, Purusha, Vacana, Lakara, Linga, Vibhakti


@pytest.fixture(scope="module")
def kosha() -> Kosha:
    path = os.environ["VIDYUT_DATA_DIR"]
    kosha_path = Path(path).absolute() / "kosha"
    assert kosha_path.exists()
    return Kosha(kosha_path)


def test_basic_tinanta(kosha):
    entries = kosha.get_all("Bavati")
    entries = [e for e in entries if e.pos == PartOfSpeech.Tinanta]

    bhavati = entries[0]
    assert bhavati.lemma == "BU"
    assert bhavati.purusha == Purusha.Prathama
    assert bhavati.vacana == Vacana.Eka
    assert bhavati.lakara == Lakara.Lat

    assert repr(bhavati) == (
        "Pada(pos=PartOfSpeech.Tinanta, dhatu=Dhatu(text='BU'), "
        "purusha=Purusha.Prathama, vacana=Vacana.Eka, lakara=Lakara.Lat, "
        "pada_prayoga=PadaPrayoga.Parasmaipada)"
    )


def test_basic_subanta(kosha):
    entries = kosha.get_all("devasya")
    entries = [
        e for e in entries if e.pos == PartOfSpeech.Subanta and e.linga == Linga.Pum
    ]

    devasya = entries[0]
    assert devasya.lemma == "deva"
    assert devasya.linga == Linga.Pum
    assert devasya.vibhakti == Vibhakti.Sasthi
    assert devasya.vacana == Vacana.Eka

    assert repr(devasya) == (
        "Pada(pos=PartOfSpeech.Subanta, pratipadika=Pratipadika(text='deva'), "
        "linga=Linga.Pum, vibhakti=Vibhakti.Sasthi, vacana=Vacana.Eka, "
        "is_purvapada=False)"
    )


def test_basic_avyaya(kosha):
    entries = kosha.get_all("ca")
    entries = [e for e in entries if e.pos == PartOfSpeech.Avyaya]

    ca = entries[0]
    assert ca.lemma == "ca"

    assert repr(ca) == (
        "Pada(pos=PartOfSpeech.Avyaya, pratipadika=Pratipadika(text='ca'), "
        "is_purvapada=False)"
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
    entries = kosha.get_all(word)
    assert any(e.pos == PartOfSpeech.Tinanta for e in entries)


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
    entries = kosha.get_all(word)
    assert any(e.pos == PartOfSpeech.Subanta for e in entries)


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
    entries = kosha.get_all(word)
    assert any(e.pos == PartOfSpeech.Avyaya for e in entries)
