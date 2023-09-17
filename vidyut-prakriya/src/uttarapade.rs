//! Adhyaya 6.3 of the Ashtadhyayi concerns itself with changes caused by a following word
//! (*uttarpada*). For now, we keep those rule here.

use crate::prakriya::Prakriya;
use crate::tag::Tag as T;

pub fn run(p: &mut Prakriya) -> Option<()> {
    let last = p.terms().last()?;
    if p.terms().len() == 2
        && p.has(0, |t| !t.is_ekac() && t.ends_with("I"))
        && (last.has_tag(T::Gha) || last.has_u_in(&["rUpap", "kalpap"]))
    {
        // TODO; check for bhASitapuMsvat
        p.op_term("6.3.43", 0, |t| {
            // Rule only applies for NI, so just change `I`.
            t.set_antya("i");
        });
    }
    Some(())
}
