/*!
Implements the taddhita rules in the "prAg ivAt kaH" section of pada 5.3, as well as some rules
immediately before it.

(5.3.70 - 5.3.86)
*/
use crate::args::Artha::*;
use crate::args::Taddhita::*;
use crate::taddhita::utils::TaddhitaPrakriya;
use crate::tag::Tag;

pub fn run(tp: &mut TaddhitaPrakriya) {
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

    let code = "5.3.55";
    tp.try_add(code, tamap);
    tp.try_add(code, izWan);

    let code = "5.3.57";
    tp.try_add(code, tarap);
    tp.try_add(code, Iyasun);

    // vaiyAkaraRarUpa
    tp.try_add("5.3.66", rUpap);

    // pawukalpa
    let code = "5.3.67";
    tp.try_add(code, kalpap);
    tp.try_add(code, deSya);
    tp.try_add(code, deSIyar);

    let prati = tp.prati();
    if prati.is_sarvanama() {
        tp.try_add("5.3.71", akac);
    }
}
