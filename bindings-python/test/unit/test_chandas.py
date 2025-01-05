import tempfile
from pathlib import Path

from vidyut import chandas
from vidyut.chandas import Chandas, Akshara, Vrtta, Jati, VrttaPada


def test_init():
    with tempfile.NamedTemporaryFile() as f:
        f.write(b"mandAkrAntA	vrtta	GGGG|LLLLLG|GLGGLGG\n")
        f.seek(0)
        _ = Chandas(f.name)


def test_init__invalid_file():
    with tempfile.NamedTemporaryFile() as f:
        f.write(b"junk data")
        f.seek(0)

        try:
            _ = Chandas(f.name)
            assert False
        except IOError:
            pass


def test_init__missing_file():
    try:
        _ = Chandas("/tmp/path/to/invalid/file.tsv")
        assert False
    except OSError:
        pass


def test_chandas__attrs():
    c = Chandas.from_text("mandAkrAntA	vrtta	GGGG|LLLLLG|GLGGLGG\n")
    assert c.vrttas == [
        Vrtta("mandAkrAntA", [VrttaPada.from_string("GGGG|LLLLLG|GLGGLGG")])
    ]
    assert c.jatis == [
        Jati("vEtAlIyam", [14, 16, 14, 16]),
        Jati("upagIti", [12, 15, 12, 15]),
        Jati("AryAgIti", [12, 20, 12, 20]),
        Jati("gIti", [12, 18, 12, 18]),
        Jati("udgIti", [12, 15, 12, 18]),
        Jati("OpacCandasikam", [16, 18, 16, 18]),
        Jati("AryA", [12, 18, 12, 15]),
    ]


def test_classify_vrtta():
    with tempfile.NamedTemporaryFile() as f:
        f.write(b"mandAkrAntA	vrtta	GGGG|LLLLLG|GLGGLGG\n")
        f.seek(0)
        c = Chandas(f.name)

    result = c.classify("kaScitkAntAvirahaguruRA svADikArapramattaH")
    assert result.aksharas == [
        [
            Akshara(text="ka", weight="G"),
            Akshara(text="Sci", weight="G"),
            Akshara(text="tkA", weight="G"),
            Akshara(text="ntA", weight="G"),
            Akshara(text="vi", weight="L"),
            Akshara(text="ra", weight="L"),
            Akshara(text="ha", weight="L"),
            Akshara(text="gu", weight="L"),
            Akshara(text="ru", weight="L"),
            Akshara(text="RA", weight="G"),
            Akshara(text="svA", weight="G"),
            Akshara(text="Di", weight="L"),
            Akshara(text="kA", weight="G"),
            Akshara(text="ra", weight="G"),
            Akshara(text="pra", weight="L"),
            Akshara(text="ma", weight="G"),
            Akshara(text="ttaH", weight="G"),
        ]
    ]
    assert result.padya == "mandAkrAntA"


def test_classify_jati():
    with tempfile.NamedTemporaryFile() as f:
        f.write(b"mandAkrAntA	vrtta	GGGG|LLLLLG|GLGGLGG\n")
        f.seek(0)
        c = Chandas(f.name)

    text = (
        "yenAmandamarande daladaravinde dinAnyanAyizata |"
        "kuwaje Kalu tenehA tenehA maDukareRa kaTam ||"
    )

    result = c.classify(text)
    assert result.padya == "AryA"
