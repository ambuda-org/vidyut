extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti as V;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn prati(text: &str) -> Pratipadika {
    Pratipadika::builder().text(text).build().unwrap()
}

fn nic(u: &str, g: Gana) -> Dhatu {
    d(u, g).with_sanadi(&[Sanadi::Nic])
}

fn yan_luk(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::YanLuk])
}

fn assert_has_lat_p_3d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, Lat, Prathama, Dvi, expected)
}

fn assert_has_lan_p_1s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, Lan, Uttama, Eka, expected)
}

fn assert_has_lot_p_1s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, Lot, Uttama, Eka, expected)
}

fn assert_has_lan_p_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, Lan, Prathama, Bahu, expected)
}

fn assert_has_vidhilin_p_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, VidhiLin, Prathama, Bahu, expected)
}

#[test]
fn sutra_7_3_32() {
    let han = d("ha\\na~", Adadi);
    let han_nic = han.clone().with_sanadi(&[Sanadi::Nic]);

    assert_has_lat_p(&[], &han_nic, &["GAtayati"]);
    assert_has_krdanta(&[], &han, Krt::Rvul, &["GAtaka"]);
    assert_has_krdanta(&[], &han, Krt::GaY, &["GAta"]);
    assert_has_krdanta(&[], &han, Krt::Rini, &["GAtin"]);
    assert_has_krdanta(&[], &han, Krt::Ramul, &["GAtam"]);
    // a-ciR-Ramul
    assert_has_lun_karmani(&[], &han, &["aGAni", "avaDi"]);
    assert_has_lit_p(&[], &han, &["jaGAna"]);
}

#[test]
fn sutra_7_3_33() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_lun_karmani(&[], &daa, &["adAyi"]);

    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_lun_karmani(&[], &dhaa, &["aDAyi"]);

    assert_has_krdanta(&[], &daa, Krt::GaY, &["dAya"]);
    assert_has_krdanta(&[], &daa, Krt::Rvul, &["dAyaka"]);
    assert_has_krdanta(&[], &dhaa, Krt::GaY, &["DAya"]);
    assert_has_krdanta(&[], &dhaa, Krt::Rvul, &["DAyaka"]);
    // ciN-kft
    assert_has_lit_p(&[], &daa, &["dadO"]);
    assert_has_lit_p(&[], &dhaa, &["daDO"]);
}

#[test]
fn sutra_7_3_34() {
    let sham = d("Samu~", Divadi);
    let tam = d("tamu~", Divadi);
    let dam = d("damu~", Divadi);
    assert_has_lun_karmani(&[], &sham, &["aSami"]);
    assert_has_lun_karmani(&[], &tam, &["atami"]);
    assert_has_lun_karmani(&[], &dam, &["adami"]);
    assert_has_krdanta(&[], &sham, Krt::Rvul, &["Samaka"]);
    assert_has_krdanta(&[], &tam, Krt::Rvul, &["tamaka"]);
    assert_has_krdanta(&[], &dam, Krt::Rvul, &["damaka"]);
    assert_has_krdanta(&[], &sham, Krt::GaY, &["Sama"]);
    assert_has_krdanta(&[], &tam, Krt::GaY, &["tama"]);
    assert_has_krdanta(&[], &dam, Krt::GaY, &["dama"]);

    // upadeza
    assert_has_krdanta(&[], &d("ya\\ma~", Bhvadi), Krt::Rvul, &["yAmaka"]);
    assert_has_krdanta(&[], &d("ra\\mu~\\", Bhvadi), Krt::Rvul, &["rAmaka"]);
    assert_has_krdanta(&[], &sham, Krt::GinuR, &["Samin"]);
    assert_has_krdanta(&[], &tam, Krt::GinuR, &["tamin"]);
    assert_has_krdanta(&[], &dam, Krt::GinuR, &["damin"]);
    // mAnta
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Krt::Rvul, &["cAraka"]);
    assert_has_krdanta(&[], &d("paWa~", Bhvadi), Krt::Rvul, &["pAWaka"]);
    // Acam
    let cam = d("camu~", Bhvadi);
    assert_has_krdanta(&["AN"], &cam, Krt::Rvul, &["AcAmaka"]);
    assert_has_krdanta(&["AN"], &cam, Krt::GaY, &["AcAma"]);
}

#[test]
fn sutra_7_3_35() {
    let jan = d("janI~\\", Divadi);
    assert_has_lun_karmani(&[], &jan, &["ajani", "ajanizwa"]);
    assert_has_krdanta(&[], &jan, Krt::Rvul, &["janaka"]);
    assert_has_krdanta(&["pra"], &jan, Krt::GaY, &["prajana"]);
    // TODO: vadh -- not in dhatupatha?
}

#[test]
fn sutra_7_3_36() {
    assert_has_lat_p(&[], &nic("f\\", Bhvadi), &["arpayati"]);
    assert_has_lat_p(&[], &nic("f\\", Juhotyadi), &["arpayati"]);
    assert_has_lat_p(&[], &nic("hrI\\", Juhotyadi), &["hrepayati"]);
    assert_has_lat_p(&[], &nic("vlI\\", Kryadi), &["vlepayati"]);
    assert_has_lat_p(&[], &nic("rI\\N", Divadi), &["repayati"]);
    assert_has_lat_p(&[], &nic("rI\\", Kryadi), &["repayati"]);
    assert_has_lat_p(&[], &nic("knUyI~\\", Bhvadi), &["knopayati"]);
    assert_has_lat_p(&[], &nic("kzmAyI~\\", Bhvadi), &["kzmApayati"]);
    assert_has_lat_p(&[], &nic("qudA\\Y", Bhvadi), &["dApayati"]);
    assert_has_lat_p(&[], &nic("quDA\\Y", Bhvadi), &["DApayati"]);
}

#[test]
fn sutra_7_3_37() {
    assert_has_lat_p(&[], &nic("So\\", Divadi), &["SAyayati"]);
    assert_has_lat_p(&["ava"], &nic("Co\\", Divadi), &["avacCAyayati"]);
    assert_has_lat_p(&["ava"], &nic("zE\\", Bhvadi), &["avasAyayati"]);
    assert_has_lat_p(&[], &nic("hve\\Y", Kryadi), &["hvAyayati"]);
    assert_has_lat_p(&["sam"], &nic("vye\\Y", Divadi), &["saMvyAyayati"]);
    assert_has_lat_p(&[], &nic("pA\\", Bhvadi), &["pAyayati"]);
    assert_has_lat_p(&[], &nic("pE\\", Bhvadi), &["pAyayati"]);
}

#[test]
fn sutra_7_3_37_v1() {
    assert_has_lat_p(&[], &nic("pA\\", Adadi), &["pAlayati"]);
}

