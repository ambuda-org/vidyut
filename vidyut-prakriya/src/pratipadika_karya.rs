use crate::args::{BasicPratipadika, Stri, Upasarga};
use crate::core::operators as op;
use crate::core::Prakriya;
use crate::core::{Morph, PrakriyaTag as PT, Tag as T, Term};
use crate::sounds as al;

/// FOO
pub fn add_basic(p: &mut Prakriya, basic: &BasicPratipadika) {
    let mut base = match basic.text.0.parse::<Upasarga>() {
        Ok(u) => u.into(),
        _ => Term::make_pratipadika(&basic.text.0),
    };

    // HACK: old implemenation of `Pratipadika` has these tags, so keep them here for consistency
    // for now.
    if basic.is_nyap {
        base.add_tags(&[T::Stri, T::StriNyap]);
    }
    if basic.is_avyaya {
        base.add_tags(&[T::Avyaya]);
    }
    p.push(base);

    // HACK: Add a dummy pratyaya so rules pass.
    // TODO: see if we can delete `is_nyap`.
    if basic.is_nyap {
        let last = p.terms().last();
        let stri = if let Some(t) = last {
            match t.antya() {
                Some('I') => Stri::NIp,
                Some('U') => Stri::UN,
                _ => Stri::wAp,
            }
        } else {
            Stri::wAp
        };
        let mut nyap = Term::make_upadesha(stri.as_str());
        nyap.add_tags(&[T::Pratyaya, T::StriNyap, T::Stri]);
        nyap.set_text("");
        nyap.morph = Morph::Stri(stri);
        p.push(nyap);
    }
}

/// Runs rurles specific to napumsaka-pratipadikas.
pub fn run_napumsaka_rules(p: &mut Prakriya) -> Option<()> {
    if p.has_tag(PT::Napumsaka) {
        let i_last_not_empty = p.find_last_where(|t| !t.is_empty() && !t.is_sup())?;
        let t = p.get_if(i_last_not_empty, |t| !t.is_avyaya())?;
        let sub = al::to_hrasva(t.antya()?)?;
        if !t.has_antya(sub) {
            p.run_at("1.2.47", i_last_not_empty, op::antya_char(&sub));
        }
    }
    None
}
