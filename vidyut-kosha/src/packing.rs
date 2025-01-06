/*!
Code for packing and unpacking Sanskrit morphological data.

**Packing* is the process of converting some data into a dense integer representation. The reverse
process is accordingly called *unpacking*,

Packed data is useful for two reasons;

1. Packed data takes up less space in memory with little or no performance penalty.
2. Our finite-state transducer can store values only if they are integers. In other words, packing
   is a necessary precondition to storing data in an FST.

The downside of packed data is that it cannot easily store string data. To work around this
problem, we can use a lookup table that maps integer indices to string values. But lookup tables
are much more cumbersome than simple structs.


Approach
========

For the use case of an inflectional dictionary, we want to store two kinds of information:
 *enumerated data* and *text data*.

*Enumerated data* includes categories like person, number, gender, case, and so on. Each category
can take one of several possible values that we know ahead of time. We can trivially convert such
data to an integer value. For example, we represent the *vacana* (number) of a word with one of
four possible values: `eka`, `dvi`, `bahu`, or `none` if the *vacana* is unknown for some reason.
Thus we can map `eka` to 1, `dvi` to 2, and so on.

*Text data* includes the stem or root of the underlying form, and we cannot feasibly enumerate it
ahead of time. We encode this data through a lookup table approach: if we append all strings to a
list, then the index of that string in the list is its integer representation. By using this
approach, we pay the price of storing these strings the old-fashioned way. But a list of lemmas is
 *much* smaller than a list of words, so the space requirements are trivial.

Once we have mapped all of our information to integer values, we can treat our 64-bit integer as a
bitfield and add values together with the appropriate shifts. For example, here's an early version
of our scheme for *tinantas*, which uses only 32 bits:

```text
OOLLLLppVVaadddddddddddddddddddd

O: part of speech (2 bits = 4 possible values)
L: lakara (4 bits = 16 possible values)
p: purusha (2 bits = 4 possible values)
V: vacana (2 bits = 4 possible values)
a: pada + prayoga (2 bits = 4 possible values)
d: dhatu ID (20 bits = ~1 million possible values)
```

One real consequence of the encoding constraint is that we can't associate words with completely
arbitrary data. But if our data is structured carefully, we have plenty of room to associate each
word with interesting information.


Implementation details
======================

We manage bitfields with the `modular_bitfield` crate. For consistency with that crate's defaults,
we treat all packed data as little-endian and hence use `from_le_bytes` and `to_le_bytes` whenever
we need to convert between representations.

TODO: investigate different packing orders to see if we can reduce the size of the FST.
*/

// Too general, but needed to suppress a warning in `PackedUnknown`.
#![allow(dead_code)]

use crate::entries::{
    DhatuEntry, KrdantaEntry, PadaEntry, PratipadikaEntry, SubantaEntry, TinantaEntry,
};
use crate::errors::{Error, Result};
use modular_bitfield::prelude::*;
use rustc_hash::FxHashMap;
use vidyut_prakriya::args::{
    BasicPratipadika, Dhatu, Krt, Lakara, Linga, Prayoga, Purusha, Vacana, Vibhakti,
};

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

/// An ID for interned linguistic data.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) struct Id(pub usize);

/// Models the part of speech for the given `Pada`. The value of `PartOfSpeech` controls how we
/// interpret the rest of the packed data.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum PartOfSpeech {
    /// A subanta.
    Subanta,
    /// A subanta prefix.
    SubantaPrefix,
    /// A tinanta.
    Tinanta,
    /// A tinanta prefix.
    Avyaya,
}

/// A subanta suffix as part of some paradigm.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct SubantaSuffix {
    text: String,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

/// A sup paradigm.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct SubantaParadigm {
    endings: Vec<SubantaSuffix>,
}

impl SubantaSuffix {
    fn new(text: String, linga: Linga, vibhakti: Vibhakti, vacana: Vacana) -> Self {
        Self {
            text,
            linga,
            vibhakti,
            vacana,
        }
    }
}

