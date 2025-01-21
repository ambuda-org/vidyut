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
    DhatuEntry, DhatuMeta, KrdantaEntry, PadaEntry, PratipadikaEntry, SubantaEntry, TinantaEntry,
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
    TinantaPrefix,
}

/// A subanta suffix as part of some paradigm.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SubantaSuffix {
    text: String,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

/// A subanta suffix table.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SubantaSuffixes {
    endings: Vec<SubantaSuffix>,
}

/// A tinanta suffix as part of some paradigm.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TinantaSuffix {
    text: String,
    prayoga: Prayoga,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
}

/// A tinanta suffix table.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TinantaSuffixes {
    endings: Vec<TinantaSuffix>,
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

    /// Returns the text of this suffix.
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl SubantaSuffixes {
    /// Returns all suffixes defined in this table.
    pub fn endings(&self) -> &[SubantaSuffix] {
        &self.endings
    }

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

impl TinantaSuffix {
    fn new(
        text: String,
        prayoga: Prayoga,
        lakara: Lakara,
        purusha: Purusha,
        vacana: Vacana,
    ) -> Self {
        Self {
            text,
            prayoga,
            lakara,
            purusha,
            vacana,
        }
    }

    /// Returns the text of this suffix.
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl TinantaSuffixes {
    /// Returns all suffixes defined in this table.
    pub fn endings(&self) -> &[TinantaSuffix] {
        &self.endings
    }

    fn from_padas(padas: &[(String, Prayoga, Lakara, Purusha, Vacana)]) -> Self {
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
                TinantaSuffix::new(ending, p.1, p.2, p.3, p.4)
            })
            .collect();

        assert!(endings.len() == padas.len());
        Self { endings }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct PratipadikaMeta {
    lingas: Vec<Linga>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
enum Sup {
    Basic {
        linga: Linga,
        vibhakti: Vibhakti,
        vacana: Vacana,
    },
    Avyaya,
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
    pub pratipadika_id: B21,
    pub paradigm_id: B9,
}

/// Semantics for a *subanta*.
#[bitfield(bits = 30)]
pub struct PackedSubanta {
    pub sup_id: B7,
    pub pratipadika_id: B23,
}

/// A prefix to some paradigm of sup endings.
#[bitfield(bits = 30)]
#[derive(Debug)]
pub struct PackedTinantaPrefix {
    pub dhatu_id: B18,
    pub paradigm_id: B12,
}

/// Semantics for a *tinanta*.
#[bitfield(bits = 30)]
pub struct PackedTinanta {
    pub tin_id: B8,
    pub dhatu_id: B22,
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
        Self::Basic {
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
    /// Interprets this packed pada as a subanta.
    pub fn as_packed_subanta_prefix(self) -> PackedSubantaPrefix {
        PackedSubantaPrefix::from_bytes(self.payload().to_le_bytes())
    }

    /// Interprets this packed pada as a tinanta.
    pub fn as_packed_subanta(self) -> PackedSubanta {
        PackedSubanta::from_bytes(self.payload().to_le_bytes())
    }

    /// Interprets this packed pada as a subanta.
    pub fn as_packed_tinanta_prefix(self) -> PackedTinantaPrefix {
        PackedTinantaPrefix::from_bytes(self.payload().to_le_bytes())
    }

    /// Interprets this packed pada as a tinanta.
    pub fn as_packed_tinanta(self) -> PackedTinanta {
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

    pub(crate) fn is_prefix(&self) -> bool {
        self.pos() == PartOfSpeech::SubantaPrefix || self.pos() == PartOfSpeech::TinantaPrefix
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Registry {
    count: usize,
    krts: Vec<RichKrt>,
    dhatus: Vec<Dhatu>,
    dhatu_meta: Vec<DhatuMeta>,
    pratipadikas: Vec<SmallPratipadika>,
    pratipadika_meta: FxHashMap<Id, PratipadikaMeta>,
    subanta_suffixes: Vec<SubantaSuffixes>,
    tinanta_suffixes: Vec<TinantaSuffixes>,
}

/// Packs and unpacks linguistic data.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Packer {
    pub(crate) count: usize,

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

    pub(crate) subanta_suffixes: Vec<SubantaSuffixes>,
    subanta_suffixes_to_index: FxHashMap<SubantaSuffixes, Id>,

    pub(crate) tinanta_suffixes: Vec<TinantaSuffixes>,
    tinanta_suffixes_to_index: FxHashMap<TinantaSuffixes, Id>,

    dhatu_meta: Vec<DhatuMeta>,
    // This is a Map because not all pratipadikas have metadata.
    pratipadika_meta: FxHashMap<Id, PratipadikaMeta>,
}

impl Packer {
    /// Creates a new packer.
    pub fn new() -> Self {
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
        {
            let id = Id(ret.sups.len());
            ret.sups.push(Sup::Avyaya);
            ret.sup_to_index.insert(Sup::Avyaya, id);
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
            count,
            krts,
            dhatus,
            pratipadikas,
            subanta_suffixes,
            tinanta_suffixes,
            dhatu_meta,
            pratipadika_meta,
        } = registry;

        ret.count = count;
        ret.krts = krts;
        ret.dhatus = dhatus;
        ret.pratipadikas = pratipadikas;
        ret.subanta_suffixes = subanta_suffixes;
        ret.tinanta_suffixes = tinanta_suffixes;
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

        ret.subanta_suffixes_to_index = ret
            .subanta_suffixes
            .iter()
            .enumerate()
            .map(|(i, x)| (x.clone(), Id(i)))
            .collect();

        Ok(ret)
    }

    /// Writes the registry to disk.
    pub(crate) fn write(&self, registry_path: &Path) -> Result<()> {
        let registry = Registry {
            count: self.count,
            krts: self.krts.clone(),
            dhatus: self.dhatus.clone(),
            pratipadikas: self.pratipadikas.clone(),
            subanta_suffixes: self.subanta_suffixes.clone(),
            tinanta_suffixes: self.tinanta_suffixes.clone(),

            dhatu_meta: self.dhatu_meta.clone(),
            pratipadika_meta: self.pratipadika_meta.clone(),
        };

        let out = rmp_serde::to_vec(&registry)?;
        let file = File::create(registry_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&out)?;

        Ok(())
    }

    // TODO: slow, just getting it working first.
    pub(crate) fn contains_suffix(&self, base_prefix: &PackedEntry, suffix: &str) -> bool {
        if base_prefix.pos() == PartOfSpeech::SubantaPrefix {
            let prefix = base_prefix.as_packed_subanta_prefix();
            if let Some(paradigm) = self.subanta_suffixes.get(prefix.paradigm_id() as usize) {
                paradigm.endings.iter().any(|e| e.text == suffix)
            } else {
                false
            }
        } else if base_prefix.pos() == PartOfSpeech::TinantaPrefix {
            let prefix = base_prefix.as_packed_tinanta_prefix();
            if let Some(paradigm) = self.tinanta_suffixes.get(prefix.paradigm_id() as usize) {
                paradigm.endings.iter().any(|e| e.text == suffix)
            } else {
                false
            }
        } else {
            false
        }
    }

    // TODO: slow, just getting it working first.
    pub(crate) fn get_all_for_suffix<'a>(
        &'a self,
        ret: &mut Vec<PadaEntry<'a>>,
        base_entry: &PackedEntry,
        suffix: &str,
    ) -> Result<()> {
        if base_entry.pos() == PartOfSpeech::SubantaPrefix {
            let entry = base_entry.as_packed_subanta_prefix();
            let paradigm = &self
                .subanta_suffixes
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
        } else if base_entry.pos() == PartOfSpeech::TinantaPrefix {
            let entry = base_entry.as_packed_tinanta_prefix();

            /*
            println!(
                "-- get_all_for_suffix: {:?} {:?}",
                entry.dhatu_id(),
                entry.paradigm_id()
            );
            */

            let paradigm = &self
                .tinanta_suffixes
                .get(entry.paradigm_id() as usize)
                .ok_or_else(|| Error::UnknownId("paradigm", entry.paradigm_id() as usize))?;
            let dhatu = self.unpack_dhatu(Id(entry.dhatu_id() as usize))?;

            for ending in &paradigm.endings {
                if ending.text == suffix {
                    ret.push(PadaEntry::Tinanta(TinantaEntry::new(
                        // Stack-allocated, clones are cheap.
                        dhatu.clone(),
                        ending.prayoga,
                        ending.lakara,
                        ending.purusha,
                        ending.vacana,
                    )));
                }
            }
        }

        Ok(())
    }

    /// Increments the count of entries stored in the packer.
    pub(crate) fn increment(&mut self, entry: &PackedEntry) -> Result<()> {
        if entry.pos() == PartOfSpeech::SubantaPrefix {
            let prefix = entry.as_packed_subanta_prefix();
            match self.subanta_suffixes.get(prefix.paradigm_id() as usize) {
                Some(paradigm) => self.count += paradigm.endings.len(),
                None => return Err(Error::UnknownId("paradigm", prefix.paradigm_id() as usize)),
            }
        } else if entry.pos() == PartOfSpeech::TinantaPrefix {
            let prefix = entry.as_packed_tinanta_prefix();
            match self.tinanta_suffixes.get(prefix.paradigm_id() as usize) {
                Some(paradigm) => self.count += paradigm.endings.len(),
                None => return Err(Error::UnknownId("paradigm", prefix.paradigm_id() as usize)),
            }
        } else {
            self.count += 1;
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
            self.dhatu_to_index.insert(dhatu.clone(), id);
            let meta = if let Some(m) = entry.meta {
                m.clone()
            } else {
                DhatuMeta::default()
            };
            self.dhatu_meta.push(meta);

            assert_eq!(self.dhatus.len(), self.dhatu_meta.len());
            assert_eq!(self.dhatus.len(), self.dhatu_to_index.len());

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

            assert!(self.pratipadikas.len() == self.pratipadika_to_index.len());

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

        let paradigm = SubantaSuffixes::from_padas(padas);
        let key = {
            let prefix = padas[0]
                .0
                .strip_suffix(&paradigm.endings[0].text)
                .expect("aligned");
            String::from(prefix)
        };

        let pratipadika_id = self.register_pratipadika_entry(pratipadika);
        let paradigm_id = match self.subanta_suffixes_to_index.get(&paradigm) {
            Some(id) => *id,
            None => {
                let id = Id(self.subanta_suffixes.len());
                self.subanta_suffixes.push(paradigm.clone());
                self.subanta_suffixes_to_index.insert(paradigm, id);
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

    pub(crate) fn register_tinanta_suffixes(
        &mut self,
        dhatu: &DhatuEntry,
        padas: &[(String, Prayoga, Lakara, Purusha, Vacana)],
    ) -> Result<(String, PackedEntry)> {
        let suffixes = TinantaSuffixes::from_padas(padas);
        let key = {
            let prefix = padas[0]
                .0
                .strip_suffix(&suffixes.endings[0].text)
                .expect("aligned");
            String::from(prefix)
        };

        let dhatu_id = self.register_dhatu_entry(dhatu);
        let paradigm_id = match self.tinanta_suffixes_to_index.get(&suffixes) {
            Some(id) => *id,
            None => {
                let id = Id(self.tinanta_suffixes.len());
                self.tinanta_suffixes.push(suffixes.clone());
                self.tinanta_suffixes_to_index.insert(suffixes, id);

                assert_eq!(
                    self.tinanta_suffixes.len(),
                    self.tinanta_suffixes_to_index.len()
                );

                id
            }
        };

        let value = {
            let prefix = PackedTinantaPrefix::new()
                .with_dhatu_id(dhatu_id.0.try_into()?)
                .with_paradigm_id(paradigm_id.0.try_into()?);

            PackedEntry::new()
                .with_pos(PartOfSpeech::TinantaPrefix)
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

    fn pack_subanta(&self, s: &SubantaEntry) -> Result<PackedSubanta> {
        let packed_pratipadika = self.pack_pratipadika(s.pratipadika_entry())?;
        match self.pratipadika_to_index.get(&packed_pratipadika) {
            Some(pratipadika_id) => {
                let sup_id = if s.pratipadika_entry().is_avyaya() {
                    self.sup_to_index
                        .get(&Sup::Avyaya)
                        .ok_or_else(|| Error::NotRegistered("sup"))?
                } else {
                    self.sup_to_index
                        .get(&Sup::new(s.linga(), s.vibhakti(), s.vacana()))
                        .ok_or_else(|| Error::NotRegistered("sup"))?
                };
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
            PadaEntry::Subanta(s) => {
                let payload = self.pack_subanta(s)?;
                PackedEntry::new()
                    .with_pos(PartOfSpeech::Subanta)
                    .with_payload(u32::from_le_bytes(payload.into_bytes()))
            }
            PadaEntry::Tinanta(t) => {
                let payload = self.pack_tinanta(t)?;
                PackedEntry::new()
                    .with_pos(PartOfSpeech::Tinanta)
                    .with_payload(u32::from_le_bytes(payload.into_bytes()))
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
            (Some(dhatu), Some(meta)) => {
                let mut entry = DhatuEntry::new(dhatu);
                entry.meta = Some(meta);
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
        match sup {
            Sup::Basic {
                linga,
                vibhakti,
                vacana,
            } => Ok(SubantaEntry::new(pratipadika, *linga, *vibhakti, *vacana)),
            Sup::Avyaya => Ok(SubantaEntry::avyaya(pratipadika)),
        }
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
            _ => Err(Error::NotRegistered("Prefix")),
        }
    }

    pub(crate) fn debug_print(&self, entry: &PackedEntry) {
        fn create_dhatu_str(dhatu: &Dhatu) -> String {
            let mut ret = String::new();
            if !dhatu.prefixes().is_empty() {
                for (i, prefix) in dhatu.prefixes().iter().enumerate() {
                    if i != 0 {
                        ret.push('-');
                    }
                    ret.push_str(prefix);
                }
                ret.push_str(" + ");
            }

            ret.push_str(dhatu.aupadeshika().unwrap_or("___"));

            if !dhatu.sanadi().is_empty() {
                ret.push_str(" + ");
                for (i, sanadi) in dhatu.sanadi().iter().enumerate() {
                    if i != 0 {
                        ret.push('-');
                    }
                    ret.push_str(sanadi.as_str());
                }
            }

            ret
        }

        let create_krdanta_entry_str = |k: &SmallKrdanta| {
            format!(
                "{} + {}",
                create_dhatu_str(&self.dhatus[k.dhatu_id.0]),
                k.krt_id.0
            )
        };

        let create_pratipadika_entry_str = |p: &SmallPratipadika| match p {
            SmallPratipadika::Basic(b) => {
                format!("(Basic {})", b.text())
            }
            SmallPratipadika::Krdanta(k) => {
                format!("(Krdanta {})", create_krdanta_entry_str(&k))
            }
        };

        let str = match entry.pos() {
            PartOfSpeech::Subanta => {
                let s = entry.as_packed_subanta();
                format!(
                    "(Subanta, pratipadika={}, sup={}) --- {}",
                    s.pratipadika_id(),
                    s.sup_id(),
                    create_pratipadika_entry_str(&self.pratipadikas[s.pratipadika_id() as usize]),
                )
            }
            PartOfSpeech::SubantaPrefix => {
                let s = entry.as_packed_subanta_prefix();
                // let paradigm_id = s.paradigm_id();
                // let paradigm = &kosha.paradigms()[paradigm_id as usize];

                format!(
                    "(SubantaPrefix, pratipadika={}, paradigm={}) --- {}",
                    s.pratipadika_id(),
                    s.paradigm_id(),
                    create_pratipadika_entry_str(&self.pratipadikas[s.pratipadika_id() as usize]),
                )
            }
            PartOfSpeech::Tinanta => {
                let s = entry.as_packed_tinanta();
                format!(
                    "(Tinanta, dhatu={}, tin={}) --- {}",
                    s.dhatu_id(),
                    s.tin_id(),
                    create_dhatu_str(&self.dhatus[s.dhatu_id() as usize]),
                )
            }
            PartOfSpeech::TinantaPrefix => {
                let t = entry.as_packed_tinanta_prefix();
                // let paradigm_id = s.paradigm_id();
                // let paradigm = &kosha.paradigms()[paradigm_id as usize];

                format!(
                    "(TinantaPrefix, dhatu={}, paradigm={}) --- {}",
                    t.dhatu_id(),
                    t.paradigm_id(),
                    create_dhatu_str(&self.dhatus[t.dhatu_id() as usize]),
                )
            }
        };
        println!("{str}");
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
        let gam_meta = DhatuMeta::builder()
            .clean_text("gam".to_string())
            .artha_sa("gatO".to_string())
            .artha_en("go".to_string())
            .artha_hi("जाना".to_string())
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

        let car = Dhatu::mula(safe("cara~"), vp::Gana::Bhvadi);
        let car_meta = DhatuMeta::builder()
            .clean_text("car".to_string())
            .build()
            .expect("ok");
        let car_entry = DhatuEntry::new(&car).with_meta(&car_meta);
        let carati = PadaEntry::Tinanta(TinantaEntry::new(
            car_entry.clone(),
            Prayoga::Kartari,
            Lakara::Lat,
            Purusha::Prathama,
            Vacana::Eka,
        ));

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

        let iti = PadaEntry::Subanta(SubantaEntry::avyaya(iti_entry));
        let ca = PadaEntry::Subanta(SubantaEntry::avyaya(ca_entry));

        let iti_code = packer.pack(&iti)?;
        let ca_code = packer.pack(&ca)?;

        assert_eq!(packer.unpack(&ca_code)?, ca);
        assert_eq!(packer.unpack(&iti_code)?, iti);
        Ok(())
    }
}
