extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::*;

/// Shorthand for creating a nijanta dhAtu.
fn nic_d(u: &str, g: Gana) -> Dhatu {
    nic(&d(u, g))
}

#[test]
fn sutra_7_3_1() {
    assert_has_taddhita("devikA", T::aR, &["dAvika"]);
    assert_has_taddhita("SiMSapA", T::aR, &["SAMSapa"]);
    // TODO: dityavAh
    assert_has_taddhita("dIrGasatra", T::aR, &["dArGasatra"]);
    assert_has_taddhita("Sreyas", T::aR, &["SrAyasa"]);
}

#[ignore]
#[test]
fn sutra_7_3_2() {
    assert_has_taddhita("kekaya", T::aY, &["kEkeya"]);
    assert_has_taddhita("mitrayu", T::vuY, &["mEtreyaka"]);
    assert_has_taddhita("pralaya", T::aR, &["prAleya"]);
}

#[ignore]
#[test]
fn sutra_7_3_3() {
    assert_has_artha_taddhita("vyasana", TatraBhava, T::aR, &["vEyasana"]);
    assert_has_artha_taddhita("vyAkaraRa", TadAdhiteTadVeda, T::aR, &["vEyAkaraRa"]);
    let svashva = create_bahuvrihi("svaSva", "su", "aSva");
    assert_has_taddhita(&svashva, T::aR, &["sOvaSva"]);

    // TODO: others
}

#[test]
fn sutra_7_3_4() {
    assert_has_taddhita("dvAra", T::Wak, &["dOvArika"]);
    assert_has_taddhita("svara", T::aR, &["sOvara"]);
    assert_has_taddhita("vyalkaSa", T::aR, &["vEyalkaSa"]);
    assert_has_taddhita("svasti", T::Wak, &["sOvastika"]);
    assert_has_taddhita("Svan", T::aY, &["SOva"]);
    assert_has_taddhita("sva", T::aR, &["sOva"]);

    // TODO: others
}

#[test]
fn sutra_7_3_5() {
    assert_has_artha_taddhita("nyagroDa", TasyaVikara, T::aR, &["nEyagroDa"]);
    // TODO: others
}

#[test]
fn sutra_7_3_32() {
    let han = d("ha\\na~", Adadi);

    assert_has_tip(&[], &nic(&han), Lat, &["GAtayati"]);
    assert_has_krdanta(&[], &han, Krt::Rvul, &["GAtaka"]);
    assert_has_krdanta(&[], &han, Krt::GaY, &["GAta"]);
    assert_has_krdanta(&[], &han, Krt::Rini, &["GAtin"]);
    assert_has_krdanta(&[], &han, Krt::Ramul, &["GAtam"]);
    // a-ciR-Ramul
    assert_has_ta_k(&[], &han, Lun, &["aGAni", "avaDi"]);
    assert_has_tip(&[], &han, Lit, &["jaGAna"]);
}

#[test]
fn sutra_7_3_33() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_ta_k(&[], &daa, Lun, &["adAyi"]);

    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_ta_k(&[], &dhaa, Lun, &["aDAyi"]);

    assert_has_krdanta(&[], &daa, Krt::GaY, &["dAya"]);
    assert_has_krdanta(&[], &daa, Krt::Rvul, &["dAyaka"]);
    assert_has_krdanta(&[], &dhaa, Krt::GaY, &["DAya"]);
    assert_has_krdanta(&[], &dhaa, Krt::Rvul, &["DAyaka"]);
    // ciN-kft
    assert_has_tip(&[], &daa, Lit, &["dadO"]);
    assert_has_tip(&[], &dhaa, Lit, &["daDO"]);
}

#[test]
fn sutra_7_3_34() {
    let sham = d("Samu~", Divadi);
    let tam = d("tamu~", Divadi);
    let dam = d("damu~", Divadi);
    assert_has_ta_k(&[], &sham, Lun, &["aSami"]);
    assert_has_ta_k(&[], &tam, Lun, &["atami"]);
    assert_has_ta_k(&[], &dam, Lun, &["adami"]);
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
    assert_has_ta_k(&[], &jan, Lun, &["ajani", "ajanizwa"]);
    assert_has_krdanta(&[], &jan, Krt::Rvul, &["janaka"]);
    assert_has_krdanta(&["pra"], &jan, Krt::GaY, &["prajana"]);
    // TODO: vadh -- not in dhatupatha?
}

#[test]
fn sutra_7_3_36() {
    assert_has_tip(&[], &nic_d("f\\", Bhvadi), Lat, &["arpayati"]);
    assert_has_tip(&[], &nic_d("f\\", Juhotyadi), Lat, &["arpayati"]);
    assert_has_tip(&[], &nic_d("hrI\\", Juhotyadi), Lat, &["hrepayati"]);
    assert_has_tip(&[], &nic_d("vlI\\", Kryadi), Lat, &["vlepayati"]);
    assert_has_tip(&[], &nic_d("rI\\N", Divadi), Lat, &["repayati"]);
    assert_has_tip(&[], &nic_d("rI\\", Kryadi), Lat, &["repayati"]);
    assert_has_tip(&[], &nic_d("knUyI~\\", Bhvadi), Lat, &["knopayati"]);
    assert_has_tip(&[], &nic_d("kzmAyI~\\", Bhvadi), Lat, &["kzmApayati"]);
    assert_has_tip(&[], &nic_d("qudA\\Y", Bhvadi), Lat, &["dApayati"]);
    assert_has_tip(&[], &nic_d("quDA\\Y", Bhvadi), Lat, &["DApayati"]);
}