#[test]
fn sutra_7_3_37_v2() {
    assert_has_lat_p(&[], &nic("DUY", Svadi), &["DUnayati", "DAvayati"]);
    assert_has_lat_p(&[], &nic("prI\\Y", Kryadi), &["prIRayati", "prAyayati"]);
}

#[test]
fn sutra_7_3_38() {
    assert_has_lat_p(&[], &nic("vA\\", Adadi), &["vAjayati", "vApayati"]);
}

#[test]
fn sutra_7_3_39() {
    assert_has_lat_p(
        &["vi"],
        &nic("lI\\N", Divadi),
        &["vilInayati", "vilAyayati", "vilAlayati", "vilApayati"],
    );
    assert_has_lat_p(
        &["vi"],
        &nic("lI\\", Kryadi),
        &["vilInayati", "vilAyayati", "vilAlayati", "vilApayati"],
    );
    assert_has_lat_p(&["vi"], &nic("lA\\", Adadi), &["vilAlayati", "vilApayati"]);
}

#[test]
fn sutra_7_3_40() {
    assert_has_lat(
        &[],
        &nic("YiBI\\", Juhotyadi),
        &["BIzayate", "BApayate", "BAyayati"],
    );
}

#[test]
fn sutra_7_3_41() {
    assert_has_lat_p(&[], &nic("sPAyI~\\", Bhvadi), &["sPAvayati"]);
}

#[test]
fn sutra_7_3_42() {
    assert_has_lat_p(&[], &nic("Sa\\dx~", Bhvadi), &["SAtayati", "SAdayati"]);
}

#[test]
fn sutra_7_3_43() {
    assert_has_lat_p(&[], &nic("ru\\ha~", Bhvadi), &["ropayati", "rohayati"]);
}

#[test]
fn sutra_7_3_52() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Bhvadi), Krt::GaY, &["tyAga"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Bhvadi), Krt::GaY, &["rAga", "raNga"]);

    assert_has_krdanta(&[], &pac, Krt::Ryat, &["pAkya", "pAcya"]);
    assert_has_krdanta(&[], &d("va\\ca~", Bhvadi), Krt::Ryat, &["vAkya", "vAcya"]);
    assert_has_krdanta(&[], &d("ri\\ci~^r", Bhvadi), Krt::Ryat, &["rekya", "recya"]);
}

#[test]
fn sutra_7_3_54() {
    let han = d("ha\\na~", Adadi);
    let han_nic = han.clone().with_sanadi(&[Sanadi::Nic]);
    assert_has_lat_p(&[], &han_nic, &["GAtayati"]);
    assert_has_krdanta(&[], &han, Krt::Rvul, &["GAtaka"]);
    assert_has_krdanta(&[], &han, Krt::Rini, &["GAtin"]);
    assert_has_krdanta(&[], &han, Krt::Ramul, &["GAtam"]);
    assert_has_krdanta(&[], &han, Krt::GaY, &["GAta"]);
    assert_has_parasmai_tinanta(&[], &han, Lat, Prathama, Bahu, &["Gnanti"]);
    assert_has_parasmai_tinanta(&[], &han, Lot, Prathama, Bahu, &["Gnantu"]);
    assert_has_parasmai_tinanta(&[], &han, Lan, Prathama, Bahu, &["aGnan"]);
    // hanteH
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&["pra"], &hf, Krt::GaY, &["prahAra"]);
    assert_has_krdanta(&["pra"], &hf, Krt::Rvul, &["prahAraka"]);
}

#[test]
fn sutra_7_3_55() {
    let han = d("ha\\na~", Adadi);
    let han_san = han.clone().with_sanadi(&[Sanadi::San]);
    let han_yan = han.clone().with_sanadi(&[Sanadi::Yan]);
    assert_has_lat(&[], &han_san, &["jiGAMsati"]);
    assert_has_lat(&[], &han_yan, &["jaNGanyate"]);
    assert_has_parasmai_tinanta(&[], &han, Lit, Uttama, Eka, &["jaGana", "jaGAna"]);
    // TODO: hananIyitum
}

#[test]
fn sutra_7_3_56() {
    let hi = d("hi\\", Svadi);
    let hi_san = hi.clone().with_sanadi(&[Sanadi::San]);
    let hi_yan = hi.clone().with_sanadi(&[Sanadi::Yan]);
    let hi_nic = hi.clone().with_sanadi(&[Sanadi::Nic]);
    assert_has_lat(&["pra"], &hi_san, &["prajiGIzati"]);
    assert_has_lat(&["pra"], &hi_yan, &["prajeGIyate"]);
    assert_has_lit(&["pra"], &hi, &["prajiGAya"]);
    // acaNi
    assert_has_lun(&["pra"], &hi_nic, &["prAjIhayat"]);
}

#[test]
fn sutra_7_3_57() {
    let ji = d("ji\\", Bhvadi);
    let ji_san = ji.clone().with_sanadi(&[Sanadi::San]);
    let ji_yan = ji.clone().with_sanadi(&[Sanadi::Yan]);
    assert_has_lat(&[], &ji_san, &["jigIzati"]);
    assert_has_lit(&[], &ji, &["jigAya"]);
    // sanlitoH
    assert_has_lat(&[], &ji_yan, &["jejIyate"]);
    // exclude jyA -> ji
    let jya = d("jyA\\", Kryadi);
    assert_has_parasmai_tinanta(&[], &jya, Lit, Prathama, Dvi, &["jijyatuH"]);
}

#[test]
fn sutra_7_3_58() {
    let ci = d("ci\\Y", Svadi);
    let ci_san = ci.clone().with_sanadi(&[Sanadi::San]);
    let ci_yan = ci.clone().with_sanadi(&[Sanadi::Yan]);
    assert_has_lat_p(&[], &ci_san, &["cicIzati", "cikIzati"]);
    assert_has_lit_p(&[], &ci, &["cikAya", "cicAya"]);
    // sanlitoH
    assert_has_lat(&[], &ci_yan, &["cecIyate"]);
}

#[test]
fn sutra_7_3_59() {
    let kuj = d("kUja~", Bhvadi);
    let kharj = d("Karja~", Bhvadi);
    let garj = d("garja~", Bhvadi);

    assert_has_krdanta(&[], &kuj, Krt::GaY, &["kUja"]);
    assert_has_krdanta(&[], &kharj, Krt::GaY, &["Karja"]);
    assert_has_krdanta(&[], &garj, Krt::GaY, &["garja"]);

    assert_has_krdanta(&[], &kuj, Krt::Ryat, &["kUjya"]);
    assert_has_krdanta(&[], &kharj, Krt::Ryat, &["Karjya"]);
    assert_has_krdanta(&[], &garj, Krt::Ryat, &["garjya"]);
}

