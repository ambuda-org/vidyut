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

    let prati = tp.prati();
    // Use `P` because `T` conventionally refers to `Tag`.
    use Taddhita as P;
    match t {
        P::tasil => {
            if prati.has_u_in(&["kim", "bahu"]) || prati.has_tag(T::Sarvanama) {
                tp.try_add("5.3.7", t);
            } else if prati.is(U::pari) || prati.is(U::aBi) {
                tp.try_add("5.3.9", t);
            }
        }
        P::tral | P::ha => {
            let is_ha = t == P::ha;
            if prati.has_u("idam") {
                if is_ha {
                    tp.try_add("5.3.11", t);
                }
            } else if prati.has_u_in(&["kim", "bahu"]) || prati.has_tag(T::Sarvanama) {
                if prati.has_u("kim") && is_ha {
                    tp.try_add("5.3.10", t);
                } else {
                    tp.try_add("5.3.13", t);
                }
            }
        }
        P::at => {
            if prati.has_u("kim") {
                tp.try_add("5.3.12", t);
            }
        }
        P::dA | P::rhil | P::dAnIm => {
            if prati.has_u_in(&["sarva", "eka", "anya", "kim", "yad", "tad"]) {
                if prati.has_u("tad") && t == P::dAnIm {
                    tp.try_add("5.3.19", t);
                } else {
                    tp.try_add("5.3.15", t);
                }
            } else if prati.has_u("idam") {
                if t == P::rhil {
                    tp.try_add("5.3.16", t);
                } else if t == P::dAnIm {
                    tp.try_add("5.3.18", t);
                }
            }
        }
        P::TAl | P::Tamu => {
            let is_tham = t == P::Tamu;
            if prati.has_u("idam") && is_tham {
                tp.try_add("5.3.23", t);
            } else if prati.has_u("kim") && is_tham {
                tp.try_add("5.3.24", t);
            } else {
                tp.try_add("5.3.25", t);
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
