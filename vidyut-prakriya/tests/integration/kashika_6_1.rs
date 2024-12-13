extern crate test_utils;
use lazy_static::lazy_static;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::*;

lazy_static! {
    static ref S: Tester = Tester::with_svara_rules();
}

#[test]
fn sutra_6_1_1() {
    assert_has_tip(
        &[],
        &d("jAgf", Adadi),
        Lit,
        &["jajAgAra", "jAgarAYcakAra", "jAgarAmbaBUva", "jAgarAmAsa"],
    );
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lit, &["papAca"]);
    assert_has_tip(&[], &d("i\\R", Adadi), Lit, &["iyAya"]);
    assert_has_tip(&[], &d("f\\", Juhotyadi), Lit, &["Ara"]);
}

#[test]
fn sutra_6_1_2() {
    let san = |u, g| d(u, g).with_sanadi(&[Sanadi::san]);
    assert_has_tip(&[], &san("awa~", Bhvadi), Lat, &["awiwizati"]);
    assert_has_tip(&[], &san("aSa~", Kryadi), Lat, &["aSiSizati"]);
    assert_has_tip(&[], &san("f\\", Kryadi), Lat, &["aririzati"]);
}

#[test]
fn sutra_6_1_3() {
    let san = |u, g| d(u, g).with_sanadi(&[Sanadi::san]);
    assert_has_tip(&[], &san("undI~", Rudhadi), Lat, &["undidizati"]);
    assert_has_tip(&[], &san("adqa~", Bhvadi), Lat, &["aqqiqizati"]);
    assert_has_tip(&[], &san("arca~", Bhvadi), Lat, &["arcicizati"]);
    // ndrA
    assert_has_lat(&[], &san("Ikza~\\", Bhvadi), &["Icikzizate"]);
    // saMyoga
    assert_has_lat(&["pra"], &san("ana~", Adadi), &["prARiRizati"]);
    // ajAdi
    assert_has_lat(&[], &san("drA\\", Adadi), &["didrAsati"]);

    // TODO: indidrIyizati
}

#[test]
fn sutra_6_1_3_v1() {
    assert_has_lat(&[], &san(&d("ubja~", Tudadi)), &["ubjijizati"]);
}

#[test]
fn sutra_6_1_3_v2() {
    assert_has_lat(&[], &yan(&d("f\\", Bhvadi)), &["arAryate"]);
}

#[test]
fn sutra_6_1_4() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    let san = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::san]);
    let yan = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::yaN]);

    assert_has_tip(&[], &pac, Lit, &["papAca"]);
    assert_has_tip(&[], &san(&pac), Lat, &["pipakzati"]);
    assert_has_lat(&[], &yan(&pac), &["pApacyate"]);
    assert_has_tip(&[], &d("hu\\", Juhotyadi), Lat, &["juhoti"]);
    assert_has_tip(&[], &nic(&pac), Lun, &["apIpacat"]);
}

#[test]
fn sutra_6_1_5() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_jhi(&[], &daa, Lat, &["dadati"]);
    assert_has_krdanta(&[], &daa, Krt::Satf, &["dadat"]);
    assert_has_jhi(&[], &d("quDA\\Y", Juhotyadi), Lot, &["daDatu"]);
    // uBe
    assert_has_mip(&[], &d("Ri\\ji~^r", Juhotyadi), Lot, &["nenijAni"]);
}

#[test]
fn sutra_6_1_6() {
    assert_has_jhi(&[], &d("jakza~", Adadi), Lat, &["jakzati"]);
    assert_has_jhi(&[], &d("jAgf", Adadi), Lat, &["jAgrati"]);
    assert_has_jhi(&[], &d("daridrA", Adadi), Lat, &["daridrati"]);
    assert_has_jhi(&[], &d("cakAsf~", Adadi), Lat, &["cakAsati"]);
    assert_has_jhi(&[], &d("SAsu~", Adadi), Lat, &["SAsati"]);
    assert_has_jha(&[], &d("dIDIN", Adadi), Lat, &["dIDyate"]);
    assert_has_jha(&[], &d("vevIN", Adadi), Lat, &["vevyate"]);
}

#[test]
fn sutra_6_1_7() {
    // TODO: no comprehensive list of tujAdi dhatus anywhere -- how to test?
}

#[test]
fn sutra_6_1_8() {
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lit, &["papAca"]);
    assert_has_tip(&[], &d("paWa~", Bhvadi), Lit, &["papAWa"]);
    assert_has_tip(&["pra"], &d("UrRuY", Adadi), Lit, &["prorRunAva"]);
    // liwi
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::tfc, &["hartf"]);
    // TODO: nonAva (amantra)
    // assert_has_tip(&[], &yan_luk(&d("nu", Adadi)), Lit, &["nonAva"]);
}

#[test]
fn sutra_6_1_9() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    let f = d("f\\", Juhotyadi);
    assert_has_tip(&[], &san(&pac), Lat, &["pipakzati"]);
    assert_has_lat(&[], &san(&d("patx~", Bhvadi)), &["pipatizati", "pitsati"]);
    assert_has_tip(&[], &san(&f), Lat, &["aririzati"]);
    assert_has_tip(&[], &san(&d("undI~", Rudhadi)), Lat, &["undidizati"]);
    // yan
    assert_has_lat(&[], &yan(&pac), &["pApacyate"]);
    assert_has_lat(&[], &yan(&d("awa~", Bhvadi)), &["awAwyate"]);
    assert_has_lat(&[], &yan(&d("ya\\ja~^", Bhvadi)), &["yAyajyate"]);
    assert_has_lat(&[], &yan(&f), &["arAryate"]);
    assert_has_lat(&["pra"], &yan(&d("UrRuY", Adadi)), &["prorRonUyate"]);
    // anaByAsasya
    assert_has_lat(&[], &san(&d("gupa~\\", Bhvadi)), &["jugupsizate"]);
    let lu_yan_san = d("lUY", Kryadi).with_sanadi(&[Sanadi::yaN, Sanadi::san]);
    assert_has_lat(&[], &lu_yan_san, &["lolUyizate"]);
}

#[test]
fn sutra_6_1_10() {
    assert_has_tip(&[], &d("hu\\", Juhotyadi), Lat, &["juhoti"]);
    assert_has_tip(&[], &d("YiBI\\", Juhotyadi), Lat, &["biBeti"]);
    assert_has_tip(&[], &d("hrI\\", Juhotyadi), Lat, &["jihreti"]);
}

#[test]
fn sutra_6_1_11() {
    let nic = |u, g| d(u, g).with_sanadi(&[Sanadi::Ric]);

    assert_has_tip(&[], &nic("qupa\\ca~^z", Bhvadi), Lun, &["apIpacat"]);
    assert_has_tip(&[], &nic("paWa~", Bhvadi), Lun, &["apIpaWat"]);
    assert_has_tip(&[], &nic("awa~", Bhvadi), Lun, &["Awiwat"]);
    assert_has_tip(&[], &nic("aSU~\\", Svadi), Lun, &["ASiSat"]);
    assert_has_tip(&[], &nic("aSa~\\", Kryadi), Lun, &["ASiSat"]);
}

