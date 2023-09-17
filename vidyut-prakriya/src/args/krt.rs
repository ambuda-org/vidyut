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
    /// -am
    kamul,
    /// -as (visfpaH, ...)
    kasun,
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
    /// -a (Izatkara, duzkara, sukara, ...)
    Kal,
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
    /// -Tu (vepaTu). Allowed only for dhatus that are `qvit`.
    aTuc,
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
    /// -u (kAru)
    uR,
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
    ani => "ani",
    anIyar => "anIyar",
    ap => "ap",
    Aluc => "Aluc",
    Aru => "Aru",
    itra => "itra",
    ini => "ini",
    izRuc => "izRuc",
    u => "u",
    ukaY => "ukaY",
    Uka => "Uka",
    cAnaS => "cAnaS",
    ka => "ka",
    kamul => "kamu~l",
    kasun => "kasu~n",
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
    aTuc => "aTuc",
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
    vuY => "vu~Y",
    vun => "vu~n",
    zAkan => "zAkan",
    zwran => "zwran",
    zvun => "zvu~n",

    // unAdi-pratyayas
    // ===============
    amac => "amac",
    alac => "alac",
    Ayya => "Ayya",
    itnuc => "itnuc",
    iTin => "iTin",
    izWuc => "izWuc",
    izWac => "izWac",
    isan => "isan",
    uR => "uR",
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

impl Krt {
    /// Returns whether the krt suffix is an ArdhadhAtuka suffix.
    ///
    /// We must track this explicitly so that we can "look ahead" and potentially add -Aya or other
    /// pratyayas for certain dhAtus. For details, see the implementation of rules 3.1.28 - 3.1.31.
    pub fn is_ardhadhatuka(&self) -> bool {
        use Krt::*;
        !matches!(self, Sa | Satf | SAnac | SAnan | cAnaS)
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
#[derive(Clone, Default, Hash, Eq, PartialEq)]
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
    pub fn build(&self) -> Result<KrdantaArgs> {
        Ok(KrdantaArgs {
            krt: match self.krt {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("krt")),
            },
        })
    }
}
