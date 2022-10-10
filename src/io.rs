use crate::padas::{EndingMap, PadaMap, StemMap};
use crate::sandhi::SandhiMap;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{BufReader, BufWriter};

/// Data paths from https://github.com/sanskrit/data
pub struct DataPaths {
    pub dump: String,
    pub indeclinables: String,
    pub nominal_endings_compounded: String,
    pub nominal_endings_inflected: String,
    pub nominal_stems: String,
    pub participle_stems: String,
    pub prefix_groups: String,
    pub prefixed_roots: String,
    pub pronouns: String,
    pub sandhi_rules: String,
    pub unprefixed_roots: String,
    pub verb_endings: String,
    pub verb_prefixes: String,
    pub verbal_indeclinables: String,
    pub verbs: String,
}

#[derive(Serialize, Deserialize)]
pub struct Context {
    pub sandhi_rules: SandhiMap,
    pub pada_map: PadaMap,
    pub stem_map: StemMap,
    pub ending_map: EndingMap,
}

fn add_indeclinables(path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let pada = r[0].to_string();
        padas.insert(pada, String::from("ind"));
    }
    Ok(())
}

fn add_nominal_endings_compounded(
    path: &str,
    endings: &mut EndingMap,
) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let ending = r[2].to_string();
        endings.insert(ending, (stem, String::from("comp")));
    }
    Ok(())
}

fn add_nominal_endings_inflected(
    path: &str,
    endings: &mut EndingMap,
) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;

        let stem = r[0].to_string();
        let ending = r[2].to_string();
        let lex = format!("{}-{}-{}", &r[3], &r[4], &r[5]);
        endings.insert(ending, (stem, lex));
    }
    Ok(())
}

fn add_nominal_stems(path: &str, padas: &mut StemMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let lex = r[1].to_string();
        padas.insert(stem, String::from(lex));
    }
    Ok(())
}

fn add_participle_stems(path: &str, padas: &mut StemMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let stem = r[0].to_string();
        let lex = format!("{}-{}-{}", &r[3], &r[4], &r[5]);
        padas.insert(stem, String::from(lex));
    }
    Ok(())
}

fn add_pronouns(path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let pada = row[0].to_string();
        let lex = format!("n-{}-{}-{}", &row[3], &row[4], &row[5]);
        padas.insert(pada, lex);
    }
    Ok(())
}

fn add_verbal_indeclinables(path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let pada = row[0].to_string();
        let lex = format!("vi-{}-{}", &row[2], &row[3]);
        padas.insert(pada, lex);
    }
    Ok(())
}

fn add_verbs(path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let pada = row[0].to_string();
        let lex = format!("v-{}-{}-{}-{}", &row[3], &row[4], &row[5], &row[6]);
        padas.insert(pada, lex);
    }
    Ok(())
}

// ==> prefix-groups.csv <==
// ==> prefixed-roots.csv <==
// ==> verb-endings.csv <==
// ==> verb-prefixes.csv <==

/// Creates a map from sandhi combinations to the sounds that created them.
///
/// # Arguments
///
/// - `tsv_path` - a TSV with columns `first`, `second`, and `result`.
pub fn read_sandhi_rules(tsv_path: &str) -> Result<SandhiMap, Box<dyn Error>> {
    let mut rules = SandhiMap::new();

    let mut rdr = csv::Reader::from_path(tsv_path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let first = String::from(&row[0]);
        let second = String::from(&row[1]);
        let result = String::from(&row[2]);
        let type_ = &row[3];
        if type_ == "internal" {
            continue;
        }

        rules.insert(result.clone(), (first.clone(), second.clone()));

        let result_no_spaces = String::from(&row[2]).replace(' ', "");
        if result_no_spaces != result {
            rules.insert(result_no_spaces, (first.clone(), second.clone()));
        }
    }
    Ok(rules)
}

pub fn read_nominal_endings(paths: &DataPaths) -> Result<EndingMap, Box<dyn Error>> {
    let mut endings = EndingMap::new();
    add_nominal_endings_compounded(&paths.nominal_endings_compounded, &mut endings)?;
    add_nominal_endings_inflected(&paths.nominal_endings_inflected, &mut endings)?;
    Ok(endings)
}

pub fn read_stems(paths: &DataPaths) -> Result<PadaMap, Box<dyn Error>> {
    let mut stems = StemMap::new();
    add_nominal_stems(&paths.nominal_stems, &mut stems)?;
    add_participle_stems(&paths.participle_stems, &mut stems)?;
    Ok(stems)
}

pub fn read_padas(paths: &DataPaths) -> Result<PadaMap, Box<dyn Error>> {
    let mut padas = PadaMap::new();
    add_indeclinables(&paths.indeclinables, &mut padas)?;
    add_pronouns(&paths.pronouns, &mut padas)?;
    add_verbal_indeclinables(&paths.verbal_indeclinables, &mut padas)?;
    add_verbs(&paths.verbs, &mut padas)?;
    Ok(padas)
}

pub fn read_all_data(paths: &DataPaths) -> Result<Context, Box<dyn Error>> {
    Ok(Context {
        sandhi_rules: read_sandhi_rules(&paths.sandhi_rules)?,
        pada_map: read_padas(&paths)?,
        stem_map: read_stems(&paths)?,
        ending_map: read_nominal_endings(&paths)?,
    })
}

/// Read a previous data context to disk.
///
/// Reading this snapshot is about twice as fast as building from scratch.
pub fn read_snapshot(binary_path: &str) -> Result<Context, Box<bincode::ErrorKind>> {
    // Use BufWriter for better performance.
    // https://stackoverflow.com/questions/43028653
    let mut f = BufReader::new(fs::File::open(binary_path)?);
    bincode::deserialize_from(&mut f)
}

/// Dump the current data context to disk.
pub fn write_snapshot(ctx: &Context, binary_path: &str) -> Result<(), Box<bincode::ErrorKind>> {
    // Use BufWriter for better performance.
    // https://stackoverflow.com/questions/43028653
    let mut f = BufWriter::new(fs::File::create(binary_path)?);
    bincode::serialize_into(&mut f, &ctx)
}
