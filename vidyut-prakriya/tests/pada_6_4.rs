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

fn prati_dhatu(text: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(text)
        .is_dhatu(true)
        .build()
        .unwrap()
}

fn prati_udit(text: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(text)
        .is_udit(true)
        .build()
        .unwrap()
}

fn assert_has_hi(prefixes: &[&str], d: &Dhatu, expected: &[&str]) {
    assert_has_sip(prefixes, &d, Lot, expected);
}

#[test]
fn sutra_6_4_1() {
    use Vibhakti::*;

    // halaH
    assert_has_krdanta(&[], &d("hve\\Y", Bhvadi), Krt::kta, &["hUta"]);
    assert_has_krdanta(&[], &d("jyA\\", Kryadi), Krt::kta, &["jIna"]);
    assert_has_krdanta(&["sam"], &d("vye\\Y", Bhvadi), Krt::kta, &["saMvIta"]);
    // angasya?
    let ve = d("ve\\Y", Bhvadi);
    assert_has_krdanta(&["nir"], &ve, Krt::kta, &["niruta"]);
    assert_has_krdanta(&["dur"], &ve, Krt::kta, &["duruta"]);

    // nAmi dIrgha
    assert_has_subantas("agni", Pum, Sasthi, Bahu, &["agnInAm"]);
    assert_has_subantas("vAyu", Pum, Sasthi, Bahu, &["vAyUnAm"]);
    // angasya?
    assert_has_subantas("krimiRA", Stri, Dvitiya, Eka, &["krimiRAm"]);
    assert_has_subantas("pAmanA", Stri, Dvitiya, Eka, &["pAmanAm"]);

    // ato bhisa ais
    assert_has_subantas("vfkza", Pum, Trtiya, Bahu, &["vfkzEH"]);
    assert_has_subantas("plakza", Pum, Trtiya, Bahu, &["plakzEH"]);
    // angasya
    assert_has_subantas_p(
        &nyap("brAhmaRaBissA"),
        Stri,
        Prathama,
        Eka,
        &["brAhmaRaBissA"],
    );
    assert_has_subantas_p(
        &nyap("odanaBissadA"),
        Stri,
        Prathama,
        Eka,
        &["odanaBissadA"],
    );
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

#[test]
fn sutra_6_4_10() {
    use Vibhakti::*;
    let shreyas = prati_udit("Sreyas");
    assert_has_subantas_p(&shreyas, Pum, Prathama, Eka, &["SreyAn"]);
    assert_has_subantas_p(&shreyas, Pum, Prathama, Dvi, &["SreyAMsO"]);
    assert_has_subantas_p(&shreyas, Pum, Prathama, Bahu, &["SreyAMsaH"]);
    assert_has_subantas_p(&shreyas, Napumsaka, Prathama, Bahu, &["SreyAMsi"]);
    assert_has_subantas("payas", Napumsaka, Prathama, Bahu, &["payAMsi"]);
    assert_has_subantas("yaSas", Napumsaka, Prathama, Bahu, &["yaSAMsi"]);
    // mahat
    let mahat = prati_udit("mahat");
    assert_has_subantas_p(&mahat, Pum, Prathama, Eka, &["mahAn"]);
    assert_has_subantas_p(&mahat, Pum, Prathama, Dvi, &["mahAntO"]);
    assert_has_subantas_p(&mahat, Pum, Prathama, Bahu, &["mahAntaH"]);
    // asambudDo
    assert_has_subantas_p(&shreyas, Pum, Sambodhana, Eka, &["Sreyan"]);
    assert_has_subantas_p(&mahat, Pum, Sambodhana, Eka, &["mahan"]);
}

#[test]
fn sutra_6_4_14() {
    use Vibhakti::*;
    assert_has_subantas_p(&prati_udit("Bavat"), Pum, Prathama, Eka, &["BavAn"]);
    assert_has_subantas_p(&prati_udit("kftavat"), Pum, Prathama, Eka, &["kftavAn"]);
    assert_has_subantas_p(&prati_udit("gomat"), Pum, Prathama, Eka, &["gomAn"]);
    assert_has_subantas_p(&prati_udit("yavamat"), Pum, Prathama, Eka, &["yavamAn"]);
    assert_has_subantas("suyaSas", Pum, Prathama, Eka, &["suyaSAH"]);
    assert_has_subantas("suSrotas", Pum, Prathama, Eka, &["suSrotAH"]);
    // TODO: others
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
    assert_has_sip(&[], &d("ha\\na~", Adadi), Lot, &["jahi", "hatAt"]);
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

#[ignore]
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

    let kf_nic = nic(&d("qukf\\Y", Tanadi));
    let hf_nic = nic(&d("hf\\Y", Bhvadi));
    assert_has_krdanta(&[], &kf_nic, Krt::yuc, &["kAraRA"]);
    assert_has_krdanta(&[], &hf_nic, Krt::yuc, &["hAraRA"]);
    assert_has_krdanta(&[], &kf_nic, Krt::Rvul, &["kAraka"]);
    assert_has_krdanta(&[], &hf_nic, Krt::Rvul, &["hAraka"]);
    assert_has_lat_karmani(&[], &kf_nic, &["kAryate"]);
    assert_has_lat_karmani(&[], &hf_nic, &["hAryate"]);

    // TODO: jYIpsati

    // aniwi
    assert_has_krdanta(&[], &kf_nic, Krt::tfc, &["kArayitf"]);
    assert_has_krdanta(&[], &hf_nic, Krt::tfc, &["hArayitf"]);
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
fn sutra_6_4_55() {
    let kf_nic = nic(&d("qukf\\Y", Tanadi));
    let hf_nic = nic(&d("hf\\Y", Bhvadi));

    // Am
    assert_has_lit_p(
        &[],
        &kf_nic,
        &["kArayAYcakAra", "kArayAmAsa", "kArayAmbaBUva"],
    );
    assert_has_lit_p(
        &[],
        &hf_nic,
        &["hArayAYcakAra", "hArayAmAsa", "hArayAmbaBUva"],
    );
    // anta
    assert_has_krdanta(&[], &nic(&d("gaqi~", Bhvadi)), Krt::Jac, &["gaRqayanta"]);
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Bhvadi)), Krt::Jac, &["maRqayanta"]);
    // Alu
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Bhvadi)), Krt::Aluc, &["spfhayAlu"]);
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Bhvadi)), Krt::Aluc, &["gfhayAlu"]);
    // Ayya
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Bhvadi)), Krt::Ayya, &["spfhayAyya"]);
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Bhvadi)), Krt::Ayya, &["gfhayAyya"]);
    // itnu
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Bhvadi)), Krt::itnuc, &["gfhayAyya"]);
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Bhvadi)), Krt::izRuc, &["pozayizRu"]);
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Bhvadi)), Krt::izRuc, &["pArayizRu"]);
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
    assert_has_sip(&[], &paa, Lit, &["papiTa", "papATa"]);
    assert_has_sip(&[], &stha, Lit, &["tasTiTa", "tasTATa"]);
    assert_has_tas(&[], &paa, Lit, &["papatuH"]);
    assert_has_jhi(&[], &paa, Lit, &["papuH"]);
    assert_has_tas(&[], &stha, Lit, &["tasTatuH"]);
    assert_has_jhi(&[], &stha, Lit, &["tasTuH"]);

    // ArdhadhAtuke?
    assert_has_jhi(&[], &d("yA\\", Adadi), Lat, &["yAnti"]);
    assert_has_jhi(&[], &d("vA\\", Adadi), Lat, &["vAnti"]);

    // aci
    assert_has_lat_karmani(&[], &d("glE\\", Bhvadi), &["glAyate"]);
    assert_has_iw(&[], &d("qudA\\Y", Juhotyadi), AshirLin, &["dAsIya"]);

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
    assert_has_tas(&[], &daa, Lit, &["dadatuH"]);
    assert_has_jhi(&[], &daa, Lit, &["daduH"]);
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

