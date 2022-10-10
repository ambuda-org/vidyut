use log::info;
use std::process;

mod padas;
mod sandhi;

fn main() {
    env_logger::init();

    let text = std::env::args().nth(1).expect("No text provided.");

    let rules = match sandhi::read_rules("data/sandhi.tsv") {
        Ok(rules) => rules,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    let pada_paths = padas::DataConfig {
        shs_verbs: "data/sanskrit-heritage-site/roots.csv".to_string(),
        shs_adverbs: "data/sanskrit-heritage-site/adverbs.csv".to_string(),
        shs_final: "data/sanskrit-heritage-site/final.csv".to_string(),
    };
    let pada_map = match padas::read_pada_data(&pada_paths) {
        Ok(padas) => padas,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let mut state = text.clone();
    let mut padas = Vec::new();
    while !state.is_empty() {
        let mut changed_state = false;
        for (first, second) in sandhi::split(&state, &rules) {
            if !sandhi::is_good_split(&text, &first, &second) {
                continue;
            }
            if padas::is_pada(&first, &pada_map) {
                padas.push(first);
                state = second;
                info!("State is: \"{}\"", state);
                changed_state = true;
            }
        }
        if !changed_state {
            info!("No results for state \"{}\"", state);
            break;
        }
    }
    println!("{:?}", padas);
}
