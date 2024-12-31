extern crate test_utils;
use lazy_static::lazy_static;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha as TA;
use vidyut_prakriya::args::*;

lazy_static! {
    static ref S: Tester = Tester::with_svara_rules();
}

fn sanadi(p: Pratipadika, s: Sanadi) -> Dhatu {
    Dhatu::nama(p, Some(s))
}

fn p(text: &str) -> Pratipadika {
    phit(text)
}

#[test]
fn sutra_3_1_1() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
}

#[test]
fn sutra_3_1_2() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya"]);
    assert_has_artha_taddhita("tittiri", TA::TenaProktam, T::CaR, &["tEttirIya"]);
}

#[test]
fn sutra_3_1_3() {
    let kf = d("qukf\\Y", Tanadi);
    S.assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya^"]);
    S.assert_has_artha_taddhita("tittiri", TA::TenaProktam, T::CaR, &["tEttirI/ya"]);
}

#[test]
fn sutra_3_1_4() {
    // sup
    S.assert_has_sup_1d("dfzad", Pum, &["dfza/dO"]);
    S.assert_has_sup_1p("dfzad", Pum, &["dfza/daH"]);

    // pit
    S.assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lat, &["pa/cati"]);
    S.assert_has_tip(&[], &d("paWa~", Bhvadi), Lat, &["pa/Wati"]);
}

#[test]
fn sutra_3_1_5() {
    assert_has_lat(&[], &d("gupa~\\", Bhvadi), &["jugupsate"]);
    assert_has_lat(&[], &d("tija~\\", Bhvadi), &["titikzate"]);
    assert_has_lat(&[], &d("kita~", Bhvadi), &["cikitsati"]);
}

#[test]
fn sutra_3_1_6() {
    assert_has_lat(&[], &d("mAna~\\", Bhvadi), &["mImAMsate"]);
    assert_has_lat(&[], &d("baDa~\\", Bhvadi), &["bIBatsate"]);
    assert_has_ta(&[], &d("dAna~^", Bhvadi), Lat, &["dIdAMsate"]);
    assert_has_ta(&[], &d("SAna~^", Bhvadi), Lat, &["SISAMsate"]);

    // TODO: mAnayati, etc.
}

#[test]
fn sutra_3_1_7() {
    let san_d = |u, gana| san(&d(u, gana));
    let kf = san_d("qukf\\Y", Tanadi);
    let hf = san_d("hf\\Y", Bhvadi);

    assert_has_tip(&[], &kf, Lat, &["cikIrzati"]);
    assert_has_tip(&[], &hf, Lat, &["jihIrzati"]);
    assert_has_tip(&["pra"], &kf, Lan, &["prAcikIrzat"]);

    assert_has_tip(
        &[],
        &san_d("patx~", Bhvadi),
        Lat,
        &["pipatizati", "pitsati"],
    );
    assert_has_tip(&[], &san_d("mf\\N", Tudadi), Lat, &["mumUrzati"]);
}

#[test]
fn sutra_3_1_8() {
    let putriya = Dhatu::nama(p("putra"), Some(Sanadi::kyac));
    assert_has_tip(&[], &putriya, Lat, &["putrIyati"]);
}

#[test]
fn sutra_3_1_9() {
    let kamyac = |prati| Dhatu::nama(prati, Some(Sanadi::kAmyac));
    assert_has_tip(&[], &kamyac(p("putra")), Lat, &["putrakAmyati"]);
    assert_has_tip(&[], &kamyac(p("yaSas")), Lat, &["yaSaskAmyati"]);

    assert_has_tip(&[], &kamyac(p("sarpis")), Lat, &["sarpizkAmyati"]);
    assert_has_tip(&[], &kamyac(p("kim")), Lat, &["kiNkAmyati"]);
    assert_has_tip(&[], &kamyac(p("svar")), Lat, &["svaHkAmyati"]);
}

#[test]
fn sutra_3_1_10() {
    assert_has_tip(&[], &kyac(p("putra")), Lat, &["putrIyati"]);
    assert_has_tip(&[], &kyac(p("prAvAra")), Lat, &["prAvArIyati"]);
}

#[test]
fn sutra_3_1_10_v1() {
    assert_has_tip(&[], &kyac(p("prAsAda")), Lat, &["prAsAdIyati"]);
    assert_has_tip(&[], &kyac(p("paryaNka")), Lat, &["paryaNkIyati"]);
}

#[test]
fn sutra_3_1_11() {
    assert_has_ta(&[], &kyan(p("Syena")), Lat, &["SyenAyate"]);
    assert_has_ta(&[], &kyan(p("puzkara")), Lat, &["puzkarAyate"]);
    assert_has_ta(&[], &kyan(p("ojas")), Lat, &["ojAyate"]);
    assert_has_krdanta(&[], &kyan(p("ojas")), Krt::SAnac, &["ojAyamAna"]);
    assert_has_ta(&[], &kyan(p("apsaras")), Lat, &["apsarAyate"]);
    assert_has_ta(&[], &kyan(p("payas")), Lat, &["payAyate", "payasyate"]);
    assert_has_ta(&[], &kyan(p("sArasa")), Lat, &["sArasAyate"]);
    assert_has_ta(&[], &kyan(p("haMsa")), Lat, &["haMsAyate"]);
}

#[test]
fn sutra_3_1_12() {
    assert_has_ta(&[], &kyan(p("BfSa")), Lat, &["BfSAyate"]);
    assert_has_ta(&[], &kyan(p("SIGra")), Lat, &["SIGrAyate"]);
}

#[test]
fn sutra_3_1_13() {
    let kyas = |prati| Dhatu::nama(prati, None);
    assert_has_lat(&[], &kyas(p("lohita")), &["lohitAyati", "lohitAyate"]);
}

#[test]
fn sutra_3_1_14() {
    assert_has_ta(&[], &kyan(p("kazwa")), Lat, &["kazwAyate"]);
}

#[test]
fn sutra_3_1_14_v1() {
    assert_has_ta(&[], &kyan(p("satra")), Lat, &["satrAyate"]);
    assert_has_ta(&[], &kyan(p("kazwa")), Lat, &["kazwAyate"]);
    assert_has_ta(&[], &kyan(p("kakza")), Lat, &["kakzAyate"]);
    assert_has_ta(&[], &kyan(p("kfcCra")), Lat, &["kfcCrAyate"]);
    assert_has_ta(&[], &kyan(p("gahana")), Lat, &["gahanAyate"]);
}

#[test]
fn sutra_3_1_15() {
    let nama = |prati| Dhatu::nama(prati, None);
    assert_has_ta(&[], &nama(p("romanTa")), Lat, &["romanTAyate"]);
    assert_has_tip(&[], &nama(p("tapas")), Lat, &["tapasyati"]);
}

#[test]
fn sutra_3_1_16() {
    assert_has_ta(&[], &kyan(p("bAzpa")), Lat, &["bAzpAyate"]);
    assert_has_ta(&[], &kyan(p("uzma")), Lat, &["uzmAyate"]);
}

#[test]
fn sutra_3_1_16_v1() {
    assert_has_ta(&[], &kyan(p("Pena")), Lat, &["PenAyate"]);
}

#[test]
fn sutra_3_1_17() {
    assert_has_ta(&[], &kyan(p("Sabda")), Lat, &["SabdAyate"]);
    assert_has_ta(&[], &kyan(p("vEra")), Lat, &["vErAyate"]);
    assert_has_ta(&[], &kyan(p("kalaha")), Lat, &["kalahAyate"]);
    assert_has_ta(&[], &kyan(p("aBra")), Lat, &["aBrAyate"]);
    assert_has_ta(&[], &kyan(p("kaRva")), Lat, &["kaRvAyate"]);
    assert_has_ta(&[], &kyan(p("meGa")), Lat, &["meGAyate"]);
}

#[test]
fn sutra_3_1_17_v1() {
    assert_has_ta(&[], &kyan(p("sudina")), Lat, &["sudinAyate"]);
    assert_has_ta(&[], &kyan(p("durdina")), Lat, &["durdinAyate"]);
    assert_has_ta(&[], &kyan(p("nIhAra")), Lat, &["nIhArAyate"]);
}

