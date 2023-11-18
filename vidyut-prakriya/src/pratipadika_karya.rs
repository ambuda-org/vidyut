use crate::args::Pratipadika;
use crate::args::SamasaArgs;
use crate::args::Vibhakti;
use crate::core::operators as op;
use crate::core::Prakriya;
use crate::core::Tag as T;
use crate::core::Term;
use crate::sounds as al;

/// Appends a pratipadika to the prakriya.
///
/// Scope: subantas, taddhitas
pub fn add_one(p: &mut Prakriya, pratipadika: &Pratipadika) {
    p.extend(pratipadika.terms());

    // HACK: Add a dummy pratyaya so rules pass.
    // TODO: see if we can delete `is_nyap`.
    if pratipadika.needs_nyap() {
        let sub = if pratipadika
            .terms()
            .last()
            .map_or(false, |t| t.has_antya('I'))
        {
            "NIp"
        } else {
            "wAp"
        };
        let mut nyap = Term::make_upadesha(sub);
        nyap.add_tags(&[T::Pratyaya, T::StriNyap]);
        nyap.set_text("");
        p.push(nyap);
    }

    add_samjnas(p);
}

/// Appends multiple pratipadikas to the prakriya.
///
/// Scope: samasas
pub fn add_all(p: &mut Prakriya, args: &SamasaArgs) {
    // Initialize the prakriya by adding all items with dummy sup-pratyayas in between.
    for pada in args.padas() {
        for t in pada.pratipadika().terms() {
            p.push(t.clone());
        }
        if pada.is_avyaya() {
            p.terms_mut().last_mut().expect("ok").add_tag(T::Avyaya);
        }
        p.push(make_sup_pratyaya(pada.vibhakti()));
    }
    // Remove the trailing sup-pratyaya.
    p.terms_mut().pop();

    add_samjnas(p);
}

/// Assigns the pratipadika-samjna to all matching terms in the prakriya.
fn add_samjnas(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let t = p.get(i)?;

        if t.is_krt() || t.is_taddhita() || t.is_samasa() {
            p.add_tag_at("1.2.46", i, T::Pratipadika);
        } else if !t.is_dhatu() && !t.is_pratyaya() && !t.is_agama() && !t.is_abhyasa() {
            // 1.2.45 specifies "arthavat", so exclude meaningless terms (agamas and abhyasas).
            // TODO: is there anything else that's not arthavat?
            p.add_tag_at("1.2.45", i, T::Pratipadika);
        }
    }

    Some(())
}

/// Runs rurles specific to napumsaka-pratipadikas.
pub fn run_napumsaka_rules(p: &mut Prakriya) -> Option<()> {
    if p.has_tag(T::Napumsaka) {
        let i_last_not_empty = p.find_last_where(|t| !t.is_empty() && !t.is_sup())?;
        let t = p.get(i_last_not_empty)?;
        let sub = al::to_hrasva(t.antya()?)?;
        if !t.has_antya(sub) {
            p.run_at("1.2.47", i_last_not_empty, op::antya(&sub.to_string()));
        }
    }
    None
}

/// Creates a dummy sup-pratyaya.
///
/// Scope: samasas
fn make_sup_pratyaya(vibhakti: Vibhakti) -> Term {
    use Vibhakti::*;
    let (u, vibhakti) = match vibhakti {
        Prathama | Sambodhana => ("su~", T::V1),
        Dvitiya => ("am", T::V2),
        Trtiya => ("wA", T::V3),
        Caturthi => ("Ne", T::V4),
        Panchami => ("Nasi", T::V5),
        Sasthi => ("Nas", T::V6),
        Saptami => ("Ni", T::V7),
    };

    let mut su = Term::make_upadesha(u);
    su.set_text("");
    su.add_tags(&[T::Pratyaya, T::Sup, T::Vibhakti, T::Pada, vibhakti]);
    su
}
