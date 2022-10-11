use crate::io;
use crate::semantics::*;
use multimap::MultiMap;

pub type StemMap = MultiMap<String, StemSemantics>;
pub type PadaMap = MultiMap<String, Semantics>;
pub type EndingMap = MultiMap<String, (String, Semantics)>;

fn get_stem_semantics(text: &str, data: &io::Context) -> Option<Semantics> {
    let stems = &data.stem_map;

    for ending in data.ending_map.keys() {
        for (stem_type, semantics) in data.ending_map.get_vec(ending).unwrap() {
            let len_text = text.len();
            if !text.ends_with(ending) {
                continue;
            }

            let mut stem = String::new();
            let len_ending = ending.len();
            stem += &text[0..(len_text - len_ending)];
            stem += stem_type;

            if stems.contains_key(&stem) {
                if text == "m" {
                    print!("{} {} {}", text, ending, stem_type);
                    std::process::exit(1);
                }
                return Some(semantics.clone());
            }
        }
    }
    None
}

pub fn analyze(text: &str, data: &io::Context) -> Option<Semantics> {
    if data.pada_map.contains_key(text) {
        return Some(data.pada_map.get(text).unwrap().clone());
    }
    get_stem_semantics(text, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandhi;

    fn toy_data() -> io::Context {
        let mut pada_map = PadaMap::new();
        pada_map.insert(
            String::from("Bavati"),
            Semantics::Tinanta(Tinanta {
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
                    linga: Linga::Stri,
                    vacana: Vacana::Eka,
                    vibhakti: Vibhakti::V2,
                    is_compounded: false,
                }),
            ),
        );

        io::Context {
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
            analyze("Bavati", &ctx),
            Some(Semantics::Tinanta(Tinanta {
                purusha: Purusha::Prathama,
                vacana: Vacana::Eka,
                lakara: Lakara::Lat,
                pada: VerbPada::Parasmaipada,
            }))
        );
    }

    #[test]
    fn analyze_inflected_nominal() {
        let ctx = toy_data();
        assert_eq!(
            analyze("narasya", &ctx),
            Some(Semantics::Subanta(Subanta {
                linga: Linga::Pum,
                vacana: Vacana::Eka,
                vibhakti: Vibhakti::V6,
                is_compounded: false,
            }))
        );
    }

    #[test]
    fn analyze_inflected_krdanta() {
        let ctx = toy_data();
        assert_eq!(
            analyze("gacCantIm", &ctx),
            Some(Semantics::Subanta(Subanta {
                linga: Linga::Stri,
                vacana: Vacana::Eka,
                vibhakti: Vibhakti::V2,
                is_compounded: false,
            }))
        );
    }
}
