use crate::args::Pratipadika;
use crate::args::Slp1String;
use crate::core::errors::{Error, Result};
use crate::enum_boilerplate;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Defines a *gaṇa*.
///
/// The dhatus in the Dhatupatha are organized in ten large *gaṇa*s or classes. These gaṇas
/// add various properties to the dhatu, most notably the specific *vikaraṇa* (stem suffix) we use
/// before *sārvadhātuka* suffixes.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// The seventh gaṇa, whose first dhatu is `ruD`.
    Rudhadi,
    /// The eighth gaṇa, whose first dhatu is `tan`.
    Tanadi,
    /// The ninth gaṇa, whose first dhatu is `krI`.
    Kryadi,
    /// The tenth gaṇa, whose first dhatu is `cur`.
    Curadi,
    /// The kandvAdi gaṇa, whose first dhatu is `kaRqU`.
    Kandvadi,
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
    Kandvadi => "kandu",
});

/// Defines an *antargaṇa*.
///
/// The dhatus in the Dhatupatha are organized in ten large *gaṇa*s or classes. Within these larger
/// *gaṇa*s, certain *antargaṇa*s or subclasses have extra properties that affect the derivations
/// they produce. For example, dhatus in the *kuṭādi antargaṇa* generally do not allow *guṇa* vowel
/// changes.
///
/// Since most dhatus appear exactly once per *gaṇa*, this crate can usually infer whether a dhatu
/// is in a specific *antargaṇa*. However, some *gaṇa*s have dhatus that repeat, and these
/// repeating dhatus cause ambiguities for our code. (For example, `juqa~` appears twice in
/// *tudādigaṇa*: once in *kuṭādi* and once outside of it.)
///
/// To avoid this ambiguity, we require that certain *antargaṇa*s are declared up-front.
///
/// (Can we disambiguate by checking the dhatu's index within its gana? Unfortunately, no. There is
/// no canonical version of the Dhatupatha, and we cannot expect that a dhatu's index is consistent
/// across all of these versions. So we thought it better to avoid hard-coding indices or requiring
/// callers to follow our specific conventions.)
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Antargana {
    /// *Antargaṇa* of *bhū* gana. A dhatu in this *antargaṇa* uses a shortened vowel when
    /// followed by *ṇic-pratyaya*.
    Ghatadi,
    /// *Antargaṇa* of *tud* gana. Pratyayas that follow dhatus in *kuṭādi-gaṇa* will generally be
    /// marked `Nit` per 1.2.1. Required because of duplicates like `juqa~`.
    Kutadi,
    /// *Antargaṇa* of *cur* gana ending with `zvada~` / `svAda~`. A dhatu in this *antargaṇa*
    /// optionaly uses *ṇic-pratyaya* when taking an object. Required because of duplicates like
    /// `tuji~`.
    Asvadiya,
    /// *Antargaṇa* of *cur* gana ending with `Dfza~`. A dhatu in this *antargaṇa* optionally uses
    /// *ṇic-pratyaya*. Required because of duplicates like `SraTa~`.
    Adhrshiya,
    /// *Antargaṇa* of *cur* gana ending with `kusma~`. A dhatu in this *antargaṇa* is always
    /// *ātmanepadī*. Required because of duplicates like `daSi~`.
    Akusmiya,
}

enum_boilerplate!(Antargana, {
    Ghatadi => "GawAdi",
    Kutadi => "kuwAdi",
    Akusmiya => "AkusmIya",
    Asvadiya => "AsvadIya",
    Adhrshiya => "ADfzIya",
});

/// A *sanādi pratyaya*.
///
/// The *sanādi pratyaya*s create new *dhātu*s per 3.1.32. They are introduced in rules 3.1.7 -
/// 3.1.30, and since rule 3.1.7 contains the word "dhAtoH", they can be called *ārdhadhātuka* by
/// 3.4.114.
///
/// Any *sanādi pratyaya*s not listed here are required by certain sutras and added by default.
///
/// For details on what these pratyayas mean and what kinds of words they produce, see the comments
/// below.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wasm_bindgen]
pub enum Sanadi {
    /// `kAmyac`, which creates nAma-dhAtus per 3.1.9.
    ///
    /// Examples: `putrakAmyati`
    kAmyac,

