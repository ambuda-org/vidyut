/*!
Various test cases for krdantas.

Each of the test functions below tests a single krt-pratyaya. These tests are arranged in
the standard Sanskrit alphabetical order by the aupadeshika form of the krt pratyaya being tested.

Test cases marked with *Kale* are from M. R. Kale's *A Higher Sanskrit Grammar*.
*/
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

/// Creates a krdanta with the given args.
fn create_krdanta(dhatu: &str, gana: u8, krt: Krt) -> Vec<String> {
    let a = Ashtadhyayi::new();
    let dhatu = Dhatu::new(dhatu, Gana::from_int(gana).unwrap());
    let args = KrdantaArgs::builder().krt(krt).build().unwrap();

    let prakriyas = a.derive_krdantas(&dhatu, &args);
    prakriyas.iter().map(|p| p.text()).collect()
}

/// Tests all of the given test cases against the given krt-pratyaya.
fn test_krdanta(cases: &Vec<(&'static str, u8, &'static str)>, krt: Krt) {
    let mut num_errors = 0;
    for (dhatu, gana, expected) in cases {
        let mut expected: Vec<_> = expected.split('|').collect();
        expected.sort();
        expected.dedup();

        let mut actual = create_krdanta(dhatu, *gana, krt);
        actual.sort();
        actual.dedup();

        if actual != expected {
            println!("Expected : {expected:?}");
            println!("Actual   : {actual:?}");
            println!();
            num_errors += 1;
        }
    }
    if num_errors > 0 {
        let n = cases.len();
        let num_ok = n - num_errors;
        println!("{num_ok} / {n} tests passed.");
    }
    assert_eq!(num_errors, 0);
}

#[test]
fn aniya() {
    let cases = vec![
        // Kale 712
        ("qudA\\Y", 3, "dAnIya"),
        ("ci\\Y", 5, "cayanIya"),
        ("RI\\Y", 1, "nayanIya"),
        ("Sru\\", 1, "SravaRIya"),
        ("BU", 1, "BavanIya"),
        ("qukf\\Y", 8, "karaRIya"),
        ("mu\\cx~^", 6, "mocanIya"),
        ("mfjU~", 2, "mArjanIya"),
        ("sf\\ja~\\", 6, "sarjanIya"),
        ("Bra\\sja~^", 6, "BarjanIya|BrajjanIya"),
        ("Bi\\di~^r", 7, "BedanIya"),
        ("Ridi~", 1, "nindanIya"),
        ("guhU~^", 1, "gUhanIya"),
        // Kale 713
        ("kaTa", 10, "kaTanIya"),
        ("cura~", 10, "coraRIya"),
    ];

    test_krdanta(&cases, Krt::anIyar);
}