#[test]
fn sutra_7_3_60() {
    let aj = d("aja~", Bhvadi);
    assert_has_krdanta(&["sam"], &aj, Krt::GaY, &["samAja"]);
    assert_has_krdanta(&["ud"], &aj, Krt::GaY, &["udAja"]);

    let vraj = d("vraja~", Bhvadi);
    assert_has_krdanta(&["pari"], &vraj, Krt::GaY, &["parivrAja"]);
    assert_has_krdanta(&["pari"], &vraj, Krt::Ryat, &["parivrAjya"]);
}

#[ignore]
#[test]
fn sutra_7_3_63() {
    let vanc = d("vancu~", Curadi);
    assert_has_krdanta(&[], &vanc, Krt::Ryat, &["vaYcya", "vaNkya"]);
}

#[test]
fn sutra_7_3_65() {
    assert_has_krdanta(
        &[],
        &d("qupa\\ca~^z", Bhvadi),
        Krt::Ryat,
        &["pAcya", "pAkya"],
    );
    assert_has_krdanta(&[], &d("va\\ca~", Adadi), Krt::Ryat, &["vAcya", "vAkya"]);
    assert_has_krdanta(
        &[],
        &d("ri\\ci~^r", Rudhadi),
        Krt::Ryat,
        &["recya", "rekya"],
    );
}

#[test]
fn sutra_7_3_66() {
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::Ryat, &["yAjya"]);
    assert_has_krdanta(&[], &d("quyAcf~^", Bhvadi), Krt::Ryat, &["yAcya"]);
    assert_has_krdanta(&[], &d("ruca~\\", Bhvadi), Krt::Ryat, &["rocya"]);
    assert_has_krdanta(&["pra"], &d("va\\ca~", Bhvadi), Krt::Ryat, &["pravAcya"]);
    assert_has_krdanta(&[], &d("fca~", Bhvadi), Krt::Ryat, &["arcya"]);
}

#[test]
fn sutra_7_3_67() {
    assert_has_krdanta(&[], &d("va\\ca~", Bhvadi), Krt::Ryat, &["vAcya", "vAkya"]);
}

#[test]
fn sutra_7_3_68() {
    assert_has_krdanta(
        &["ni"],
        &d("yu\\ji~^r", Rudhadi),
        Krt::Ryat,
        &["niyojya", "niyogya"],
    );
    assert_has_krdanta(
        &["pra"],
        &d("yu\\ji~^r", Rudhadi),
        Krt::Ryat,
        &["prayojya", "prayogya"],
    );
}

#[test]
fn sutra_7_3_69() {
    assert_has_krdanta(&[], &d("Bu\\ja~", Rudhadi), Krt::Ryat, &["Bojya", "Bogya"]);
}

#[test]
fn sutra_7_3_71() {
    assert_has_lat(&["ni"], &d("So\\", Divadi), &["niSyati"]);
    assert_has_lat(&["ava"], &d("Co\\", Divadi), &["avacCyati"]);
    assert_has_lat(&["ava"], &d("do\\", Divadi), &["avadyati"]);
    assert_has_lat(&["ava"], &d("zo\\", Divadi), &["avasyati"]);
}

#[test]
fn sutra_7_3_72() {
    let duh = d("du\\ha~^", Adadi);
    assert_has_atmane_tinanta(&[], &duh, Lun, Prathama, Dvi, &["aDukzAtAm"]);
    assert_has_atmane_tinanta(&[], &duh, Lun, Madhyama, Dvi, &["aDukzATAm"]);
    assert_has_atmane_tinanta(&[], &duh, Lun, Uttama, Eka, &["aDukzi"]);
    // aci
    assert_has_parasmai_tinanta(&[], &duh, Lun, Prathama, Eka, &["aDukzat"]);
    assert_has_parasmai_tinanta(&[], &duh, Lun, Prathama, Dvi, &["aDukzatAm"]);
    // TODO: others
}

#[test]
fn sutra_7_3_73() {
    let duh = d("du\\ha~^", Adadi);
    assert_has_atmane_tinanta(&[], &duh, Lun, Prathama, Eka, &["adugDa", "aDukzata"]);
    assert_has_atmane_tinanta(&[], &duh, Lun, Madhyama, Eka, &["adugDAH", "aDukzaTAH"]);
    assert_has_atmane_tinanta(&[], &duh, Lun, Madhyama, Bahu, &["aDugDvam", "aDukzaDvam"]);
    assert_has_atmane_tinanta(&[], &duh, Lun, Uttama, Dvi, &["aduhvahi", "aDukzAvahi"]);
    assert_has_lun_a(&[], &d("di\\ha~^", Adadi), &["adigDa", "aDikzata"]);
    assert_has_lun_a(&[], &d("li\\ha~^", Adadi), &["alIQa", "alikzata"]);
    assert_has_lun_a(
        &["ni"],
        &d("guhU~^", Bhvadi),
        &["nyagUQa", "nyaGukzata", "nyagUhizwa"],
    );
    // durAdInAm
    // TODO: how to derive this?
    // assert_has_lun_a(&["vi", "ati"], &d("ru\\Sa~", Tudadi), &["vyatyarukzata"]);
    // Atmanepade
    assert_has_lun_p(&[], &duh, &["aDukzat"]);
}

#[test]
fn sutra_7_3_74() {
    assert_has_lat(&[], &d("Samu~", Divadi), &["SAmyati"]);
    assert_has_lat(&[], &d("tamu~", Divadi), &["tAmyati"]);
    assert_has_lat(&[], &d("damu~", Divadi), &["dAmyati"]);
    assert_has_lat(&[], &d("Sramu~", Divadi), &["SrAmyati"]);
    assert_has_lat(&[], &d("Bramu~", Divadi), &["BrAmyati", "Bramati"]);
    assert_has_lat(&[], &d("kzamU~", Divadi), &["kzAmyati"]);
    assert_has_lat(&[], &d("klamu~", Divadi), &["klAmyati", "klAmati"]);
    assert_has_lat(&[], &d("madI~", Divadi), &["mAdyati"]);
    // azwARAm
    assert_has_lat(&[], &d("asu~", Divadi), &["asyati"]);
    // Syani
    assert_has_lat(&[], &d("asu~", Divadi), &["asyati"]);
}

#[test]
fn sutra_7_3_75() {
    assert_has_lat_p(&[], &d("zWivu~", Bhvadi), &["zWIvati"]);
    assert_has_lat_p(&[], &d("klamu~", Bhvadi), &["klAmati", "klAmyati"]);
    assert_has_lat_p(&["AN"], &d("camu~", Bhvadi), &["AcAmati"]);
    // counterexamples
    assert_has_lat_p(&[], &d("wuvama~", Bhvadi), &["vamati"]);
    assert_has_lat_p(&["vi"], &d("camu~", Bhvadi), &["vicamati"]);
}

