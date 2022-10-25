/// Train a model by collecting features from our dataset.
use clap::{Arg, Command};
use glob::glob;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;

use vidyut::conllu::Reader;
use vidyut::dcs;
use vidyut::parsing::ParsedWord;
use vidyut::semantics::*;
use vidyut::translit::to_slp1;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

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

/// Freq(`lemma[n]`)
///
/// Simple frequency counts on the lemma.
type Counts = HashMap<String, u32>;

struct Statistics {
    transitions: Transitions,
    emissions: Emissions,
    counts: Counts,
}

/// Value of state_0 and any other tokens with unclear semantics.
const INITIAL_STATE: &str = "START";

/// Create a state label for the given subanta (noun, pronoun, adjective, numeral).
///
/// The state describes linga, vibhakti, and vacana, which are sufficient for our current needs.
fn subanta_state(s: &Subanta) -> String {
    format!(
        "n-{}-{}-{}",
        s.linga.to_str(),
        s.vibhakti.to_str(),
        s.vacana.to_str()
    )
}

/// Create a state label for the given tinanta.
///
/// The state describes purusha and vacana, which are sufficient for our current needs.
fn tinanta_state(t: &Tinanta) -> String {
    // "v" for verb
    format!("v-{}-{}", t.purusha.to_str(), t.vacana.to_str())
}

/// Create a state label for the given avyaya.
fn avyaya_state() -> String {
    // "i" for indeclinable
    "i".to_string()
}

fn unknown_state() -> String {
    INITIAL_STATE.to_string()
}

/// Create a state label for the given word.
fn word_state(w: &ParsedWord) -> String {
    match &w.semantics {
        Semantics::Subanta(s) => subanta_state(s),
        Semantics::Tinanta(t) => tinanta_state(t),
        Semantics::Avyaya => avyaya_state(),
        _ => unknown_state(),
    }
}

fn process_sentence(sentence: &[ParsedWord], s: &mut Statistics) {
    let mut prev_state = INITIAL_STATE.to_string();
    for word in sentence {
        let cur_state = word_state(word);
        let lemma = word.lemma();

        // Freq(cur_state | prev_state )
        let c = s
            .transitions
            .entry(prev_state.clone())
            .or_insert_with(HashMap::new)
            .entry(cur_state.clone())
            .or_insert(0);
        *c += 1;

        // Freq(cur_token | cur_state )
        //
        // The DCS data doesn't contain explicit forms, so make do with the lemma.
        let c = s
            .emissions
            .entry(cur_state.clone())
            .or_insert_with(HashMap::new)
            .entry(to_slp1(&lemma))
            .or_insert(0);
        *c += 1;

        let c = s.counts.entry(lemma).or_insert(0);
        *c += 1;

        prev_state = cur_state;
    }
}

fn process_file(path: PathBuf, s: &mut Statistics) -> Result<()> {
    let reader = Reader::from_path(&path)?;
    for sentence in reader {
        let words: Result<Vec<_>> = sentence.tokens.iter().map(dcs::standardize).collect();
        let words = words?;
        process_sentence(&words, s);
    }
    Ok(())
}

fn write_transitions(transitions: Transitions, path: &str) -> Result<()> {
    let mut w = csv::Writer::from_path(path)?;
    w.write_record(&["prev_state", "cur_state", "probability"])?;

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

fn write_emissions(emissions: Emissions, path: &str) -> Result<()> {
    let mut w = csv::Writer::from_path(path)?;
    w.write_record(&["state", "token", "probability"])?;

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

/// Writes each observed lemma with its frequency.
///
/// We write just the raw frequency so that downstream logic can more easily implement its own
/// normalization on top.
fn write_lemma_counts(counts: Counts, path: &str) -> Result<()> {
    let mut w = csv::Writer::from_path(path)?;
    w.write_record(&["lemma", "count"])?;

    for (lemma, count) in counts {
        w.write_record(&[&lemma, &count.to_string()])?;
        w.flush()?;
    }
    Ok(())
}

fn process_files(include_patterns: &[&String], exclude_patterns: &[&String]) -> Result<()> {
    let mut stats = Statistics {
        transitions: Transitions::new(),
        emissions: Emissions::new(),
        counts: Counts::new(),
    };

    let include_paths: Vec<_> = include_patterns
        .iter()
        .flat_map(|p| glob(p).expect("Glob pattern is invalid").flatten())
        .collect();

    let exclude_paths: HashSet<_> = exclude_patterns
        .iter()
        .flat_map(|p| glob(p).expect("Glob pattern is invalid").flatten())
        .collect();

    // TODO: implement include/exclude logic.

    for path in include_paths {
        if exclude_paths.contains(&path) {
            println!("Skipping excluded path: {:?}", path.display());
        } else {
            println!("Processing: {:?}", path.display());
            process_file(path, &mut stats)?;
        }
    }

    write_transitions(stats.transitions, "data/model/transitions.csv")?;
    write_emissions(stats.emissions, "data/model/emissions.csv")?;
    write_lemma_counts(stats.counts, "data/model/lemma-counts.csv")?;
    Ok(())
}

fn main() {
    let matches = Command::new("Vidyut model training")
        .arg(Arg::new("include").long("include").num_args(1..))
        .arg(Arg::new("exclude").long("exclude").num_args(0..))
        .get_matches();

    let include = matches
        .get_many::<String>("include")
        .map(|v| v.collect::<Vec<_>>())
        .unwrap();

    let exclude = matches
        .get_many::<String>("exclude")
        .map(|v| v.collect::<Vec<_>>())
        .unwrap();

    println!("Beginning training.");
    if let Err(e) = process_files(&include, &exclude) {
        println!("{}", e);
        std::process::exit(1);
    }
}
