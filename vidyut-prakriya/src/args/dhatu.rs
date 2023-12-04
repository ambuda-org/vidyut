use crate::args::Pratipadika;
use crate::core::errors::Error;
use crate::enum_boilerplate;
use wasm_bindgen::prelude::wasm_bindgen;

/// Defines a *gaṇa*.
///
/// The dhatus in the Dhatupatha are organized in ten large *gaṇa*s or classes. These gaṇas
/// add various properties to the dhatu, most notably the specific *vikaraṇa* (stem suffix) we use
/// before sarvadhatuka suffixes.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[wasm_bindgen]
pub enum Gana {
    /// The first gaṇa, whose first dhatu is `BU`.
    Bhvadi,
    /// The second gaṇa, whose first dhatu is `ad`.
    Adadi,
    /// The third gaṇa, whose first dhatu is `hu`.
    Juhotyadi,
    /// The fourth gaṇa, whose first dhatu is `div`.
    Divadi,
    /// The fifth gaṇa, whose first dhatu is `su`.
    Svadi,
    /// The sixth gaṇa, whose first dhatu is `tud`.
    Tudadi,
    /// The seventh gaṇa, whose first dhatu is `rudh`.
    Rudhadi,
    /// The eighth gaṇa, whose first dhatu is `tan`.
    Tanadi,
    /// The ninth gaṇa, whose first dhatu is `krI`.
    Kryadi,
    /// The tenth gaṇa, whose first dhatu is `cur`.
    Curadi,
}

enum_boilerplate!(Gana, {
    Bhvadi => "1",
    Adadi => "2",
    Juhotyadi => "3",
    Divadi => "4",
    Svadi => "5",
    Tudadi => "6",
    Rudhadi => "7",
    Tanadi => "8",
    Kryadi => "9",
    Curadi => "10",
});

/// Defines an antargana.
///
/// The dhatus in the Dhatupatha are organized in ten large *gaṇa*s or classes. Within these larger
/// *gaṇa*s, certain *antargaṇa*s or subclasses have extra properties that affect the derivations
/// they produce. For example, dhatus in the *kuṭādi antargaṇa* generally do not allow *guṇa* vowel
/// changes.
///
/// Since most dhatus appear exactly once per *gaṇa*, this crate can usually infer whether a dhatu
/// is in a specific *antargaṇa*. However, some *gaṇa*s have dhatus that repeat, and these
/// repeating dhatus cause ambiguities for our code. (Examples: `juqa~` appears twice in
/// *tudādigaṇa*, once in *kuṭādi* and once outside of it.)
///
/// To avoid this ambiguity, we require that certain *antargaṇa*s are declared up-front.
///
/// (Can't we disambiguate by checking the dhatu's index within its gana? Unfortunately, no. There
/// is no canonical version of the Dhatupatha, and we cannot expect that a dhatu's index is
/// consistent across all of these versions. So we thought it better to avoid hard-coding indices
/// or requiring callers to follow our specific conventions.)
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Antargana {
    /// Antargana of *tud* gana. Pratyayas that follow dhatus in kut-Adi will generally be marked
    /// Nit per 1.2.1. Required because of duplicates like `juqa~`.
    Kutadi,
    /// Antargana of *cur* gana ending with `zvada~` / `svAda~`. A dhatu in this antargana
    /// optionaly uses Ric-pratyaya when taking an object. Required because of duplicates like
    /// `tuji~`.
    Asvadiya,
    /// Antargana of *cur* gana ending with `Dfza~`. A dhatu in this antargana optionaly uses
    /// Ric-pratyaya. Required because of duplicates like `SraTa~`.
    Adhrshiya,
    /// Antargana of *cur* gana ending with `kusma~`. A dhatu in this antargana is always
    /// ātmanepadī. Required because of duplicates like `daSi~`.
    Akusmiya,
}

enum_boilerplate!(Antargana, {
    Kutadi => "kuwAdi",
    Akusmiya => "AkusmIya",
    Asvadiya => "AsvadIya",
    Adhrshiya => "ADfzIya",
});

/// One of the three common *sanAdi* pratyayas.
///
/// The *sanAdi* pratyayas create new dhatus per 3.1.32. They are introduced in rules 3.1.7 -
/// 3.1.30, and since rule 3.1.7 contains the word "dhAtoH", they can be called Ardhadhatuka by
/// 3.4.114.
///
/// Of the sanAdi pratyayas, most are added after either subantas or a handful of dhatus. But
/// three of these pratyayas are added after dhatus more generally: `san`, `yaN`, and `Ric`.
///
/// For details on what these pratyayas mean and what kinds of words they produce, see the comments
/// below.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[wasm_bindgen]
pub enum Sanadi {
    /// `kAmyac`, which creates nAma-dhAtus per 3.1.9.
    kAmyac,

    /// `kyaN`, which creates nAma-dhAtus per 3.1.11.
    kyaN,

    /// `kyac`, which creates nAma-dhAtus per 3.1.8.
    kyac,

