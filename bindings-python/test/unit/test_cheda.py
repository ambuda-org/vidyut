# DEBUG
"""
import tempfile
from pathlib import Path

import pytest

from vidyut.cheda import Chedaka


from vidyut.kosha import (
    Builder,
    Pada,
    Pratipadika,
)


def create_kosha(output_dir):
    ""Create a sample Kosha.""
    words = [
        "arjunas",
        "gacCati",
    ]

    b = Builder(output_dir)
    for word in words:
        # For this test, we don't care about the semantics, so just use
        # "avyaya" for the semantics.
        pada = Pada.make_avyaya(
            pratipadika=Pratipadika(text=word),
        )
        b.insert(word, pada)
    b.finish()


def create_sandhi_rules(output_path):
    with open(output_path, "w") as f:
        f.write("first,second,result\n")
        f.write("i,a,y a\n")
        f.write("as,g,o g\n")


def create_model_files(model_dir):
    model_dir.mkdir(parents=True)

    with open(model_dir / "transitions.csv", "w") as f:
        f.write("prev_state,cur_state,probability")

    with open(model_dir / "emissions.csv", "w") as f:
        f.write("state,token,probability")

    with open(model_dir / "lemma-counts.csv", "w") as f:
        f.write("lemma,tag,count")


@pytest.fixture(scope="module")
def chedaka() -> Chedaka:
    with tempfile.TemporaryDirectory() as tempdir:
        tempdir: Path = Path(tempdir)
        create_kosha(tempdir / "kosha")
        create_sandhi_rules(tempdir / "sandhi-rules.csv")
        create_model_files(tempdir / "model")

        return Chedaka(tempdir)


def test_init(chedaka):
    assert True


def test_init__directory_empty():
    with tempfile.TemporaryDirectory() as tempdir:
        tempdir: Path = Path(tempdir)
    with pytest.raises(OSError):
        return Chedaka(tempdir)


def test_init__kosha_invalid():
    with tempfile.TemporaryDirectory() as tempdir:
        tempdir: Path = Path(tempdir)
        create_kosha(tempdir / "kosha")
        create_sandhi_rules(tempdir / "sandhi-rules.csv")
        create_model_files(tempdir / "model")

        with open(tempdir / "kosha" / "padas.fst", "w") as f:
            f.write("junk data")

    with pytest.raises(OSError):
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
    assert gacchati.info.pos is None


def test_run__invalid_input(chedaka):
    with pytest.raises(ValueError, match="ASCII") as _e:
        _tokens = chedaka.run("गच्छति")
"""
