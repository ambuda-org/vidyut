// Score a parse state (higher scores are better).
use crate::parsing::State;

pub fn heuristic_score(state: &State) -> i32 {
    -(state.remaining.len() as i32)
}
