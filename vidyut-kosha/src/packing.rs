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

use crate::entries::*;
use crate::errors::*;
use modular_bitfield::prelude::*;
use rustc_hash::FxHashMap;
use vidyut_prakriya::args::{
    Dhatu, Lakara, Linga, Pratipadika, Prayoga, Purusha, Subanta, Tinanta, Vacana, Vibhakti,
};

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

macro_rules! packed_enum {
    ($Packed:ident, $Raw:ident, [$( $variant:ident ),*], $num_bits:literal ) => {
        #[doc = concat!("A space-efficient version of ", stringify!($Raw))]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, BitfieldSpecifier)]
        #[bits = $num_bits]
        pub enum $Packed {
            $(
                #[doc = concat!("A packed version of ", stringify!($Raw), "::", stringify!($variant), ".")]
                $variant,
            )*
        }

        impl $Packed {
            #[doc = concat!("Converts from ", stringify!($Raw), " to ", stringify!($Packed), ".")]
            pub fn pack(value: $Raw) -> $Packed {
                match value {
                    $(
                        $Raw::$variant => $Packed::$variant,
                    )*
                }
            }

            #[doc = concat!("Converts from ", stringify!($Packed), " to ", stringify!($Raw), ".")]
            pub fn unpack(&self) -> $Raw {
                match self {
                    $(
                        $Packed::$variant => $Raw::$variant,
                    )*
                }
            }
        }

        impl From<$Raw> for $Packed {
            fn from(val: $Raw) -> $Packed {
                $Packed::pack(val)
            }
        }
    };
}

/// Models the part of speech for the given `Pada`. The value of `PartOfSpeech` controls how we
/// interpret the rest of the packed data.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, BitfieldSpecifier)]
#[bits = 2]
enum PartOfSpeech {
    Unknown,
    Subanta,
    Tinanta,
    Avyaya,
}

/// Semantics for an unknown term.
#[bitfield(bits = 30)]
pub struct PackedUnknown {
    #[skip]
    unused: B30,
}

impl PackedUnknown {
    #[allow(unused)]
    fn pack() -> Self {
        Self::new()
    }

    #[allow(unused)]
    fn unpack(&self) -> PadaEntry {
        PadaEntry::Unknown
    }
}

packed_enum!(PackedPrayoga, Prayoga, [Kartari, Karmani, Bhave], 2);
packed_enum!(PackedLinga, Linga, [Pum, Stri, Napumsaka], 2);
packed_enum!(PackedVacana, Vacana, [Eka, Dvi, Bahu], 2);
packed_enum!(
    PackedVibhakti,
    Vibhakti,
    [Prathama, Dvitiya, Trtiya, Caturthi, Panchami, Sasthi, Saptami, Sambodhana],
    4
);
packed_enum!(
    PackedLakara,
    Lakara,
    [Lat, Lit, Lut, Lrt, Let, Lot, Lan, VidhiLin, AshirLin, Lun, Lrn],
    4
);
packed_enum!(PackedPurusha, Purusha, [Prathama, Madhyama, Uttama], 2);

/// Semantics for a *subanta*.
#[bitfield(bits = 30)]
pub struct PackedSubanta {
    linga: PackedLinga,
    vacana: PackedVacana,
    vibhakti: PackedVibhakti,
    is_avyaya: bool,
    pratipadika_id: B21,
}

impl PackedSubanta {
    fn pack(s: &SubantaEntry, pratipadika_id: usize) -> Result<Self> {
        let s = s.subanta();
        Ok(Self::new()
            .with_pratipadika_id(pratipadika_id.try_into()?)
            .with_linga(s.linga().into())
            .with_vacana(s.vacana().into())
            .with_vibhakti(s.vibhakti().into())
            .with_is_avyaya(s.is_avyaya()))
    }

