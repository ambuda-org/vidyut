//! Benchmark an FST kosha.
use bencher::black_box;
use clap::Parser;
use fst::Streamer;
use log::info;
use rustc_hash::FxHashMap;
use std::error::Error;
use std::path::PathBuf;
use std::process;
use std::time::{Duration, Instant};

use vidyut_kosha::packing::*;
use vidyut_kosha::Kosha;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type NaiveKosha = FxHashMap<String, Vec<PackedEntry>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the Vidyut data directory
    #[arg(short, long)]
    data_dir: PathBuf,
    /// Ignored
    #[arg(short, long)]
    bench: bool,
}

fn create_naive_kosha(fst_lex: &Kosha) -> Result<NaiveKosha> {
    let mut ret = FxHashMap::default();
    let mut stream = fst_lex.stream();
    while let Some((key, value)) = stream.next() {
        let key = std::str::from_utf8(key)?;
        let value = PackedEntry::from_u32(value as u32);
        let v = ret.entry(key.to_string()).or_insert(Vec::new());
        v.push(value);
    }
    Ok(ret)
}

fn sample_from_fst_kosha(lex: &Kosha, prob: f32) -> Result<Vec<String>> {
    info!("Sampling from kosha (rate = {prob})");

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

fn stats_for_word_sample(dur: &Duration, num_words: usize) {
    let s_elapsed = dur.as_secs_f32();
    let ns_elapsed = dur.as_secs() * 1_000_000_000 + (dur.subsec_nanos() as u64);
    let ns_per_word = ns_elapsed / (num_words as u64);
    println!("Fetched {num_words} arbitrary words in {s_elapsed} seconds ({ns_per_word} ns/word)");
}

fn bench_fst_kosha_sample_1p(lex: &Kosha, words: &[String]) {
    for w in words {
        for _pada in lex.get_all(w) {
            // println!("{w}: {:?}", lex.unpack(&pada));
        }
    }
}

fn bench_naive_kosha_sample_1p(lex: &NaiveKosha, words: &[String]) {
    for w in words {
        if let Some(vec) = lex.get(w) {
            for _pada in vec {
                // println!("{w}: {:?}", lex.unpack(&pada));
            }
        }
    }
}

fn run(args: Args) -> Result<()> {
    info!("Loading kosha");
    let lex = Kosha::new(&args.data_dir)?;
    let words = sample_from_fst_kosha(&lex, 0.01)?;

    println!();
    println!("================================");
    println!("FST kosha");
    println!("================================");
    let start = Instant::now();
    black_box(bench_fst_kosha_sample_1p(&lex, &words));
    let dur = start.elapsed();
    stats_for_word_sample(&dur, words.len());

    println!();
    println!("================================");
    println!("Naive kosha");
    println!("================================");
    info!("Initializing ...");
    let naive_lex = create_naive_kosha(&lex)?;
    info!("Begin.");
    let start = Instant::now();
    black_box(bench_naive_kosha_sample_1p(&naive_lex, &words));
    let dur = start.elapsed();
    stats_for_word_sample(&dur, words.len());

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
