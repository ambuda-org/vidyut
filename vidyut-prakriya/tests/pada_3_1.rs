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

fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan])
}

#[test]
fn sutra_3_1_1() {
    let kf = Dhatu::new("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
}

#[test]
fn sutra_3_1_2() {
    let kf = Dhatu::new("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya"]);
    // TODO: tEtttirIya
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
    assert_has_lat_a(&[], &d("dAna~^", Bhvadi), &["dIdAMsate"]);
    assert_has_lat_a(&[], &d("SAna~^", Bhvadi), &["SISAMsate"]);

    // TODO: mAnayati, etc.
}

#[test]
fn sutra_3_1_7() {
    let san = |u, gana| Dhatu::new(u, gana).with_sanadi(&[Sanadi::San]);
    let kf = san("qukf\\Y", Tanadi);
    let hf = san("hf\\Y", Bhvadi);

    assert_has_lat_p(&[], &kf, &["cikIrzati"]);
    assert_has_lat_p(&[], &hf, &["jihIrzati"]);
    assert_has_lan_p(&["pra"], &kf, &["prAcikIrzat"]);

    assert_has_lat_p(&[], &san("patx~", Bhvadi), &["pipatizati", "pitsati"]);
    assert_has_lat_p(&[], &san("mf\\N", Tudadi), &["mumUrzati"]);
}

#[test]
fn sutra_3_1_22() {
    assert_has_lat(&[], &yan(&d("qupa\\ca~^z", Bhvadi)), &["pApacyate"]);
    assert_has_lat(&[], &yan(&d("ya\\ja~^", Bhvadi)), &["yAyajyate"]);
    assert_has_lat(&[], &yan(&d("jvala~", Bhvadi)), &["jAjvalyate"]);
    assert_has_lat(&[], &yan(&d("dIpI~\\", Divadi)), &["dedIpyate"]);
}

#[ignore]
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

#[test]
fn sutra_3_1_25() {
    assert_has_lat_p(&[], &d("cura~", Curadi), &["corayati"]);
    assert_has_lat_p(&[], &d("citi~", Curadi), &["cintayati"]);
}

#[test]
fn sutra_3_1_26() {
    let san = |u, gana| Dhatu::new(u, gana).with_sanadi(&[Sanadi::Nic]);
    assert_has_lat_p(&[], &san("qukf\\Y", Tanadi), &["kArayati"]);
    assert_has_lat_p(&[], &san("qupa\\ca~^z", Bhvadi), &["pAcayati"]);

    // TODO: add others
}

#[test]
fn sutra_3_1_28() {
    assert_has_lat_p(&[], &d("gupU~", Bhvadi), &["gopAyati"]);
    assert_has_lat_p(&[], &d("DUpa~", Bhvadi), &["DUpAyati"]);
    assert_has_lat_p(&[], &d("viCa~", Tudadi), &["vicCAyati"]);
    assert_has_lat_p(&[], &d("paRa~\\", Bhvadi), &["paRAyati"]);
    assert_has_lat_p(&[], &d("pana~\\", Bhvadi), &["panAyati"]);
}

#[test]
fn sutra_3_1_29() {
    assert_has_lat_a(&[], &Dhatu::new("fti", Bhvadi), &["ftIyate"]);
}

#[test]
fn sutra_3_1_30() {
    assert_has_lat_a(&[], &Dhatu::new("kamu~\\", Bhvadi), &["kAmayate"]);
}

#[test]
fn sutra_3_1_31() {
    let assert_has_tfc = |u, gana, expected| {
        assert_has_krdanta(&[], &Dhatu::new(u, gana), Krt::tfc, expected);
    };
    assert_has_tfc("gupU~", Bhvadi, &["goptf", "gopitf", "gopAyitf"]);
    assert_has_tfc("fti", Bhvadi, &["artitf", "ftIyitf"]);
    assert_has_tfc("kamu~\\", Bhvadi, &["kamitf", "kAmayitf"]);
}

#[test]
fn sutra_3_1_33() {
    let kf = Dhatu::new("qukf\\Y", Tanadi);
    assert_has_lrt_p(&[], &kf, &["karizyati"]);
    assert_has_lrn_p(&[], &kf, &["akarizyat"]);
    assert_has_lut(&[], &kf, &["kartA"]);
    assert_has_lut(&[], &Dhatu::new("ma\\na~\\", Divadi), &["mantA"]);
    assert_has_lut(&["sam"], &Dhatu::new("ga\\mx~", Bhvadi), &["saNgantA"]);
}

