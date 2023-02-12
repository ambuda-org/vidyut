use vidyut_prakriya::args::*;
use vidyut_prakriya::dhatupatha;
use vidyut_prakriya::Ashtadhyayi;

fn derive(upadesha: &str, gana: u8, prayoga: Prayoga) -> Vec<String> {
    let gana = Gana::from_int(gana).expect("valid");
    let a = Ashtadhyayi::new();
    let dhatu = Dhatu::builder()
        .upadesha(upadesha)
        .gana(gana)
        .build()
        .unwrap();

    let args = TinantaArgs::builder()
        .prayoga(prayoga)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();

    let prakriyas = a.derive_tinantas(&dhatu, &args);
    prakriyas.iter().map(|p| p.text()).collect()
}

fn assert_contains(words: &[String], needle: &str) {
    let found = words.iter().any(|w| w == needle);
    assert!(found, "Could not find item `{needle}` in {words:?}");
}

fn run_test_cases(cases: &[(&str, u8, &str)], prayoga: Prayoga) {
    let mut num_failed = 0;
    for (dhatu, gana, expected) in cases {
        let mut expected: Vec<_> = expected.split('|').collect();
        expected.sort();

        let mut actual: Vec<_> = derive(dhatu, *gana, prayoga);
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
fn with_upasargas() {
    let a = Ashtadhyayi::new();

    let lat = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();

    let lan = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lan)
        .build()
        .unwrap();

    let derive = |dhatu, args| {
        let prakriyas = a.derive_tinantas(dhatu, args);
        let results: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        results
    };

    // Default upasarga
    let adhi_i = dhatupatha::create_dhatu("i\\N", Gana::Adadi, 41).unwrap();
    let results = derive(&adhi_i, &lat);
    assert_contains(&results, "aDIte");

    // Optional upasarga
    let anu_bhu = Dhatu::builder()
        .upadesha("BU")
        .gana(Gana::Bhvadi)
        .prefixes(&["anu"])
        .build()
        .unwrap();

    let results = derive(&anu_bhu, &lat);
    assert_contains(&results, "anuBavati");

    let results = derive(&anu_bhu, &lan);
    assert_contains(&results, "anvaBavat");
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
        ("qumi\\Y", 5, "mIyate"),
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
