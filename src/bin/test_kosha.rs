//! Test the production setup.

use clap::Parser;
use std::path::PathBuf;
use vidyut_cheda::Result;
use vidyut_kosha::morph::Pada;
use vidyut_kosha::Kosha;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // The base directory from which we read our model.
    #[arg(long)]
    data_dir: PathBuf,
}

fn test_tinantas(k: &Kosha) -> Result<()> {
    let keys = vec![
        // Basic lakaras (kartari)
        "Bavati",
        "baBUva",
        "BavitA",
        "Bavizyati",
        "Bavatu",
        "aBavat",
        "BUyAt",
        "Bavet",
        "aBUt",
        "aBavizyat",
        // Basic lakaras (karmani)
        "BUyate",
        "baBUve",
        "BavitA",
        "BAvitA",
        "Bavizyate",
        "BAvizyate",
        "BavyatAm",
        "aBUyata",
        "BUyeta",
        "BavizIzwa",
        "BAvizIzwa",
        "aBAvi",
        "aBavizyata",
        "aBAvizyata",
        // sannanta (kartari)
        "buBUzati",
        "buBUzAmbaBUva",
        "buBUzAYcakAra",
        "buBUzAmAsa",
        "buBUzitA",
        "buBUzizyati",
        "buBUzatu",
        "abuBUzat",
        "buBUzet",
        "buBUzyAt",
        "abuBUzIt",
        "abuBUzizyat",
        // Nijanta (kartari)
        "BAvayati",
        "BAvayAmbaBUva",
        "BAvayAYcakAra",
        "BAvayAmAsa",
        "BAvayitA",
        "BAvayizyati",
        "BAvayatu",
        "aBAvayat",
        "BAvayet",
        "BAvyAt",
        "abIBavat",
        "aBAvayizyat",
        // yaGanta (kartari)
        "boBUyate",
        "boBUyAmbaBUva",
        "boBUyAYcakre",
        "boBUyAmAsa",
        "boBUyitA",
        "boBUyizyate",
        "boBUyatAm",
        "aboBUyata",
        "boBUyeta",
        "boBUyizIzwa",
        "aboBUyizwa",
        "aboBUyizyata",
        // Prefixes
        "aBiBavati",
        "praBavati",
        // Other tricky tinantas
        "saMskaroti",
        "saYcaskAra",
        "saYcaskrire",
    ];

    let mut i = 0;
    for key in &keys {
        let ok = k.contains_key(key);
        if ok {
            i += 1;
        } else {
            println!("FAILED: key {key} is missing");
        }
    }
    let n = keys.len();
    println!("{i} / {n} tinanta tests passed.");

    Ok(())
}

fn test_krdantas(k: &Kosha) -> Result<()> {
    let keys = vec![
        // kta, ktavat
        "BUtaH",
        "BUtam",
        "BUtA",
        "BUtavAn",
        "BUtavat",
        "BUtavatI",
        // Satf
        "Bavan",
        "BavantaH",
        "BavantI",
        "Bavizyan",
        "BavizyantaH",
        "BavizyantI",
        // krtya
        "Bavyam",
        "Bavitavyam",
        "BavanIyam",
        // Other
        "BAvakaH",
        "Bavanam",
        // With prefixes
        "aBiBUtam",
        "praBUtam",
        "saMskftam",
    ];

    let mut i = 0;
    for key in &keys {
        let ok = k.contains_key(key);
        if ok {
            i += 1;
        } else {
            println!("FAILED: key {key} is missing");
        }
    }
    let n = keys.len();
    println!("{i} / {n} tinanta tests passed.");

    Ok(())
}

fn test_subantas(k: &Kosha) -> Result<()> {
    let keys = vec![
        ("devas", "deva"),
        ("senA", "senA"),
        ("agnis", "agni"),
        ("devI", "devI"),
        ("gurus", "guru"),
        ("vaDUs", "vaDU"),
        ("kartA", "kartf"),
        ("rAs", "rE"),
        // ("dyOs", "div"),
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
        ("trayas", "tri"),
        ("zaRRAm", "zaz"),
        ("sapta", "saptan"),
        ("daSa", "daSan"),
        ("pitaras", "pitf"),
        ("mAtaras", "mAtf"),
        ("BrAtaras", "BrAtf"),
        ("panTAnam", "paTin"),
        ("patyus", "pati"),
        ("yUnAm", "yuvan"),
    ];

    let mut i = 0;
    for (key, lemma) in &keys {
        let present = k.contains_key(key);
        let entries: std::result::Result<Vec<Pada>, _> =
            k.get_all(key).iter().map(|x| k.unpack(x)).collect();
        let entries = entries?;
        let has_lemma = entries.iter().any(|x| &x.lemma() == lemma);

        if present && has_lemma {
            i += 1;
        } else {
            println!("FAILED: key {key} is missing (present={present}, has_lemma={has_lemma})");
        }
    }
    let n = keys.len();
    println!("{i} / {n} tinanta tests passed.");

    Ok(())
}

fn run_tests(args: Args) -> Result<()> {
    let kosha = Kosha::new(args.data_dir)?;
    test_tinantas(&kosha)?;
    test_krdantas(&kosha)?;
    test_subantas(&kosha)?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run_tests(args) {
        println!("Test runner failed with: `{}`", e);
        std::process::exit(1);
    }
}