#[test]
fn sutra_3_1_44() {
    assert_has_lun_p(&[], &d("qukf\\Y", Tanadi), &["akArzIt"]);
    assert_has_lun_p(&[], &d("hf\\Y", Bhvadi), &["ahArzIt"]);
    assert_has_lun_p(
        &[],
        &d("spf\\Sa~", Tudadi),
        &["asprAkzIt", "aspArkzIt", "aspfkzat"],
    );
    assert_has_lun_p(
        &[],
        &d("mf\\Sa~", Tudadi),
        &["amrAkzIt", "amArkzIt", "amfkzat"],
    );
    assert_has_lun_p(
        &[],
        &d("kf\\za~", Bhvadi),
        &["akArkzIt", "akrAkzIt", "akfkzat"],
    );
    assert_has_lun_p(
        &[],
        &d("tf\\pa~", Divadi),
        &["atrApsIt", "atArpsIt", "atfpat", "atarpIt"],
    );
    assert_has_lun_p(
        &[],
        &d("df\\pa~", Divadi),
        &["adrApsIt", "adArpsIt", "adfpat", "adarpIt"],
    );
}

#[test]
fn sutra_3_1_45() {
    assert_has_lun_p(&[], &d("du\\ha~^", Adadi), &["aDukzat"]);
    assert_has_lun_p(&[], &d("li\\ha~^", Adadi), &["alikzat"]);
    // SalaH
    assert_has_lun_p(&[], &d("Bi\\di~^r", Rudhadi), &["aBEtsIt", "aBidat"]);
    assert_has_lun_p(&[], &d("Ci\\di~^r", Rudhadi), &["acCEtsIt", "acCidat"]);
    // igupaDAt
    assert_has_lun_p(&[], &d("da\\ha~", Bhvadi), &["aDAkzIt"]);
    // aniwaH
    assert_has_lun_p(&[], &d("kuza~", Kryadi), &["akozIt"]);
    assert_has_lun_p(&[], &d("muza~", Kryadi), &["amozIt"]);
}

#[test]
fn sutra_3_1_46() {
    assert_has_lun_p(&["AN"], &d("Sli\\za~", Divadi), &["ASlikzat", "ASlizat"]);
}

#[test]
fn sutra_3_1_47() {
    assert_has_lun_p(&[], &d("df\\Si~r", Bhvadi), &["adarSat", "adrAkzIt"]);
}

#[test]
fn sutra_3_1_48() {
    assert_has_lun_p(&[], &nic(&d("qukf\\Y", Tanadi)), &["acIkarat"]);
    assert_has_lun_p(&[], &nic(&d("hf\\Y", Bhvadi)), &["ajIharat"]);
    assert_has_lun_p(&[], &d("SriY", Bhvadi), &["aSiSriyat"]);
    assert_has_lun_p(&[], &d("dru\\", Bhvadi), &["adudruvat"]);
    assert_has_lun_p(&[], &d("sru\\", Bhvadi), &["asusruvat"]);
}

#[test]
fn sutra_3_1_49() {
    assert_has_lun_p(&[], &d("De\\w", Bhvadi), &["adaDat", "aDAt", "aDAsIt"]);
    assert_has_lun_p(
        &[],
        &d("wuo~Svi", Bhvadi),
        &["aSiSviyat", "aSvat", "aSvayIt"],
    );
}

#[ignore]
#[test]
fn sutra_3_1_52() {
    let asu = d("asu~", Divadi);
    assert_has_parasmai_tinanta(&["pari"], &asu, Lun, Prathama, Eka, &["paryAsTata"]);
    assert_has_parasmai_tinanta(&["pari"], &asu, Lun, Prathama, Dvi, &["paryAsTetAm"]);
    assert_has_parasmai_tinanta(&["pari"], &asu, Lun, Prathama, Bahu, &["paryAsTanta"]);

    let vac = d("va\\ca~", Adadi);
    assert_has_parasmai_tinanta(&[], &vac, Lun, Prathama, Eka, &["avocat"]);
    assert_has_parasmai_tinanta(&[], &vac, Lun, Prathama, Dvi, &["avocatAm"]);
    assert_has_parasmai_tinanta(&[], &vac, Lun, Prathama, Bahu, &["avocan"]);

    let khya = d("KyA\\", Adadi);
    assert_has_parasmai_tinanta(&[], &khya, Lun, Prathama, Eka, &["aKyat"]);
    assert_has_parasmai_tinanta(&[], &khya, Lun, Prathama, Dvi, &["aKyatAm"]);
    assert_has_parasmai_tinanta(&[], &khya, Lun, Prathama, Bahu, &["aKyan"]);
}