    fn unpack(&self, pratipadikas: &[PratipadikaEntry]) -> Result<PadaEntry> {
        let p_entry = pratipadikas
            .get(self.pratipadika_id() as usize)
            .ok_or_else(|| Error::UnknownPratipadikaId(self.pratipadika_id()))?;
        let subanta = Subanta::builder()
            .pratipadika(p_entry.pratipadika().clone())
            .linga(self.linga().unpack())
            .vacana(self.vacana().unpack())
            .vibhakti(self.vibhakti().unpack())
            .build()
            .expect("has required fields");
        let val = PadaEntry::Subanta(SubantaEntry::new(subanta, p_entry.clone()));
        Ok(val)
    }
}

/// Semantics for a *tinanta*.
#[bitfield(bits = 30)]
pub struct PackedTinanta {
    lakara: PackedLakara,
    purusha: PackedPurusha,
    vacana: PackedVacana,
    prayoga: PackedPrayoga,
    dhatu_id: B20,
}

impl PackedTinanta {
    fn pack(t: &Tinanta, dhatu_id: usize) -> Result<Self> {
        Ok(Self::new()
            .with_dhatu_id(dhatu_id.try_into()?)
            .with_lakara(t.lakara().into())
            .with_purusha(t.purusha().into())
            .with_vacana(t.vacana().into())
            .with_prayoga(t.prayoga().into()))
    }

    fn unpack(&self, dhatus: &[DhatuEntry]) -> Result<PadaEntry> {
        let dhatu = dhatus
            .get(self.dhatu_id() as usize)
            .ok_or_else(|| Error::UnknownDhatuId(self.dhatu_id()))?
            .dhatu()
            .clone();
        let val = PadaEntry::Tinanta(
            Tinanta::builder()
                .dhatu(dhatu)
                .purusha(self.purusha().unpack())
                .lakara(self.lakara().unpack())
                .vacana(self.vacana().unpack())
                .prayoga(self.prayoga().unpack())
                .build()
                .expect("has required fields"),
        );
        Ok(val)
    }
}

/// Semantics for an *avyaya*.
#[bitfield(bits = 30)]
pub struct PackedAvyaya {
    pratipadika_id: B30,
}

impl PackedAvyaya {
    fn pack(pratipadika_id: usize) -> Result<Self> {
        Ok(Self::new().with_pratipadika_id(pratipadika_id.try_into()?))
    }

    fn unpack(&self, pratipadikas: &[PratipadikaEntry]) -> Result<PadaEntry> {
        let p_entry = pratipadikas
            .get(self.pratipadika_id() as usize)
            .ok_or_else(|| Error::UnknownPratipadikaId(self.pratipadika_id()))?;

        let pratipadika = p_entry.pratipadika().clone();
        let subanta = Subanta::avyaya(pratipadika);
        let val = PadaEntry::Avyaya(SubantaEntry::new(subanta, p_entry.clone()));
        Ok(val)
    }
}

/// Semantics for a *pada*.
#[bitfield]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PackedEntry {
    /// The part of speech for these semantics. We use this value to decide how to interpret the
    /// `payload` field.
    pos: PartOfSpeech,
    /// The core data for this semantics type. We interpret `paylaad` differently based on the
    /// value of `pos`.
    payload: B30,
}

impl PackedEntry {
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

    /// Unwraps the bitfield as an ordinary `u32.
    pub fn to_u32(self) -> u32 {
        u32::from_le_bytes(self.into_bytes())
    }

    /// Wraps an integer value as a `PackedPada` bitfield.
    pub fn from_u32(u: u32) -> Self {
        Self::from_bytes(u.to_le_bytes())
    }
}

/// Packs and unpacks linguistic data.
#[derive(Clone, Default)]
pub struct Packer {
    dhatus: Vec<DhatuEntry>,
    pratipadikas: Vec<PratipadikaEntry>,
    pratipadika_to_index: FxHashMap<Pratipadika, usize>,
    dhatu_to_index: FxHashMap<Dhatu, usize>,
}

impl Packer {
    /// Creates a new packer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads a packer from disk.
    pub fn read(dhatu_path: impl AsRef<Path>, pratipadika_path: impl AsRef<Path>) -> Result<Self> {
        Self::read_inner(dhatu_path.as_ref(), pratipadika_path.as_ref())
    }