#[test]
fn sutra_7_3_37() {
    assert_has_tip(&[], &nic_d("So\\", Divadi), Lat, &["SAyayati"]);
    assert_has_tip(&["ava"], &nic_d("Co\\", Divadi), Lat, &["avacCAyayati"]);
    assert_has_tip(&["ava"], &nic_d("zE\\", Bhvadi), Lat, &["avasAyayati"]);
    assert_has_tip(&[], &nic_d("hve\\Y", Bhvadi), Lat, &["hvAyayati"]);
    assert_has_tip(&["sam"], &nic_d("vye\\Y", Divadi), Lat, &["saMvyAyayati"]);
    assert_has_tip(&[], &nic_d("pA\\", Bhvadi), Lat, &["pAyayati"]);
    assert_has_tip(&[], &nic_d("pE\\", Bhvadi), Lat, &["pAyayati"]);
}

#[test]
fn sutra_7_3_37_v1() {
    assert_has_tip(&[], &nic_d("pA\\", Adadi), Lat, &["pAlayati"]);
}

#[test]
fn sutra_7_3_37_v2() {
    assert_has_tip(&[], &nic_d("DUY", Svadi), Lat, &["DUnayati", "DAvayati"]);
    assert_has_tip(
        &[],
        &nic_d("prI\\Y", Kryadi),
        Lat,
        &["prIRayati", "prAyayati"],
    );
}

#[test]
fn sutra_7_3_38() {
    assert_has_tip(&[], &nic_d("vA\\", Adadi), Lat, &["vAjayati", "vApayati"]);
}

#[test]
fn sutra_7_3_39() {
    assert_has_tip(
        &["vi"],
        &nic_d("lI\\N", Divadi),
        Lat,
        &["vilInayati", "vilAyayati", "vilAlayati", "vilApayati"],
    );
    assert_has_tip(
        &["vi"],
        &nic_d("lI\\", Kryadi),
        Lat,
        &["vilInayati", "vilAyayati", "vilAlayati", "vilApayati"],
    );

    // snehavipAtane?
    assert_has_tip(
        &["vi"],
        &nic_d("lA\\", Adadi),
        Lat,
        &["vilAlayati", "vilApayati"],
    );
}

#[test]
fn sutra_7_3_40() {
    assert_has_lat(
        &[],
        &nic_d("YiBI\\", Juhotyadi),
        &["BIzayate", "BApayate", "BAyayati", "BAyayate"],
    );
}

#[test]
fn sutra_7_3_41() {
    assert_has_tip(&[], &nic_d("sPAyI~\\", Bhvadi), Lat, &["sPAvayati"]);
}

#[test]
fn sutra_7_3_42() {
    assert_has_tip(
        &[],
        &nic_d("Sa\\dx~", Bhvadi),
        Lat,
        &["SAtayati", "SAdayati"],
    );
}

#[test]
fn sutra_7_3_43() {
    assert_has_tip(
        &[],
        &nic_d("ru\\ha~", Bhvadi),
        Lat,
        &["ropayati", "rohayati"],
    );
}

#[test]
fn sutra_7_3_50() {
    assert_has_taddhita("akza", T::Wak, &["Akzika"]);
    assert_has_taddhita("SalAkA", T::Wak, &["SAlAkika"]);
    assert_has_taddhita("lavaRa", T::WaY, &["lAvaRika"]);
}

#[test]
fn sutra_7_3_51() {
    // is
    assert_has_taddhita("sarpis", T::Wak, &["sArpizka"]);
    // us
    assert_has_taddhita("Danus", T::Wak, &["DAnuzka"]);
    assert_has_taddhita("yajus", T::Wak, &["yAjuzka"]);
    // uk
    assert_has_taddhita("nizAdakarzu", T::Wak, &["nEzAdakarzuka"]);
    assert_has_taddhita("Sabarajambu", T::Wak, &["SAbarajambuka"]);
    assert_has_taddhita("mAtf", T::Wak, &["mAtfka"]);
    assert_has_taddhita("pitf", T::Wak, &["pEtfka"]);
    // t
    assert_has_taddhita("udaSvit", T::Wak, &["OdaSvitka"]);
    assert_has_taddhita("Sakft", T::Wak, &["SAkftka"]);
    assert_has_taddhita("yakft", T::Wak, &["yAkftka"]);
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
    assert_has_jhi(&[], &han, Lat, &["Gnanti"]);

    assert_has_tip(&[], &nic(&han), Lat, &["GAtayati"]);
    assert_has_krdanta(&[], &han, Krt::Rvul, &["GAtaka"]);
    assert_has_krdanta(&[], &han, Krt::Rini, &["GAtin"]);
    assert_has_krdanta(&[], &han, Krt::Ramul, &["GAtam"]);
    assert_has_krdanta(&[], &han, Krt::GaY, &["GAta"]);
    assert_has_jhi(&[], &han, Lot, &["Gnantu"]);
    assert_has_jhi(&[], &han, Lan, &["aGnan"]);
    // hanteH
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&["pra"], &hf, Krt::GaY, &["prahAra"]);
    assert_has_krdanta(&["pra"], &hf, Krt::Rvul, &["prahAraka"]);
}