#[test]
fn sutra_6_1_15() {
    let vac = d("va\\ca~", Adadi);
    assert_has_krdanta(&[], &vac, Krt::kta, &["ukta"]);
    assert_has_krdanta(&[], &vac, Krt::ktavatu, &["uktavat"]);

    let svap = d("Yizva\\pa~", Adadi);
    assert_has_krdanta(&[], &svap, Krt::kta, &["supta"]);
    assert_has_krdanta(&[], &svap, Krt::ktavatu, &["suptavat"]);

    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_krdanta(&[], &yaj, Krt::kta, &["izwa"]);
    assert_has_krdanta(&[], &yaj, Krt::ktavatu, &["izwavat"]);

    let vap = d("quva\\pa~^", Bhvadi);
    assert_has_krdanta(&[], &vap, Krt::kta, &["upta"]);
    assert_has_krdanta(&[], &vap, Krt::ktavatu, &["uptavat"]);

    let vah = d("va\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::kta, &["UQa"]);
    assert_has_krdanta(&[], &vah, Krt::ktavatu, &["UQavat"]);

    let vas = d("va\\sa~", Bhvadi);
    assert_has_krdanta(&[], &vas, Krt::kta, &["uzita"]);
    assert_has_krdanta(&[], &vas, Krt::ktavatu, &["uzitavat"]);

    let ve = d("ve\\Y", Bhvadi);
    assert_has_krdanta(&[], &ve, Krt::kta, &["uta"]);
    assert_has_krdanta(&[], &ve, Krt::ktavatu, &["utavat"]);

    let vye = d("vye\\Y", Bhvadi);
    assert_has_krdanta(&["sam"], &vye, Krt::kta, &["saMvIta"]);
    assert_has_krdanta(&["sam"], &vye, Krt::ktavatu, &["saMvItavat"]);

    let hve = d("hve\\Y", Bhvadi);
    assert_has_krdanta(&["AN"], &hve, Krt::ktavatu, &["AhUtavat"]);

    let vad = d("vada~", Bhvadi);
    assert_has_krdanta(&[], &vad, Krt::kta, &["udita"]);
    assert_has_krdanta(&[], &vad, Krt::ktavatu, &["uditavat"]);

    let svi = d("wuo~Svi", Bhvadi);
    assert_has_krdanta(&[], &svi, Krt::kta, &["SUna"]);
    assert_has_krdanta(&[], &svi, Krt::ktavatu, &["SUnavat"]);
}

#[test]
fn sutra_6_1_16() {
    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&[], &grah, Krt::kta, &["gfhIta"]);
    assert_has_krdanta(&[], &grah, Krt::ktavatu, &["gfhItavat"]);
    assert_has_tip(&[], &grah, Lat, &["gfhRAti"]);
    assert_has_ta(&[], &yan(&grah), Lat, &["jarIgfhyate"]);

    let jya = d("jyA\\", Kryadi);
    assert_has_krdanta(&[], &jya, Krt::kta, &["jIna"]);
    assert_has_krdanta(&[], &jya, Krt::ktavatu, &["jInavat"]);
    assert_has_tip(&[], &jya, Lat, &["jinAti"]);
    assert_has_ta(&[], &yan(&jya), Lat, &["jejIyate"]);

    let ve = d("ve\\Y", Bhvadi);
    assert_has_tas(&[], &ve, Lit, &["UyatuH", "UvatuH", "vavatuH"]);
    assert_has_jhi(&[], &ve, Lit, &["UyuH", "UvuH", "vavuH"]);

    let vyadh = d("vya\\Da~", Divadi);
    assert_has_krdanta(&[], &vyadh, Krt::kta, &["vidDa"]);
    assert_has_krdanta(&[], &vyadh, Krt::ktavatu, &["vidDavat"]);
    assert_has_tip(&[], &vyadh, Lat, &["viDyati"]);
    assert_has_ta(&[], &yan(&vyadh), Lat, &["veviDyate"]);

    let vash = d("vaSa~", Adadi);
    assert_has_krdanta(&[], &vash, Krt::kta, &["uSita"]);
    assert_has_krdanta(&[], &vash, Krt::ktavatu, &["uSitavat"]);
    assert_has_tas(&[], &vash, Lat, &["uzwaH"]);
    assert_has_jhi(&[], &vash, Lat, &["uSanti"]);

    let vyac = d("vyaca~", Tudadi);
    assert_has_krdanta(&[], &vyac, Krt::kta, &["vicita"]);
    assert_has_krdanta(&[], &vyac, Krt::ktavatu, &["vicitavat"]);
    assert_has_tip(&[], &vyac, Lat, &["vicati"]);
    assert_has_ta(&[], &yan(&vyac), Lat, &["vevicyate"]);
    assert_has_krdanta(&["ud"], &vyac, Krt::tfc, &["udvicitf"]);
    assert_has_krdanta(&["ud"], &vyac, Krt::tumun, &["udvicitum"]);
    assert_has_krdanta(&["ud"], &vyac, Krt::tavya, &["udvicitavya"]);

    let vrasc = d("o~vrascU~", Tudadi);
    assert_has_krdanta(&[], &vrasc, Krt::kta, &["vfkRa"]);
    assert_has_krdanta(&[], &vrasc, Krt::ktavatu, &["vfkRavat"]);
    assert_has_tip(&[], &vrasc, Lat, &["vfScati"]);
    assert_has_ta(&[], &yan(&vrasc), Lat, &["varIvfScyate"]);

    let prach = d("pra\\Ca~", Tudadi);
    assert_has_krdanta(&[], &prach, Krt::kta, &["pfzwa"]);
    assert_has_krdanta(&[], &prach, Krt::ktavatu, &["pfzwavat"]);
    assert_has_tip(&[], &prach, Lat, &["pfcCati"]);
    assert_has_ta(&[], &yan(&prach), Lat, &["parIpfcCyate"]);
    assert_has_krdanta(&[], &prach, Krt::naN, &["praSna"]);

    let bhrasj = d("Bra\\sja~^", Tudadi);
    assert_has_krdanta(&[], &bhrasj, Krt::kta, &["Bfzwa"]);
    assert_has_krdanta(&[], &bhrasj, Krt::ktavatu, &["Bfzwavat"]);
    assert_has_tip(&[], &bhrasj, Lat, &["Bfjjati"]);
    assert_has_ta(&[], &yan(&bhrasj), Lat, &["barIBfjjyate"]);
}

#[test]
fn sutra_6_1_17() {
    let assert_has_nal = |x, y, z| assert_has_tip(x, y, Lit, z);
    let assert_has_thal = |x, y, z| assert_has_sip(x, y, Lit, z);

    let vac = d("va\\ca~", Adadi);
    assert_has_nal(&[], &vac, &["uvAca"]);
    assert_has_thal(&[], &vac, &["uvaciTa", "uvakTa"]);

    let svap = d("Yizva\\pa~", Adadi);
    assert_has_nal(&[], &svap, &["suzvApa"]);
    assert_has_thal(&[], &svap, &["suzvapiTa", "suzvapTa"]);

    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_nal(&[], &yaj, &["iyAja"]);
    assert_has_thal(&[], &yaj, &["iyajiTa", "iyazWa"]);

    let vap = d("quva\\pa~^", Bhvadi);
    assert_has_nal(&[], &vap, &["uvApa"]);
    assert_has_thal(&[], &vap, &["uvapiTa", "uvapTa"]);

    let grah = d("graha~^", Kryadi);
    assert_has_nal(&[], &grah, &["jagrAha"]);
    assert_has_thal(&[], &grah, &["jagrahiTa"]);

    let jya = d("jyA\\", Kryadi);
    assert_has_nal(&[], &jya, &["jijyO"]);
    assert_has_thal(&[], &jya, &["jijyiTa", "jijyATa"]);

    let ve = d("ve\\Y", Bhvadi);
    assert_has_nal(&[], &ve, &["uvAya", "vavO"]);
    assert_has_thal(&[], &ve, &["uvayiTa", "vavATa", "vaviTa"]);

    let vyadh = d("vya\\Da~", Divadi);
    assert_has_nal(&[], &vyadh, &["vivyADa"]);
    assert_has_thal(&[], &vyadh, &["vivyaDiTa", "vivyadDa"]);

    let vash = d("vaSa~", Adadi);
    assert_has_nal(&[], &vash, &["uvASa"]);
    assert_has_thal(&[], &vash, &["uvaSiTa"]);

    let vyac = d("vyaca~", Tudadi);
    assert_has_nal(&[], &vyac, &["vivyAca"]);
    assert_has_thal(&[], &vyac, &["vivyaciTa"]);

    // SK justifies vavrazWa
    let vrasc = d("o~vrascU~", Tudadi);
    assert_has_nal(&[], &vrasc, &["vavraSca"]);
    assert_has_thal(&[], &vrasc, &["vavraSciTa", "vavrazWa"]);
}

