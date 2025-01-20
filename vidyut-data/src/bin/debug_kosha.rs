//! Utilities for debugging and understanding data for a `Kosha`.
//!
//! I wrote this debugger both to quickly diagnose errors and to find better ways of
//! reducing the size of the kosha.
//!
//! Size metrics:
//!
//! FST basic only:           5.7M (13.2M entries)
//! FST avyayas only:        38.0K (5K entries)
//! FST tinantas only:       22.0M (15.7M entries)
//! FST krdantas only:       11.0M (2.3M prefix entries)
//! FST krdantas + tinantas: 55.0M (17.8M entries)
//!                          35.0M (17.8M entries, no nic+san or san+nic)
//!                          16.0M (17.8M entries, verbs are not sanadi)
//!                           6.0M (17.8M entries, no sanadi)
//! FST krdantas + basic:    19.0M (15.4M entries)
//! FST all:                 65.0M (35M entries)
use clap::{Args, Parser, Subcommand};
use fst::Streamer;
use regex::Regex;
use std::error::Error;
use std::path::PathBuf;
use vidyut_kosha::entries::{KrdantaEntry, PadaEntry, PratipadikaEntry};
use vidyut_kosha::packing::{PackedEntry, PartOfSpeech};
use vidyut_kosha::Kosha;
use vidyut_prakriya::args::Dhatu;
use vidyut_prakriya::Vyakarana;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct MainArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List all dhatus in the kosha.
    Dhatus(DhatuArgs),
    /// List all pratipadikas in the kosha.
    Pratipadikas(PratipadikaArgs),
    /// List all paradigms used in the kosha.
    Paradigms(ParadigmArgs),
    /// List all entries in the kosha.
    Entries(EntryArgs),
    /// List all prakriyas that derive the given key.
    ///
    /// TODO: buggy for "DraB", perhaps other SubantaPrefix entries with empty prefixes.
    Derive(DeriveArgs),
}

#[derive(Debug, Args)]
struct DhatuArgs {
    /// Path to the kosha data.
    #[arg(long)]
    data_dir: PathBuf,
}

#[derive(Debug, Args)]
struct PratipadikaArgs {
    /// Path to the kosha data.
    #[arg(long)]
    data_dir: PathBuf,
}

#[derive(Debug, Args)]
struct ParadigmArgs {
    /// Path to the kosha data.
    #[arg(long)]
    data_dir: PathBuf,
}

#[derive(Debug, Args)]
struct EntryArgs {
    /// Path to the kosha data.
    #[arg(long)]
    data_dir: PathBuf,
    /// Regex filter
    #[arg(long)]
    filter: Option<String>,
}

#[derive(Debug, Args)]
struct DeriveArgs {
    /// Path to the kosha data.
    #[arg(long)]
    data_dir: PathBuf,
    /// The string to derive.
    #[arg(long)]
    key: String,
}

fn create_dhatu_str(dhatu: &Dhatu) -> String {
    let mut ret = String::new();
    if !dhatu.prefixes().is_empty() {
        for (i, prefix) in dhatu.prefixes().iter().enumerate() {
            if i != 0 {
                ret.push('-');
            }
            ret.push_str(prefix);
        }
        ret.push_str(" + ");
    }

    ret.push_str(dhatu.aupadeshika().unwrap_or("___"));

    if !dhatu.sanadi().is_empty() {
        ret.push_str(" + ");
        for (i, sanadi) in dhatu.sanadi().iter().enumerate() {
            if i != 0 {
                ret.push('-');
            }
            ret.push_str(sanadi.as_str());
        }
    }

    ret
}

fn create_krdanta_entry_str(k: &KrdantaEntry) -> String {
    format!(
        "{} + {} ({:?}, {:?})",
        create_dhatu_str(k.dhatu()),
        k.krt().as_str(),
        k.prayoga(),
        k.lakara()
    )
}

fn create_pratipadika_entry_str(p: &PratipadikaEntry) -> String {
    match p {
        PratipadikaEntry::Basic(b) => {
            format!("(Basic {}, {:?})", b.pratipadika().text(), b.lingas())
        }
        PratipadikaEntry::Krdanta(k) => {
            format!("(Krdanta {})", create_krdanta_entry_str(&k))
        }
    }
}

fn print_dhatus(args: DhatuArgs) -> Result<(), Box<dyn Error>> {
    let kosha = Kosha::new(args.data_dir)?;
    for (i, d) in kosha.dhatus().enumerate() {
        println!(
            "{i:<5}: {:<20} {}",
            d.clean_text(),
            create_dhatu_str(d.dhatu())
        );
    }
    Ok(())
}

