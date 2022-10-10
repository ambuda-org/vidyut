/// Splits Sanskrit sentences into separate words with their semantics.
use log::debug;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

use crate::io;
use crate::padas;
use crate::sandhi;
use crate::semantics::Semantics;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParsedWord {
    text: String,
    semantics: Semantics,
}

#[derive(PartialEq, Eq, Hash)]
struct State {
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

fn priority(state: &State) -> i32 {
    -(state.remaining.len() as i32)
}

pub fn parse(text: &str, ctx: &io::Context) -> Option<Vec<ParsedWord>> {
    let mut pq = PriorityQueue::new();
    let mut cache: HashMap<String, Option<Semantics>> = HashMap::new();

    let initial_state = State {
        items: Vec::new(),
        remaining: text.to_string(),
    };
    let initial_priority = priority(&initial_state);
    pq.push(initial_state, initial_priority);

    while !pq.is_empty() {
        let (cur_state, _priority) = pq.pop().unwrap();
        debug!("Pop state: {:?} {}", cur_state.items, cur_state.remaining);

        if cur_state.remaining.is_empty() {
            return Some(cur_state.items);
        }
        for (first, second) in sandhi::split(&cur_state.remaining, &ctx.sandhi_rules) {
            if !sandhi::is_good_split(&cur_state.remaining, &first, &second) {
                continue;
            }
            if let Some(semantics) = analyze_pada(&first, ctx, &mut cache) {
                let mut new_state = State {
                    items: cur_state.items.clone(),
                    remaining: second.clone(),
                };
                new_state.items.push(ParsedWord {
                    text: first,
                    semantics,
                });
                let new_priority = priority(&new_state);
                pq.push(new_state, new_priority);
            }
        }
        debug!("Length of priority queue: {}", pq.len());
    }
    None
}