#[test]
fn sutra_3_1_17_v2() {
    assert_has_ta(&[], &kyan(p("awA")), Lat, &["awAyate"]);
    assert_has_ta(&[], &kyan(p("awwA")), Lat, &["awwAyate"]);
    assert_has_ta(&[], &kyan(p("SIkA")), Lat, &["SIkAyate"]);
    assert_has_ta(&[], &kyan(p("kowA")), Lat, &["kowAyate"]);
    assert_has_ta(&[], &kyan(p("powA")), Lat, &["powAyate"]);
    assert_has_ta(&[], &kyan(p("sowA")), Lat, &["sowAyate"]);
    assert_has_ta(&[], &kyan(p("pruzwA")), Lat, &["pruzwAyate"]);
    assert_has_ta(&[], &kyan(p("pluzwA")), Lat, &["pluzwAyate"]);
}

#[test]
fn sutra_3_1_18() {
    assert_has_ta(&[], &kyan(p("suKa")), Lat, &["suKAyate"]);
    assert_has_ta(&[], &kyan(p("duHKa")), Lat, &["duHKAyate"]);
}

#[test]
fn sutra_3_1_19() {
    let nama = |prati| Dhatu::nama(p(prati), None);
    assert_has_tip(&[], &nama("namas"), Lat, &["namasyati"]);
    assert_has_tip(&[], &nama("varivas"), Lat, &["varivasyati"]);
    assert_has_ta(&[], &nama("citra"), Lat, &["citrIyate"]);
}

#[test]
fn sutra_3_1_20() {
    let nama = |prati| Dhatu::nama(p(prati), None);
    assert_has_ta(&["ud"], &nama("pucCa"), Lat, &["utpucCayate"]);
    assert_has_ta(&["pari"], &nama("pucCa"), Lat, &["paripucCayate"]);
    assert_has_ta(&["sam"], &nama("BARqa"), Lat, &["samBARqayate"]);
    assert_has_ta(&["sam"], &nama("cIvara"), Lat, &["saYcIvarayate"]);
}

#[ignore]
#[test]
fn sutra_3_1_21() {
    let nama = |prati| Dhatu::nama(p(prati), None);
    assert_has_tip(&[], &nama("muRqa"), Lat, &["muRqayati"]);
    assert_has_tip(&[], &nama("miSra"), Lat, &["miSrayati"]);
    assert_has_tip(&[], &nama("SlakzRa"), Lat, &["SlakzRayati"]);
    assert_has_tip(&[], &nama("lavaRa"), Lat, &["lavaRayati"]);
    assert_has_tip(&[], &nama("vrata"), Lat, &["vratayati"]);
    assert_has_tip(&["sam"], &nama("vastra"), Lat, &["saMvastrayati"]);
    assert_has_tip(&[], &nama("hali"), Lat, &["halayati"]);
    assert_has_tip(&[], &nama("kali"), Lat, &["kalayati"]);
    assert_has_tip(&[], &nama("hali"), Lun, &["ajahalat"]);
    assert_has_tip(&[], &nama("kali"), Lun, &["acakalat"]);
    assert_has_tip(&[], &nama("kft"), Lat, &["kftayati"]);
    assert_has_tip(&["vi"], &nama("tUsta"), Lat, &["tUstayati"]);
}

#[test]
fn sutra_3_1_22() {
    assert_has_lat(&[], &yan(&d("qupa\\ca~^z", Bhvadi)), &["pApacyate"]);
    assert_has_lat(&[], &yan(&d("ya\\ja~^", Bhvadi)), &["yAyajyate"]);
    assert_has_lat(&[], &yan(&d("jvala~", Bhvadi)), &["jAjvalyate"]);
    assert_has_lat(&[], &yan(&d("dIpI~\\", Divadi)), &["dedIpyate"]);
}

#[test]
fn sutra_3_1_22_v1() {
    assert_has_lat(&[], &yan(&d("sUca", Curadi)), &["sosUcyate"]);
    assert_has_lat(&[], &yan(&d("sUtra", Curadi)), &["sosUtryate"]);
    assert_has_lat(&[], &yan(&d("mUtra", Curadi)), &["momUtryate"]);
    assert_has_lat(&[], &yan(&d("awa~", Bhvadi)), &["awAwyate"]);
    assert_has_lat(&[], &yan(&d("f\\", Juhotyadi)), &["arAryate"]);
    assert_has_lat(&[], &yan(&d("aSa~", Kryadi)), &["aSASyate"]);
    assert_has_lat(&["pra"], &yan(&d("UrRuY", Adadi)), &["prorRonUyate"]);
}

#[test]
fn sutra_3_1_23() {
    assert_has_lat(&[], &yan(&d("kramu~", Bhvadi)), &["caNkramyate"]);
    assert_has_lat(&[], &yan(&d("drama~", Bhvadi)), &["dandramyate"]);
}

#[test]
fn sutra_3_1_24() {
    assert_has_lat(&[], &yan(&d("lu\\px~^", Tudadi)), &["lolupyate"]);
    assert_has_lat(&[], &yan(&d("za\\dx~", Bhvadi)), &["sAsadyate"]);
    assert_has_lat(&[], &yan(&d("cara~", Bhvadi)), &["caYcUryate"]);
    assert_has_lat(&[], &yan(&d("japa~", Bhvadi)), &["jaYjapyate"]);
    assert_has_lat(&[], &yan(&d("jaBI~\\", Bhvadi)), &["jaYjaByate"]);
    assert_has_lat(&[], &yan(&d("da\\ha~", Bhvadi)), &["dandahyate"]);
    assert_has_lat(&[], &yan(&d("da\\nSa~", Bhvadi)), &["dandaSyate"]);
    assert_has_lat(&["ni"], &yan(&d("gF", Tudadi)), &["nijegilyate"]);
}

#[ignore]
#[test]
fn sutra_3_1_25() {
    let nama = |prati| Dhatu::nama(p(prati), None);
    assert_has_tip(&[], &nama("satya"), Lat, &["satyApayati"]);
    assert_has_tip(&["vi"], &nama("pASa"), Lat, &["vipASayati"]);
    assert_has_tip(&[], &nama("rUpa"), Lat, &["rUpayati"]);
    assert_has_tip(&["upa"], &nama("vIRA"), Lat, &["upavIRayati"]);
    assert_has_tip(&["anu"], &nama("tUla"), Lat, &["anutUlayati"]);
    assert_has_tip(&["upa"], &nama("Sloka"), Lat, &["upaSlokayati"]);
    assert_has_tip(&["aBi"], &nama("senA"), Lat, &["aBizeRayati"]);
    assert_has_tip(&["anu"], &nama("loman"), Lat, &["anulomayati"]);
    assert_has_tip(&[], &nama("tvaca"), Lat, &["tvacayati"]);
    assert_has_tip(&["sam"], &nama("varman"), Lat, &["saMvarmayati"]);
    assert_has_tip(&[], &nama("varRa"), Lat, &["varRayati"]);
    assert_has_tip(&["ava"], &nama("cUrRa"), Lat, &["avacUrRayati"]);

    assert_has_tip(&[], &d("cura~", Curadi), Lat, &["corayati"]);
    assert_has_tip(&[], &d("citi~", Curadi), Lat, &["cintayati"]);
}

#[test]
fn sutra_3_1_25_v1() {
    let nama = |prati| Dhatu::nama(p(prati), None);
    assert_has_tip(&[], &nama("satya"), Lat, &["satyApayati"]);
    assert_has_tip(&[], &nama("arTa"), Lat, &["arTApayati"]);
    assert_has_tip(&[], &nama("veda"), Lat, &["vedApayati"]);
}

#[test]
fn sutra_3_1_26() {
    let san = |u, gana| d(u, gana).with_sanadi(&[Sanadi::Ric]);
    assert_has_tip(&[], &san("qukf\\Y", Tanadi), Lat, &["kArayati"]);
    assert_has_tip(&[], &san("qupa\\ca~^z", Bhvadi), Lat, &["pAcayati"]);

    // TODO: add others
}

#[test]
fn sutra_3_1_27() {
    let kandu = d("kaRqUY", Kandvadi);
    assert_has_lat(&[], &kandu, &["kaRqUyati", "kaRqUyate"]);
}

