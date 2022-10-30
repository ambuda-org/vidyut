//! Utilities for converting between `Pada` and u32 bitfields.
//!
//! We use these utilities both to save memory and to support storing dictionary keys in a finite
//! state transducer through the `fst` crate. For details, see `fst_lexicon`.
//!
//! For consistency with the crate defaults in `modular_bitfield`, we treat all packed data as
//! little-endian and hence use `from_le_bytes` and `to_le_bytes`.
//!
//! TODO: investigate different packing orders to see if we can reduce the size of the FST.

use crate::semantics::*;
use modular_bitfield::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A generic error related to packing or unpacking data.
#[derive(Debug, Clone)]
pub struct PackingError {
    msg: String,
}

impl PackingError {
    fn new(s: &str) -> Self {
        PackingError { msg: s.to_owned() }
    }

    fn unknown_pratipadika_id(id: u32) -> PackingError {
        Self::new(&format!("Unknown pratipadika ID {}", id))
    }

    fn unknown_dhatu_id(id: u32) -> PackingError {
        Self::new(&format!("Unknown dhatu ID {}", id))
    }
}

impl Error for PackingError {}

impl fmt::Display for PackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

/// A lookup table for `Dhatu`s.
#[derive(Debug)]
pub struct DhatuTable(Vec<Dhatu>);

impl DhatuTable {
    pub fn default() -> Self {
        DhatuTable(Vec::new())
    }
    pub fn get(&self, index: usize) -> Option<&Dhatu> {
        self.0.get(index)
    }

    /// Reads this table from disk.
    pub fn read(path: &Path) -> Result<Self, Box<dyn Error>> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);

        let mut ret = Vec::new();
        for line in reader.lines() {
            ret.push(Dhatu(line?.to_string()));
        }
        Ok(Self(ret))
    }

    /// Writes this table to disk.
    pub fn write(&self, path: &Path) -> Result<(), Box<dyn Error>> {
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
#[derive(Debug)]
pub struct PratipadikaTable(Vec<Pratipadika>);

impl PratipadikaTable {
    pub fn default() -> Self {
        PratipadikaTable(Vec::new())
    }
    pub fn get(&self, index: usize) -> Option<&Pratipadika> {
        self.0.get(index)
    }

    /// Reads this table from disk.
    pub fn read(path: &Path) -> Result<Self, Box<dyn Error>> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);

        let mut ret = Vec::new();
        for line in reader.lines() {
            ret.push(line?.to_string().parse()?);
        }
        Ok(Self(ret))
    }

    /// Writes this table to disk.
    pub fn write(&self, out: &Path) -> Result<(), Box<dyn Error>> {
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
#[derive(BitfieldSpecifier)]
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
    pub fn pack() -> Self {
        Self::new()
    }
    pub fn unpack(&self) -> Pada {
        Pada::None
    }
}

/// Semantics for a *subanta*.
#[bitfield(bits = 30)]
pub struct PackedSubanta {
    linga: Linga,
    vacana: Vacana,
    vibhakti: Vibhakti,
    is_purvapada: bool,
    pratipadika_id: B21,
}
impl PackedSubanta {
    fn pack(s: &Subanta, pratipadika_id: usize) -> Self {
        Self::new()
            .with_pratipadika_id(pratipadika_id.try_into().unwrap())
            .with_linga(s.linga)
            .with_vacana(s.vacana)
            .with_vibhakti(s.vibhakti)
            .with_is_purvapada(s.is_purvapada)
    }

