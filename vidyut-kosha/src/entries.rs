//! Models the entries stored in the kosha.
//!
//! All entries make heavy use of lifetime annotations to refer to data defined on `Kosha`.
//! To persist this data for your application, clone the specific fields you need.
use crate::errors::{Error, Result};
use serde::{Deserialize, Serialize};
use vidyut_prakriya::args as vp;
use vidyut_prakriya::args::{
    BasicPratipadika, Dhatu, Krdanta, Krt, Lakara, Linga, Pada, Pratipadika, Prayoga, Purusha,
    Subanta, Tinanta, Vacana, Vibhakti,
};

/// A dhatu with its metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DhatuEntry<'a> {
    pub(crate) dhatu: &'a Dhatu,
    pub(crate) meta: Option<&'a DhatuMeta>,
}

/// Metadata for some dhatu.
///
/// We store metadata in its own `struct` so that we avoid bloating `DhatuEntry` and the objects
/// that use `DhatuEntry`, such as `PratipadikaEntry` and `PadaEntry`.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DhatuMeta {
    pub(crate) clean_text: String,
    pub(crate) artha_sa: Option<String>,
    pub(crate) artha_hi: Option<String>,
    pub(crate) artha_en: Option<String>,
    pub(crate) karmatva: Option<String>,
    pub(crate) ittva: Option<String>,
    pub(crate) pada: Option<String>,
}

/// A builder for a `DhatuEntry`.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct DhatuMetaBuilder {
    clean_text: Option<String>,
    artha_sa: Option<String>,
    artha_hi: Option<String>,
    artha_en: Option<String>,
    karmatva: Option<String>,
    ittva: Option<String>,
    pada: Option<String>,
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
    /// A *subanta*.
    Subanta(SubantaEntry<'a>),
    /// A *tiṅanta* (verb).
    Tinanta(TinantaEntry<'a>),
}

impl<'a> DhatuEntry<'a> {
    /// Creates a new `DhatuEntry` with no metadata. To set metadata, use `builder()` instead.
    pub fn new(dhatu: &'a Dhatu) -> Self {
        Self { dhatu, meta: None }
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
        self.meta.map_or("", |x| &x.clean_text)
    }

    /// Returns the Sanskrit meaning of this dhatu's *mūla* as an SLP1 string. All of these
    /// meaning strings come directly from the Dhatupatha.
    ///
    /// We have meaning strings only for the ~2000 *mūla* dhatus from the Dhatupatha. Any roots
    /// derived from these ~2000 will share their `artha` with the dhatu they come from.
    ///
    /// Examples:
    ///
    /// - `BU` --> `sattAyAm`
    /// - `aBiBU` --> `sattAyAm`
    /// - `aBibuBUza` --> `sattAyAm`
    ///
    /// Data is sourced from <https://ashtadhyayi.com>.
    pub fn artha_sa(&self) -> Option<&str> {
        self.meta.map(|x| x.artha_sa.as_deref()).flatten()
    }

    /// Returns the English meaning of this dhatu's *mūla* in Latin text.
    ///
    /// Data is sourced from <https://ashtadhyayi.com>.
    pub fn artha_en(&self) -> Option<&str> {
        self.meta.map(|x| x.artha_en.as_deref()).flatten()
    }

    /// Returns the Hindi meaning of this dhatu's *mūla* in Devanagari.
    ///
    /// Data is sourced from <https://ashtadhyayi.com>.
    pub fn artha_hi(&self) -> Option<&str> {
        self.meta.map(|x| x.artha_hi.as_deref()).flatten()
    }

    /// Sets the metadata on this dhatu.
    ///
    /// This method is for libraries building a `Kosha` from scratch. Otherwise, prefer using
    /// the accessor methods defined on `DhatuEntry`.
    pub fn with_meta(mut self, meta: &'a DhatuMeta) -> Self {
        self.meta = Some(meta);
        self
    }
}

impl<'a> From<&'a Dhatu> for DhatuEntry<'a> {
    fn from(val: &'a Dhatu) -> Self {
        DhatuEntry::new(val)
    }
}

impl<'a> From<DhatuEntry<'a>> for Dhatu {
    fn from(val: DhatuEntry<'a>) -> Self {
        val.dhatu.clone()
    }
}

impl DhatuMeta {
    /// Returns a builder for some `DhatuMeta` struct.
    ///
    /// This builder is utility code for inserting new `DhatuEntry` objects into a `Kosha`. If you
    /// are not building a `Kosha` yourself, you can ignore this method.
    pub fn builder() -> DhatuMetaBuilder {
        DhatuMetaBuilder::default()
    }
}

