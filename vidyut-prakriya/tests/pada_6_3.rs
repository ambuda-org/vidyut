extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

/// Checks parasmaipada + the given lakara/purusha/vacana
fn assert_has_p_lpv(
    prefixes: &[&str],
    dhatu: &Dhatu,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    expected: &[&str],
) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(purusha)
        .vacana(vacana)
        .lakara(lakara)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

fn assert_has_lun_3d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_p_lpv(
        prefixes,
        dhatu,
        Lakara::Lun,
        Purusha::Prathama,
        Vacana::Dvi,
        expected,
    );
}
fn assert_has_lun_2d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_p_lpv(
        prefixes,
        dhatu,
        Lakara::Lun,
        Purusha::Madhyama,
        Vacana::Dvi,
        expected,
    );
}

#[test]
fn sutra_6_3_111() {
    let sah = Dhatu::new("zaha~\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &sah, Krt::tfc, &["soQf", "sahitf"]);
    assert_has_krdanta(&[], &sah, Krt::tumun, &["soQum", "sahitum"]);
    assert_has_krdanta(&[], &sah, Krt::tavya, &["soQavya", "sahitavya"]);

    let vah = Dhatu::new("va\\ha~^", Gana::Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::tfc, &["voQf"]);
    assert_has_krdanta(&[], &vah, Krt::tumun, &["voQum"]);
    assert_has_krdanta(&[], &vah, Krt::tavya, &["voQavya"]);

    assert_has_krdanta(&[], &vah, Krt::kta, &["UQa"]);
    assert_has_krdanta(&[], &vah, Krt::ktavatu, &["UQavat"]);

    assert_has_lun_3d(&["ud"], &vah, &["udavoQAm"]);
    assert_has_lun_2d(&["ud"], &vah, &["udavoQam"]);
}
