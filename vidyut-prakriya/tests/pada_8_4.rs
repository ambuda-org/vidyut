extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

pub fn assert_has_lot_p_1s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Uttama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lot)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();

    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

#[test]
fn sutra_8_4_14() {
    let nam = d("Ra\\ma~", Bhvadi);
    assert_has_lat(&["pra"], &nam, &["praRamati"]);
    assert_has_lat(&["pari"], &nam, &["pariRamati"]);

    let ni = d("RI\\Y", Bhvadi);
    assert_has_krdanta(&["pra"], &ni, Krt::Rvul, &["praRAyaka"]);
    assert_has_krdanta(&["pari"], &ni, Krt::Rvul, &["pariRAyaka"]);

    let nard = d("narda~", Bhvadi);
    assert_has_lat(&["pra"], &nard, &["pranardati"]);
    assert_has_krdanta(&["pra"], &nard, Krt::Rvul, &["pranardaka"]);
}

#[test]
fn sutra_8_4_15() {
    let hi = d("hi\\", Svadi);
    let mi = d("mI\\Y", Kryadi);
    assert_has_lat(&["pra"], &hi, &["prahiRoti"]);
    assert_has_lat_p(&["pra"], &mi, &["pramIRAti"]);
}

#[test]
fn sutra_8_4_16() {
    let vap = d("quva\\pa~^", Bhvadi);
    let yaa = d("yA\\", Adadi);
    assert_has_lot_p_1s(&["pra"], &vap, &["pravapARi"]);
    assert_has_lot_p_1s(&["pari"], &vap, &["parivapARi"]);
    assert_has_lot_p_1s(&["pra"], &yaa, &["prayARi"]);
    assert_has_lot_p_1s(&["pari"], &yaa, &["pariyARi"]);
}

#[test]
fn sutra_8_4_17() {
    let gad = d("gada~", Bhvadi);
    assert_has_lat(&["pra", "ni"], &gad, &["praRigadati"]);
    assert_has_lat(&["pari", "ni"], &gad, &["pariRigadati"]);

    let nad = d("Rada~", Bhvadi);
    assert_has_lat(&["pra", "ni"], &nad, &["praRinadati"]);
    assert_has_lat(&["pari", "ni"], &nad, &["pariRinadati"]);

    let pat = d("patx~", Bhvadi);
    assert_has_lat(&["pra", "ni"], &pat, &["praRipatati"]);
    assert_has_lat(&["pari", "ni"], &pat, &["pariRipatati"]);

    let pad = d("pa\\da~\\", Divadi);
    assert_has_lat(&["pra", "ni"], &pad, &["praRipadyate"]);
    assert_has_lat(&["pari", "ni"], &pad, &["pariRipadyate"]);

    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_lat_p(&["pra", "ni"], &daa, &["praRidadAti"]);
    assert_has_lat_p(&["pari", "ni"], &daa, &["pariRidadAti"]);

    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_lat_p(&["pra", "ni"], &dhaa, &["praRidaDAti"]);
    assert_has_lat_p(&["pari", "ni"], &dhaa, &["pariRidaDAti"]);

    let maa = d("mA\\N", Juhotyadi);
    assert_has_lat(&["pra", "ni"], &maa, &["praRimimIte"]);
    assert_has_lat(&["pari", "ni"], &maa, &["pariRimimIte"]);

    let me = d("me\\N", Bhvadi);
    assert_has_lat(&["pra", "ni"], &me, &["praRimayate"]);
    assert_has_lat(&["pari", "ni"], &me, &["pariRimayate"]);

    let so = d("zo\\", Divadi);
    assert_has_lat(&["pra", "ni"], &so, &["praRizyati"]);
    assert_has_lat(&["pari", "ni"], &so, &["pariRizyati"]);

    let han = d("ha\\na~", Adadi);
    assert_has_lat(&["pra", "ni"], &han, &["praRihanti"]);
    assert_has_lat(&["pari", "ni"], &han, &["pariRihanti"]);

    let yaa = d("yA\\", Adadi);
    assert_has_lat(&["pra", "ni"], &yaa, &["praRiyAti"]);
    assert_has_lat(&["pari", "ni"], &yaa, &["pariRiyAti"]);

    let vaa = d("vA\\", Adadi);
    assert_has_lat(&["pra", "ni"], &vaa, &["praRivAti"]);
    assert_has_lat(&["pari", "ni"], &vaa, &["pariRivAti"]);

    let draa = d("drA\\", Adadi);
    assert_has_lat(&["pra", "ni"], &draa, &["praRidrAti"]);
    assert_has_lat(&["pari", "ni"], &draa, &["pariRidrAti"]);

    let psaa = d("psA\\", Adadi);
    assert_has_lat(&["pra", "ni"], &psaa, &["praRipsAti"]);
    assert_has_lat(&["pari", "ni"], &psaa, &["pariRipsAti"]);

    let vap = d("quva\\pa~^", Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &vap, &["praRivapati"]);
    assert_has_lat_p(&["pari", "ni"], &vap, &["pariRivapati"]);

    let vah = d("va\\ha~^", Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &vah, &["praRivahati"]);
    assert_has_lat_p(&["pari", "ni"], &vah, &["pariRivahati"]);

    let sham = d("Samu~", Divadi);
    assert_has_lat(&["pra", "ni"], &sham, &["praRiSAmyati"]);
    assert_has_lat(&["pari", "ni"], &sham, &["pariRiSAmyati"]);

    let ci = d("ci\\Y", Svadi);
    assert_has_lat_p(&["pra", "ni"], &ci, &["praRicinoti"]);
    assert_has_lat_p(&["pari", "ni"], &ci, &["pariRicinoti"]);

    let dih = d("di\\ha~^", Adadi);
    assert_has_lat_p(&["pra", "ni"], &dih, &["praRidegDi"]);
    assert_has_lat_p(&["pari", "ni"], &dih, &["pariRidegDi"]);

    // Also applies for aw-vyavAya
    assert_has_lan(&["pra", "ni"], &gad, &["praRyagadat"]);
    assert_has_lan(&["pari", "ni"], &gad, &["pariRyagadat"]);
}

