use vidyut_prakriya::args::*;
use vidyut_prakriya::Vyakarana;

fn derive(upadesha: &str, gana: &str, prayoga: Prayoga) -> Vec<String> {
    let v = Vyakarana::new();
    let dhatu = Dhatu::builder()
        .upadesha(upadesha)
        .gana(gana.parse().expect("ok"))
        .build()
        .unwrap();

    let args = Tinanta::builder()
        .dhatu(dhatu)
        .prayoga(prayoga)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();

    let prakriyas = v.derive_tinantas(&args);
    prakriyas.iter().map(|p| p.text()).collect()
}

fn run_test_cases(cases: &[(&str, u8, &str)], prayoga: Prayoga) {
    let mut num_failed = 0;
    for (dhatu, gana, expected) in cases {
        let mut expected: Vec<_> = expected.split('|').collect();
        expected.sort();

        let mut actual: Vec<_> = derive(dhatu, &gana.to_string(), prayoga);
        actual.sort();
        actual.dedup();

        if actual != expected {
            println!("Dhatu    : {dhatu} ({gana})");
            println!("Expected : {expected:?}");
            println!("Actual   : {actual:?}");
            println!();
            num_failed += 1;
        }
    }
    if num_failed > 0 {
        let num_total = cases.len();
        let num_passed = num_total - num_failed;
        println!("{num_passed} / {num_total} tests passed.")
    }
    assert_eq!(num_failed, 0);
}

#[test]
fn karmani() {
    let cases = &[
        // Kale 594
        ("GrA\\", 1, "GrAyate"),
        ("jyA\\", 9, "jIyate"),
        ("dA\\R", 1, "dIyate"),
        ("qudA\\Y", 3, "dIyate"),
        ("dA\\p", 2, "dAyate"),
        ("quDA\\Y", 1, "DIyate"),
        ("pA\\", 1, "pIyate"),
        ("pA\\", 2, "pAyate"),
        ("mA\\", 2, "mIyate"),
        ("De\\w", 1, "DIyate"),
        ("ve\\Y", 1, "Uyate"),
        ("vye\\Y", 1, "vIyate"),
        ("hve\\Y", 1, "hUyate"),
        ("gE\\", 1, "gIyate"),
        ("pE\\", 1, "pAyate"),
        ("do\\", 4, "dIyate"),
        ("zo\\", 4, "sIyate"),
        ("o~hA\\k", 3, "hIyate"),
        ("o~hA\\N", 3, "hAyate"),
        ("ci\\Y", 5, "cIyate"),
        ("wuo~Svi", 1, "SUyate"),
        // TODO: debug this?
        // ("qumi\\Y", 5, "mIyate"),
        ("mI\\N", 4, "mIyate"),
        ("SIN", 2, "Sayyate"),
        ("UrRuY", 2, "UrRUyate"),
        ("f\\", 3, "aryate"),
        ("qukf\\Y", 8, "kriyate"),
        ("jAgf", 2, "jAgaryate"),
        ("smf", 1, "smaryate"),
        ("vFY", 9, "vUryate"),
        ("stf\\Y", 5, "staryate"),
        ("kF", 6, "kIryate"),
        ("stF\\Y", 9, "stIryate"),
        ("de\\N", 1, "dIyate"),
        ("tanu~^", 8, "tAyate|tanyate"),
        ("pana~\\", 1, "panAyyate|panyate"),
        // Kale has "gopyate" here, but I think it occurs only in curAdi.
        ("gupU~", 1, "gupyate|gopAyyate"),
        ("quva\\pa~^", 1, "upyate"),
        ("Yizva\\pa~", 2, "supyate"),
        ("kamu~\\", 1, "kamyate|kAmyate"),
        ("cura~", 10, "coryate"),
        ("divu~", 1, "dIvyate"),
        ("vaSa~", 2, "uSyate"),
        ("va\\ca~", 1, "ucyate"),
        ("o~vrascU~", 6, "vfScyate"),
        ("vyaca~", 6, "vicyate"),
        ("pra\\Ca~", 6, "pfcCyate"),
        ("viCa~", 6, "vicCyate|vicCAyyate"),
        ("Bra\\sja~^", 6, "Bfjjyate"),
        ("ya\\ja~^", 1, "ijyate"),
        ("paRa~\\", 1, "paRAyyate|paRyate"),
        // Kale has "ftIyate," likely a typo
        ("fti", 1, "ftyate|ftIyyate"),
        ("a\\da~", 2, "adyate"),
        ("vada~", 1, "udyate"),
        ("YiinDI~\\", 7, "iDyate"),
        ("vya\\Da~", 4, "viDyate"),
        ("ba\\nDa~", 9, "baDyate"),
        ("zaRu~^", 8, "sAyate|sanyate"),
        ("janI~\\", 4, "jAyate|janyate"),
        ("va\\sa~", 1, "uzyate"),
        ("vasa~\\", 2, "vasyate"),
        ("asa~", 2, "BUyate"),
        ("SAsu~", 2, "Sizyate"),
        ("sransu~\\", 1, "srasyate"),
        ("vadi~\\", 1, "vandyate"),
        ("va\\ha~^", 1, "uhyate"),
        ("graha~^", 1, "gfhyate"),
        // TODO: samuhyate
    ];

    run_test_cases(cases, Prayoga::Karmani);
}
