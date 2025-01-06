import pytest

from vidyut.lipi import transliterate, detect, Scheme


def test_scheme_can_be_stringified():
    for val in Scheme.choices():
        assert Scheme.from_string(str(val)) == val


def test_scheme_iso_15924_code():
    for val in Scheme.choices():
        _ = val.iso_15924_code


def test_scheme_iso_15924_numeric_code():
    for val in Scheme.choices():
        _ = val.iso_15924_numeric_code


def test_scheme_icu_numeric_code():
    for val in Scheme.choices():
        _ = val.icu_numeric_code


def test_scheme_can_be_ordered():
    _ = sorted(Scheme.choices())


def test_scheme_is_in_sorted_order():
    assert Scheme.choices()
    assert sorted(Scheme.choices()) == Scheme.choices()


def test_scheme_defines_attr_name():
    for val in Scheme.choices():
        assert getattr(Scheme, val.name) == val


def test_scheme_repr():
    for val in Scheme.choices():
        assert eval(repr(val)) == val


def test_scheme_is_hashable():
    _ = {val: "foo" for val in Scheme.choices()}


@pytest.mark.parametrize(
    "input,source,dest,expected",
    [
        ("saMskRtam", Scheme.HarvardKyoto, Scheme.Devanagari, "संस्कृतम्"),
    ],
)
def test_transliterate(input, source, dest, expected):
    assert transliterate(input, source, dest) == expected


@pytest.mark.parametrize(
    "text,expected",
    [
        ("", Scheme.HarvardKyoto),
        ("संस्कृतम्", Scheme.Devanagari),
    ],
)
def test_detect(text, expected):
    assert detect(text) == expected