#[ignore]
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
    let stri = nyap("strI");
    assert_has_subantas_p(&stri, Stri, V::Prathama, Eka, &["strI"]);
    assert_has_subantas_p(&stri, Stri, V::Prathama, Dvi, &["striyO"]);
    assert_has_subantas_p(&stri, Stri, V::Prathama, Bahu, &["striyaH"]);
}

#[test]
fn sutra_6_4_80() {
    let stri = nyap("strI");
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
    assert_has_tas(&[], &lu, Lit, &["luluvatuH"]);
    assert_has_jhi(&[], &lu, Lit, &["luluvuH"]);
    // asaMyogapUrva
    assert_has_subantas("kawaprU", Pum, V::Prathama, Dvi, &["kawapruvO"]);
    assert_has_subantas("kawaprU", Pum, V::Prathama, Bahu, &["kawapruvaH"]);

    // TODO: luvO, luvaH, etc.
}

#[test]
fn sutra_6_4_88() {
    let bhu = d("BU", Bhvadi);
    assert_has_jhi(&[], &bhu, Lun, &["aBUvan"]);
    assert_has_mip(&[], &bhu, Lun, &["aBUvam"]);
    assert_has_tip(&[], &bhu, Lit, &["baBUva"]);
    assert_has_tas(&[], &bhu, Lit, &["baBUvatuH"]);
    assert_has_jhi(&[], &bhu, Lit, &["baBUvuH"]);
}

