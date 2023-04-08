use crate::args::Krt;
use crate::prakriya::Prakriya;

mod basic;
mod unadi_sutras;
mod utils;

pub fn run(p: &mut Prakriya, krt: Krt) -> bool {
    // First, check if the pratyaya is an unAdi-pratyaya.
    let mut added = unadi_sutras::run(p, krt);
    if !added {
        added = basic::run(p, krt);
    }
    added
}