#[test]
fn sutra_3_1_53() {
    assert_has_lun_p(&[], &d("li\\pa~^", Tudadi), &["alipat"]);
    assert_has_lun_p(&[], &d("zi\\ca~^", Tudadi), &["asicat"]);
    assert_has_lun_p(&[], &d("hve\\Y", Bhvadi), &["ahvat"]);
}

#[test]
fn sutra_3_1_54() {
    assert_has_lun_a(&[], &d("li\\pa~^", Tudadi), &["alipata", "alipta"]);
    assert_has_lun_a(&[], &d("zi\\ca~^", Tudadi), &["asicata", "asikta"]);
    assert_has_lun_a(&[], &d("hve\\Y", Bhvadi), &["ahvata", "ahvAsta"]);
}

#[test]
fn sutra_3_1_55() {
    assert_has_lun_p(&[], &d("pu\\za~", Divadi), &["apuzat"]);
    assert_has_lun_p(&[], &d("dyuta~\\", Bhvadi), &["adyutat"]);
    assert_has_lun_p(&[], &d("SvitA~\\", Bhvadi), &["aSvitat"]);
    assert_has_lun_p(&[], &d("ga\\mx~", Bhvadi), &["agamat"]);
    assert_has_lun_p(&[], &d("Sa\\kx~", Svadi), &["aSakat"]);
    // parasmEpadezu
    assert_has_lun_a(&["vi"], &d("dyuta~\\", Bhvadi), &["vyadyotizwa"]);
    assert_has_lun_a(&[], &d("luwa~\\", Bhvadi), &["alowizwa"]);
}

#[test]
fn sutra_3_1_56() {
    assert_has_lun_p(&[], &d("sf\\", Bhvadi), &["asarat"]);
    assert_has_lun_p(&[], &d("SAsu~", Adadi), &["aSizat"]);
    assert_has_lun_p(&[], &d("f\\", Bhvadi), &["Arat"]);
    assert_has_lun_a(&["sam"], &d("f\\", Bhvadi), &["samArata"]);
}

#[test]
fn sutra_3_1_57() {
    assert_has_lun_p(&[], &d("Bi\\di~^r", Rudhadi), &["aBidat", "aBEtsIt"]);
    assert_has_lun_p(&[], &d("Ci\\di~^r", Rudhadi), &["acCidat", "acCEtsIt"]);
    // parasmEpadezu
    assert_has_lun_a(&[], &d("Bi\\di~^r", Rudhadi), &["aBitta"]);
    assert_has_lun_a(&[], &d("Ci\\di~^r", Rudhadi), &["acCitta"]);
}

#[test]
fn sutra_3_1_58() {
    assert_has_lun_p(&[], &d("jF", Kryadi), &["ajarat", "ajArIt"]);
    // TODO: astamBIt?
    // assert_has_lun_p(&[], &d("", Kryadi), &["astaBat", "astamBIt"])
    assert_has_lun_p(&[], &d("mrucu~", Bhvadi), &["amrucat", "amrocIt"]);
    assert_has_lun_p(&[], &d("mlucu~", Bhvadi), &["amlucat", "amlocIt"]);
    assert_has_lun_p(&[], &d("grucu~", Bhvadi), &["agrucat", "agrocIt"]);
    assert_has_lun_p(&[], &d("glucu~", Bhvadi), &["aglucat", "aglocIt"]);
    assert_has_lun_p(&[], &d("gluncu~", Bhvadi), &["aglucat", "agluYcIt"]);
    assert_has_lun_p(
        &[],
        &d("wuo~Svi", Bhvadi),
        &["aSvat", "aSvayIt", "aSiSviyat"],
    );
}

