//! Splits Sanskrit phrases into separate words with their semantics.
use lazy_static::lazy_static;
use log::{debug, log_enabled, Level};
use priority_queue::PriorityQueue;
use regex::Regex;
use std::collections::HashMap;

use crate::context::Context;
use crate::sandhi;
use crate::scoring;
use crate::semantics::{Semantics, Stem};
use crate::strict_mode;

/// Represnts a Sanskrit word and its semantics.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParsedWord {
    pub text: String,
    pub semantics: Semantics,
}

impl ParsedWord {
    /// Get the word's root/stem.
    pub fn lemma(&self) -> String {
        match &self.semantics {
            Semantics::Tinanta(s) => s.root.clone(),
            Semantics::Subanta(s) => match &s.stem {
                Stem::Basic { stem, lingas: _ } => stem.clone(),
                Stem::Krdanta {
                    root,
                    tense: _,
                    prayoga: _,
                } => root.clone(),
            },
            Semantics::Ktva(s) => s.root.clone(),
            Semantics::Tumun(s) => s.root.clone(),
            _ => self.text.clone(),
        }
    }
}

/// Represents an in-progress parse of a phrase.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParsedPhrase {
    /// The words that we've recognized so far.
    pub words: Vec<ParsedWord>,
    /// The text we still need to parse.
    pub remaining: String,
    /// The score associated with this in-progress parse.
    pub score: i32,
}

impl ParsedPhrase {
    /// Create a new state.
    pub fn new(text: String) -> ParsedPhrase {
        ParsedPhrase {
            words: Vec::new(),
            remaining: text,
            // log_10(1) = 0
            score: 0,
        }
    }
}

/// Normalize text by replacing all runs of whitespace with " ".
/// TODO: also split Sanskrit symbols from non-Sanskrit symbols (numbers, punct, etc.)
fn normalize(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }
    RE.replace(text, " ").to_string()
}

fn analyze_pada(
    text: &str,
    split: &sandhi::Split,
    data: &Context,
    cache: &mut HashMap<String, Vec<Semantics>>,
) -> Vec<Semantics> {
    if !cache.contains_key(text) {
        let mut res = data.lexicon.find(text);

        // Add the option to skip an entire chunk. (For typos, junk, etc.)
        if split.is_end_of_chunk {
            res.push(Semantics::None);
        }

        cache.insert(text.to_string(), res);
    }
    cache.get(text).unwrap().to_vec()
}

#[allow(dead_code)]
fn debug_print_stack(pq: &PriorityQueue<ParsedPhrase, i32>) {
    if log_enabled!(Level::Debug) {
        debug!("Stack:");

        // The queue isn't sorted by default. So, sort from highest to lowest priotity.
        let mut words: Vec<(&ParsedPhrase, &i32)> = pq.iter().collect();
        words.sort_by(|x, y| y.1.cmp(x.1));

        for (i, (s, score)) in words.iter().enumerate() {
            let words: Vec<String> = s.words.iter().map(|x| x.text.clone()).collect();
            debug!("{}: \"{:?}\" + \"{}\" ({})", i, words, s.remaining, score);
        }
        debug!("-------------------");
    }
}

#[allow(dead_code)]
fn debug_print_viterbi(v: &HashMap<String, HashMap<String, ParsedPhrase>>) {
    if log_enabled!(Level::Debug) {
        debug!("Viterbi:");
        for (key1, entries) in v.iter() {
            for (key2, state) in entries.iter() {
                let words: Vec<String> = state.words.iter().map(|x| x.text.clone()).collect();
                debug!("(`{}`, {}) -> {:?} : {}", key1, key2, words, state.score);
            }
        }
        debug!("-------------------");
    }
}

/// Parse the given text.
///
/// # Arguments:
/// - `raw_text` - a text string in SLP1.
///
/// The parser makes a best effort and will make a best effort to understand the input as valid
/// Sanskrit text, even if it contains typos or any content that is not valid Sanskrit.
pub fn parse(raw_text: &str, ctx: &Context) -> Vec<ParsedWord> {
    let text = normalize(raw_text);
    let mut pq = PriorityQueue::new();
    let mut word_cache: HashMap<String, Vec<Semantics>> = HashMap::new();

    // viterbi_cache[remainder][state] = the best result that ends with $state and has $remainder
    // text remaining in the parse.
    let mut viterbi_cache: HashMap<String, HashMap<String, ParsedPhrase>> = HashMap::new();

    let initial_state = ParsedPhrase::new(text);
    let score = initial_state.score;
    pq.push(initial_state, score);

    while !pq.is_empty() {
        debug_print_stack(&pq);
        // debug_print_viterbi(&viterbi_cache);

        let (cur, cur_score) = pq.pop().unwrap();

        for split in ctx.sandhi.split_all(&cur.remaining) {
            if !split.is_valid() || split.is_recursive(&cur.remaining) {
                continue;
            }

            let first = &split.first;
            let second = &split.second;
            for semantics in analyze_pada(first, &split, ctx, &mut word_cache) {
                if !strict_mode::is_valid_word(&cur, &split, &semantics) {
                    continue;
                }

                let mut new = ParsedPhrase {
                    words: cur.words.clone(),
                    remaining: second.clone(),
                    // HACK: this is buggy -- scoring based on cur score set here?
                    score: cur_score,
                };
                new.words.push(ParsedWord {
                    text: first.clone(),
                    semantics,
                });
                new.score = scoring::heuristic_score(&new);

                // Use state "STATE" for now since we don't have any states implemented.
                let maybe_rival = viterbi_cache
                    .entry(second.clone())
                    .or_insert_with(HashMap::new)
                    .get("STATE");
                let new_score = new.score;
                if let Some(rival) = maybe_rival {
                    if rival.score >= new.score {
                        continue;
                    }
                };
                viterbi_cache
                    .entry(second.clone())
                    .or_insert_with(HashMap::new)
                    .insert("STATE".to_string(), new.clone());
                pq.push(new, new_score);
            }
        }
    }

    // Return the best result we could find above.
    if let Some(solutions) = viterbi_cache.get("") {
        if let Some(best) = solutions.values().max_by_key(|s| s.score) {
            return best.words.clone();
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let before = "some   whitespace";
        assert_eq!(normalize(before), "some whitespace".to_string());
    }
}
