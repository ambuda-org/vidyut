/*!
Various test cases for sanAdi-dhatus, particulary the dhatus ending in nic, san, or yan.

Test cases marked with *Kale* are from M. R. Kale's *A Higher Sanskrit Grammar*.
*/
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

fn create_sanadyanta(upadesha: &str, gana: u8, sanadi: Sanadi) -> Vec<String> {
    let a = Ashtadhyayi::new();
    let gana = Gana::from_int(gana).unwrap();
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

        let mut actual: Vec<_> = create_sanadyanta(dhatu, *gana, sanadi);
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
fn nic_tinantas() {
    let cases = &[
        // Kale 602
        ("buDa~", 1, "boDayati"),
        ("kzuBa~\\", 1, "kzoBayati"),
        ("gaRa", 1, "gaRayati"),
        ("RI\\Y", 1, "nAyayati"),
        ("qukf\\Y", 8, "kArayati"),
        ("kF", 6, "kArayati"),
        ("kFta~", 10, "kIrtayati"),
        // Kale 603 -- mittva for dhatus ending in m
        ("ga\\mx~", 1, "gamayati"),
        ("kramu~", 1, "kramayati"),
        // TODO: ghaTAdi
        // mittva for dhatus ending in m -- exceptions
        ("kamu~\\", 1, "kAmayati"),
        ("camu~", 1, "cAmayati"),
        ("ama~", 1, "Amayati"),
        ("Samo~", 1, "SAmayati|Samayati"),
        ("ya\\ma~", 1, "yAmayati|yamayati"),
        // mittva optional when no upasarga
        ("wuvama~", 1, "vamayati|vAmayati"),
        ("Ra\\ma~", 1, "namayati|nAmayati"),
        ("vana~", 1, "vanayati|vAnayati"),
        ("jvala~", 1, "jvalayati|jvAlayati"),
        ("hvala~", 1, "hvalayati|hvAlayati"),
        ("hmala~", 1, "hmalayati|hmAlayati"),
        // puk
        ("zWA\\", 1, "sTApayati"),
        ("f\\", 3, "arpayati"),
        ("hrI\\", 3, "hrepayati"),
        ("rI\\N", 4, "repayati"),
        ("rI\\", 9, "repayati"),
        ("vlI\\", 9, "vlepayati"),
        // Irregular
        ("knUyI~\\", 1, "knopayati"),
        ("knUY", 9, "knopayati"),
        ("kzmAyI~\\", 1, "kzmAyayati"),
        ("guhU~^", 1, "gUhayati"),
        ("ci\\Y", 5, "cApayati|cAyayati"),
        ("jAgf", 2, "jAgarayati"),
        ("du\\za~", 4, "dUzayati|duzayati|dozayati"),
        ("DUY", 2, "DUnayati"),
        ("prI\\N", 4, "prIRayati"),
        ("YiBI\\", 3, "BAyayati|BApayati"),
        ("Bra\\sja~^", 6, "Barjayati|Brajjayati"),
        ("mfjU~", 2, "mArjayati"),
        ("ra\\nja~^", 1, "raYjayati"),
        ("ru\\ha~", 2, "rohayati|ropayati"),
        ("vA\\", 2, "vApayati|vAjayati"),
        ("vI\\", 2, "vApayati|vAyayati"),
        ("Sa\\dx~", 6, "SAtayati|SAdayati"),
        ("zi\\Du~", 4, "sADayati|seDayati"),
        ("sPAyI~\\", 1, "sPAvayati"),
        ("sPura~", 6, "sPorayati|sPArayati"),
        ("ha\\na~", 2, "GAtayati"),
    ];

    run_sanadi_test_cases(cases, Sanadi::Nic);
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

#[ignore]
#[test]
fn yan_tinantas() {
    // Kale 613
    let cases = &[
        ("lupa~", 4, "lolupyate"),
        ("za\\dx~", 1, "sAsadyate"),
        ("cara~", 1, "caYcUryate"),
        ("jYA\\", 1, "jAjYAyate"),
        ("De\\w", 1, "deDIyate"),
        ("BU", 1, "boBUyate"),
        ("f", 1, "arAryate"),
        ("qukrI\\Y", 9, "cekrIyate"),
        ("pF", 9, "popUryate"),
        ("awa~", 1, "awAwyate"),
        ("aSU~\\", 5, "aSASyate"),
        ("vraja~", 1, "vAvrajyate"),
        ("QOkf~\\", 1, "qoQOkyate"),
        ("vyaca~", 1, "vevicyate"),
        ("Yizva\\pa~", 1, "sozupyate"),
        ("SAsu~", 1, "SeSizyate"),
        ("o~pyAyI~\\", 1, "pepIyate"),
        ("GrA\\", 1, "jeGrIyate"),
        ("DmA\\", 1, "deDmIyate"),
        ("ya\\ma~", 1, "yaMyamyate"),
        // Also check jAjAyate per 6.4.43
        ("janI~\\", 4, "jaYjanyate|jAjAyate"),
        ("YiPalA~", 1, "pamPulyate"),
        ("da\\ha~", 1, "dandahyate"),
        ("japa~", 1, "jaYjapyate"),
        // Kale hs "vacIvaYcyate"
        ("vancu~", 1, "vanIvacyate"),
        ("sransu~\\", 1, "sanIsrasyate"),
        ("Dvansu~\\", 1, "danIDvasyate"),
        ("Bransu~\\", 1, "banIBrasyate"),
        // Kale has "kanIkasyate"
        ("kasa~", 1, "canIkasyate"),
        ("patx~", 1, "panIpatyate"),
        // Kale has "kanIskadyate"
        ("ska\\ndi~r", 1, "canIskadyate"),
        // Insertion of rI
        ("vftu~\\", 1, "varIvftyate"),
        ("pra\\Ca~", 6, "parIpfcCyate"),
        ("nftI~", 4, "narInftyate"),
        ("graha~^", 9, "jarIgfhyate"),
    ];

    run_sanadi_test_cases(cases, Sanadi::Yan);
}
