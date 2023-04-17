extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti as V;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan])
}

fn nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic])
}

fn prati(text: &str) -> Pratipadika {
    Pratipadika::builder().text(text).build().unwrap()
}

fn assert_has_hi(prefixes: &[&str], d: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, &d, Lot, Madhyama, Eka, expected);
}

fn assert_has_vas(prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, &d, la, Uttama, Dvi, expected);
}

fn assert_has_mas(prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, &d, la, Uttama, Bahu, expected);
}

#[test]
fn sutra_6_4_2() {
    assert_has_krdanta(&[], &d("hve\\Y", Bhvadi), Krt::kta, &["hUta"]);
    assert_has_krdanta(&[], &d("jyA\\", Kryadi), Krt::kta, &["jIna"]);
    assert_has_krdanta(&["sam"], &d("vye\\Y", Bhvadi), Krt::kta, &["saMvIta"]);
    // halaH
    let ve = d("ve\\Y", Bhvadi);
    assert_has_krdanta(&[], &ve, Krt::kta, &["uta"]);
    assert_has_krdanta(&[], &ve, Krt::ktavatu, &["utavat"]);
    // aNgAvayavAt
    assert_has_krdanta(&["nir"], &ve, Krt::kta, &["niruta"]);
    // tadantasya
    assert_has_krdanta(&[], &d("vya\\Da~", Divadi), Krt::kta, &["vidDa"]);
    assert_has_krdanta(&[], &d("vyaca~", Tudadi), Krt::kta, &["vicita"]);
}

#[test]
fn sutra_6_4_3() {
    use Vibhakti::*;
    assert_has_subantas("agni", Pum, Sasthi, Bahu, &["agnInAm"]);
    assert_has_subantas("vAyu", Pum, Sasthi, Bahu, &["vAyUnAm"]);
    assert_has_subantas("kartf", Pum, Sasthi, Bahu, &["kartFRAm"]);
    assert_has_subantas("hartf", Pum, Sasthi, Bahu, &["hartFRAm"]);
}

#[test]
fn sutra_6_4_4() {
    use Vibhakti::*;
    assert_has_subantas("tisf", Stri, Sasthi, Bahu, &["tisfRAm"]);
    assert_has_subantas("catasf", Stri, Sasthi, Bahu, &["catasfRAm"]);
}

#[test]
fn sutra_6_4_6() {
    use Vibhakti::*;
    assert_has_subantas("nf", Pum, Sasthi, Bahu, &["nFRAm", "nfRAm"]);
}

#[test]
fn sutra_6_4_7() {
    use Vibhakti::*;
    assert_has_subantas("paYcan", Pum, Sasthi, Bahu, &["paYcAnAm"]);
    assert_has_subantas("saptan", Pum, Sasthi, Bahu, &["saptAnAm"]);
    assert_has_subantas("navan", Pum, Sasthi, Bahu, &["navAnAm"]);
    assert_has_subantas("daSan", Pum, Sasthi, Bahu, &["daSAnAm"]);
    // naH
    assert_has_subantas("catur", Pum, Sasthi, Bahu, &["caturRAm"]);
    // nAmi
    assert_has_subantas("carman", Pum, Sasthi, Bahu, &["carmaRAm"]);
}

#[test]
fn sutra_6_4_8() {
    use Vibhakti::*;
    assert_has_subantas("rAjan", Pum, Prathama, Eka, &["rAjA"]);
    assert_has_subantas("rAjan", Pum, Prathama, Dvi, &["rAjAnO"]);
    assert_has_subantas("rAjan", Pum, Prathama, Bahu, &["rAjAnaH"]);
    assert_has_subantas("rAjan", Pum, Dvitiya, Eka, &["rAjAnam"]);
    assert_has_subantas("rAjan", Pum, Dvitiya, Dvi, &["rAjAnO"]);
    assert_has_subantas("sAman", Napumsaka, Prathama, Bahu, &["sAmAni"]);
    assert_has_subantas("sAman", Napumsaka, Dvitiya, Bahu, &["sAmAni"]);
    // sarvanAmasTAne
    assert_has_subantas("rAjan", Pum, Saptami, Eka, &["rAjani", "rAjYi"]);
    assert_has_subantas("sAman", Pum, Saptami, Eka, &["sAmani", "sAmni"]);
    // asambudDO
    assert_has_subantas("rAjan", Pum, Sambodhana, Eka, &["rAjan"]);
    assert_has_subantas("takzan", Pum, Sambodhana, Eka, &["takzan"]);
}

#[ignore]
#[test]
fn sutra_6_4_10() {
    use Vibhakti::*;
    assert_has_subantas("Sreyas", Pum, Prathama, Eka, &["SreyAn"]);
    assert_has_subantas("Sreyas", Pum, Prathama, Dvi, &["SreyAMsO"]);
    assert_has_subantas("Sreyas", Pum, Prathama, Bahu, &["SreyAMsaH"]);
    assert_has_subantas("Sreyas", Napumsaka, Prathama, Bahu, &["SreyAMsi"]);
    assert_has_subantas("payas", Napumsaka, Prathama, Bahu, &["payAMsi"]);
    assert_has_subantas("yaSas", Napumsaka, Prathama, Bahu, &["yaSAMsi"]);
    // mahat
    assert_has_subantas("mahat", Pum, Prathama, Eka, &["mahAn"]);
    assert_has_subantas("mahat", Pum, Prathama, Dvi, &["mahAntO"]);
    assert_has_subantas("mahat", Pum, Prathama, Bahu, &["mahAntaH"]);
    // asambudDo
    assert_has_subantas("Sreyas", Pum, Sambodhana, Eka, &["Sreyan"]);
    assert_has_subantas("mahat", Pum, Sambodhana, Eka, &["mahan"]);
}

#[test]
fn sutra_6_4_15() {
    let sham = d("Samu~", Divadi);
    assert_has_krdanta(&[], &sham, Krt::kta, &["SAnta"]);
    assert_has_krdanta(&[], &sham, Krt::ktavatu, &["SAntavat"]);
    assert_has_krdanta(&[], &sham, Krt::ktvA, &["SAntvA", "SamitvA"]);
    assert_has_krdanta(&[], &sham, Krt::ktin, &["SAnti"]);
}

#[ignore]
#[test]
fn sutra_6_4_16() {
    assert_has_lat(&[], &san(&d("vI\\", Adadi)), &["vivIzati"]);
    assert_has_lat_p(&[], &san(&d("zwu\\Y", Adadi)), &["tuzwUzati"]);
    assert_has_lat_p(&[], &san(&d("qukf\\Y", Tanadi)), &["cikIrzati"]);
    assert_has_lat_p(&[], &san(&d("hf\\Y", Bhvadi)), &["jihIrzati"]);
    assert_has_lat(&[], &san(&d("ha\\na~", Adadi)), &["jiGAMsati"]);
    assert_has_lat(&["aDi"], &san(&d("i\\N", Adadi)), &["aDijigaMsate"]);
}

#[test]
fn sutra_6_4_17() {
    assert_has_lat_p(
        &[],
        &san(&d("tanu~^", Tanadi)),
        &["titanizati", "titaMsati", "titAMsati"],
    );
}

#[test]
fn sutra_6_4_18() {
    let kram = d("kramu~", Bhvadi);
    assert_has_krdanta(&[], &kram, Krt::ktvA, &["krantvA", "krAntvA", "kramitvA"]);
    assert_has_krdanta(&["pra"], &kram, Krt::ktvA, &["prakramya"]);
    assert_has_krdanta(&["upa"], &kram, Krt::ktvA, &["upakramya"]);
}