#[test]
fn sutra_3_1_28() {
    let pan = &d("paRa~\\", Bhvadi);
    assert_has_tip(&[], &d("gupU~", Bhvadi), Lat, &["gopAyati"]);
    assert_has_tip(&[], &d("DUpa~", Bhvadi), Lat, &["DUpAyati"]);
    assert_has_tip(&[], &d("viCa~", Tudadi), Lat, &["vicCAyati"]);
    assert_has_tip(&[], &pan, Lat, &["paRAyati"]);
    assert_has_tip(&[], &d("pana~\\", Bhvadi), Lat, &["panAyati"]);
    assert_has_ta(&[], &pan, Lat, &["paRate"]);
}

#[test]
fn sutra_3_1_29() {
    assert_has_ta(&[], &d("fti", Bhvadi), Lat, &["ftIyate"]);
}

#[test]
fn sutra_3_1_30() {
    assert_has_ta(&[], &d("kamu~\\", Bhvadi), Lat, &["kAmayate"]);
}

#[test]
fn sutra_3_1_31() {
    let assert_has_tfc = |u, gana, expected| {
        assert_has_krdanta(&[], &d(u, gana), Krt::tfc, expected);
    };
    assert_has_tfc("gupU~", Bhvadi, &["goptf", "gopitf", "gopAyitf"]);
    assert_has_tfc("fti", Bhvadi, &["artitf", "ftIyitf"]);
    assert_has_tfc("kamu~\\", Bhvadi, &["kamitf", "kAmayitf"]);
}

#[test]
fn sutra_3_1_32() {
    use Sanadi::*;
    assert_has_tip(&[], &sanadi(p("putra"), kyac), Lat, &["putrIyati"]);
    assert_has_tip(&[], &sanadi(p("putra"), kAmyac), Lat, &["putrakAmyati"]);
}

#[test]
fn sutra_3_1_33() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tip(&[], &kf, Lrt, &["karizyati"]);
    assert_has_tip(&[], &kf, Lrn, &["akarizyat"]);
    assert_has_lut(&[], &kf, &["kartA"]);
    assert_has_lut(&[], &d("ma\\na~\\", Divadi), &["mantA"]);
    assert_has_lut(&["sam"], &d("ga\\mx~", Bhvadi), &["saNgantA"]);
}

#[test]
fn sutra_3_1_34() {
    // No `\\` to force parasmaipada
    // -ti forms are not attested but optional by 3.4.97.
    assert_has_tip(&[], &d("juzI~", Tudadi), Let, &["jozizat"]);
    assert_has_tip(&[], &d("tF", Bhvadi), Let, &["tArizat"]);
    assert_has_tip(&[], &d("madi~", Bhvadi), Let, &["mandizat"]);

    // -t forms are not attested but optional by 3.4.97.
    assert_has_tip(&[], &d("patx~", Bhvadi), Let, &["patAti"]);
    assert_has_tip(&[], &nic(&d("cyu\\N", Bhvadi)), Let, &["cyAvayAti"]);
}

#[test]
fn sutra_3_1_35() {
    assert_has_lit(
        &[],
        &d("kAsf~\\", Bhvadi),
        &["kAsAYcakre", "kAsAmAsa", "kAsAmbaBUva"],
    );
    assert_has_lit(
        &[],
        &yan(&d("lUY", Kryadi)),
        &["lolUyAYcakre", "lolUyAmAsa", "lolUyAmbaBUva"],
    );
    // TODO: amantre
}

#[test]
fn sutra_3_1_36() {
    assert_has_lit(
        &[],
        &d("Iha~\\", Bhvadi),
        &["IhAYcakre", "IhAmAsa", "IhAmbaBUva"],
    );
    assert_has_lit(
        &[],
        &d("Uha~\\", Bhvadi),
        &["UhAYcakre", "UhAmAsa", "UhAmbaBUva"],
    );

    // ijAdeH?
    assert_has_tip(&[], &d("takzU~", Bhvadi), Lit, &["tatakza"]);
    assert_has_tip(&[], &d("rakza~", Bhvadi), Lit, &["rarakza"]);
    // gurumataH
    assert_has_mip(&[], &d("ya\\ja~^", Bhvadi), Lit, &["iyaja", "iyAja"]);
    assert_has_mip(&[], &d("quva\\pa~^", Bhvadi), Lit, &["uvapa", "uvApa"]);
    // anfcCaH
    assert_has_tip(&[], &d("fCa~", Tudadi), Lit, &["AnarcCa", "AnarCa"]);
    assert_has_tas(&[], &d("fCa~", Tudadi), Lit, &["AnarcCatuH", "AnarCatuH"]);
    assert_has_jhi(&[], &d("fCa~", Tudadi), Lit, &["AnarcCuH", "AnarCuH"]);
}

#[test]
fn sutra_3_1_36_v1() {
    assert_has_tip(&["pra"], &d("UrRuY", Adadi), Lit, &["prorRunAva"]);
}

#[test]
fn sutra_3_1_37() {
    assert_has_lit(
        &[],
        &d("daya~\\", Bhvadi),
        &["dayAYcakre", "dayAmAsa", "dayAmbaBUva"],
    );
    assert_has_lit(
        &[],
        &d("aya~\\", Bhvadi),
        &["ayAYcakre", "ayAmAsa", "ayAmbaBUva"],
    );
    assert_has_lit(
        &[],
        &d("Asa~\\", Adadi),
        &["AsAYcakre", "AsAmAsa", "AsAmbaBUva"],
    );
}

#[test]
fn sutra_3_1_38() {
    assert_has_lit(
        &[],
        &d("uza~", Bhvadi),
        &["ozAYcakAra", "ozAmAsa", "ozAmbaBUva", "uvoza"],
    );
    assert_has_lit(
        &[],
        &d("vida~", Adadi),
        &["vidAYcakAra", "vidAmAsa", "vidAmbaBUva", "viveda"],
    );
    assert_has_lit(
        &[],
        &d("jAgf", Adadi),
        &["jAgarAYcakAra", "jAgarAmAsa", "jAgarAmbaBUva", "jajAgAra"],
    );
}

#[test]
fn sutra_3_1_39() {
    assert_has_lit(
        &[],
        &d("YiBI\\", Juhotyadi),
        &["biBayAYcakAra", "biBayAmAsa", "biBayAmbaBUva", "biBAya"],
    );
    assert_has_lit(
        &[],
        &d("hrI\\", Juhotyadi),
        &["jihrayAYcakAra", "jihrayAmAsa", "jihrayAmbaBUva", "jihrAya"],
    );
    assert_has_tip(
        &[],
        &d("quBf\\Y", Juhotyadi),
        Lit,
        &["biBarAYcakAra", "biBarAmAsa", "biBarAmbaBUva", "baBAra"],
    );
    assert_has_lit(
        &[],
        &d("hu\\", Juhotyadi),
        &["juhavAYcakAra", "juhavAmAsa", "juhavAmbaBUva", "juhAva"],
    );
}

#[test]
fn sutra_3_1_40() {
    assert_has_tip(
        &[],
        &nic(&d("qupa\\ca~^z", Bhvadi)),
        Lit,
        &["pAcayAYcakAra", "pAcayAmAsa", "pAcayAmbaBUva"],
    );
}

#[test]
fn sutra_3_1_41() {
    let vid = d("vida~", Adadi);
    assert_has_jhi(&[], &vid, Lot, &["vidANkurvantu", "vidantu"]);
    // Also include other purusha+vacana combinations.
    assert_has_tip(
        &[],
        &vid,
        Lot,
        &["vidANkarotu", "vidANkurutAt", "vettu", "vittAt"],
    );
    assert_has_tas(&[], &vid, Lot, &["vidANkurutAm", "vittAm"]);
    assert_has_sip(
        &[],
        &vid,
        Lot,
        &["vidANkuru", "vidANkurutAt", "vidDi", "vittAt"],
    );
    assert_has_thas(&[], &vid, Lot, &["vidANkurutam", "vittam"]);
}

#[test]
fn skip_sutra_3_1_43() {}