impl DhatuMetaBuilder {
    /// Sets `clean_text`.
    pub fn clean_text(mut self, text: String) -> Self {
        self.clean_text = Some(text);
        self
    }

    /// (Optional) Sets `artha_sa`.
    pub fn artha_sa(mut self, artha: String) -> Self {
        self.artha_sa = Some(artha);
        self
    }

    /// (Optional) Sets `artha_en`.
    pub fn artha_en(mut self, artha: String) -> Self {
        self.artha_en = Some(artha);
        self
    }

    /// (Optional) Sets `artha_hi`.
    pub fn artha_hi(mut self, artha: String) -> Self {
        self.artha_hi = Some(artha);
        self
    }

    /// (Optional) Sets the karmatva.
    pub fn karmatva(mut self, artha: String) -> Self {
        self.artha_hi = Some(artha);
        self
    }

    /// (Optional) Sets the ittva.
    pub fn ittva(mut self, ittva: String) -> Self {
        self.ittva = Some(ittva);
        self
    }

    /// (Optional) Sets the dhatu pada.
    pub fn pada(mut self, pada: String) -> Self {
        self.pada = Some(pada);
        self
    }

    /// Builds a `DhatuMeta`.
    pub fn build(self) -> Result<DhatuMeta> {
        Ok(DhatuMeta {
            clean_text: match self.clean_text {
                Some(x) => x,
                _ => return Err(Error::UnsupportedType),
            },
            artha_sa: self.artha_sa,
            artha_en: self.artha_en,
            artha_hi: self.artha_hi,
            karmatva: self.karmatva,
            ittva: self.ittva,
            pada: self.pada,
        })
    }
}

impl<'a> BasicPratipadikaEntry<'a> {
    /// Creates a new `BasicPratipadikaEntry`.
    pub fn new(pratipadika: &'a BasicPratipadika, lingas: &'a [Linga]) -> Self {
        Self {
            pratipadika,
            lingas,
        }
    }

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

    /// Returns whether this entry represents an *avyaya*.
    pub fn is_avyaya(&self) -> bool {
        self.pratipadika.is_avyaya()
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

    /// The *dhātu* entry that corresponds to this *krdanta*'s dhātu.
    pub fn dhatu_entry(&self) -> &DhatuEntry<'a> {
        &self.dhatu_entry
    }

    /// The *dhātu* used by this krdanta.
    pub fn dhatu(&self) -> &Dhatu {
        self.dhatu_entry.dhatu
    }

