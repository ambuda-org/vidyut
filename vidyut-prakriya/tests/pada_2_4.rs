extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::*;

fn nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic])
}

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

#[ignore]
#[test]
fn sutra_2_4_35() {
    let han = d("ha\\na~", Adadi);
    assert_has_ashirlin(&[], &han, &["vaDyAt"]);
    assert_has_vidhilin(&[], &han, &["hanyAt"]);
    // TODO: wait until base substitution before adding krt
    assert_has_krdanta(&[], &d("asa~", Adadi), Krt::yat, &["Bavya"]);
    assert_has_krdanta(&["pra"], &d("aja~", Bhvadi), Krt::yat, &["praveya"]);
    assert_has_krdanta(&["AN"], &d("ca\\kzi~\\N", Adadi), Krt::yat, &["AKyeya"]);
}

#[test]
fn sutra_2_4_36() {
    let ad = d("a\\da~", Adadi);
    assert_has_krdanta(&["pra"], &ad, Krt::ktvA, &["prajagDya"]);
    assert_has_krdanta(&["vi"], &ad, Krt::ktvA, &["vijagDya"]);
    assert_has_krdanta(&[], &ad, Krt::kta, &["jagDa", "jagdDa"]);
    assert_has_krdanta(&[], &ad, Krt::ktavatu, &["jagDavat", "jagdDavat"]);

    assert_has_lat_karmani(&[], &ad, &["adyate"]);
    assert_has_krdanta(&[], &ad, Krt::tavya, &["attavya"]);

    // TODO: annam
}

#[test]
fn sutra_2_4_37() {
    let ad = d("a\\da~", Adadi);
    assert_has_parasmai_tinanta(&[], &ad, Lun, Prathama, Eka, &["aGasat"]);
    assert_has_parasmai_tinanta(&[], &ad, Lun, Prathama, Dvi, &["aGasatAm"]);
    assert_has_parasmai_tinanta(&[], &ad, Lun, Prathama, Bahu, &["aGasan"]);
    assert_has_parasmai_tinanta(&[], &san(&ad), Lat, Prathama, Eka, &["jiGatsati"]);
    assert_has_parasmai_tinanta(&[], &san(&ad), Lat, Prathama, Dvi, &["jiGatsataH"]);
    assert_has_parasmai_tinanta(&[], &san(&ad), Lat, Prathama, Bahu, &["jiGatsanti"]);
}

#[ignore]
#[test]
fn sutra_2_4_37_v1() {
    let ad = d("a\\da~", Adadi);
    assert_has_krdanta(&["pra"], &ad, Krt::ac, &["praGasa"]);
}

#[ignore]
#[test]
fn sutra_2_4_38() {
    let ad = d("a\\da~", Adadi);
    assert_has_krdanta(&[], &ad, Krt::GaY, &["GAsa"]);
    assert_has_krdanta(&["pra"], &ad, Krt::ap, &["praGasa"]);
}

// 2.4.39 is chAndasa.

#[test]
fn sutra_2_4_40() {
    let ad = d("a\\da~", Adadi);
    assert_has_parasmai_tinanta(&[], &ad, Lit, Prathama, Eka, &["jaGAsa", "Ada"]);
    assert_has_parasmai_tinanta(&[], &ad, Lit, Prathama, Dvi, &["jakzatuH", "AdatuH"]);
    assert_has_parasmai_tinanta(&[], &ad, Lit, Prathama, Bahu, &["jakzuH", "AduH"]);
}

#[test]
fn sutra_2_4_41() {
    let ve = d("ve\\Y", Bhvadi);
    assert_has_parasmai_tinanta(&[], &ve, Lit, Prathama, Eka, &["uvAya", "vavO"]);
    assert_has_parasmai_tinanta(
        &[],
        &ve,
        Lit,
        Prathama,
        Dvi,
        &["UyatuH", "UvatuH", "vavatuH"],
    );
    assert_has_parasmai_tinanta(&[], &ve, Lit, Prathama, Bahu, &["UyuH", "UvuH", "vavuH"]);
}

#[test]
fn sutra_2_4_42() {
    let han = d("ha\\na~", Adadi);
    assert_has_parasmai_tinanta(&[], &han, AshirLin, Prathama, Eka, &["vaDyAt"]);
    assert_has_parasmai_tinanta(&[], &han, AshirLin, Prathama, Dvi, &["vaDyAstAm"]);
    assert_has_parasmai_tinanta(&[], &han, AshirLin, Prathama, Bahu, &["vaDyAsuH"]);
}

