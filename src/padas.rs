use crate::io;
use multimap::MultiMap;

pub type Semantics = String;
pub type StemMap = MultiMap<String, Semantics>;
pub type PadaMap = MultiMap<String, Semantics>;
pub type EndingMap = MultiMap<String, (String, Semantics)>;

fn get_stem_semantics(text: &str, data: &io::Context) -> Option<Semantics> {
    let stems = &data.stem_map;

    for ending in data.ending_map.keys() {
        for (stem_type, lex) in data.ending_map.get_vec(ending).unwrap() {
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
                return Some(lex.clone());
            }
        }
    }
    None
}

pub fn analyze(text: &str, data: &io::Context) -> Option<Semantics> {
    if data.pada_map.contains_key(text) {
        return Some(data.pada_map.get(text).unwrap().to_string());
    }
    get_stem_semantics(text, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandhi;

    fn toy_data() -> io::Context {
        let mut pada_map = PadaMap::new();
        pada_map.insert(String::from("Bavati"), String::from("v-3-s"));

        let mut stem_map = StemMap::new();
        stem_map.insert(String::from("nara"), String::from("m"));
        stem_map.insert(String::from("gacCat"), String::from("f"));

        let mut ending_map = EndingMap::new();
        ending_map.insert(
            String::from("asya"),
            (String::from("a"), String::from("m-6-s")),
        );
        ending_map.insert(
            String::from("antIm"),
            (String::from("at"), String::from("f-2-s")),
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
        assert_eq!(analyze("Bavati", &ctx), Some(String::from("v-3-s")));
    }

    #[test]
    fn analyze_inflected_nominal() {
        let ctx = toy_data();
        assert_eq!(analyze("narasya", &ctx), Some(String::from("m-6-s")));
    }

    #[test]
    fn analyze_inflected_krdanta() {
        let ctx = toy_data();
        assert_eq!(analyze("gacCantIm", &ctx), Some(String::from("f-2-s")));
    }
}
