use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds::{s, Set};
use crate::stem_gana::{PRATHAMA_ADI, PURVA_ADI, SARVA_ADI, USES_DATARA_DATAMA};
use crate::tag::Tag as T;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
}

fn try_run_for_pratipadika(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Pratipadika)?;
    let prati = p.get(i)?;
    let sup = p.get(i + 1)?;

    let ii_uu = prati.has_antya('I') || prati.has_antya('U');
    let i_u = prati.has_antya('i') || prati.has_antya('u');
    if prati.has_text_in(SARVA_ADI) || prati.has_text_in(USES_DATARA_DATAMA) {
        let mut sarvanama = true;
        if prati.has_text_in(PURVA_ADI) && sup.has_u("jas") {
            sarvanama = !p.op_optional("1.1.34", |_| {});
        }
        if sarvanama {
            p.op_term("1.1.27", i, op::add_tag(T::Sarvanama));
        }
    } else if prati.has_text_in(PRATHAMA_ADI) && sup.has_u("jas") {
        p.op_optional("1.1.33", op::t(i, op::add_tag(T::Sarvanama)));
    } else if ii_uu && p.has_tag(T::Stri) {
        p.op_term("1.4.3", i, op::add_tag(T::Nadi));
    } else if i_u && !prati.has_text("saKi") {
        let mut is_nadi = false;
        if sup.has_tag(T::Nit) && p.has_tag(T::Stri) {
            is_nadi = p.op_optional("1.4.6", op::t(i, op::add_tag(T::Nadi)));
        }
        if !is_nadi {
            p.op_term("1.4.7", i, op::add_tag(T::Ghi));
        }
    }

    Some(())
}

/// Runs rules that add the "pada"-samjna to various terms.
///
/// NOTE: Technically, `pada` applies to the matched term as well that all that precedes it. But
/// since this is difficult for us to model right now, just use the last term.
pub fn try_run_for_pada(p: &mut Prakriya) -> Option<()> {
    let n = p.terms().len();
    for i in 0..n {
        let term = p.get(i)?;
        if term.has_tag_in(&[T::Sup, T::Tin]) {
            p.op_term("1.4.13", i, op::add_tag(T::Pada));
        } else {
            let next = p.get(i + 1)?;
            if next.has_tag(T::Sup) && !next.has_tag(T::Sarvanamasthana) {
                if next.has_adi('y') || next.has_adi(&*AC) {
                    p.op_term("1.4.18", i, op::add_tag(T::Bha));
                } else {
                    p.op_term("1.4.17", i, op::add_tag(T::Pada));
                }
            }
        }
    }

    Some(())
}

fn try_run_for_sup(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Sup)?;

    if p.has_tag(T::Sambodhana) {
        p.op_term("2.3.48", i, op::add_tag(T::Amantrita));
        if p.has_tag(T::Ekavacana) {
            p.op_term("2.3.49", i, op::add_tag(T::Sambuddhi));
        }
    }

    let sup = p.get(i)?;
    // For 1.1.42, see the `sup_adesha` module.
    if sup.has_u_in(&["su~", "O", "jas", "am", "Ow"]) && !p.has_tag(T::Napumsaka) {
        p.op_term("1.1.43", i, op::add_tag(T::Sarvanamasthana));
    }

    Some(())
}

fn try_run_for_dhatu_pratyaya(p: &mut Prakriya, i: usize) -> Option<()> {
    // TODO: add other exclusions here.
    let pratyaya = p.get_if(i, |t| !t.has_tag_in(&[T::Sup, T::Taddhita]))?;

    let add_sarva = op::t(i, op::add_tag(T::Sarvadhatuka));
    let add_ardha = op::t(i, op::add_tag(T::Ardhadhatuka));

    if pratyaya.is_pratyaya() {
        if pratyaya.has_lakshana("li~w") {
            p.op("3.4.115", add_ardha);
        } else if pratyaya.has_lakshana("li~N") && p.has_tag(T::Ashih) {
            p.op("3.4.116", add_ardha);
        } else if pratyaya.has_tag_in(&[T::Tin, T::Sit]) {
            if !pratyaya.has_tag(T::Sarvadhatuka) {
                p.op("3.4.113", add_sarva);
            }
        } else {
            // Suffixes introduced before "dhAtoH" are not called ArdhadhAtuka.
            // So they will not cause guNa and will not condition iT-Agama.
            if pratyaya.has_tag(T::FlagNoArdhadhatuka) {
                // do nothing
            } else if !pratyaya.is_empty() && !pratyaya.has_tag(T::Ardhadhatuka) {
                // Check `is_empty` to avoid including luk, etc.
                p.op("3.4.114", add_ardha);
            }
        }
    }

    Some(())
}

fn try_run_for_dhatu(p: &mut Prakriya) -> Option<()> {
    p.find_first_where(|t| t.is_dhatu())?;

    for i in 0..p.terms().len() {
        try_run_for_dhatu_pratyaya(p, i);
    }

    Some(())
}

pub fn run(p: &mut Prakriya) {
    try_run_for_dhatu(p);
    try_run_for_pratipadika(p);
    try_run_for_sup(p);
}
