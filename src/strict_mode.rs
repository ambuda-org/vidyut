use crate::parsing::ParsedPhrase;
/// Simple hand-coded rules to avoid overgenerating.
use crate::sandhi::Split;
use crate::semantics::*;
use crate::sounds;

/// Returns whether the given word semantics are invalid for the current parse.
pub fn is_valid_word(cur: &ParsedPhrase, split: &Split, semantics: &Semantics) -> bool {
    if let Semantics::Subanta(s) = &semantics {
        if_purvapada_then_not_chunk_end(split, s)
            && if_ac_pada_then_not_hal(split, s)
            && if_not_in_compound_then_linga_match(cur, s)
    } else {
        true
        // TODO: extend if_ac_pada... to verbs
    }
}

/// Avoid compounds with whitespace.
/// (`Darmakzetre` vs. `Darma kzetre`)
fn if_purvapada_then_not_chunk_end(split: &Split, s: &Subanta) -> bool {
    if s.is_purvapada {
        !split.is_end_of_chunk
    } else {
        true
    }
}

// Require that vowel-final words are not immediately followed by consonants.
// (`iti ca` vs. `itica`)
fn if_ac_pada_then_not_hal(split: &Split, s: &Subanta) -> bool {
    if split.first.ends_with(sounds::is_ac) && !s.is_purvapada {
        // iti ca
        split.is_end_of_chunk
        // sEva (sA eva)
        || split.second.starts_with(sounds::is_ac)
    } else {
        true
    }
}

// Require that subantas use the endings that match their declared linga.
// Exception: words in a compound, since these might be bahuvrihi compounds.
fn if_not_in_compound_then_linga_match(cur: &ParsedPhrase, s: &Subanta) -> bool {
    let in_compound = match cur.words.last() {
        Some(word) => match &word.semantics {
            Semantics::Subanta(s) => s.is_purvapada,
            _ => false,
        },
        None => false,
    };

    if !in_compound {
        match &s.stem {
            Stem::Basic { stem: _, lingas } => lingas.contains(&s.linga),
            // Otherwise, any linga is allowed.
            _ => true,
        }
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandhi::SplitKind;

    #[test]
    fn test_is_valid_word() {
        let cur = ParsedPhrase::new("tatra".to_string());
        let split = Split {
            first: "tatra".to_string(),
            second: "".to_string(),
            is_end_of_chunk: true,
            kind: SplitKind::Prefix,
        };
        let semantics = Semantics::Avyaya;

        assert!(is_valid_word(&cur, &split, &semantics));
    }

    #[test]
    fn test_is_valid_word_with_invalid() {
        let cur = ParsedPhrase::new("grAmesa".to_string());
        let split = Split {
            first: "grAme".to_string(),
            second: "sa".to_string(),
            is_end_of_chunk: false,
            kind: SplitKind::Prefix,
        };
        let semantics = Semantics::Subanta(Subanta {
            stem: Stem::Basic {
                stem: "grAma".to_string(),
                lingas: vec![Linga::Pum],
            },
            linga: Linga::Pum,
            vacana: Vacana::Eka,
            vibhakti: Vibhakti::V7,
            is_purvapada: false,
        });

        assert!(!is_valid_word(&cur, &split, &semantics));
    }
}