    fn read_inner(dhatu_path: &Path, pratipadika_path: &Path) -> Result<Self> {
        let file = File::open(dhatu_path)?;
        let reader = BufReader::new(file);
        let dhatus: Vec<DhatuEntry> = serde_json::from_reader(reader)?;

        let file = File::open(pratipadika_path)?;
        let reader = BufReader::new(file);
        let pratipadikas: Vec<PratipadikaEntry> = serde_json::from_reader(reader)?;

        let pratipadika_to_index: FxHashMap<_, _> = pratipadikas
            .iter()
            .enumerate()
            .map(|(i, x)| (x.pratipadika().clone(), i))
            .collect();
        let dhatu_to_index: FxHashMap<_, _> = dhatus
            .iter()
            .enumerate()
            .map(|(i, x)| (x.dhatu().clone(), i))
            .collect();

        Ok(Self {
            dhatus,
            pratipadikas,
            pratipadika_to_index,
            dhatu_to_index,
        })
    }

    /// Writes daat in the registry to disk.
    pub fn write(
        &self,
        dhatu_path: impl AsRef<Path>,
        pratipadika_path: impl AsRef<Path>,
    ) -> Result<()> {
        self.write_inner(dhatu_path.as_ref(), pratipadika_path.as_ref())
    }

    fn write_inner(&self, dhatu_path: &Path, pratipadika_path: &Path) -> Result<()> {
        let file = File::create(dhatu_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.dhatus)?;

        let file = File::create(pratipadika_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.pratipadikas)?;

        Ok(())
    }

    /// Registers the given dhatus on the packer. Duplicate dhatus are ignored.
    pub fn register_dhatus(&mut self, entries: &[DhatuEntry]) {
        let n = self.dhatus.len();
        for e in entries {
            if !self.dhatu_to_index.contains_key(&e.dhatu()) {
                self.dhatus.push(e.clone());
            }
        }

        for (i, d) in self.dhatus[n..].iter().enumerate() {
            self.dhatu_to_index.insert(d.dhatu().clone(), n + i);
        }
    }

    /// Registers the given pratipadikas on the packer. Duplicate pratipadikas are ignored.
    pub fn register_pratipadikas(&mut self, entries: &[PratipadikaEntry]) {
        let n = self.pratipadikas.len();
        for e in entries {
            if !self.pratipadika_to_index.contains_key(&e.pratipadika()) {
                self.pratipadikas.push(e.clone());
            }
        }

        for (i, p) in self.pratipadikas[n..].iter().enumerate() {
            self.pratipadika_to_index
                .insert(p.pratipadika().clone(), n + i);
        }
    }

    /// Packs the given semantics into an integer value.
    pub fn pack(&self, pada: &PadaEntry) -> Result<PackedEntry> {
        let to_u32 = u32::from_le_bytes;

        let val = match pada {
            PadaEntry::Avyaya(a) => {
                match self.pratipadika_to_index.get(a.subanta().pratipadika()) {
                    Some(i) => {
                        let payload = PackedAvyaya::pack(*i)?.into_bytes();
                        PackedEntry::new()
                            .with_pos(PartOfSpeech::Avyaya)
                            .with_payload(to_u32(payload))
                    }
                    None => return Err(Error::Generic("Pratipadika not in index.".to_string())),
                }
            }
            PadaEntry::Subanta(s) => match self.pratipadika_to_index.get(s.subanta().pratipadika())
            {
                Some(i) => {
                    let payload = PackedSubanta::pack(s, *i)?.into_bytes();
                    PackedEntry::new()
                        .with_pos(PartOfSpeech::Subanta)
                        .with_payload(to_u32(payload))
                }
                None => return Err(Error::Generic("Pratipadika not in index.".to_string())),
            },
            PadaEntry::Tinanta(t) => match self.dhatu_to_index.get(t.dhatu()) {
                Some(i) => {
                    let payload = PackedTinanta::pack(t, *i)?.into_bytes();
                    PackedEntry::new()
                        .with_pos(PartOfSpeech::Tinanta)
                        .with_payload(to_u32(payload))
                }
                None => return Err(Error::Generic("Dhatu not in index.".to_string())),
            },
            PadaEntry::Unknown => PackedEntry::new().with_pos(PartOfSpeech::Unknown),
        };
        Ok(val)
    }

