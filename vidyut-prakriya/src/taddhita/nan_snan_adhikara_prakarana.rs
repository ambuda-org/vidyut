use crate::args::Taddhita::*;
use crate::args::TaddhitaArtha::*;
use crate::ganapatha as gana;
use crate::taddhita::utils::TaddhitaPrakriya;

pub fn run(tp: &mut TaddhitaPrakriya) {
    tp.with_context(TasyaBhava, |tp| {
        let prati = tp.prati();
        if prati.has_text_in(gana::PRTHU_ADI) {
            tp.try_add("5.1.122", imanic);
        } else if prati.has_text_in(&["mitrayu"]) {
            tp.try_add("5.1.134", vuY);
        }

        let code = "5.1.119";
        tp.try_add(code, tva);
        tp.try_add(code, tal);
    })
}
