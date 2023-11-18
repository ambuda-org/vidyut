/*!
Implements the taddhita rules in the "prAg vatez Wan" section of pada 5.1.

(5.1.18 - 5.1.114)
*/
use crate::args::Taddhita;
use crate::args::TaddhitaArtha::*;
use crate::core::Tag as T;
use crate::ganapatha as gana;
use crate::taddhita::utils::TaddhitaPrakriya;

fn try_base_cases(tp: &mut TaddhitaPrakriya, _code: &'static str) {
    use Taddhita as P;

    tp.try_add("5.1.18", P::WaY);
}

pub fn run(tp: &mut TaddhitaPrakriya) {
    use Taddhita as P;

    tp.with_context(TenaKritam, |tp| {
        let prati = tp.prati();
        if prati.has_tag(T::Sankhya) {
            tp.try_add("5.1.22", P::kan);
        } else if prati.has_text_in(&["viMSati", "triMSat"]) {
            tp.try_add("5.1.24", P::qvun);
        } else if prati.has_text("kaMsa") {
            tp.try_add("5.1.25", P::wiWan);
        } else if prati.has_text("SUrpA") {
            tp.optional_try_add("5.1.26", P::aY);
        } else if prati.has_text_in(&["SatamAna", "viMSatika", "sahasra", "vasana"]) {
            tp.try_add("5.1.27", P::aR);
        } else if prati.has_text_in(&["dviSARa", "triSARa"]) {
            let code = "5.1.36";
            tp.try_add(code, P::yat);
            tp.try_add(code, P::aR);
        } else {
            try_base_cases(tp, "5.1.37");
        }
    });

    tp.with_context(TadArhati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::DANDA_ADI) {
            tp.try_add("5.1.66", P::yat);
        } else if prati.has_text("pAtra") {
            let code = "5.1.68";
            tp.try_add(code, P::yat);
            tp.try_add(code, P::Gan);
        } else if prati.has_text_in(&["kaqaNgara", "dakzirA"]) {
            let code = "5.1.69";
            tp.try_add(code, P::yat);
            tp.try_add(code, P::Ca);
        } else if prati.has_text("sTAlIbila") {
            let code = "5.1.70";
            tp.try_add(code, P::yat);
            tp.try_add(code, P::Ca);
        } else if prati.has_text_in(&["yajYa", "ftvij"]) {
            let t = if prati.has_text("yajYa") {
                P::Ga
            } else {
                P::KaY
            };
            tp.try_add("5.1.71", t);
        } else {
            try_base_cases(tp, "5.1.63");
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
            try_base_cases(tp, "5.1.74");
        } else if prati.has_text("paTin") {
            tp.try_add("5.1.75", P::zkan);
            tp.try_add_with("5.1.76", P::Ra, |p| p.set(i_prati, |t| t.set_text("panTa")));
        }
    });

    let prati = tp.prati();
    if prati.has_text("ftu") {
        tp.try_add("5.1.105", P::aR);
        tp.try_add("5.1.106", P::Gas);
    }
}