#[test]
fn sutra_6_4_19() {
    let prach = d("pra\\Ca~", Tudadi);
    let vich = d("viCa~", Tudadi);
    assert_has_krdanta(&[], &prach, Krt::naN, &["praSna"]);
    assert_has_krdanta(&[], &vich, Krt::naN, &["viSna"]);
    // TODO: kvip, unadi
    assert_has_krdanta(&[], &prach, Krt::kta, &["pfzwa"]);
    assert_has_krdanta(&[], &prach, Krt::ktavatu, &["pfzwavat"]);
    assert_has_krdanta(&[], &prach, Krt::ktvA, &["pfzwvA"]);
    let div = d("divu~", Divadi);
    assert_has_krdanta(&[], &div, Krt::kta, &["dyUta", "dyUna"]);
    assert_has_krdanta(&[], &div, Krt::ktavatu, &["dyUtavat", "dyUnavat"]);
    assert_has_krdanta(&[], &div, Krt::ktvA, &["dyUtvA", "devitvA"]);
}

#[test]
fn sutra_6_4_20() {
    let jvar = d("jvara~", Bhvadi);
    assert_has_krdanta(&[], &jvar, Krt::kvip, &["jUr"]);
    assert_has_krdanta(&[], &jvar, Krt::ktin, &["jUrti"]);

    let tvar = d("YitvarA~\\", Bhvadi);
    assert_has_krdanta(&[], &tvar, Krt::kvip, &["tUr"]);
    assert_has_krdanta(&[], &tvar, Krt::ktin, &["tUrti"]);

    let sriv = d("srivu~", Divadi);
    assert_has_krdanta(&[], &sriv, Krt::kvip, &["srU"]);
    assert_has_krdanta(&[], &sriv, Krt::kta, &["srUta"]);
    assert_has_krdanta(&[], &sriv, Krt::ktavatu, &["srUtavat"]);
    assert_has_krdanta(&[], &sriv, Krt::ktin, &["srUti"]);

    let av = d("ava~", Bhvadi);
    assert_has_krdanta(&[], &av, Krt::kvip, &["U"]);
    assert_has_krdanta(&[], &av, Krt::ktin, &["Uti"]);

    let mav = d("mava~", Bhvadi);
    assert_has_krdanta(&[], &mav, Krt::kvip, &["mU"]);
    // TODO: why mUta and not mavita?
    // assert_has_krdanta(&[], &mav, Krt::kta, &["mUta"]);
    // assert_has_krdanta(&[], &mav, Krt::ktavatu, &["mUtavat"]);
    // assert_has_krdanta(&[], &mav, Krt::ktin, &["mUti"]);
}

#[test]
fn sutra_6_4_21() {
    let murch = d("murCA~", Bhvadi);
    assert_has_krdanta(&[], &murch, Krt::kvip, &["mur"]);
    // TODO: why mUrta and not mUrRa?
    // assert_has_krdanta(&[], &murch, Krt::kta, &["mUrta"]);
    // assert_has_krdanta(&[], &murch, Krt::ktavatu, &["mUrtavat"]);
    assert_has_krdanta(&[], &murch, Krt::ktin, &["mUrti"]);

    let hurch = d("hurCA~", Bhvadi);
    assert_has_krdanta(&[], &hurch, Krt::kvip, &["hur"]);
    assert_has_krdanta(&[], &hurch, Krt::kta, &["hUrRa"]);
    assert_has_krdanta(&[], &hurch, Krt::ktavatu, &["hUrRavat"]);
    assert_has_krdanta(&[], &hurch, Krt::ktin, &["hUrti"]);

    let turv = d("turvI~", Bhvadi);
    assert_has_krdanta(&[], &turv, Krt::kvip, &["tur"]);
    assert_has_krdanta(&[], &turv, Krt::kta, &["tUrRa"]);
    assert_has_krdanta(&[], &turv, Krt::ktavatu, &["tUrRavat"]);
    assert_has_krdanta(&[], &turv, Krt::ktin, &["tUrti"]);

    let dhurv = d("DurvI~", Bhvadi);
    assert_has_krdanta(&[], &dhurv, Krt::kvip, &["Dur"]);
    assert_has_krdanta(&[], &dhurv, Krt::kta, &["DUrRa"]);
    assert_has_krdanta(&[], &dhurv, Krt::ktavatu, &["DUrRavat"]);
    assert_has_krdanta(&[], &dhurv, Krt::ktin, &["DUrti"]);
}

#[test]
fn sutra_6_4_23() {
    use Vibhakti::*;
    assert_has_lat(&[], &d("anjU~", Rudhadi), &["anakti"]);
    assert_has_lat(&[], &d("Ba\\njo~", Rudhadi), &["Banakti"]);
    assert_has_lat(&[], &d("hisi~", Rudhadi), &["hinasti"]);

    // TODO: make these proper tests and model the dhatu.
    assert_has_subantas("yajYa", Pum, Sasthi, Bahu, &["yajYAnAm"]);
    assert_has_subantas("yatna", Pum, Sasthi, Bahu, &["yatnAnAm"]);
    assert_has_subantas("viSna", Pum, Sasthi, Bahu, &["viSnAnAm"]);
    assert_has_subantas("praSna", Pum, Sasthi, Bahu, &["praSnAnAm"]);
}

#[test]
fn sutra_6_4_24() {
    let sras = d("sransu~\\", Bhvadi);
    let dhvas = d("Dvansu~\\", Bhvadi);

    assert_has_krdanta(&[], &sras, Krt::kta, &["srasta"]);
    assert_has_krdanta(&[], &dhvas, Krt::kta, &["Dvasta"]);
    assert_has_lat_karmani(&[], &sras, &["srasyate"]);
    assert_has_lat_karmani(&[], &dhvas, &["Dvasyate"]);
    assert_has_lat(&[], &yan(&sras), &["sanIsrasyate"]);
    assert_has_lat(&[], &yan(&dhvas), &["danIDvasyate"]);

    let nand = d("wunadi~", Bhvadi);
    assert_has_lat_karmani(&[], &nand, &["nandyate"]);
    assert_has_lat(&[], &yan(&nand), &["nAnandyate"]);

    let ni = d("RI\\Y", Bhvadi);
    assert_has_lat_karmani(&[], &ni, &["nIyate"]);
    assert_has_lat(&[], &yan(&ni), &["nenIyate"]);

    let nah = d("Ra\\ha~^", Divadi);
    assert_has_lat_karmani(&[], &nah, &["nahyate"]);
    assert_has_lat(&[], &yan(&nah), &["nAnahyate"]);

    assert_has_krdanta(&[], &sras, Krt::tfc, &["sraMsitf"]);
    assert_has_krdanta(&[], &dhvas, Krt::tfc, &["DvaMsitf"]);

    // TODO: varttikas
}

#[test]
fn sutra_6_4_25() {
    assert_has_lat(&[], &d("da\\nSa~", Bhvadi), &["daSati"]);
    assert_has_lat(&[], &d("za\\nja~", Bhvadi), &["sajati"]);
    assert_has_lat(&["pari"], &d("zva\\nja~\\", Bhvadi), &["parizvajate"]);
}

#[test]
fn sutra_6_4_26() {
    assert_has_lat_p(&[], &d("ra\\nja~^", Bhvadi), &["rajati"]);
}

#[test]
fn sutra_6_4_26_v2() {
    let ranj_nic = nic(&d("ra\\nja~^", Divadi));
    assert_has_lat_p(&[], &ranj_nic, &["rajayati", "raYjayati"]);
}

#[test]
fn sutra_6_4_26_v3() {
    let ranj = d("ra\\nja~^", Bhvadi);
    assert_has_krdanta(&[], &ranj, Krt::GinuR, &["rAgin"]);
}

#[test]
fn sutra_6_4_27() {
    let ranj = d("ra\\nja~^", Bhvadi);
    assert_has_krdanta(&[], &ranj, Krt::GaY, &["rAga", "raNga"]);
}

#[ignore]
#[test]
fn sutra_6_4_28() {
    let syand = d("syandU~\\", Bhvadi);
    assert_has_krdanta(&[], &syand, Krt::GaY, &["syada", "syanda"]);
}

#[test]
fn sutra_6_4_31() {
    assert_has_krdanta(
        &[],
        &d("ska\\ndi~r", Bhvadi),
        Krt::ktvA,
        &["skantvA", "skanttvA"],
    );
    assert_has_krdanta(
        &[],
        &d("syandU~\\", Bhvadi),
        Krt::ktvA,
        &["syantvA", "syanttvA", "syanditvA"],
    );
}