#[test]
fn sutra_6_4_89() {
    let guh = d("guhU~^", Bhvadi);
    assert_has_lat_p(&["ni"], &guh, &["nigUhati"]);
    assert_has_krdanta(&["ni"], &guh, Krt::Rvul, &["nigUhaka"]);
    assert_has_krdanta(&["ni"], &guh, Krt::Rini, &["nigUhin"]);
    assert_has_krdanta(&["ni"], &guh, Krt::Ramul, &["nigUham"]);
    assert_has_jhi(&["ni"], &guh, Lat, &["nigUhanti"]);
    assert_has_krdanta(&[], &guh, Krt::GaY, &["gUha"]);
    assert_has_tas(&["ni"], &guh, Lit, &["nijuguhatuH"]);
    assert_has_jhi(&["ni"], &guh, Lit, &["nijuguhuH"]);
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

#[ignore]
#[test]
fn sutra_6_4_96() {
    let chad = d("Cada~", Curadi);
    assert_has_upapada_krdanta("uras", &[], &chad, Krt::Ga, &["uraSCada"]);
    assert_has_krdanta(&["pra"], &chad, Krt::Ga, &["pracCada"]);
    assert_has_krdanta(&["pra"], &chad, Krt::Ga, &["pracCada"]);
    assert_has_upapada_krdanta("danta", &[], &chad, Krt::Ga, &["dantacCada"]);
    // dvi-upasargasya?
    assert_has_krdanta(&["sam", "upa"], &chad, Krt::Ga, &["samupacCAda"]);
}

#[test]
fn sutra_6_4_96_v1() {
    let chad = d("Cada~", Curadi);
    assert_has_krdanta(&["sam", "upa", "ati"], &chad, Krt::Ga, &["samupAticCAda"]);
}

#[ignore]
#[test]
fn sutra_6_4_97() {
    let chad = d("Cada~", Curadi);
    assert_has_krdanta(&[], &chad, Krt::isi, &["Cadis"]);
    assert_has_krdanta(&[], &chad, Krt::manin, &["Cadman"]);
    assert_has_krdanta(&[], &chad, Krt::zwran, &["Cattra"]);
    assert_has_upapada_krdanta("dAman", &[], &chad, Krt::kvip, &["dAmacCad"]);
    assert_has_krdanta(&["upa"], &chad, Krt::kvip, &["upacCad"]);
}

#[test]
fn sutra_6_4_98() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_tas(&[], &gam, Lit, &["jagmatuH"]);
    assert_has_jhi(&[], &gam, Lit, &["jagmuH"]);

    let han = d("ha\\na~", Adadi);
    assert_has_tas(&[], &han, Lit, &["jaGnatuH"]);
    assert_has_jhi(&[], &han, Lit, &["jaGnuH"]);

    let jan = d("janI~\\", Divadi);
    assert_has_ta(&[], &jan, Lit, &["jajYe"]);
    assert_has_aataam(&[], &jan, Lit, &["jajYAte"]);
    assert_has_jha(&[], &jan, Lit, &["jajYire"]);

    let khan = d("Kanu~^", Bhvadi);
    assert_has_tas(&[], &khan, Lit, &["caKnatuH"]);
    assert_has_jhi(&[], &khan, Lit, &["caKnuH"]);

    let ad = d("a\\da~", Adadi);
    assert_has_tas(&[], &ad, Lit, &["jakzatuH", "AdatuH"]);
    assert_has_jhi(&[], &ad, Lit, &["jakzuH", "AduH"]);

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
fn sutra_6_4_100() {
    let hu = d("hu\\", Juhotyadi);
    assert_has_hi(&[], &hu, &["juhuDi", "juhutAt"]);
    assert_has_hi(
        &[],
        &d("Bi\\di~^r", Rudhadi),
        &["BinDi", "BindDi", "BintAt", "BinttAt"],
    );
    assert_has_hi(
        &[],
        &d("Ci\\di~^r", Rudhadi),
        &["CinDi", "CindDi", "CintAt", "CinttAt"],
    );

    // hu-jhal
    assert_has_hi(&[], &d("qukrI\\Y", Kryadi), &["krIRIhi", "krIRItAt"]);
    assert_has_hi(&[], &d("prI\\Y", Kryadi), &["prIRIhi", "prIRItAt"]);
    // heH
    assert_has_tas(&[], &hu, Lot, &["juhutAm"]);
    // hali
    assert_has_hi(&[], &d("rudi~r", Adadi), &["rudihi", "ruditAt"]);

    // TODO: others
}

#[test]
fn sutra_6_4_104() {
    assert_has_lun_karmani(&[], &d("qukf\\Y", Tanadi), &["akAri"]);
    assert_has_lun_karmani(&[], &d("hf\\Y", Bhvadi), &["ahAri"]);
    assert_has_lun_karmani(&[], &d("lUY", Kryadi), &["alAvi"]);
    assert_has_lun_karmani(&[], &d("qupa\\ca~^z", Bhvadi), &["apAci"]);

    // ciR
    assert_has_lat_karmani(&[], &d("Bi\\di~^r", Rudhadi), &["Bidyate"]);

    // TODO: tarAm
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
    assert_has_tip(&[], &kf, VidhiLin, &["kuryAt"]);
    assert_has_tas(&[], &kf, VidhiLin, &["kuryAtAm"]);
    assert_has_jhi(&[], &kf, VidhiLin, &["kuryuH"]);
}

#[test]
fn sutra_6_4_110() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tas(&[], &kf, Lat, &["kurutaH"]);
    assert_has_jhi(&[], &kf, Lat, &["kurvanti"]);
    // sArvadhAtuka
    assert_has_hi(&[], &kf, &["kuru", "kurutAt"]);
    // kNiti
    assert_has_tip(&[], &kf, Lat, &["karoti"]);
    assert_has_sip(&[], &kf, Lat, &["karozi"]);
    assert_has_mip(&[], &kf, Lat, &["karomi"]);
}

#[test]
fn sutra_6_4_111() {
    let rudh = d("ru\\Di~^r", Rudhadi);
    assert_has_tas(&[], &rudh, Lat, &["runDaH", "rundDaH"]);
    assert_has_jhi(&[], &rudh, Lat, &["runDanti"]);
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_tas(&[], &bhid, Lat, &["BintaH", "BinttaH"]);
    assert_has_jhi(&[], &bhid, Lat, &["Bindanti"]);
    // asteH
    let as_ = d("asa~", Adadi);
    assert_has_tas(&[], &as_, Lat, &["staH"]);
    assert_has_jhi(&[], &as_, Lat, &["santi"]);
    // kNiti
    assert_has_lat_p(&[], &bhid, &["Binatti"]);
    assert_has_lat_p(&[], &as_, &["asti"]);
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
    assert_has_jhi(&[], &d("yA\\", Adadi), Lat, &["yAnti"]);
    assert_has_jhi(&[], &d("vA\\", Adadi), Lat, &["vAnti"]);

    // AtaH
    assert_has_jhi(&[], &d("quBf\\Y", Juhotyadi), Lat, &["biBrati"]);

    // kNiti
    assert_has_lan_p(&[], &lu, &["alunAt"]);
}

