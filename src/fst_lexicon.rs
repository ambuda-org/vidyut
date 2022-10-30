//! A highly memory-efficient Sanskrit dictionary.
//!
//! As of this comment, the FST lexicon stores more than 11 million words in about 20 megabytes.
use crate::packing::*;
use crate::semantics::*;
use fst::map::Stream;
use fst::raw::Output;
use fst::{Map, MapBuilder};
use log::{debug, info};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

pub struct FstLexicon {
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

impl FstLexicon {
    pub fn load_from(base_path: &Path) -> Result<FstLexicon, Box<dyn Error>> {
        let paths = Paths {
            base: base_path.to_path_buf(),
        };

        let fst = Map::new(std::fs::read(paths.fst()).unwrap())?;
        let unpacker = Unpacker::from_data(
            PratipadikaTable::read(&paths.pratipadikas())?,
            DhatuTable::read(&paths.dhatus())?,
        );
        Ok(FstLexicon { fst, unpacker })
    }

    #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.fst.contains_key(key)
    }

    /// Checks whether the lexicon contains *any* words that start with this prefix.
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

    #[inline]
    pub fn get_first(&self, key: &str) -> Option<PackedPada> {
        self.fst
            .get(key)
            .map(|val| PackedPada::from_u32(val as u32))
    }

    pub fn unpack(&self, p: &PackedPada) -> Result<Pada, Box<dyn Error>> {
        self.unpacker.unpack(p)
    }

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
        if !node.is_final() {
            Vec::new()
        } else {
            let mut ret = vec![out.cat(node.final_output())];

            // first ASCII letter is 65 (01000001)
            for counter in 1..65 {
                let b = counter as u8;
                match node.find_input(b) {
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
        }
    }

    /// Iterate over all keys in the FST.
    pub fn stream(&self) -> Stream<'_> {
        self.fst.stream()
    }
}

pub struct FstLexiconBuilder {
    seen_keys: HashMap<String, usize>,
    fst_builder: MapBuilder<io::BufWriter<File>>,
    packer: Packer,
    paths: Paths,
}

impl FstLexiconBuilder {
    pub fn new(base_path: &Path) -> Result<FstLexiconBuilder, Box<dyn Error>> {
        std::fs::create_dir_all(base_path)?;
        let paths = Paths {
            base: base_path.to_path_buf(),
        };

        let writer = io::BufWriter::new(File::create(paths.fst()).unwrap());
        Ok(FstLexiconBuilder {
            seen_keys: HashMap::new(),
            fst_builder: MapBuilder::new(writer).unwrap(),
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

        let value = self.packer.pack(value).to_u32() as u64;

        self.fst_builder.insert(&final_key, value)?;
        Ok(())
    }

    /// Writes all FST data to disk and returns a complete `FSTLexicon`.
    pub fn into_lexicon(self) -> Result<FstLexicon, Box<dyn Error>> {
        info!("Writing FST and packer data to `{:?}`.", self.paths.base);
        self.fst_builder.finish()?;
        let unpacker = Unpacker::from_packer(&self.packer);
        unpacker.write(&self.paths.dhatus(), &self.paths.pratipadikas())?;

        info!("Reading new FST from `{:?}`.", self.paths.base);
        let fst_data = std::fs::read(self.paths.fst())?;
        let fst = Map::new(fst_data)?;
        Ok(FstLexicon { fst, unpacker })
    }
}
