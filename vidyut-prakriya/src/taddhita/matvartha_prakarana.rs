use crate::args::Agama as A;
use crate::args::Taddhita;
use crate::args::Taddhita::*;
use crate::args::TaddhitaArtha;
use crate::args::TaddhitaArtha::*;
use crate::core::operators as op;
use crate::core::Rule;
use crate::core::Rule::Varttika;
use crate::ganapatha as gana;
use crate::taddhita::utils::TaddhitaPrakriya;

/// Shorthand for contexts that model simple one-time rules.
fn with_context_simple(
    tp: &mut TaddhitaPrakriya,
    rule: impl Into<Rule>,
    values: &[&str],
    artha: TaddhitaArtha,
    taddhita: Taddhita,
) {
    let rule = rule.into();
    tp.with_context(artha, move |tp| {
        let prati = tp.prati();
        if prati.has_text_in(values) {
            tp.try_add(rule, taddhita);
        }
    });
}

pub fn run(tp: &mut TaddhitaPrakriya) {
    tp.with_context(Nimana, |tp| {
        let prati = tp.prati();
        if prati.is_sankhya() {
            // dvimaya, ...
            tp.try_add("5.2.47", mayaw);
        }
    });

    tp.with_context(Purana, |tp| {
        let prati = tp.prati();
        if prati.is_sankhya() {
            let i_prati = tp.i_prati;
            if prati.has_text("dvi") {
                // dvitIya
                tp.try_add("5.2.54", tIya);
            } else if prati.has_text("tri") {
                // tftIya
                tp.try_add_with("5.2.55", tIya, |p| p.set(i_prati, |t| t.set_text("tf")));
            } else {
                // ekadaSaH, ...
                let added = tp.try_add("5.2.48", qaw);
                if added {
                    op::insert_before("5.2.49", tp.p, tp.i_prati + 1, A::maw);
                }
            }
        }
    });

    tp.with_context(TadAsyaAstiAsmin, |tp| {
        let prati = tp.prati();
        // TODO: do these block matup?
        if prati.has_text_in(gana::VIMUKTA_ADI) {
            tp.try_add("5.2.61", aR);
        } else if prati.has_text_in(gana::GOSHADA_ADI) {
            tp.try_add("5.2.62", vun);
        }
    });

    tp.with_context(TatraKushala, |tp| {
        let prati = tp.prati();
        if prati.has_text("paTin") {
            tp.try_add("5.2.63", vun);
        } else if prati.has_text_in(gana::AKARSHA_ADI) {
            tp.try_add("5.2.64", kan);
        }
    });

    with_context_simple(tp, "5.2.65", &["Dana", "hiraRya"], TatraKama, kan);
    with_context_simple(tp, "5.2.67", &["udara"], TatraAdyuna, Wak);
    with_context_simple(tp, "5.2.68", &["sasya"], TatraParijata, kan);
    with_context_simple(tp, "5.2.69", &["aMSa"], Hari, kan);
    with_context_simple(tp, "5.2.70", &["tantra"], AciraApahrta, kan);
    with_context_simple(tp, "5.2.72", &["SIta", "uzRa"], Karin, kan);
    with_context_simple(tp, "5.2.75", &["pArSva"], Anvicchati, kan);

    tp.with_context(TadAsyaAstiAsmin, |tp| {
        let prati = tp.prati();

        if prati.has_text_in(gana::RASA_ADI) {
            tp.try_add("5.2.95", matup);
        } else if prati.has_antya('A') {
            tp.optional_try_add("5.2.96", lac);
        } else if prati.has_text_in(gana::SIDHMA_ADI) {
            tp.optional_try_add("5.2.97", lac);
        } else if prati.has_text_in(&["vatsa", "aMsa"]) {
            tp.optional_try_add("5.2.98", lac);
        } else if prati.has_text("Pena") {
            let code = "5.2.99";
            tp.try_add(code, lac);
            tp.try_add(code, ilac);
        } else if prati.has_text_in(gana::LOMA_ADI)
            || prati.has_text_in(gana::PAMA_ADI)
            || prati.has_text_in(gana::PICCHA_ADI)
        {
            let sub = if prati.has_text_in(gana::LOMA_ADI) {
                Sa
            } else if prati.has_text_in(gana::PAMA_ADI) {
                na
            } else {
                ilac
            };
            tp.try_add("5.2.100", sub);
        } else if prati.has_text_in(&["rajas", "kfzi", "Asuti", "parizad"]) {
            // rajasvala, ...
            tp.try_add("5.2.112", valac);
        } else if prati.has_text_in(gana::TUNDA_ADI) {
            let code = "5.2.117";
            tp.optional_try_add(code, ini);
            tp.optional_try_add(code, Wan);
            tp.optional_try_add(code, ilac);
        } else if prati.has_antya('a') {
            let code = "5.2.115";
            tp.optional_try_add(code, ini);
            tp.optional_try_add(code, Wan);
        }

        // BrAtfvala, ...
        tp.try_add(Varttika("5.2.112.1"), valac);

        let prati = tp.prati();
        if prati.ends_with("as") || prati.has_text_in(&["mAyA", "meDA", "sraj"]) {
            tp.try_add("5.2.121", vini);
        } else if prati.has_text("UrRA") {
            tp.try_add("5.2.123", yus);
        }

        // Base case.
        tp.try_add("5.2.94", matup);
    });
}
