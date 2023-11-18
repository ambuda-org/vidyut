use crate::args::KrdantaArgs;
use crate::core::Prakriya;

mod basic;
mod unadi_sutras;
mod utils;

pub fn run(p: &mut Prakriya, args: &KrdantaArgs) -> bool {
    // First, check if the pratyaya is an unAdi-pratyaya.
    let mut added = unadi_sutras::run(p, args.krt());
    if !added {
        added = basic::run(p, args.krt());
    }
    added
}
