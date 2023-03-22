/*!
Implements rules from the pāṇiṇīyaliṅgānuśāśanam, which assigns lingas to various terms.
*/

use crate::prakriya::Prakriya;
use crate::tag::Tag as T;

/// Runs the linganushasana rules over the given prakriya.
pub fn run(p: &mut Prakriya) -> Option<()> {
    let last = p.terms().last()?;

    if last.is_taddhita() && last.has_u("tal") {
        p.add_tag(T::Stri);
        p.step("li.17");
    } else if last.is_krt() {
        if last.has_u_in(&["GaY", "ap"]) {
            p.add_tag(T::Pum);
            p.step("li.36");
        } else if last.has_u("naN") {
            let i_dhatu = p.find_last_where(|t| t.is_dhatu())?;
            let dhatu = p.get(i_dhatu)?;
            if dhatu.has_text("yAc") {
                p.add_tag(T::Stri);
                p.step("li.40");
            } else {
                p.add_tag(T::Pum);
                p.step("li.39");
            }
        }
    }

    Some(())
}