#[test]
fn sutra_2_4_43() {
    let han = d("ha\\na~", Adadi);
    assert_has_parasmai_tinanta(&[], &han, Lun, Prathama, Eka, &["avaDIt"]);
    assert_has_parasmai_tinanta(&[], &han, Lun, Prathama, Dvi, &["avaDizwAm"]);
    assert_has_parasmai_tinanta(&[], &han, Lun, Prathama, Bahu, &["avaDizuH"]);
}

#[test]
fn sutra_2_4_44() {
    let han = d("ha\\na~", Adadi);
    assert_has_atmane_tinanta(&["AN"], &han, Lun, Prathama, Eka, &["AvaDizwa", "Ahata"]);
    assert_has_atmane_tinanta(
        &["AN"],
        &han,
        Lun,
        Prathama,
        Dvi,
        &["AvaDizAtAm", "AhasAtAm"],
    );
    assert_has_atmane_tinanta(
        &["AN"],
        &han,
        Lun,
        Prathama,
        Bahu,
        &["AvaDizata", "Ahasata"],
    );
}

#[test]
fn sutra_2_4_45() {
    let i = d("i\\R", Adadi);
    assert_has_parasmai_tinanta(&[], &i, Lun, Prathama, Eka, &["agAt"]);
    assert_has_parasmai_tinanta(&[], &i, Lun, Prathama, Dvi, &["agAtAm"]);
    assert_has_parasmai_tinanta(&[], &i, Lun, Prathama, Bahu, &["aguH"]);
    // karmani
    assert_has_lun_karmani(&[], &i, &["agAyi"]);
}

#[test]
fn sutra_2_4_45_v1() {
    let ik = d("i\\k", Adadi);
    assert_has_parasmai_tinanta(&["aDi"], &ik, Lun, Prathama, Eka, &["aDyagAt"]);
    assert_has_parasmai_tinanta(&["aDi"], &ik, Lun, Prathama, Dvi, &["aDyagAtAm"]);
    assert_has_parasmai_tinanta(&["aDi"], &ik, Lun, Prathama, Bahu, &["aDyaguH"]);
}

#[ignore]
#[test]
fn sutra_2_4_46() {
    let i = d("i\\R", Adadi);
    assert_has_tip(&[], &nic(&i), Lat, &["gamayati", "Ayayati"]);
    assert_has_tas(&[], &nic(&i), Lat, &["gamayataH", "AyayataH"]);
    assert_has_jhi(&[], &nic(&i), Lat, &["gamayanti", "Ayayanti"]);

    // TODO: iRvad
}

#[ignore]
#[test]
fn sutra_2_4_47() {
    let i_san = san(&d("i\\R", Adadi));
    assert_has_tip(&[], &i_san, Lat, &["jigamizati", "Izizati"]);
    assert_has_tip(&[], &i_san, Lat, &["jigamizataH", "IzizataH"]);
    assert_has_tip(&[], &i_san, Lat, &["jigamizanti", "Izizanti"]);
    // TODO: others
}

#[test]
fn sutra_2_4_48() {
    let i_san = san(&d("i\\N", Adadi));
    assert_has_tinanta(&["aDi"], &i_san, Lat, Prathama, Eka, &["aDijigAMsate"]);
    assert_has_tinanta(&["aDi"], &i_san, Lat, Prathama, Dvi, &["aDijigAMsete"]);
    assert_has_tinanta(&["aDi"], &i_san, Lat, Prathama, Bahu, &["aDijigAMsante"]);
}

#[test]
fn sutra_2_4_49() {
    let i = d("i\\N", Adadi);
    assert_has_tinanta(&["aDi"], &i, Lit, Prathama, Eka, &["aDijage"]);
    assert_has_tinanta(&["aDi"], &i, Lit, Prathama, Dvi, &["aDijagAte"]);
    assert_has_tinanta(&["aDi"], &i, Lit, Prathama, Bahu, &["aDijagire"]);
}