#[test]
fn sutra_6_4_32() {
    assert_has_krdanta(
        &[],
        &d("ra\\nja~^", Bhvadi),
        Krt::ktvA,
        &["raNktvA", "raktvA"],
    );
    assert_has_krdanta(
        &[],
        &d("Ba\\njo~", Bhvadi),
        Krt::ktvA,
        &["BaNktvA", "BaktvA"],
    );
    assert_has_krdanta(
        &[],
        &d("Ra\\Sa~", Divadi),
        Krt::ktvA,
        &["naMzwvA", "nazwvA", "naSitvA"],
    );
}

#[test]
fn sutra_6_4_33() {
    assert_has_lun_karmani(&[], &d("Ba\\njo~", Rudhadi), &["aBAji", "aBaYji"]);
}

#[test]
fn sutra_6_4_34() {
    let shas = d("SAsu~", Adadi);
    assert_has_tinanta(&["anu"], &shas, Lun, Prathama, Eka, &["anvaSizat"]);
    assert_has_tinanta(&["anu"], &shas, Lun, Prathama, Dvi, &["anvaSizatAm"]);
    assert_has_tinanta(&["anu"], &shas, Lun, Prathama, Bahu, &["anvaSizan"]);
    // Niti
    assert_has_tinanta(&[], &shas, Lat, Uttama, Dvi, &["SizvaH"]);
    assert_has_tinanta(&[], &shas, Lat, Uttama, Bahu, &["SizmaH"]);
    // aNhaloH
    assert_has_tinanta(&[], &shas, Lat, Prathama, Bahu, &["SAsati"]);
    assert_has_tinanta(&[], &shas, Lit, Prathama, Dvi, &["SaSAsatuH"]);
    assert_has_tinanta(&[], &shas, Lit, Prathama, Bahu, &["SaSAsuH"]);
    // Not A-SAs
    let ashas = d("SAsu~\\", Adadi);
    assert_has_tinanta(&["AN"], &ashas, Lat, Prathama, Eka, &["ASAste"]);

    // TODO: others
}

#[test]
fn sutra_6_4_35() {
    let shas = d("SAsu~", Adadi);
    assert_has_tinanta(
        &["anu"],
        &shas,
        Lot,
        Madhyama,
        Eka,
        &["anuSADi", "anuSizwAt"],
    );
    assert_has_tinanta(
        &["pra"],
        &shas,
        Lot,
        Madhyama,
        Eka,
        &["praSADi", "praSizwAt"],
    );
}

#[test]
fn sutra_6_4_36() {
    assert_has_parasmai_tinanta(
        &[],
        &d("ha\\na~", Adadi),
        Lot,
        Madhyama,
        Eka,
        &["jahi", "hatAt"],
    );
}

#[test]
fn sutra_6_4_37() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_krdanta(&[], &yam, Krt::ktvA, &["yatvA"]);
    assert_has_krdanta(&[], &yam, Krt::kta, &["yata"]);
    assert_has_krdanta(&[], &yam, Krt::ktavatu, &["yatavat"]);
    assert_has_krdanta(&[], &yam, Krt::ktin, &["yati"]);
    let ram = d("ra\\ma~", Bhvadi);
    assert_has_krdanta(&[], &ram, Krt::ktvA, &["ratvA"]);
    assert_has_krdanta(&[], &ram, Krt::kta, &["rata"]);
    assert_has_krdanta(&[], &ram, Krt::ktavatu, &["ratavat"]);
    assert_has_krdanta(&[], &ram, Krt::ktin, &["rati"]);
    let van = d("vana~", Bhvadi);
    assert_has_krdanta(&[], &van, Krt::ktin, &["vati"]);
    // TODO: ktic test
    let tan = d("tanu~^", Tanadi);
    assert_has_krdanta(&[], &tan, Krt::kta, &["tata"]);
    assert_has_krdanta(&[], &tan, Krt::ktavatu, &["tatavat"]);
    let kzan = d("kzaRu~^", Tanadi);
    assert_has_krdanta(&[], &kzan, Krt::kta, &["kzata"]);
    assert_has_krdanta(&[], &kzan, Krt::ktavatu, &["kzatavat"]);
    let rn = d("fRu~^", Tanadi);
    assert_has_krdanta(&[], &rn, Krt::kta, &["fta"]);
    assert_has_krdanta(&[], &rn, Krt::ktavatu, &["ftavat"]);
    let tfn = d("tfRu~^", Tanadi);
    assert_has_krdanta(&[], &tfn, Krt::kta, &["tfta"]);
    assert_has_krdanta(&[], &tfn, Krt::ktavatu, &["tftavat"]);
    let ghfn = d("GfRu~^", Tanadi);
    assert_has_krdanta(&[], &ghfn, Krt::kta, &["Gfta"]);
    assert_has_krdanta(&[], &ghfn, Krt::ktavatu, &["Gftavat"]);
    let van = d("vanu~\\", Tanadi);
    assert_has_krdanta(&[], &van, Krt::kta, &["vata"]);
    assert_has_krdanta(&[], &van, Krt::ktavatu, &["vatavat"]);
    let man = d("manu~\\", Tanadi);
    assert_has_krdanta(&[], &man, Krt::kta, &["mata"]);
    assert_has_krdanta(&[], &man, Krt::ktavatu, &["matavat"]);
    // Niti
    assert_has_atmane_tinanta(&[], &tan, Lun, Prathama, Eka, &["atata", "atanizwa"]);
    assert_has_atmane_tinanta(&[], &tan, Lun, Madhyama, Eka, &["ataTAH", "atanizWAH"]);
    // anudAttopadeSavanatitanotyAdInAm ?
    let sham = d("Samu~", Divadi);
    assert_has_krdanta(&[], &sham, Krt::kta, &["SAnta"]);
    assert_has_krdanta(&[], &sham, Krt::ktavatu, &["SAntavat"]);
    let tam = d("tamu~", Divadi);
    assert_has_krdanta(&[], &tam, Krt::kta, &["tAnta"]);
    assert_has_krdanta(&[], &tam, Krt::ktavatu, &["tAntavat"]);
    let dam = d("damu~", Divadi);
    assert_has_krdanta(&[], &dam, Krt::kta, &["dAnta"]);
    assert_has_krdanta(&[], &dam, Krt::ktavatu, &["dAntavat"]);
    // anunAsikasya
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::kta, &["pakva"]);
    assert_has_krdanta(&[], &pac, Krt::ktavatu, &["pakvavat"]);
    // Jali
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_lat_karmani(&[], &gam, &["gamyate"]);
    assert_has_lat_karmani(&[], &d("ra\\mu~\\", Bhvadi), &["ramyate"]);
    // kNiti
    assert_has_krdanta(&[], &yam, Krt::tfc, &["yantf"]);
    assert_has_krdanta(&[], &yam, Krt::tavya, &["yantavya"]);
    // upadeSagrahanam
    assert_has_krdanta(&[], &gam, Krt::ktin, &["gati"]);
}

#[test]
fn sutra_6_4_38() {
    assert_has_krdanta(
        &["pra"],
        &d("ya\\ma~", Bhvadi),
        Krt::ktvA,
        &["prayatya", "prayamya"],
    );
    assert_has_krdanta(
        &["pra"],
        &d("ra\\mu~", Bhvadi),
        Krt::ktvA,
        &["praratya", "praramya"],
    );
    assert_has_krdanta(
        &["pra"],
        &d("Ra\\ma~", Bhvadi),
        Krt::ktvA,
        &["praRatya", "praRamya"],
    );
    assert_has_krdanta(
        &["AN"],
        &d("ga\\mx~", Bhvadi),
        Krt::ktvA,
        &["Agatya", "Agamya"],
    );
    assert_has_krdanta(&["AN"], &d("ha\\na~", Adadi), Krt::ktvA, &["Ahatya"]);
    assert_has_krdanta(&["pra"], &d("manu~\\", Tanadi), Krt::ktvA, &["pramatya"]);
    assert_has_krdanta(&["pra"], &d("vanu~\\", Tanadi), Krt::ktvA, &["pravatya"]);
    assert_has_krdanta(&["pra"], &d("kzaRu~^", Tanadi), Krt::ktvA, &["prakzatya"]);
}

