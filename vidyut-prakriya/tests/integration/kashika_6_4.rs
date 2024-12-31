extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::*;

fn assert_has_hi(prefixes: &[&str], d: &Dhatu, expected: &[&str]) {
    assert_has_sip(prefixes, &d, Lot, expected);
}

#[test]
fn sutra_6_4_1() {
    // halaH
    assert_has_krdanta(&[], &d("hve\\Y", Bhvadi), Krt::kta, &["hUta"]);
    assert_has_krdanta(&[], &d("jyA\\", Kryadi), Krt::kta, &["jIna"]);
    assert_has_krdanta(&["sam"], &d("vye\\Y", Bhvadi), Krt::kta, &["saMvIta"]);
    // angasya?
    let ve = d("ve\\Y", Bhvadi);
    assert_has_krdanta(&["nir"], &ve, Krt::kta, &["niruta"]);
    assert_has_krdanta(&["dur"], &ve, Krt::kta, &["duruta"]);

    // nAmi dIrgha
    assert_has_sup_6p("agni", Pum, &["agnInAm"]);
    assert_has_sup_6p("vAyu", Pum, &["vAyUnAm"]);
    // angasya?
    assert_has_sup_2s("krimiRA", Stri, &["krimiRAm"]);
    assert_has_sup_2s("pAmanA", Stri, &["pAmanAm"]);

    // ato bhisa ais
    assert_has_sup_3p("vfkza", Pum, &["vfkzEH"]);
    assert_has_sup_3p("plakza", Pum, &["plakzEH"]);
    // angasya
    assert_has_sup_1s(&nyap("brAhmaRaBissA"), Stri, &["brAhmaRaBissA"]);
    assert_has_sup_1s(&nyap("odanaBissadA"), Stri, &["odanaBissadA"]);
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
    assert_has_sup_6p("agni", Pum, &["agnInAm"]);
    assert_has_sup_6p("vAyu", Pum, &["vAyUnAm"]);
    assert_has_sup_6p("kartf", Pum, &["kartFRAm"]);
    assert_has_sup_6p("hartf", Pum, &["hartFRAm"]);
}

#[test]
fn sutra_6_4_4() {
    assert_has_sup_6p("tri", Stri, &["tisfRAm"]);
    assert_has_sup_6p("catur", Stri, &["catasfRAm"]);
}

#[test]
fn sutra_6_4_6() {
    assert_has_sup_6p("nf", Pum, &["nFRAm", "nfRAm"]);
}

#[test]
fn sutra_6_4_7() {
    assert_has_sup_6p("paYcan", Pum, &["paYcAnAm"]);
    assert_has_sup_6p("saptan", Pum, &["saptAnAm"]);
    assert_has_sup_6p("navan", Pum, &["navAnAm"]);
    assert_has_sup_6p("daSan", Pum, &["daSAnAm"]);
    // naH
    assert_has_sup_6p("catur", Pum, &["caturRAm"]);
    // nAmi
    assert_has_sup_6p("carman", Pum, &["carmaRAm"]);
}

#[test]
fn sutra_6_4_8() {
    assert_has_sup_1s("rAjan", Pum, &["rAjA"]);
    assert_has_sup_1d("rAjan", Pum, &["rAjAnO"]);
    assert_has_sup_1p("rAjan", Pum, &["rAjAnaH"]);
    assert_has_sup_2s("rAjan", Pum, &["rAjAnam"]);
    assert_has_sup_2d("rAjan", Pum, &["rAjAnO"]);
    assert_has_sup_1p("sAman", Napumsaka, &["sAmAni"]);
    assert_has_sup_2p("sAman", Napumsaka, &["sAmAni"]);
    // sarvanAmasTAne
    assert_has_sup_7s("rAjan", Pum, &["rAjani", "rAjYi"]);
    assert_has_sup_7s("sAman", Pum, &["sAmani", "sAmni"]);
    // asambudDO
    assert_has_sup_ss("rAjan", Pum, &["rAjan"]);
    assert_has_sup_ss("takzan", Pum, &["takzan"]);
}

#[test]
fn sutra_6_4_10() {
    let shreyas = taddhitanta("praSasya", T::Iyasun).with_require("Sreyas");
    assert_has_sup_1s(&shreyas, Pum, &["SreyAn"]);
    assert_has_sup_1d(&shreyas, Pum, &["SreyAMsO"]);
    assert_has_sup_1p(&shreyas, Pum, &["SreyAMsaH"]);
    assert_has_sup_1p(&shreyas, Napumsaka, &["SreyAMsi"]);
    assert_has_sup_1p("payas", Napumsaka, &["payAMsi"]);
    assert_has_sup_1p("yaSas", Napumsaka, &["yaSAMsi"]);

    // mahat
    let mahat = create_krdanta("mahat", &[], &d("maha~", Bhvadi), Unadi::ati);
    assert_has_sup_1s(&mahat, Pum, &["mahAn"]);
    assert_has_sup_1d(&mahat, Pum, &["mahAntO"]);
    assert_has_sup_1p(&mahat, Pum, &["mahAntaH"]);

    // asambudDo
    assert_has_sup_ss(&shreyas, Pum, &["Sreyan"]);
    assert_has_sup_ss(&mahat, Pum, &["mahan"]);
}

#[ignore]
#[test]
fn sutra_6_4_11() {
    assert_has_sup_1p("ap", Stri, &["ApaH"]);
    let bahvap = create_bahuvrihi("bahvap", "bahu", "ap");
    assert_has_sup_1p(&bahvap, Napumsaka, &["bahvAmpi"]);
}

#[ignore]
#[test]
fn sutra_6_4_12() {
    let bahudandin = create_bahuvrihi("bahudaRqin", "bahu", "daRqin");
    assert_has_sup_1p(&bahudandin, Napumsaka, &["bahudaRqIni"]);

    let bahucchatrin = create_bahuvrihi("bahucCatrin", "bahu", "Catrin");
    assert_has_sup_1p(&bahucchatrin, Napumsaka, &["bahucCatrIRi"]);
}

#[test]
fn sutra_6_4_13() {
    assert_has_sup_1s("daRqin", Pum, &["daRqI"]);
    assert_has_sup_1s("vftrahan", Pum, &["vftrahA"]);
    assert_has_sup_1s("pUzan", Pum, &["pUzA"]);
    assert_has_sup_1s("aryaman", Pum, &["aryamA"]);
}

