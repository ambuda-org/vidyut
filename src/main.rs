use log::{debug, info};
use std::process;

mod io;
mod padas;
mod sandhi;

struct State {
    pub items: Vec<String>,
    pub remaining: String,
}

fn parse(text: &str, ctx: io::Context) -> Option<Vec<String>> {
    info!("Beginning parse: \"{}\"", text);

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
        sandhi_rules: "data/sandhi.tsv".to_string(),
        shs_verbs: "data/sanskrit-heritage-site/roots.csv".to_string(),
        shs_adverbs: "data/sanskrit-heritage-site/adverbs.csv".to_string(),
        shs_final: "data/sanskrit-heritage-site/final.csv".to_string(),
    };
    let ctx = match io::load_data(&data_paths) {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let padas = parse(&text, ctx);
    println!("{:?}", padas);
}
