use crate::args::{Linga, Sup, Vacana, Vibhakti};
use crate::core::Prakriya;
use crate::core::{PrakriyaTag as PT, Term};
use crate::it_samjna;

pub fn run(p: &mut Prakriya, linga: Linga, vibhakti: Vibhakti, vacana: Vacana) -> Option<()> {
    let s = Sup::from_args(vibhakti, vacana);
    let mut t: Term = s.into();
    t.add_tags(&[vibhakti.as_tag(), vacana.as_tag()]);

    p.add_tags(&[linga.as_tag().into(), vacana.as_tag().into()]);
    if vibhakti == Vibhakti::Sambodhana {
        p.add_tag(PT::Sambodhana);
    }

    p.push(t);
    p.step("4.1.2");

    let i = p.terms().len() - 1;
    it_samjna::run(p, i).ok()?;

    Some(())
}