#[test]
fn sutra_3_1_61() {
    assert_has_lun_karmani(&[], &d("dIpI~\\", Divadi), &["adIpi", "adIpizwa"]);
    assert_has_lun_karmani(&[], &d("janI~\\", Divadi), &["ajani", "ajanizwa"]);
    assert_has_lun_karmani(&[], &d("buDa~", Bhvadi), &["aboDi", "aboDizwa"]);
    assert_has_lun_karmani(&[], &d("pUrI~\\", Divadi), &["apUri", "apUrizwa"]);
    assert_has_lun_karmani(&[], &d("tAyf~\\", Bhvadi), &["atAyi", "atAyizwa"]);
    assert_has_lun_karmani(&[], &d("o~pyAyI~\\", Bhvadi), &["apyAyi", "apyAyizwa"]);
}

#[test]
fn sutra_3_1_67() {
    assert_has_lat_karmani(&[], &d("Asa~\\", Adadi), &["Asyate"]);
    assert_has_lat_karmani(&[], &d("SIN", Adadi), &["Sayyate"]);
    assert_has_lat_karmani(&[], &d("qukf\\Y", Tanadi), &["kriyate"]);
    assert_has_lat_karmani(&[], &d("ga\\mx~", Bhvadi), &["gamyate"]);
    assert_has_lat_karmani(&[], &d("qupa\\ca~^z", Bhvadi), &["pacyate"]);
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
    assert_has_lat_p(&[], &d("kramu~", Bhvadi), &["krAmyati", "krAmati"]);
    assert_has_lat(&[], &d("klamu~", Divadi), &["klAmyati", "klAmati"]);
    assert_has_lat(&[], &d("trasI~", Divadi), &["trasyati", "trasati"]);
    assert_has_lat(&[], &d("truwa~", Tudadi), &["truwyati", "truwati"]);
    assert_has_lat_p(&[], &d("laza~^", Bhvadi), &["lazyati", "lazati"]);
}

#[test]
fn sutra_3_1_71() {
    let yas = Dhatu::new("yasu~", Divadi);
    assert_has_lat(&[], &yas, &["yasyati", "yasati"]);
    assert_has_lat(&["AN"], &yas, &["Ayasyati"]);
    assert_has_lat(&["pra"], &yas, &["prayasyati"]);
}

#[test]
fn sutra_3_1_72() {
    let yas = Dhatu::new("yasu~", Divadi);
    assert_has_lat(&["sam"], &yas, &["saMyasyati", "saMyasati"]);
}

#[test]
fn sutra_3_1_73() {
    assert_has_lat_p(&[], &d("zu\\Y", Svadi), &["sunoti"]);
    assert_has_lat_p(&[], &d("zi\\Y", Svadi), &["sinoti"]);
}

#[test]
fn sutra_3_1_74() {
    assert_has_lat_p(&[], &d("Sru\\", Bhvadi), &["SfRoti"]);
}

#[test]
fn sutra_3_1_75() {
    assert_has_lat_p(&[], &d("akzU~", Bhvadi), &["akzRoti", "akzati"]);
}

#[test]
fn sutra_3_1_76() {
    assert_has_lat_p(&[], &d("takzU~", Bhvadi), &["takzRoti", "takzati"]);
}

#[test]
fn sutra_3_1_77() {
    assert_has_lat_p(&[], &d("tu\\da~^", Tudadi), &["tudati"]);
    assert_has_lat_p(&[], &d("Ru\\da~^", Tudadi), &["nudati"]);
}

#[test]
fn sutra_3_1_78() {
    assert_has_lat_p(&[], &d("ru\\Di~^r", Rudhadi), &["ruRadDi"]);
    assert_has_lat_p(&[], &d("Bi\\di~^r", Rudhadi), &["Binatti"]);
}

#[test]
fn sutra_3_1_79() {
    assert_has_lat_p(&[], &d("tanu~^", Tanadi), &["tanoti"]);
    assert_has_lat_p(&[], &d("zaRu~^", Tanadi), &["sanoti"]);
    assert_has_lat_p(&[], &d("kzaRu~^", Tanadi), &["kzaRoti"]);
    assert_has_lat_p(&[], &d("qukf\\Y", Tanadi), &["karoti"]);

    assert_has_lun_a(&[], &d("qukf\\Y", Tanadi), &["akfta"]);
}

