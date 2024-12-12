extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::*;

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

    assert_has_ta_k(&[], &ad, Lat, &["adyate"]);
    assert_has_krdanta(&[], &ad, Krt::tavya, &["attavya"]);

    // TODO: annam
}

#[test]
fn sutra_2_4_37() {
    let ad = d("a\\da~", Adadi);
    assert_has_tip(&[], &ad, Lun, &["aGasat"]);
    assert_has_tas(&[], &ad, Lun, &["aGasatAm"]);
    assert_has_jhi(&[], &ad, Lun, &["aGasan"]);
    assert_has_tip(&[], &san(&ad), Lat, &["jiGatsati"]);
    assert_has_tas(&[], &san(&ad), Lat, &["jiGatsataH"]);
    assert_has_jhi(&[], &san(&ad), Lat, &["jiGatsanti"]);
}

#[ignore]
#[test]
fn sutra_2_4_37_v1() {
    let ad = d("a\\da~", Adadi);
    assert_has_krdanta(&["pra"], &ad, Krt::ac, &["praGasa"]);
}

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
    assert_has_tip(&[], &ad, Lit, &["jaGAsa", "Ada"]);
    assert_has_tas(&[], &ad, Lit, &["jakzatuH", "AdatuH"]);
    assert_has_jhi(&[], &ad, Lit, &["jakzuH", "AduH"]);
}

#[test]
fn sutra_2_4_41() {
    let ve = d("ve\\Y", Bhvadi);
    assert_has_tip(&[], &ve, Lit, &["uvAya", "vavO"]);
    assert_has_tas(&[], &ve, Lit, &["UyatuH", "UvatuH", "vavatuH"]);
    assert_has_jhi(&[], &ve, Lit, &["UyuH", "UvuH", "vavuH"]);
}

#[test]
fn sutra_2_4_42() {
    let han = d("ha\\na~", Adadi);
    assert_has_tip(&[], &han, AshirLin, &["vaDyAt"]);
    assert_has_tas(&[], &han, AshirLin, &["vaDyAstAm"]);
    assert_has_jhi(&[], &han, AshirLin, &["vaDyAsuH"]);
}

#[test]
fn sutra_2_4_43() {
    let han = d("ha\\na~", Adadi);
    assert_has_tip(&[], &han, Lun, &["avaDIt"]);
    assert_has_tas(&[], &han, Lun, &["avaDizwAm"]);
    assert_has_jhi(&[], &han, Lun, &["avaDizuH"]);
}

#[test]
fn sutra_2_4_44() {
    let han = d("ha\\na~", Adadi);
    assert_has_ta(&["AN"], &han, Lun, &["AvaDizwa", "Ahata"]);
    assert_has_aataam(&["AN"], &han, Lun, &["AvaDizAtAm", "AhasAtAm"]);
    assert_has_jha(&["AN"], &han, Lun, &["AvaDizata", "Ahasata"]);
}

#[test]
fn sutra_2_4_45() {
    let i = d("i\\R", Adadi);
    assert_has_tip(&[], &i, Lun, &["agAt"]);
    assert_has_tas(&[], &i, Lun, &["agAtAm"]);
    assert_has_jhi(&[], &i, Lun, &["aguH"]);
    // karmani
    assert_has_ta_k(&[], &i, Lun, &["agAyi"]);
}

#[test]
fn sutra_2_4_45_v1() {
    let ik = d("i\\k", Adadi);
    assert_has_tip(&["aDi"], &ik, Lun, &["aDyagAt"]);
    assert_has_tas(&["aDi"], &ik, Lun, &["aDyagAtAm"]);
    assert_has_jhi(&["aDi"], &ik, Lun, &["aDyaguH"]);
}

#[test]
fn sutra_2_4_46() {
    let i = d("i\\R", Adadi);
    assert_has_tip(&[], &nic(&i), Lat, &["gamayati"]);
    assert_has_tas(&[], &nic(&i), Lat, &["gamayataH"]);
    assert_has_jhi(&[], &nic(&i), Lat, &["gamayanti"]);

    // aboDane?
    assert_has_tip(&["prati"], &nic(&i), Lat, &["pratyAyayati"]);

    // iRvadika?
    let ik = d("i\\k", Adadi);
    assert_has_tip(&["aDi"], &nic(&ik), Lat, &["aDigamayati"]);
}

