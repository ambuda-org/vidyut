//! Scores a parse state (higher scores are better).

use crate::parsing::State;
use crate::semantics::*;

/// Assign a heuristic score to the given state.
///
/// Our heuristic models a log probability:
///
/// ------------+-------------------
/// Probability | log_10 probability
/// ------------+-------------------
///           1 |  0
///         0.1 | -1
///        0.01 | -2
/// ------------+-------------------
///
/// Unknown tokens, which are extremely rare, are treated as -100.
pub fn heuristic_score(state: &State) -> i32 {
    if let Some(last) = state.words.last() {
        let semantics = &last.semantics;
        match semantics {
            Semantics::None => state.score - 100,
            &_ => state.score - 1,
        }
    } else {
        state.score - 100
    }
}
