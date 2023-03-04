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
    let sras = Dhatu::new("sransu~\\", Bhvadi);
    let dhvas = Dhatu::new("Dvansu~\\", Bhvadi);

    assert_has_krdanta(&[], &sras, Krt::kta, &["srasta"]);
    assert_has_krdanta(&[], &dhvas, Krt::kta, &["Dvasta"]);
    assert_has_lat_karmani(&[], &sras, &["srasyate"]);
    assert_has_lat_karmani(&[], &dhvas, &["Dvasyate"]);
    assert_has_lat(&[], &yan(&sras), &["sanIsrasyate"]);
    assert_has_lat(&[], &yan(&dhvas), &["danIDvasyate"]);

    let nand = Dhatu::new("wunadi~", Bhvadi);
    assert_has_lat_karmani(&[], &nand, &["nandyate"]);
    assert_has_lat(&[], &yan(&nand), &["nAnandyate"]);

    let ni = Dhatu::new("RI\\Y", Bhvadi);
    assert_has_lat_karmani(&[], &ni, &["nIyate"]);
    assert_has_lat(&[], &yan(&ni), &["nenIyate"]);

    let nah = Dhatu::new("Ra\\ha~^", Divadi);
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
    let ranj = Dhatu::new("ra\\nja~^", Bhvadi);
    assert_has_krdanta(&[], &ranj, Krt::GinuR, &["rAgin"]);
}

#[test]
fn sutra_6_4_27() {
    let ranj = Dhatu::new("ra\\nja~^", Bhvadi);
    assert_has_krdanta(&[], &ranj, Krt::GaY, &["rAga", "raNga"]);
}

#[ignore]
#[test]
fn sutra_6_4_28() {
    let syand = Dhatu::new("syandU~\\", Bhvadi);
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
    // TODO: do the others
}

#[test]
fn sutra_6_4_89() {
    let guh = Dhatu::new("guhU~^", Bhvadi);
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
