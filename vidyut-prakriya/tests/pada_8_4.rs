extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

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
    let nam = Dhatu::new("Ra\\ma~", Gana::Bhvadi);
    assert_has_lat(&["pra"], &nam, &["praRamati"]);
    assert_has_lat(&["pari"], &nam, &["pariRamati"]);

    let ni = Dhatu::new("RI\\Y", Gana::Bhvadi);
    assert_has_krdanta(&["pra"], &ni, Krt::Rvul, &["praRAyaka"]);
    assert_has_krdanta(&["pari"], &ni, Krt::Rvul, &["pariRAyaka"]);

    let nard = Dhatu::new("narda~", Gana::Bhvadi);
    assert_has_lat(&["pra"], &nard, &["pranardati"]);
    assert_has_krdanta(&["pra"], &nard, Krt::Rvul, &["pranardaka"]);
}

#[test]
fn sutra_8_4_15() {
    let hi = Dhatu::new("hi\\", Gana::Svadi);
    let mi = Dhatu::new("mI\\Y", Gana::Kryadi);
    assert_has_lat(&["pra"], &hi, &["prahiRoti"]);
    assert_has_lat_p(&["pra"], &mi, &["pramIRAti"]);
}

#[test]
fn sutra_8_4_16() {
    let vap = Dhatu::new("quva\\pa~^", Gana::Bhvadi);
    let yaa = Dhatu::new("yA\\", Gana::Adadi);
    assert_has_lot_p_1s(&["pra"], &vap, &["pravapARi"]);
    assert_has_lot_p_1s(&["pari"], &vap, &["parivapARi"]);
    assert_has_lot_p_1s(&["pra"], &yaa, &["prayARi"]);
    assert_has_lot_p_1s(&["pari"], &yaa, &["pariyARi"]);
}

#[test]
fn sutra_8_4_17() {
    let gad = Dhatu::new("gada~", Gana::Bhvadi);
    assert_has_lat(&["pra", "ni"], &gad, &["praRigadati"]);
    assert_has_lat(&["pari", "ni"], &gad, &["pariRigadati"]);

    let nad = Dhatu::new("Rada~", Gana::Bhvadi);
    assert_has_lat(&["pra", "ni"], &nad, &["praRinadati"]);
    assert_has_lat(&["pari", "ni"], &nad, &["pariRinadati"]);

    let pat = Dhatu::new("patx~", Gana::Bhvadi);
    assert_has_lat(&["pra", "ni"], &pat, &["praRipatati"]);
    assert_has_lat(&["pari", "ni"], &pat, &["pariRipatati"]);

    let pad = Dhatu::new("pa\\da~\\", Gana::Divadi);
    assert_has_lat(&["pra", "ni"], &pad, &["praRipadyate"]);
    assert_has_lat(&["pari", "ni"], &pad, &["pariRipadyate"]);

    let daa = Dhatu::new("qudA\\Y", Gana::Juhotyadi);
    assert_has_lat_p(&["pra", "ni"], &daa, &["praRidadAti"]);
    assert_has_lat_p(&["pari", "ni"], &daa, &["pariRidadAti"]);

    let dhaa = Dhatu::new("quDA\\Y", Gana::Juhotyadi);
    assert_has_lat_p(&["pra", "ni"], &dhaa, &["praRidaDAti"]);
    assert_has_lat_p(&["pari", "ni"], &dhaa, &["pariRidaDAti"]);

    let maa = Dhatu::new("mA\\N", Gana::Juhotyadi);
    assert_has_lat(&["pra", "ni"], &maa, &["praRimimIte"]);
    assert_has_lat(&["pari", "ni"], &maa, &["pariRimimIte"]);

    let me = Dhatu::new("me\\N", Gana::Bhvadi);
    assert_has_lat(&["pra", "ni"], &me, &["praRimayate"]);
    assert_has_lat(&["pari", "ni"], &me, &["pariRimayate"]);

    let so = Dhatu::new("zo\\", Gana::Divadi);
    assert_has_lat(&["pra", "ni"], &so, &["praRizyati"]);
    assert_has_lat(&["pari", "ni"], &so, &["pariRizyati"]);

    let han = Dhatu::new("ha\\na~", Gana::Adadi);
    assert_has_lat(&["pra", "ni"], &han, &["praRihanti"]);
    assert_has_lat(&["pari", "ni"], &han, &["pariRihanti"]);

    let yaa = Dhatu::new("yA\\", Gana::Adadi);
    assert_has_lat(&["pra", "ni"], &yaa, &["praRiyAti"]);
    assert_has_lat(&["pari", "ni"], &yaa, &["pariRiyAti"]);

    let vaa = Dhatu::new("vA\\", Gana::Adadi);
    assert_has_lat(&["pra", "ni"], &vaa, &["praRivAti"]);
    assert_has_lat(&["pari", "ni"], &vaa, &["pariRivAti"]);

    let draa = Dhatu::new("drA\\", Gana::Adadi);
    assert_has_lat(&["pra", "ni"], &draa, &["praRidrAti"]);
    assert_has_lat(&["pari", "ni"], &draa, &["pariRidrAti"]);

    let psaa = Dhatu::new("psA\\", Gana::Adadi);
    assert_has_lat(&["pra", "ni"], &psaa, &["praRipsAti"]);
    assert_has_lat(&["pari", "ni"], &psaa, &["pariRipsAti"]);

    let vap = Dhatu::new("quva\\pa~^", Gana::Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &vap, &["praRivapati"]);
    assert_has_lat_p(&["pari", "ni"], &vap, &["pariRivapati"]);

    let vah = Dhatu::new("va\\ha~^", Gana::Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &vah, &["praRivahati"]);
    assert_has_lat_p(&["pari", "ni"], &vah, &["pariRivahati"]);

    let sham = Dhatu::new("Samu~", Gana::Divadi);
    assert_has_lat(&["pra", "ni"], &sham, &["praRiSAmyati"]);
    assert_has_lat(&["pari", "ni"], &sham, &["pariRiSAmyati"]);

    let ci = Dhatu::new("ci\\Y", Gana::Svadi);
    assert_has_lat_p(&["pra", "ni"], &ci, &["praRicinoti"]);
    assert_has_lat_p(&["pari", "ni"], &ci, &["pariRicinoti"]);

    let dih = Dhatu::new("di\\ha~^", Gana::Adadi);
    assert_has_lat_p(&["pra", "ni"], &dih, &["praRidegDi"]);
    assert_has_lat_p(&["pari", "ni"], &dih, &["pariRidegDi"]);

    // Also applies for aw-vyavAya
    assert_has_lan(&["pra", "ni"], &gad, &["praRyagadat"]);
    assert_has_lan(&["pari", "ni"], &gad, &["pariRyagadat"]);
}

