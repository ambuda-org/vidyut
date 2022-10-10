use crate::io;
use multimap::MultiMap;
use std::collections::HashMap;

pub type StemMap = MultiMap<String, String>;
pub type PadaMap = MultiMap<String, String>;
pub type EndingMap = MultiMap<String, (String, String)>;

fn is_valid_stem(text: &str, data: &io::Context) -> bool {
    let stems = &data.stem_map;

    for ending in data.ending_map.keys() {
        for (stem_type, _lex) in data.ending_map.get_vec(ending).unwrap() {
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
                return true;
            }
        }
    }
    false
}

fn is_pada_uncached(text: &str, data: &io::Context) -> bool {
    if data.pada_map.contains_key(text) {
        return true;
    }
    if is_valid_stem(text, &data) {
        return true;
    }
    false
}

pub fn is_pada(text: &str, data: &io::Context, cache: &mut HashMap<String, bool>) -> bool {
    if !cache.contains_key(text) {
        cache.insert(text.to_string(), is_pada_uncached(&text, &data));
    }
    *cache.get(text).unwrap()
}