#[test]
fn sutra_8_4_18() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &pac, &["praRipacati", "pranipacati"]);

    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_lat_p(&["pra", "ni"], &bhid, &["praRiBinatti", "praniBinatti"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_lat_p(&["pra", "ni"], &kf, &["pranikaroti"]);

    let khaad = d("KAdf~", Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &khaad, &["praniKAdati"]);

    let pish = d("pi\\zx~", Rudhadi);
    assert_has_lat_p(&["pra", "ni"], &pish, &["pranipinazwi"]);

    // Examples of why the rule has "upadesha"
    assert_has_lit_p(&["pra", "ni"], &kf, &["pranicakAra"]);
    assert_has_lit_p(&["pra", "ni"], &khaad, &["pranicaKAda"]);
    assert_has_lrt(&["pra", "ni"], &pish, &["pranipekzyati"]);

    let vish = d("vi\\Sa~", Tudadi);
    assert_has_lut(&["pra", "ni"], &vish, &["praRivezwA", "pranivezwA"]);
    // Per Neelesh, the `pranivekzyati` in the KV online is likely a typo.
    assert_has_lrt(&["pra", "ni"], &vish, &["praRivekzyate", "pranivekzyate"]);
}

#[test]
fn sutra_8_4_19() {
    let an = d("ana~", Adadi);
    assert_has_lat(&["pra"], &an, &["prARiti"]);
    assert_has_lat(&["parA"], &an, &["parARiti"]);
}

#[test]
fn sutra_8_4_21() {
    let an = d("ana~", Adadi);
    let nic = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Nic]);
    assert_has_lat(&["pra"], &san(&an), &["prARiRizati"]);
    assert_has_lun(&["pra"], &nic(&an), &["prARiRat"]);
    assert_has_lat(&["parA"], &san(&an), &["parARiRizati"]);
    assert_has_lun(&["parA"], &nic(&an), &["parARiRat"]);
}

#[test]
fn sutra_8_4_35() {
    let paa = d("pA\\", Bhvadi);
    assert_has_krdanta(&["nis"], &paa, Krt::lyuw, &["nizpAna"]);
}

#[test]
fn sutra_8_4_36() {
    let nas = d("Ra\\Sa~", Divadi);
    assert_has_krdanta(&["pra"], &nas, Krt::kta, &["pranazwa"]);
    assert_has_krdanta(&["pari"], &nas, Krt::kta, &["parinazwa"]);
    // zAntasya
    assert_has_lat(&["pra"], &nas, &["praRaSyati"]);
    assert_has_lat(&["pari"], &nas, &["pariRaSyati"]);
    // others
    assert_has_lrt(&["pra"], &nas, &["pranaNkzyati", "praRaSizyati"]);
    assert_has_lrt(&["pari"], &nas, &["parinaNkzyati", "pariRaSizyati"]);
}

#[test]
fn sutra_8_4_37() {
    use Vibhakti::*;
    assert_has_subantas("vfkza", Pum, Dvitiya, Bahu, &["vfkzAn"]);
    assert_has_subantas("plakza", Pum, Dvitiya, Bahu, &["plakzAn"]);
    assert_has_subantas("ari", Pum, Dvitiya, Bahu, &["arIn"]);
    assert_has_subantas("giri", Pum, Dvitiya, Bahu, &["girIn"]);
}

