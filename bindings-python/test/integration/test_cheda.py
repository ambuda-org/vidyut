import os
import pytest

from vidyut.cheda import Chedaka


@pytest.fixture(scope="module")
def chedaka() -> Chedaka:
    path = os.environ["VIDYUT_DATA_DIR"]
    assert path
    return Chedaka(path)


@pytest.mark.parametrize(
    "word",
    [
        "Bavati",
        "devaH",
        "eva",
    ],
)
def test_run_for_word(chedaka, word):
    entries = chedaka.run(word)
    assert len(entries) == 1

    token = entries[0]
    assert token.info.pos is not None


@pytest.mark.parametrize(
    "raw,expected",
    [
        ("gacCatyarjunaH", ["gacCati", "arjunas"]),
        ("rAjasUnunA", ["rAja", "sUnunA"]),
    ],
)
def test_run_for_phrase(chedaka, raw, expected):
    entries = chedaka.run(raw)
    actual = [t.text for t in entries]
    assert expected == actual