    /// Unpacks the given packed pada.
    pub fn unpack(&self, pada: &PackedEntry) -> Result<PadaEntry> {
        match pada.pos() {
            PartOfSpeech::Avyaya => pada.unwrap_as_avyaya().unpack(&self.pratipadikas),
            PartOfSpeech::Subanta => pada.unwrap_as_subanta().unpack(&self.pratipadikas),
            PartOfSpeech::Tinanta => pada.unwrap_as_tinanta().unpack(&self.dhatus),
            PartOfSpeech::Unknown => Ok(PadaEntry::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vidyut_prakriya::args as vp;
    type TestResult = Result<()>;

    fn safe(s: &str) -> vp::Slp1String {
        vp::Slp1String::from(s).expect("static")
    }

    fn entry(p: Pratipadika) -> PratipadikaEntry {
        PratipadikaEntry::new(p, vec![])
    }

    #[test]
    fn test_subanta_packing() -> TestResult {
        let deva = Pratipadika::basic(safe("deva"));
        let nara = Pratipadika::basic(safe("nara"));
        let devasya = PadaEntry::Subanta(
            Subanta::new(deva.clone(), Linga::Pum, Vibhakti::Sasthi, Vacana::Eka).into(),
        );
        let narasya = PadaEntry::Subanta(
            Subanta::new(nara.clone(), Linga::Pum, Vibhakti::Sasthi, Vacana::Eka).into(),
        );

        let mut packer = Packer::new();
        packer.register_pratipadikas(&[entry(deva), entry(nara)]);

        let devasya_code = packer.pack(&devasya)?;
        let narasya_code = packer.pack(&narasya)?;
        assert_eq!(packer.unpack(&narasya_code)?, narasya);
        assert_eq!(packer.unpack(&devasya_code)?, devasya);
        Ok(())
    }

    #[test]
    fn test_tinanta_packing() -> TestResult {
        let gacchati = PadaEntry::Tinanta(
            Tinanta::builder()
                .dhatu(Dhatu::mula(safe("ga\\mx~"), vp::Gana::Bhvadi))
                .purusha(Purusha::Prathama)
                .vacana(Vacana::Eka)
                .lakara(Lakara::Lat)
                .prayoga(Prayoga::Kartari)
                .build()
                .unwrap(),
        );

        let carati = PadaEntry::Tinanta(
            Tinanta::builder()
                .dhatu(Dhatu::mula(safe("cara~"), vp::Gana::Bhvadi))
                .purusha(Purusha::Prathama)
                .vacana(Vacana::Eka)
                .lakara(Lakara::Lat)
                .prayoga(Prayoga::Kartari)
                .build()
                .unwrap(),
        );

        let mut packer = Packer::new();
        packer.register_dhatus(&[
            DhatuEntry::new(
                Dhatu::mula(safe("ga\\mx~"), vp::Gana::Bhvadi),
                "gam".to_string(),
            ),
            DhatuEntry::new(
                Dhatu::mula(safe("cara~"), vp::Gana::Bhvadi),
                "car".to_string(),
            ),
        ]);
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
        let iti = PadaEntry::Avyaya(Subanta::avyaya(iti_stem.clone()).into());
        let ca = PadaEntry::Avyaya(Subanta::avyaya(ca_stem.clone()).into());

        let mut packer = Packer::new();
        packer.register_pratipadikas(&[entry(iti_stem), entry(ca_stem)]);
        let iti_code = packer.pack(&iti)?;
        let ca_code = packer.pack(&ca)?;

        assert_eq!(packer.unpack(&ca_code)?, ca);
        assert_eq!(packer.unpack(&iti_code)?, iti);
        Ok(())
    }
}