#[test]
fn sutra_3_1_44() {
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lun, &["akArzIt"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lun, &["ahArzIt"]);
}

#[test]
fn sutra_3_1_44_v1() {
    assert_has_tip(
        &[],
        &d("spf\\Sa~", Tudadi),
        Lun,
        &["asprAkzIt", "aspArkzIt", "aspfkzat"],
    );
    assert_has_tip(
        &[],
        &d("mf\\Sa~", Tudadi),
        Lun,
        &["amrAkzIt", "amArkzIt", "amfkzat"],
    );
    assert_has_tip(
        &[],
        &d("kf\\za~", Bhvadi),
        Lun,
        &["akArkzIt", "akrAkzIt", "akfkzat"],
    );
    assert_has_tip(
        &[],
        &d("tf\\pa~", Divadi),
        Lun,
        &["atrApsIt", "atArpsIt", "atfpat", "atarpIt"],
    );
    assert_has_tip(
        &[],
        &d("df\\pa~", Divadi),
        Lun,
        &["adrApsIt", "adArpsIt", "adfpat", "adarpIt"],
    );
}

#[test]
fn sutra_3_1_45() {
    assert_has_tip(&[], &d("du\\ha~^", Adadi), Lun, &["aDukzat"]);
    assert_has_tip(&[], &d("li\\ha~^", Adadi), Lun, &["alikzat"]);
    // SalaH
    assert_has_tip(&[], &d("Bi\\di~^r", Rudhadi), Lun, &["aBEtsIt", "aBidat"]);
    assert_has_tip(&[], &d("Ci\\di~^r", Rudhadi), Lun, &["acCEtsIt", "acCidat"]);
    // igupaDAt
    assert_has_tip(&[], &d("da\\ha~", Bhvadi), Lun, &["aDAkzIt"]);
    // aniwaH
    assert_has_tip(&[], &d("kuza~", Kryadi), Lun, &["akozIt"]);
    assert_has_tip(&[], &d("muza~", Kryadi), Lun, &["amozIt"]);
}

#[test]
fn sutra_3_1_46() {
    assert_has_tip(
        &["AN"],
        &d("Sli\\za~", Divadi),
        Lun,
        &["ASlikzat", "ASlizat"],
    );
}

#[test]
fn sutra_3_1_47() {
    assert_has_tip(&[], &d("df\\Si~r", Bhvadi), Lun, &["adarSat", "adrAkzIt"]);
}

#[test]
fn sutra_3_1_48() {
    assert_has_tip(&[], &nic(&d("qukf\\Y", Tanadi)), Lun, &["acIkarat"]);
    assert_has_tip(&[], &nic(&d("hf\\Y", Bhvadi)), Lun, &["ajIharat"]);
    assert_has_tip(&[], &d("SriY", Bhvadi), Lun, &["aSiSriyat"]);
    assert_has_tip(&[], &d("dru\\", Bhvadi), Lun, &["adudruvat"]);
    assert_has_tip(&[], &d("sru\\", Bhvadi), Lun, &["asusruvat"]);
}

#[test]
fn sutra_3_1_49() {
    assert_has_tip(&[], &d("De\\w", Bhvadi), Lun, &["adaDat", "aDAt", "aDAsIt"]);
    assert_has_tip(
        &[],
        &d("wuo~Svi", Bhvadi),
        Lun,
        &["aSiSviyat", "aSvat", "aSvayIt"],
    );
}

#[test]
fn sutra_3_1_50() {
    let gup = d("gupU~", Bhvadi);
    let t = Tester::with_chaandasa();
    t.assert_has_thas(
        &[],
        &gup,
        Lun,
        &["ajugupatam", "agOptam", "agopizwam", "agopAyizwam"],
    );
}

#[test]
fn sutra_3_1_52() {
    let asu = d("asu~", Divadi);
    assert_has_ta(&["pari"], &asu, Lun, &["paryAsTata"]);
    assert_has_aataam(&["pari"], &asu, Lun, &["paryAsTetAm"]);
    assert_has_jha(&["pari"], &asu, Lun, &["paryAsTanta"]);

    let vac = d("va\\ca~", Adadi);
    assert_has_tip(&[], &vac, Lun, &["avocat"]);
    assert_has_tas(&[], &vac, Lun, &["avocatAm"]);
    assert_has_jhi(&[], &vac, Lun, &["avocan"]);

    let khya = d("KyA\\", Adadi);
    assert_has_tip(&[], &khya, Lun, &["aKyat"]);
    assert_has_tas(&[], &khya, Lun, &["aKyatAm"]);
    assert_has_jhi(&[], &khya, Lun, &["aKyan"]);
}

#[test]
fn sutra_3_1_53() {
    assert_has_tip(&[], &d("li\\pa~^", Tudadi), Lun, &["alipat"]);
    assert_has_tip(&[], &d("zi\\ca~^", Tudadi), Lun, &["asicat"]);
    assert_has_tip(&[], &d("hve\\Y", Bhvadi), Lun, &["ahvat"]);
}

#[test]
fn sutra_3_1_54() {
    assert_has_ta(&[], &d("li\\pa~^", Tudadi), Lun, &["alipata", "alipta"]);
    assert_has_ta(&[], &d("zi\\ca~^", Tudadi), Lun, &["asicata", "asikta"]);
    assert_has_ta(&[], &d("hve\\Y", Bhvadi), Lun, &["ahvata", "ahvAsta"]);
}

#[test]
fn sutra_3_1_55() {
    assert_has_tip(&[], &d("pu\\za~", Divadi), Lun, &["apuzat"]);
    assert_has_tip(&[], &d("dyuta~\\", Bhvadi), Lun, &["adyutat"]);
    assert_has_tip(&[], &d("SvitA~\\", Bhvadi), Lun, &["aSvitat"]);
    assert_has_tip(&[], &d("ga\\mx~", Bhvadi), Lun, &["agamat"]);
    assert_has_tip(&[], &d("Sa\\kx~", Svadi), Lun, &["aSakat"]);
    // parasmEpadezu
    assert_has_ta(&["vi"], &d("dyuta~\\", Bhvadi), Lun, &["vyadyotizwa"]);
    assert_has_ta(&[], &d("luwa~\\", Bhvadi), Lun, &["alowizwa"]);
}

#[test]
fn sutra_3_1_56() {
    assert_has_tip(&[], &d("sf\\", Juhotyadi), Lun, &["asarat"]);
    assert_has_tip(&[], &d("SAsu~", Adadi), Lun, &["aSizat"]);
    assert_has_tip(&[], &d("f\\", Juhotyadi), Lun, &["Arat"]);
    assert_has_ta(&["sam"], &d("f\\", Juhotyadi), Lun, &["samArata"]);
}

#[test]
fn sutra_3_1_57() {
    assert_has_tip(&[], &d("Bi\\di~^r", Rudhadi), Lun, &["aBidat", "aBEtsIt"]);
    assert_has_tip(&[], &d("Ci\\di~^r", Rudhadi), Lun, &["acCidat", "acCEtsIt"]);
    // parasmEpadezu
    assert_has_ta(&[], &d("Bi\\di~^r", Rudhadi), Lun, &["aBitta"]);
    assert_has_ta(&[], &d("Ci\\di~^r", Rudhadi), Lun, &["acCitta"]);
}

#[test]
fn sutra_3_1_58() {
    assert_has_tip(&[], &d("jF", Kryadi), Lun, &["ajarat", "ajArIt"]);
    assert_has_tip(&[], &d("stanBu~", Kryadi), Lun, &["astaBat", "astamBIt"]);
    assert_has_tip(&[], &d("mrucu~", Bhvadi), Lun, &["amrucat", "amrocIt"]);
    assert_has_tip(&[], &d("mlucu~", Bhvadi), Lun, &["amlucat", "amlocIt"]);
    assert_has_tip(&[], &d("grucu~", Bhvadi), Lun, &["agrucat", "agrocIt"]);
    assert_has_tip(&[], &d("glucu~", Bhvadi), Lun, &["aglucat", "aglocIt"]);
    assert_has_tip(&[], &d("gluncu~", Bhvadi), Lun, &["aglucat", "agluYcIt"]);
    assert_has_tip(
        &[],
        &d("wuo~Svi", Bhvadi),
        Lun,
        &["aSvat", "aSvayIt", "aSiSviyat"],
    );
}

