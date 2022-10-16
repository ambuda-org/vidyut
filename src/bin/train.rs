/// Train a model by collecting features from our dataset.
use glob::glob;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use vidyut::conllu::{Reader, Sentence, Token, TokenFeatures};
use vidyut::translit::to_slp1;

/// Freq(`state[n]` | `state[n-1]`).
///
/// The first key is `state[n-1]` so that we can normalize more easily.
type Transitions = HashMap<String, HashMap<String, u32>>;

/// Freq(`lemma[n]` | `state[n]`).
///
/// The first key is `state[n]` so that we can normalize more easily.
///
/// Ideally, we should use `token[n]` instead of `lemma[n]`. However, the DCS data doesn't
/// realiably expose the inflected word for a given entry. Additionally, using the lemma helps us
/// work around data sparsity.
type Emissions = HashMap<String, HashMap<String, u32>>;

/// Value of state_0 and any other tokens with unclear semantics.
const INITIAL_STATE: &str = "START";

/// Create a state label for the given nominal (noun, pronoun, adjective, numeral).
///
/// The state describes gender, case, and number, which are sufficient for our current needs.
fn nominal_state(features: &TokenFeatures) -> String {
    let gender = match features.get("Gender") {
        Some(s) => match s.as_str() {
            "Masc" => "m",
            "Fem" => "f",
            "Neut" => "n",
            &_ => panic!("Unknown gender `{}`", s),
        },
        None => "_",
    };
    let case = match features.get("Case") {
        Some(s) => match s.as_str() {
            "Nom" => "1",
            "Acc" => "2",
            "Ins" => "3",
            "Dat" => "4",
            "Abl" => "5",
            "Gen" => "6",
            "Loc" => "7",
            "Voc" => "8",
            "Cpd" => "comp",
            &_ => panic!("Unknown case `{}`", s),
        },
        None => "_",
    };
    let number = match features.get("Number") {
        Some(s) => match s.as_str() {
            "Sing" => "s",
            "Dual" => "d",
            "Plur" => "p",
            &_ => panic!("Unknown number `{}`", s),
        },
        None => "_",
    };
    // "n" for nominal
    format!("n-{}-{}-{}", gender, case, number)
}

/// Create a state label for the given verb.
///
/// The state describes person and number, which are sufficient for our current needs.
fn tinanta_state(features: &TokenFeatures) -> String {
    let person = match features.get("Person") {
        Some(s) => match s.as_str() {
            "3" => "3",
            "2" => "2",
            "1" => "1",
            &_ => panic!("Unknown person `{}`", s),
        },
        None => "_",
    };
    let number = match features.get("Number") {
        Some(s) => match s.as_str() {
            "Sing" => "s",
            "Dual" => "d",
            "Plur" => "p",
            &_ => panic!("Unknown number `{}`", s),
        },
        None => "_",
    };
    // "v" for verb
    format!("v-{}-{}", person, number)
}

/// Create a state label for the given avyaya.
fn avyaya_state() -> String {
    // "i" for indeclinable
    "i".to_string()
}

fn unknown_state() -> String {
    INITIAL_STATE.to_string()
}

/// Create a state label for the given token.
fn token_state(token: &Token) -> String {
    let upos = &token.upos;
    let features = &token.features;

    match upos.as_str() {
        "NOUN" | "PRON" | "ADJ" | "PART" | "NUM" => nominal_state(features),
        "CCONJ" | "SCONJ" | "ADV" => avyaya_state(),
        "VERB" => tinanta_state(features),
        "MANTRA" => unknown_state(),
        _ => panic!("Unknown upos `{}`", upos),
    }
}

fn process_sentence(sentence: Sentence, transitions: &mut Transitions, emissions: &mut Emissions) {
    let mut prev_state = INITIAL_STATE.to_string();
    for token in sentence.tokens {
        let cur_state = token_state(&token);
        let lemma = token.lemma;

        // Freq(cur_state | prev_state )
        let c = transitions
            .entry(prev_state.clone())
            .or_insert_with(HashMap::new)
            .entry(cur_state.clone())
            .or_insert(0);
        *c += 1;

        // Freq(cur_token | cur_state )
        //
        // The DCS data doesn't contain explicit forms, so make do with the lemma.
        let c = emissions
            .entry(cur_state.clone())
            .or_insert_with(HashMap::new)
            .entry(to_slp1(&lemma))
            .or_insert(0);
        *c += 1;

        prev_state = cur_state;
    }
}

fn process_file(
    path: PathBuf,
    transitions: &mut Transitions,
    emissions: &mut Emissions,
) -> Result<(), Box<dyn Error>> {
    let reader = Reader::from_path(&path)?;
    for sentence in reader {
        process_sentence(sentence, transitions, emissions);
    }
    Ok(())
}

fn write_transitions(transitions: Transitions, path: &str) -> Result<(), Box<dyn Error>> {
    let mut w = csv::Writer::from_path(path)?;
    for (prev_state, counts) in transitions {
        let n = counts.values().sum::<u32>();
        for (cur_state, count) in counts {
            let prob = (count as f64) / (n as f64);
            w.write_record(&[&prev_state, &cur_state, &prob.to_string()])?;
        }
        w.flush()?;
    }
    Ok(())
}

fn write_emissions(emissions: Emissions, path: &str) -> Result<(), Box<dyn Error>> {
    let mut w = csv::Writer::from_path(path)?;
    for (state, counts) in emissions {
        let n = counts.values().sum::<u32>();
        for (token, count) in counts {
            let prob = (count as f64) / (n as f64);
            w.write_record(&[&state, &token, &prob.to_string()])?;
        }
        w.flush()?;
    }
    Ok(())
}

fn process_files() -> Result<(), Box<dyn Error>> {
    let paths = glob("dcs-data/dcs/data/conllu/files/**/*.conllu")
        .expect("Glob pattern is invalid")
        .flatten();
    let mut transitions = Transitions::new();
    let mut emissions = Emissions::new();
    for path in paths {
        println!("Processing: {:?}", path.display());
        process_file(path, &mut transitions, &mut emissions)?;
    }

    write_transitions(transitions, "data/model/transitions.csv")?;
    write_emissions(emissions, "data/model/emissions.csv")?;
    Ok(())
}

fn main() {
    println!("Beginning training.");
    if let Err(e) = process_files() {
        println!("{}", e);
        std::process::exit(1);
    }
}
