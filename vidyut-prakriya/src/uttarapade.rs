//! Adhyaya 6.3 of the Ashtadhyayi concerns itself with changes caused by a following word
//! (*uttarpada*). For now, we keep those rule here.

use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AA: Set = s("a");
    static ref AC: Set = s("ac");
}

/// Runs rules that apply when an uttarapada is present.
pub fn run(p: &mut Prakriya) -> Option<()> {
    let last = p.terms().last()?;

    if p.terms().len() == 2
        && p.has(0, |t| !t.is_ekac() && t.ends_with("I"))
        && (last.has_tag(T::Gha) || last.has_u_in(&["rUpap", "kalpap"]))
    {
        // TODO; check for bhASitapuMsvat
        p.op_term("6.3.43", 0, |t| {
            // Rule only applies for NI, so just change `I`.
            t.set_antya("i");
        });
    }

    // TODO: calculate this properly.
    let i_purva = 0;
    let i_uttara = p.find_next_where(i_purva, |t| !t.is_empty())?;
    let purva = p.get(i_purva)?;
    let uttara = p.get(i_uttara)?;

    if purva.is_sarvanama() && uttara.has_text_in(&["dfS"]) {
        p.op_term("6.3.91", i_purva, |t| t.set_antya("A"));
    } else if !purva.is_avyaya() {
        let ajanta = al::is_ac(purva.antya()?);
        let i_khit = p.find_last_where(|t| t.has_tag(T::Kit))?;
        debug_assert!(i_khit > 0);

        if purva.has_text_in(&["vAc", "pur"]) {
            // vAcaMyama, purandara
            p.op("6.3.69", |p| {
                let mut am = Term::make_upadesha("am");
                am.add_tags(&[T::Pratyaya, T::Vibhakti, T::Sup, T::V2, T::Pada]);
                p.insert_after(i_purva, am);
                p.set(i_purva, |t| t.remove_tag(T::Pada));
            });
        } else if purva.num_vowels() == 1 && purva.has_antya(&*AC) && !purva.has_antya(&*AA) {
            p.op("6.3.68", |p| {
                let mut am = Term::make_upadesha("am");
                am.add_tags(&[T::Pratyaya, T::Vibhakti, T::Sup, T::V2, T::Pada]);
                p.insert_after(i_purva, am);
            });
        } else if purva.has_u_in(&["arus", "dvizat"]) || ajanta {
            if al::is_dirgha(purva.antya()?) && !purva.is_avyaya() {
                let sub = al::to_hrasva(purva.antya()?)?;
                p.op_term("6.3.66", i_purva, |t| {
                    t.set_antya(&sub.to_string());
                });
            }

            // aruntuda, dvizantapa
            p.op_term("6.3.67", i_purva, |t| {
                op::mit("m")(t);
                // HACK: add `pada` for m -> M (8.3.23).
                t.add_tag(T::Pada);
            });
        }
    }

    Some(())
}