#[test]
fn sutra_6_4_42() {
    let jan = d("janI~\\", Divadi);
    assert_has_krdanta(&[], &jan, Krt::kta, &["jAta"]);
    assert_has_krdanta(&[], &jan, Krt::ktavatu, &["jAtavat"]);
    assert_has_krdanta(&[], &jan, Krt::ktin, &["jAti"]);

    let sanu = d("zaRu~^", Tanadi);
    assert_has_krdanta(&[], &sanu, Krt::kta, &["sAta"]);
    assert_has_krdanta(&[], &sanu, Krt::ktavatu, &["sAtavat"]);
    assert_has_krdanta(&[], &sanu, Krt::ktin, &["sAti"]);
    assert_has_lat_p(&[], &san(&sanu), &["sizAsati", "sisanizati"]);

    let khan = d("Kanu~^", Bhvadi);
    assert_has_krdanta(&[], &khan, Krt::kta, &["KAta"]);
    assert_has_krdanta(&[], &khan, Krt::ktavatu, &["KAtavat"]);
    assert_has_krdanta(&[], &khan, Krt::ktin, &["KAti"]);

    assert_has_lat_p(&[], &san(&khan), &["ciKanizati"]);
    // TODO: others
}

#[test]
fn sutra_6_4_43() {
    let jan = d("janI~\\", Divadi);
    assert_has_lat_karmani(&[], &jan, &["jAyate", "janyate"]);
    assert_has_lat(&[], &yan(&jan), &["jAjAyate", "jaYjanyate"]);

    let sanu = d("zaRu~^", Tanadi);
    assert_has_lat_karmani(&[], &sanu, &["sAyate", "sanyate"]);
    assert_has_lat(&[], &yan(&sanu), &["sAsAyate", "saMsanyate"]);

    let khan = d("Kanu~^", Bhvadi);
    assert_has_lat_karmani(&[], &khan, &["KAyate", "Kanyate"]);
    assert_has_lat(&[], &yan(&khan), &["cAKAyate", "caNKanyate"]);

    // Not for Syani
    assert_has_lat(&[], &jan, &["jAyate"]);
}

#[test]
fn sutra_6_4_44() {
    let tan = d("tanu~^", Tanadi);
    assert_has_lat_karmani(&[], &tan, &["tAyate", "tanyate"]);
    assert_has_lat(&[], &yan(&tan), &["tantanyate"]);
}

#[test]
fn sutra_6_4_47() {
    let bhrasj = d("Bra\\sja~^", Tudadi);
    assert_has_krdanta(&[], &bhrasj, Krt::tfc, &["Brazwf", "Barzwf"]);
    assert_has_krdanta(&[], &bhrasj, Krt::tumun, &["Brazwum", "Barzwum"]);
    assert_has_krdanta(&[], &bhrasj, Krt::tavya, &["Brazwavya", "Barzwavya"]);
    assert_has_krdanta(&[], &bhrasj, Krt::lyuw, &["Brajjana", "Barjana"]);
}

#[test]
fn sutra_6_4_48() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &san(&kf), Krt::tfc, &["cikIrzitf"]);
    assert_has_krdanta(&[], &san(&kf), Krt::tumun, &["cikIrzitum"]);
    assert_has_krdanta(&[], &san(&kf), Krt::tavya, &["cikIrzitavya"]);
    assert_has_tinanta(&[], &d("Divi~", Bhvadi), Lat, Prathama, Dvi, &["DinutaH"]);
    assert_has_tinanta(&[], &d("kfvi~", Bhvadi), Lat, Prathama, Dvi, &["kfRutaH"]);

    // ataH
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::tfc, &["stotf"]);

    // taparakaraRa
    assert_has_krdanta(&[], &d("yA\\", Adadi), Krt::tfc, &["yAtf"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), Krt::tfc, &["vAtf"]);

    // ArdhadhAtuke
    assert_has_taddhitanta(&prati("vfkza"), T::tva, &["vfkzatva"]);
    assert_has_taddhitanta(&prati("vfkza"), T::tal, &["vfkzatA"]);

    // Others
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&[], &san(&kf), Krt::Rvul, &["cikIrzaka"]);
    assert_has_krdanta(&[], &san(&hf), Krt::Rvul, &["jihIrzaka"]);
    assert_has_lat_karmani(&[], &san(&kf), &["cikIrzyate"]);
    assert_has_lat_karmani(&[], &san(&hf), &["jihIrzyate"]);
}

#[test]
fn sutra_6_4_49() {
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_krdanta(&[], &yan(&bhid), Krt::tfc, &["beBiditf"]);
    assert_has_krdanta(&[], &yan(&bhid), Krt::tumun, &["beBiditum"]);
    assert_has_krdanta(&[], &yan(&bhid), Krt::tavya, &["beBiditavya"]);

    // yasya (as y+a)
    assert_has_krdanta(&[], &d("Irzya~", Bhvadi), Krt::tfc, &["Irzyitf"]);
    assert_has_krdanta(&[], &d("mavya~", Bhvadi), Krt::tfc, &["mavyitf"]);

    // halaH
    assert_has_krdanta(&[], &yan(&d("lUY", Kryadi)), Krt::tfc, &["lolUyitf"]);
    assert_has_krdanta(&[], &yan(&d("pUY", Kryadi)), Krt::tfc, &["popUyitf"]);
}

#[ignore]
#[test]
fn sutra_6_4_51() {
    assert_has_lun_p(&[], &nic(&d("takza~", Bhvadi)), &["atatakzat"]);
    assert_has_lun_p(&[], &nic(&d("rakza~", Bhvadi)), &["ararakzat"]);
    assert_has_lun_p(&[], &nic(&d("aSa~", Kryadi)), &["ASiSat"]);
    assert_has_lun_p(&[], &nic(&d("awa~", Bhvadi)), &["Awiwat"]);

    let kf = nic(&d("qukf\\Y", Tanadi));
    let hf = nic(&d("hf\\Y", Bhvadi));
    assert_has_krdanta(&[], &kf, Krt::yuc, &["kAraRA"]);
    assert_has_krdanta(&[], &hf, Krt::yuc, &["hAraRA"]);
    assert_has_krdanta(&[], &kf, Krt::Rvul, &["kAraka"]);
    assert_has_krdanta(&[], &hf, Krt::Rvul, &["hAraka"]);
    assert_has_lat_karmani(&[], &kf, &["kAryate"]);
    assert_has_lat_karmani(&[], &hf, &["hAryate"]);

    // TODO: jYIpsati

    // aniwi
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kArayitf"]);
    assert_has_krdanta(&[], &hf, Krt::tfc, &["hArayitf"]);
}

#[test]
fn sutra_6_4_52() {
    assert_has_krdanta(&[], &nic(&d("qukf\\Y", Tanadi)), Krt::kta, &["kArita"]);
    assert_has_krdanta(&[], &nic(&d("hf\\Y", Bhvadi)), Krt::kta, &["hArita"]);
    assert_has_krdanta(&[], &d("gaRa", Curadi), Krt::kta, &["gaRita"]);
    assert_has_krdanta(&[], &d("lakza~", Curadi), Krt::kta, &["lakzita"]);
}

#[ignore]
#[test]
fn sutra_6_4_62() {
    let ci = d("ci\\Y", Svadi);
    assert_has_lrt_karmani(&[], &ci, &["cAyizyate", "cezyate"]);
    assert_has_lrn_karmani(&[], &ci, &["acAyizyata", "acezyata"]);
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_lrn_karmani(&[], &daa, &["dAyizyate", "dAsyate"]);
    assert_has_lrn_karmani(&[], &daa, &["adAyizyata", "adAsyata"]);

    let han = &d("ha\\na~", Adadi);
    assert_has_lrt_karmani(&[], &han, &["GAnizyate", "hanizyate"]);
    assert_has_lrn_karmani(&[], &han, &["aGAnizyata", "ahanizyata"]);
    // TODO: others
}

