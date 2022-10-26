//! Evaluate our parser against some standard input data.

use clap::{Arg, ArgAction, Command};
use glob::glob;
use std::error::Error;
use std::ops::AddAssign;
use std::path::Path;

use vidyut::conllu::Reader;
use vidyut::dcs;
use vidyut::io;
use vidyut::parsing::{ParsedWord, Parser};
use vidyut::semantics::*;
use vidyut::translit::to_slp1;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

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

/// Converts a word's semantics into a short human-readable code, which we use for comparisons.
///
/// We use codes for our comparison because the DCS data generally contains a subset of the
/// information we have in our `Semantics` object. So, this code is a simple way to convert both
/// Vidyut semantics and DCS semantics into a coarser space.
fn as_code(w: &ParsedWord) -> String {
    match &w.semantics {
        Semantics::Subanta(s) => {
            format!(
                "n-{}-{}-{}",
                s.linga.to_str(),
                s.vibhakti.to_str(),
                s.vacana.to_str()
            )
        }
        Semantics::Tinanta(s) => {
            format!("v-{}-{}", s.purusha.to_str(), s.vacana.to_str())
        }
        Semantics::None => "_".to_string(),
        Semantics::Avyaya => "i".to_string(),
        Semantics::PrefixGroup => "p".to_string(),
        Semantics::Ktva(_) => "ktva".to_string(),
        Semantics::Tumun(_) => "tumun".to_string(),
    }
}

/// Pretty-prints a parsed sentence by displaying its lemmas and parse codes.
fn pretty_print(parse: &[ParsedWord], show_parses: &bool) -> String {
    let mut s = String::new();

    for w in parse {
        let code = as_code(w);
        if *show_parses {
            s += &format!("{} ({})", w.lemma(), code);
        } else {
            s += &w.lemma();
        }
        s += " ";
    }

    s
}

/// Compares two parses and computes summary statistics for the comparison.
fn eval_sentence(vidyut_parses: &[ParsedWord], dcs_parses: &[ParsedWord]) -> Stats {
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
fn eval_input_path(path: &Path, parser: &Parser, show_parses: &bool) -> Result<Stats> {
    let reader = Reader::from_path(path)?;
    let mut stats = Stats::new();

    for sentence in reader {
        let slp1_text = to_slp1(&sentence.text);
        let vidyut_parse = parser.parse(&slp1_text);

        let dcs_parse: Result<Vec<ParsedWord>> =
            sentence.tokens.iter().map(dcs::standardize).collect();
        let dcs_parse = dcs_parse?;

        let sentence_stats = eval_sentence(&vidyut_parse, &dcs_parse);

        let ok = if *show_parses {
            sentence_stats.num_lemma_and_parse_matches == 1
        } else {
            sentence_stats.num_lemma_matches == 1
        };

        if ok {
            println!("[  OK  ]: {}", &slp1_text);
        } else {
            println!("[ FAIL ]: {}", &slp1_text);
            println!("- {}", pretty_print(&vidyut_parse, show_parses));
            println!("- {}", pretty_print(&dcs_parse, show_parses));
        }
        stats += sentence_stats
    }

    Ok(stats)
}

/// Computes summary statistics for the given glob patterns.
fn eval_patterns(patterns: Vec<&String>, parser: &Parser, show_parses: &bool) -> Result<Stats> {
    let mut stats = Stats::new();
    for pattern in patterns {
        let paths = glob(pattern).expect("Glob pattern is invalid").flatten();
        for path in paths {
            stats += eval_input_path(&path, parser, show_parses)?;
        }
    }
    Ok(stats)
}

/// Runs an end-to-end evaluation over the given glob patterns.
fn run_eval(patterns: Vec<&String>, show_parses: bool) -> Result<()> {
    let paths = io::DataPaths::from_dir(Path::new("data"));
    let ctx = Parser::from_paths(&paths)?;
    let stats = eval_patterns(patterns, &ctx, &show_parses)?;

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
    let matches = Command::new("Vidyut model eval")
        .arg(Arg::new("paths").long("paths").num_args(1..))
        .arg(
            Arg::new("show-parses")
                .long("show-parses")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let paths = matches
        .get_many::<String>("paths")
        .map(|v| v.collect::<Vec<_>>())
        .unwrap();

    let show_parses = matches.get_flag("show-parses");

    if let Err(e) = run_eval(paths, show_parses) {
        println!("{}", e);
        std::process::exit(1);
    }
}