    /// The *dhātu* used by this krdanta.
    pub fn dhatu_text(&self) -> &str {
        self.dhatu_entry.clean_text()
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

    /// Returns whether this entry represents an *avyaya*.
    pub fn is_avyaya(&self) -> bool {
        self.krt.is_avyaya()
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

    /// Returns the lingas that this *prātipadika* is allowed to use.
    ///
    /// If empty, lingas might not yet be implemented for this *prātipadika* type.
    pub fn lingas(&self) -> &[Linga] {
        match self {
            Self::Basic(b) => b.lingas(),
            Self::Krdanta(k) => match k.krt {
                Krt::Base(b) => b.lingas(),
                _ => &[],
            },
        }
    }

    /// Returns whether this entry represents an *avyaya*.
    pub fn is_avyaya(&self) -> bool {
        match self {
            PratipadikaEntry::Basic(b) => b.is_avyaya(),
            PratipadikaEntry::Krdanta(k) => k.is_avyaya(),
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
                let dhatu_entry = DhatuEntry::new(k.dhatu());
                let krdanta_entry =
                    KrdantaEntry::new(dhatu_entry, k.krt(), k.prayoga(), k.lakara());
                Ok(PratipadikaEntry::Krdanta(krdanta_entry))
            }
            _ => Err(Error::UnsupportedType),
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

    /// Returns whether this subanta must be the purvapada in a samasa.
    pub fn is_purvapada(&self) -> bool {
        // TODO: implement
        false
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

    /// Returns whether this entry represents an *avyaya*.
    ///
    /// This is just sugar for `self.pratipadika_entry().is_avyaya()`.
    pub fn is_avyaya(&self) -> bool {
        self.pratipadika_entry.is_avyaya()
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
        if val.is_avyaya() {
            Ok(SubantaEntry::avyaya(val.pratipadika().try_into()?))
        } else {
            Ok(SubantaEntry::new(
                val.pratipadika().try_into()?,
                val.linga(),
                val.vibhakti(),
                val.vacana(),
            ))
        }
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

    /// The *dhātu* entry that corresponds to this *tiṅanta*'s dhātu.
    pub fn dhatu_entry(&self) -> &DhatuEntry<'a> {
        &self.dhatu_entry
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
    /// Returns the lemma associated with this `PadaEntry`.
    pub fn lemma(&self) -> Option<&str> {
        match self {
            PadaEntry::Tinanta(t) => Some(t.dhatu_text()),
            PadaEntry::Subanta(s) => Some(s.pratipadika_entry.lemma()),
        }
    }
}

impl<'a> From<PadaEntry<'a>> for Pada {
    fn from(val: PadaEntry<'a>) -> Pada {
        match val {
            PadaEntry::Subanta(s) => Pada::Subanta(s.into()),
            PadaEntry::Tinanta(t) => Pada::Tinanta(t.into()),
        }
    }
}

impl<'a> TryFrom<&'a Pada> for PadaEntry<'a> {
    type Error = Error;

    fn try_from(val: &'a Pada) -> Result<PadaEntry<'a>> {
        match val {
            Pada::Subanta(s) => {
                let entry: SubantaEntry = s.try_into()?;
                Ok(PadaEntry::Subanta(entry))
            }
            Pada::Tinanta(t) => {
                let entry: TinantaEntry = t.into();
                Ok(entry.into())
            }
            _ => Err(Error::UnsupportedType),
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

#[cfg(test)]
mod tests {
    use super::*;
    use vidyut_prakriya::args::*;

    fn safe(s: &str) -> Slp1String {
        Slp1String::from(s).expect("ok")
    }

    #[test]
    fn pada_entry_tinanta() {
        let gam = Dhatu::mula(safe("ga\\mx~"), Gana::Bhvadi);
        let gam_meta = DhatuMeta::builder()
            .clean_text("gam".to_string())
            .artha_sa("gatO".to_string())
            .build()
            .expect("ok");
        let gam_entry = DhatuEntry::new(&gam).with_meta(&gam_meta);
        let gacchati = PadaEntry::Tinanta(TinantaEntry::new(
            gam_entry.clone(),
            Prayoga::Kartari,
            Lakara::Lat,
            Purusha::Prathama,
            Vacana::Eka,
        ));
        assert_eq!(gacchati.lemma(), Some("gam"));
    }

    #[test]
    fn tinanta_entry_round_trip() {
        let gam = Dhatu::mula(safe("ga\\mx~"), Gana::Bhvadi);
        let expected = Tinanta::new(
            gam,
            Prayoga::Kartari,
            Lakara::Lat,
            Purusha::Prathama,
            Vacana::Eka,
        );
        let entry: TinantaEntry = (&expected).into();
        let actual: Tinanta = entry.into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn pada_entry_basic_subanta() {
        let rama = Pratipadika::basic(safe("rAma"));
        let rama_entry: PratipadikaEntry = (&rama).try_into().expect("ok");
        let ramah = PadaEntry::Subanta(SubantaEntry::new(
            rama_entry,
            Linga::Pum,
            Vibhakti::Prathama,
            Vacana::Eka,
        ));
        assert_eq!(ramah.lemma(), Some("rAma"));
    }

    #[test]
    fn subanta_entry_round_trip() {
        let rama = Pratipadika::basic(safe("rAma"));
        let expected = Subanta::new(rama, Linga::Pum, Vibhakti::Prathama, Vacana::Eka);
        let entry: SubantaEntry = (&expected).try_into().expect("ok");
        let actual: Subanta = entry.into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn pada_entry_krdanta_subanta() {
        let gam = Dhatu::mula(safe("ga\\mx~"), Gana::Bhvadi);
        let gam_meta = DhatuMeta::builder()
            .clean_text("gam".to_string())
            .artha_sa("gatO".to_string())
            .build()
            .expect("ok");
        let gam_entry = DhatuEntry::new(&gam).with_meta(&gam_meta);
        let gamaka = PratipadikaEntry::Krdanta(KrdantaEntry::new(
            gam_entry,
            BaseKrt::Rvul.into(),
            None,
            None,
        ));
        let gamakah = PadaEntry::Subanta(SubantaEntry::new(
            gamaka,
            Linga::Pum,
            Vibhakti::Prathama,
            Vacana::Eka,
        ));
        assert_eq!(gamakah.lemma(), Some("gam"));
    }

    #[test]
    fn pada_entry_avyaya() {
        let iti = Pratipadika::basic(safe("iti"));
        let iti_entry: PratipadikaEntry = (&iti).try_into().expect("ok");
        let iti_pada = PadaEntry::Subanta(SubantaEntry::avyaya(iti_entry));
        assert_eq!(iti_pada.lemma(), Some("iti"));
    }
}
