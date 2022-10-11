// Score a parse state (higher scores are better).
use crate::parsing::State;
use crate::semantics::*;

fn subanta_bonus(s: &Subanta) -> i32 {
    let mut bonus = 0;
    if s.vacana != Vacana::Dvi {
        bonus += 1;
    }
    if s.linga == Linga::Pum {
        bonus += 1;
    }
    if s.vibhakti == Vibhakti::V1 {
        bonus += 1;
    }
    bonus
}

pub fn heuristic_score(state: &State) -> i32 {
    // - Subtract from a large positive number so that the score is non-negative.
    // - Multiply by some positive number so that we can award bonuses without interfering with
    //   this score.
    let progress_score = 1000000 - 10 * (state.remaining.len() as i32);
    assert!(progress_score > 0);

    let mut bonus = 0;
    if let Some(last) = state.items.last() {
        let semantics = &last.semantics;
        bonus = match semantics {
            Semantics::Subanta(s) => subanta_bonus(s),
            &_ => 0,
        };
    }

    progress_score + bonus
}
