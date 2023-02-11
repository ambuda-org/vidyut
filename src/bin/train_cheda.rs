/// Train a model by collecting features from our dataset.
use clap::Parser;
use glob::glob;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use vidyut_cheda::conllu::Reader;
use vidyut_cheda::dcs;
use vidyut_cheda::model::State;
use vidyut_cheda::Result;
use vidyut_cheda::{Config, Token};
use vidyut_kosha::morph::*;
use vidyut_lipi::{transliterate, Scheme};

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

/// Freq(`state[n]` | `state[n-1]`).
///
/// The first key is `state[n-1]` so that we can normalize more easily.
type Transitions = HashMap<State, HashMap<State, u32>>;

/// Freq(`lemma[n]` | `state[n]`).
///
/// The first key is `state[n]` so that we can normalize more easily.
///
/// Ideally, we should use `token[n]` instead of `lemma[n]`. However, the DCS data doesn't
/// realiably expose the inflected word for a given entry. Additionally, using the lemma helps us
/// work around data sparsity.
type Emissions = HashMap<State, HashMap<String, u32>>;

/// Freq(`lemma[n]`)
///
/// Simple frequency counts on the lemma and part of speech.
type Counts = HashMap<(String, POSTag), u32>;

struct Statistics {
    transitions: Transitions,
    emissions: Emissions,
    lemma_counts: Counts,
}

fn to_slp1(text: &str) -> String {
    transliterate(text, Scheme::Iast, Scheme::Slp1)
}

fn process_sentence(tokens: &[Token], s: &mut Statistics) {
    let mut prev_state = State::new();
    for token in tokens {
        let cur_state = State::from_pada(&token.info);
        let lemma = token.lemma();

        // Freq(cur_state | prev_state )
        let c = s
            .transitions
            .entry(prev_state)
            .or_insert_with(HashMap::new)
            .entry(cur_state)
            .or_insert(0);
        *c += 1;

        // Freq(cur_token | cur_state )
        //
        // The DCS data doesn't contain explicit forms, so make do with the lemma.
        let c = s
            .emissions
            .entry(cur_state)
            .or_insert_with(HashMap::new)
            .entry(to_slp1(lemma))
            .or_insert(0);
        *c += 1;

        let tag = token.info.part_of_speech_tag();
        let c = s.lemma_counts.entry((lemma.to_string(), tag)).or_insert(0);
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
    w.write_record(["prev_state", "cur_state", "probability"])?;

    for (prev_state, counts) in transitions {
        let n = counts.values().sum::<u32>();
        for (cur_state, count) in counts {
            let prob = (count as f64) / (n as f64);
            w.write_record([
                &prev_state.to_string(),
                &cur_state.to_string(),
                &prob.to_string(),
            ])?;
        }
        w.flush()?;
    }
    Ok(())
}

fn write_emissions(emissions: Emissions, path: &Path) -> Result<()> {
    let mut w = csv::Writer::from_path(path)?;
    w.write_record(["state", "token", "probability"])?;

    for (state, counts) in emissions {
        let n = counts.values().sum::<u32>();
        for (token, count) in counts {
            let prob = (count as f64) / (n as f64);
            w.write_record([&state.to_string(), &token, &prob.to_string()])?;
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
    w.write_record(["lemma", "tag", "count"])?;

    for ((lemma, tag), count) in counts {
        w.write_record([&lemma, &tag.to_string(), &count.to_string()])?;
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