#[test]
fn kanac() {
    let cases = vec![
        // Kale 675 (a)
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

#[ignore]
#[test]
fn kta() {
    // TODO: expand "yasyeti ca"
    let cases = vec![
        // Kale 679 -- basic examples
        ("zRA\\", 2, "snAta"),
        ("ji\\", 1, "jita"),
        ("RI\\Y", 1, "nIta"),
        ("Sru\\", 1, "Sruta"),
        ("BU", 1, "BUta"),
        ("hf\\", 3, "hfta"),
        ("tya\\ja~", 1, "tyakta"),
        // Kale 681 (a) -- guna
        ("SIN", 2, "Sayita"),
        // The four below are allowed by 7.2.17.
        // ("YizvidA~\\", 1, "svedita"),
        // ("YimidA~", 1, "medita"),
        // ("YikzvidA~", 1, "kzvedita"),
        // ("YiDfzA~~", 5, "Darzita"),
        // Kale 681 (b).
        ("muda~\\", 1, "mudita"),
        // Kale 684 -- general rules
        ("pA\\", 2, "pAta"),
        ("SriY", 1, "Srita"),
        // nIta, Sruta, and BUta are above.
        ("qukf\\Y", 8, "kfta"),
        ("UrRuY", 1, "UrRuta"),
        ("ve\\Y", 1, "uta"),
        ("vye\\Y", 1, "vIta"),
        ("hve\\Y", 1, "hUta"),
        // "tyakta" is above.
        ("Bra\\sja~^", 1, "Bfzwa"),
        ("ya\\ja~^", 1, "izwa"),
        ("bu\\Da~\\", 1, "budDa"),
        ("vya\\Da~", 1, "vidDa"),
        ("Yizva\\pa~", 1, "supta"),
        ("qula\\Ba~\\z", 1, "labDa"),
        ("ba\\nDa~", 9, "badDa"),
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
        ("zaha~\\", 1, "soQa"),
        ("Dvansu~\\", 6, "Dvasta"),
        ("li\\ha~^", 2, "lIQa"),
        ("mu\\ha~", 6, "mugDa|mUQa"),
        ("Ra\\ha~^", 6, "nadDa"),
        ("sransu~\\", 6, "srasta"),
        // Kale 685 -- seT
        ("Saki~\\", 1, "SaNkita"),
        ("vada~", 1, "udita"),
        ("kaTa", 10, "kaTita"),
        ("praTa~\\", 1, "praTita"),
        ("eDa~\\", 1, "eDita"),
        ("kapi~\\", 1, "kampita"),
        ("muza~", 1, "muzita"),
        ("graha~^", 9, "gfhIta"),
        // Kale 685 -- seT exceptions
        ("YiinDI~\\", 7, "idDa"),
        ("fzI~", 6, "fzwa"),
        ("citI~", 1, "citta"),
        ("juzI~\\", 6, "juzwa"),
        ("trasI~", 4, "trasta"),
        ("dIpI~\\", 4, "dIpta"),
        ("madI~", 4, "matta"),
        ("yatI~\\", 1, "yatta"),
        // Kale 686 (a) -- veT
        ("damu~", 4, "dAnta|damita"),
        ("Samu~", 4, "SAnta|Samita"),
        ("pura~", 1, "pUrRa|pUrita"),
        ("dasu~", 4, "dasta|dAsita"),
        ("spaSa~^", 1, "spazwa|spaSita"),
        ("Cada", 10, "Canna|CAdita"),
        ("jYapa~", 10, "jYapta|jYapita"),
        ("ruza~", 1, "ruzwa|ruzita"),
        ("ama~", 1, "Anta|amita"),
        // TODO: saMGuz, Asvan, hfz
        // Kale 686 (b) -- more veT
        ("kliSU~", 9, "klizwa|kliSita"),
        ("pUN", 1, "pUta|pavita"),
        // Kale 701
        ("a\\da~", 1, "jagDa|anna"),
        ("UyI~\\", 1, "Uta"),
        ("kaza~", 1, "kazwa"),
        ("kfSa~", 4, "kfSa"),
        ("kzIbf~\\", 4, "kzIba"),
        ("knUyI~\\", 1, "knUta"),
        ("kzmAyI~\\", 1, "kzmAta"),
        ("kzE\\", 1, "kzAma"),
        ("gE\\", 1, "gIta"),
        ("Co\\", 4, "CAta|Cita"),
        // Other
        ("Su\\za~", 4, "Suzka"),
    ];

    test_krdanta(&cases, Krt::kta);
}

#[ignore]
#[test]
fn ktva() {
    let cases = vec![
        // Kale 745
        ("jYA\\", 9, "jYAtvA"),
        ("qudA\\Y", 3, "dattvA"),
        ("zWA\\", 1, "sTitvA"),
        ("o~hA\\N", 3, "hAtvA"),
        ("o~hA\\k", 3, "hItvA"),
        ("quDA\\Y", 3, "hitvA"),
        ("ji\\", 1, "jitvA"),
        ("pUN", 1, "pUtvA|pavitvA"),
        ("BU", 1, "BUtvA"),
        ("qukf\\Y", 8, "kftvA"),
        ("tF", 1, "tIrtvA"),
        ("pF", 9, "pUrtvA"),
        ("trE\\N", 1, "trAtvA"),
        ("mu\\cx~^", 6, "muktvA"),
        ("a\\da~", 2, "jagDvA"),
        ("Co\\", 1, "CAtvA|CitvA"),
        ("df\\Si~r", 1, "dfzwvA"),
        ("kzu\\Da~", 4, "kzuDitvA|kzoDitvA"),
        ("va\\sa~", 1, "uzitvA"),
        ("va\\ca~", 2, "uktvA"),
        ("va\\ha~^", 1, "UQvA"),
        ("ya\\ja~^", 1, "izwvA"),
        ("quva\\pa~^", 1, "uptvA"),
        ("ba\\nDa~", 9, "badDvA"),
        ("bu\\Da~\\", 1, "budDvA"),
        ("SAsu~", 2, "SizwvA"),
        // Kale 746
        ("SIN", 2, "SayitvA"),
        ("jAgf", 2, "jAgaritvA"),
        // Kale 746 (a)
        ("YitfzA~", 4, "tfzitvA|tarzitvA"),
        ("mfzu~", 1, "mfzitvA|marzitvA"),
        // Kale erroneously has "kfz" here.
        ("kfSa~", 4, "kfSitvA|karSitvA"),
        // TODO: ft?
        // Kale 746 (b)
        ("mfqa~", 9, "mfqitvA"),
        ("mfda~", 9, "mfditvA"),
        ("guDa~", 9, "guDitvA"),
        ("kuza~", 9, "kuzitvA"),
        ("kliSU~", 9, "kliSitvA|klizwvA"),
        ("vada~", 1, "uditvA"),
        // Kale 747
        ("mfjU~", 2, "mArjitvA|mfzwvA"),
        ("gAhU~\\", 1, "gAhitvA|gAQvA"),
        ("guhU~^", 1, "guhitvA|gUhitvA|gUQvA"),
        ("gupU~", 1, "gopAyitvA|gopitvA|guptvA|gupitvA"),
        ("izu~", 1, "ezitvA|izwvA"),
        ("zaha~\\", 1, "sahitvA|soQvA"),
        ("luBa~", 4, "loBitvA|lubDvA"),
        ("ancu~", 1, "aktvA|aYcitvA"),
        // TODO: more
        ("Kanu~^", 1, "KanitvA|KAtvA"),
        ("tanu~^", 8, "tanitvA|tatvA"),
        ("damu~", 4, "damitvA|dAntvA"),
        ("Samu~", 4, "SamitvA|SAntvA"),
        ("kramu~", 1, "kramitvA|krantvA|krAntvA"),
        ("kramu~", 1, "kramitvA|krantvA|krAntvA"),
        // Kale 750
        ("liKa~", 1, "liKitvA|leKitvA"),
        ("klidU~", 4, "kliditvA|kleditvA|klittvA"),
        ("luBa~", 6, "luBitvA|loBitvA"),
        ("dyuta~\\", 1, "dyutitvA|dyotitvA"),
        ("riza~", 1, "rizitvA|rezitvA"),
        ("ruza~", 1, "ruzitvA|rozitvA"),
        ("divu~", 4, "devitvA|dyutvA"),
    ];

    test_krdanta(&cases, Krt::ktvA);
}

#[ignore]
#[test]
fn kvasu() {
    let cases = vec![
        // Kale 675
        ("i\\R", 2, "Iyivas"),
        ("f\\", 3, "Arivas"),
        ("RI\\Y", 1, "ninIvas"),
        ("qupa\\ca~^z", 1, "pecivas"),
        ("va\\ca~", 2, "Ucivas"),
        ("ya\\ja~^", 1, "Ijivas"),
        // Kale erroneously has "baBaYjvas."
        ("Ba\\njo~", 7, "Bejivas"),
        ("zwu\\Y", 2, "tuzwuvas"),
        ("qukf\\Y", 8, "cakfvas"),
        ("Bi\\di~^r", 2, "biBidvas"),
        ("Gasx~", 1, "jakzivas"),
        ("df\\Si~r", 1, "dadfSivas|dadfSvas"),
        ("vida~", 2, "vividvas|vividivas"),
        ("vi\\Sa~", 6, "viviSivas|viviSvas"),
        ("janI~\\", 2, "jajanvas"),
        ("Kanu~^", 1, "caKanvas"),
        ("ga\\mx~", 1, "jagmivas|jaganvas"),
        ("ha\\na~", 2, "jaGnivas|jaGanvas"),
        // F, tF, jF
        ("tF", 1, "titIrvas"),
    ];

    test_krdanta(&cases, Krt::kvasu);
}

#[test]
fn kvin() {
    let cases = vec![("spf\\Sa~", 6, "spfS")];

    test_krdanta(&cases, Krt::kvin);
}

#[ignore]
#[test]
fn kvip() {
    let cases = vec![
        ("zUN", 2, "sU"),
        ("RI\\Y", 1, "nI"),
        ("za\\dx~", 1, "sad"),
        ("dvi\\za~^", 2, "dviz"),
        ("yu\\ji~^r", 7, "yuj"),
        ("rAjf~^", 1, "rAj"),
        ("ci\\Y", 5, "cit"),
    ];

    test_krdanta(&cases, Krt::kvip);
}

#[test]
fn nyat() {
    let cases = vec![("BU", 1, "BAvya"), ("qukf\\Y", 8, "kArya")];

    test_krdanta(&cases, Krt::Ryat);
}

#[test]
fn nvul() {
    let cases = vec![
        // Basic
        ("qukf\\Y", 8, "kAraka"),
        ("qupa\\ca~^z", 1, "pAcaka"),
        ("ha\\na~", 2, "GAtaka"),
        ("qudA\\Y", 3, "dAyaka"),
        ("quDA\\Y", 3, "DAyaka"),
        ("Samu~", 4, "Samaka"),
        ("damu~", 4, "damaka"),
        // Kale has "vaDaka" but I don't know what its dhatu is.
        ("janI~\\", 4, "janaka"),
    ];

    test_krdanta(&cases, Krt::Rvul);
}

#[test]
fn tavya() {
    let cases = vec![
        // Kale 712
        ("qudA\\Y", 3, "dAtavya"),
        ("ci\\Y", 5, "cetavya"),
        ("RI\\Y", 1, "netavya"),
        ("Sru\\", 1, "Srotavya"),
        ("BU", 1, "Bavitavya"),
        ("qukf\\Y", 8, "kartavya"),
        ("mu\\cx~^", 6, "moktavya"),
        // Kale is missing "mArjitavya".
        ("mfjU~", 2, "mArzwavya|mArjitavya"),
        ("sf\\ja~\\", 6, "srazwavya"),
        ("Bra\\sja~^", 6, "Barzwavya|Brazwavya"),
        ("Bi\\di~^r", 7, "Bettavya"),
        ("Ridi~", 1, "ninditavya"),
        ("guhU~^", 1, "goQavya|gUhitavya"),
    ];

    test_krdanta(&cases, Krt::tavya);
}

#[test]
fn tumun() {
    let cases = vec![
        // Kale 776
        ("i\\R", 2, "etum"),
        ("eDa~\\", 1, "eDitum"),
        ("qudA\\Y", 3, "dAtum"),
        ("RI\\Y", 1, "netum"),
        ("qukf\\Y", 8, "kartum"),
        ("BU", 1, "Bavitum"),
        ("DUY", 5, "Davitum|Dotum"),
        ("vFY", 9, "varitum|varItum"),
        ("gE\\", 1, "gAtum"),
        ("ga\\mx~", 1, "gantum"),
        ("granTa~", 1, "granTitum"),
        ("qupa\\ca~^z", 1, "paktum"),
        ("o~vrascU~", 6, "vraScitum|vrazwum"),
        ("guhU~^", 1, "gUhitum|goQum"),
        ("zaha~\\", 1, "sahitum|soQum"),
        ("cura~", 10, "corayitum"),
    ];

    test_krdanta(&cases, Krt::tumun);
}

#[ignore]
#[test]
fn yat() {
    let cases = vec![
        ("BU", 1, "Bavya"),
        // Kale 714
        ("qudA\\Y", 3, "deya"),
        ("De\\w", 1, "Deya"),
        ("gE\\", 1, "geya"),
        ("Co\\", 1, "Ceya"),
        ("ci\\Y", 5, "ceya"),
        ("RI\\Y", 1, "neya"),
        ("Sa\\kx~", 5, "Sakya"),
        ("zaha~\\", 1, "sahya"),
    ];

    test_krdanta(&cases, Krt::yat);
}

#[test]
fn lyuw() {
    let cases = vec![
        // Basic
        ("zaha~\\", 1, "sahana"),
        ("hase~", 1, "hasana"),
        ("SIN", 2, "Sayana"),
        ("o~vrascU~", 6, "vraScana"),
        ("du\\ha~^", 2, "dohana"),
    ];

    test_krdanta(&cases, Krt::lyuw);
}

#[test]
fn shatr() {
    let cases = vec![
        // Kale 667
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
    // Kale 666
    let cases = vec![
        // Basic cases
        ("eDa~\\", 1, "eDamAna"),
        ("vadi~\\", 1, "vandamAna"),
        ("SIN", 2, "SayAna"),
        ("dvi\\za~^", 2, "dvizARa"),
        ("quDA\\Y", 3, "daDAna"),
        // ("hu\\", 3, "juhvAna"),
        // ("divu~", 4, "dIvyamAna"),
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