#[test]
fn sutra_2_4_50() {
    let i = d("i\\N", Adadi);
    assert_has_tinanta(&["aDi"], &i, Lun, Prathama, Eka, &["aDyagIzwa", "aDyEzwa"]);
    assert_has_tinanta(
        &["aDi"],
        &i,
        Lun,
        Prathama,
        Dvi,
        &["aDyagIzAtAm", "aDyEzAtAm"],
    );
    assert_has_tinanta(
        &["aDi"],
        &i,
        Lun,
        Prathama,
        Bahu,
        &["aDyagIzata", "aDyEzata"],
    );
    assert_has_tinanta(
        &["aDi"],
        &i,
        Lrn,
        Prathama,
        Eka,
        &["aDyagIzyata", "aDyEzyata"],
    );
    assert_has_tinanta(
        &["aDi"],
        &i,
        Lrn,
        Prathama,
        Dvi,
        &["aDyagIzyetAm", "aDyEzyetAm"],
    );
    assert_has_tinanta(
        &["aDi"],
        &i,
        Lrn,
        Prathama,
        Bahu,
        &["aDyagIzyanta", "aDyEzyanta"],
    );
}

#[ignore]
#[test]
fn sutra_2_4_51() {
    let i = d("i\\N", Adadi);
    let i_nic_san = i.clone().with_sanadi(&[Sanadi::Nic, Sanadi::San]);
    assert_has_lat(&["aDi"], &i_nic_san, &["aDijigApayizati", "aDyApipayizati"]);
    assert_has_lun(&["aDi"], &nic(&i), &["aDyajIgapat", "aDyApipat"]);
}

#[test]
fn sutra_2_4_52() {
    let asa = d("asa~", Adadi);
    assert_has_krdanta(&[], &asa, Krt::tfc, &["Bavitf"]);
    assert_has_krdanta(&[], &asa, Krt::tumun, &["Bavitum"]);
    assert_has_krdanta(&[], &asa, Krt::tavya, &["Bavitavya"]);

    // TODO: others;
}

#[test]
fn sutra_2_4_53() {
    let bru = d("brUY", Adadi);
    assert_has_krdanta(&[], &bru, Krt::tfc, &["vaktf"]);
    assert_has_krdanta(&[], &bru, Krt::tumun, &["vaktum"]);
    assert_has_krdanta(&[], &bru, Krt::tavya, &["vaktavya"]);
}

#[ignore]
#[test]
fn sutra_2_4_54() {
    let cakz = d("ca\\kzi~\\N", Adadi);
    assert_has_krdanta(&["AN"], &cakz, Krt::tfc, &["AKyAtf"]);
    assert_has_krdanta(&["AN"], &cakz, Krt::tumun, &["AKyAtum"]);
    assert_has_krdanta(&["AN"], &cakz, Krt::tavya, &["AKyAtavya"]);

    // sTAnivadBAva
    assert_has_lrt(&["AN"], &cakz, &["AKyAsyati", "AKyAsyate"]);
}

#[test]
fn sutra_2_4_55() {
    let cakz = d("ca\\kzi~\\N", Adadi);
    assert_has_tinanta(
        &["AN"],
        &cakz,
        Lit,
        Prathama,
        Eka,
        &["AcaKyO", "AcaKye", "Acacakze"],
    );
    assert_has_tinanta(
        &["AN"],
        &cakz,
        Lit,
        Prathama,
        Dvi,
        &["AcaKyatuH", "AcaKyAte", "AcacakzAte"],
    );
    assert_has_tinanta(
        &["AN"],
        &cakz,
        Lit,
        Prathama,
        Bahu,
        &["AcaKyuH", "AcaKyire", "Acacakzire"],
    );
}

#[test]
fn sutra_2_4_56() {
    let aj = d("aja~", Bhvadi);
    assert_has_krdanta(&["pra"], &aj, Krt::anIyar, &["pravayaRIya"]);
    assert_has_krdanta(&["pra"], &aj, Krt::Rvul, &["pravAyaka"]);
    // aGaYap?
    assert_has_krdanta(&["sam"], &aj, Krt::GaY, &["samAja"]);
    assert_has_krdanta(&["ud"], &aj, Krt::GaY, &["udAja"]);
    assert_has_krdanta(&["sam"], &aj, Krt::ap, &["samaja"]);
    assert_has_krdanta(&["ud"], &aj, Krt::ap, &["udaja"]);
}

#[test]
fn sutra_2_4_56_v2() {
    let aj = d("aja~", Bhvadi);
    assert_has_krdanta(&["pra"], &aj, Krt::tfc, &["pravetf", "prAjitf"]);
    assert_has_krdanta(&["pra"], &aj, Krt::tumun, &["pravetum", "prAjitum"]);
}

#[test]
fn sutra_2_4_57() {
    let aj = d("aja~", Bhvadi);
    assert_has_krdanta(&["pra"], &aj, Krt::lyuw, &["pravayaRa", "prAjana"]);
}