    /// `kyaN`, which creates nAma-dhAtus per 3.1.11.
    ///
    /// Examples: `SyenAyate`, `BfSAyate`
    kyaN,

    /// `kyac`, which creates nAma-dhAtus per 3.1.8.
    ///
    /// Examples: `putrIyati`
    kyac,

    /// `Nic`, which creates causal roots per 3.1.26.
    ///
    /// Examples: `BAvayati`, `nAyayati`.
    Ric,

    /// `yaN`, which creates intensive roots per 3.1.22. For certain dhatus, the semantics are
    /// instead "crooked movement" (by 3.1.23) or "contemptible" action (by 3.1.24).
    ///
    /// Examples: boBUyate, nenIyate.
    ///
    /// Constraints: can be used only if the dhatu starts with a consonant and has exactly one
    /// vowel. If this constraint is violated, our APIs will return an `Error`.
    yaN,

    /// `yaN`, with lopa per 2.4.74. This is often listed separately due to its rarity and its
    /// very different form.
    ///
    /// Examples: boBavIti, boBoti, nenayIti, neneti.
    yaNluk,

    /// `san`, which creates desiderative roots per 3.1.7.
    ///
    /// Examples: buBUzati, ninIzati.
    san,
}

impl Sanadi {
    /// Returns whether this *pratyaya* can be added only after subantas.
    pub fn is_namadhatu(&self) -> bool {
        use Sanadi::*;
        matches!(self, kAmyac | kyaN | kyac)
    }

    /// Returns the *aupadeśika* form of this *pratyaya*.
    pub(crate) fn aupadeshika(&self) -> &'static str {
        self.as_str()
    }
}

enum_boilerplate!(Sanadi, {
    kAmyac => "kAmyac",
    kyaN => "kyaN",
    kyac => "kyac",
    Ric => "Ric",
    yaN => "yaN",
    yaNluk => "yaNluk",
    san => "san",
});

/// A *dhātu* from the Dhatupatha.
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Muladhatu {
    aupadeshika: Slp1String,
    gana: Gana,
    antargana: Option<Antargana>,
    sanadi: Vec<Sanadi>,
    prefixes: Vec<String>,
}

impl Muladhatu {
    /// Creates a new *mūla-dhātu*.
    pub fn new(aupadeshika: Slp1String, gana: Gana) -> Self {
        Self {
            aupadeshika,
            gana,
            antargana: None,
            sanadi: Vec::new(),
            prefixes: Vec::new(),
        }
    }
    /// The dhatu as stated in its *aupadeśika* form. `upadesha` should be an SLP1 string that
    /// includes any necessary svaras. For examples, see the `dhatu` column in the
    /// `data/dhatupatha.tsv` file included in this crate.
    pub fn aupadeshika(&self) -> &str {
        &self.aupadeshika.0
    }

    /// The dhatu's *gaṇa*.
    pub fn gana(&self) -> Gana {
        self.gana
    }

    /// The *antargaṇa* this dhatu belongs to.
    pub fn antargana(&self) -> Option<Antargana> {
        self.antargana
    }

    /// The *sanādi pratyaya*s to use with this *dhātu*.
    pub fn sanadi(&self) -> &[Sanadi] {
        &self.sanadi
    }

    /// The prefixes to use with this .
    pub fn prefixes(&self) -> &[String] {
        &self.prefixes
    }

    /// Returns whether the dhatu has the given gana.
    pub fn has_gana(&self, gana: impl Into<Gana>) -> bool {
        self.gana == gana.into()
    }

    /// Sets the *antargaṇa* to use with this *dhātu*.
    pub fn with_antargana(mut self, antargana: Antargana) -> Self {
        self.antargana = Some(antargana);
        self
    }
}