#[test]
fn sutra_6_1_18() {
    let svap_nic = d("Yizva\\pa~", Adadi).with_sanadi(&[Sanadi::Ric]);
    assert_has_tip(&[], &svap_nic, Lun, &["asUzupat"]);
    assert_has_tas(&[], &svap_nic, Lun, &["asUzupatAm"]);
    assert_has_jhi(&[], &svap_nic, Lun, &["asUzupan"]);
    // caNi
    assert_has_ta_k(&[], &svap_nic, Lat, &["svApyate"]);
    assert_has_krdanta(&[], &svap_nic, Krt::kta, &["svApita"]);
}

#[test]
fn sutra_6_1_19() {
    let yan = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::yaN]);

    let svap = d("Yizva\\pa~", Adadi);
    let syam = d("syamu~", Bhvadi);
    let vye = d("vye\\Y", Bhvadi);
    assert_has_lat(&[], &yan(&svap), &["sozupyate"]);
    assert_has_lat(&[], &yan(&syam), &["sesimyate"]);
    assert_has_lat(&[], &yan(&vye), &["vevIyate"]);
    // yaNi
    assert_has_krdanta(&[], &svap, Krt::najiN, &["svapnaj"]);
}

#[test]
fn sutra_6_1_20() {
    let vash = d("vaSa~", Adadi);
    assert_has_ta(&[], &yan(&vash), Lat, &["vAvaSyate"]);
    assert_has_aataam(&[], &yan(&vash), Lat, &["vAvaSyete"]);
    assert_has_jha(&[], &yan(&vash), Lat, &["vAvaSyante"]);
    assert_has_tas(&[], &vash, Lat, &["uzwaH"]);
    assert_has_jhi(&[], &vash, Lat, &["uSanti"]);
}

#[test]
fn sutra_6_1_21() {
    let caay_yan = yan(&d("cAyf~^", Bhvadi));
    assert_has_ta(&[], &caay_yan, Lat, &["cekIyate"]);
    assert_has_aataam(&[], &caay_yan, Lat, &["cekIyete"]);
    assert_has_jha(&[], &caay_yan, Lat, &["cekIyante"]);
    // TODO: cekIta?
}

#[test]
fn sutra_6_1_22() {
    let sphay = d("sPAyI~\\", Bhvadi);
    assert_has_krdanta(&[], &sphay, Krt::kta, &["sPIta"]);
    assert_has_krdanta(&[], &sphay, Krt::ktavatu, &["sPItavat"]);
    assert_has_krdanta(&[], &sphay, Krt::ktin, &["sPAti"]);
}

#[test]
fn sutra_6_1_23() {
    let styai = d("styE\\", Bhvadi);
    assert_has_krdanta(&["pra"], &styai, Krt::kta, &["prastIta", "prastIma"]);
    assert_has_krdanta(
        &["pra"],
        &styai,
        Krt::ktavatu,
        &["prastItavat", "prastImavat"],
    );
    let zwyai = d("zwyE\\", Bhvadi);
    assert_has_krdanta(&["pra"], &zwyai, Krt::kta, &["prastIta", "prastIma"]);
    assert_has_krdanta(
        &["pra"],
        &zwyai,
        Krt::ktavatu,
        &["prastItavat", "prastImavat"],
    );
}

#[test]
fn sutra_6_1_28() {
    let pyay = d("o~pyAyI~\\", Bhvadi);
    assert_has_krdanta(&[], &pyay, Krt::kta, &["pIna"]);
}

#[test]
fn sutra_6_1_29() {
    let pyay = d("o~pyAyI~\\", Bhvadi);
    assert_has_ta(&["AN"], &pyay, Lit, &["Apipye"]);
    assert_has_aataam(&["AN"], &pyay, Lit, &["ApipyAte"]);
    assert_has_jha(&["AN"], &pyay, Lit, &["Apipyire"]);
    // yaNi
    assert_has_ta(&["AN"], &yan(&pyay), Lat, &["ApepIyate"]);
    assert_has_aataam(&["AN"], &yan(&pyay), Lat, &["ApepIyete"]);
    assert_has_jha(&["AN"], &yan(&pyay), Lat, &["ApepIyante"]);
}

#[test]
fn sutra_6_1_30() {
    let svi = d("wuo~Svi", Bhvadi);
    assert_has_tip(&[], &svi, Lit, &["SuSAva", "SiSvAya"]);
    assert_has_tas(&[], &svi, Lit, &["SuSuvatuH", "SiSviyatuH"]);
    assert_has_ta(&[], &yan(&svi), Lat, &["SoSUyate", "SeSvIyate"]);
}

#[test]
fn sutra_6_1_31() {
    let svi = d("wuo~Svi", Bhvadi);
    assert_has_tip(&[], &nic_san(&svi), Lat, &["SuSAvayizati", "SiSvAyayizati"]);
    assert_has_tip(&[], &nic(&svi), Lun, &["aSUSavat", "aSiSvayat"]);
}

#[test]
fn sutra_6_1_32() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_tip(&[], &nic_san(&hve), Lat, &["juhAvayizati"]);
    assert_has_tas(&[], &nic_san(&hve), Lat, &["juhAvayizataH"]);
    assert_has_jhi(&[], &nic_san(&hve), Lat, &["juhAvayizanti"]);
    assert_has_tip(&[], &nic(&hve), Lun, &["ajUhavat"]);
    assert_has_tas(&[], &nic(&hve), Lun, &["ajUhavatAm"]);
    assert_has_jhi(&[], &nic(&hve), Lun, &["ajUhavan"]);
}

#[test]
fn sutra_6_1_33() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_tip(&[], &hve, Lit, &["juhAva"]);
    assert_has_ta(&[], &yan(&hve), Lat, &["johUyate"]);
    assert_has_tip(&[], &san(&hve), Lat, &["juhUzati"]);
}

#[test]
fn sutra_6_1_38_and_sutra_6_1_39_and_sutra_6_1_40() {
    let ve = d("ve\\Y", Bhvadi);
    assert_has_tip(&[], &ve, Lit, &["uvAya", "vavO"]);
    assert_has_tas(&[], &ve, Lit, &["UyatuH", "UvatuH", "vavatuH"]);
    assert_has_jhi(&[], &ve, Lit, &["UyuH", "UvuH", "vavuH"]);
    // TODO: uvayitha
}

#[test]
fn sutra_6_1_41() {
    let ve = d("ve\\Y", Bhvadi);
    assert_has_krdanta(&["pra"], &ve, Krt::ktvA, &["pravAya"]);
    assert_has_krdanta(&["upa"], &ve, Krt::ktvA, &["upavAya"]);
}

#[test]
fn sutra_6_1_42() {
    let jya = d("jyA\\", Kryadi);
    assert_has_krdanta(&["pra"], &jya, Krt::ktvA, &["prajyAya"]);
    assert_has_krdanta(&["upa"], &jya, Krt::ktvA, &["upajyAya"]);
}

#[test]
fn sutra_6_1_43() {
    let vye = d("vye\\Y", Bhvadi);
    assert_has_krdanta(&["pra"], &vye, Krt::ktvA, &["pravyAya"]);
    assert_has_krdanta(&["upa"], &vye, Krt::ktvA, &["upavyAya"]);
}

#[test]
fn sutra_6_1_44() {
    let vye = d("vye\\Y", Bhvadi);
    assert_has_krdanta(&["pari"], &vye, Krt::ktvA, &["parivyAya", "parivIya"]);
}

