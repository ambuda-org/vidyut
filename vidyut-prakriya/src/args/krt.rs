use crate::enum_boilerplate;
use crate::errors::*;
use wasm_bindgen::prelude::wasm_bindgen;

/// The complete list of krt-pratyayas.
///
/// Rust's naming convention is to start enum values with capital letters. However, we allow mixed
/// case explicitly here so that we can name pratyayas more concisely with SLP1. Doing so helps us
/// distinguish between pratyayas like `naN` and `nan`.
#[allow(dead_code, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[wasm_bindgen]
pub enum Krt {
    /// -a
    a,
    /// -a
    ac,
    /// -a
    aR,
    /// -at (jarat)
    atfn,
    /// -aTu (vepaTu). Allowed only for dhatus that are `qvit`.
    aTuc,
    /// -ani
    ani,
    /// -anIya (gamanIya, BavanIya, ...)
    anIyar,
    /// -a
    ap,
    /// -Alu
    Aluc,
    /// -Aru
    Aru,
    /// -itra
    itra,
    /// -in. The trailing `_` is to avoid colliding with Rust's `in` keyword.
    in_,
    /// -in
    ini,
    /// -izRu (alaMkarizRu, prajanizRu, ...)
    izRuc,
    /// -u (yuyutsu, Bikzu, ...)
    u,
    /// -uka
    ukaY,
    /// -Uka
    Uka,
    /// -a
    ka,
    /// -a
    kaY,
    /// -am
    kamul,
    /// -as (visfpaH, ...)
    kasun,
    /// -a
    kap,
    /// -Ana (cakrARa, ...)
    kAnac,
    /// -i (udaDi, ...)
    ki,
    /// -i
    kin,
    /// -ura (BaNgura, ...)
    kurac,
    /// -elima (pacelima, ...)
    kelimar,
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
    /// -nu
    knu,
    /// -mara
    kmarac,
    /// -ya
    kyap,
    /// -ru (BIru)
    kru,
    /// -ruka (BIruka)
    kruka,
    /// -luka (BIluka)
    klukan,
    /// -van
    kvanip,
    /// -vara
    kvarap,
    /// -vas
    kvasu,
    /// -snu (glAsnu, jizRu, ...)
    ksnu,
    /// (empty suffix)
    kvin,
    /// (empty suffix)
    kvip,
    /// -a (priyaMvada, vaSaMvada)
    Kac,
    /// -a
    KaS,
    /// -a (Izatkara, duzkara, sukara, ...)
    Kal,
    /// -izRu
    KizRuc,
    /// -uka
    KukaY,
    /// -ana
    Kyun,
    /// -a
    Ga,
    /// -a
    GaY,
    /// -in
    GinuR,
    /// -ura
    Gurac,
    /// -van
    Nvanip,
    /// -Ana
    cAnaS,
    /// -anta,
    Jac,
    /// -a
    wa,
    /// -a
    wak,
    /// -a
    qa,
    /// -u
    qu,
    /// -a
    Ra,
    /// -am
    Ramul,
    /// -in
    Rini,
    /// -ya
    Ryat,
    /// -ana
    Ryuw,
    /// (empty)
    Rvi,
    /// -aka
    Rvuc,
    /// -aka
    Rvul,
    /// -tavya (gantavya, bhavitavya, ...)
    tavya,
    /// -tavya
    tavyat,
    /// -tum (gantum, bhavitum, ...)
    tumun,
    /// -tf (gantA, bhavitA, ...)
    tfc,
    /// -tf
    tfn,
    /// -Taka (gATaka)
    Takan,
    /// -na
    naN,
    /// -naj
    najiN,
    /// -na (svapna)
    nan,
    /// -man
    manin,
    /// -a
    Sa,
    /// -at (gacCat, Bavat, ...)
    Satf,
    /// -Ana (laBamAna, sevamAna, ...)
    SAnac,
    /// -Ana
    SAnan,
    /// -ya
    yat,
    /// -ana
    yuc,
    /// -na (namra, kampra, ...)
    ra,
    /// -ru
    ru,
    /// -ana
    lyu,
    /// -ana
    lyuw,
    /// -van
    vanip,
    /// -vara
    varac,
    /// (empty suffix)
    vic,
    /// (none)
    viw,
    /// -aka
    vuY,
    /// -aka
    vun,
    /// -Aka
    zAkan,
    /// -tra
    zwran,
    /// -aka
    zvun,