/// A *dhātu* created from a *subanta*.
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Namadhatu {
    pratipadika: Pratipadika,
    nama_sanadi: Option<Sanadi>,
    other_sanadi: Vec<Sanadi>,
    pub(crate) prefixes: Vec<String>,
}

impl Namadhatu {
    /// The *prātipadika* to use as the basis of this *dhātu*.
    pub fn pratipadika(&self) -> &Pratipadika {
        &self.pratipadika
    }

    /// The main *sanādi pratyaya*s to use with this *dhātu*.
    pub fn nama_sanadi(&self) -> &Option<Sanadi> {
        &self.nama_sanadi
    }

    /// Any other *sanādi pratyaya*s to use after `nama_sanadi`.
    pub fn other_sanadi(&self) -> &[Sanadi] {
        &self.other_sanadi
    }

    /// The prefixes to use with the *dhātu*.
    pub fn prefixes(&self) -> &[String] {
        &self.prefixes
    }
}

/// Models the verb root to use in some derivation.
///
///
/// ### Model
///
/// A `Dhatu` is an enum with two variants:
///
/// - `Mula` holds a `MulaDhatu` from the Dhatupatha.
/// - `Nama` holds a NamaDhatu` that combines a subanta with a *sanādi pratyaya*. For simplicity, our
///   API uses `Pratipadika`s instead of `Subanta`s.
///
/// A `Dhatu` may optionally have:
///
/// - one or more prefixes. We currently support only upasargas and basic gati-prefixes. In the
///   future, we hope to also support cvi- and DAc-prefixes.
/// - one or more *sanādi pratyaya*s. For details, see `Sanadi`.
///
///
/// ### Usage
///
/// `Dhatu` is an argument to `Tinanta` and `Krdanta`. You may also pass it to
/// `Vyakarana::derive_tinantas`, which shows how the dhatu will appear after applying the normal
/// operations of the grammar (satva, natva, num-Agama-adesha, sandhi, guna, etc.)
///
/// Some dhatus are used only with specific upasargas, and others appear in specific antarganas.
/// Our `Dhatupatha` struct provides a simple API that manages these upasargas and antarganas for
/// you automatically, and we encourage you to use it. For example usage, see the `examples`
/// directory of this crate.
///
/// ### Examples
///
/// A *mūla-dhātu* in a specific gana:
///
/// ```
/// # use vidyut_prakriya::*;
/// use vidyut_prakriya::args::*;
///
/// let slp = Slp1String::from;
///
/// let bhu = Dhatu::mula(slp("BU")?, Gana::Bhvadi);
/// let ad = Dhatu::mula(slp("a\\da~")?, Gana::Adadi);
/// let hu = Dhatu::mula(slp("hu\\")?, Gana::Juhotyadi);
/// let div = Dhatu::mula(slp("divu~")?, Gana::Divadi);
/// let su = Dhatu::mula(slp("zu\\Y")?, Gana::Svadi);
/// let tud = Dhatu::mula(slp("tu\\da~^")?, Gana::Tudadi);
/// let rudh = Dhatu::mula(slp("ru\\Di~^r")?, Gana::Rudhadi);
/// let tan = Dhatu::mula(slp("tanu~^")?, Gana::Tanadi);
/// let kri = Dhatu::mula(slp("qukrI\\Y")?, Gana::Kryadi);
/// let cur = Dhatu::mula(slp("cura~")?, Gana::Curadi);
/// # Ok::<(), Error>(())
/// ```
///
/// A *mūla-dhātu* in a specific *gaṇa* and *antargaṇa*:
///
/// ```
/// # use vidyut_prakriya::*;
/// # use vidyut_prakriya::args::*;
/// # let slp = Slp1String::from;
///
/// let kut = Dhatu::mula_with_antargana(slp("kuwa~")?, Gana::Tudadi, Antargana::Kutadi);
/// # Ok::<(), Error>(())
/// ````
///
/// A *mūla-dhātu* with *sanādi pratyaya*s:
///
/// ```
/// # use vidyut_prakriya::*;
/// # use vidyut_prakriya::args::*;
/// let bhu = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi);
/// let bhavi = bhu.clone().with_sanadi(&[Sanadi::Ric]);
/// let bubhusha = bhu.clone().with_sanadi(&[Sanadi::san]);
/// let bobhuya = bhu.clone().with_sanadi(&[Sanadi::yaN]);
/// let bobhu = bhu.clone().with_sanadi(&[Sanadi::yaNluk]);
/// let bibhavayiza = bhu.clone().with_sanadi(&[Sanadi::Ric, Sanadi::san]);
/// # Ok::<(), Error>(())
/// ```
///
/// A *mūla-dhātu* with prefixes:
///
/// ```
/// # use vidyut_prakriya::*;
/// # use vidyut_prakriya::args::*;
/// let bhu = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi);
/// let prabhu = bhu.clone().with_prefixes(&["pra"]);
/// let pratisambhu = bhu.clone().with_prefixes(&["prati", "sam"]);
/// # Ok::<(), Error>(())
/// ```
///
/// A *mūla-dhātu* with both prefixes and *sanādi pratyaya*s:
///
/// ```
/// # use vidyut_prakriya::*;
/// # use vidyut_prakriya::args::*;
/// # let slp = Slp1String::from;
/// let bhu = Dhatu::mula(slp("BU")?, Gana::Bhvadi);
/// let bhu = Dhatu::mula(slp("BU")?, Gana::Bhvadi);
/// let pratisambibhavayiza = bhu.clone()
///     .with_prefixes(&["prati", "sam"])
///     .with_sanadi(&[Sanadi::Ric, Sanadi::san]);
/// # Ok::<(), Error>(())
/// ```
///
/// A *nāma-dhātu* with an optional *sanādi pratyaya*:
///
/// ```
/// # use vidyut_prakriya::*;
/// # use vidyut_prakriya::args::*;
/// let putra = Pratipadika::basic(Slp1String::from("putra")?);
/// let putriya = Dhatu::nama(putra, Some(Sanadi::kyac));
/// # Ok::<(), Error>(())
/// ```
///
/// A *nāma-dhātu* with a mandatory *sanādi pratyaya* from some other sutra:
///
/// ```
/// # use vidyut_prakriya::*;
/// # use vidyut_prakriya::args::*;
/// let lohita = Pratipadika::basic(Slp1String::from("lohita")?);
/// // "kyaN" will be added by sutra 3.1.13.
/// let lohitaya = Dhatu::nama(lohita, None);
/// # Ok::<(), Error>(())
/// ````
///
/// let bhu = Dhatu::mula("BU", Gana::Bhvadi);
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum Dhatu {
    /// Indicates a muladhAtu from the Dhatupatha.
    Mula(Muladhatu),
    /// Indicates a nAma-dhAtu created with a *sanādi pratyaya*.
    Nama(Namadhatu),
}

