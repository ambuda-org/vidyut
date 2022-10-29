//! Maps Sanskrit words and stems to their semantics.

use crate::semantics::*;
use multimap::MultiMap;

pub type StemMap = MultiMap<String, Pratipadika>;
pub type PadaMap = MultiMap<String, Pada>;
pub type EndingMap = MultiMap<String, (String, Pada)>;

pub struct Lexicon {
    pub stems: StemMap,
    pub padas: PadaMap,
    pub endings: EndingMap,
}

impl Lexicon {
    pub fn find(&self, text: &str) -> Vec<Pada> {
        let mut all_semantics = Vec::new();

        if self.padas.contains_key(text) {
            all_semantics.push(self.padas.get(text).unwrap().clone());
        }
        add_stem_semantics(self, text, &mut all_semantics);
        all_semantics
    }
}

fn add_stem_semantics(lex: &Lexicon, text: &str, all_semantics: &mut Vec<Pada>) {
    for (ending, pairs) in lex.endings.iter_all() {
        if !text.ends_with(ending) {
            continue;
        }

        for (stem_type, ending_semantics) in pairs {
            let mut stem_text = String::new();
            stem_text += &text[0..(text.len() - ending.len())];
            stem_text += stem_type;

            if let Some(stem) = lex.stems.get(&stem_text) {
                if let Pada::Subanta(s) = ending_semantics {
                    let mut s = s.clone();
                    s.pratipadika = stem.clone();
                    all_semantics.push(Pada::Subanta(s));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_lexicon() -> Lexicon {
        let mut padas = PadaMap::new();
        padas.insert(
            String::from("Bavati"),
            Pada::Tinanta(Tinanta {
                dhatu: Dhatu("BU".to_string()),
                purusha: Purusha::Prathama,
                vacana: Vacana::Eka,
                lakara: Lakara::Lat,
                pada: PadaPrayoga::Parasmaipada,
            }),
        );

        let mut stems = StemMap::new();
        stems.insert(
            String::from("nara"),
            Pratipadika::Basic {
                text: "nara".to_string(),
                lingas: vec![Linga::Pum],
            },
        );
        stems.insert(
            String::from("gacCat"),
            Pratipadika::Krdanta {
                dhatu: Dhatu("gam".to_string()),
                pratyaya: KrtPratyaya::Shatr,
            },
        );

        let mut endings = EndingMap::new();
        endings.insert(
            String::from("asya"),
            (
                String::from("a"),
                Pada::Subanta(Subanta {
                    pratipadika: Pratipadika::Basic {
                        text: "a".to_string(),
                        lingas: vec![Linga::Pum],
                    },
                    linga: Linga::Pum,
                    vacana: Vacana::Eka,
                    vibhakti: Vibhakti::V6,
                    is_purvapada: false,
                }),
            ),
        );
        endings.insert(
            String::from("antIm"),
            (
                String::from("at"),
                Pada::Subanta(Subanta {
                    pratipadika: Pratipadika::Basic {
                        text: "at".to_string(),
                        lingas: vec![Linga::Pum, Linga::Stri, Linga::Napumsaka],
                    },
                    linga: Linga::Stri,
                    vacana: Vacana::Eka,
                    vibhakti: Vibhakti::V2,
                    is_purvapada: false,
                }),
            ),
        );

        Lexicon {
            padas,
            stems,
            endings,
        }
    }

    #[test]
    fn analyze_verb() {
        let lex = toy_lexicon();
        assert_eq!(
            *lex.find("Bavati").first().unwrap(),
            Pada::Tinanta(Tinanta {
                dhatu: Dhatu("BU".to_string()),
                purusha: Purusha::Prathama,
                vacana: Vacana::Eka,
                lakara: Lakara::Lat,
                pada: PadaPrayoga::Parasmaipada,
            })
        );
    }

    #[test]
    fn analyze_inflected_nominal() {
        let lex = toy_lexicon();
        assert_eq!(
            *lex.find("narasya").first().unwrap(),
            Pada::Subanta(Subanta {
                pratipadika: Pratipadika::Basic {
                    text: "nara".to_string(),
                    lingas: vec![Linga::Pum,]
                },
                linga: Linga::Pum,
                vacana: Vacana::Eka,
                vibhakti: Vibhakti::V6,
                is_purvapada: false,
            })
        );
    }

    #[test]
    fn analyze_inflected_krdanta() {
        let lex = toy_lexicon();
        assert_eq!(
            *lex.find("gacCantIm").first().unwrap(),
            Pada::Subanta(Subanta {
                pratipadika: Pratipadika::Krdanta {
                    dhatu: Dhatu("gam".to_string()),
                    pratyaya: KrtPratyaya::Shatr,
                },
                linga: Linga::Stri,
                vacana: Vacana::Eka,
                vibhakti: Vibhakti::V2,
                is_purvapada: false,
            })
        );
    }
}