// All pass except for pratIzizati
#[ignore]
#[test]
fn sutra_2_4_47() {
    let i_san = san(&d("i\\R", Adadi));
    assert_has_tip(&[], &i_san, Lat, &["jigamizati"]);
    assert_has_tas(&[], &i_san, Lat, &["jigamizataH"]);
    assert_has_jhi(&[], &i_san, Lat, &["jigamizanti"]);

    // aboDane?
    assert_has_tip(&["prati"], &i_san, Lat, &["pratIzizati"]);

    // iRvadika?
    let ik = d("i\\k", Adadi);
    assert_has_tip(&["aDi"], &nic(&ik), Lat, &["aDijigamizati"]);
}

#[test]
fn sutra_2_4_48() {
    let i_san = san(&d("i\\N", Adadi));
    assert_has_ta(&["aDi"], &i_san, Lat, &["aDijigAMsate"]);
    assert_has_aataam(&["aDi"], &i_san, Lat, &["aDijigAMsete"]);
    assert_has_jha(&["aDi"], &i_san, Lat, &["aDijigAMsante"]);
}

#[test]
fn sutra_2_4_49() {
    let i = d("i\\N", Adadi);
    assert_has_ta(&["aDi"], &i, Lit, &["aDijage"]);
    assert_has_aataam(&["aDi"], &i, Lit, &["aDijagAte"]);
    assert_has_jha(&["aDi"], &i, Lit, &["aDijagire"]);
}

#[test]
fn sutra_2_4_50() {
    let i = d("i\\N", Adadi);
    assert_has_ta(&["aDi"], &i, Lun, &["aDyagIzwa", "aDyEzwa"]);
    assert_has_aataam(&["aDi"], &i, Lun, &["aDyagIzAtAm", "aDyEzAtAm"]);
    assert_has_jha(&["aDi"], &i, Lun, &["aDyagIzata", "aDyEzata"]);

    assert_has_ta(&["aDi"], &i, Lrn, &["aDyagIzyata", "aDyEzyata"]);
    assert_has_aataam(&["aDi"], &i, Lrn, &["aDyagIzyetAm", "aDyEzyetAm"]);
    assert_has_jha(&["aDi"], &i, Lrn, &["aDyagIzyanta", "aDyEzyanta"]);
}

#[ignore]
#[test]
fn sutra_2_4_51() {
    let i = d("i\\N", Adadi);
    let i_nic_san = i.clone().with_sanadi(&[Sanadi::Ric, Sanadi::san]);
    assert_has_tip(
        &["aDi"],
        &i_nic_san,
        Lat,
        &["aDijigApayizati", "aDyApipayizati"],
    );
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
    assert_has_tinantas(
        &["AN"],
        &cakz,
        Lit,
        Prathama,
        Eka,
        &["AcaKyO", "AcaKye", "AcakSO", "AcakSe", "Acacakze"],
    );
    assert_has_tinantas(
        &["AN"],
        &cakz,
        Lit,
        Prathama,
        Dvi,
        &[
            "AcaKyatuH",
            "AcaKyAte",
            "AcakSatuH",
            "AcakSAte",
            "AcacakzAte",
        ],
    );
    assert_has_tinantas(
        &["AN"],
        &cakz,
        Lit,
        Prathama,
        Bahu,
        &["AcaKyuH", "AcaKyire", "AcakSuH", "AcakSire", "Acacakzire"],
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
    assert_has_tip(&[], &d("a\\da~", Adadi), Lat, &["atti"]);
    assert_has_tip(&[], &d("ha\\na~", Adadi), Lat, &["hanti"]);
    assert_has_tip(&[], &d("dvi\\za~^", Adadi), Lat, &["dvezwi"]);
}

#[ignore]
#[test]
fn sutra_2_4_74() {
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &yan(&lu), Krt::ac, &["loluva"]);

    let pu = d("pUY", Kryadi);
    assert_has_krdanta(&[], &yan(&pu), Krt::ac, &["popuva"]);

    // TODO: we currently create sanIsrasya, then luk --> sanIsras.
    // Instead, we should perform luk earlier based on the pratyaya we want to use.
    let srans = d("sransu~\\", Bhvadi);
    assert_has_krdanta(&[], &yan(&srans), Krt::ac, &["sanIsraMsa"]);

    let dhvans = d("Dvansu~\\", Bhvadi);
    assert_has_krdanta(&[], &yan(&dhvans), Krt::ac, &["danIDvaMsa"]);

    let lap = d("lapa~", Bhvadi);
    assert_has_lat(&[], &yan_luk(&lap), &["lAlapIti", "lAlapti"]);

    let vad = d("vada~", Bhvadi);
    assert_has_lat(&[], &yan_luk(&vad), &["vAvadIti", "vAvatti"]);
}

