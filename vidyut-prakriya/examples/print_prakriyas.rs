//! Prints some basic prakriyas. Usage:
//!
//!     cargo run --example print_prakriyas
//!
//! In debug mode, prakriyas will also include debug information. To exclude this debug
//! information, build with `--release` instead:
//!
//!     cargo run --release --example print_prakriyas
use vidyut_prakriya::args::*;
use vidyut_prakriya::{Prakriya, Vyakarana};

/// Prints the `prakriyas` provided.
fn print_prakriyas(prakriyas: &[Prakriya]) {
    for p in prakriyas {
        // `p.text()` contains the final output.
        println!("{}", p.text());
        println!("---------------------------");
        // `p.history()` contains each rule that was applied as well as the rule's results.
        for step in p.history() {
            // `step.rule().code()` is a string that identifies the rule (e.g. "1.3.1").
            let code = step.rule().code();
            // `step.result()` contains all of the *terms* that are part of this step. These
            // include dhatus, agamas, pratyayas, etc.
            //
            // Here, we create a single string to show all of the results from each term.
            let terms: Vec<_> = step
                .result()
                .iter()
                .map(|x| x.text())
                .filter(|x| !x.is_empty())
                .collect();
            let result = terms.join(" + ");
            println!("{:<10} | {}", code, result);
        }
        println!("---------------------------");
        println!("\n");
    }
}

fn main() {
    // For extra options, see `Vyakarana::builder()`.
    let v = Vyakarana::new();

    // Create a basic dhatu with `Dhatu::mula`
    //
    // For supported dhatus, see `dhatupatha.tsv`.
    let bhu = Dhatu::mula(Slp1String::from("BU").expect("ok"), Gana::Bhvadi);

    let args = Tinanta::builder()
        .dhatu(bhu)
        .lakara(Lakara::Lat)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .build()
        .unwrap();
    let prakriyas = v.derive_tinantas(&args);
    print_prakriyas(&prakriyas);

    // Create a sannanta dhatu with `with_sanadi`.
    let jijnasa = Dhatu::mula(Slp1String::from("jYA\\").expect("ok"), Gana::Kryadi)
        .with_sanadi(&[Sanadi::san]);

    let args = Tinanta::builder()
        .dhatu(jijnasa)
        .lakara(Lakara::Lat)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .build()
        .unwrap();
    let prakriyas = v.derive_tinantas(&args);
    print_prakriyas(&prakriyas);
}
