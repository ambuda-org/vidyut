//! Creates all test files for our snapshot tests.
use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::path::{Path, PathBuf};
use vidyut_prakriya::args::{
    BaseKrt, Dhatu, Gana, Krdanta, Lakara, Linga, Muladhatu, Prayoga, Purusha, Sanadi, Subanta,
    Tinanta, Vacana, Vibhakti,
};
use vidyut_prakriya::{Dhatupatha, Vyakarana};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    output_dir: PathBuf,
}

#[derive(Debug, Serialize)]
struct TinantaRow<'a> {
    padas: String,
    dhatu: &'a str,
    gana: Gana,
    number: u16,
    sanadi: String,
    prayoga: Prayoga,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
}

#[derive(Debug, Serialize)]
struct KrdantaRow<'a> {
    padas: String,
    dhatu: &'a str,
    gana: Gana,
    number: u16,
    sanadi: &'a str,
    krt: BaseKrt,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

#[derive(Debug, Serialize)]
struct DhatuRow<'a> {
    results: String,
    dhatu: &'a str,
    gana: Gana,
    number: u16,
    sanadi: String,
}

fn to_mula(dhatu: &Dhatu) -> &Muladhatu {
    match dhatu {
        Dhatu::Mula(m) => m,
        _ => panic!("unknown dhatu"),
    }
}

/// Creates a collection of (lakara, purusha, vacana) combinations.
fn prayoga_lakara_purusha_vacana() -> Vec<(Prayoga, Lakara, Purusha, Vacana)> {
    let mut ret = Vec::new();
    for prayoga in Prayoga::iter() {
        for lakara in Lakara::iter() {
            if lakara == Lakara::Let {
                continue;
            }
            for purusha in Purusha::iter() {
                for vacana in Vacana::iter() {
                    ret.push((prayoga, lakara, purusha, vacana))
                }
            }
        }
    }
    ret
}

/// Creates a collection of (linga, vibhakti, vacana) combinations.
fn linga_vibhakti_vacana_options() -> Vec<(Linga, Vibhakti, Vacana)> {
    let mut ret = Vec::new();
    for linga in Linga::iter() {
        for vibhakti in Vibhakti::iter() {
            for vacana in Vacana::iter() {
                ret.push((linga, vibhakti, vacana))
            }
        }
    }
    ret
}

fn create_tinantas_file(
    dhatupatha: &Dhatupatha,
    sanadi: &[Sanadi],
    output_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(output_file)?;
    let v = Vyakarana::builder().log_steps(false).build();
    let plvp = prayoga_lakara_purusha_vacana();

    for entry in dhatupatha {
        let dhatu = to_mula(entry.dhatu());

        // Add sanadi to the dhatu.
        let mut builder = Dhatu::builder()
            .aupadeshika(dhatu.aupadeshika())
            .prefixes(dhatu.prefixes())
            .gana(dhatu.gana())
            .sanadi(&sanadi);
        if let Some(x) = dhatu.antargana() {
            builder = builder.antargana(x);
        }
        let dhatu = builder.build()?;
        let mula = to_mula(&dhatu);

        for (prayoga, lakara, purusha, vacana) in plvp.iter().copied() {
            let tinanta = Tinanta::builder()
                .dhatu(dhatu.clone())
                .prayoga(prayoga)
                .purusha(purusha)
                .vacana(vacana)
                .lakara(lakara)
                .build()?;

            let prakriyas = v.derive_tinantas(&tinanta);
            let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
            padas.sort();
            padas.dedup();
            let padas = padas.join("|");
            if padas.is_empty() {
                continue;
            }

            let sanadi_str = dhatu
                .sanadi()
                .iter()
                .map(|x| x.as_str())
                .fold(String::new(), |b, x| b + x + "+");
            let sanadi_str = sanadi_str.trim_end_matches('+');

            let dhatu_text = mula.aupadeshika();
            let row = TinantaRow {
                padas,
                dhatu: dhatu_text,
                gana: mula.gana(),
                number: entry.number(),
                sanadi: sanadi_str.to_string(),
                lakara,
                purusha,
                vacana,
                prayoga,
            };

            wtr.serialize(row)?;
        }
    }

    wtr.flush()?;
    Ok(())
}

