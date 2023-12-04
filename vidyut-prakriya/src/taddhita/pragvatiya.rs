/*!
Implements the taddhita rules in the "prAg vatez Wan" section of pada 5.1.

(5.1.18 - 5.1.114)
*/
use crate::args::Taddhita;
use crate::args::TaddhitaArtha::*;
use crate::core::Tag as T;
use crate::ganapatha as gana;
use crate::taddhita::utils::TaddhitaPrakriya;
use crate::Rule::Varttika;

fn try_base_cases(tp: &mut TaddhitaPrakriya, _code: &'static str) {
    use Taddhita as P;
    tp.try_add("5.1.18", P::WaY);
}

/// Overrides WaY (5.1.18) up to 5.1.63 inclusive.
fn try_base_cases_arhiya(tp: &mut TaddhitaPrakriya, code: &'static str) {
    use Taddhita as P;
    let prati = tp.prati();

    if !prati.has_text("gopucCa") && !prati.is_sankhya() {
        tp.try_add("5.1.19", P::Wak);
    } else if prati.has_text_in(gana::NISHKA_ADI) && !prati.is_samasa() {
        tp.try_add("5.1.20", P::Wak);
    } else if prati.has_text("Sata") && !prati.is_samasa() {
        // Satika, Satya
        let code = "5.1.21";
        tp.try_add(code, P::Wan);
        tp.try_add(code, P::yat);
    } else if prati.has_text("kaMsa") {
        tp.try_add("5.1.25", P::wiWan);
    } else if prati.has_text("SUrpa") {
        tp.optional_try_add("5.1.26", P::aY);
    } else if prati.has_text_in(&["SatamAna", "viMSatika", "sahasra", "vasana"]) {
        tp.optional_try_add("5.1.27", P::aR);
    } else if prati.is_sankhya() && !prati.has_prefix_in(&["ti", "Sat"]) {
        tp.try_add("5.1.22", P::kan);
    }

    try_base_cases(tp, code);
}

pub fn run(tp: &mut TaddhitaPrakriya) {
    use Taddhita::*;

    tp.with_context(TenaKritam, |tp| {
        let prati = tp.prati();
        if prati.has_tag(T::Sankhya) {
            tp.try_add("5.1.22", kan);
        } else if prati.has_text_in(&["viMSati", "triMSat"]) {
            tp.try_add("5.1.24", qvun);
        } else if prati.has_text("kaMsa") {
            tp.try_add("5.1.25", wiWan);
        } else if prati.has_text("SUrpA") {
            tp.optional_try_add("5.1.26", aY);
        } else if prati.has_text_in(&["SatamAna", "viMSatika", "sahasra", "vasana"]) {
            tp.try_add("5.1.27", aR);
        } else if prati.has_text_in(&["dviSARa", "triSARa"]) {
            let code = "5.1.36";
            tp.try_add(code, yat);
            tp.try_add(code, aR);
        } else {
            try_base_cases_arhiya(tp, "5.1.37");
        }
    });

    tp.with_context(TasyaVapa, |tp| {
        let prati = tp.prati();
        if prati.has_text("pAtra") {
            tp.try_add("5.1.46", zWan);
        } else {
            try_base_cases_arhiya(tp, "5.1.45")
        }
    });

    tp.with_context(TadAsminVrddhiAyaLabhaSulkaUpada, |tp| {
        let prati = tp.prati();
        if prati.has_text("BAga") {
            let code = "5.1.49";
            tp.try_add(code, Wan);
            tp.try_add(code, yat);
        } else {
            try_base_cases_arhiya(tp, "5.1.47")
        }
    });

    tp.with_context(TadDharatiVahatiAvahati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["vasna", "dravya"]) {
            // vasnika, dravyaka
            let t = if prati.has_text("vasna") { Wan } else { kan };
            tp.try_add("5.1.51", t);
        } else {
            try_base_cases_arhiya(tp, "5.1.47")
        }
    });

    tp.with_context(SambhavatiAharatiPacati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["AQaka", "Acita", "pAtra"]) {
            tp.optional_try_add("5.1.53", Ka);
        }
        // prAsTika, ...
        try_base_cases_arhiya(tp, "5.1.52")
    });

    tp.with_context(TadArhati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::DANDA_ADI) {
            tp.try_add("5.1.66", yat);
        } else if prati.has_text("pAtra") {
            let code = "5.1.68";
            tp.try_add(code, yat);
            tp.try_add(code, Gan);
        } else if prati.has_text_in(&["kaqaNkara", "dakziRA"]) {
            let code = "5.1.69";
            tp.try_add(code, yat);
            tp.try_add(code, Ca);
        } else if prati.has_text("sTAlIbila") {
            let code = "5.1.70";
            tp.try_add(code, yat);
            tp.try_add(code, Ca);
        } else if prati.has_text_in(&["yajYa", "ftvij"]) {
            let t = if prati.has_text("yajYa") { Ga } else { KaY };
            tp.try_add("5.1.71", t);
        } else if tp.p.is_chandasi() {
            tp.try_add("5.1.67", yat);
        } else {
            try_base_cases_arhiya(tp, "5.1.63");
        }
    });

    tp.with_context(TadVartayati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["pArAyaRa", "turAyaRa", "cAndrAyaRa"]) {
            try_base_cases(tp, "5.1.72");
        }
    });

    tp.with_context(Apanna, |tp| {
        let prati = tp.prati();
        if prati.has_text("saMSaya") {
            try_base_cases(tp, "5.1.73");
        }
    });

    tp.with_context(Gacchati, |tp| {
        let i_prati = tp.i_prati;
        let prati = tp.prati();
        if prati.has_text("yojana") {
            tp.try_add("5.1.74", WaY);
        } else if prati.has_text("paTin") {
            tp.try_add("5.1.75", zkan);
            tp.try_add_with("5.1.76", Ra, |p| p.set(i_prati, |t| t.set_text("panTa")));
        }
    });

    tp.with_context(AbhigamanamArhati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["kroSaSata", "yojanaSata"]) {
            tp.try_add(Varttika("5.1.74", "1"), WaY);
        }
    });

    let prati = tp.prati();
    if prati.has_text("ftu") {
        tp.try_add("5.1.105", aR);
        tp.try_add("5.1.106", Gas);
    }
}
