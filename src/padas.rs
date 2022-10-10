use crate::io;
use multimap::MultiMap;
use std::error::Error;

pub type PadaMap = MultiMap<String, String>;

/// Read verb data from the Sanskrit Heritage Site.
fn add_shs_verbs(csv_path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let pada = row[0].to_string();
        let lex = format!("v-{}-{}-{}-{}", &row[3], &row[4], &row[5], &row[6]);
        padas.insert(pada, lex);
    }
    Ok(())
}

fn add_shs_adverbs(csv_path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let pada = row[0].to_string();
        padas.insert(pada, String::from("tvA"));
    }
    Ok(())
}

fn add_shs_final(csv_path: &str, padas: &mut PadaMap) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    for maybe_row in rdr.records() {
        let row = maybe_row?;
        let pada = row[0].to_string();
        let lex = format!("v-{}-{}", &row[2], &row[3]);
        padas.insert(pada, lex);
    }
    Ok(())
}

pub fn read_pada_data(paths: &io::DataPaths) -> Result<PadaMap, Box<dyn Error>> {
    let mut padas = PadaMap::new();
    add_shs_verbs(&paths.shs_verbs, &mut padas)?;
    add_shs_adverbs(&paths.shs_adverbs, &mut padas)?;
    add_shs_final(&paths.shs_final, &mut padas)?;
    Ok(padas)
}

pub fn is_pada(text: &str, data: &PadaMap) -> bool {
    data.contains_key(text)
}
