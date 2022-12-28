use crate::args::errors::*;
use std::str::FromStr;

/// The complete list of krt-pratyayas.
///
/// Rust's naming convention is to start enum values with capital letters. However, we allow mixed
/// case explicitly here so that we can name pratyayas more concisely with SLP1. Doing so helps us
/// distinguish between pratyayas like `naN` and `nan`.
#[allow(dead_code, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Krt {
    /// -Alu
    Aluc,
    /// -Aru
    Aru,
    /// -a
    Ga,
    /// -a
    GaY,
    /// -in
    GinuR,
    /// -ura
    Gurac,
    /// -a (Izatkara, duzkara, sukara, ...)
    Kal,
    /// -van
    Nvanip,
    /// -a
    Ra,
    /// -in
    Rini,
    /// -aka
    Rvuc,
    /// -aka
    Rvul,
    /// -ya
    Ryat,
    /// -ana
    Ryuw,
    /// -Ana (laBamAna, sevamAna, ...)
    SAnac,
    /// -Ana
    SAnan,
    /// -a
    Sa,
    /// -at (gacCat, Bavat, ...)
    Satf,
    /// -Taka (gATaka)
    Takan,
    /// -Tu (vepaTu). Allowed only for dhatus that are `qvit`.
    Tuc,
    /// -Uka
    Uka,
    /// -a
    aR,
    /// -a
    ac,
    /// -anIya (gamanIya, BavanIya, ...)
    anIyar,
    /// -ani
    ani,
    /// -at (jarat)
    atfn,
    /// -Ana
    cAnaS,
    /// -in
    ini,
    /// -itra
    itra,
    /// -izRu (alaMkarizRu, prajanizRu, ...)
    izRuc,
    /// -a
    ka,
    /// -i (udaDi, ...)
    ki,
    /// -i
    kin,
    /// -luka (BIluka)
    klukan,
    /// -mara
    kmarac,
    /// -nu
    knu,
    /// -ru (BIru)
    kru,
    /// -ruka (BIruka)
    kruka,
    /// -snu (glAsnu, jizRu, ...)
    ksnu,
    /// -ta (gata, bhUta, ...)
    kta,
    /// -tavat (gatavat, bhUtavat, ...)
    ktavatu,
    /// -ti
    ktic,
    /// -ti
    ktin,
    /// -tri
    ktri,
    /// -tvA (gatvA, bhUtva, ...)
    ktvA,
    /// -ura (BaNgura, ...)
    kurac,
    /// -vara
    kvarap,
    /// -vas
    kvasu,
    /// (empty suffix)
    kvip,
    /// -ya
    kyap,
    /// -ana
    lyu,
    /// -ana
    lyuw,
    /// -na
    naN,
    /// -naj
    najiN,
    /// -na (svapna)
    nan,
    /// -u
    qu,
    /// -na (namra, kampra, ...)
    ra,
    /// -ru
    ru,
    /// -tavya (gantavya, bhavitavya, ...)
    tavya,
    /// -tavya
    tavyat,
    /// -tf (gantA, bhavitA, ...)
    tfc,
    /// -tf
    tfn,
    /// -tum (gantum, bhavitum, ...)
    tumun,
    /// -u (yuyutsu, Bikzu, ...)
    u,
    /// -uka
    ukaY,
    /// -vaca
    varac,
    /// -aka
    vuY,
    /// -aka
    vun,
    /// -a
    wak,
    /// -ya
    yat,
    /// -ana
    yuc,
    /// -Aka
    zAkan,
    /// -aka
    zvun,
    /// -tra
    zwran,
}

impl Krt {
    /// Converts each krt pratyaya to its SLP1 value.
    ///
    /// We can't simply use the enum name because we also need to indicate nasal wovels wtih
    pub fn as_str(&self) -> &'static str {
        use Krt as K;
        match self {
            K::Aluc => "Aluc",
            K::Aru => "Aru",
            K::Ga => "Ga",
            K::GaY => "GaY",
            K::GinuR => "Ginu~R",
            K::Gurac => "Gurac",
            K::Kal => "Kal",
            K::Nvanip => "Nvani~p",
            K::Ra => "Ra",
            K::Rini => "Rini~",
            K::Rvuc => "Rvuc",
            K::Rvul => "Rvul",
            K::Ryat => "Ryat",
            K::Ryuw => "Ryuw",
            K::SAnac => "SAnac",
            K::SAnan => "SAnan",
            K::Sa => "Sa",
            K::Satf => "Satf",
            K::Takan => "Takan",
            K::Tuc => "Tuc",
            K::Uka => "Uka",
            K::aR => "aR",
            K::ac => "ac",
            K::anIyar => "anIyar",
            K::ani => "ani",
            K::atfn => "atf~n",
            K::cAnaS => "cAnaS",
            K::ini => "ini",
            K::itra => "itra",
            K::izRuc => "izRuc",
            K::ka => "ka",
            K::ki => "ki",
            K::kin => "kin",
            K::klukan => "klukan",
            K::kmarac => "kmarac",
            K::knu => "knu",
            K::kru => "kru",
            K::kruka => "kruka",
            K::ksnu => "ksnu",
            K::kta => "kta",
            K::ktavatu => "ktavatu~",
            K::ktic => "ktic",
            K::ktin => "ktin",
            K::ktri => "ktri",
            K::ktvA => "ktvA",
            K::kurac => "kurac",
            K::kvarap => "kvarap",
            K::kvasu => "kvasu~",
            K::kvip => "kvip",
            K::kyap => "kyap",
            K::lyu => "lyu",
            K::lyuw => "lyuw",
            K::naN => "naN",
            K::najiN => "naji~N",
            K::nan => "nan",
            K::qu => "qu",
            K::ra => "ra",
            K::ru => "ru",
            K::tavya => "tavya",
            K::tavyat => "tavyat",
            K::tfc => "tfc",
            K::tfn => "tfn",
            K::tumun => "tumu~n",
            K::u => "u",
            K::ukaY => "ukaY",
            K::varac => "varac",
            K::vuY => "vuY",
            K::vun => "vun",
            K::wak => "wak",
            K::yat => "yat",
            K::yuc => "yuc",
            K::zAkan => "zAkan",
            K::zvun => "zvun",
            K::zwran => "zwran",
        }
    }

    /// Returns whether the krt suffix is an ArdhadhAtuka suffix.
    ///
    /// We must track this explicitly so that we can "look ahead" and potentially add -Aya or other
    /// pratyayas for certain dhAtus. For details, see the implementation of rules 3.1.28 - 3.1.31.
    pub fn is_ardhadhatuka(&self) -> bool {
        use Krt::*;
        !matches!(self, Sa | Satf | SAnac | SAnan | cAnaS)
    }
}

