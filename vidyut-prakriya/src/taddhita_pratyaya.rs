/*!
Runs rules that add taddhita-pratyayas after a prAtipadika..

*Status: experimental.*
*/

use crate::args::Taddhita;
use crate::it_samjna;
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

fn try_add_taddhita(p: &mut Prakriya, t: Taddhita) -> Option<bool> {
    let prati = p.terms().last()?;

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
        P::akac => {
            if prati.has_tag(T::Sarvanama) {
                add("5.3.7", p, t);
            }
        }
        P::tamap | P::izWan => {
            add("5.3.55", p, t);
        }
        P::tarap | P::Iyasun => {
            add("5.3.57", p, t);
        }
    }

    Some(true)
}

/// Runs the rules that add a taddhita-pratyaya to a given pratipadika.
/// Returns whether a pratyaya was added.
pub fn run(p: &mut Prakriya, t: Taddhita) -> bool {
    try_add_taddhita(p, t).unwrap_or(false)
}
