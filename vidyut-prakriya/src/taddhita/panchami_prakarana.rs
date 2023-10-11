use crate::args::Taddhita::*;
use crate::args::TaddhitaArtha::*;
use crate::taddhita::utils::TaddhitaPrakriya;

pub fn run(tp: &mut TaddhitaPrakriya) {
    tp.with_context(DhanyanamBhavaneKshetre, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["vrIhi", "SAli"]) {
            tp.try_add("5.2.2", Qak);
        } else if prati.has_text_in(&["yava", "yavaka", "zazwika"]) {
            tp.try_add("5.2.3", yat);
        } else if prati.has_text_in(&["tila", "mAza", "umA", "BaNgA", "aRu"]) {
            tp.optional_try_add("5.2.4", yat);
        }
        tp.try_add("5.2.1", KaY);
    });

    tp.with_context(Krta, |tp| {
        let prati = tp.prati();
        if prati.has_text("sarvacarman") {
            let code = "5.2.5";
            tp.try_add(code, Ka);
            tp.try_add(code, KaY);
        }
    });

    tp.with_context(Darshana, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&["yaTAmuKa", "sammuKa"]) {
            tp.try_add("5.2.6", Ka);
        }
    });

    tp.with_context(Vyapnoti, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&[
            "sarvapaTi",
            "sarvANga",
            "sarvakarman",
            "sarvapatra",
            "sarvapAtra",
        ]) {
            tp.try_add("5.2.7", Ka);
        }
    });

    tp.with_context(Prapnoti, |tp| {
        let prati = tp.prati();
        if prati.has_text("Aprapada") {
            tp.try_add("5.2.8", Ka);
        }
    });
}
