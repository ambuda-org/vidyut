//! Train a model by collecting features from our dataset.

use clap::Parser;
use glob::glob;
use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::path::{Path, PathBuf};

use vidyut_cheda::ModelBuilder;
use vidyut_cheda::State;
use vidyut_data::conllu::Reader;
use vidyut_data::dcs;
use vidyut_lipi::{Lipika, Scheme};
use vidyut_prakriya::args::Pada;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // The base directory to which we will write our training data.
    #[arg(long)]
    model_dir: PathBuf,

    /// Files to use as training data.
    #[arg(short, long, num_args=1..)]
    include: Vec<String>,

    /// Files to exclude from the training data.
    ///
    /// It's best to exclude any files that you will use to evaluate the model.
    #[arg(short, long)]
    exclude: Vec<String>,
}

fn to_slp1(lipika: &mut Lipika, text: &str) -> String {
    lipika.transliterate(text, Scheme::Iast, Scheme::Slp1)
}

fn process_file(path: &Path) -> Result<ModelBuilder> {
    println!("Processing: {:?}", path.display());
    let mut builder = ModelBuilder::new();
    let mut lipika = Lipika::new();

    let reader = Reader::from_path(path)?;
    for sentence in reader {
        let tokens: Vec<(String, Pada)> = sentence
            .tokens
            .iter()
            .flat_map(|t| dcs::standardize(t).ok())
            .collect();

        let mut prev_state = State::new();
        for (raw_lemma, artha) in tokens {
            let slp_lemma = to_slp1(&mut lipika, &raw_lemma);
            let cur_state = State::from_pada(&artha);

            builder.count_transition(prev_state, cur_state);
            builder.count_emission(cur_state, slp_lemma.to_string());

            prev_state = cur_state;
        }
    }

    Ok(builder)
}

fn run(args: Args) -> Result<()> {
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

    let mut paths = Vec::new();
    for path in include_paths {
        if exclude_paths.contains(&path) {
            println!("Skipping excluded path: {:?}", path.display());
        } else {
            paths.push(path);
        }
    }

    let all_stats: Vec<_> = paths
        .par_iter()
        .map(|path| process_file(&path).ok())
        .collect();

    let mut builder = ModelBuilder::new();
    for stats in all_stats {
        if let Some(stats) = stats {
            builder.merge(stats);
        }
    }
    builder.write_model(&args.model_dir)?;

    println!("Complete. Wrote model to {:?}", &args.model_dir);

    Ok(())
}

fn main() {
    let args = Args::parse();

    println!("Beginning training.");
    if let Err(e) = run(args) {
        println!("{}", e);
        std::process::exit(1);
    }
}
