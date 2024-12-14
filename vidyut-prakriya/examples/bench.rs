//! Creates a large list of tinantas and krdantas.
use std::error::Error;
use vidyut_prakriya::args::{BaseKrt as Krt, *};
use vidyut_prakriya::{Dhatupatha, Vyakarana};

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

fn run(dhatupatha: Dhatupatha) -> Result<(), Box<dyn Error>> {
    let sanadi_choices = vec![
        vec![],
        vec![Sanadi::san],
        vec![Sanadi::Ric],
        vec![Sanadi::yaN],
        vec![Sanadi::yaNluk],
        vec![Sanadi::san, Sanadi::Ric],
        vec![Sanadi::Ric, Sanadi::san],
    ];

    let v = Vyakarana::builder().log_steps(false).build();
    let dhatu_sample: Vec<_> = dhatupatha
        .iter()
        .enumerate()
        .filter(|(i, _)| *i % 10 == 0)
        .map(|(_, x)| x)
        .collect();

    let mut total = 0;
    for entry in dhatu_sample {
        let dhatu = entry.dhatu();
        for sanadis in &sanadi_choices {
            let full_dhatu = dhatu.clone().with_sanadi(&sanadis);

            for prayoga in [Prayoga::Kartari, Prayoga::Karmani] {
                for lakara in Lakara::iter() {
                    for purusha in Purusha::iter() {
                        for vacana in Vacana::iter() {
                            let tinanta = Tinanta::builder()
                                .dhatu(full_dhatu.clone())
                                .prayoga(prayoga)
                                .purusha(purusha)
                                .vacana(vacana)
                                .lakara(lakara)
                                .build()?;

                            let prakriyas = v.derive_tinantas(&tinanta);
                            total += prakriyas.len();
                        }
                    }
                }
            }

            let lvv = linga_vibhakti_vacana_options();
            for krt in &[Krt::tavya, Krt::anIyar, Krt::Rvul, Krt::tfc, Krt::Satf] {
                let krdanta = Krdanta::new(full_dhatu.clone(), *krt);
                for (linga, vibhakti, vacana) in &lvv {
                    let args = Subanta::new(krdanta.clone(), *linga, *vibhakti, *vacana);
                    let prakriyas = v.derive_subantas(&args);
                    total += prakriyas.len();
                }
            }
        }
        println!("{:?}", dhatu.aupadeshika());
    }

    println!("Generated {} forms.", total);
    Ok(())
}

fn main() {
    let dhatus = match Dhatupatha::from_path("data/dhatupatha.tsv") {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    match run(dhatus) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