#[test]
fn sutra_6_1_45() {
    let glai = d("glE\\", Bhvadi);
    assert_has_krdanta(&[], &glai, Krt::tfc, &["glAtf"]);
    assert_has_krdanta(&[], &glai, Krt::tumun, &["glAtum"]);
    assert_has_krdanta(&[], &glai, Krt::tavya, &["glAtavya"]);
    let so = d("So\\", Divadi);
    assert_has_krdanta(&["ni"], &so, Krt::tfc, &["niSAtf"]);
    assert_has_krdanta(&["ni"], &so, Krt::tumun, &["niSAtum"]);
    assert_has_krdanta(&["ni"], &so, Krt::tavya, &["niSAtavya"]);
    // ecaH
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::tfc, &["hartf"]);
    // upadeSe
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::tfc, &["stotf"]);
    // aSiti
    let mlai = d("mlE\\", Bhvadi);
    assert_has_tip(&[], &glai, Lat, &["glAyati"]);
    assert_has_tip(&[], &mlai, Lat, &["mlAyati"]);
    assert_has_ta_k(&[], &glai, Lit, &["jagle"]);
    assert_has_ta_k(&[], &mlai, Lit, &["mamle"]);
}

#[test]
fn sutra_6_1_46() {
    let vye = d("vye\\Y", Bhvadi);
    assert_has_tip(&["sam"], &vye, Lit, &["saMvivyAya"]);
    assert_has_sip(&["sam"], &vye, Lit, &["saMvivyayiTa"]);
}

#[test]
fn sutra_6_1_47() {
    assert_has_krdanta(
        &["vi"],
        &d("sPura~", Bhvadi),
        Krt::GaY,
        &["visPAra", "vizPAra"],
    );
    assert_has_krdanta(
        &["vi"],
        &d("sPula~", Bhvadi),
        Krt::GaY,
        &["visPAla", "vizPAla"],
    );
}

#[test]
fn sutra_6_1_48() {
    assert_has_tip(&[], &nic(&d("qukrI\\Y", Kryadi)), Lat, &["krApayati"]);
    assert_has_tip(&["aDi"], &nic(&d("i\\N", Adadi)), Lat, &["aDyApayati"]);
    assert_has_tip(&[], &nic(&d("ji\\", Bhvadi)), Lat, &["jApayati"]);
}

#[test]
fn sutra_6_1_49() {
    assert_has_tip(
        &[],
        &nic(&d("zi\\Du~", Divadi)),
        Lat,
        &["sADayati", "seDayati"],
    );
}

#[test]
fn sutra_6_1_50() {
    let mii = d("mI\\Y", Kryadi);
    assert_has_krdanta(&["pra"], &mii, Krt::tfc, &["pramAtf"]);
    assert_has_krdanta(&["pra"], &mii, Krt::tavya, &["pramAtavya"]);
    assert_has_krdanta(&["pra"], &mii, Krt::tumun, &["pramAtum"]);
    assert_has_krdanta(&["pra"], &mii, Krt::ktvA, &["pramAya"]);
    let mi = d("qumi\\Y", Svadi);
    assert_has_krdanta(&["ni"], &mi, Krt::tfc, &["nimAtf"]);
    assert_has_krdanta(&["ni"], &mi, Krt::tavya, &["nimAtavya"]);
    assert_has_krdanta(&["ni"], &mi, Krt::tumun, &["nimAtum"]);
    assert_has_krdanta(&["ni"], &mi, Krt::ktvA, &["nimAya"]);
    let di = d("dI\\N", Divadi);
    assert_has_krdanta(&["upa"], &di, Krt::tfc, &["upadAtf"]);
    assert_has_krdanta(&["upa"], &di, Krt::tavya, &["upadAtavya"]);
    assert_has_krdanta(&["upa"], &di, Krt::tumun, &["upadAtum"]);
    assert_has_krdanta(&["upa"], &di, Krt::ktvA, &["upadAya"]);
    // TODO: upadAya, IzadupadAna
    // assert_has_krdanta(&["upa"], &di, Krt::GaY, &["upadAya"]);
    // assert_has_krdanta(&["upa"], &di, Krt::yuc, &["upadAna"]);
}

#[test]
fn sutra_6_1_51() {
    let li = d("lI\\N", Divadi);
    assert_has_krdanta(&["vi"], &li, Krt::tfc, &["vilAtf", "viletf"]);
    assert_has_krdanta(&["vi"], &li, Krt::tavya, &["vilAtavya", "viletavya"]);
    assert_has_krdanta(&["vi"], &li, Krt::tumun, &["vilAtum", "viletum"]);
    assert_has_krdanta(&["vi"], &li, Krt::ktvA, &["vilAya", "vilIya"]);
}

#[test]
fn sutra_6_1_54() {
    assert_has_tip(
        &[],
        &nic(&d("ci\\Y", Svadi)),
        Lat,
        &["cApayati", "cAyayati"],
    );
    assert_has_tip(
        &[],
        &nic(&d("sPura~", Tudadi)),
        Lat,
        &["sPArayati", "sPorayati"],
    );
}

#[test]
fn sutra_6_1_55() {
    assert_has_tip(
        &["pra"],
        &nic(&d("vI\\", Adadi)),
        Lat,
        &["pravApayati", "pravAyayati"],
    );
}

#[test]
fn sutra_6_1_56() {
    assert_has_lat(
        &[],
        &nic(&d("YiBI\\", Juhotyadi)),
        &["BAyayati", "BAyayate", "BIzayate", "BApayate"],
    );
}

#[test]
fn sutra_6_1_57() {
    assert_has_lat(
        &["vi"],
        &nic(&d("zmi\\N", Bhvadi)),
        &["vismAyayati", "vismAyayate", "vismApayate"],
    );
}

#[test]
fn sutra_6_1_58() {
    let sfj = d("sf\\ja~", Tudadi);
    assert_has_krdanta(&[], &sfj, Krt::tfc, &["srazwf"]);
    assert_has_krdanta(&[], &sfj, Krt::tumun, &["srazwum"]);
    assert_has_krdanta(&[], &sfj, Krt::tavya, &["srazwavya"]);

    let dfs = d("df\\Si~r", Bhvadi);
    assert_has_krdanta(&[], &dfs, Krt::tfc, &["drazwf"]);
    assert_has_krdanta(&[], &dfs, Krt::tumun, &["drazwum"]);
    assert_has_krdanta(&[], &dfs, Krt::tavya, &["drazwavya"]);

    assert_has_tip(&[], &sfj, Lun, &["asrAkzIt"]);
    assert_has_tip(&[], &dfs, Lun, &["adrAkzIt", "adarSat"]);
    // jhali
    assert_has_krdanta(&[], &sfj, Krt::lyuw, &["sarjana"]);
    assert_has_krdanta(&[], &dfs, Krt::lyuw, &["darSana"]);
    // akiti
    assert_has_krdanta(&[], &sfj, Krt::kta, &["sfzwa"]);
    assert_has_krdanta(&[], &dfs, Krt::kta, &["dfzwa"]);
}

#[test]
fn sutra_6_1_59() {
    let tfp = d("tf\\pa~", Divadi);
    let dfp = d("df\\pa~", Divadi);
    assert_has_krdanta(&[], &tfp, Krt::tfc, &["traptf", "tarptf", "tarpitf"]);
    assert_has_krdanta(&[], &dfp, Krt::tfc, &["draptf", "darptf", "darpitf"]);
    // anudAttasya
    let vfh = d("vfhU~", Tudadi);
    assert_has_krdanta(&[], &vfh, Krt::tfc, &["varQf", "varhitf"]);
    assert_has_krdanta(&[], &vfh, Krt::tumun, &["varQum", "varhitum"]);
    assert_has_krdanta(&[], &vfh, Krt::tavya, &["varQavya", "varhitavya"]);
    // fdupaDasya
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &d("Ci\\di~^r", Rudhadi), Krt::tfc, &["Cettf"]);
    // Jali
    assert_has_krdanta(&[], &tfp, Krt::lyuw, &["tarpaRa"]);
    assert_has_krdanta(&[], &dfp, Krt::lyuw, &["darpaRa"]);
    // akiti
    assert_has_krdanta(&[], &tfp, Krt::kta, &["tfpta"]);
    assert_has_krdanta(&[], &dfp, Krt::kta, &["dfpta"]);
}

