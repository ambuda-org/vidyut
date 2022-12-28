use std::error::Error;
use vidyut_prakriya::args::{Linga, SubantaArgs};
use vidyut_prakriya::Ashtadhyayi;

const fn pum(s: &'static str) -> (&'static str, Linga) {
    (s, Linga::Pum)
}
const fn stri(s: &'static str) -> (&'static str, Linga) {
    (s, Linga::Stri)
}
const fn na(s: &'static str) -> (&'static str, Linga) {
    (s, Linga::Napumsaka)
}
const fn sarva_pum(s: &'static str) -> (&'static str, Linga) {
    (s, Linga::Pum)
}

/// Test cases are derived from the data on ashtadhyayi.com, fetched from [1].
///
/// [1]: https://github.com/ashtadhyayi-com/data/blob/master/shabda/data.txt
#[allow(unused)]
const PRATIPADIKAS: &[(&str, Linga)] = &[
    pum("a"),
    pum("deva"),
    pum("rAma"),
    sarva_pum("sarva"),
    sarva_pum("viSva"),
    sarva_pum("katara"),
    sarva_pum("yatara"),
    sarva_pum("tatara"),
    sarva_pum("ekatara"),
    sarva_pum("katama"),
    sarva_pum("yatama"),
    sarva_pum("tatama"),
    sarva_pum("ekatama"),
    sarva_pum("tva"),
    sarva_pum("sama"),
    sarva_pum("sima"),
    sarva_pum("eka"),
    sarva_pum("aneka"),
    sarva_pum("uBa"),
    sarva_pum("uBaya"),
    sarva_pum("nema"),
    sarva_pum("pUrva"),
    sarva_pum("para"),
    sarva_pum("avara"),
    sarva_pum("dakziRa"),
    sarva_pum("uttara"),
    sarva_pum("apara"),
    sarva_pum("aDara"),
    sarva_pum("sva"),
    pum("sva"),
    sarva_pum("antara"),
    pum("antara"),
    pum("i"),
    pum("kavi"),
    pum("hari"),
    pum("saKi"),
    pum("susaKi"),
    pum("pati"),
    pum("BUpati"),
    pum("papI"),
    pum("yayI"),
    pum("praDI"),
    pum("u"),
    pum("SamBu"),
    pum("guru"),
    pum("hUhU"),
    pum("aticamU"),
    pum("KalapU"),
    pum("varzaBU"),
    pum("DAtf"),
    pum("pitf"),
    pum("jAmAtf"),
    pum("BrAtf"),
    pum("se"),
    pum("smfte"),
    pum("go"),
    pum("rE"),
    pum("glO"),
    pum("janO"),
    stri("mAlA"),
    stri("ramA"),
    stri("sarvA"),
    stri("mati"),
    stri("nadI"),
    stri("Denu"),
    stri("vaDU"),
    stri("go"),
    stri("dyo"),
    stri("rE"),
    stri("nO"),
    na("Pala"),
    na("puzpa"),
    na("sarva"),
    na("viSva"),
    na("katara"),
    na("yatara"),
    na("tatara"),
    na("payas"),
    pum("asmad"),
    pum("yuzmad"),
];

#[derive(Debug, Clone)]
struct ArgumentError {
    message: &'static str,
}

use std::fmt;
impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let a = Ashtadhyayi::builder().log_steps(false).build();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("test-files/subantas.tsv")?;

    let mut num_matches = 0;
    let mut n = 0;

    for maybe_row in rdr.records() {
        let r = maybe_row?;
        let pratipadika = &r[0];
        let linga = &r[1].parse()?;
        let vibhakti = &r[2].parse()?;
        let vacana = &r[3].parse()?;
        let expected = &r[4];

        let args = SubantaArgs::builder()
            .linga(*linga)
            .vibhakti(*vibhakti)
            .vacana(*vacana)
            .build()?;
        let prakriyas = a.derive_subantas(pratipadika, &args);

        let mut expected: Vec<_> = expected.split(',').collect();
        let mut actual: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        expected.sort();
        actual.sort();

        n += 1;
        if expected == actual {
            num_matches += 1;
        } else {
            println!("[ FAIL ]  {pratipadika:<10} {linga:?} {vibhakti:?} {vacana:?}");
            println!("          Expected: {:?}", expected);
            println!("          Actual  : {:?}", actual);
        }
    }

    let pct = 100_f32 * (num_matches as f32) / (n as f32);
    println!("{num_matches} / {n} tests pass ({pct:.2}%)");
    Ok(())
}

fn main() {
    run().ok();
}
