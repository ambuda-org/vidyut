import pytest

from vidyut.prakriya import (
    Vyakarana,
    Data,
    Krt,
    Lakara,
    Linga,
    Gana,
    Dhatu,
    Pada,
    Pratipadika,
    Prayoga,
    Purusha,
    Sanadi,
    Source,
    Step,
    Taddhita,
    Vibhakti,
    Vacana,
)


v = Vyakarana()
# Path is relative to the project root.
d = {e.code: e.dhatu for e in Data("test/data").load_dhatu_entries()}


def test_vyakarana():
    v = Vyakarana()
    assert repr(v) == "Vyakarana()"


def test_dhatupatha():
    bhu = d["01.0001"]
    assert bhu.aupadeshika == "BU"

    kr = d["08.0010"]
    assert kr.aupadeshika == "qukf\\Y"


def test_prakriya():
    dhatu = d["01.0001"]
    prakriyas = v.derive(Pada.Tinanta(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    ))
    assert len(prakriyas) == 1
    p = prakriyas[0]
    assert p.text == "Bavati"

    # Filter out debugging rules, which have `rule` = "    "
    filtered_steps = [step for step in p.history if step.code.strip()]

    a = Source.Ashtadhyayi
    assert filtered_steps == [
        Step(source=a, code="1.3.1", result=["BU"]),
        Step(source=a, code="3.2.123", result=["BU", "la~w"]),
        Step(source=a, code="1.3.2", result=["BU", "la~w"]),
        Step(source=a, code="1.3.3", result=["BU", "la~w"]),
        Step(source=a, code="1.3.9", result=["BU", "l"]),
        Step(source=a, code="1.3.78", result=["BU", "l"]),
        Step(source=a, code="3.4.78", result=["BU", "tip"]),
        Step(source=a, code="1.3.3", result=["BU", "tip"]),
        Step(source=a, code="1.3.9", result=["BU", "ti"]),
        Step(source=a, code="3.4.113", result=["BU", "ti"]),
        Step(source=a, code="3.1.68", result=["BU", "Sap", "ti"]),
        Step(source=a, code="1.3.3", result=["BU", "Sap", "ti"]),
        Step(source=a, code="1.3.8", result=["BU", "Sap", "ti"]),
        Step(source=a, code="1.3.9", result=["BU", "a", "ti"]),
        Step(source=a, code="3.4.113", result=["BU", "a", "ti"]),
        Step(source=a, code="1.4.13", result=["BU", "a", "ti"]),
        Step(source=a, code="7.3.84", result=["Bo", "a", "ti"]),
        Step(source=a, code="1.4.14", result=["Bo", "a", "ti"]),
        Step(source=a, code="6.1.78", result=["Bav", "a", "ti"]),
        Step(source=a, code="8.4.68", result=["Bav", "a", "ti"]),
    ]


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "Bavati"),
        ("08.0010", "karoti|kurute"),
    ],
)
def test_derive_basic_kartari_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = v.derive(Pada.Tinanta(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    ))
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BUyate"),
        ("08.0010", "kriyate"),
    ],
)
def test_derive_basic_karmani_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = v.derive(Pada.Tinanta(
        dhatu=dhatu,
        prayoga=Prayoga.Karmani,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    ))
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "buBUzati"),
        # ("08.0010", "cikIrzati"),
    ],
)
def test_derive_sannanta_tinantas(code, expected):
    dhatu = d[code]
    dhatu_san = dhatu.with_sanadi([Sanadi.san])
    prakriyas = v.derive(Pada.Tinanta(
        dhatu=dhatu_san,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    ))
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BAvayati|BAvayate"),
        ("08.0010", "kArayati|kArayate"),
    ],
)
def test_derive_nijanta_tinantas(code, expected):
    dhatu = d[code]
    dhatu_nic = dhatu.with_sanadi([Sanadi.Ric])
    prakriyas = v.derive(Pada.Tinanta(
        dhatu=dhatu_nic,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    ))
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "boBUyate"),
        # ("08.0010", "cekrIyate"),
    ],
)
def test_derive_yananta_tinantas(code, expected):
    dhatu = d[code]
    dhatu_yan = dhatu.with_sanadi([Sanadi.yaN])
    prakriyas = v.derive(Pada.Tinanta(
        dhatu=dhatu_yan,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    ))
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


def test_derive_tinantas_without_at_agama():
    bhu = Dhatu.mula("BU", Gana.Bhvadi)
    prakriyas = v.derive(Pada.Tinanta(
        dhatu=bhu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lun,
        skip_at_agama=True,
    ))
    assert prakriyas[0].text == "BUt"


def test_derive_subantas():
    prakriyas = v.derive(Pada.Subanta(
        pratipadika=Pratipadika.basic("deva"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.Prathama,
        vacana=Vacana.Eka,
    ))
    expected = {"devaH"}
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BUtvA"),
        ("08.0010", "kftvA"),
    ],
)
def test_derive_pratipadikas_with_krdanta(code, expected):
    dhatu = d[code]
    anga = Pratipadika.krdanta(dhatu, Krt.ktvA)
    prakriyas = v.derive(anga)

    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


def test_derive_pratipadikas_with_taddhitanta():
    guru = Pratipadika.basic("guru")
    anga = Pratipadika.taddhitanta(guru, Taddhita.aR)
    prakriyas = v.derive(anga)
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "gOrava"


def test_derive_dhatus():
    upa_r = Dhatu.mula(aupadeshika="f\\", gana=Gana.Bhvadi).with_prefixes(["upa"])
    prakriyas = v.derive(upa_r)
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "upAr"