#[test]
fn sutra_6_1_64() {
    assert_has_lat(&[], &d("zaha~\\", Bhvadi), &["sahate"]);
    assert_has_tip(&[], &d("zi\\ca~^", Tudadi), Lat, &["siYcati"]);
    // DAtu?
    // assert_has_sup_1p("zoqaSan", Pum, &["zoqaSa"]);
    assert_has_sup_1s("zaRqa", Pum, &["zaRqaH"]);
    assert_has_sup_1s("zaqika", Pum, &["zaqikaH"]);
    // AdeH?
    assert_has_lat(&[], &d("kaza~", Bhvadi), &["kazati"]);
    assert_has_tip(&[], &d("laza~^", Bhvadi), Lat, &["lazati", "lazyati"]);
    assert_has_lat(&[], &d("kf\\za~", Tudadi), &["kfzati"]);
    assert_has_lat(&[], &d("zWivu~", Bhvadi), &["zWIvati"]);
    assert_has_lat(&[], &d("zvazka~\\", Bhvadi), &["zvazkate"]);
    assert_has_lat(
        &[],
        &yan(&d("zWivu~", Divadi)),
        &["tezWIvyate", "wezWIvyate"],
    );
    // TODO: others
}

#[test]
fn sutra_6_1_65() {
    assert_has_tip(&[], &d("RI\\Y", Bhvadi), Lat, &["nayati"]);
    assert_has_tip(&[], &d("Ra\\ma~", Bhvadi), Lat, &["namati"]);
    assert_has_tip(&[], &d("Ra\\ha~^", Divadi), Lat, &["nahyati"]);
    // dhAtu-AdeH
    assert_has_tip(&[], &d("aRa~", Bhvadi), Lat, &["aRati"]);
    // TODO: others
}

#[test]
fn sutra_6_1_66() {
    assert_has_krdanta(&[], &d("divu~", Divadi), Krt::kvasu, &["didivas"]);
    assert_has_krdanta(&[], &d("UyI~\\", Bhvadi), Krt::kta, &["Uta"]);
    assert_has_krdanta(&[], &d("knUyI~\\", Bhvadi), Krt::kta, &["knUta"]);

    // TODO: gODera
    assert_has_jha(&[], &d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceran"]);
    assert_has_jha(&[], &d("ya\\ja~^", Bhvadi), VidhiLin, &["yajeran"]);

    // vali?
    assert_has_ta_k(&[], &d("UyI~\\", Bhvadi), Lat, &["Uyyate"]);
    assert_has_ta_k(&[], &d("knUyI~\\", Bhvadi), Lat, &["knUyyate"]);

    // TODO: others
}

#[test]
fn sutra_6_1_69() {
    // eN
    assert_has_sup_ss("agni", Pum, &["agne"]);
    assert_has_sup_ss("vAyu", Pum, &["vAyo"]);
    // hrasva
    assert_has_sup_ss("devadatta", Pum, &["devadatta"]);
    assert_has_sup_ss("nadI", Stri, &["nadi"]);
    assert_has_sup_ss("vaDU", Stri, &["vaDu"]);
    assert_has_sup_ss("kuRqa", Napumsaka, &["kuRqa"]);
    assert_has_sup_ss("katarat", Napumsaka, &["katarat"]);
}

// saMhitAyAm ...

#[test]
fn sutra_6_1_75() {
    assert_has_lat(&[], &d("hrICa~", Bhvadi), &["hrIcCati"]);
    assert_has_lat(&[], &d("mleCa~", Bhvadi), &["mlecCati"]);
    let cho = d("Co\\", Divadi);
    assert_has_lat(&["apa"], &yan(&cho), &["apacAcCAyate"]);
    assert_has_lat(&["vi"], &yan(&cho), &["vicAcCAyate"]);
}

#[test]
fn sutra_6_1_77() {
    assert_has_sandhi("daDi", "atra", &["daDyatra"]);
    assert_has_sandhi("maDu", "atra", &["maDvatra"]);
    assert_has_sandhi("kartf", "arTam", &["kartrarTam"]);
    assert_has_sandhi("x", "AkftiH", &["lAkftiH"]);
}

#[test]
fn sutra_6_1_78() {
    let ci = d("ci\\Y", Svadi);
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &ci, Krt::lyuw, &["cayana"]);
    assert_has_krdanta(&[], &lu, Krt::lyuw, &["lavana"]);
    assert_has_krdanta(&[], &ci, Krt::Rvul, &["cAyaka"]);
    assert_has_krdanta(&[], &lu, Krt::Rvul, &["lAvaka"]);
    // TODO: others
}

#[test]
fn sutra_6_1_79() {
    assert_has_taddhita("baBru", T::yaY, &["bABravya"]);
    assert_has_taddhita("maRqu", T::yaY, &["mARqavya"]);
    assert_has_taddhita("SaNku", T::yat, &["SaNkavya"]);
    assert_has_taddhita("picu", T::yat, &["picavya"]);
    assert_has_taddhita("nO", T::yat, &["nAvya"]);

    // TODO: vAntaH?

    // yi?
    assert_has_sup_3d("go", Stri, &["goByAm"]);
    assert_has_sup_3d("nO", Stri, &["nOByAm"]);

    // pratyaye?
    assert_has_sandhi("go", "yAnam", &["goyAnam"]);
    assert_has_sandhi("nO", "yAnam", &["nOyAnam"]);
}

