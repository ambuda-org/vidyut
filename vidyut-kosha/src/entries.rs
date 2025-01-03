//! Models the entries stored in the kosha.
//!
//! All entries make heavy use of lifetime annotations to refer to data defined on `Kosha`.
use crate::errors::{Error, Result};
use vidyut_prakriya::args as vp;
use vidyut_prakriya::args::{
    BasicPratipadika, Dhatu, Krdanta, Krt, Lakara, Linga, Pada, Pratipadika, Prayoga, Purusha,
    Subanta, Tinanta, Vacana, Vibhakti,
};

/// A dhatu with its metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DhatuEntry<'a> {
    dhatu: &'a Dhatu,
    clean_text: &'a str,
}

/// A basic *prātipadika* with its metadata.
///
/// Prefer working with `PratipadikaEntry` instead.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct BasicPratipadikaEntry<'a> {
    pratipadika: &'a BasicPratipadika,
    lingas: &'a [Linga],
}

/// A *kṛdanta* with its metadata.
///
/// Prefer working with `PratipadikaEntry` instead.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct KrdantaEntry<'a> {
    dhatu_entry: DhatuEntry<'a>,
    krt: Krt,
    prayoga: Option<Prayoga>,
    lakara: Option<Lakara>,
}

/// A *prātipadika* with its metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum PratipadikaEntry<'a> {
    /// A basic *prātipadika*.
    Basic(BasicPratipadikaEntry<'a>),
    /// A *kṛdanta prātipadika*.
    Krdanta(KrdantaEntry<'a>),
}

/// A *subanta* (nominal) entry.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct SubantaEntry<'a> {
    pratipadika_entry: PratipadikaEntry<'a>,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

/// A *tinanta* (verb) entry.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct TinantaEntry<'a> {
    dhatu_entry: DhatuEntry<'a>,
    prayoga: Prayoga,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
}

