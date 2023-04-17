/*!
Runs rules that add taddhita-pratyayas after a prAtipadika..

*Status: experimental.*
*/

use crate::args::Taddhita;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::taddhita_gana as gana;
use crate::tag::Tag as T;
use crate::term::Term;

impl Taddhita {
    fn to_term(self) -> Term {
        let mut taddhita = Term::make_upadesha(self.as_str());
        taddhita.add_tags(&[T::Pratyaya, T::Taddhita]);
        taddhita
    }
}

fn add_vibhakti_pratyayas(p: &mut Prakriya, t: Taddhita) -> Option<bool> {
    let i = p.terms().len() - 1;
    let prati = p.get(i)?;

    let add = |rule, p: &mut Prakriya, t: Taddhita| {
        let i = p.terms().len();
        let taddhita = t.to_term();
        p.terms_mut().push(taddhita);
        p.step(rule);

        if t != Taddhita::at {
            // TODO: not sure how to support `at`.
            p.set(i, |t| t.add_tag(T::Vibhakti));
        }

        it_samjna::run(p, i).expect("should never fail");
    };

    // Use `P` because `T` conventionally refers to `Tag`.
    use Taddhita as P;
    match t {
        P::tasil => {
            if prati.has_u_in(&["kim", "bahu"]) || prati.has_tag(T::Sarvanama) {
                add("5.3.7", p, t);
            } else if prati.has_u_in(&["pari", "aBi"]) {
                add("5.3.9", p, t);
            }
        }
        P::tral | P::ha => {
            let is_ha = t == P::ha;
            if prati.has_u("idam") {
                if is_ha {
                    add("5.3.11", p, t);
                }
            } else if prati.has_u_in(&["kim", "bahu"]) || prati.has_tag(T::Sarvanama) {
                if prati.has_u("kim") && is_ha {
                    add("5.3.10", p, t);
                } else {
                    add("5.3.13", p, t);
                }
            }
        }
        P::at => {
            if prati.has_u("kim") {
                add("5.3.12", p, t);
            }
        }
        P::dA | P::rhil | P::dAnIm => {
            if prati.has_u_in(&["sarva", "eka", "anya", "kim", "yad", "tad"]) {
                if prati.has_u("tad") && t == P::dAnIm {
                    add("5.3.19", p, t);
                } else {
                    add("5.3.15", p, t);
                }
            } else if prati.has_u("idam") {
                if t == P::rhil {
                    add("5.3.16", p, t);
                } else if t == P::dAnIm {
                    add("5.3.18", p, t);
                }
            }
        }
        P::TAl | P::Tamu => {
            let is_tham = t == P::Tamu;
            if prati.has_u("idam") && is_tham {
                add("5.3.23", p, t);
            } else if prati.has_u("kim") && is_tham {
                add("5.3.24", p, t);
            } else {
                add("5.3.25", p, t);
            }
        }
        _ => return None,
    }

    let added = p.terms().last()?.is_taddhita();

    if added {
        let prati = p.get(i)?;
        let t = p.get(i + 1)?;
        if prati.has_u("idam") {
            if t.has_adi('r') || t.has_adi('T') {
                let sub = if t.has_adi('r') { "eta" } else { "it" };
                p.op_term("5.3.4", i, |t| t.set_text(sub));
            } else {
                p.op_term("5.3.3", i, |t| t.set_text("i"));
            }
        } else if prati.has_u("etad") {
            p.op_term("5.3.5", i, |t| t.set_text("a"));
        } else if prati.has_u("sarva") && t.has_adi('d') {
            p.op_optional("5.3.6", op::t(i, |t| t.set_text("sa")));
        }
    }

    Some(added)
}

fn try_add_taddhita(p: &mut Prakriya, t: Taddhita) -> Option<bool> {
    let i = p.terms().len() - 1;
    let prati = p.get(i)?;

    let add = |rule, p: &mut Prakriya, t: Taddhita| {
        let i = p.terms().len();
        p.terms_mut().push(t.to_term());
        p.step(rule);
        it_samjna::run(p, i).expect("should never fail");
    };

    // Use `P` because `T` conventionally refers to `Tag`.
    use Taddhita as P;
    match t {
        P::aR => {
            add("4.1.83", p, t);
        }
        P::iY => {
            if prati.has_antya('a') {
                add("4.1.95", p, t);
            }
        }
        P::Pak => {
            if prati.has_text_in(gana::NADADI) {
                add("4.1.99", p, t);
            }
        }
        P::yaY => {
            if prati.has_text_in(gana::GARGADI) {
                add("4.1.105", p, t);
            }
        }
        P::Qak => {
            if prati.has_tag(T::Stri) {
                add("4.1.120", p, t);
            }
        }
        P::Ga => {
            if prati.has_text("kzatra") {
                add("4.1.138", p, t);
            }
        }
        P::Ka => {
            if prati.has_text("kula") {
                add("4.1.139", p, t);
            }
        }
        P::Wak => {
            if prati.has_text_in(gana::REVATYADI) {
                add("4.1.146", p, t);
            } else {
                add("4.4.1", p, t);
            }
        }
        P::PiY => {
            if prati.has_tag(T::Vrddha) {
                add("4.1.157", p, t);
            }
        }
        P::Gac => {}
        P::Ca => {
            if prati.has_tag(T::Vrddha) {
                add("4.2.114", p, t);
            }
        }
        P::yat => {
            if prati.has_antya('u') || prati.has_antya('U') {
                add("5.1.2", p, t);
            }
        }
        P::tva | P::tal => {
            add("5.1.119", p, t);
        }
        P::matup => {
            add("5.2.94", p, t);
        }
        P::vini => {
            if prati.text.ends_with("as") || prati.has_u_in(&["mAyA", "meDA", "sraj"]) {
                add("5.2.121", p, t);
            }
        }
        P::tamap | P::izWan => {
            add("5.3.55", p, t);
        }
        P::tarap | P::Iyasun => {
            add("5.3.57", p, t);
        }
        P::akac => {
            if prati.has_tag(T::Sarvanama) {
                add("5.3.71", p, t);
            }
        }
        _ => {
            add_vibhakti_pratyayas(p, t);
        }
    }

    let added = p.terms().last()?.is_taddhita();
    Some(added)
}

/// Runs the rules that add a taddhita-pratyaya to a given pratipadika.
/// Returns whether a pratyaya was added.
pub fn run(p: &mut Prakriya, t: Taddhita) -> bool {
    try_add_taddhita(p, t).unwrap_or(false)
}
