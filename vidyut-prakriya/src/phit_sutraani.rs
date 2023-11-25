use crate::core::term::Svara;
use crate::core::{Prakriya, Rule};

fn run_at(p: &mut Prakriya, i: usize) -> Option<()> {
    use Rule::Phit as P;
    use Svara::*;

    let num_vowels = p.get(i)?.num_vowels();

    p.run_at(P("1.1"), i, |t| t.set_svara(Udatta(num_vowels - 1)));

    Some(())
}

/// Runs the rules of the Phit-sutras.
pub fn run(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        if p.has(i, |t| t.is_pratipadika() && !t.is_pratyaya())
            && !p.has(i + 1, |t| t.is_taddhita())
        {
            run_at(p, i);
        }
    }
}