/// A Sanskrit *pada* (word).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum PadaEntry<'a> {
    /// Unknown data.
    Unknown,
    /// A *subanta* that is not an *avyaya*.
    Subanta(SubantaEntry<'a>),
    /// A *subanta* that is also an *avyaya*.
    Avyaya(SubantaEntry<'a>),
    /// A *tiṅanta* (verb).
    Tinanta(TinantaEntry<'a>),
}

impl<'a> DhatuEntry<'a> {
    /// Creates a new `DhatuEntry`.
    ///
    /// `readable` should be the text obtained by calling `Vyakarana::derive_dhatus` on `dhatu`.
    pub(crate) fn new(dhatu: &'a Dhatu, clean_text: &'a str) -> Self {
        Self { dhatu, clean_text }
    }

    /// Returns the dhatu that generates this entry.
    pub fn dhatu(&self) -> &Dhatu {
        self.dhatu
    }

    /// Returns the human-readable text representation of this dhatu.
    ///
    /// Examples:
    ///
    /// - `qukf\\Y` --> `kf`
    /// - `vidi~` --> `vind`
    pub fn clean_text(&self) -> &str {
        self.clean_text
    }
}

impl<'a> From<&'a Dhatu> for DhatuEntry<'a> {
    fn from(val: &'a Dhatu) -> Self {
        DhatuEntry::new(val, "")
    }
}

impl<'a> From<DhatuEntry<'a>> for Dhatu {
    fn from(val: DhatuEntry<'a>) -> Self {
        val.dhatu.clone()
    }
}

impl<'a> BasicPratipadikaEntry<'a> {
    /// Returns the *prātipadika* that generates this entry.
    pub fn pratipadika(&self) -> &BasicPratipadika {
        self.pratipadika
    }

    /// Returns the lingas that this *prātipadika* is allowed to use.
    ///
    /// If empty, the *prātipadika* has no specific linga.
    pub fn lingas(&self) -> &[Linga] {
        self.lingas
    }
}

impl<'a> KrdantaEntry<'a> {
    /// Creates a new `KrdantaEntry`.
    pub fn new(
        dhatu_entry: DhatuEntry<'a>,
        krt: Krt,
        prayoga: Option<Prayoga>,
        lakara: Option<Lakara>,
    ) -> Self {
        Self {
            dhatu_entry,
            krt,
            prayoga,
            lakara,
        }
    }

    /// The *dhātu* used by this krdanta.
    pub fn dhatu(&self) -> &Dhatu {
        self.dhatu_entry.dhatu
    }

    /// The *dhātu* used by this krdanta.
    pub fn dhatu_text(&self) -> &str {
        self.dhatu_entry.clean_text
    }

    /// Returns the *krt pratyaya* used by this krdanta.
    pub fn krt(&self) -> Krt {
        self.krt
    }

    /// Returns the *prayoga* used by this krdanta.
    pub fn prayoga(&self) -> Option<Prayoga> {
        self.prayoga
    }

    /// Returns the *lakara* used by this krdanta.
    pub fn lakara(&self) -> Option<Lakara> {
        self.lakara
    }
}

impl<'a> From<KrdantaEntry<'a>> for Krdanta {
    fn from(val: KrdantaEntry<'a>) -> Krdanta {
        (&val).into()
    }
}

impl<'a> From<&KrdantaEntry<'a>> for Krdanta {
    fn from(val: &KrdantaEntry<'a>) -> Krdanta {
        let mut builder = vp::Krdanta::builder()
            .dhatu(val.dhatu_entry.dhatu.clone())
            .krt(val.krt);
        if let Some(prayoga) = val.prayoga {
            builder = builder.prayoga(prayoga);
        }
        if let Some(lakara) = val.lakara {
            builder = builder.lakara(lakara);
        }

        builder.build().expect("validated")
    }
}

impl<'a> PratipadikaEntry<'a> {
    /// Creates a new `PratipadikaEntry`.
    pub fn basic(pratipadika: &'a BasicPratipadika, lingas: &'a [Linga]) -> Self {
        Self::Basic(BasicPratipadikaEntry {
            pratipadika,
            lingas,
        })
    }

    /// Returns the lemma that corresponds to this *prātipadika*.
    pub fn lemma(&self) -> &str {
        match self {
            PratipadikaEntry::Basic(b) => b.pratipadika().text(),
            PratipadikaEntry::Krdanta(k) => k.dhatu_text(),
        }
    }
}

impl<'a> From<PratipadikaEntry<'a>> for Pratipadika {
    fn from(val: PratipadikaEntry<'a>) -> Pratipadika {
        (&val).into()
    }
}

impl<'a> From<&PratipadikaEntry<'a>> for Pratipadika {
    fn from(val: &PratipadikaEntry<'a>) -> Pratipadika {
        match val {
            PratipadikaEntry::Basic(b) => Pratipadika::Basic(b.pratipadika.clone()),
            PratipadikaEntry::Krdanta(k) => Pratipadika::Krdanta(Krdanta::from(k).into()),
        }
    }
}

impl<'a> TryFrom<&'a Pratipadika> for PratipadikaEntry<'a> {
    type Error = Error;

    fn try_from(val: &'a Pratipadika) -> Result<PratipadikaEntry<'a>> {
        match val {
            Pratipadika::Basic(b) => {
                let basic_entry = BasicPratipadikaEntry {
                    pratipadika: b,
                    lingas: &[],
                };
                Ok(PratipadikaEntry::Basic(basic_entry))
            }
            Pratipadika::Krdanta(k) => {
                let dhatu_entry = DhatuEntry::new(k.dhatu(), "");
                let krdanta_entry =
                    KrdantaEntry::new(dhatu_entry, k.krt(), k.prayoga(), k.lakara());
                Ok(PratipadikaEntry::Krdanta(krdanta_entry))
            }
            _ => Err(Error::Generic("Unsupported pratipadika type".to_string())),
        }
    }
}

impl<'a> SubantaEntry<'a> {
    /// Creates a new `SubantaEntry`.
    pub fn new(
        pratipadika_entry: PratipadikaEntry<'a>,
        linga: Linga,
        vibhakti: Vibhakti,
        vacana: Vacana,
    ) -> Self {
        Self {
            pratipadika_entry,
            linga,
            vibhakti,
            vacana,
        }
    }

    /// Creates a new `SubantaEntry` that is an *avyaya*.
    pub fn avyaya(pratipadika_entry: PratipadikaEntry<'a>) -> Self {
        Self {
            pratipadika_entry,
            linga: Linga::Pum,
            vibhakti: Vibhakti::Prathama,
            vacana: Vacana::Eka,
        }
    }

    /// The `PratipadikaEntry` that generates this `SubantaEntry`.
    pub fn pratipadika_entry(&self) -> &PratipadikaEntry {
        &self.pratipadika_entry
    }

    /// The *liṅga* used by this *subanta*.
    pub fn linga(&self) -> Linga {
        self.linga
    }

    /// The *vibhakti* used by this *subanta*.
    pub fn vibhakti(&self) -> Vibhakti {
        self.vibhakti
    }

    /// The *vacana* used by this *subanta*.
    pub fn vacana(&self) -> Vacana {
        self.vacana
    }
}

impl<'a> From<SubantaEntry<'a>> for vp::Subanta {
    fn from(val: SubantaEntry) -> Self {
        (&val).into()
    }
}

impl<'a> From<&SubantaEntry<'a>> for vp::Subanta {
    fn from(val: &SubantaEntry) -> Self {
        Subanta::new(
            vp::Pratipadika::from(&val.pratipadika_entry),
            val.linga,
            val.vibhakti,
            val.vacana,
        )
    }
}

impl<'a> TryFrom<&'a vp::Subanta> for SubantaEntry<'a> {
    type Error = Error;

    fn try_from(val: &'a Subanta) -> Result<Self> {
        Ok(SubantaEntry::new(
            val.pratipadika().try_into()?,
            val.linga(),
            val.vibhakti(),
            val.vacana(),
        ))
    }
}

