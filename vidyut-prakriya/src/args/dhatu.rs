use crate::enum_boilerplate;
use crate::errors::Error;
use wasm_bindgen::prelude::wasm_bindgen;

/// Defines a gaṇa.
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

impl Gana {
    /// Parses the given integer as a `Gana`.
    pub fn from_int(value: u8) -> Result<Gana, Error> {
        use Gana::*;
        let ret = match value {
            1 => Bhvadi,
            2 => Adadi,
            3 => Juhotyadi,
            4 => Divadi,
            5 => Svadi,
            6 => Tudadi,
            7 => Rudhadi,
            8 => Tanadi,
            9 => Kryadi,
            10 => Curadi,
            _ => return Err(Error::gana_parse_error(value)),
        };
        Ok(ret)
    }
}

impl From<Gana> for u8 {
    fn from(value: Gana) -> u8 {
        use Gana::*;
        match value {
            Bhvadi => 1,
            Adadi => 2,
            Juhotyadi => 3,
            Divadi => 4,
            Svadi => 5,
            Tudadi => 6,
            Rudhadi => 7,
            Tanadi => 8,
            Kryadi => 9,
            Curadi => 10,
        }
    }
}

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
    /// Antargana of *cur* gana ending with `kusma~`. A dhatu in this antargana is always
    /// ātmanepadī. Required because of duplicates like `daSi~`.
    Akusmiya,
}

enum_boilerplate!(Antargana, {
    Kutadi => "kutadi",
    Akusmiya => "akusmiya",
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
#[wasm_bindgen]
pub enum Sanadi {
    /// `san`, which creates desiderative roots per 3.1.7.
    ///
    /// Examples: buBUzati, ninIzati.
    San,
    /// `yaN`, which creates intensive roots per 3.1.22. For certain dhatus, the semantics are
    /// instead "crooked movement" (by 3.1.23) or "contemptible" action (by 3.1.24).
    ///
    /// Examples: boBUyate, nenIyate.
    ///
    /// Constraints: can be used only if the dhatu starts with a consonant and has exactly one
    /// vowel. If this constraint is violated, our APIs will return an `Error`.
    Yan,
    /// `yaN`, with elision per 2.4.74. This is often listed separately due to its rarity and its
    /// very different form.
    ///
    /// Examples: boBavIti, boBoti, nenayIti, neneti.
    YanLuk,
    /// `Nic`, which creates causal roots per 3.1.26.
    ///
    /// Examples: BAvayati, nAyayati.
    Nic,
}

enum_boilerplate!(Sanadi, {
    San => "san",
    Yan => "yaN",
    YanLuk => "yaN-luk",
    Nic => "Ric",
});

/// The verb root to use for the derivation.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Dhatu {
    upadesha: String,
    gana: Gana,
    antargana: Option<Antargana>,
    sanadi: Vec<Sanadi>,
    prefixes: Vec<String>,
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
    /// # use vidyut_prakriya::args::{Dhatu, Gana};
    /// let bhu = Dhatu::new("BU", Gana::Bhvadi);
    /// let kr = Dhatu::new("qukf\\Y", Gana::Tanadi);
    /// ```
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

    /// Sets the prefixes on the dhatu.
    pub fn with_prefixes(mut self, values: &[impl AsRef<str>]) -> Self {
        self.prefixes.clear();
        self.prefixes
            .extend(values.iter().map(|x| String::from(x.as_ref())));
        self
    }

    /// Sets the sanadi-pratyayas on the dhatu.
    pub fn with_sanadi(mut self, sanadi: &[Sanadi]) -> Self {
        self.sanadi.clear();
        self.sanadi.extend(sanadi);
        self
    }

    /// Sets the sanadi-pratyayas on the dhatu.
    pub fn with_antargana(mut self, antargana: Option<Antargana>) -> Self {
        self.antargana = antargana;
        self
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> DhatuBuilder {
        DhatuBuilder::default()
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
    /// Sets the upadesha of the dhatu.
    pub fn upadesha(mut self, text: &str) -> Self {
        self.upadesha = Some(String::from(text));
        self
    }

    /// Sets the gana of the dhatu.
    pub fn gana(mut self, gana: Gana) -> Self {
        self.gana = Some(gana);
        self
    }

    /// Sets the antargana of the dhatu, if one is necessary.
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
        Ok(Dhatu {
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
        })
    }
}
