use crate::enum_boilerplate;
use crate::errors::*;
use wasm_bindgen::prelude::wasm_bindgen;

/// An auNAdika krt-pratyaya, added through 3.3.1 "uNadayo bahulam"
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Unadi {
    /// -u (kAru)
    uR,
    /// -vi (jAgfvi)
    kvin,
}

enum_boilerplate!(Unadi, {
    uR => "uR",
    kvin => "kvin",
});

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
    ac,
    /// -a
    aR,
    /// -at (jarat)
    atfn,
    /// -ani
    ani,
    /// -anIya (gamanIya, BavanIya, ...)
    anIyar,
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
    /// -Ana
    cAnaS,
    /// -a
    ka,
    /// -Ana (cakrARa, ...)
    kAnac,
    /// -i (udaDi, ...)
    ki,
    /// -i
    kin,
    /// -ura (BaNgura, ...)
    kurac,
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
    /// -a
    wak,
    /// -u
    qu,
    /// -a
    Ra,
    /// -in
    Rini,
    /// -ya
    Ryat,
    /// -ana
    Ryuw,
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
    Tuc,
    /// -na
    naN,
    /// -naj
    najiN,
    /// -na (svapna)
    nan,
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
    /// -vaca
    varac,
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
    /// An auNAdika pratyaya
    __unAdi,
}

enum_boilerplate!(Krt, {
    ac => "ac",
    aR => "aR",
    atfn => "atf~n",
    ani => "ani",
    anIyar => "anIyar",
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
    kAnac => "kAnac",
    ki => "ki",
    kin => "kin",
    kurac => "kurac",
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
    kvarap => "kvarap",
    kvasu => "kvasu~",
    kvin => "kvi~n",
    kvip => "kvi~p",
    ksnu => "ksnu",
    Kal => "Kal",
    Ga => "Ga",
    GaY => "GaY",
    GinuR => "Ginu~R",
    Gurac => "Gurac",
    Nvanip => "Nvani~p",
    wak => "wak",
    qu => "qu",
    Ra => "Ra",
    Rini => "Rini~",
    Ryat => "Ryat",
    Ryuw => "Ryuw",
    Rvuc => "Rvu~c",
    Rvul => "Rvu~l",
    tavya => "tavya",
    tavyat => "tavyat",
    tumun => "tumu~n",
    tfc => "tfc",
    tfn => "tfn",
    Takan => "Takan",
    Tuc => "Tuc",
    naN => "naN",
    najiN => "naji~N",
    nan => "nan",
    Sa => "Sa",
    Satf => "Satf~",
    SAnac => "SAnac",
    SAnan => "SAnan",
    yat => "yat",
    yuc => "yuc",
    ra => "ra",
    ru => "ru",
    lyu => "lyu~",
    lyuw => "lyu~w",
    varac => "varac",
    vuY => "vu~Y",
    vun => "vu~n",
    zAkan => "zAkan",
    zwran => "zwran",
    zvun => "zvu~n",
    __unAdi => "unAdi",
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