    /// `Nic`, which creates causal roots per 3.1.26.
    ///
    /// Examples: BAvayati, nAyayati.
    Ric,

    /// `yaN`, which creates intensive roots per 3.1.22. For certain dhatus, the semantics are
    /// instead "crooked movement" (by 3.1.23) or "contemptible" action (by 3.1.24).
    ///
    /// Examples: boBUyate, nenIyate.
    ///
    /// Constraints: can be used only if the dhatu starts with a consonant and has exactly one
    /// vowel. If this constraint is violated, our APIs will return an `Error`.
    yaN,

    /// `yaN`, with elision per 2.4.74. This is often listed separately due to its rarity and its
    /// very different form.
    ///
    /// Examples: boBavIti, boBoti, nenayIti, neneti.
    yaNluk,

    /// `san`, which creates desiderative roots per 3.1.7.
    ///
    /// Examples: buBUzati, ninIzati.
    san,
}

enum_boilerplate!(Sanadi, {
    san => "san",
    kyac => "kyac",
    kAmyac => "kAmyac",
    kyaN => "kyaN",
    yaN => "yaN",
    yaNluk => "yaNluk",
    Ric => "Ric",
});

/// The verb root to use for the derivation.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Dhatu {
    /// Indicates a muladhAtu from the Dhatupatha.
    Mula(Muladhatu),
    /// Indicates a nAma-dhAtu created with a sanAdi-pratyaya.
    Nama(Namadhatu),
}

/// A dhatu from the Dhatupatha.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Muladhatu {
    upadesha: String,
    gana: Gana,
    antargana: Option<Antargana>,
    sanadi: Vec<Sanadi>,
    prefixes: Vec<String>,
}

/// A dhatu created from a subanta.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Namadhatu {
    pratipadika: Pratipadika,
    sanadi: Vec<Sanadi>,
    pub(crate) prefixes: Vec<String>,
}

impl Muladhatu {
    /// Creates a new Muladhatu
    pub fn new(upadesha: &str, gana: Gana) -> Self {
        Self {
            upadesha: String::from(upadesha),
            gana,
            antargana: None,
            sanadi: Vec::new(),
            prefixes: Vec::new(),
        }
    }
    /// The dhatu as stated in its aupadeshka form. `upadesha` should be an SLP1 string that
    /// includes any necessary svaras. For examples, see the `dhatu` column in the
    /// `data/dhatupatha.tsv` file included in this crate.
    pub fn upadesha(&self) -> &String {
        &self.upadesha
    }

    /// The dhatu's gana.
    pub fn gana(&self) -> Gana {
        self.gana
    }

    /// The antargana this dhatu belongs to.
    pub fn antargana(&self) -> Option<Antargana> {
        self.antargana
    }

    /// The sanAdi pratyayas to use with this dhatu.
    pub fn sanadi(&self) -> &Vec<Sanadi> {
        &self.sanadi
    }

    /// The prefixes to use with the dhatu.
    pub fn prefixes(&self) -> &Vec<String> {
        &self.prefixes
    }

    /// Returns whether the dhatu has the given gana.
    pub fn has_gana(&self, gana: impl Into<Gana>) -> bool {
        self.gana == gana.into()
    }

    /// Sets the sanadi-pratyayas on the dhatu.
    pub fn with_antargana(mut self, antargana: Antargana) -> Self {
        self.antargana = Some(antargana);
        self
    }
}

impl Namadhatu {
    /// The pratipadika to use as the basis of this dhatu.
    pub fn pratipadika(&self) -> &Pratipadika {
        &self.pratipadika
    }

    /// The sanAdi pratyayas to use with this dhatu.
    pub fn sanadi(&self) -> &Vec<Sanadi> {
        &self.sanadi
    }

    /// The prefixes to use with the dhatu.
    pub fn prefixes(&self) -> &Vec<String> {
        &self.prefixes
    }
}

impl Dhatu {
    /// Creates a new dhatu with its gana.
    ///
    /// `upadesha` refers to the dhatu in its *aupadeshka* form. `upadesha` should be an SLP1
    /// string and include any necessary svaras. For example values, see the `dhatupatha.tsv` file
    /// included with this crate.
    ///
    /// `gana` refers to one of the ten major verb classes.
    ///
    /// For more customization, use the `builder()` API instead.
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::args::*;
    /// let bhu = Dhatu::mula("BU", Gana::Bhvadi);
    /// let kr = Dhatu::mula("qukf\\Y", Gana::Tanadi);
    /// ```
    pub fn mula(upadesha: &str, gana: Gana) -> Self {
        Self::Mula(Muladhatu::new(upadesha, gana))
    }

