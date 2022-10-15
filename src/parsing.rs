/// Splits Sanskrit sentences into separate words with their semantics.
use lazy_static::lazy_static;
use log::{debug, log_enabled, Level};
use priority_queue::PriorityQueue;
use regex::Regex;
use std::collections::HashMap;

use crate::io;
use crate::padas;
use crate::sandhi;
use crate::scoring;
use crate::semantics::Semantics;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParsedWord {
    pub text: String,
    pub semantics: Semantics,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub items: Vec<ParsedWord>,
    pub remaining: String,
    pub score: i32,
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
    data: &io::Context,
    cache: &mut HashMap<String, Vec<Semantics>>,
) -> Vec<Semantics> {
    if !cache.contains_key(text) {
        cache.insert(text.to_string(), padas::analyze(text, data));
    }
    cache.get(text).unwrap().to_vec()
}

#[allow(dead_code)]
fn debug_print_stack(pq: &PriorityQueue<State, i32>) {
    if log_enabled!(Level::Debug) {
        debug!("Stack:");
        for (i, (s, score)) in pq.iter().enumerate() {
            let words: Vec<String> = s.items.iter().map(|x| x.text.clone()).collect();
            debug!("{}: \"{:?}\" + \"{}\" ({})", i, words, s.remaining, score);
        }
        debug!("-------------------");
    }
}

#[allow(dead_code)]
fn debug_print_viterbi(v: &HashMap<String, HashMap<String, State>>) {
    if log_enabled!(Level::Debug) {
        debug!("Viterbi:");
        for (key1, entries) in v.iter() {
            for (key2, state) in entries.iter() {
                let words: Vec<String> = state.items.iter().map(|x| x.text.clone()).collect();
                debug!("(`{}`, {}) -> {:?} : {}", key1, key2, words, state.score);
            }
        }
        debug!("-------------------");
    }
}

pub fn parse(raw_text: &str, ctx: &io::Context) -> Option<Vec<ParsedWord>> {
    let text = normalize(raw_text);
    let mut pq = PriorityQueue::new();
    let mut word_cache: HashMap<String, Vec<Semantics>> = HashMap::new();

    // viterbi_cache[remainder][state] = the best result that ends with $state and has $remainder
    // text remaining in the parse.
    let mut viterbi_cache: HashMap<String, HashMap<String, State>> = HashMap::new();

    // log_10(1) = 0
    let initial_score = 0;
    let initial_state = State {
        items: Vec::new(),
        remaining: text,
        score: initial_score,
    };
    pq.push(initial_state, initial_score);

    while !pq.is_empty() {
        debug_print_stack(&pq);
        debug_print_viterbi(&viterbi_cache);

        let (cur, cur_score) = pq.pop().unwrap();

        for (first, second) in sandhi::split(&cur.remaining, &ctx.sandhi_rules) {
            // Skip splits that have obvious problems.
            if !sandhi::is_good_split(&cur.remaining, &first, &second) {
                continue;
            }

            for semantics in analyze_pada(&first, ctx, &mut word_cache) {
                let mut new = State {
                    items: cur.items.clone(),
                    remaining: second.clone(),
                    // HACK: this is buggy -- scoring based on cur score set here?
                    score: cur_score,
                };
                new.items.push(ParsedWord {
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
    match viterbi_cache.get("") {
        Some(solutions) => {
            let best = solutions.values().max_by_key(|s| s.score);
            best.map(|s| s.items.clone())
        }
        None => None,
    }
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