    // unAdi-pratyayas
    // ===============
    /// -ama (praTama)
    amac,
    /// -ala (maNgala)
    alac,
    /// -as (cetaH)
    asun,
    /// -Ayya
    Ayya,
    /// -itnu
    itnuc,
    /// -iTi
    iTin,
    /// -iza
    wizac,
    /// -izWu
    izWuc,
    /// -izWa
    izWac,
    /// -isa
    isan,
    /// -is
    isi,
    /// -u (kAru)
    uR,
    /// -us (Danus)
    usi,
    /// -atu (kratu)
    katu,
    /// -ka
    kan,
    /// -Ta
    kTan,
    /// -vi (jAgfvi)
    kvinUnadi,
    /// -sara
    ksaran,
    /// -si
    ksi,
    /// -su
    ksu,
    /// -u (tAlu)
    YuR,
    /// -ta
    tan,
    /// -tu
    tun,
    /// -tra,
    tran,
    /// -sa
    sa,
    /// -sara
    sara,
    /// -su
    suk,
    /// -atni,
    katnic,
    /// -yatu,
    yatuc,
    /// -ali
    alic,
    /// -sya
    syan,
    /// -uli
    uli,
    /// -as (use trailing `_` since `as` is a reserved keyword in Rust.)
    asa,
    /// -As,
    Asa,
    /// -Anu
    Anuk,
}

enum_boilerplate!(Krt, {
    a => "a",
    ac => "ac",
    aR => "aR",
    atfn => "atf~n",
    aTuc => "aTuc",
    ani => "ani",
    anIyar => "anIyar",
    ap => "ap",
    Aluc => "Aluc",
    Aru => "Aru",
    itra => "itra",
    in_ => "in",
    ini => "ini~",
    izRuc => "izRuc",
    u => "u",
    ukaY => "ukaY",
    Uka => "Uka",
    cAnaS => "cAnaS",
    ka => "ka",
    kaY => "kaY",
    kamul => "kamu~l",
    kasun => "kasu~n",
    kap => "kap",
    kAnac => "kAnac",
    ki => "ki",
    kin => "kin",
    kurac => "kurac",
    kelimar => "kelimar",
    kta => "kta",
    ktavatu => "ktavatu~",
    ktic => "ktic",
    ktin => "ktin",
    ktri => "ktri",
    ktvA => "ktvA",
    knu => "knu",
    kmarac => "kmarac",
    kyap => "kyap",
    kru => "kru",
    kruka => "kruka",
    klukan => "klukan",
    kvanip => "kvani~p",
    kvarap => "kvarap",
    kvasu => "kvasu~",
    kvin => "kvi~n",
    kvip => "kvi~p",
    ksnu => "ksnu",
    Kac => "Kac",
    KaS => "KaS",
    KizRuc => "KizRuc",
    KukaY => "KukaY",
    Kyun => "Kyu~n",
    Kal => "Kal",
    Ga => "Ga",
    GaY => "GaY",
    GinuR => "Ginu~R",
    Gurac => "Gurac",
    Nvanip => "Nvani~p",
    wa => "wa",
    wak => "wak",
    qa => "qa",
    qu => "qu",
    Ra => "Ra",
    Ramul => "Ramu~l",
    Rini => "Rini~",
    Ryat => "Ryat",
    Ryuw => "Ryu~w",
    Rvi => "Rvi~",
    Rvuc => "Rvu~c",
    Rvul => "Rvu~l",
    tavya => "tavya",
    tavyat => "tavyat",
    tumun => "tumu~n",
    tfc => "tfc",
    tfn => "tfn",
    Takan => "Takan",
    naN => "naN",
    najiN => "naji~N",
    nan => "nan",
    manin => "mani~n",
    Sa => "Sa",
    Satf => "Satf~",
    SAnac => "SAnac",
    SAnan => "SAnan",
    yat => "yat",
    yuc => "yu~c",
    ra => "ra",
    ru => "ru",
    lyu => "lyu~",
    lyuw => "lyu~w",
    vanip => "vani~p",
    varac => "varac",
    vic => "vi~c",
    viw => "vi~w",
    vuY => "vu~Y",
    vun => "vu~n",
    zAkan => "zAkan",
    zwran => "zwran",
    zvun => "zvu~n",

    // unAdi-pratyayas
    // ===============
    amac => "amac",
    alac => "alac",
    asun => "asu~n",
    Ayya => "Ayya",
    itnuc => "itnuc",
    iTin => "iTin",
    izWuc => "izWuc",
    izWac => "izWac",
    isan => "isan",
    isi => "isi~",
    uR => "uR",
    usi => "usi~",
    // TODO: why do we keep the initial 'k' here?
    kan => "a~kan",
    katu => "katu",
    kTan => "kTan",
    kvinUnadi => "kvin",
    ksaran => "ksaran",
    ksi => "ksi",
    ksu => "ksu",
    Jac => "Jac",
    YuR => "YuR",
    wizac => "wizac",
    tan => "tan",
    tun => "tun",
    tran => "tran",
    sa => "sa",
    sara => "sara",
    suk => "suk",
    katnic => "katnic",
    yatuc => "yatuc",
    alic => "alic",
    syan => "syan",
    uli => "uli",
    asa => "asa",
    Asa => "Asa",
    Anuk => "Anuk",
});