#[test]
fn sutra_6_4_113() {
    let lu = d("lUY", Kryadi);
    let pu = d("pUY", Kryadi);
    assert_has_tas(&[], &lu, Lat, &["lunItaH"]);
    assert_has_tas(&[], &pu, Lat, &["punItaH"]);
    assert_has_thas(&[], &lu, Lat, &["lunITaH"]);
    assert_has_thas(&[], &pu, Lat, &["punITaH"]);
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
    assert_has_jhi(&[], &lu, Lat, &["lunanti"]);
    assert_has_atmane_tinanta(&[], &maa, Lat, Prathama, Bahu, &["mimate"]);

    // aGoH
    assert_has_tas(&[], &d("qudA\\Y", Juhotyadi), Lat, &["dattaH"]);
    assert_has_tas(&[], &d("quDA\\Y", Juhotyadi), Lat, &["DattaH"]);

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
    assert_has_tas(&[], &bhi, Lat, &["biBitaH", "biBItaH"]);
    assert_has_thas(&[], &bhi, Lat, &["biBiTaH", "biBITaH"]);
    assert_has_vas(&[], &bhi, Lat, &["biBivaH", "biBIvaH"]);
    assert_has_mas(&[], &bhi, Lat, &["biBimaH", "biBImaH"]);

    // hali
    assert_has_jhi(&[], &bhi, Lat, &["biByati"]);

    // kNiti
    assert_has_tip(&[], &bhi, Lat, &["biBeti"]);

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
fn sutra_6_4_120() {
    let ran = d("raRa~", Bhvadi);
    assert_has_tas(&[], &ran, Lit, &["reRatuH"]);
    assert_has_jhi(&[], &ran, Lit, &["reRuH"]);

    let yam = d("yama~", Bhvadi);
    assert_has_tas(&[], &yam, Lit, &["yematuH"]);
    assert_has_jhi(&[], &yam, Lit, &["yemuH"]);

    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tas(&[], &pac, Lit, &["pecatuH"]);
    assert_has_jhi(&[], &pac, Lit, &["pecuH"]);

    let dam = d("damu~", Divadi);
    assert_has_tas(&[], &dam, Lit, &["dematuH"]);
    assert_has_jhi(&[], &dam, Lit, &["demuH"]);

    // ataH?
    let div = d("divu~", Divadi);
    assert_has_tas(&[], &div, Lit, &["didivatuH"]);
    assert_has_jhi(&[], &div, Lit, &["didivuH"]);

    // taparakaraRa?
    let ras = d("rAsf~\\", Bhvadi);
    assert_has_atmane_tinanta(&[], &ras, Lit, Prathama, Eka, &["rarAse"]);
    assert_has_atmane_tinanta(&[], &ras, Lit, Prathama, Dvi, &["rarAsAte"]);
    assert_has_atmane_tinanta(&[], &ras, Lit, Prathama, Bahu, &["rarAsire"]);

    // ekahalmadhya?
    let shram = d("Sramu~", Divadi);
    assert_has_tas(&[], &shram, Lit, &["SaSramatuH"]);
    assert_has_jhi(&[], &shram, Lit, &["SaSramuH"]);

    let tsar = d("tsara~", Bhvadi);
    assert_has_tas(&[], &tsar, Lit, &["tatsaratuH"]);
    assert_has_jhi(&[], &tsar, Lit, &["tatsaruH"]);

    // anAdeze?
    let kan = d("kaRa~", Bhvadi);
    assert_has_tas(&[], &kan, Lit, &["cakaRatuH"]);
    assert_has_jhi(&[], &kan, Lit, &["cakaRuH"]);
    // TODO: how?
    // let gan = d("gaRa", Curadi);
    // assert_has_tas(&[], &gan, Lit, &["jagaRatuH"]);
    // assert_has_jhi(&[], &gan, Lit, &["jagaRuH"]);
    let bhan = d("BaRa~", Bhvadi);
    assert_has_tas(&[], &bhan, Lit, &["baBaRatuH"]);
    assert_has_jhi(&[], &bhan, Lit, &["baBaRuH"]);

    let nam = d("Ra\\ma~", Bhvadi);
    assert_has_tas(&[], &nam, Lit, &["nematuH"]);
    assert_has_jhi(&[], &nam, Lit, &["nemuH"]);

    let sah = d("zaha~\\", Bhvadi);
    assert_has_atmane_tinanta(&[], &sah, Lit, Prathama, Eka, &["sehe"]);
    assert_has_atmane_tinanta(&[], &sah, Lit, Prathama, Dvi, &["sehAte"]);
    assert_has_atmane_tinanta(&[], &sah, Lit, Prathama, Bahu, &["sehire"]);

    // kNiti?
    assert_has_mip(&[], &pac, Lit, &["papaca", "papAca"]);
    assert_has_mip(&[], &d("paWa~", Bhvadi), Lit, &["papaWa", "papAWa"]);
}

#[test]
fn sutra_6_4_121() {
    assert_has_sip(&[], &d("qupa\\ca~^z", Bhvadi), Lit, &["peciTa", "papakTa"]);
    assert_has_sip(&[], &d("Sa\\kx~", Svadi), Lit, &["SekiTa", "SaSakTa"]);
    // ataH
    assert_has_sip(&[], &d("divu~", Divadi), Lit, &["dideviTa"]);
    // ekahalmadhya
    assert_has_sip(&[], &d("takzU~", Bhvadi), Lit, &["tatakziTa"]);
    assert_has_sip(&[], &d("rakza~", Bhvadi), Lit, &["rarakziTa"]);
    // anAdezAdi
    assert_has_sip(&[], &d("kaRa~", Bhvadi), Lit, &["cakaRiTa"]);
    assert_has_sip(&[], &d("BaRa~", Bhvadi), Lit, &["baBaRiTa"]);
}

#[test]
fn sutra_6_4_122() {
    let tf = d("tF", Bhvadi);
    assert_has_tas(&[], &tf, Lit, &["teratuH"]);
    assert_has_jhi(&[], &tf, Lit, &["teruH"]);
    assert_has_sip(&[], &tf, Lit, &["teriTa"]);

    let phal = d("YiPalA~", Bhvadi);
    assert_has_tas(&[], &phal, Lit, &["PelatuH"]);
    assert_has_jhi(&[], &phal, Lit, &["PeluH"]);
    assert_has_sip(&[], &phal, Lit, &["PeliTa"]);

    let bhaj = d("Ba\\ja~^", Bhvadi);
    assert_has_tas(&[], &bhaj, Lit, &["BejatuH"]);
    assert_has_jhi(&[], &bhaj, Lit, &["BejuH"]);
    assert_has_sip(&[], &bhaj, Lit, &["BejiTa", "baBakTa"]);

    let trap = d("trapU~\\z", Bhvadi);
    assert_has_ta(&[], &trap, Lit, &["trepe"]);
    assert_has_aataam(&[], &trap, Lit, &["trepAte"]);
    assert_has_jha(&[], &trap, Lit, &["trepire"]);
}

#[test]
fn sutra_6_4_122_v1() {
    // For SaSranTatuH, see 1.2.6.v1.
    let shranth = d("SranTa~", Kryadi);
    assert_has_tas(&[], &shranth, Lit, &["SreTatuH", "SaSranTatuH"]);
    assert_has_jhi(&[], &shranth, Lit, &["SreTuH", "SaSranTuH"]);
}

#[test]
fn sutra_6_4_124() {
    let jf = d("jF", Kryadi);
    assert_has_tas(&[], &jf, Lit, &["jeratuH", "jajaratuH"]);
    assert_has_jhi(&[], &jf, Lit, &["jeruH", "jajaruH"]);
    assert_has_sip(&[], &jf, Lit, &["jeriTa", "jajariTa"]);

    let bhram = d("Bramu~", Bhvadi);
    assert_has_tas(&[], &bhram, Lit, &["BrematuH", "baBramatuH"]);
    assert_has_jhi(&[], &bhram, Lit, &["BremuH", "baBramuH"]);
    assert_has_sip(&[], &bhram, Lit, &["BremiTa", "baBramiTa"]);

    let tras = d("trasI~", Divadi);
    assert_has_tas(&[], &tras, Lit, &["tresatuH", "tatrasatuH"]);
    assert_has_jhi(&[], &tras, Lit, &["tresuH", "tatrasuH"]);
    assert_has_sip(&[], &tras, Lit, &["tresiTa", "tatrasiTa"]);
}

#[test]
fn sutra_6_4_125() {
    let phan = d("PaRa~", Bhvadi);
    assert_has_tas(&[], &phan, Lit, &["PeRatuH", "paPaRatuH"]);
    assert_has_jhi(&[], &phan, Lit, &["PeRuH", "paPaRuH"]);
    assert_has_sip(&[], &phan, Lit, &["PeRiTa", "paPaRiTa"]);

    let raj = d("rAjf~^", Bhvadi);
    assert_has_tas(&[], &raj, Lit, &["rejatuH", "rarAjatuH"]);
    assert_has_jhi(&[], &raj, Lit, &["rejuH", "rarAjuH"]);
    assert_has_sip(&[], &raj, Lit, &["rejiTa", "rarAjiTa"]);

    let bhraj = d("wuBrAjf~\\", Bhvadi);
    assert_has_ta(&[], &bhraj, Lit, &["Breje", "baBrAje"]);
    assert_has_aataam(&[], &bhraj, Lit, &["BrejAte", "baBrAjAte"]);
    assert_has_jha(&[], &bhraj, Lit, &["Brejire", "baBrAjire"]);

    let bhrash = d("wuBrASf~\\", Bhvadi);
    assert_has_ta(&[], &bhrash, Lit, &["BreSe", "baBrASe"]);
    assert_has_aataam(&[], &bhrash, Lit, &["BreSAte", "baBrASAte"]);
    assert_has_jha(&[], &bhrash, Lit, &["BreSire", "baBrASire"]);

    let bhlash = d("wuBlASf~\\", Bhvadi);
    assert_has_ta(&[], &bhlash, Lit, &["BleSe", "baBlASe"]);
    assert_has_aataam(&[], &bhlash, Lit, &["BleSAte", "baBlASAte"]);
    assert_has_jha(&[], &bhlash, Lit, &["BleSire", "baBlASire"]);

    let syam = d("syamu~", Bhvadi);
    assert_has_tas(&[], &syam, Lit, &["syematuH", "sasyamatuH"]);
    assert_has_jhi(&[], &syam, Lit, &["syemuH", "sasyamuH"]);
    assert_has_sip(&[], &syam, Lit, &["syemiTa", "sasyamiTa"]);

    let svan = d("svana~", Bhvadi);
    assert_has_tas(&[], &svan, Lit, &["svenatuH", "sasvanatuH"]);
    assert_has_jhi(&[], &svan, Lit, &["svenuH", "sasvanuH"]);
    assert_has_sip(&[], &svan, Lit, &["sveniTa", "sasvaniTa"]);
}

#[test]
fn sutra_6_4_126() {
    let shas = d("Sasu~", Bhvadi);
    assert_has_tas(&[], &shas, Lit, &["SaSasatuH"]);
    assert_has_jhi(&[], &shas, Lit, &["SaSasuH"]);
    assert_has_sip(&[], &shas, Lit, &["SaSasiTa"]);

    let dad = d("dada~\\", Bhvadi);
    assert_has_ta(&[], &dad, Lit, &["dadade"]);
    assert_has_aataam(&[], &dad, Lit, &["dadadAte"]);
    assert_has_jha(&[], &dad, Lit, &["dadadire"]);

    let vam = d("wuvama~", Bhvadi);
    assert_has_tas(&[], &vam, Lit, &["vavamatuH"]);
    assert_has_jhi(&[], &vam, Lit, &["vavamuH"]);
    assert_has_sip(&[], &vam, Lit, &["vavamiTa"]);

    // "viSaSratuH" etc. come from rule 7.4.12.
    let shf = d("SF", Kryadi);
    assert_has_tas(&["vi"], &shf, Lit, &["viSaSaratuH", "viSaSratuH"]);
    assert_has_jhi(&["vi"], &shf, Lit, &["viSaSaruH", "viSaSruH"]);
    assert_has_sip(&["vi"], &shf, Lit, &["viSaSariTa"]);

    assert_has_sip(&[], &d("lUY", Kryadi), Lit, &["lulaviTa"]);
    assert_has_sip(&[], &d("pUY", Kryadi), Lit, &["pupaviTa"]);
}

#[ignore]
#[test]
fn sutra_6_4_131() {
    use V::*;
    assert_has_subantas("vidvas", Pum, Sasthi, Eka, &["viduzaH"]);
    assert_has_subantas("vidvas", Pum, Trtiya, Eka, &["viduzA"]);
    assert_has_subantas("vidvas", Pum, Caturthi, Eka, &["viduze"]);
    assert_has_subantas("pecivas", Pum, Sasthi, Eka, &["pecuzaH"]);
    assert_has_subantas("pecivas", Pum, Trtiya, Eka, &["pecuzA"]);
    assert_has_subantas("pecivas", Pum, Caturthi, Eka, &["pecuze"]);
    assert_has_subantas("papivas", Pum, Sasthi, Eka, &["papuzaH"]);
}

#[test]
fn sutra_6_4_133() {
    use V::*;
    assert_has_subantas("Svan", Pum, Sasthi, Eka, &["SunaH"]);
    assert_has_subantas("Svan", Pum, Trtiya, Eka, &["SunA"]);
    assert_has_subantas("Svan", Pum, Caturthi, Eka, &["Sune"]);
    assert_has_subantas("yuvan", Pum, Sasthi, Eka, &["yUnaH"]);
    assert_has_subantas("yuvan", Pum, Trtiya, Eka, &["yUnA"]);
    assert_has_subantas("yuvan", Pum, Caturthi, Eka, &["yUne"]);
    assert_has_subantas("maGavan", Pum, Sasthi, Eka, &["maGonaH"]);
    assert_has_subantas("maGavan", Pum, Trtiya, Eka, &["maGonA"]);
    assert_has_subantas("maGavan", Pum, Caturthi, Eka, &["maGone"]);

    // atadDite?
    assert_has_taddhitanta(&prati("Svan"), T::aY, &["SOva"]);
    assert_has_taddhitanta(&prati("yuvan"), T::aR, &["yOvana"]);
    assert_has_taddhitanta(&prati("maGavan"), T::aR, &["mAGavana"]);

    // TODO: others
}

#[test]
fn sutra_6_4_134() {
    use V::*;
    assert_has_subantas("rAjan", Pum, Sasthi, Eka, &["rAjYaH"]);
    assert_has_subantas("rAjan", Pum, Trtiya, Eka, &["rAjYA"]);
    assert_has_subantas("rAjan", Pum, Caturthi, Eka, &["rAjYe"]);
    assert_has_subantas("takzan", Pum, Trtiya, Eka, &["takzRA"]);
    assert_has_subantas("takzan", Pum, Caturthi, Eka, &["takzRe"]);
}

#[ignore]
#[test]
fn sutra_6_4_135() {
    use V::*;
    assert_has_subantas("ukzan", Pum, Sasthi, Eka, &["rAjYaH"]);
    assert_has_subantas("BrUnahan", Pum, Trtiya, Eka, &["rAjYA"]);
    assert_has_subantas("DrtarAjan", Pum, Caturthi, Eka, &["rAjYe"]);
    assert_has_subantas("sAman", Pum, Trtiya, Eka, &["takzRA"]);
    assert_has_subantas("vima", Pum, Caturthi, Eka, &["takzRe"]);
}

#[test]
fn sutra_6_4_136() {
    use V::*;
    assert_has_subantas("rAjan", Pum, Saptami, Eka, &["rAjYi", "rAjani"]);
    assert_has_subantas("sAman", Napumsaka, Saptami, Eka, &["sAmni", "sAmani"]);
    assert_has_subantas("sAman", Napumsaka, Prathama, Dvi, &["sAmnI", "sAmanI"]);
}

#[test]
fn sutra_6_4_137() {
    use V::*;
    assert_has_subantas("parvan", Pum, Trtiya, Eka, &["parvaRA"]);
    assert_has_subantas("parvan", Pum, Caturthi, Eka, &["parvaRe"]);
    assert_has_subantas("aTarvan", Pum, Trtiya, Eka, &["aTarvaRA"]);
    assert_has_subantas("aTarvan", Pum, Caturthi, Eka, &["aTarvaRe"]);

    // saMyogAt?
    // TODO
    // assert_has_subantas("pratidIvan", Napumsaka, Trtiya, Eka, &["pratidIvnA"]);
    // assert_has_subantas("pratidIvan", Napumsaka, Caturthi, Eka, &["pratidIvne"]);
    assert_has_subantas("sAman", Napumsaka, Trtiya, Eka, &["sAmnA"]);
    assert_has_subantas("sAman", Napumsaka, Caturthi, Eka, &["sAmne"]);

    // vamAntAt?
    assert_has_subantas("takzan", Pum, Trtiya, Eka, &["takzRA"]);
    assert_has_subantas("takzan", Pum, Caturthi, Eka, &["takzRe"]);
}

#[test]
fn sutra_6_4_140() {
    use V::*;
    let kilalapa = &prati_dhatu("kilAlapA");
    assert_has_subantas_p(&kilalapa, Pum, Dvitiya, Bahu, &["kilAlapaH"]);
    assert_has_subantas_p(&kilalapa, Pum, Trtiya, Eka, &["kilAlapA"]);
    assert_has_subantas_p(&kilalapa, Pum, Caturthi, Eka, &["kilAlape"]);
    let shubhamya = &prati_dhatu("SuBaMyA");
    assert_has_subantas_p(&shubhamya, Pum, Dvitiya, Bahu, &["SuBaMyaH"]);
    assert_has_subantas_p(&shubhamya, Pum, Trtiya, Eka, &["SuBaMyA"]);
    assert_has_subantas_p(&shubhamya, Pum, Caturthi, Eka, &["SuBaMye"]);

    // AtaH?
    let ni = &prati_dhatu("nI");
    assert_has_subantas_p(&ni, Pum, Trtiya, Eka, &["niyA"]);
    assert_has_subantas_p(&ni, Pum, Caturthi, Eka, &["niye"]);

    // dhAtoH?
    assert_has_subantas("KawvA", Stri, Dvitiya, Bahu, &["KawvAH"]);
    assert_has_subantas("mAlA", Stri, Dvitiya, Bahu, &["mAlAH"]);
}

#[ignore]
#[test]
fn sutra_6_4_142() {
    assert_has_taddhitanta(&prati("viMSati"), T::qvun, &["viMSaka"]);
    // TODO: assert_has_taddhitanta(&prati("viMSati"), T::waq, &["viMSa"]);

    // qiti?
    assert_has_subantas("viMSati", Pum, V::Trtiya, Eka, &["viMSatyA"]);
}

#[ignore]
#[test]
fn sutra_6_4_143() {
    assert_has_taddhitanta(&prati("kumuda"), T::qmatup, &["kumudvat"]);
    assert_has_taddhitanta(&prati("naqa"), T::qmatup, &["naqvat"]);
    assert_has_taddhitanta(&prati("vetasa"), T::qmatup, &["vetasvat"]);
    // TODO: others
    assert_has_taddhitanta(&prati("triMSat"), T::qvun, &["triMSaka"]);
}

#[test]
fn sutra_6_4_144() {
    use V::*;
    assert_has_taddhitanta(&prati("agniSarman"), T::iY, &["AgniSarmi"]);
    assert_has_taddhitanta(&prati("uquloman"), T::iY, &["Oqulomi"]);

    // naH?
    // taddhite?
    assert_has_subantas("Sarman", Napumsaka, Trtiya, Eka, &["SarmaRA"]);
    assert_has_subantas("Sarman", Napumsaka, Caturthi, Eka, &["SarmaRe"]);
}

#[test]
fn sutra_6_4_144_v1() {
    assert_has_taddhitanta(&prati("sabrahmacArin"), T::aR, &["sAbrahmacAra"]);
    assert_has_taddhitanta(&prati("pIWasarpin"), T::aR, &["pEWasarpa"]);
    assert_has_taddhitanta(&prati("kalApin"), T::aR, &["kAlApa"]);
    assert_has_taddhitanta(&prati("kuTumin"), T::aR, &["kOTuma"]);
    assert_has_taddhitanta(&prati("tEtilin"), T::aR, &["tEtila"]);
    assert_has_taddhitanta(&prati("jAjalin"), T::aR, &["jAjala"]);
    assert_has_taddhitanta(&prati("lANgalin"), T::aR, &["lANgala"]);
    assert_has_taddhitanta(&prati("SilAlin"), T::aR, &["SElAla"]);
    assert_has_taddhitanta(&prati("SiKaRqin"), T::aR, &["SEKaRqa"]);
    assert_has_taddhitanta(&prati("sukarasadman"), T::aR, &["sOkarasadma"]);
    assert_has_taddhitanta(&prati("suparvan"), T::aR, &["sOparva"]);
}

#[test]
fn sutra_6_4_146() {
    assert_has_taddhitanta(&prati("baBru"), T::yaY, &["bABravya"]);
    assert_has_taddhitanta(&prati("maRqu"), T::yaY, &["mARqavya"]);
    assert_has_taddhitanta(&prati("SaNku"), T::yat, &["SaNkavya"]);
    assert_has_taddhitanta(&prati("picu"), T::yat, &["picavya"]);
    assert_has_taddhitanta(&prati("kamaRqalu"), T::yat, &["kamaRqalavya"]);
    assert_has_taddhitanta(&prati("upagu"), T::aR, &["Opagava"]);
    assert_has_taddhitanta(&prati("kapawu"), T::aR, &["kApawava"]);
    // TODO: svAyamBuva
}

#[test]
fn sutra_6_4_155() {
    let patu = prati("pawu");
    assert_has_taddhitanta(&patu, T::izWan, &["pawizWa"]);
    assert_has_taddhitanta(&patu, T::imanic, &["pawiman"]);
    assert_has_taddhitanta(&patu, T::Iyasun, &["pawIyas"]);
    let laghu = prati("laGu");
    assert_has_taddhitanta(&laghu, T::izWan, &["laGizWa"]);
    assert_has_taddhitanta(&laghu, T::imanic, &["laGiman"]);
    assert_has_taddhitanta(&laghu, T::Iyasun, &["laGIyas"]);
}

#[test]
fn sutra_6_4_156() {
    assert_has_taddhitanta(&prati("sTUla"), T::izWan, &["sTavizWa"]);
    assert_has_taddhitanta(&prati("sTUla"), T::Iyasun, &["sTavIyas"]);
    assert_has_taddhitanta(&prati("dUra"), T::izWan, &["davizWa"]);
    assert_has_taddhitanta(&prati("dUra"), T::Iyasun, &["davIyas"]);
    assert_has_taddhitanta(&prati("yuvan"), T::izWan, &["yavizWa"]);
    assert_has_taddhitanta(&prati("yuvan"), T::Iyasun, &["yavIyas"]);
    assert_has_taddhitanta(&prati("hrasva"), T::izWan, &["hrasizWa"]);
    assert_has_taddhitanta(&prati("hrasva"), T::imanic, &["hrasiman"]);
    assert_has_taddhitanta(&prati("hrasva"), T::Iyasun, &["hrasIyas"]);
    assert_has_taddhitanta(&prati("kzipra"), T::izWan, &["kzepizWa"]);
    assert_has_taddhitanta(&prati("kzipra"), T::imanic, &["kzepiman"]);
    assert_has_taddhitanta(&prati("kzipra"), T::Iyasun, &["kzepIyas"]);
    assert_has_taddhitanta(&prati("kzudra"), T::izWan, &["kzodizWa"]);
    assert_has_taddhitanta(&prati("kzudra"), T::imanic, &["kzodiman"]);
    assert_has_taddhitanta(&prati("kzudra"), T::Iyasun, &["kzodIyas"]);
}

#[test]
fn sutra_6_4_157() {
    assert_has_taddhitanta(&prati("priya"), T::izWan, &["prezWa"]);
    assert_has_taddhitanta(&prati("priya"), T::imanic, &["preman"]);
    assert_has_taddhitanta(&prati("priya"), T::Iyasun, &["preyas"]);
    assert_has_taddhitanta(&prati("sTira"), T::izWan, &["sTezWa"]);
    assert_has_taddhitanta(&prati("sTira"), T::imanic, &[]);
    assert_has_taddhitanta(&prati("sTira"), T::Iyasun, &["sTeyas"]);
    assert_has_taddhitanta(&prati("sPira"), T::izWan, &["sPezWa"]);
    assert_has_taddhitanta(&prati("sPira"), T::imanic, &[]);
    assert_has_taddhitanta(&prati("sPira"), T::Iyasun, &["sPeyas"]);
    assert_has_taddhitanta(&prati("uru"), T::izWan, &["varizWa"]);
    assert_has_taddhitanta(&prati("uru"), T::imanic, &["variman"]);
    assert_has_taddhitanta(&prati("uru"), T::Iyasun, &["varIyas"]);
    assert_has_taddhitanta(&prati("bahula"), T::izWan, &["baMhizWa"]);
    assert_has_taddhitanta(&prati("bahula"), T::imanic, &["baMhiman"]);
    assert_has_taddhitanta(&prati("bahula"), T::Iyasun, &["baMhIyas"]);
    assert_has_taddhitanta(&prati("guru"), T::izWan, &["garizWa"]);
    assert_has_taddhitanta(&prati("guru"), T::imanic, &["gariman"]);
    assert_has_taddhitanta(&prati("guru"), T::Iyasun, &["garIyas"]);
    assert_has_taddhitanta(&prati("vfdDa"), T::izWan, &["varzizWa"]);
    assert_has_taddhitanta(&prati("vfdDa"), T::imanic, &[]);
    assert_has_taddhitanta(&prati("vfdDa"), T::Iyasun, &["varzIyas"]);
    assert_has_taddhitanta(&prati("tfpra"), T::izWan, &["trapizWa"]);
    assert_has_taddhitanta(&prati("tfpra"), T::imanic, &[]);
    assert_has_taddhitanta(&prati("tfpra"), T::Iyasun, &["trapIyas"]);
    assert_has_taddhitanta(&prati("dIrGa"), T::izWan, &["drAGizWa"]);
    assert_has_taddhitanta(&prati("dIrGa"), T::imanic, &["drAGiman"]);
    assert_has_taddhitanta(&prati("dIrGa"), T::Iyasun, &["drAGIyas"]);
    assert_has_taddhitanta(&prati("vfndAraka"), T::izWan, &["vfndizWa"]);
    assert_has_taddhitanta(&prati("vfndAraka"), T::imanic, &[]);
    assert_has_taddhitanta(&prati("vfndAraka"), T::Iyasun, &["vfndIyas"]);
}

#[test]
fn sutra_6_4_158() {
    assert_has_taddhitanta(&prati("bahu"), T::imanic, &["BUman"]);
    assert_has_taddhitanta(&prati("bahu"), T::Iyasun, &["BUyas"]);
}

#[test]
fn sutra_6_4_159() {
    assert_has_taddhitanta(&prati("bahu"), T::izWan, &["BUyizWa"]);
}

#[test]
fn sutra_6_4_160() {
    assert_has_taddhitanta(&prati("jya"), T::Iyasun, &["jyAyas"]);
}

#[test]
fn sutra_6_4_161() {
    assert_has_taddhitanta(&prati("pfTu"), T::izWan, &["praTizWa"]);
    assert_has_taddhitanta(&prati("pfTu"), T::imanic, &["praTiman"]);
    assert_has_taddhitanta(&prati("pfTu"), T::Iyasun, &["praTIyas"]);
    assert_has_taddhitanta(&prati("mfdu"), T::izWan, &["mradizWa"]);
    assert_has_taddhitanta(&prati("mfdu"), T::imanic, &["mradiman"]);
    assert_has_taddhitanta(&prati("mfdu"), T::Iyasun, &["mradIyas"]);
    // ftaH?
    assert_has_taddhitanta(&prati("pawu"), T::izWan, &["pawizWa"]);
    assert_has_taddhitanta(&prati("pawu"), T::imanic, &["pawiman"]);
    assert_has_taddhitanta(&prati("pawu"), T::Iyasun, &["pawIyas"]);
    // halAdeH?
    assert_has_taddhitanta(&prati("fju"), T::izWan, &["fjizWa"]);
    assert_has_taddhitanta(&prati("fju"), T::imanic, &["fjiman"]);
    assert_has_taddhitanta(&prati("fju"), T::Iyasun, &["fjIyas"]);
    // laGoH?
    assert_has_taddhitanta(&prati("kfzRA"), T::izWan, &["kfzRizWa"]);
    // TODO: assert_has_taddhitanta(&prati("kfzRA"), T::imanic, &["kfzRiman"]);
    assert_has_taddhitanta(&prati("kfzRA"), T::Iyasun, &["kfzRIyas"]);
    // TODO: others
}

#[test]
fn sutra_6_4_164() {
    assert_has_taddhitanta(&prati("saNkUwin"), T::aR, &["sANkUwina"]);
    assert_has_taddhitanta(&prati("saMrAvin"), T::aR, &["sAMrAviRa"]);
    assert_has_taddhitanta(&prati("sammArjin"), T::aR, &["sAmmArjina"]);
    assert_has_taddhitanta(&prati("sragvin"), T::aR, &["srAgviRa"]);
    // aRi?
    assert_has_taddhitanta(&prati("daRqin"), T::aY, &["dARqa"]);
}

#[test]
fn sutra_6_4_165() {
    assert_has_taddhitanta(&prati("gATin"), T::aR, &["gATina"]);
    assert_has_taddhitanta(&prati("vidaTin"), T::aR, &["vEdaTina"]);
    assert_has_taddhitanta(&prati("keSin"), T::aR, &["kESina"]);
    assert_has_taddhitanta(&prati("gaRin"), T::aR, &["gARina"]);
    assert_has_taddhitanta(&prati("paRin"), T::aR, &["pARina"]);
}

#[test]
fn sutra_6_4_166() {
    assert_has_taddhitanta(&prati("SaNKin"), T::aR, &["SANKina"]);
    assert_has_taddhitanta(&prati("madrin"), T::aR, &["mAdriRa"]);
    assert_has_taddhitanta(&prati("vajrin"), T::aR, &["vAjriRa"]);
}

#[test]
fn sutra_6_4_167() {
    assert_has_taddhitanta(&prati("sAman"), T::aR, &["sAmana"]);
    assert_has_taddhitanta(&prati("veman"), T::aR, &["vEmana"]);
    assert_has_taddhitanta(&prati("sutvan"), T::aR, &["sOtvana"]);
    assert_has_taddhitanta(&prati("jitvan"), T::aR, &["jEtvana"]);
}