#[test]
fn sutra_6_4_63() {
    let di = d("dI\\N", Divadi);
    assert_has_tinanta(&["upa"], &di, Lit, Prathama, Eka, &["upadidIye"]);
    assert_has_tinanta(&["upa"], &di, Lit, Prathama, Dvi, &["upadidIyAte"]);
    assert_has_tinanta(&["upa"], &di, Lit, Prathama, Bahu, &["upadidIyire"]);

    // aci?
    assert_has_lat(&["upa"], &yan(&di), &["upadedIyate"]);
    // kNiti?
    assert_has_krdanta(&["upa"], &di, Krt::lyuw, &["upadAna"]);
}

#[test]
fn sutra_6_4_64() {
    let paa = d("pA\\", Bhvadi);
    let stha = d("zWA\\", Bhvadi);
    assert_has_parasmai_tinanta(&[], &paa, Lit, Madhyama, Eka, &["papiTa", "papATa"]);
    assert_has_parasmai_tinanta(&[], &stha, Lit, Madhyama, Eka, &["tasTiTa", "tasTATa"]);
    assert_has_parasmai_tinanta(&[], &paa, Lit, Prathama, Dvi, &["papatuH"]);
    assert_has_parasmai_tinanta(&[], &paa, Lit, Prathama, Bahu, &["papuH"]);
    assert_has_parasmai_tinanta(&[], &stha, Lit, Prathama, Dvi, &["tasTatuH"]);
    assert_has_parasmai_tinanta(&[], &stha, Lit, Prathama, Bahu, &["tasTuH"]);

    // ArdhadhAtuke?
    assert_has_parasmai_tinanta(&[], &d("yA\\", Adadi), Lat, Prathama, Bahu, &["yAnti"]);
    assert_has_parasmai_tinanta(&[], &d("vA\\", Adadi), Lat, Prathama, Bahu, &["vAnti"]);

    // aci
    assert_has_lat_karmani(&[], &d("glE\\", Bhvadi), &["glAyate"]);
    assert_has_atmane_tinanta(
        &[],
        &d("qudA\\Y", Juhotyadi),
        AshirLin,
        Uttama,
        Eka,
        &["dAsIya"],
    );

    // TODO: others
}

#[test]
fn sutra_6_4_65() {
    assert_has_krdanta(&[], &d("qudA\\Y", Juhotyadi), Krt::yat, &["deya"]);
    assert_has_krdanta(&[], &d("quDA\\Y", Juhotyadi), Krt::yat, &["Deya"]);
    assert_has_krdanta(&[], &d("o~hA\\k", Juhotyadi), Krt::yat, &["heya"]);
    assert_has_krdanta(&[], &d("zwE\\", Bhvadi), Krt::yat, &["steya"]);
}

#[test]
fn sutra_6_4_66() {
    let daa = d("qudA\\Y", Juhotyadi);
    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_lat_karmani(&[], &daa, &["dIyate"]);
    assert_has_lat_karmani(&[], &dhaa, &["DIyate"]);
    assert_has_lat(&[], &yan(&daa), &["dedIyate"]);
    assert_has_lat(&[], &yan(&dhaa), &["deDIyate"]);

    let maa = d("mA\\", Adadi);
    assert_has_lat_karmani(&[], &maa, &["mIyate"]);
    assert_has_lat(&[], &yan(&maa), &["memIyate"]);

    let sthaa = d("zWA\\", Bhvadi);
    assert_has_lat_karmani(&[], &sthaa, &["sTIyate"]);
    assert_has_lat(&[], &yan(&sthaa), &["tezWIyate"]);

    let gai = d("gE\\", Bhvadi);
    let i = d("i\\N", Adadi);
    assert_has_lat_karmani(&[], &gai, &["gIyate"]);
    assert_has_lat(&[], &yan(&gai), &["jegIyate"]);
    assert_has_atmane_tinanta(&["aDi"], &i, Lun, Prathama, Eka, &["aDyEzwa", "aDyagIzwa"]);
    assert_has_atmane_tinanta(
        &["aDi"],
        &i,
        Lun,
        Prathama,
        Dvi,
        &["aDyEzAtAm", "aDyagIzAtAm"],
    );
    assert_has_atmane_tinanta(
        &["aDi"],
        &i,
        Lun,
        Prathama,
        Bahu,
        &["aDyEzata", "aDyagIzata"],
    );

    let paa = d("pA\\", Bhvadi);
    let paa_paati = d("pA\\", Adadi);
    assert_has_lat_karmani(&[], &paa, &["pIyate"]);
    assert_has_lat(&[], &yan(&paa), &["pepIyate"]);
    assert_has_lat_karmani(&[], &paa_paati, &["pAyate"]);

    let haak = d("o~hA\\k", Juhotyadi);
    let haan = d("o~hA\\N", Juhotyadi);
    assert_has_lat_karmani(&[], &haak, &["hIyate"]);
    assert_has_lat(&[], &yan(&haak), &["jehIyate"]);
    assert_has_lat_karmani(&[], &haan, &["hAyate"]);

    let so = d("zo\\", Divadi);
    assert_has_lat_karmani(&["ava"], &so, &["avasIyate"]);
    // TODO: not sure if a typo
    assert_has_lat(&["ava"], &yan(&so), &["avasezIyate"]);

    // hali
    assert_has_parasmai_tinanta(&[], &daa, Lit, Prathama, Dvi, &["dadatuH"]);
    assert_has_parasmai_tinanta(&[], &daa, Lit, Prathama, Bahu, &["daduH"]);
    // kNiti
    assert_has_krdanta(&[], &daa, Krt::tfc, &["dAtf"]);
    assert_has_krdanta(&[], &dhaa, Krt::tfc, &["DAtf"]);
}

#[test]
fn sutra_6_4_67() {
    let daa = d("qudA\\Y", Juhotyadi);
    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_ashirlin_p(&[], &daa, &["deyAt"]);
    assert_has_ashirlin_p(&[], &dhaa, &["DeyAt"]);
    assert_has_ashirlin_p(&[], &d("mA\\", Adadi), &["meyAt"]);
    assert_has_ashirlin_p(&[], &d("zWA\\", Bhvadi), &["sTeyAt"]);
    assert_has_ashirlin_p(&[], &d("gE\\", Bhvadi), &["geyAt"]);
    assert_has_ashirlin_p(&[], &d("pA\\", Bhvadi), &["peyAt"]);
    assert_has_ashirlin_p(&[], &d("o~hA\\k", Juhotyadi), &["heyAt"]);
    assert_has_ashirlin_p(&["ava"], &d("zo\\", Divadi), &["avaseyAt"]);

    // kNiti
    assert_has_ashirlin_a(&[], &daa, &["dAsIzwa"]);
    assert_has_ashirlin_a(&[], &dhaa, &["DAsIzwa"]);
}

#[test]
fn sutra_6_4_68() {
    let glai = d("glE\\", Bhvadi);
    assert_has_ashirlin_p(&[], &glai, &["gleyAt", "glAyAt"]);
    assert_has_ashirlin_p(&[], &d("mlE\\", Bhvadi), &["mleyAt", "mlAyAt"]);
    // anyasya
    assert_has_ashirlin_p(&[], &d("zWA\\", Bhvadi), &["sTeyAt"]);
    // saMyogAdeH
    assert_has_ashirlin_p(&[], &d("yA\\", Adadi), &["yAyAt"]);
    // kNiti
    assert_has_ashirlin_karmani(&[], &glai, &["glAsIzwa"]);
    // aNgasya
    assert_has_ashirlin_p(&["nis"], &d("vA\\", Adadi), &["nirvAyAt"]);
}