/// Models the meaning of a krt-pratyaya.
///
/// krts are often available only in specific senses. A given krta might be allowed in one sense
/// but blocked in another. To model and test this behavior, we use the enum below.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum KrtArtha {
    /// Agent. (3.4.67)
    Karta,
    /// Existence. (3.3.18)
    Bhava,
    /// Having a habit, nature, or skill.
    TacchilaTaddharmaTatsadhukara,
    /// Designation. (3.3.118)
    Samjna,
    /// Solidity. (3.3.77)
    Murti,
    /// Location. (3.3.78)
    Desha,
}

impl Krt {
    /// Returns whether the krt suffix is an ArdhadhAtuka suffix.
    ///
    /// We must track this explicitly so that we can "look ahead" and potentially add -Aya or other
    /// pratyayas for certain dhAtus. For details, see the implementation of rules 3.1.28 - 3.1.31.
    pub fn is_ardhadhatuka(&self) -> bool {
        use Krt::*;
        !matches!(self, Sa | Satf | SAnac | SAnan | cAnaS | KaS)
    }
}

/// An upapada (dependent word) for a krdanta derivation.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Upapada {
    /// A generic subanta.
    Subanta(String),
    /// An avyaya.
    Avyaya(String),
    /// An upasarga.
    Upasarga(String),
}

impl Upapada {
    /// Creates an upapada from a generic subanta.
    pub fn make_subanta(text: impl AsRef<str>) -> Self {
        Self::Subanta(text.as_ref().to_string())
    }

    /// Creates an upapada from an avyaya.
    pub fn make_avyaya(text: impl AsRef<str>) -> Self {
        Self::Avyaya(text.as_ref().to_string())
    }

    /// Creates an upapada from an upasarga.
    pub fn make_upasarga(text: impl AsRef<str>) -> Self {
        Self::Upasarga(text.as_ref().to_string())
    }

    /// Returns the text corresponding to this upapada.
    pub fn text(&self) -> &str {
        match self {
            Self::Subanta(x) => x,
            Self::Avyaya(x) => x,
            Self::Upasarga(x) => x,
        }
    }
}

/// The information required to derive a krdanta in the grammar.
pub struct KrdantaArgs {
    krt: Krt,
    artha: Option<KrtArtha>,
    upapada: Option<Upapada>,
}

impl KrdantaArgs {
    /// The krt pratyaya to use in the derivation.
    pub fn krt(&self) -> Krt {
        self.krt
    }

    /// The upapada that conditions the krt pratyaya.
    pub fn upapada(&self) -> &Option<Upapada> {
        &self.upapada
    }

    /// The artha condition to use in the derivation. If not set, any artha is allowed.
    pub fn artha(&self) -> Option<KrtArtha> {
        self.artha
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> KrdantaArgsBuilder {
        KrdantaArgsBuilder::default()
    }
}

/// Convenience struct for building a `KrdantaArgs` object.
#[derive(Clone, Default, Eq, PartialEq)]
pub struct KrdantaArgsBuilder {
    krt: Option<Krt>,
    upapada: Option<Upapada>,
    artha: Option<KrtArtha>,
}

impl KrdantaArgsBuilder {
    /// Sets the krt-pratyaya to use in the derivation.
    pub fn krt(&mut self, val: Krt) -> &mut Self {
        self.krt = Some(val);
        self
    }

    /// Sets the upapada to use in the derivation.
    pub fn upapada(&mut self, upapada: Upapada) -> &mut Self {
        self.upapada = Some(upapada);
        self
    }

    /// Sets the upapada to use in the derivation.
    pub fn artha(&mut self, artha: KrtArtha) -> &mut Self {
        self.artha = Some(artha);
        self
    }

    /// Converts the arguments in this builder into a `TinantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<KrdantaArgs> {
        Ok(KrdantaArgs {
            krt: match self.krt {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("krt")),
            },
            upapada: self.upapada.as_ref().cloned(),
            artha: self.artha,
        })
    }
}
