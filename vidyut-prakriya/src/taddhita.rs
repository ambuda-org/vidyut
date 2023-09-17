use crate::args::Taddhita;
use crate::prakriya::Prakriya;

mod basic;
mod gana;
mod utils;

/// Runs the rules that add a taddhita-pratyaya to a given pratipadika.
/// Returns whether a pratyaya was added.
pub fn run(p: &mut Prakriya, t: Taddhita) -> bool {
    basic::try_add_taddhita(p, t).unwrap_or(false)
}
