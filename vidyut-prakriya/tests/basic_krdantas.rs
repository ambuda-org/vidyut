/*!
Various test cases for krdantas.

Tests marked with *Kale* are from M. R. Kale's *A Higher Sanskrit Grammar*.
*/
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
fn test_krdanta(cases: &Vec<(&'static str, u8, &'static str)>, krt: Krt) {
    for (dhatu, gana, expected) in cases {
        let mut expected: Vec<_> = expected.split('|').collect();
        expected.sort();
        expected.dedup();

        let mut actual = create_krdanta(dhatu, *gana, krt);
        actual.sort();
        actual.dedup();

        assert_eq!(actual, expected);
    }
}

#[test]
fn kta() {
    // TODO: expand "yasyeti ca"
    let cases = vec![
        // Kale 679.
        ("zRA\\", 2, "snAta"),
        ("ji\\", 1, "jita"),
        ("RI\\Y", 1, "nIta"),
        ("Sru\\", 1, "Sruta"),
        ("BU", 1, "BUta"),
        ("hf\\", 3, "hfta"),
        ("hf\\", 3, "hfta"),
        ("tya\\ja~", 1, "tyakta"),
        // Kale 681 (a).
        ("SIN", 2, "Sayita"),
        // The four below are allowed by 7.2.17.
        // ("YizvidA~\\", 1, "svedita"),
        // ("YimidA~", 1, "medita"),
        // ("YikzvidA~", 1, "kzvedita"),
        // ("YiDfzA~~", 5, "Darzita"),
        // Kale 681 (b).
        ("muda~\\", 1, "mudita"),
        // Kale 684.
        ("pA\\", 2, "pAta"),
        ("SriY", 1, "Srita"),
        // nIta and Sruta are above.
        ("Bf\\Y", 1, "Bfta"),
        ("qukf\\Y", 8, "kfta"),
        // ("UrRuY", 1, "UrRuta"),
        ("ve\\Y", 1, "uta"),
        ("vye\\Y", 1, "vIta"),
        ("hve\\Y", 1, "hUta"),
        // tyakta is above.
        ("Bra\\sja~^", 1, "Bfzwa"),
        ("ya\\ja~^", 1, "izwa"),
        ("bu\\Da~\\", 1, "budDa"),
        ("vya\\Da~", 1, "vidDa"),
        ("Yizva\\pa~", 1, "supta"),
        ("qula\\Ba~\\z", 1, "labDa"),
        ("ba\\nDa~", 1, "badDa"),
        ("df\\Si~r", 1, "dfzwa"),
        ("kru\\Sa~", 1, "kruzwa"),
        ("va\\ca~", 2, "ukta"),
        ("guhU~^", 1, "gUQa"),
        ("mfjU~", 2, "mfzwa"),
        ("zi\\Du~", 4, "sidDa"),
        // ("tf\\pa~", 4, "tfpta"),
        // ("Ra\\Sa~", 1, "nazwa"),
        ("vfDu~\\", 1, "vfdDa"),
        ("vftu~\\", 1, "vftta"),
        ("Sa\\kx~", 5, "Sakta"),
        ("zi\\ca~", 6, "sikta"),
        ("pra\\Ca~", 6, "pfzwa"),
        ("da\\nSa~", 1, "dazwa"),
        // Kale erroneously has "dizwa."
        ("dvi\\za~^", 2, "dvizwa"),
        ("SAsu~", 2, "Sizwa"),
        ("da\\ha~", 1, "dagDa"),
        ("va\\ha~^", 1, "UQa"),
        // ("zaha~\\", 1, "soQa"),
        ("Dvansu~\\", 6, "Dvasta"),
        ("li\\ha~^", 2, "lIQa"),
        // ("mu\\ha~", 6, "mugDa|mUQa"),
        ("Ra\\ha~^", 6, "nadDa"),
        ("sransu~\\", 6, "srasta"),
        // Kale 684 (a)
        // Kale 701
        ("qupa\\ca~^z", 1, "pakva"),
        ("Su\\za~", 4, "Suzka"),
    ];

    test_krdanta(&cases, Krt::kta);
}

// todo: 7.4.40+ (ti kiti)
#[test]
fn ktva() {
    let cases = vec![
        // Basic
        ("BU", 1, "BUtvA"),
        ("eDa~\\", 1, "eDitvA"),
        ("qukf\\Y", 8, "kftvA"),
        ("cura~", 10, "corayitvA"),
        // Exceptions
        ("va\\ha~^", 1, "UQvA"),
        // ("zWA\\", 1, "sTitvA"),
        // ("a\\da~", 2, "jagDvA"),
        // Samprasarana
        ("va\\ca~", 2, "uktvA"),
        // 1.2.7
        ("mfqa~", 9, "mfqitvA"),
        ("mfda~", 9, "mfditvA"),
        ("guDa~", 9, "guDitvA"),
        ("kuza~", 9, "kuzitvA"),
        ("kliSU~", 9, "kliSitvA|klizwvA"),
        ("vada~", 1, "uditvA"),
        ("va\\sa~", 1, "uzitvA"),
    ];

    test_krdanta(&cases, Krt::ktvA);
}

