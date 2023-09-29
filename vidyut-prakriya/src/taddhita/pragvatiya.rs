/*!
Implements the taddhita rules in the "prAg vatez Wan" section of pada 5.1.

(5.1.18 - 5.1.114)
*/
use crate::args::Artha::*;
use crate::args::Taddhita;
use crate::taddhita::utils::TaddhitaPrakriya;
use crate::tag::Tag as T;

pub fn run(tp: &mut TaddhitaPrakriya) {
    use Taddhita as P;

    tp.with_context(TenaKritam, |tp| {
        let prati = tp.prati();
        if prati.has_tag(T::Sankhya) {
            tp.try_add("5.1.22", P::kan);
        } else if prati.has_text_in(&["viMSati", "triMSat"]) {
            tp.try_add("5.1.24", P::qvun);
        } else if prati.has_text("kaMsa") {
            tp.try_add("5.1.25", P::wiWan);
        } else if prati.has_text("SUrpA") {
            tp.optional_try_add("5.1.26", P::aY);
        } else if prati.has_text_in(&["SatamAna", "viMSatika", "sahasra", "vasana"]) {
            tp.try_add("5.1.27", P::aR);
        } else if prati.has_text_in(&["dviSARa", "triSARa"]) {
            let code = "5.1.36";
            tp.try_add(code, P::yat);
            tp.try_add(code, P::aR);
        }
    });

    let prati = tp.prati();
    if prati.has_text("ftu") {
        tp.try_add("5.1.105", P::aR);
        tp.try_add("5.1.106", P::Gas);
    }
}