    fn unpack(&self, pratipadikas: &PratipadikaTable) -> Result<Pada, Box<dyn Error>> {
        let val = Pada::Subanta(Subanta {
            pratipadika: pratipadikas
                .get(self.pratipadika_id() as usize)
                .ok_or_else(|| PackingError::unknown_pratipadika_id(self.pratipadika_id()))?
                .clone(),
            linga: self.linga(),
            vacana: self.vacana(),
            vibhakti: self.vibhakti(),
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
    fn pack(t: &Tinanta, dhatu_id: usize) -> Self {
        Self::new()
            .with_dhatu_id(dhatu_id.try_into().unwrap())
            .with_lakara(t.lakara)
            .with_purusha(t.purusha)
            .with_vacana(t.vacana)
            .with_pada(t.pada)
    }

    fn unpack(&self, dhatus: &DhatuTable) -> Result<Pada, Box<dyn Error>> {
        let val = Pada::Tinanta(Tinanta {
            dhatu: dhatus
                .get(self.dhatu_id() as usize)
                .ok_or_else(|| PackingError::unknown_dhatu_id(self.dhatu_id()))?
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
    fn pack(_a: &Avyaya, pratipadika_id: usize) -> Self {
        Self::new().with_pratipadika_id(pratipadika_id.try_into().unwrap())
    }

    fn unpack(&self, pratipadikas: &PratipadikaTable) -> Result<Pada, Box<dyn Error>> {
        let val = Pada::Avyaya(Avyaya {
            pratipadika: pratipadikas
                .get(self.pratipadika_id() as usize)
                .ok_or_else(|| PackingError::unknown_pratipadika_id(self.pratipadika_id()))?
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
    // FIXME: this is unsafe -- what if this is called on a tinanta?
    pub fn unwrap_as_avyaya(&self) -> PackedAvyaya {
        PackedAvyaya::from_bytes(self.payload().to_le_bytes())
    }
    pub fn unwrap_as_subanta(&self) -> PackedSubanta {
        PackedSubanta::from_bytes(self.payload().to_le_bytes())
    }
    pub fn unwrap_as_tinanta(&self) -> PackedTinanta {
        PackedTinanta::from_bytes(self.payload().to_le_bytes())
    }

    pub fn to_u32(self) -> u32 {
        u32::from_le_bytes(self.into_bytes())
    }

    pub fn from_u32(u: u32) -> Self {
        Self::from_bytes(u.to_le_bytes())
    }
}

/// Packs a `Pada` enum into a u32 code.
pub struct Packer {
    stem_mapper: HashMap<Pratipadika, usize>,
    dhatu_mapper: HashMap<Dhatu, usize>,
}

impl Packer {
    /// Creates a new packer with no data.
    pub fn new() -> Self {
        Packer {
            stem_mapper: HashMap::new(),
            dhatu_mapper: HashMap::new(),
        }
    }

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

    /// Pack the given semantics into a u32
    pub fn pack(&mut self, semantics: &Pada) -> PackedPada {
        let to_u32 = u32::from_le_bytes;

        match semantics {
            Pada::Subanta(s) => {
                let stem_index = self.stem_index_for(&s.pratipadika);
                let payload = PackedSubanta::pack(s, stem_index).into_bytes();
                PackedPada::new()
                    .with_pos(PartOfSpeech::Subanta)
                    .with_payload(to_u32(payload))
            }
            Pada::Tinanta(t) => {
                let dhatu_index = self.dhatu_index_for(&t.dhatu);
                let payload = PackedTinanta::pack(t, dhatu_index).into_bytes();
                PackedPada::new()
                    .with_pos(PartOfSpeech::Tinanta)
                    .with_payload(to_u32(payload))
            }
            Pada::Avyaya(a) => {
                let stem_index = self.stem_index_for(&a.pratipadika);
                let payload = PackedAvyaya::pack(a, stem_index).into_bytes();
                PackedPada::new()
                    .with_pos(PartOfSpeech::Avyaya)
                    .with_payload(to_u32(payload))
            }
            Pada::None => PackedPada::new().with_pos(PartOfSpeech::None),
            _ => panic!("Unknown semantics"),
        }
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
impl Default for Packer {
    fn default() -> Self {
        Self::new()
    }
}

/// Unpacks a u32 code into a `Pada` enum.
pub struct Unpacker {
    pratipadikas: PratipadikaTable,
    dhatus: DhatuTable,
}

impl Unpacker {
    pub fn from_packer(p: &Packer) -> Self {
        Unpacker {
            pratipadikas: p.create_stem_table(),
            dhatus: p.create_dhatu_table(),
        }
    }

    pub fn write(&self, dhatu_path: &Path, pratipadika_path: &Path) -> Result<(), Box<dyn Error>> {
        self.dhatus.write(dhatu_path)?;
        self.pratipadikas.write(pratipadika_path)?;
        Ok(())
    }

    pub fn from_data(pratipadikas: PratipadikaTable, dhatus: DhatuTable) -> Self {
        Unpacker {
            pratipadikas,
            dhatus,
        }
    }

    pub fn unpack(&self, pada: &PackedPada) -> Result<Pada, Box<dyn Error>> {
        match pada.pos() {
            PartOfSpeech::Avyaya => pada.unwrap_as_avyaya().unpack(&self.pratipadikas),
            PartOfSpeech::Subanta => pada.unwrap_as_subanta().unpack(&self.pratipadikas),
            PartOfSpeech::Tinanta => pada.unwrap_as_tinanta().unpack(&self.dhatus),
            PartOfSpeech::None => Ok(Pada::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn test_subanta_packing() -> TestResult {
        let devasya = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: "deva".to_owned(),
                lingas: vec![Linga::Pum],
            },
            linga: Linga::Pum,
            vacana: Vacana::Eka,
            vibhakti: Vibhakti::V6,
            is_purvapada: false,
        });
        let narasya = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: "nara".to_owned(),
                lingas: vec![Linga::Pum],
            },
            linga: Linga::Pum,
            vacana: Vacana::Eka,
            vibhakti: Vibhakti::V6,
            is_purvapada: false,
        });

        let mut packer = Packer::new();
        let devasya_code = packer.pack(&devasya);
        let narasya_code = packer.pack(&narasya);

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
        let gacchati_code = packer.pack(&gacchati);
        let carati_code = packer.pack(&carati);

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
        let iti_code = packer.pack(&iti);
        let ca_code = packer.pack(&ca);

        let unpacker = Unpacker::from_packer(&packer);
        assert_eq!(unpacker.unpack(&ca_code)?, ca);
        assert_eq!(unpacker.unpack(&iti_code)?, iti);
        Ok(())
    }
}
