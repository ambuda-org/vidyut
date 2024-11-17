use crate::args::dhatu::Dhatu;
use crate::args::unadi::Unadi;
use crate::args::Lakara;
use crate::args::Subanta;
use crate::core::errors::*;
use crate::enum_boilerplate;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The complete list of ordinary *kṛt pratyaya*s.
///
/// Rust's naming convention is to start enum values with capital letters. However, we allow mixed
/// case explicitly here so that we can name pratyayas more concisely with SLP1. Doing so helps us
/// distinguish between pratyayas like `naN` and `nan`.
#[allow(dead_code, non_camel_case_types)]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BaseKrt {
    /// -a
    a,
    /// -a,
    aN,
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
    /// -ika
    ika,
    /// -ikavaka
    ikavaka,
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
    /// -a
    wa,
    /// -a
    wak,
    /// -a
    qa,
    /// -ara,
    qara,
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
}

enum_boilerplate!(BaseKrt, {
    a => "a",
    aN => "aN",
    ac => "ac",
    aR => "aR",
    atfn => "atf~n",
    aTuc => "aTuc",
    ani => "ani",
    anIyar => "anIyar",
    ap => "ap",
    Aluc => "Aluc",
    Aru => "Aru",
    ika => "ika",
    ikavaka => "ikavaka",
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
    qara => "qara",
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
});

/// Models a *kṛt pratyaya*.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Krt {
    /// An ordinary *kṛt pratyaya* as declared in the Ashtadhyayi.
    Base(BaseKrt),
    /// An *uṇādi pratyaya* as declared in the Unadipatha.
    Unadi(Unadi),
}

impl From<BaseKrt> for Krt {
    fn from(base: BaseKrt) -> Krt {
        Krt::Base(base)
    }
}

impl From<Unadi> for Krt {
    fn from(unadi: Unadi) -> Krt {
        Krt::Unadi(unadi)
    }
}

impl Krt {
    /// Returns whether the krt suffix is an *ārdhadhātuka* suffix.
    ///
    /// We must track this explicitly so that we can "look ahead" and potentially add `-Aya` or
    /// other *pratyaya*s for certain *dhātu*s. For details, see the implementation of rules 3.1.28
    /// - 3.1.31.
    pub fn is_ardhadhatuka(&self) -> bool {
        use BaseKrt::*;
        match self {
            Krt::Base(k) => !matches!(k, Sa | Satf | SAnac | SAnan | cAnaS | KaS),
            Krt::Unadi(_) => true,
        }
    }

    /// Returns a simple human-readable string that represents this enum's value.
    ///
    /// This mapping is not reversible. This is because some pratyayas are in both `Base` and
    /// `Unadi`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Krt::Base(b) => b.as_str(),
            Krt::Unadi(u) => u.as_str(),
        }
    }
}

/// Models the meaning of a *kṛt pratyaya*.
///
/// *kṛt*s are often available only in specific senses. A given *kṛt* might be allowed in one sense
/// but blocked in another.
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KrtArtha {
    /// Having a habit, nature, or skill. (3.2.134)
    TacchilaTaddharmaTatsadhukara,
    /// Existence. (3.3.18)
    Bhava,
    /// Solidity. (3.3.77)
    Murti,
    /// Location. (3.3.78)
    Desha,
    /// Designation. (3.3.118)
    Samjna,
    /// Agent. (3.4.67)
    Karta,
}

/// The information required to derive a *kṛdanta*.
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Krdanta {
    /// The dhatu to which we will add our krt-pratyaya.
    dhatu: Dhatu,
    /// The krt-pratyaya to use.
    krt: Krt,
    /// Whether this krdanta must follow a specific `KrtArtha` condition.
    artha: Option<KrtArtha>,
    /// Whether this krdanta must replace a specific `Lakara`. If unset, default to `Lat` if
    /// necessary.
    lakara: Option<Lakara>,
    /// Whether this krdanta is allowed only with a specific *upapada*.
    upapada: Option<Subanta>,
    /// Whether the derived krdanta must have exactly the specified value.
    require: Option<String>,
}