fn create_krdantas_file(
    dhatupatha: &Dhatupatha,
    sanadi: &[Sanadi],
    output_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(output_file)?;
    let v = Vyakarana::builder().log_steps(false).build();
    let lvv = linga_vibhakti_vacana_options();

    for entry in dhatupatha {
        let dhatu = entry.dhatu().clone().with_sanadi(sanadi);
        let dhatu_text = &dhatu.aupadeshika().expect("mula");
        let sanadi_str = sanadi
            .iter()
            .map(|x| x.as_str())
            .fold(String::new(), |b, x| b + x + "+");
        let sanadi_str = sanadi_str.trim_end_matches('+');

        for krt in BaseKrt::iter() {
            let krdanta = Krdanta::builder().dhatu(dhatu.clone()).krt(krt).build()?;

            for (linga, vibhakti, vacana) in lvv.iter().copied() {
                let args = Subanta::new(krdanta.clone(), linga, vibhakti, vacana);
                let prakriyas = v.derive_subantas(&args);

                let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                padas.sort();
                padas.dedup();
                let padas = padas.join("|");

                let row = KrdantaRow {
                    padas,
                    dhatu: dhatu_text,
                    gana: dhatu.gana().expect("ok"),
                    number: entry.number(),
                    sanadi: &sanadi_str,
                    krt,
                    linga,
                    vibhakti,
                    vacana,
                };
                wtr.serialize(row)?;
            }
        }
    }

    wtr.flush()?;
    Ok(())
}

fn create_dhatus_file(dhatupatha: &Dhatupatha, output_file: &Path) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(output_file)?;
    let v = Vyakarana::builder().log_steps(false).build();

    let sanadi: Vec<Vec<Sanadi>> = vec![
        vec![],
        vec![Sanadi::san],
        vec![Sanadi::Ric],
        vec![Sanadi::yaN],
        vec![Sanadi::yaNluk],
        vec![Sanadi::san, Sanadi::Ric],
        vec![Sanadi::Ric, Sanadi::san],
    ];

    for s in sanadi {
        for entry in dhatupatha {
            let dhatu = entry.dhatu().clone().with_sanadi(&s);

            let prakriyas = v.derive_dhatus(&dhatu);
            let dhatu_text = &dhatu.aupadeshika().expect("ok");
            let mut results: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
            results.sort();
            results.dedup();
            let results = results.join("|");

            let sanadi_str = dhatu
                .sanadi()
                .iter()
                .map(|x| x.as_str())
                .fold(String::new(), |b, x| b + x + "+");
            let sanadi_str = sanadi_str.trim_end_matches('+');

            let row = DhatuRow {
                results,
                dhatu: dhatu_text,
                gana: dhatu.gana().expect("ok"),
                number: entry.number(),
                sanadi: sanadi_str.to_string(),
            };
            wtr.serialize(row)?;
        }
    }

    Ok(())
}

fn run(dhatupatha: Dhatupatha, args: Args) -> Result<(), Box<dyn Error>> {
    use Sanadi::*;

    let dir = args.output_dir;

    let sanadis = &[
        (vec![], "basic"),
        (vec![Ric], "nic"),
        (vec![san], "san"),
        (vec![yaN], "yan"),
        (vec![yaNluk], "yan-luk"),
        (vec![san, Ric], "san-nic"),
        (vec![Ric, san], "nic-san"),
    ];

    for (sanadis, s_name) in sanadis {
        let filename = format!("tinantas-{s_name}.csv");
        println!("Creating {filename}.");
        create_tinantas_file(&dhatupatha, &sanadis, &dir.join(filename))?;
    }

    for (sanadis, s_name) in sanadis {
        let filename = format!("krdantas-{s_name}.csv");
        println!("Creating {filename}.");
        create_krdantas_file(&dhatupatha, &sanadis, &dir.join(filename))?;
    }

    let filename = "dhatus.csv";
    println!("Creating {filename}.");
    create_dhatus_file(&dhatupatha, &dir.join(filename))?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    let dhatupatha = match Dhatupatha::from_path("data/dhatupatha.tsv") {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    match run(dhatupatha, args) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