#[test]
fn sutra_6_4_14() {
    let bhavat = create_krdanta("Bavat", &[], &d("BA\\", Adadi), Unadi::qavatu);
    assert_has_sup_1s(bhavat, Pum, &["BavAn"]);
    assert_has_sup_1s(taddhitanta("kfta", T::matup), Pum, &["kftavAn"]);
    assert_has_sup_1s(taddhitanta("go", T::matup), Pum, &["gomAn"]);
    assert_has_sup_1s(taddhitanta("yava", T::matup), Pum, &["yavamAn"]);
    assert_has_sup_1s("suyaSas", Pum, &["suyaSAH"]);
    assert_has_sup_1s("suSrotas", Pum, &["suSrotAH"]);
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

#[test]
fn sutra_6_4_16() {
    assert_has_lat(&[], &san(&d("vI\\", Adadi)), &["vivIzati"]);
    assert_has_tip(&[], &san(&d("zwu\\Y", Adadi)), Lat, &["tuzwUzati"]);
    assert_has_tip(&[], &san(&d("qukf\\Y", Tanadi)), Lat, &["cikIrzati"]);
    assert_has_tip(&[], &san(&d("hf\\Y", Bhvadi)), Lat, &["jihIrzati"]);
    assert_has_lat(&[], &san(&d("ha\\na~", Adadi)), &["jiGAMsati"]);
    assert_has_lat(&["aDi"], &san(&d("i\\N", Adadi)), &["aDijigAMsate"]);
}

#[test]
fn sutra_6_4_17() {
    assert_has_tip(
        &[],
        &san(&d("tanu~^", Tanadi)),
        Lat,
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
fn sutra_6_4_22() {
    assert_has_sip(&[], &d("asa~", Adadi), Lot, &["eDi", "stAt"]);
    assert_has_sip(&[], &d("SAsu~", Adadi), Lot, &["SADi", "SizwAt"]);

    // TODO: AgAhi
    assert_has_sip(&[], &d("ha\\na~", Adadi), Lot, &["jahi", "hatAt"]);

    // ABAt?
    assert_has_ta_k(&[], &d("Ba\\njo~", Rudhadi), Lun, &["aBAji", "aBaYji"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Bhvadi), Krt::GaY, &["rAga", "raNga"]);

    // atra?
    let papivas = krdanta(&[], &d("pA\\", Bhvadi), Krt::kvasu);
    let cicyivas = krdanta(&[], &d("ci\\Y", Svadi), Krt::kvasu);
    let luluvivas = krdanta(&[], &d("lUY", Kryadi), Krt::kvasu);
    assert_has_sup_2p(&papivas, Pum, &["papuzaH"]);
    // cikyuzaH also allowed by 7.3.58.
    assert_has_sup_2p(&cicyivas, Pum, &["cicyuzaH", "cikyuzaH"]);
    assert_has_sup_2p(&luluvivas, Pum, &["luluvuzaH"]);
}

#[test]
fn sutra_6_4_23() {
    assert_has_lat(&[], &d("anjU~", Rudhadi), &["anakti"]);
    assert_has_lat(&[], &d("Ba\\njo~", Rudhadi), &["Banakti"]);
    assert_has_lat(&[], &d("hisi~", Rudhadi), &["hinasti"]);

    // TODO: make these proper tests and model the dhatu.
    assert_has_sup_6p("yajYa", Pum, &["yajYAnAm"]);
    assert_has_sup_6p("yatna", Pum, &["yatnAnAm"]);
    assert_has_sup_6p("viSna", Pum, &["viSnAnAm"]);
    assert_has_sup_6p("praSna", Pum, &["praSnAnAm"]);
}

#[test]
fn sutra_6_4_24() {
    let sras = d("sransu~\\", Bhvadi);
    let dhvas = d("Dvansu~\\", Bhvadi);

    assert_has_krdanta(&[], &sras, Krt::kta, &["srasta"]);
    assert_has_krdanta(&[], &dhvas, Krt::kta, &["Dvasta"]);
    assert_has_ta_k(&[], &sras, Lat, &["srasyate"]);
    assert_has_ta_k(&[], &dhvas, Lat, &["Dvasyate"]);
    assert_has_lat(&[], &yan(&sras), &["sanIsrasyate"]);
    assert_has_lat(&[], &yan(&dhvas), &["danIDvasyate"]);

    let nand = d("wunadi~", Bhvadi);
    assert_has_ta_k(&[], &nand, Lat, &["nandyate"]);
    assert_has_lat(&[], &yan(&nand), &["nAnandyate"]);

    let ni = d("RI\\Y", Bhvadi);
    assert_has_ta_k(&[], &ni, Lat, &["nIyate"]);
    assert_has_lat(&[], &yan(&ni), &["nenIyate"]);

    let nah = d("Ra\\ha~^", Divadi);
    assert_has_ta_k(&[], &nah, Lat, &["nahyate"]);
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
    assert_has_tip(&[], &d("ra\\nja~^", Bhvadi), Lat, &["rajati"]);
}

#[test]
fn sutra_6_4_26_v2() {
    let ranj_nic = nic(&d("ra\\nja~^", Divadi));
    assert_has_tip(&[], &ranj_nic, Lat, &["rajayati", "raYjayati"]);
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

#[test]
fn sutra_6_4_28() {
    let syand = d("syandU~\\", Bhvadi);
    assert_has_krdanta(&[], &syand, Krt::GaY, &["syada", "syanda"]);
    // TODO: expand with gosyada, aSvasyada, etc.
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
    assert_has_ta_k(&[], &d("Ba\\njo~", Rudhadi), Lun, &["aBAji", "aBaYji"]);
}

#[test]
fn sutra_6_4_34() {
    let shas = d("SAsu~", Adadi);
    assert_has_tip(&["anu"], &shas, Lun, &["anvaSizat"]);
    assert_has_tas(&["anu"], &shas, Lun, &["anvaSizatAm"]);
    assert_has_jhi(&["anu"], &shas, Lun, &["anvaSizan"]);

    // Niti
    assert_has_vas(&[], &shas, Lat, &["SizvaH"]);
    assert_has_mas(&[], &shas, Lat, &["SizmaH"]);

    // aNhaloH
    assert_has_jhi(&[], &shas, Lat, &["SAsati"]);
    assert_has_tas(&[], &shas, Lit, &["SaSAsatuH"]);
    assert_has_jhi(&[], &shas, Lit, &["SaSAsuH"]);

    // Not A-SAs
    let ashas = d("SAsu~\\", Adadi);
    assert_has_ta(&["AN"], &ashas, Lat, &["ASAste"]);

    // TODO: others
}

#[test]
fn sutra_6_4_35() {
    let shas = d("SAsu~", Adadi);
    assert_has_sip(&["anu"], &shas, Lot, &["anuSADi", "anuSizwAt"]);
    assert_has_sip(&["pra"], &shas, Lot, &["praSADi", "praSizwAt"]);
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
    assert_has_ta(&[], &tan, Lun, &["atata", "atanizwa"]);
    assert_has_thaas(&[], &tan, Lun, &["ataTAH", "atanizWAH"]);
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
    assert_has_ta_k(&[], &gam, Lat, &["gamyate"]);
    assert_has_ta_k(&[], &d("ra\\mu~\\", Bhvadi), Lat, &["ramyate"]);
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
fn sutra_6_4_39() {
    assert_has_krdanta(&[], &d("ya\\ma~", Bhvadi), Krt::ktic, &["yanti"]);
    assert_has_krdanta(&[], &d("vanu~\\", Tanadi), Krt::ktic, &["vanti"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), Krt::ktic, &["tanti"]);
}

#[test]
fn sutra_6_4_40() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_upapada_krdanta("aNga", &[], &gam, Krt::kvip, &["aNgagat"]);
    assert_has_upapada_krdanta("kaliNga", &[], &gam, Krt::kvip, &["kaliNgagat"]);
    assert_has_upapada_krdanta("aDvan", &[], &gam, Krt::kvip, &["aDvagat"]);
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
    assert_has_tip(&[], &san(&sanu), Lat, &["sizAsati", "sisanizati"]);

    let khan = d("Kanu~^", Bhvadi);
    assert_has_krdanta(&[], &khan, Krt::kta, &["KAta"]);
    assert_has_krdanta(&[], &khan, Krt::ktavatu, &["KAtavat"]);
    assert_has_krdanta(&[], &khan, Krt::ktin, &["KAti"]);

    assert_has_tip(&[], &san(&khan), Lat, &["ciKanizati"]);
    // TODO: others
}

#[test]
fn sutra_6_4_43() {
    let jan = d("janI~\\", Divadi);
    assert_has_ta_k(&[], &jan, Lat, &["jAyate", "janyate"]);
    assert_has_lat(&[], &yan(&jan), &["jAjAyate", "jaYjanyate"]);

    let sanu = d("zaRu~^", Tanadi);
    assert_has_ta_k(&[], &sanu, Lat, &["sAyate", "sanyate"]);
    assert_has_lat(&[], &yan(&sanu), &["sAsAyate", "saMsanyate"]);

    let khan = d("Kanu~^", Bhvadi);
    assert_has_ta_k(&[], &khan, Lat, &["KAyate", "Kanyate"]);
    assert_has_lat(&[], &yan(&khan), &["cAKAyate", "caNKanyate"]);

    // Not for Syani
    assert_has_lat(&[], &jan, &["jAyate"]);
}

#[test]
fn sutra_6_4_44() {
    let tan = d("tanu~^", Tanadi);
    assert_has_ta_k(&[], &tan, Lat, &["tAyate", "tanyate"]);
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
    assert_has_tas(&[], &d("Divi~", Bhvadi), Lat, &["DinutaH"]);
    assert_has_tas(&[], &d("kfvi~", Bhvadi), Lat, &["kfRutaH"]);

    // ataH
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::tfc, &["stotf"]);

    // taparakaraRa
    assert_has_krdanta(&[], &d("yA\\", Adadi), Krt::tfc, &["yAtf"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), Krt::tfc, &["vAtf"]);

    // ArdhadhAtuke
    assert_has_taddhita("vfkza", T::tva, &["vfkzatva"]);
    assert_has_taddhita("vfkza", T::tal, &["vfkzatA"]);

    // Others
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&[], &san(&kf), Krt::Rvul, &["cikIrzaka"]);
    assert_has_krdanta(&[], &san(&hf), Krt::Rvul, &["jihIrzaka"]);
    assert_has_ta_k(&[], &san(&kf), Lat, &["cikIrzyate"]);
    assert_has_ta_k(&[], &san(&hf), Lat, &["jihIrzyate"]);
}

#[test]
fn sutra_6_4_49() {
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_krdanta(&[], &yan(&bhid), Krt::tfc, &["beBiditf"]);
    assert_has_krdanta(&[], &yan(&bhid), Krt::tumun, &["beBiditum"]);
    assert_has_krdanta(&[], &yan(&bhid), Krt::tavya, &["beBiditavya"]);

    // yasya (as "ya", not "y" and "a")?
    assert_has_krdanta(&[], &d("Irzya~", Bhvadi), Krt::tfc, &["Irzyitf"]);
    assert_has_krdanta(&[], &d("mavya~", Bhvadi), Krt::tfc, &["mavyitf"]);

    // halaH?
    assert_has_krdanta(&[], &yan(&d("lUY", Kryadi)), Krt::tfc, &["lolUyitf"]);
    assert_has_krdanta(&[], &yan(&d("pUY", Kryadi)), Krt::tfc, &["popUyitf"]);
}

#[test]
fn sutra_6_4_50() {
    let samidhya = &kyac(phit("samiD"));
    assert_has_tip(&[], &samidhya, Lut, &["samiDitA", "samiDyitA"]);

    let drshadya = &kyan(phit("dfzad"));
    assert_has_ta(&[], &drshadya, Lut, &["dfzadyitA", "dfzaditA"]);
}

#[test]
fn sutra_6_4_51() {
    assert_has_tip(&[], &nic(&d("takza~", Bhvadi)), Lun, &["atatakzat"]);
    assert_has_tip(&[], &nic(&d("rakza~", Bhvadi)), Lun, &["ararakzat"]);
    assert_has_tip(&[], &nic(&d("aSa~", Kryadi)), Lun, &["ASiSat"]);
    assert_has_tip(&[], &nic(&d("awa~", Bhvadi)), Lun, &["Awiwat"]);

    let kf_nic = nic(&d("qukf\\Y", Tanadi));
    let hf_nic = nic(&d("hf\\Y", Bhvadi));
    assert_has_krdanta(&[], &kf_nic, Krt::yuc, &["kAraRA"]);
    assert_has_krdanta(&[], &hf_nic, Krt::yuc, &["hAraRA"]);
    assert_has_krdanta(&[], &kf_nic, Krt::Rvul, &["kAraka"]);
    assert_has_krdanta(&[], &hf_nic, Krt::Rvul, &["hAraka"]);
    assert_has_ta_k(&[], &kf_nic, Lat, &["kAryate"]);
    assert_has_ta_k(&[], &hf_nic, Lat, &["hAryate"]);

    assert_has_tip(
        &[],
        &san(&d("jYapa~", Curadi)),
        Lat,
        &["jYIpsati", "jijYapayizati"],
    );

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

#[test]
fn sutra_6_4_55() {
    let kf_nic = nic(&d("qukf\\Y", Tanadi));
    let hf_nic = nic(&d("hf\\Y", Bhvadi));

    // Am
    assert_has_tip(
        &[],
        &kf_nic,
        Lit,
        &["kArayAYcakAra", "kArayAmAsa", "kArayAmbaBUva"],
    );
    assert_has_tip(
        &[],
        &hf_nic,
        Lit,
        &["hArayAYcakAra", "hArayAmAsa", "hArayAmbaBUva"],
    );
    let spfha = &d("spfha", Curadi);
    let gfha = &d("gfha", Curadi);

    // anta
    assert_has_krdanta(&[], &nic(&d("gaqi~", Bhvadi)), Unadi::Jac, &["gaRqayanta"]);
    assert_has_krdanta(
        &[],
        &nic(&d("maqi~\\", Bhvadi)),
        Unadi::Jac,
        &["maRqayanta"],
    );

    // Alu
    assert_has_krdanta(&[], &spfha, Krt::Aluc, &["spfhayAlu"]);
    assert_has_krdanta(&[], &gfha, Krt::Aluc, &["gfhayAlu"]);

    // Ayya
    assert_has_krdanta(&[], &spfha, Unadi::Ayya, &["spfhayAyya"]);
    assert_has_krdanta(&[], &gfha, Unadi::Ayya, &["gfhayAyya"]);

    // itnu
    assert_has_krdanta(&[], &d("stana", Curadi), Unadi::itnuc, &["stanayitnu"]);

    // izRu
    let t = Tester::with_chaandasa();
    t.assert_has_krdanta(&[], &nic(&d("puza~", Kryadi)), Krt::izRuc, &["pozayizRu"]);
    t.assert_has_krdanta(&[], &d("pAra", Curadi), Krt::izRuc, &["pArayizRu"]);
}

#[test]
fn sutra_6_4_56() {
    use Krt::ktvA;
    assert_has_krdanta(&["pra"], &nic(&d("Ra\\ma~", Bhvadi)), ktvA, &["praRamayya"]);
    assert_has_krdanta(&["pra"], &nic(&d("tamu~", Divadi)), ktvA, &["pratamayya"]);
    assert_has_krdanta(&["pra"], &nic(&d("damu~", Divadi)), ktvA, &["pradamayya"]);
    assert_has_krdanta(&["pra"], &nic(&d("Samu~", Divadi)), ktvA, &["praSamayya"]);
    assert_has_krdanta(&["sam"], &nic(&d("damu~", Divadi)), ktvA, &["sandamayya"]);

    let bebhidya = yan_nic(&d("Bi\\di~^r", Rudhadi));
    assert_has_krdanta(&["pra"], &bebhidya, ktvA, &["prabeBidayya"]);
    assert_has_krdanta(&["pra"], &d("gaRa", Curadi), ktvA, &["pragaRayya"]);

    // laGupUrva?
    assert_has_krdanta(&["pra"], &nic(&d("patx~", Bhvadi)), ktvA, &["prapAtya"]);
}

#[ignore]
#[test]
fn sutra_6_4_62() {
    let ci = d("ci\\Y", Svadi);
    let daa = d("qudA\\Y", Juhotyadi);
    let sham = nic(&d("Samo~", Bhvadi));
    let han = d("ha\\na~", Adadi);
    let grah = d("graha~^", Kryadi);
    let drsh = d("df\\Si~r", Bhvadi);

    // sya
    assert_has_ta_k(&[], &ci, Lrt, &["cAyizyate", "cezyate"]);
    assert_has_ta_k(&[], &ci, Lrn, &["acAyizyata", "acezyata"]);
    assert_has_ta_k(&[], &daa, Lrt, &["dAyizyate", "dAsyate"]);
    assert_has_ta_k(&[], &daa, Lrn, &["adAyizyata", "adAsyata"]);
    // SAmayizyate is allowed here because mittva is optional by Dhatupatha 1.0934
    assert_has_ta_k(
        &[],
        &sham,
        Lrt,
        &["SAmizyate", "Samizyate", "Samayizyate", "SAmayizyate"],
    );
    // aSAmayizyata is allowed per DP 1.0934.
    assert_has_ta_k(
        &[],
        &sham,
        Lrn,
        &["aSAmizyata", "aSamizyata", "aSamayizyata", "aSAmayizyata"],
    );
    assert_has_ta_k(&[], &han, Lrt, &["GAnizyate", "hanizyate"]);
    assert_has_ta_k(&[], &han, Lrn, &["aGAnizyata", "ahanizyata"]);
    assert_has_ta_k(&[], &grah, Lrt, &["grAhizyate", "grahIzyate"]);
    assert_has_ta_k(&[], &grah, Lrn, &["agrAhizyata", "agrahIzyata"]);
    assert_has_ta_k(&[], &drsh, Lrt, &["darSizyate", "drakzyate"]);
    assert_has_ta_k(&[], &drsh, Lrn, &["adarSizyata", "adrakzyata"]);

    // sic
    assert_has_aataam_k(&[], &ci, Lun, &["acAyizAtAm", "acezAtAm"]);
    assert_has_aataam_k(&[], &daa, Lun, &["adAyizAtAm", "adizAtAm"]);
    // aSAmayizAtAm is allowed per DP 1.0934.
    assert_has_aataam_k(
        &[],
        &sham,
        Lun,
        &["aSAmizAtAm", "aSamizAtAm", "aSamayizAtAm", "aSAmayizAtAm"],
    );
    assert_has_aataam_k(&[], &han, Lun, &["aGAnizAtAm", "avaDizAtAm", "ahasAtAm"]);
    assert_has_aataam_k(&[], &grah, Lun, &["agrAhizAtAm", "agrahIzAtAm"]);

    // sIyut
    assert_has_ta_k(&[], &ci, AshirLin, &["cAyizIzwa", "cezIzwa"]);
    assert_has_ta_k(&[], &daa, AshirLin, &["dAyizIzwa", "dAsIzwa"]);
    // SAmayizIzwa is allowed per DP 1.0934.
    assert_has_ta_k(
        &[],
        &sham,
        AshirLin,
        &["SAmizIzwa", "SamizIzwa", "SamayizIzwa", "SAmayizIzwa"],
    );
    // TODO: broken below this line.
    assert_has_ta_k(&[], &han, AshirLin, &["GAnizIzwa", "vaDizIzwa"]);
    assert_has_ta_k(&[], &grah, AshirLin, &["grAhizIzwa", "grahIzIzwa"]);
    assert_has_ta_k(&[], &drsh, AshirLin, &["darSizIzwa", "dfkzIzwa"]);

    // tAsi
    assert_has_ta_k(&[], &ci, Lut, &["cAyitA", "cetA"]);
    assert_has_ta_k(&[], &daa, Lut, &["dAyitA", "dAtA"]);
    assert_has_ta_k(&[], &sham, Lut, &["SAmitA", "SamitA", "SamayitA"]);
    assert_has_ta_k(&[], &han, Lut, &["GAnitA", "hantA"]);
    assert_has_ta_k(&[], &grah, Lut, &["grAhitA", "grahItA"]);
    assert_has_ta_k(&[], &drsh, Lut, &["darSitA", "drazwA"]);

    // sya-sic-sIyut-tAsi?
    assert_has_krdanta(&[], &ci, Krt::tavya, &["cetavya"]);
    assert_has_krdanta(&[], &daa, Krt::tavya, &["dAtavya"]);

    // bhAvakarmaNoH?
    assert_has_tip(&[], &ci, Lrt, &["cezyati"]);
    assert_has_tip(&[], &daa, Lrt, &["dAsyati"]);

    // upadeSe?
    assert_has_ta_k(&[], &d("qukf\\Y", Tanadi), Lrt, &["kArizyate", "karizyate"]);

    // ac-hana-graha-dRSAm?
    assert_has_ta_k(&[], &d("paWa~", Bhvadi), Lrt, &["paWizyate"]);
}

#[test]
fn sutra_6_4_63() {
    let di = d("dI\\N", Divadi);
    assert_has_ta(&["upa"], &di, Lit, &["upadidIye"]);
    assert_has_aataam(&["upa"], &di, Lit, &["upadidIyAte"]);
    assert_has_jha(&["upa"], &di, Lit, &["upadidIyire"]);

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

    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_upapada_krdanta("go", &[], &daa, Krt::ka, &["goda"]);
    assert_has_upapada_krdanta("kambala", &[], &daa, Krt::ka, &["kambalada"]);

    // Niti?
    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_krdanta(&["pra"], &daa, Krt::aN, &["pradA"]);
    assert_has_krdanta(&["pra"], &dhaa, Krt::aN, &["praDA"]);

    // ArdhadhAtuke?
    assert_has_jhi(&[], &d("yA\\", Adadi), Lat, &["yAnti"]);
    assert_has_jhi(&[], &d("vA\\", Adadi), Lat, &["vAnti"]);

    // aci
    assert_has_ta_k(&[], &d("glE\\", Bhvadi), Lat, &["glAyate"]);
    assert_has_iw(&[], &d("qudA\\Y", Juhotyadi), AshirLin, &["dAsIya"]);
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
    assert_has_ta_k(&[], &daa, Lat, &["dIyate"]);
    assert_has_ta_k(&[], &dhaa, Lat, &["DIyate"]);
    assert_has_lat(&[], &yan(&daa), &["dedIyate"]);
    assert_has_lat(&[], &yan(&dhaa), &["deDIyate"]);

    let maa = d("mA\\", Adadi);
    assert_has_ta_k(&[], &maa, Lat, &["mIyate"]);
    assert_has_lat(&[], &yan(&maa), &["memIyate"]);

    let sthaa = d("zWA\\", Bhvadi);
    assert_has_ta_k(&[], &sthaa, Lat, &["sTIyate"]);
    assert_has_lat(&[], &yan(&sthaa), &["tezWIyate"]);

    let gai = d("gE\\", Bhvadi);
    let i = d("i\\N", Adadi);
    assert_has_ta_k(&[], &gai, Lat, &["gIyate"]);
    assert_has_lat(&[], &yan(&gai), &["jegIyate"]);
    assert_has_ta(&["aDi"], &i, Lun, &["aDyEzwa", "aDyagIzwa"]);
    assert_has_aataam(&["aDi"], &i, Lun, &["aDyEzAtAm", "aDyagIzAtAm"]);
    assert_has_jha(&["aDi"], &i, Lun, &["aDyEzata", "aDyagIzata"]);

    let paa = d("pA\\", Bhvadi);
    let paa_paati = d("pA\\", Adadi);
    assert_has_ta_k(&[], &paa, Lat, &["pIyate"]);
    assert_has_lat(&[], &yan(&paa), &["pepIyate"]);
    assert_has_ta_k(&[], &paa_paati, Lat, &["pAyate"]);

    let haak = d("o~hA\\k", Juhotyadi);
    let haan = d("o~hA\\N", Juhotyadi);
    assert_has_ta_k(&[], &haak, Lat, &["hIyate"]);
    assert_has_lat(&[], &yan(&haak), &["jehIyate"]);
    assert_has_ta_k(&[], &haan, Lat, &["hAyate"]);

    let so = d("zo\\", Divadi);
    assert_has_ta_k(&["ava"], &so, Lat, &["avasIyate"]);
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
    assert_has_tip(&[], &daa, AshirLin, &["deyAt"]);
    assert_has_tip(&[], &dhaa, AshirLin, &["DeyAt"]);
    assert_has_tip(&[], &d("mA\\", Adadi), AshirLin, &["meyAt"]);
    assert_has_tip(&[], &d("zWA\\", Bhvadi), AshirLin, &["sTeyAt"]);
    assert_has_tip(&[], &d("gE\\", Bhvadi), AshirLin, &["geyAt"]);
    assert_has_tip(&[], &d("pA\\", Bhvadi), AshirLin, &["peyAt"]);
    assert_has_tip(&[], &d("o~hA\\k", Juhotyadi), AshirLin, &["heyAt"]);
    assert_has_tip(&["ava"], &d("zo\\", Divadi), AshirLin, &["avaseyAt"]);

    // kNiti
    assert_has_ta(&[], &daa, AshirLin, &["dAsIzwa"]);
    assert_has_ta(&[], &dhaa, AshirLin, &["DAsIzwa"]);
}

#[test]
fn sutra_6_4_68() {
    let glai = d("glE\\", Bhvadi);
    assert_has_tip(&[], &glai, AshirLin, &["gleyAt", "glAyAt"]);
    assert_has_tip(&[], &d("mlE\\", Bhvadi), AshirLin, &["mleyAt", "mlAyAt"]);
    // anyasya
    assert_has_tip(&[], &d("zWA\\", Bhvadi), AshirLin, &["sTeyAt"]);
    // saMyogAdeH
    assert_has_tip(&[], &d("yA\\", Adadi), AshirLin, &["yAyAt"]);
    // kNiti
    assert_has_ta_k(&[], &glai, AshirLin, &["glAsIzwa", "glAyizIzwa"]);
    // aNgasya
    assert_has_tip(&["nis"], &d("vA\\", Adadi), AshirLin, &["nirvAyAt"]);
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

    assert_has_tip(&[], &kf, Lun, &["akArzIt"]);
    assert_has_tip(&[], &hf, Lun, &["ahArzIt"]);
    assert_has_tip(&[], &kf, Lan, &["akarot"]);
    assert_has_tip(&[], &hf, Lan, &["aharat"]);
    assert_has_tip(&[], &kf, Lrn, &["akarizyat"]);
    assert_has_tip(&[], &hf, Lrn, &["aharizyat"]);
}

#[test]
fn sutra_6_4_72() {
    let ikz = d("Ikza~\\", Bhvadi);
    let ih = d("Iha~\\", Bhvadi);
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
fn sutra_6_4_74() {
    fn assert_has_no_agama(
        prefixes: &[&str],
        dhatu: &Dhatu,
        pada: DhatuPada,
        lakara: Lakara,
        expected: &[&str],
    ) {
        use vidyut_prakriya::Vyakarana;

        let args = Tinanta::builder()
            .dhatu(dhatu.clone().with_prefixes(prefixes))
            .prayoga(Prayoga::Kartari)
            .purusha(Purusha::Prathama)
            .vacana(Vacana::Eka)
            .lakara(lakara)
            .pada(pada)
            .skip_at_agama(true)
            .build()
            .unwrap();
        let t = Tester::new(Vyakarana::new());
        t.assert_has_tinantas(&args, expected);
    }

    let assert_has_tip_no_agama = |a, b, c, d| assert_has_no_agama(a, b, DhatuPada::Parasmai, c, d);
    let assert_has_ta_no_agama = |a, b, c, d| assert_has_no_agama(a, b, DhatuPada::Atmane, c, d);

    let kf = d("qukf\\Y", Tanadi);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_tip_no_agama(&[], &kf, Lun, &["kArzIt"]);
    assert_has_tip_no_agama(&[], &hf, Lun, &["hArzIt"]);
    assert_has_tip_no_agama(&[], &kf, Lan, &["karot"]);
    assert_has_tip_no_agama(&[], &hf, Lan, &["harat"]);

    let ikz = d("Ikza~\\", Bhvadi);
    let ih = d("Iha~\\", Bhvadi);
    assert_has_ta_no_agama(&[], &ih, Lun, &["Ihizwa"]);
    assert_has_ta_no_agama(&[], &ikz, Lun, &["Ikzizwa"]);
    assert_has_ta_no_agama(&[], &ih, Lan, &["Ihata"]);
    assert_has_ta_no_agama(&[], &ikz, Lan, &["Ikzata"]);
}

#[test]
fn sutra_6_4_77() {
    // Snu
    let aap = d("A\\px~", Svadi);
    let radh = d("rA\\Da~", Svadi);
    let shak = d("Sa\\kx~", Svadi);
    assert_has_jhi(&[], &aap, Lat, &["Apnuvanti"]);
    assert_has_jhi(&[], &radh, Lat, &["rADnuvanti"]);
    assert_has_jhi(&[], &shak, Lat, &["Saknuvanti"]);

    // DAtu
    let kzi = d("kzi\\", Bhvadi);
    let lu = d("lUY", Kryadi);
    let ni_kvip = create_krdanta("nI", &[], &d("RI\\Y", Bhvadi), Krt::kvip);
    let lu_kvip = create_krdanta("lU", &[], &lu, Krt::kvip);
    assert_has_tas(&[], &kzi, Lit, &["cikziyatuH"]);
    assert_has_jhi(&[], &kzi, Lit, &["cikziyuH"]);
    assert_has_tas(&[], &lu, Lit, &["luluvatuH"]);
    assert_has_jhi(&[], &lu, Lit, &["luluvuH"]);
    assert_has_sup_1d(&ni_kvip, Pum, &["niyO"]);
    assert_has_sup_1p(&ni_kvip, Pum, &["niyaH"]);
    assert_has_sup_1d(&lu_kvip, Pum, &["luvO"]);
    assert_has_sup_1p(&lu_kvip, Pum, &["luvaH"]);

    // BrU
    assert_has_sup_1d("BrU", Stri, &["BruvO"]);
    assert_has_sup_1p("BrU", Stri, &["BruvaH"]);

    // aci?
    assert_has_tip(&[], &aap, VidhiLin, &["ApnuyAt"]);
    assert_has_tip(&[], &shak, VidhiLin, &["SaknuyAt"]);
    assert_has_tip(&[], &d("sA\\Da~", Svadi), VidhiLin, &["sADnuyAt"]);

    // SnuDAtuBruvAm?
    assert_has_sup_4s("lakzmI", Stri, &["lakzmyE"]);
    assert_has_sup_4s("vaDU", Stri, &["vaDvE"]);

    // yvoH?
    let kr = d("qukf\\Y", Tanadi);
    assert_has_tas(&[], &kr, Lit, &["cakratuH"]);
    assert_has_jhi(&[], &kr, Lit, &["cakruH"]);

    // guna-vrddhi
    let ci = d("ci\\Y", Svadi);
    assert_has_krdanta(&[], &ci, Krt::lyuw, &["cayana"]);
    assert_has_krdanta(&[], &ci, Krt::Rvul, &["cAyaka"]);

    assert_has_krdanta(&[], &lu, Krt::lyuw, &["lavana"]);
    assert_has_krdanta(&[], &lu, Krt::Rvul, &["lAvaka"]);
}

#[test]
fn sutra_6_4_78() {
    let iz = d("izu~", Tudadi);
    let uz = d("uza~", Bhvadi);
    assert_has_lit(&[], &iz, &["iyeza"]);
    assert_has_lit(&[], &uz, &["uvoza", "ozAYcakAra", "ozAmAsa", "ozAmbaBUva"]);
    assert_has_lat(&[], &d("f\\", Juhotyadi), &["iyarti"]);

    // asavarRe
    assert_has_tas(&[], &iz, Lit, &["IzatuH"]);
    assert_has_jhi(&[], &iz, Lit, &["IzuH"]);
    assert_has_tas(
        &[],
        &uz,
        Lit,
        &["UzatuH", "ozAYcakratuH", "ozAmAsatuH", "ozAmbaBUvatuH"],
    );
    assert_has_jhi(
        &[],
        &uz,
        Lit,
        &["UzuH", "ozAYcakruH", "ozAmAsuH", "ozAmbaBUvuH"],
    );

    // aci
    assert_has_tip(&[], &d("ya\\ja~^", Bhvadi), Lit, &["iyAja"]);
    assert_has_tip(&[], &d("quva\\pa~^", Bhvadi), Lit, &["uvApa"]);
}

#[test]
fn sutra_6_4_79() {
    let stri = nyap("strI");
    assert_has_sup_1s(&stri, Stri, &["strI"]);
    assert_has_sup_1d(&stri, Stri, &["striyO"]);
    assert_has_sup_1p(&stri, Stri, &["striyaH"]);
}

#[test]
fn sutra_6_4_80() {
    let stri = nyap("strI");
    assert_has_sup_2s(&stri, Stri, &["strIm", "striyam"]);
    assert_has_sup_2p(&stri, Stri, &["strIH", "striyaH"]);
}

#[test]
fn sutra_6_4_81() {
    let i = d("i\\R", Adadi);
    assert_has_jhi(&[], &i, Lat, &["yanti"]);
    assert_has_jhi(&[], &i, Lot, &["yantu"]);
    assert_has_jhi(&[], &i, Lan, &["Ayan"]);
    assert_has_krdanta(&[], &i, Krt::lyuw, &["ayana"]);
    assert_has_krdanta(&[], &i, Krt::Rvul, &["Ayaka"]);
}

#[ignore]
#[test]
fn sutra_6_4_82() {
    let ni = d("RI\\Y", Bhvadi);
    assert_has_tas(&[], &ni, Lit, &["ninyatuH"]);
    assert_has_jhi(&[], &ni, Lit, &["ninyuH"]);

    let unni = create_krdanta("unnI", &["ud"], &ni, Krt::kvip);
    assert_has_sup_1d(&unni, Pum, &["unnyO"]);
    assert_has_sup_1p(&unni, Pum, &["unnyaH"]);
}

#[ignore]
#[test]
fn sutra_6_4_83() {
    assert_has_sup_1d("KalapU", Pum, &["KalapvO"]);
    assert_has_sup_1p("KalapU", Pum, &["KalapvaH"]);
    assert_has_sup_1d("SatasU", Pum, &["SatasvO"]);
    assert_has_sup_1p("SatasU", Pum, &["SatasvaH"]);
    assert_has_sup_1d("sakfllU", Pum, &["sakfllvO"]);
    assert_has_sup_1p("sakfllU", Pum, &["sakfllvaH"]);
    // supi
    let lu = d("lUY", Kryadi);
    assert_has_tas(&[], &lu, Lit, &["luluvatuH"]);
    assert_has_jhi(&[], &lu, Lit, &["luluvuH"]);
    // asaMyogapUrva
    assert_has_sup_1d("kawaprU", Pum, &["kawapruvO"]);
    assert_has_sup_1p("kawaprU", Pum, &["kawapruvaH"]);

    // TODO: luvO, luvaH, etc.
}

#[test]
fn sutra_6_4_84() {
    assert_has_sup_1d("varzABU", Pum, &["varzABvO"]);
    assert_has_sup_1p("varzABU", Pum, &["varzABvaH"]);
}

#[test]
fn sutra_6_4_84_v1() {
    let punarbhu = create_bahuvrihi("punarBU", "punar", "BU");
    assert_has_sup_1d(&punarbhu, Stri, &["punarBvO"]);
    assert_has_sup_1p(&punarbhu, Stri, &["punarBvaH"]);

    let karabhu = create_bahuvrihi("kAraBU", "kAra", "BU");
    assert_has_sup_1d(&karabhu, Stri, &["kAraBvO"]);
    assert_has_sup_1p(&karabhu, Stri, &["kAraBvaH"]);
}

#[test]
fn sutra_6_4_85() {
    let pratibhu = create_upapada_krdanta("pratiBU", "prati", &[], &d("BU", Bhvadi), Krt::kvip);
    assert_has_sup_1d(&pratibhu, Stri, &["pratiBuvO"]);
    assert_has_sup_1p(&pratibhu, Stri, &["pratiBuvaH"]);
}

#[test]
fn sutra_6_4_87() {
    let hu = d("hu\\", Juhotyadi);
    assert_has_jhi(&[], &hu, Lat, &["juhvati"]);
    assert_has_jhi(&[], &hu, Lot, &["juhvatu"]);
    assert_has_krdanta(&[], &hu, Krt::Satf, &["juhvat"]);

    let su = d("zu\\Y", Svadi);
    assert_has_jhi(&[], &su, Lat, &["sunvanti"]);
    assert_has_jhi(&[], &su, Lot, &["sunvantu"]);
    assert_has_jhi(&[], &su, Lan, &["asunvan"]);

    // huSnuvoH?
    assert_has_jhi(&[], &yan_luk(&d("yu", Adadi)), Lat, &["yoyuvati"]);
    assert_has_jhi(&[], &yan_luk(&d("ru", Adadi)), Lat, &["roruvati"]);

    // sArvadhAtuke?
    assert_has_tas(
        &[],
        &hu,
        Lit,
        &[
            "juhuvatuH",
            "juhavAYcakratuH",
            "juhavAmAsatuH",
            "juhavAmbaBUvatuH",
        ],
    );
    assert_has_jhi(
        &[],
        &hu,
        Lit,
        &["juhuvuH", "juhavAYcakruH", "juhavAmAsuH", "juhavAmbaBUvuH"],
    );

    // asaMyogapUrvasya?
    assert_has_jhi(&[], &d("A\\px~", Svadi), Lat, &["Apnuvanti"]);
    assert_has_jhi(&[], &d("rA\\Da~", Svadi), Lat, &["rADnuvanti"]);
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
    assert_has_tip(&["ni"], &guh, Lat, &["nigUhati"]);
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
    assert_has_tip(&[], &dus, Lat, &["dUzayati", "dozayati"]);
}

#[test]
fn sutra_6_4_92() {
    let d_g = d_ghatadi;
    assert_has_tip(&[], &nic(&d_g("Gawa~\\", Bhvadi)), Lat, &["Gawayati"]);
    assert_has_tip(&[], &nic(&d_g("vyaTa~\\", Bhvadi)), Lat, &["vyaTayati"]);
    assert_has_tip(&[], &nic(&d("janI~\\", Divadi)), Lat, &["janayati"]);
    assert_has_tip(
        &[],
        &nic(&d("ra\\nja~^", Divadi)),
        Lat,
        &["rajayati", "raYjayati"],
    );

    assert_has_tip(&[], &nic(&d("Samu~", Bhvadi)), Lat, &["Samayati"]);
    assert_has_tip(&[], &d("jYapa~", Curadi), Lat, &["jYapayati"]);
}

#[test]
fn sutra_6_4_93() {
    let sham = d("Samu~", Bhvadi);
    let tam = d("tamu~", Divadi);
    assert_has_ta_k(&[], &nic(&sham), Lun, &["aSami", "aSAmi"]);
    assert_has_ta_k(&[], &nic(&tam), Lun, &["atami", "atAmi"]);

    assert_has_krdanta(&[], &nic(&sham), Krt::Ramul, &["Samam", "SAmam"]);
    assert_has_krdanta(&[], &nic(&tam), Krt::Ramul, &["tamam", "tAmam"]);

    let nic_nic = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Ric, Sanadi::Ric]);
    assert_has_krdanta(&[], &nic_nic(&sham), Krt::Satf, &["Samayat"]);
    assert_has_ta_k(&[], &nic_nic(&sham), Lun, &["aSami", "aSAmi"]);
    assert_has_krdanta(&[], &nic_nic(&sham), Krt::Ramul, &["Samam", "SAmam"]);

    assert_has_tip(&[], &yan_nic(&sham), Lat, &["SaMSamayati"]);
    assert_has_ta_k(&[], &yan_nic(&sham), Lun, &["aSaMSami", "aSaMSAmi"]);
    assert_has_krdanta(&[], &yan_nic(&sham), Krt::Ramul, &["SaMSamam", "SaMSAmam"]);
}

#[test]
fn sutra_6_4_94() {
    let taapi = nic(&d("ta\\pa~", Bhvadi));
    assert_has_upapada_krdanta("dvizat", &[], &taapi, Krt::Kac, &["dvizantapa"]);
    assert_has_upapada_krdanta("para", &[], &taapi, Krt::Kac, &["parantapa"]);

    let daari = nic(&d("dF", Bhvadi));
    assert_has_upapada_krdanta("pur", &[], &daari, Krt::Kac, &["purandara"]);
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

#[ignore]
#[test]
fn sutra_6_4_96_v1() {
    let chad = d("Cada~", Curadi);
    assert_has_krdanta(&["sam", "upa", "ati"], &chad, Krt::Ga, &["samupAticCAda"]);
}

#[ignore]
#[test]
fn sutra_6_4_97() {
    let chad = d("Cada~", Curadi);
    assert_has_krdanta(&[], &chad, Unadi::isi, &["Cadis"]);
    assert_has_krdanta(&[], &chad, Krt::manin, &["Cadman"]);
    assert_has_krdanta(&[], &chad, Unadi::zwran, &["Cattra"]);
    // TODO: implement 8.2.2 correctly
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
    assert_has_ta_k(&[], &gam, Lat, &["gamyate"]);
    assert_has_ta_k(&[], &han, Lat, &["hanyate"]);
}

#[test]
fn sutra_6_4_101() {
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
    assert_has_ta_k(&[], &d("qukf\\Y", Tanadi), Lun, &["akAri"]);
    assert_has_ta_k(&[], &d("hf\\Y", Bhvadi), Lun, &["ahAri"]);
    assert_has_ta_k(&[], &d("lUY", Kryadi), Lun, &["alAvi"]);
    assert_has_ta_k(&[], &d("qupa\\ca~^z", Bhvadi), Lun, &["apAci"]);

    // ciR
    assert_has_ta_k(&[], &d("Bi\\di~^r", Rudhadi), Lat, &["Bidyate"]);

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
    assert_has_tip(&[], &bhid, Lat, &["Binatti"]);
    assert_has_tip(&[], &as_, Lat, &["asti"]);
}

#[test]
fn sutra_6_4_112() {
    let lu = d("lUY", Kryadi);
    assert_has_jha(&[], &lu, Lat, &["lunate"]);
    assert_has_jha(&[], &lu, Lot, &["lunatAm"]);
    assert_has_jha(&[], &lu, Lan, &["alunata"]);

    let maa = d("mA\\N", Juhotyadi);
    assert_has_jha(&[], &maa, Lat, &["mimate"]);
    assert_has_jha(&[], &maa, Lot, &["mimatAm"]);
    assert_has_jha(&[], &maa, Lan, &["amimata"]);

    let haa = d("o~hA\\N", Juhotyadi);
    assert_has_jha(&["sam"], &haa, Lat, &["saYjihate"]);
    assert_has_jha(&["sam"], &haa, Lot, &["saYjihatAm"]);
    assert_has_jha(&["sam"], &haa, Lan, &["samajihata"]);

    // SnAByastayoH
    assert_has_jhi(&[], &d("yA\\", Adadi), Lat, &["yAnti"]);
    assert_has_jhi(&[], &d("vA\\", Adadi), Lat, &["vAnti"]);

    // AtaH
    assert_has_jhi(&[], &d("quBf\\Y", Juhotyadi), Lat, &["biBrati"]);

    // kNiti
    assert_has_tip(&[], &lu, Lan, &["alunAt"]);
}

#[test]
fn sutra_6_4_113() {
    let lu = d("lUY", Kryadi);
    let pu = d("pUY", Kryadi);
    assert_has_tas(&[], &lu, Lat, &["lunItaH"]);
    assert_has_tas(&[], &pu, Lat, &["punItaH"]);
    assert_has_thas(&[], &lu, Lat, &["lunITaH"]);
    assert_has_thas(&[], &pu, Lat, &["punITaH"]);
    assert_has_ta(&[], &lu, Lat, &["lunIte"]);
    assert_has_ta(&[], &pu, Lat, &["punIte"]);

    let maa = d("mA\\N", Juhotyadi);
    assert_has_ta(&[], &maa, Lat, &["mimIte"]);
    assert_has_thaas(&[], &maa, Lat, &["mimIze"]);
    assert_has_dhvam(&[], &maa, Lat, &["mimIDve"]);

    let haa = d("o~hA\\N", Juhotyadi);
    assert_has_ta(&["sam"], &haa, Lat, &["saYjihIte"]);
    assert_has_thaas(&["sam"], &haa, Lat, &["saYjihIze"]);
    assert_has_dhvam(&["sam"], &haa, Lat, &["saYjihIDve"]);

    // hali
    assert_has_jhi(&[], &lu, Lat, &["lunanti"]);
    assert_has_jha(&[], &maa, Lat, &["mimate"]);

    // aGoH
    assert_has_tas(&[], &d("qudA\\Y", Juhotyadi), Lat, &["dattaH"]);
    assert_has_tas(&[], &d("quDA\\Y", Juhotyadi), Lat, &["DattaH"]);

    // kNiti
    assert_has_tip(&[], &lu, Lat, &["lunAti"]);
    assert_has_tip(&[], &d("o~hA\\k", Juhotyadi), Lat, &["jahAti"]);
}

#[test]
fn sutra_6_4_114() {
    let daridra = d("daridrA", Adadi);
    assert_has_tas(&[], &daridra, Lat, &["daridritaH"]);
    assert_has_thas(&[], &daridra, Lat, &["daridriTaH"]);
    assert_has_vas(&[], &daridra, Lat, &["daridrivaH"]);
    assert_has_mas(&[], &daridra, Lat, &["daridrimaH"]);

    // hali
    assert_has_jhi(&[], &daridra, Lat, &["daridrati"]);

    // kNiti
    assert_has_tip(&[], &daridra, Lat, &["daridrAti"]);
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
    assert_has_ta_k(&[], &bhi, Lat, &["BIyate"]);
}

#[test]
fn sutra_6_4_116() {
    let haa = d("o~hA\\k", Juhotyadi);
    assert_has_tas(&[], &haa, Lat, &["jahitaH", "jahItaH"]);
    assert_has_thas(&[], &haa, Lat, &["jahiTaH", "jahITaH"]);
    assert_has_vas(&[], &haa, Lat, &["jahivaH", "jahIvaH"]);
    assert_has_mas(&[], &haa, Lat, &["jahimaH", "jahImaH"]);

    // hali
    assert_has_jhi(&[], &haa, Lat, &["jahati"]);

    // kNiti
    assert_has_tip(&[], &haa, Lat, &["jahAti"]);

    // sArvadhAtuke
    assert_has_ta_k(&[], &haa, Lat, &["hIyate"]);
    assert_has_lat(&[], &yan(&haa), &["jehIyate"]);
}

#[test]
fn sutra_6_4_117() {
    let haa = d("o~hA\\k", Juhotyadi);
    assert_has_sip(
        &[],
        &haa,
        Lot,
        &["jahAhi", "jahihi", "jahIhi", "jahitAt", "jahItAt"],
    );
}

#[test]
fn sutra_6_4_118() {
    let haa = d("o~hA\\k", Juhotyadi);
    assert_has_tip(&[], &haa, VidhiLin, &["jahyAt"]);
    assert_has_tas(&[], &haa, VidhiLin, &["jahyAtAm"]);
    assert_has_jhi(&[], &haa, VidhiLin, &["jahyuH"]);
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
    assert_has_ta(&[], &ras, Lit, &["rarAse"]);
    assert_has_aataam(&[], &ras, Lit, &["rarAsAte"]);
    assert_has_jha(&[], &ras, Lit, &["rarAsire"]);

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
    assert_has_ta(&[], &sah, Lit, &["sehe"]);
    assert_has_aataam(&[], &sah, Lit, &["sehAte"]);
    assert_has_jha(&[], &sah, Lit, &["sehire"]);

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
    // SK justifies tatazWa
    assert_has_sip(&[], &d("takzU~", Bhvadi), Lit, &["tatakziTa", "tatazWa"]);
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
    let jf = d("jFz", Divadi);
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
fn sutra_6_4_130() {
    let dvipad = create_bahuvrihi("dvipAd", "dvi", "pAda");
    assert_has_sup_2p(&dvipad, Pum, &["dvipadaH"]);
    assert_has_sup_3s(&dvipad, Pum, &["dvipadA"]);
    assert_has_sup_4s(&dvipad, Pum, &["dvipade"]);

    let tripad = create_bahuvrihi("tripAd", "tri", "pAda");
    let vyaghrapad = create_bahuvrihi("vyAGrapad", "vyAGra", "pAda");
    assert_has_taddhita(&dvipad, T::vun, &["dvipadika"]);
    assert_has_taddhita(&tripad, T::vun, &["tripadika"]);
    assert_has_taddhita(&vyaghrapad, T::yak, &["vEyAGrapadya"]);
}

#[test]
fn sutra_6_4_131() {
    let vidvas = krdanta(&[], &d("vida~", Adadi), Krt::Satf);
    assert_has_sup_2p(&vidvas, Pum, &["viduzaH", "vidataH"]);
    assert_has_sup_3s(&vidvas, Pum, &["viduzA", "vidatA"]);
    assert_has_sup_4s(&vidvas, Pum, &["viduze", "vidate"]);

    let pecivas = krdanta(&[], &d("qupa\\ca~^z", Adadi), Krt::kvasu);
    assert_has_sup_2p(&pecivas, Pum, &["pecuzaH"]);
    assert_has_sup_3s(&pecivas, Pum, &["pecuzA"]);
    assert_has_sup_4s(&pecivas, Pum, &["pecuze"]);

    let papivas = krdanta(&[], &d("pA\\", Bhvadi), Krt::kvasu);
    assert_has_sup_2p(&papivas, Pum, &["papuzaH"]);
}

#[test]
fn sutra_6_4_132() {
    let vah = d("va\\ha~^", Bhvadi);

    let prasthavah = create_krdanta("prazWavAh", &["prazWa"], &vah, Krt::Rvi);
    assert_has_sup_2p(&prasthavah, Pum, &["prazWOhaH"]);
    assert_has_sup_3s(&prasthavah, Pum, &["prazWOhA"]);
    assert_has_sup_4s(&prasthavah, Pum, &["prazWOhe"]);

    let dityavah = create_krdanta("dityavAh", &["ditya"], &vah, Krt::Rvi);
    assert_has_sup_2p(&dityavah, Pum, &["dityOhaH"]);
    assert_has_sup_3s(&dityavah, Pum, &["dityOhA"]);
    assert_has_sup_4s(&dityavah, Pum, &["dityOhe"]);
}

#[test]
fn sutra_6_4_133() {
    assert_has_sup_6s("Svan", Pum, &["SunaH"]);
    assert_has_sup_3s("Svan", Pum, &["SunA"]);
    assert_has_sup_4s("Svan", Pum, &["Sune"]);

    assert_has_sup_6s("yuvan", Pum, &["yUnaH"]);
    assert_has_sup_3s("yuvan", Pum, &["yUnA"]);
    assert_has_sup_4s("yuvan", Pum, &["yUne"]);

    assert_has_sup_6s("maGavan", Pum, &["maGonaH"]);
    assert_has_sup_3s("maGavan", Pum, &["maGonA"]);
    assert_has_sup_4s("maGavan", Pum, &["maGone"]);

    // atadDite?
    assert_has_taddhita("Svan", T::aY, &["SOva"]);
    assert_has_taddhita("yuvan", T::aR, &["yOvana"]);
    assert_has_taddhita("maGavan", T::aR, &["mAGavana"]);

    // TODO: others
}

#[test]
fn sutra_6_4_134() {
    assert_has_sup_6s("rAjan", Pum, &["rAjYaH"]);
    assert_has_sup_3s("rAjan", Pum, &["rAjYA"]);
    assert_has_sup_4s("rAjan", Pum, &["rAjYe"]);

    assert_has_sup_3s("takzan", Pum, &["takzRA"]);
    assert_has_sup_4s("takzan", Pum, &["takzRe"]);
}

#[ignore]
#[test]
fn sutra_6_4_135() {
    use TaddhitaArtha::TasyaApatyam;
    assert_has_artha_taddhita("ukzan", TasyaApatyam, T::aR, &["OkzRa"]);
    let bhrunahan = upapada_krdanta("BrURa", &[], &d("ha\\na~", Adadi), Krt::kvip);
    assert_has_artha_taddhita(bhrunahan, TasyaApatyam, T::aR, &["BrORaGna"]);
    assert_has_artha_taddhita("DftarAjan", TasyaApatyam, T::aR, &["DArtarAjYa"]);
    assert_has_artha_taddhita("sAman", TasyaApatyam, T::aR, &["sAmana"]);
    assert_has_artha_taddhita("viman", TasyaApatyam, T::aR, &["vEmana"]);
}

#[test]
fn sutra_6_4_136() {
    assert_has_sup_7s("rAjan", Pum, &["rAjYi", "rAjani"]);
    assert_has_sup_7s("sAman", Napumsaka, &["sAmni", "sAmani"]);
    assert_has_sup_1d("sAman", Napumsaka, &["sAmnI", "sAmanI"]);
}

#[test]
fn sutra_6_4_137() {
    assert_has_sup_3s("parvan", Pum, &["parvaRA"]);
    assert_has_sup_4s("parvan", Pum, &["parvaRe"]);

    assert_has_sup_3s("aTarvan", Pum, &["aTarvaRA"]);
    assert_has_sup_4s("aTarvan", Pum, &["aTarvaRe"]);

    // saMyogAt?
    // TODO
    // assert_has_sup_3s("pratidIvan", Napumsaka, &["pratidIvnA"]);
    // assert_has_sup_4s("pratidIvan", Napumsaka, &["pratidIvne"]);
    assert_has_sup_3s("sAman", Napumsaka, &["sAmnA"]);
    assert_has_sup_4s("sAman", Napumsaka, &["sAmne"]);

    // vamAntAt?
    assert_has_sup_3s("takzan", Pum, &["takzRA"]);
    assert_has_sup_4s("takzan", Pum, &["takzRe"]);
}

#[test]
fn sutra_6_4_138() {
    let dadhyac = create_upapada_krdanta("daDyac", "daDi", &[], &d("ancu~", Bhvadi), Krt::kvin);
    assert_has_sup_2p(&dadhyac, Pum, &["daDIcaH"]);
    assert_has_sup_3s(&dadhyac, Pum, &["daDIcA"]);
    assert_has_sup_4s(&dadhyac, Pum, &["daDIce"]);

    let madhvac = create_upapada_krdanta("maDvac", "maDu", &[], &d("ancu~", Bhvadi), Krt::kvin);
    assert_has_sup_2p(&madhvac, Pum, &["maDUcaH"]);
    assert_has_sup_3s(&madhvac, Pum, &["maDUcA"]);
    assert_has_sup_4s(&madhvac, Pum, &["maDUce"]);
}

#[test]
fn sutra_6_4_140() {
    let kilalapa = create_upapada_krdanta("kilAlapA", "kilAla", &[], &d("pA\\", Adadi), Krt::vic);
    assert_has_sup_2p(&kilalapa, Pum, &["kilAlapaH"]);
    assert_has_sup_3s(&kilalapa, Pum, &["kilAlapA"]);
    assert_has_sup_4s(&kilalapa, Pum, &["kilAlape"]);

    // TODO: how to derive shubhamya? For now, use SuBam as a placeholder so the test works.
    let shubhamya = create_upapada_krdanta("SuBaMyA", "SuBam", &[], &d("yA\\", Adadi), Krt::vic);
    assert_has_sup_2p(&shubhamya, Pum, &["SuBaMyaH"]);
    assert_has_sup_3s(&shubhamya, Pum, &["SuBaMyA"]);
    assert_has_sup_4s(&shubhamya, Pum, &["SuBaMye"]);

    // AtaH?
    let ni = create_krdanta("nI", &[], &d("RI\\Y", Bhvadi), Krt::kvip);
    assert_has_sup_3s(&ni, Pum, &["niyA"]);
    assert_has_sup_4s(&ni, Pum, &["niye"]);

    // dhAtoH?
    assert_has_sup_2p("KawvA", Stri, &["KawvAH"]);
    assert_has_sup_2p("mAlA", Stri, &["mAlAH"]);
}

#[ignore]
#[test]
fn sutra_6_4_142() {
    assert_has_taddhita("viMSati", T::qvun, &["viMSaka"]);
    // TODO: assert_has_taddhitanta("viMSati", T::waq, &["viMSa"]);

    // qiti?
    assert_has_sup_3s("viMSati", Pum, &["viMSatyA"]);
}

#[ignore]
#[test]
fn sutra_6_4_143() {
    assert_has_taddhita("kumuda", T::qmatup, &["kumudvat"]);
    assert_has_taddhita("naqa", T::qmatup, &["naqvat"]);
    assert_has_taddhita("vetasa", T::qmatup, &["vetasvat"]);
    // TODO: others
    assert_has_taddhita("triMSat", T::qvun, &["triMSaka"]);
}

#[test]
fn sutra_6_4_144() {
    assert_has_taddhita("agniSarman", T::iY, &["AgniSarmi"]);
    assert_has_taddhita("uquloman", T::iY, &["Oqulomi"]);

    // naH?
    // taddhite?
    assert_has_sup_3s("Sarman", Napumsaka, &["SarmaRA"]);
    assert_has_sup_4s("Sarman", Napumsaka, &["SarmaRe"]);
}

#[test]
fn sutra_6_4_144_v1() {
    assert_has_taddhita("sabrahmacArin", T::aR, &["sAbrahmacAra"]);
    assert_has_taddhita("pIWasarpin", T::aR, &["pEWasarpa"]);
    assert_has_taddhita("kalApin", T::aR, &["kAlApa"]);
    assert_has_taddhita("kuTumin", T::aR, &["kOTuma"]);
    assert_has_taddhita("tEtilin", T::aR, &["tEtila"]);
    assert_has_taddhita("jAjalin", T::aR, &["jAjala"]);
    assert_has_taddhita("lANgalin", T::aR, &["lANgala"]);
    assert_has_taddhita("SilAlin", T::aR, &["SElAla"]);
    assert_has_taddhita("SiKaRqin", T::aR, &["SEKaRqa"]);
    assert_has_taddhita("sukarasadman", T::aR, &["sOkarasadma"]);
    assert_has_taddhita("suparvan", T::aR, &["sOparva"]);
}

#[test]
fn sutra_6_4_146() {
    assert_has_taddhita("baBru", T::yaY, &["bABravya"]);
    assert_has_taddhita("maRqu", T::yaY, &["mARqavya"]);
    assert_has_taddhita("SaNku", T::yat, &["SaNkavya"]);
    assert_has_taddhita("picu", T::yat, &["picavya"]);
    assert_has_taddhita("kamaRqalu", T::yat, &["kamaRqalavya"]);
    assert_has_taddhita("upagu", T::aR, &["Opagava"]);
    assert_has_taddhita("kapawu", T::aR, &["kApawava"]);
    // TODO: svAyamBuva
}

#[ignore]
#[test]
fn sutra_6_4_147() {
    assert_has_taddhita("kamaRqalu", T::QaY, &["kAmaRqaleya"]);
    // TODO: what is the pratipadika here?
    assert_has_taddhita("SItabAhu", T::QaY, &["SEtabAheya"]);
    assert_has_taddhita("jambu", T::QaY, &["jAmbeya"]);

    let madrabahu = create_bahuvrihi("madrabAhu", "madra", "bAhu").with_stri(true);
    assert_has_taddhita(&madrabahu, T::Qak, &["mAdrabAheya"]);

    // a-kadrvAH?
    assert_has_taddhita(&nyap("kadrU"), T::Qak, &["kAdraveya"]);
}

#[ignore]
#[test]
fn sutra_6_4_149() {
    assert_has_taddhita("sUrya", T::aR, &["sOra"]);
    assert_has_taddhita("tizya", T::aR, &["tEza"]);
    assert_has_taddhita("agastya", T::aR, &["Agasta"]);
    assert_has_taddhita("agastya", T::Ka, &["AgastIya"]);
    assert_has_sup_1s("matsya", Stri, &["matsI"]);
}

#[test]
fn sutra_6_4_155() {
    assert_has_taddhita("pawu", T::izWan, &["pawizWa"]);
    assert_has_taddhita("pawu", T::imanic, &["pawiman"]);
    assert_has_taddhita("pawu", T::Iyasun, &["pawIyas"]);

    assert_has_taddhita("laGu", T::izWan, &["laGizWa"]);
    assert_has_taddhita("laGu", T::imanic, &["laGiman"]);
    assert_has_taddhita("laGu", T::Iyasun, &["laGIyas"]);
}

#[test]
fn sutra_6_4_156() {
    assert_has_taddhita("sTUla", T::izWan, &["sTavizWa"]);
    assert_has_taddhita("sTUla", T::Iyasun, &["sTavIyas"]);

    assert_has_taddhita("dUra", T::izWan, &["davizWa"]);
    assert_has_taddhita("dUra", T::Iyasun, &["davIyas"]);

    // kanizWa and kanIyas are by 5.3.64.
    assert_has_taddhita("yuvan", T::izWan, &["yavizWa", "kanizWa"]);
    assert_has_taddhita("yuvan", T::Iyasun, &["yavIyas", "kanIyas"]);

    assert_has_taddhita("hrasva", T::izWan, &["hrasizWa"]);
    assert_has_taddhita("hrasva", T::imanic, &["hrasiman"]);
    assert_has_taddhita("hrasva", T::Iyasun, &["hrasIyas"]);

    assert_has_taddhita("kzipra", T::izWan, &["kzepizWa"]);
    assert_has_taddhita("kzipra", T::imanic, &["kzepiman"]);
    assert_has_taddhita("kzipra", T::Iyasun, &["kzepIyas"]);

    assert_has_taddhita("kzudra", T::izWan, &["kzodizWa"]);
    assert_has_taddhita("kzudra", T::imanic, &["kzodiman"]);
    assert_has_taddhita("kzudra", T::Iyasun, &["kzodIyas"]);
}

#[test]
fn sutra_6_4_157() {
    assert_has_taddhita("priya", T::izWan, &["prezWa"]);
    assert_has_taddhita("priya", T::imanic, &["preman"]);
    assert_has_taddhita("priya", T::Iyasun, &["preyas"]);

    assert_has_taddhita("sTira", T::izWan, &["sTezWa"]);
    assert_has_taddhita("sTira", T::imanic, &[]);
    assert_has_taddhita("sTira", T::Iyasun, &["sTeyas"]);

    assert_has_taddhita("sPira", T::izWan, &["sPezWa"]);
    assert_has_taddhita("sPira", T::imanic, &[]);
    assert_has_taddhita("sPira", T::Iyasun, &["sPeyas"]);

    assert_has_taddhita("uru", T::izWan, &["varizWa"]);
    assert_has_taddhita("uru", T::imanic, &["variman"]);
    assert_has_taddhita("uru", T::Iyasun, &["varIyas"]);

    assert_has_taddhita("bahula", T::izWan, &["baMhizWa"]);
    assert_has_taddhita("bahula", T::imanic, &["baMhiman"]);
    assert_has_taddhita("bahula", T::Iyasun, &["baMhIyas"]);

    assert_has_taddhita("guru", T::izWan, &["garizWa"]);
    assert_has_taddhita("guru", T::imanic, &["gariman"]);
    assert_has_taddhita("guru", T::Iyasun, &["garIyas"]);

    // jyezWa and jyAyas are by 5.3.62.
    assert_has_taddhita("vfdDa", T::izWan, &["varzizWa", "jyezWa"]);
    assert_has_taddhita("vfdDa", T::imanic, &[]);
    assert_has_taddhita("vfdDa", T::Iyasun, &["varzIyas", "jyAyas"]);

    assert_has_taddhita("tfpra", T::izWan, &["trapizWa"]);
    assert_has_taddhita("tfpra", T::imanic, &[]);
    assert_has_taddhita("tfpra", T::Iyasun, &["trapIyas"]);

    assert_has_taddhita("dIrGa", T::izWan, &["drAGizWa"]);
    assert_has_taddhita("dIrGa", T::imanic, &["drAGiman"]);
    assert_has_taddhita("dIrGa", T::Iyasun, &["drAGIyas"]);

    assert_has_taddhita("vfndAraka", T::izWan, &["vfndizWa"]);
    assert_has_taddhita("vfndAraka", T::imanic, &[]);
    assert_has_taddhita("vfndAraka", T::Iyasun, &["vfndIyas"]);
}

#[test]
fn sutra_6_4_158() {
    assert_has_taddhita("bahu", T::imanic, &["BUman"]);
    assert_has_taddhita("bahu", T::Iyasun, &["BUyas"]);
}

#[test]
fn sutra_6_4_159() {
    assert_has_taddhita("bahu", T::izWan, &["BUyizWa"]);
}

#[test]
fn sutra_6_4_160() {
    assert_has_taddhita("jya", T::Iyasun, &["jyAyas"]);
}

#[test]
fn sutra_6_4_161() {
    assert_has_taddhita("pfTu", T::izWan, &["praTizWa"]);
    assert_has_taddhita("pfTu", T::imanic, &["praTiman"]);
    assert_has_taddhita("pfTu", T::Iyasun, &["praTIyas"]);

    assert_has_taddhita("mfdu", T::izWan, &["mradizWa"]);
    assert_has_taddhita("mfdu", T::imanic, &["mradiman"]);
    assert_has_taddhita("mfdu", T::Iyasun, &["mradIyas"]);

    // ftaH?
    assert_has_taddhita("pawu", T::izWan, &["pawizWa"]);
    assert_has_taddhita("pawu", T::imanic, &["pawiman"]);
    assert_has_taddhita("pawu", T::Iyasun, &["pawIyas"]);

    // halAdeH?
    assert_has_taddhita("fju", T::izWan, &["fjizWa"]);
    assert_has_taddhita("fju", T::imanic, &["fjiman"]);
    assert_has_taddhita("fju", T::Iyasun, &["fjIyas"]);

    // laGoH?
    assert_has_taddhita("kfzRA", T::izWan, &["kfzRizWa"]);

    // TODO: assert_has_taddhitanta("kfzRA", T::imanic, &["kfzRiman"]);
    assert_has_taddhita("kfzRA", T::Iyasun, &["kfzRIyas"]);
    // TODO: others
}

#[test]
fn sutra_6_4_164() {
    assert_has_taddhita("saNkUwin", T::aR, &["sANkUwina"]);
    assert_has_taddhita("saMrAvin", T::aR, &["sAMrAviRa"]);
    assert_has_taddhita("sammArjin", T::aR, &["sAmmArjina"]);
    assert_has_taddhita("sragvin", T::aR, &["srAgviRa"]);
    // aRi?
    assert_has_taddhita("daRqin", T::aY, &["dARqa"]);
}

#[test]
fn sutra_6_4_165() {
    assert_has_taddhita("gATin", T::aR, &["gATina"]);
    assert_has_taddhita("vidaTin", T::aR, &["vEdaTina"]);
    assert_has_taddhita("keSin", T::aR, &["kESina"]);
    assert_has_taddhita("gaRin", T::aR, &["gARina"]);
    assert_has_taddhita("paRin", T::aR, &["pARina"]);
}

#[test]
fn sutra_6_4_166() {
    assert_has_taddhita("SaNKin", T::aR, &["SANKina"]);
    assert_has_taddhita("madrin", T::aR, &["mAdriRa"]);
    assert_has_taddhita("vajrin", T::aR, &["vAjriRa"]);
}

#[test]
fn sutra_6_4_167() {
    assert_has_taddhita("sAman", T::aR, &["sAmana"]);
    assert_has_taddhita("veman", T::aR, &["vEmana"]);
    assert_has_taddhita("sutvan", T::aR, &["sOtvana"]);
    assert_has_taddhita("jitvan", T::aR, &["jEtvana"]);
}

#[test]
fn sutra_6_4_168() {
    assert_has_artha_taddhita("sAman", TatraSadhu, T::yat, &["sAmanya"]);
    assert_has_artha_taddhita("veman", TatraSadhu, T::yat, &["vemanya"]);
    // TODO: rAjya
}

#[test]
fn sutra_6_4_169() {
    assert_has_artha_taddhita("Atman", TasmaiHitam, T::Ka, &["AtmanIna"]);
    assert_has_artha_taddhita("aDvan", AlamGami, T::Ka, &["aDvanIna"]);
}
