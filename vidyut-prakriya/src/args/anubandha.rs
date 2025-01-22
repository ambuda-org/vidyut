use crate::args::macros::sanskrit_enum;
use crate::core::errors::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// One of the "indicatory" letters attached to an *aupadeśika*.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Anubandha {
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
    /// (taddhita) replaced by "ik".
    Wit,
    /// (adesha) indicates replacement of the "Ti" section of the previous term per 6.4.143.
    qit,
    /// (taddhita) replaced with "ey" per 7.1.2.
    Qit,
    /// (pratyaya) causes vrddhi per 7.2.115.
    Rit,
    /// (pratyaya) causes *svarita*.
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
    /// (pratyaya)
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
}

sanskrit_enum!(Anubandha, {
    adit => "adit",
    Adit => "Adit",
    idit => "idit",
    Idit => "Idit",
    udit => "udit",
    Udit => "Udit",
    fdit => "fdit",
    xdit => "xdit",
    edit => "edit",
    odit => "odit",
    kit => "kit",
    Kit => "Kit",
    Git => "Git",
    Nit => "Nit",
    cit => "cit",
    Cit => "Cit",
    jit => "jit",
    Jit => "Jit",
    Yit => "Yit",
    wit => "wit",
    Wit => "Wit",
    qit => "qit",
    Qit => "Qit",
    Rit => "Rit",
    tit => "tit",
    nit => "nit",
    pit => "pit",
    Pit => "Pit",
    mit => "mit",
    rit => "rit",
    lit => "lit",
    Sit => "Sit",
    sit => "sit",
    zit => "zit",
    irit => "irit",
    YIt => "YIt",
    wvit => "wvit",
    qvit => "qvit",
});

impl TryFrom<char> for Anubandha {
    type Error = Error;

    fn try_from(val: char) -> Result<Anubandha> {
        use Anubandha::*;
        let res = match val {
            'a' => adit,
            'A' => Adit,
            'i' => idit,
            'I' => Idit,
            'u' => udit,
            'U' => Udit,
            'f' => fdit,
            'x' => xdit,
            'e' => edit,
            'o' => odit,
            'k' => kit,
            'K' => Kit,
            'G' => Git,
            'N' => Nit,
            'c' => cit,
            'C' => Cit,
            'j' => jit,
            'J' => Jit,
            'Y' => Yit,
            'w' => wit,
            'W' => Wit,
            'q' => qit,
            'Q' => Qit,
            'R' => Rit,
            't' => tit,
            'n' => nit,
            'p' => pit,
            'P' => Pit,
            'm' => mit,
            'r' => rit,
            'l' => lit,
            'S' => Sit,
            'z' => zit,
            's' => sit,
            _ => return Err(Error::UnknownIt(val)),
        };
        Ok(res)
    }
}
