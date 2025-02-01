/*!
tripadi
=======
(8.2.1 - end of 8.4)

The final three pādas of the Ashtadhyayi are called the **tripādi**. The tripādi generally contains
sandhi rules and other miscellaneous sound change rules.

The tripādi uses a different rule selection mechanism from the rest of the Ashtadhyayi: whereas the
rest of the text selects rules based on their priority and allows a rule to apply if it has scope,
the tripādi applies rules in order and will (generally) never "go back" to apply an earlier rule.
*/

mod pada_8_2;
mod pada_8_3;
mod pada_8_4;

use crate::args::BaseKrt as K;
use crate::args::Upasarga as U;
use crate::core::Prakriya;

/// Runs rules that should apply before dvitva.
pub fn run_before_dvitva(p: &mut Prakriya) {
    p.debug("trying tripadi::run_before_dvitva");
    for i_x in 0..p.len() {
        let x = p.get_if(i_x, |t| !t.is_empty());
        let i_y = p.next_not_empty(i_x);
        if let (Some(x), Some(i_y)) = (x, i_y) {
            let y = &p.terms()[i_y];
            let z = p.get_if(i_y + 1, |t| !t.is_empty());

            if x.is_any_upasarga(&[U::su, U::vi, U::nir, U::dur])
                && ((y.has_u(&"Yizva\\pa~") && y.has_text("sup"))
                    || (y.has_text("sU") && z.map_or(false, |t| t.is(K::ktin)))
                    || (y.has_text("sama") && y.is_pratipadika()))
            {
                // suzuzupatuH, ...
                //
                // (must apply before dvitva so that the abhyAsa is also `z`)
                p.run_at("8.3.88", i_y, |t| t.set_adi_char('z'));
            }
        }
    }
}

/// Runs all rules of the tripadi.
///
/// This function is responsible for around 25% of runtime.
pub fn run(p: &mut Prakriya) {
    pada_8_2::run(p);
    pada_8_3::run(p);
    pada_8_4::run(p);
}
