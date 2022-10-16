use crate::context::Context;
use crate::semantics::*;
use multimap::MultiMap;

pub type StemMap = MultiMap<String, StemSemantics>;
pub type PadaMap = MultiMap<String, Semantics>;
pub type EndingMap = MultiMap<String, (String, Semantics)>;

fn add_stem_semantics(text: &str, ctx: &Context, all_semantics: &mut Vec<Semantics>) {
    let stems = &ctx.stem_map;

    for ending in ctx.ending_map.keys() {
        for (stem_type, semantics) in ctx.ending_map.get_vec(ending).unwrap() {
            let len_text = text.len();
            if !text.ends_with(ending) {
                continue;
            }

            let mut stem = String::new();
            let len_ending = ending.len();
            stem += &text[0..(len_text - len_ending)];
            stem += stem_type;

            if let Some(stem_semantics) = stems.get(&stem) {
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

pub fn analyze(text: &str, data: &Context) -> Vec<Semantics> {
    let mut all_semantics = Vec::new();

    if data.pada_map.contains_key(text) {
        all_semantics.push(data.pada_map.get(text).unwrap().clone());
    }
    add_stem_semantics(text, data, &mut all_semantics);

    // As a default option, mark this as "none"
    all_semantics.push(Semantics::None);

    all_semantics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandhi;

    fn toy_data() -> Context {
        let mut pada_map = PadaMap::new();
        pada_map.insert(
            String::from("Bavati"),
            Semantics::Tinanta(Tinanta {
                root: "BU".to_string(),
                purusha: Purusha::Prathama,
                vacana: Vacana::Eka,
                lakara: Lakara::Lat,
                pada: VerbPada::Parasmaipada,
            }),
        );

        let mut stem_map = StemMap::new();
        stem_map.insert(
            String::from("nara"),
            StemSemantics::Basic {
                lingas: vec![Linga::Pum],
            },
        );
        stem_map.insert(
            String::from("gacCat"),
            StemSemantics::Basic {
                lingas: vec![Linga::Pum, Linga::Stri, Linga::Napumsaka],
            },
        );

        let mut ending_map = EndingMap::new();
        ending_map.insert(
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
        ending_map.insert(
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

        Context {
            sandhi_rules: sandhi::SandhiMap::new(),
            pada_map,
            stem_map,
            ending_map,
        }
    }

    #[test]
    fn analyze_verb() {
        let ctx = toy_data();
        assert_eq!(
            *analyze("Bavati", &ctx).first().unwrap(),
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
        let ctx = toy_data();
        assert_eq!(
            *analyze("narasya", &ctx).first().unwrap(),
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
        let ctx = toy_data();
        assert_eq!(
            *analyze("gacCantIm", &ctx).first().unwrap(),
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
