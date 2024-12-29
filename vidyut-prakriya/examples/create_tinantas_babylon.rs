//! Generates a comprehensive list of tinantas and saves them to a `.babylon` dictionary file.
//! These files are used by projects like https://github.com/indic-dict to generate offline
//! Sanskrit dictionaries.
//!
//! ### Example usage
//!
//! ```
//! # yaNantas
//! cargo run --release --example create_tinantas_babylon -- \
//! --sanadi yaN \
//! --desc "vidyud-yantreRa janitAni yaNanteByas tiNantAni" > vidyut-yaN.babylon
//!
//! # yaNlugantas
//! cargo run --release --example create_tinantas_babylon -- \
//! --sanadi yaNluk \
//! --desc "vidyud-yantreRa janitAni yaNluganteByas tiNantAni" > vidyut-yaN-luk.babylon
//! ```

use clap::Parser;
use vidyut_lipi::{Lipika, Scheme};
use vidyut_prakriya::args::*;
use vidyut_prakriya::dhatupatha::Entry as DhatuEntry;
use vidyut_prakriya::{Dhatupatha, Vyakarana};

/// Command line arguments.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Name of the *sanādi pratyaya* to use. If not set, defaults to `None`.
    #[arg(long)]
    sanadi: Option<Sanadi>,
    /// Name of the prayoga to use. If not set, defaults to kartari.
    #[arg(long)]
    prayoga: Option<Prayoga>,
    /// This file's description in SLP1. This text will be transliterated to Devanagari.
    #[arg(long)]
    desc: String,
}

/// Simple structured information for a Babylon dictionary entry.
#[derive(Debug)]
struct BabylonEntry {
    search_row: String,
    html_row: String,
}

fn to_devanagari(lipika: &mut Lipika, text: &str) -> String {
    lipika.transliterate(&text, Scheme::Slp1, Scheme::Devanagari)
}

/// Removes svarita and anudAtta from the dhatu (for easier searching)
fn replace_svaras(text: &str) -> String {
    String::from(text).replace("\\", "").replace("^", "")
}

/// Creates a `BabylonEntry` from the given initial condiitons, or `None` if such an entry could
/// not be created.
fn create_entry(
    v: &Vyakarana,
    lipika: &mut Lipika,
    dhatu_entry: &DhatuEntry,
    sanadi: Option<Sanadi>,
    prayoga: Prayoga,
    pada: DhatuPada,
    lakara: Lakara,
) -> Option<BabylonEntry> {
    use Lakara::*;

    let mut search_row = String::new();
    let mut html_row = String::new();

    let mula = match dhatu_entry.dhatu() {
        Dhatu::Mula(m) => m,
        _ => return None,
    };

    // Human-readable mula dhatu
    let prakriyas = v.derive_dhatus(dhatu_entry.dhatu());
    assert!(prakriyas.len() > 0);
    for p in prakriyas {
        let human_dhatu = p.text();
        if !search_row.is_empty() {
            search_row += "_";
        }
        search_row += &human_dhatu;

        html_row += &human_dhatu;
        html_row += " ";
    }

    // Human-readable sanadi dhatu
    let prakriyas = match sanadi {
        Some(s) => v.derive_dhatus(&dhatu_entry.dhatu().clone().with_sanadi(&[s])),
        None => v.derive_dhatus(&dhatu_entry.dhatu()),
    };
    if prakriyas.is_empty() {
        return None;
    }
    for p in prakriyas {
        let human_dhatu = p.text();
        if !search_row.is_empty() {
            search_row += "_";
        }
        search_row += &human_dhatu;

        html_row += &human_dhatu;
        html_row += " ";
    }

    // Dhatupatha sutra
    html_row += "(";
    html_row += &replace_svaras(&mula.aupadeshika());
    html_row += " ";
    html_row += dhatu_entry.artha();
    html_row += " ";

    // Pada and lakara
    html_row += match pada {
        DhatuPada::Parasmai => "parasmEpadi",
        DhatuPada::Atmane => "Atmanepadi",
    };
    html_row += "-";
    html_row += match lakara {
        Lat => "law",
        Lit => "liw",
        Lot => "low",
        Lut => "luw",
        Lrt => "lfw",
        Lan => "laN",
        VidhiLin => "viDiliN",
        AshirLin => "AzIrliN",
        Lun => "luN",
        Lrn => "lfN",
        _ => return None,
    };
    html_row += ")";
    search_row = to_devanagari(lipika, &search_row).replace("_", "|");
    html_row = to_devanagari(lipika, &html_row);

    html_row += "<br>";

    let mut has_data = false;
    for purusha in Purusha::iter() {
        for vacana in Vacana::iter() {
            let dhatu = match sanadi {
                Some(s) => dhatu_entry.dhatu().clone().with_sanadi(&[s]),
                None => dhatu_entry.dhatu().clone(),
            };
            let tinanta = Tinanta::builder()
                .dhatu(dhatu)
                .prayoga(prayoga)
                .purusha(purusha)
                .vacana(vacana)
                .pada(pada)
                .lakara(lakara)
                .build()
                .expect("ok");

            let prakriyas = v.derive_tinantas(&tinanta);
            let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
            padas.sort();
            padas.dedup();

            if !has_data {
                has_data = !padas.is_empty();
            }

            for pada in &padas {
                search_row += "|";
                search_row += &to_devanagari(lipika, &pada);
            }

            html_row += "<br>";
            let mut added_one = false;
            for pada in &padas {
                if added_one {
                    html_row += " / ";
                    html_row += &to_devanagari(lipika, &pada);
                } else {
                    html_row += &to_devanagari(lipika, &pada);
                    added_one = true;
                }
            }
        }

        // Don't print for last purusha.
        if purusha != Purusha::Uttama {
            html_row += "<br>---"
        }
    }
    if has_data {
        Some(BabylonEntry {
            search_row,
            html_row,
        })
    } else {
        None
    }
}

fn run(dp: Dhatupatha, args: Args) {
    // Disable `log_steps` so that we can generate words more quickly.
    let v = Vyakarana::builder().log_steps(false).build();
    let mut lipika = Lipika::new();

    let prayoga = match args.prayoga {
        Some(p) => p,
        None => Prayoga::Kartari,
    };

    // Header
    println!("");
    match args.sanadi {
        Some(s) => println!(
            "#bookname=vidyut-{}-{} (sa-sa)",
            s.as_str(),
            prayoga.as_str()
        ),
        None => println!("#bookname=vidyut-{} (sa-sa)", prayoga.as_str()),
    }

    let mut description = String::new();
    description += &to_devanagari(&mut lipika, &args.desc);
    description += " (https://ambuda-org.github.io/vidyullekha/)";
    println!("#description={description}\n");

    // Entries
    for dhatu_entry in dp {
        for lakara in Lakara::iter() {
            for pada in &[DhatuPada::Parasmai, DhatuPada::Atmane] {
                let entry = create_entry(
                    &v,
                    &mut lipika,
                    &dhatu_entry,
                    args.sanadi,
                    prayoga,
                    *pada,
                    lakara,
                );
                if let Some(e) = entry {
                    print!("{}\n{}\n\n", e.search_row, e.html_row);
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let dhatus = match Dhatupatha::from_path("data/dhatupatha.tsv") {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    run(dhatus, args);
}
