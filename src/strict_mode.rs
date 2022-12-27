//! Heuristics for validating segmented candidates.

/// Simple hand-coded rules to avoid overgenerating.
use crate::sandhi::Split;
use crate::segmenting::Phrase;
use crate::sounds;
use vidyut_kosha::semantics::*;

/// Returns whether the given word semantics are invalid for the current solution.
pub fn is_valid_word(cur: &Phrase, split: &Split, semantics: &Pada) -> bool {
    if let Pada::Subanta(s) = &semantics {
        if_purvapada_then_not_chunk_end(split, s)
            && if_ac_pada_then_not_hal(split, s.is_purvapada)
            && if_not_in_compound_then_linga_match(cur, s)
    } else if let Pada::Tinanta(_) = &semantics {
        if_ac_pada_then_not_hal(split, false)
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
fn if_ac_pada_then_not_hal(split: &Split, is_purvapada: bool) -> bool {
    if split.first.ends_with(sounds::is_ac) && !is_purvapada {
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
fn if_not_in_compound_then_linga_match(cur: &Phrase, s: &Subanta) -> bool {
    let in_compound = match cur.words.last() {
        Some(word) => match &word.semantics {
            Pada::Subanta(s) => s.is_purvapada,
            _ => false,
        },
        None => false,
    };

    if in_compound {
        true
    } else {
        match &s.pratipadika {
            Pratipadika::Basic { text: _, lingas } => lingas.contains(&s.linga),
            // Otherwise, any linga is allowed.
            Pratipadika::Krdanta { .. } => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandhi::SplitKind;

    #[test]
    fn test_is_valid_word() {
        let cur = Phrase::new("tatra".to_string());
        let split = Split {
            first: "tatra".to_string(),
            second: "".to_string(),
            is_end_of_chunk: true,
            kind: SplitKind::Prefix,
        };
        let semantics = Pada::Avyaya(Avyaya {
            pratipadika: Pratipadika::Basic {
                text: "grAma".to_string(),
                lingas: Vec::new(),
            },
        });

        assert!(is_valid_word(&cur, &split, &semantics));
    }

    #[test]
    fn test_is_valid_word_with_invalid() {
        let cur = Phrase::new("grAmesa".to_string());
        let split = Split {
            first: "grAme".to_string(),
            second: "sa".to_string(),
            is_end_of_chunk: false,
            kind: SplitKind::Prefix,
        };
        let semantics = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: "grAma".to_string(),
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