impl<'a> TinantaEntry<'a> {
    /// Creates a new `TinantaEntry`.
    pub fn new(
        dhatu_entry: DhatuEntry<'a>,
        prayoga: Prayoga,
        lakara: Lakara,
        purusha: Purusha,
        vacana: Vacana,
    ) -> Self {
        Self {
            dhatu_entry,
            prayoga,
            lakara,
            purusha,
            vacana,
        }
    }

    /// The *dhātu* used by this *tiṅanta*.
    pub fn dhatu(&self) -> &Dhatu {
        self.dhatu_entry.dhatu()
    }

    /// The clean text of this *tiṅanta*'s *dhātu*.
    pub fn dhatu_text(&self) -> &str {
        self.dhatu_entry.clean_text()
    }

    /// The *prayoga* used by this *tiṅanta*.
    pub fn prayoga(&self) -> Prayoga {
        self.prayoga
    }

    /// The *lakāra* used by this *tiṅanta*.
    pub fn lakara(&self) -> Lakara {
        self.lakara
    }

    /// The *puruṣa* used by this *tiṅanta*.
    pub fn purusha(&self) -> Purusha {
        self.purusha
    }

    /// The *vacana* used by this *tiṅanta*.
    pub fn vacana(&self) -> Vacana {
        self.vacana
    }
}

impl<'a> From<&'a TinantaEntry<'a>> for vp::Tinanta {
    fn from(val: &TinantaEntry) -> Self {
        vp::Tinanta::new(
            val.dhatu_entry.dhatu.clone(),
            val.prayoga,
            val.lakara,
            val.purusha,
            val.vacana,
        )
    }
}

impl<'a> From<TinantaEntry<'a>> for vp::Tinanta {
    fn from(val: TinantaEntry) -> Self {
        vp::Tinanta::new(
            val.dhatu_entry.dhatu.clone(),
            val.prayoga,
            val.lakara,
            val.purusha,
            val.vacana,
        )
    }
}

impl<'a> From<&'a vp::Tinanta> for TinantaEntry<'a> {
    fn from(val: &'a Tinanta) -> Self {
        TinantaEntry::new(
            val.dhatu().into(),
            val.prayoga(),
            val.lakara(),
            val.purusha(),
            val.vacana(),
        )
    }
}

impl<'a> PadaEntry<'a> {
    /// TODO
    pub fn lemma(&self) -> Option<&str> {
        match self {
            PadaEntry::Unknown => None,
            PadaEntry::Tinanta(t) => Some(t.dhatu_text()),
            PadaEntry::Subanta(s) | PadaEntry::Avyaya(s) => Some(s.pratipadika_entry.lemma()),
        }
    }
}

impl<'a> TryFrom<&'a Pada> for PadaEntry<'a> {
    type Error = Error;

    fn try_from(val: &'a Pada) -> Result<PadaEntry<'a>> {
        match val {
            Pada::Subanta(s) => {
                let entry: SubantaEntry = s.try_into()?;
                Ok(entry.into())
            }
            Pada::Tinanta(t) => {
                let entry: TinantaEntry = t.into();
                Ok(entry.into())
            }
            _ => Err(Error::Generic("Unsupported pada type".to_string())),
        }
    }
}

impl<'a> From<SubantaEntry<'a>> for PadaEntry<'a> {
    fn from(val: SubantaEntry<'a>) -> Self {
        PadaEntry::Subanta(val)
    }
}

impl<'a> From<TinantaEntry<'a>> for PadaEntry<'a> {
    fn from(val: TinantaEntry<'a>) -> Self {
        PadaEntry::Tinanta(val)
    }
}
