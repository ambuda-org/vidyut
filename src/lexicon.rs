//! A memory-efficient Sanskrit lexicon with basic support for searching prefixes.
//!
//!
//! Implementation
//! --------------
//! We implement our lexicon as a finite state transducer using the `fst` crate. Finite state
//! transducers are a generalization of tries in that they support both shared prefixes and shared
//! suffixes.
//!
//! The `fst` crate that we use internally has two important restrictions that affect the design of
//! our lexicon:
//!
//! 1. Each key can be stored at most once.
//! 2. Each value must be an unsigned 64 bit integer.
//!
//! To work around (1), we mark synonyms by appending an extra `u8` tag to each duplicated key. For
//! example, we would encode two instances of `gacCati` with the keys `gacCati` and `gacCati\u{1}`.
//! To avoid confusion with ASCII strings, we limit the value of this tag to at most 64. Thus we
//! can store at most 64 duplicates of a given string.
//!
//! To work around (2), we pack the semantics of Sanskrit words into integers with our
//! `vidyut::packing` crate. Since strings are difficult to pack, we instead store them in a lookup
//! table and pack their integer ID instead. For details, see `packing::Unpacker`.
//!
//!
//! Efficiency
//! ----------
//!
//! According to our benchmarks, an average Sanskrit word can be retrieved in around 500ns and is
//! roughly 1.5x slower than a default `HashMap`. Our production lexicon stores more than 20
//! million words in around 31MB of data with an average storage cost of 1.5 bytes per word. Of
//! course, the specific storage cost will vary depending on the words in the input list.
use crate::packing::*;
use crate::semantics::Pada;
use fst::map::Stream;
use fst::raw::Output;
use fst::{Map, MapBuilder};
use log::{debug, info};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

/// A highly memory-efficient Sanskrit lexicon.
pub struct Lexicon {
    /// The underlying FST object.
    fst: Map<Vec<u8>>,
    /// Maps indices to semantics objects.
    unpacker: Unpacker,
}

struct Paths {
    /// Path to the data dir for this lexicon. If writing, the writer will create the directory if
    /// it does not exist already.
    base: PathBuf,
}
impl Paths {
    /// Path to the underlying FST.
    fn fst(&self) -> PathBuf {
        self.base.join("padas.fst")
    }
    /// Path to the dhatus table, which maps indices to `Dhatu`s.
    fn dhatus(&self) -> PathBuf {
        self.base.join("dhatus.csv")
    }
    /// Path to the pratipadikas table, which maps indices to `Pratipadika`s.
    fn pratipadikas(&self) -> PathBuf {
        self.base.join("pratipadikas.csv")
    }
}

impl Lexicon {
    /// Reads the lexicon from the given `base_path`.
    pub fn load_from(base_path: &Path) -> Result<Self, Box<dyn Error>> {
        let paths = Paths {
            base: base_path.to_path_buf(),
        };

        let fst = Map::new(std::fs::read(paths.fst())?)?;
        let unpacker = Unpacker::from_data(
            PratipadikaTable::read(&paths.pratipadikas())?,
            DhatuTable::read(&paths.dhatus())?,
        );
        Ok(Self { fst, unpacker })
    }

    /// Returns whether this lexicon contains at least one word with exact value `key`.
    #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.fst.contains_key(key)
    }

    /// Returns whether the lexicon contains at least one word that starts with `key`.
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

    pub fn unpack(&self, p: &PackedPada) -> Result<Pada, Box<dyn Error>> {
        self.unpacker.unpack(p)
    }

    /// Get all resuts for the given `key`.
    #[inline]
    pub fn get_all(&self, key: &str) -> Vec<PackedPada> {
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
            let mut ret = vec![out.cat(node.final_output())];

            // first ASCII letter is 65 (01000001)
            for counter in 1_u8..65 {
                match node.find_input(counter) {
                    None => break,
                    Some(i) => {
                        let t = node.transition(i);
                        let new_out = out.cat(t.out);
                        let new_node = fst.node(t.addr);

                        // In our current scheme, this node is always final.
                        if node.is_final() {
                            ret.push(new_out.cat(new_node.final_output()));
                        }
                    }
                }
            }

            // FIXME: don't allocate a new vec here.
            ret.iter()
                .map(|val| PackedPada::from_u32(val.value() as u32))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Iterate over all keys in the FST.
    pub fn stream(&self) -> Stream<'_> {
        self.fst.stream()
    }
}

/// Builder for a `Lexicon`.
///
/// Memory usage is linear in the number of unique lemmas (`Dhatu`s or `Pratipadika`s).
pub struct Builder {
    seen_keys: HashMap<String, usize>,
    fst_builder: MapBuilder<io::BufWriter<File>>,
    packer: Packer,
    paths: Paths,
}

impl Builder {
    /// Creates a new builder whose output will be written to `base_path`.
    ///
    /// If `base_path` does not exist, the builder will create it.
    pub fn new(base_path: &Path) -> Result<Self, Box<dyn Error>> {
        std::fs::create_dir_all(base_path)?;
        let paths = Paths {
            base: base_path.to_path_buf(),
        };

        let writer = io::BufWriter::new(File::create(paths.fst())?);
        Ok(Self {
            seen_keys: HashMap::new(),
            fst_builder: MapBuilder::new(writer)?,
            packer: Packer::new(),
            paths,
        })
    }

    /// Inserts the given `key` with the given semantics in `value`.
    ///
    /// Keys must be inserted in lexicographic order. If a key is received out of order,
    /// the build process will fail.
    pub fn insert(&mut self, key: &str, value: &Pada) -> Result<(), Box<dyn Error>> {
        let seen_keys = &mut self.seen_keys;

        let num_repeats = match seen_keys.get(key) {
            Some(c) => *c,
            None => 0,
        };
        seen_keys.insert(key.to_string(), num_repeats + 1);

        // For duplicates, add another u8 to make this key unique.
        let final_key = if num_repeats > 0 {
            let tag = num_repeats as u8;

            // FIXME: support more than 64 duplicates.
            if tag > 64 {
                info!("Skipping {key} (appears more than 64 times)");
                return Ok(());
            }

            let mut extended_key = key.as_bytes().to_vec();
            extended_key.push(tag);
            debug!("Inserting duplicate key {key}");
            std::str::from_utf8(&extended_key)?.to_owned()
        } else {
            key.to_string()
        };

        let value = u64::from(self.packer.pack(value).to_u32());

        self.fst_builder.insert(&final_key, value)?;
        Ok(())
    }

    /// Writes all FST data to disk and returns a complete `FSTLexicon`.
    pub fn into_lexicon(self) -> Result<Lexicon, Box<dyn Error>> {
        info!("Writing FST and packer data to `{:?}`.", self.paths.base);
        self.fst_builder.finish()?;
        let unpacker = Unpacker::from_packer(&self.packer);
        unpacker.write(&self.paths.dhatus(), &self.paths.pratipadikas())?;

        info!("Reading new FST from `{:?}`.", self.paths.base);
        let fst_data = std::fs::read(self.paths.fst())?;
        let fst = Map::new(fst_data)?;
        Ok(Lexicon { fst, unpacker })
    }
}
