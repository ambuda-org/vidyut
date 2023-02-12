/*!
Code for packing and unpacking Sanskrit morphological data.

**Packing* is the process of converting some data into a dense integer representation. The reverse
process is accordingly called *unpacking*,

Packed data is useful for two reasons. First, packed data takes up less space in memory with little
or no performance penalty. Second, our finite-state transducer can store values only if they are
integers. In other words, packing is a necessary precondition to storing data in an FST.

The downside of packed data is that it cannot easily store string data. To work around this
problem, we can use a lookup table that maps integer indices to string values. But lookup tables
are much more cumbersome than simple structs.

Therefore, we recommend using packed data only when the following conditions obtain:
-


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

use crate::errors::*;
use crate::morph::*;
use modular_bitfield::prelude::*;
use rustc_hash::FxHashMap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Defines boilerplate methods for packing and unpacking enums.
///
/// Requirements: `$Packed` and `$Raw` should be ordinary C-style enums. `$Packed` must have the
/// same exact values as `$Raw` plus an extra `None` value.
macro_rules! boilerplate {
    ($Packed:ident, $Raw:ident, [$( $variant:ident ),*]) => {
        impl From<Option<$Raw>> for $Packed {
            fn from(val: Option<$Raw>) -> $Packed {
                match val {
                    $(
                        Some($Raw::$variant) => Self::$variant,
                    )*
                    None => Self::None,
                }
            }
        }

        impl $Packed {
            /// Unpack this data into its corresponding Raw value.
            fn unpack(&self) -> Option<$Raw> {
                match self {
                    $(
                        $Packed::$variant => Some($Raw::$variant),
                    )*
                    Self::None => None,
                }
            }
        }
    }
}

/// A lookup table for `Dhatu`s.
#[derive(Default, Debug)]
pub struct DhatuTable(Vec<Dhatu>);

impl DhatuTable {
    /// Returns the dhatu at the given index.
    pub fn get(&self, index: usize) -> Option<&Dhatu> {
        self.0.get(index)
    }

    /// Reads this table from disk.
    pub fn read(path: &Path) -> Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);

        let mut ret = Vec::new();
        for line in reader.lines() {
            ret.push(Dhatu(line?.to_string()));
        }
        Ok(Self(ret))
    }

    /// Writes this table to disk.
    pub fn write(&self, path: &Path) -> Result<()> {
        let data: String = self
            .0
            .iter()
            .map(|d| &d.0)
            .fold(String::new(), |x, y| x + y + "\n");
        std::fs::write(path, data)?;

        Ok(())
    }
}

/// A lookup table for `Pratipadika` data.
#[derive(Debug, Default)]
pub struct PratipadikaTable(Vec<Pratipadika>);

impl PratipadikaTable {
    /// Returns the pratipadika at the given index.
    pub fn get(&self, index: usize) -> Option<&Pratipadika> {
        self.0.get(index)
    }

    /// Reads this table from disk.
    pub fn read(path: &Path) -> Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);

        let mut ret = Vec::new();
        for line in reader.lines() {
            ret.push(line?.to_string().parse()?);
        }
        Ok(Self(ret))
    }

    /// Writes this table to disk.
    pub fn write(&self, out: &Path) -> Result<()> {
        let data: String = self
            .0
            .iter()
            .map(Pratipadika::as_str)
            .fold(String::new(), |x, y| x + &y + "\n");

        std::fs::write(out, data)?;
        Ok(())
    }
}

/// Models the part of speech for the given `Pada`. The value of `PartOfSpeech` controls how we
/// interpret the rest of the packed data.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, BitfieldSpecifier)]
#[bits = 2]
enum PartOfSpeech {
    None,
    Subanta,
    Tinanta,
    Avyaya,
}

/// Semantics for an unknown term.
#[bitfield(bits = 30)]
pub struct PackedNone {
    #[skip]
    unused: B30,
}

impl PackedNone {
    #[allow(unused)]
    fn pack() -> Self {
        Self::new()
    }

    #[allow(unused)]
    fn unpack(&self) -> Pada {
        Pada::Unknown
    }
}

/// A space-efficient version of `Linga`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum PackedLinga {
    /// Unknown or missing `Linga`.
    None,
    /// The masculine gender.
    Pum,
    /// The feminine gender.
    Stri,
    /// The neuter gender.
    Napumsaka,
}

boilerplate!(PackedLinga, Linga, [Pum, Stri, Napumsaka]);

/// A space-efficient version of `Vacana`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum PackedVacana {
    /// Unknown or missing vacana.
    None,
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}

boilerplate!(PackedVacana, Vacana, [Eka, Dvi, Bahu]);

/// A space-efficient version of `Vibhakti`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, BitfieldSpecifier)]
#[bits = 4]
pub enum PackedVibhakti {
    /// Unknown or missing vibhakti.
    None,
    /// The first *vibhakti* (nominative case).
    V1,
    /// The second *vibhakti* (accusative case).
    V2,
    /// The third *vibhakti* (instrumental case).
    V3,
    /// The fourth *vibhakti* (dative case).
    V4,
    /// The fifth *vibhakti* (ablative case).
    V5,
    /// The sixth *vibhakti* (genitive case).
    V6,
    /// The seventh *vibhakti* (locative case).
    V7,
    /// The first *vibhakti* in the condition of *sambodhana* (vocative case).
    Sambodhana,
}

boilerplate!(
    PackedVibhakti,
    Vibhakti,
    [V1, V2, V3, V4, V5, V6, V7, Sambodhana]
);

/// Semantics for a *subanta*.
#[bitfield(bits = 30)]
pub struct PackedSubanta {
    linga: PackedLinga,
    vacana: PackedVacana,
    vibhakti: PackedVibhakti,
    is_purvapada: bool,
    pratipadika_id: B21,
}

impl PackedSubanta {
    fn pack(s: &Subanta, pratipadika_id: usize) -> Result<Self> {
        Ok(Self::new()
            .with_pratipadika_id(pratipadika_id.try_into()?)
            .with_linga(s.linga.into())
            .with_vacana(s.vacana.into())
            .with_vibhakti(s.vibhakti.into())
            .with_is_purvapada(s.is_purvapada))
    }

    fn unpack(&self, pratipadikas: &PratipadikaTable) -> Result<Pada> {
        let val = Pada::Subanta(Subanta {
            pratipadika: pratipadikas
                .get(self.pratipadika_id() as usize)
                .ok_or_else(|| Error::UnknownPratipadikaId(self.pratipadika_id()))?
                .clone(),
            linga: self.linga().unpack(),
            vacana: self.vacana().unpack(),
            vibhakti: self.vibhakti().unpack(),
            is_purvapada: self.is_purvapada(),
        });
        Ok(val)
    }
}

/// Semantics for a *tinanta*.
#[bitfield(bits = 30)]
pub struct PackedTinanta {
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    pada: PadaPrayoga,
    dhatu_id: B20,
}

impl PackedTinanta {
    fn pack(t: &Tinanta, dhatu_id: usize) -> Result<Self> {
        Ok(Self::new()
            .with_dhatu_id(dhatu_id.try_into()?)
            .with_lakara(t.lakara)
            .with_purusha(t.purusha)
            .with_vacana(t.vacana)
            .with_pada(t.pada))
    }

    fn unpack(&self, dhatus: &DhatuTable) -> Result<Pada> {
        let val = Pada::Tinanta(Tinanta {
            dhatu: dhatus
                .get(self.dhatu_id() as usize)
                .ok_or_else(|| Error::UnknownDhatuId(self.dhatu_id()))?
                .clone(),
            purusha: self.purusha(),
            lakara: self.lakara(),
            vacana: self.vacana(),
            pada: self.pada(),
        });
        Ok(val)
    }
}

/// Semantics for an *avyaya*.
#[bitfield(bits = 30)]
pub struct PackedAvyaya {
    pratipadika_id: B30,
}

impl PackedAvyaya {
    fn pack(_a: &Avyaya, pratipadika_id: usize) -> Result<Self> {
        Ok(Self::new().with_pratipadika_id(pratipadika_id.try_into()?))
    }

    fn unpack(&self, pratipadikas: &PratipadikaTable) -> Result<Pada> {
        let val = Pada::Avyaya(Avyaya {
            pratipadika: pratipadikas
                .get(self.pratipadika_id() as usize)
                .ok_or_else(|| Error::UnknownPratipadikaId(self.pratipadika_id()))?
                .clone(),
        });
        Ok(val)
    }
}

/// Semantics for a *pada*.
#[bitfield]
pub struct PackedPada {
    /// The part of speech for these semantics. We use this value to decide how to interpret the
    /// `payload` field.
    pos: PartOfSpeech,
    /// The core data for this semantics type. We interpret `paylaad` differently based on the
    /// value of `pos`.
    payload: B30,
}

impl PackedPada {
    /// Unsafely interprets this packed pada as an avyaya.
    pub fn unwrap_as_avyaya(&self) -> PackedAvyaya {
        PackedAvyaya::from_bytes(self.payload().to_le_bytes())
    }

    /// Unsafely interprets this packed pada as a subanta.
    pub fn unwrap_as_subanta(&self) -> PackedSubanta {
        PackedSubanta::from_bytes(self.payload().to_le_bytes())
    }

    /// Unsafely interprets this packed pada as a tinanta.
    pub fn unwrap_as_tinanta(&self) -> PackedTinanta {
        PackedTinanta::from_bytes(self.payload().to_le_bytes())
    }

    /// Unwraps the bitfield as an ordinary integer.
    pub fn to_u32(self) -> u32 {
        u32::from_le_bytes(self.into_bytes())
    }

    /// Wraps an integer value as a `PackedPada` bitfield.
    pub fn from_u32(u: u32) -> Self {
        Self::from_bytes(u.to_le_bytes())
    }
}

/// Packs a `Pada` enum into a u32 code.
#[derive(Clone, Default)]
pub struct Packer {
    /// Maps a pratipadika to its numeric ID.
    stem_mapper: FxHashMap<Pratipadika, usize>,
    /// Maps a dhatu to its numeric ID.
    dhatu_mapper: FxHashMap<Dhatu, usize>,
}

impl Packer {
    /// Creates a new packer with no data.
    pub fn new() -> Self {
        Packer {
            stem_mapper: FxHashMap::default(),
            dhatu_mapper: FxHashMap::default(),
        }
    }

    /// Creates a mapping from integers to dhatus.
    ///
    /// Here, our integers are just the values 0, 1, ..., *n*. So to create a mapping from integers
    /// to dhatus, we can return a simple vector. Then the dhatu at index i implicitly defines a
    /// mpping from i to that dhatu.
    pub fn create_dhatu_table(&self) -> DhatuTable {
        let mut unsorted = self.dhatu_mapper.iter().collect::<Vec<_>>();
        unsorted.sort_by_key(|x| x.1);
        DhatuTable(
            unsorted
                .into_iter()
                .map(|(dhatu, _)| dhatu.clone())
                .collect::<Vec<_>>(),
        )
    }

    /// Creates a mapping from integers to pratipadikas.
    ///
    /// The construction here is similar to what do we do in `create_dhatu_table`.
    pub fn create_stem_table(&self) -> PratipadikaTable {
        let mut unsorted = self.stem_mapper.iter().collect::<Vec<_>>();
        unsorted.sort_by_key(|x| x.1);
        PratipadikaTable(
            unsorted
                .into_iter()
                .map(|(stem, _)| stem.clone())
                .collect::<Vec<_>>(),
        )
    }

    /// Packs the given semantics into an integer value.
    pub fn pack(&mut self, semantics: &Pada) -> Result<PackedPada> {
        let to_u32 = u32::from_le_bytes;

        let val = match semantics {
            Pada::Subanta(s) => {
                let stem_index = self.stem_index_for(&s.pratipadika);
                let payload = PackedSubanta::pack(s, stem_index)?.into_bytes();
                PackedPada::new()
                    .with_pos(PartOfSpeech::Subanta)
                    .with_payload(to_u32(payload))
            }
            Pada::Tinanta(t) => {
                let dhatu_index = self.dhatu_index_for(&t.dhatu);
                let payload = PackedTinanta::pack(t, dhatu_index)?.into_bytes();
                PackedPada::new()
                    .with_pos(PartOfSpeech::Tinanta)
                    .with_payload(to_u32(payload))
            }
            Pada::Avyaya(a) => {
                let stem_index = self.stem_index_for(&a.pratipadika);
                let payload = PackedAvyaya::pack(a, stem_index)?.into_bytes();
                PackedPada::new()
                    .with_pos(PartOfSpeech::Avyaya)
                    .with_payload(to_u32(payload))
            }
            Pada::Unknown => PackedPada::new().with_pos(PartOfSpeech::None),
        };
        Ok(val)
    }

    fn stem_index_for(&mut self, p: &Pratipadika) -> usize {
        if let Some(i) = self.stem_mapper.get(p) {
            *i
        } else {
            let n = self.stem_mapper.len();
            self.stem_mapper.insert(p.clone(), n);
            n
        }
    }

    fn dhatu_index_for(&mut self, d: &Dhatu) -> usize {
        if let Some(i) = self.dhatu_mapper.get(d) {
            *i
        } else {
            let n = self.dhatu_mapper.len();
            self.dhatu_mapper.insert(d.clone(), n);
            n
        }
    }
}

/// Unpacks a u32 code into a `Pada` enum.
pub struct Unpacker {
    pratipadikas: PratipadikaTable,
    dhatus: DhatuTable,
}

impl Unpacker {
    /// Creates an unpacker from the given packer.
    pub fn from_packer(p: &Packer) -> Self {
        Unpacker {
            pratipadikas: p.create_stem_table(),
            dhatus: p.create_dhatu_table(),
        }
    }

    /// Creates an unpacker from the given data.
    pub fn from_data(pratipadikas: PratipadikaTable, dhatus: DhatuTable) -> Self {
        Unpacker {
            pratipadikas,
            dhatus,
        }
    }

    /// Writes this unpacker's data files to disk.
    pub fn write(&self, dhatu_path: &Path, pratipadika_path: &Path) -> Result<()> {
        self.dhatus.write(dhatu_path)?;
        self.pratipadikas.write(pratipadika_path)?;
        Ok(())
    }

    /// Unpacks the given packed pada.
    pub fn unpack(&self, pada: &PackedPada) -> Result<Pada> {
        match pada.pos() {
            PartOfSpeech::Avyaya => pada.unwrap_as_avyaya().unpack(&self.pratipadikas),
            PartOfSpeech::Subanta => pada.unwrap_as_subanta().unpack(&self.pratipadikas),
            PartOfSpeech::Tinanta => pada.unwrap_as_tinanta().unpack(&self.dhatus),
            PartOfSpeech::None => Ok(Pada::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type TestResult = Result<()>;

    #[test]
    fn test_subanta_packing() -> TestResult {
        let devasya = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: "deva".to_owned(),
                lingas: vec![Linga::Pum],
            },
            linga: Some(Linga::Pum),
            vacana: Some(Vacana::Eka),
            vibhakti: Some(Vibhakti::V6),
            is_purvapada: false,
        });
        let narasya = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: "nara".to_owned(),
                lingas: vec![Linga::Pum],
            },
            linga: Some(Linga::Pum),
            vacana: Some(Vacana::Eka),
            vibhakti: Some(Vibhakti::V6),
            is_purvapada: false,
        });

        let mut packer = Packer::new();
        let devasya_code = packer.pack(&devasya)?;
        let narasya_code = packer.pack(&narasya)?;

        let unpacker = Unpacker::from_packer(&packer);
        assert_eq!(unpacker.unpack(&narasya_code)?, narasya);
        assert_eq!(unpacker.unpack(&devasya_code)?, devasya);
        Ok(())
    }

    #[test]
    fn test_tinanta_packing() -> TestResult {
        let gacchati = Pada::Tinanta(Tinanta {
            dhatu: Dhatu("gam".to_string()),
            purusha: Purusha::Prathama,
            vacana: Vacana::Eka,
            lakara: Lakara::Lat,
            pada: PadaPrayoga::Parasmaipada,
        });

        let carati = Pada::Tinanta(Tinanta {
            dhatu: Dhatu("car".to_string()),
            purusha: Purusha::Prathama,
            vacana: Vacana::Eka,
            lakara: Lakara::Lat,
            pada: PadaPrayoga::Parasmaipada,
        });

        let mut packer = Packer::new();
        let gacchati_code = packer.pack(&gacchati)?;
        let carati_code = packer.pack(&carati)?;

        let unpacker = Unpacker::from_packer(&packer);
        assert_eq!(unpacker.unpack(&carati_code)?, carati);
        assert_eq!(unpacker.unpack(&gacchati_code)?, gacchati);
        Ok(())
    }

    #[test]
    fn test_avyaya_packing() -> TestResult {
        let iti = Pada::Avyaya(Avyaya {
            pratipadika: Pratipadika::Basic {
                text: "iti".to_owned(),
                lingas: vec![],
            },
        });
        let ca = Pada::Avyaya(Avyaya {
            pratipadika: Pratipadika::Basic {
                text: "ca".to_owned(),
                lingas: vec![],
            },
        });

        let mut packer = Packer::new();
        let iti_code = packer.pack(&iti)?;
        let ca_code = packer.pack(&ca)?;

        let unpacker = Unpacker::from_packer(&packer);
        assert_eq!(unpacker.unpack(&ca_code)?, ca);
        assert_eq!(unpacker.unpack(&iti_code)?, iti);
        Ok(())
    }
}
