use log::{debug, info};
use std::path::Path;
use std::process;

mod io;
mod padas;
mod sandhi;

struct State {
    pub items: Vec<String>,
    pub remaining: String,
}

fn parse(text: &str, ctx: io::Context) -> Option<Vec<String>> {
    let mut stack = Vec::new();
    stack.push(State {
        items: Vec::new(),
        remaining: text.to_string(),
    });
    while !stack.is_empty() {
        let cur_state = stack.pop().unwrap();
        debug!("Pop state: {:?} {}", cur_state.items, cur_state.remaining);

        if cur_state.remaining.is_empty() {
            return Some(cur_state.items);
        }
        for (first, second) in sandhi::split(&cur_state.remaining, &ctx.sandhi_rules) {
            if !sandhi::is_good_split(&cur_state.remaining, &first, &second) {
                continue;
            }
            if padas::is_pada(&first, &ctx.pada_map) {
                let mut new_state = State {
                    items: cur_state.items.clone(),
                    remaining: second,
                };
                new_state.items.push(first);
                stack.push(new_state);
            }
        }
    }
    return None;
}

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
    let padas = parse(&text, ctx);
    println!("{:?}", padas);
}
