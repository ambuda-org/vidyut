use crate::args::Krdanta;
use crate::core::Prakriya;

mod basic;
mod unadipatha;
mod utils;

pub fn run(p: &mut Prakriya, args: &Krdanta) -> bool {
    // First, check if the pratyaya is an *uṇādi pratyaya*.
    let mut added = unadipatha::run(p, args.krt());
    if !added {
        added = basic::run(p, args.krt());
    }
    added
}
