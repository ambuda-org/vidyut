//! Adhyaya 6.3 of the Ashtadhyayi concerns itself with changes caused by a following word
//! (*uttarpada*). For now, we keep those rule here.

use crate::args::Artha;
use crate::args::TaddhitaArtha;
use crate::core::operators as op;
use crate::core::Prakriya;
use crate::core::Tag as T;
use crate::core::Term;
use crate::sounds as al;
use crate::sounds::{s, Set};
use lazy_static::lazy_static;

lazy_static! {
    static ref AA: Set = s("a");
    static ref IK: Set = s("ik");
    static ref AC: Set = s("ac");
}

/// Runs rules that apply when an uttarapada is present.
pub fn run(p: &mut Prakriya) -> Option<()> {
    p.debug("==== uttarapade ====");
    let last = p.terms().last()?;

    if p.terms().len() == 2
        && p.has(0, |t| !t.is_ekac() && t.ends_with("I"))
        && (last.has_tag(T::Gha) || last.has_u_in(&["rUpap", "kalpap"]))
    {
        // TODO; check for bhASitapuMsvat
        p.run_at("6.3.43", 0, |t| {
            // Rule only applies for NI, so just change `I`.
            t.set_antya("i");
        });
    }

    // TODO: calculate this properly.
    let i_purva = 0;
    let i_uttara = p.find_next_where(i_purva, |t| !t.is_empty())?;
    let purva = p.get(i_purva)?;
    let uttara = p.get(i_uttara)?;

    if purva.has_text("mahat")
        && (p.has_tag_in(&[T::Karmadharaya, T::Bahuvrihi]) || uttara.has_text("jAtIya"))
    {
        p.run_at("6.3.46", i_purva, |t| t.set_antya("A"));
    } else if purva.has_text("hfdaya")
        && (uttara.has_u_in(&["yat", "aR"]) || uttara.has_text_in(&["leKa", "lAsa"]))
    {
        // hflleKa, ...
        p.run_at("6.3.50", i_purva, |t| t.set_text("hfd"));
    } else if purva.has_text("pAda")
        && uttara.has_u("yat")
        && !p.has_artha(Artha::Taddhita(TaddhitaArtha::Tadarthye))
    {
        p.run_at("6.3.53", i_purva, |t| t.set_text("pad"));
    }

    let purva = p.get(i_purva)?;
    let uttara = p.get(i_uttara)?;
    if purva.is_sarvanama() && uttara.has_text_in(&["dfS"]) {
        p.run_at("6.3.91", i_purva, |t| t.set_antya("A"));
    } else if !purva.is_avyaya() && p.find_last(T::Kit).is_some() {
        let ajanta = al::is_ac(purva.antya()?);
        let i_khit = p.find_last_where(|t| t.has_tag(T::Kit))?;
        debug_assert!(i_khit > 0);

        if purva.has_text_in(&["vAc", "pur"]) {
            // vAcaMyama, purandara
            p.run("6.3.69", |p| {
                let mut am = Term::make_upadesha("am");
                am.add_tags(&[T::Pratyaya, T::Vibhakti, T::Sup, T::V2, T::Pada]);
                p.insert_after(i_purva, am);
                p.set(i_purva, |t| t.remove_tag(T::Pada));
            });
        } else if purva.num_vowels() == 1 && purva.has_antya(&*AC) && !purva.has_antya(&*AA) {
            p.run("6.3.68", |p| {
                let mut am = Term::make_upadesha("am");
                am.add_tags(&[T::Pratyaya, T::Vibhakti, T::Sup, T::V2, T::Pada]);
                p.insert_after(i_purva, am);
            });
        } else if purva.has_u_in(&["arus", "dvizat"]) || ajanta {
            if al::is_dirgha(purva.antya()?) && !purva.is_avyaya() {
                let sub = al::to_hrasva(purva.antya()?)?;
                p.run_at("6.3.66", i_purva, |t| {
                    t.set_antya(&sub.to_string());
                });
            }

            // aruntuda, dvizantapa
            p.run_at("6.3.67", i_purva, |t| {
                op::mit("m")(t);
                // HACK: add `pada` for m -> M (8.3.23).
                t.add_tag(T::Pada);
            });
        }
    }
    Some(())
}

pub fn run_after_guna(p: &mut Prakriya) -> Option<()> {
    let i_purva = 0;
    let i_uttara = p.find_next_where(i_purva, |t| !t.is_empty())?;
    let purva = p.get(i_purva)?;
    let uttara = p.get(i_uttara)?;

    if purva.is_upasarga() {
        if p.has(i_uttara + 1, |t| t.has_u("GaY")) {
            // rule is "bahulam"
            if purva.has_u("ni") && uttara.has_u_in(&["vfN", "vfY"]) {
                // nIvAra
                p.run_at("6.3.122", i_purva, |t| t.set_antya("I"));
            }
        } else if purva.has_antya(&*IK) {
            if uttara.has_text("kAS") && p.has(i_uttara + 1, |t| t.has_u("ac")) {
                // nIkASa, vIkASa, anUkASa
                let sub = al::to_dirgha(purva.antya()?)?;
                p.run_at("6.3.123", i_purva, |t| t.set_antya(&sub.to_string()));
            } else if uttara.has_tag(T::Ghu) && uttara.has_text("t") {
                // nItta, vItta, parItta
                let sub = al::to_dirgha(purva.antya()?)?;
                p.run_at("6.3.124", i_purva, |t| t.set_antya(&sub.to_string()));
            }
        }
    } else if uttara.has_text("citi") && p.has(i_uttara + 1, |t| t.has_u("kap")) {
        // citIka
        p.run_at("6.3.125", i_uttara, |t| t.set_antya("I"));
    }

    Some(())
}