impl Dhatu {
    /// Creates a new *dhātu* with its gana.
    ///
    /// `upadesha` refers to the dhatu in its *aupadeśika* form. `upadesha` should be an SLP1
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
    /// # use vidyut_prakriya::*;
    /// use vidyut_prakriya::args::*;
    ///
    /// let bhu = Dhatu::mula(Slp1String::from("BU")?, Gana::Bhvadi);
    /// let kr = Dhatu::mula(Slp1String::from("qukf\\Y")?, Gana::Tanadi);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn mula(aupadeshika: Slp1String, gana: Gana) -> Self {
        Self::Mula(Muladhatu::new(aupadeshika, gana))
    }

    /// Creates a new dhatu with its gana and antargana.
    ///
    /// An *antargana* is a subsection of a larger gana. Generally, dhatus in the same antargana
    /// share specific properties. For example, the *kuṭādi* antargana generally does not change
    /// its vowel to guna.
    ///
    /// For more customization, use the `builder()` API instead.
    ///
    /// ### Example
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// use vidyut_prakriya::args::*;
    ///
    /// let kut = Dhatu::mula_with_antargana(Slp1String::from("kuwa~")?, Gana::Tudadi, Antargana::Kutadi);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn mula_with_antargana(aupadeshika: Slp1String, gana: Gana, antargana: Antargana) -> Self {
        Self::Mula(Muladhatu::new(aupadeshika, gana).with_antargana(antargana))
    }

    /// Creates a new *nāmadhātu* with its *sanādi pratyaya*.
    ///
    /// If `sanadi` is `None`, the program will try finding a *sanādi* match by appling the
    /// rules in 3.1. If no match is found, the prakriya will abort.
    ///
    /// ### Example
    ///
    /// With an explicit `Sanadi` pratyaya:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// use vidyut_prakriya::args::*;
    ///
    /// let putriya = Dhatu::nama(
    ///     Pratipadika::basic(Slp1String::from("putra")?),
    ///     Some(Sanadi::kyac));
    /// let putrakamya = Dhatu::nama(
    ///     Pratipadika::basic(Slp1String::from("putra")?),
    ///     Some(Sanadi::kAmyac));
    /// # Ok::<(), Error>(())
    /// ````
    ///
    /// With an implicit `Sanadi` pratyaya:
    ///
    /// ```
    /// # use vidyut_prakriya::*;
    /// # use vidyut_prakriya::args::*;
    /// let lohitaya = Dhatu::nama(Pratipadika::basic(Slp1String::from("lohita")?), None);
    /// # Ok::<(), Error>(())
    /// ````
    pub fn nama(subanta: Pratipadika, nama_sanadi: Option<Sanadi>) -> Self {
        Self::Nama(Namadhatu {
            pratipadika: subanta,
            nama_sanadi,
            other_sanadi: Vec::new(),
            prefixes: Vec::new(),
        })
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> DhatuBuilder {
        DhatuBuilder::default()
    }

    /// The *sanādi pratyaya*s to use with this dhatu.
    pub fn sanadi(&self) -> &[Sanadi] {
        match self {
            Self::Mula(m) => m.sanadi(),
            Self::Nama(n) => n.other_sanadi(),
        }
    }

    /// The prefixes to use with the dhatu.
    pub fn prefixes(&self) -> &[String] {
        match self {
            Self::Mula(m) => m.prefixes(),
            Self::Nama(n) => n.prefixes(),
        }
    }

    /// The aupadeshika text for this dhatu, if defined.
    pub fn aupadeshika(&self) -> Option<&str> {
        match self {
            Self::Mula(m) => Some(m.aupadeshika()),
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

    /// Sets the *sanādi pratyaya*s on the dhatu.
    pub fn with_sanadi(mut self, sanadi: &[Sanadi]) -> Self {
        match self {
            Self::Mula(ref mut m) => {
                m.sanadi.clear();
                m.sanadi.extend(sanadi);
            }
            Self::Nama(ref mut n) => {
                n.other_sanadi.clear();
                n.other_sanadi.extend(sanadi);
            }
        }
        self
    }
}

/// Convenience struct for building a `Dhatu` object.
#[derive(Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DhatuBuilder {
    aupadeshika: Option<String>,
    gana: Option<Gana>,
    antargana: Option<Antargana>,
    sanadi: Vec<Sanadi>,
    prefixes: Vec<String>,
}

impl DhatuBuilder {
    /// Sets the *aupadeśika* form of the dhatu.
    ///
    /// - For mula dhatus, this should be the dhatu as listed in the Dhatupatha, including svaras.
    /// - For namadhatus, this should be the text of the pratipadika.
    pub fn aupadeshika(mut self, text: &str) -> Self {
        self.aupadeshika = Some(String::from(text));
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

    /// Sets the *sanādi pratyaya* to add to the dhatu.
    pub fn sanadi(mut self, values: &[Sanadi]) -> Self {
        self.sanadi.clear();
        self.sanadi.extend(values);
        self
    }

    /// Converts the arguments in this builder into a `Dhatu` struct.
    pub fn build(self) -> Result<Dhatu> {
        Ok(Dhatu::Mula(Muladhatu {
            aupadeshika: match self.aupadeshika {
                Some(x) => Slp1String::from(x)?,
                _ => return Err(Error::missing_required_field("aupadeshika")),
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
