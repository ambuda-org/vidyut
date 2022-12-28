use crate::args::SubantaArgs;
use crate::prakriya::Prakriya;
use crate::tag::Tag as T;
use crate::term::Term;

pub fn run(p: &mut Prakriya, pratipadika: &str, args: &SubantaArgs) {
    // The prAtipadika enters the prakriyA
    let pratipadika = Term::make_upadesha(pratipadika);
    p.push(pratipadika);

    // Add samjnas
    p.op_term("1.2.45", 0, |t| {
        t.add_tag(T::Pratipadika);
    });
    p.add_tag(args.linga().as_tag());
}
