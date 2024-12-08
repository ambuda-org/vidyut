use crate::core::errors::*;
use enumset::EnumSetType;

/// An annotation on some `Term`.
///
/// `Tag` is a generalization of the traditional samjnA concept and models both traditional samjnas
/// and other long-term dependencies that we need to track during the derivation, such as whether
/// guna was performed in an earlier rule.
///
/// We use Rust's `enumset` crate to efficiently store tags in an array of unsigned integers.
///
///
/// # Naming conventions
///
/// We allow non-camel-case names so that we can name specific `Tag`s according to SLP1
/// conventions. Doing so lets us more easily distinguish among `Tag`s like `Nit`, `Yit`, Rit`, and
/// `nit`.
#[allow(non_camel_case_types)]
#[derive(Debug, EnumSetType)]
pub enum Tag {
    // Morpheme types
    Upasarga,
    Gati,
    Anga,
    Dhatu,
    MulaDhatu,
    Ghu,
    Avyaya,
    Pratyaya,

    Samasa,
    Upasarjana,

    Pratipadika,
    Vibhakti,
    Sarvanama,
    Sarvanamasthana,
    Nipata,
    Nistha,
    Krtya,

    // TODO: can remove this group by using Morph instead.
    Tin,
    Sup,
    Krt,
    Nyap,
    Taddhita,

    // it-samjnas
    // ==========
    /// Placeholder *it* with no specific meaning.
    adit,
    /// (pratyaya) prevents it-agama for nisthA pratyayas per 7.2.16 but allows it optionally in
    /// non-kartari usage per 7.2.17.
    Adit,
    /// (dhatu) indicates the mandatory use of num-Agama (vidi~ -> vind).
    idit,
    /// (pratyaya) prevents it-Agama for nisthA pratyayas per 7.2.14.
    Idit,
    /// (pratyaya) optionally allows it-agama for ktvA-pratyaya per 7.2.56.
    udit,
    /// (pratyaya) optionally allows it-agama per 7.2.44.
    Udit,
    /// (dhatu) prevents shortening of the dhatu vowel when followed by Ni + caN per 7.4.2.
    fdit,
    /// (dhatu) indicates the use of aN-pratyaya in luN-lakAra per 3.1.55. (gamx~ -> agamat)
    xdit,
    /// (dhatu) prevents vrddhi in luN-lakara when followed by it-Agama per 7.2.5
    edit,
    /// (dhatu) indicates replacement of the "t" of a nistha-pratyaya with "n" per 8.2.45 (lagta ->
    /// lagna).
    odit,
    /// (krt) prevents guna and vrddhi. Causes samprasarana for vac-Adi roots (vac -> ukta) per
    /// 6.1.15 and grah-Adi roots (grah -> gfhIta) per 6.1.16.
    ///
    /// (taddhita) causes vrddhi per 7.2.118. Indicates antodAtta per 6.1.165.
    ///
    /// (agama) indicates that the Agama should be added after the term, per 1.1.46.
    kit,
    /// (taddhita) replaced with "In" per 7.1.2.
    Kit,
    /// (pratyaya) causes a term's final cavarga sound to shift to kavarga per 7.3.52 (yuj ->
    /// yoga).
    Git,
    /// (pratyaya) prevents guna and vrddhi. Causes samprasarana for grah-Adi roots (grah ->
    /// gfhIta) per 6.1.15.
    ///
    /// (dhatu) marks the dhAtu as taking only Atmanepada endings per 1.3.12.
    Nit,
    /// (pratyaya) indicates that the last syllable of the stem is udAtta per 6.1.153.
    cit,
    /// (taddhita) replaced with "Iy" per 7.1.2.
    Cit,
    /// (pratyaya) used to give distinct names to certain pratyayas, such as `jas`, `jus`, ...
    jit,
    /// (pratyaya) first letter of the bahuvacana-prathama-parasmaipada tinanta suffix. It is
    /// replaced with "ant" or similar options per 7.1.3 - 7.1.5 and with "jus" by 3.4.108 -
    /// 3.4.112.
    Jit,
    /// (dhatu) marks the dhAtu as taking either parasamaipada or Atmanepada endings per 1.3.72.
    ///
    /// (pratyaya) causes vrddhi per 7.2.115.
    Yit,
    /// (pratyaya) in a lakAra-pratyaya, indicates various transformations such as 3.4.79 and
    /// 3.4.80.
    wit,
    /// (adesha) indicates replacement of the "Ti" section of the previous term per 6.4.143.
    qit,
    /// (taddhita) replaced with "ey" per 7.1.2.
    Qit,
    /// (pratyaya) causes vrddhi per 7.2.115.
    Rit,
    /// (pratyaya)
    tit,
    /// (pratyaya)
    nit,
    /// (pratyaya) indicates anudatta accent per 3.1.4. For sarvadhatuka pratyayas, allows guna and
    /// vrddhi; all other sarvadhatuka pratyayas are marked as `Nit` per 1.2.4 and are thus blocked
    /// from causing guna and vrddhi changes per 1.1.5.
    pit,
    /// (taddhita) replaced with "Ayan" per 7.1.2.
    Pit,
    /// (adesha) indicates insertion after the term's last vowel per 1.1.47.
    ///
    /// (dhatu) indicates shortening of the dhatu's penultimate vowel when followed by a
    /// `RI`-pratyaya per 6.4.92.
    mit,
    /// (pratyaya)
    rit,
    lit,
    /// (adesha) indicates a total replacement per 1.1.55.
    ///
    /// (pratyaya) marks the pratyaya as sArvadhAtuka per 3.4.113.
    Sit,
    /// (pratyaya) uses NIz-pratyaya in strI-linga per 4.1.41.
    zit,
    /// (pratyaya) indicates that the previous term should be called `pada` per 1.4.16.
    sit,
    /// (dhatu) indicates the optional use of aN-pratyaya in luN-lakAra per 3.1.57.
    irit,
    /// (dhatu) indicates that kta-pratyaya denotes the present tense as opposed to the past tense.
    YIt,
    /// (dhatu) allows the krt-pratyaya "Tuc" per 3.1.90.
    wvit,
    /// (dhatu) allows the krt-pratyaya "ktri" per 3.1.89.
    qvit,