#[test]
fn sutra_6_1_80() {
    let lu = d("lUY", Kryadi);
    let pu = d("pUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::yat, &["lavya"]);
    assert_has_krdanta(&[], &pu, Krt::yat, &["pavya"]);
    assert_has_krdanta(&[], &lu, Krt::Ryat, &["lAvya"]);
    assert_has_krdanta(&[], &pu, Krt::Ryat, &["pAvya"]);

    // DAtoH?
    assert_has_taddhita("baBru", T::yaY, &["bABravya"]);
    assert_has_taddhita("go", T::yat, &["gavya"]);
    assert_has_taddhita("nO", T::yat, &["nAvya"]);

    // tan-nimittasya?
    let u = d("u\\N", Bhvadi);
    assert_has_ta_k(&["upa"], &u, Lat, &["upoyate"]);
    assert_has_ta_k(&[], &u, Lan, &["Oyata"]);
    assert_has_taddhita("loyamAna", T::iY, &["lOyamAni"]);
    assert_has_taddhita("poyamAna", T::iY, &["pOyamAni"]);
}

#[test]
fn sutra_6_1_84() {
    assert_has_sandhi("KawvA", "indraH", &["KawvendraH"]);
    assert_has_sandhi("mAlA", "indraH", &["mAlendraH"]);
}

#[test]
fn sutra_6_1_87() {
    assert_has_sandhi("tava", "idam", &["tavedam"]);
    assert_has_sandhi("KawvA", "indraH", &["KawvendraH"]);
    assert_has_sandhi("mAlA", "indraH", &["mAlendraH"]);
    assert_has_sandhi("tava", "Ihate", &["tavehate"]);
    assert_has_sandhi("KawvA", "Ihate", &["Kawvehate"]);
    assert_has_sandhi("tava", "udakam", &["tavodakam"]);
    assert_has_sandhi("KawvA", "udakam", &["Kawvodakam"]);
    assert_has_sandhi("tava", "fSayaH", &["tavarSayaH"]);
    assert_has_sandhi("KawvA", "fSayaH", &["KawvarSayaH"]);
    assert_has_sandhi("tava", "xkAraH", &["tavalkAraH"]);
    assert_has_sandhi("KawvA", "xkAraH", &["KawvalkAraH"]);
}

#[test]
fn sutra_6_1_88() {
    assert_has_sandhi("brahma", "eqakA", &["brahmEqakA"]);
    assert_has_sandhi("KawvA", "eqakA", &["KawvEqakA"]);
    assert_has_sandhi("brahma", "EtikAyanaH", &["brahmEtikAyanaH"]);
    assert_has_sandhi("KawvA", "EtikAyanaH", &["KawvEtikAyanaH"]);
    assert_has_sandhi("brahma", "odanaH", &["brahmOdanaH"]);
    assert_has_sandhi("KawvA", "odanaH", &["KawvOdanaH"]);
    assert_has_sandhi("brahma", "OpagavaH", &["brahmOpagavaH"]);
    assert_has_sandhi("KawvA", "OpagavaH", &["KawvOpagavaH"]);
}

#[test]
fn sutra_6_1_89() {
    let i = d("i\\R", Adadi);
    assert_has_tip(&["upa"], &i, Lat, &["upEti"]);
    assert_has_sip(&["upa"], &i, Lat, &["upEzi"]);
    assert_has_mip(&["upa"], &i, Lat, &["upEmi"]);
    let edh = d("eDa~\\", Bhvadi);
    assert_has_ta(&["upa"], &edh, Lat, &["upEDate"]);
    assert_has_ta(&["pra"], &edh, Lat, &["prEDate"]);
    // TODO: others
}

#[test]
fn sutra_6_1_90() {
    let ikz = d("Ikza~\\", Bhvadi);
    assert_has_lun(&[], &ikz, &["Ekzizwa"]);
    assert_has_lan(&[], &ikz, &["Ekzata"]);
    assert_has_lrn(&[], &ikz, &["Ekzizyata"]);

    assert_has_lun(&[], &d("uBa~", Tudadi), &["OBIt"]);
    assert_has_lan(&[], &d("fDu~", Svadi), &["ArDnot"]);
    assert_has_lun(&[], &d("ubja~", Tudadi), &["ObjIt"]);
    // TODO: others
}

#[test]
fn sutra_6_1_91() {
    let r = d("f\\", Bhvadi);
    assert_has_lat(&["upa"], &r, &["upArCati", "upArcCati"]);
    assert_has_lat(&["pra"], &r, &["prArCati", "prArcCati"]);

    let rdh = d("fDu~", Svadi);
    assert_has_lat(&["upa"], &rdh, &["upArDnoti"]);

    // Exception -- KawvA is not an upasarga
    assert_has_lat(&["KawvA"], &r, &["KawvarCati", "KawvarcCati"]);
}

#[test]
fn sutra_6_1_93() {
    assert_has_sup_2s("go", Stri, &["gAm"]);
    assert_has_sup_2p("go", Stri, &["gAH"]);
    assert_has_sup_2s("dyo", Stri, &["dyAm"]);
    assert_has_sup_2p("dyo", Stri, &["dyAH"]);
}

#[test]
fn sutra_6_1_94() {
    let il = d("ila~", Curadi);
    assert_has_tip(&["upa"], &il, Lat, &["upelayati"]);
    assert_has_tip(&["pra"], &il, Lat, &["prelayati"]);

    let uz = d("uza~", Bhvadi);
    assert_has_lat(&["upa"], &uz, &["upozati"]);
    assert_has_lat(&["pra"], &uz, &["prozati"]);
    // TODO: others.
}

#[test]
fn sutra_6_1_96() {
    assert_has_jhi(&[], &d("Bi\\di~^r", Rudhadi), VidhiLin, &["BindyuH"]);
    assert_has_jhi(&[], &d("Ci\\di~^r", Rudhadi), VidhiLin, &["CindyuH"]);
    assert_has_jhi(&[], &d("qudA\\Y", Juhotyadi), Lun, &["aduH"]);
    assert_has_jhi(&[], &d("yA\\", Adadi), Lan, &["ayuH", "ayAn"]);
    // TODO: apadAntAt
}

#[test]
fn sutra_6_1_97() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_jhi(&[], &pac, Lat, &["pacanti"]);
    assert_has_jhi(&[], &yaj, Lat, &["yajanti"]);
    assert_has_iw(&[], &pac, Lat, &["pace"]);
    assert_has_iw(&[], &yaj, Lat, &["yaje"]);

    // akaH?
    assert_has_jhi(&[], &d("yA\\", Adadi), Lat, &["yAnti"]);
    assert_has_jhi(&[], &d("vA\\", Adadi), Lat, &["vAnti"]);

    // guRe
    assert_has_iw(&[], &pac, Lan, &["apace"]);
    assert_has_iw(&[], &yaj, Lan, &["ayaje"]);

    // a-padAnta
    assert_has_sandhi("daRqa", "agram", &["daRqAgram"]);
    assert_has_sandhi("yUpa", "agram", &["yUpAgram"]);
}

#[test]
fn sutra_6_1_101() {
    assert_has_sandhi("daRqa", "agram", &["daRqAgram"]);
    assert_has_sandhi("daDi", "indraH", &["daDIndraH"]);
    assert_has_sandhi("maDu", "udake", &["maDUdake"]);
    assert_has_sandhi("hotf", "fSayaH", &["hotFSayaH"]);

    // akaH?
    assert_has_sup_4s("agni", Pum, &["agnaye"]);

    // savarne?
    assert_has_sandhi("daDi", "atra", &["daDyatra"]);

    // aci
    assert_has_sandhi("kumArI", "Sete", &["kumArI Sete"]);
}

#[test]
fn sutra_6_1_102() {
    assert_has_sup_1d("agni", Pum, &["agnI"]);
    assert_has_sup_1d("vAyu", Pum, &["vAyU"]);
    assert_has_sup_1p("vfkza", Pum, &["vfkzAH"]);
    assert_has_sup_1p("plakza", Pum, &["plakzAH"]);
    assert_has_sup_2p("vfkza", Pum, &["vfkzAn"]);
    assert_has_sup_2p("plakza", Pum, &["plakzAn"]);
}

#[test]
fn sutra_6_1_103() {
    assert_has_sup_2p("vfkza", Pum, &["vfkzAn"]);
    assert_has_sup_2p("agni", Pum, &["agnIn"]);
    assert_has_sup_2p("vAyu", Pum, &["vAyUn"]);
    assert_has_sup_2p("kartf", Pum, &["kartFn"]);
    assert_has_sup_2p("hartf", Pum, &["hartFn"]);
    assert_has_sup_2p("zaRqaka", Pum, &["zaRqakAn"]);
    assert_has_sup_2p("zaRQaka", Pum, &["zaRQakAn"]);
    assert_has_sup_2p("sTUra", Pum, &["sTUrAn"]);
    assert_has_sup_2p("araka", Pum, &["arakAn"]);
    // tasmAt
    assert_has_sup_2p("go", Pum, &["gAH"]);
    // SazaH
    assert_has_sup_1p("vfkza", Pum, &["vfkzAH"]);
    assert_has_sup_1p("plakza", Pum, &["plakzAH"]);
    // puMsi
    assert_has_sup_2p("Denu", Stri, &["DenUH"]);
    assert_has_sup_2p("bahvI", Stri, &["bahvIH"]);
    assert_has_sup_2p("kumArI", Stri, &["kumArIH"]);
    // TODO: caYca
}

#[test]
fn sutra_6_1_104() {
    assert_has_sup_1d("vfkza", Pum, &["vfkzO"]);
    assert_has_sup_1d("plakza", Pum, &["plakzO"]);
    assert_has_sup_1d(&nyap("KawvA"), Stri, &["Kawve"]);
    assert_has_sup_1d(&nyap("kuRqA"), Stri, &["kuRqe"]);
    // At
    assert_has_sup_1d("agni", Pum, &["agnI"]);
    // ici
    assert_has_sup_1p("vfkza", Pum, &["vfkzAH"]);
}