#[ignore]
#[test]
fn sutra_3_1_59() {
    let t = Tester::with_chaandasa();
    t.assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lun, &["akarat"]);
    t.assert_has_tip(&[], &d("mf\\N", Tudadi), Lun, &["amarat"]);
    t.assert_has_tip(&[], &d("dF", Kryadi), Lun, &["adarat"]);
    t.assert_has_tip(&["AN"], &d("ru\\ha~", Bhvadi), Lun, &["Aruhat"]);
}

#[test]
fn sutra_3_1_60() {
    let pad = d("pa\\da~\\", Divadi);
    assert_has_ta(&["ud"], &pad, Lun, &["udapAdi"]);
    assert_has_ta(&["sam"], &pad, Lun, &["samapAdi"]);
    // te?
    assert_has_aataam(&["ud"], &pad, Lun, &["udapatsAtAm"]);
    assert_has_jha(&["ud"], &pad, Lun, &["udapatsata"]);
}

#[test]
fn sutra_3_1_61() {
    assert_has_ta_k(&[], &d("dIpI~\\", Divadi), Lun, &["adIpi", "adIpizwa"]);
    assert_has_ta_k(&[], &d("janI~\\", Divadi), Lun, &["ajani", "ajanizwa"]);
    assert_has_ta_k(&[], &d("buDa~", Bhvadi), Lun, &["aboDi", "aboDizwa"]);
    assert_has_ta_k(&[], &d("pUrI~\\", Divadi), Lun, &["apUri", "apUrizwa"]);
    assert_has_ta_k(&[], &d("tAyf~\\", Bhvadi), Lun, &["atAyi", "atAyizwa"]);
    assert_has_ta_k(&[], &d("o~pyAyI~\\", Bhvadi), Lun, &["apyAyi", "apyAyizwa"]);
}

#[test]
fn sutra_3_1_66() {
    assert_has_ta_k(&[], &d("SE\\", Bhvadi), Lun, &["aSAyi"]);
    assert_has_ta_k(&[], &d("qukf\\Y", Tanadi), Lun, &["akAri"]);
    assert_has_ta_k(&[], &d("hf\\Y", Bhvadi), Lun, &["ahAri"]);
}

#[test]
fn sutra_3_1_67() {
    assert_has_ta_k(&[], &d("Asa~\\", Adadi), Lat, &["Asyate"]);
    assert_has_ta_k(&[], &d("SIN", Adadi), Lat, &["Sayyate"]);
    assert_has_ta_k(&[], &d("qukf\\Y", Tanadi), Lat, &["kriyate"]);
    assert_has_ta_k(&[], &d("ga\\mx~", Bhvadi), Lat, &["gamyate"]);
    assert_has_ta_k(&[], &d("qupa\\ca~^z", Bhvadi), Lat, &["pacyate"]);
}

#[test]
fn sutra_3_1_68() {
    assert_has_lat(&[], &d("BU", Bhvadi), &["Bavati"]);
    assert_has_lat(&[], &d("qupa\\ca~^z", Bhvadi), &["pacati", "pacate"]);
}

#[test]
fn sutra_3_1_69() {
    assert_has_lat(&[], &d("divu~", Divadi), &["dIvyati"]);
    assert_has_lat(&[], &d("zivu~", Divadi), &["sIvyati"]);
}

#[test]
fn sutra_3_1_70() {
    assert_has_lat(&[], &d("wuBrASf~\\", Bhvadi), &["BrASyate", "BrASate"]);
    assert_has_lat(&[], &d("wuBlASf~\\", Bhvadi), &["BlASyate", "BlASate"]);
    assert_has_lat(&[], &d("Bramu~", Divadi), &["BrAmyati", "Bramati"]);
    assert_has_tip(&[], &d("kramu~", Bhvadi), Lat, &["krAmyati", "krAmati"]);
    assert_has_lat(&[], &d("klamu~", Divadi), &["klAmyati", "klAmati"]);
    assert_has_lat(&[], &d("trasI~", Divadi), &["trasyati", "trasati"]);
    assert_has_lat(&[], &d("truwa~", Tudadi), &["truwyati", "truwati"]);
    assert_has_tip(&[], &d("laza~^", Bhvadi), Lat, &["lazyati", "lazati"]);
}

#[test]
fn sutra_3_1_71() {
    let yas = d("yasu~", Divadi);
    assert_has_lat(&[], &yas, &["yasyati", "yasati"]);
    assert_has_lat(&["AN"], &yas, &["Ayasyati"]);
    assert_has_lat(&["pra"], &yas, &["prayasyati"]);
}

#[test]
fn sutra_3_1_72() {
    let yas = d("yasu~", Divadi);
    assert_has_lat(&["sam"], &yas, &["saMyasyati", "saMyasati"]);
}

#[test]
fn sutra_3_1_73() {
    assert_has_tip(&[], &d("zu\\Y", Svadi), Lat, &["sunoti"]);
    assert_has_tip(&[], &d("zi\\Y", Svadi), Lat, &["sinoti"]);
}

#[test]
fn sutra_3_1_74() {
    assert_has_tip(&[], &d("Sru\\", Bhvadi), Lat, &["SfRoti"]);
}

#[test]
fn sutra_3_1_75() {
    assert_has_tip(&[], &d("akzU~", Bhvadi), Lat, &["akzRoti", "akzati"]);
}

#[test]
fn sutra_3_1_76() {
    assert_has_tip(&[], &d("takzU~", Bhvadi), Lat, &["takzRoti", "takzati"]);
}

#[test]
fn sutra_3_1_77() {
    assert_has_tip(&[], &d("tu\\da~^", Tudadi), Lat, &["tudati"]);
    assert_has_tip(&[], &d("Ru\\da~^", Tudadi), Lat, &["nudati"]);
}

#[test]
fn sutra_3_1_78() {
    assert_has_tip(&[], &d("ru\\Di~^r", Rudhadi), Lat, &["ruRadDi"]);
    assert_has_tip(&[], &d("Bi\\di~^r", Rudhadi), Lat, &["Binatti"]);
}

#[test]
fn sutra_3_1_79() {
    assert_has_tip(&[], &d("tanu~^", Tanadi), Lat, &["tanoti"]);
    assert_has_tip(&[], &d("zaRu~^", Tanadi), Lat, &["sanoti"]);
    assert_has_tip(&[], &d("kzaRu~^", Tanadi), Lat, &["kzaRoti"]);
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lat, &["karoti"]);

    assert_has_ta(&[], &d("qukf\\Y", Tanadi), Lun, &["akfta"]);
}

#[test]
fn sutra_3_1_80() {
    assert_has_lat(&[], &d("Divi~", Bhvadi), &["Dinoti"]);
    assert_has_lat(&[], &d("kfvi~", Bhvadi), &["kfRoti"]);
}

#[test]
fn sutra_3_1_81() {
    assert_has_tip(&[], &d("qukfI\\Y", Kryadi), Lat, &["krIRAti"]);
    assert_has_tip(&[], &d("prI\\Y", Kryadi), Lat, &["prIRAti"]);
}

#[test]
fn sutra_3_1_82() {
    assert_has_lat(&[], &d("stanBu~", Kryadi), &["staBnAti", "staBnoti"]);
    assert_has_lat(&[], &d("stunBu~", Kryadi), &["stuBnAti", "stuBnoti"]);
    assert_has_lat(&[], &d("skanBu~", Kryadi), &["skaBnAti", "skaBnoti"]);
    assert_has_lat(&[], &d("skunBu~", Kryadi), &["skuBnAti", "skuBnoti"]);
    assert_has_tip(&[], &d("sku\\Y", Kryadi), Lat, &["skunAti", "skunoti"]);
}

#[test]
fn sutra_3_1_83() {
    let muz = d("muza~", Kryadi);
    assert_has_sip(&[], &muz, Lot, &["muzARa", "muzRItAt"]);
    assert_has_sip(&[], &d("puza~", Kryadi), Lot, &["puzARa", "puzRItAt"]);
    // halaH
    assert_has_sip(&[], &d("qukrI\\Y", Kryadi), Lot, &["krIRIhi", "krIRItAt"]);
    // hO
    assert_has_tip(&[], &muz, Lat, &["muzRAti"]);
}

