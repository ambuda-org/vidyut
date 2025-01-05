import pytest

from vidyut.prakriya import (
    Dhatu,
    Pratipadika,
    Antargana,
    Gana,
    Krt,
    Lakara,
    Linga,
    Prayoga,
    Purusha,
    Sanadi,
    Source,
    Taddhita,
    Vacana,
    Vibhakti,
)


ENUMS = [
    Antargana,
    Gana,
    Krt,
    Lakara,
    Linga,
    Prayoga,
    Purusha,
    Sanadi,
    Taddhita,
    Vacana,
    Vibhakti,
    # Also include data enums
    Source,
]


@pytest.mark.parametrize(
    "kwargs",
    [
        {"aupadeshika": "BU", "gana": Gana.Bhvadi},
        {"aupadeshika": "BU", "gana": Gana.Bhvadi, "antargana": Antargana.Akusmiya},
        {"aupadeshika": "BU", "gana": Gana.Bhvadi, "prefixes": ["aBi"]},
        {"aupadeshika": "BU", "gana": Gana.Bhvadi, "sanadi": [Sanadi.san]},
        {
            "aupadeshika": "BU",
            "gana": Gana.Bhvadi,
            "prefixes": ["aBi"],
            "sanadi": [Sanadi.san],
        },
    ],
)
def test_dhatu__mula(kwargs):
    d = Dhatu.mula(**kwargs)

    assert d.aupadeshika == kwargs["aupadeshika"]
    assert d.gana == kwargs["gana"]
    assert d.antargana == kwargs.get("antargana")
    assert d.prefixes == kwargs.get("prefixes", [])
    assert d.sanadi == kwargs.get("sanadi", [])


def test_dhatu__mula_with_positional_args():
    d1 = Dhatu.mula("BU", Gana.Bhvadi)
    d2 = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    assert d1 == d2


def test_dhatu__mula_must_have_aupadeshika_and_gana():
    try:
        d = Dhatu.mula(aupadeshika="BU")
        assert False, "Must have gana"
    except TypeError:
        pass

    try:
        d = Dhatu.mula(gana=Gana.Bhvadi)
        assert False, "Must have aupadeshika"
    except TypeError:
        pass


def test_dhatu__nama():
    _ = Dhatu.nama(Pratipadika.basic("putra"), nama_sanadi=Sanadi.kAmyac)

    # Missing arg
    _ = Dhatu.nama(Pratipadika.basic("putra"))

    # By keyword
    _ = Dhatu.nama(pratipadika=Pratipadika.basic("putra"))


def test_dhatu__nama_must_have_pratipadika():
    try:
        d = Dhatu.nama(nama_sanadi=Sanadi.kAmyac)
        assert False, "Must have pratipadika"
    except TypeError:
        pass


def test_dhatu__dunders():
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    gam = Dhatu.mula(aupadeshika="ga\\mx~", gana=Gana.Bhvadi)
    kut = Dhatu.mula(aupadeshika="kuwa~", gana=Gana.Tudadi, antargana=Antargana.Kutadi)
    abhibubhusha = bhu.with_prefixes(["aBi"]).with_sanadi([Sanadi.san])

    # __eq__, __ne__
    bhu2 = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    assert bhu == bhu
    assert bhu == bhu2
    assert bhu != gam

    # __lt__, __gt__
    _ = sorted([bhu, gam])

    # __repr__
    assert repr(bhu) == "Dhatu(aupadeshika='BU', gana=Gana.Bhvadi)"
    assert (
        repr(kut)
        == "Dhatu(aupadeshika='kuwa~', gana=Gana.Tudadi, antargana=Antargana.Kutadi)"
    )
    assert (
        repr(abhibubhusha)
        == "Dhatu(aupadeshika='BU', gana=Gana.Bhvadi, prefixes=['aBi'], sanadi=[Sanadi.san])"
    )


def test_pratipadika_new():
    p = Pratipadika.basic("deva")
    assert repr(p) == "Pratipadika(text='deva')"


def test_pratipadika_new__fails_if_no_args():
    with pytest.raises(TypeError):
        _p = Pratipadika()


def test_pratipadika__dunders():
    deva = Pratipadika.basic("deva")
    eva = Pratipadika.basic("eva")

    # __eq__, __ne__
    deva2 = Pratipadika.basic("deva")
    assert deva == deva
    assert deva == deva2
    assert deva != eva

    # __lt__, __gt__
    _ = sorted([deva, eva])

    # __repr__
    assert repr(deva) == "Pratipadika(text='deva')"


@pytest.mark.parametrize("enum", ENUMS)
def test_enums_can_be_stringified(enum):
    for val in enum.choices():
        assert enum.from_string(str(val)) == val


@pytest.mark.parametrize("enum", ENUMS)
def test_enums_can_be_ordered(enum):
    _ = sorted(enum.choices())


@pytest.mark.parametrize("enum", ENUMS)
def test_enums_are_in_sorted_order(enum):
    assert enum.choices()
    assert sorted(enum.choices()) == enum.choices()


@pytest.mark.parametrize("enum", ENUMS)
def test_enums_define_attr_name(enum):
    for val in enum.choices():
        assert getattr(enum, val.name) == val


@pytest.mark.parametrize("enum", ENUMS)
def test_enums_repr(enum):
    for val in enum.choices():
        assert eval(repr(val)) == val


@pytest.mark.parametrize("enum", ENUMS)
def test_enums_hashable(enum):
    _ = {val: "foo" for val in enum.choices()}
