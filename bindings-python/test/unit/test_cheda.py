import tempfile
from pathlib import Path

import pytest

from vidyut.cheda import Chedaka, ModelBuilder


from vidyut.kosha import (
    Builder,
    PratipadikaEntry,
    PadaEntry,
)

from vidyut.prakriya import Pratipadika


def create_kosha(output_dir):
    """Create a sample Kosha."""
    words = [
        "arjunas",
        "gacCati",
    ]

    b = Builder(output_dir)
    for word in words:
        # For this test, we don't care about the semantics, so just use
        # "avyaya" for the semantics.
        pada = PadaEntry.Avyaya(
            pratipadika_entry=PratipadikaEntry.Basic(
                pratipadika=Pratipadika.basic(word), lingas=[]
            ),
        )
        b.insert(word, pada)
    b.finish()


def create_sandhi_rules(output_path):
    with open(output_path, "w") as f:
        f.write("first,second,result\n")
        f.write("i,a,y a\n")
        f.write("as,g,o g\n")


def create_model(model_dir):
    b = ModelBuilder()
    b.write_model(model_dir)


@pytest.fixture(scope="module")
def chedaka() -> Chedaka:
    with tempfile.TemporaryDirectory() as tempdir:
        tempdir: Path = Path(tempdir)
        create_kosha(tempdir / "kosha")
        create_sandhi_rules(tempdir / "sandhi-rules.csv")
        create_model(tempdir)

        return Chedaka(tempdir)


def test_init(chedaka):
    # Initialized in `chedaka` fixture.
    assert True


def test_init__directory_empty():
    with tempfile.TemporaryDirectory() as tempdir:
        tempdir: Path = Path(tempdir)

    # TODO: make more specific
    with pytest.raises(Exception):
        return Chedaka(tempdir)


def test_init__kosha_invalid():
    with tempfile.TemporaryDirectory() as tempdir:
        tempdir: Path = Path(tempdir)
        create_kosha(tempdir / "kosha")
        create_sandhi_rules(tempdir / "sandhi-rules.csv")
        create_model(tempdir)

        with open(tempdir / "kosha" / "padas.fst", "w") as f:
            f.write("junk data")

    # TODO: make more specific
    with pytest.raises(Exception):
        _c = Chedaka(tempdir)


def test_run__single_word(chedaka):
    tokens = chedaka.run("gacCati")

    assert len(tokens) == 1
    gacchati = tokens[0]
    assert gacchati.text == "gacCati"


def test_run__unknown_word(chedaka):
    tokens = chedaka.run("gacCatf")

    assert len(tokens) == 1
    gacchati = tokens[0]
    assert gacchati.text == "gacCatf"
    assert gacchati.data == PadaEntry.Unknown()


def test_run__invalid_input(chedaka):
    with pytest.raises(ValueError, match="Ascii") as _e:
        _tokens = chedaka.run("गच्छति")