#[test]
fn sutra_8_4_18() {
    let pac = Dhatu::new("qupa\\ca~^z", Gana::Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &pac, &["praRipacati", "pranipacati"]);

    let bhid = Dhatu::new("Bi\\di~^r", Gana::Rudhadi);
    assert_has_lat_p(&["pra", "ni"], &bhid, &["praRiBinatti", "praniBinatti"]);

    let kf = Dhatu::new("qukf\\Y", Gana::Tanadi);
    assert_has_lat_p(&["pra", "ni"], &kf, &["pranikaroti"]);

    let khaad = Dhatu::new("KAdf~", Gana::Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &khaad, &["praniKAdati"]);

    let pish = Dhatu::new("pi\\zx~", Gana::Rudhadi);
    assert_has_lat_p(&["pra", "ni"], &pish, &["pranipinazwi"]);

    // Examples of why the rule has "upadesha"
    assert_has_lit_p(&["pra", "ni"], &kf, &["pranicakAra"]);
    assert_has_lit_p(&["pra", "ni"], &khaad, &["pranicaKAda"]);
    assert_has_lrt(&["pra", "ni"], &pish, &["pranipekzyati"]);

    let vish = Dhatu::new("vi\\Sa~", Gana::Tudadi);
    assert_has_lut(&["pra", "ni"], &vish, &["praRivezwA", "pranivezwA"]);
    // Per Neelesh, the `pranivekzyati` in the KV online is likely a typo.
    assert_has_lrt(&["pra", "ni"], &vish, &["praRivekzyate", "pranivekzyate"]);
}

#[test]
fn sutra_8_4_19() {
    let an = Dhatu::new("ana~", Gana::Adadi);
    assert_has_lat_p(&["pra"], &an, &["prARiti"]);
    assert_has_lat_p(&["parA"], &an, &["parARiti"]);
}

#[test]
fn sutra_8_4_35() {
    let paa = Dhatu::new("pA\\", Gana::Bhvadi);
    assert_has_krdanta(&["nis"], &paa, Krt::lyuw, &["nizpAna"]);
}
