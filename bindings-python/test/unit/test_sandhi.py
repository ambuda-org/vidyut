import tempfile
from pathlib import Path

import pytest

from vidyut.sandhi import Splitter


def create_sandhi_rules(output_path):
    with open(output_path, "w") as f:
        f.write("first,second,result\n")
        f.write("i,a,y a\n")
        f.write("I,a,y a\n")
        f.write("as,g,o g\n")


@pytest.fixture(scope="module")
def splitter() -> Splitter:
    with tempfile.TemporaryDirectory() as tempdir:
        path = Path(tempdir) / "sandhi-rules.csv"
        create_sandhi_rules(path)
        return Splitter.from_csv(path)


def test_init(splitter):
    assert True


def test_split_at(splitter):
    splits = splitter.split_at("ityapi", 2)
    assert len(splits) == 3

    s = splits[0]
    assert s.first == "ity"
    assert s.second == "api"
    assert not s.is_valid

    s = splits[1]
    assert s.first == "iti"
    assert s.second == "api"
    assert s.is_valid

    s = splits[2]
    assert s.first == "itI"
    assert s.second == "api"
    assert s.is_valid


def test_split__dunders(splitter):
    splits = splitter.split_at("ityapi", 2)
    assert len(splits) == 3

    s1 = splits[0]
    s2 = splits[1]

    assert s1 == s1
    assert s1 != s2
    _ = sorted(splits)
    _ = {s: "foo" for s in splits}
