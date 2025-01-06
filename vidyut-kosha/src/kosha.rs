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
use crate::errors::{Error, Result};
use crate::packing::{Id, PackedEntry, Packer, PartOfSpeech};
use fst::map::Stream;
use fst::raw::{Fst, Node, Output};
use fst::{Map, MapBuilder};
use log::info;
use rustc_hash::FxHashMap;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use vidyut_prakriya::args::{Linga, Vacana, Vibhakti};

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

    /// Path to our registry of interned data.
    fn registry(&self) -> PathBuf {
        self.base.join("registry.msgpack")
    }
}

fn to_packed_entry(output: Output) -> PackedEntry {
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

        info!("Loading registry from `{:?}`", paths.registry());
        let packer = Packer::read(&paths.registry())?;

        Ok(Self { fst, packer })
    }

    /// Returns an iterator over all dhatus contained in the kosha.
    ///
    /// # Usage
    ///
    /// ```rust,no_run
    /// # use vidyut_kosha::*;
    /// use vidyut_kosha::Kosha;
    ///
    /// let kosha = Kosha::new("/path/to/kosha/data")?;
    ///
    /// for dhatu in kosha.dhatus() {
    ///   println!("{} --> {:?}", dhatu.clean_text(), dhatu);
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn dhatus(&self) -> impl Iterator<Item = DhatuEntry> {
        let n = self.packer.dhatus.len();
        (0..n).filter_map(|i| self.packer.unpack_dhatu(Id(i)).ok())
    }

    /// Returns an iterator over all pratipadikas contained in the kosha.
    ///
    /// # Usage
    ///
    /// ```rust,no_run
    /// # use vidyut_kosha::*;
    /// use vidyut_kosha::Kosha;
    ///
    /// let kosha = Kosha::new("/path/to/kosha/data")?;
    ///
    /// for pratipadika in kosha.pratipadikas() {
    ///   println!("{:?}", pratipadika);
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn pratipadikas(&self) -> impl Iterator<Item = PratipadikaEntry> {
        let n = self.packer.pratipadikas.len();
        (0..n).filter_map(|i| self.packer.unpack_pratipadika(Id(i)).ok())
    }

    /// Returns a reference to this kosha's underlying FST.
    pub fn fst(&self) -> &Map<Vec<u8>> {
        &self.fst
    }

    /// Returns whether the kosha contains at least one entry with the exact value `key`.
    ///
    /// In our provided kosha data, all keys are SLP1 strings, and final *visarga* is replaced
    /// with `s` and `r` as appropriate.
    ///
    /// # Usage
    ///
    /// ```rust,no_run
    /// # use vidyut_kosha::*;
    /// use vidyut_kosha::Kosha;
    ///
    /// let kosha = Kosha::new("/path/to/kosha/data")?;
    ///
    /// assert!(kosha.contains_key("naras"));
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        let fst = self.fst.as_fst();
        let mut node = fst.root();
        let mut out = Output::zero();
        for (i_b, b) in key.bytes().enumerate() {
            node = match node.find_input(b) {
                None => return false,
                Some(i_n) => {
                    let t = node.transition(i_n);
                    out = out.cat(t.out);
                    fst.node(t.addr)
                }
            };
            if node.is_final() {
                let suffix = &key[i_b + 1..];
                if self.contains_subanta_suffix(suffix, node, out) {
                    return true;
                }
            }
        }
        node.is_final()
    }

    fn contains_subanta_suffix(&self, suffix: &str, node: Node, out_base: Output) -> bool {
        let entry_base = to_packed_entry(out_base);

        if entry_base.pos() == PartOfSpeech::SubantaPrefix {
            let entry_base = entry_base.as_packed_subanta_prefix();
            if self.packer.contains_subanta_suffix(&entry_base, suffix) {
                return true;
            }
        }

        let fst = self.fst.as_fst();
        for c1 in 0..=DUPES_PER_BYTE {
            if let Some(i1) = node.find_input(c1) {
                let t1 = node.transition(i1);
                let o1 = out_base.cat(t1.out);
                let n1 = fst.node(t1.addr);

                for c2 in 0..=DUPES_PER_BYTE {
                    if let Some(i2) = n1.find_input(c2) {
                        let t2 = n1.transition(i2);
                        let o2 = o1.cat(t2.out);
                        let n2 = fst.node(t2.addr);

                        // In our current scheme, this node is always final.
                        assert!(n2.is_final());

                        let output = o2.cat(n2.final_output());
                        let entry = to_packed_entry(output);
                        if entry.pos() == PartOfSpeech::SubantaPrefix {
                            let entry = entry.as_packed_subanta_prefix();
                            if self.packer.contains_subanta_suffix(&entry, suffix) {
                                return true;
                            }
                        }
                    } else {
                        return false;
                    }
                }
            } else {
                break;
            }
        }

        false
    }

    /// TODO: make public again
    ///
    /// Returns whether the kosha contains at least one word that starts with `key`.
    ///
    /// Prefix checks are slightly faster than ordinary key lookups. I also tried implementing this
    /// with `fst::Stream`, but that approach was much slower than just accessing the FST directly,
    /// which is what we do here.
    #[inline]
    #[allow(unused)]
    fn contains_prefix(&self, key: &str) -> bool {
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

    /// Returns all results for the given `key`, including duplicates.
    ///
    /// In our provided kosha data, all keys are SLP1 strings, and final *visarga* is replaced
    /// with `s` and `r` as appropriate.
    ///
    /// # Usage
    ///
    /// ```rust,no_run
    /// # use vidyut_kosha::*;
    /// use vidyut_kosha::Kosha;
    ///
    /// let kosha = Kosha::new("/path/to/kosha/data")?;
    ///
    /// for entry in kosha.get_all("Bavati") {
    ///     println!("{:#?}", entry);
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn get_all(&self, key: &str) -> Vec<PadaEntry> {
        let mut ret = Vec::new();

        let fst = self.fst.as_fst();
        let mut node = fst.root();
        let mut out = Output::zero();
        for (i_b, b) in key.bytes().enumerate() {
            node = match node.find_input(b) {
                None => return ret,
                Some(i) => {
                    let t = node.transition(i);
                    out = out.cat(t.out);
                    fst.node(t.addr)
                }
            };

            // Possible subanta prefix -- check for all suffix matches.
            if node.is_final() {
                let out_final = out.cat(node.final_output());
                let suffix = &key[i_b + 1..];
                self.get_all_from_subanta_suffixes(&mut ret, suffix, node, out_final)
                    .ok();
            }
        }

        if node.is_final() {
            let packed = to_packed_entry(out.cat(node.final_output()));
            let entry = match self.packer.unpack(&packed) {
                Ok(e) => e,
                _ => panic!("aoeu"),
            };

            ret.push(entry);
            self.add_duplicates(node, out, fst, &mut ret);
        }

        ret
    }

    fn get_all_from_subanta_suffixes<'a>(
        &'a self,
        ret: &mut Vec<PadaEntry<'a>>,
        suffix: &str,
        node: Node,
        out_base: Output,
    ) -> Result<()> {
        let entry_base = to_packed_entry(out_base);

        if entry_base.pos() == PartOfSpeech::SubantaPrefix {
            let entry_base = entry_base.as_packed_subanta_prefix();
            self.packer
                .get_all_from_subanta_paradigm(ret, &entry_base, suffix)?;
        }

        let fst = self.fst.as_fst();
        for c1 in 0..=DUPES_PER_BYTE {
            if let Some(i1) = node.find_input(c1) {
                let t1 = node.transition(i1);
                let o1 = out_base.cat(t1.out);
                let n1 = fst.node(t1.addr);

                for c2 in 0..=DUPES_PER_BYTE {
                    if let Some(i2) = n1.find_input(c2) {
                        let t2 = n1.transition(i2);
                        let o2 = o1.cat(t2.out);
                        let n2 = fst.node(t2.addr);

                        // In our current scheme, this node is always final.
                        assert!(n2.is_final());

                        let output = o2.cat(n2.final_output());
                        let entry = to_packed_entry(output);
                        if entry.pos() == PartOfSpeech::SubantaPrefix {
                            let entry = entry.as_packed_subanta_prefix();
                            self.packer
                                .get_all_from_subanta_paradigm(ret, &entry, suffix)?;
                        }
                    } else {
                        return Ok(());
                    }
                }
            } else {
                return Ok(());
            }
        }

        Ok(())
    }

    /// Iterates over all keys in the FST.
    ///
    /// NOTE: this method currently has limited functionality for krdantas.
    pub fn stream(&self) -> Stream<'_> {
        self.fst.stream()
    }

    /// Appends all available duplicates to our list of results.
    ///
    /// Args:
    /// - `node`: the node corresponding to the last ASCII character of the input string.
    /// - `out`: the output corresponding to this state.
    /// - `fst`: the underlying FST.
    /// - `results`: the results list.
    fn add_duplicates<'a>(
        &'a self,
        node: Node,
        out: Output,
        fst: &Fst<Vec<u8>>,
        results: &mut Vec<PadaEntry<'a>>,
    ) {
        for c1 in 0..DUPES_PER_BYTE {
            if let Some(i1) = node.find_input(c1) {
                let t1 = node.transition(i1);
                let o1 = out.cat(t1.out);
                let n1 = fst.node(t1.addr);

                for c2 in 0..DUPES_PER_BYTE {
                    if let Some(i2) = n1.find_input(c2) {
                        let t2 = n1.transition(i2);
                        let o2 = o1.cat(t2.out);
                        let n2 = fst.node(t2.addr);

                        // In our current scheme, this node is always final.
                        if n2.is_final() {
                            let packed = to_packed_entry(o2.cat(n2.final_output()));
                            let entry = match self.packer.unpack(&packed) {
                                Ok(e) => e,
                                _ => panic!("aoeu"),
                            };
                            results.push(entry);
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

    /// Packs the given `PadaEntry` into a simple integer code.
    pub fn pack(&self, value: &PadaEntry) -> Result<PackedEntry> {
        self.packer.pack(value)
    }

    /// Unpacks the given `PackedEntry` into a full `PadaEntry`.
    pub fn unpack(&self, value: PackedEntry) -> Result<PadaEntry> {
        self.packer.unpack(&value)
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
    /// Notes:
    /// - Keys must be inserted in lexicographic order. If a key is received out of order,
    ///   the build process will fail.
    /// - All linguistic data on `value` must be registered beforehand. You can do this by calling
    ///   `register_pada_entry`.
    pub fn insert(&mut self, key: &str, value: &PadaEntry) -> Result<()> {
        let value = self.pack(value)?;
        self.insert_packed(key, &value)
    }

    /// Inserts the given `key` with the packed semantics in `value`.
    ///
    /// Notes:
    /// - Keys must be inserted in lexicographic order. If a key is received out of order,
    ///   the build process will fail.
    /// - All linguistic data on `value` must be registered beforehand. You can do this by calling
    ///   `register_pada_entry`.
    pub fn insert_packed(&mut self, key: &str, value: &PackedEntry) -> Result<()> {
        let u64_payload = u64::from(value.to_u32());

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

    /// Registers the given dhatus on the internal packer. Duplicates are ignored.
    pub fn register_dhatu_entry(&mut self, dhatu: &DhatuEntry) {
        self.packer.register_dhatu_entry(dhatu);
    }

    /// Registers the given pratipadikas on the internal packer. Duplicates are ignored.
    pub fn register_pratipadika_entry(&mut self, pratipadika: &PratipadikaEntry) {
        self.packer.register_pratipadika_entry(pratipadika);
    }

    /// Registers all linguistic data defined on `entry` on this kosha's internal registry.
    pub fn register_pada_entry(&mut self, entry: &PadaEntry) {
        match entry {
            PadaEntry::Subanta(s) => self.register_pratipadika_entry(&s.pratipadika_entry()),
            PadaEntry::Avyaya(a) => self.register_pratipadika_entry(&a.pratipadika_entry()),
            PadaEntry::Tinanta(t) => self.register_dhatu_entry(&t.dhatu_entry()),
            PadaEntry::Unknown => (),
        }
    }

    /// Registers the given paradigm of subantas and returns the prefix they all share.
    pub fn register_subanta_paradigm(
        &mut self,
        pratipadika: &PratipadikaEntry,
        padas: &[(String, Linga, Vibhakti, Vacana)],
    ) -> Result<(String, PackedEntry)> {
        self.packer.register_subanta_paradigm(pratipadika, padas)
    }

    /// Packs the given *pada* into a more compact format.
    pub fn pack(&self, pada: &PadaEntry) -> Result<PackedEntry> {
        self.packer.pack(pada)
    }

    /// Writes all kosha data to disk.
    pub fn finish(self) -> Result<()> {
        info!("Writing FST and packer data to {:?}.", self.paths.base);
        self.fst_builder.finish()?;
        self.packer.write(&self.paths.registry())?;

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::entries::*;
    use vidyut_prakriya::args as vp;
    use vidyut_prakriya::args::{Dhatu, Pratipadika, Slp1String};

    use tempfile::tempdir;

    type TestResult = Result<()>;

    #[test]
    fn test_paths() {
        let paths = Paths {
            base: Path::new("foo").to_path_buf(),
        };
        assert!(paths.fst().starts_with("foo/"));
        assert!(paths.registry().starts_with("foo/"));
    }

    fn safe(s: &str) -> Slp1String {
        Slp1String::from(s).expect("ok")
    }

    #[test]
    fn write_and_load() -> TestResult {
        let gam = Dhatu::mula(safe("gam"), vp::Gana::Bhvadi);
        let gacchati = vp::Tinanta::new(
            gam.clone(),
            vp::Prayoga::Kartari,
            vp::Lakara::Lat,
            vp::Purusha::Prathama,
            vp::Vacana::Eka,
        );
        let gacchati_e: TinantaEntry = (&gacchati).into();

        let gacchan: Pratipadika = vp::Krdanta::builder()
            .dhatu(gam.clone())
            .krt(vp::BaseKrt::Satf)
            .lakara(vp::Lakara::Lat)
            .prayoga(vp::Prayoga::Kartari)
            .build()
            .unwrap()
            .into();
        let gacchan_7s = vp::Subanta::new(
            gacchan.clone(),
            vp::Linga::Pum,
            vp::Vibhakti::Saptami,
            vp::Vacana::Eka,
        );
        let gacchan_7s_e: SubantaEntry = (&gacchan_7s).try_into().expect("ok");

        let agni = Pratipadika::basic(safe("agni"));
        let agni_2s = vp::Subanta::new(
            agni.clone(),
            vp::Linga::Pum,
            vp::Vibhakti::Dvitiya,
            vp::Vacana::Eka,
        );
        let agni_2s_e: SubantaEntry = (&agni_2s).try_into().expect("ok");

        // Builder
        let dir = tempdir()?;
        let mut builder = Builder::new(dir.path())?;

        let gam_entry = DhatuEntry::new(&gam, "gam");
        builder.register_dhatu_entry(&gam_entry);
        builder.register_pratipadika_entry(&(&gacchan).try_into().expect("ok"));
        builder.register_pratipadika_entry(&(&agni).try_into().expect("ok"));

        builder.insert("agnim", &agni_2s_e.into())?;
        builder.insert("gacCati", &gacchati_e.into())?;
        builder.insert("gacCati", &gacchan_7s_e.into())?;
        builder.finish()?;

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

        /*
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
