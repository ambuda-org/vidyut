//! Evaluate our segmenter against some standard input data.

use clap::Parser;
use glob::glob;
use std::ops::AddAssign;
use std::path::{Path, PathBuf};

use vidyut_cheda::conllu::Reader;
use vidyut_cheda::dcs;
use vidyut_cheda::Result;
use vidyut_cheda::{Chedaka, Config, Token};
use vidyut_kosha::morph::*;
use vidyut_lipi::{transliterate, Scheme};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // The base directory from which we read our model.
    #[arg(long)]
    vidyut_dir: PathBuf,

    /// Paths to use for the eval.
    #[arg(short, long, num_args=1..)]
    paths: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    show_semantics: bool,
}

/// Summary statistics for our eval.
#[derive(Debug)]
struct Stats {
    num_sentences: i32,
    num_equal_length: i32,
    num_lemma_matches: i32,
    num_parse_matches: i32,
    num_lemma_and_parse_matches: i32,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            num_sentences: 0,
            num_equal_length: 0,
            num_lemma_matches: 0,
            num_parse_matches: 0,
            num_lemma_and_parse_matches: 0,
        }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, other: Stats) {
        self.num_sentences += other.num_sentences;
        self.num_equal_length += other.num_equal_length;
        self.num_lemma_matches += other.num_lemma_matches;
        self.num_lemma_and_parse_matches += other.num_lemma_and_parse_matches;
    }
}

fn to_slp1(text: &str) -> String {
    transliterate(text, Scheme::Iast, Scheme::Slp1)
}

/// Converts a word's semantics into a short human-readable code, which we use for comparisons.
///
/// We use codes for our comparison because the DCS data generally contains a subset of the
/// information we have in our `Pada` enum. So, this code is a simple way to convert both
/// Vidyut semantics and DCS semantics into a coarser space.
fn as_code(w: &Token) -> String {
    match &w.info {
        Pada::Subanta(s) => {
            format!(
                "n-{}-{}-{}",
                s.linga.map_or("", |x| x.as_str()),
                s.vibhakti.map_or("", |x| x.as_str()),
                s.vacana.map_or("", |x| x.as_str())
            )
        }
        Pada::Tinanta(s) => {
            format!("v-{}-{}", s.purusha.as_str(), s.vacana.as_str())
        }
        Pada::Unknown => "_".to_string(),
        Pada::Avyaya(a) => {
            let val = match &a.pratipadika {
                Pratipadika::Basic { .. } => "i",
                Pratipadika::Krdanta { pratyaya, .. } => match pratyaya {
                    KrtPratyaya::Ktva => "ktva",
                    KrtPratyaya::Tumun => "tumun",
                    _ => "_",
                },
            };
            val.to_string()
        }
    }
}

/// Pretty-prints a parsed sentence by displaying its lemmas and parse codes.
fn pretty_print(parse: &[Token], show_semantics: &bool) -> String {
    let mut s = String::new();

    for w in parse {
        let code = as_code(w);
        if *show_semantics {
            s += &format!("{} ({})", w.lemma(), code);
        } else {
            s += w.lemma();
        }
        s += " ";
    }

    s
}

/// Compares two parses and computes summary statistics for the comparison.
fn eval_sentence(vidyut_parses: &[Token], dcs_parses: &[Token]) -> Stats {
    let mut stats = Stats::new();
    stats.num_sentences = 1;

    if vidyut_parses.len() == dcs_parses.len() {
        stats.num_equal_length = 1;
    }

    let vidyut_lemmas: Vec<_> = vidyut_parses.iter().map(|w| w.lemma()).collect();
    let dcs_lemmas: Vec<_> = dcs_parses.iter().map(|w| w.lemma()).collect();
    let same_lemmas = vidyut_lemmas == dcs_lemmas;

    if same_lemmas {
        stats.num_lemma_matches = 1;
    }

    let vidyut_codes: Vec<_> = vidyut_parses.iter().map(as_code).collect();
    let dcs_codes: Vec<_> = dcs_parses.iter().map(as_code).collect();
    let same_codes = vidyut_codes == dcs_codes;

    if same_codes {
        stats.num_parse_matches = 1;
    }

    if same_lemmas && same_codes {
        stats.num_lemma_and_parse_matches = 1;
    }

    stats
}

/// Computes summary statistics for the given file.
fn eval_input_path(path: &Path, segmenter: &Chedaka, show_semantics: &bool) -> Result<Stats> {
    let reader = Reader::from_path(path)?;
    let mut stats = Stats::new();

    for sentence in reader {
        let slp1_text = to_slp1(&sentence.text);
        let vidyut_parse = segmenter.run(&slp1_text)?;

        let dcs_parse: std::result::Result<Vec<Token>, _> =
            sentence.tokens.iter().map(dcs::standardize).collect();
        let dcs_parse = dcs_parse?;

        let sentence_stats = eval_sentence(&vidyut_parse, &dcs_parse);

        let ok = if *show_semantics {
            sentence_stats.num_lemma_and_parse_matches == 1
        } else {
            sentence_stats.num_lemma_matches == 1
        };

        if ok {
            println!("[  OK  ]: {}", &slp1_text);
        } else {
            println!("[ FAIL ]: {}", &slp1_text);
            println!("vid: {}", pretty_print(&vidyut_parse, show_semantics));
            println!("dcs: {}", pretty_print(&dcs_parse, show_semantics));
        }
        println!();
        stats += sentence_stats
    }

    Ok(stats)
}

/// Computes summary statistics for the given glob patterns.
fn eval_patterns(
    patterns: Vec<String>,
    segmenter: &Chedaka,
    show_semantics: &bool,
) -> Result<Stats> {
    let mut stats = Stats::new();
    for pattern in patterns {
        let paths = glob(&pattern).expect("Glob pattern is invalid").flatten();
        for path in paths {
            stats += eval_input_path(&path, segmenter, show_semantics)?;
        }
    }
    Ok(stats)
}

/// Runs an end-to-end evaluation over the given glob patterns.
fn run_eval(args: Args) -> Result<()> {
    let config = Config::new(&args.vidyut_dir);
    let segmenter = Chedaka::new(config)?;
    let stats = eval_patterns(args.paths, &segmenter, &args.show_semantics)?;

    let pct = |x, y| 100_f32 * (x as f32) / (y as f32);
    let lemma_pct = pct(stats.num_lemma_matches, stats.num_sentences);
    let lemma_and_parse_pct = pct(stats.num_lemma_and_parse_matches, stats.num_sentences);
    let length_pct = pct(stats.num_equal_length, stats.num_sentences);

    println!();
    println!("================================================");
    println!("               Evaluation summary               ");
    println!("================================================");
    println!(
        "Number of sentences:                    {: >8}",
        stats.num_sentences
    );
    println!(
        "% with the same number of tokens:       {: >8.2}",
        length_pct,
    );
    println!(
        "% with exact lemma match:               {: >8.2}",
        lemma_pct
    );
    println!(
        "% with exact lemma+parse match:         {: >8.2}",
        lemma_and_parse_pct
    );
    println!("================================================");
    println!();
    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run_eval(args) {
        println!("{}", e);
        std::process::exit(1);
    }
}
