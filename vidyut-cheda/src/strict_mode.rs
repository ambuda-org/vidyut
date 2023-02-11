//! Heuristics for validating segmented candidates.

use crate::segmenting::{Phrase, TokenPool};
use crate::sounds;
use vidyut_kosha::morph::*;
/// Simple hand-coded rules to avoid overgenerating.
use vidyut_sandhi::Split;

/// Returns whether the given word semantics are invalid for the current solution.
pub(crate) fn is_valid_word(
    cur: &Phrase,
    pool: &TokenPool,
    split: &Split,
    semantics: &Pada,
) -> bool {
    if let Pada::Subanta(s) = &semantics {
        if_purvapada_then_not_chunk_end(split, s)
            && if_ac_pada_then_not_hal(split, s.is_purvapada)
            && if_not_in_compound_then_linga_match(cur, pool, s)
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
        !split.is_end_of_chunk()
    } else {
        true
    }
}

// Require that vowel-final words are not immediately followed by consonants.
// (`iti ca` vs. `itica`)
fn if_ac_pada_then_not_hal(split: &Split, is_purvapada: bool) -> bool {
    if split.first().ends_with(sounds::is_ac) && !is_purvapada {
        // iti ca
        split.is_end_of_chunk()
        // sEva (sA eva)
        || split.second().starts_with(sounds::is_ac)
    } else {
        true
    }
}

// Require that subantas use the endings that match their declared linga.
// Exception: words in a compound, since these might be bahuvrihi compounds.
fn if_not_in_compound_then_linga_match(cur: &Phrase, pool: &TokenPool, s: &Subanta) -> bool {
    let in_compound = match cur.tokens.last() {
        Some(i) => match pool.get(*i) {
            Some(t) => match &t.info {
                Pada::Subanta(s) => s.is_purvapada,
                _ => false,
            },
            None => false,
        },
        None => false,
    };

    if in_compound {
        true
    } else {
        match (&s.linga, &s.pratipadika) {
            (Some(x), Pratipadika::Basic { text: _, lingas }) => lingas.contains(x),
            // Otherwise, any linga is allowed.
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token;
    use compact_str::CompactString;
    use vidyut_sandhi::{Kind, Location};

    #[test]
    fn test_is_valid_word() {
        let cur = Phrase::new("tatra".to_string());
        let split = Split::new(
            "tatra".to_string(),
            "".to_string(),
            Location::EndOfChunk,
            Kind::Prefix,
        );
        let info = Pada::Avyaya(Avyaya {
            pratipadika: Pratipadika::Basic {
                text: "grAma".to_string(),
                lingas: Vec::new(),
            },
        });

        let mut token_pool = TokenPool::new();
        token_pool.insert(Token {
            text: CompactString::from("tatra"),
            info: info.clone(),
        });
        assert!(is_valid_word(&cur, &token_pool, &split, &info));
    }

    #[test]
    fn test_is_valid_word_with_invalid() {
        let cur = Phrase::new("grAmesa".to_string());
        let split = Split::new(
            "grAme".to_string(),
            "sa".to_string(),
            Location::WithinChunk,
            Kind::Prefix,
        );
        let info = Pada::Subanta(Subanta {
            pratipadika: Pratipadika::Basic {
                text: "grAma".to_string(),
                lingas: vec![Linga::Pum],
            },
            linga: Some(Linga::Pum),
            vacana: Some(Vacana::Eka),
            vibhakti: Some(Vibhakti::V7),
            is_purvapada: false,
        });

        let mut token_pool = TokenPool::new();
        token_pool.insert(Token {
            text: CompactString::from("grAme"),
            info: info.clone(),
        });
        assert!(!is_valid_word(&cur, &token_pool, &split, &info));
    }
}
