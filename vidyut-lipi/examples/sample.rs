use vidyut_lipi::{Lipika, Scheme};

fn main() {
    let mut input = String::new();
    for _ in 0..1_000_000 {
        input.push_str(concat!(
            "nArAyaRaM namaskftya naraM cEva narottamam . ",
            "devIM sarasvatIM cEva tato jayamudIrayet .. 1 .."
        ));
    }

    let mut lipika = Lipika::new();
    let output = lipika.transliterate(input, Scheme::Slp1, Scheme::Tibetan);
    println!("{output}");
}
