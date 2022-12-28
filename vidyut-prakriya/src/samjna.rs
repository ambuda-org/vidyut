use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::stem_gana::{PRATHAMA_ADI, PURVA_ADI, SARVA_ADI, USES_DATARA_DATAMA};
use crate::tag::Tag as T;

fn try_run_for_pratipadika(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Pratipadika)?;
    let prati = p.get(i)?;
    let sup = p.get(i + 1)?;

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
    } else if (prati.has_antya('I') || prati.has_antya('U')) && prati.has_tag(T::Stri) {
        p.op_term("1.4.3", i, op::add_tag(T::Nadi));
    } else if (prati.has_antya('i') || prati.has_antya('u')) && !prati.has_text("saKi") {
        p.op_term("1.4.7", i, op::add_tag(T::Ghi));
    }

    Some(())
}

fn try_run_for_sup(p: &mut Prakriya) -> Option<()> {
    let i = p.terms().len() - 1;
    let _sup = p.get_if(i, |t| t.has_tag(T::Sup))?;

    if p.has_tag(T::Sambodhana) {
        p.op_term("2.3.48", i, op::add_tag(T::Amantrita));
        if p.has_tag(T::Ekavacana) {
            p.op_term("2.3.49", i, op::add_tag(T::Sambuddhi));
        }
    }

    let anga = p.get(i - 1)?;
    let sup = p.get_if(i, |t| t.has_tag(T::Sup))?;
    if sup.has_u("Si") {
        p.op_term("1.1.42", i, op::add_tag(T::Sarvanamasthana));
    } else if sup.has_u_in(&["su~", "O", "jas", "am", "Ow"]) && !anga.has_tag(T::Napumsaka) {
        p.op_term("1.1.43", i, op::add_tag(T::Sarvanamasthana));
    }

    Some(())
}

fn try_run_for_dhatu_pratyaya(p: &mut Prakriya, i: usize) -> Option<()> {
    let add_sarva = op::t(i, op::add_tag(T::Sarvadhatuka));
    let add_ardha = op::t(i, op::add_tag(T::Ardhadhatuka));

    let pratyaya = p.get(i)?;

    if pratyaya.has_tag(T::Pratyaya) {
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
    p.find_first_where(|t| t.has_tag(T::Dhatu))?;

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
