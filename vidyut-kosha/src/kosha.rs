//! A memory-efficient Sanskrit kosha (lexicon) with basic support for searching prefixes.
//!
//!
//! Implementation
//! --------------
//!
//! We implement our kosha as a finite state transducer using the `fst` crate. Finite state
//! transducers are a generalization of tries in that they support both shared prefixes and shared
//! suffixes.
//!
//! The `fst` crate that we use internally has two important restrictions that affect the design of
//! our kosha:
//!
//! 1. Each key can be stored at most once.
//! 2. Each value must be an unsigned 64 bit integer.
//!
//! To work around (1), we mark synonyms by appending additional bytes to the key. Although this
//! approach is hacky, it generally works well, as duplicate keys generally follow systematic
//! patterns that the FST can exploit during construction.
//!
//! (Our current implementation supports up to 4226 instances of a key. For implementation
//! details, see `create_extended_key` and its tests.)
//!
//! To work around (2), we pack the semantics of Sanskrit words into integers with our
//! `vidyut::packing` crate. Since strings are difficult to pack, we instead store them in a lookup
//! table and pack their integer ID instead.
//!
//!
//! Efficiency
//! ----------
//!
//! According to our benchmarks, an average Sanskrit word can be retrieved in around 500ns and is
//! roughly 1.5x slower than a default `HashMap`. Our production kosha stores more than 29 million
//! words in around 31MB of data with an average storage cost of 1 byte per word. Of course, the
//! specific storage cost will vary depending on the words in the input list.
use crate::entries::{DhatuEntry, PadaEntry, PratipadikaEntry};
use crate::errors::*;
use crate::packing::*;
use fst::map::Stream;
use fst::raw::{Fst, Node, Output};
use fst::{Map, MapBuilder};
use log::info;
use rustc_hash::FxHashMap;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

// Use the range [0, 64] to avoid confusion with the ASCII range, which starts at 65 (01000001,
// i.e. uppercase `A`).
const DUPES_PER_BYTE: u8 = 65;
const MAX_DUPLICATES: usize = (DUPES_PER_BYTE as usize) * (DUPES_PER_BYTE as usize);

struct Paths {
    /// Path to the data dir for this kosha. If writing, the writer will create the directory if
    /// it does not exist already.
    base: PathBuf,
}

impl Paths {
    fn new(base_path: impl AsRef<Path>) -> Self {
        Paths {
            base: base_path.as_ref().to_path_buf(),
        }
    }

    /// Path to the underlying FST.
    fn fst(&self) -> PathBuf {
        self.base.join("padas.fst")
    }

    /// Path to the dhatus registry.
    fn dhatus(&self) -> PathBuf {
        self.base.join("dhatus.json")
    }

    /// Path to the pratipadikas registry.
    fn pratipadikas(&self) -> PathBuf {
        self.base.join("pratipadikas.json")
    }
}

fn to_packed_pada(output: Output) -> PackedEntry {
    PackedEntry::from_u32(output.value() as u32)
}

/// A compact Sanskrit kosha.
pub struct Kosha {
    /// The underlying FST object.
    fst: Map<Vec<u8>>,
    /// Maps indices to semantics objects.
    packer: Packer,
}

impl Kosha {
    /// Reads the kosha from the given `base_path`.
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let paths = Paths::new(base_path);

        info!("Loading fst from `{:?}`", paths.fst());
        let fst = Map::new(std::fs::read(paths.fst())?)?;
        let packer = Packer::read(&paths.dhatus(), &paths.pratipadikas())?;

