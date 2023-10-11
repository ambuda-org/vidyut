/*!
Implements the taddhita rules in the "prAg GitAd yat" section of pada 4.4.

(4.4.75 - 4.4.144)
*/
use crate::args::Taddhita;
use crate::args::TaddhitaArtha::*;
use crate::taddhita::utils::TaddhitaPrakriya;

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

    // [good first issue]
    //
    // Try implementing rule 4.4.84 and rule 4.4.85!
    //
    // - rule: add `P::yat` after the pratipadikas "Dana" or "gaRa."
    // - meaning context: `Labdha`. You will need to add `Labdha` as a new entry to `Artha`.
    // - test examples: `Danya` and `gaRya`.
    //   - You will need to add these to `pada_4_4.rs`.
    //   - Look at other tests in that file for help on the test syntax.
    //   - Remember to change the second argument for `assert_has_artha_taddhita` to `Labdha`.

    let prati = tp.prati();
    if prati.has_text_in(&["nO", "vayas", "Darma", "viza", "mUla", "sItA", "tulA"]) {
        tp.try_add("4.4.91", P::yat);
    }
}