#[test]
fn sutra_6_4_69() {
    assert_has_krdanta(&["pra"], &d("qudA\\Y", Juhotyadi), Krt::ktvA, &["pradAya"]);
    assert_has_krdanta(&["pra"], &d("quDA\\Y", Juhotyadi), Krt::ktvA, &["praDAya"]);
    assert_has_krdanta(&["pra"], &d("mA\\", Adadi), Krt::ktvA, &["pramAya"]);
    assert_has_krdanta(&["pra"], &d("zWA\\", Bhvadi), Krt::ktvA, &["prasTAya"]);
    assert_has_krdanta(&["pra"], &d("gE\\", Bhvadi), Krt::ktvA, &["pragAya"]);
    assert_has_krdanta(&["pra"], &d("pA\\", Bhvadi), Krt::ktvA, &["prapAya"]);
    assert_has_krdanta(&["pra"], &d("o~hA\\k", Juhotyadi), Krt::ktvA, &["prahAya"]);
    assert_has_krdanta(&["ava"], &d("zo\\", Divadi), Krt::ktvA, &["avasAya"]);
}

#[test]
fn sutra_6_4_70() {
    assert_has_krdanta(
        &["apa"],
        &d("me\\N", Bhvadi),
        Krt::ktvA,
        &["apamitya", "apamAya"],
    );
}

#[test]
fn sutra_6_4_71() {
    let kf = d("qukf\\Y", Tanadi);
    let hf = d("hf\\Y", Bhvadi);

    assert_has_lun_p(&[], &kf, &["akArzIt"]);
    assert_has_lun_p(&[], &hf, &["ahArzIt"]);
    assert_has_lan_p(&[], &kf, &["akarot"]);
    assert_has_lan_p(&[], &hf, &["aharat"]);
    assert_has_lrn_p(&[], &kf, &["akarizyat"]);
    assert_has_lrn_p(&[], &hf, &["aharizyat"]);
}

#[test]
fn sutra_6_4_72() {
    let ikz = d("Ikza~\\", Bhvadi);
    let ih = d("Eha~\\", Bhvadi);
    let ubj = d("ubja~", Tudadi);
    let umbh = d("unBa~", Tudadi);

    assert_has_lun(&[], &ikz, &["Ekzizwa"]);
    assert_has_lun(&[], &ih, &["Ehizwa"]);
    assert_has_lun(&[], &ubj, &["ObjIt"]);
    assert_has_lun(&[], &umbh, &["OmBIt"]);
    assert_has_lan(&[], &ikz, &["Ekzata"]);
    assert_has_lan(&[], &ih, &["Ehata"]);
    assert_has_lan(&[], &ubj, &["Objat"]);
    assert_has_lan(&[], &umbh, &["OmBat"]);
    assert_has_lrn(&[], &ikz, &["Ekzizyata"]);
    assert_has_lrn(&[], &ih, &["Ehizyata"]);
    assert_has_lrn(&[], &ubj, &["Objizyat"]);
    assert_has_lrn(&[], &umbh, &["OmBizyat"]);
}

#[test]
fn sutra_6_4_78() {
    let iz = d("izu~", Tudadi);
    let uz = d("uza~", Bhvadi);
    assert_has_lit(&[], &iz, &["iyeza"]);
    assert_has_lit(&[], &uz, &["uvoza", "ozAYcakAra", "ozAmAsa", "ozAmbaBUva"]);
    assert_has_lat(&[], &d("f\\", Juhotyadi), &["iyarti"]);

    // asavarRe
    assert_has_tinanta(&[], &iz, Lit, Prathama, Dvi, &["IzatuH"]);
    assert_has_tinanta(&[], &iz, Lit, Prathama, Bahu, &["IzuH"]);
    assert_has_tinanta(
        &[],
        &uz,
        Lit,
        Prathama,
        Dvi,
        &["UzatuH", "ozAYcakratuH", "ozAmAsatuH", "ozAmbaBUvatuH"],
    );
    assert_has_tinanta(
        &[],
        &uz,
        Lit,
        Prathama,
        Bahu,
        &["UzuH", "ozAYcakruH", "ozAmAsuH", "ozAmbaBUvuH"],
    );

    // aci
    assert_has_lit_p(&[], &d("ya\\ja~^", Bhvadi), &["iyAja"]);
    assert_has_lit_p(&[], &d("quva\\pa~^", Bhvadi), &["uvApa"]);
}

#[test]
fn sutra_6_4_79() {
    let stri = stri("strI");
    assert_has_subantas_p(&stri, Stri, V::Prathama, Eka, &["strI"]);
    assert_has_subantas_p(&stri, Stri, V::Prathama, Dvi, &["striyO"]);
    assert_has_subantas_p(&stri, Stri, V::Prathama, Bahu, &["striyaH"]);
}

#[test]
fn sutra_6_4_80() {
    let stri = stri("strI");
    assert_has_subantas_p(&stri, Stri, V::Dvitiya, Eka, &["strIm", "striyam"]);
    assert_has_subantas_p(&stri, Stri, V::Dvitiya, Bahu, &["strIH", "striyaH"]);
}

#[test]
fn sutra_6_4_81() {
    let i = d("i\\R", Adadi);
    assert_has_tinanta(&[], &i, Lat, Prathama, Bahu, &["yanti"]);
    assert_has_tinanta(&[], &i, Lot, Prathama, Bahu, &["yantu"]);
    assert_has_tinanta(&[], &i, Lan, Prathama, Bahu, &["Ayan"]);
    assert_has_krdanta(&[], &i, Krt::lyuw, &["ayana"]);
    assert_has_krdanta(&[], &i, Krt::Rvul, &["Ayaka"]);
}

#[ignore]
#[test]
fn sutra_6_4_83() {
    assert_has_subantas("KalapU", Pum, V::Prathama, Dvi, &["KalapvO"]);
    assert_has_subantas("KalapU", Pum, V::Prathama, Bahu, &["KalapvaH"]);
    assert_has_subantas("SatasU", Pum, V::Prathama, Dvi, &["SatasvO"]);
    assert_has_subantas("SatasU", Pum, V::Prathama, Bahu, &["SatasvaH"]);
    assert_has_subantas("sakfllU", Pum, V::Prathama, Dvi, &["sakfllvO"]);
    assert_has_subantas("sakfllU", Pum, V::Prathama, Bahu, &["sakfllvaH"]);
    // supi
    let lu = d("lUY", Kryadi);
    assert_has_parasmai_tinanta(&[], &lu, Lit, Prathama, Dvi, &["luluvatuH"]);
    assert_has_parasmai_tinanta(&[], &lu, Lit, Prathama, Bahu, &["luluvuH"]);
    // asaMyogapUrva
    assert_has_subantas("kawaprU", Pum, V::Prathama, Dvi, &["kawapruvO"]);
    assert_has_subantas("kawaprU", Pum, V::Prathama, Bahu, &["kawapruvaH"]);

    // TODO: luvO, luvaH, etc.
}

#[test]
fn sutra_6_4_88() {
    let bhu = d("BU", Bhvadi);
    assert_has_tinanta(&[], &bhu, Lun, Prathama, Bahu, &["aBUvan"]);
    assert_has_tinanta(&[], &bhu, Lun, Uttama, Eka, &["aBUvam"]);
    assert_has_tinanta(&[], &bhu, Lit, Prathama, Eka, &["baBUva"]);
    assert_has_tinanta(&[], &bhu, Lit, Prathama, Dvi, &["baBUvatuH"]);
    assert_has_tinanta(&[], &bhu, Lit, Prathama, Bahu, &["baBUvuH"]);
}

#[test]
fn sutra_6_4_89() {
    let guh = d("guhU~^", Bhvadi);
    assert_has_lat_p(&["ni"], &guh, &["nigUhati"]);
    assert_has_krdanta(&["ni"], &guh, Krt::Rvul, &["nigUhaka"]);
    assert_has_krdanta(&["ni"], &guh, Krt::Rini, &["nigUhin"]);
    assert_has_krdanta(&["ni"], &guh, Krt::Ramul, &["nigUham"]);
    assert_has_parasmai_tinanta(&["ni"], &guh, Lat, Prathama, Bahu, &["nigUhanti"]);
    assert_has_krdanta(&[], &guh, Krt::GaY, &["gUha"]);
    assert_has_parasmai_tinanta(&["ni"], &guh, Lit, Prathama, Dvi, &["nijuguhatuH"]);
    assert_has_parasmai_tinanta(&["ni"], &guh, Lit, Prathama, Bahu, &["nijuguhuH"]);
    // aci
    assert_has_krdanta(&["ni"], &guh, Krt::tfc, &["nigoQf", "nigUhitf"]);
    assert_has_krdanta(&["ni"], &guh, Krt::tumun, &["nigoQum", "nigUhitum"]);
}

