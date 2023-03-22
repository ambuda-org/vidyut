extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

fn nic(u: &str, g: Gana) -> Dhatu {
    d(u, g).with_sanadi(&[Sanadi::Nic])
}

fn prati(text: &str) -> Pratipadika {
    Pratipadika::builder().text(text).build().unwrap()
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
    let yan = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Yan]);
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
    let ranj_nic = nic("ra\\nja~^", Divadi);
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
    let dus = nic("du\\za~", Divadi);
    assert_has_lat_p(&[], &dus, &["dUzayati", "dozayati"]);
}

#[test]
fn sutra_6_4_92() {
    assert_has_lat_p(&[], &nic("Gawa~\\", Bhvadi), &["Gawayati"]);
    assert_has_lat_p(&[], &nic("vyaTa~\\", Bhvadi), &["vyaTayati"]);
    assert_has_lat_p(&[], &nic("janI~\\", Divadi), &["janayati"]);
    assert_has_lat_p(&[], &nic("ra\\nja~^", Divadi), &["rajayati", "raYjayati"]);

    // TODO: not sure
    // assert_has_lat_p(&[], &nic("Samu~", Bhvadi), &["Samayati"]);
    // assert_has_lat_p(&[], &nic("", Bhvadi), &["jYapayati"]);
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
