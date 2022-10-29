//! Utilities for converting between `Pada` and u32 bitfields.
//!
//! We use these utilities both to save memory and to support storing dictionary keys in a finite
//! state transducer through the `fst` crate. For details, see `fst_lexicon`.

// TODO: streamline semantics into subanta, tinanta, and avyaya

use crate::semantics::*;
use modular_bitfield::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// A generic error related to packing or unpacking data.
#[derive(Debug, Clone)]
pub struct PackingError {
    msg: String,
}
impl PackingError {
    fn new(s: &str) -> Self {
        PackingError { msg: s.to_owned() }
    }
}
impl Error for PackingError {}
impl fmt::Display for PackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

/// A lookup table for root data.
pub struct RootTable(Vec<String>);

impl RootTable {
    pub fn get(&self, index: usize) -> Option<&String> {
        self.0.get(index)
    }
}

/// A lookup table for `Stem` data.
pub struct StemTable(Vec<Pratipadika>);

impl StemTable {
    pub fn get(&self, index: usize) -> Option<&Pratipadika> {
        self.0.get(index)
    }
}

#[derive(BitfieldSpecifier)]
#[bits = 2]
enum PartOfSpeech {
    None,
    Subanta,
    Tinanta,
}

/// Semantics for a *tinanta*.
#[bitfield(bits = 30)]
pub struct PackedTinanta {
    root_id: B20,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    pada: PadaPrayoga,
}

impl PackedTinanta {
    fn pack(t: &Tinanta, root_id: usize) -> Self {
        PackedTinanta::new()
            .with_root_id(root_id.try_into().unwrap())
            .with_lakara(t.lakara)
            .with_purusha(t.purusha)
            .with_vacana(t.vacana)
            .with_pada(t.pada)
    }

    fn unpack(&self, root_table: &RootTable) -> Result<Pada, Box<dyn Error>> {
        let val = Pada::Tinanta(Tinanta {
            dhatu: Dhatu(
                root_table
                    .get(self.root_id() as usize)
                    .expect("Unknown string")
                    .clone(),
            ),
            purusha: self.purusha(),
            lakara: self.lakara(),
            vacana: self.vacana(),
            pada: self.pada(),
        });
        Ok(val)
    }
}

/// Semantics for a *subanta*.
#[bitfield(bits = 30)]
pub struct PackedSubanta {
    stem_id: B21,
    linga: Linga,
    vacana: Vacana,
    vibhakti: Vibhakti,
    is_purvapada: bool,
}
impl PackedSubanta {
    fn pack(s: &Subanta, stem_id: usize) -> Self {
        PackedSubanta::new()
            .with_stem_id(stem_id.try_into().unwrap())
            .with_linga(s.linga)
            .with_vacana(s.vacana)
            .with_vibhakti(s.vibhakti)
            .with_is_purvapada(s.is_purvapada)
    }