        Ok(Self { fst, packer })
    }

    /// Returns a reference to this kosha's underlying FST.
    pub fn fst(&self) -> &Map<Vec<u8>> {
        &self.fst
    }

    /// Returns whether this kosha contains at least one word with exact value `key`.
    #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.fst.contains_key(key)
    }

    /// Returns whether the kosha contains at least one word that starts with `key`.
    ///
    /// Prefix checks are slightly faster than ordinary key lookups. I also tried implementing this
    /// with `fst::Stream`, but that approach was much slower than just accessing the FST directly,
    /// which is what we do here.
    #[inline]
    pub fn contains_prefix(&self, key: &str) -> bool {
        // Adapted from `FstRef::get`
        let fst = self.fst.as_fst();
        let mut node = fst.root();
        for &b in key.as_bytes() {
            node = match node.find_input(b) {
                None => return false,
                Some(i) => {
                    let t = node.transition(i);
                    fst.node(t.addr)
                }
            }
        }
        true
    }

    /// Unpacks the given word via this kosha's `Unpacker` instance.
    pub fn unpack(&self, p: &PackedEntry) -> Result<PadaEntry> {
        self.packer.unpack(p)
    }

    /// Gets all results for the given `key`, including duplicates.
    #[inline]
    pub fn get_all(&self, key: &str) -> Vec<PackedEntry> {
        // Adapted from `FstRef::get`
        // https://docs.rs/fst/0.4.7/src/fst/raw/mod.rs.html#682
        let fst = self.fst.as_fst();
        let mut node = fst.root();
        let mut out = Output::zero();
        for &b in key.as_bytes() {
            node = match node.find_input(b) {
                None => return Vec::new(),
                Some(i) => {
                    let t = node.transition(i);
                    out = out.cat(t.out);
                    fst.node(t.addr)
                }
            }
        }

        if node.is_final() {
            let mut ret = vec![to_packed_pada(out.cat(node.final_output()))];
            add_duplicates(node, out, fst, &mut ret);

            ret
        } else {
            Vec::new()
        }
    }

    /// Iterates over all keys in the FST.
    pub fn stream(&self) -> Stream<'_> {
        self.fst.stream()
    }
}

/// Appends all available duplicates to our list of results.
///
/// Args:
/// - `node`: the node corresponding to the last ASCII character of the input string.
/// - `out`: the output corresponding to this state.
/// - `fst`: the underlying FST.
/// - `results`: the results list.
fn add_duplicates(node: Node, out: Output, fst: &Fst<Vec<u8>>, results: &mut Vec<PackedEntry>) {
    for c1 in 0..=DUPES_PER_BYTE {
        if let Some(i1) = node.find_input(c1) {
            let t1 = node.transition(i1);
            let o1 = out.cat(t1.out);
            let n1 = fst.node(t1.addr);

            for c2 in 0..=DUPES_PER_BYTE {
                if let Some(i2) = n1.find_input(c2) {
                    let t2 = n1.transition(i2);
                    let o2 = o1.cat(t2.out);
                    let n2 = fst.node(t2.addr);

                    // In our current scheme, this node is always final.
                    if n2.is_final() {
                        let result = to_packed_pada(o2.cat(n2.final_output()));
                        results.push(result);
                    }
                } else {
                    return;
                }
            }
        } else {
            return;
        }
    }
}

/// Builder for a `Kosha`.
///
/// Memory usage is linear in the number of unique lemmas (`Dhatu`s or `Pratipadika`s).
pub struct Builder {
    seen_keys: FxHashMap<String, usize>,
    fst_builder: MapBuilder<io::BufWriter<File>>,
    packer: Packer,
    paths: Paths,
}

/// Create an extended insertion key (for duplicates).
///
/// The output of this function must respect the FST's insertion constraint on key orderings. That
/// is, if a key is extended with indices `m` and `n`, `m < n` iff `extended_m` < `extended_n`.
///
/// For this reason, we cannot easily support an arbitrary number of duplicate keys. Instead,
/// assume a hard cap of at most 4096 duplicated keys, as this is the largest number we can fit in
/// two bytes. (Our most duplicated forms appear around 100 times, so we need at least 2 bytes to
/// support them.)
fn create_extended_key(key: &str, tag: usize) -> Result<Vec<u8>> {
    // FIXME: make this an Error.
    if tag < MAX_DUPLICATES {
        let mut extended_key = key.as_bytes().to_vec();
        extended_key.push((tag / (DUPES_PER_BYTE as usize)) as u8);
        extended_key.push((tag % (DUPES_PER_BYTE as usize)) as u8);
        Ok(extended_key)
    } else {
        Err(Error::TooManyDuplicates(key.to_string()))
    }
}