#[test]
fn sutra_7_3_76() {
    let kram = d("kramu~", Bhvadi);
    assert_has_parasmai_tinanta(&[], &kram, Lat, Prathama, Eka, &["krAmati", "krAmyati"]);
    assert_has_parasmai_tinanta(&[], &kram, Lat, Prathama, Dvi, &["krAmataH", "krAmyataH"]);
    assert_has_parasmai_tinanta(&[], &kram, Lat, Prathama, Bahu, &["krAmanti", "krAmyanti"]);
    // parasmaipadezu
    assert_has_lat_a(&["AN"], &kram, &["Akramate", "Akramyate"]);
    // edge cases
    assert_has_parasmai_tinanta(
        &["ud"],
        &kram,
        Lot,
        Madhyama,
        Eka,
        &["utkrAma", "utkrAmatAt", "utkrAmya", "utkrAmyatAt"],
    );
    assert_has_parasmai_tinanta(
        &["sam"],
        &kram,
        Lot,
        Madhyama,
        Eka,
        &["saNkrAma", "saNkrAmatAt", "saNkrAmya", "saNkrAmyatAt"],
    );
}

#[test]
fn sutra_7_3_77() {
    let ish = d("izu~", Tudadi);
    let gam = d("ga\\mx~", Bhvadi);
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_lat(&[], &ish, &["icCati"]);
    assert_has_lat(&[], &gam, &["gacCati"]);
    assert_has_lat(&[], &yam, &["yacCati"]);
    // counterexamples
    assert_has_lat(&[], &d("iza~", Divadi), &["izyati"]);
    assert_has_lat(&[], &d("iza~", Kryadi), &["izRAti"]);
}

#[test]
fn sutra_7_3_78() {
    assert_has_lat_p(&[], &d("pA\\", Bhvadi), &["pibati"]);
    assert_has_lat_p(&[], &d("GrA\\", Bhvadi), &["jiGrati"]);
    assert_has_lat_p(&[], &d("DmA\\", Bhvadi), &["Damati"]);
    assert_has_lat_p(&[], &d("zWA\\", Bhvadi), &["tizWati"]);
    assert_has_lat_p(&[], &d("mnA\\", Bhvadi), &["manati"]);
    assert_has_lat_p(&[], &d("dA\\R", Bhvadi), &["yacCati"]);
    assert_has_lat_p(&[], &d("df\\Si~r", Bhvadi), &["paSyati"]);
    assert_has_lat_p(&[], &d("f\\", Bhvadi), &["fcCati"]);
    assert_has_lat_p(&[], &d("sf\\", Bhvadi), &["DAvati", "sarati"]);
    assert_has_lat_a(&[], &d("Sa\\dx~", Bhvadi), &["SIyate"]);
    assert_has_lat_p(&[], &d("za\\dx~", Bhvadi), &["sIdati"]);
}

#[test]
fn sutra_7_3_79() {
    assert_has_lat_p(&[], &d("jYA\\", Kryadi), &["jAnAti"]);
    assert_has_lat_a(&[], &d("janI~\\", Divadi), &["jAyate"]);
    // daivAdikasya
    assert_has_lat_p(&[], &d("jana~", Juhotyadi), &["jajanti"]);
}

#[test]
fn sutra_7_3_80() {
    assert_has_lat_p(&[], &d("pUY", Kryadi), &["punAti"]);
    assert_has_lat_p(&[], &d("lUY", Kryadi), &["lunAti"]);
    assert_has_lat_p(&[], &d("stFY", Kryadi), &["stfRAti"]);

    // Regression test
    assert_has_lat_p(&[], &d("jyA\\", Kryadi), &["jinAti"]);
}

#[test]
fn sutra_7_3_82() {
    let mid = d("YimidA~", Divadi);
    assert_has_lat(&[], &mid, &["medyati"]);
    assert_has_lat_karmani(&[], &mid, &["midyate"]);
}

#[test]
fn sutra_7_3_83() {
    assert_has_lan_p_3p(&[], &d("hu\\", Juhotyadi), &["ajuhavuH"]);
    assert_has_lan_p_3p(&[], &d("YiBI\\", Juhotyadi), &["abiBayuH"]);
    assert_has_lan_p_3p(&[], &d("quBf\\Y", Juhotyadi), &["abiBaruH"]);

    assert_has_vidhilin_p_3p(&[], &d("ci\\Y", Svadi), &["cinuyuH"]);
    assert_has_vidhilin_p_3p(&[], &d("zu\\Y", Svadi), &["sunuyuH"]);
}

#[test]
fn sutra_7_3_84() {
    assert_has_lat_p(&[], &d("tF", Bhvadi), &["tarati"]);
    assert_has_lat_p(&[], &d("RI\\Y", Bhvadi), &["nayati"]);
    assert_has_lat_p(&[], &d("BU", Bhvadi), &["Bavati"]);

    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::tfc, &["stotf"]);

    // sArvadhAtukArdhadhAtukayoH
    assert_has_taddhitanta(&prati("agni"), Taddhita::tva, &["agnitva"]);
    // TODO: agnikAmyati
}

#[test]
fn sutra_7_3_85() {
    let jagf = d("jAgf", Adadi);

    assert_has_lat_p(
        &[],
        &jagf.clone().with_sanadi(&[Sanadi::Nic]),
        &["jAgarayati"],
    );
    assert_has_krdanta(&[], &jagf, Krt::Rvul, &["jAgaraka"]);
    assert_has_krdanta(&[], &jagf, Krt::GaY, &["jAgara"]);
    assert_has_krdanta(&[], &jagf, Krt::kta, &["jAgarita"]);
    assert_has_krdanta(&[], &jagf, Krt::ktavatu, &["jAgaritavat"]);

    assert_has_krdanta(&[], &jagf, Krt::kvinUnadi, &["jAgfvi"]);
    assert_has_lun_karmani(&[], &jagf, &["ajAgAri"]);
    assert_has_lit(
        &[],
        &jagf,
        &["jajAgAra", "jAgarAYcakAra", "jAgarAmAsa", "jAgarAmbaBUva"],
    );
    assert_has_lat_p_3d(&[], &jagf, &["jAgftaH"]);
}

