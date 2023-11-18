/*!
Implements the taddhita rules in the "prAg GitAd yat" section of pada 4.4.

(4.4.75 - 4.4.144)
*/
use crate::args::Taddhita;
use crate::args::TaddhitaArtha::*;
use crate::ganapatha as gana;
use crate::taddhita::utils::TaddhitaPrakriya;
use crate::Rule;

pub fn run(tp: &mut TaddhitaPrakriya) {
    use Taddhita as P;

    tp.with_context(TadVahati, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["raTa", "yuga", "prAsaNga"]) {
            tp.try_add("4.4.78", P::yat);
        } else if prati.has_text("Dur") {
            let code = "4.4.79";
            tp.try_add(code, P::yat);
            tp.try_add(code, P::Qak);
        } else if prati.ends_with("DurA") {
            tp.try_add("4.4.78", P::Ka);
        } else if prati.has_text("Sakawa") {
            tp.try_add("4.4.80", P::aR);
        } else if prati.has_text_in(&["hala", "sIra"]) {
            tp.try_add("4.4.81", P::Wak);
        } else if prati.has_text("janI") {
            tp.try_add("4.4.82", P::yat);
        }
    });

    tp.with_context(TadVidhyati, |tp| {
        tp.try_add("4.4.83", P::yat);
    });

    tp.with_context(Labdha, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["Dana", "gaRa"]) {
            // Danya, gaRya
            tp.try_add("4.4.84", P::yat);
        } else if prati.has_text("anna") {
            // Anna
            tp.try_add("4.4.85", P::Ra);
        }
    });

    tp.with_context(Gata, |tp| {
        let prati = tp.prati();
        if prati.has_text("vaSa") {
            // vaSya
            tp.try_add("4.4.86", P::yat);
        }
    });

    tp.with_context(AsminDrshyam, |tp| {
        let prati = tp.prati();
        if prati.has_text("pada") {
            // padya
            tp.try_add("4.4.87", P::yat);
        }
    });

    tp.with_context(AsyaAbarhi, |tp| {
        let prati = tp.prati();
        if prati.has_text("mUla") {
            // mUlya
            tp.try_add("4.4.88", P::yat);
        }
    });

    tp.with_context(Samyukta, |tp| {
        let prati = tp.prati();
        if prati.has_text("gfhapati") {
            // gArhapatya
            tp.try_add("4.4.90", P::Yya);
        }
    });

    let prati = tp.prati();
    if prati.has_text_in(&["nO", "vayas", "Darma", "viza", "mUla", "sItA", "tulA"]) {
        tp.try_add("4.4.91", P::yat);
    }

    tp.with_context(Anapeta, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["Darma", "paTin", "arTa", "nyAya"]) {
            // gArhapatya
            tp.try_add("4.4.92", P::yat);
        }
    });

    tp.with_context(Nirmita, |tp| {
        let prati = tp.prati();
        if prati.has_text("Candas") {
            // Candasya
            tp.try_add("4.4.93", P::yat);
        } else if prati.has_text("uras") {
            // urasya, Orasa
            tp.optional_try_add("4.4.94.a", P::yat);
            tp.try_add("4.4.94.b", P::aR);
        }
    });

    tp.with_context(Priya, |tp| {
        let prati = tp.prati();
        if prati.has_text("hfdaya") {
            // hfdya
            tp.try_add("4.4.95", P::yat);
        }
    });

    let prati = tp.prati();
    if prati.has_text_in(&["mata", "jana", "hala"]) {
        tp.try_add("4.4.97", P::yat);
    }

    tp.with_context(TatraSadhu, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::PRATIJANA_ADI) {
            // prAtijanIna, ...
            tp.try_add("4.4.99", P::KaY);
        } else if prati.has_text("Bakta") {
            // BAkta
            tp.try_add("4.4.100", P::Ra);
        } else if prati.has_text("parizad") {
            // pArizada
            tp.try_add(Rule::Kashika("4.4.101"), P::Ra);
            // pArizadya
            tp.try_add("4.4.101", P::Rya);
        } else if prati.has_text_in(gana::KATHA_ADI) {
            // kATika, ...
            tp.try_add("4.4.102", P::Wak);
        } else if prati.has_text_in(gana::GUDA_ADI) {
            // gOqika, ...
            tp.try_add("4.4.103", P::WaY);
        } else if prati.has_text_in(&["paTi", "atiTi", "vasati", "svapati"]) {
            // pATeya, ...
            tp.try_add("4.4.104", P::QaY);
        } else if prati.has_text("saBA") {
            if tp.p.is_chandasi() {
                // saBeya
                tp.try_add("4.4.106", P::Qa);
            } else {
                // saBya
                tp.try_add("4.4.105", P::ya);
            }
        } else {
            // sAmanya, ...
            tp.try_add("4.4.98", P::yat);
        }
    });

    tp.with_context(TatraVasi, |tp| {
        let prati = tp.prati();
        if prati.has_text("samAnatIrTa") {
            // satIrTya
            tp.try_add("4.4.107", P::yat);
        }
    });

    tp.with_context(TatraBhava, |tp| {
        let prati = tp.prati();
        if tp.p.is_chandasi() {
            if prati.has_text_in(&["pATas", "nadI"]) {
                // pATya, ...
                tp.try_add("4.4.111", P::qyaR);
            } else if prati.has_text_in(&["veSanta", "himavat"]) {
                // vESanta, ...
                tp.try_add("4.4.112", P::aR);
            } else if prati.has_text("tugra") {
                // tugriya
                tp.try_add("4.4.115", P::Gan);
            } else if prati.has_text("agra") {
                tp.try_add("4.4.116", P::yat);
                let code = "4.4.117";
                tp.try_add(code, P::Ga);
                tp.try_add(code, P::Ca);
            } else if prati.has_text_in(&["samudra", "aBra"]) {
                tp.try_add("4.4.118", P::Ga);
            }
        }
    });
}
