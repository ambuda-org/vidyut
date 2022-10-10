use log::debug;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

use crate::io;
use crate::padas;
use crate::sandhi;

#[derive(PartialEq, Eq, Hash)]
struct State {
    pub items: Vec<String>,
    pub remaining: String,
}

pub fn parse(text: &str, ctx: &io::Context) -> Option<Vec<String>> {
    let mut pq = PriorityQueue::new();
    let mut cache: HashMap<String, bool> = HashMap::new();

    let priority = -1 * (text.len() as i32);
    pq.push(
        State {
            items: Vec::new(),
            remaining: text.to_string(),
        },
        priority,
    );

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
            if padas::is_pada(&first, &ctx, &mut cache) {
                let mut new_state = State {
                    items: cur_state.items.clone(),
                    remaining: second.clone(),
                };
                new_state.items.push(first);
                let priority = -1 * (new_state.remaining.len() as i32);
                pq.push(new_state, priority);
            }
        }
        debug!("Length of priority queue: {}", pq.len());
    }
    return None;
}