#[test]
fn sutra_6_1_105() {
    assert_has_sup_1d("kumArI", Stri, &["kumAryO"]);
    assert_has_sup_1p("kumArI", Stri, &["kumAryaH"]);
    assert_has_sup_1d("brahmabanDU", Stri, &["brahmabanDvO"]);
    assert_has_sup_1p("brahmabanDU", Stri, &["brahmabanDvaH"]);
}

#[test]
fn sutra_6_1_107() {
    assert_has_sup_2s("vfkza", Pum, &["vfkzam"]);
    assert_has_sup_2s("plakza", Pum, &["plakzam"]);
    assert_has_sup_2s("agni", Pum, &["agnim"]);
    assert_has_sup_2s("vAyu", Pum, &["vAyum"]);
    assert_has_sup_2s("kumArI", Stri, &["kumArIm"]);
}

#[test]
fn sutra_6_1_108() {
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::kta, &["izwa"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), Krt::kta, &["upta"]);
    assert_has_krdanta(&[], &d("graha~^", Kryadi), Krt::kta, &["gfhIta"]);

    // TODO: others
}

#[ignore]
#[test]
fn sutra_6_1_109() {
    assert_has_sandhi("agne", "atra", &["agne tra"]);
    assert_has_sandhi("vAyo", "atra", &["vAyo tra"]);

    // eNaH?
    assert_has_sandhi("daDi", "atra", &["daDyatra"]);
    assert_has_sandhi("maDu", "atra", &["maDvatra"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::lyuw, &["cayana"]);
    assert_has_krdanta(&[], &d("lUY", Kryadi), Krt::lyuw, &["lavana"]);

    // ati
    assert_has_sandhi("vAyo", "iti", &["vAyo iti"]);
    assert_has_sandhi("BAno", "iti", &["BAno iti"]);

    // TODO: others
}

#[test]
fn sutra_6_1_110() {
    assert_has_sup_5s("agni", Pum, &["agneH"]);
    assert_has_sup_5s("vAyu", Pum, &["vAyoH"]);
    assert_has_sup_6s("agni", Pum, &["agneH"]);
    assert_has_sup_6s("vAyu", Pum, &["vAyoH"]);
}

#[test]
fn sutra_6_1_111() {
    assert_has_sup_5s("hotf", Pum, &["hotuH"]);
    assert_has_sup_6s("hotf", Pum, &["hotuH"]);
}

#[test]
fn sutra_6_1_112() {
    assert_has_sup_5s("saKi", Pum, &["saKyuH"]);
    assert_has_sup_6s("saKi", Pum, &["saKyuH"]);
    assert_has_sup_5s("pati", Pum, &["patyuH"]);
    assert_has_sup_6s("pati", Pum, &["patyuH"]);

    // TODO: others
}

#[test]
fn sutra_6_1_113() {
    assert_has_sandhi("vfkzas", "atra", &["vfkzo tra"]);
    assert_has_sandhi("plakzas", "atra", &["plakzo tra"]);
    // ataH
    assert_has_sandhi("agnis", "atra", &["agnir atra"]);
    // taparakaraRa
    assert_has_sandhi("vfkzAs", "atra", &["vfkzA atra"]);
}

#[test]
fn sutra_6_1_114() {
    assert_has_sandhi("puruzas", "yAti", &["puruzo yAti"]);
    assert_has_sandhi("puruzas", "hasati", &["puruzo hasati"]);
    assert_has_sandhi("puruzas", "dadAti", &["puruzo dadAti"]);
}

#[test]
fn sutra_6_1_125() {
    let agni = sup_1d("agnI", "agni", Pum);
    let vayu = sup_1d("vAyU", "vAyu", Pum);
    let khatve = sup_1d("Kawve", &nyap("KawvA"), Stri);
    let male = sup_1d("mAle", &nyap("mAlA"), Stri);
    let iti = Pada::from_text("iti");

    assert_has_vakya(&agni, &iti, &["agnI iti"]);
    assert_has_vakya(&vayu, &iti, &["vAyU iti"]);
    assert_has_vakya(&khatve, &iti, &["Kawve iti"]);
    assert_has_vakya(&male, &iti, &["mAle iti"]);

    // TODO: pluta and others
}

#[test]
fn sutra_6_1_135() {
    let kf = d("qukf\\Y", Tanadi);

    assert_has_tip(&["sam"], &kf, Lut, &["saMskartA", "saNkartA"]);
    assert_has_krdanta(&["sam"], &kf, Krt::tumun, &["saMskartum", "saNkartum"]);
    assert_has_krdanta(&["sam"], &kf, Krt::tavya, &["saMskartavya", "saNkartavya"]);

    assert_has_ta(&["sam"], &kf, AshirLin, &["saMskfzIzwa", "saNkfzIzwa"]);
    assert_has_ta_k(&["sam"], &kf, Lat, &["saMskriyate", "saNkriyate"]);

    assert_has_tas(&["sam"], &kf, Lit, &["saYcaskaratuH", "saYcakratuH"]);
    assert_has_jhi(&["sam"], &kf, Lit, &["saYcaskaruH", "saYcakruH"]);
}

#[test]
fn sutra_6_1_136() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_tip(&["sam"], &kr, Lan, &["samaskarot", "samakarot"]);
    assert_has_tip(&["sam"], &kr, Lun, &["samaskArzIt", "samakArzIt"]);
    assert_has_tip(&["sam"], &kr, Lit, &["saYcaskAra", "saYcakAra"]);
    assert_has_tip(&["pari"], &kr, Lit, &["paricaskAra", "paricakAra"]);
}

#[test]
fn sutra_6_1_137() {
    let kf = d("qukf\\Y", Tanadi);

    assert_has_tip(&["sam"], &kf, Lut, &["saMskartA", "saNkartA"]);
    assert_has_krdanta(&["sam"], &kf, Krt::tumun, &["saMskartum", "saNkartum"]);
    assert_has_krdanta(&["sam"], &kf, Krt::tavya, &["saMskartavya", "saNkartavya"]);

    assert_has_tip(&["pari"], &kf, Lut, &["parizkartA", "parikartA"]);
    assert_has_krdanta(&["pari"], &kf, Krt::tumun, &["parizkartum", "parikartum"]);
    assert_has_krdanta(
        &["pari"],
        &kf,
        Krt::tavya,
        &["parizkartavya", "parikartavya"],
    );

    assert_has_tip(&["upa"], &kf, Lut, &["upaskartA", "upakartA"]);
    assert_has_krdanta(&["upa"], &kf, Krt::tumun, &["upaskartum", "upakartum"]);
    assert_has_krdanta(&["upa"], &kf, Krt::tavya, &["upaskartavya", "upakartavya"]);
}

#[test]
fn sutra_6_1_139() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&["upa"], &kr, Krt::kta, &["upaskfta", "upakfta"]);
    assert_has_ta(&["upa"], &kr, Lat, &["upaskurute", "upakurute"]);
}

#[test]
fn sutra_6_1_140() {
    let kr = d("kF", Tudadi);
    assert_has_lat(&["upa"], &kr, &["upaskirati", "upakirati"]);
}

#[test]
fn sutra_6_1_141() {
    let kr = d("kF", Tudadi);
    assert_has_krdanta(&["prati"], &kr, Krt::kta, &["pratiskIrRa", "pratikIrRa"]);
}

#[test]
fn sutra_6_1_159() {
    S.assert_has_krdanta(&[], &d("kf\\za~", Bhvadi), Krt::GaY, &["karza/"]);
}

#[ignore]
#[test]
fn sutra_6_1_162() {
    S.assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lat, &["pa/cati"]);
    S.assert_has_tip(&[], &d("paWa~", Bhvadi), Lat, &["pa/Wati"]);
    S.assert_has_tip(&[], &d("UrRuY", Adadi), Lat, &["UrRo/ti", "UrRO/ti"]);
    S.assert_has_tip(&[], &d("gupU~", Bhvadi), Lat, &["gopAya/ti"]);
    S.assert_has_tip(&[], &d("yA\\", Adadi), Lat, &["yA/ti"]);
}

