/*!
Creates a test file containing the inputs to `Ashtadhyayi`'s derivation functions and all of the
padas produced by those inputs.
*/
use serde::Serialize;
use std::error::Error;
use std::io;
use vidyut_prakriya::args::{Linga, Pratipadika, SubantaArgs, Vacana, Vibhakti};
use vidyut_prakriya::Ashtadhyayi;

struct TestCase {
    pratipadika: Pratipadika,
    linga: Linga,
}

fn dhatu_pratipadika(s: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(s)
        .is_dhatu(true)
        .build()
        .unwrap()
}

fn pum(s: &str) -> TestCase {
    TestCase {
        pratipadika: Pratipadika::new(s),
        linga: Linga::Pum,
    }
}

fn pum_dhatu(s: &str) -> TestCase {
    TestCase {
        pratipadika: dhatu_pratipadika(s),
        linga: Linga::Pum,
    }
}

fn stri(s: &str) -> TestCase {
    TestCase {
        pratipadika: Pratipadika::builder()
            .text(s)
            .is_nyap(s.ends_with('A') || s.ends_with('I'))
            .build()
            .unwrap(),
        linga: Linga::Stri,
    }
}

#[allow(unused)]
fn stri_dhatu(s: &str) -> TestCase {
    TestCase {
        pratipadika: Pratipadika::builder()
            .text(s)
            .is_dhatu(true)
            .build()
            .unwrap(),
        linga: Linga::Stri,
    }
}

fn stri_no_nyap(s: &str) -> TestCase {
    TestCase {
        pratipadika: Pratipadika::new(s),
        linga: Linga::Stri,
    }
}

fn na(s: &str) -> TestCase {
    TestCase {
        pratipadika: Pratipadika::new(s),
        linga: Linga::Napumsaka,
    }
}

/// Sourced from <https://ashtadhyayi.com/shabda>.
fn create_test_cases() -> Vec<TestCase> {
    vec![
        // ------------------------------
        // Ajantas (pum, stri, napumsaka)
        // ------------------------------
        pum("a"),
        pum("deva"),
        pum("rAma"),
        pum("sarva"),
        pum("viSva"),
        // TODO: viSva when not a sarvanama
        pum("katara"),
        pum("yatara"),
        pum("tatara"),
        pum("ekatara"),
        pum("katama"),
        pum("yatama"),
        pum("tatama"),
        pum("ekatama"),
        pum("anya"),
        pum("anyatara"),
        pum("anyatama"),
        pum("itara"),
        pum("sima"),
        pum("eka"),
        pum("pUrva"),
        pum("para"),
        pum("avara"),
        pum("dakziRa"),
        pum("uttara"),
        pum("apara"),
        pum("aDara"),
        pum("praTama"),
        pum("carama"),
        pum("alpa"),
        pum("arDa"),
        pum("katipaya"),
        pum_dhatu("viSvapA"),
        pum("hAhA"),
        pum("i"),
        pum("kavi"),
        pum("hari"),
        pum("BUpati"),
        pum_dhatu("nI"),
        pum_dhatu("SuzkI"),
        pum_dhatu("pakvI"),
        pum("u"),
        pum("SamBu"),
        pum("guru"),
        pum_dhatu("KalapU"),
        pum("hUhU"),
        pum("pitf"),
        pum("jAmAtf"),
        pum("BrAtf"),
        pum("nf"),
        pum("se"),
        pum("smfte"),
        pum("go"),
        pum("sudyo"),
        pum("smfto"),
        pum("rE"),
        pum("glO"),
        pum("janO"),
        stri("mAlA"),
        stri("ramA"),
        stri("ambA"),
        stri("mati"),
        stri("rAtri"),
        stri("nadI"),
        stri("gOrI"),
        stri_no_nyap("avI"),
        stri_no_nyap("tantrI"),
        stri_no_nyap("tarI"),
        stri_no_nyap("starI"),
        stri_no_nyap("lakzmI"),
        stri("Denu"),
        stri("Karu"),
        stri("vaDU"),
        stri("SvaSrU"),
        stri("svasf"),
        stri("mAtf"),
        stri("nAnAndf"),
        stri("duhitf"),
        stri("duhitf"),
        stri("yAtf"),
        stri("go"),
        stri("dyo"),
        stri("rE"),
        stri("nO"),
        na("Pala"),
        na("puzpa"),
        na("sarva"),
        na("pUrva"),
        na("para"),
        na("avara"),
        na("dakziRa"),
        na("uttara"),
        na("apara"),
        na("aDara"),
        na("praTama"),
        na("carama"),
        na("vAri"),
        na("Suci"),
        na("maDu"),
        na("vasu"),
        na("ambu"),
        na("aSru"),
        na("vastu"),
        na("pIlu"),
        na("susmfte"),
        na("prarE"),
        na("pradyo"),
        na("sunO"),
        // -------------------------------
        // Halantas (pum, stri, napumsaka)
        // -------------------------------
        // Ordered alphabetically by last consonant (in shiva sutra order)
        pum("lih"),
        pum("kamal"),
        pum("rAjan"),
        pum("yajvan"),
        pum("aryaman"),
        pum("brahman"),
        pum("SArNgin"),
        pum("Atman"),
        pum("guRin"),
        pum("Svan"),
        pum("yaSasvin"),
        pum("pUzan"),
        pum("yuvan"),
        pum("gup"),
        pum("veDas"),
        pum("uSanas"),
        pum("anehas"),
        pum_dhatu("suvas"),
        pum("adas"),
        stri("vAc"),
        stri("yozit"),
        stri("Apad"),
        stri("kzuD"),
        stri("sIman"),
        stri("kakuB"),
        na("brahman"),
        na("nAman"),
        na("daRqin"),
        na("sragvin"),
        na("manohArin"),
        na("varman"),
        na("jagat"),
        na("payas"),
    ]
}

#[derive(Debug, Serialize)]
struct Row<'a> {
    padas: String,
    linga: &'static str,
    vibhakti: &'static str,
    vacana: &'static str,
    pratipadika: &'a str,
    is_nyap: bool,
    is_dhatu: bool,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let a = Ashtadhyayi::builder().log_steps(false).build();

    for test_case in create_test_cases() {
        let pratipadika = test_case.pratipadika;
        let linga = test_case.linga;

        for vibhakti in Vibhakti::iter() {
            for vacana in Vacana::iter() {
                let subanta_args = SubantaArgs::builder()
                    .linga(linga)
                    .vibhakti(*vibhakti)
                    .vacana(*vacana)
                    .build()?;

                let prakriyas = a.derive_subantas(&pratipadika, &subanta_args);
                let mut padas: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
                padas.sort();
                padas.dedup();
                let padas = padas.join("|");
                if padas.is_empty() {
                    continue;
                }

                let row = Row {
                    padas,
                    linga: linga.as_str(),
                    vibhakti: vibhakti.as_str(),
                    vacana: vacana.as_str(),
                    pratipadika: pratipadika.text(),
                    is_nyap: pratipadika.is_nyap(),
                    is_dhatu: pratipadika.is_dhatu(),
                };

                wtr.serialize(row)?;
            }
        }
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