fn print_pratipadikas(args: PratipadikaArgs) -> Result<(), Box<dyn Error>> {
    let kosha = Kosha::new(args.data_dir)?;
    for (i, p) in kosha.pratipadikas().enumerate() {
        println!("{i:<7}: {}", create_pratipadika_entry_str(&p));
    }
    Ok(())
}

fn print_paradigms(args: ParadigmArgs) -> Result<(), Box<dyn Error>> {
    let kosha = Kosha::new(args.data_dir)?;
    for (i, paradigm) in kosha.paradigms().iter().enumerate() {
        println!("{i}:");
        for ending in paradigm.endings() {
            println!("- {ending:?}");
        }
    }
    Ok(())
}

fn print_entries(args: EntryArgs) -> Result<(), Box<dyn Error>> {
    let kosha = Kosha::new(args.data_dir)?;
    let dhatus: Vec<_> = kosha.dhatus().collect();
    let phits: Vec<_> = kosha.pratipadikas().collect();

    let mut stream = kosha.stream();

    let filter = match args.filter {
        Some(s) => Some(Regex::new(&s).unwrap()),
        None => None,
    };

    while let Some((raw_key, raw_value)) = stream.next() {
        let (key, version) = if !raw_key.is_empty() && raw_key[raw_key.len() - 1] < 65 {
            let i_last = raw_key.len() - 2;
            // +1 since the `else` condition is the first key.
            let version = 1 + 65 * raw_key[i_last] + raw_key[i_last + 1];
            let key = String::from_utf8(raw_key[..i_last].to_vec()).expect("ok");
            (key, version)
        } else {
            let key = String::from_utf8(raw_key.to_vec()).expect("ok");
            (key, 0)
        };

        if let Some(filter) = &filter {
            if !filter.is_match(&key) {
                continue;
            }
        }

        let value = PackedEntry::from_u32(raw_value as u32);
        let value_str = match value.pos() {
            PartOfSpeech::Subanta => {
                let s = value.as_packed_subanta();
                format!(
                    "{raw_value} (Subanta, pratipadika={}, sup={}) --- {}",
                    s.pratipadika_id(),
                    s.sup_id(),
                    create_pratipadika_entry_str(&phits[s.pratipadika_id() as usize]),
                )
            }
            PartOfSpeech::SubantaPrefix => {
                let s = value.as_packed_subanta_prefix();
                // let paradigm_id = s.paradigm_id();
                // let paradigm = &kosha.paradigms()[paradigm_id as usize];

                format!(
                    "{raw_value} (SubantaPrefix, pratipadika={}, paradigm={}) --- {}",
                    s.pratipadika_id(),
                    s.paradigm_id(),
                    create_pratipadika_entry_str(&phits[s.pratipadika_id() as usize]),
                )
            }
            PartOfSpeech::Tinanta => {
                let s = value.as_packed_tinanta();
                format!(
                    "{raw_value} (Tinanta, dhatu={}, tin={}) --- {}",
                    s.dhatu_id(),
                    s.tin_id(),
                    create_dhatu_str(&dhatus[s.dhatu_id() as usize].dhatu()),
                )
            }
            PartOfSpeech::Avyaya => {
                let a = value.as_packed_avyaya();
                format!("{raw_value} (Avyaya, {})", a.pratipadika_id())
            }
        };

        let key = format!("{}, {}", key, version);
        println!("{key:<30}: {}", value_str);
    }

    Ok(())
}

fn derive_entries(args: DeriveArgs) -> Result<(), Box<dyn Error>> {
    let kosha = Kosha::new(args.data_dir)?;

    let v = Vyakarana::new();
    for entry in kosha.get_all(&args.key) {
        let prakriyas = match entry {
            PadaEntry::Tinanta(t) => v.derive_tinantas(&t.into()),
            PadaEntry::Subanta(s) => v.derive_subantas(&s.into()),
            PadaEntry::Avyaya(a) => v.derive_subantas(&a.into()),
            _ => panic!("Unsupported"),
        };

        for p in prakriyas {
            println!("--------------------------------");
            for step in p.history() {
                let result = step
                    .result()
                    .iter()
                    .map(|x| x.text())
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>()
                    .join(" + ");
                println!("{:<10} | {}", step.rule().code(), result);
            }
        }
    }

    Ok(())
}

fn main() {
    env_logger::init();
    let args = MainArgs::parse();
    let ret = match args.command {
        Command::Dhatus(args) => print_dhatus(args),
        Command::Pratipadikas(args) => print_pratipadikas(args),
        Command::Paradigms(args) => print_paradigms(args),
        Command::Entries(args) => print_entries(args),
        Command::Derive(args) => derive_entries(args),
    };

    match ret {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
