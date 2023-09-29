use crate::args::Artha::*;
use crate::args::Taddhita::*;
use crate::taddhita::utils::TaddhitaPrakriya;

pub fn run(tp: &mut TaddhitaPrakriya) {
    const RASA_ADI: &[&str] = &[
        "rasa", "rUpa", "ganDa", "sparSa", "Sabda", "sneha", "guRAt", "ekAcaH",
    ];
    // TODO: others
    const SIDHMA_ADI: &[&str] = &[
        "siDma", "gaqu", "maRi", "nABi", "jIva", "nizpAva", "pAMsu", "saktu", "hanu", "mAMsa",
        "paraSu",
    ];
    // TODO: others
    const LOMA_ADI: &[&str] = &[
        "loman", "roman", "valgu", "baBrO", "hari", "kapi", "Suni", "taru",
    ];
    // TODO: others
    const PAMA_ADI: &[&str] = &[
        "pAman", "vAman", "heman", "Slezman", "kadru", "bali", "SrezWa", "palala", "sAman",
    ];
    // TODO: others
    const PICCHA_ADI: &[&str] = &[
        "picCa", "uras", "GruvakA", "kzuvakA", "varRa", "udaka", "paNka", "prajYA",
    ];

    tp.with_context(TadAsyaAstiAsmin, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(&RASA_ADI) {
            tp.try_add("5.2.95", matup);
        } else if prati.has_antya('A') {
            tp.optional_try_add("5.2.96", lac);
        } else if prati.has_text_in(&SIDHMA_ADI) {
            tp.optional_try_add("5.2.97", lac);
        } else if prati.has_text_in(&["vatsa", "aMsa"]) {
            tp.optional_try_add("5.2.98", lac);
        } else if prati.has_text("Pena") {
            let code = "5.2.99";
            tp.optional_try_add(code, lac);
            tp.optional_try_add(code, ilac);
        } else if prati.has_text_in(LOMA_ADI)
            || prati.has_text_in(PAMA_ADI)
            || prati.has_text_in(PICCHA_ADI)
        {
            let sub = if prati.has_text_in(LOMA_ADI) {
                Sa
            } else if prati.has_text_in(PAMA_ADI) {
                na
            } else {
                ilac
            };
            tp.try_add("5.2.100", sub);
        }

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