#[test]
fn sutra_7_3_55() {
    let han = d("ha\\na~", Adadi);
    assert_has_lat(&[], &san(&han), &["jiGAMsati"]);
    // jeGnIyate is from 7.4.30.v1.
    assert_has_lat(&[], &yan(&han), &["jaNGanyate", "jeGnIyate"]);
    assert_has_mip(&[], &han, Lit, &["jaGana", "jaGAna"]);
    // TODO: hananIyitum
}

#[test]
fn sutra_7_3_56() {
    let hi = d("hi\\", Svadi);
    assert_has_lat(&["pra"], &san(&hi), &["prajiGIzati"]);
    assert_has_lat(&["pra"], &yan(&hi), &["prajeGIyate"]);
    assert_has_lit(&["pra"], &hi, &["prajiGAya"]);
    // acaNi
    assert_has_tip(&["pra"], &nic(&hi), Lun, &["prAjIhayat"]);
}

#[test]
fn sutra_7_3_57() {
    let ji = d("ji\\", Bhvadi);
    assert_has_lat(&[], &san(&ji), &["jigIzati"]);
    assert_has_lit(&[], &ji, &["jigAya"]);
    // sanlitoH
    assert_has_lat(&[], &yan(&ji), &["jejIyate"]);
    // exclude jyA -> ji
    let jya = d("jyA\\", Kryadi);
    assert_has_tas(&[], &jya, Lit, &["jijyatuH"]);
}

