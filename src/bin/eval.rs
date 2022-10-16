use clap::{Arg, Command};
use glob::glob;
use std::error::Error;
use std::ops::AddAssign;
use std::path::PathBuf;

use vidyut::conllu::Reader;
use vidyut::context::Context;
use vidyut::parsing::parse;
use vidyut::translit::to_slp1;

#[derive(Debug)]
struct Stats {
    num_sentences: i32,
    num_lemma_matches: i32,
    num_lemma_and_state_matches: i32,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            num_sentences: 0,
            num_lemma_matches: 0,
            num_lemma_and_state_matches: 0,
        }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, other: Stats) {
        self.num_sentences += other.num_sentences;
        self.num_lemma_matches += other.num_lemma_matches;
        self.num_lemma_and_state_matches += other.num_lemma_and_state_matches;
    }
}

fn standardize_dcs_lemma(raw_lemma: &str) -> String {
    let lemma = to_slp1(raw_lemma);

    // Bagavant, hanumant, etc.
    if let Some(fragment) = lemma.strip_suffix("ant") {
        return String::from(fragment) + "at";
    }
    // kIrtay,
    if let Some(fragment) = lemma.strip_suffix("ay") {
        return String::from(fragment);
    }
    match lemma.as_str() {
        "mad" => "asmad".to_string(),
        "tvad" => "yuzmad".to_string(),
        _ => lemma,
    }
}

fn is_vec_match(left: &Vec<String>, right: &Vec<String>) -> bool {
    if left.len() == right.len() {
        let num_matches = left
            .iter()
            .zip(right.iter())
            .filter(|&(a, b)| a == b)
            .count();
        num_matches == left.len()
    } else {
        false
    }
}

fn eval_path(path: PathBuf, ctx: &Context) -> Result<Stats, Box<dyn Error>> {
    let reader = Reader::from_path(&path)?;
    let mut stats = Stats::new();

    for sentence in reader {
        stats.num_sentences += 1;

        let slp1_text = to_slp1(&sentence.text);

        let vidyut_parse = parse(&slp1_text, ctx);
        let vidyut_lemmas: Vec<_> = vidyut_parse.iter().map(|p| p.lemma()).collect();
        let dcs_lemmas: Vec<_> = sentence
            .tokens
            .iter()
            .map(|t| standardize_dcs_lemma(&t.lemma))
            .collect();

        if is_vec_match(&vidyut_lemmas, &dcs_lemmas) {
            println!("[  OK  ]: {}", &slp1_text);
            stats.num_lemma_matches += 1;
        } else {
            println!("[ FAIL ]: {}", &slp1_text);
            println!("- {:?}", &vidyut_lemmas);
            println!("- {:?}", &dcs_lemmas);
        }
    }

    Ok(stats)
}

fn eval_patterns(patterns: Vec<&String>, ctx: &Context) -> Result<Stats, Box<dyn Error>> {
    let mut stats = Stats::new();
    for pattern in patterns {
        let paths = glob(pattern).expect("Glob pattern is invalid").flatten();
        for path in paths {
            println!("Parsing: {}", path.display());
            stats += eval_path(path, ctx)?;
        }
    }
    Ok(stats)
}

fn run_eval(patterns: Vec<&String>, cache_file: &str) -> Result<(), Box<dyn Error>> {
    let ctx = Context::from_snapshot(cache_file)?;
    let stats = eval_patterns(patterns, &ctx)?;

    let pct = |x, y| 100_f32 * (x as f32) / (y as f32);
    let lemma_pct = pct(stats.num_lemma_matches, stats.num_sentences);

    println!();
    println!("================================================");
    println!("               Evaluation summary               ");
    println!("================================================");
    println!(
        "Number of sentences:                    {: >8}",
        stats.num_sentences
    );
    println!(
        "Percentage with exact lemma match:     {: >8.2}%",
        lemma_pct
    );
    println!("================================================");
    println!();
    Ok(())
}

fn main() {
    let matches = Command::new("Vidyut model eval")
        .arg(Arg::new("paths").long("paths").num_args(1..))
        .arg(Arg::new("cache-file").long("cache-file"))
        .get_matches();

    let paths = matches
        .get_many::<String>("paths")
        .map(|v| v.collect::<Vec<_>>())
        .unwrap();

    let cache_file = matches.get_one::<String>("cache-file").unwrap();

    if let Err(e) = run_eval(paths, cache_file) {
        println!("{}", e);
        std::process::exit(1);
    }
}
