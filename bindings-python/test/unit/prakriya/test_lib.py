import pytest

from vidyut.prakriya import (
    Vyakarana,
    Data,
    Krt,
    Unadi,
    Lakara,
    Linga,
    Gana,
    Dhatu,
    DhatuPada,
    Pada,
    Pratipadika,
    Prayoga,
    Purusha,
    Term,
    Sanadi,
    Source,
    Sutra,
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


def test_sutra():
    s = Sutra(source=Source.Ashtadhyayi, code="1.1.1", text="vfdDirAdEc")

    assert s.source == Source.Ashtadhyayi
    assert s.code == "1.1.1"
    assert s.text == "vfdDirAdEc"

    # Repr round-trip
    assert (
        repr(s) == "Sutra(source=Source.Ashtadhyayi, code='1.1.1', text='vfdDirAdEc')"
    )
    assert eval(repr(s)) == s


def test_prakriya():
    dhatu = d["01.0001"]
    prakriyas = v.derive(
        Pada.Tinanta(
            dhatu=dhatu,
            prayoga=Prayoga.Kartari,
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
        )
    )
    assert len(prakriyas) == 1
    p = prakriyas[0]
    assert p.text == "Bavati"

    # Filter out debugging rules, which have `rule` = "    "
    filtered_steps = [step for step in p.history if step.code.strip()]

    # o = ok, c = changed
    o = lambda x: Term(x)
    c = lambda x: Term(x, was_changed=True)

    # TODO: these c/o assignments don't make sense, but just match Rust
    # for now.
    a = Source.Ashtadhyayi
    assert filtered_steps == [
        Step(source=a, code="1.3.1", result=[c("BU")]),
        Step(source=a, code="3.2.123", result=[o("BU"), c("la~w")]),
        Step(source=a, code="1.3.2", result=[o("BU"), c("la~w")]),
        Step(source=a, code="1.3.3", result=[o("BU"), c("la~w")]),
        Step(source=a, code="1.3.9", result=[o("BU"), c("l")]),
        Step(source=a, code="1.3.78", result=[o("BU"), o("l")]),
        Step(source=a, code="3.4.78", result=[o("BU"), c("tip")]),
        Step(source=a, code="1.3.3", result=[o("BU"), c("tip")]),
        Step(source=a, code="1.3.9", result=[o("BU"), c("ti")]),
        Step(source=a, code="3.1.68", result=[o("BU"), c("Sap"), o("ti")]),
        Step(source=a, code="1.3.3", result=[o("BU"), c("Sap"), o("ti")]),
        Step(source=a, code="1.3.8", result=[o("BU"), c("Sap"), o("ti")]),
        Step(source=a, code="1.3.9", result=[o("BU"), c("a"), o("ti")]),
        Step(source=a, code="3.4.113", result=[o("BU"), c("a"), o("ti")]),
        Step(source=a, code="3.4.113", result=[o("BU"), o("a"), c("ti")]),
        Step(source=a, code="1.4.13", result=[c("BU"), c("a"), c("ti")]),
        Step(source=a, code="7.3.84", result=[c("Bo"), o("a"), o("ti")]),
        Step(source=a, code="6.1.78", result=[c("Bav"), o("a"), c("ti")]),
        Step(source=a, code="1.4.14", result=[o("Bav"), o("a"), o("ti")]),
        Step(source=a, code="8.4.68", result=[o("Bav"), o("a"), o("ti")]),
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
    prakriyas = v.derive(
        Pada.Tinanta(
            dhatu=dhatu,
            prayoga=Prayoga.Kartari,
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
        )
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


def test_derive_tinanta_with_dhatu_pada():
    dhatu = d["08.0010"]

    prakriyas = v.derive(
        Pada.Tinanta(
            dhatu=dhatu,
            prayoga=Prayoga.Kartari,
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
            dhatu_pada=DhatuPada.Parasmaipada,
        )
    )
    assert [x.text for x in prakriyas] == ["karoti"]

    prakriyas = v.derive(
        Pada.Tinanta(
            dhatu=dhatu,
            prayoga=Prayoga.Kartari,
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
            dhatu_pada=DhatuPada.Atmanepada,
        )
    )
    assert [x.text for x in prakriyas] == ["kurute"]


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BUyate"),
        ("08.0010", "kriyate"),
    ],
)
def test_derive_basic_karmani_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = v.derive(
        Pada.Tinanta(
            dhatu=dhatu,
            prayoga=Prayoga.Karmani,
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
        )
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,sanadi,expected",
    [
        ("01.0001", Sanadi.Ric, "BAvayati|BAvayate"),
        ("08.0010", Sanadi.Ric, "kArayati|kArayate"),
        ("01.0001", Sanadi.san, "buBUzati"),
        ("08.0010", Sanadi.san, "cikIrzati|cikIrzate"),
        ("01.0001", Sanadi.yaN, "boBUyate"),
        ("08.0010", Sanadi.yaN, "cekrIyate"),
    ],
)
def test_derive_sanadyanta_tinantas(code, sanadi, expected):
    dhatu = d[code]
    dhatu_nic = dhatu.with_sanadi([sanadi])
    prakriyas = v.derive(
        Pada.Tinanta(
            dhatu=dhatu_nic,
            prayoga=Prayoga.Kartari,
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
        )
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


def test_derive_tinantas_without_at_agama():
    bhu = Dhatu.mula("BU", Gana.Bhvadi)
    prakriyas = v.derive(
        Pada.Tinanta(
            dhatu=bhu,
            prayoga=Prayoga.Kartari,
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lun,
            skip_at_agama=True,
        )
    )
    assert prakriyas[0].text == "BUt"


def test_derive_subantas():
    prakriyas = v.derive(
        Pada.Subanta(
            pratipadika=Pratipadika.basic("deva"),
            linga=Linga.Pum,
            vibhakti=Vibhakti.Prathama,
            vacana=Vacana.Eka,
        )
    )
    expected = {"devaH"}
    actual = {x.text for x in prakriyas}
    assert expected == actual


def test_derive_subantas_with_nyap():
    v = Vyakarana()
    prakriyas = v.derive(
        Pada.Subanta(
            Pratipadika.nyap("nadI"), Linga.Stri, Vibhakti.Prathama, Vacana.Eka
        )
    )
    expected = {"nadI"}
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


@pytest.mark.parametrize(
    "code,krt,prayoga,expected",
    [
        ("01.1137", Krt.SAnac, Prayoga.Karmani, "gamyamAna"),
        ("08.0010", Krt.SAnac, Prayoga.Karmani, "kriyamARa"),
    ],
)
def test_derive_pratipadikas_with_krdanta_prayoga(code, krt, prayoga, expected):
    dhatu = d[code]
    anga = Pratipadika.krdanta(dhatu, krt, prayoga)
    prakriyas = v.derive(anga)

    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,krt,lakara,expected",
    [
        ("01.0001", Krt.Satf, Lakara.Lrt, "Bavizyat"),
        ("01.1137", Krt.Satf, Lakara.Lrt, "gamizyat"),
    ],
)
def test_derive_pratipadikas_with_krdanta_lakara(code, krt, lakara, expected):
    dhatu = d[code]
    anga = Pratipadika.krdanta(dhatu, krt, lakara=lakara)
    prakriyas = v.derive(anga)

    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,krt,prayoga,lakara,expected",
    [
        ("01.1137", Krt.Satf, Prayoga.Kartari, Lakara.Lat, "gacCat"),
        ("01.1137", Krt.Satf, Prayoga.Kartari, Lakara.Lrt, "gamizyat"),
        ("01.1137", Krt.SAnac, Prayoga.Karmani, Lakara.Lat, "gamyamAna"),
    ],
)
def test_derive_pratipadikas_with_krdanta_prayoga_lakara(
    code, krt, prayoga, lakara, expected
):
    dhatu = d[code]
    anga = Pratipadika.krdanta(dhatu, krt, prayoga, lakara)
    prakriyas = v.derive(anga)

    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


def test_derive_pratipadikas_with_krdanta_unadi():
    dhatu = Dhatu.mula("bfhi~", Gana.Bhvadi)
    anga = Pratipadika.krdanta(dhatu, Unadi.manin)
    prakriyas = v.derive(anga)

    actual = {x.text for x in prakriyas}
    assert {"brahman"} == actual


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