#[test]
fn sutra_7_3_58() {
    let ci = d("ci\\Y", Svadi);
    assert_has_tip(&[], &san(&ci), Lat, &["cicIzati", "cikIzati"]);
    assert_has_tip(&[], &ci, Lit, &["cikAya", "cicAya"]);
    // sanlitoH
    assert_has_lat(&[], &yan(&ci), &["cecIyate"]);
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

#[test]
fn sutra_7_3_63() {
    let vanc = d("vancu~", Bhvadi);
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
    assert_has_aataam(&[], &duh, Lun, &["aDukzAtAm"]);
    assert_has_aathaam(&[], &duh, Lun, &["aDukzATAm"]);
    assert_has_iw(&[], &duh, Lun, &["aDukzi"]);
    // aci
    assert_has_tip(&[], &duh, Lun, &["aDukzat"]);
    assert_has_tas(&[], &duh, Lun, &["aDukzatAm"]);
    // TODO: others
}

#[test]
fn sutra_7_3_73() {
    let duh = d("du\\ha~^", Adadi);
    assert_has_ta(&[], &duh, Lun, &["adugDa", "aDukzata"]);
    assert_has_thaas(&[], &duh, Lun, &["adugDAH", "aDukzaTAH"]);
    assert_has_dhvam(&[], &duh, Lun, &["aDugDvam", "aDukzaDvam"]);
    assert_has_vahi(&[], &duh, Lun, &["aduhvahi", "aDukzAvahi"]);
    assert_has_ta(&[], &d("di\\ha~^", Adadi), Lun, &["adigDa", "aDikzata"]);
    assert_has_ta(&[], &d("li\\ha~^", Adadi), Lun, &["alIQa", "alikzata"]);
    assert_has_ta(
        &["ni"],
        &d("guhU~^", Bhvadi),
        Lun,
        &["nyagUQa", "nyaGukzata", "nyagUhizwa"],
    );
    // durAdInAm
    // TODO: how to derive this?
    // assert_has_ta(&["vi", "ati"], &d("ru\\Sa~", Tudadi), Lun, &["vyatyarukzata"]);
    // Atmanepade
    assert_has_tip(&[], &duh, Lun, &["aDukzat"]);
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
    assert_has_tip(&[], &d("zWivu~", Bhvadi), Lat, &["zWIvati"]);
    assert_has_tip(&[], &d("klamu~", Bhvadi), Lat, &["klAmati", "klAmyati"]);
    assert_has_tip(&["AN"], &d("camu~", Bhvadi), Lat, &["AcAmati"]);
    // counterexamples
    assert_has_tip(&[], &d("wuvama~", Bhvadi), Lat, &["vamati"]);
    assert_has_tip(&["vi"], &d("camu~", Bhvadi), Lat, &["vicamati"]);
}

#[test]
fn sutra_7_3_76() {
    let kram = d("kramu~", Bhvadi);
    assert_has_tip(&[], &kram, Lat, &["krAmati", "krAmyati"]);
    assert_has_tas(&[], &kram, Lat, &["krAmataH", "krAmyataH"]);
    assert_has_jhi(&[], &kram, Lat, &["krAmanti", "krAmyanti"]);
    // parasmaipadezu
    assert_has_ta(&["AN"], &kram, Lat, &["Akramate", "Akramyate"]);
    // edge cases
    assert_has_sip(
        &["ud"],
        &kram,
        Lot,
        &["utkrAma", "utkrAmatAt", "utkrAmya", "utkrAmyatAt"],
    );
    assert_has_sip(
        &["sam"],
        &kram,
        Lot,
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
    assert_has_tip(&[], &d("pA\\", Bhvadi), Lat, &["pibati"]);
    assert_has_tip(&[], &d("GrA\\", Bhvadi), Lat, &["jiGrati"]);
    assert_has_tip(&[], &d("DmA\\", Bhvadi), Lat, &["Damati"]);
    assert_has_tip(&[], &d("zWA\\", Bhvadi), Lat, &["tizWati"]);
    assert_has_tip(&[], &d("mnA\\", Bhvadi), Lat, &["manati"]);
    assert_has_tip(&[], &d("dA\\R", Bhvadi), Lat, &["yacCati"]);
    assert_has_tip(&[], &d("df\\Si~r", Bhvadi), Lat, &["paSyati"]);
    assert_has_tip(&[], &d("f\\", Bhvadi), Lat, &["fcCati"]);
    assert_has_tip(&[], &d("sf\\", Bhvadi), Lat, &["DAvati", "sarati"]);
    assert_has_ta(&[], &d("Sa\\dx~", Bhvadi), Lat, &["SIyate"]);
    assert_has_tip(&[], &d("za\\dx~", Bhvadi), Lat, &["sIdati"]);
}

#[test]
fn sutra_7_3_79() {
    assert_has_tip(&[], &d("jYA\\", Kryadi), Lat, &["jAnAti"]);
    assert_has_ta(&[], &d("janI~\\", Divadi), Lat, &["jAyate"]);
    // daivAdikasya
    assert_has_tip(&[], &d("jana~", Juhotyadi), Lat, &["jajanti"]);
}

#[test]
fn sutra_7_3_80() {
    assert_has_tip(&[], &d("pUY", Kryadi), Lat, &["punAti"]);
    assert_has_tip(&[], &d("lUY", Kryadi), Lat, &["lunAti"]);
    assert_has_tip(&[], &d("stFY", Kryadi), Lat, &["stfRAti"]);

    // Regression test
    assert_has_tip(&[], &d("jyA\\", Kryadi), Lat, &["jinAti"]);
}

#[test]
fn sutra_7_3_82() {
    let mid = d("YimidA~", Divadi);
    assert_has_lat(&[], &mid, &["medyati"]);
    assert_has_ta_k(&[], &mid, Lat, &["midyate"]);
}

#[test]
fn sutra_7_3_83() {
    assert_has_jhi(&[], &d("hu\\", Juhotyadi), Lan, &["ajuhavuH"]);
    assert_has_jhi(&[], &d("YiBI\\", Juhotyadi), Lan, &["abiBayuH"]);
    assert_has_jhi(&[], &d("quBf\\Y", Juhotyadi), Lan, &["abiBaruH"]);

    assert_has_jhi(&[], &d("ci\\Y", Svadi), VidhiLin, &["cinuyuH"]);
    assert_has_jhi(&[], &d("zu\\Y", Svadi), VidhiLin, &["sunuyuH"]);
}

#[test]
fn sutra_7_3_84() {
    assert_has_tip(&[], &d("tF", Bhvadi), Lat, &["tarati"]);
    assert_has_tip(&[], &d("RI\\Y", Bhvadi), Lat, &["nayati"]);
    assert_has_tip(&[], &d("BU", Bhvadi), Lat, &["Bavati"]);

    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::tfc, &["stotf"]);

    // sArvadhAtukArdhadhAtukayoH
    assert_has_taddhita("agni", Taddhita::tva, &["agnitva"]);
    // TODO: agnikAmyati
}

#[test]
fn sutra_7_3_85() {
    let jagf = d("jAgf", Adadi);

    assert_has_tip(&[], &nic(&jagf), Lat, &["jAgarayati"]);
    assert_has_krdanta(&[], &jagf, Krt::Rvul, &["jAgaraka"]);
    assert_has_krdanta(&[], &jagf, Krt::GaY, &["jAgara"]);
    assert_has_krdanta(&[], &jagf, Krt::kta, &["jAgarita"]);
    assert_has_krdanta(&[], &jagf, Krt::ktavatu, &["jAgaritavat"]);

    assert_has_krdanta(&[], &jagf, Unadi::kvin, &["jAgfvi"]);
    assert_has_ta_k(&[], &jagf, Lun, &["ajAgAri"]);
    assert_has_lit(
        &[],
        &jagf,
        &["jajAgAra", "jAgarAYcakAra", "jAgarAmAsa", "jAgarAmbaBUva"],
    );
    assert_has_tas(&[], &jagf, Lat, &["jAgftaH"]);
}

#[test]
fn sutra_7_3_86() {
    assert_has_tip(&[], &nic_d("vlI\\", Kryadi), Lat, &["vlepayati"]);
    assert_has_tip(&[], &nic_d("hrI\\", Juhotyadi), Lat, &["hrepayati"]);
    assert_has_tip(&[], &nic_d("knUyI~\\", Bhvadi), Lat, &["knopayati"]);

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

    assert_has_mip(&[], &nij, Lot, &["nenijAni"]);
    assert_has_mip(&[], &vij, Lot, &["vevijAni"]);
    assert_has_mip(&["pari"], &viz, Lot, &["parivevizARi"]);

    assert_has_mip(&[], &nij, Lan, &["anenijam"]);
    assert_has_mip(&[], &vij, Lan, &["avevijam"]);
    assert_has_mip(&["pari"], &viz, Lan, &["paryavevizam"]);

    assert_has_mip(&[], &d("vida~", Adadi), Lot, &["vedAni", "vidANkaravARi"]);
    assert_has_tip(&[], &nij, Lat, &["nenekti"]);
    assert_has_tip(&[], &nij, Lit, &["nineja"]);

    let hu = d("hu\\", Juhotyadi);
    assert_has_mip(&[], &hu, Lot, &["juhavAni"]);
    assert_has_mip(&[], &hu, Lan, &["ajuhavam"]);

    // TODO: other examples
}

#[test]
fn sutra_7_3_88() {
    let bhu = d("BU", Bhvadi);
    assert_has_lun(&[], &bhu, &["aBUt"]);
    assert_has_mip(&[], &bhu, Lun, &["aBUvam"]);

    let su = d("zUN", Adadi);
    assert_has_iw(&[], &su, Lot, &["suvE"]);
    assert_has_vahi(&[], &su, Lot, &["suvAvahE"]);
    assert_has_mahin(&[], &su, Lot, &["suvAmahE"]);

    assert_has_lat(&[], &bhu, &["Bavati"]);

    // TODO: boBavIti, boBUtu. How to derive vyatiBavizIzwa?
}

#[test]
fn sutra_7_3_89() {
    let yu = d("yu", Adadi);
    assert_has_tip(&[], &yu, Lat, &["yOti"]);
    assert_has_sip(&[], &yu, Lat, &["yOzi"]);
    assert_has_mip(&[], &yu, Lat, &["yOmi"]);

    let nu = d("Ru", Adadi);
    assert_has_tip(&[], &nu, Lat, &["nOti"]);
    assert_has_sip(&[], &nu, Lat, &["nOzi"]);
    assert_has_mip(&[], &nu, Lat, &["nOmi"]);

    let stu = d("zwu\\Y", Adadi);
    assert_has_tip(&[], &stu, Lat, &["stOti", "stavIti"]);
    assert_has_sip(&[], &stu, Lat, &["stOzi", "stavIzi"]);
    assert_has_mip(&[], &stu, Lat, &["stOmi", "stavImi"]);

    let i = d("i\\R", Adadi);
    assert_has_tip(&[], &i, Lat, &["eti"]);
    assert_has_sip(&[], &i, Lat, &["ezi"]);
    assert_has_mip(&[], &i, Lat, &["emi"]);

    // luki
    let su = d("zu\\Y", Svadi);
    assert_has_tip(&[], &su, Lat, &["sunoti"]);
    assert_has_sip(&[], &su, Lat, &["sunozi"]);
    assert_has_mip(&[], &su, Lat, &["sunomi"]);

    // hali
    let ru = d("ru", Adadi);
    assert_has_mip(&[], &yu, Lot, &["yavAni"]);
    assert_has_mip(&[], &ru, Lot, &["ravARi"]);

    // piti
    assert_has_tas(&[], &yu, Lat, &["yutaH"]);
    assert_has_tas(&[], &ru, Lat, &["rutaH", "ruvItaH"]);

    // yAsuw
    assert_has_tip(&[], &stu, VidhiLin, &["stuyAt", "stuvIyAt"]);

    // TODO: yoyogi, roroti
}

#[test]
fn sutra_7_3_90() {
    let urnu = d("UrRuY", Adadi);
    assert_has_tip(&["pra"], &urnu, Lat, &["prorROti", "prorRoti"]);
    assert_has_sip(&["pra"], &urnu, Lat, &["prorROzi", "prorRozi"]);
    assert_has_mip(&["pra"], &urnu, Lat, &["prorROmi", "prorRomi"]);

    // hali
    assert_has_mip(&["pra"], &urnu, Lot, &["prorRavAni"]);
}

#[test]
fn sutra_7_3_91() {
    let urnu = d("UrRuY", Adadi);
    assert_has_tip(&["pra"], &urnu, Lan, &["prOrRot"]);
    assert_has_sip(&["pra"], &urnu, Lan, &["prOrRoH"]);
}

#[test]
fn sutra_7_3_92() {
    let tfh = d("tfha~", Rudhadi);
    assert_has_tip(&[], &tfh, Lat, &["tfReQi"]);
    assert_has_sip(&[], &tfh, Lat, &["tfRekzi"]);
    assert_has_mip(&[], &tfh, Lat, &["tfRehmi"]);
    assert_has_tip(&[], &tfh, Lan, &["atfRew"]);

    // hali
    assert_has_mip(&[], &tfh, Lot, &["tfRahAni"]);
    // piti
    assert_has_tas(&[], &tfh, Lat, &["tfRQaH"]);
}

#[test]
fn sutra_7_3_93() {
    let bru = d("brUY", Adadi);
    assert_has_tip(&[], &bru, Lat, &["bravIti", "Aha"]);
    assert_has_sip(&[], &bru, Lat, &["bravIzi", "AtTa"]);
    assert_has_mip(&[], &bru, Lat, &["bravImi"]);
    assert_has_tip(&[], &bru, Lan, &["abravIt"]);

    // hali
    assert_has_mip(&[], &bru, Lot, &["bravARi"]);
    // piti
    assert_has_tas(&[], &bru, Lat, &["brUtaH", "AhatuH"]);
}

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
    assert_has_tip(&["upa"], &stu, Lat, &["upastOti", "upastavIti"]);
    assert_has_ta(&[], &stu, VidhiLin, &["stuvIta"]);

    // Sam and am are chAndasa.
}