#[test]
fn sutra_7_3_86() {
    let nic = |u, g| d(u, g).with_sanadi(&[Sanadi::Nic]);

    assert_has_lat_p(&[], &nic("vlI\\", Kryadi), &["vlepayati"]);
    assert_has_lat_p(&[], &nic("hrI\\", Juhotyadi), &["hrepayati"]);
    assert_has_lat_p(&[], &nic("knUyI~\\", Bhvadi), &["knopayati"]);

    let bhid = d("Bi\\di~^r", Rudhadi);
    let chid = d("Ci\\di~^r", Rudhadi);
    assert_has_krdanta(&[], &bhid, Krt::lyuw, &["Bedana"]);
    assert_has_krdanta(&[], &chid, Krt::lyuw, &["Cedana"]);
    assert_has_krdanta(&[], &bhid, Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &chid, Krt::tfc, &["Cettf"]);
}

#[test]
fn sutra_7_3_87() {
    let nij = d("Ri\\ji~^r", Juhotyadi);
    let vij = d("vi\\ji~^r", Juhotyadi);
    let viz = d("vi\\zx~^", Juhotyadi);

    assert_has_lot_p_1s(&[], &nij, &["nenijAni"]);
    assert_has_lot_p_1s(&[], &vij, &["vevijAni"]);
    assert_has_lot_p_1s(&["pari"], &viz, &["parivevizARi"]);

    assert_has_lan_p_1s(&[], &nij, &["anenijam"]);
    assert_has_lan_p_1s(&[], &vij, &["avevijam"]);
    assert_has_lan_p_1s(&["pari"], &viz, &["paryavevizam"]);

    assert_has_lot_p_1s(&[], &d("vida~", Adadi), &["vedAni", "vidANkaravARi"]);
    assert_has_lat_p(&[], &nij, &["nenekti"]);
    assert_has_lit_p(&[], &nij, &["nineja"]);

    let hu = d("hu\\", Juhotyadi);
    assert_has_lot_p_1s(&[], &hu, &["juhavAni"]);
    assert_has_lan_p_1s(&[], &hu, &["ajuhavam"]);

    // TODO: other examples
}

#[test]
fn sutra_7_3_88() {
    let bhu = d("BU", Bhvadi);
    assert_has_lun(&[], &bhu, &["aBUt"]);
    assert_has_parasmai_tinanta(&[], &bhu, Lun, Uttama, Eka, &["aBUvam"]);

    let su = d("zUN", Adadi);
    assert_has_atmane_tinanta(&[], &su, Lot, Uttama, Eka, &["suvE"]);
    assert_has_atmane_tinanta(&[], &su, Lot, Uttama, Dvi, &["suvAvahE"]);
    assert_has_atmane_tinanta(&[], &su, Lot, Uttama, Bahu, &["suvAmahE"]);

    assert_has_lat(&[], &bhu, &["Bavati"]);

    // TODO: boBavIti, boBUtu. How to derive vyatiBavizIzwa?
}

#[test]
fn sutra_7_3_89() {
    let yu = d("yu", Adadi);
    assert_has_parasmai_tinanta(&[], &yu, Lat, Prathama, Eka, &["yOti"]);
    assert_has_parasmai_tinanta(&[], &yu, Lat, Madhyama, Eka, &["yOzi"]);
    assert_has_parasmai_tinanta(&[], &yu, Lat, Uttama, Eka, &["yOmi"]);

    let nu = d("Ru", Adadi);
    assert_has_parasmai_tinanta(&[], &nu, Lat, Prathama, Eka, &["nOti"]);
    assert_has_parasmai_tinanta(&[], &nu, Lat, Madhyama, Eka, &["nOzi"]);
    assert_has_parasmai_tinanta(&[], &nu, Lat, Uttama, Eka, &["nOmi"]);

    let stu = d("zwu\\Y", Adadi);
    assert_has_parasmai_tinanta(&[], &stu, Lat, Prathama, Eka, &["stOti", "stavIti"]);
    assert_has_parasmai_tinanta(&[], &stu, Lat, Madhyama, Eka, &["stOzi", "stavIzi"]);
    assert_has_parasmai_tinanta(&[], &stu, Lat, Uttama, Eka, &["stOmi", "stavImi"]);

    let i = d("i\\R", Adadi);
    assert_has_parasmai_tinanta(&[], &i, Lat, Prathama, Eka, &["eti"]);
    assert_has_parasmai_tinanta(&[], &i, Lat, Madhyama, Eka, &["ezi"]);
    assert_has_parasmai_tinanta(&[], &i, Lat, Uttama, Eka, &["emi"]);

    // luki
    let su = d("zu\\Y", Svadi);
    assert_has_parasmai_tinanta(&[], &su, Lat, Prathama, Eka, &["sunoti"]);
    assert_has_parasmai_tinanta(&[], &su, Lat, Madhyama, Eka, &["sunozi"]);
    assert_has_parasmai_tinanta(&[], &su, Lat, Uttama, Eka, &["sunomi"]);

    // hali
    let ru = d("ru", Adadi);
    assert_has_parasmai_tinanta(&[], &yu, Lot, Uttama, Eka, &["yavAni"]);
    assert_has_parasmai_tinanta(&[], &ru, Lot, Uttama, Eka, &["ravARi"]);

    // piti
    assert_has_parasmai_tinanta(&[], &yu, Lat, Prathama, Dvi, &["yutaH"]);
    assert_has_parasmai_tinanta(&[], &ru, Lat, Prathama, Dvi, &["rutaH", "ruvItaH"]);

    // yAsuw
    assert_has_vidhilin_p(&[], &stu, &["stuyAt", "stuvIyAt"]);

    // TODO: yoyogi, roroti
}

#[test]
fn sutra_7_3_90() {
    let urnu = d("UrRuY", Adadi);
    assert_has_parasmai_tinanta(
        &["pra"],
        &urnu,
        Lat,
        Prathama,
        Eka,
        &["prorROti", "prorRoti"],
    );
    assert_has_parasmai_tinanta(
        &["pra"],
        &urnu,
        Lat,
        Madhyama,
        Eka,
        &["prorROzi", "prorRozi"],
    );
    assert_has_parasmai_tinanta(&["pra"], &urnu, Lat, Uttama, Eka, &["prorROmi", "prorRomi"]);

    // hali
    assert_has_parasmai_tinanta(&["pra"], &urnu, Lot, Uttama, Eka, &["prorRavAni"]);
}

#[test]
fn sutra_7_3_91() {
    let urnu = d("UrRuY", Adadi);
    assert_has_parasmai_tinanta(&["pra"], &urnu, Lan, Prathama, Eka, &["prOrRot"]);
    assert_has_parasmai_tinanta(&["pra"], &urnu, Lan, Madhyama, Eka, &["prOrRoH"]);
}