#[test]
fn sutra_3_1_91() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
}

#[test]
fn sutra_3_1_92() {
    // example: 3.2.1
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("kumBa", &[], &kf, Krt::aR, &["kumBakAra"]);

    // TODO: stamberama, karRejapa

    // example: 3.2.97
    let jan = d("janI~\\", Divadi);
    assert_has_upapada_krdanta("upasara", &[], &jan, Krt::qa, &["upasaraja"]);
    assert_has_upapada_krdanta("mandura", &[], &jan, Krt::qa, &["manduraja"]);
}

#[test]
fn sutra_3_1_93() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
    assert_has_tip(&[], &d("ci\\Y", Svadi), AshirLin, &["cIyAt"]);
    assert_has_tip(&[], &d("zwu\\Y", Adadi), AshirLin, &["stUyAt"]);
}

// krt-pratyayas
// -------------

#[test]
fn sutra_3_1_94() {
    let kzip = d("kzi\\pa~^", Tudadi);
    assert_has_krdanta(&["vi"], &kzip, Krt::Rvul, &["vikzepaka"]);
    assert_has_krdanta(&["vi"], &kzip, Krt::tfc, &["vikzeptf"]);
    assert_has_krdanta(&["vi"], &kzip, Krt::ka, &["vikzipa"]);

    // asarUpa?
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_upapada_krdanta("go", &[], &daa, Krt::ka, &["goda"]);
    assert_has_upapada_krdanta("kambala", &[], &daa, Krt::ka, &["kambalada"]);

    // striyAm?
    assert_has_krdanta(&[], &san(&d("qukf\\Y", Tanadi)), Krt::a, &["cikIrzA"]);
    assert_has_krdanta(&[], &san(&d("hf\\Y", Bhvadi)), Krt::a, &["jihIrzA"]);
}

#[test]
fn sutra_3_1_95() {
    // No examples from KV.
}

#[test]
fn sutra_3_1_96() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
}

#[test]
fn sutra_3_1_96_v1() {
    let vas = d("va\\sa~", Bhvadi);
    assert_has_krdanta(&[], &vas, Krt::tavyat, &["vastavya", "vAstavya"]);
}

#[test]
fn sutra_3_1_96_v2() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::kelimar, &["pacelima"]);
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::kelimar, &["Bidelima"]);
}

#[test]
fn sutra_3_1_97() {
    assert_has_krdanta(&[], &d("gE\\", Bhvadi), Krt::yat, &["geya"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), Krt::yat, &["peya"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::yat, &["ceya"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::yat, &["jeya"]);

    // ajanta-bhUta-pUrva
    assert_has_krdanta(&[], &san(&d("do\\", Divadi)), Krt::yat, &["ditsya"]);
    assert_has_krdanta(&[], &san(&d("quDA\\Y", Juhotyadi)), Krt::yat, &["Ditsya"]);
}

#[test]
fn sutra_3_1_97_v1() {
    // TODO: not sure which "tak" or "Sas"
    assert_has_krdanta(&[], &d("taka~", Bhvadi), Krt::yat, &["takya"]);
    assert_has_krdanta(&[], &d("Sasu~\\", Bhvadi), Krt::yat, &["Sasya"]);
    assert_has_krdanta(&[], &d("cate~^", Bhvadi), Krt::yat, &["catya"]);
    assert_has_krdanta(&[], &d("yatI~\\", Bhvadi), Krt::yat, &["yatya"]);
    assert_has_krdanta(&[], &d("janI~\\", Divadi), Krt::yat, &["janya"]);
}

#[test]
fn sutra_3_1_97_v2() {
    let han = d("ha\\na~", Adadi);
    assert_has_krdanta(&[], &han, Krt::yat, &["vaDya"]);
    assert_has_krdanta(&[], &han, Krt::Ryat, &["GAtya"]);
}

#[test]
fn sutra_3_1_98() {
    assert_has_krdanta(&[], &d("Sa\\pa~^", Bhvadi), Krt::yat, &["Sapya"]);
    assert_has_krdanta(&[], &d("qula\\Ba~^z", Bhvadi), Krt::yat, &["laBya"]);
    // poH
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::yat, &[]);
    assert_has_krdanta(&[], &pac, Krt::Ryat, &["pAkya", "pAcya"]);
    let vac = d("va\\ca~", Adadi);
    assert_has_krdanta(&[], &vac, Krt::yat, &[]);
    assert_has_krdanta(&[], &vac, Krt::Ryat, &["vAkya", "vAcya"]);
    // ad-upaDAt
    let kup = d("kupa~", Divadi);
    assert_has_krdanta(&[], &kup, Krt::yat, &[]);
    assert_has_krdanta(&[], &kup, Krt::Ryat, &["kopya"]);
    let gup = d("gupa~", Divadi);
    assert_has_krdanta(&[], &gup, Krt::yat, &[]);
    assert_has_krdanta(&[], &gup, Krt::Ryat, &["gopya"]);
    let aap = d("A\\px~", Svadi);
    assert_has_krdanta(&[], &aap, Krt::yat, &[]);
    assert_has_krdanta(&[], &aap, Krt::Ryat, &["Apya"]);
}

#[test]
fn sutra_3_1_99() {
    assert_has_krdanta(&[], &d("Sa\\kx~", Svadi), Krt::yat, &["Sakya"]);
    assert_has_krdanta(&[], &d("zaha~\\", Bhvadi), Krt::yat, &["sahya"]);
}

#[test]
fn sutra_3_1_100() {
    let gad = d("gada~", Bhvadi);
    let mad = d("madI~", Divadi);
    let car = d("cara~", Bhvadi);
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_krdanta(&[], &gad, Krt::yat, &["gadya"]);
    assert_has_krdanta(&[], &mad, Krt::yat, &["madya"]);
    assert_has_krdanta(&[], &car, Krt::yat, &["carya"]);
    assert_has_krdanta(&[], &yam, Krt::yat, &["yamya"]);
    assert_has_krdanta(&["pra"], &gad, Krt::yat, &[]);
    assert_has_krdanta(&["pra"], &gad, Krt::Ryat, &["pragAdya"]);
    assert_has_krdanta(&["pra"], &mad, Krt::yat, &[]);
    assert_has_krdanta(&["pra"], &mad, Krt::Ryat, &["pramAdya"]);
}

#[test]
fn sutra_3_1_100_v1() {
    let car = d("cara~", Bhvadi);
    assert_has_krdanta(&["AN"], &car, Krt::yat, &["Acarya"]);
    assert_has_krdanta(&["AN"], &car, Krt::Ryat, &["AcArya"]);
}

#[test]
fn sutra_3_1_102() {
    let vah = d("va\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::yat, &["vahya"]);
    assert_has_krdanta(&[], &vah, Krt::Ryat, &["vAhya"]);
}

#[test]
fn sutra_3_1_103() {
    let f = d("f\\", Bhvadi);
    assert_has_krdanta(&[], &f, Krt::yat, &["arya"]);
    assert_has_krdanta(&[], &f, Krt::Ryat, &["Arya"]);
}

#[test]
fn sutra_3_1_104() {
    let sf = d("sf\\", Bhvadi);
    assert_has_krdanta(&["upa"], &sf, Krt::yat, &["upasaryA"]);
    assert_has_krdanta(&["upa"], &sf, Krt::Ryat, &["upasArya"]);
}

#[test]
fn sutra_3_1_106() {
    let vad = d("vada~", Bhvadi);
    assert_has_upapada_krdanta("brahma", &[], &vad, Krt::yat, &["brahmavadya"]);
    assert_has_upapada_krdanta("brahma", &[], &vad, Krt::kyap, &["brahmodya"]);
}

#[test]
fn sutra_3_1_107() {
    let bhu = d("BU", Bhvadi);
    assert_has_upapada_krdanta("brahma", &[], &bhu, Krt::kyap, &["brahmaBUya"]);
    assert_has_upapada_krdanta("deva", &[], &bhu, Krt::kyap, &["devaBUya"]);
    // supi?
    assert_has_krdanta(&[], &bhu, Krt::yat, &["Bavya"]);
    // an-upasarge?
    assert_has_krdanta(&["pra"], &bhu, Krt::yat, &["praBavya"]);
}

#[test]
fn sutra_3_1_108() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("brahma", &[], &han, Krt::kyap, &["brahmahatya"]);
    assert_has_upapada_krdanta("aSva", &[], &han, Krt::kyap, &["aSvahatya"]);

    // anupasargAt?
    assert_has_krdanta(&["pra"], &han, Krt::kyap, &[]);
    assert_has_krdanta(&["pra"], &han, Krt::GaY, &["praGAta"]);

    // supi?
    assert_has_krdanta(&[], &han, Krt::GaY, &["GAta"]);
    // TODO: block GAtya
    // assert_has_krdanta(&[], &han, Krt::Ryat, &[]);
}

#[test]
fn sutra_3_1_109() {
    // ityA is from 3.3.99.
    assert_has_krdanta(&[], &d("i\\R", Adadi), Krt::kyap, &["itya", "ityA"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::kyap, &["stutya"]);
    assert_has_krdanta(&[], &d("SAsu~", Adadi), Krt::kyap, &["Sizya"]);
    assert_has_krdanta(&[], &d("vfY", Svadi), Krt::kyap, &["vftya"]);
    assert_has_krdanta(&["AN"], &d("df", Svadi), Krt::kyap, &["Adftya"]);
    assert_has_krdanta(&[], &d("juzI~\\", Tudadi), Krt::kyap, &["juzya"]);

    // blocks Ryat
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::Ryat, &[]);

    // vfY, not vfN
    let vrnite = &d("vfN", Kryadi);
    assert_has_krdanta(&[], &vrnite, Krt::kyap, &[]);
    assert_has_krdanta(&[], &vrnite, Krt::Ryat, &["vArya"]);
}

#[test]
fn sutra_3_1_110() {
    assert_has_krdanta(&[], &d("vftu~\\", Bhvadi), Krt::kyap, &["vftya"]);
    assert_has_krdanta(&[], &d("vfDu~\\", Bhvadi), Krt::kyap, &["vfDya"]);
    // a-kxpi-cfteh?
    assert_has_krdanta(&[], &d("kfpU~\\", Bhvadi), Krt::kyap, &[]);
    assert_has_krdanta(&[], &d("cftI~", Tudadi), Krt::kyap, &[]);
}

#[test]
fn sutra_3_1_111() {
    assert_has_krdanta(&[], &d("Kanu~^", Bhvadi), Krt::kyap, &["Keya"]);
}

#[test]
fn sutra_3_1_112() {
    let bhf = d("Bf\\Y", Bhvadi);
    // BftyA is from 3.3.99.
    assert_has_krdanta(&[], &bhf, Krt::kyap, &["Bftya", "BftyA"]);
    assert_has_krdanta(&[], &bhf, Krt::Ryat, &["BArya"]);
}

#[test]
fn sutra_3_1_113() {
    let mfj = d("mfjU~", Adadi);
    assert_has_krdanta(&["pari"], &mfj, Krt::kyap, &["parimfjya"]);
    assert_has_krdanta(&["pari"], &mfj, Krt::Ryat, &["parimArgya", "parimArjya"]);
}

#[test]
fn sutra_3_1_120() {
    let kf = d("qukf\\Y", Tanadi);
    // kftyA (strI) by 3.3.100.
    assert_has_krdanta(&[], &kf, Krt::kyap, &["kftya", "kftyA"]);
    assert_has_krdanta(&[], &kf, Krt::Ryat, &["kArya"]);
    let vfz = d("vfzu~", Bhvadi);
    assert_has_krdanta(&[], &vfz, Krt::kyap, &["vfzya"]);
    assert_has_krdanta(&[], &vfz, Krt::Ryat, &["varzya"]);
}

#[test]
fn sutra_3_1_124() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::Ryat, &["kArya"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::Ryat, &["hArya"]);
    assert_has_krdanta(&[], &d("Df\\Y", Bhvadi), Krt::Ryat, &["DArya"]);
    assert_has_krdanta(&[], &d("va\\ca~", Adadi), Krt::Ryat, &["vAkya", "vAcya"]);
    assert_has_krdanta(
        &[],
        &d("qupa\\ca~^z", Bhvadi),
        Krt::Ryat,
        &["pAkya", "pAcya"],
    );
}