    /// (pratyaya) indicates general lopa.
    Luk,
    /// (pratyaya) indicates lopa that causes dvitva (hu -> juhoti)
    Slu,
    Lup,
    /// Indicates that luk is blocked.
    Aluk,

    /// (dhatu) various functions:
    /// - blocks it-agama per 7.2.10.
    /// - causes deletion of a final nasal sound per 6.4.73.
    /// - optionally allows insertion of "a" under certain conditions per 6.1.59.
    Anudatta,
    Svarita,
    /// (dhatu) marks the dhatu as taking only Atmanepada endings per 1.3.12.
    anudattet,
    /// (dhatu) marks the dhatu as taking either parasamaipada or Atmanepada endings per 1.3.72.
    svaritet,

    // Pada
    Parasmaipada,
    Atmanepada,

    // Purusha
    Prathama,
    Madhyama,
    Uttama,

    // Vacana
    Ekavacana,
    Dvivacana,
    Bahuvacana,

    // Vibhakti (subanta)
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,

    // Vibhakti conditions
    Sambodhana,
    Amantrita,
    Sambuddhi,

    // Linga (subanta)
    Pum,
    Stri,
    Napumsaka,

    // Stem types
    Nadi,
    Ghi,

    // Dvitva
    /// The doubled
    Abhyasa,
    Abhyasta,

    // Dhatuka
    Ardhadhatuka,
    Sarvadhatuka,

    // Other flags
    //
    // Certain conditions cross prakaranas in a way that is difficult to track.
    // Since these conditions are limited, we just keep track of them with
    // these flags.
    Adesha,

    // Flags on the `Term`:
    FlagGunaApavada,
    FlagGuna,
    FlagVrddhi,

    FlagTrjvat,

    FlagPratipadikaTiLopa,