impl FromStr for Krt {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use Krt as K;
        let res = match value {
            "Aluc" => K::Aluc,
            "Aru" => K::Aru,
            "Ga" => K::Ga,
            "GaY" => K::GaY,
            "Ginu~R" => K::GinuR,
            "Gurac" => K::Gurac,
            "Kal" => K::Kal,
            "Nvani~p" => K::Nvanip,
            "Ra" => K::Ra,
            "Rini~" => K::Rini,
            "Rvuc" => K::Rvuc,
            "Rvul" => K::Rvul,
            "Ryat" => K::Ryat,
            "Ryuw" => K::Ryuw,
            "SAnac" => K::SAnac,
            "SAnan" => K::SAnan,
            "Sa" => K::Sa,
            "Satf" => K::Satf,
            "Takan" => K::Takan,
            "Tuc" => K::Tuc,
            "Uka" => K::Uka,
            "aR" => K::aR,
            "ac" => K::ac,
            "anIyar" => K::anIyar,
            "ani" => K::ani,
            "atf~n" => K::atfn,
            "cAnaS" => K::cAnaS,
            "ini" => K::ini,
            "itra" => K::itra,
            "izRuc" => K::izRuc,
            "ka" => K::ka,
            "ki" => K::ki,
            "kin" => K::kin,
            "klukan" => K::klukan,
            "kmarac" => K::kmarac,
            "knu" => K::knu,
            "kru" => K::kru,
            "kruka" => K::kruka,
            "ksnu" => K::ksnu,
            "kta" => K::kta,
            "ktavatu~" => K::ktavatu,
            "ktic" => K::ktic,
            "ktin" => K::ktin,
            "ktri" => K::ktri,
            "ktvA" => K::ktvA,
            "kurac" => K::kurac,
            "kvarap" => K::kvarap,
            "kvasu~" => K::kvasu,
            "kvip" => K::kvip,
            "kyap" => K::kyap,
            "lyu" => K::lyu,
            "lyuw" => K::lyuw,
            "naN" => K::naN,
            "naji~N" => K::najiN,
            "nan" => K::nan,
            "qu" => K::qu,
            "ra" => K::ra,
            "ru" => K::ru,
            "tavya" => K::tavya,
            "tavyat" => K::tavyat,
            "tfc" => K::tfc,
            "tfn" => K::tfn,
            "tumu~n" => K::tumun,
            "u" => K::u,
            "ukaY" => K::ukaY,
            "varac" => K::varac,
            "vuY" => K::vuY,
            "vun" => K::vun,
            "wak" => K::wak,
            "yat" => K::yat,
            "yuc" => K::yuc,
            "zAkan" => K::zAkan,
            "zvun" => K::zvun,
            "zwran" => K::zwran,
            _ => return Err("Could not parse krt"),
        };

        Ok(res)
    }
}

/// The information required to derive a krdanta in the grammar.
pub struct KrdantaArgs {
    krt: Krt,
}

impl KrdantaArgs {
    /// The krt pratyaya to use in the derivation.
    pub fn krt(&self) -> Krt {
        self.krt
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> KrdantaArgsBuilder {
        KrdantaArgsBuilder::default()
    }
}

/// Convenience struct for building a `KrdantaArgs` object.
#[derive(Default)]
pub struct KrdantaArgsBuilder {
    krt: Option<Krt>,
}

impl KrdantaArgsBuilder {
    /// Sets the krt-pratyaya to use in the derivation.
    pub fn krt(&mut self, val: Krt) -> &mut Self {
        self.krt = Some(val);
        self
    }

    /// Converts the arguments in this builder into a `TinantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<KrdantaArgs, ArgumentError> {
        Ok(KrdantaArgs {
            krt: match self.krt {
                Some(x) => x,
                _ => return Err(ArgumentError::new("krt")),
            },
        })
    }
}