#[test]
fn sutra_3_1_125() {
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::Ryat, &["lAvya"]);
    assert_has_krdanta(&[], &lu, Krt::yat, &["lavya"]);
    let pu = d("pUY", Kryadi);
    assert_has_krdanta(&[], &pu, Krt::Ryat, &["pAvya"]);
    assert_has_krdanta(&[], &pu, Krt::yat, &["pavya"]);
}

#[test]
fn sutra_3_1_126() {
    assert_has_krdanta(&["AN"], &d("zu\\Y", Svadi), Krt::Ryat, &["AsAvya"]);
    assert_has_krdanta(&[], &d("yu", Adadi), Krt::Ryat, &["yAvya"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), Krt::Ryat, &["vApya"]);
    assert_has_krdanta(&[], &d("rapa~", Bhvadi), Krt::Ryat, &["rApya"]);
    assert_has_krdanta(&[], &d("lapa~", Adadi), Krt::Ryat, &["lApya"]);
    assert_has_krdanta(&[], &d("trapU~\\z", Bhvadi), Krt::Ryat, &["trApya"]);
    assert_has_krdanta(&["AN"], &d("camu~", Bhvadi), Krt::Ryat, &["AcAmya"]);
}

#[test]
fn sutra_3_1_133() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::Rvul, &["kAraka"]);
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kartf"]);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&[], &hf, Krt::Rvul, &["hAraka"]);
    assert_has_krdanta(&[], &hf, Krt::tfc, &["hartf"]);
}

#[test]
fn sutra_3_1_135() {
    let kzip = &d("kzi\\pa~^", Tudadi);
    assert_has_krdanta(&["vi"], &kzip, Krt::ka, &["vikzipa"]);
    assert_has_krdanta(&[], &d("bu\\Da~\\", Divadi), Krt::ka, &["buDa"]);
    assert_has_krdanta(&[], &d("kfSa~", Divadi), Krt::ka, &["kfSa"]);
    assert_has_krdanta(&[], &d("jYA\\", Kryadi), Krt::ka, &["jYa"]);
    assert_has_krdanta(&[], &d("prI\\Y", Kryadi), Krt::ka, &["priya"]);
    assert_has_krdanta(&[], &d("kF", Tudadi), Krt::ka, &["kira"]);

    // vAsarUpa-vidhi
    assert_has_krdanta(&[], &kzip, Krt::Rvul, &["kzepaka"]);
    assert_has_krdanta(&[], &kzip, Krt::tfc, &["kzeptf"]);
}

#[ignore]
#[test]
fn sutra_3_1_136() {
    assert_has_krdanta(&["pra"], &d("zWA\\", Bhvadi), Krt::ka, &["prasTa"]);
    assert_has_krdanta(&["su"], &d("glE\\", Bhvadi), Krt::ka, &["sugla"]);
    assert_has_krdanta(&["su"], &d("mlE\\", Bhvadi), Krt::ka, &["sumla"]);
}

#[test]
fn sutra_3_1_137() {
    let paa = &d("pA\\", Bhvadi);
    assert_has_krdanta(&["ud"], &paa, Krt::Sa, &["utpiba"]);
    assert_has_krdanta(&["vi"], &paa, Krt::Sa, &["vipiba"]);

    let ghra = &d("GrA\\", Bhvadi);
    assert_has_krdanta(&["ud"], &ghra, Krt::Sa, &["ujjiGra"]);
    assert_has_krdanta(&["vi"], &ghra, Krt::Sa, &["vijiGra"]);

    let dhma = &d("DmA\\", Bhvadi);
    assert_has_krdanta(&["ud"], &dhma, Krt::Sa, &["udDama"]);
    assert_has_krdanta(&["vi"], &dhma, Krt::Sa, &["viDama"]);

    let dhe = &d("De\\w", Bhvadi);
    assert_has_krdanta(&["ud"], &dhe, Krt::Sa, &["udDaya"]);
    assert_has_krdanta(&["vi"], &dhe, Krt::Sa, &["viDaya"]);

    let dfs = &d("df\\Si~r", Bhvadi);
    assert_has_krdanta(&["ud"], &dfs, Krt::Sa, &["utpaSya"]);
    assert_has_krdanta(&["vi"], &dfs, Krt::Sa, &["vipaSya"]);
}