impl Builder {
    /// Creates a new builder whose output will be written to `base_path`.
    ///
    /// If `base_path` does not exist, the builder will create it.
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let base_path = base_path.as_ref();
        std::fs::create_dir_all(base_path)?;
        let paths = Paths::new(base_path);

        let writer = io::BufWriter::new(File::create(paths.fst())?);
        Ok(Self {
            seen_keys: FxHashMap::default(),
            fst_builder: MapBuilder::new(writer)?,
            packer: Packer::new(),
            paths,
        })
    }

    /// Inserts the given `key` with the given semantics in `value`.
    ///
    /// Keys must be inserted in lexicographic order. If a key is received out of order,
    /// the build process will fail.
    pub fn insert(&mut self, key: &str, value: &PadaEntry) -> Result<()> {
        let value = self.pack(value)?;
        self.insert_packed(key, &value)
    }

    /// Inserts the given `key` with the packed semantics in `value`.
    ///
    /// Keys must be inserted in lexicographic order. If a key is received out of order,
    /// the build process will fail.
    pub fn insert_packed(&mut self, key: &str, value: &PackedEntry) -> Result<()> {
        let u64_payload = value.to_u32() as u64;

        let seen_keys = &mut self.seen_keys;
        let num_repeats = match seen_keys.get(key) {
            Some(c) => *c,
            None => 0,
        };
        seen_keys.insert(key.to_string(), num_repeats + 1);

        // For duplicates, add another u8 to make this key unique.
        if num_repeats > 0 {
            // Subtract 1 so that the duplicate tag always starts at 0.
            let final_key = create_extended_key(key, num_repeats - 1)?;
            self.fst_builder.insert(&final_key, u64_payload)?;
        } else {
            self.fst_builder.insert(key, u64_payload)?;
        };

        Ok(())
    }

    /// Registers the given dhatus on the internal packer. Duplicate dhatus are ignored.
    pub fn register_dhatus(&mut self, dhatus: &[DhatuEntry]) {
        self.packer.register_dhatus(dhatus);
    }

    /// Registers the given pratipadikas on the internal packer. Duplicate pratipadikas are ignored.
    pub fn register_pratipadikas(&mut self, pratipadikas: &[PratipadikaEntry]) {
        self.packer.register_pratipadikas(pratipadikas);
    }

    /// Packs the given `pada` into a more compact format.
    pub fn pack(&self, pada: &PadaEntry) -> Result<PackedEntry> {
        self.packer.pack(pada)
    }

    /// Writes all FST data to disk.
    pub fn finish(self) -> Result<()> {
        info!("Writing FST and packer data to {:?}.", self.paths.base);
        self.fst_builder.finish()?;

        self.packer
            .write(&self.paths.dhatus(), &self.paths.pratipadikas())?;

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use vidyut_prakriya::args as vp;
    use vidyut_prakriya::args::{Dhatu, Pratipadika};

    use crate::entries::*;
    use tempfile::tempdir;

    type TestResult = Result<()>;

    fn safe(s: &str) -> vp::Slp1String {
        vp::Slp1String::from(s).expect("ok")
    }

    fn d_entry(d: Dhatu) -> DhatuEntry {
        DhatuEntry::new(d, "text".to_string())
    }

    fn entry(p: Pratipadika) -> PratipadikaEntry {
        PratipadikaEntry::new(p, vec![])
    }

    #[test]
    fn test_paths() {
        let paths = Paths {
            base: Path::new("foo").to_path_buf(),
        };
        assert!(paths.fst().starts_with("foo/"));
        assert!(paths.dhatus().starts_with("foo/"));
        assert!(paths.pratipadikas().starts_with("foo/"));
    }

    #[test]
    fn write_and_load() -> TestResult {
        let gam = Dhatu::mula(safe("gam"), vp::Gana::Bhvadi);
        let tin = PadaEntry::Tinanta(vp::Tinanta::new(
            gam.clone(),
            vp::Prayoga::Kartari,
            vp::Lakara::Lat,
            vp::Purusha::Prathama,
            vp::Vacana::Eka,
        ));

        let gacchan = vp::Krdanta::new(
            Dhatu::mula(safe("gam"), vp::Gana::Bhvadi),
            vp::BaseKrt::Satf,
        );
        let gacchati = PadaEntry::Subanta(
            vp::Subanta::new(
                gacchan.clone(),
                vp::Linga::Pum,
                vp::Vibhakti::Saptami,
                vp::Vacana::Eka,
            )
            .into(),
        );

        let agni = Pratipadika::basic(safe("agni"));
        let sup = PadaEntry::Subanta(
            vp::Subanta::new(
                agni.clone(),
                vp::Linga::Pum,
                vp::Vibhakti::Dvitiya,
                vp::Vacana::Eka,
            )
            .into(),
        );

        // Builder
        let dir = tempdir()?;
        let mut builder = Builder::new(dir.path())?;
        builder.register_dhatus(&[d_entry(gam)]);
        builder.register_pratipadikas(&[entry(gacchan.into()), entry(agni)]);

        builder.insert("agnim", &sup)?;
        builder.insert("gacCati", &tin)?;
        builder.insert("gacCati", &gacchati)?;
        builder.finish()?;

        /*
        // Constructor
        let lex = Kosha::new(dir.path())?;

        // contains_key
        assert!(lex.contains_key("agnim"));
        assert!(lex.contains_key("gacCati"));
        assert!(!lex.contains_key("gacCat"));
        assert!(!lex.contains_key("gacCanti"));

        // contains_prefix
        assert!(lex.contains_prefix("a"));
        assert!(lex.contains_prefix("agn"));
        assert!(lex.contains_prefix("agnim"));
        assert!(lex.contains_prefix("g"));
        assert!(lex.contains_prefix("gacCat"));
        assert!(lex.contains_prefix("gacCati"));
        assert!(!lex.contains_prefix("gacCant"));

        // get_all
        fn get_all_padas(lex: &Kosha, key: &str) -> Result<Vec<Pada>> {
            lex.get_all(key).iter().map(|p| lex.unpack(p)).collect()
        }

        assert_eq!(get_all_padas(&lex, "agnim")?, vec![sup]);
        assert_eq!(get_all_padas(&lex, "gacCati")?, vec![tin, gacchati]);
        assert_eq!(get_all_padas(&lex, "gacCat")?, vec![]);
        assert_eq!(get_all_padas(&lex, "123")?, vec![]);

        // stream
        let mut stream = lex.stream();
        let mut kvs = vec![];
        while let Some((k, _v)) = stream.next() {
            kvs.push(k.to_vec());
        }
        // FIXME: yield duplicates identically?
        assert_eq!(
            kvs,
            vec![
                b"agnim".to_vec(),
                b"gacCati".to_vec(),
                b"gacCati\x00\x00".to_vec(),
            ]
        );

        */
        Ok(())
    }

    #[test]
    fn test_create_extended_key() -> TestResult {
        let cases = vec![
            (("a", 0), [97, 0, 0].to_vec()),
            (("a", 64), [97, 0, 64].to_vec()),
            (("a", 65), [97, 1, 0].to_vec()),
            (("a", 129), [97, 1, 64].to_vec()),
            (("a", 4224), [97, 64, 64].to_vec()),
        ];
        for ((base, num), result) in cases {
            assert_eq!(create_extended_key(base, num).unwrap(), result);
        }

        Ok(())
    }
}
