extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_7_4_1() {
    let d = |u, gana| Dhatu::new(u, gana).with_sanadi(&[Sanadi::Nic]);
    assert_has_lun_p(&[], &d("qukf\\Y", Gana::Tanadi), &["acIkarat"]);
    assert_has_lun_p(&[], &d("hf\\Y", Gana::Bhvadi), &["ajIharat"]);
    assert_has_lun_p(&[], &d("lUY", Gana::Kryadi), &["alIlavat"]);
    assert_has_lun_p(&[], &d("pUY", Gana::Kryadi), &["apIpavat"]);

    // TODO: many more.
}

#[test]
fn sutra_7_4_23() {
    let uh = Dhatu::new("Uha~\\", Gana::Bhvadi);
    assert_has_lat_karmani(&["sam"], &uh, &["samuhyate"]);
    assert_has_krdanta(&["sam"], &uh, Krt::ktvA, &["samuhya"]);
    assert_has_lat_karmani(&["aBi"], &uh, &["aByuhyate"]);
    assert_has_krdanta(&["aBi"], &uh, Krt::ktvA, &["aByuhya"]);

    assert_has_lat_karmani(&[], &uh, &["Uhyate"]);

    let ih = Dhatu::new("Iha~\\", Gana::Bhvadi);
    assert_has_lat_karmani(&["sam"], &ih, &["samIhyate"]);
    assert_has_krdanta(&["sam"], &uh, Krt::kta, &["samUhita"]);

    assert_has_lat_karmani(&["AN"], &uh, &["ohyate"]);
    assert_has_lat_karmani(&["sam", "AN"], &uh, &["samohyate"]);
}

#[test]
fn sutra_7_4_24() {
    let i = Dhatu::new("i\\R", Gana::Adadi);
    assert_has_ashirlin(&["ud"], &i, &["udiyAt"]);
    assert_has_ashirlin(&["sam"], &i, &["samiyAt"]);
    assert_has_ashirlin(&["anu"], &i, &["anviyAt"]);

    assert_has_ashirlin(&[], &i, &["IyAt"]);

    assert_has_ashirlin(&["AN"], &i, &["eyAt"]);
    assert_has_ashirlin(&["sam", "AN"], &i, &["sameyAt"]);
}

#[test]
fn sutra_7_4_28() {
    let d = Dhatu::new;

    let kf = Dhatu::new("qukf\\Y", Gana::Tanadi);
    let hf = Dhatu::new("hf\\Y", Gana::Bhvadi);
    let bhf = Dhatu::new("quBf\\Y", Gana::Juhotyadi);

    assert_has_lat_karmani(&["AN"], &d("df\\N", Gana::Tudadi), &["Adriyate"]);
    assert_has_lat_karmani(&["AN"], &d("Df\\Y", Gana::Bhvadi), &["ADriyate"]);
    assert_has_lat_karmani(&[], &kf, &["kriyate"]);
    assert_has_lat_karmani(&[], &hf, &["hriyate"]);
    assert_has_ashirlin_p(&[], &kf, &["kriyAt"]);
    assert_has_ashirlin_p(&[], &hf, &["hriyAt"]);
    assert_has_vidhilin_p(&[], &bhf, &["biBfyAt"]);
    assert_has_ashirlin_a(&[], &kf, &["kfzIzwa"]);
    assert_has_ashirlin_a(&[], &hf, &["hfzIzwa"]);
}

#[test]
fn sutra_7_4_29() {
    let f = Dhatu::new("f\\", Gana::Bhvadi);
    assert_has_lat_karmani(&[], &f, &["aryate"]);
    assert_has_ashirlin_p(&[], &f, &["aryAt"]);

    let smf = Dhatu::new("smf", Gana::Bhvadi);
    assert_has_lat_karmani(&[], &smf, &["smaryate"]);
    assert_has_ashirlin_p(&[], &smf, &["smaryAt"]);

    let kf = Dhatu::new("qukf\\Y", Gana::Tanadi);
    assert_has_lat_karmani(&["sam"], &kf, &["saMskriyate", "saNkriyate"]);
    assert_has_ashirlin_p(&["sam"], &kf, &["saMskriyAt", "saNkriyAt"]);

    // TODO: also derives svarizIzwa and svArizIzwa, which I need to check.
    /*
    let svf = Dhatu::new("svf", Gana::Bhvadi);
    assert_has_ashirlin_karmani(&[], &svf, &["svfzIzwa"]);
    */
}
