/*!
Implements the taddhita rules in the "prAg ivAt kaH" section of pada 5.3, as well as some rules
immediately before it.

(5.3.27 - 5.3.95)
*/
use crate::args::Taddhita::*;
use crate::args::TaddhitaArtha::*;
use crate::core::operators as op;
use crate::core::Tag;
use crate::taddhita::utils::TaddhitaPrakriya;

fn try_base_cases(tp: &mut TaddhitaPrakriya, _rule: &'static str) {
    let prati = tp.prati();
    if prati.is_sarvanama() {
        // TODO: this is incorrect.
        tp.try_add("5.3.71", akac);
    } else {
        tp.try_add("5.3.70", ka);
    }
}

pub fn run(tp: &mut TaddhitaPrakriya) -> Option<()> {
    tp.with_context(DigDeshaKala, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["dakziRa", "uttara"]) {
            tp.try_add("5.3.28", atasuc);
        } else if prati.has_text_in(&["para", "avara"]) {
            tp.optional_try_add("5.3.29", atasuc);
        }
        if !tp.has_taddhita {
            tp.try_add("5.3.27", astAti);
        }
    });

    let prati = tp.prati();
    if prati.has_tag(Tag::Sankhya) {
        tp.try_add("5.3.42", DA);
    }

    let last = tp.p.terms().last().expect("present");
    if last.is_taddhita() && last.is(DA) {
        let i_last = tp.p.terms().len() - 1;
        let prati = tp.prati();
        if prati.has_text("eka") {
            op::optional_adesha("5.3.44", tp.p, i_last, "Dyamu~Y");
        } else if prati.has_text_in(&["dvi", "tri"]) {
            let done = op::optional_adesha("5.3.45", tp.p, i_last, "Damu~Y");
            if !done {
                op::optional_adesha("5.3.46", tp.p, i_last, "eDAc");
            }
        }
    }

    tp.try_add("5.3.47", pASap);
    let prati = tp.prati();
    if prati.has_text("eka") {
        tp.try_add("5.3.52", Akinic);
    }

    tp.try_add("5.3.53", caraw);

    let code = "5.3.54";
    tp.try_add(code, caraw);
    tp.try_add(code, rUpya);

    let code = "5.3.55";
    tp.try_add(code, tamap);
    tp.try_add(code, izWan);

    let code = "5.3.57";
    tp.try_add(code, tarap);
    tp.try_add(code, Iyasun);

    let last = tp.p.terms().last()?;
    if last.is(Iyasun) || last.is(izWan) {
        let prati = tp.prati();
        if prati.has_text("praSasya") {
            // Sreyas, SrezWa
            let done =
                tp.p.optional_run_at("5.3.60", tp.i_prati, |t| t.set_text("Sra"));
            if !done {
                // jyAyas, jyezWa
                tp.p.run_at("5.3.61", tp.i_prati, |t| t.set_text("jya"));
            }
        } else if prati.has_text("vfdDa") {
            // jyAyas, jyezWa
            // This is optional so that 6.4.157 can produce varzIyas and varzizWa.
            tp.p.optional_run_at("5.3.62", tp.i_prati, |t| t.set_text("jya"));
        } else if prati.has_text_in(&["antika", "bAQa"]) {
            let sub = if prati.has_text("antika") {
                "neda"
            } else {
                "sADa"
            };
            tp.p.run_at("5.3.63", tp.i_prati, |t| t.set_text(sub));
        } else if prati.has_text_in(&["yuvan", "alpa"]) {
            // kanIyas, kanizWa
            tp.p.optional_run_at("5.3.64", tp.i_prati, |t| t.set_text("kan"));
        }
    }

    // vaiyAkaraRarUpa

    // 5.3.66 is stated near 5.4.41 -- see that rule for details.

    // pawukalpa
    let code = "5.3.67";
    tp.try_add(code, kalpap);
    tp.try_add(code, deSya);
    tp.try_add(code, deSIyar);

    tp.try_prepend("5.3.68", bahuc);

    // 5.3.69 is stated near 5.4.3 -- see that rule for details.

    // --------------------
    // 5.3.70 prAg ivAt kaH
    // --------------------

    tp.with_context(Ajnate, |tp| {
        try_base_cases(tp, "5.3.73");
    });

    tp.with_context(Kutsite, |tp| {
        tp.optional_try_add("5.3.75", kan);
        try_base_cases(tp, "5.3.74");
    });

    tp.with_context(Anukampayam, |tp| {
        try_base_cases(tp, "5.3.76");
    });

    tp.with_context(Alpe, |tp| {
        try_base_cases(tp, "5.3.85");
    });

    tp.with_context(Hrasve, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["kuwI", "SamI", "SuRqA"]) {
            // kuwIra, SamIra, SuRqAra
            tp.try_add("5.3.88", ra);
        } else if prati.has_text("kutU") {
            // kuwupa
            tp.try_add("5.3.89", qupac);
        } else if prati.has_text_in(&["kAsU", "goRI"]) {
            // kAsUtarI, goRItarI
            tp.try_add("5.3.90", zwarac);
        }
        tp.optional_try_add("5.3.87", kan);
        if !tp.had_match {
            try_base_cases(tp, "5.3.86");
        }
    });

    tp.with_context(Tanutve, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["vatsa", "ukzan", "aSva", "fzaBa"]) {
            tp.try_add("5.3.91", zwarac);
        }
    });

    tp.with_context(DvayorEka, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["kim", "yad", "tad"]) {
            tp.try_add("5.3.92", qatarac);
        } else if prati.has_text("eka") {
            tp.try_add("5.3.94", qatarac);
        }
    });

    tp.with_context(BahunamEka, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["kim", "yad", "tad"]) {
            tp.try_add("5.3.93", qatamac);
        } else if prati.has_text("eka") {
            // Also appears in DvayorEka above.
            tp.try_add("5.3.94", qatarac);
        }
    });

    tp.with_context(Avakshepane, |tp| {
        tp.try_add("5.3.95", kan);
    });

    None
}