#[test]
fn sutra_7_3_92() {
    let tfh = d("tfha~", Rudhadi);
    assert_has_parasmai_tinanta(&[], &tfh, Lat, Prathama, Eka, &["tfReQi"]);
    assert_has_parasmai_tinanta(&[], &tfh, Lat, Madhyama, Eka, &["tfRekzi"]);
    assert_has_parasmai_tinanta(&[], &tfh, Lat, Uttama, Eka, &["tfRehmi"]);
    assert_has_parasmai_tinanta(&[], &tfh, Lan, Prathama, Eka, &["atfRew"]);

    // hali
    assert_has_parasmai_tinanta(&[], &tfh, Lot, Uttama, Eka, &["tfRahAni"]);
    // piti
    assert_has_parasmai_tinanta(&[], &tfh, Lat, Prathama, Dvi, &["tfRQaH"]);
}

#[test]
fn sutra_7_3_93() {
    let bru = d("brUY", Adadi);
    assert_has_parasmai_tinanta(&[], &bru, Lat, Prathama, Eka, &["bravIti", "Aha"]);
    assert_has_parasmai_tinanta(&[], &bru, Lat, Madhyama, Eka, &["bravIzi", "AtTa"]);
    assert_has_parasmai_tinanta(&[], &bru, Lat, Uttama, Eka, &["bravImi"]);
    assert_has_parasmai_tinanta(&[], &bru, Lan, Prathama, Eka, &["abravIt"]);

    // hali
    assert_has_parasmai_tinanta(&[], &bru, Lot, Uttama, Eka, &["bravARi"]);
    // piti
    assert_has_parasmai_tinanta(&[], &bru, Lat, Prathama, Dvi, &["brUtaH", "AhatuH"]);
}

#[ignore]
#[test]
fn sutra_7_3_94() {
    let yl = |u, g| yan_luk(&d(u, g));
    assert_has_lat(&[], &yl("lapa~", Bhvadi), &["lAlapIti", "lAlapti"]);
    assert_has_lat(&[], &yl("vada~", Bhvadi), &["vAvadIti", "vAvatti"]);
    assert_has_lat(&[], &yl("ru", Adadi), &["roravIti", "roroti"]);
}

#[test]
fn sutra_7_3_95() {
    assert_has_lat(&["ud"], &d("tu\\", Adadi), &["uttOti", "uttavIti"]);
    assert_has_lat(&["upa"], &d("ru", Adadi), &["uparOti", "uparavIti"]);

    let stu = d("zwu\\Y", Adadi);
    assert_has_lat_p(&["upa"], &stu, &["upastOti", "upastavIti"]);
    assert_has_atmane_tinanta(&[], &stu, VidhiLin, Prathama, Eka, &["stuvIta"]);

    // Sam and am are chAndasa.
}

