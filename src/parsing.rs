/// Splits Sanskrit sentences into separate words with their semantics.
use log::debug;
use priority_queue::PriorityQueue;
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

#[derive(PartialEq, Eq, Hash)]
pub struct State {
    pub items: Vec<ParsedWord>,
    pub remaining: String,
}

fn analyze_pada(
    text: &str,
    data: &io::Context,
    cache: &mut HashMap<String, Option<Semantics>>,
) -> Option<Semantics> {
    if !cache.contains_key(text) {
        cache.insert(text.to_string(), padas::analyze(text, data));
    }
    cache.get(text).unwrap().as_ref().cloned()
}

pub fn parse(text: &str, ctx: &io::Context) -> Option<Vec<ParsedWord>> {
    let mut pq = PriorityQueue::new();
    let mut cache: HashMap<String, Option<Semantics>> = HashMap::new();

    let initial_state = State {
        items: Vec::new(),
        remaining: text.to_string(),
    };
    let initial_score = scoring::heuristic_score(&initial_state);
    pq.push(initial_state, initial_score);

    while !pq.is_empty() {
        let (cur, cur_score) = pq.pop().unwrap();
        debug!("Pop state: {} {} {:?}", cur.remaining, cur_score, cur.items);

        // If the state is solved (no remaining text), return it.
        //
        // We return at the first solved state we see, and we rank all states in our queue.
        // Therefore, this is the best solution we'll be able to find.
        if cur.remaining.is_empty() {
            return Some(cur.items);
        }

        for (first, second) in sandhi::split(&cur.remaining, &ctx.sandhi_rules) {
            // Skip splits that have obvious problems.
            if !sandhi::is_good_split(&cur.remaining, &first, &second) {
                continue;
            }

            if let Some(semantics) = analyze_pada(&first, ctx, &mut cache) {
                let mut new = State {
                    items: cur.items.clone(),
                    remaining: second.clone(),
                };
                new.items.push(ParsedWord {
                    text: first,
                    semantics,
                });
                let new_score = scoring::heuristic_score(&new);
                debug!(
                    "Push state: {} {} {:?}",
                    new.remaining, new_score, new.items
                );
                pq.push(new, new_score);
            }
        }
        debug!("Length of priority queue: {}", pq.len());
    }
    None
}
