use crate::args::{Linga, Sup, Vacana, Vibhakti};
use crate::core::Prakriya;
use crate::core::Tag as T;
use crate::core::{Morph, Term};
use crate::it_samjna;

pub fn run(p: &mut Prakriya, linga: Linga, vibhakti: Vibhakti, vacana: Vacana) -> Option<()> {
    let s = Sup::from_args(vibhakti, vacana);
    let mut t = Term::make_upadesha(s.as_str());

    t.add_tags(&[
        T::Pratyaya,
        T::Vibhakti,
        T::Sup,
        vibhakti.as_tag(),
        vacana.as_tag(),
    ]);
    t.morph = Morph::Sup(s);

    p.add_tags(&[linga.as_tag(), vacana.as_tag()]);
    if vibhakti == Vibhakti::Sambodhana {
        p.add_tag(T::Sambodhana);
    }

    p.push(t);
    p.step("4.1.2");

    let i = p.terms().len() - 1;
    it_samjna::run(p, i).ok()?;

    Some(())
}