    /// Creates a new namadhatu with its sanadi-pratyaya.
    ///
    /// If `sanadi` is `None`, the program will try finding a sanAdi match by appling the
    /// rules in 3.1. If no match is found, the prakriya will abort.
    ///
    /// ### Example
    ///
    /// With an explicit `Sanadi` pratyaya:
    ///
    /// ```
    /// # use vidyut_prakriya::args::*;
    /// let putriya = Dhatu::nama(Pratipadika::basic("putra"), Some(Sanadi::kyac));
    /// let putrakamya = Dhatu::nama(Pratipadika::basic("putra"), Some(Sanadi::kAmyac));
    /// ````
    ///
    /// With an implicit `Sanadi` pratyaya:
    ///
    /// ```
    /// # use vidyut_prakriya::args::*;
    /// let lohitaya = Dhatu::nama(Pratipadika::basic("lohita"), None);
    /// ````
    pub fn nama(subanta: Pratipadika, sanadi: Option<Sanadi>) -> Self {
        let sanadi = match sanadi {
            Some(x) => vec![x],
            None => Vec::new(),
        };
        Self::Nama(Namadhatu {
            pratipadika: subanta,
            sanadi,
            prefixes: Vec::new(),
        })
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> DhatuBuilder {
        DhatuBuilder::default()
    }

    /// The sanAdi pratyayas to use with this dhatu.
    pub fn sanadi(&self) -> &Vec<Sanadi> {
        match self {
            Self::Mula(m) => m.sanadi(),
            Self::Nama(n) => n.sanadi(),
        }
    }

    /// The prefixes to use with the dhatu.
    pub fn prefixes(&self) -> &Vec<String> {
        match self {
            Self::Mula(m) => m.prefixes(),
            Self::Nama(n) => n.prefixes(),
        }
    }

    /// The upadesha to use with this dhatu, if defined.
    pub fn upadesha(&self) -> Option<&String> {
        match self {
            Self::Mula(m) => Some(m.upadesha()),
            _ => None,
        }
    }

    /// The gana to use with this dhatu, if defined.
    pub fn gana(&self) -> Option<Gana> {
        match self {
            Self::Mula(m) => Some(m.gana()),
            _ => None,
        }
    }

    /// The antargana to use with this dhatu, if defined.
    pub fn antargana(&self) -> Option<Antargana> {
        match self {
            Self::Mula(m) => m.antargana(),
            _ => None,
        }
    }

    /// Sets the prefixes on the dhatu.
    pub fn with_prefixes(mut self, values: &[impl AsRef<str>]) -> Self {
        match self {
            Self::Mula(ref mut m) => {
                m.prefixes.clear();
                m.prefixes
                    .extend(values.iter().map(|x| String::from(x.as_ref())));
            }
            Self::Nama(ref mut n) => {
                n.prefixes.clear();
                n.prefixes
                    .extend(values.iter().map(|x| String::from(x.as_ref())));
            }
        }
        self
    }

    /// Sets the sanadi-pratyayas on the dhatu.
    pub fn with_sanadi(mut self, sanadi: &[Sanadi]) -> Self {
        match self {
            Self::Mula(ref mut m) => {
                m.sanadi.clear();
                m.sanadi.extend(sanadi);
            }
            Self::Nama(ref mut n) => {
                n.sanadi.clear();
                n.sanadi.extend(sanadi);
            }
        }
        self
    }
}

/// Convenience struct for building a `Dhatu` object.
#[derive(Clone, Default, Hash, Eq, PartialEq)]
pub struct DhatuBuilder {
    upadesha: Option<String>,
    gana: Option<Gana>,
    antargana: Option<Antargana>,
    sanadi: Vec<Sanadi>,
    prefixes: Vec<String>,
}

impl DhatuBuilder {
    /// Sets the aupadeshika form of the dhatu.
    ///
    /// - For mula dhatus, this should be the dhatu as listed in the Dhatupatha, including svaras.
    /// - For namadhatus, this should be the text of the pratipadika.
    pub fn upadesha(mut self, text: &str) -> Self {
        self.upadesha = Some(String::from(text));
        self
    }

    /// Sets the gana of the dhatu.
    pub fn gana(mut self, gana: Gana) -> Self {
        self.gana = Some(gana);
        self
    }

    /// Sets whether or not this
    pub fn antargana(mut self, value: Antargana) -> Self {
        self.antargana = Some(value);
        self
    }

    /// Sets the prefixes to use with the dhatu.
    pub fn prefixes(mut self, values: &[impl AsRef<str>]) -> Self {
        self.prefixes.clear();
        self.prefixes
            .extend(values.iter().map(|x| String::from(x.as_ref())));
        self
    }

    /// Sets the `sanAdi` pratyaya to add to the dhatu.
    pub fn sanadi(mut self, values: &[Sanadi]) -> Self {
        self.sanadi.clear();
        self.sanadi.extend(values);
        self
    }

    /// Converts the arguments in this builder into a `Dhatu` struct.
    pub fn build(self) -> Result<Dhatu, Error> {
        Ok(Dhatu::Mula(Muladhatu {
            upadesha: match self.upadesha {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("upadesha")),
            },
            gana: match self.gana {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("gana")),
            },
            antargana: self.antargana,
            sanadi: self.sanadi,
            prefixes: self.prefixes,
        }))
    }
}