#[test]
fn kanac() {
    let cases = vec![
        // Kale 675 (a).
        ("RI\\Y", 1, "ninyAna"),
        ("qudA\\Y", 3, "dadAna"),
        ("qupa\\ca~^z", 1, "pecAna"),
        ("ya\\ja~^", 1, "IjAna"),
        ("qukf\\Y", 8, "cakrARa"),
        ("va\\ca~", 2, "UcAna"),
        ("zwu\\Y", 2, "tuzwuvAna"),
        ("Sru\\", 1, "SuSruvARa"),
    ];

    test_krdanta(&cases, Krt::kAnac);
}

#[test]
fn kvasu() {
    let cases = vec![
        // Kale 675.
        ("i\\R", 2, "Iyivas"),
        ("f\\", 3, "Arivas"),
        ("RI\\Y", 1, "ninIvas"),
        // ("qupa\\ca~^z", 1, "pecivas"),
        ("va\\ca~", 2, "Ucivas"),
        ("ya\\ja~^", 1, "Ijivas"),
        // Kale erroneously has "baBaYjvas."
        // ("Ba\\njo~", 7, "Bejivas"),
        ("zwu\\Y", 2, "tuzwuvas"),
        ("qukf\\Y", 8, "cakfvas"),
        ("Bi\\di~^r", 2, "biBidvas"),
        ("Gasx~", 1, "jakzivas"),
        ("df\\Si~r", 1, "dadfSivas|dadfSvas"),
        // ("vida~", 2, "vividvas|vividivas"),
        ("vi\\Sa~", 6, "viviSivas|viviSvas"),
        // ("janI~\\", 2, "jajanvas"),
        ("Kanu~^", 1, "caKanvas"),
        ("ga\\mx~", 1, "jagmivas|jaganvas"),
        ("ha\\na~", 2, "jaGnivas|jaGanvas"),
        // F, tF, jF
        // ("tF", 1, "titIrvas"),
    ];

    test_krdanta(&cases, Krt::kvasu);
}

#[test]
fn nvul() {
    let cases = vec![
        // Basic
        ("BU", 1, "BAvaka"),
        ("qukf\\Y", 8, "kAraka"),
    ];

    test_krdanta(&cases, Krt::Rvul);
}

#[test]
fn shatr() {
    // Kale 667.
    let cases = vec![
        ("BU", 1, "Bavat"),
        ("zWA\\", 1, "tizWat"),
        ("dvi\\za~^", 2, "dvizat"),
        ("a\\da~", 2, "adat"),
        ("yA\\", 2, "yAt"),
        ("hu\\", 3, "juhvat"),
        ("divu~", 4, "dIvyat"),
        ("zu\\Y", 5, "sunvat"),
        ("tu\\da~^", 6, "tudat"),
        ("ru\\Di~^r", 7, "runDat"),
        ("qukf\\Y", 8, "kurvat"),
        ("tanu~^", 8, "tanvat"),
        ("qukrI\\Y", 9, "krIRat"),
        ("muza~", 9, "muzRat"),
        ("cura~", 10, "corayat"),
    ];

    test_krdanta(&cases, Krt::Satf);
}

#[test]
fn shanac() {
    // kale 666.
    let cases = vec![
        // Basic cases
        ("eDa~\\", 1, "eDamAna"),
        ("vadi~\\", 1, "vandamAna"),
        ("SIN", 2, "SayAna"),
        ("dvi\\za~^", 2, "dvizARa"),
        ("quDA\\Y", 3, "daDAna"),
        ("hu\\", 3, "juhvAna"),
        ("divu~", 4, "dIvyamAna"),
        ("zu\\Y", 5, "sunvAna"),
        ("tu\\da~^", 6, "tudamAna"),
        ("ru\\Di~^r", 7, "runDAna"),
        ("qukf\\Y", 8, "kurvARa"),
        ("tanu~^", 8, "tanvAna"),
        ("qukrI\\Y", 9, "krIRAna"),
        ("cura~", 10, "corayamARa"),
        // Special cases
        ("Asa~\\", 2, "AsIna"),
    ];

    test_krdanta(&cases, Krt::SAnac);
}

#[test]
fn lyuw() {
    let cases = vec![
        // Basic
        ("BU", 1, "Bavana"),
        ("qukf\\Y", 8, "karaRa"),
    ];

    test_krdanta(&cases, Krt::lyuw);
}