#[test]
fn sutra_7_3_96() {
    let asa = d("asa~", Adadi);
    assert_has_tip(&[], &asa, Lan, &["AsIt"]);
    assert_has_sip(&[], &asa, Lan, &["AsIH"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_tip(&[], &kf, Lun, &["akArzIt"]);
    assert_has_tip(&[], &d("zu\\Y", Svadi), Lun, &["asAvIt"]);
    assert_has_tip(&[], &d("lUY", Kryadi), Lun, &["alAvIt"]);
    assert_has_tip(&[], &d("pUY", Kryadi), Lun, &["apAvIt"]);

    assert_has_lat(&[], &asa, &["asti"]);
    assert_has_mip(&[], &kf, Lun, &["akArzam"]);
}

#[test]
fn sutra_7_3_98_and_sutra_7_3_99() {
    let rud = d("rudi~r", Adadi);
    assert_has_tip(&[], &rud, Lan, &["arodIt", "arodat"]);
    assert_has_sip(&[], &rud, Lan, &["arodIH", "arodaH"]);

    let svap = d("Yizva\\pa~", Adadi);
    assert_has_tip(&[], &svap, Lan, &["asvapIt", "asvapat"]);
    assert_has_sip(&[], &svap, Lan, &["asvapIH", "asvapaH"]);

    let svas = d("Svasa~", Adadi);
    assert_has_tip(&[], &svas, Lan, &["aSvasIt", "aSvasat"]);
    assert_has_sip(&[], &svas, Lan, &["aSvasIH", "aSvasaH"]);

    let an = d("ana~", Adadi);
    assert_has_tip(&["pra"], &an, Lan, &["prARIt", "prARat"]);
    assert_has_sip(&["pra"], &an, Lan, &["prARIH", "prARaH"]);

    let jaks = d("jakza~", Adadi);
    assert_has_tip(&[], &jaks, Lan, &["ajakzIt", "ajakzat"]);
    assert_has_sip(&[], &jaks, Lan, &["ajakzIH", "ajakzaH"]);

    // pancabhyaH
    let jagf = d("jAgf", Adadi);
    assert_has_sip(&[], &jagf, Lan, &["ajAgaH"]);
    // apkrtasya
    assert_has_lat(&[], &rud, &["roditi"]);
}

#[test]
fn sutra_7_3_100() {
    let ad = d("a\\da~", Adadi);
    assert_has_tip(&[], &ad, Lan, &["Adat"]);
    assert_has_sip(&[], &ad, Lan, &["AdaH"]);
    assert_has_tip(&[], &ad, Lat, &["atti"]);
    assert_has_sip(&[], &ad, Lat, &["atsi"]);
}

#[test]
fn sutra_7_3_101() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_mip(&[], &pac, Lat, &["pacAmi"]);
    assert_has_vas(&[], &pac, Lat, &["pacAvaH"]);
    assert_has_mas(&[], &pac, Lat, &["pacAmaH"]);
    assert_has_mip(&[], &pac, Lrt, &["pakzyAmi"]);
    assert_has_vas(&[], &pac, Lrt, &["pakzyAvaH"]);
    assert_has_mas(&[], &pac, Lrt, &["pakzyAmaH"]);
    // ataH
    let ci = d("ci\\Y", Svadi);
    assert_has_vas(&[], &ci, Lat, &["cinuvaH", "cinvaH"]);
    assert_has_mas(&[], &ci, Lat, &["cinumaH", "cinmaH"]);
    // yaYi
    assert_has_tas(&[], &pac, Lat, &["pacataH"]);
    assert_has_thas(&[], &pac, Lat, &["pacaTaH"]);
    // TODO: sArvadhAtuke
}

#[test]
fn sutra_7_3_102() {
    assert_has_sup_4s("vfkza", Pum, &["vfkzAya"]);
    assert_has_sup_4s("plakza", Pum, &["plakzAya"]);
    assert_has_sup_4d("vfkza", Pum, &["vfkzAByAm"]);
    assert_has_sup_4d("plakza", Pum, &["plakzAByAm"]);
    // ataH
    assert_has_sup_4d("agni", Pum, &["agniByAm"]);
    // yaYi
    assert_has_sup_6s("vfkza", Pum, &["vfkzasya"]);
    assert_has_sup_6s("plakza", Pum, &["plakzasya"]);
}

#[test]
fn sutra_7_3_103() {
    assert_has_sup_4p("vfkza", Pum, &["vfkzeByaH"]);
    assert_has_sup_4p("plakza", Pum, &["plakzeByaH"]);
    assert_has_sup_7p("vfkza", Pum, &["vfkzezu"]);
    assert_has_sup_7p("plakza", Pum, &["plakzezu"]);
    // bahuvacane
    assert_has_sup_4d("vfkza", Pum, &["vfkzAByAm"]);
    assert_has_sup_4d("plakza", Pum, &["plakzAByAm"]);
    // Jali
    assert_has_sup_6p("vfkza", Pum, &["vfkzARAm"]);
    // TODO: supi
}

#[test]
fn sutra_7_3_104() {
    assert_has_sup_6d("vfkza", Pum, &["vfkzayoH"]);
    assert_has_sup_6d("plakza", Pum, &["plakzayoH"]);
    assert_has_sup_7d("vfkza", Pum, &["vfkzayoH"]);
    assert_has_sup_7d("plakza", Pum, &["plakzayoH"]);
}

#[test]
fn sutra_7_3_105() {
    assert_has_sup_3s(&nyap("KawvA"), Stri, &["KawvayA"]);
    assert_has_sup_3s(&nyap("mAlA"), Stri, &["mAlayA"]);
    assert_has_sup_6d(&nyap("KawvA"), Stri, &["KawvayoH"]);
    assert_has_sup_6d(&nyap("mAlA"), Stri, &["mAlayoH"]);
    assert_has_sup_3s(&nyap("bahurAjA"), Stri, &["bahurAjayA"]);
    assert_has_sup_3s(&nyap("kArIzaganDyA"), Stri, &["kArIzaganDyayA"]);
    assert_has_sup_6d(&nyap("bahurAjA"), Stri, &["bahurAjayoH"]);
    assert_has_sup_6d(&nyap("kArIzaganDyA"), Stri, &["kArIzaganDyayoH"]);
    // TODO: add others
}

#[test]
fn sutra_7_3_106() {
    assert_has_sup_ss(&nyap("KawvA"), Stri, &["Kawve"]);
    assert_has_sup_ss(&nyap("bahurAjA"), Stri, &["bahurAje"]);
    assert_has_sup_ss(&nyap("kArIzaganDyA"), Stri, &["kArIzaganDye"]);
}

#[test]
fn sutra_7_3_107() {
    assert_has_sup_ss(&nyap("ambA"), Stri, &["amba"]);
    assert_has_sup_ss(&nyap("akkA"), Stri, &["akka"]);
    assert_has_sup_ss(&nyap("allA"), Stri, &["alla"]);
    // nadyoH
    assert_has_sup_ss(&nyap("kumArI"), Stri, &["kumAri"]);
    assert_has_sup_ss(&nyap("SArNgaravI"), Stri, &["SArNgaravi"]);
    assert_has_sup_ss("brahmabanDU", Stri, &["brahmabanDu"]);
    assert_has_sup_ss("vIrabanDU", Stri, &["vIrabanDu"]);
}

#[test]
fn sutra_7_3_108() {
    assert_has_sup_ss("agni", Pum, &["agne"]);
    assert_has_sup_ss("vAyu", Pum, &["vAyo"]);
    assert_has_sup_ss("pawu", Pum, &["pawo"]);
    // But, not for these because the prAtipadika was not originally hrasva
    assert_has_sup_ss(&nyap("kumArI"), Stri, &["kumAri"]);
    assert_has_sup_ss("brahmabanDU", Stri, &["brahmabanDu"]);
}

#[test]
fn sutra_7_3_109() {
    assert_has_sup_1p("agni", Pum, &["agnayaH"]);
    assert_has_sup_1p("vAyu", Pum, &["vAyavaH"]);
    assert_has_sup_1p("pawu", Pum, &["pawavaH"]);
    assert_has_sup_1p("Denu", Stri, &["DenavaH"]);
    assert_has_sup_1p("budDi", Stri, &["budDayaH"]);
}

#[test]
fn sutra_7_3_110() {
    // Ni
    assert_has_sup_7s("mAtf", Stri, &["mAtari"]);
    assert_has_sup_7s("pitf", Pum, &["pitari"]);
    assert_has_sup_7s("BrAtf", Pum, &["BrAtari"]);
    assert_has_sup_7s("kartf", Pum, &["kartari"]);

    // sarvanAmasTAne
    let kartr = create_krdanta("kartf", &[], &d("qukf\\Y", Tanadi), Krt::tfc);
    assert_has_sup_1d(&kartr, Pum, &["kartArO"]);
    assert_has_sup_1p(&kartr, Pum, &["kartAraH"]);
    assert_has_sup_1d("mAtf", Stri, &["mAtarO"]);
    assert_has_sup_1d("pitf", Pum, &["pitarO"]);
    assert_has_sup_1d("BrAtf", Pum, &["BrAtarO"]);
}

#[test]
fn sutra_7_3_111() {
    assert_has_sup_4s("agni", Pum, &["agnaye"]);
    assert_has_sup_4s("vAyu", Pum, &["vAyave"]);
    assert_has_sup_5s("agni", Pum, &["agneH"]);
    assert_has_sup_5s("vAyu", Pum, &["vAyoH"]);
    assert_has_sup_6s("agni", Pum, &["agneH"]);
    assert_has_sup_6s("vAyu", Pum, &["vAyoH"]);
    // GeH
    assert_has_sup_4s("saKi", Pum, &["saKye"]);
    assert_has_sup_4s("pati", Pum, &["patye"]);
    // Niti
    assert_has_sup_4d("agni", Pum, &["agniByAm"]);
    // TODO: supi
}

#[test]
fn sutra_7_3_112() {
    assert_has_sup_4s("kumArI", Stri, &["kumAryE"]);
    assert_has_sup_4s("brahmabanDU", Stri, &["brahmabanDvE"]);
    assert_has_sup_5s("kumArI", Stri, &["kumAryAH"]);
    assert_has_sup_5s("brahmabanDU", Stri, &["brahmabanDvAH"]);
}

#[test]
fn sutra_7_3_113() {
    assert_has_sup_4s(&nyap("KawvA"), Stri, &["KawvAyE"]);
    assert_has_sup_4s(&nyap("bahurAjA"), Stri, &["bahurAjAyE"]);
    assert_has_sup_4s(&nyap("kArIzaganDyA"), Stri, &["kArIzaganDyAyE"]);
    assert_has_sup_5s(&nyap("KawvA"), Stri, &["KawvAyAH"]);
    assert_has_sup_5s(&nyap("bahurAjA"), Stri, &["bahurAjAyAH"]);
    assert_has_sup_5s(&nyap("kArIzaganDyA"), Stri, &["kArIzaganDyAyAH"]);
}

#[test]
fn sutra_7_3_114() {
    assert_has_sup_4s("sarva", Stri, &["sarvasyE"]);
    assert_has_sup_4s("viSva", Stri, &["viSvasyE"]);
    assert_has_sup_4s("yad", Stri, &["yasyE"]);
    assert_has_sup_4s("tad", Stri, &["tasyE"]);
    assert_has_sup_4s("kim", Stri, &["kasyE"]);
    assert_has_sup_4s("anya", Stri, &["anyasyE"]);
    assert_has_sup_5s("sarva", Stri, &["sarvasyAH"]);
    assert_has_sup_5s("viSva", Stri, &["viSvasyAH"]);
    assert_has_sup_5s("yad", Stri, &["yasyAH"]);
    assert_has_sup_5s("tad", Stri, &["tasyAH"]);
    assert_has_sup_5s("kim", Stri, &["kasyAH"]);
    assert_has_sup_5s("anya", Stri, &["anyasyAH"]);

    // ApaH?
    // TODO: what does KV mean here?
    assert_has_sup_7s("Bavat", Stri, &["Bavati"]);
    assert_has_sup_4s("Bavat", Stri, &["Bavate"]);
}

#[test]
fn sutra_7_3_116() {
    assert_has_sup_7s(&nyap("kumArI"), Stri, &["kumAryAm"]);
    assert_has_sup_7s(&nyap("gOrI"), Stri, &["gOryAm"]);
    assert_has_sup_7s("brahmabanDU", Stri, &["brahmabanDvAm"]);
    assert_has_sup_7s("DIBanDU", Stri, &["DIBanDvAm"]);
}

#[test]
fn sutra_7_3_117() {
    assert_has_sup_7s("kfti", Stri, &["kftO", "kftyAm"]);
    assert_has_sup_7s("Denu", Stri, &["DenO", "DenvAm"]);
}

#[test]
fn sutra_7_3_118() {
    assert_has_sup_7s("saKi", Pum, &["saKyO"]);
    assert_has_sup_7s("pati", Pum, &["patyO"]);
}

#[test]
fn sutra_7_3_119() {
    assert_has_sup_7s("agni", Pum, &["agnO"]);
    assert_has_sup_7s("vAyu", Pum, &["vAyO"]);
    assert_has_sup_7s("kfti", Stri, &["kftO", "kftyAm"]);
    assert_has_sup_7s("Denu", Stri, &["DenO", "DenvAm"]);
    assert_has_sup_7s("pawu", Pum, &["pawO"]);
}

#[test]
fn sutra_7_3_120() {
    assert_has_sup_3s("agni", Pum, &["agninA"]);
    assert_has_sup_3s("vAyu", Pum, &["vAyunA"]);
    assert_has_sup_3s("pawu", Pum, &["pawunA"]);
    // astriyAm
    assert_has_sup_3s("kfti", Stri, &["kftyA"]);
    assert_has_sup_3s("Denu", Stri, &["DenvA"]);
    // TODO: others
}
