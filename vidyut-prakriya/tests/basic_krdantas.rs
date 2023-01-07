use compact_str::CompactString;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

/// Creates a krdanta with the given args.
fn create_krdanta(dhatu: &str, gana: u8, krt: Krt) -> Vec<CompactString> {
    let a = Ashtadhyayi::new();
    let dhatu = Dhatu::new(dhatu, Gana::from_int(gana).unwrap());
    let args = KrdantaArgs::builder().krt(krt).build().unwrap();

    let prakriyas = a.derive_krdantas(&dhatu, &args);
    prakriyas.iter().map(|p| p.text()).collect()
}

/// Tests all of the given test cases against the given krt-pratyaya.
fn test_krdanta(cases: &Vec<(&'static str, u8, Vec<&'static str>)>, krt: Krt) {
    for (dhatu, gana, expected) in cases {
        let mut expected = expected.to_vec();
        let mut actual = create_krdanta(dhatu, *gana, krt);

        expected.sort();
        actual.sort();
        assert_eq!(actual, expected);
    }
}

// todo: 7.4.40+ (ti kiti)
#[test]
fn ktva() {
    let cases = vec![
        // Basic
        ("BU", 1, vec!["BUtvA"]),
        ("eDa~\\", 1, vec!["eDitvA"]),
        ("qukf\\Y", 8, vec!["kftvA"]),
        ("cura~", 10, vec!["corayitvA"]),
        // Exceptions
        ("va\\ha~^", 1, vec!["UQvA"]),
        // ("zWA\\", 1, vec!["sTitvA"]),
        // ("a\\da~", 2, vec!["jagDvA"]),
        // Samprasarana
        ("va\\ca~", 2, vec!["uktvA"]),
        // 1.2.7
        ("mfqa~", 9, vec!["mfqitvA"]),
        ("mfda~", 9, vec!["mfditvA"]),
        ("guDa~", 9, vec!["guDitvA"]),
        ("kuza~", 9, vec!["kuzitvA"]),
        ("kliSU~", 9, vec!["kliSitvA", "klizwvA"]),
        ("vada~", 1, vec!["uditvA"]),
        ("va\\sa~", 1, vec!["uzitvA"]),
    ];

    test_krdanta(&cases, Krt::ktvA);
}

#[test]
fn kta() {
    let cases = vec![
        // Basic
        ("BU", 1, vec!["BUta"]),
        ("eDa~\\", 1, vec!["eDita"]),
        ("qukf\\Y", 8, vec!["kfta"]),
        ("cura~", 10, vec!["corita"]),
        // 1.2.19
        ("SIN", 2, vec!["Sayita"]),
        // The four below are allowed by 7.2.17.
        // ("YizvidA~\\", 1, vec!["svedita"]),
        // ("YimidA~", 1, vec!["medita"]),
        // ("YikzvidA~", 1, vec!["kzvedita"]),
        // ("YiDfzA~~", 5, vec!["Darzita"]),
        // Irregular
        ("qupa\\ca~^z", 1, vec!["pakva"]),
        ("Su\\za~", 4, vec!["Suzka"]),
    ];

    test_krdanta(&cases, Krt::kta);
}

#[test]
fn lyuw() {
    let cases = vec![
        // Basic
        ("BU", 1, vec!["Bavana"]),
        ("qukf\\Y", 8, vec!["karaRa"]),
    ];

    test_krdanta(&cases, Krt::lyuw);
}

#[test]
fn nvul() {
    let cases = vec![
        // Basic
        ("BU", 1, vec!["BAvaka"]),
        ("qukf\\Y", 8, vec!["kAraka"]),
    ];

    test_krdanta(&cases, Krt::Rvul);
}
