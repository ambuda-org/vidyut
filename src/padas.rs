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
            stem += &stem_type;

            if stems.contains_key(&stem) {
                if text == "m" {
                    print!("{} {} {}", text, ending, stem_type);
                    std::process::exit(1);
                }
                return Some(lex.clone());
            }
        }
    }
    return None;
}

pub fn analyze(text: &str, data: &io::Context) -> Option<Semantics> {
    if data.pada_map.contains_key(text) {
        return Some(data.pada_map.get(text).unwrap().to_string());
    }
    return get_stem_semantics(text, &data);
}
