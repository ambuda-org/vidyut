use log::info;
use std::collections::HashSet;
use std::process;

mod sandhi;

fn is_word(word: &str) -> bool {
    let mut words: HashSet<String> = HashSet::new();
    words.insert("Darma".to_string());
    words.insert("kzetre".to_string());
    words.insert("kuru".to_string());
    words.insert("samavetAs".to_string());
    words.insert("yuyutsavaH".to_string());
    words.insert("mAmakAH".to_string());
    words.insert("pARqavAH".to_string());
    words.insert("ca".to_string());
    words.insert("eva".to_string());
    words.insert("kim".to_string());
    words.insert("akurvata".to_string());
    words.insert("saMjaya".to_string());

    words.contains(word)
}

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

    let mut state = text.clone();
    let mut words = Vec::new();
    while !state.is_empty() {
        let mut changed_state = false;
        for (first, second) in sandhi::split(&state, &rules) {
            if !sandhi::is_good_split(&text, &first, &second) {
                continue;
            }
            if is_word(&first) {
                words.push(first);
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
    println!("{:?}", words);
}