impl Krdanta {
    /// Defines a simple `Krdanta`.
    ///
    /// For more options, use `Krdanta::builder()` instead.
    pub fn new(dhatu: Dhatu, krt: impl Into<Krt>) -> Self {
        Self {
            dhatu,
            krt: krt.into(),
            artha: None,
            lakara: None,
            upapada: None,
            require: None,
        }
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> KrdantaBuilder {
        KrdantaBuilder::default()
    }

    /// The dhatu to use in the derivation.
    pub fn dhatu(&self) -> &Dhatu {
        &self.dhatu
    }

    /// The krt-pratyaya to use in the derivation.
    pub fn krt(&self) -> Krt {
        self.krt
    }

    /// The lakara that this krt-pratyaya will replace.
    pub fn lakara(&self) -> Option<Lakara> {
        use BaseKrt::*;
        use Lakara::*;

        if let Krt::Base(krt) = self.krt {
            match krt {
                Satf | SAnac => match self.lakara {
                    Some(Lat) | Some(Lrt) => self.lakara,
                    _ => Some(Lat),
                },
                kAnac | kvasu => Some(Lit),
                _ => None,
            }
        } else {
            None
        }
    }

    /// The upapada that conditions the krt pratyaya.
    pub fn upapada(&self) -> &Option<Subanta> {
        &self.upapada
    }

    /// The artha condition to use in the derivation. If not set, any artha is allowed.
    pub fn artha(&self) -> Option<KrtArtha> {
        self.artha
    }

    /// The value that the krdanta must match, if defined.
    pub fn require(&self) -> &Option<String> {
        &self.require
    }

    /// Sets the required value for this krdanta.
    pub fn with_require(mut self, s: impl AsRef<str>) -> Self {
        self.require = Some(s.as_ref().to_string());
        self
    }
}

/// Convenience struct for building a `KrdantaArgs` object.
#[derive(Clone, Default, Eq, PartialEq)]
pub struct KrdantaBuilder {
    dhatu: Option<Dhatu>,
    krt: Option<Krt>,
    upapada: Option<Subanta>,
    artha: Option<KrtArtha>,
    lakara: Option<Lakara>,
    require: Option<String>,
}

impl KrdantaBuilder {
    /// Sets the krt-pratyaya to use in the derivation.
    pub fn dhatu(&mut self, dhatu: Dhatu) -> &mut Self {
        self.dhatu = Some(dhatu);
        self
    }

    /// Sets the krt-pratyaya to use in the derivation.
    pub fn krt(&mut self, val: impl Into<Krt>) -> &mut Self {
        self.krt = Some(val.into());
        self
    }

    /// Sets the upapada to use in the derivation.
    pub fn upapada(&mut self, upapada: Subanta) -> &mut Self {
        self.upapada = Some(upapada);
        self
    }

    /// Sets the upapada to use in the derivation.
    pub fn artha(&mut self, artha: KrtArtha) -> &mut Self {
        self.artha = Some(artha);
        self
    }

    /// Sets the lakara to use in the derivation.
    ///
    /// This field is necessary for pratyayas like Satf and SAnac, which replace a specific lakara.
    /// If `lakara` is not specified, prakriyas will default to lat-lakara.
    pub fn lakara(&mut self, lakara: Lakara) -> &mut Self {
        self.lakara = Some(lakara);
        self
    }

    /// Sets the value that the krdanta must have.
    pub fn require(&mut self, text: impl AsRef<str>) -> &mut Self {
        self.require = Some(text.as_ref().to_string());
        self
    }

    /// Converts the arguments in this builder into a `TinantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<Krdanta> {
        Ok(Krdanta {
            dhatu: match &self.dhatu {
                Some(x) => x.clone(),
                _ => return Err(Error::missing_required_field("dhatu")),
            },
            krt: match self.krt {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("krt")),
            },
            upapada: self.upapada.as_ref().cloned(),
            lakara: self.lakara,
            artha: self.artha,
            require: self.require.clone(),
        })
    }
}
