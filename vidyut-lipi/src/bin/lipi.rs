use clap::Parser;
use vidyut_lipi::{Lipika, Scheme};

#[derive(clap::ValueEnum, Clone, Debug)]
enum ClapScheme {
    Balinese,
    Bengali,
    Brahmi,
    Burmese,
    Devanagari,
    Gujarati,
    Grantha,
    Gurmukhi,
    Javanese,
    Kannada,
    Malayalam,
    Odia,
    Sharada,
    Siddham,
    Sinhala,
    Tamil,
    Telugu,
    BarahaSouth,

    HarvardKyoto,
    Iast,
    Iso15919,
    Itrans,
    Slp1,
    Velthuis,
    Wx,
}

impl From<ClapScheme> for Scheme {
    fn from(x: ClapScheme) -> Scheme {
        use ClapScheme as C;
        use Scheme::*;
        match x {
            C::Balinese => Balinese,
            C::Bengali => Bengali,
            C::Brahmi => Brahmi,
            C::Burmese => Burmese,
            C::Devanagari => Devanagari,
            C::Gujarati => Gujarati,
            C::Grantha => Grantha,
            C::Gurmukhi => Gurmukhi,
            C::Javanese => Javanese,
            C::Kannada => Kannada,
            C::Malayalam => Malayalam,
            C::Odia => Odia,
            C::Sharada => Sharada,
            C::Siddham => Siddham,
            C::Sinhala => Sinhala,
            C::Tamil => Tamil,
            C::Telugu => Telugu,
            C::BarahaSouth => BarahaSouth,

            C::HarvardKyoto => HarvardKyoto,
            C::Iast => Iast,
            C::Iso15919 => Iso15919,
            C::Itrans => Itrans,
            C::Slp1 => Slp1,
            C::Velthuis => Velthuis,
            C::Wx => Wx,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The scheme to transliterate from.
    #[arg(short, long)]
    from: ClapScheme,

    /// The scheme to transliterate to.
    #[arg(short, long)]
    to: ClapScheme,

    /// The text to transliterate.
    text: String,
}

fn run(args: Args) {
    let mut lipika = Lipika::new();
    let result = lipika.transliterate(args.text, args.from.into(), args.to.into());
    println!("{result}");
}

fn main() {
    let args = Args::parse();
    run(args)
}