#[test]
fn sutra_6_4_90_and_sutra_6_4_91() {
    let dus = nic(&d("du\\za~", Divadi));
    assert_has_lat_p(&[], &dus, &["dUzayati", "dozayati"]);
}

#[test]
fn sutra_6_4_92() {
    assert_has_lat_p(&[], &nic(&d("Gawa~\\", Bhvadi)), &["Gawayati"]);
    assert_has_lat_p(&[], &nic(&d("vyaTa~\\", Bhvadi)), &["vyaTayati"]);
    assert_has_lat_p(&[], &nic(&d("janI~\\", Divadi)), &["janayati"]);
    assert_has_lat_p(
        &[],
        &nic(&d("ra\\nja~^", Divadi)),
        &["rajayati", "raYjayati"],
    );

    // TODO: not sure
    // assert_has_lat_p(&[], &nic("Samu~", Bhvadi), &["Samayati"]);
    // assert_has_lat_p(&[], &nic("", Bhvadi), &["jYapayati"]);
}

#[test]
fn sutra_6_4_98() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_tinanta(&[], &gam, Lit, Prathama, Dvi, &["jagmatuH"]);
    assert_has_tinanta(&[], &gam, Lit, Prathama, Bahu, &["jagmuH"]);

    let han = d("ha\\na~", Adadi);
    assert_has_tinanta(&[], &han, Lit, Prathama, Dvi, &["jaGnatuH"]);
    assert_has_tinanta(&[], &han, Lit, Prathama, Bahu, &["jaGnuH"]);

    let jan = d("janI~\\", Divadi);
    assert_has_tinanta(&[], &jan, Lit, Prathama, Eka, &["jajYe"]);
    assert_has_tinanta(&[], &jan, Lit, Prathama, Dvi, &["jajYAte"]);
    assert_has_tinanta(&[], &jan, Lit, Prathama, Bahu, &["jajYire"]);

    let khan = d("Kanu~^", Bhvadi);
    assert_has_parasmai_tinanta(&[], &khan, Lit, Prathama, Dvi, &["caKnatuH"]);
    assert_has_parasmai_tinanta(&[], &khan, Lit, Prathama, Bahu, &["caKnuH"]);

    let ad = d("a\\da~", Adadi);
    assert_has_tinanta(&[], &ad, Lit, Prathama, Dvi, &["jakzatuH", "AdatuH"]);
    assert_has_tinanta(&[], &ad, Lit, Prathama, Bahu, &["jakzuH", "AduH"]);

    // kNiti
    assert_has_krdanta(&[], &gam, Krt::lyuw, &["gamana"]);
    assert_has_krdanta(&[], &han, Krt::lyuw, &["hanana"]);

    // anaNi
    assert_has_lun(&[], &gam, &["agamat"]);
    assert_has_lun(&[], &ad, &["aGasat"]);

    // aci
    assert_has_lat_karmani(&[], &gam, &["gamyate"]);
    assert_has_lat_karmani(&[], &han, &["hanyate"]);
}

#[test]
fn sutra_6_4_105() {
    assert_has_hi(&[], &d("qupa\\ca~^z", Bhvadi), &["paca", "pacatAt"]);
    assert_has_hi(&[], &d("paWa~", Bhvadi), &["paWa", "paWatAt"]);
    assert_has_hi(&[], &d("ga\\mx~", Bhvadi), &["gacCa", "gacCatAt"]);
    assert_has_hi(&[], &d("DAvu~^", Bhvadi), &["DAva", "DAvatAt"]);
    // ataH
    assert_has_hi(&[], &d("yu", Adadi), &["yuhi", "yutAt"]);
    assert_has_hi(
        &[],
        &d("ru", Adadi),
        &["ruhi", "rutAt", "ruvIhi", "ruvItAt"],
    );
    // taparakaraRa
    assert_has_hi(&[], &d("lUY", Kryadi), &["lunIhi", "lunItAt"]);
    assert_has_hi(&[], &d("pUY", Kryadi), &["punIhi", "punItAt"]);
}

#[test]
fn sutra_6_4_106() {
    assert_has_hi(&[], &d("ci\\Y", Svadi), &["cinu", "cinutAt"]);
    assert_has_hi(&[], &d("zu\\Y", Svadi), &["sunu", "sunutAt"]);
    assert_has_hi(&[], &d("qukf\\Y", Tanadi), &["kuru", "kurutAt"]);
    // utaH
    assert_has_hi(&[], &d("lUY", Kryadi), &["lunIhi", "lunItAt"]);
    assert_has_hi(&[], &d("pUY", Kryadi), &["punIhi", "punItAt"]);
    // pratyayAt
    assert_has_hi(&[], &d("yu", Adadi), &["yuhi", "yutAt"]);
    assert_has_hi(
        &[],
        &d("ru", Adadi),
        &["ruhi", "rutAt", "ruvIhi", "ruvItAt"],
    );
    // asaMyogapUrvAt
    assert_has_hi(&["pra"], &d("A\\px~", Svadi), &["prApnuhi", "prApnutAt"]);
    assert_has_hi(&[], &d("rA\\Da~", Svadi), &["rADnuhi", "rADnutAt"]);
    assert_has_hi(&[], &d("takzU~", Svadi), &["takzRuhi", "takzRutAt"]);
}

#[test]
fn sutra_6_4_107() {
    let su = d("zu\\Y", Svadi);
    assert_has_vas(&[], &su, Lat, &["sunvaH", "sunuvaH"]);
    assert_has_mas(&[], &su, Lat, &["sunmaH", "sunumaH"]);
    let tan = d("tanu~^", Tanadi);
    assert_has_vas(&[], &tan, Lat, &["tanvaH", "tanuvaH"]);
    assert_has_mas(&[], &tan, Lat, &["tanmaH", "tanumaH"]);

    // pratyayasya?
    let yu = d("yu", Adadi);
    assert_has_vas(&[], &yu, Lat, &["yuvaH"]);
    assert_has_mas(&[], &yu, Lat, &["yumaH"]);
    // asaMyogapUrvasya?
    let shak = d("Sa\\kx~", Svadi);
    assert_has_vas(&[], &shak, Lat, &["SaknuvaH"]);
    assert_has_mas(&[], &shak, Lat, &["SaknumaH"]);
}

#[test]
fn sutra_6_4_108() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_vas(&[], &kf, Lat, &["kurvaH"]);
    assert_has_mas(&[], &kf, Lat, &["kurmaH"]);
}

#[test]
fn sutra_6_4_109() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_parasmai_tinanta(&[], &kf, VidhiLin, Prathama, Eka, &["kuryAt"]);
    assert_has_parasmai_tinanta(&[], &kf, VidhiLin, Prathama, Dvi, &["kuryAtAm"]);
    assert_has_parasmai_tinanta(&[], &kf, VidhiLin, Prathama, Bahu, &["kuryuH"]);
}

#[test]
fn sutra_6_4_112() {
    let lu = d("lUY", Kryadi);
    assert_has_atmane_tinanta(&[], &lu, Lat, Prathama, Bahu, &["lunate"]);
    assert_has_atmane_tinanta(&[], &lu, Lot, Prathama, Bahu, &["lunatAm"]);
    assert_has_atmane_tinanta(&[], &lu, Lan, Prathama, Bahu, &["alunata"]);

    let maa = d("mA\\N", Juhotyadi);
    assert_has_atmane_tinanta(&[], &maa, Lat, Prathama, Bahu, &["mimate"]);
    assert_has_atmane_tinanta(&[], &maa, Lot, Prathama, Bahu, &["mimatAm"]);
    assert_has_atmane_tinanta(&[], &maa, Lan, Prathama, Bahu, &["amimata"]);

    let haa = d("o~hA\\N", Juhotyadi);
    assert_has_atmane_tinanta(&["sam"], &haa, Lat, Prathama, Bahu, &["saYjihate"]);
    assert_has_atmane_tinanta(&["sam"], &haa, Lot, Prathama, Bahu, &["saYjihatAm"]);
    assert_has_atmane_tinanta(&["sam"], &haa, Lan, Prathama, Bahu, &["samajihata"]);

    // SnAByastayoH
    assert_has_tinanta(&[], &d("yA\\", Adadi), Lat, Prathama, Bahu, &["yAnti"]);
    assert_has_tinanta(&[], &d("vA\\", Adadi), Lat, Prathama, Bahu, &["vAnti"]);

    // AtaH
    assert_has_parasmai_tinanta(
        &[],
        &d("quBf\\Y", Juhotyadi),
        Lat,
        Prathama,
        Bahu,
        &["biBrati"],
    );

    // kNiti
    assert_has_lan_p(&[], &lu, &["alunAt"]);
}

