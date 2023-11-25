use crate::args::{Linga, Vacana, Vibhakti};
use crate::core::Prakriya;
use crate::core::Tag as T;
use crate::core::Term;
use crate::it_samjna;

fn find_sup(vibhakti: Vibhakti, vacana: Vacana) -> &'static str {
    use Vacana::*;
    use Vibhakti::*;
    match (vibhakti, vacana) {
        (Prathama, Eka) => "su~",
        (Prathama, Dvi) => "O",
        (Prathama, Bahu) => "jas",
        (Dvitiya, Eka) => "am",
        (Dvitiya, Dvi) => "Ow",
        (Dvitiya, Bahu) => "Sas",
        (Trtiya, Eka) => "wA",
        (Trtiya, Dvi) => "ByAm",
        (Trtiya, Bahu) => "Bis",
        (Caturthi, Eka) => "Ne",
        (Caturthi, Dvi) => "ByAm",
        (Caturthi, Bahu) => "Byas",
        (Panchami, Eka) => "Nasi~",
        (Panchami, Dvi) => "ByAm",
        (Panchami, Bahu) => "Byas",
        (Sasthi, Eka) => "Nas",
        (Sasthi, Dvi) => "os",
        (Sasthi, Bahu) => "Am",
        (Saptami, Eka) => "Ni",
        (Saptami, Dvi) => "os",
        (Saptami, Bahu) => "sup",

        (Sambodhana, Eka) => "su~",
        (Sambodhana, Dvi) => "O",
        (Sambodhana, Bahu) => "jas",
    }
}

pub fn run(p: &mut Prakriya, linga: Linga, vibhakti: Vibhakti, vacana: Vacana) -> Option<()> {
    let sup = find_sup(vibhakti, vacana);
    let mut sup = Term::make_upadesha(sup);

    sup.add_tags(&[
        T::Pratyaya,
        T::Vibhakti,
        T::Sup,
        vibhakti.as_tag(),
        vacana.as_tag(),
    ]);

    p.add_tags(&[linga.as_tag(), vacana.as_tag()]);
    if vibhakti == Vibhakti::Sambodhana {
        p.add_tag(T::Sambodhana);
    }

    p.push(sup);
    p.step("4.1.2");

    let i = p.terms().len() - 1;
    it_samjna::run(p, i).ok()?;

    Some(())
}