#[test]
fn sutra_7_3_96() {
    let asa = d("asa~", Adadi);
    assert_has_parasmai_tinanta(&[], &asa, Lan, Prathama, Eka, &["AsIt"]);
    assert_has_parasmai_tinanta(&[], &asa, Lan, Madhyama, Eka, &["AsIH"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_lun_p(&[], &kf, &["akArzIt"]);
    assert_has_lun_p(&[], &d("zu\\Y", Svadi), &["asAvIt"]);
    assert_has_lun_p(&[], &d("lUY", Kryadi), &["alAvIt"]);
    assert_has_lun_p(&[], &d("pUY", Kryadi), &["apAvIt"]);

    assert_has_lat(&[], &asa, &["asti"]);
    assert_has_parasmai_tinanta(&[], &kf, Lun, Uttama, Eka, &["akArzam"]);
}

#[test]
fn sutra_7_3_98_and_sutra_7_3_99() {
    let rud = d("rudi~r", Adadi);
    assert_has_parasmai_tinanta(&[], &rud, Lan, Prathama, Eka, &["arodIt", "arodat"]);
    assert_has_parasmai_tinanta(&[], &rud, Lan, Madhyama, Eka, &["arodIH", "arodaH"]);

    let svap = d("Yizva\\pa~", Adadi);
    assert_has_parasmai_tinanta(&[], &svap, Lan, Prathama, Eka, &["asvapIt", "asvapat"]);
    assert_has_parasmai_tinanta(&[], &svap, Lan, Madhyama, Eka, &["asvapIH", "asvapaH"]);

    let svas = d("Svasa~", Adadi);
    assert_has_parasmai_tinanta(&[], &svas, Lan, Prathama, Eka, &["aSvasIt", "aSvasat"]);
    assert_has_parasmai_tinanta(&[], &svas, Lan, Madhyama, Eka, &["aSvasIH", "aSvasaH"]);

    let an = d("ana~", Adadi);
    assert_has_parasmai_tinanta(&["pra"], &an, Lan, Prathama, Eka, &["prARIt", "prARat"]);
    assert_has_parasmai_tinanta(&["pra"], &an, Lan, Madhyama, Eka, &["prARIH", "prARaH"]);

    let jaks = d("jakza~", Adadi);
    assert_has_parasmai_tinanta(&[], &jaks, Lan, Prathama, Eka, &["ajakzIt", "ajakzat"]);
    assert_has_parasmai_tinanta(&[], &jaks, Lan, Madhyama, Eka, &["ajakzIH", "ajakzaH"]);

    // pancabhyaH
    let jagf = d("jAgf", Adadi);
    assert_has_parasmai_tinanta(&[], &jagf, Lan, Madhyama, Eka, &["ajAgaH"]);
    // apkrtasya
    assert_has_lat(&[], &rud, &["roditi"]);
}

#[test]
fn sutra_7_3_100() {
    let ad = d("a\\da~", Adadi);
    assert_has_parasmai_tinanta(&[], &ad, Lan, Prathama, Eka, &["Adat"]);
    assert_has_parasmai_tinanta(&[], &ad, Lan, Madhyama, Eka, &["AdaH"]);
    assert_has_parasmai_tinanta(&[], &ad, Lat, Prathama, Eka, &["atti"]);
    assert_has_parasmai_tinanta(&[], &ad, Lat, Madhyama, Eka, &["atsi"]);
}

#[test]
fn sutra_7_3_101() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_parasmai_tinanta(&[], &pac, Lat, Uttama, Eka, &["pacAmi"]);
    assert_has_parasmai_tinanta(&[], &pac, Lat, Uttama, Dvi, &["pacAvaH"]);
    assert_has_parasmai_tinanta(&[], &pac, Lat, Uttama, Bahu, &["pacAmaH"]);
    assert_has_parasmai_tinanta(&[], &pac, Lrt, Uttama, Eka, &["pakzyAmi"]);
    assert_has_parasmai_tinanta(&[], &pac, Lrt, Uttama, Dvi, &["pakzyAvaH"]);
    assert_has_parasmai_tinanta(&[], &pac, Lrt, Uttama, Bahu, &["pakzyAmaH"]);
    // ataH
    let ci = d("ci\\Y", Svadi);
    assert_has_parasmai_tinanta(&[], &ci, Lat, Uttama, Dvi, &["cinuvaH", "cinvaH"]);
    assert_has_parasmai_tinanta(&[], &ci, Lat, Uttama, Bahu, &["cinumaH", "cinmaH"]);
    // yaYi
    assert_has_parasmai_tinanta(&[], &pac, Lat, Prathama, Dvi, &["pacataH"]);
    assert_has_parasmai_tinanta(&[], &pac, Lat, Madhyama, Dvi, &["pacaTaH"]);
    // TODO: sArvadhAtuke
}

#[test]
fn sutra_7_3_102() {
    assert_has_subantas("vfkza", Pum, V::Caturthi, Eka, &["vfkzAya"]);
    assert_has_subantas("plakza", Pum, V::Caturthi, Eka, &["plakzAya"]);
    assert_has_subantas("vfkza", Pum, V::Caturthi, Dvi, &["vfkzAByAm"]);
    assert_has_subantas("plakza", Pum, V::Caturthi, Dvi, &["plakzAByAm"]);
    // ataH
    assert_has_subantas("agni", Pum, V::Caturthi, Dvi, &["agniByAm"]);
    // yaYi
    assert_has_subantas("vfkza", Pum, V::Sasthi, Eka, &["vfkzasya"]);
    assert_has_subantas("plakza", Pum, V::Sasthi, Eka, &["plakzasya"]);
}

#[test]
fn sutra_7_3_103() {
    assert_has_subantas("vfkza", Pum, V::Caturthi, Bahu, &["vfkzeByaH"]);
    assert_has_subantas("plakza", Pum, V::Caturthi, Bahu, &["plakzeByaH"]);
    assert_has_subantas("vfkza", Pum, V::Saptami, Bahu, &["vfkzezu"]);
    assert_has_subantas("plakza", Pum, V::Saptami, Bahu, &["plakzezu"]);
    // bahuvacane
    assert_has_subantas("vfkza", Pum, V::Caturthi, Dvi, &["vfkzAByAm"]);
    assert_has_subantas("plakza", Pum, V::Caturthi, Dvi, &["plakzAByAm"]);
    // Jali
    assert_has_subantas("vfkza", Pum, V::Sasthi, Bahu, &["vfkzARAm"]);
    // TODO: supi
}

#[test]
fn sutra_7_3_104() {
    assert_has_subantas("vfkza", Pum, V::Sasthi, Dvi, &["vfkzayoH"]);
    assert_has_subantas("plakza", Pum, V::Sasthi, Dvi, &["plakzayoH"]);
    assert_has_subantas("vfkza", Pum, V::Saptami, Dvi, &["vfkzayoH"]);
    assert_has_subantas("plakza", Pum, V::Saptami, Dvi, &["plakzayoH"]);
}

#[test]
fn sutra_7_3_105() {
    use Vibhakti::*;
    assert_has_subantas_p(&stri("KawvA"), Stri, Trtiya, Eka, &["KawvayA"]);
    assert_has_subantas_p(&stri("mAlA"), Stri, Trtiya, Eka, &["mAlayA"]);
    assert_has_subantas_p(&stri("KawvA"), Stri, Sasthi, Dvi, &["KawvayoH"]);
    assert_has_subantas_p(&stri("mAlA"), Stri, Sasthi, Dvi, &["mAlayoH"]);
    assert_has_subantas_p(&stri("bahurAjA"), Stri, Trtiya, Eka, &["bahurAjayA"]);
    assert_has_subantas_p(
        &stri("kArIzaganDyA"),
        Stri,
        Trtiya,
        Eka,
        &["kArIzaganDyayA"],
    );
    assert_has_subantas_p(&stri("bahurAjA"), Stri, Sasthi, Dvi, &["bahurAjayoH"]);
    assert_has_subantas_p(
        &stri("kArIzaganDyA"),
        Stri,
        Sasthi,
        Dvi,
        &["kArIzaganDyayoH"],
    );
    // TODO: add others
}

#[test]
fn sutra_7_3_106() {
    assert_has_subantas_p(&stri("KawvA"), Stri, V::Sambodhana, Eka, &["Kawve"]);
    assert_has_subantas_p(&stri("bahurAjA"), Stri, V::Sambodhana, Eka, &["bahurAje"]);
    assert_has_subantas_p(
        &stri("kArIzaganDyA"),
        Stri,
        V::Sambodhana,
        Eka,
        &["kArIzaganDye"],
    );
}

#[test]
fn sutra_7_3_107() {
    assert_has_subantas_p(&stri("ambA"), Stri, V::Sambodhana, Eka, &["amba"]);
    assert_has_subantas_p(&stri("akkA"), Stri, V::Sambodhana, Eka, &["akka"]);
    assert_has_subantas_p(&stri("allA"), Stri, V::Sambodhana, Eka, &["alla"]);
    // nadyoH
    assert_has_subantas_p(&stri("kumArI"), Stri, V::Sambodhana, Eka, &["kumAri"]);
    assert_has_subantas_p(
        &stri("SArNgaravI"),
        Stri,
        V::Sambodhana,
        Eka,
        &["SArNgaravi"],
    );
    assert_has_subantas("brahmabanDU", Stri, V::Sambodhana, Eka, &["brahmabanDu"]);
    assert_has_subantas("vIrabanDU", Stri, V::Sambodhana, Eka, &["vIrabanDu"]);
}

#[test]
fn sutra_7_3_108() {
    assert_has_subantas("agni", Pum, V::Sambodhana, Eka, &["agne"]);
    assert_has_subantas("vAyu", Pum, V::Sambodhana, Eka, &["vAyo"]);
    assert_has_subantas("pawu", Pum, V::Sambodhana, Eka, &["pawo"]);
    // But, not for these because the prAtipadika was not originally hrasva
    assert_has_subantas_p(&stri("kumArI"), Stri, V::Sambodhana, Eka, &["kumAri"]);
    assert_has_subantas("brahmabanDU", Stri, V::Sambodhana, Eka, &["brahmabanDu"]);
}

#[test]
fn sutra_7_3_109() {
    assert_has_subantas("agni", Pum, V::Prathama, Bahu, &["agnayaH"]);
    assert_has_subantas("vAyu", Pum, V::Prathama, Bahu, &["vAyavaH"]);
    assert_has_subantas("pawu", Pum, V::Prathama, Bahu, &["pawavaH"]);
    assert_has_subantas("Denu", Stri, V::Prathama, Bahu, &["DenavaH"]);
    assert_has_subantas("budDi", Stri, V::Prathama, Bahu, &["budDayaH"]);
}

#[ignore]
#[test]
fn sutra_7_3_110() {
    assert_has_subantas("mAtf", Stri, V::Saptami, Eka, &["mAtari"]);
    assert_has_subantas("pitf", Pum, V::Saptami, Eka, &["pitari"]);
    assert_has_subantas("BrAtf", Pum, V::Saptami, Eka, &["BrAtari"]);
    assert_has_subantas("kartf", Pum, V::Saptami, Eka, &["kartari"]);
    // sarvanAmasTAne
    assert_has_subantas("kartf", Pum, V::Prathama, Dvi, &["kartArO"]);
    assert_has_subantas("kartf", Pum, V::Prathama, Bahu, &["kartAraH"]);
    assert_has_subantas("mAtf", Stri, V::Prathama, Dvi, &["mAtarO"]);
    assert_has_subantas("pitf", Pum, V::Prathama, Dvi, &["pitarO"]);
    assert_has_subantas("BrAtf", Pum, V::Prathama, Dvi, &["BrAtarO"]);
}

#[test]
fn sutra_7_3_111() {
    assert_has_subantas("agni", Pum, V::Caturthi, Eka, &["agnaye"]);
    assert_has_subantas("vAyu", Pum, V::Caturthi, Eka, &["vAyave"]);
    assert_has_subantas("agni", Pum, V::Panchami, Eka, &["agneH"]);
    assert_has_subantas("vAyu", Pum, V::Panchami, Eka, &["vAyoH"]);
    assert_has_subantas("agni", Pum, V::Sasthi, Eka, &["agneH"]);
    assert_has_subantas("vAyu", Pum, V::Sasthi, Eka, &["vAyoH"]);
    // GeH
    assert_has_subantas("saKi", Pum, V::Caturthi, Eka, &["saKye"]);
    assert_has_subantas("pati", Pum, V::Caturthi, Eka, &["patye"]);
    // Niti
    assert_has_subantas("agni", Pum, V::Caturthi, Dvi, &["agniByAm"]);
    // TODO: supi
}

#[test]
fn sutra_7_3_112() {
    assert_has_subantas("kumArI", Stri, V::Caturthi, Eka, &["kumAryE"]);
    assert_has_subantas("brahmabanDU", Stri, V::Caturthi, Eka, &["brahmabanDvE"]);
    assert_has_subantas("kumArI", Stri, V::Panchami, Eka, &["kumAryAH"]);
    assert_has_subantas("brahmabanDU", Stri, V::Panchami, Eka, &["brahmabanDvAH"]);
}

#[test]
fn sutra_7_3_113() {
    use Vibhakti::*;
    assert_has_subantas_p(&stri("KawvA"), Stri, Caturthi, Eka, &["KawvAyE"]);
    assert_has_subantas_p(&stri("bahurAjA"), Stri, Caturthi, Eka, &["bahurAjAyE"]);
    assert_has_subantas_p(
        &stri("kArIzaganDyA"),
        Stri,
        Caturthi,
        Eka,
        &["kArIzaganDyAyE"],
    );
    assert_has_subantas_p(&stri("KawvA"), Stri, Panchami, Eka, &["KawvAyAH"]);
    assert_has_subantas_p(&stri("bahurAjA"), Stri, Panchami, Eka, &["bahurAjAyAH"]);
    assert_has_subantas_p(
        &stri("kArIzaganDyA"),
        Stri,
        Panchami,
        Eka,
        &["kArIzaganDyAyAH"],
    );
}

#[ignore]
#[test]
fn sutra_7_3_114() {
    use Vibhakti::*;
    assert_has_subantas("sarva", Stri, Caturthi, Eka, &["sarvasyE"]);
    assert_has_subantas("viSva", Stri, Caturthi, Eka, &["viSvasyE"]);
    assert_has_subantas("yad", Stri, Caturthi, Eka, &["yasyE"]);
    assert_has_subantas("tad", Stri, Caturthi, Eka, &["tasyE"]);
    assert_has_subantas("kim", Stri, Caturthi, Eka, &["kasyE"]);
    assert_has_subantas("anya", Stri, Caturthi, Eka, &["anyasyE"]);
    assert_has_subantas("sarva", Stri, Panchami, Eka, &["sarvasyAH"]);
    assert_has_subantas("viSva", Stri, Panchami, Eka, &["viSvasyAH"]);
    assert_has_subantas("yad", Stri, Panchami, Eka, &["yasyAH"]);
    assert_has_subantas("tad", Stri, Panchami, Eka, &["tasyAH"]);
    assert_has_subantas("kim", Stri, Panchami, Eka, &["kasyAH"]);
    assert_has_subantas("anya", Stri, Panchami, Eka, &["anyasyAH"]);
    // TODO: ApaH
}

#[test]
fn sutra_7_3_116() {
    assert_has_subantas_p(&stri("kumArI"), Stri, V::Saptami, Eka, &["kumAryAm"]);
    assert_has_subantas_p(&stri("gOrI"), Stri, V::Saptami, Eka, &["gOryAm"]);
    assert_has_subantas("brahmabanDU", Stri, V::Saptami, Eka, &["brahmabanDvAm"]);
    assert_has_subantas("DIBanDU", Stri, V::Saptami, Eka, &["DIBanDvAm"]);
}

#[test]
fn sutra_7_3_117() {
    use Vibhakti::*;
    assert_has_subantas("kfti", Stri, Saptami, Eka, &["kftO", "kftyAm"]);
    assert_has_subantas("Denu", Stri, Saptami, Eka, &["DenO", "DenvAm"]);
}

#[test]
fn sutra_7_3_118() {
    assert_has_subantas("saKi", Pum, V::Saptami, Eka, &["saKyO"]);
    assert_has_subantas("pati", Pum, V::Saptami, Eka, &["patyO"]);
}

#[test]
fn sutra_7_3_119() {
    use Vibhakti::*;
    assert_has_subantas("agni", Pum, Saptami, Eka, &["agnO"]);
    assert_has_subantas("vAyu", Pum, Saptami, Eka, &["vAyO"]);
    assert_has_subantas("kfti", Stri, Saptami, Eka, &["kftO", "kftyAm"]);
    assert_has_subantas("Denu", Stri, Saptami, Eka, &["DenO", "DenvAm"]);
    assert_has_subantas("pawu", Pum, Saptami, Eka, &["pawO"]);
}

#[test]
fn sutra_7_3_120() {
    use Vibhakti::*;
    assert_has_subantas("agni", Pum, Trtiya, Eka, &["agninA"]);
    assert_has_subantas("vAyu", Pum, Trtiya, Eka, &["vAyunA"]);
    assert_has_subantas("pawu", Pum, Trtiya, Eka, &["pawunA"]);
    // astriyAm
    assert_has_subantas("kfti", Stri, Trtiya, Eka, &["kftyA"]);
    assert_has_subantas("Denu", Stri, Trtiya, Eka, &["DenvA"]);
    // TODO: others
}
