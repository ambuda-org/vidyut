use log::info;
use std::path::Path;
use std::process;

mod io;
mod padas;
mod parsing;
mod sandhi;

fn main() {
    env_logger::init();

    let text = std::env::args().nth(1).expect("No text provided.");

    let data_paths = io::DataPaths {
        dump: "data/snapshot.bin".to_string(),
        indeclinables: "data/indeclinables.csv".to_string(),
        nominal_endings_compounded: "data/nominal-endings-compounded.csv".to_string(),
        nominal_endings_inflected: "data/nominal-endings-inflected.csv".to_string(),
        nominal_stems: "data/nominal-stems.csv".to_string(),
        participle_stems: "data/participle-stems.csv".to_string(),
        prefix_groups: "data/prefix-groups.csv".to_string(),
        prefixed_roots: "data/prefixed-roots.csv".to_string(),
        pronouns: "data/pronouns.csv".to_string(),
        sandhi_rules: "data/sandhi-rules.csv".to_string(),
        unprefixed_roots: "data/unprefixed-roots.csv".to_string(),
        verb_endings: "data/verb-endings.csv".to_string(),
        verb_prefixes: "data/verb-prefixes.csv".to_string(),
        verbal_indeclinables: "data/verbal-indeclinables.csv".to_string(),
        verbs: "data/verbs.csv".to_string(),
    };

    let ctx = if Path::new(&data_paths.dump).exists() {
        info!("Loading previous snapshot from \"{}\"", &data_paths.dump);
        match io::read_snapshot(&data_paths.dump) {
            Ok(data) => data,
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        }
    } else {
        info!("No previous snapshot found. Loading raw data.");
        let ctx = match io::read_all_data(&data_paths) {
            Ok(data) => data,
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        };

        info!("Creating snapshot for faster loading next time.");
        match io::write_snapshot(&ctx, &data_paths.dump) {
            Ok(data) => data,
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        };

        info!("Wrote snapshot data to {}", data_paths.dump);
        ctx
    };

    info!("Beginning parse: \"{}\"", text);
    let padas = parsing::parse(&text, ctx);
    println!("{:?}", padas);
}
