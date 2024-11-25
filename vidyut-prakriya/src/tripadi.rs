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
use crate::core::Prakriya;

/// Runs all rules of the tripadi.
///
/// This function is responsible for around 25% of runtime.
pub fn run(p: &mut Prakriya) {
    pada_8_2::run(p);
    pada_8_3::run(p);
    pada_8_4::run(p);
}