#[test]
fn sutra_3_1_138() {
    let lip = d("li\\pa~^", Tudadi);
    assert_has_krdanta(&[], &lip, Krt::Sa, &["limpa"]);
    assert_has_krdanta(&[], &d("vi\\dx~^", Tudadi), Krt::Sa, &["vinda"]);
    assert_has_krdanta(&[], &nic(&d("Df\\Y", Bhvadi)), Krt::Sa, &["DAraya"]);
    assert_has_krdanta(&[], &d("pF", Curadi), Krt::Sa, &["pAraya"]);
    assert_has_krdanta(&[], &nic(&d("vida~", Bhvadi)), Krt::Sa, &["vedaya"]);
    assert_has_krdanta(&["ud"], &nic(&d("ejf~\\", Bhvadi)), Krt::Sa, &["udejaya"]);
    assert_has_krdanta(&[], &d("cita~", Curadi), Krt::Sa, &["cetaya"]);
    assert_has_krdanta(&[], &d("sAti", Bhvadi), Krt::Sa, &["sAtaya"]);
    assert_has_krdanta(&[], &nic(&d("zaha~\\", Bhvadi)), Krt::Sa, &["sAhaya"]);

    // anupasargAt?
    assert_has_krdanta(&["pra"], &lip, Krt::Sa, &[]);
    assert_has_krdanta(&["pra"], &lip, Krt::ka, &["pralipa"]);
}

#[test]
fn sutra_3_1_139() {
    let da = d("qudA\\Y", Juhotyadi);
    let dha = d("quDA\\Y", Juhotyadi);
    assert_has_krdanta(&[], &da, Krt::Sa, &["dada"]);
    assert_has_krdanta(&[], &da, Krt::Ra, &["dAya"]);
    assert_has_krdanta(&[], &dha, Krt::Sa, &["daDa"]);
    assert_has_krdanta(&[], &dha, Krt::Ra, &["DAya"]);

    assert_has_krdanta(&["pra"], &da, Krt::Sa, &[]);
    assert_has_krdanta(&["pra"], &dha, Krt::Sa, &[]);
    assert_has_krdanta(&["pra"], &da, Krt::ka, &["prada"]);
    assert_has_krdanta(&["pra"], &dha, Krt::ka, &["praDa"]);
}

#[test]
fn sutra_3_1_140() {
    assert_has_krdanta(&[], &d("jvala~", Bhvadi), Krt::Ra, &["jvAla"]);
    assert_has_krdanta(&[], &d("jvala~", Bhvadi), Krt::ac, &["jvala"]);
    assert_has_krdanta(&[], &d("cala~", Bhvadi), Krt::Ra, &["cAla"]);
    assert_has_krdanta(&[], &d("cala~", Bhvadi), Krt::ac, &["cala"]);

    // anupasargAt?
    assert_has_krdanta(&["pra"], &d("jvala~", Bhvadi), Krt::Ra, &[]);
    assert_has_krdanta(&["pra"], &d("jvala~", Bhvadi), Krt::ac, &["prajvala"]);
}

#[test]
fn sutra_3_1_140_v1() {
    assert_has_krdanta(&["ava"], &d("tanu~^", Tanadi), Krt::Ra, &["avatAna"]);
    assert_has_krdanta(&["ava"], &d("tanu~^", Tanadi), Krt::ac, &[]);
}

#[test]
fn sutra_3_1_141() {
    assert_has_krdanta(&["ava"], &d("SyE\\N", Bhvadi), Krt::Ra, &["avaSyAya"]);
    assert_has_krdanta(&["prati"], &d("SyE\\N", Bhvadi), Krt::Ra, &["pratiSyAya"]);
    assert_has_krdanta(&[], &d("qudA\\Y", Juhotyadi), Krt::Ra, &["dAya"]);
    assert_has_krdanta(&[], &d("quDA\\Y", Juhotyadi), Krt::Ra, &["DAya"]);
    assert_has_krdanta(&[], &d("vya\\Da~", Divadi), Krt::Ra, &["vyADa"]);
    assert_has_krdanta(&["AN"], &d("sru\\", Bhvadi), Krt::Ra, &["AsrAva"]);
    assert_has_krdanta(&["sam"], &d("sru\\", Bhvadi), Krt::Ra, &["saMsrAva"]);
    assert_has_krdanta(&["ati"], &d("i\\R", Adadi), Krt::Ra, &["atyAya"]);
    assert_has_krdanta(&["ava"], &d("zo\\", Divadi), Krt::Ra, &["avasAya"]);
    assert_has_krdanta(&["ava"], &d("hf\\Y", Bhvadi), Krt::Ra, &["avahAra"]);
    assert_has_krdanta(&[], &d("li\\ha~^", Adadi), Krt::Ra, &["leha"]);
    assert_has_krdanta(&[], &d("Sli\\za~", Divadi), Krt::Ra, &["Sleza"]);
    assert_has_krdanta(&[], &d("Svasa~", Adadi), Krt::Ra, &["SvAsa"]);
}

#[test]
fn sutra_3_1_142() {
    assert_has_krdanta(&[], &d("wudu\\", Svadi), Krt::Ra, &["dAva"]);
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), Krt::Ra, &["nAya"]);

    // anupasarge?
    assert_has_krdanta(&["pra"], &d("wudu\\", Svadi), Krt::Ra, &[]);
    assert_has_krdanta(&["pra"], &d("wudu\\", Svadi), Krt::ac, &["pradava"]);
    assert_has_krdanta(&["pra"], &d("RI\\Y", Bhvadi), Krt::Ra, &[]);
    assert_has_krdanta(&["pra"], &d("RI\\Y", Bhvadi), Krt::ac, &["praRaya"]);
}

#[test]
fn sutra_3_1_143() {
    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&[], &grah, Krt::Ra, &["grAha"]);
    assert_has_krdanta(&[], &grah, Krt::ac, &["graha"]);
}

#[test]
fn sutra_3_1_144() {
    assert_has_krdanta(&[], &d("graha~^", Kryadi), Krt::ka, &["gfha"]);
}

#[test]
fn sutra_3_1_145() {
    // TODO: change examples to nartakI, etc.
    assert_has_krdanta(&[], &d("nftI~", Divadi), Krt::zvun, &["nartaka"]);
    assert_has_krdanta(&[], &d("Kanu~^", Bhvadi), Krt::zvun, &["Kanaka"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Bhvadi), Krt::zvun, &["rajaka"]);
}

#[test]
fn sutra_3_1_146() {
    assert_has_krdanta(&[], &d("gE\\", Bhvadi), Krt::Takan, &["gATaka"]);
    // TODO: gATikA
}

#[test]
fn sutra_3_1_147() {
    assert_has_krdanta(&[], &d("gE\\", Bhvadi), Krt::Ryuw, &["gAyana"]);
    // TODO: gAyanI
}

#[test]
fn sutra_3_1_148() {
    assert_has_krdanta(&[], &d("o~hA\\N", Juhotyadi), Krt::Ryuw, &["hAyana"]);
    assert_has_krdanta(&[], &d("o~hA\\N", Juhotyadi), Krt::Ryuw, &["hAyana"]);
}

#[test]
fn sutra_3_1_149() {
    assert_has_krdanta(&[], &d("pru\\N", Bhvadi), Krt::vun, &["pravaka"]);
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), Krt::vun, &["saraka"]);
    assert_has_krdanta(&[], &d("lUY", Kryadi), Krt::vun, &["lavaka"]);
}

#[test]
fn sutra_3_1_150() {
    assert_has_krdanta(&[], &d("jIva~", Bhvadi), Krt::vun, &["jIvaka"]);
    assert_has_krdanta(&[], &d("wunadi~", Bhvadi), Krt::vun, &["nandaka"]);
}
