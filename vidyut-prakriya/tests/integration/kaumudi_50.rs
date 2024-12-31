extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as K;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2547() {
    let tan = d("tanu~^", Tanadi);
    assert_has_tip(&[], &tan, Lat, &["tanoti"]);
    assert_has_vas(&[], &tan, Lat, &["tanvaH", "tanuvaH"]);
    assert_has_ta(&[], &tan, Lat, &["tanute"]);
    assert_has_tip(&[], &tan, Lit, &["tatAna"]);
    assert_has_ta(&[], &tan, Lit, &["tene"]);
    assert_has_sip(&[], &tan, Lot, &["tanu", "tanutAt"]);
    assert_has_tip(&[], &tan, Lun, &["atanIt", "atAnIt"]);
    assert_has_ta(&[], &tan, Lun, &["atata", "atanizwa"]);
    assert_has_thaas(&[], &tan, Lun, &["ataTAH", "atanizWAH"]);

    let shan = d("zaRu~^", Tanadi);
    assert_has_tip(&[], &shan, Lat, &["sanoti"]);
    assert_has_ta(&[], &shan, Lat, &["sanute"]);
    assert_has_tip(&[], &shan, AshirLin, &["sAyAt", "sanyAt"]);
    assert_has_ta(&[], &shan, Lun, &["asAta", "asanizwa"]);
    assert_has_thaas(&[], &shan, Lun, &["asATAH", "asanizWAH"]);

    let kshan = d("kzaRu~^", Tanadi);
    assert_has_tip(&[], &kshan, Lat, &["kzaRoti"]);
    assert_has_ta(&[], &kshan, Lat, &["kzaRute"]);
    assert_has_tip(&[], &kshan, Lun, &["akzaRIt"]);
    assert_has_ta(&[], &kshan, Lun, &["akzata", "akzaRizwa"]);
    assert_has_thaas(&[], &kshan, Lun, &["akzaTAH", "akzaRizWAH"]);

    // Per SK, `u` indicates optional upaDA-guna.
    // This seems to extend to fRu, tfRu, GfRu, etc. per other sources.
    let kshin = d("kziRu~^", Tanadi);
    assert_has_tip(&[], &kshin, Lat, &["kziRoti", "kzeRoti"]);
    assert_has_sip(&[], &kshin, Lut, &["kzeRitAsi"]);
    assert_has_thaas(&[], &kshin, Lut, &["kzeRitAse"]);
    assert_has_tip(&[], &kshin, Lun, &["akzeRIt"]);
    assert_has_ta(&[], &kshin, Lun, &["akzita", "akzeRizwa"]);

    let rn = d("fRu~^", Tanadi);
    assert_has_tip(&[], &rn, Lat, &["arRoti", "fRoti"]);
    assert_has_tas(&[], &rn, Lat, &["arRutaH", "fRutaH"]);
    assert_has_jhi(&[], &rn, Lat, &["arRvanti", "fRvanti"]);
    assert_has_tip(&[], &rn, Lit, &["AnarRa"]);
    assert_has_ta(&[], &rn, Lit, &["AnfRe"]);
    assert_has_thaas(&[], &rn, Lut, &["arRitAse"]);
    assert_has_tip(&[], &rn, Lun, &["ArRIt"]);
    assert_has_ta(&[], &rn, Lun, &["Arta", "ArRizwa"]);
    assert_has_thaas(&[], &rn, Lun, &["ArTAH", "ArRizWAH"]);

    let trn = d("tfRu~^", Tanadi);
    assert_has_tip(&[], &trn, Lat, &["tfRoti", "tarRoti"]);
    assert_has_ta(&[], &trn, Lat, &["tfRute", "tarRute"]);

    let ghrn = d("GfRu~^", Tanadi);
    assert_has_tip(&[], &ghrn, Lit, &["jaGarRa"]);
    assert_has_ta(&[], &ghrn, Lit, &["jaGfRe"]);

    let van = d("vanu~\\", Tanadi);
    assert_has_ta(&[], &van, Lat, &["vanute"]);
    assert_has_ta(&[], &van, Lit, &["vavane"]);
    assert_has_tip(&[], &van, Lat, &["vanoti"]);
    assert_has_tip(&[], &van, Lit, &["vavAna"]);

    let man = d("manu~\\", Tanadi);
    assert_has_ta(&[], &man, Lat, &["manute"]);
    assert_has_ta(&[], &man, Lit, &["mene"]);

    let kr = d("qukf\\Y", Tanadi);
    assert_has_tip(&[], &kr, Lat, &["karoti"]);
    assert_has_jhi(&[], &kr, Lat, &["kurvanti"]);
}

#[test]
fn sk_2548() {
    let kf = &d("qukf\\Y", Tanadi);
    assert_has_vas(&[], kf, Lat, &["kurvaH"]);
    assert_has_mas(&[], kf, Lat, &["kurmaH"]);
    assert_has_sip(&[], kf, Lit, &["cakarTa"]);
    assert_has_vas(&[], kf, Lit, &["cakfva"]);
    assert_has_thaas(&[], kf, Lit, &["cakfze"]);
    assert_has_tip(&[], kf, Lut, &["kartA"]);
    assert_has_tip(&[], kf, Lrt, &["karizyati"]);
}

#[test]
fn sk_2549() {
    let kf = &d("qukf\\Y", Tanadi);
    assert_has_tip(&[], kf, VidhiLin, &["kuryAt"]);
    assert_has_tip(&[], kf, AshirLin, &["kriyAt"]);
    assert_has_ta(&[], kf, AshirLin, &["kfzIzwa"]);
    assert_has_tip(&[], kf, Lun, &["akArzIt"]);
    assert_has_ta(&[], kf, Lun, &["akfta"]);
    assert_has_thaas(&[], kf, Lun, &["akfTAH"]);
}

#[test]
fn skip_sk_2550() {}

#[test]
fn sk_2551() {
    let kf = &d("qukf\\Y", Tanadi);
    assert_has_tip(&["sam"], kf, Lat, &["saMskaroti", "saNkaroti"]);
    assert_has_jhi(&["sam"], kf, Lat, &["saMskurvanti", "saNkurvanti"]);
    assert_has_tip(&["pari"], kf, Lat, &["parizkaroti", "parikaroti"]);
    assert_has_tip(
        &["pari"],
        kf,
        Lun,
        &["paryazkArzIt", "paryaskArzIt", "paryakArzIt"],
    );
}

#[test]
fn sk_2552() {
    let kf = &d("qukf\\Y", Tanadi);
    assert_has_krdanta(&["upa"], kf, K::kta, &["upakfta", "upaskfta"]);
}

#[test]
fn sk_2553() {
    let kf = &d("qukf\\Y", Tanadi);
    assert_has_tip(&["sam"], kf, Lit, &["saYcaskAra", "saYcakAra"]);
    assert_has_tas(&["sam"], kf, Lit, &["saYcaskaratuH", "saYcakratuH"]);
    assert_has_sip(&["sam"], kf, Lit, &["saYcaskariTa", "saYcakarTa"]);
    assert_has_vas(&["sam"], kf, Lit, &["saYcaskariva", "saYcakfva"]);
    assert_has_ta(&["sam"], kf, AshirLin, &["saMskfzIzwa", "saNkfzIzwa"]);
    assert_has_ta(&["sam"], kf, Lun, &["samaskfta", "samakfta"]);
    assert_has_aataam(&["sam"], kf, Lun, &["samaskfzAtAm", "samakfzAtAm"]);
}
