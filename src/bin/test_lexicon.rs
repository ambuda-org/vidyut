//! Test the production setup.

use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use vidyut::config::Config;
use vidyut::segmenting::Segmenter;
use vidyut_kosha::semantics::Pada;
use vidyut_kosha::Kosha;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // The base directory from which we read our model.
    #[arg(long)]
    data_dir: PathBuf,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn test_lexicon_tinantas(lex: &Kosha) -> Result<()> {
    let keys = vec![
        // Basic lakaras (kartari, karmani/bhAve)
        "nayati",
        "ninAya",
        "netA",
        "nezyati",
        "nayatu",
        "anayat",
        // "nIyAt",
        "nayet",
        "anEzIt",
        // "anezyat",
        "nIyate",
        "nIyatAm",
        "anIyata",
        "nIyeta",
        // san dhAtus (kartari, karmani/bhAve)
        "ninIzati",
        "ninIzatu",
        "aninIzat",
        "ninIzet",
        "ninIzyate",
        "ninIzyatAm",
        "aninIzyata",
        "ninIzyeta",
        // Nic dhAtus (kartari, karmani/bhAve)
        "nAyayati",
        "nAyayatu",
        "anAyayat",
        "nAyayet",
        "nAyyate",
        "nAyyatAm",
        "anAyyata",
        "nAyyeta",
        // TODO: yaG
    ];

    for key in keys {
        assert!(lex.contains_key(key), "{key}");
    }
    Ok(())
}

fn test_lexicon_subantas(lex: &Kosha) -> Result<()> {
    let keys = vec![
        ("devas", "deva"),
        ("senA", "senA"),
        ("agnis", "agni"),
        ("devI", "devI"),
        ("gurus", "guru"),
        ("vaDUs", "vaDU"),
        ("kartA", "kartf"),
        // ("rEs", "rE"),
        // "dyOs",
        ("nOs", "nO"),
        ("AtmA", "Atman"),
        ("manasA", "manas"),
        ("havizA", "havis"),
        ("DanurByAm", "Danus"),
        ("hanumAn", "hanumat"),
        ("Bagavantam", "Bagavat"),
        ("jagmivAn", "jagmivas"),
        // Consonant stems
        ("vAk", "vAc"),
        ("vit", "vid"),
        // ("kakuB", "kakup"),

        // Irregular subantas
        ("mahAn", "mahat"),
        ("tri", "trayas"),
        ("zaz", "zaRRAm"),
        ("sapta", "saptan"),
        ("daSa", "daSan"),
        ("pitaras", "pitf"),
        ("mAtaras", "mAtf"),
        ("BrAtaras", "BrAtf"),
        ("panTAnam", "paTin"),
        ("patyus", "pati"),
        ("yUnAm", "yuvan"),
    ];

    for (key, lemma) in keys {
        let entries: std::result::Result<Vec<Pada>, _> =
            lex.get_all(key).iter().map(|x| lex.unpack(x)).collect();
        let entries = entries?;

        assert!(
            entries.iter().any(|x| x.lemma() == lemma),
            "{} {}",
            key,
            lemma
        );
    }
    Ok(())
}

fn run_tests(args: Args) -> Result<()> {
    let config = Config::new(&args.data_dir);
    let segmenter = Segmenter::new(config)?;
    let lex = segmenter.lexicon();

    test_lexicon_tinantas(lex)?;
    test_lexicon_subantas(lex)?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run_tests(args) {
        println!("{}", e);
        std::process::exit(1);
    }
}
