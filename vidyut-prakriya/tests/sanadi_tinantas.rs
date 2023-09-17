/*!
Various test cases for sanAdi-dhatus, particulary the dhatus ending in nic, san, or yan.

Test cases marked with *Kale* are from M. R. Kale's *A Higher Sanskrit Grammar*.
*/
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

fn create_sanadyanta(upadesha: &str, gana: &str, sanadi: Sanadi) -> Vec<String> {
    let a = Ashtadhyayi::new();
    let gana = gana.parse().expect("ok");
    let dhatu = Dhatu::builder()
        .upadesha(upadesha)
        .gana(gana)
        .sanadi(&[sanadi])
        .build()
        .unwrap();

    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();

    let prakriyas = a.derive_tinantas(&dhatu, &args);
    prakriyas.iter().map(|p| p.text()).collect()
}

fn run_sanadi_test_cases(cases: &[(&str, u8, &str)], sanadi: Sanadi) {
    let mut num_failed = 0;
    for (dhatu, gana, expected) in cases {
        let mut expected: Vec<_> = expected.split('|').collect();
        expected.sort();

        let mut actual: Vec<_> = create_sanadyanta(dhatu, &gana.to_string(), sanadi);
        if sanadi == Sanadi::Nic {
            // All Nijantas are ubhayapadI, so to simplify the tests, focus on just parasmaipada.
            actual.retain(|x| x.ends_with("ti"));
        }
        actual.sort();
        actual.dedup();

        if actual != expected {
            println!("Dhatu:    {dhatu} ({gana})");
            println!("Expected: {expected:?}");
            println!("Actual  : {actual:?}");
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

#[ignore]
#[test]
fn san_tinantas() {
    // Kale 613
    let cases = &[
        ("paWa~", 1, "pipaWizati"),
        // Kala has only parasmaipada, but the dhatu is Yit so include both.
        ("zi\\Y", 1, "sisIzati|sisIzate"),
        ("zi\\ca~^", 1, "sisikzati|sisikzate"),
        ("zmi\\N", 1, "sismayizate"),
        ("zUN", 2, "susUzate"),
        ("zWA\\", 1, "tizWAsati|tizWAsate"),
        ("Ru", 2, "nunUzati"),
        ("BU", 1, "buBUzati"),
        // Irregular
        ("a\\da~", 2, "jiGatsati"),
        ("A\\px~", 5, "Ipsati"),
        ("i\\R", 1, "jigamizati"),
        // ("BU", 1, "aDijigAMsate"),
        // ("BU", 1, "pratIzizati"),
        ("izu~", 6, "ezizizati"),
        ("Uza", 1, "Uzizati"),
        (
            "UrRuY",
            1,
            "UrRunUzati|UrRunUzate|UrRunuvizati|UrRunuvizate|UrRunavizati|UrRunavizate",
        ),
        ("f\\", 3, "aririzati"),
        ("fDu~", 4, "Irtsati|ardiDizati"),
        ("ga\\mx~", 1, "jigamizati"),
        // ("BU", 1, "saMjigAmsate"),
        ("gF", 6, "jigarizati|jigalizati"),
        // Ubhayayadi per Yit
        ("ci\\Y", 5, "cicIzati|cicIzate|cikIzati|cikIzate"),
        ("ji\\", 1, "jigIzati"),
        ("jYA\\", 9, "jYIpsati"),
        // ("BU", 1, "jijJapayizati"),
        ("tanu~^", 1, "titaMsati|titAMsati|titanizati"),
        ("tfnhU~", 1, "titfkzati|titfMhizati"),
        ("danBu~", 1, "Dipsati|DIpsati|didamBizati"),
        ("BU", 1, "didaridrAsati|didaridrizati"),
        ("qudA\\Y", 3, "ditsati"),
        ("deN", 1, "ditsate"),
        ("do", 4, "ditsati"),
        ("divu~", 4, "dudyUzati|didevizati"),
        ("quDA\\Y", 3, "Ditsati"),
        ("De\\w", 1, "Ditsati"),
        ("Ra\\Sa~", 1, "ninaNkzyati|ninaSizati"),
        ("patx~", 1, "pitsati|pipatizati"),
        ("pa\\da~\\", 4, "pitsate"),
        ("pUN", 1, "pipavizate"),
        (
            "Bra\\sja~^",
            6,
            // Kale is missing atmanepada
            "biBarkzati|biBarkzate|biBrakzati|biBrakzate|biBarjizati|biBarjizate|biBrajjizati|biBrajjizate",
        ),
        ("wuma\\sjo~", 6, "mimaNkzati|mimajjizati"),
        ("mA\\", 2, "mitsati"),
        ("qumi\\Y", 1, "mitsati"),
        ("mI\\N", 4, "mitsate"),
        ("me\\N", 1, "mitsate"),
        // Also include mumuksate -- see Kashika on 7.4.57.
        ("mu\\cx~^", 6, "mokzate|mumukzati|mumukzate"),
        ("mfjU~", 2, "mimfkzati|mimArjizati"),
        ("yu", 2, "yuyUzati|yiyavizati"),
        ("ra\\Ba~\\", 1, "ripsate"),
        ("rA\\Da~", 4, "ritsati|rirAtsati"),
        ("qula\\Ba~\\z", 1, "lipsate"),
        ("Sa\\kx~", 1, "Sikzati"),
        ("vfDu~\\", 1, "vivftsati|vivarDizate"),
        ("syandU~\\", 1, "sisyantsati|sisyandizate|sisyantsate"),
        ("kfpU~\\", 1, "cikxpsati|cikalpizate|cikxpsate"),
    ];

    run_sanadi_test_cases(cases, Sanadi::San);
}