#[test]
fn sutra_6_4_113() {
    let lu = d("lUY", Kryadi);
    let pu = d("pUY", Kryadi);
    assert_has_parasmai_tinanta(&[], &lu, Lat, Prathama, Dvi, &["lunItaH"]);
    assert_has_parasmai_tinanta(&[], &pu, Lat, Prathama, Dvi, &["punItaH"]);
    assert_has_parasmai_tinanta(&[], &lu, Lat, Madhyama, Dvi, &["lunITaH"]);
    assert_has_parasmai_tinanta(&[], &pu, Lat, Madhyama, Dvi, &["punITaH"]);
    assert_has_lat_a(&[], &lu, &["lunIte"]);
    assert_has_lat_a(&[], &pu, &["punIte"]);

    let maa = d("mA\\N", Juhotyadi);
    assert_has_atmane_tinanta(&[], &maa, Lat, Prathama, Eka, &["mimIte"]);
    assert_has_atmane_tinanta(&[], &maa, Lat, Madhyama, Eka, &["mimIze"]);
    assert_has_atmane_tinanta(&[], &maa, Lat, Madhyama, Bahu, &["mimIDve"]);

    let haa = d("o~hA\\N", Juhotyadi);
    assert_has_atmane_tinanta(&["sam"], &haa, Lat, Prathama, Eka, &["saYjihIte"]);
    assert_has_atmane_tinanta(&["sam"], &haa, Lat, Madhyama, Eka, &["saYjihIze"]);
    assert_has_atmane_tinanta(&["sam"], &haa, Lat, Madhyama, Bahu, &["saYjihIDve"]);

    // hali
    assert_has_parasmai_tinanta(&[], &lu, Lat, Prathama, Bahu, &["lunanti"]);
    assert_has_atmane_tinanta(&[], &maa, Lat, Prathama, Bahu, &["mimate"]);

    // aGoH
    assert_has_parasmai_tinanta(
        &[],
        &d("qudA\\Y", Juhotyadi),
        Lat,
        Prathama,
        Dvi,
        &["dattaH"],
    );
    assert_has_parasmai_tinanta(
        &[],
        &d("quDA\\Y", Juhotyadi),
        Lat,
        Prathama,
        Dvi,
        &["DattaH"],
    );

    // kNiti
    assert_has_lat_p(&[], &lu, &["lunAti"]);
    assert_has_lat_p(&[], &d("o~hA\\k", Juhotyadi), &["jahAti"]);
}

#[test]
fn sutra_6_4_114() {
    let daridra = d("daridrA", Adadi);
    assert_has_tinanta(&[], &daridra, Lat, Prathama, Dvi, &["daridritaH"]);
    assert_has_tinanta(&[], &daridra, Lat, Madhyama, Dvi, &["daridriTaH"]);
    assert_has_tinanta(&[], &daridra, Lat, Uttama, Dvi, &["daridrivaH"]);
    assert_has_tinanta(&[], &daridra, Lat, Uttama, Bahu, &["daridrimaH"]);

    // hali
    assert_has_tinanta(&[], &daridra, Lat, Prathama, Bahu, &["daridrati"]);

    // kNiti
    assert_has_tinanta(&[], &daridra, Lat, Prathama, Eka, &["daridrAti"]);
}

#[test]
fn sutra_6_4_115() {
    let bhi = d("YiBI\\", Juhotyadi);
    assert_has_parasmai_tinanta(&[], &bhi, Lat, Prathama, Dvi, &["biBitaH", "biBItaH"]);
    assert_has_parasmai_tinanta(&[], &bhi, Lat, Madhyama, Dvi, &["biBiTaH", "biBITaH"]);
    assert_has_parasmai_tinanta(&[], &bhi, Lat, Uttama, Dvi, &["biBivaH", "biBIvaH"]);
    assert_has_parasmai_tinanta(&[], &bhi, Lat, Uttama, Bahu, &["biBimaH", "biBImaH"]);

    // hali
    assert_has_parasmai_tinanta(&[], &bhi, Lat, Prathama, Bahu, &["biByati"]);

    // kNiti
    assert_has_parasmai_tinanta(&[], &bhi, Lat, Prathama, Eka, &["biBeti"]);

    // sArvadhAtuke
    assert_has_lat_karmani(&[], &bhi, &["BIyate"]);
}

#[test]
fn sutra_6_4_116() {
    let haa = d("o~hA\\k", Juhotyadi);
    assert_has_tinanta(&[], &haa, Lat, Prathama, Dvi, &["jahitaH", "jahItaH"]);
    assert_has_tinanta(&[], &haa, Lat, Madhyama, Dvi, &["jahiTaH", "jahITaH"]);
    assert_has_tinanta(&[], &haa, Lat, Uttama, Dvi, &["jahivaH", "jahIvaH"]);
    assert_has_tinanta(&[], &haa, Lat, Uttama, Bahu, &["jahimaH", "jahImaH"]);

    // hali
    assert_has_tinanta(&[], &haa, Lat, Prathama, Bahu, &["jahati"]);

    // kNiti
    assert_has_tinanta(&[], &haa, Lat, Prathama, Eka, &["jahAti"]);

    // sArvadhAtuke
    assert_has_lat_karmani(&[], &haa, &["hIyate"]);
    assert_has_lat(&[], &yan(&haa), &["jehIyate"]);
}

#[ignore]
#[test]
fn sutra_6_4_117() {
    let haa = d("o~hA\\k", Juhotyadi);
    assert_has_tinanta(
        &[],
        &haa,
        Lot,
        Madhyama,
        Eka,
        &["jahAhi", "jahihi", "jahIhi", "jahitAt", "jahItAt"],
    );
}

#[test]
fn sutra_6_4_118() {
    let haa = d("o~hA\\k", Juhotyadi);
    assert_has_tinanta(&[], &haa, VidhiLin, Prathama, Eka, &["jahyAt"]);
    assert_has_tinanta(&[], &haa, VidhiLin, Prathama, Dvi, &["jahyAtAm"]);
    assert_has_tinanta(&[], &haa, VidhiLin, Prathama, Bahu, &["jahyuH"]);
}

#[test]
fn sutra_6_4_119() {
    assert_has_hi(&[], &d("qudA\\Y", Juhotyadi), &["dehi", "dattAt"]);
    assert_has_hi(&[], &d("quDA\\Y", Juhotyadi), &["Dehi", "DattAt"]);
    assert_has_hi(&[], &d("asa~", Adadi), &["eDi", "stAt"]);
}

#[test]
fn sutra_6_4_146() {
    use Taddhita as T;
    assert_has_taddhitanta(&prati("baBru"), T::yaY, &["bABravya"]);
    assert_has_taddhitanta(&prati("maRqu"), T::yaY, &["mARqavya"]);
    assert_has_taddhitanta(&prati("SaNku"), T::yat, &["SaNkavya"]);
    assert_has_taddhitanta(&prati("picu"), T::yat, &["picavya"]);
    assert_has_taddhitanta(&prati("kamaRqalu"), T::yat, &["kamaRqalavya"]);
    assert_has_taddhitanta(&prati("upagu"), T::aR, &["Opagava"]);
    assert_has_taddhitanta(&prati("kapawu"), T::aR, &["kApawava"]);
    // TODO: svAyamBuva
}
