use crate::args::{Linga, Pratipadika, SubantaArgs};
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::tag::Tag as T;
use crate::term::Term;

pub fn run(p: &mut Prakriya, pratipadika: &Pratipadika, args: &SubantaArgs) -> Option<()> {
    // The prAtipadika enters the prakriyA
    let mut term = Term::make_upadesha(pratipadika.text());
    if pratipadika.is_nyap() {
        term.add_tag(T::StriNyap);
    }
    if pratipadika.is_dhatu() {
        term.add_tag(T::Dhatu);
    }
    if pratipadika.is_pratyaya() {
        term.add_tag(T::Pratyaya);
    }

    p.push(term);

    // Add samjnas
    p.op_term("1.2.45", 0, |t| {
        t.add_tag(T::Pratipadika);
    });
    p.add_tag(args.linga().as_tag());

    if args.linga() == Linga::Napumsaka {
        let prati = p.get(0)?;
        let sub = al::to_hrasva(prati.antya()?)?;
        if !prati.has_antya(sub) {
            p.op_term("1.2.47", 0, op::antya(&sub.to_string()));
        }
    }

    Some(())
}