#[test]
fn sutra_2_4_72() {
    assert_has_lat_p(&[], &d("a\\da~", Adadi), &["atti"]);
    assert_has_lat_p(&[], &d("ha\\na~", Adadi), &["hanti"]);
    assert_has_lat_p(&[], &d("dvi\\za~^", Adadi), &["dvezwi"]);
}

#[test]
fn sutra_2_4_75() {
    assert_has_lat_p(&[], &d("hu\\", Juhotyadi), &["juhoti"]);
    assert_has_lat_p(&[], &d("quBf\\Y", Juhotyadi), &["biBarti"]);
    assert_has_lat_p(&[], &d("Ri\\ji~^r", Juhotyadi), &["nenekti"]);
}

#[test]
fn sutra_2_4_77() {
    let i = &d("i\\R", Adadi);
    assert_has_lun_p(&[], &i, &["agAt"]);
    assert_has_lun_p(&[], &d("zWA\\", Bhvadi), &["asTAt"]);
    assert_has_lun_p(&[], &d("qudA\\Y", Juhotyadi), &["adAt"]);
    assert_has_lun_p(&[], &d("quDA\\Y", Juhotyadi), &["aDAt"]);
    assert_has_lun_p(&[], &d("pA\\", Bhvadi), &["apAt"]);
    assert_has_lun_p(&[], &d("BU", Bhvadi), &["aBUt"]);

    // But, not gAyati or pAti
    assert_has_lun_p(&[], &d("gE\\", Bhvadi), &["agAsIt"]);
    assert_has_lun_p(&[], &d("pA\\", Adadi), &["apAsIt"]);

    // TODO: parasmEpadezu
    // assert_has_karmani_tinanta(&[], &i, Lun, Prathama, Dvi, &["agAsAtAm"]);
}

#[test]
fn sutra_2_4_78() {
    assert_has_lun_p(&[], &d("GrA\\", Bhvadi), &["aGrAt", "aGrAsIt"]);
    assert_has_lun_p(&[], &d("De\\w", Bhvadi), &["aDAt", "aDAsIt", "adaDat"]);
    assert_has_lun_p(&[], &d("So\\", Divadi), &["aSAt", "aSAsIt"]);
    assert_has_lun_p(&[], &d("Co\\", Divadi), &["acCAt", "acCAsIt"]);
    assert_has_lun_p(&[], &d("zo\\", Divadi), &["asAt", "asAsIt"]);

    // TODO: parasmEpadezu
}

#[test]
fn sutra_2_4_79() {
    let assert_has_ta = |prefixes, dhatu, expected| {
        assert_has_atmane_tinanta(prefixes, dhatu, Lun, Prathama, Eka, expected);
    };
    let assert_has_thas = |prefixes, dhatu, expected| {
        assert_has_atmane_tinanta(prefixes, dhatu, Lun, Madhyama, Eka, expected);
    };

    let tan = d("tanu~^", Tanadi);
    assert_has_ta(&[], &tan, &["atata", "atanizwa"]);
    assert_has_thas(&[], &tan, &["ataTAH", "atanizWAH"]);
    let san = d("zaRu~^", Tanadi);
    assert_has_ta(&[], &san, &["asAta", "asanizwa"]);
    assert_has_thas(&[], &san, &["asATAH", "asanizWAH"]);

    // parasmEpadezu
    assert_has_parasmai_tinanta(&[], &tan, Lun, Madhyama, Bahu, &["atAnizwa", "atanizwa"]);
}

#[test]
fn sutra_2_4_85() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_parasmai_tinanta(&[], &kf, Lut, Prathama, Eka, &["kartA"]);
    assert_has_parasmai_tinanta(&[], &kf, Lut, Prathama, Dvi, &["kartArO"]);
    assert_has_parasmai_tinanta(&[], &kf, Lut, Prathama, Bahu, &["kartAraH"]);

    let i = d("i\\N", Adadi);
    assert_has_tinanta(&["aDi"], &i, Lut, Prathama, Eka, &["aDyetA"]);
    assert_has_tinanta(&["aDi"], &i, Lut, Prathama, Dvi, &["aDyetArO"]);
    assert_has_tinanta(&["aDi"], &i, Lut, Prathama, Bahu, &["aDyetAraH"]);

    // praTamasya
    assert_has_parasmai_tinanta(&[], &kf, Lut, Madhyama, Eka, &["kartAsi"]);
    assert_has_tinanta(&["aDi"], &i, Lut, Madhyama, Eka, &["aDyetAse"]);
}