#[test]
fn sutra_8_4_39() {
    let kzubh = d("kzuBa~", Kryadi);
    assert_has_parasmai_tinanta(&[], &kzubh, Lat, Prathama, Eka, &["kzuBnAti"]);
    assert_has_parasmai_tinanta(&[], &kzubh, Lat, Prathama, Dvi, &["kzuBnItaH"]);
    assert_has_parasmai_tinanta(&[], &kzubh, Lat, Prathama, Bahu, &["kzuBnanti"]);
    // TODO: others
}

#[test]
fn sutra_8_4_44() {
    assert_has_krdanta(&[], &d("pra\\Ca~", Tudadi), Krt::naN, &["praSna"]);
    assert_has_krdanta(&[], &d("viCa~", Tudadi), Krt::naN, &["viSna"]);
}

#[test]
fn sutra_8_4_53() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_krdanta(&[], &labh, Krt::tfc, &["labDf"]);
    assert_has_krdanta(&[], &labh, Krt::tumun, &["labDum"]);
    assert_has_krdanta(&[], &labh, Krt::tavya, &["labDavya"]);
    let duh = d("du\\ha~^", Adadi);
    assert_has_krdanta(&[], &duh, Krt::tfc, &["dogDf"]);
    assert_has_krdanta(&[], &duh, Krt::tumun, &["dogDum"]);
    assert_has_krdanta(&[], &duh, Krt::tavya, &["dogDavya"]);
    let budh = d("bu\\Da~\\", Divadi);
    assert_has_krdanta(&[], &budh, Krt::tfc, &["bodDf"]);
    assert_has_krdanta(&[], &budh, Krt::tumun, &["bodDum"]);
    assert_has_krdanta(&[], &budh, Krt::tavya, &["bodDavya"]);
    // JaSi
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_parasmai_tinanta(&[], &daa, Lat, Prathama, Dvi, &["dattaH"]);
    assert_has_parasmai_tinanta(&[], &daa, Lat, Madhyama, Dvi, &["datTaH"]);
    assert_has_parasmai_tinanta(
        &[],
        &d("quDA\\Y", Juhotyadi),
        Lat,
        Uttama,
        Bahu,
        &["daDmaH"],
    );
}

#[test]
fn sutra_8_4_55() {
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_krdanta(&[], &bhid, Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &bhid, Krt::tumun, &["Bettum"]);
    assert_has_krdanta(&[], &bhid, Krt::tavya, &["Bettavya"]);
    assert_has_lat(&[], &san(&d("yu\\Da~\\", Divadi)), &["yuyutsate"]);
    assert_has_lat(&["AN"], &san(&d("ra\\Ba~\\", Bhvadi)), &["Aripsate"]);
    assert_has_lat(&["AN"], &san(&d("qula\\Ba~\\z", Bhvadi)), &["Alipsate"]);
}

#[test]
fn sutra_8_4_58() {
    let sank = d("Saki~\\", Bhvadi);
    assert_has_krdanta(&[], &sank, Krt::tfc, &["SaNkitf"]);
    assert_has_krdanta(&[], &sank, Krt::tumun, &["SaNkitum"]);
    assert_has_krdanta(&[], &sank, Krt::tavya, &["SaNkitavya"]);
    let unch = d("uCi~", Bhvadi);
    assert_has_krdanta(&[], &unch, Krt::tfc, &["uYCitf"]);
    assert_has_krdanta(&[], &unch, Krt::tumun, &["uYCitum"]);
    assert_has_krdanta(&[], &unch, Krt::tavya, &["uYCitavya"]);
    let kund = d("kuqi~\\", Bhvadi);
    assert_has_krdanta(&[], &kund, Krt::tfc, &["kuRqitf"]);
    assert_has_krdanta(&[], &kund, Krt::tumun, &["kuRqitum"]);
    assert_has_krdanta(&[], &kund, Krt::tavya, &["kuRqitavya"]);
    let nand = d("wunadi~", Bhvadi);
    assert_has_krdanta(&[], &nand, Krt::tfc, &["nanditf"]);
    assert_has_krdanta(&[], &nand, Krt::tumun, &["nanditum"]);
    assert_has_krdanta(&[], &nand, Krt::tavya, &["nanditavya"]);
    let kamp = d("kapi~\\", Bhvadi);
    assert_has_krdanta(&[], &kamp, Krt::tfc, &["kampitf"]);
    assert_has_krdanta(&[], &kamp, Krt::tumun, &["kampitum"]);
    assert_has_krdanta(&[], &kamp, Krt::tavya, &["kampitavya"]);
}