    fn unpack(&self, stem_table: &StemTable) -> Result<Pada, Box<dyn Error>> {
        let val = Pada::Subanta(Subanta {
            pratipadika: stem_table
                .get(self.stem_id() as usize)
                .expect("Unknown stem")
                .clone(),
            linga: self.linga(),
            vacana: self.vacana(),
            vibhakti: self.vibhakti(),
            is_purvapada: self.is_purvapada(),
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
    pub fn unwrap_as_subanta(&self) -> PackedSubanta {
        PackedSubanta::from_bytes(self.payload().to_be_bytes())
    }
    pub fn unwrap_as_tinanta(&self) -> PackedTinanta {
        PackedTinanta::from_bytes(self.payload().to_be_bytes())
    }
}

/// Packs a `Pada` enum into a u32 code.
pub struct Packer {
    stem_mapper: HashMap<Pratipadika, usize>,
    root_mapper: HashMap<String, usize>,
}

impl Packer {
    pub fn from_data(
        stem_mapper: HashMap<Pratipadika, usize>,
        root_mapper: HashMap<String, usize>,
    ) -> Packer {
        Packer {
            stem_mapper,
            root_mapper,
        }
    }

    pub fn pack(&self, semantics: &Pada) -> Result<u32, PackingError> {
        let to_u32 = u32::from_be_bytes;

        let val = match semantics {
            Pada::Subanta(s) => match self.stem_mapper.get(&s.pratipadika) {
                Some(i) => {
                    let payload = PackedSubanta::pack(s, *i).into_bytes();
                    PackedPada::new()
                        .with_pos(PartOfSpeech::Subanta)
                        .with_payload(to_u32(payload))
                }
                None => return Err(PackingError::new("Could not find stem in mapper.")),
            },
            Pada::Tinanta(t) => match self.root_mapper.get(&t.dhatu.0) {
                Some(i) => {
                    let payload = PackedTinanta::pack(t, *i).into_bytes();
                    PackedPada::new()
                        .with_pos(PartOfSpeech::Tinanta)
                        .with_payload(to_u32(payload))
                }
                None => return Err(PackingError::new("Could not find root in mapper.")),
            },
            _ => panic!("Unknown semantics"),
        };
        Ok(to_u32(val.into_bytes()))
    }
}

/// Unpacks a u32 code into a `Pada` enum.
pub struct Unpacker {
    stem_table: StemTable,
    root_table: RootTable,
}

impl Unpacker {
    pub fn from_data(stem_table: StemTable, root_table: RootTable) -> Unpacker {
        Unpacker {
            stem_table,
            root_table,
        }
    }

    pub fn unpack(&self, code: u32) -> Result<Pada, Box<dyn Error>> {
        let pada = PackedPada::from_bytes(code.to_be_bytes());

        // FIXME: magic values.
        match pada.pos() {
            PartOfSpeech::Subanta => pada.unwrap_as_subanta().unpack(&self.stem_table),
            PartOfSpeech::Tinanta => pada.unwrap_as_tinanta().unpack(&self.root_table),
            PartOfSpeech::None => Ok(Pada::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type TestResult = Result<(), Box<dyn Error>>;

    fn new_packer() -> Packer {
        let root_mapper = HashMap::from([("gam".to_string(), 0_usize)]);
        let nara = Pratipadika::Basic {
            text: "nara".to_owned(),
            lingas: vec![Linga::Pum],
        };
        let stem_mapper = HashMap::from([(nara, 0_usize)]);
        Packer::from_data(stem_mapper, root_mapper)
    }

    fn new_unpacker() -> Unpacker {
        let root_table = RootTable(vec!["gam".to_string()]);
        let stem_table = StemTable(vec![Pratipadika::Basic {
            text: "nara".to_owned(),
            lingas: vec![Linga::Pum],
        }]);
        Unpacker::from_data(stem_table, root_table)
    }

    #[test]
    fn test_tinanta_packing() -> TestResult {
        let semantics = Pada::Tinanta(Tinanta {
            dhatu: Dhatu("gam".to_string()),
            purusha: Purusha::Prathama,
            vacana: Vacana::Eka,
            lakara: Lakara::Lat,
            pada: PadaPrayoga::Parasmaipada,
        });

        let packer = new_packer();
        let unpacker = new_unpacker();

        let code = packer.pack(&semantics)?;
        assert_eq!(unpacker.unpack(code)?, semantics);
        Ok(())
    }

    #[test]
    fn test_subanta_packing() -> TestResult {
        let semantics = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: "nara".to_owned(),
                lingas: vec![Linga::Pum],
            },
            linga: Linga::Pum,
            vacana: Vacana::Eka,
            vibhakti: Vibhakti::V1,
            is_purvapada: false,
        });

        let packer = new_packer();
        let unpacker = new_unpacker();

        let code = packer.pack(&semantics)?;
        assert_eq!(unpacker.unpack(code)?, semantics);
        Ok(())
    }
}
