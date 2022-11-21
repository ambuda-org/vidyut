/// A hacky generator that creates nominal stems.
use multimap::MultiMap;
use regex::Regex;

use crate::old_lexicon::{EndingMap, PadaMap, StemMap};
use crate::semantics::*;
use crate::sounds::{is_ac, is_ghosha};

fn inflect_halanta_stem(stem: &str, sup: &str) -> String {
    if sup.starts_with(is_ac) {
        String::from(stem) + sup
    } else {
        let n = stem.len();
        let prefix = &stem[..n - 1];
        let stem_ending = &stem[n - 1..n];

        let stem_ending = match stem_ending {
            "k" | "K" | "g" | "G" => "k",
            "c" | "C" | "j" | "J" => "k",
            "w" | "W" | "q" | "Q" => "w",
            "t" | "T" | "d" | "D" => "t",
            "p" | "P" | "b" | "B" => "p",
            _ => stem_ending,
        };
        let stem_ending = if sup.starts_with(is_ghosha) {
            match stem_ending {
                "k" => "g",
                "w" => "q",
                "t" => "d",
                "p" => "b",
                _ => stem_ending,
            }
        } else {
            stem_ending
        };

        String::from(prefix) + stem_ending + sup
    }
}

// Generates all nominal padas and adds them to the pada map.
pub fn add_nominals(stems: &StemMap, endings: &EndingMap, padas: &mut PadaMap) {
    let stem_to_endings = endings
        .iter_all()
        .flat_map(|(ending, vs)| {
            vs.iter()
                .map(|(stem, pada)| (stem.clone(), (ending.clone(), pada.clone())))
        })
        .collect::<MultiMap<String, (String, Pada)>>();

    let re_halanta = Regex::new(r".*[kKgGNcCjJYwWqQRtTdDnpPbBmSzsh]$").unwrap();

    // For all stems, ...
    for (stem_text, all_stem_semantics) in stems.iter_all() {
        let mut has_match = false;

        // And all stem endings ...
        for (stem_ending, sup_pratyayas) in stem_to_endings.iter_all() {
            // If the stem ends in this ending ...
            if let Some(prefix) = stem_text.strip_suffix(stem_ending) {
                // Then for all pratyayas that the ending allows, ...
                for (sup_text, sup_semantics) in sup_pratyayas {
                    let pada_text = prefix.to_string() + sup_text;

                    if let Pada::Subanta(sup_semantics) = sup_semantics {
                        for stem_semantics in all_stem_semantics {
                            // Create and insert the corresponding pada.
                            let pada_semantics = Pada::Subanta(Subanta {
                                pratipadika: stem_semantics.clone(),
                                ..sup_semantics.clone()
                            });
                            padas.insert(pada_text.clone(), pada_semantics);
                        }
                    }
                }
                has_match = true;
            }
        }

        if !has_match {
            // If the stem is a special consonant ending ...
            if re_halanta.is_match(stem_text) {
                let pratyayas = stem_to_endings
                    .get_vec("_")
                    .expect("`_` ending should be defined");
                for (sup_text, sup_semantics) in pratyayas {
                    let pada_text = inflect_halanta_stem(stem_text, sup_text);

                    if let Pada::Subanta(sup_semantics) = sup_semantics {
                        for stem_semantics in all_stem_semantics {
                            // Create and insert the corresponding pada.
                            let pada_semantics = Pada::Subanta(Subanta {
                                pratipadika: stem_semantics.clone(),
                                ..sup_semantics.clone()
                            });
                            padas.insert(pada_text.clone(), pada_semantics);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inflect_halanta_stem() {
        let tests = vec![
            ("vAc", "as", "vAcas"),
            ("vAc", "", "vAk"),
            ("vAc", "ByAm", "vAgByAm"),
            ("vid", "as", "vidas"),
            ("vid", "", "vit"),
            ("vid", "ByAm", "vidByAm"),
            ("kakuB", "as", "kakuBas"),
            ("kakuB", "", "kakup"),
            ("kakuB", "ByAm", "kakubByAm"),
        ];

        for (stem, sup, pada) in tests {
            assert_eq!(inflect_halanta_stem(stem, sup), pada);
        }
    }
}
