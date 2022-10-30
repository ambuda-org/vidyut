//! Benchmark an FST lexicon.
use clap::Parser;
use fst::Streamer;
use log::info;
use std::error::Error;
use std::path::Path;
use std::process;
use std::time::Instant;

use vidyut::fst_lexicon::FstLexicon;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the Vidyut data directory
    #[arg(short, long)]
    data_dir: String,
    /// Ignored
    #[arg(short, long)]
    bench: bool,
}

fn sample(lex: &FstLexicon, prob: f32) -> Result<Vec<String>> {
    info!("Sampling from lexicon (rate = {prob})");

    let mut keys = Vec::new();
    let mut stream = lex.stream();
    while let Some((key, _v)) = stream.next() {
        if rand::random::<f32>() < prob {
            keys.push(std::str::from_utf8(key)?.to_string())
        }
    }
    info!("Sampled {} words", keys.len());
    Ok(keys)
}

fn run(args: Args) -> Result<()> {
    info!("Loading lexicon");
    let lex = FstLexicon::load_from(Path::new(&args.data_dir))?;

    let words = sample(&lex, 0.01)?;

    let start = Instant::now();
    for w in &words {
        for _pada in lex.get_all(w) {
            // println!("{w}: {:?}", lex.unpack(&pada));
        }
    }
    let dur = start.elapsed();

    let num_words = words.len();
    let s_elapsed = dur.as_secs_f32();
    let ns_elapsed = dur.as_secs() * 1_000_000_000 + (dur.subsec_nanos() as u64);
    let ns_per_word = ns_elapsed / (num_words as u64);

    println!("Fetched {num_words} arbitrary words in {s_elapsed} seconds ({ns_per_word} ns/word)");

    Ok(())
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    if let Err(err) = run(args) {
        println!("{}", err);
        process::exit(1);
    }
}
