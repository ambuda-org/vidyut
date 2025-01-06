import os
from pathlib import Path

import pytest

from vidyut import chandas
from vidyut.chandas import Chandas


@pytest.fixture(scope="module")
def chandas() -> Chandas:
    path = os.environ["VIDYUT_DATA_DIR"]
    chandas_path = Path(path).absolute() / "chandas" / "meters.tsv"
    assert chandas_path.exists()
    return Chandas(chandas_path)


def test_vasantatilaka(chandas):
    result = chandas.classify("mAtaH samastajagatAM maDukEwaBAreH")
    assert result.padya == "vasantatilakA"

    assert len(result.aksharas) == 1
    row = result.aksharas[0]
    assert (
        "-".join(a.text for a in row) == "mA-taH-sa-ma-sta-ja-ga-tAM-ma-Du-kE-wa-BA-reH"
    )
    assert "-".join(a.weight for a in row) == "G-G-L-G-L-L-L-G-L-L-G-L-G-G"


@pytest.mark.skip(reason="upstream code is buggy")
def test_nonsense(chandas):
    result = chandas.classify("120312031289318230")
    assert result.vrtta is None

    assert len(result.aksharas) == 0
