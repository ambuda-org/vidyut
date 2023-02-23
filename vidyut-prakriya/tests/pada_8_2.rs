extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_8_2_30() {
    let pac = Dhatu::new("qupa\\ca~^z", Gana::Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::tfc, &["paktf"]);
    assert_has_krdanta(&[], &pac, Krt::tumun, &["paktum"]);
    assert_has_krdanta(&[], &pac, Krt::tavya, &["paktavya"]);

    let vac = Dhatu::new("va\\ca~", Gana::Adadi);
    assert_has_krdanta(&[], &vac, Krt::tfc, &["vaktf"]);
    assert_has_krdanta(&[], &vac, Krt::tumun, &["vaktum"]);
    assert_has_krdanta(&[], &vac, Krt::tavya, &["vaktavya"]);
    // TODO: add the rest of the examples
}

#[test]
fn sutra_8_2_31() {
    let sah = Dhatu::new("zaha~\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &sah, Krt::tfc, &["soQf", "sahitf"]);
    assert_has_krdanta(&[], &sah, Krt::tumun, &["soQum", "sahitum"]);
    assert_has_krdanta(&[], &sah, Krt::tavya, &["soQavya", "sahitavya"]);

    let vah = Dhatu::new("va\\ha~^", Gana::Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::tfc, &["voQf"]);
    assert_has_krdanta(&[], &vah, Krt::tumun, &["voQum"]);
    assert_has_krdanta(&[], &vah, Krt::tavya, &["voQavya"]);
    // TODO: jalAzAt, dityavAw
}

#[test]
fn sutra_8_2_32() {
    let dah = Dhatu::new("da\\ha~", Gana::Bhvadi);
    assert_has_krdanta(&[], &dah, Krt::tfc, &["dagDf"]);
    assert_has_krdanta(&[], &dah, Krt::tumun, &["dagDum"]);
    assert_has_krdanta(&[], &dah, Krt::tavya, &["dagDavya"]);

    let duh = Dhatu::new("du\\ha~^", Gana::Bhvadi);
    assert_has_krdanta(&[], &duh, Krt::tfc, &["dogDf"]);
    assert_has_krdanta(&[], &duh, Krt::tumun, &["dogDum"]);
    assert_has_krdanta(&[], &duh, Krt::tavya, &["dogDavya"]);

    let lih = Dhatu::new("li\\ha~^", Gana::Bhvadi);
    assert_has_krdanta(&[], &lih, Krt::tfc, &["leQf"]);
    assert_has_krdanta(&[], &lih, Krt::tumun, &["leQum"]);
    assert_has_krdanta(&[], &lih, Krt::tavya, &["leQavya"]);
    // TODO: kAzWaDak, etc.
}

#[test]
fn sutra_8_2_51() {
    let sus = Dhatu::new("Su\\za~", Gana::Divadi);
    assert_has_krdanta(&[], &sus, Krt::kta, &["Suzka"]);
    assert_has_krdanta(&[], &sus, Krt::ktavatu, &["Suzkavat"]);
}

#[test]
fn sutra_8_2_52() {
    let pac = Dhatu::new("qupa\\ca~^z", Gana::Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::kta, &["pakva"]);
    assert_has_krdanta(&[], &pac, Krt::ktavatu, &["pakvavat"]);
}

#[test]
fn sutra_8_2_53() {
    let kzai = Dhatu::new("kzE\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &kzai, Krt::kta, &["kzAma"]);
    assert_has_krdanta(&[], &kzai, Krt::ktavatu, &["kzAmavat"]);
}

#[ignore]
#[test]
fn sutra_8_2_54() {
    let styai = Dhatu::new("styE\\", Gana::Bhvadi);
    assert_has_krdanta(&["pra"], &styai, Krt::kta, &["prastIma", "prastIta"]);
    assert_has_krdanta(&[], &styai, Krt::ktavatu, &["prastImavat", "prastItavat"]);
}

#[test]
fn sutra_8_2_55() {
    let phal = Dhatu::new("YiPalA~", Gana::Bhvadi);
    assert_has_krdanta(&[], &phal, Krt::kta, &["Pulla"]);
    assert_has_krdanta(&[], &phal, Krt::ktavatu, &["Pullavat"]);

    let kzib = Dhatu::new("kzIbf~\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &kzib, Krt::kta, &["kzIba"]);

    let kfs = Dhatu::new("kfSa~", Gana::Divadi);
    assert_has_krdanta(&[], &kfs, Krt::kta, &["kfSa"]);

    let lagh = Dhatu::new("lAGf~\\", Gana::Bhvadi);
    assert_has_krdanta(&["ud"], &lagh, Krt::kta, &["ullAGa"]);

    // TODO: allow praPulta
    // assert_has_krdanta(&["pra"], &phal, Krt::kta, &["praPulta"]);
    assert_has_krdanta(&["pra"], &kzib, Krt::kta, &["prakzIbita"]);
    assert_has_krdanta(&["pra"], &kfs, Krt::kta, &["prakfSita"]);
    assert_has_krdanta(&["pra", "ud"], &lagh, Krt::kta, &["prollAGita"]);
}
