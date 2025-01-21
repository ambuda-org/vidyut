import pytest

from vidyut.prakriya import (
    Dhatu,
    DhatuPada,
    Pratipadika,
    Antargana,
    Gana,
    Krt,
    Lakara,
    Linga,
    Pada,
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
    DhatuPada,
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
    with pytest.raises(TypeError):
        _ = Dhatu.mula(aupadeshika="BU")

    with pytest.raises(TypeError):
        _ = Dhatu.mula(gana=Gana.Bhvadi)


def test_dhatu__mula_must_have_slp1_aupadeshika():
    with pytest.raises(ValueError):
        _ = Dhatu.mula(aupadeshika="한글", gana=Gana.Bhvadi)


def test_dhatu__nama():
    _ = Dhatu.nama(Pratipadika.basic("putra"), nama_sanadi=Sanadi.kAmyac)

    # Missing arg
    _ = Dhatu.nama(Pratipadika.basic("putra"))

    # By keyword
    _ = Dhatu.nama(pratipadika=Pratipadika.basic("putra"))


def test_dhatu__nama_must_have_pratipadika():
    with pytest.raises(TypeError):
        _ = Dhatu.nama(nama_sanadi=Sanadi.kAmyac)


def test_dhatu__dunders():
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    gam = Dhatu.mula(aupadeshika="ga\\mx~", gana=Gana.Bhvadi)
    kut = Dhatu.mula(aupadeshika="kuwa~", gana=Gana.Tudadi, antargana=Antargana.Kutadi)
    abhisambibhavayisha = bhu.with_prefixes(["aBi", "sam"]).with_sanadi(
        [Sanadi.Ric, Sanadi.san]
    )

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
    assert repr(abhisambibhavayisha) == (
        "Dhatu(aupadeshika='BU', gana=Gana.Bhvadi, "
        "prefixes=['aBi', 'sam'], "
        "sanadi=[Sanadi.Ric, Sanadi.san])"
    )


def test_pratipadika_new():
    p = Pratipadika.basic("deva")
    assert repr(p) == "Pratipadika(text='deva')"


def test_pratipadika_new__fails_if_no_args():
    with pytest.raises(TypeError):
        _p = Pratipadika()


def test_pratipadika__must_have_slp1_aupadeshika():
    with pytest.raises(ValueError):
        _ = Pratipadika.basic("한글")


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


def test_subanta():
    rama = Pratipadika.basic("rAma")
    s = Pada.Subanta(rama, Linga.Pum, Vibhakti.Prathama, Vacana.Eka)
    assert s.pratipadika == rama
    assert s.linga == Linga.Pum
    assert s.vibhakti == Vibhakti.Prathama
    assert s.vacana == Vacana.Eka


def test_tinanta():
    bhu = Dhatu.mula("BU", Gana.Bhvadi)
    s = Pada.Tinanta(
        bhu,
        Prayoga.Kartari,
        Lakara.Lat,
        Purusha.Prathama,
        Vacana.Eka,
    )
    assert s.dhatu == bhu
    assert s.prayoga == Prayoga.Kartari
    assert s.lakara == Lakara.Lat
    assert s.purusha == Purusha.Prathama
    assert s.vacana == Vacana.Eka


def test_enums_use_slp1_for_str():
    assert str(Gana.Bhvadi) == "BvAdi"
    assert str(Antargana.Adhrshiya) == "ADfzIya"
    assert str(Krt.kvasu) == "kvasu~"


@pytest.mark.parametrize("enum", ENUMS)
def test_enum_can_be_stringified_round_trip(enum):
    for val in enum.choices():
        assert enum(str(val)) == val


@pytest.mark.parametrize("enum", ENUMS)
def test_enum_isinstance(enum):
    for val in enum.choices():
        assert isinstance(val, enum)


@pytest.mark.parametrize("enum", ENUMS)
def test_enum_raises_error_on_unknown_value(enum):
    with pytest.raises(ValueError):
        enum("unsupported value")


@pytest.mark.parametrize("enum", ENUMS)
def test_enum_dunders(enum):
    # __bool__
    for val in enum.choices():
        assert val

    # __eq__
    for val in enum.choices():
        assert val is val
        assert val == val
        # "Comparisons against non-enumeration values will always compare not equal"
        assert val != str(val)

    # __format__
    # TODO: not rendered correctly.
    # for val in enum.choices():
    #     _ = f"{val:>10}"

    # __getitem__
    # TODO: __getitem__ defined, but this still fails on "not subscriptable"
    # for val in enum.choices():
    #     assert enum[val.name] == val
    # with pytest.raises(KeyError):
    #     _ = enum["unknown enum value"]

    # __hash__
    _ = {val: "foo" for val in enum.choices()}

    # __lt__, __gt__
    _ = sorted(enum.choices())


@pytest.mark.parametrize("enum", ENUMS)
def test_enum_variants_are_in_sorted_order(enum):
    assert enum.choices()
    assert sorted(enum.choices()) == enum.choices()


@pytest.mark.parametrize("enum", ENUMS)
def test_enum_defines_attr_name(enum):
    for val in enum.choices():
        assert getattr(enum, val.name) == val
        assert val.name == val._name_


@pytest.mark.parametrize("enum", ENUMS)
def test_enum_defines_attr_value_as_slp1_string(enum):
    for val in enum.choices():
        assert val.value == str(val)
        assert val.value == val._value_


@pytest.mark.parametrize("enum", ENUMS)
def test_enum_attr_value_is_unique(enum):
    values = {x.value for x in enum.choices()}
    assert len(values) == len(enum.choices())


@pytest.mark.parametrize("enum", ENUMS)
def test_enum_repr_evals_to_variant(enum):
    for val in enum.choices():
        assert eval(repr(val)) == val