#[test]
fn sutra_3_1_80() {
    assert_has_lat(&[], &d("Divi~", Bhvadi), &["Dinoti"]);
    assert_has_lat(&[], &d("kfvi~", Bhvadi), &["kfRoti"]);
}

#[test]
fn sutra_3_1_81() {
    assert_has_lat_p(&[], &d("qukfI\\Y", Kryadi), &["krIRAti"]);
    assert_has_lat_p(&[], &d("prI\\Y", Kryadi), &["prIRAti"]);
}

#[test]
fn sutra_3_1_82() {
    assert_has_lat(&[], &d("sta\\nBu~", Kryadi), &["staBnAti", "staBnoti"]);
    assert_has_lat(&[], &d("stu\\nBu~", Kryadi), &["stuBnAti", "stuBnoti"]);
    assert_has_lat(&[], &d("ska\\nBu~", Kryadi), &["skaBnAti", "skaBnoti"]);
    assert_has_lat(&[], &d("sku\\nBu~", Kryadi), &["skuBnAti", "skuBnoti"]);
    assert_has_lat_p(&[], &d("sku\\Y", Kryadi), &["skunAti", "skunoti"]);
}

#[test]
fn sutra_3_1_83() {
    let muz = Dhatu::new("muza~", Kryadi);
    assert_has_parasmai_tinanta(&[], &muz, Lot, Madhyama, Eka, &["muzARa", "muzRItAt"]);
    assert_has_parasmai_tinanta(
        &[],
        &d("puza~", Kryadi),
        Lot,
        Madhyama,
        Eka,
        &["puzARa", "puzRItAt"],
    );
    // halaH
    assert_has_parasmai_tinanta(
        &[],
        &d("qukrI\\Y", Kryadi),
        Lot,
        Madhyama,
        Eka,
        &["krIRIhi", "krIRItAt"],
    );
    // hO
    assert_has_parasmai_tinanta(&[], &muz, Lat, Prathama, Eka, &["muzRAti"]);
}

// krt-pratyayas
// -------------

#[test]
fn sutra_3_1_96() {
    let kf = Dhatu::new("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
}

#[test]
fn sutra_3_1_96_v1() {
    let vas = Dhatu::new("va\\sa~", Bhvadi);
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
fn sutra_3_1_111() {
    assert_has_krdanta(&[], &d("Kanu~^", Bhvadi), Krt::kyap, &["Keya"]);
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
    assert_has_krdanta(&[], &kf, Krt::kyap, &["kftya"]);
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
fn sutra_3_1_133() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::Rvul, &["kAraka"]);
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kartf"]);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&[], &hf, Krt::Rvul, &["hAraka"]);
    assert_has_krdanta(&[], &hf, Krt::tfc, &["hartf"]);
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

#[ignore]
#[test]
fn sutra_3_1_138() {
    assert_has_krdanta(&[], &d("li\\pa~^", Tudadi), Krt::Sa, &["limpa"]);
    assert_has_krdanta(&[], &d("vi\\dx~^", Tudadi), Krt::Sa, &["vinda"]);
    assert_has_krdanta(&[], &d("", Bhvadi), Krt::Sa, &["DAraya"]);
    assert_has_krdanta(&[], &d("pF", Curadi), Krt::Sa, &["pAraya"]);
    assert_has_krdanta(&[], &d("vida~", Bhvadi), Krt::Sa, &["vedaya"]);
    assert_has_krdanta(&["ud"], &d("ejf~\\", Bhvadi), Krt::Sa, &["udejaya"]);
    assert_has_krdanta(&[], &d("cita~", Curadi), Krt::Sa, &["cetaya"]);
    assert_has_krdanta(&[], &d("sAti", Bhvadi), Krt::Sa, &["sAtaya"]);
    assert_has_krdanta(&[], &d("zaha~\\", Bhvadi), Krt::Sa, &["sAhaya"]);
}

#[test]
fn sutra_3_1_142() {
    assert_has_krdanta(&[], &d("wudu\\", Svadi), Krt::Ra, &["dAva"]);
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), Krt::Ra, &["nAya"]);
    // anupasarge
    assert_has_krdanta(&["pra"], &d("wudu\\", Svadi), Krt::Ra, &[]);
    assert_has_krdanta(&["pra"], &d("RI\\Y", Bhvadi), Krt::Ra, &[]);
    // TODO: others
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
