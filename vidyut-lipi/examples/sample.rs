use vidyut_lipi::{Lipika, Scheme};

fn main() {
    let mut lipika = Lipika::new();

    for _ in 0..2_000_000 {
        let input = concat!(
            "nArAyaRaM namaskftya naraM cEva narottamam . ",
            "devIM sarasvatIM cEva tato jayamudIrayet .. 1 .."
        );
        lipika.transliterate(input, Scheme::Slp1, Scheme::Devanagari);
    }
}
