//! Maps Sanskrit words and stems to their semantics.

use crate::semantics::*;
use multimap::MultiMap;
use serde::{Deserialize, Serialize};

pub type StemMap = MultiMap<String, StemSemantics>;
pub type PadaMap = MultiMap<String, Semantics>;
pub type EndingMap = MultiMap<String, (String, Semantics)>;

#[derive(Serialize, Deserialize)]
pub struct Lexicon {
    pub stems: StemMap,
    pub padas: PadaMap,
    pub endings: EndingMap,
}

impl Lexicon {
    pub fn find(&self, text: &str) -> Vec<Semantics> {
        let mut all_semantics = Vec::new();

        if self.padas.contains_key(text) {
            all_semantics.push(self.padas.get(text).unwrap().clone());
        }
        add_stem_semantics(self, text, &mut all_semantics);

        // As a default option, mark this as "none"
        all_semantics.push(Semantics::None);
        all_semantics
    }
}

fn add_stem_semantics(lex: &Lexicon, text: &str, all_semantics: &mut Vec<Semantics>) {
    for (ending, pairs) in lex.endings.iter_all() {
        if !text.ends_with(ending) {
            continue;
        }

        for (stem_type, semantics) in pairs {
            let mut stem = String::new();
            let len_text = text.len();
            let len_ending = ending.len();
            stem += &text[0..(len_text - len_ending)];
            stem += stem_type;

            if let Some(stem_semantics) = lex.stems.get(&stem) {
                let root = match &stem_semantics {
                    StemSemantics::Krdanta {
                        root,
                        tense: _,
                        prayoga: _,
                    } => Some(root),
                    &_ => None,
                };

                let mut word_semantics = semantics.clone();
                match word_semantics {
                    Semantics::Subanta(ref mut s) => {
                        s.stem = stem;
                        all_semantics.push(word_semantics);
                    }
                    Semantics::KrtSubanta(ref mut s) => {
                        s.root = root.unwrap_or(&stem).to_string();
                        all_semantics.push(word_semantics);
                    }
                    _ => (),
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
            Semantics::Tinanta(Tinanta {
                root: "BU".to_string(),
                purusha: Purusha::Prathama,
                vacana: Vacana::Eka,
                lakara: Lakara::Lat,
                pada: VerbPada::Parasmaipada,
            }),
        );

        let mut stems = StemMap::new();
        stems.insert(
            String::from("nara"),
            StemSemantics::Basic {
                lingas: vec![Linga::Pum],
            },
        );
        stems.insert(
            String::from("gacCat"),
            StemSemantics::Basic {
                lingas: vec![Linga::Pum, Linga::Stri, Linga::Napumsaka],
            },
        );

        let mut endings = EndingMap::new();
        endings.insert(
            String::from("asya"),
            (
                String::from("a"),
                Semantics::Subanta(Subanta {
                    stem: "".to_string(),
                    linga: Linga::Pum,
                    vacana: Vacana::Eka,
                    vibhakti: Vibhakti::V6,
                    is_compounded: false,
                }),
            ),
        );
        endings.insert(
            String::from("antIm"),
            (
                String::from("at"),
                Semantics::Subanta(Subanta {
                    stem: "".to_string(),
                    linga: Linga::Stri,
                    vacana: Vacana::Eka,
                    vibhakti: Vibhakti::V2,
                    is_compounded: false,
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
            Semantics::Tinanta(Tinanta {
                root: "BU".to_string(),
                purusha: Purusha::Prathama,
                vacana: Vacana::Eka,
                lakara: Lakara::Lat,
                pada: VerbPada::Parasmaipada,
            })
        );
    }

    #[test]
    fn analyze_inflected_nominal() {
        let lex = toy_lexicon();
        assert_eq!(
            *lex.find("narasya").first().unwrap(),
            Semantics::Subanta(Subanta {
                stem: "nara".to_string(),
                linga: Linga::Pum,
                vacana: Vacana::Eka,
                vibhakti: Vibhakti::V6,
                is_compounded: false,
            })
        );
    }

    #[test]
    fn analyze_inflected_krdanta() {
        let lex = toy_lexicon();
        assert_eq!(
            *lex.find("gacCantIm").first().unwrap(),
            Semantics::Subanta(Subanta {
                stem: "gacCat".to_string(),
                linga: Linga::Stri,
                vacana: Vacana::Eka,
                vibhakti: Vibhakti::V2,
                is_compounded: false,
            })
        );
    }
}
