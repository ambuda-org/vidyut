/// Train a model by collecting features from our dataset.
use clap::Parser;
use glob::glob;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::Path;

use vidyut::config::Config;
use vidyut::conllu::Reader;
use vidyut::dcs;
use vidyut::segmenting::Word;
use vidyut::semantics::*;
use vidyut::translit::to_slp1;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // The base directory to which we will write our training data.
    #[arg(long)]
    vidyut_dir: String,

    /// Files to use as training data.
    #[arg(short, long, num_args=1..)]
    include: Vec<String>,

    /// Files to exclude from the training data.
    ///
    /// It's best to exclude any files that you will use to evaluate the model.
    #[arg(short, long)]
    exclude: Vec<String>,
}

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
/// Simple frequency counts on the lemma and PartOfSpeech.
type Counts = HashMap<String, u32>;

struct Statistics {
    transitions: Transitions,
    emissions: Emissions,
    lemma_counts: Counts,
}

/// Value of state_0 and any other tokens with unclear semantics.
const INITIAL_STATE: &str = "START";

/// Create a state label for the given subanta (noun, pronoun, adjective, numeral).
///
/// The state describes linga, vibhakti, and vacana, which are sufficient for our current needs.
fn subanta_state(s: &Subanta) -> String {
    format!(
        "n-{}-{}-{}",
        s.linga.as_str(),
        s.vibhakti.as_str(),
        s.vacana.as_str()
    )
}

/// Create a state label for the given tinanta.
///
/// The state describes purusha and vacana, which are sufficient for our current needs.
fn tinanta_state(t: &Tinanta) -> String {
    // "v" for verb
    format!("v-{}-{}", t.purusha.as_str(), t.vacana.as_str())
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
fn word_state(w: &Word) -> String {
    match &w.semantics {
        Pada::Subanta(s) => subanta_state(s),
        Pada::Tinanta(t) => tinanta_state(t),
        Pada::Avyaya(_) => avyaya_state(),
        Pada::None => unknown_state(),
    }
}

fn process_sentence(sentence: &[Word], s: &mut Statistics) {
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

        let c = s.lemma_counts.entry(lemma).or_insert(0);
        *c += 1;

        prev_state = cur_state;
    }
}

fn process_file(path: &Path, s: &mut Statistics) -> Result<()> {
    let reader = Reader::from_path(path)?;
    for sentence in reader {
        let words: Result<Vec<_>> = sentence.tokens.iter().map(dcs::standardize).collect();
        let words = words?;
        process_sentence(&words, s);
    }
    Ok(())
}

fn write_transitions(transitions: Transitions, path: &Path) -> Result<()> {
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

fn write_emissions(emissions: Emissions, path: &Path) -> Result<()> {
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
fn write_lemma_counts(counts: Counts, path: &Path) -> Result<()> {
    let mut w = csv::Writer::from_path(path)?;
    w.write_record(&["lemma", "count"])?;

    for (lemma, count) in counts {
        w.write_record(&[&lemma, &count.to_string()])?;
        w.flush()?;
    }
    Ok(())
}

fn process_files(args: Args) -> Result<()> {
    let mut stats = Statistics {
        transitions: Transitions::new(),
        emissions: Emissions::new(),
        lemma_counts: Counts::new(),
    };

    let include_paths: Vec<_> = args
        .include
        .iter()
        .flat_map(|p| glob(p).expect("Glob pattern is invalid").flatten())
        .collect();

    let exclude_paths: HashSet<_> = args
        .exclude
        .iter()
        .flat_map(|p| glob(p).expect("Glob pattern is invalid").flatten())
        .collect();

    for path in include_paths {
        if exclude_paths.contains(&path) {
            println!("Skipping excluded path: {:?}", path.display());
        } else {
            println!("Processing: {:?}", path.display());
            process_file(&path, &mut stats)?;
        }
    }

    let config = Config::new(Path::new(&args.vidyut_dir));
    config.create_dirs()?;
    write_transitions(stats.transitions, &config.model_transitions())?;
    write_emissions(stats.emissions, &config.model_emissions())?;
    write_lemma_counts(stats.lemma_counts, &config.model_lemma_counts())?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    println!("Beginning training.");
    if let Err(e) = process_files(args) {
        println!("{}", e);
        std::process::exit(1);
    }
}
