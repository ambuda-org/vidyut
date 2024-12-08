/*!
Implements the taddhita rules in the "prAg diSo viBaktiH" section of pada 5.3.

(5.3.1 - 5.3.26)
*/
use crate::args::Taddhita;
use crate::args::Upasarga as U;
use crate::core::Tag as T;
use crate::taddhita::utils::TaddhitaPrakriya;

pub fn run(tp: &mut TaddhitaPrakriya) {
    let i_prati = tp.i_prati;
    let t = tp.taddhita;

    let add = |rule, tp: &mut TaddhitaPrakriya, taddhita| {
        tp.try_add_with(rule, taddhita, |p| {
            p.set(i_prati + 1, |t| {
                t.add_tag(T::Vibhakti);
            })
        });
    };

    let prati = tp.prati();
    // Use `P` because `T` conventionally refers to `Tag`.
    use Taddhita as P;
    match t {
        P::tasil => {
            if prati.has_u_in(&["kim", "bahu"]) || prati.has_tag(T::Sarvanama) {
                add("5.3.7", tp, t);
            } else if prati.is(U::pari) || prati.is(U::aBi) {
                add("5.3.9", tp, t);
            }
        }
        P::tral | P::ha => {
            let is_ha = t == P::ha;
            if prati.has_u("idam") {
                if is_ha {
                    add("5.3.11", tp, t);
                }
            } else if prati.has_u_in(&["kim", "bahu"]) || prati.has_tag(T::Sarvanama) {
                if prati.has_u("kim") && is_ha {
                    add("5.3.10", tp, t);
                } else {
                    add("5.3.13", tp, t);
                }
            }
        }
        P::at => {
            if prati.has_u("kim") {
                add("5.3.12", tp, t);
            }
        }
        P::dA | P::rhil | P::dAnIm => {
            if prati.has_u_in(&["sarva", "eka", "anya", "kim", "yad", "tad"]) {
                if prati.has_u("tad") && t == P::dAnIm {
                    add("5.3.19", tp, t);
                } else {
                    add("5.3.15", tp, t);
                }
            } else if prati.has_u("idam") {
                if t == P::rhil {
                    add("5.3.16", tp, t);
                } else if t == P::dAnIm {
                    add("5.3.18", tp, t);
                }
            }
        }
        P::TAl | P::Tamu => {
            let is_tham = t == P::Tamu;
            if prati.has_u("idam") && is_tham {
                add("5.3.23", tp, t);
            } else if prati.has_u("kim") && is_tham {
                add("5.3.24", tp, t);
            } else {
                add("5.3.25", tp, t);
            }
        }
        _ => return,
    }

    if tp.has_taddhita {
        let prati = tp.prati();
        let t = tp.p.terms().last().expect("ok");
        if prati.has_u("idam") {
            if t.has_adi('r') || t.has_adi('T') {
                let sub = if t.has_adi('r') { "eta" } else { "it" };
                tp.p.run_at("5.3.4", i_prati, |t| t.set_text(sub));
            } else {
                tp.p.run_at("5.3.3", i_prati, |t| t.set_text("i"));
            }
        } else if prati.has_u("etad") {
            tp.p.run_at("5.3.5", i_prati, |t| t.set_text("a"));
        } else if prati.has_u("sarva") && t.has_adi('d') {
            tp.p.optional_run_at("5.3.6", i_prati, |t| t.set_text("sa"));
        }
    }
}