    // Flags on the `Prakriya`.
    FlagNaAdeshadi,
    FlagSaAdeshadi,
    FlagNum,
    FlagNoArdhadhatuka,
    FlagAtLopa,
    FlagAntyaAcSandhi,
    /// Optionally blocks Ric-pratyaya.
    FlagNoNic,
    /// Indicates deletion of a term's final "n" in the asiddhavat section.
    FlagNaLopa,
    /// Indicates that a dhatu ends in `z` in upadesha.
    FlagShanta,
    /// Indicates the application of samprasarana.
    FlagSamprasarana,
    // Indicates that ittva was applied.
    FlagIttva,
    /// Blocks a rule that causes dirgha.
    FlagNoDirgha,
    /// Blocks a rule that causes hrasva.
    FlagNoHrasva,
    /// Indicates use of UW-adesha.
    FlagUth,

    Sankhya,
    Sat,
    // zRAntA zat
    zaw,
    /// Indicates the insertion of `na` through the Snam-vikarana.
    Snam,

    // Indicates atidesha of `ciR`-pratyaya's behavior, per 6.4.62.
    Cinvat,

    /// A sound whose first vowel is vrddhi.
    Vrddha,

    StriNyap,
    Pada,
    Bha,
    Dvitva,
    Gha,

    Pragrhya,
    Complete,

    /// Indicates use of ru-Adesha.
    Ru,
}

impl Tag {
    /// Converts a sound representing an it to its corresponding `Tag`.
    pub fn parse_it(it: char) -> Result<Tag> {
        let res = match it {
            'a' => Tag::adit,
            'A' => Tag::Adit,
            'i' => Tag::idit,
            'I' => Tag::Idit,
            'u' => Tag::udit,
            'U' => Tag::Udit,
            'f' => Tag::fdit,
            'x' => Tag::xdit,
            'e' => Tag::edit,
            'o' => Tag::odit,
            'k' => Tag::kit,
            'K' => Tag::Kit,
            'G' => Tag::Git,
            'N' => Tag::Nit,
            'c' => Tag::cit,
            'C' => Tag::Cit,
            'j' => Tag::jit,
            'J' => Tag::Jit,
            'Y' => Tag::Yit,
            'w' => Tag::wit,
            'q' => Tag::qit,
            'Q' => Tag::Qit,
            'R' => Tag::Rit,
            't' => Tag::tit,
            'n' => Tag::nit,
            'p' => Tag::pit,
            'P' => Tag::Pit,
            'm' => Tag::mit,
            'r' => Tag::rit,
            'l' => Tag::lit,
            'S' => Tag::Sit,
            'z' => Tag::zit,
            's' => Tag::sit,
            _ => return Err(Error::UnknownIt(it)),
        };
        Ok(res)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, EnumSetType)]
pub enum PrakriyaTag {
    Atmanepada,
    Parasmaipada,
    AmAtmanepada,

    Pum,
    Stri,
    Napumsaka,

    Sambodhana,
    Ekavacana,
    Dvivacana,
    Bahuvacana,

    Karmadharaya,
    Avyayibhava,
    Tatpurusha,
    Bahuvrihi,
    Dvandva,
    Samahara,

    Kartari,
    Bhave,
    Karmani,
    Ashih,

    Prathama,
    Madhyama,
    Uttama,

    /// Indicates the sense of hetu-bhaya
    FlagHetuBhaya,
    FlagHasAnitKsa,
    FlagHasSetSic,

    // Placeholder for From<Tag> default case.
    Nothing,
}

impl From<Tag> for PrakriyaTag {
    fn from(val: Tag) -> Self {
        use PrakriyaTag::*;
        use Tag as T;
        match val {
            T::Atmanepada => Atmanepada,
            T::Parasmaipada => Parasmaipada,

            T::Pum => Pum,
            T::Stri => Stri,
            T::Napumsaka => Napumsaka,

            T::Sambodhana => Sambodhana,
            T::Ekavacana => Ekavacana,
            T::Dvivacana => Dvivacana,
            T::Bahuvacana => Bahuvacana,

            T::Prathama => Prathama,
            T::Madhyama => Madhyama,
            T::Uttama => Uttama,

            _ => Nothing,
        }
    }
}