impl SubantaParadigm {
    fn from_padas(padas: &[(String, Linga, Vibhakti, Vacana)]) -> Self {
        assert!(!padas.is_empty());

        let mut common_prefix = String::from(&padas[0].0);
        for pada in &padas[1..] {
            for ((i, x), y) in common_prefix.char_indices().zip(pada.0.chars()) {
                if x != y {
                    common_prefix.truncate(i);
                    break;
                }
            }
        }

        let offset = common_prefix.len();
        let endings: Vec<_> = padas
            .iter()
            .map(|p| {
                let ending = String::from(&p.0[offset..]);
                SubantaSuffix::new(ending, p.1, p.2, p.3)
            })
            .collect();

        assert!(endings.len() == padas.len());
        Self { endings }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct DhatuMeta {
    clean_text: String,
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct PratipadikaMeta {
    lingas: Vec<Linga>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct Sup {
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct Tin {
    prayoga: Prayoga,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct RichKrt {
    krt: Krt,
    prayoga: Option<Prayoga>,
    lakara: Option<Lakara>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) enum SmallPratipadika {
    Basic(BasicPratipadika),
    Krdanta(SmallKrdanta),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub(crate) struct SmallKrdanta {
    dhatu_id: Id,
    krt_id: Id,
}

/// A prefix to some paradigm of sup endings.
#[bitfield(bits = 30)]
#[derive(Debug)]
pub struct PackedSubantaPrefix {
    #[allow(unused)]
    pratipadika_id: B21,
    paradigm_id: B9,
}

/// Semantics for an *avyaya*.
#[bitfield(bits = 30)]
struct PackedSubanta {
    sup_id: B7,
    pratipadika_id: B23,
}

/// Semantics for a *tinanta*.
#[bitfield(bits = 30)]
struct PackedTinanta {
    tin_id: B8,
    dhatu_id: B22,
}

/// Semantics for an *avyaya*.
#[bitfield(bits = 30)]
struct PackedAvyaya {
    pratipadika_id: B30,
}

/// Semantics for a *pada*.
#[bitfield]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PackedEntry {
    /// The part of speech for these semantics. We use this value to decide how to interpret the
    /// `payload` field.
    pub pos: PartOfSpeech,
    /// The core data for this semantics type. We interpret `paylaad` differently based on the
    /// value of `pos`.
    pub(crate) payload: B30,
}

/// Semantics for an unknown term.
#[bitfield(bits = 30)]
struct PackedUnknown {
    #[skip]
    unused: B30,
}

impl Sup {
    fn new(linga: Linga, vibhakti: Vibhakti, vacana: Vacana) -> Self {
        Self {
            linga,
            vibhakti,
            vacana,
        }
    }
}

impl Tin {
    fn new(prayoga: Prayoga, lakara: Lakara, purusha: Purusha, vacana: Vacana) -> Self {
        Self {
            prayoga,
            lakara,
            purusha,
            vacana,
        }
    }
}

impl PackedEntry {
    /// Unsafely interprets this packed pada as an avyaya.
    fn as_packed_avyaya(self) -> PackedAvyaya {
        PackedAvyaya::from_bytes(self.payload().to_le_bytes())
    }

    /// Unsafely interprets this packed pada as a subanta.
    pub fn as_packed_subanta_prefix(self) -> PackedSubantaPrefix {
        PackedSubantaPrefix::from_bytes(self.payload().to_le_bytes())
    }

    /// Unsafely interprets this packed pada as a tinanta.
    fn as_packed_subanta(self) -> PackedSubanta {
        PackedSubanta::from_bytes(self.payload().to_le_bytes())
    }

    /// Unsafely interprets this packed pada as a tinanta.
    fn as_packed_tinanta(self) -> PackedTinanta {
        PackedTinanta::from_bytes(self.payload().to_le_bytes())
    }

    /// Unwraps the bitfield as an ordinary `u32`.
    pub(crate) fn to_u32(self) -> u32 {
        u32::from_le_bytes(self.into_bytes())
    }

    /// Wraps an integer value as a `PackedPada` bitfield.
    pub fn from_u32(u: u32) -> Self {
        Self::from_bytes(u.to_le_bytes())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Registry {
    krts: Vec<RichKrt>,
    dhatus: Vec<Dhatu>,
    dhatu_meta: Vec<DhatuMeta>,
    pratipadikas: Vec<SmallPratipadika>,
    pratipadika_meta: FxHashMap<Id, PratipadikaMeta>,
    paradigms: Vec<SubantaParadigm>,
}

/// Packs and unpacks linguistic data.
#[derive(Clone, Default)]
pub(crate) struct Packer {
    sups: Vec<Sup>,
    sup_to_index: FxHashMap<Sup, Id>,

    tins: Vec<Tin>,
    tin_to_index: FxHashMap<Tin, Id>,

    krts: Vec<RichKrt>,
    krt_to_index: FxHashMap<RichKrt, Id>,

    pub(crate) dhatus: Vec<Dhatu>,
    dhatu_to_index: FxHashMap<Dhatu, Id>,

    pub(crate) pratipadikas: Vec<SmallPratipadika>,
    pratipadika_to_index: FxHashMap<SmallPratipadika, Id>,

    paradigms: Vec<SubantaParadigm>,
    paradigm_to_index: FxHashMap<SubantaParadigm, Id>,

    dhatu_meta: Vec<DhatuMeta>,
    pratipadika_meta: FxHashMap<Id, PratipadikaMeta>,
}

impl Packer {
    /// Creates a new packer.
    pub(crate) fn new() -> Self {
        let mut ret = Self::default();

        for linga in Linga::iter() {
            for vibhakti in Vibhakti::iter() {
                for vacana in Vacana::iter() {
                    let sup = Sup::new(linga, vibhakti, vacana);
                    let id = Id(ret.sups.len());
                    ret.sups.push(sup);
                    ret.sup_to_index.insert(sup, id);
                }
            }
        }
        assert!(ret.sups.len() < 1 << 7);

        for prayoga in Prayoga::iter() {
            if prayoga == Prayoga::Bhave {
                // Skip -- same as Karmani.
                continue;
            }
            for lakara in Lakara::iter() {
                for purusha in Purusha::iter() {
                    for vacana in Vacana::iter() {
                        let tin = Tin::new(prayoga, lakara, purusha, vacana);
                        let id = Id(ret.tins.len());
                        ret.tins.push(tin);
                        ret.tin_to_index.insert(tin, id);
                    }
                }
            }
        }
        assert!(ret.tins.len() < 1 << 8);

        ret
    }

    /// Loads a packer from disk.
    pub(crate) fn read(registry_path: &Path) -> Result<Self> {
        let mut ret = Self::new();

        let file = File::open(registry_path)?;
        let reader = BufReader::new(file);
        let registry: Registry = rmp_serde::from_read(reader)?;

        let Registry {
            krts,
            dhatus,
            pratipadikas,
            paradigms,
            dhatu_meta,
            pratipadika_meta,
        } = registry;

        ret.krts = krts;
        ret.dhatus = dhatus;
        ret.pratipadikas = pratipadikas;
        ret.paradigms = paradigms;
        ret.dhatu_meta = dhatu_meta;
        ret.pratipadika_meta = pratipadika_meta;

        ret.krt_to_index = ret
            .krts
            .iter()
            .enumerate()
            .map(|(i, x)| (x.clone(), Id(i)))
            .collect();

        ret.dhatu_to_index = ret
            .dhatus
            .iter()
            .enumerate()
            .map(|(i, x)| (x.clone(), Id(i)))
            .collect();

        ret.pratipadika_to_index = ret
            .pratipadikas
            .iter()
            .enumerate()
            .map(|(i, x)| (x.clone(), Id(i)))
            .collect();

        ret.paradigm_to_index = ret
            .paradigms
            .iter()
            .enumerate()
            .map(|(i, x)| (x.clone(), Id(i)))
            .collect();

        Ok(ret)
    }

    /// Writes the registry to disk.
    pub fn write(&self, registry_path: &Path) -> Result<()> {
        let registry = Registry {
            krts: self.krts.clone(),
            dhatus: self.dhatus.clone(),
            pratipadikas: self.pratipadikas.clone(),
            paradigms: self.paradigms.clone(),

            dhatu_meta: self.dhatu_meta.clone(),
            pratipadika_meta: self.pratipadika_meta.clone(),
        };

        let out = rmp_serde::to_vec(&registry)?;
        let file = File::create(registry_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&out)?;

        Ok(())
    }

    pub(crate) fn contains_subanta_suffix(
        &self,
        prefix: &PackedSubantaPrefix,
        suffix: &str,
    ) -> bool {
        // TODO: slow, just getting it working first.
        if let Some(paradigm) = self.paradigms.get(prefix.paradigm_id() as usize) {
            paradigm.endings.iter().any(|e| e.text == suffix)
        } else {
            false
        }
    }

    pub(crate) fn get_all_from_subanta_paradigm<'a>(
        &'a self,
        ret: &mut Vec<PadaEntry<'a>>,
        entry: &PackedSubantaPrefix,
        suffix: &str,
    ) -> Result<()> {
        // TODO: slow, just getting it working first.
        let paradigm = &self
            .paradigms
            .get(entry.paradigm_id() as usize)
            .ok_or_else(|| Error::UnknownId("paradigm", entry.paradigm_id() as usize))?;
        let phit = self.unpack_pratipadika(Id(entry.pratipadika_id() as usize))?;

        for ending in &paradigm.endings {
            if ending.text == suffix {
                ret.push(PadaEntry::Subanta(SubantaEntry::new(
                    // Stack-allocated, clones are cheap.
                    phit.clone(),
                    ending.linga,
                    ending.vibhakti,
                    ending.vacana,
                )));
            }
        }

        Ok(())
    }

    /// Registers the given dhatu and returns its interned ID.
    pub(crate) fn register_dhatu_entry(&mut self, entry: &DhatuEntry) -> Id {
        let dhatu = entry.dhatu();

        if let Some(id) = self.dhatu_to_index.get(dhatu) {
            *id
        } else {
            let id = Id(self.dhatus.len());
            self.dhatus.push(dhatu.clone());
            self.dhatu_meta.push(DhatuMeta::default());
            self.dhatu_to_index.insert(dhatu.clone(), id);

            self.dhatu_meta
                .get_mut(id.0)
                .expect("just pushed")
                .clean_text = entry.clean_text().to_string();

            id
        }
    }

    pub(crate) fn register_krt(&mut self, entry: &KrdantaEntry) -> Id {
        let krt = RichKrt {
            krt: entry.krt(),
            prayoga: entry.prayoga(),
            lakara: entry.lakara(),
        };
        if let Some(id) = self.krt_to_index.get(&krt) {
            *id
        } else {
            let id = Id(self.krts.len());
            self.krts.push(krt.clone());
            self.krt_to_index.insert(krt.clone(), id);
            id
        }
    }

    /// Registers the and pratipadika and returns its interned ID.
    pub(crate) fn register_pratipadika_entry(&mut self, entry: &PratipadikaEntry) -> Id {
        use PratipadikaEntry as PE;

        let small = match entry {
            PE::Basic(b) => SmallPratipadika::Basic(b.pratipadika().clone()),
            PE::Krdanta(k) => {
                let dhatu_id = self.register_dhatu_entry(k.dhatu_entry());
                let krt_id = self.register_krt(&k);
                SmallPratipadika::Krdanta(SmallKrdanta { dhatu_id, krt_id })
            }
        };

        if let Some(i) = self.pratipadika_to_index.get(&small) {
            *i
        } else {
            let id = Id(self.pratipadikas.len());
            self.pratipadikas.push(small.clone());
            self.pratipadika_to_index.insert(small.clone(), id);

            if let PE::Basic(b) = entry {
                self.pratipadika_meta.insert(
                    id,
                    PratipadikaMeta {
                        lingas: b.lingas().to_vec(),
                    },
                );
            }

            id
        }
    }

    /// Registers the given paradigm of subantas and returns the `SubantaPrefix` that wraps them.
    pub(crate) fn register_subanta_paradigm(
        &mut self,
        pratipadika: &PratipadikaEntry,
        padas: &[(String, Linga, Vibhakti, Vacana)],
    ) -> Result<(String, PackedEntry)> {
        assert!(!padas.is_empty());

        let paradigm = SubantaParadigm::from_padas(padas);
        let key = {
            let prefix = padas[0]
                .0
                .strip_suffix(&paradigm.endings[0].text)
                .expect("aligned");
            String::from(prefix)
        };

        let pratipadika_id = self.register_pratipadika_entry(pratipadika);
        let paradigm_id = match self.paradigm_to_index.get(&paradigm) {
            Some(id) => *id,
            None => {
                let id = Id(self.paradigms.len());
                self.paradigms.push(paradigm.clone());
                self.paradigm_to_index.insert(paradigm, id);
                id
            }
        };

        let value = {
            let prefix = PackedSubantaPrefix::new()
                .with_pratipadika_id(pratipadika_id.0.try_into()?)
                .with_paradigm_id(paradigm_id.0.try_into()?);

            PackedEntry::new()
                .with_pos(PartOfSpeech::SubantaPrefix)
                .with_payload(u32::from_le_bytes(prefix.into_bytes()))
        };

        Ok((key, value))
    }

    fn pack_dhatu(&self, dhatu: &Dhatu) -> Result<Id> {
        self.dhatu_to_index
            .get(dhatu)
            .copied()
            .ok_or_else(|| Error::NotRegistered("dhatu"))
    }

    fn pack_pratipadika(&self, pratipadika: &PratipadikaEntry) -> Result<SmallPratipadika> {
        use PratipadikaEntry as PE;

        match pratipadika {
            PE::Basic(b) => Ok(SmallPratipadika::Basic(b.pratipadika().clone())),
            PE::Krdanta(k) => {
                let dhatu_id = self.pack_dhatu(k.dhatu())?;
                let krt_id = self.pack_krt(k)?;
                Ok(SmallPratipadika::Krdanta(SmallKrdanta { dhatu_id, krt_id }))
            }
        }
    }

    fn pack_krt(&self, krdanta: &KrdantaEntry) -> Result<Id> {
        assert!(!self.krt_to_index.is_empty());
        let krt = RichKrt {
            krt: krdanta.krt(),
            prayoga: krdanta.prayoga(),
            lakara: krdanta.lakara(),
        };
        self.krt_to_index
            .get(&krt)
            .copied()
            .ok_or_else(|| Error::NotRegistered("krt"))
    }

    fn pack_avyaya(&self, s: &SubantaEntry) -> Result<PackedAvyaya> {
        let packed_pratipadika = self.pack_pratipadika(s.pratipadika_entry())?;

        match self.pratipadika_to_index.get(&packed_pratipadika) {
            Some(pratipadika_id) => {
                let ret = PackedAvyaya::new().with_pratipadika_id(pratipadika_id.0.try_into()?);
                Ok(ret)
            }
            None => Err(Error::NotRegistered("pratipadika")),
        }
    }

    fn pack_subanta(&self, s: &SubantaEntry) -> Result<PackedSubanta> {
        let packed_pratipadika = self.pack_pratipadika(s.pratipadika_entry())?;
        match self.pratipadika_to_index.get(&packed_pratipadika) {
            Some(pratipadika_id) => {
                let sup_id = self
                    .sup_to_index
                    .get(&Sup::new(s.linga(), s.vibhakti(), s.vacana()))
                    .ok_or_else(|| Error::NotRegistered("sup"))?;
                let ret = {
                    PackedSubanta::new()
                        .with_pratipadika_id(pratipadika_id.0.try_into()?)
                        .with_sup_id(sup_id.0 as u8)
                };
                Ok(ret)
            }
            None => Err(Error::NotRegistered("pratipadika")),
        }
    }

    fn pack_tinanta(&self, t: &TinantaEntry) -> Result<PackedTinanta> {
        let dhatu_id = self.pack_dhatu(t.dhatu())?;
        let tin_id = self
            .tin_to_index
            .get(&Tin::new(t.prayoga(), t.lakara(), t.purusha(), t.vacana()))
            .ok_or_else(|| Error::NotRegistered("tin"))?;
        let ret = PackedTinanta::new()
            .with_dhatu_id(dhatu_id.0.try_into()?)
            .with_tin_id(tin_id.0 as u8);
        Ok(ret)
    }

    /// Packs the given semantics into an integer value.
    pub(crate) fn pack(&self, pada: &PadaEntry) -> Result<PackedEntry> {
        let ret = match pada {
            PadaEntry::Avyaya(a) => {
                let payload = self.pack_avyaya(a)?;
                PackedEntry::new()
                    .with_pos(PartOfSpeech::Avyaya)
                    .with_payload(u32::from_le_bytes(payload.into_bytes()))
            }
            PadaEntry::Tinanta(t) => {
                let payload = self.pack_tinanta(t)?;
                PackedEntry::new()
                    .with_pos(PartOfSpeech::Tinanta)
                    .with_payload(u32::from_le_bytes(payload.into_bytes()))
            }
            PadaEntry::Subanta(s) => {
                let payload = self.pack_subanta(s)?;
                PackedEntry::new()
                    .with_pos(PartOfSpeech::Subanta)
                    .with_payload(u32::from_le_bytes(payload.into_bytes()))
            }
            PadaEntry::Unknown => {
                return Err(Error::NotRegistered("Unknown"));
            }
        };

        Ok(ret)
    }

    fn unpack_sup(&self, id: Id) -> Result<&Sup> {
        match self.sups.get(id.0) {
            Some(s) => Ok(s),
            None => Err(Error::UnknownId("sup", id.0)),
        }
    }

    fn unpack_tin(&self, id: Id) -> Result<&Tin> {
        match self.tins.get(id.0) {
            Some(t) => Ok(t),
            None => Err(Error::UnknownId("tin", id.0)),
        }
    }

    fn unpack_krt(&self, id: Id) -> Result<&RichKrt> {
        match self.krts.get(id.0) {
            Some(k) => Ok(k),
            None => Err(Error::UnknownId("krt", id.0)),
        }
    }

    pub(crate) fn unpack_dhatu(&self, id: Id) -> Result<DhatuEntry> {
        match (self.dhatus.get(id.0), self.dhatu_meta.get(id.0)) {
            (Some(d), Some(m)) => {
                let entry = DhatuEntry::new(d, &m.clean_text);
                Ok(entry)
            }
            _ => Err(Error::UnknownId("dhatu", id.0)),
        }
    }

    pub(crate) fn unpack_pratipadika(&self, id: Id) -> Result<PratipadikaEntry> {
        match self.pratipadikas.get(id.0) {
            Some(p) => match p {
                SmallPratipadika::Basic(b) => {
                    let lingas = match self.pratipadika_meta.get(&id) {
                        Some(m) => m.lingas.as_slice(),
                        None => &[],
                    };

                    Ok(PratipadikaEntry::basic(b, lingas))
                }
                SmallPratipadika::Krdanta(k) => {
                    let dhatu_entry = self.unpack_dhatu(k.dhatu_id)?;
                    let krt = self.unpack_krt(k.krt_id)?;
                    Ok(PratipadikaEntry::Krdanta(KrdantaEntry::new(
                        dhatu_entry,
                        krt.krt,
                        krt.prayoga,
                        krt.lakara,
                    )))
                }
            },
            None => Err(Error::UnknownId("pratipadika", id.0)),
        }
    }

    fn unpack_subanta(&self, packed: PackedSubanta) -> Result<SubantaEntry> {
        let pratipadika = self.unpack_pratipadika(Id(packed.pratipadika_id() as usize))?;
        let sup = self.unpack_sup(Id(packed.sup_id() as usize))?;
        Ok(SubantaEntry::new(
            pratipadika,
            sup.linga,
            sup.vibhakti,
            sup.vacana,
        ))
    }

    fn unpack_tinanta(&self, packed: PackedTinanta) -> Result<TinantaEntry> {
        let dhatu_entry = self.unpack_dhatu(Id(packed.dhatu_id() as usize))?;
        let tin = self.unpack_tin(Id(packed.tin_id() as usize))?;

        Ok(TinantaEntry::new(
            dhatu_entry,
            tin.prayoga,
            tin.lakara,
            tin.purusha,
            tin.vacana,
        ))
    }

    fn unpack_avyaya(&self, packed: PackedAvyaya) -> Result<SubantaEntry> {
        let pratipadika = self.unpack_pratipadika(Id(packed.pratipadika_id() as usize))?;
        Ok(SubantaEntry::avyaya(pratipadika))
    }

    /// Unpacks the given packed pada.
    pub fn unpack(&self, pada: &PackedEntry) -> Result<PadaEntry> {
        match pada.pos() {
            PartOfSpeech::Subanta => {
                let subanta = self.unpack_subanta(pada.as_packed_subanta())?;
                Ok(PadaEntry::Subanta(subanta))
            }
            PartOfSpeech::Tinanta => {
                let tinanta = self.unpack_tinanta(pada.as_packed_tinanta())?;
                Ok(PadaEntry::Tinanta(tinanta))
            }
            PartOfSpeech::Avyaya => {
                let avyaya = self.unpack_avyaya(pada.as_packed_avyaya())?;
                Ok(PadaEntry::Avyaya(avyaya))
            }
            _ => Ok(PadaEntry::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vidyut_prakriya::args as vp;
    use vidyut_prakriya::args::{Pratipadika, Subanta};
    type TestResult = Result<()>;

    fn safe(s: &str) -> vp::Slp1String {
        vp::Slp1String::from(s).expect("static")
    }

    #[test]
    fn test_subanta_packing() -> TestResult {
        let deva = Pratipadika::basic(safe("deva"));
        let nara = Pratipadika::basic(safe("nara"));
        let devasya = Subanta::new(deva.clone(), Linga::Pum, Vibhakti::Sasthi, Vacana::Eka);
        let narasya = Subanta::new(nara.clone(), Linga::Pum, Vibhakti::Sasthi, Vacana::Eka);
        let devasya_entry: SubantaEntry = (&devasya).try_into().expect("ok");
        let narasya_entry: SubantaEntry = (&narasya).try_into().expect("ok");

        let mut packer = Packer::new();
        packer.register_pratipadika_entry(devasya_entry.pratipadika_entry());
        packer.register_pratipadika_entry(narasya_entry.pratipadika_entry());

        let devasya_entry: PadaEntry = devasya_entry.into();
        let narasya_entry: PadaEntry = narasya_entry.into();
        let devasya_code = packer.pack(&devasya_entry)?;
        let narasya_code = packer.pack(&narasya_entry)?;
        assert_eq!(packer.unpack(&narasya_code)?, narasya_entry);
        assert_eq!(packer.unpack(&devasya_code)?, devasya_entry);
        Ok(())
    }

    #[test]
    fn test_tinanta_packing() -> TestResult {
        let gam = Dhatu::mula(safe("ga\\mx~"), vp::Gana::Bhvadi);
        let gacchati = PadaEntry::Tinanta(TinantaEntry::new(
            DhatuEntry::new(&gam, "gam"),
            Prayoga::Kartari,
            Lakara::Lat,
            Purusha::Prathama,
            Vacana::Eka,
        ));

        let car = Dhatu::mula(safe("cara~"), vp::Gana::Bhvadi);
        let carati = PadaEntry::Tinanta(TinantaEntry::new(
            DhatuEntry::new(&car, "car"),
            Prayoga::Kartari,
            Lakara::Lat,
            Purusha::Prathama,
            Vacana::Eka,
        ));

        let gam_entry = DhatuEntry::new(&gam, "gam");
        let car_entry = DhatuEntry::new(&car, "car");

        let mut packer = Packer::new();
        packer.register_dhatu_entry(&gam_entry);
        packer.register_dhatu_entry(&car_entry);

        let gacchati_code = packer.pack(&gacchati)?;
        let carati_code = packer.pack(&carati)?;

        assert_eq!(packer.unpack(&carati_code)?, carati);
        assert_eq!(packer.unpack(&gacchati_code)?, gacchati);
        Ok(())
    }

    #[test]
    fn test_avyaya_packing() -> TestResult {
        let iti_stem = Pratipadika::basic(safe("iti"));
        let ca_stem = Pratipadika::basic(safe("ca"));

        let mut packer = Packer::new();
        let iti_entry = (&iti_stem).try_into().expect("ok");
        let ca_entry = (&ca_stem).try_into().expect("ok");
        packer.register_pratipadika_entry(&iti_entry);
        packer.register_pratipadika_entry(&ca_entry);

        let iti = PadaEntry::Avyaya(SubantaEntry::avyaya(iti_entry));
        let ca = PadaEntry::Avyaya(SubantaEntry::avyaya(ca_entry));

        let iti_code = packer.pack(&iti)?;
        let ca_code = packer.pack(&ca)?;

        assert_eq!(packer.unpack(&ca_code)?, ca);
        assert_eq!(packer.unpack(&iti_code)?, iti);
        Ok(())
    }
}