#[test]
fn sutra_6_1_163() {
    S.assert_has_krdanta(&[], &d("Ba\\njo~", Rudhadi), Krt::Gurac, &["BaNgura/"]);
    S.assert_has_krdanta(&[], &d("BAsf~\\", Bhvadi), Krt::Gurac, &["BAsura/"]);
    S.assert_has_krdanta(&[], &d("YimidA~", Divadi), Krt::Gurac, &["medura/"]);
    // TODO: others
}

#[test]
fn sutra_6_1_164() {
    S.assert_has_artha_taddhita("kuYja", Gotra, T::cPaY, &["kOYjAyana/"]);
    // TODO: where is this from?
    // S.assert_has_artha_taddhita("muYja", Gotra, T::cPaY, &["mOYjAyana/"]);
}

#[test]
fn sutra_6_1_165() {
    S.assert_has_artha_taddhita("naqa", Gotra, T::Pak, &["nAqAyana/"]);
    S.assert_has_artha_taddhita("cara", Gotra, T::Pak, &["cArAyaRa/"]);
    S.assert_has_artha_taddhita("akza", TenaDivyatiJayatiJitam, T::Wak, &["Akzika/"]);
}

#[test]
fn sutra_6_1_166() {
    S.assert_has_sup_1p("tri", Stri, &["tisra/H"]);
}

#[test]
fn sutra_6_1_167() {
    S.assert_has_sup_2p("catur", Pum, &["catu/raH"]);
}

#[test]
fn sutra_6_1_179() {
    S.assert_has_sup_3p("zaz", Pum, &["zaqBi/H"]);
    S.assert_has_sup_4p("zaz", Pum, &["zaqBya/H"]);
    S.assert_has_sup_6p("paYcan", Pum, &["paYcAnA/m"]);
    S.assert_has_sup_6p("zaz", Pum, &["zaRRA/m"]);
    S.assert_has_sup_6p("saptan", Pum, &["saptAnA/m"]);

    S.assert_has_sup_3p("tri", Pum, &["triBi/H"]);
    S.assert_has_sup_4p("tri", Pum, &["triBya/H"]);
    S.assert_has_sup_6p("tri", Pum, &["trayARA/m"]);

    // TODO: caturByaH?
    S.assert_has_sup_6p("catur", Pum, &["caturRA/m"]);

    // halAdiH?
    S.assert_has_sup_2p("catur", Stri, &["cata/sraH"]);
}

#[test]
fn sutra_6_1_180() {
    S.assert_has_sup_3p("paYcan", Pum, &["paYca/BiH"]);
    S.assert_has_sup_3p("saptan", Pum, &["sapta/BiH"]);
    S.assert_has_sup_3p("tri", Stri, &["tisf/BiH"]);
    S.assert_has_sup_3p("catur", Pum, &["catu/rBiH"]);

    // Jali?
    S.assert_has_sup_6p("paYcan", Pum, &["paYcAnA/m"]);
    S.assert_has_sup_6p("saptan", Pum, &["saptAnA/m"]);

    // upottama?
    S.assert_has_sup_3p("zaz", Pum, &["zaqBi/H"]);
    S.assert_has_sup_4p("zaz", Pum, &["zaqBya/H"]);
}

#[test]
fn sutra_6_1_183() {
    S.assert_has_sup_3d("div", Pum, &["dyu/ByAm"]);
    S.assert_has_sup_3p("div", Pum, &["dyu/BiH"]);

    // Jal?
    S.assert_has_sup_3s("div", Pum, &["di/vA"]);
}

#[test]
fn sutra_6_1_185() {
    let kr = d("qukf\\Y", Tanadi);
    let hr = d("hf\\Y", Bhvadi);
    S.assert_has_krdanta(&[], &san(&kr), Krt::yat, &["cikIrzya^"]);
    S.assert_has_krdanta(&[], &san(&hr), Krt::yat, &["jihIrzya^"]);
    S.assert_has_krdanta(&[], &kr, Krt::Ryat, &["kArya^"]);
    S.assert_has_krdanta(&[], &hr, Krt::Ryat, &["hArya^"]);
}

#[test]
fn sutra_6_1_191() {
    S.assert_has_sup_1s("sarva", Pum, &["sa/rvaH"]);
    S.assert_has_sup_1d("sarva", Pum, &["sa/rvO"]);
    S.assert_has_sup_1p("sarva", Pum, &["sa/rve"]);
}

#[test]
fn sutra_6_1_193() {
    let kr = d("qukf\\Y", Tanadi);
    let hr = d("hf\\Y", Bhvadi);
    S.assert_has_krdanta(&[], &san(&kr), Krt::Rvul, &["cikI/rzaka"]);
    S.assert_has_krdanta(&[], &san(&hr), Krt::Rvul, &["jihI/rzaka"]);

    S.assert_has_taddhita("BOriki", T::viDal, &["BOriki/viDa"]);
    S.assert_has_taddhita("BOliki", T::viDal, &["BOliki/viDa"]);
    S.assert_has_taddhita("EzukAri", T::Baktal, &["EzukAri/Bakta"]);
}

#[test]
fn sutra_6_1_197() {
    S.assert_has_taddhita("garga", T::yaY, &["gA/rgya"]);
}

#[test]
fn sutra_6_1_199() {
    S.assert_has_sup_1s("paTin", Pum, &["pa/nTAH"]);
    S.assert_has_sup_1d("paTin", Pum, &["pa/nTAnO"]);
    S.assert_has_sup_1p("paTin", Pum, &["pa/nTAnaH"]);

    S.assert_has_sup_1s("maTin", Pum, &["ma/nTAH"]);
    S.assert_has_sup_1d("maTin", Pum, &["ma/nTAnO"]);
    S.assert_has_sup_1p("maTin", Pum, &["ma/nTAnaH"]);

    // TODO: others
}

#[test]
fn sutra_6_1_211() {
    S.assert_has_sup_6s("yuzmad", Pum, &["ta/va"]);
    S.assert_has_sup_6s("asmad", Pum, &["ma/ma"]);
}

#[test]
fn sutra_6_1_212() {
    S.assert_has_sup_4s("yuzmad", Pum, &["tu/Byam"]);
    S.assert_has_sup_4s("asmad", Pum, &["ma/hyam"]);
}

#[test]
fn sutra_6_1_217() {
    let kr = d("qukf\\Y", Tanadi);
    let hr = d("hf\\Y", Bhvadi);
    S.assert_has_krdanta(&[], &kr, Krt::anIyar, &["karaRI/ya"]);
    S.assert_has_krdanta(&[], &hr, Krt::anIyar, &["haraRI/ya"]);

    S.assert_has_taddhita("pawu", T::jAtIyar, &["pawujAtI/ya"]);
    S.assert_has_taddhita("mfdu", T::jAtIyar, &["mfdujAtI/ya"]);
}

#[test]
fn sutra_6_1_222() {
    let dadhyac = create_upapada_krdanta("daDyac", "daDi", &[], &d("ancu~", Bhvadi), Krt::kvin);
    S.assert_has_sup_2p(&dadhyac, Pum, &["daDI/caH"]);
    S.assert_has_sup_3s(&dadhyac, Pum, &["daDI/cA"]);
    S.assert_has_sup_4s(&dadhyac, Pum, &["daDI/ce"]);

    let madhvac = create_upapada_krdanta("maDvac", "maDu", &[], &d("ancu~", Bhvadi), Krt::kvin);
    S.assert_has_sup_2p(&madhvac, Pum, &["maDU/caH"]);
    S.assert_has_sup_3s(&madhvac, Pum, &["maDU/cA"]);
    S.assert_has_sup_4s(&madhvac, Pum, &["maDU/ce"]);
}
