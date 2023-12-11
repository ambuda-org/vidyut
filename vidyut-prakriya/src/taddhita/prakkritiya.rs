/*!
Implements the taddhita rules in the "prAk krItAc CaH" section of pada 5.1.

(5.1.1 - 5.1.17)
*/
use crate::args::Taddhita::*;
use crate::args::TaddhitaArtha::*;
use crate::core::Rule::Varttika;
use crate::ganapatha as gana;
use crate::taddhita::utils::TaddhitaPrakriya;

fn try_base_cases(tp: &mut TaddhitaPrakriya, _rule: &'static str) {
    // TODO: use `_rule` as well -- this should be simultaneous application.
    let prati = tp.prati();
    if prati.has_text("kambala") {
        tp.optional_try_add("5.1.3", yat);
    } else if prati.has_text_in(&["AmikzA", "puroqASa"]) || prati.has_text_in(gana::APUPA_ADI) {
        tp.optional_try_add("5.1.4", yat);
    }

    let prati = tp.prati();
    if prati.has_antya('u') || prati.has_antya('U') || prati.has_text_in(gana::GAVADI) {
        tp.try_add("5.1.2", yat);
    } else {
        tp.try_add("5.1.1", Ca);
    }
}

pub fn run(tp: &mut TaddhitaPrakriya) {
    let prati = tp.prati();
    if prati.has_text("kambala") {
        tp.optional_try_add("5.1.3", yat);
    }

    tp.with_context(TasmaiHitam, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["danta", "kaRWa", "ozWa", "nABi", "nAsikA"]) {
            // TODO: support the class, not just specific examples.
            tp.try_add("5.1.6", yat);
            if tp.prati().has_text("nAsikA") {
                tp.p.run_at(Varttika("5.1.6.1"), tp.i_prati, |t| t.set_text("nas"));
            }
        } else if prati.has_text_in(&["Kala", "yava", "mAza", "tila", "vfza", "brahman"]) {
            tp.try_add("5.1.7", yat);
        } else if prati.has_text_in(&["aja", "avi"]) {
            tp.try_add("5.1.8", Tyan);
        } else if prati.has_text_in(&["Atman", "viSvajana"]) || prati.ends_with("Boga") {
            tp.try_add("5.1.9", Ka);
        } else if prati.has_text_in(&["sarva", "puruza"]) {
            let sub = if prati.has_text("sarva") { Ra } else { QaY };
            tp.try_add("5.1.10", sub);
        } else if prati.has_text_in(&["mARava", "caraka"]) {
            tp.try_add("5.1.11", KaY);
        } else {
            try_base_cases(tp, "5.1.5");
        }
    });

    tp.with_context(TadarthamVikrtehPrakrtau, |tp| {
        let prati = tp.prati();
        let pratipadika = tp.nyap_pratipadika();
        if pratipadika.has_text_in(&["Cadis", "upaDi", "bAli"]) {
            tp.try_add("5.1.13", QaY);
        } else if prati.has_text_in(&["fzaBa", "upAnah"]) {
            tp.try_add("5.1.14", Yya);
        } else if prati.has_text_in(&["varDrI", "varatrA"]) {
            // HACK: The rule states "carman," i.e. words referring to a skin or leather. For now,
            // use the two examples found in the Kashika Vrtti.
            tp.try_add("5.1.15", aY);
        } else {
            // aNgArIya, ...
            try_base_cases(tp, "5.1.12");
        }
    });

    tp.with_context(TadAsyaTadAsminSyat, |tp| {
        let prati = tp.prati();
        if prati.has_text("pariKA") {
            tp.try_add("5.1.17", QaY);
        } else {
            try_base_cases(tp, "5.1.16");
        }
    });
}