#[test]
fn sutra_2_4_75() {
    assert_has_tip(&[], &d("hu\\", Juhotyadi), Lat, &["juhoti"]);
    assert_has_tip(&[], &d("quBf\\Y", Juhotyadi), Lat, &["biBarti"]);
    assert_has_tip(&[], &d("Ri\\ji~^r", Juhotyadi), Lat, &["nenekti"]);
}

#[test]
fn sutra_2_4_77() {
    let i = &d("i\\R", Adadi);
    assert_has_tip(&[], &i, Lun, &["agAt"]);
    assert_has_tip(&[], &d("zWA\\", Bhvadi), Lun, &["asTAt"]);
    assert_has_tip(&[], &d("qudA\\Y", Juhotyadi), Lun, &["adAt"]);
    assert_has_tip(&[], &d("quDA\\Y", Juhotyadi), Lun, &["aDAt"]);
    assert_has_tip(&[], &d("pA\\", Bhvadi), Lun, &["apAt"]);
    assert_has_tip(&[], &d("BU", Bhvadi), Lun, &["aBUt"]);

    // But, not gAyati or pAti
    assert_has_tip(&[], &d("gE\\", Bhvadi), Lun, &["agAsIt"]);
    assert_has_tip(&[], &d("pA\\", Adadi), Lun, &["apAsIt"]);

    // TODO: parasmEpadezu
    // assert_has_karmani_tinanta(&[], &i, Lun, Prathama, Dvi, &["agAsAtAm"]);
}

#[test]
fn sutra_2_4_78() {
    assert_has_tip(&[], &d("GrA\\", Bhvadi), Lun, &["aGrAt", "aGrAsIt"]);
    assert_has_tip(&[], &d("De\\w", Bhvadi), Lun, &["aDAt", "aDAsIt", "adaDat"]);
    assert_has_tip(&[], &d("So\\", Divadi), Lun, &["aSAt", "aSAsIt"]);
    assert_has_tip(&[], &d("Co\\", Divadi), Lun, &["acCAt", "acCAsIt"]);
    assert_has_tip(&[], &d("zo\\", Divadi), Lun, &["asAt", "asAsIt"]);

    // TODO: parasmEpadezu
}

#[test]
fn sutra_2_4_79() {
    let assert_has_ta = |prefixes, dhatu, expected| {
        assert_has_ta(prefixes, dhatu, Lun, expected);
    };
    let assert_has_thas = |prefixes, dhatu, expected| {
        assert_has_thaas(prefixes, dhatu, Lun, expected);
    };

    let tan = d("tanu~^", Tanadi);
    assert_has_ta(&[], &tan, &["atata", "atanizwa"]);
    assert_has_thas(&[], &tan, &["ataTAH", "atanizWAH"]);
    let san = d("zaRu~^", Tanadi);
    assert_has_ta(&[], &san, &["asAta", "asanizwa"]);
    assert_has_thas(&[], &san, &["asATAH", "asanizWAH"]);

    // parasmEpadezu
    assert_has_tha(&[], &tan, Lun, &["atAnizwa", "atanizwa"]);
}

#[test]
fn sutra_2_4_85() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tip(&[], &kf, Lut, &["kartA"]);
    assert_has_tas(&[], &kf, Lut, &["kartArO"]);
    assert_has_jhi(&[], &kf, Lut, &["kartAraH"]);

    let i = d("i\\N", Adadi);
    assert_has_tinantas(&["aDi"], &i, Lut, Prathama, Eka, &["aDyetA"]);
    assert_has_tinantas(&["aDi"], &i, Lut, Prathama, Dvi, &["aDyetArO"]);
    assert_has_tinantas(&["aDi"], &i, Lut, Prathama, Bahu, &["aDyetAraH"]);

    // praTamasya
    assert_has_sip(&[], &kf, Lut, &["kartAsi"]);
    assert_has_thaas(&["aDi"], &i, Lut, &["aDyetAse"]);
}
