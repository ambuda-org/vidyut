extern crate vidyut_prakriya;

use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;
use vidyut_prakriya::Prakriya;

pub fn derive_tinantas(dhatu: &Dhatu, args: &TinantaArgs) -> Vec<Prakriya> {
    let a = Ashtadhyayi::new();
    let mut results = a.derive_tinantas(dhatu, args);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());

    results
        .into_iter()
        .filter(|p| !p.text().ends_with('d'))
        .collect()
}

pub fn derive_krdantas(dhatu: &Dhatu, krt: Krt) -> Vec<Prakriya> {
    let args = KrdantaArgs::builder().krt(krt).build().unwrap();

    let a = Ashtadhyayi::new();
    let mut results = a.derive_krdantas(dhatu, &args);
    results.sort_by_key(|p| p.text());

    results
        .into_iter()
        .filter(|p| !p.text().ends_with('d'))
        .collect()
}

pub fn derive_lakara(dhatu: &Dhatu, lakara: Lakara) -> Vec<Prakriya> {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(lakara)
        .build()
        .unwrap();

    derive_tinantas(dhatu, &args)
}

pub fn derive_parasmai(dhatu: &Dhatu, lakara: Lakara) -> Vec<Prakriya> {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(lakara)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();

    derive_tinantas(dhatu, &args)
}

pub fn derive_atmane(dhatu: &Dhatu, lakara: Lakara) -> Vec<Prakriya> {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(lakara)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    derive_tinantas(dhatu, &args)
}

pub fn derive_lat(dhatu: &Dhatu) -> Vec<Prakriya> {
    derive_lakara(dhatu, Lakara::Lat)
}

pub fn derive_lan(dhatu: &Dhatu) -> Vec<Prakriya> {
    derive_lakara(dhatu, Lakara::Lan)
}

pub fn derive_lat_p(dhatu: &Dhatu) -> Vec<Prakriya> {
    derive_parasmai(dhatu, Lakara::Lat)
}

pub fn derive_lan_p(dhatu: &Dhatu) -> Vec<Prakriya> {
    derive_parasmai(dhatu, Lakara::Lan)
}

pub fn derive_lit_p(dhatu: &Dhatu) -> Vec<Prakriya> {
    derive_parasmai(dhatu, Lakara::Lit)
}

pub fn derive_lrt_p(dhatu: &Dhatu) -> Vec<Prakriya> {
    derive_parasmai(dhatu, Lakara::Lrt)
}

pub fn assert_padas(prakriyas: Vec<Prakriya>, expected: &[&str]) {
    let actuals: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();

    if actuals.len() != expected.len() {
        for p in &prakriyas {
            for step in p.history() {
                println!("{} --> {}", step.rule(), step.result());
            }
            println!("{:?}", p.rule_choices());
            println!();
        }
    }

    assert_eq!(
        actuals.len(),
        expected.len(),
        "expected {expected:?}, saw {actuals:?}"
    );

    let mut expected = Vec::from(expected);
    expected.sort();
    expected.dedup();

    for (i, p) in prakriyas.iter().enumerate() {
        let actual = p.text();

        if actual != expected[i] {
            for step in p.history() {
                println!("{} --> {}", step.rule(), step.result());
            }
            println!();
        }
        assert_eq!(actual, expected[i], "{expected:?}, {actuals:?}");
    }
}

pub fn assert_has_lat(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(&dhatu.clone().with_prefixes(prefixes), Lakara::Lat);
    assert_padas(actual, expected);
}

pub fn assert_has_lat_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(&dhatu.clone().with_prefixes(prefixes), Lakara::Lat);
    assert_padas(actual, expected);
}

pub fn assert_has_lat_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_atmane(&dhatu.clone().with_prefixes(prefixes), Lakara::Lat);
    assert_padas(actual, expected);
}

pub fn assert_has_lat_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Karmani)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();

    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lit(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(&dhatu.clone().with_prefixes(prefixes), Lakara::Lit);
    assert_padas(actual, expected);
}

pub fn assert_has_lit_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lit_p(&dhatu.clone().with_prefixes(prefixes));
    assert_padas(actual, expected);
}

pub fn assert_has_lut(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(&dhatu.clone().with_prefixes(prefixes), Lakara::Lut);
    assert_padas(actual, expected);
}

pub fn assert_has_lut_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(&dhatu.clone().with_prefixes(prefixes), Lakara::Lut);
    assert_padas(actual, expected);
}

pub fn assert_has_lrt(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(&dhatu.clone().with_prefixes(prefixes), Lakara::Lrt);
    assert_padas(actual, expected);
}

pub fn assert_has_lrt_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lrt_p(&dhatu.clone().with_prefixes(prefixes));
    assert_padas(actual, expected);
}

pub fn assert_has_lot(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(&dhatu.clone().with_prefixes(prefixes), Lakara::Lot);
    assert_padas(actual, expected);
}

pub fn assert_has_lot_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(&dhatu.clone().with_prefixes(prefixes), Lakara::Lot);
    assert_padas(actual, expected);
}

pub fn assert_has_lan(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(&dhatu.clone().with_prefixes(prefixes), Lakara::Lan);
    assert_padas(actual, expected);
}

pub fn assert_has_lan_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lan_p(&dhatu.clone().with_prefixes(prefixes));
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(&dhatu.clone().with_prefixes(prefixes), Lakara::AshirLin);
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(&dhatu.clone().with_prefixes(prefixes), Lakara::AshirLin);
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_atmane(&dhatu.clone().with_prefixes(prefixes), Lakara::AshirLin);
    assert_padas(actual, expected);
}

pub fn assert_has_ashirlin_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Karmani)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::AshirLin)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_vidhilin_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(&dhatu.clone().with_prefixes(prefixes), Lakara::VidhiLin);
    assert_padas(actual, expected);
}

pub fn assert_has_lun(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(&dhatu.clone().with_prefixes(prefixes), Lakara::Lun);
    assert_padas(actual, expected);
}

pub fn assert_has_lun_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(&dhatu.clone().with_prefixes(prefixes), Lakara::Lun);
    assert_padas(actual, expected);
}

pub fn assert_has_lun_a(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_atmane(&dhatu.clone().with_prefixes(prefixes), Lakara::Lun);
    assert_padas(actual, expected);
}

pub fn assert_has_lun_karmani(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Karmani)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lun)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lrn(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_lakara(&dhatu.clone().with_prefixes(prefixes), Lakara::Lrn);
    assert_padas(actual, expected);
}

pub fn assert_has_lrn_p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let actual = derive_parasmai(&dhatu.clone().with_prefixes(prefixes), Lakara::Lrn);
    assert_padas(actual, expected);
}

pub fn assert_has_krdanta(prefixes: &[&str], dhatu: &Dhatu, krt: Krt, expected: &[&str]) {
    assert_padas(
        derive_krdantas(&dhatu.clone().with_prefixes(prefixes), krt),
        expected,
    );
}

pub fn assert_has_subantas(
    text: &str,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
    expected: &[&str],
) {
    let a = Ashtadhyayi::new();
    let pratipadika = Pratipadika::new(text);
    let args = SubantaArgs::builder()
        .linga(linga)
        .vacana(vacana)
        .vibhakti(vibhakti)
        .build()
        .unwrap();

    let mut results = a.derive_subantas(&pratipadika, &args);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    let actual: Vec<_> = results
        .into_iter()
        .filter(|p| !(p.text().ends_with('d') || p.text().ends_with('q')))
        .collect();
    assert_padas(actual, expected);
}
