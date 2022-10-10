use log::{debug, info};
use std::process;

mod padas;
mod sandhi;

struct State {
    pub items: Vec<String>,
    pub remaining: String,
}

struct Context {
    pub sandhi_rules: sandhi::SandhiMap,
    pub pada_map: padas::PadaMap,
}

fn parse(text: &str, ctx: Context) -> Option<Vec<String>> {
    info!("Beginning parse: \"{}\"", text);

    let mut stack = Vec::new();
    stack.push(State {
        items: Vec::new(),
        remaining: text.to_string(),
    });
    while !stack.is_empty() {
        let cur_state = stack.pop().unwrap();
        debug!(
            "Pop state: {:?} {}",
            cur_state.items, cur_state.remaining
        );

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

    let sandhi_rules = match sandhi::read_rules("data/sandhi.tsv") {
        Ok(sandhi_rules) => sandhi_rules,
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
    let ctx = Context {
        sandhi_rules: sandhi_rules,
        pada_map: pada_map,
    };
    let padas = parse(&text, ctx);
    println!("{:?}", padas);
}
